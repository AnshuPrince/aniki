use axum::extract::State;
use axum::Json;
use serde_json::json;

use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> Json<serde_json::Value> {
    let db_ok = sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    Json(json!({
        "status": if db_ok { "ok" } else { "degraded" },
        "service": "aniki-api",
        "version": env!("CARGO_PKG_VERSION"),
        "database": if db_ok { "up" } else { "unreachable" },
        "db_pool": {
            "size": state.db.size(),
            "idle": state.db.num_idle(),
        },
    }))
}
