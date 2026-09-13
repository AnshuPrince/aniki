# Landing sections (Parakeet IA, Aniki truth)

**Slug:** landing  
**Story:** 2 of 3

## User story

As a visitor, I want a single long page that explains the product the way [Parakeet’s site](https://www.parakeet-ai.com/) is organized so that I can decide whether to sign in.

## Acceptance criteria

- [ ] GIVEN `/` THEN sticky nav: Features, Privacy, Pricing, FAQ, Sign in, Get started
- [ ] GIVEN hero THEN one headline (real-time interview overlay), one subhead, primary CTA Get started → `/login`, secondary Sign in
- [ ] GIVEN “How it works” THEN three steps we actually do: upload resume on web → copy access token → desktop overlay (pebble, live Answer/Screenshot)
- [ ] GIVEN features THEN only shipped: dual-channel STT, resume-backed answers, macOS OCR, collapse pebble, click-through, session notes — **no** question bank / mocks / headshots
- [ ] GIVEN privacy THEN honest stealth (no Dock icon, click-through, content protection) without claiming “undetectable on every platform” unless we have checks
- [ ] GIVEN pricing THEN credits copy matching dashboard (0.5 credit per session) and “subscription later”; CTA to sign in, not fake checkout
- [ ] GIVEN FAQ THEN 4–6 questions we can answer (languages: English first; coding: OCR macOS; desktop macOS/Windows; web is dashboard not live call)
- [ ] GIVEN visual THEN dark glass consistent with overlay; optional static mock of stacked live UI (no fake company logos as if they are customers)
- [ ] GIVEN copy THEN original Aniki wording; do not paste Parakeet headlines or testimonials

## Existing context

- Overlay screens as visual reference; `@aniki/ui` Button
- Frodo `ux-copy` for labels/CTAs

## Out of scope

CMS, blog, i18n, video embeds required for v1 (optional later).

## Risks

Over-claiming stealth. Mitigation: “designed to stay out of screen share on macOS; verify before a real interview.”

## Definition of done

- [ ] Desktop + mobile viewport of `/` looks coherent
- [ ] Accessibility: heading order, skip to content, contrast
