# Production deployment plan — Aniki

Adapted from Frodo `create-prod-deployment-plan` (read-only; no gp-nova `repos/` walk).

## Current vs target

| Component | Today | Target |
|-----------|--------|--------|
| API | Local `cargo run` / compose `full` | Fly `aniki-api` (`iad`), image from `apps/api/Dockerfile` |
| Web | `pnpm dev:web` :3000 | Cloudflare Pages production branch `main` |
| Postgres | Docker pgvector:pg16 | Neon Postgres 16 + `pgvector` extension |
| Redis | Docker redis:7 | Upstash Redis |
| Objects | Bytes through API | R2 bucket (after upload slice) |
| Desktop | Local `pnpm dev:desktop` | CI Tauri artifacts only |
| CI | `ci.yml` on PR/`main` | Unchanged as gate |
| CD | None | `deploy.yml` after green CI on `main` |

## Per-provider click-ops (story 1)

1. **Neon** — Create project; enable pgvector; copy pooled `DATABASE_URL`; run `001_init.sql` + `002_indexes.sql` once.
2. **Upstash** — Create Redis; copy TLS URL → Fly `REDIS_URL`.
3. **Fly** — `fly apps create aniki-api`; `fly secrets set` from table below; `flyctl deploy` once manually to prove `fly.toml`.
4. **Cloudflare** — Pages project; build command `pnpm install && pnpm --filter @aniki/web build`; output `apps/web/dist`; env `VITE_API_URL`.
5. **R2** — Create bucket `aniki-resumes` (private); keys unused until product story.
6. **GitHub** — Environment `production`; secrets only for deploy identities.

## Fly secrets (runtime)

| Name | Source |
|------|--------|
| `DATABASE_URL` | Neon |
| `REDIS_URL` | Upstash |
| `JWT_SECRET` | generated, ≥32 bytes |
| `APP_URL` | Pages origin `https://…pages.dev` or custom |
| `API_URL` | `https://aniki-api.fly.dev` |
| `SPEECHMATICS_API_KEY` | vendor |
| `OPENAI_API_KEY` / `ANTHROPIC_API_KEY` | vendor |
| `PORT` | already `8080` in fly.toml |

## GitHub Environment `production` secrets

| Name | Used by |
|------|---------|
| `FLY_API_TOKEN` | API deploy (if not OIDC) |
| `CLOUDFLARE_API_TOKEN` | Pages (if not OIDC) |
| `CLOUDFLARE_ACCOUNT_ID` | Pages |
| `DATABASE_URL` | migrate job only, if not using Fly for migrate |

## Proposed `deploy.yml` (Build implements)

```yaml
# Sketch — not in .github yet
on:
  workflow_run:
    workflows: [CI]
    types: [completed]
    branches: [main]

concurrency:
  group: deploy-production
  cancel-in-progress: false

jobs:
  deploy-api:
    if: ${{ github.event.workflow_run.conclusion == 'success' }}
    environment: production
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          ref: ${{ github.event.workflow_run.head_sha }}
      # migrate Neon
      # superfly/flyctl-actions deploy --config apps/api/fly.toml
      # curl -f https://aniki-api.fly.dev/health
  deploy-web:
    needs: deploy-api
    environment: production
    # pnpm install && pnpm --filter @aniki/web build
    # cloudflare/pages-action
```

Web **needs** API URL; deploy API first, then Pages.

## Rollback

| Layer | How |
|-------|-----|
| Fly | `fly releases rollback -a aniki-api` |
| Pages | Dashboard → previous deployment |
| Neon | Restore branch / PITR |
| Upstash | Redis is cache; flush if poison |

## First-cut release checklist

1. Secrets set; health on Fly after manual deploy.
2. Pages build with that API URL; magic-link `APP_URL`.
3. Enable `deploy.yml`.
4. Tag optional later (`v0.1.0`) if we switch off `workflow_run`.
