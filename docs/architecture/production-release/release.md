# Desktop release (local, macOS)

Hosted `release.yml` is **removed**. Overlay v1 is **Apple Silicon `.dmg` only**, built on the operator Mac.

## Publish

```bash
./scripts/ci-local.sh
export VITE_API_URL=https://aniki-api.fly.dev
export VITE_WEB_URL=https://aniki-web.pages.dev
./scripts/release-desktop.sh v0.1.1
```

Fails unless the host is Darwin and URLs are non-localhost HTTPS. `gh` creates or updates the GitHub Release. `GET /desktop/latest` serves the `.dmg`; the SPA does not offer Windows.

## Asset names

- `*.dmg` (aarch64)

## Operator steps

1. Run `./scripts/ci-local.sh` (optional `--desktop`).
2. Run `./scripts/release-desktop.sh vX.Y.Z`.
3. Hard-refresh Pages; signed-in **Download for Mac** resolves `latest`.
