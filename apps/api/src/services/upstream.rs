/// Builds an error that carries the upstream response body. `error_for_status()` keeps only the
/// status line, and OpenAI and Anthropic explain a refusal (missing scope, model access, region)
/// in the body alone.
pub async fn error(label: &str, response: reqwest::Response) -> anyhow::Error {
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    anyhow::anyhow!("{label} failed: {status}: {}", truncate(&body, 500))
}

fn truncate(body: &str, max: usize) -> String {
    let body = body.trim();
    match body.char_indices().nth(max) {
        Some((cut, _)) => format!("{}…", &body[..cut]),
        None => body.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_keeps_short_bodies_whole() {
        assert_eq!(
            truncate("  insufficient permissions  ", 500),
            "insufficient permissions"
        );
    }

    #[test]
    fn truncate_cuts_on_char_boundary() {
        let body = "é".repeat(10);
        assert_eq!(truncate(&body, 4), "éééé…");
    }
}
