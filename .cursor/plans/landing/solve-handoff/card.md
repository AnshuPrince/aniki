---
status: done
from_zone: solve
to_zone: build
slug: landing
created: 2026-09-12
---

# Handoff: solve → build

## Artifacts
- docs/architecture/landing/README.md
- docs/architecture/landing/adr.md
- docs/architecture/landing/api.md
- docs/stories/landing/story-01.md
- docs/stories/landing/story-02.md
- docs/stories/landing/story-03.md

## Touches (Build only)
- apps/web/src/App.tsx
- apps/web/src/pages/LandingPage.tsx
- apps/web/src/pages/LoginPage.tsx
- apps/web/src/pages/VerifyPage.tsx
- apps/web/src/components/DashboardLayout.tsx
- README.md
- apps/desktop overlay WEB_URL targets if they assume /sessions at root

## Notes
Parakeet is IA reference only. No fake metrics. No analytics.

## Gate
Approved with “build”; completed 2026-09-12.
