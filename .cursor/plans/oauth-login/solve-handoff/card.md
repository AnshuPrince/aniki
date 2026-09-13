---
status: done
from_zone: solve
to_zone: build
slug: oauth-login
created: 2026-09-13
---

# Handoff: solve → build

## Artifacts
- docs/architecture/oauth-login/README.md
- docs/architecture/oauth-login/adr.md
- docs/architecture/oauth-login/api.md
- docs/architecture/oauth-login/threat-models/google-oidc.md

## Touches (Build only)
- apps/api/src/config.rs
- apps/api/src/routes/mod.rs
- apps/api/src/routes/auth.rs
- apps/api/src/services/auth.rs
- apps/api/src/redis.rs
- apps/api/migrations/003_google_sub.sql
- apps/api/.env.example
- crates/domain/src/auth.rs
- packages/shared/src/types.ts
- packages/shared/src/api/client.ts
- apps/web/src/App.tsx
- apps/web/src/pages/LoginPage.tsx
- apps/web/src/pages/OAuthCallbackPage.tsx
- apps/web/src/pages/PrivacyPage.tsx
- apps/web/src/pages/LandingPage.tsx
- apps/web/src/lib/auth.tsx
- README.md

## Notes
Implemented. Set Fly Google secrets and run `003_google_sub.sql` on Neon. Google testing-mode 100-user cap until verification.

## Gate
Approved by the user with “build”; Build completed 2026-09-13.
