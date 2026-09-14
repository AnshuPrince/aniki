---
status: done
from_zone: solve
to_zone: build
slug: production-release
created: 2026-09-14
---

# Handoff: solve → build

## Artifacts
- docs/architecture/production-release/README.md
- docs/architecture/production-release/adr.md
- docs/architecture/production-release/api.md
- docs/architecture/production-release/release.md
- docs/architecture/production-release/threat-models/signing-and-tokens.md

## Touches (Build only)
- `.github/workflows/release.yml`
- `.github/workflows/ci.yml`
- `apps/desktop/src-tauri/tauri.conf.json`
- `apps/desktop/src-tauri/src/commands.rs`
- `apps/desktop/src/lib/api.ts`
- `apps/desktop/src/overlay/context/OverlayContext.tsx`
- `apps/desktop/src/overlay/screens/LoginScreen.tsx`
- `apps/api/src/config.rs`
- `apps/api/src/main.rs`
- `apps/api/src/routes/resumes.rs`
- `apps/api/src/routes/mod.rs`
- `apps/api/fly.toml`
- `apps/web/src/App.tsx` (`LandingRoute`)
- `apps/web/src/pages/DashboardPage.tsx`
- `apps/web/src/pages/LandingPage.tsx`
- `apps/web/src/components/DashboardLayout.tsx`
- `apps/api/src/routes/` desktop latest
- `packages/shared` API client
- `README.md`
- `README.md`
- `apps/api/.env.example`

## Notes for the next zone
Do not put GitHub tokens in the SPA. `ANIKI_ENV=production` on Fly. Keyring via Tauri commands + `keyring` crate. Resume `ON DELETE CASCADE` already exists. **Operator override 2026-09-14:** unsigned GitHub Release assets are allowed; do not require Apple/Windows signing secrets.

## Gate
Human must say continue/chain before Build implements apps/workflows.
