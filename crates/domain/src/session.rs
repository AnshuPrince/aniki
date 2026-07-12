use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Ended,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmModel {
    Gpt41,
    ClaudeSonnet,
    Gpt41Mini,
}

impl Default for LlmModel {
    fn default() -> Self {
        Self::Gpt41
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub resume_id: Option<Uuid>,
    pub model: Option<LlmModel>,
    pub extra_context: Option<String>,
    pub enable_screen_ocr: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub resume_id: Option<Uuid>,
    pub status: SessionStatus,
    pub model: LlmModel,
    pub extra_context: Option<String>,
    pub enable_screen_ocr: bool,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionResponse {
    pub session: Session,
    pub stt_jwt: String,
    pub stt_endpoint: String,
    pub stt_expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerRequest {
    pub question: String,
    pub transcript_context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizeSessionResponse {
    pub session_id: Uuid,
    pub notes_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionNotes {
    pub session_id: Uuid,
    pub summary: String,
    pub key_points: Vec<String>,
    pub action_items: Vec<String>,
    pub generated_at: DateTime<Utc>,
}
