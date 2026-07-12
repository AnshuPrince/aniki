use std::time::Duration;

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
                .unwrap_or_else(|_| "gpt-4.1".to_string()),
            anthropic_chat_model: std::env::var("ANTHROPIC_CHAT_MODEL")
                .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string()),
            confirm_model: std::env::var("CONFIRM_MODEL")
                .unwrap_or_else(|_| "gpt-4.1-mini".to_string()),
        })
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
