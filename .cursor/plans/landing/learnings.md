# Learnings — landing

## 2026-09-12 — discover / plan / solve

- `/` is currently the dashboard; landing requires moving the app to `/app`.
- Parakeet IA (hero, features, privacy, pricing, FAQ) without their copy, social proof, or Prepare tools we do not have.

## 2026-09-12 — build

- Public `/` now redirects signed-in users and renders the marketing page for visitors; the dashboard moved to `/app`.
- Old `/resumes`, `/sessions`, `/billing` URLs redirect to their `/app/*` equivalents.
- Landing includes original hero, live overlay preview, workflow, shipped features, qualified privacy claims, credits, FAQ, and CTA.
- Desktop dashboard/billing/session links now target `/app`.
- Shared ESLint config was missing its two React plugins; declared them so CI lint can start.
- Web lint (one pre-existing Fast Refresh warning), typecheck, production build, desktop typecheck, and diff check passed.
