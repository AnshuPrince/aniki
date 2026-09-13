# App routes under /app; / is public

**Slug:** landing  
**Story:** 1 of 3

## User story

As a visitor, I want the site root to be public so that I can read what Aniki is before signing in.

## Acceptance criteria

- [ ] GIVEN no session WHEN I GET `/` THEN I see the landing page (not a login redirect loop)
- [ ] GIVEN no session WHEN I GET `/app` or `/app/sessions` THEN I am sent to `/login` with return path optional
- [ ] GIVEN a session WHEN I GET `/` THEN I am sent to `/app` (dashboard)
- [ ] GIVEN a session WHEN I finish login/verify THEN I go to `/app` not `/`
- [ ] GIVEN old URLs `/resumes`, `/sessions`, `/billing` THEN they redirect to `/app/...`
- [ ] GIVEN `DashboardLayout` nav THEN links use `/app/...`

## Existing context

- Paths: `apps/web/src/App.tsx`, `LoginPage.tsx`, `VerifyPage.tsx`, `DashboardLayout.tsx`

## Out of scope

Landing visual design (story 2).

## Risks

Desktop/docs that link `http://localhost:3000/` expecting Sessions — README should mention `/app/sessions`.

## Definition of done

- [ ] Criteria met
- [ ] README auth flow URLs updated
