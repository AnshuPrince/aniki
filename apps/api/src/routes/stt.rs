use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use aniki_domain::SttJwtResponse;

use crate::services::speechmatics;
use crate::state::AppState;

pub async fn refresh_stt_jwt(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
) -> Result<Json<SttJwtResponse>, StatusCode> {
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM sessions WHERE id = $1 AND user_id = $2 AND status = 'active')",
    )
    .bind(session_id)
    .bind(claims.sub)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !exists {
        return Err(StatusCode::NOT_FOUND);
    }

    let stt = if state.config.speechmatics_api_key.is_some() {
        speechmatics::mint_stt_jwt(&state.config, session_id, None).map_err(|e| {
            tracing::error!(error = %e, "STT JWT refresh failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    } else {
        speechmatics::mint_dev_stt_jwt(&state.config, session_id)
    };

    Ok(Json(stt))
}
