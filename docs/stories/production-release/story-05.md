# Logged-in users can still open the landing page

**Slug:** production-release  
**Story:** 5 of 5

## User story

As a signed-in candidate, I want to read the public landing page without being forced into the dashboard so that I can share the marketing URL and still get back to `/app`.

## Acceptance criteria

- [ ] GIVEN a session WHEN I GET `/` THEN I see `LandingPage` (not a redirect to `/app`)
- [ ] GIVEN a session WHEN I view landing chrome THEN “Sign in / Get started” become **Open app** (link `/app`) and optional Download (story 4)
- [ ] GIVEN no session WHEN I GET `/` THEN landing is unchanged (public marketing)
- [ ] GIVEN a session WHEN I GET `/login` THEN I may still skip to `/app` (existing `LoginPage` redirect is OK)
- [ ] GIVEN a session WHEN I finish OAuth THEN I still land on `/app` (callback unchanged)
- [ ] GIVEN `DashboardLayout` THEN a way back to `/` (logo or “Home”)
- [ ] GIVEN `docs/stories/landing/story-01.md` AC “session on `/` → `/app`” THEN this story **wins**

## Existing context

- Paths: `apps/web/src/App.tsx` `LandingRoute` (`if (user) return <Navigate to="/app" />`)
- Pattern: `ProtectedRoute` stays on `/app/*` only

## Out of scope

New marketing sections. Changing OAuth redirect to `/`.

## Risks

Users bookmark `/` and think they are logged out. Mitigation: signed-in chrome (Open app, avatar/email or “Dashboard”).

## Definition of done

- [ ] Criteria met
- [ ] Manual: sign in → visit `/` → still landing → Open app → dashboard
