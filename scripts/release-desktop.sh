#!/usr/bin/env bash
# Build an unsigned macOS overlay on this Mac and attach the .dmg to a GitHub Release.
# Usage: ./scripts/release-desktop.sh v0.1.1
set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Desktop release is macOS-only for now. This host is $(uname -s)."
  exit 1
fi

tag="${1:?usage: $0 vX.Y.Z}"
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

export VITE_API_URL="${VITE_API_URL:-https://aniki-api.fly.dev}"
export VITE_WEB_URL="${VITE_WEB_URL:-https://aniki-web.pages.dev}"
export CI=true

case "$VITE_API_URL$VITE_WEB_URL" in
  *localhost*) echo "VITE_API_URL and VITE_WEB_URL must be HTTPS production origins"; exit 1 ;;
esac
if [[ -z "$VITE_API_URL" || -z "$VITE_WEB_URL" ]]; then
  echo "VITE_API_URL and VITE_WEB_URL are required"; exit 1
fi

command -v gh >/dev/null || { echo "gh CLI is required"; exit 1; }

pnpm install --frozen-lockfile

(cd apps/desktop && pnpm tauri build --target aarch64-apple-darwin)
assets=()
while IFS= read -r -d '' f; do assets+=("$f"); done < <(
  find apps/desktop/src-tauri/target -name '*.dmg' -print0 2>/dev/null || true
)

if [[ ${#assets[@]} -eq 0 ]]; then
  echo "No installer artifacts found under apps/desktop/src-tauri/target"
  exit 1
fi

echo "Uploading ${#assets[@]} asset(s) to GitHub Release $tag"
if gh release view "$tag" >/dev/null 2>&1; then
  gh release upload "$tag" "${assets[@]}" --clobber
else
  if ! git rev-parse "$tag" >/dev/null 2>&1; then
    git tag "$tag"
    git push origin "$tag"
  fi
  gh release create "$tag" "${assets[@]}" --title "Aniki $tag" --generate-notes
fi

echo "Release $tag updated. Signed-in dashboard downloads use GET /desktop/latest."
