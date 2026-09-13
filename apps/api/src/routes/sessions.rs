use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Json;
use futures::stream::{self, StreamExt};
use serde::Deserialize;
use sqlx::Row;
use std::convert::Infallible;
use uuid::Uuid;

use aniki_domain::{
    AnswerRequest, AppendTranscriptRequest, CreateSessionRequest, CreateSessionResponse,
    FinalizeSessionResponse, LlmModel, Session, SessionDetailResponse, SessionListResponse,
    SessionNotes, SessionStatus,
};

use crate::services::{billing, llm, notes, rag, speechmatics};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ConfirmQuestionRequest {
    pub text: String,
}

pub async fn list_sessions(
    State(state): State<AppState>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
) -> Result<Json<SessionListResponse>, StatusCode> {
    let rows = sqlx::query(
        "SELECT id, user_id, resume_id, status, model, extra_context, enable_screen_ocr, started_at, ended_at
         FROM sessions WHERE user_id = $1 ORDER BY started_at DESC LIMIT 50",
    )
    .bind(claims.sub)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let sessions = rows.into_iter().map(|r| row_to_session(&r)).collect();
    Ok(Json(SessionListResponse { sessions }))
}

pub async fn get_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
) -> Result<Json<SessionDetailResponse>, StatusCode> {
    let row = sqlx::query(
        "SELECT id, user_id, resume_id, status, model, extra_context, enable_screen_ocr, started_at, ended_at, transcript, notes
         FROM sessions WHERE id = $1 AND user_id = $2",
    )
    .bind(session_id)
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let session = row_to_session(&row);
    let transcript: Option<String> = row.get("transcript");
    let notes: Option<serde_json::Value> = row.get("notes");
    let notes = notes.and_then(|n| serde_json::from_value::<SessionNotes>(n).ok());

    Ok(Json(SessionDetailResponse {
        session,
        notes,
        transcript,
    }))
}

pub async fn append_transcript(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
    Json(req): Json<AppendTranscriptRequest>,
) -> Result<StatusCode, StatusCode> {
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM sessions WHERE id = $1 AND user_id = $2)")
            .bind(session_id)
            .bind(claims.sub)
            .fetch_one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !exists {
        return Err(StatusCode::NOT_FOUND);
    }

    let lines: Vec<String> = req
        .segments
        .iter()
        .filter(|s| s.is_final)
        .map(|s| format!("[{}] {}", s.speaker, s.text))
        .collect();

    if lines.is_empty() {
        return Ok(StatusCode::NO_CONTENT);
    }

    let block = lines.join("\n");
    sqlx::query(
        "UPDATE sessions SET transcript = COALESCE(transcript, '') || E'\\n' || $2 WHERE id = $1",
    )
    .bind(session_id)
    .bind(block)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn confirm_question(
    State(state): State<AppState>,
    Json(req): Json<ConfirmQuestionRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let is_question = llm::confirm_question(&state.config, &req.text)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({ "is_question": is_question })))
}

