# Discovery brief — landing

## Problem
`apps/web` has no marketing site. `/` is a **protected dashboard**. Unauthenticated visitors hit `/login` only. There is nothing like [ParakeetAI](https://www.parakeet-ai.com/) (hero, features, privacy, pricing, CTA) for Aniki.

## Evidence
- Code / docs (paths):
  - `apps/web/src/App.tsx` — `/` requires auth; `/login`, `/auth/verify`, `/resumes`, `/sessions`, `/billing`
  - `apps/web/src/pages/LoginPage.tsx` — if `user`, `navigate("/")`
  - `apps/web/src/pages/DashboardPage.tsx` — “Welcome back”, credits, desktop instructions
- User statements (verbatim):
  - “Plan to create a landing page as well, take reference from https://www.parakeet-ai.com/”
- Competitor **structure** (public site, not copied copy):
  - Nav: product, features, reviews, privacy, pricing
  - Hero + live-session visual (listening, Auto Answer, Answer / Screenshot / Chat)
  - Mid-call coding/OCR, transcription, LLM answers
  - Stealth/privacy claims (screen share, Dock, Activity Monitor)
  - Session list / prepare tools (question bank, mocks, resume maker, headshots)
  - Pricing: subscription vs credit packs
  - Desktop / browser / phone
  - FAQ

## Users and context
Visitor considering Aniki; signed-in operator who should skip the landing and land in the app.

## In scope
- Public marketing page at `/` (or `/` landing + `/app` for the dashboard)
- Sections Aniki can honestly support: hero, how it works (web token → overlay), live overlay, resume RAG, screen OCR (macOS), stealth overlay, credits, sign-in CTA
- Anchor nav + footer
- Logged-in users skip marketing → dashboard
- Reuse `@aniki/ui` + existing Tailwind; Parakeet is **reference layout/IA**, not a clone or Lyra/MFE

## Out of scope
- Invented metrics (Parakeet’s “1.5M+ people”, review scores)
- Question Bank, Mock Interviews, Resume Maker, Headshots
- Native mobile app, in-browser live call (Aniki live path is desktop)
- Pixel-perfect clone, Parakeet trademarks, verbatim testimonials/FAQ
- Stripe checkout on the landing (link to `/billing` after login is enough)
- Employer blocklist / compliance@ email unless we add a real address

## Examples (example mapping)
- Rule: Logged-out `/` is marketing; logged-in `/` or `/app` is the product.
- Example: Visit `/` → hero “Try free / Sign in” → `/login`.
- Rule: Feature bullets only for shipped behavior (pebble, click-through, dual audio, RAG, OCR macOS).
- Rule: Pricing describes **current** credits (0.5 / session) and says subscriptions are later.
- Question: Dark glass (overlay) vs light marketing (Parakeet landing is light)? Default: dark-adjacent to overlay, not a second brand.

## Success
A visitor understands Aniki, signs in, and existing dashboard routes still work. Pages deploy can use `/` as the public origin (`APP_URL`).

## Open questions
- Route: keep dashboard at `/` and put marketing at `/home` vs swap (`/` marketing, `/app` dashboard). **Recommend swap** so `APP_URL` is the landing.
