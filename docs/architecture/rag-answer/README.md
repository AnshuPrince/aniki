# Architecture — rag-answer

Approved for Build after human continue.

| Artifact | Path |
|----------|------|
| ADR | [adr.md](adr.md) |
| API | [api.md](api.md) |
| Threat | [threat-models/ocr-llm.md](threat-models/ocr-llm.md) |
| Stories | [docs/stories/rag-answer/](../../stories/rag-answer/) |

## Touches

- `apps/api/src/services/rag.rs`
- `apps/api/src/services/resume_processor.rs`
- `apps/api/src/routes/sessions.rs`
- `apps/api/src/services/llm.rs` (prompt wrapper only if needed)
- `apps/api/Cargo.toml` (PDF/DOCX extract crate)
- `crates/domain/src/session.rs`
- `packages/shared/src/types.ts`
- `packages/shared/src/api/client.ts`
- `apps/desktop/src/overlay/context/OverlayContext.tsx`
- `apps/desktop/src/overlay/screens/CreateTab.tsx` and/or `SessionsTab.tsx` (join ID)
- `apps/web/src/pages/ResumesPage.tsx` (failed not treated as ready) if the picker allows failed today
