# Resume-backed live answers — Enhancement

**Slug:** rag-answer  
**Epic Goal:** Live `/answer` uses the candidate resume (retrieval + ingest that can fail honestly), OCR is not leaked on the URL, credits and join-session behave correctly.

## Existing system context
Axum sessions + Redis prewarm + SSE LLM; desktop overlay creates sessions and posts OCR as `?screen_ocr=`.

## Enhancement details
Wire existing `search_resume_context`; fix PDF/embeddings; move OCR to body; transactional create; desktop join by ID.

## Stories
1. Ingest + per-question retrieval
2. OCR on answer body
3. Atomic credits + join existing session

## Compatibility
- Keep SSE answer stream and session list/get shapes except additive `screen_ocr` on answer JSON.
- Do not charge twice when joining an active session.

## Risks
Zero-vector rows already in DB — Build should not rely on them; optional note to re-upload resumes.

## Definition of done
- [ ] All three stories accepted
- [ ] Shared client + desktop use the new answer body
- [ ] Manual or unit check for retrieval + failed PDF
