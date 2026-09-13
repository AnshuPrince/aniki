# ADR: Google OIDC in Axum, SPA callback

**Slug:** oauth-login  
**Status:** accepted for Build  
**Date:** 2026-09-13

## Context
Magic-link email is not sent. Discover chose Google-only UI (A+F1). Users are unique on email. SPA on Pages cannot keep PKCE secrets; the API holds the client secret.

## Decisions

1. **Protocol:** Google OAuth 2 authorization code + PKCE S256. Scopes: `openid email profile` only.
2. **Start:** `GET /auth/oauth/google/start` returns JSON `{ "authorization_url": "..." }` (not a 302 from XHR). Server generates `state` + `code_verifier`, stores `{ verifier }` in Redis key `oauth:google:{state}` TTL **10 minutes**, then builds the Google URL with `redirect_uri={APP_URL}/auth/oauth/callback` (exact console match).
3. **Callback:** Google redirects the **browser** to Pages. SPA `POST /auth/oauth/google/callback` with `{ "code", "state" }`. API loads Redis, deletes the key (one-time), exchanges code at Google token endpoint, fetches userinfo (or ID token), requires `email_verified == true`.
4. **Identity:** Migration `users.google_sub TEXT UNIQUE`. Lookup order: `google_sub` → else `email` (attach `google_sub` if null) → else `get_or_create_user` + set `google_sub`. Signup credits unchanged.
5. **Session:** Same `AuthResponse` / HMAC JWT as `/auth/verify`.
6. **Magic link:** Leave routes; web stops calling them.
7. **Secrets:** Fly `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`. Redirect URI never taken from the query string (always `APP_URL` + fixed path).
8. **Privacy:** Public `GET /privacy` on the SPA for Google consent.

## Consequences
- Redis is required for login (already required for RAG).
- Google testing mode: 100 users until verification.
- Same-email magic-link accounts merge onto Google at first OAuth (acceptable v1).
