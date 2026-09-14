# Threat notes — signing, Releases, keychain, resume delete

**Mode:** story. Sources: `docs/stories/production-release/story-01.md`–`03`.

## Data flow

Operator laptop → git tag → GitHub Actions `release` Environment (`API_URL` / `WEB_URL`) → unsigned binaries → GitHub Release (public) → candidate browser (Pages) → download → Tauri overlay → OS keychain (JWT) → Fly API (Bearer) → Neon (resume rows).

## STRIDE

| Threat | Mitigation |
|--------|------------|
| Spoofing (fake installer) | HTTPS GitHub Release for this repo; dashboard links only via `GET /desktop/latest`; residual: no Developer ID / Authenticode |
| Tampering | PR builds never published as Latest; tag workflow still requires green CI; residual: unsigned Latest is allowed |
| Repudiation | Release tags immutable; Actions logs; Fly boot errors in logs |
| Information disclosure | No PAT in Pages; no `echo` of PFX/Apple secrets; JWT in keychain not `localStorage`; delete resume 404 not 403; desktop asset list only with JWT (`GET /desktop/latest`) |
| Denial of service | GitHub API rate limit on latest — cache in SPA; Fly boot-fail is fail-closed not crash-loop without secrets (operator must set secrets before deploy) |
| Elevation | `DELETE` scoped `user_id`; Release `contents: write` only on tag workflow |

## Residual risk

- Unsigned macOS/Windows installers trigger Gatekeeper and SmartScreen; document Open anyway / Run anyway.
- Without code signing, a substituted binary is harder for candidates to detect — only download from in-app buttons after sign-in.
- GitHub unauthenticated `releases/latest` can 403 under rate limit — empty state, not a guessed URL.
- Public repo Release files remain downloadable if someone has the asset URL; the product gate is session UI + 401 on `/desktop/latest`.
- Keychain item is readable by any process the user runs as themselves (same as browser password managers at OS user scope).

## Out of scope

App Store review; supply-chain of `pnpm tauri`; physical theft of unlocked Mac.
