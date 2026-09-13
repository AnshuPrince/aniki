# Provision clouds and GitHub Environments

**Slug:** ci-cd  
**Story:** 1 of 3

## User story

As the operator, I want each cloud account and GitHub Environment created with named secrets so that CD jobs have somewhere to authenticate.

## Acceptance criteria

- [ ] GIVEN a new GitHub Environment `production` WHEN a deploy job runs THEN it requires that environment (optional reviewer)
- [ ] GIVEN Fly THEN app `aniki-api` exists in `iad` matching `fly.toml` and GitHub can deploy via **Fly OIDC** (preferred) or `FLY_API_TOKEN` as Environment secret
- [ ] GIVEN Neon THEN a Postgres 16 + pgvector project exists; `DATABASE_URL` is a Fly secret, not a workflow input
- [ ] GIVEN Upstash THEN Redis URL is a Fly secret `REDIS_URL`
- [ ] GIVEN Cloudflare THEN an account + Pages project for `apps/web` and (later) an R2 bucket; deploy uses **Cloudflare OIDC** or `CLOUDFLARE_API_TOKEN` + `CLOUDFLARE_ACCOUNT_ID`
- [ ] GIVEN docs THEN `.env.example` lists prod-only names without values; README deploy section lists the Environment secret table

## Existing context

- Paths: `README.md` Deployment; `apps/api/fly.toml`; `apps/api/.env.example`
- Pattern: no secrets in git

## Out of scope

Writing the deploy YAML (story 2–3); creating Stripe/Resend.

## Risks

Personal tokens in repo secrets instead of Environments.

## Definition of done

- [ ] Secret table in architecture doc matches what Build will implement
- [ ] Human has a click-ops checklist
