use pgvector::Vector;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::config::Config;
use crate::redis::{cache_get, cache_set, RedisPool};
use crate::services::{embeddings, resume_processor};

pub async fn process_resume(
    pool: &PgPool,
    config: &Config,
    resume_id: Uuid,
    filename: &str,
    content: &[u8],
) -> anyhow::Result<()> {
    sqlx::query("UPDATE resumes SET status = 'processing', updated_at = NOW() WHERE id = $1")
        .bind(resume_id)
        .execute(pool)
        .await?;

    if let Err(error) = process_resume_inner(pool, config, resume_id, filename, content).await {
        sqlx::query(
            "UPDATE resumes SET status = 'failed', chunk_count = 0, updated_at = NOW() WHERE id = $1",
        )
        .bind(resume_id)
        .execute(pool)
        .await?;
        return Err(error);
    }

    Ok(())
}

async fn process_resume_inner(
    pool: &PgPool,
    config: &Config,
    resume_id: Uuid,
    filename: &str,
    content: &[u8],
) -> anyhow::Result<()> {
    let text = resume_processor::extract_text_from_bytes(filename, content)?;
    let chunks = resume_processor::chunk_text(&text);

    if chunks.is_empty() {
        anyhow::bail!("resume contains no extractable text");
    }

    sqlx::query("DELETE FROM resume_chunks WHERE resume_id = $1")
        .bind(resume_id)
        .execute(pool)
        .await?;

    for (index, chunk) in chunks.iter().enumerate() {
        let embedding = match &config.openai_api_key {
            Some(_) => Some(Vector::from(embeddings::embed_text(config, chunk).await?)),
            None => None,
        };

        sqlx::query(
            "INSERT INTO resume_chunks (resume_id, chunk_index, content, embedding)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(resume_id)
        .bind(index as i32)
        .bind(chunk)
        .bind(embedding)
        .execute(pool)
        .await?;
    }

    sqlx::query(
        "UPDATE resumes SET status = 'ready', chunk_count = $2, updated_at = NOW() WHERE id = $1",
    )
    .bind(resume_id)
    .bind(chunks.len() as i32)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn search_resume_context(
    pool: &PgPool,
    config: &Config,
    resume_id: Uuid,
    query: &str,
    limit: i64,
) -> anyhow::Result<Vec<String>> {
    if config.openai_api_key.is_none() {
        let rows = sqlx::query(
            "SELECT content FROM resume_chunks WHERE resume_id = $1 ORDER BY chunk_index LIMIT $2",
        )
        .bind(resume_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;
        return Ok(rows.iter().map(|r| r.get("content")).collect());
    }

    let query_embedding = embeddings::embed_text(config, query).await?;
    let rows = sqlx::query(
        "SELECT content, embedding <=> $1 AS distance
         FROM resume_chunks
         WHERE resume_id = $2 AND embedding IS NOT NULL
         ORDER BY distance
         LIMIT $3",
    )
    .bind(Vector::from(query_embedding))
    .bind(resume_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(|r| r.get("content")).collect())
}

pub async fn prewarm_session_context(
    pool: &PgPool,
    redis: &mut RedisPool,
    session_id: Uuid,
    resume_id: Option<Uuid>,
    extra_context: Option<&str>,
) -> anyhow::Result<()> {
    let mut chunks: Vec<String> = Vec::new();

    if let Some(resume_id) = resume_id {
        let resume_chunks = sqlx::query(
            "SELECT content FROM resume_chunks WHERE resume_id = $1 ORDER BY chunk_index LIMIT 8",
        )
        .bind(resume_id)
        .fetch_all(pool)
        .await?;
        chunks.extend(resume_chunks.iter().map(|r| r.get::<String, _>("content")));
    }

    if let Some(extra) = extra_context.filter(|s| !s.is_empty()) {
        chunks.push(extra.to_string());
    }

    let context_json = serde_json::to_string(&chunks)?;
    let key = format!("session:{session_id}:context");
    cache_set(redis, &key, &context_json, 7200).await?;
    Ok(())
}

pub async fn get_session_context(
    redis: &mut RedisPool,
    session_id: Uuid,
) -> anyhow::Result<Vec<String>> {
    let key = format!("session:{session_id}:context");
    if let Some(raw) = cache_get(redis, &key).await? {
        if let Ok(chunks) = serde_json::from_str::<Vec<String>>(&raw) {
            return Ok(chunks);
        }
    }
    Ok(vec![])
}
