# Authenticated desktop download only

**Slug:** production-release  
**Story:** 4 of 5

## User story

As the operator, I want installer links to appear only after Google sign-in so that casual visitors cannot grab the overlay from the marketing page.

## Acceptance criteria

- [ ] GIVEN no session WHEN I view `/` THEN I do not see Mac/Windows installer URLs or a working Download button (Sign in / Get started only)
- [ ] GIVEN no session WHEN I `GET /desktop/latest` (or equivalent) THEN **401**
- [ ] GIVEN a session WHEN I `GET /desktop/latest` THEN **200** JSON with platform asset URLs (or 404 if no GitHub Release yet)
- [ ] GIVEN a session WHEN I open `/app` THEN platform-aware Download for Mac / Windows uses that authenticated response — not a hardcoded public `github.com/.../releases` link in HTML
- [ ] GIVEN a session WHEN I open `/` (story 5) THEN I may also download from the landing chrome; still no anonymous download
- [ ] GIVEN the SPA THEN it does not call `api.github.com` with a token baked into Pages (server fetches Release metadata)

## Existing context

- Paths: `apps/web/src/pages/LandingPage.tsx`, `DashboardPage.tsx`, `apps/api/src/routes/`
- Pattern: JWT on `Authorization` like other `/app` APIs
- Supersedes Solve ADR decision 6 (browser → GitHub Releases API)

## Out of scope

Private GitHub repo / R2-only assets (public Release URLs remain guessable if the repo is public — document in README). Intel/Linux binaries.

## Risks

Security-through-UI only if assets stay on a public Release. Mitigation: authenticated API is the product gate; optional later: non-public assets.

## Definition of done

- [ ] Criteria met
- [ ] Manual: logged-out landing has no `.dmg`/`.exe` href; logged-in dashboard download works
- [ ] Architecture `api.md` lists `GET /desktop/latest`
