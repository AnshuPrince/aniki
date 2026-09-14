# ADR: GitHub Releases, keychain JWT, fail-closed Fly

**Slug:** production-release  
**Status:** proposed (approve before Build)  
**Date:** 2026-09-14

## Context
Fly + Pages are live. Desktop is compiled in CI and thrown away. Dashboard copy promises a download. Overlay defaults to localhost and stores the session JWT in `localStorage`. Stories 1–3 in `docs/stories/production-release/`. Operator later chose to ship unsigned installers rather than enroll for Apple/Windows certificates.

## Decisions

1. **Distribution:** Direct download via **GitHub Releases**. No App Store, Microsoft Store, or TestFlight in this slice. Auto-update (`tauri-plugin-updater`) is **P1**, not v1.

2. **When to cut a desktop build:** Operator **Mac** via `scripts/release-desktop.sh` (unsigned Apple Silicon `.dmg`). Windows NSIS is deferred. Do **not** attach installers to every `master` deploy. Do **not** use GitHub-hosted macOS/Windows runners for Tauri.

3. **Matrix:** macOS `aarch64-apple-darwin` only. No Linux, no Intel Mac, no Windows in this slice. Landing/dashboard must not offer those assets.

4. **Signing (v1 override, 2026-09-14):** Publish **unsigned** GitHub Release installers from a **local** Tauri build. Do **not** require Apple Developer Program or Windows Authenticode. `scripts/release-desktop.sh` fails if `VITE_API_URL` / `VITE_WEB_URL` are empty or contain `localhost`. GitHub `ci.yml` is Linux-only (no Tauri). Developer ID + notarization and Authenticode remain a later upgrade.

5. **Baked URLs:** Release Tauri build **requires** `VITE_API_URL` and `VITE_WEB_URL` (HTTPS Fly and Pages origins). Empty or `localhost` → fail the job. Same values as GitHub variables `API_URL` and a new `WEB_URL` (Pages origin).

6. **Download UX:** Installer links are **session-gated**. Logged-out landing has no asset URLs. `GET /desktop/latest` (JWT required) returns the latest GitHub Release asset URLs; the API uses a server-side GitHub fetch (optional `GITHUB_TOKEN` for rate limit, never sent to the browser). Dashboard and signed-in landing chrome consume that API. Pages must not call `api.github.com` as the download source.


7. **Token storage:** Tauri commands wrapping the `keyring` crate (macOS Keychain, Windows Credential Manager). Service name `ai.aniki.desktop`, account `access_token`. On successful paste-login, write keyring and **delete** `localStorage` key `aniki_access_token`. On logout or API 401, delete keyring. No overlay Google OAuth in this slice (Pages still issues the JWT).

8. **Production boot (`ANIKI_ENV=production`):** Set in `apps/api/fly.toml` `[env]`. When set, `Config::from_env` **returns Err** (process exit) if:
   - `JWT_SECRET` empty or equal to `dev-jwt-secret-change-in-production`
   - Google OAuth not configured
   - `OPENAI_API_KEY` empty
   - `SPEECHMATICS_API_KEY` empty  
   Local/dev: `ANIKI_ENV` unset; current defaults remain.

9. **Resume delete:** `DELETE /resumes/{id}` authenticated, `WHERE id = $1 AND user_id = $2`. `resume_chunks` already `ON DELETE CASCADE`. Sessions already `ON DELETE SET NULL` on `resume_id`. No extra migration unless we add an index (not required). 204 empty body; other user’s id → **404**.

10. **Speechmatics:** Fail at **boot** in production (decision 8), not at first JWT mint, so a live interview never gets the demo-transcript stub.

11. **Landing while signed in:** `/` always renders the landing page. Remove `LandingRoute` bounce to `/app`. Signed-in nav: Open app → `/app`. Login/OAuth success still goes to `/app`. This supersedes `docs/stories/landing/story-01.md` “session on `/` → `/app`”.

## Consequences
- Candidates must bypass Gatekeeper (right-click → Open). Windows is not shipped.
- Anyone can spoof an unsigned installer more easily; mitigate with HTTPS GitHub Releases for this repo and session-gated `GET /desktop/latest`.
- Keychain will show an OS prompt on first save; document it.
- `docs/architecture/ci-cd/adr.md` decision 7 is **replaced** by this ADR.
