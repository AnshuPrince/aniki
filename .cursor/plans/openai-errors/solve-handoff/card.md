---
status: open
from_zone: solve
to_zone: build
slug: openai-errors
created: 2026-09-14
---

# Handoff: solve → build

## Artifacts
- none; raised from an Operate incident, not a Solve document

## Touches (Build only)
- apps/api/src/services/upstream.rs (new)
- apps/api/src/services/mod.rs
- apps/api/src/services/embeddings.rs
- apps/api/src/services/llm.rs
- apps/api/src/services/notes.rs

## Notes
Prod incident: resume `f93b7d6a` failed with `403 Forbidden` from `https://api.openai.com/v1/embeddings`.
`error_for_status()` drops the response body, so the Fly log carries the status line and nothing else —
OpenAI states the reason (scope, model access, region) only in the body.

Match the existing status-check shape in `services/auth.rs::finish_google_oauth`: read `status()`,
then `text()` on failure. Truncate the body; it is an API error message, not user data, and carries
no credential. Keep `embed_text`'s signature and the success path unchanged.

Covers every upstream LLM call, not just embeddings: `confirm_question`, both streaming paths in
`services/llm.rs`, and `generate_via_llm` in `services/notes.rs` swallowed bodies the same way.
`upstream::error` is the single place that reads the body.

## Gate
Human asked for this directly during the incident (Operate → Build).
