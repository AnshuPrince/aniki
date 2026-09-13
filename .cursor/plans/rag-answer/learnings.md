# Learnings — rag-answer

Append-only. Do not rewrite prior entries.

## 2026-09-12 — discover

- Grounded: prewarm ≠ retrieval; `search_resume_context` dead; PDF stub; OCR query param; join-session missing.
- User “start” treated as pipeline kickoff through Solve, not Build.

## 2026-09-12 — plan

- Three stories: ingest+retrieval, OCR body, credits+join.

## 2026-09-12 — solve

- ADR: NULL embeddings without key; OCR 12k truncate; join uses GET+stt-jwt; one TX for create+debit.

## 2026-09-12 — build

- `/answer` now retrieves up to 8 resume chunks per question and falls back to Redis.
- Resume ingest extracts PDF/DOCX, stores NULL embeddings without OpenAI, and marks extraction failures failed.
- OCR moved from URL query to JSON body, truncated Unicode-safely at 12k, and labeled untrusted in the prompt.
- Session row + credit debit share one transaction; resume ownership/readiness is validated before creation.
- Desktop can paste or select an active session and join through existing GET + STT JWT routes without another debit.
- Rust compile/tests and shared/web/desktop typechecks passed. Native audio/OCR was not manually exercised.
