use sqlx::postgres::PgPoolOptions;
use sqlx::{postgres::PgRow, PgPool, Row};

pub type DbPool = PgPool;

pub async fn create_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
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
