# Aniki

Cloud-first, privacy-focused real-time interview assistant. ParakeetAI-style product with a Rust (Axum) backend, React web dashboard, and Tauri 2 desktop overlay.

## Architecture

```
apps/api       — Axum backend (auth, STT JWT minting, RAG, LLM proxy)
apps/web       — React dashboard (login, resumes, billing)
apps/desktop   — Tauri 2 stealth overlay
packages/shared — Typed API client and shared types
packages/ui    — Shared React components
crates/domain  — Shared Rust domain models
crates/audio-core — Audio capture pipeline (M2)
crates/stealth-window — Stealth overlay APIs (M4)
```

Agent workflow (Discover → Plan → Solve → Build → Operate) lives in [`AGENTS.md`](AGENTS.md) and `.cursor/skills/aniki-*/`. It mirrors Frodo zone discipline without G-P factory tooling.

## Prerequisites

- Node.js 20+
- pnpm 9+
- Rust stable
- Docker (for Postgres + Redis)

## Local development

```bash
# Start Postgres + Redis (migrations run automatically via the migrate service)
docker compose up -d postgres migrate redis

# Install JS dependencies
pnpm install

# Run API (separate terminal)
cargo run -p aniki-api

# Run web dashboard
pnpm dev:web

# Run desktop overlay
pnpm dev:desktop
```

### Environment

Copy `apps/api/.env.example` to `apps/api/.env` and set:

- `SPEECHMATICS_API_KEY` — for real STT (optional in dev; stub JWT used if unset)
- `JWT_SECRET` — session token signing key

Web/desktop use `VITE_API_URL=http://localhost:8080` (default).

`Cargo.lock` is committed. Build the workspace as locked — running `cargo update` can pull a
`pgvector` release whose sqlx range spans 0.8 and 0.9, which puts two `sqlx-core` versions in the
graph and fails with `the trait bound pgvector::Vector: sqlx::Type<_> is not satisfied`.

### Troubleshooting

`GET /health` reports database reachability and live pool stats:

```bash
curl -s localhost:8080/health
# {"database":"up","db_pool":{"idle":1,"size":2},"status":"ok",...}
```

**`pool timed out while waiting for an open connection`** — the API could not get a Postgres
connection within `DATABASE_ACQUIRE_TIMEOUT_SECS`. Check, in order:

1. Postgres is up: `docker compose ps` (the container must be `healthy`, not just running).
2. It did not restart underneath a running API — restart `cargo run -p aniki-api` if it did.
3. `DATABASE_URL` points at the right host/port.
4. Server capacity: `docker compose exec postgres psql -U aniki -d aniki -c "show max_connections"`
   and compare with `select count(*) from pg_stat_activity where datname='aniki'`.

If `db_pool.size` is pinned at `DATABASE_MAX_CONNECTIONS` with `idle: 0`, requests are queueing —
raise the limit or look for a slow query holding connections.

### Auth flow (dev)

1. Open http://localhost:3000 (public landing), choose **Get started**, and **Continue with Google**
   (`GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET` in `apps/api/.env`; Google redirect `http://localhost:3000/auth/oauth/callback`)
2. On the **Sessions** page (`/app/sessions`), click **Copy access token**
3. In the desktop overlay, choose **Access token** and paste it

The overlay **Magic link** tab is leftover from log-only email sign-in and is not the production path.

### Desktop (macOS)

Production installers are GitHub Releases (unsigned Apple Silicon `.dmg`), shown after you sign in. Windows is not offered yet. Gatekeeper may warn; right-click → Open.

- There is no Dock icon. Use the **menu bar tray**: Show overlay, Toggle click-through, Quit Aniki.
- Grant **Microphone** and **Screen Recording** (interviewer system audio + OCR) in System Settings → Privacy & Security.
- Overlay hotkeys: `⌘⇧H` collapse to the pebble icon / expand, `⌘⇧C` click-through.
- **Collapse** shrinks the overlay to a ~1cm circular icon that stays on screen; click it to expand.
  **Stealth hide** (red ×) removes it entirely — restore from the tray.
- The overlay stores the session JWT in the OS credential store (Keychain / Credential Manager), not browser storage.

## Docker (full stack)

