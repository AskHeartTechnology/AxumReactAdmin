#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> Building frontend..."
cd "$ROOT/app"
if command -v bun >/dev/null; then
  bun run build:dev
elif command -v pnpm >/dev/null; then
  pnpm build:dev
else
  npm run build:dev
fi

echo "==> Building backend (release, embed static/)..."
cd "$ROOT"
cargo build --release

echo "==> Done: $ROOT/target/release/AxumReactAdmin"
