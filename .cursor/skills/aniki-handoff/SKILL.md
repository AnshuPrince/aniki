---
name: aniki-handoff
description: Chains Aniki zones when the user says continue or chain. Use after a zone finishes, to absorb a handoff card, or to start the next zone without re-explaining context.
---

# aniki-handoff

Mirrors Frodo `continue-zone-handoff` without FSA drift scripts.

## Do

1. Confirm `slug`. Infer direction from user (`continue to plan|solve|build|operate`) or the oldest `status: open` card under `.cursor/plans/<slug>/`.
2. Preflight: required artifacts exist (brief / stories / architecture README / solve card `touches`).
3. Load the **next** zone skill and run it in this session **only if** the user opted in this turn (continue/chain).
4. Report: `[Zone handoff] <from> → <to>` and what you will write.

If both a `build-handoff` (→ solve) and a `solve-handoff` (→ build) are open, ask which to process.

Do not chain if the user only said “looks good” without continue.
