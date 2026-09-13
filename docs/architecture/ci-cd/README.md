# Architecture — ci-cd

Frodo skills used with Aniki substitutions: `create-epic` (stories), `github-workflows` (CI gate, environments, concurrency — **not** `gp-nova/devx-shared-workflows`), `create-prod-deployment-plan` (this folder — **not** a gp-nova Slack canvas).

| Artifact | Path |
|----------|------|
| ADR | [adr.md](adr.md) |
| Deployment plan | [deployment-plan.md](deployment-plan.md) |
| Env / CORS | [api.md](api.md) |
| Threat | [threat-models/github-secrets.md](threat-models/github-secrets.md) |

## Touches (when Build runs)

- `.github/workflows/ci.yml` (unchanged or `workflow_run` trigger)
- `.github/workflows/deploy.yml` (new)
- `apps/api/fly.toml` / Dockerfile (only if health/migrate needs it)
- `apps/api/src/main.rs` CORS from `APP_URL`
- `README.md`, `apps/api/.env.example`
