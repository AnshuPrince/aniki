# ADR: Public `/`, app at `/app`

**Slug:** landing  
**Status:** proposed  
**Date:** 2026-09-12

## Context
Parakeet uses a marketing origin plus signed-in workspace. Aniki currently makes `/` the workspace.

## Decisions

1. **`/`** — public `LandingPage`.
2. **`/app/*`** — current dashboard (index, resumes, sessions, billing).
3. **Redirects:** logged-in `/` → `/app`; old `/sessions` → `/app/sessions` (same for resumes/billing).
4. **IA** follows Parakeet’s section list (hero, features, privacy, pricing, FAQ), content is Aniki-only.
5. **No** invented testimonials, download counts, or “verified on Zoom/Teams” grid unless we later produce our own checks.
6. **No** trackers in v1 (`APP_URL` stays a static origin).
7. Visual: dark glass, not a Lyra/MFE and not a 1:1 Parakeet skin.

## Consequences
Magic-link emails still use `APP_URL` as origin. Desktop “open dashboard” should prefer `/app/sessions` for operators already signed in; landing is fine as default `WEB_URL`.
