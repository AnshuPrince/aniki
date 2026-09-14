use aniki_domain::{Resume, ResumeListResponse, ResumeStatus, UploadResumeResponse};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use base64::{engine::general_purpose::STANDARD, Engine};

use crate::services::rag;
use crate::state::AppState;

#[derive(serde::Deserialize)]
pub struct UploadResumeRequest {
    pub filename: String,
    pub content_base64: Option<String>,
}

pub async fn list_resumes(
    State(state): State<AppState>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
) -> Result<Json<ResumeListResponse>, StatusCode> {
    let resumes = sqlx::query_as::<_, ResumeRow>(
        "SELECT id, user_id, filename, status, chunk_count, created_at, updated_at
         FROM resumes WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(claims.sub)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "list resumes failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .into_iter()
    .map(ResumeRow::into_resume)
    .collect();

    Ok(Json(ResumeListResponse { resumes }))
}

pub async fn upload_resume(
    State(state): State<AppState>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
    Json(req): Json<UploadResumeRequest>,
) -> Result<(StatusCode, Json<UploadResumeResponse>), StatusCode> {
    let resume = sqlx::query_as::<_, ResumeRow>(
        "INSERT INTO resumes (user_id, filename, status)
         VALUES ($1, $2, 'pending')
         RETURNING id, user_id, filename, status, chunk_count, created_at, updated_at",
    )
    .bind(claims.sub)
    .bind(&req.filename)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "upload resume failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Some(b64) = req.content_base64 {
        let bytes = STANDARD.decode(b64).map_err(|_| StatusCode::BAD_REQUEST)?;
        let pool = state.db.clone();
        let config = state.config.clone();
        let resume_id = resume.id;
        let filename = req.filename.clone();
        tokio::spawn(async move {
            if let Err(e) = rag::process_resume(&pool, &config, resume_id, &filename, &bytes).await
            {
                tracing::error!(error = %e, %resume_id, "resume processing failed");
            }
        });
    }

    Ok((
        StatusCode::ACCEPTED,
        Json(UploadResumeResponse {
            resume: resume.into_resume(),
            upload_url: None,
        }),
    ))
}

pub async fn delete_resume(
    State(state): State<AppState>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
    Path(resume_id): Path<uuid::Uuid>,
) -> Result<StatusCode, StatusCode> {
    let deleted = sqlx::query("DELETE FROM resumes WHERE id = $1 AND user_id = $2 RETURNING id")
        .bind(resume_id)
        .bind(claims.sub)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, %resume_id, "delete resume failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    deleted.map_or(Err(StatusCode::NOT_FOUND), |_| Ok(StatusCode::NO_CONTENT))
}

struct ResumeRow {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    filename: String,
    status: String,
    chunk_count: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for ResumeRow {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        Ok(Self {
            id: row.get("id"),
            user_id: row.get("user_id"),
            filename: row.get("filename"),
            status: row.get("status"),
            chunk_count: row.get("chunk_count"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

impl ResumeRow {
    fn into_resume(self) -> Resume {
        let status = match self.status.as_str() {
            "processing" => ResumeStatus::Processing,
            "ready" => ResumeStatus::Ready,
            "failed" => ResumeStatus::Failed,
            _ => ResumeStatus::Pending,
        };
        Resume {
            id: self.id,
            user_id: self.user_id,
            filename: self.filename,
            status,
            chunk_count: self.chunk_count,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
