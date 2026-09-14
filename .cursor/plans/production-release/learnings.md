# Learnings — production-release

Append-only. Do not rewrite prior entries.

## 2026-09-14 — discover

- User chose recommended desktop distribution: signed GitHub Releases, not stores. Auto-update deferred to P1.
- Current CI Tauri job builds but does not `upload-artifact`; dashboard download copy is a lie.
- `rag::process_resume` already sets `failed`; earlier Operate note that rows stay `pending` was wrong if `process_resume` ran to the error handler.

## 2026-09-14 — plan

- Desktop CD is tag-triggered (`v*`), not `workflow_run` on every master, so API/web deploys stay decoupled.
- Story 2 is trust-boundary (keychain). Story 1 signing secrets and story 3 fail-closed/delete also require Solve.

## 2026-09-14 — solve

- Production detection is explicit `ANIKI_ENV=production` in fly.toml, not an APP_URL heuristic.
- Speechmatics: boot-fail in production (no demo STT in a real interview).
- Token: `keyring` crate behind Tauri commands, not Stronghold.
- Resume chunks already CASCADE; delete is SQL + route + UI only.

## 2026-09-14 — discover (download gate + landing)

- User: only logged-in users download; logged-in users must still see `/`.
- `LandingRoute` today redirects sessions to `/app` (landing story-01). Story 5 supersedes that.
- Public GitHub Release URLs remain reachable; product gate is UI + `GET /desktop/latest` 401.

## 2026-09-14 — plan (stories 4–5)

- story-04 authenticated download; story-05 landing always public including signed-in.
- ADR decision 6 replaced; decision 11 added. Removed SPA→api.github.com.

## 2026-09-14 — build

- API/web: `ANIKI_ENV`, resume delete, `GET /desktop/latest`, landing no bounce, dashboard + signed-in landing downloads.
- Overlay: `keyring` commands; Magic link tab removed; first-run permission copy.
- `release.yml` uses `tauri-action` with Apple notary env + Windows PFX; gate job fails if secrets or CI missing so unsigned Latest cannot publish.
- Did not locally notarize or Authenticode-sign; needs Environment `release` secrets and a `v*` tag.
- Fly boot-fail requires `SPEECHMATICS_API_KEY` before this API ships.

## 2026-09-14 — build (unsigned installers)

- Operator chose not to enroll Apple Developer / buy Authenticode. `release.yml` no longer requires or passes signing secrets.
- Gate still requires green CI + production `API_URL` / `WEB_URL`.
- Download UI documents Gatekeeper Open anyway and SmartScreen Run anyway.
- ADR decision 4 overridden in place (unsigned Latest is now allowed).

## 2026-09-14 — operate (release gate 403)

- Tag `v0.1.0` gate failed: `gh run list` returned HTTP 403 Resource not accessible by integration.
- Cause: workflow `permissions` was only `contents: write`, which drops default `actions: read`.
- Fix: add `actions: read`. Retag or push a new `v*` after this lands; the tagged SHA must include the workflow change.

