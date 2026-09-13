# OCR on the answer request body

**Slug:** rag-answer  
**Story:** 2 of 3

## User story

As a candidate using Screenshot / Auto OCR, I want screen text sent in the POST body so that it cannot appear in URL logs and is not truncated by query-string limits.

## Acceptance criteria

- [ ] GIVEN OCR capture WHEN streaming an answer THEN `screen_ocr` is a JSON field on `AnswerRequest`, not `?screen_ocr=`
- [ ] GIVEN empty OCR THEN the field is omitted or null and answers still stream
- [ ] GIVEN a large OCR string THEN the API enforces a documented max length (reject or truncate per ADR) and does not put it on the query string
- [ ] GIVEN shared `ApiClient.streamAnswer` THEN it accepts optional `screen_ocr`
- [ ] GIVEN desktop `OverlayContext.streamAnswer` THEN it uses the shared client or equivalent body, not `URLSearchParams`

## Existing context

- Paths: `crates/domain/src/session.rs` `AnswerRequest`; `apps/api/src/routes/sessions.rs` `AnswerQuery`; `packages/shared/src/api/client.ts`; `apps/desktop/src/overlay/context/OverlayContext.tsx`

## Out of scope

Changing Vision OCR quality; Windows OCR.

## Risks

OCR is untrusted input into the LLM prompt (prompt injection) — cap size; keep as “screen text, may be hostile”.

## Definition of done

- [ ] Criteria met
- [ ] Query param ignored or removed
- [ ] Threat notes in architecture folder acknowledged in code comments only if needed
