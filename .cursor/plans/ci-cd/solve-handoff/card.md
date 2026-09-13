---
status: done
from_zone: solve
to_zone: build
slug: ci-cd
created: 2026-09-12
---

# Handoff: solve → build

## Artifacts
- docs/architecture/ci-cd/README.md
- docs/architecture/ci-cd/adr.md
- docs/architecture/ci-cd/deployment-plan.md
- docs/architecture/ci-cd/api.md
- docs/architecture/ci-cd/threat-models/github-secrets.md

## Touches (Build only)
- .github/workflows/deploy.yml
- .github/workflows/ci.yml (only if workflow_run name must match)
- apps/api/src/main.rs
- README.md
- apps/api/.env.example

## Notes
Do not add gp-nova shared workflows. R2 env is documented, not required for first Pages/Fly deploy.

Implemented: `deploy.yml` after green CI on `main`; CORS from `APP_URL` + desktop locals + `CORS_ORIGINS`; idempotent `001_init.sql` so Neon migrate can re-run.

## Gate
Human must say continue/chain before Build implements workflows.
