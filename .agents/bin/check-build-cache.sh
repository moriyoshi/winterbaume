#!/usr/bin/env bash
# Verify that the sccache-wrapper actually ran for the most recent build in
# the current per-session target directory. The discriminator is the
# `.cachekey` sidecar that `cache_restore` and `cache_store` always write
# next to a produced `.rmeta`: a deps dir with `.rmeta` files but zero
# `.cachekey` files means rustc was invoked without the wrapper, so the
# cross-worktree cache at `.agents-workspace/tmp/winterbaume-rustc-cache/` was not
# consulted and the resulting artefacts are NOT hardlinked from it.
#
# Usage:
#   .agents/bin/check-build-cache.sh                 # check $CARGO_TARGET_DIR (or wrapper default)
#   .agents/bin/check-build-cache.sh <target-dir>    # check a specific dir
#
# Exit codes:
#   0  -- wrapper is active for this target dir
#   1  -- usage / no target dir found
#   2  -- target dir has .rmeta files but no .cachekey sidecars (wrapper bypassed)
#
# See the 2026-04-30 JOURNAL entry "sccache-wrapper cache not shared across
# agents" for the failure mode this script catches.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
WS="$(cd "$SCRIPT_DIR/../.." && pwd -P)"

if [[ $# -gt 0 ]]; then
  target="$1"
elif [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
  target="$CARGO_TARGET_DIR"
else
  # Best-effort guess: if the user runs this without going through
  # cargo.sh, look for the most recently touched per-session dir.
  target="$(ls -dt "$WS"/.agents-workspace/tmp/target-* 2>/dev/null | head -1 || true)"
  if [[ -z "$target" ]]; then
    echo "$0: no target dir specified, no \$CARGO_TARGET_DIR set, and no .agents-workspace/tmp/target-* found" >&2
    echo "$0: usage: $0 [<target-dir>]" >&2
    exit 1
  fi
  echo "(no \$CARGO_TARGET_DIR set; checking most recent per-session dir: $target)"
fi

deps="$target/debug/deps"
if [[ ! -d "$deps" ]]; then
  echo "$0: $deps does not exist; run a debug build first" >&2
  exit 1
fi

cachekeys=$(find "$deps" -maxdepth 1 -name '*.cachekey' 2>/dev/null | wc -l | tr -d ' ')
rmetas=$(find "$deps" -maxdepth 1 -name 'lib*.rmeta' 2>/dev/null | wc -l | tr -d ' ')

# Count rmeta files that share their inode with another path on the
# filesystem -- i.e. those that were hardlinked from the shared cache or
# from another session's deps dir. macOS `stat -f %l` gives the link count.
hardlinked=0
if [[ "$rmetas" -gt 0 ]]; then
  # shellcheck disable=SC2012
  hardlinked=$(find "$deps" -maxdepth 1 -name 'lib*.rmeta' -exec stat -f '%l' {} + 2>/dev/null \
                  | awk '$1>1' | wc -l | tr -d ' ')
fi

printf 'target dir:        %s\n' "$target"
printf 'rmeta files:       %s\n' "$rmetas"
printf 'cachekey sidecars: %s\n' "$cachekeys"
printf 'rmeta hardlinked:  %s\n' "$hardlinked"
expected_wrapper="$WS/.agents-workspace/tmp/sccache-wrapper"
rustc_wrapper_status="(unset)"
if [[ -n "${RUSTC_WRAPPER:-}" ]]; then
  if [[ "$RUSTC_WRAPPER" == "$expected_wrapper" ]]; then
    rustc_wrapper_status="$RUSTC_WRAPPER (workspace wrapper)"
  else
    rustc_wrapper_status="$RUSTC_WRAPPER (NOT the workspace wrapper -- expected $expected_wrapper)"
  fi
fi
printf 'wrapper env in this shell:\n'
printf '  RUSTC_WRAPPER         = %s\n' "$rustc_wrapper_status"
printf '  WB_WORKSPACE_ROOT     = %s\n' "${WB_WORKSPACE_ROOT:-(unset)}"
printf '  WB_RUSTC_CACHE_DIR    = %s\n' "${WB_RUSTC_CACHE_DIR:-(unset)}"

if [[ "$rmetas" -gt 0 && "$cachekeys" -eq 0 ]]; then
  cat >&2 <<'MSG'

PROBLEM: this target dir has compiled artefacts but no `.cachekey` sidecars,
so sccache-wrapper did NOT run for any rustc invocation in it. The
cross-worktree cache at `.agents-workspace/tmp/winterbaume-rustc-cache/` was not
consulted and the produced `.rmeta` files are NOT hardlinked from it --
concurrent agents will redundantly rebuild byte-identical artefacts.

Cause is almost always: bare `cargo` was invoked instead of
`./.agents/bin/cargo.sh`, possibly with a hand-set `CARGO_TARGET_DIR=...`.
Re-run the build via the wrapper script and re-check.

See the 2026-04-30 JOURNAL entry "sccache-wrapper cache not shared across
agents" for the full diagnostic.
MSG
  exit 2
fi

if [[ "$rmetas" -eq 0 ]]; then
  echo
  echo "(no rmeta files yet; nothing built in this target dir)"
  exit 0
fi

echo
printf 'OK: sccache-wrapper is active for this target dir (%d / %d rmeta hardlinked from cache).\n' \
  "$hardlinked" "$rmetas"
