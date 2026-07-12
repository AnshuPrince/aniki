use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::config::Config;
use crate::redis::RedisPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisPool,
    pub config: Arc<Config>,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for RedisPool {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}

impl FromRef<AppState> for Arc<Config> {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}
