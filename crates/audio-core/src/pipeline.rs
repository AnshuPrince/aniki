use crate::capture::{MicCapture, SystemCapture};

#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub vad_enabled: bool,
    pub vad_threshold: f32,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16_000,
            channels: 2,
            // VAD on short capture windows drops too much speech; STT handles silence.
            vad_enabled: false,
            vad_threshold: 0.15,
        }
    }
}

pub struct DualCapture {
    mic: Option<MicCapture>,
    system: Option<SystemCapture>,
    config: PipelineConfig,
    running: bool,
}

impl DualCapture {
    pub fn new(config: PipelineConfig) -> Self {
        Self {
            mic: None,
            system: None,
            config,
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.running {
            return Err("already running".to_string());
        }
        self.mic = Some(MicCapture::start()?);
        self.system = Some(SystemCapture::start()?);
        self.running = true;
        tracing::info!("Dual audio capture started");
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.mic = None;
        self.system = None;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn drain_interleaved_16k(&mut self) -> Vec<i16> {
        if !self.running {
            return vec![];
        }
        let vad = self.config.vad_enabled;
        let interviewer = self
            .system
            .as_mut()
            .map(|s| s.drain_mono_16k())
            .unwrap_or_default();
        let candidate = self
            .mic
            .as_mut()
            .map(|m| m.drain_mono_16k(vad))
            .unwrap_or_default();

        if interviewer.is_empty() && candidate.is_empty() {
            return vec![];
        }

        interleave_for_stt(&interviewer, &candidate)
    }

    /// RMS level of the mic channel (0.0–1.0) for UI metering.
    pub fn mic_level(&mut self) -> f32 {
        if !self.running {
            return 0.0;
        }
        let samples = self
            .mic
            .as_mut()
            .map(|m| m.drain_mono_16k(false))
            .unwrap_or_default();
        if samples.is_empty() {
            return 0.0;
        }
        let energy: f64 = samples
            .iter()
            .map(|&s| {
                let n = s as f64 / i16::MAX as f64;
                n * n
            })
            .sum::<f64>()
            / samples.len() as f64;
        (energy.sqrt() as f32).min(1.0)
    }

    pub fn config(&self) -> &PipelineConfig {
        &self.config
    }
}

pub fn interleave_for_stt(interviewer: &[i16], candidate: &[i16]) -> Vec<i16> {
    let len = interviewer.len().max(candidate.len());
    let mut interleaved = Vec::with_capacity(len * 2);
    for i in 0..len {
        interleaved.push(interviewer.get(i).copied().unwrap_or(0));
        interleaved.push(candidate.get(i).copied().unwrap_or(0));
    }
    interleaved
}

pub type AudioPipeline = DualCapture;

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("pipeline is already running")]
    AlreadyRunning,
    #[error("pipeline is not running")]
    NotRunning,
    #[error("capture error: {0}")]
    Capture(String),
}
