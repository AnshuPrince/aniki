---
name: aniki-solve
description: Writes slice architecture for Aniki (ADR, API sketch, optional STRIDE). Use for solve, ADR, API design, threat model; after approved stories. Not a G-P FSA pipeline.
---

# aniki-solve

Confirm `slug`. Read stories under `docs/stories/<slug>/`. Read [../aniki-shared.md](../aniki-shared.md).

## Do

1. Decide depth: copy/docs → skip (user must have allowed). API or data change → ADR + API sketch. Auth/billing/OCR-in-LLM → also threat model.
2. Optional Frodo (isolated): `write-adr`, `write-rfc`, `api-design-workshop`, `threat-model`. Output **only** `docs/architecture/<slug>/`. Optional `@threat-modeller` / `@solutions-architect` with prompt: Aniki paths, **no FSA**.
3. Write `docs/architecture/<slug>/README.md` listing artifacts and files Build may touch.
4. Close plan handoff. Open `solve-handoff/` → `to_zone: build` with `touches[]` (repo paths).
5. Stop for approval. Do not implement `apps/` / `crates/` / `packages/`.

## Do not

Run Architecture Zone Phases 0–7, Event Catalog, SAM, MFE, or `primary-solve-agent` full orchestration.
