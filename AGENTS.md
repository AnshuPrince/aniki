# Aniki agents

Personal interview-copilot monorepo (Axum + React + Tauri). Zone workflow mirrors **Frodo philosophy** (orchestrator routes, specialists execute, human gates, written handoff). It is **not** a G-P factory workspace.

Load the matching project skill under `.cursor/skills/aniki-<zone>/SKILL.md`. Index: [`.cursor/skills/SKILLS.md`](.cursor/skills/SKILLS.md).

## Zones

| Zone | Skill | Invoke when |
|------|--------|-------------|
| Discover | `aniki-discover` | Problem, evidence, UX brief, “what should we build” |
| Plan | `aniki-plan` | Epic/stories, acceptance criteria |
| Solve | `aniki-solve` | ADR, API sketch, threat model for this slice |
| Build | `aniki-build` | Implement a Solve handoff |
| Operate | `aniki-operate` | Health, incidents, playbooks, test strategy |
| Handoff | `aniki-handoff` | User said continue/chain to the next zone |

## Rules

1. Do not skip a zone on trust-boundary work (auth, billing, OCR in prompts, uploads).
2. Do not start Build until a Solve handoff card exists (`status: open`) **or** the user explicitly says the change is Plan-only (docs/hotkeys, copy).
3. Never invent customer quotes. Ground Discover in this repo, `docs/IMPLEMENTATION_PLAN.md`, and the user’s words.
4. Do not call Frodo `primary-solve-agent` / `primary-build-agent` full orchestration.

## Example

```text
aniki-discover — slug: rag-answer
Brief: live answers must use the resume. Ground in apps/api/src/services/rag.rs
```

Then `aniki-plan` → `aniki-solve` → user approves → `aniki-build`.
