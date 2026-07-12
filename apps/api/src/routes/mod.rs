mod auth;
mod billing;
mod health;
mod resumes;
mod sessions;
mod stt;

use axum::routing::{get, post};
use axum::Router;

use crate::middleware::auth_middleware;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let public = Router::new()
        .route("/health", get(health::health_check))
        .route("/auth/magic-link", post(auth::request_magic_link))
        .route("/auth/verify", post(auth::verify_token));

    let protected = Router::new()
        .route("/auth/me", get(auth::me))
        .route("/resumes", get(resumes::list_resumes).post(resumes::upload_resume))
        .route("/sessions", get(sessions::list_sessions).post(sessions::create_session))
        .route("/sessions/{id}", get(sessions::get_session))
        .route("/sessions/{id}/transcript", post(sessions::append_transcript))
        .route("/sessions/{id}/answer", post(sessions::answer_question))
        .route("/sessions/{id}/finalize", post(sessions::finalize_session))
        .route("/sessions/{id}/stt-jwt", post(stt::refresh_stt_jwt))
        .route("/questions/confirm", post(sessions::confirm_question))
        .route("/billing/credits", get(billing::get_credits))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state)
}
