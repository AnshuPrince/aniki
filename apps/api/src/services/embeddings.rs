use crate::config::Config;
use crate::services::upstream;

pub async fn embed_text(config: &Config, text: &str) -> anyhow::Result<Vec<f32>> {
    let api_key = config
        .openai_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("OPENAI_API_KEY not configured"))?;

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/embeddings")
        .bearer_auth(api_key)
        .json(&serde_json::json!({
            "model": config.embedding_model,
            "input": text,
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(upstream::error("openai embeddings", response).await);
    }

    let response = response.json::<serde_json::Value>().await?;

    let embedding = response["data"][0]["embedding"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("invalid embedding response"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect();

    Ok(embedding)
}
