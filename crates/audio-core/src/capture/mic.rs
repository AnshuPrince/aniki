use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;

use crate::vad::VoiceActivityDetector;

static MIC_SAMPLES_SEEN: AtomicBool = AtomicBool::new(false);

pub struct MicCapture {
    buffer: Arc<Mutex<Vec<i16>>>,
    sample_rate: u32,
    vad: VoiceActivityDetector,
    _thread: Option<JoinHandle<()>>,
}

impl MicCapture {
    pub fn start() -> Result<Self, String> {
        MIC_SAMPLES_SEEN.store(false, Ordering::Relaxed);
        let buffer = Arc::new(Mutex::new(Vec::<i16>::new()));
        let buf_thread = buffer.clone();
        let (ready_tx, ready_rx) = mpsc::sync_channel::<Result<u32, String>>(1);

        // cpal::Stream is !Send on macOS — build and own it entirely on this thread.
        let thread = thread::spawn(move || {
            let result = run_mic_thread(buf_thread, ready_tx);
            if let Err(e) = result {
                tracing::error!(error = %e, "mic capture thread failed");
            }
        });

        let sample_rate = ready_rx
            .recv()
            .map_err(|_| "mic thread exited before ready".to_string())??;

        tracing::info!(sample_rate, "Microphone capture started");
        Ok(Self {
            buffer,
            sample_rate,
            vad: VoiceActivityDetector::new(0.15, sample_rate),
            _thread: Some(thread),
        })
    }

    pub fn has_received_samples(&self) -> bool {
        MIC_SAMPLES_SEEN.load(Ordering::Relaxed)
    }

    pub fn drain_mono_16k(&mut self, vad_enabled: bool) -> Vec<i16> {
        let samples = {
            let mut buf = self.buffer.lock().expect("mic buffer lock");
            std::mem::take(&mut *buf)
        };
        if samples.is_empty() {
            return vec![];
        }
        let resampled = resample_to_16k(&samples, self.sample_rate);
        if vad_enabled && !self.vad.is_speech(&resampled) {
            return vec![];
        }
        resampled
    }
}

fn run_mic_thread(
    buffer: Arc<Mutex<Vec<i16>>>,
    ready_tx: mpsc::SyncSender<Result<u32, String>>,
) -> Result<(), String> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| "No input device".to_string())?;
    let device_name = device.name().unwrap_or_else(|_| "unknown".to_string());
    tracing::info!(device = %device_name, "Opening default input device");

    let config = device.default_input_config().map_err(|e| e.to_string())?;
    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;

    let stream = match config.sample_format() {
        SampleFormat::F32 => device
            .build_input_stream(
                &config.into(),
                move |data: &[f32], _| append_f32(&buffer, data, channels),
                |e| tracing::error!(error = %e, "mic stream error"),
                None,
            )
            .map_err(|e| e.to_string())?,
        SampleFormat::I16 => device
            .build_input_stream(
                &config.into(),
                move |data: &[i16], _| append_i16(&buffer, data, channels),
                |e| tracing::error!(error = %e, "mic stream error"),
                None,
            )
            .map_err(|e| e.to_string())?,
        _ => return Err("Unsupported sample format".to_string()),
    };

    stream.play().map_err(|e| e.to_string())?;
    let _ = ready_tx.send(Ok(sample_rate));

    loop {
        thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn append_f32(buf: &Arc<Mutex<Vec<i16>>>, data: &[f32], channels: usize) {
    if !data.is_empty() && !MIC_SAMPLES_SEEN.swap(true, Ordering::Relaxed) {
        tracing::info!("Microphone receiving audio samples");
    }
    let mut lock = buf.lock().expect("mic buffer lock");
    for frame in data.chunks(channels) {
        let sample = frame.first().copied().unwrap_or(0.0);
        lock.push((sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16);
    }
}

fn append_i16(buf: &Arc<Mutex<Vec<i16>>>, data: &[i16], channels: usize) {
    if !data.is_empty() && !MIC_SAMPLES_SEEN.swap(true, Ordering::Relaxed) {
        tracing::info!("Microphone receiving audio samples");
    }
    let mut lock = buf.lock().expect("mic buffer lock");
    for frame in data.chunks(channels) {
        if let Some(&s) = frame.first() {
            lock.push(s);
        }
    }
}

fn resample_to_16k(samples: &[i16], from_rate: u32) -> Vec<i16> {
    if from_rate == 16_000 || samples.is_empty() {
        return samples.to_vec();
    }
    let ratio = 16_000.0 / from_rate as f64;
    let out_len = ((samples.len() as f64) * ratio) as usize;
    let mut out = Vec::with_capacity(out_len);
    for i in 0..out_len {
        let src_idx = (i as f64 / ratio) as usize;
        out.push(samples.get(src_idx).copied().unwrap_or(0));
    }
    out
}
