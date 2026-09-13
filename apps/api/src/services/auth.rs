use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::RngCore;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use aniki_domain::{
    AuthResponse, GoogleOAuthCallbackRequest, GoogleOAuthStartResponse, MagicLinkResponse,
    SessionClaims, User, VerifyTokenRequest,
};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

use crate::config::Config;
use crate::db;
use crate::redis::{cache_del, cache_get, cache_set, RedisPool};

const OAUTH_STATE_TTL_SECS: u64 = 600;

#[derive(Debug)]
pub enum OauthError {
    Unconfigured,
    BadRequest,
    Unauthorized,
    Internal(anyhow::Error),
}

impl OauthError {
    pub fn status(&self) -> axum::http::StatusCode {
        match self {
            Self::Unconfigured => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Self::BadRequest => axum::http::StatusCode::BAD_REQUEST,
            Self::Unauthorized => axum::http::StatusCode::UNAUTHORIZED,
            Self::Internal(e) => {
                tracing::error!(error = %e, "oauth internal error");
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

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

pub async fn start_google_oauth(
    redis: &mut RedisPool,
    config: &Config,
) -> Result<GoogleOAuthStartResponse, OauthError> {
    let client_id = config
        .google_client_id
        .as_deref()
        .ok_or(OauthError::Unconfigured)?;
    if config.google_client_secret.is_none() {
        return Err(OauthError::Unconfigured);
    }

    let state = random_token();
    let verifier = random_token();
    let challenge = pkce_challenge(&verifier);
    cache_set(redis, &oauth_key(&state), &verifier, OAUTH_STATE_TTL_SECS)
        .await
        .map_err(OauthError::Internal)?;

    let redirect = config.oauth_redirect_uri();
    let authorization_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256&access_type=online",
        enc(client_id),
        enc(&redirect),
        enc("openid email profile"),
        enc(&state),
        enc(&challenge),
    );

    Ok(GoogleOAuthStartResponse { authorization_url })
}

pub async fn finish_google_oauth(
    pool: &PgPool,
    redis: &mut RedisPool,
    config: &Config,
    req: &GoogleOAuthCallbackRequest,
) -> Result<AuthResponse, OauthError> {
    let client_id = config
        .google_client_id
        .as_deref()
        .ok_or(OauthError::Unconfigured)?;
    let client_secret = config
        .google_client_secret
        .as_deref()
        .ok_or(OauthError::Unconfigured)?;

    if req.code.is_empty() || req.state.is_empty() {
        return Err(OauthError::BadRequest);
    }

    let key = oauth_key(&req.state);
    let verifier = cache_get(redis, &key)
        .await
        .map_err(OauthError::Internal)?
        .ok_or(OauthError::Unauthorized)?;
    cache_del(redis, &key)
        .await
        .map_err(OauthError::Internal)?;

    let redirect = config.oauth_redirect_uri();
    let client = reqwest::Client::new();
    let token_res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", req.code.as_str()),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("redirect_uri", redirect.as_str()),
            ("grant_type", "authorization_code"),
            ("code_verifier", verifier.as_str()),
        ])
        .send()
        .await
        .map_err(|e| OauthError::Internal(e.into()))?;

    if !token_res.status().is_success() {
        tracing::warn!(status = %token_res.status(), "google token exchange failed");
        return Err(OauthError::Unauthorized);
    }

    let token_json: serde_json::Value = token_res
        .json()
        .await
        .map_err(|e| OauthError::Internal(e.into()))?;
    let access_token = token_json["access_token"]
        .as_str()
        .ok_or(OauthError::Unauthorized)?;

    let userinfo: serde_json::Value = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| OauthError::Internal(e.into()))?
        .error_for_status()
        .map_err(|_| OauthError::Unauthorized)?
        .json()
        .await
        .map_err(|e| OauthError::Internal(e.into()))?;

    if !google_email_verified(&userinfo) {
        return Err(OauthError::Unauthorized);
    }

    let email = userinfo["email"]
        .as_str()
        .ok_or(OauthError::Unauthorized)?
        .to_string();
    let sub = userinfo["sub"]
        .as_str()
        .ok_or(OauthError::Unauthorized)?
        .to_string();
    let name = userinfo["name"].as_str().map(str::to_string);

