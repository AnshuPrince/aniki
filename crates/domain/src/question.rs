use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub id: String,
    pub speaker: String,
    pub channel: Option<String>,
    pub text: String,
    pub is_final: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionDetectedEvent {
    pub question: String,
    pub segment_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendTranscriptRequest {
    pub segments: Vec<TranscriptSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub sessions: Vec<crate::Session>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDetailResponse {
    pub session: crate::Session,
    pub notes: Option<crate::SessionNotes>,
    pub transcript: Option<String>,
}
