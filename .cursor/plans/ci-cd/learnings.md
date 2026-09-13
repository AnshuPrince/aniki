# Learnings — ci-cd

## 2026-09-12 — discover / plan / solve

- Frodo `github-workflows` and `create-prod-deployment-plan` are G-P AWS/SAM canvases; Aniki uses the same gates (CI before CD, environments, concurrency, rollback) with Fly + Pages + Neon + Upstash.
- R2 provisioned in docs; API wiring waits for the upload slice.
- Desktop stays CI artifacts, not store CD.

## 2026-09-12 — build

- Keep workflow `name: CI` so `workflow_run` matches; only deploy when that run was a **push** to `main` (PR CI must not ship).
- Prod `psql` against Neon needs `CREATE … IF NOT EXISTS` (or a real migrator). Swallowing SQL errors in `db::run_migrations` is still local-only.
- Pages `VITE_API_URL` is a GitHub **variable** (`API_URL`), not a secret — it is baked into the static bundle.
- Fly Neon/Upstash need sqlx `tls-rustls-ring-native-roots` and redis `tokio-rustls-comp`. Local `postgres://` / `redis://` still work.

