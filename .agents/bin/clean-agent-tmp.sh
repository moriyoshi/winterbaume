#!/usr/bin/env bash
# Clean stale per-session and per-crate cargo target directories under
# `.agents-workspace/tmp/`. These accumulate as Claude Code / Codex sessions come and go
# (the `cargo.sh` wrapper mints `.agents-workspace/tmp/target-<session>/` per session)
# and from the 2026-04-27 quality-gate sweep (`target-qg-<crate>/` per crate).
#
# This script is the "separate clean-agent-tmp script" alternative described
# in the agent-tooling TODO entry; it is intentionally NOT triggered from
# `cargo.sh` so that no agent invocation can silently delete another agent's
# in-progress build state.
#
# Usage:
#   .agents/bin/clean-agent-tmp.sh            # delete dirs unmodified for 7+ days
#   .agents/bin/clean-agent-tmp.sh --days N   # adjust the age threshold
#   .agents/bin/clean-agent-tmp.sh --dry-run  # list what would be deleted
#   .agents/bin/clean-agent-tmp.sh --all      # delete every matching dir regardless of age
#
# Safe to run while other agents are working: dirs younger than the age
# threshold are skipped, and the current session's CARGO_TARGET_DIR (if set)
# is always preserved.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
WS="$(cd "$SCRIPT_DIR/../.." && pwd -P)"
TMP="$WS/.agents-workspace/tmp"

days=7
dry_run=0
delete_all=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --days) days="$2"; shift 2 ;;
    --dry-run) dry_run=1; shift ;;
    --all) delete_all=1; shift ;;
    -h|--help)
      sed -n '2,18p' "$0" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "$0: unknown argument: $1" >&2; exit 2 ;;
  esac
done

if [[ ! -d "$TMP" ]]; then
  echo "$0: nothing to do; $TMP does not exist"
  exit 0
fi

current="${CARGO_TARGET_DIR:-}"

find_args=("$TMP" -maxdepth 1 -type d \( -name 'target-*' -o -name 'target-qg-*' \))
if [[ "$delete_all" -eq 0 ]]; then
  find_args+=(-mtime "+$days")
fi

count=0
total_bytes=0
while IFS= read -r dir; do
  [[ -z "$dir" ]] && continue
  if [[ -n "$current" && "$dir" == "$current" ]]; then
    continue
  fi
  bytes=$(du -sk "$dir" 2>/dev/null | awk '{print $1}')
  total_bytes=$((total_bytes + ${bytes:-0}))
  count=$((count + 1))
  if [[ "$dry_run" -eq 1 ]]; then
    printf 'would delete: %s (%s KiB)\n' "$dir" "${bytes:-?}"
  else
    rm -rf "$dir"
    printf 'deleted: %s (%s KiB reclaimed)\n' "$dir" "${bytes:-?}"
  fi
done < <(find "${find_args[@]}" 2>/dev/null)

if [[ "$count" -eq 0 ]]; then
  echo "$0: nothing matched (age >= ${days} days, target-* / target-qg-* under ${TMP})"
else
  total_mib=$((total_bytes / 1024))
  if [[ "$dry_run" -eq 1 ]]; then
    printf '%d dir(s) would be deleted; ~%d MiB reclaimable.\n' "$count" "$total_mib"
  else
    printf 'deleted %d dir(s); ~%d MiB reclaimed.\n' "$count" "$total_mib"
  fi
fi
