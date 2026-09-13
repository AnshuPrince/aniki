---
name: aniki-discover
description: Writes a discovery brief for an Aniki product slice from repo evidence (no fake users). Use when the user asks to discover, brief a problem, example-map, or start a zone pipeline before stories.
---

# aniki-discover

Confirm `slug` (kebab-case). Read [../aniki-shared.md](../aniki-shared.md).

## Do

1. Ground in `docs/IMPLEMENTATION_PLAN.md`, relevant `apps/` / `crates/` code, and the user’s words. Optional Frodo: `example-mapping`, `ux-copy`, `flowchart-generator`, `facilitate-brainstorming`.
2. Write `docs/discovery/<slug>/brief.md` using [../../plans/_templates/discovery-brief.md](../../plans/_templates/discovery-brief.md).
3. Create `.cursor/plans/<slug>/` if missing; copy learnings stub; write `discover-handoff/` card (`to_zone: plan`, `status: open`) from [../../plans/_templates/handoff-card.md](../../plans/_templates/handoff-card.md).
4. Stop. Offer `aniki-plan` / `aniki-handoff`. Do not write stories, ADRs, or product code.

## Do not

Invent quotes, call Vocal/Genie, or skip to Build.
