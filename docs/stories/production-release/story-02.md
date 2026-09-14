# Production overlay: URLs, token storage, permissions

**Slug:** production-release  
**Story:** 2 of 5

## User story

As a candidate, I want the installed overlay to talk to production and keep my session token off disk-in-the-clear so that a live interview works after I quit and reopen the app.

## Acceptance criteria

- [ ] GIVEN a release build THEN the overlay calls `VITE_API_URL` (Fly) and opens web help/token instructions at `VITE_WEB_URL` (Pages)
- [ ] GIVEN the user pastes a valid access token WHEN they quit and relaunch THEN the token is restored from the OS credential store (macOS Keychain / Windows Credential Manager), not `localStorage`
- [ ] GIVEN logout or token 401 THEN the stored credential is deleted
- [ ] GIVEN first launch on macOS THEN copy explains Microphone and Screen Recording (system audio + OCR) before a silent permission failure
- [ ] GIVEN first launch on Windows THEN copy explains microphone and (if used) loopback/capture permissions we actually need
- [ ] GIVEN auto-update THEN **not required** for v1; README points at downloading a newer GitHub Release
- [ ] GIVEN the leftover Magic link tab THEN it is hidden or labeled non-production (OAuth token paste remains the path)

## Existing context

- Paths: `apps/desktop/src/lib/api.ts`; `apps/desktop/src/overlay/constants.ts`; `apps/desktop/src/overlay/screens/LoginScreen.tsx`; `apps/desktop/src/overlay/context/OverlayContext.tsx`
- Pattern: Sessions page still copies JWT; overlay still accepts paste. Only storage backend changes.
- Trust boundary: OS keychain plugins (`tauri-plugin-stronghold` vs keyring vs custom) — Solve picks; do not invent in Plan.

## Out of scope

OAuth inside the overlay (no Google redirect in Tauri in this slice). Windows OCR. In-app updater.

## Risks

Keychain prompt every launch if access control is wrong; token theft if we keep `localStorage` as fallback. Mitigation: migrate once, then delete `localStorage` copy.

## Definition of done

- [ ] Criteria met
- [ ] Manual check: paste token → quit → reopen → still authenticated; 401 clears keychain
- [ ] Docs: README desktop install + permissions