```bash
docker compose --profile full up --build
```

## CI

GitHub Actions workflow **CI** (`.github/workflows/ci.yml`) runs on pull requests and pushes to `master`:

- Rust: Postgres + Redis services, `001` + `002` migrations, `cargo check` / `clippy` / `test`
- Frontend: `pnpm typecheck`, `pnpm lint`, `@aniki/web` production build
- No hosted Tauri jobs (macOS/Windows minutes). Overlay compile and installer upload: [`docs/playbook/local-ci-and-desktop-release.md`](docs/playbook/local-ci-and-desktop-release.md) (`./scripts/ci-local.sh`, `./scripts/release-desktop.sh`).

PR runs cancel when a newer commit is pushed. Keep the workflow `name: CI` — deploy keys off that name.

## Deployment

CD is `.github/workflows/deploy.yml`. After **CI succeeds on a push to `master`**, it deploys production (or run **Deploy** → **Run workflow**). Jobs use GitHub Environment `production` and `concurrency: deploy-production` (`cancel-in-progress: false`). Cloudflare Pages production branch is `master` (match that in the Pages project settings).

| Piece | Target |
|-------|--------|
| API | Fly.io app `aniki-api` (`apps/api/fly.toml`, region `iad`) |
| Web | Cloudflare Pages (`pnpm --filter @aniki/web build` → `apps/web/dist`) |
| Postgres | Neon + pgvector |
| Redis | Upstash (Fly secret `REDIS_URL`) |
| Objects | R2 later — not required for first deploy |
| Desktop | GitHub Releases (unsigned Mac `.dmg`), download after sign-in |

### One-time vendor setup

1. **Neon** — Postgres 16, enable `pgvector`, copy the pooled `DATABASE_URL`.
2. **Upstash** — Redis TLS URL → Fly `REDIS_URL`.
3. **Fly** — `fly apps create aniki-api`, then `fly secrets set` (see below). First image: `flyctl deploy --config apps/api/fly.toml`.
4. **Cloudflare Pages** — project name matching GitHub variable `CLOUDFLARE_PAGES_PROJECT`.
5. **GitHub** — Environment `production` with the secrets and variables below.

### Fly runtime secrets (not GitHub)

Set on the Fly app: `DATABASE_URL`, `REDIS_URL`, `JWT_SECRET` (≥32 bytes, not the documented-dev default), `APP_URL` (Pages origin, no trailing slash), `API_URL` (`https://aniki-api.fly.dev` or custom), `SPEECHMATICS_API_KEY`, `OPENAI_API_KEY`, `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, optional `ANTHROPIC_API_KEY`, optional `CORS_ORIGINS`, optional `GITHUB_TOKEN` (rate-limit for `/desktop/latest`). `ANIKI_ENV=production` is set in `fly.toml`. Do not put database, LLM, or Google client secrets on Pages.

Google sign-in uses a Web OAuth client. Redirects must be `http://localhost:3000/auth/oauth/callback` and `https://aniki-web.pages.dev/auth/oauth/callback`. Until Google verifies the app, **testing mode** allows about 100 test users. Consent can use `{APP_URL}/privacy`.

### GitHub Environment `production`

**Secrets:** `FLY_API_TOKEN`, `CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_ACCOUNT_ID`, `DATABASE_URL` (Neon; used only to apply migrations, never logged).

**Variables:** `API_URL` (Fly HTTPS origin, used for health checks and `VITE_API_URL` at Pages build), `CLOUDFLARE_PAGES_PROJECT`.

### Desktop installers (local, not Actions)

Do not tag to trigger hosted macOS/Windows runners. On this machine:

```bash
./scripts/ci-local.sh
VITE_API_URL=https://aniki-api.fly.dev VITE_WEB_URL=https://aniki-web.pages.dev ./scripts/release-desktop.sh v0.1.1
```

See [`docs/playbook/local-ci-and-desktop-release.md`](docs/playbook/local-ci-and-desktop-release.md). GitHub Environment `release` is unused after this change.

Rollback: `fly releases rollback -a aniki-api`; Pages dashboard → previous deployment; Neon PITR / branch restore. GitHub Release: mark the previous tag as latest.

## License

MIT