    let suffix: String = sub.chars().rev().take(6).collect();
    tracing::info!(google_sub_suffix = %suffix, "google oauth login");

    let user = upsert_google_user(pool, &email, &sub, name.as_deref())
        .await
        .map_err(|e| {
            if e.to_string().contains("already linked") {
                OauthError::Unauthorized
            } else {
                OauthError::Internal(e)
            }
        })?;
    let access_token = create_session_token(config, &user).map_err(OauthError::Internal)?;
    let expires_at = Utc::now() + chrono::Duration::from_std(config.jwt_ttl()).expect("jwt ttl");

    Ok(AuthResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_at,
        user: user.into(),
    })
}

async fn upsert_google_user(
    pool: &PgPool,
    email: &str,
    google_sub: &str,
    display_name: Option<&str>,
) -> anyhow::Result<User> {
    if let Some(row) = sqlx::query(
        "SELECT id, email, display_name, credits, created_at, updated_at FROM users WHERE google_sub = $1",
    )
    .bind(google_sub)
    .fetch_optional(pool)
    .await?
    {
        return Ok(row_to_user(row));
    }

    if let Some(row) = sqlx::query(
        "SELECT id, email, display_name, credits, created_at, updated_at, google_sub FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?
    {
        let existing: Option<String> = row.get("google_sub");
        if existing.as_deref().is_some_and(|s| s != google_sub) {
            anyhow::bail!("email already linked to another Google account");
        }
        sqlx::query(
            "UPDATE users SET google_sub = $1, display_name = COALESCE(display_name, $2), updated_at = NOW()
             WHERE id = $3",
        )
        .bind(google_sub)
        .bind(display_name)
        .bind(row.get::<Uuid, _>("id"))
        .execute(pool)
        .await?;
        return get_user_by_id(pool, row.get("id"))
            .await?
            .ok_or_else(|| anyhow::anyhow!("user missing after google link"));
    }

    let row = sqlx::query(
        "INSERT INTO users (email, credits, google_sub, display_name) VALUES ($1, $2, $3, $4)
         RETURNING id, email, display_name, credits, created_at, updated_at",
    )
    .bind(email)
    .bind(5.0_f64)
    .bind(google_sub)
    .bind(display_name)
    .fetch_one(pool)
    .await?;

    let user = row_to_user(row);
    sqlx::query(
        "INSERT INTO credit_ledger (user_id, delta, reason) VALUES ($1, $2, 'signup_bonus')",
    )
    .bind(user.id)
    .bind(5.0_f64)
    .execute(pool)
    .await?;
    Ok(user)
}

fn row_to_user(row: sqlx::postgres::PgRow) -> User {
    User {
        id: row.get("id"),
        email: row.get("email"),
        display_name: row.get("display_name"),
        credits: db::decimal_f64(&row, "credits"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn oauth_key(state: &str) -> String {
    format!("oauth:google:{state}")
}

fn random_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn pkce_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}

fn enc(value: &str) -> String {
    value
        .bytes()
        .flat_map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                vec![char::from(b)]
            }
            _ => format!("%{b:02X}").chars().collect(),
        })
        .collect()
}

fn google_email_verified(userinfo: &serde_json::Value) -> bool {
    match &userinfo["email_verified"] {
        serde_json::Value::Bool(v) => *v,
        serde_json::Value::String(s) => s.eq_ignore_ascii_case("true"),
        _ => false,
    }
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

#[cfg(test)]
mod oauth_tests {
    use super::*;

    #[test]
    fn pkce_challenge_is_stable_base64url() {
        let challenge = pkce_challenge("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk");
        assert_eq!(challenge, "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
    }

    #[test]
    fn google_verified_accepts_bool_and_string() {
        assert!(google_email_verified(&serde_json::json!({"email_verified": true})));
        assert!(google_email_verified(&serde_json::json!({"email_verified": "true"})));
        assert!(!google_email_verified(&serde_json::json!({"email_verified": false})));
    }
}
