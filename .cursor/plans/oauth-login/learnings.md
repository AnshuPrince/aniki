# Learnings — oauth-login

## 2026-09-13 — discover

- User leaned OAuth-only; counter is Google testing-mode cap and no-Gmail users. Slice still picks **Google-only UI (A+F1)** and leaves magic-link API in place unused.
- GitHub OAuth is a poor primary for interview candidates. Hosted IdPs (Clerk/Auth0) add a second JWT story we do not want.
- Resend stays P2; OAuth unblocks production sign-in without mail.

## 2026-09-13 — plan

- Three stories: API OIDC, Google-only web UI, secrets + `/privacy` for consent.
- Must Solve before Build. Magic-link API stays; UI does not use it.

## 2026-09-13 — solve

- SPA callback on Pages; API returns `authorization_url` JSON (not XHR 302).
- Redis holds PKCE verifier keyed by `state`. Identity: `google_sub` then email merge.

## 2026-09-13 — build

- Redis one-time `state` + PKCE; Google userinfo `email_verified`; `users.google_sub`.
- Login is Google-only; `/privacy` for consent. Magic-link API left in place.
- Operator must `fly secrets set GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET` and apply `003_google_sub.sql`.


