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
        self.system = Some(match SystemCapture::start() {
            Ok(capture) => {
                if capture.is_capturing() {
                    tracing::info!("System audio (interviewer) channel is live");
                } else {
                    tracing::warn!(
                        "System audio not capturing yet — grant Screen Recording (macOS) or check WASAPI loopback (Windows)"
                    );
                }
                capture
            }
            Err(e) => {
                tracing::warn!(error = %e, "System audio unavailable; interviewer channel will be silent");
                SystemCapture::silent()
            }
        });
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

    /// RMS level of the mic channel (0.0–1.0) for UI metering. Does not consume samples.
    pub fn mic_level(&self) -> f32 {
        if !self.running {
            return 0.0;
        }
        self.mic.as_ref().map(|m| m.level()).unwrap_or(0.0)
    }

    pub fn system_level(&self) -> f32 {
        if !self.running {
            return 0.0;
        }
        self.system.as_ref().map(|s| s.level()).unwrap_or(0.0)
    }

    pub fn system_capturing(&self) -> bool {
        self.system
            .as_ref()
            .map(|s| s.is_capturing())
            .unwrap_or(false)
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
