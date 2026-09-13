use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use aniki_domain::{
    AuthResponse, GoogleOAuthCallbackRequest, GoogleOAuthStartResponse, MagicLinkRequest,
    MagicLinkResponse, UserProfile, VerifyTokenRequest,
};

use crate::services::auth;
use crate::state::AppState;

pub async fn request_magic_link(
    State(state): State<AppState>,
    Json(req): Json<MagicLinkRequest>,
) -> Result<Json<MagicLinkResponse>, StatusCode> {
    auth::request_magic_link(&state.db, &state.config, &req.email)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!(error = %e, "magic link request failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn verify_token(
    State(state): State<AppState>,
    Json(req): Json<VerifyTokenRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    auth::verify_magic_link(&state.db, &state.config, &req)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::warn!(error = %e, "token verification failed");
            StatusCode::UNAUTHORIZED
        })
}

pub async fn me(
    State(state): State<AppState>,
    axum::Extension(claims): axum::Extension<aniki_domain::SessionClaims>,
) -> Result<Json<UserProfile>, StatusCode> {
    let user = auth::get_user_by_id(&state.db, claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user.into()))
}

pub async fn google_oauth_start(
    State(state): State<AppState>,
) -> Result<Json<GoogleOAuthStartResponse>, StatusCode> {
    let mut redis = state.redis.clone();
    auth::start_google_oauth(&mut redis, &state.config)
        .await
        .map(Json)
        .map_err(|e| {
            if !matches!(e, auth::OauthError::Unconfigured) {
                tracing::warn!(error = ?e, "google oauth start failed");
            }
            e.status()
        })
}

pub async fn google_oauth_callback(
    State(state): State<AppState>,
    Json(req): Json<GoogleOAuthCallbackRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let mut redis = state.redis.clone();
    auth::finish_google_oauth(&state.db, &mut redis, &state.config, &req)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::warn!(error = ?e, "google oauth callback failed");
            e.status()
        })
}
