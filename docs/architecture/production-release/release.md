# Desktop release job

New workflow `.github/workflows/release.yml`.

## Trigger

```yaml
on:
  push:
    tags: ["v*.*.*"]
```

`permissions: contents: write` (create Release + upload assets) and `actions: read` (gate job lists the CI workflow for the tag SHA). `environment: release`.

## Jobs (sketch)

1. **gate** — `ubuntu-latest`: assert GitHub Actions check `CI` succeeded for that SHA; require `API_URL` and `WEB_URL`; fail if either is empty or contains `localhost`. Signing secrets are **not** required.
2. **tauri-macos** — `macos-latest`: `pnpm install --frozen-lockfile`, then `tauri-action` with `tauriScript: pnpm tauri` for `aarch64-apple-darwin`, upload unsigned `.dmg`.
3. **tauri-windows** — `windows-latest`: same install, `x86_64-pc-windows-msvc`, upload unsigned NSIS `.exe`.

Env for Tauri:

```text
VITE_API_URL=${{ vars.API_URL }}   # https://aniki-api.fly.dev
VITE_WEB_URL=${{ vars.WEB_URL }}   # https://aniki-web.pages.dev
```

Fail if either is empty or contains `localhost`. Do not set `APPLE_*` or `WINDOWS_CERTIFICATE*` so `tauri-action` does not attempt signing.

## Asset names (convention for the dashboard)

- `Aniki_{version}_aarch64.dmg`
- `Aniki_{version}_x64-setup.exe` (or Tauri’s default NSIS name — Build must pin names in `tauri.conf.json` `bundle` so the SPA matcher is stable)

## GitHub Environment `release`

**Variables:** `API_URL`, `WEB_URL` (duplicate of `production` vars is OK; do not read Fly `DATABASE_URL` here).

No Apple/Windows secrets for v1.

## Operator steps for v0.1.0

1. Set Environment `release` variables `API_URL` and `WEB_URL`.
2. `git tag v0.1.0 && git push origin v0.1.0` from a SHA that already has green CI.
3. Wait for `release.yml`; download the `.dmg` / `.exe` from the GitHub Release.
4. Hard-refresh Pages; dashboard Download buttons resolve `latest`.
