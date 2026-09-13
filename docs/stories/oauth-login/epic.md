# Google OAuth sign-in — Enhancement

**Slug:** oauth-login  
**Epic Goal:** Let visitors sign in on Pages with Google (OIDC in Axum) and receive the same session JWT as magic-link verify, without sending email.

## Existing system context
Email magic link is stored and logged only. Users are unique on `email`. Web JWT in localStorage; desktop pastes that token. `APP_URL` is `https://aniki-web.pages.dev`. Google client is a Web application with JS origins and redirects on localhost:3000 and that Pages host (`/auth/oauth/callback`).

## Enhancement details
Authorization code + PKCE. Scopes `openid email profile`. Require `email_verified`. Persist `google_sub` (Solve). Login UI is Google-only; keep `POST /auth/magic-link` unused. Must go through Solve (auth).

## Stories
1. API: start, callback, PKCE/state, user upsert, JWT
2. Web: Continue with Google, callback route, no email form
3. Secrets, README, public privacy URL for Google consent

## Compatibility
- `/auth/me`, overlay access-token paste unchanged
- Magic-link routes remain callable, not in the UI
- CORS already allows `APP_URL`

## Risks
Google testing-mode 100-user cap. Open redirect / CSRF if `state` is weak. Email-only identity if `google_sub` is skipped.

## Definition of done
- [ ] Logged-out visitor completes Google consent and reaches `/app`
- [ ] Unverified Google email is rejected
- [ ] Fly holds `GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET`
