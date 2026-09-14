---
status: done
from_zone: plan
to_zone: solve
slug: production-release
created: 2026-09-14
---

# Handoff: plan → solve

## Artifacts
- docs/stories/production-release/epic.md
- docs/stories/production-release/story-01.md
- docs/stories/production-release/story-02.md
- docs/stories/production-release/story-03.md

## Notes for the next zone
Trust-boundary work: signing certs in GitHub, OS credential store, resume delete, production fail-closed. Write ADR + threat notes. Do not invent App Store. Tag-based desktop release vs master CD for API/web.

Must decide: Apple notarization secret shape; Windows cert; Tauri plugin for keychain; Speechmatics fail vs boot-fail; Intel Mac in v1 (default no).

## Gate
Human must say continue/chain before Solve writes architecture.
