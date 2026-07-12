use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumeStatus {
    Pending,
    Processing,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub status: ResumeStatus,
    pub chunk_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResumeResponse {
    pub resume: Resume,
    pub upload_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeListResponse {
    pub resumes: Vec<Resume>,
}
