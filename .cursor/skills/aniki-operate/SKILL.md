---
name: aniki-operate
description: Operates Aniki locally and in deploy (health, incidents, playbooks, test strategy). Use for /health, pool timeouts, smoke tests, Fly, runbooks; not for new features.
---

# aniki-operate

Read [../aniki-shared.md](../aniki-shared.md). State environment: `dev` (default), `prod` if Fly.

## Do

1. Prefer evidence: `GET /health`, `docker compose ps`, API logs, `scripts/smoke-test.sh`.
2. Optional Frodo: `build-test-strategy` → `docs/test-strategies/<slug>/`; `production-incident-triage-and-fix` **without** assuming Coralogix. Do not use `cx-*` skills unless Aniki actually ships CX telemetry.
3. Write or update `docs/playbook/` (one topic per file). Recurring issues → offer `aniki-discover`.
4. Do not implement product features here; hand off to Discover/Plan.

## Local checks

- Postgres/Redis: `docker compose up -d postgres migrate redis`
- API: `cargo run -p aniki-api` then `curl -s localhost:8080/health`
- Web: http://localhost:3000 (not 5173)
