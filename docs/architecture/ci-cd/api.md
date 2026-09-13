# Env and CORS for production

## API (`apps/api`)

| Var | Prod value |
|-----|------------|
| `APP_URL` | Cloudflare Pages origin (no trailing slash) |
| `API_URL` | Fly HTTPS origin |
| CORS | Parse `APP_URL` plus desktop: `tauri://localhost`, `http://localhost:1420` |

Today CORS is hardcoded in `main.rs` (localhost). Build must make it env-driven (`CORS_ORIGINS` comma-separated or derive from `APP_URL`).

## Web build

`VITE_API_URL` baked at Pages build = Fly origin.

## Desktop release (docs only)

`VITE_API_URL` / `VITE_WEB_URL` at Tauri build time; not Pages.
