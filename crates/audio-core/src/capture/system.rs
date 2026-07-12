/// System audio loopback capture (interviewer channel).
///
/// Platform implementation deferred: ScreenCaptureKit (macOS) / WASAPI loopback (Windows).
/// Returns silence until platform capture is wired — mic-only sessions still work.
pub struct SystemCapture;

impl SystemCapture {
    pub fn start() -> Result<Self, String> {
        tracing::warn!(
            "System audio capture not yet implemented for this platform; interviewer channel will be silent"
        );
        Ok(Self)
    }

    pub fn drain_mono_16k(&mut self) -> Vec<i16> {
        vec![]
    }
}
