use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use aniki_domain::{SpeechmaticsTranscriptionConfig, SttJwtResponse};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct SpeechmaticsJwtClaims {
    sub: String,
    exp: i64,
    iat: i64,
    #[serde(rename = "transcription_config")]
    transcription_config: serde_json::Value,
}

/// Mint a short-lived JWT for direct desktop → Speechmatics WSS connection.
pub fn mint_stt_jwt(
    config: &Config,
    session_id: Uuid,
    transcription_config: Option<SpeechmaticsTranscriptionConfig>,
) -> anyhow::Result<SttJwtResponse> {
    let api_key = config
        .speechmatics_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("SPEECHMATICS_API_KEY not configured"))?;

    let transcription_config = transcription_config
        .unwrap_or_default()
        .to_json();

    let now = Utc::now();
    let ttl = config.stt_jwt_ttl();
    let expires_at = now + Duration::from_std(ttl)?;

    let claims = SpeechmaticsJwtClaims {
        sub: session_id.to_string(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
        transcription_config: transcription_config.clone(),
    };

    // Speechmatics JWT uses the API key as the signing secret
    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(api_key.as_bytes()),
    )?;

    Ok(SttJwtResponse {
        jwt,
        endpoint: config.speechmatics_endpoint(),
        expires_at,
        transcription_config,
    })
}

/// Dev-mode stub JWT when Speechmatics key is not configured.
pub fn mint_dev_stt_jwt(config: &Config, session_id: Uuid) -> SttJwtResponse {
    let transcription_config = SpeechmaticsTranscriptionConfig::default().to_json();
    let expires_at = Utc::now() + Duration::seconds(60);

    SttJwtResponse {
        jwt: format!("dev-stt-jwt-{}", session_id),
        endpoint: config.speechmatics_endpoint(),
        expires_at,
        transcription_config,
    }
}
