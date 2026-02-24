#!/usr/bin/env bash
# Agent-only cargo wrapper. Mints a per-session CARGO_TARGET_DIR and ensures
# RUSTC_WRAPPER points at a fresh sccache-wrapper, so concurrent coding agents
# do not contend on the shared default `target/` lock.
#
# Use this script in place of bare `cargo` from any agent shell:
#   .agents/bin/cargo.sh build -p winterbaume-foo
#
# Humans: keep using mise + bare `cargo` as before.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
WS="$(cd "$SCRIPT_DIR/../.." && pwd -P)"

# Resolve the main worktree.  The sccache-wrapper binary, the rustc cache, and
# the scoreboard all live there so sibling worktrees share one binary and one
# cache instead of each rebuilding their own copy.  WB_WORKSPACE_ROOT itself
# stays tied to the current worktree — the wrapper substitutes that string out
# of rustc args, so it has to match cargo's absolute paths.
MAIN_WS="$WS"
if candidate=$(git -C "$WS" worktree list --porcelain 2>/dev/null | awk '/^worktree /{print $2; exit}') \
    && [[ -n "$candidate" && -d "$candidate" ]]; then
  MAIN_WS="$candidate"
fi

session_id() {
  if [[ -n "${CLAUDE_CODE_SSE_PORT:-}" ]]; then
    printf 'claude-%s\n' "$CLAUDE_CODE_SSE_PORT"
    return
  fi
  local pid=$PPID
  while [[ "$pid" != "0" && "$pid" != "1" ]]; do
    local name
    name=$(ps -o comm= -p "$pid" 2>/dev/null | tr -d ' ' || true)
    case "${name##*/}" in
      claude) printf 'claude-%s\n' "$pid"; return ;;
      codex)  printf 'codex-%s\n'  "$pid"; return ;;
    esac
    pid=$(ps -o ppid= -p "$pid" 2>/dev/null | tr -d ' ' || echo 0)
  done
  printf 'shell-%s\n' "$$"
}

SESSION="$(session_id)"

WRAPPER_BIN="$MAIN_WS/.agents-workspace/tmp/sccache-wrapper"
WRAPPER_SRC="$MAIN_WS/tools/sccache-wrapper/src/main.rs"
WRAPPER_TOML="$MAIN_WS/tools/sccache-wrapper/Cargo.toml"
WRAPPER_BUILD_DIR="$MAIN_WS/.agents-workspace/tmp/sccache-wrapper-build"

needs_rebuild() {
  [[ ! -x "$WRAPPER_BIN" ]] && return 0
  [[ "$WRAPPER_SRC"  -nt "$WRAPPER_BIN" ]] && return 0
  [[ "$WRAPPER_TOML" -nt "$WRAPPER_BIN" ]] && return 0
  return 1
}

if needs_rebuild; then
  printf '.agents/bin/cargo: rebuilding sccache-wrapper\n' >&2
  mkdir -p "$MAIN_WS/.agents-workspace/tmp"
  (
    cd "$MAIN_WS"
    RUSTC_WRAPPER= CARGO_TARGET_DIR="$WRAPPER_BUILD_DIR" \
      cargo build --release -p sccache-wrapper >&2
  )
  # Atomic install: stage to a unique tmp path then rename, so a concurrent
  # rebuilder from a sibling worktree never observes a partial binary.
  tmp_bin="$WRAPPER_BIN.$$.tmp"
  rm -f "$tmp_bin"
  cp "$WRAPPER_BUILD_DIR/release/sccache-wrapper" "$tmp_bin"
  mv -f "$tmp_bin" "$WRAPPER_BIN"
fi

export CARGO_INCREMENTAL=0
export RUSTC_WRAPPER="$WRAPPER_BIN"
export WB_WORKSPACE_ROOT="$WS"
export WB_RUSTC_CACHE_DIR="$MAIN_WS/.agents-workspace/tmp/winterbaume-rustc-cache"
export WB_SCCACHE_WRAPPER_SCOREBOARD="$MAIN_WS/.agents-workspace/tmp/sccache-wrapper-scoreboard"
export CARGO_TARGET_DIR="$WS/.agents-workspace/tmp/target-$SESSION"

# Cap build parallelism so a single crate build does not saturate every core
# and spike load averages. Honour an explicit override if the caller set one.
if [[ -z "${CARGO_BUILD_JOBS:-}" ]]; then
  if command -v nproc >/dev/null 2>&1; then
    _wb_ncpu=$(nproc 2>/dev/null || echo 2)
  elif command -v sysctl >/dev/null 2>&1; then
    _wb_ncpu=$(sysctl -n hw.ncpu 2>/dev/null || echo 2)
  else
    _wb_ncpu=2
  fi
  _wb_jobs=$(( _wb_ncpu / 2 ))
  [[ "$_wb_jobs" -lt 1 ]] && _wb_jobs=1
  export CARGO_BUILD_JOBS="$_wb_jobs"
  unset _wb_ncpu _wb_jobs
fi

exec cargo "$@"
