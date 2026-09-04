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

### Auth flow (dev)

1. Open http://localhost:3000 and sign in with email
2. Check API logs for the magic link URL (email not sent in dev) and open it
3. On the **Sessions** page, click **Copy access token**
4. In the desktop overlay, choose **Access token** and paste it

The magic-link token is single-use: once the browser verifies it, the desktop cannot reuse it. The
overlay's **Magic link** tab only works if you copy the `token=` value from the logs without opening
the link first.

### Desktop (macOS)

- There is no Dock icon. Use the **menu bar tray**: Show overlay, Toggle click-through, Quit Aniki.
- Grant **Microphone** and **Screen Recording** (interviewer system audio + OCR) in System Settings → Privacy & Security.
- Overlay hotkeys: `⌘⇧H` collapse to the pebble icon / expand, `⌘⇧C` click-through.
- **Collapse** shrinks the overlay to a ~1cm circular icon that stays on screen; click it to expand.
  **Stealth hide** (red ×) removes it entirely — restore from the tray.

## Docker (full stack)

```bash
docker compose --profile full up --build
```

## CI

PR checks: `cargo check`, `cargo clippy`, `pnpm lint`, `pnpm typecheck`

## Deployment (pre-beta checklist)

- API: `fly deploy` from `apps/api` (scale-to-zero configured in `fly.toml`)
- Web: Cloudflare Pages (static build from `apps/web`)
- Postgres: Neon free tier
- Redis: Upstash free tier
- Storage: Cloudflare R2

## License

MIT
