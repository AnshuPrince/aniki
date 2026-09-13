---
name: aniki-plan
description: Turns an Aniki discovery brief into an epic and 1–3 stories with acceptance criteria. Use when the user asks to plan, write stories, or slice a brief; after aniki-discover.
---

# aniki-plan

Confirm `slug`. Read `docs/discovery/<slug>/brief.md` (stop if missing). Read [../aniki-shared.md](../aniki-shared.md).

## Do

1. Optional Frodo: `create-epic`, `create-quick-story` or `create-story`, then `validate-story`. No Jira publish.
2. Write `docs/stories/<slug>/epic.md` and `story-01.md` (+ 02/03 if needed) from [../../plans/_templates/story.md](../../plans/_templates/story.md).
3. Close discover handoff (`status: done`). Open `plan-handoff/` → `to_zone: solve`.
4. Stop for human approval. Offer Solve.

Trust-boundary stories (auth, payments, OCR/LLM context, uploads) **must** go through Solve. Copy-only / README / hotkey docs may skip Solve if the user says so — note that on the card.
