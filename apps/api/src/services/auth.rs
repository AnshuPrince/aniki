use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use aniki_domain::{AuthResponse, MagicLinkResponse, SessionClaims, User, VerifyTokenRequest};

use crate::config::Config;
use crate::db;

pub async fn request_magic_link(
    pool: &PgPool,
    config: &Config,
    email: &str,
) -> anyhow::Result<MagicLinkResponse> {
    let token = Uuid::new_v4().to_string();
    let token_hash = hash_token(&token);
    let expires_at = Utc::now() + Duration::hours(1);

    sqlx::query(
        "INSERT INTO magic_link_tokens (email, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(email)
    .bind(&token_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;

    let magic_link = format!(
        "{}/auth/verify?token={}",
        config.app_url, token
    );

    // Dev stub: log magic link instead of sending email
    tracing::info!(email = %email, link = %magic_link, "Magic link generated (dev mode)");

    Ok(MagicLinkResponse {
        message: "If an account exists, a magic link has been sent.".to_string(),
    })
}

pub async fn verify_magic_link(
    pool: &PgPool,
    config: &Config,
    req: &VerifyTokenRequest,
) -> anyhow::Result<AuthResponse> {
    let token_hash = hash_token(&req.token);

    let row: Option<(String,)> = sqlx::query_as(
        "SELECT email FROM magic_link_tokens
         WHERE token_hash = $1 AND expires_at > NOW() AND used_at IS NULL",
    )
    .bind(&token_hash)
    .fetch_optional(pool)
    .await?;

    let email = row
        .map(|(e,)| e)
        .ok_or_else(|| anyhow::anyhow!("Invalid or expired token"))?;

    sqlx::query("UPDATE magic_link_tokens SET used_at = NOW() WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(pool)
        .await?;

    let user = get_or_create_user(pool, &email).await?;
    let access_token = create_session_token(config, &user)?;
    let expires_at = Utc::now() + chrono::Duration::from_std(config.jwt_ttl())?;

    Ok(AuthResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_at,
        user: user.into(),
    })
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> anyhow::Result<Option<User>> {
    let row = sqlx::query(
        "SELECT id, email, display_name, credits, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.get("id"),
        email: r.get("email"),
        display_name: r.get("display_name"),
        credits: db::decimal_f64(&r, "credits"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}

pub async fn get_or_create_user(pool: &PgPool, email: &str) -> anyhow::Result<User> {
    if let Some(row) = sqlx::query(
        "SELECT id, email, display_name, credits, created_at, updated_at FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?
    {
        return Ok(User {
            id: row.get("id"),
            email: row.get("email"),
            display_name: row.get("display_name"),
            credits: db::decimal_f64(&row, "credits"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    let row = sqlx::query(
        "INSERT INTO users (email, credits) VALUES ($1, $2)
         RETURNING id, email, display_name, credits, created_at, updated_at",
    )
    .bind(email)
    .bind(5.0_f64)
    .fetch_one(pool)
    .await?;

    let user = User {
        id: row.get("id"),
        email: row.get("email"),
        display_name: row.get("display_name"),
        credits: db::decimal_f64(&row, "credits"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };

    sqlx::query(
        "INSERT INTO credit_ledger (user_id, delta, reason) VALUES ($1, $2, 'signup_bonus')",
    )
    .bind(user.id)
    .bind(5.0_f64)
    .execute(pool)
    .await?;

    Ok(user)
}

pub fn create_session_token(config: &Config, user: &User) -> anyhow::Result<String> {
    let now = Utc::now();
    let claims = SessionClaims {
        sub: user.id,
        email: user.email.clone(),
        exp: (now + chrono::Duration::from_std(config.jwt_ttl())?).timestamp(),
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_session_token(config: &Config, token: &str) -> anyhow::Result<SessionClaims> {
    let token_data = decode::<SessionClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}
