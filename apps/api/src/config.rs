use std::collections::BTreeSet;
use std::time::Duration;

use axum::http::HeaderValue;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub speechmatics_api_key: Option<String>,
    pub speechmatics_region: String,
    pub app_url: String,
    pub api_url: String,
    pub port: u16,
    pub session_credit_cost: f64,
    pub free_trial_credits: f64,
    pub openai_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub embedding_model: String,
    pub openai_chat_model: String,
    pub anthropic_chat_model: String,
    pub confirm_model: String,
    /// Extra browser origins, comma-separated. Always includes APP_URL and desktop locals.
    pub cors_origins_extra: Option<String>,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://aniki:aniki@localhost:5432/aniki".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-jwt-secret-change-in-production".to_string()),
            speechmatics_api_key: std::env::var("SPEECHMATICS_API_KEY").ok().filter(|s| !s.is_empty()),
            speechmatics_region: std::env::var("SPEECHMATICS_REGION")
                .unwrap_or_else(|_| "eu".to_string()),
            app_url: std::env::var("APP_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            api_url: std::env::var("API_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            session_credit_cost: 0.5,
            free_trial_credits: 5.0,
            openai_api_key: std::env::var("OPENAI_API_KEY").ok().filter(|s| !s.is_empty()),
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").ok().filter(|s| !s.is_empty()),
            embedding_model: std::env::var("EMBEDDING_MODEL")
                .unwrap_or_else(|_| "text-embedding-3-small".to_string()),
            openai_chat_model: std::env::var("OPENAI_CHAT_MODEL")
                .unwrap_or_else(|_| "gpt-5.6-luna".to_string()),
            anthropic_chat_model: std::env::var("ANTHROPIC_CHAT_MODEL")
                .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string()),
            confirm_model: std::env::var("CONFIRM_MODEL")
                .unwrap_or_else(|_| "gpt-5.6-luna".to_string()),
            cors_origins_extra: std::env::var("CORS_ORIGINS").ok().filter(|s| !s.is_empty()),
            google_client_id: std::env::var("GOOGLE_CLIENT_ID").ok().filter(|s| !s.is_empty()),
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")
                .ok()
                .filter(|s| !s.is_empty()),
        })
    }

    pub fn google_oauth_configured(&self) -> bool {
        self.google_client_id.is_some() && self.google_client_secret.is_some()
    }

    pub fn oauth_redirect_uri(&self) -> String {
        format!("{}/auth/oauth/callback", self.app_url.trim_end_matches('/'))
    }

    pub fn cors_origin_headers(&self) -> anyhow::Result<Vec<HeaderValue>> {
        parse_cors_origins(&self.app_url, self.cors_origins_extra.as_deref())
    }

    pub fn speechmatics_endpoint(&self) -> String {
        format!("wss://{}.rt.speechmatics.com/v2", self.speechmatics_region)
    }

    pub fn jwt_ttl(&self) -> Duration {
        Duration::from_secs(60 * 60 * 24 * 7) // 7 days for session tokens
    }

    pub fn stt_jwt_ttl(&self) -> Duration {
        Duration::from_secs(60) // 60s Speechmatics JWT
    }
}

fn push_origin(seen: &mut BTreeSet<String>, out: &mut Vec<HeaderValue>, raw: &str) -> anyhow::Result<()> {
    let origin = raw.trim().trim_end_matches('/').to_string();
    if origin.is_empty() || !seen.insert(origin.clone()) {
        return Ok(());
    }
    let header = HeaderValue::from_str(&origin)
        .map_err(|e| anyhow::anyhow!("invalid CORS origin {origin:?}: {e}"))?;
    out.push(header);
    Ok(())
}

pub(crate) fn parse_cors_origins(app_url: &str, extra: Option<&str>) -> anyhow::Result<Vec<HeaderValue>> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    push_origin(&mut seen, &mut out, app_url)?;
    for origin in [
        "http://localhost:3000",
        "http://localhost:1420",
        "tauri://localhost",
    ] {
        push_origin(&mut seen, &mut out, origin)?;
    }
    if let Some(extra) = extra {
        for part in extra.split(',') {
            push_origin(&mut seen, &mut out, part)?;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn origin_set(headers: &[HeaderValue]) -> BTreeSet<String> {
        headers
            .iter()
            .map(|h| h.to_str().unwrap().to_string())
            .collect()
    }

    #[test]
    fn cors_includes_app_url_and_desktop() {
        let headers = parse_cors_origins("https://aniki.pages.dev", None).unwrap();
        let set = origin_set(&headers);
        assert!(set.contains("https://aniki.pages.dev"));
        assert!(set.contains("http://localhost:3000"));
        assert!(set.contains("http://localhost:1420"));
        assert!(set.contains("tauri://localhost"));
    }

    #[test]
    fn cors_extra_origins_dedupe_and_strip_slash() {
        let headers = parse_cors_origins(
            "https://aniki.pages.dev/",
            Some("https://aniki.example, https://aniki.pages.dev/"),
        )
        .unwrap();
        let set = origin_set(&headers);
        assert!(set.contains("https://aniki.example"));
        assert_eq!(set.iter().filter(|o| *o == "https://aniki.pages.dev").count(), 1);
    }

    #[test]
    fn cors_rejects_invalid_origin() {
        let err = parse_cors_origins("https://ok.example", Some("not a header\nvalue")).unwrap_err();
        assert!(err.to_string().contains("invalid CORS origin"));
    }
}
