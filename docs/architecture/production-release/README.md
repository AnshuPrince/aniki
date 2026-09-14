# Architecture — production-release

| Artifact | Path |
|----------|------|
| ADR | [adr.md](adr.md) |
| API / boot / env | [api.md](api.md) |
| Release job sketch | [release.md](release.md) |
| Threat | [threat-models/signing-and-tokens.md](threat-models/signing-and-tokens.md) |

Supersedes `docs/architecture/ci-cd/adr.md` **decision 7** (desktop CI-only). API/web `deploy.yml` on `master` is unchanged.

## Touches (when Build runs)

- `.github/workflows/ci.yml` (Linux only; no Tauri)
- `scripts/ci-local.sh`, `scripts/release-desktop.sh`
- `apps/desktop/src-tauri/tauri.conf.json` (bundle names; no signing placeholders for v1)
- `apps/desktop/src-tauri/src/commands.rs` (keyring get/set/delete)
- `apps/desktop/src/lib/api.ts`, `overlay/context/OverlayContext.tsx`, `overlay/screens/LoginScreen.tsx`
- `apps/api/src/config.rs`, `apps/api/src/main.rs`
- `apps/api/src/routes/resumes.rs`, `apps/api/src/routes/mod.rs` (also `GET /desktop/latest`)
- `apps/api/fly.toml` (`ANIKI_ENV=production`)
- `apps/web` `App.tsx` (no logged-in bounce from `/`), dashboard + signed-in landing download, resumes delete
- `packages/shared` client `deleteResume`, `desktopLatest`
- `README.md`, `apps/api/.env.example`
