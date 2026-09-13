# Discovery brief — oauth-login

## Problem
Production sign-in tells people to check email. The API never sends mail: it stores a one-time token and logs `{APP_URL}/auth/verify?token=…`. Unauthenticated visitors on Pages cannot complete login without Fly logs. Magic-link email (Resend) is still P2 in the implementation plan; OAuth can unblock sign-in without a mail vendor.

## Evidence
- Code / docs (paths):
  - `apps/api/src/services/auth.rs` — insert `magic_link_tokens`, log link, `get_or_create_user` by email, HMAC session JWT
  - `apps/api/src/routes/auth.rs` — `POST /auth/magic-link`, `POST /auth/verify`, `GET /auth/me`
  - `apps/api/migrations/001_init.sql` — `users.email` unique; no OAuth subject column
  - `packages/shared/src/api/client.ts` — `requestMagicLink` / `verifyToken`
  - `apps/web/src/pages/LoginPage.tsx` — email form; copy says check inbox
  - `apps/web/src/pages/VerifyPage.tsx` — `?token=` then `/app`
  - `docs/IMPLEMENTATION_PLAN.md` item 12 — “Magic-link email (Resend) instead of API log only”
- User statements (verbatim):
  - “Plan for google auth login or any other free oauth provider with large userbase coverage”
  - “Discover multiple ideas and we'll choose the best one”
  - “leaning towards oauth only, you counter it if you feel before we decide”
  - Implement the attached Discover plan (recommended **A** + **F1**)

## Choice (this slice)

| Decision | Pick | Why |
|----------|------|-----|
| Provider | **A — Google OIDC in Axum** | Largest free consumer coverage; `$0` at Aniki scale; no extra IdP vendor; fits SPA + Axum |
| Login UI | **F1 — Google button only** | Magic link is unusable in production until Resend; showing it is worse UX |
| Magic-link API | **Keep dormant** | Do not delete `POST /auth/magic-link` or `magic_link_tokens`; Plan/Build hide the email field. Email can return without a schema rewrite |
| Not chosen | B Microsoft, C GitHub, D Clerk/Auth0/Supabase, E Apple | B later if Outlook-only users appear; C wrong audience; D extra trust-boundary vendor; E not free |

Counter to OAuth-only (recorded, not blocking v1): no Google account, Workspace blocks, Google **testing mode 100 testers** until app verification. Accept testing cap until landing + privacy policy support verification.

## Users and context
Visitor on `https://aniki-web.pages.dev` (and localhost:3000). Operator already using dashboard JWT. Desktop overlay still pastes the web access token (unchanged in this slice). Platforms: macOS/Windows overlay; OAuth is **web-first**.

## In scope
- Google authorization-code + PKCE; scopes `openid email profile` only
- Redirect: `APP_URL/auth/oauth/callback` (Pages + local)
- API start + callback; require Google `email_verified`; `get_or_create_user(email)` then existing `AuthResponse` JWT
- Fly secrets `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET` (not Pages, not GitHub except if a local `.env.example` documents names)
- Login page: Continue with Google; no email form
- CSRF `state` (and PKCE `code_verifier`) stored server-side (short Redis TTL or signed cookie)

## Out of scope
- Sending magic-link email (Resend)
- Microsoft / GitHub / Apple buttons
- Hosted auth (Clerk, Auth0, Supabase Auth)
- Tauri system-browser OAuth or `aniki://` deep links
- Linking multiple IdPs to one user; v1 identity = verified Google email
- Changing desktop login UX
- Google app verification paperwork beyond documenting the need (privacy URL on landing)

## Examples (example mapping)
- Rule: Only verified Google emails become users.
- Example: Consent with `email_verified=true` → same JWT as today’s verify path → `/app` with signup credits if new.
- Example: Unverified Google email → 401, no user row.
- Rule: Login UI has no email field.
- Example: Existing `/auth/magic-link` still returns 200 if called, but the web app does not call it.
- Rule: Desktop still copies access token from Sessions after web login.
- Question (accepted): Testing-mode 100-user cap until Google verifies.

## Success
A logged-out visitor completes Google consent on Pages and lands on `/app` with a session JWT. Fly logs are not required. Overlay paste still works. Magic-link routes remain but are unused by the UI.

## Open questions
- Exact Google Cloud project + consent app name (operator click-ops).
- Whether to persist `google_sub` on `users` in the first migration (Solve should prefer yes for account stability if email changes).
- Privacy policy URL required for Google verification (landing exists; dedicated `/privacy` may be a tiny Plan story or out of this slug).
