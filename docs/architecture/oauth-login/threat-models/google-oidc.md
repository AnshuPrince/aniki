# Threat notes — Google OIDC

**Mode:** story. Sources: `docs/stories/oauth-login/story-01.md`–`03`.

## Data flow

Browser → Axum `start` → Redis (state, PKCE verifier) → Google authorize → Pages callback → Axum `callback` → Google token + userinfo → Neon `users` → HMAC JWT to browser localStorage.

## Classification

Google email and `sub` are identifiers. Client secret is Fly-only. JWT is the existing session secret.

## STRIDE

| Threat | Mitigation for Build |
|--------|----------------------|
| Spoofing | Require `email_verified`; one-time Redis `state`; PKCE |
| Tampering | `redirect_uri` from `APP_URL` only, never client-supplied |
| Repudiation | Log Google `sub` hash or suffix, not access tokens |
| Information disclosure | No client secret on Pages; do not log `code` or verifier |
| Denial of service | Redis TTL 10m; rate-limit start in Solve-later if abused (not v1) |
| Elevation | Do not expand scopes; do not admin-link accounts across emails |

## Out of scope

Google app verification. Account takeover via Google-account compromise (same as any IdP).
