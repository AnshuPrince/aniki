use sqlx::{PgPool, Row};
use uuid::Uuid;

use aniki_domain::{CreditLedgerEntry, CreditsResponse};

use crate::db;

pub async fn get_credits(pool: &PgPool, user_id: Uuid) -> anyhow::Result<CreditsResponse> {
    let balance = db::credits_balance(pool, user_id).await?;

    let rows = sqlx::query(
        "SELECT id, user_id, delta, reason, session_id, created_at
         FROM credit_ledger WHERE user_id = $1 ORDER BY created_at DESC LIMIT 20",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let entries = rows
        .into_iter()
        .map(|row| CreditLedgerEntry {
            id: row.get("id"),
            user_id: row.get("user_id"),
            delta: db::decimal_f64(&row, "delta"),
            reason: row.get("reason"),
            session_id: row.get("session_id"),
            created_at: row.get("created_at"),
        })
        .collect();

    Ok(CreditsResponse { balance, entries })
}

pub async fn deduct_credits(
    pool: &PgPool,
    user_id: Uuid,
    amount: f64,
    reason: &str,
    session_id: Option<Uuid>,
) -> anyhow::Result<bool> {
    let mut tx = pool.begin().await?;

    let balance = db::credits_balance_for_update(&mut tx, user_id).await?;

    if balance < amount {
        tx.rollback().await?;
        return Ok(false);
    }

    sqlx::query("UPDATE users SET credits = credits - $1, updated_at = NOW() WHERE id = $2")
        .bind(amount)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "INSERT INTO credit_ledger (user_id, delta, reason, session_id) VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(-amount)
    .bind(reason)
    .bind(session_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(true)
}
