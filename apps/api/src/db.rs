use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{postgres::PgRow, PgPool, Row};

pub type DbPool = PgPool;

fn env_usize(key: &str, default: u32) -> u32 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

pub async fn create_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let max_connections = env_usize("DATABASE_MAX_CONNECTIONS", 20);
    let acquire_timeout = Duration::from_secs(env_usize("DATABASE_ACQUIRE_TIMEOUT_SECS", 10) as u64);

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(1)
        .acquire_timeout(acquire_timeout)
        // Recycle connections so a Postgres restart cannot leave the pool
        // full of dead handles that only fail once a request needs them.
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "Could not connect to Postgres at {}: {e}. \
                 Is it running? Try: docker compose up -d postgres redis",
                redact_url(database_url)
            )
        })?;

    tracing::info!(max_connections, "Postgres pool ready");
    Ok(pool)
}

/// Strip credentials so connection errors are safe to log.
fn redact_url(url: &str) -> String {
    match (url.find("://"), url.rfind('@')) {
        (Some(scheme_end), Some(at)) if at > scheme_end => {
            format!("{}://***{}", &url[..scheme_end], &url[at..])
        }
        _ => url.to_string(),
    }
}

pub async fn run_migrations(pool: &DbPool) -> anyhow::Result<()> {
    for migration in [
        include_str!("../migrations/001_init.sql"),
        include_str!("../migrations/002_indexes.sql"),
    ] {
        for statement in migration.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("--") {
                sqlx::query(trimmed).execute(pool).await.ok();
            }
        }
    }
    Ok(())
}

pub fn decimal_f64(row: &PgRow, column: &str) -> f64 {
    row.get::<sqlx::types::BigDecimal, _>(column)
        .to_string()
        .parse()
        .unwrap_or(0.0)
}

pub async fn credits_balance(pool: &PgPool, user_id: uuid::Uuid) -> anyhow::Result<f64> {
    let row = sqlx::query("SELECT credits FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(decimal_f64(&row, "credits"))
}

pub async fn credits_balance_for_update(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: uuid::Uuid,
) -> anyhow::Result<f64> {
    let row = sqlx::query("SELECT credits FROM users WHERE id = $1 FOR UPDATE")
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await?;
    Ok(decimal_f64(&row, "credits"))
}
