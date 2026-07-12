use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct StartRecognition {
    pub message: &'static str,
    pub audio_format: AudioFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AudioFormat {
    #[serde(rename = "type")]
    pub format_type: &'static str,
    pub encoding: &'static str,
    pub sample_rate: u32,
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self {
            format_type: "raw",
            encoding: "pcm_s16le",
            sample_rate: 16_000,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EndOfStream {
    pub message: &'static str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "message")]
pub enum SttServerMessage {
    #[serde(rename = "AddPartialTranscript")]
    AddPartialTranscript { metadata: TranscriptMetadata, results: Vec<TranscriptResult> },
    #[serde(rename = "AddTranscript")]
    AddTranscript { metadata: TranscriptMetadata, results: Vec<TranscriptResult> },
    #[serde(rename = "RecognitionStarted")]
    RecognitionStarted {},
    #[serde(rename = "Error")]
    Error { reason: String },
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TranscriptMetadata {
    pub transcript: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TranscriptResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub alternatives: Vec<TranscriptAlternative>,
    #[serde(default)]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TranscriptAlternative {
    pub content: String,
    #[serde(default)]
    pub speaker: Option<String>,
}

pub fn map_speaker(channel: Option<&str>, speaker: Option<&str>) -> String {
    match channel {
        Some("channel_0") | Some("0") | Some("interviewer") => "interviewer".to_string(),
        Some("channel_1") | Some("1") | Some("candidate") => "candidate".to_string(),
        _ => match speaker {
            Some("S1") | Some("UU") => "interviewer".to_string(),
            Some("S2") => "candidate".to_string(),
            _ => "interviewer".to_string(),
        },
    }
}
