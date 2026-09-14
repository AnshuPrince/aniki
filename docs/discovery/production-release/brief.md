# Discovery brief — production-release

## Problem
API and web are on Fly and Cloudflare Pages, but a candidate still cannot install a production desktop overlay. CI compiles Tauri on macOS and Windows and discards the binaries. The dashboard says “download the desktop overlay” with no link. Release builds default `VITE_API_URL` to `http://localhost:8080`. Session JWTs live in desktop `localStorage`. Unsigned apps will fail Gatekeeper and Windows SmartScreen. The operator cannot ship Aniki as a product until desktop is signed, downloadable, and pointed at Fly, and the API/web refuse to run half-configured.

## Evidence
- Code / docs (paths):
  - `.github/workflows/ci.yml` — Tauri job `pnpm tauri build` for `aarch64-apple-darwin` and `x86_64-pc-windows-msvc`; no `upload-artifact`, no GitHub Release
  - `.github/workflows/deploy.yml` — Fly + Pages only
  - `docs/architecture/ci-cd/adr.md` decision 7 — “Desktop Tauri remains CI artifact-only; no store CD in v1”
  - `apps/desktop/src-tauri/tauri.conf.json` — `bundle.targets: all`, identifier `ai.aniki.desktop`, no updater, no signing config
  - `apps/desktop/src/overlay/constants.ts` — `VITE_API_URL` / `VITE_WEB_URL` fall back to localhost
  - `apps/desktop/src/lib/api.ts` — JWT in `localStorage`
  - `apps/web/src/App.tsx` — `LandingRoute` redirects **logged-in** users from `/` to `/app`; `ProtectedRoute` gates `/app`
  - `docs/stories/landing/story-01.md` — accepted AC: session on `/` → `/app` (this slice **supersedes** that)
  - `apps/api/src/config.rs` — `JWT_SECRET` defaults to `dev-jwt-secret-change-in-production`; Google OAuth optional
  - `README.md` — Desktop: “CI Tauri artifacts only”
- User statements (verbatim):
  - “How will end user install the desktop app?”
  - “plan for a production ready app for all the apps, specially the desktop app.”
  - “continue with recommended”
  - “Should w allow deletion of resume ?”
  - “discover and add more stories”
  - “Only logged in user can download the desktop app”
  - “Logged in user can still visit the landing page”

## Users and context
Candidate on macOS (Apple Silicon first) or Windows x64: install overlay, grant mic/screen permissions, sign in with a token from Pages, run a live session against Fly. Operator: cut a release without cloning the repo on two laptops. Dashboard user: upload/delete resumes and copy a token.

## In scope
- Signed, notarized macOS `.dmg` and signed Windows installer via **GitHub Releases** (direct download, not App Store / Microsoft Store)
- Bake production `VITE_API_URL` / `VITE_WEB_URL` into Tauri release builds
- Dashboard + **authenticated** download (not a public landing installer)
- `/` stays the marketing page for **logged-out and logged-in** users (no bounce to `/app`)
- Persist desktop session token in the OS credential store (not `localStorage`)
- First-run permission copy (Microphone, Screen Recording on macOS)
- API fail-closed in production when `JWT_SECRET` is the dev default or Google OAuth is missing; OpenAI required for resume embeddings
- Owner-scoped resume delete (chunks cascade)
- Operator runbook: signing secrets, release steps, rollback of a GitHub Release

## Out of scope
- Apple App Store / Microsoft Store / TestFlight
- Linux desktop
- Windows OCR (still unimplemented)
- Stripe billing, Resend email, R2 presigned uploads
- Auto-update in v1 (document `tauri-plugin-updater` as P1; v1 users re-download from Releases)
- Custom domains (keep `*.fly.dev` / `*.pages.dev` unless already set)
- Store-style notarization of Intel macOS unless CI matrix adds `x86_64-apple-darwin`

## Examples (example mapping)
- Rule: Installer URLs are not shown without a session.
- Example: Logged-out `/` has Sign in / Get started only. Logged-in `/app` (and logged-in `/`) show Mac/Windows download.
- Rule: A session must not trap the user in the dashboard.
- Example: Logged-in user opens `https://aniki-web.pages.dev/` and still sees the landing; nav has “Open app”.
- Residual: if the GitHub repo is public, Release assets are still fetchable by URL. UI + authenticated `GET /desktop/latest` hide the casual path; making Releases private is a later hardening (private repo or R2).
- Rule: Production overlay never talks to localhost.
- Example: Built with `VITE_API_URL=https://aniki-api.fly.dev` and `VITE_WEB_URL=https://aniki-web.pages.dev`.
- Rule: Unsigned builds are not “production.”
- Example: macOS notarized Developer ID; Windows Authenticode. Missing certs → release job fails, no draft published.
- Rule: Resume delete is owner-only and removes chunks.
- Example: `DELETE /resumes/{id}` 204 for owner; 404 for another user’s id.
- Question: Apple notarization Apple ID vs App Store Connect API key in GitHub Environment `production`?

## Success
A candidate can download a signed overlay from the web app, paste (or later keychain-restore) a token, and complete a live session against production API/web. Operator can cut that release from GitHub without a local `tauri build`.

## Open questions
- Apple Developer Program team id and whether we ship Apple Silicon only in v1.
- Windows EV vs OV code-signing certificate (SmartScreen reputation).
- GitHub Release on every green `master` vs explicit `v*` tags (recommend **tags** so desktop versions stay meaningful).
- Intel Mac support in v1.
