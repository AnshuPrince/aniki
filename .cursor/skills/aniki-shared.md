# Frodo skill substitutions

If a Frodo plugin skill is installed, **read its SKILL.md** and apply these replacements. If it is not installed, follow the Aniki skill’s local procedure only.

| Frodo | Aniki |
|-------|--------|
| `.core/core-config.yaml` | Skip, or use `docs/architecture` as output root |
| `devStoryLocation` / Jira | `docs/stories/<slug>/` |
| FSA / Event Catalog / SAM / MFE / Lyra / PBAC | Do not produce |
| `publish-story` / Atlassian | Do not run |
| Vocal / Genie | Do not query |
| Coralogix-first incident | `GET /health`, docker compose, API logs, then optional CX |
| `track_usage` MCP | Skip if unavailable |

Allowed Frodo skills: `facilitate-brainstorming`, `example-mapping`, `ux-copy`, `flowchart-generator`, `create-epic`, `create-quick-story`, `create-story`, `validate-story`, `write-adr`, `write-rfc`, `api-design-workshop`, `threat-model`, `review-pr-moscow`, `review-story`, `owasp-test-generator`, `build-test-strategy`, `github-workflows` (Aniki CI only), `production-incident-triage-and-fix` (adapted).
