# API Google OIDC start and callback

**Slug:** oauth-login  
**Story:** 1 of 3

## User story

As a visitor, I want the API to complete Google sign-in so that I get an Aniki session without a magic-link email.

## Acceptance criteria

- [ ] GIVEN Google OAuth env is set WHEN I start login THEN the API issues a Google authorize URL with PKCE (`S256`), `state`, and redirect `{APP_URL}/auth/oauth/callback`
- [ ] GIVEN a valid `code` + `state` WHEN the SPA posts them to the API THEN Google token exchange succeeds, `email_verified` is true, user is `get_or_create`d, and the body is the same `AuthResponse` shape as `/auth/verify`
- [ ] GIVEN `email_verified` is false WHEN callback runs THEN 401 and no new `users` row
- [ ] GIVEN a reused or unknown `state` WHEN callback runs THEN 401
- [ ] GIVEN a new Google user WHEN upsert succeeds THEN they receive the same signup credits as magic-link signup
- [ ] GIVEN `POST /auth/magic-link` WHEN still called THEN it still works (log-only); this story does not remove it

## Existing context

- Paths: `apps/api/src/routes/auth.rs`, `apps/api/src/services/auth.rs`, `apps/api/src/config.rs`, `apps/api/src/redis.rs`, `crates/domain/src/auth.rs`
- Pattern to follow: `get_or_create_user` + `create_session_token`; Redis already on the API
- Solve should add `google_sub` (unique, nullable) rather than email-only if email can change

## Out of scope

Web button (story 2). Microsoft/Apple. Resend. Tauri OAuth.

## Risks

Client secret in Fly only. Redirect URI must match the Google console exactly. `state`/`verifier` TTL must be short (Redis).

## Definition of done

- [ ] Criteria met
- [ ] Unit or integration notes for PKCE/state failure paths
- [ ] `.env.example` names `GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET`
