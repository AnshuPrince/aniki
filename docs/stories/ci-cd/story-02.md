# Deploy API to Fly after CI

**Slug:** ci-cd  
**Story:** 2 of 3

## User story

As the operator, I want `master` (or a release tag) to build the API image and deploy to Fly only if CI passed so that production is not a laptop `fly deploy`.

## Acceptance criteria

- [ ] GIVEN CI on `master` failed THEN deploy does not run
- [ ] GIVEN CI succeeded THEN a deploy job uses `apps/api/Dockerfile` + `fly.toml` (`flyctl deploy --config apps/api/fly.toml --remote-only`)
- [ ] GIVEN schema files changed THEN migrations apply to Neon **before** or atomically with the new API (explicit step; do not rely on API `run_migrations` `.ok()` swallow)
- [ ] GIVEN `/health` THEN Fly HTTP checks continue; deploy job curls the public URL and fails if `database` is not `up`
- [ ] GIVEN concurrency THEN only one production API deploy at a time (`cancel-in-progress: false` for prod)
- [ ] GIVEN rollback THEN docs name `fly releases rollback` (or equivalent)

## Existing context

- Paths: `.github/workflows/ci.yml`; `apps/api/Dockerfile`; `apps/api/fly.toml`; `apps/api/migrations/`

## Out of scope

Pages; R2 bucket IAM.

## Risks

Migrate after a breaking API ships. Order: migrate expand → deploy API → later contract cleanup.

## Definition of done

- [ ] Workflow sketched in architecture (Build implements)
- [ ] JWT_SECRET, SPEECHMATICS_*, OPENAI_*, ANTHROPIC_* live as Fly secrets
