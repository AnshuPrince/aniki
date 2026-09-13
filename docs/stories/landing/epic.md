# Public landing page — Enhancement

**Slug:** landing  
**Epic Goal:** Give Aniki a public homepage inspired by Parakeet’s IA (hero, features, privacy, pricing, CTA) without cloning their copy or claiming unshipped products.

## Existing system context
React Vite app; `/` is auth-gated `DashboardLayout`. Login redirects home. Overlay already has Parakeet-like chrome.

## Enhancement details
Add `LandingPage` public route. Move authenticated shell to `/app` (and children `/app/resumes` etc.). Update login/verify redirects. Honest copy via Frodo `ux-copy` principles: no fake social proof.

## Stories
1. Routing: public `/`, app under `/app`, auth redirects
2. Landing sections (hero → how it works → features → privacy → pricing → FAQ) + nav/footer
3. CTAs and signed-in skip

## Compatibility
- Magic-link still `/auth/verify`
- Desktop `WEB_URL` should open landing or sessions; document `/app/sessions` if we move Sessions
- `APP_URL` remains site origin (landing)

## Risks
Broken bookmarks to `/resumes`. Mitigation: redirects from old paths to `/app/...`.

## Definition of done
- [ ] Logged-out visitor sees landing
- [ ] Logged-in visitor reaches dashboard without the marketing page as a trap
- [ ] No invented user counts or Parakeet verbatim copy
