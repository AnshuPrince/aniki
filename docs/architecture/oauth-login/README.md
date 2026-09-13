# Architecture — oauth-login

Approved for Build after human continue. Auth trust boundary.

| Artifact | Path |
|----------|------|
| ADR | [adr.md](adr.md) |
| API | [api.md](api.md) |
| Threat | [threat-models/google-oidc.md](threat-models/google-oidc.md) |
| Stories | [docs/stories/oauth-login/](../../stories/oauth-login/) |

## Touches

- `apps/api/src/config.rs`
- `apps/api/src/routes/mod.rs`
- `apps/api/src/routes/auth.rs`
- `apps/api/src/services/auth.rs` (or new `services/oauth.rs`)
- `apps/api/src/redis.rs` (state/PKCE TTL helpers)
- `apps/api/migrations/003_google_sub.sql`
- `apps/api/.env.example`
- `crates/domain/src/auth.rs`
- `packages/shared/src/types.ts`
- `packages/shared/src/api/client.ts`
- `apps/web/src/App.tsx`
- `apps/web/src/pages/LoginPage.tsx`
- `apps/web/src/pages/OAuthCallbackPage.tsx` (new)
- `apps/web/src/pages/PrivacyPage.tsx` (new)
- `apps/web/src/pages/LandingPage.tsx` (footer link)
- `apps/web/src/lib/auth.tsx`
- `README.md`
