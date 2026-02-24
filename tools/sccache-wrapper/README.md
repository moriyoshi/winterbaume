# sccache-wrapper

A `RUSTC_WRAPPER` that provides cross-worktree Rust compilation caching on top of sccache.

## Problem

sccache's `SCCACHE_BASEDIRS` does not normalise paths for Rust compilations (only C/C++ preprocessor output). When building from different git worktrees, every rustc invocation contains absolute paths that differ between worktrees (e.g. `--out-dir`, `--extern`, `-L`), so sccache always cache-misses across trees — even for identical source code.

## Solution

sccache-wrapper interposes a normalising cache layer between cargo and sccache:

1. **Parse** rustc arguments minimally to extract crate name, output dir, emit flags, and source file.
2. **Filter** workspace-dependent arguments from the cache key: `-C extra-filename=`, `-C incremental=`, `--out-dir`, `-L`, `--extern` paths (only the crate name is kept), and display-only flags (`--diagnostic-width`, `--color`). `-C metadata=` is kept because it distinguishes target vs host compilations.
3. **Normalise** remaining arguments by replacing the workspace root with a fixed placeholder (`@@WORKSPACE@@`).
4. **Hash** the normalised arguments together with the source tree content and compiler identity to produce a workspace-independent cache key.
5. **On HIT**: restore cached artifacts to `--out-dir` via hardlinks (zero-copy, no disk duplication), mapping filenames from the stored `extra-filename` to the current one. Replay captured stdout/stderr and rustc's recorded exit status.
6. **On MISS**: acquire a per-key `flock` (singleflight — concurrent builds of the same key wait rather than compile redundantly), delegate to sccache (which delegates to rustc), capture the outputs, and store them in the cache. Only successful builds (exit status 0) are stored — negative caching is intentionally off because stderr is not yet path-normalised.
7. **`--test` invocations** are cached too: rustc emits a test-harness binary (`<name><ef>`, no `lib` prefix; `.exe` on Windows) instead of `lib*.rlib`/`lib*.rmeta` even when `--crate-type lib` is also passed, and the wrapper recognises this and stores/restores the binary directly. Hardlink restoration preserves the executable bit.
8. **Non-cacheable invocations** (e.g. `--print`, `-vV`, proc-macro/binary crate types) are passed through to sccache via `exec()` with zero overhead.
9. **Incremental stripping**: `-C incremental=…` is removed from the arguments passed to sccache/rustc, since incremental compilation is incompatible with sccache's deterministic-output requirement.

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `WB_WORKSPACE_ROOT` | `git rev-parse --show-toplevel` | Workspace root for path normalisation. Set this to avoid a git subprocess per rustc invocation. |
| `WB_RUSTC_CACHE_DIR` | `$TMPDIR/winterbaume-rustc-cache` | Cache directory. Use a project-local path in sandboxed environments. |
| `WB_RUSTC_CACHE` | (enabled) | Set to `0` to disable cross-worktree caching entirely. |
| `WB_RUSTC_CACHE_DEBUG` | (off) | Set to any value to log HIT/MISS/waiting messages and cache-key args to stderr. |
| `WB_SCCACHE_WRAPPER_SCOREBOARD` | (off) | Directory to write per-crate scoreboard files into. When set, every cache-miss invocation registers itself so external observers can see which compilations are running, who is contending for them, and how long each one has been (or was) running. See [Scoreboard](#scoreboard). |

When using `mise`, all variables are set automatically via `mise.toml`.

## Usage

```sh
# Build the wrapper (must unset RUSTC_WRAPPER to avoid circular invocation)
RUSTC_WRAPPER= cargo build -p sccache-wrapper

# Set environment
export RUSTC_WRAPPER="$(realpath ./target/debug/sccache-wrapper)"
export WB_WORKSPACE_ROOT="$PWD"
export WB_RUSTC_CACHE_DIR="$PWD/.agents-workspace/tmp/winterbaume-rustc-cache"

# Build normally — the wrapper is transparent
cargo build -p winterbaume-server --all-features
```

## Diagnostics

Dump all cache entries with their stored command lines:

```sh
./target/debug/sccache-wrapper --dump-cache
```

Each entry shows:

- **received** — the original command line cargo passed to the wrapper
- **emitted** — the command line actually sent downstream to sccache (with `-C incremental=…` stripped)
- **files** — the cached artifacts

Example output:

```
--- bytes (fd8ac43c4611) ef=-09b1a263836cddec ---
  received: rustc --crate-name bytes --edition=2021 …
  emitted:  rustc --crate-name bytes --edition=2021 …
  files:    bytes-09b1a263836cddec.d, libbytes-09b1a263836cddec.rmeta, libbytes-09b1a263836cddec.rlib
```

To see live HIT/MISS messages during a build, set `WB_RUSTC_CACHE_DEBUG=1`.

## Scoreboard

When `WB_SCCACHE_WRAPPER_SCOREBOARD` is set to a writable directory, each cache-miss invocation drops a per-crate JSON file under that directory:

```
$WB_SCCACHE_WRAPPER_SCOREBOARD/
  <crate>-<short_key>.json        # one file per cache key
  <crate>-<short_key>.json.lock   # fslock sibling for serialised RMW
```

Each file records:

- `state` — `building` while a leader is compiling, `ready` once it has finished.
- `started_at`, `last_update`, `completed_at` — Unix timestamps; subtract to get build duration (live or final).
- `command_line` — the full rustc argv as cargo passed it.
- `contenders` — every wrapper process currently registered for this cache key, with its `pid`, role (`building` for the singleflight leader, `waiting` for followers), `joined_at`/`last_seen` heartbeats, the inferred `session` id (extracted from `CARGO_TARGET_DIR`'s `target-<session>` suffix when the agent cargo wrapper is in use), and a `process_tree` chain (caller → cargo → sccache-wrapper).

A background heartbeat thread refreshes `last_seen` every couple of seconds while the build runs, so external observers can distinguish live builds from abandoned ones. Stale contenders (no heartbeat for ~30s) and `ready` entries older than five minutes are pruned opportunistically by the next session that touches the file.

Dump a human-readable view of the directory:

```sh
WB_SCCACHE_WRAPPER_SCOREBOARD=/path/to/dir ./target/debug/sccache-wrapper --show-scoreboard
```

Example output:

```
--- aws_config (e1731f628173) ready 6s (done) ---
  cmd:  /Users/foo/.rustup/toolchains/.../bin/rustc --crate-name aws_config …
  contenders: (none)
--- tokio (56f0412baa90) building 4s (live) ---
  cmd:  /Users/foo/.rustup/toolchains/.../bin/rustc --crate-name tokio …
  contenders:
    [B] pid=62043 session=claude-60027 sccache-wrapper(62043) → cargo(61174) → zsh(61172) → claude(49917) (last_seen 1s ago)
    [W] pid=62120 session=codex-58002  sccache-wrapper(62120) → cargo(60900) → zsh(60898) → codex(57001)  (last_seen 2s ago)
```

## Cache structure

```
$WB_RUSTC_CACHE_DIR/
  <first-2-hex>/<sha256>/
    manifest              # extra-filename + line-oriented file list
    args_received         # original rustc command line (normalised)
    args_emitted          # command line sent to sccache (normalised)
    stdout                # captured stdout (paths use @@WORKSPACE@@ placeholder)
    stderr                # captured stderr
    libfoo-abc123.rlib    # hardlinked artifact
    libfoo-abc123.rmeta
    foo-abc123.d          # dep-info with normalised paths
  locks/<first-2-hex>/<sha256>.lock   # flock files for singleflight
```

## How it works with worktrees

Cargo's metadata hashes (the `-abc123` suffix in `libfoo-abc123.rlib`) are stable across worktrees at the same commit. The only difference in rustc invocations is the absolute workspace path prefix and some display-only flags. By normalising/filtering these before hashing, the wrapper produces identical cache keys regardless of which worktree the build runs in.

When cargo compiles a crate twice in one build (target context + host context for proc-macro support), the two invocations receive different `-C metadata=` values. Because the wrapper keeps metadata in the cache key, these produce distinct cache entries and do not interfere with each other.

Artifacts are shared via filesystem hardlinks: the cache and `target/` directories share the same inodes, so no data is duplicated on disk. The singleflight lock ensures that concurrent builds of the same cache key (e.g. two worktrees compiling `bytes` simultaneously) do not duplicate work — the first acquires the lock and compiles, the second waits and then reads from cache.
