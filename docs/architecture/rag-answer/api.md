# API sketch — rag-answer

## `POST /sessions/{id}/answer`

Body (JSON):

```json
{
  "question": "string",
  "transcript_context": "string | null",
  "screen_ocr": "string | null"
}
```

- Auth: Bearer, session `active` and owned.
- Response: unchanged SSE (`data` tokens, `event: done`).
- Server: retrieve resume chunks; pass OCR into LLM user prompt as today, from body not query.
- Do not document `?screen_ocr=`. Drop `AnswerQuery` or ignore it.

## `POST /sessions` (create)

Unchanged JSON. Implementation: single transaction for row + credit ledger. Response unchanged (`session`, `stt_jwt`, …).

## `GET /sessions/{id}` / `POST /sessions/{id}/stt-jwt`

Unchanged. Join uses these only. `404` if not owned; overlay must not start audio.

## Shared TS

`AnswerRequest.screen_ocr?: string | null`  
`streamAnswer(sessionId, question, transcriptContext?, screenOcr?)`
