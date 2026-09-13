/// Simple energy-based voice activity detection.
///
/// Production VAD will use a proper model (e.g. WebRTC VAD or silero).
pub struct VoiceActivityDetector {
    threshold: f32,
    _sample_rate: u32,
}

impl VoiceActivityDetector {
    pub fn new(threshold: f32, sample_rate: u32) -> Self {
        Self {
            threshold,
            _sample_rate: sample_rate,
        }
    }

    /// Returns true if the audio chunk contains speech above the energy threshold.
    pub fn is_speech(&self, samples: &[i16]) -> bool {
        if samples.is_empty() {
            return false;
        }
        let energy: f64 = samples
            .iter()
            .map(|&s| {
                let normalized = s as f64 / i16::MAX as f64;
                normalized * normalized
            })
            .sum::<f64>()
            / samples.len() as f64;
        energy > (self.threshold as f64).powi(2)
    }

    pub fn filter_speech<'a>(&self, samples: &'a [i16]) -> Option<&'a [i16]> {
        if self.is_speech(samples) {
            Some(samples)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silence_is_not_speech() {
        let vad = VoiceActivityDetector::new(0.5, 16_000);
        let silence = vec![0i16; 320];
        assert!(!vad.is_speech(&silence));
    }

    #[test]
    fn loud_signal_is_speech() {
        let vad = VoiceActivityDetector::new(0.1, 16_000);
        let loud: Vec<i16> = (0..320).map(|i| ((i as f32 * 0.1).sin() * 10_000.0) as i16).collect();
        assert!(vad.is_speech(&loud));
    }
}
