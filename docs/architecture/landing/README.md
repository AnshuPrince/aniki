# Architecture — landing

| Artifact | Path |
|----------|------|
| ADR | [adr.md](adr.md) |
| Routes | [api.md](api.md) (web routes only) |

No LLM/OCR change. No analytics in v1.

## Touches (Build)

- `apps/web/src/App.tsx`
- `apps/web/src/pages/LandingPage.tsx` (new) + optional `components/landing/*`
- `apps/web/src/pages/LoginPage.tsx`, `VerifyPage.tsx`
- `apps/web/src/components/DashboardLayout.tsx`
- `README.md` URLs
- Desktop `WEB_URL` links if they assume `/sessions` at root
