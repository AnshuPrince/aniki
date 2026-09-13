# Learnings — ci-cd

## 2026-09-12 — discover / plan / solve

- Frodo `github-workflows` and `create-prod-deployment-plan` are G-P AWS/SAM canvases; Aniki uses the same gates (CI before CD, environments, concurrency, rollback) with Fly + Pages + Neon + Upstash.
- R2 provisioned in docs; API wiring waits for the upload slice.
- Desktop stays CI artifacts, not store CD.

## 2026-09-12 — build

- Keep workflow `name: CI` so `workflow_run` matches; only deploy when that run was a **push** to `master` (PR CI must not ship). Primary branch stays `master`.
- Prod `psql` against Neon needs `CREATE … IF NOT EXISTS` (or a real migrator). Swallowing SQL errors in `db::run_migrations` is still local-only.
- Pages `VITE_API_URL` is a GitHub **variable** (`API_URL`), not a secret — it is baked into the static bundle.
- Fly Neon/Upstash need sqlx `tls-rustls-ring-native-roots` and redis `tokio-rustls-comp`. Local `postgres://` / `redis://` still work.

## 2026-09-13 — CI green on master

- `pnpm/action-setup@v4` cannot set `version: 9` when `package.json` has `packageManager: pnpm@9.15.0`; omit `version` and let Corepack read the pin.
- Workspace `clippy -D warnings` excludes `aniki-desktop` (cocoa/objc `msg_send!` unexpected_cfgs + deprecated). Tauri job still compiles the overlay.
- Linux Rust job must `--exclude aniki-desktop --exclude aniki-audio-core` on check/clippy/test: desktop pulls GTK; audio-core pulls `cpal`/`alsa-sys` and ubuntu has no `alsa`. Overlay + capture belong on the macOS/Windows Tauri jobs.

