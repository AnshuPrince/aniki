use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use aniki_domain::CreditsResponse;

use crate::state::AppState;

pub async fn get_credits(
    State(state): State<AppState>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
) -> Result<Json<CreditsResponse>, StatusCode> {
    crate::services::billing::get_credits(&state.db, claims.sub)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!(error = %e, "get credits failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
