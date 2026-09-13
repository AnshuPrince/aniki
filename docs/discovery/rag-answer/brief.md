# Discovery brief — rag-answer

## Problem
Live coaching answers look connected to the candidate’s resume but are not. Session create caches the first eight resume chunks (by index) in Redis; `POST /sessions/{id}/answer` only reads that cache. `search_resume_context` exists and is never called. Binary PDFs extract as empty text. Missing `OPENAI_API_KEY` writes zero vectors into pgvector. Screen OCR is stuffed into the answer URL query string. The overlay always creates a new session, so a web-copied session ID cannot be joined.

## Evidence
- Code / docs (paths):
  - `apps/api/src/services/rag.rs` — `process_resume` zero-vector fallback; `search_resume_context` unused; `prewarm_session_context` `LIMIT 8` by `chunk_index`
  - `apps/api/src/routes/sessions.rs` — create: credit check then insert then `deduct_credits`; answer: `get_session_context` + `Query(AnswerQuery.screen_ocr)`
  - `apps/api/src/services/resume_processor.rs` — `%PDF` → empty string + warn
  - `apps/desktop/src/overlay/context/OverlayContext.tsx` — `url.searchParams.set("screen_ocr", ocrContext)`; `startSession` always `createSession`
  - `packages/shared/src/api/client.ts` — `streamAnswer` body is `{ question, transcript_context }` only
  - `crates/domain/src/session.rs` — `AnswerRequest` has no `screen_ocr`
  - User / plan: remaining work Phase A; “answers must use the resume”
- User statements (verbatim):
  - “plan to build the remaining pieces”
  - “start” (this pipeline: slug `rag-answer`)

## Users and context
Candidate using the overlay; operator using the web dashboard. Note platform (macOS/Windows). OCR capture is macOS-only today; join-session is both.

## In scope
- Per-question resume retrieval on `/answer` (plus extra_context / recent transcript)
- Fail-closed resume ingest without embeddings; real PDF/DOCX text extract
- OCR on the answer JSON body, not the query string
- Atomic session create + credit deduct
- Desktop attach to an existing session ID (GET + STT JWT refresh)

## Out of scope
- Stripe, R2, Resend, keychain, speaker-swap, Interview Post URL, Windows OCR, Fly deploy

## Examples (example mapping)
- Rule: If OpenAI embeddings are configured, `/answer` retrieves by similarity to the question (and OCR text if present), not the first eight chunks only.
- Example: Resume chunk 20 mentions “Kubernetes”; question “Tell me about K8s” retrieves that chunk.
- Rule: If `OPENAI_API_KEY` is unset, do not insert `[0.0; 1536]`; resume is not `ready` as if embedded.
- Example: Dev upload of a `.txt` without a key → status `failed` or `ready_unembedded` and picker shows it cannot do semantic RAG.
- Rule: Binary PDF with no extractable text is `failed`, visible in web/desktop resume lists.
- Rule: OCR text never appears in access logs as a query parameter.
- Rule: Overlay can attach to `GET /sessions/{id}` for the same user without charging a second session.
- Question: Cap OCR body size (e.g. 8–16k chars) vs reject?

## Success
A session with a real resume returns answers that cite resume content beyond the first eight chunks; a bad PDF shows failed; attaching to a web session ID starts live audio without a second credit debit.

## Open questions
- Status name for unembedded-but-text-ready resumes (`failed` vs `ready_unembedded`) — Solve will pick.
- Retrieval `limit` (suggest 6–8 chunks) and whether Redis prewarm remains a fallback only.
