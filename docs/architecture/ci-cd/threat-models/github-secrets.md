# Threat notes — GitHub CD and cloud secrets

**Mode:** story. Sources: `docs/stories/ci-cd/story-01.md`–`03`.

## Data flow

GitHub Actions (OIDC or deploy token) → Fly API (push image, set nothing of DATABASE_URL in logs) → app reads Neon/Upstash. Pages build receives only `VITE_API_URL` (public). Desktop artifacts are unsigned CI builds.

## STRIDE

| Threat | Mitigation |
|--------|------------|
| Spoofing | GitHub Environment; OIDC audience bound to this repo |
| Tampering | Deploy SHA from successful `workflow_run.head_sha`, not floating `main` |
| Repudiation | Actions logs; Fly release history |
| Information disclosure | No `echo` of secrets; Fly runtime secrets; Pages has no DB URL |
| Denial of service | `concurrency` group; Fly auto-stop |
| Elevation | Least-privilege Fly token (deploy one app); Cloudflare token scoped to Pages |

## Out of scope

Provider account takeover; desktop code signing.
