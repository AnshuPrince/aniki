# Threat notes — rag-answer (OCR + retrieval)

**Mode:** story (not FSA). Source: `docs/stories/rag-answer/story-02.md`.

## Data flow

Overlay OCR (macOS Vision) → HTTPS POST body `screen_ocr` → Axum → concatenated into LLM user prompt with resume chunks and transcript → provider (OpenAI/Anthropic) → SSE tokens back to overlay.

## Classification

Resume and OCR may include PII, employer secrets, interview questions. LLM providers are a third-party trust boundary.

## STRIDE (this slice)

| Threat | Mitigation for Build |
|--------|----------------------|
| Spoofing | Existing JWT; session must be active and owned |
| Tampering | TLS; do not accept OCR from query string (logged/cached more often than bodies) |
| Repudiation | Existing request logs: log **length** of OCR, not the text |
| Information disclosure | Cap OCR 12k chars; never put OCR in URLs; do not persist OCR on the session row in this slice |
| Denial of service | Cap size; retrieval `LIMIT` small (8) |
| Elevation / injection | Prompt-wrap OCR as untrusted screen text; session creation must verify `resume_id` is `ready` and owned by the authenticated user |

## Out of scope here

Provider DPA, prompt-injection classifiers, Windows OCR.
