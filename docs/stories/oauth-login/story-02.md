# Web: Continue with Google only

**Slug:** oauth-login  
**Story:** 2 of 3

## User story

As a visitor, I want a Google button on the login page so that I can reach the dashboard without checking email or API logs.

## Acceptance criteria

- [ ] GIVEN `/login` WHEN I am logged out THEN I see **Continue with Google** and no email field
- [ ] GIVEN I click that button WHEN start succeeds THEN the browser goes to Google (redirect or `window.location` to the authorize URL)
- [ ] GIVEN Google returns to `{APP_URL}/auth/oauth/callback?code&state` WHEN the page loads THEN the SPA posts code+state to the API, stores the JWT like verify does, and navigates to `/app`
- [ ] GIVEN Google or API error WHEN callback fails THEN I see a short error and a way back to `/login`
- [ ] GIVEN landing **Get started** / sign-in CTAs WHEN clicked THEN they still go to `/login` (now Google-only)
- [ ] GIVEN `/auth/verify` WHEN used with an old magic-link token THEN it may still work; the login page does not send people there

## Existing context

- Paths: `apps/web/src/pages/LoginPage.tsx`, `apps/web/src/lib/auth.tsx`, `apps/web/src/App.tsx`, `packages/shared/src/api/client.ts`, `apps/web/src/pages/LandingPage.tsx`
- Pattern to follow: `VerifyPage` + `api.verifyToken` storing `TOKEN_KEY`

## Out of scope

Desktop overlay OAuth. Changing Sessions “copy access token”.

## Risks

Popup blockers if we use a popup; prefer full-page redirect. `VITE_API_URL` must be the Fly origin in production.

## Definition of done

- [ ] Criteria met
- [ ] Manual: local Google redirect URIs already registered
