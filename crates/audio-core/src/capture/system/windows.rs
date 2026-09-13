use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use wasapi::*;

use super::{push_mono_i16, SystemCapture};

struct LoopbackKeepAlive {
    stop: Arc<AtomicBool>,
    _thread: JoinHandle<()>,
}

impl Drop for LoopbackKeepAlive {
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
        if let Err(e) = run_wasapi_loopback(buf_t, rms_t, cap_t, stop_t, ready_tx) {
            tracing::error!(error = %e, "WASAPI loopback thread failed");
        }
    });

    let sample_rate = match ready_rx.recv() {
        Ok(Ok(rate)) => rate,
        Ok(Err(e)) => {
            tracing::warn!(error = %e, "WASAPI system audio unavailable");
            return Ok(SystemCapture::silent());
        }
        Err(_) => return Ok(SystemCapture::silent()),
    };

    tracing::info!(sample_rate, "System audio capture started (WASAPI loopback)");
    Ok(SystemCapture {
        buffer,
        sample_rate,
        rms,
        capturing,
        _backend: Some(Box::new(LoopbackKeepAlive { stop, _thread: thread })),
    })
}

fn run_wasapi_loopback(
    buffer: Arc<Mutex<Vec<i16>>>,
    rms: Arc<AtomicU32>,
    capturing: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    ready_tx: mpsc::SyncSender<Result<u32, String>>,
) -> Result<(), String> {
    initialize_mta()
        .ok()
        .map_err(|e| e.to_string())?;

    let device = get_default_device(&Direction::Render).map_err(|e| e.to_string())?;
    let mut audio_client = device.get_iaudioclient().map_err(|e| e.to_string())?;
    let mix_format = audio_client.get_mixformat().map_err(|e| e.to_string())?;
    let sample_rate = mix_format.get_samplespersec();
    let channels = mix_format.get_nchannels() as usize;

    let stream_mode = StreamMode::EventsShared {
        autoconvert: true,
        buffer_duration_hns: 200_000,
    };

    audio_client
        .initialize_client(&mix_format, &Direction::Capture, &stream_mode)
        .map_err(|e| e.to_string())?;

    let h_event = audio_client.set_get_eventhandle().map_err(|e| e.to_string())?;
    let capture_client = audio_client
        .get_audiocaptureclient()
        .map_err(|e| e.to_string())?;
    audio_client.start_stream().map_err(|e| e.to_string())?;

    let bytes_per_frame = mix_format.get_blockalign().max(1) as usize;
    let _ = ready_tx.send(Ok(sample_rate));

    while !stop.load(Ordering::Relaxed) {
        let _ = h_event.wait_for_event(200);
        let frames_available = match capture_client.get_next_packet_size() {
            Ok(Some(n)) if n > 0 => n as usize,
            Ok(_) => continue,
            Err(_) => {
                thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }
        };
        let mut frames = vec![0u8; frames_available * bytes_per_frame];
        match capture_client.read_from_device(&mut frames) {
            Ok((frames_read, _)) if frames_read > 0 => {
                let used = (frames_read as usize).saturating_mul(bytes_per_frame);
                let pcm = bytes_to_mono_i16(
                    &frames[..used.min(frames.len())],
                    &mix_format,
                    channels,
                );
                push_mono_i16(&buffer, &rms, &capturing, &pcm);
            }
            Ok(_) => {}
            Err(_) => {
                thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }

    let _ = audio_client.stop_stream();
    Ok(())
}

fn bytes_to_mono_i16(bytes: &[u8], format: &WaveFormat, channels: usize) -> Vec<i16> {
    let bits = format.get_bitspersample();
    let ch = channels.max(1);
    if bits == 32 && bytes.len() % 4 == 0 {
        let mut out = Vec::new();
        let frame_bytes = 4 * ch;
        for frame in bytes.chunks(frame_bytes) {
            if frame.len() < 4 {
                break;
            }
            let sample = f32::from_le_bytes(frame[0..4].try_into().unwrap());
            out.push((sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16);
        }
        return out;
    }
    if bits == 16 && bytes.len() % 2 == 0 {
        let mut out = Vec::new();
        let frame_bytes = 2 * ch;
        for frame in bytes.chunks(frame_bytes) {
            if frame.len() < 2 {
                break;
            }
            out.push(i16::from_le_bytes(frame[0..2].try_into().unwrap()));
        }
        return out;
    }
    vec![]
}
