# Aniki — Implementation Plan

Last updated: 2026-09-12

This document tracks milestone progress and what to pick up next. Use it as the handoff artifact when resuming work.

## Product summary

Cloud-first interview assistant: web dashboard for sessions/resumes/billing, desktop stealth overlay for live transcription + LLM coaching during interviews.

## Milestone status

| Milestone | Scope | Status | Notes |
|-----------|--------|--------|-------|
| **M1** | Monorepo scaffold, API, web, desktop shell, Docker | ✅ Done | Axum API, React web, Tauri 2 desktop, shared packages |
| **M2** | Audio capture, Speechmatics STT, overlay transcript | ✅ Done | Mic + **system audio** (ScreenCaptureKit / WASAPI), dual-channel STT |
| **M3** | RAG + LLM streaming answers | ✅ Done | pgvector embeddings, Redis cache, SSE answer stream |
| **M4** | Stealth overlay (hide, click-through, hotkeys) | ✅ Done | NSPanel, global shortcuts, content protection, **tray Quit** |
| **M5** | Web polish (sessions, notes, billing UI) | ✅ Done | Sessions page, smoke test script, billing stub |
| **M5b** | Screen OCR | ✅ Done (macOS) | Vision OCR on main display when session has `enable_screen_ocr` |

## Desktop overlay (macOS)

- **All Spaces / fullscreen apps**: `NSPanel` with `CanJoinAllSpaces` + `FullScreenAuxiliary`.
- **Collapse**: `⌘⇧H` resizes to the pebble icon (process stays visible).
- **Stealth hide**: red × / menu **Stealth hide** uses `order_out`; restore from the tray. Process stays alive.
- **Quit**: menu bar tray → **Quit Aniki** (Accessory app has no Dock icon).
- **Click-through**: `⌘⇧C` or tray item.
- **System audio**: ScreenCaptureKit loopback on the interviewer channel. Requires Screen Recording permission.

### Desktop hotkeys

| Shortcut | Action |
|----------|--------|
| `⌘⇧H` | Collapse to pebble / expand |
| `⌘⇧C` | Toggle click-through |

## Local dev quick start

```bash
unset DOCKER_HOST   # if leftover Rancher Desktop socket
docker compose up -d postgres migrate redis
pnpm install
cargo run -p aniki-api          # terminal 1
pnpm dev:web                    # terminal 2
pnpm dev:desktop                # terminal 3 (full restart after Rust changes)
```

API smoke test: `./scripts/smoke-test.sh`

Auth: magic link URL in API logs → web Sessions page → copy token + session ID → desktop overlay.

## Agent workflow

Product work runs **Discover → Plan → Solve → Build → Operate**. Skills: `.cursor/skills/aniki-*/`. Invoke via [`AGENTS.md`](../AGENTS.md). Do not use GP FSA / Jira / SAM / MFE.

## Next implementation priorities

### P0 — Production readiness

1. ~~Screen OCR~~ — macOS Vision path is live; Windows still unsupported.
2. ~~System audio capture~~ — macOS ScreenCaptureKit; Windows WASAPI loopback (compile-check on Windows CI).
3. ~~Quit path~~ — tray menu with Show / click-through / Quit.
4. **Window position persistence** — `tauri-plugin-window-state` is wired; verify restore after Hide.

### P1 — Quality & platform

5. **Windows overlay parity** — verify stealth + WASAPI on a Windows machine.
6. **Speechmatics production path** — set `SPEECHMATICS_API_KEY`, JWT refresh on long sessions.
7. **E2E tests** — Playwright for web; desktop overlay checklist.
8. **Desktop keychain token** — stop pasting access tokens from localStorage.

### P2 — Product polish

9. Speaker-swap UI when channel diarization is wrong.
10. Billing — Stripe (Stage B).
11. Presigned R2 uploads (resume bytes currently POST through the API).
12. Magic-link email (Resend) instead of API log only.

## Known limitations

- macOS 15+ may degrade screen-capture exclusion (`contentProtected`).
- ScreenCaptureKit v9 was skipped (needs full Xcode Swift); using objc `screencapturekit` 0.3.x.
- Windows OCR is not implemented.
- Dev STT mode emits demo transcripts when `SPEECHMATICS_API_KEY` is unset.

## Key files (resume here)

| Area | Path |
|------|------|
| System audio | `crates/audio-core/src/capture/system/` |
| Desktop commands | `apps/desktop/src-tauri/src/commands.rs` |
| Tray | `apps/desktop/src-tauri/src/tray.rs` |
| OCR | `apps/desktop/src-tauri/src/ocr.rs` |
| Overlay UI | `apps/desktop/src/OverlayApp.tsx` |

## Suggested next session

1. Run `aniki-discover` for slug `rag-answer` (per-question RAG) then chain Plan → Solve → Build.
2. Dry-run a live session with Screen Recording enabled: interviewer on system audio, candidate on mic.
3. Confirm tray Quit actually exits (ALLOW_EXIT flag).
4. Windows WASAPI + overlay smoke test.
