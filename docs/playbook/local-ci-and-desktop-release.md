# Local CI and desktop release

GitHub **does not** build Tauri. Overlay shipping is **Apple Silicon macOS only** for now. Linux **CI** still runs on GitHub so **Deploy** can key off a green `master` push.

## Checks on this Mac

```bash
docker compose up -d postgres redis
./scripts/ci-local.sh
```

Scripts exit if the host is not Darwin.

Optional overlay compile:

```bash
./scripts/ci-local.sh --desktop
```

## Publish the Mac installer (no Actions runners)

```bash
export VITE_API_URL=https://aniki-api.fly.dev
export VITE_WEB_URL=https://aniki-web.pages.dev
./scripts/release-desktop.sh v0.1.1
```

Uploads an unsigned `.dmg`. Windows NSIS is out of scope until we reopen that matrix. `GET /desktop/latest` still returns `windows_exe: null`; the site only shows **Download for Mac**.

Do **not** `git push origin v*` to trigger a hosted desktop build. That workflow is removed.

## What still uses GitHub Actions

| Workflow | When | Runners |
|----------|------|---------|
| CI | PR and `master` (skips docs-only) | `ubuntu-latest` only |
| Deploy | After green CI on `master` | `ubuntu-latest` |

If Deploy is blocked by billing, fix GitHub spending, then **Re-run** Deploy or **Run workflow**.
