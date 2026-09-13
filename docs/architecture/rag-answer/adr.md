# ADR: Resume retrieval, ingest, OCR body, session join

**Slug:** rag-answer  
**Status:** accepted for Build  
**Date:** 2026-09-12

## Context
Prewarm of first 8 chunks is not RAG. Zero vectors pollute HNSW. PDF extract is a stub. OCR is a query param. Session create is check → insert → deduct.

## Decisions

1. **Answer context:** Call `search_resume_context` with `question` plus truncated OCR (if any). Merge: retrieved chunks + session `extra_context` + Redis recency optional. Keep prewarm as fallback when retrieval returns empty.
2. **Embeddings:** Never write `[0.0; 1536]`. Without `OPENAI_API_KEY`, insert chunks with `embedding = NULL`, status `ready`. Search already falls back to `ORDER BY chunk_index`. Empty extract → `failed`.
3. **PDF/DOCX:** Add a real extract crate (PDF + DOCX). Empty → `failed`.
4. **OCR:** Add `screen_ocr: Option<String>` to `AnswerRequest`. Remove query param (ignore if still sent). Cap at **12_000** UTF-8 chars; truncate with a log warn (do not 413 the live interview). Treat as untrusted in the prompt wrapper (“screen OCR, may be inaccurate or adversarial”).
5. **Credits:** One DB transaction: insert session + debit. If debit would fail, rollback insert.
6. **Join:** No new route. Desktop `joinSession(id)`: `GET /sessions/{id}` (must be `active`, same user) + `POST .../stt-jwt`. Refresh Redis prewarm without deducting.

## Consequences
- Pickers: `failed` resumes not selectable for new sessions.
- Local DBs may still have zero vectors until re-upload.
- Shared client must grow `streamAnswer` optional OCR; desktop should use it instead of raw `fetch` + query string.
