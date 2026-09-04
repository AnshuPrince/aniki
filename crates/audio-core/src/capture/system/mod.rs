use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use crate::resample::{cap_buffer, f32_from_bits, resample_to_16k, rms_i16};

const MAX_BUFFER_SAMPLES: usize = 16_000 * 3;

/// System audio loopback (interviewer channel).
///
/// macOS: ScreenCaptureKit (requires Screen Recording permission).
/// Windows: WASAPI loopback on the default render device.
pub struct SystemCapture {
    pub(super) buffer: Arc<Mutex<Vec<i16>>>,
    pub(super) sample_rate: u32,
    pub(super) rms: Arc<AtomicU32>,
    pub(super) capturing: Arc<AtomicBool>,
    pub(super) _backend: Option<Box<dyn Send>>,
}

impl SystemCapture {
    pub fn silent() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            sample_rate: 16_000,
            rms: Arc::new(AtomicU32::new(0)),
            capturing: Arc::new(AtomicBool::new(false)),
            _backend: None,
        }
    }

    pub fn start() -> Result<Self, String> {
        #[cfg(target_os = "macos")]
        {
            return macos::start();
        }
        #[cfg(target_os = "windows")]
        {
            return windows::start();
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            tracing::warn!("System audio capture is not implemented on this platform");
            Ok(Self::silent())
        }
    }

    pub fn is_capturing(&self) -> bool {
        self.capturing.load(Ordering::Relaxed)
    }

    pub fn level(&self) -> f32 {
        f32_from_bits(self.rms.load(Ordering::Relaxed))
    }

    pub fn drain_mono_16k(&mut self) -> Vec<i16> {
        let samples = {
            let mut buf = self.buffer.lock().expect("system audio buffer lock");
            std::mem::take(&mut *buf)
        };
        if samples.is_empty() {
            return vec![];
        }
        resample_to_16k(&samples, self.sample_rate)
    }
}

fn push_mono_i16(
    buffer: &Arc<Mutex<Vec<i16>>>,
    rms: &Arc<AtomicU32>,
    capturing: &Arc<AtomicBool>,
    samples: &[i16],
) {
    if samples.is_empty() {
        return;
    }
    capturing.store(true, Ordering::Relaxed);
    rms.store(crate::resample::f32_bits(rms_i16(samples)), Ordering::Relaxed);
    let mut lock = buffer.lock().expect("system audio buffer lock");
    lock.extend_from_slice(samples);
    cap_buffer(&mut lock, MAX_BUFFER_SAMPLES);
}

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;
