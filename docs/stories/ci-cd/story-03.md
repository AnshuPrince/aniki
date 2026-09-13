# Deploy web to Cloudflare Pages

**Slug:** ci-cd  
**Story:** 3 of 3

## User story

As the operator, I want the dashboard built with the production API URL and published to Cloudflare Pages so that login and “copy access token” work against Fly, not localhost.

## Acceptance criteria

- [ ] GIVEN CD THEN `pnpm --filter @aniki/web build` with `VITE_API_URL=https://<fly-app>.fly.dev` (or custom API domain)
- [ ] GIVEN Pages THEN the deploy uses the same GitHub Environment `production`
- [ ] GIVEN API THEN `APP_URL` is the Pages origin (magic-link redirects); CORS allowlist includes that origin plus Tauri (`tauri://localhost`, `http://localhost:1420` for local overlay)
- [ ] GIVEN desktop THEN `VITE_API_URL` / `VITE_WEB_URL` for release builds are documented (Tauri job may stay CI-only; store upload out of scope)
- [ ] GIVEN R2 not in this story THEN Pages deploy does not require R2; architecture notes R2 as a **later** Fly secret + API env when the upload slice lands

## Existing context

- Paths: `apps/web` Vite; `apps/api/src/main.rs` CORS; `apps/desktop` `WEB_URL` default localhost:3000

## Out of scope

Custom DNS unless chosen in Solve; Wrangler config invented at Build.

## Risks

SPA talking to wrong API if `VITE_*` baked at build time. Mitigation: one Pages project per environment (`preview` vs `production`).

## Definition of done

- [ ] CORS + env matrix in architecture
- [ ] Rollback: Cloudflare Pages previous deployment
