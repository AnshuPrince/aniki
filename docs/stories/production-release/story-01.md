# GitHub Releases and dashboard downloads

**Slug:** production-release  
**Story:** 1 of 5

## User story

As a candidate, I want to download an Aniki overlay installer for my OS from the website so that I can install it without compiling the repo.

## Acceptance criteria

- [ ] GIVEN a git tag `v*` WHEN CI is green THEN a GitHub Release is created with macOS Apple Silicon `.dmg` (or `.app` in dmg) and Windows x64 installer from `apps/desktop` `bundle.targets`
- [ ] GIVEN missing Apple/Windows signing secrets THEN the release job still publishes unsigned `.dmg` / `.exe` as Latest (operator chose not to register for certs)
- [ ] GIVEN the Tauri release build THEN `VITE_API_URL` and `VITE_WEB_URL` are required and must be the Fly/Pages HTTPS origins (not localhost)
- [ ] GIVEN `/app` dashboard **and a session** THEN platform-aware download buttons work; no dead “download the overlay” copy
- [ ] GIVEN landing **logged out** THEN no installer URL (story 4); CTA is sign-in — do not claim App Store
- [ ] GIVEN CI on pull requests THEN Tauri still **builds** (unsigned is OK for PR proof) but does not attach to Latest
- [ ] GIVEN Intel Mac / Linux THEN we do not advertise a download we do not produce

## Existing context

- Paths: `.github/workflows/ci.yml` `tauri` job; `apps/desktop/src-tauri/tauri.conf.json`; `apps/web/src/pages/DashboardPage.tsx`; `apps/web/src/pages/LandingPage.tsx`
- Pattern: `deploy.yml` uses Environment `production`; Environment `release` holds `API_URL` / `WEB_URL` only for v1
- Frodo `github-workflows` only for Aniki YAML ideas — no gp-nova workflows

## Out of scope

App Store, Microsoft Store, Linux, auto-updater, Intel macOS (unless Solve adds a matrix row).

## Risks

Unsigned installers are easier to spoof. Mitigation: session-gated download metadata, pin actions, only advertise GitHub Release assets for this repo. Document Gatekeeper/SmartScreen bypass.

## Definition of done

- [ ] Criteria met
- [ ] Manual check: download Latest on a clean Mac/Windows VM or second machine; Gatekeeper/SmartScreen notes in Solve
- [ ] README desktop section updated from “CI artifacts only” to Releases
