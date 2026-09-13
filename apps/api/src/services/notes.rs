use sqlx::{PgPool, Row};
use uuid::Uuid;

use aniki_domain::SessionNotes;

use crate::config::Config;
use crate::services::upstream;

pub async fn generate_session_notes(
    pool: &PgPool,
    config: &Config,
    session_id: Uuid,
) -> anyhow::Result<SessionNotes> {
    let row = sqlx::query(
        "SELECT transcript, extra_context FROM sessions WHERE id = $1",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;

    let transcript: Option<String> = row.get("transcript");
    let extra: Option<String> = row.get("extra_context");

    let transcript = transcript.unwrap_or_default();
    let notes = if config.openai_api_key.is_some() || config.anthropic_api_key.is_some() {
        generate_via_llm(config, session_id, &transcript, extra.as_deref()).await?
    } else {
        SessionNotes {
            session_id,
            summary: "Session completed. Configure LLM API keys for AI-generated notes.".to_string(),
            key_points: vec![],
            action_items: vec![],
            generated_at: chrono::Utc::now(),
        }
    };

    sqlx::query("UPDATE sessions SET notes = $2 WHERE id = $1")
        .bind(session_id)
        .bind(serde_json::to_value(&notes)?)
        .execute(pool)
        .await?;

    Ok(notes)
}

async fn generate_via_llm(
    config: &Config,
    session_id: Uuid,
    transcript: &str,
    extra_context: Option<&str>,
) -> anyhow::Result<SessionNotes> {
    let prompt = format!(
        "Summarize this interview session transcript as JSON with keys: summary (string), \
         key_points (string array), action_items (string array).\n\n\
         Extra context: {}\n\nTranscript:\n{}",
        extra_context.unwrap_or(""),
        transcript
    );

    if let Some(key) = &config.openai_api_key {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(key)
            .json(&serde_json::json!({
                "model": config.confirm_model,
                "messages": [{"role": "user", "content": prompt}],
                "response_format": {"type": "json_object"},
                "max_completion_tokens": 1200,
                "reasoning_effort": "low",
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(upstream::error("openai chat", response).await);
        }

        let response = response.json::<serde_json::Value>().await?;

        let content = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("{}");
        let parsed: serde_json::Value = serde_json::from_str(content)?;
        return Ok(SessionNotes {
            session_id,
            summary: parsed["summary"].as_str().unwrap_or("").to_string(),
            key_points: parsed["key_points"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            action_items: parsed["action_items"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            generated_at: chrono::Utc::now(),
        });
    }

  Ok(SessionNotes {
        session_id,
        summary: "Notes generation requires OPENAI_API_KEY.".to_string(),
        key_points: vec![],
        action_items: vec![],
        generated_at: chrono::Utc::now(),
    })
}
