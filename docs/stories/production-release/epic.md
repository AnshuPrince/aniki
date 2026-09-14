# Production release — Enhancement

**Slug:** production-release  
**Epic Goal:** Ship Aniki as an installable product: desktop overlays on GitHub Releases (not stores; unsigned v1), production-baked API/web URLs, fail-closed API secrets, and owner resume delete — without G-P factory tooling.

## Existing system context
- API on Fly (`aniki-api`), web on Cloudflare Pages (`aniki-web.pages.dev`), Neon + Upstash.
- CI builds Tauri on `macos-latest` (aarch64) and `windows-latest` (x86_64) and does not upload artifacts.
- `docs/architecture/ci-cd/adr.md` deferred desktop to CI-only; this epic **replaces** that decision for distribution only (API/web CD unchanged).
- Desktop JWT in `localStorage`; overlay login is paste-token from Sessions.
- Resume processing marks `failed` on error; there is no `DELETE /resumes/{id}`.
- Trust boundary: public unsigned installers, session tokens, resume delete.

## Enhancement details
GitHub Release on `v*` tags after green CI. Unsigned macOS `.dmg` and Windows NSIS `.exe` (no Apple/Windows developer enrollment in v1). **Download CTAs only for a logged-in session** (dashboard, and landing if they visit `/` while signed in). Logged-in users can still open `/` (supersedes landing story-01 bounce). Tauri release env `VITE_API_URL` / `VITE_WEB_URL`. OS credential store for the JWT. Fly refuses to boot with the documented-dev `JWT_SECRET` or without Google OAuth. Resume delete with chunk cascade.

## Stories
1. GitHub Releases + **authenticated** download links
2. Production overlay (URLs, keychain, permissions copy)
3. API/web fail-closed + owner resume delete
4. Only a logged-in user can download the desktop app
5. Logged-in user can still visit the landing page

## Compatibility
- `deploy.yml` stays `workflow_run` on `master` for API/web.
- Desktop release is a **separate** workflow on tags so a docs-only master push does not mint a new `.dmg`.
- Existing paste-token login remains until story 2 stores the same JWT in the keychain.

## Risks
Spoofed unsigned installer; overlay still on localhost if Vite env omitted. Mitigation: Environment `release` with required `API_URL` / `WEB_URL`; fail the Tauri build if either is empty or localhost; session-gated `GET /desktop/latest`.

## Rollback
GitHub Release: unpublish / mark previous as latest. Fly/Pages unchanged from ci-cd plan. Resume delete is irreversible for that row — confirm in UI.

## Definition of done
- [ ] Stories accepted
- [ ] Solve ADR for signing + token storage + tag vs master
- [ ] Candidate path: Pages → Download → install → token → session against Fly
