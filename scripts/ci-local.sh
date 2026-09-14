#!/usr/bin/env bash
# Mirror GitHub Linux CI on this Mac (no hosted Tauri minutes). Desktop is macOS-only for now.
set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Local CI/desktop is macOS-only for now. This host is $(uname -s)."
  exit 1
fi

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

export CARGO_TERM_COLOR="${CARGO_TERM_COLOR:-always}"
export DATABASE_URL="${DATABASE_URL:-postgres://aniki:aniki@localhost:5432/aniki}"
export REDIS_URL="${REDIS_URL:-redis://localhost:6379}"
export JWT_SECRET="${JWT_SECRET:-ci-test-secret}"
excludes=(--exclude aniki-desktop --exclude aniki-audio-core)

echo "==> Postgres + Redis"
docker compose up -d postgres redis
docker compose up migrate

echo "==> cargo check / clippy / test (Linux-parity excludes)"
cargo check --workspace --all-targets "${excludes[@]}"
cargo clippy --workspace --all-targets "${excludes[@]}" -- -D warnings
cargo test --workspace "${excludes[@]}"

echo "==> pnpm typecheck / lint / web build"
pnpm install --frozen-lockfile
pnpm typecheck
pnpm lint
pnpm --filter @aniki/web build

if [[ "${1:-}" == "--desktop" ]]; then
  echo "==> desktop overlay (Apple Silicon .dmg)"
  (cd apps/desktop && pnpm tauri build --target aarch64-apple-darwin)
fi

echo "==> local CI passed"
