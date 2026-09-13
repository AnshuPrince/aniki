# Discovery brief — ci-cd

## Problem
CI exists (`.github/workflows/ci.yml`: check/clippy/test, frontend, Tauri macOS/Windows). Nothing deploys. README names Fly, Cloudflare Pages, Neon, Upstash, and R2, but those accounts, secrets, and workflows are not in the repo. Manual `fly deploy` is the only documented path.

## Evidence
- Code / docs (paths):
  - `.github/workflows/ci.yml` — PR/push `master` only; no `environment:`, no deploy jobs
  - `apps/api/fly.toml` — app `aniki-api`, region `iad`, Dockerfile, `/health`, scale-to-zero
  - `apps/api/Dockerfile` — cargo-chef release binary
  - `apps/api/Dockerfile.migrate` — local compose only
  - `README.md` Deployment checklist
- User statements (verbatim):
  - “do we have CI/CD pipeline?”
  - “use frodo to plan the CI/CD deployment to each cloud provider”

## Users and context
Operator (you) shipping Aniki. Desktop users install Tauri builds; they do not hit Pages for the overlay.

## In scope
- GitHub Actions **CD** for API → Fly and web → Cloudflare Pages
- Document provision of Neon, Upstash, R2 and secret wiring
- CORS / `APP_URL` / `VITE_API_URL` for production hosts
- Migration story for Neon (not swallowed `.ok()` forever)
- OIDC or scoped tokens (no long-lived secrets in YAML)

## Out of scope
- Implementing workflows in this slice (Plan/Solve only)
- G-P `devx-shared-workflows`, SAM, AWS `repoTrusts`
- Stripe/Resend product features (secrets listed if needed later)
- Auto-publish desktop to App Store

## Examples (example mapping)
- Rule: CI stays the gate; CD runs only after CI succeeds on `master` (or a tag).
- Example: Failed clippy → no Fly deploy.
- Rule: Database URL never in the workflow file; Fly secrets / GitHub Environments only.
- Rule: Pages build injects `VITE_API_URL` to the Fly HTTPS hostname.
- Question: Deploy on every `master` push vs GitHub Release tags?

## Success
A written plan an operator can follow: provision each vendor, which GitHub Environment secrets exist, which workflow jobs deploy what, rollback.

## Open questions
- First prod hostname (Fly default `*.fly.dev` vs custom domain).
- Whether R2 CD is in v1 or waits for the R2 product slice.
