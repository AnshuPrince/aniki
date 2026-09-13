# ADR: Native GitHub Actions to Fly + Pages

**Slug:** ci-cd  
**Status:** proposed (approve before Build)  
**Date:** 2026-09-12

## Context
README already names Fly, Cloudflare Pages, Neon, Upstash, R2. Frodo `github-workflows` would pull G-P SAM/OIDC shared workflows. Aniki is a personal monorepo.

## Decisions

1. **Keep vendors** as README: Fly (API), Pages (web), Neon (Postgres+pgvector), Upstash (Redis), R2 (later with resume uploads).
2. **Do not** call `gp-nova/devx-shared-workflows` or AWS `deploy-v1`.
3. **CI stays** `.github/workflows/ci.yml`. **CD** is a second workflow triggered on successful CI on `main` (`workflow_run`) **or** on `v*` tags after CI. Prefer `workflow_run` on `main` for pre-beta.
4. **Auth:** Fly Machines OIDC from GitHub; Cloudflare Pages via `cloudflare/pages-action` or Wrangler + OIDC. Fallback: Environment secrets `FLY_API_TOKEN`, `CLOUDFLARE_API_TOKEN`.
5. **Data plane secrets** (DATABASE_URL, REDIS_URL, JWT_SECRET, LLM keys) live on **Fly**, not in GitHub, except tokens needed to *deploy*.
6. **R2** is provisioned empty in story 1 docs; API env `R2_*` waits for the R2 product slice.
7. **Desktop Tauri** remains CI artifact-only (GitHub Actions upload-artifact); no store CD in v1.
8. **Migrations:** dedicated Fly one-shot or `fly ssh console` / GitHub job using `psql` against Neon with a GitHub secret `DATABASE_URL` marked Environment + no logs — prefer Fly secret + `Dockerfile.migrate` as a Fly release command if feasible. Do not ship “swallow failed SQL” as the prod migrator.

## Consequences
Operator must create four vendor accounts before the first green deploy. Preview environments (Fly apps per PR) are **out** of v1.