pub async fn create_session(
    State(state): State<AppState>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
    Json(req): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, StatusCode> {
    let credit_cost = state.config.session_credit_cost;

    let model = req.model.unwrap_or_default();
    let model_str = match model {
        LlmModel::Gpt41 => "gpt41",
        LlmModel::ClaudeSonnet => "claude_sonnet",
        LlmModel::Gpt41Mini => "gpt41_mini",
    };

    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(resume_id) = req.resume_id {
        let resume_is_usable: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1 FROM resumes
                WHERE id = $1 AND user_id = $2 AND status = 'ready'
            )",
        )
        .bind(resume_id)
        .bind(claims.sub)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if !resume_is_usable {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let session_row = sqlx::query(
        "INSERT INTO sessions (user_id, resume_id, model, extra_context, enable_screen_ocr)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, user_id, resume_id, status, model, extra_context, enable_screen_ocr, started_at, ended_at",
    )
    .bind(claims.sub)
    .bind(req.resume_id)
    .bind(model_str)
    .bind(&req.extra_context)
    .bind(req.enable_screen_ocr.unwrap_or(false))
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "create session failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let session_id: Uuid = session_row.get("id");

    let deducted = billing::deduct_credits(
        &mut tx,
        claims.sub,
        credit_cost,
        "session_start",
        Some(session_id),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !deducted {
        tx.rollback()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Err(StatusCode::PAYMENT_REQUIRED);
    }
    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut redis = state.redis.clone();
    if let Err(e) = rag::prewarm_session_context(
        &state.db,
        &mut redis,
        session_id,
        req.resume_id,
        req.extra_context.as_deref(),
    )
    .await
    {
        tracing::warn!(error = %e, %session_id, "prewarm context failed; answer will query Postgres");
    }

    let stt = if state.config.speechmatics_api_key.is_some() {
        speechmatics::mint_stt_jwt(&state.config, session_id, None).map_err(|e| {
            tracing::error!(error = %e, "STT JWT mint failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    } else {
        speechmatics::mint_dev_stt_jwt(&state.config, session_id)
    };

    Ok(Json(CreateSessionResponse {
        session: row_to_session(&session_row),
        stt_jwt: stt.jwt,
        stt_endpoint: stt.endpoint,
        stt_expires_at: stt.expires_at,
    }))
}

pub async fn answer_question(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
    Json(req): Json<AnswerRequest>,
) -> Result<Sse<impl stream::Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let row = sqlx::query(
        "SELECT model, resume_id, extra_context
         FROM sessions WHERE id = $1 AND user_id = $2 AND status = 'active'",
    )
    .bind(session_id)
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let model_str: String = row.get("model");
    let model = match model_str.as_str() {
        "claude_sonnet" => LlmModel::ClaudeSonnet,
        "gpt41_mini" => LlmModel::Gpt41Mini,
        _ => LlmModel::Gpt41,
    };

    let screen_ocr = req.screen_ocr.as_deref().map(truncate_ocr);
    let mut retrieval_query = req.question.clone();
    if let Some(ocr) = screen_ocr.as_deref() {
        retrieval_query.push('\n');
        retrieval_query.push_str(ocr);
    }

    let resume_id: Option<Uuid> = row.get("resume_id");
    let extra_context: Option<String> = row.get("extra_context");
    let mut context = if let Some(resume_id) = resume_id {
        rag::search_resume_context(&state.db, &state.config, resume_id, &retrieval_query, 8)
            .await
            .unwrap_or_else(|e| {
                tracing::warn!(error = %e, %session_id, "resume retrieval failed; using cached context");
                Vec::new()
            })
    } else {
        Vec::new()
    };
    if context.is_empty() {
        let mut redis = state.redis.clone();
        context = rag::get_session_context(&mut redis, session_id)
            .await
            .unwrap_or_default();
    }
    if let Some(extra) = extra_context.filter(|value| !value.is_empty()) {
        if !context.iter().any(|item| item == &extra) {
            context.push(extra);
        }
    }

    let llm_stream = llm::stream_answer(
        &state.config,
        model,
        &context,
        &req.question,
        req.transcript_context.as_deref(),
        screen_ocr.as_deref(),
    )
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "LLM stream failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let stream = llm_stream.flat_map(|chunk| match chunk {
        Ok(c) => {
            let mut events = Vec::new();
            if !c.text.is_empty() {
                events.push(Ok(Event::default().data(c.text)));
            }
            if c.done {
                events.push(Ok(Event::default().event("done").data("")));
            }
            stream::iter(events)
        }
        Err(e) => stream::iter(vec![Ok(Event::default()
            .event("error")
            .data(e.to_string()))]),
    });

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

fn truncate_ocr(value: &str) -> String {
    const MAX_OCR_CHARS: usize = 12_000;
    if value.chars().count() <= MAX_OCR_CHARS {
        return value.to_owned();
    }
    tracing::warn!(
        original_chars = value.chars().count(),
        max_chars = MAX_OCR_CHARS,
        "screen OCR truncated"
    );
    value.chars().take(MAX_OCR_CHARS).collect()
}

pub async fn finalize_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
) -> Result<Json<FinalizeSessionResponse>, StatusCode> {
    let result = sqlx::query(
        "UPDATE sessions SET status = 'ended', ended_at = NOW() WHERE id = $1 AND user_id = $2 AND status = 'active'",
    )
    .bind(session_id)
    .bind(claims.sub)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let pool = state.db.clone();
    let config = state.config.clone();
    tokio::spawn(async move {
        if let Err(e) = notes::generate_session_notes(&pool, &config, session_id).await {
            tracing::error!(error = %e, %session_id, "notes generation failed");
        }
    });

    Ok(Json(FinalizeSessionResponse {
        session_id,
        notes_status: "queued".to_string(),
    }))
}

fn row_to_session(row: &sqlx::postgres::PgRow) -> Session {
    let status_str: String = row.get("status");
    let model_str: String = row.get("model");
    let status = match status_str.as_str() {
        "ended" => SessionStatus::Ended,
        "failed" => SessionStatus::Failed,
        _ => SessionStatus::Active,
    };
    let model = match model_str.as_str() {
        "claude_sonnet" => LlmModel::ClaudeSonnet,
        "gpt41_mini" => LlmModel::Gpt41Mini,
        _ => LlmModel::Gpt41,
    };
    Session {
        id: row.get("id"),
        user_id: row.get("user_id"),
        resume_id: row.get("resume_id"),
        status,
        model,
        extra_context: row.get("extra_context"),
        enable_screen_ocr: row.get("enable_screen_ocr"),
        started_at: row.get("started_at"),
        ended_at: row.get("ended_at"),
    }
}

#[cfg(test)]
mod tests {
    use super::truncate_ocr;

    #[test]
    fn ocr_limit_counts_unicode_characters_without_splitting_them() {
        let input = "界".repeat(12_001);
        let truncated = truncate_ocr(&input);
        assert_eq!(truncated.chars().count(), 12_000);
        assert!(truncated.is_char_boundary(truncated.len()));
    }
}
