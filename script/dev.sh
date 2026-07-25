#!/usr/bin/env bash
# 同时启动 Axum 后端 + Vite 前端；Ctrl+C 一并退出
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BACKEND_DIR="$ROOT"
FRONTEND_DIR="$ROOT/app"

BACKEND_PID=""
FRONTEND_PID=""
CLEANED=0

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m'

log()  { echo -e "${GREEN}==>${NC} $*"; }
info() { echo -e "${BLUE}==>${NC} $*"; }
warn() { echo -e "${YELLOW}==>${NC} $*"; }
err()  { echo -e "${RED}==>${NC} $*" >&2; }

# 递归结束进程树（兼容 macOS / Linux）
kill_tree() {
  local pid=$1
  local children
  children="$(pgrep -P "$pid" 2>/dev/null || true)"
  for child in $children; do
    kill_tree "$child"
  done
  if kill -0 "$pid" 2>/dev/null; then
    kill -TERM "$pid" 2>/dev/null || true
  fi
}

force_kill_tree() {
  local pid=$1
  local children
  children="$(pgrep -P "$pid" 2>/dev/null || true)"
  for child in $children; do
    force_kill_tree "$child"
  done
  if kill -0 "$pid" 2>/dev/null; then
    kill -KILL "$pid" 2>/dev/null || true
  fi
}

cleanup() {
  [[ "$CLEANED" -eq 1 ]] && return
  CLEANED=1
  trap - INT TERM EXIT

  echo ""
  info "Shutting down frontend & backend..."

  [[ -n "$FRONTEND_PID" ]] && kill_tree "$FRONTEND_PID"
  [[ -n "$BACKEND_PID" ]] && kill_tree "$BACKEND_PID"

  # 给优雅退出一点时间
  sleep 0.4

  [[ -n "$FRONTEND_PID" ]] && force_kill_tree "$FRONTEND_PID"
  [[ -n "$BACKEND_PID" ]] && force_kill_tree "$BACKEND_PID"

  wait 2>/dev/null || true
  log "All services stopped."
}

trap cleanup INT TERM EXIT

detect_pkg_mgr() {
  local dir=$1
  if command -v bun >/dev/null 2>&1 && { [[ -f "$dir/bun.lockb" ]] || [[ -f "$dir/bun.lock" ]]; }; then
    echo bun
  elif command -v pnpm >/dev/null 2>&1 && [[ -f "$dir/pnpm-lock.yaml" ]]; then
    echo pnpm
  elif command -v yarn >/dev/null 2>&1 && [[ -f "$dir/yarn.lock" ]]; then
    echo yarn
  else
    echo npm
  fi
}

[[ -f "$BACKEND_DIR/Cargo.toml" ]] || { err "找不到后端 Cargo.toml: $BACKEND_DIR"; exit 1; }
[[ -f "$FRONTEND_DIR/package.json" ]] || {
  err "找不到前端 app/package.json"
  err "请先创建 Vite 前端到: $FRONTEND_DIR"
  exit 1
}

if ! command -v cargo >/dev/null 2>&1; then
  err "未找到 cargo，请先安装 Rust"
  exit 1
fi

PKG_MGR="$(detect_pkg_mgr "$FRONTEND_DIR")"
info "Package manager: $PKG_MGR"

log "Starting Axum backend (cargo run)..."
(
  cd "$BACKEND_DIR"
  exec cargo run
) &
BACKEND_PID=$!

log "Starting Vite frontend ($PKG_MGR run dev)..."
(
  cd "$FRONTEND_DIR"
  case "$PKG_MGR" in
    bun)  exec bun run dev ;;
    pnpm) exec pnpm dev ;;
    yarn) exec yarn dev ;;
    *)    exec npm run dev ;;
  esac
) &
FRONTEND_PID=$!

info "Backend PID : $BACKEND_PID"
info "Frontend PID: $FRONTEND_PID"
info "Press Ctrl+C to stop both."
echo ""

# 任一进程退出则一起收尾
while true; do
  if ! kill -0 "$BACKEND_PID" 2>/dev/null; then
    warn "Backend exited."
    break
  fi
  if ! kill -0 "$FRONTEND_PID" 2>/dev/null; then
    warn "Frontend exited."
    break
  fi
  sleep 0.5
done

cleanup
