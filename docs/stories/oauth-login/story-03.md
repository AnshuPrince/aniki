# Secrets, docs, and privacy URL

**Slug:** oauth-login  
**Story:** 3 of 3

## User story

As an operator, I want documented Google secrets and a public privacy URL so that the OAuth client and consent screen can go live on Pages.

## Acceptance criteria

- [ ] GIVEN Fly WHEN secrets are set THEN `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET` are listed next to other API secrets in README (no values)
- [ ] GIVEN `apps/api/.env.example` THEN those names exist empty for local
- [ ] GIVEN a logged-out visitor WHEN they open a stable privacy URL on the site THEN they see a short, honest privacy page (no invented legal firm); Google consent can point at `{APP_URL}/privacy`
- [ ] GIVEN landing footer WHEN present THEN it can link to `/privacy`
- [ ] GIVEN GitHub/Pages WHEN deploying THEN they do **not** receive the Google client secret

## Existing context

- Paths: `README.md` Deployment section, `apps/api/.env.example`, `apps/web/src/pages/LandingPage.tsx`, `apps/web/src/App.tsx`
- Google console already has JS origins + redirects for localhost:3000 and `https://aniki-web.pages.dev/auth/oauth/callback`

## Out of scope

Submitting Google app verification. Microsoft. Resend.

## Risks

Thin privacy copy vs Google review — keep it accurate (what we store: email, resume, session). Testing-mode 100 users until verified.

## Definition of done

- [ ] Criteria met
- [ ] README mentions testing-mode cap
