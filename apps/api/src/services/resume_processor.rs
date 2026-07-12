const CHUNK_SIZE: usize = 1500;
const CHUNK_OVERLAP: usize = 200;

pub fn chunk_text(text: &str) -> Vec<String> {
    let cleaned: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if cleaned.is_empty() {
        return vec![];
    }
    if cleaned.len() <= CHUNK_SIZE {
        return vec![cleaned];
    }

    let mut chunks = Vec::new();
    let mut start = 0;
    while start < cleaned.len() {
        let end = (start + CHUNK_SIZE).min(cleaned.len());
        let mut chunk_end = end;
        if end < cleaned.len() {
            if let Some(pos) = cleaned[start..end].rfind(' ') {
                chunk_end = start + pos;
            }
        }
        chunks.push(cleaned[start..chunk_end].trim().to_string());
        if chunk_end >= cleaned.len() {
            break;
        }
        start = chunk_end.saturating_sub(CHUNK_OVERLAP);
    }
    chunks.retain(|c| !c.is_empty());
    chunks
}

pub fn extract_text_from_bytes(filename: &str, bytes: &[u8]) -> anyhow::Result<String> {
    let lower = filename.to_lowercase();
    if lower.ends_with(".txt") || lower.ends_with(".md") {
        return Ok(String::from_utf8_lossy(bytes).to_string());
    }
    if lower.ends_with(".pdf") {
        // Lightweight PDF text extraction via pdf-extract would add dep;
        // accept UTF-8 fallback for dev uploads of text exports.
        let text = String::from_utf8_lossy(bytes).to_string();
        if text.contains("%PDF") {
            tracing::warn!("PDF binary detected; store plain-text resume for best RAG quality");
            return Ok(String::new());
        }
        return Ok(text);
    }
    Ok(String::from_utf8_lossy(bytes).to_string())
}
