---
status: done
from_zone: solve
to_zone: build
slug: rag-answer
created: 2026-09-12
---

# Handoff: solve → build

## Artifacts
- docs/architecture/rag-answer/README.md
- docs/architecture/rag-answer/adr.md
- docs/architecture/rag-answer/api.md
- docs/architecture/rag-answer/threat-models/ocr-llm.md
- docs/stories/rag-answer/story-01.md
- docs/stories/rag-answer/story-02.md
- docs/stories/rag-answer/story-03.md

## Touches (Build only)
- apps/api/src/services/rag.rs
- apps/api/src/services/resume_processor.rs
- apps/api/src/routes/sessions.rs
- apps/api/src/services/llm.rs
- apps/api/Cargo.toml
- crates/domain/src/session.rs
- packages/shared/src/types.ts
- packages/shared/src/api/client.ts
- apps/desktop/src/overlay/context/OverlayContext.tsx
- apps/desktop/src/overlay/screens/CreateTab.tsx
- apps/desktop/src/overlay/screens/SessionsTab.tsx
- apps/web/src/pages/ResumesPage.tsx

## Notes for the next zone
Follow ADR. Do not invent R2/Stripe. Truncate OCR at 12k. NULL embeddings when no OpenAI key.

## Gate
Approved by the user with “continue to build”; Build completed 2026-09-12.
