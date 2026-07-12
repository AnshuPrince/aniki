use crate::config::Config;

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
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?;

    let embedding = response["data"][0]["embedding"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("invalid embedding response"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect();

    Ok(embedding)
}

pub async fn embed_texts(config: &Config, texts: &[String]) -> anyhow::Result<Vec<Vec<f32>>> {
    let mut out = Vec::with_capacity(texts.len());
    for text in texts {
        out.push(embed_text(config, text).await?);
    }
    Ok(out)
}
