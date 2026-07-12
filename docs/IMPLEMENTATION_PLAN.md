# Aniki — Implementation Plan

Last updated: 2026-07-12

This document tracks milestone progress and what to pick up next. Use it as the handoff artifact when resuming work.

## Product summary

Cloud-first interview assistant: web dashboard for sessions/resumes/billing, desktop stealth overlay for live transcription + LLM coaching during interviews.

## Milestone status

| Milestone | Scope | Status | Notes |
|-----------|--------|--------|-------|
| **M1** | Monorepo scaffold, API, web, desktop shell, Docker | ✅ Done | Axum API, React web, Tauri 2 desktop, shared packages |
| **M2** | Audio capture, Speechmatics STT, overlay transcript | ✅ Done | `audio-core`, desktop STT client, dev STT mode without API key |
| **M3** | RAG + LLM streaming answers | ✅ Done | pgvector embeddings, Redis cache, SSE answer stream |
| **M4** | Stealth overlay (hide, click-through, hotkeys) | ✅ Done | NSPanel on macOS, global shortcuts, content protection |
| **M5** | Web polish (sessions, notes, billing UI) | ✅ Done | Sessions page, smoke test script, billing fixes |

## Desktop overlay (macOS) — recent fixes

These were blocking real usage and are implemented on `master`:

- **All Spaces / fullscreen apps**: overlay uses `tauri-nspanel` (`NSPanel`) with `CanJoinAllSpaces` + `FullScreenAuxiliary` (plain `NSWindow` is insufficient).
- **Draggable**: title bar grip + `startDragging` + Tauri window capabilities.
- **Click-through recovery**: `⌘⇧C` toggles click-through; `⌘⇧H` show restores interactivity.
- **Hide without quit**: Hide / `⌘⇧H` hides panel only; app stays alive (`prevent_exit`), session + mic continue.

### Desktop hotkeys

| Shortcut | Action |
|----------|--------|
| `⌘⇧H` | Hide overlay / show overlay (on top) |
| `⌘⇧C` | Toggle click-through |

## Local dev quick start

```bash
docker compose up -d postgres migrate redis
pnpm install
cargo run -p aniki-api          # terminal 1
pnpm dev:web                    # terminal 2
pnpm dev:desktop                # terminal 3 (full restart after Rust changes)
```

API smoke test: `./scripts/smoke-test.sh`

Auth: magic link URL in API logs → web Sessions page → copy token + session ID → desktop overlay.

## Next implementation priorities

### P0 — Production readiness

1. **Screen OCR** — `capture_screen_ocr` is a stub; wire Tesseract or platform capture for coding interviews.
2. **System audio capture** — `audio-core` system capture path exists but is not fully integrated in desktop dual-channel STT.
3. **Quit path for desktop** — app currently `prevent_exit` for overlay lifecycle; add tray menu with Quit (Accessory app has no dock icon).
4. **Window position persistence** — overlay resets position on restart; optional `tauri-plugin-window-state`.

### P1 — Quality & platform

5. **Windows overlay parity** — NSPanel is macOS-only; verify/improve Windows stealth (`stealth-window` crate).
6. **Speechmatics production path** — set `SPEECHMATICS_API_KEY`, verify JWT refresh + reconnect on long sessions.
7. **E2E tests** — extend `scripts/smoke-test.sh` or add Playwright for web; desktop overlay manual test checklist.
8. **CI** — ensure `cargo clippy`, desktop Tauri build on macOS runner (if available).

### P2 — Product polish

9. **AI session notes** — verify post-session notes generation and polling on web Sessions page.
10. **Billing** — Stripe or manual credits flow beyond ledger UI.
11. **Resume ingest** — PDF upload + chunking quality; embedding refresh on resume edit.
12. **Question detection** — tune client heuristic + `confirmQuestion` API; reduce false positives.

## Known limitations

- macOS 15+ may degrade screen-capture exclusion (`contentProtected`); stealth warning shown in overlay.
- Overlay may not appear above some system fullscreen surfaces (e.g. native video players).
- Desktop `Accessory` activation policy hides dock icon by design.
- Dev STT mode emits demo transcripts when `SPEECHMATICS_API_KEY` is unset; mic still active for level meter.

## Key files (resume here)

| Area | Path |
|------|------|
| API routes | `apps/api/src/routes/` |
| LLM / RAG | `apps/api/src/services/llm.rs`, `rag.rs` |
| Desktop commands | `apps/desktop/src-tauri/src/commands.rs` |
| macOS NSPanel | `apps/desktop/src-tauri/src/macos_overlay.rs` |
| Overlay UI | `apps/desktop/src/OverlayApp.tsx` |
| Audio pipeline | `crates/audio-core/` |
| Web sessions | `apps/web/src/pages/SessionsPage.tsx` |

## Suggested next session

1. Add system tray with **Show overlay** / **Quit** on macOS.
2. Implement real screen OCR for `enable_screen_ocr` sessions.
3. Run full interview dry-run: web session → desktop join → STT → question → streamed answer → finalize → notes on web.
