use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use core_media_rs::cm_sample_buffer::CMSampleBuffer;
use screencapturekit::{
    shareable_content::SCShareableContent,
    stream::{
        configuration::SCStreamConfiguration, content_filter::SCContentFilter,
        output_trait::SCStreamOutputTrait, output_type::SCStreamOutputType, SCStream,
    },
};

use super::{push_mono_i16, SystemCapture};

struct StreamKeepAlive {
    stop: Arc<AtomicBool>,
    _thread: JoinHandle<()>,
}

impl Drop for StreamKeepAlive {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

pub fn start() -> Result<SystemCapture, String> {
    let buffer = Arc::new(Mutex::new(Vec::<i16>::new()));
    let rms = Arc::new(AtomicU32::new(0));
    let capturing = Arc::new(AtomicBool::new(false));
    let stop = Arc::new(AtomicBool::new(false));

    let buf_t = buffer.clone();
    let rms_t = rms.clone();
    let cap_t = capturing.clone();
    let stop_t = stop.clone();
    let (ready_tx, ready_rx) = mpsc::sync_channel::<Result<u32, String>>(1);

    let thread = thread::spawn(move || {
        if let Err(e) = run_sck_thread(buf_t, rms_t, cap_t, stop_t, ready_tx) {
            tracing::error!(error = %e, "system audio thread failed");
        }
    });

    let sample_rate = match ready_rx.recv() {
        Ok(Ok(rate)) => rate,
        Ok(Err(e)) => {
            tracing::warn!(error = %e, "ScreenCaptureKit system audio unavailable");
            return Ok(SystemCapture::silent());
        }
        Err(_) => {
            tracing::warn!("system audio thread exited before ready");
            return Ok(SystemCapture::silent());
        }
    };

    tracing::info!(sample_rate, "System audio capture started (ScreenCaptureKit)");
    Ok(SystemCapture {
        buffer,
        sample_rate,
        rms,
        capturing,
        _backend: Some(Box::new(StreamKeepAlive {
            stop,
            _thread: thread,
        })),
    })
}

struct AudioOutput {
    buffer: Arc<Mutex<Vec<i16>>>,
    rms: Arc<AtomicU32>,
    capturing: Arc<AtomicBool>,
}

impl SCStreamOutputTrait for AudioOutput {
    fn did_output_sample_buffer(&self, sample_buffer: CMSampleBuffer, of_type: SCStreamOutputType) {
        if of_type != SCStreamOutputType::Audio {
            return;
        }
        let Ok(list) = sample_buffer.get_audio_buffer_list() else {
            return;
        };
        let mut mixed: Vec<i16> = Vec::new();
        for i in 0..list.num_buffers() {
            let Some(buf) = list.get(i) else {
                continue;
            };
            let samples = f32_bytes_to_i16(buf.data());
            if mixed.is_empty() {
                mixed = samples;
            } else {
                for (idx, s) in samples.iter().enumerate() {
                    if idx < mixed.len() {
                        mixed[idx] = ((mixed[idx] as i32 + i32::from(*s)) / 2) as i16;
                    }
                }
            }
        }
        if !mixed.is_empty() {
            push_mono_i16(&self.buffer, &self.rms, &self.capturing, &mixed);
        }
    }
}

fn run_sck_thread(
    buffer: Arc<Mutex<Vec<i16>>>,
    rms: Arc<AtomicU32>,
    capturing: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    ready_tx: mpsc::SyncSender<Result<u32, String>>,
) -> Result<(), String> {
    let sample_rate: u32 = 48_000;
    let config = SCStreamConfiguration::new()
        .set_captures_audio(true)
        .map_err(|e| e.to_string())?
        .set_excludes_current_process_audio(true)
        .map_err(|e| e.to_string())?
        .set_sample_rate(sample_rate)
        .map_err(|e| e.to_string())?
        .set_channel_count(2)
        .map_err(|e| e.to_string())?
        .set_width(8)
        .map_err(|e| e.to_string())?
        .set_height(8)
        .map_err(|e| e.to_string())?;

    let mut displays = SCShareableContent::get()
        .map_err(|e| {
            format!(
                "ScreenCaptureKit failed ({e}). Grant Screen Recording to Aniki in System Settings → Privacy & Security → Screen Recording."
            )
        })?
        .displays();
    let display = displays.pop().ok_or_else(|| {
        "No display available for system audio capture".to_string()
    })?;
    let filter = SCContentFilter::new().with_display_excluding_windows(&display, &[]);

    let mut stream = SCStream::new(&filter, &config);
    stream.add_output_handler(
        AudioOutput {
            buffer,
            rms,
            capturing,
        },
        SCStreamOutputType::Audio,
    );
    stream
        .start_capture()
        .map_err(|e| format!("Failed to start ScreenCaptureKit audio: {e}"))?;

    let _ = ready_tx.send(Ok(sample_rate));

    while !stop.load(Ordering::Relaxed) {
        thread::sleep(std::time::Duration::from_millis(200));
    }

    let _ = stream.stop_capture();
    Ok(())
}

fn f32_bytes_to_i16(bytes: &[u8]) -> Vec<i16> {
    bytes
        .chunks_exact(4)
        .map(|c| {
            let v = f32::from_le_bytes(c.try_into().unwrap());
            (v.clamp(-1.0, 1.0) * i16::MAX as f32) as i16
        })
        .collect()
}
