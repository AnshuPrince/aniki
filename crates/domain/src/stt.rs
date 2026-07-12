use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttJwtResponse {
    pub jwt: String,
    pub endpoint: String,
    pub expires_at: DateTime<Utc>,
    pub transcription_config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechmaticsTranscriptionConfig {
    pub language: String,
    pub enable_partials: bool,
    pub max_delay: f64,
    pub diarization: String,
    pub channel_diarization_labels: Vec<String>,
}

impl Default for SpeechmaticsTranscriptionConfig {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            enable_partials: true,
            max_delay: 0.7,
            diarization: "channel".to_string(),
            channel_diarization_labels: vec![
                "interviewer".to_string(),
                "candidate".to_string(),
            ],
        }
    }
}

impl SpeechmaticsTranscriptionConfig {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "language": self.language,
            "enable_partials": self.enable_partials,
            "max_delay": self.max_delay,
            "diarization": self.diarization,
            "channel_diarization_labels": self.channel_diarization_labels,
        })
    }
}
