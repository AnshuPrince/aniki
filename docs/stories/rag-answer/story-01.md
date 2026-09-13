# Per-question RAG and honest resume ingest

**Slug:** rag-answer  
**Story:** 1 of 3

## User story

As a candidate, I want answers grounded in the relevant parts of my resume so that coaching matches skills that are not in the first page of the file.

## Acceptance criteria

- [ ] GIVEN a resume with embeddings WHEN I POST `/sessions/{id}/answer` with a question THEN the LLM context includes chunks from `search_resume_context` (question ± OCR), not only Redis’s first eight by index
- [ ] GIVEN extra_context on the session THEN it is still included in the prompt
- [ ] GIVEN no `OPENAI_API_KEY` WHEN a resume is processed THEN no `[0.0; 1536]` rows are inserted; chunks use `NULL` embeddings and ordered fallback retrieval
- [ ] GIVEN a binary PDF with no extractable text THEN resume `status` is `failed` and the web/desktop picker does not treat it as ready
- [ ] GIVEN a `.txt`/`.md` resume THEN extract + chunk still works
- [ ] GIVEN embeddings unavailable at answer time THEN fall back to ordered chunks / Redis prewarm, do not crash
- [ ] GIVEN a missing, failed, or another user's resume ID THEN session creation rejects it before charging credits

## Existing context

- Paths: `apps/api/src/services/rag.rs`, `resume_processor.rs`, `routes/sessions.rs` `answer_question`, web/desktop resume lists
- Pattern to follow: existing `search_resume_context` + SSE `llm::stream_answer`

## Out of scope

R2 uploads; Windows OCR; Stripe.

## Risks

Existing zero-vector chunks in local DBs — re-upload after this ships.

## Definition of done

- [ ] Criteria met
- [ ] Tests or manual check noted
- [ ] Docs updated if user-facing
