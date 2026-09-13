---
name: aniki-build
description: Implements an approved Aniki Solve handoff in this monorepo and reviews the change. Use when implementing a slice, coding from a handoff, or the user says continue to build.
---

# aniki-build

Confirm `slug`. Require `.cursor/plans/<slug>/solve-handoff/` with `status: open` **or** an explicit user skip of Solve (Plan-only). Read [../aniki-shared.md](../aniki-shared.md).

## Do

1. Implement only `touches[]` plus strictly necessary call sites. Follow existing Aniki patterns.
2. Verify: `cargo`/`pnpm` as relevant; browser for web UI; say what you could not verify for desktop/Tauri.
3. Optional Frodo: `review-pr-moscow`, `review-story`, `owasp-test-generator` for auth/billing. Native: security-review/Bugbot only if the user asks.
4. Close solve handoff. Append `.cursor/plans/<slug>/learnings.md`. If an ADR is wrong, open `build-handoff/` → `to_zone: solve` instead of silently changing the contract.
5. Do not open a PR or commit unless the user asked.

## Do not

Invent Stripe/R2/auth contracts, use SAM/MFE/Lyra skills, or start unrelated remaining phases.
