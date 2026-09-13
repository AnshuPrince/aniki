# Production CI/CD — Enhancement

**Slug:** ci-cd  
**Epic Goal:** Automate deploy of the Aniki API to Fly and the dashboard to Cloudflare Pages, with Neon/Upstash (and later R2) provisioned out-of-band, without G-P shared workflows.

## Existing system context
- CI: GitHub Actions `ci.yml` (Rust + frontend + Tauri).
- API image: `apps/api/Dockerfile`; Fly app config: `apps/api/fly.toml`.
- Web: Vite static build (`pnpm --filter @aniki/web build`).
- Data: local Docker Postgres+pgvector and Redis; production targets Neon and Upstash.
- Frodo `github-workflows` / `create-prod-deployment-plan` assume gp-nova AWS/SAM — **do not use those workflows**. Keep their ideas: CI gate, environment protection, concurrency, pinned actions, rollback notes.

## Enhancement details
Add `deploy.yml` (or split `deploy-api.yml` / `deploy-web.yml`) using GitHub Environments `production`. Wire OIDC to Fly and Cloudflare. Document Neon/Upstash/R2 as click-ops + secrets.

## Stories
1. Provision + secrets + GitHub Environments
2. API CD: migrate Neon + `fly deploy`
3. Web CD: Cloudflare Pages + production CORS/`APP_URL`

## Compatibility
- Existing `ci.yml` stays; CD **needs** it (or duplicates checks).
- No API contract change except env-driven CORS origins.

## Risks
Leaked `DATABASE_URL` / `JWT_SECRET`. Mitigation: GitHub Environment + Fly secrets; never echo secrets in logs.

## Rollback
Fly: `fly releases` / previous image. Pages: prior deployment in dashboard. Neon: PITR if enabled.

## Definition of done
- [ ] Stories accepted
- [ ] Architecture + threat notes in `docs/architecture/ci-cd/`
- [ ] Operator can execute Build later without inventing vendors
