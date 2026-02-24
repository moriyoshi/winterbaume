# sccache-wrapper Cross-Worktree Cache

## Summary

`tools/sccache-wrapper` is Winterbaume's Rust compilation cache layer around `sccache`. It exists because ordinary `sccache` does not normalise Rust compiler command lines enough to share cache entries across git worktrees, and because Cargo emits local target paths, output suffixes, and incremental flags that must be filtered or rewritten for safe reuse.

## Key Facts

- `SCCACHE_BASEDIRS` is not enough for Rust. It is read by the sccache server at startup and does not normalise the Rust cache key path data that matters here.
- The wrapper creates its own content-derived cache key from normalised rustc arguments, source content, compiler identity, and dependency cache keys.
- `-C incremental=...` must be stripped from rustc arguments before invoking sccache. Setting `CARGO_INCREMENTAL=0` inside the wrapper is too late.
- `-C metadata=...` must stay in the key because it distinguishes host and target crate compilations.
- `-C extra-filename=...`, `--out-dir`, `-L`, `--diagnostic-width`, and `--color` are filtered or normalised because they are local filename or display concerns.
- Every cached output gets a `.cachekey` sidecar. Parent crates hash dependency sidecar values instead of brittle `--extern` paths.
- Per-key singleflight locking prevents many agents compiling and storing the same cache entry concurrently.
- `--dump-cache` is the diagnostic entry point for inspecting stored entries, emitted arguments, files, and resolved dependency keys.
- A stale sccache server can hang every build routed through the wrapper. `sccache --show-stats` is the health check, and `pkill -f sccache` plus `sccache --start-server` is the recovery path.
- `--test` invocations are recognised and cached: cargo's test-harness builds ( `--test --crate-type lib`, no leading `lib` prefix on the binary, no extension on Unix ) flow through the wrapper. `is_test` is part of `ParsedArgs`; expected output files include `<name><ef>` ( `.exe` on Windows ) for `link`, plus the usual `<name><ef>.d` and `lib<name><ef>.rmeta`.
- Manifest stores `exit:N` with rustc's exit status. `cache_restore` returns `Some(code)` instead of an unconditional `Some(0)`. The `if exit_code == 0` gate in `compile_and_cache` is retained for now because stderr is not yet path-normalised, so negative caching would replay original-worktree absolute paths in error diagnostics.
- GC grouping key is `(program_kind, crate_name, metadata)` with `program_kind` suffixed `-test` when `args_received` contains `--test`. This keeps lib and test units in separate buckets even on the rare path where cargo gives them the same `-Cmetadata=` ( e.g. when stripped or forced to collide ).

## Details

### Why a Wrapper Exists

Initial work tried to make cross-worktree Rust builds share cache entries by setting `SCCACHE_BASEDIRS`. That failed for two reasons:

- the sccache daemon reads `SCCACHE_BASEDIRS` once at startup, so a per-invocation wrapper cannot change an already-running server
- sccache's Rust path hashes the full rustc command line, including absolute paths and Cargo-derived output locations

Cargo usually passes source paths relative to the workspace, but many compiler arguments still contain worktree-local state. The wrapper therefore computes a path-normalised key before deciding whether to restore a cached result or call through to sccache.

### Cache Key Rules

The wrapper hashes the stable parts of a rustc invocation:

- normalised rustc arguments with the workspace root replaced by `@@WORKSPACE@@`
- source tree content hash
- compiler identity, currently based on binary metadata
- dependency cache keys resolved from `.cachekey` sidecars

Important filtering rules:

- keep `-C metadata=...`; it is the crate disambiguator and separates host and target builds
- skip `-C extra-filename=...`; it is a filename suffix that the restore path can remap
- strip `-C incremental=...` before invoking sccache
- skip display-only flags such as `--diagnostic-width` and `--color`
- avoid hashing local `--out-dir` and `-L` paths directly

An earlier design hashed only the `--extern` crate name. That was cross-worktree friendly but unsound: parent crates could be restored from cache even when a dependency identity changed, causing `E0460` and `E0463` errors. The `.cachekey` sidecar design fixes that by making dependency identity both content-derived and worktree-independent.

### Artefact Restore

Cached artefacts are restored into the current target directory using hardlinks where possible. The manifest stores the original extra-filename suffix and generated file set. On restore, filenames and dep-info content are rewritten from the cached suffix to the current suffix.

Each restored output also receives a `.cachekey` sidecar next to the `.rmeta`, `.rlib`, or dep-info files so dependent crates can include the restored dependency identity in their own cache key.

### Singleflight Locking

When a cache key misses, the wrapper acquires a per-key lock:

- the leader compiles and stores the result
- followers wait for the leader, then attempt restore again
- if the leader fails, followers fall back to compiling

The original Unix-only `flock(2)` implementation was replaced by the `fslock` crate so the singleflight path works across platforms. Lock files live under `$WB_RUSTC_CACHE_DIR/locks/`.

### Diagnostics

Every new cache entry stores:

- `args_received`: the normalised command line cargo passed to the wrapper
- `args_emitted`: the normalised command line sent downstream to sccache
- manifest lines for files, extra-filename, and resolved dependencies
- captured stdout and stderr for replay

`sccache-wrapper --dump-cache` walks the cache and prints the crate name, key, emitted arguments, files, and dependency cache keys. Entries written before diagnostic storage show missing argument lines.

### Scoreboard for In-Flight Compilations

External observers ( humans, agent dashboards, other tooling ) need to see which compilations are in flight, who is contending for them, and how long each one has been ( or was ) running. The scoreboard subsystem provides this without per-build cost on the fast path.

- New env var `WB_SCCACHE_WRAPPER_SCOREBOARD` points at a directory. Each cache-miss compilation drops a `<crate>-<short_key>.json` file there with a sibling `.lock` for fslock-based serialised RMW.
- Per-file locks ( one lock per crate-key, not one global lock ) keep contention scoped per-crate so unrelated builds do not serialise.
- Each entry records:
  - `state`: `building` or `ready`
  - `started_at`, `last_update`, `completed_at`
  - the full rustc `command_line`
  - a `contenders` array, where each contender carries `pid`, `role` ( `building` or `waiting` ), `joined_at`, `last_seen`, the `session` id ( extracted from `CARGO_TARGET_DIR`'s `target-<session>` suffix ), and a `process_tree` chain ( caller -> cargo -> sccache-wrapper )
- Session lifecycle: register as `Waiting` after a HIT miss, promote to `Building` once the singleflight flock is held, mark `Ready` on success, drop unregisters. A 2 s heartbeat thread keeps `last_seen` fresh so external readers can distinguish live builds from abandoned ones.
- Opportunistic pruning during every read-modify-write drops stale contenders ( `last_seen` older than 30 s ) and aged-out `ready` entries ( older than 5 min ).
- Process tree walked via `sysinfo` ( `default-features = false, features = ["system"]` ) using `refresh_processes_specifics` per-pid, so cost is O ( depth ), not O ( all processes ). Cross-platform; Windows benefits without cfg-gated platform code.
- Cache HIT and passthrough paths skip the scoreboard entirely so the fast path remains free of overhead.

`sccache-wrapper --show-scoreboard` renders the directory in a human-readable form. The agent cargo wrappers ( `.agents/bin/cargo.sh` and `cargo.ps1` ) export `WB_SCCACHE_WRAPPER_SCOREBOARD=.agents-workspace/tmp/sccache-wrapper-scoreboard/` so every session writes into the same directory and can see the others.

End-to-end smoke test against a fresh `WB_RUSTC_CACHE_DIR` confirmed entries appear during a `winterbaume-core` build, transition to `ready` with realistic durations ( 1 - 7 s per crate ), contenders empty out cleanly on Drop, and the process tree is captured as `sccache-wrapper -> cargo -> zsh -> claude -> zsh -> tmux`.

Useful when:

- A build appears stuck and you want to know whether the session is genuinely compiling, waiting on another agent's singleflight lock, or has died abandoning a half-built crate.
- Diagnosing cross-session contention: live `building` state plus a non-empty `contenders` list with multiple sessions tells you several agents are queued on the same crate.

### Garbage Collection ( `--gc` / `--gc --dry-run` )

The wrapper's cache is content-addressed: every distinct `(compiler, normalised-args, source-tree-hash)` triple gets its own directory. Source changes derive a new key and store a fresh entry; the old entry is never visited again but remains on disk. `--gc` walks the cache, groups entries, and removes everything except the most recently written entry per group.

Grouping key is `(program_kind, crate_name, metadata_hash)`. `program_kind` is `Path::new(args_received[0]).file_stem()` ( e.g. `rustc`, `clippy-driver`, `rustdoc` ) -- using `file_stem` rather than `file_name` collapses `rustc.exe` and `rustc` to the same bucket on Windows. The `metadata` flag is cargo's stable compilation-context discriminator -- it encodes the crate's position in the dependency graph ( features, profile, target ) and is identical across worktrees for the same context. Within each group only the entry with the highest `manifest` mtime is kept.

The grouping rule is deliberately coarse: different feature sets or build profiles for the same crate produce different `metadata` hashes and are therefore in separate groups, so GC never evicts an entry that is still reachable by an active cargo invocation.

A subtle bug in an early implementation grouped only by `(crate_name, metadata)`. After a run that mixed `cargo check` ( rustc ) and `cargo clippy` ( clippy-driver ), the older program's freshest entry would disappear because both produce distinct, non-interchangeable cache entries that share `--crate-name` and `-Cmetadata=`. The cache key already separates them ( `compiler_identity(rustc_path)` is hashed in ), so both entries co-exist on disk; the GC grouping conflated them. Adding `program_kind` to the grouping key fixed it.

Removal also takes the sibling lock file at `<base>/locks/<2-char>/<key>.lock`. Output includes one line per removed entry ( `crate [program_kind] (short_key)` ) and a summary ( `N entries removed (M bytes freed)` ).

The compiler path lives at `args_received[0]` ( the wrapper is invoked as `RUSTC_WRAPPER`, so the first argument cargo passes is whatever should be invoked as rustc -- could be rustc, clippy-driver, rustdoc, or any future tool ). When you need to distinguish artefact provenance after the fact, that is the canonical place to look.

Cache-key differing is **necessary but not sufficient** for correct GC. Any "freshest one wins" heuristic must make sure the group really is interchangeable. Same crate + same metadata + different compiler is the trap that bit this one; future traps could be different `--emit` sets or different `-Z` flags if those start varying without changing metadata.

### Residual-Error Handling

`cache_store` and `cache_restore` previously committed partial work after intermediate failures: tmp dirs renamed into the live cache even when manifest writes failed, output files half-replaced when one step in the file list errored. The current shape wraps both function bodies in immediately-invoked `(|| -> io::Result<()> { ... })()` closures so every `fs::write` / `fs::read_to_string` / `fs::create_dir_all` / `fs::copy` step uses `?`. Each closure returns `io::Result<()>`; the outer function logs in debug mode and either skips the rename ( store ) or rolls back written files ( restore ).

For the restore, the closure tracks every `dst` it writes in a `Vec<PathBuf>` declared outside the closure, so failure cleanup can iterate and `fs::remove_file` each one. This was needed because `let _ = fs::write(...)` followed by an unconditional rename produced corrupt entries: `cache_restore` would later fail to read the manifest and treat it as a miss, wasting disk and confusing diagnostics.

The closure pattern was chosen because `cache_store` / `cache_restore` are leaf functions returning `()` / `Option<i32>` respectively. Converting them to return `io::Result<()>` would have rippled through callers ( `compile_and_cache`, `main` ) that already model success / failure via exit codes. The immediately-invoked closure gives `?`-style early-return ergonomics inside one function without changing the public signature.

Sidecar writes ( `cachekey_sidecar_for_output` ) remain non-fatal but now log under `WB_RUSTC_CACHE_DEBUG`. `flock.exclusive()` return value is now checked; `main` emits "proceeding unsynchronised" in debug mode for both wait-failure and lock-open-failure paths. `scoreboard::with_lock` removes the staged tmp file when its atomic rename fails, plugging the `json.tmp.<pid>` leak.

### Test-Harness Caching ( `--test` )

`tools/sccache-wrapper` originally rejected `--test` invocations because `parse_rustc_args` only inspected `--crate-type` and cargo passes `--test --crate-type lib` ( or no `--crate-type` at all ) for test units. rustc then ignores the lib request and emits a binary at `<out_dir>/<crate_name><extra-filename>` ( no `lib` prefix, no extension on Unix; `.exe` on Windows ), plus a matching `.d` and an empty `lib<name><ef>.rmeta` if `--emit=metadata` is set. The previous `expected_output_files` enumerated `lib<name><ef>.rlib`, which never existed, so `cache_store`'s "expected output ... was not produced" check failed silently after the wrapper had already paid its overhead. Net effect: every `cargo test` paid the wrapper cost and never cached.

The fix:

- `ParsedArgs` gains `is_test: bool`; `parse_rustc_args` recognises `--test` and only rejects `bin/cdylib/staticlib/proc-macro` crate types when `is_test` is false, so `--test --crate-type lib` flows through.
- `expected_output_files` emits the correct shape under `is_test`: `<name><ef>` for `link` ( `.exe` on Windows ), unchanged `<name><ef>.d` and `lib<name><ef>.rmeta` for `dep-info` and `metadata`.
- `cache_store` accepts an `exit_code: i32` parameter and writes `exit:N` into the manifest. `cache_restore`'s closure is retyped from `io::Result<()>` to `io::Result<i32>` and returns the recorded code ( default 0 for legacy entries that pre-date the field ); the `match result` arm returns `Some(code)`.
- The `if exit_code == 0` gate in `compile_and_cache` is retained: rustc errors are deterministic for the same source + args + compiler, so negative caching is **valid in principle**, but stderr is not path-normalised today ( only stdout is rewritten through the `@@WORKSPACE@@` placeholder ), and `--error-format=json` / `--json=…` embed paths inside JSON strings ( on Windows the `\\` escaping makes naive substring rewrites brittle ). Behavioural surprise: users sometimes re-run hoping a transient failure clears, and instant replay would defeat that.
- GC grouping must distinguish lib and test units even when cargo hands them the same `-Cmetadata=` ( the `tests/` subcommand normally generates distinct metadata, but stripping it or forcing a collision is observable ). `extract_entry_identity` scans `args_received` for `--test` and suffixes `program_kind` with `-test` when set ( same pattern as the rustc-vs-clippy separation ).

### `cache_restore` Filename-Rewrite Bug ( anchored end-of-stem )

When restoring a cache entry whose `extra-filename` differs from the caller's, the wrapper rewrites the stored output filenames. The original implementation used `String::replace`:

```rust
let expected_name = if stored_ef != parsed.extra_filename {
    f.replace(stored_ef, &parsed.extra_filename)
} else { f.clone() };
```

`String::replace` is **unanchored**. If `stored_ef` ( e.g. `-d` ) appeared inside the crate-name component of the filename, not just at the end where extra-filename actually lives, the substitution mangled the name. The rewritten path then did not exist on disk, so the parent compilation that loaded it via `--extern` failed with `E0463: can't find crate for X` against unrelated crates. The symptom: a clippy run failed against `serde` / `serde_spanned` / `toml_datetime` simultaneously, with reruns passing cleanly because the wrapper had moved on to a different cache entry shape.

Fix: `rewrite_extra_filename(filename, stored_ef, new_ef)` anchors to the end of the stem:

```rust
fn rewrite_extra_filename(filename: &str, stored_ef: &str, new_ef: &str) -> String {
    if stored_ef == new_ef || stored_ef.is_empty() { return filename.to_owned(); }
    let (stem, ext) = match filename.rfind('.') {
        Some(idx) => (&filename[..idx], &filename[idx..]),
        None => (filename, ""),
    };
    match stem.strip_suffix(stored_ef) {
        Some(prefix) => format!("{prefix}{new_ef}{ext}"),
        None => filename.to_owned(),
    }
}
```

Handles all four artefact shapes ( `lib<crate><ef>.rmeta`, `lib<crate><ef>.rlib`, `<crate><ef>.d`, bare `<crate><ef>` test binaries on Unix / `<crate><ef>.exe` on Windows ). When `stored_ef` is empty, refuses to rewrite ( avoids `String::replace("", _)`'s splat-between-every-char behaviour ). When the suffix does not match, leaves the filename alone so the existence check in the caller surfaces a clean miss instead of a corrupted name.

The same unanchored-replace pattern still exists for `.d` file content rewriting ( `content.replace(EXTFN_PLACEHOLDER, &parsed.extra_filename)` and the corresponding store-side inverse ). In practice extra-filename hashes are unique enough that collisions are unlikely, but it is the same class of bug. Tracked in TODO.md as `sccache-wrapper-d-file-rewrite`.

### Cross-`CARGO_TARGET_DIR` Cache Misses

When the same workspace is built under different `CARGO_TARGET_DIR` values, the wrapper used to rebuild every external crate from scratch, defeating its whole purpose. The agent cargo wrapper at `.agents/bin/cargo.sh` mints a per-session `CARGO_TARGET_DIR` like `.agents-workspace/tmp/target-claude-<pid>/`, so this hit every concurrent agent.

Diagnosis: counting cache entries grouped by `(crate_name, -Cmetadata=…)` showed duplicates with byte-identical `args_received` modulo `--out-dir` / `-L` / `--extern` paths. Diffing two `tracing` entries with `metadata=723a9f84b3315681`:

```
< @@WORKSPACE@@/target/debug/deps
> /tmp/target-tmp/debug/deps
…
< tracing_attributes=@@WORKSPACE@@/target/debug/deps/libtracing_attributes-4d22862bb3dd8ae3.dylib
> tracing_attributes=/tmp/target-tmp/debug/deps/libtracing_attributes-4d22862bb3dd8ae3.dylib
```

Identical filenames, different prefixes. `--out-dir` / `-L` are stripped from cache keys, but `--extern` paths matter.

Root cause in `compute_cache_key`'s `--extern` handling: it preferred the dep's sidecar `.cachekey` file ( workspace-independent ) but fell back to `normalise_arg(val, workspace_root)` when no sidecar existed. Two cases where sidecars are absent:

1. **Proc-macro deps.** `parse_rustc_args` rejects `crate-type=proc-macro` outright, so the wrapper never caches them and never writes a sidecar. Every consumer of a proc-macro dep ( basically every crate in this workspace, given derive macros are ubiquitous ) hit the fallback path.
2. **`CARGO_TARGET_DIR` outside the workspace.** When the target dir lives outside the workspace, `normalise_arg` cannot substitute anything, so the full absolute path lands in the hash.

Even *inside* the workspace, the per-session `target-claude-<pid>` subdir varies between sessions, so workspace-normalisation alone was not enough.

Fix: new helper `extern_basename_key` keys off the artefact's *basename* rather than its full path. The basename embeds cargo's metadata hash ( e.g. `libtracing_attributes-4d22862bb3dd8ae3.dylib` ), which is the part that genuinely disambiguates one compilation of a dep from another; the path prefix just records where cargo happens to be writing today.

```rust
fn extern_basename_key(val: &str) -> String {
    if let Some((name, path)) = val.split_once('=') {
        let basename = Path::new(path).file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_owned());
        format!("{name}={basename}")
    } else {
        val.to_owned()
    }
}
```

Replaced the fallback's `normalise_arg(val, workspace_root)` call with `extern_basename_key(val)`. Sidecar path remains preferred when available — it is stronger ( content-addressed ) than the basename path, which still trusts cargo's metadata hash to be deterministic across target dirs. If cargo ever starts hashing in `CARGO_TARGET_DIR` itself, the wrapper will need a different strategy ( probably mining the metadata hash directly from the filename rather than trusting the whole basename ).

Existing duplicate cache entries from before the fix stay until GC removes them. `sccache-wrapper --gc` groups by `(program_kind, crate_name, metadata)` and keeps the newest per group, so it converges automatically as builds touch each crate again. Manual `rm -rf .agents-workspace/tmp/winterbaume-rustc-cache` is fine if an immediate reset is desired.

### Bypassing `cargo.sh` Defeats Cross-Agent Cache Sharing

The wrapper only participates when Cargo is invoked through `.agents/bin/cargo.sh` or `.agents/bin/cargo.ps1`. Hand-setting `CARGO_TARGET_DIR` and running bare `cargo` creates a target directory that looks isolated but bypasses the workspace `RUSTC_WRAPPER`, `WB_WORKSPACE_ROOT`, and `WB_RUSTC_CACHE_DIR` exports. The result is a full fresh compile with no cache lookup, no hardlinks from `.agents-workspace/tmp/winterbaume-rustc-cache/`, and no `.cachekey` sidecars next to the produced `.rmeta` files.

The durable diagnostic is the sidecar count: zero `.cachekey` files in `<target>/debug/deps/` means `sccache-wrapper` did not run for that target. `.agents/bin/check-build-cache.sh` automates this check and exits non-zero with a "bare cargo was invoked" style diagnostic when the wrapper was bypassed. A clean-slate verification built `sccache-wrapper` twice through `cargo.sh` with two different `CLAUDE_CODE_SSE_PORT` values: first run populated 24 manifests in the rustc cache, second run completed 3.75x faster and representative `.rmeta` files had link count 3 ( cache + first session + second session ). Repeating the same build through bare `cargo` with wrapper env stripped produced zero sidecars and no speed-up, confirming the populated cache was unused.

Two adjacent caveats remain: ready scoreboard entries are only pruned during scoreboard read-modify-write paths, so pure cache-hit fast paths do not age them out; and `--gc` still runs only on explicit invocation, so duplicate cache entries remain until an operator runs GC or a future maintenance task adds a routine trigger.

### Stale sccache Server Recovery

A stale sccache server can remain running while no longer accepting socket connections. Builds then appear to hang inside unrelated crates.

Recovery sequence:

```sh
sccache --show-stats
pkill -f sccache
sccache --start-server
```

Retry the build after the server responds again. In multi-agent work, one stale shared server can block every worktree that routes through the wrapper.

## Files

- `tools/sccache-wrapper/src/main.rs`: cache key computation, incremental stripping, restore/store logic, singleflight locks, diagnostics, `--dump-cache`, and scoreboard registration / heartbeat / `--show-scoreboard`
- `tools/sccache-wrapper/Cargo.toml`: wrapper dependencies, including `sha2`, `fslock`, and `sysinfo`
- `tools/sccache-wrapper/README.md`: operator documentation for environment variables, cache layout, diagnostics, and the scoreboard
- `.agents/bin/check-build-cache.sh`: doctor script for verifying that a target dir was built through the workspace wrapper
- `.agents-workspace/tmp/sccache-wrapper-scoreboard/`: shared scoreboard directory written by every session ( `WB_SCCACHE_WRAPPER_SCOREBOARD` )
- `mise.toml`: project-local wrapper environment defaults
- `AGENTS.md`: required build setup and cache environment variables

## Test Coverage

- Cross-worktree cache hits were validated for workspace crates such as `winterbaume_core` and external crates such as `bytes`.
- Host and target duplicate builds were validated by confirming crates such as `parking_lot` produced distinct cache entries instead of hardlinked aliases.
- Fresh clean worktree builds succeeded after the `.cachekey` sidecar fix and no longer produced `E0460` or `E0463` dependency poisoning errors.
- `--dump-cache` was checked against entries written with dependency metadata and showed resolved deps such as `serde`, `serde_json`, `tracing`, and `winterbaume_core`.
- `rewrite_extra_filename`: 9 unit tests under `#[cfg(test)] mod tests` covering each artefact shape, the regression case ( `libred-d.rlib` with `stored_ef = "-d"` ), and no-op cases ( matching ef, non-matching suffix, empty stored_ef ).
- `extern_basename_key`: 5 unit tests covering workspace-internal target dir, per-session target dir ( asserts two different `target-<id>` paths produce identical keys ), target dir entirely outside workspace, proc-macro `.dylib`, no-`=` fallback.
- `--test` cache flow: end-to-end test invoked the rebuilt wrapper against `rustc` twice with `--test --crate-type lib --emit=dep-info,link --test -C extra-filename=-deadbeef`, into separate `out_dir`s and with a project-local cache dir. First run MISS + store ( `out_dir` contains `foo-deadbeef` with exec bit set, `foo-deadbeef.d`, and the `.cachekey` sidecar ). Second run HIT, `nlink=3` on the binary confirming hardlink restoration without copy. Restored binary `foo-deadbeef --list` prints `t: test` and exits 0.
- Test-bucket GC: forced two compilations through with `-C metadata=samehash` ( same hash on both invocations to force the worst case ), once as lib and once with `--test`, then ran `--gc --dry-run` and `--gc`. Both reported `nothing to collect`, and the cache directory still contained both entries afterwards. Without the fix this would have evicted the lib entry.
- Cross-agent cache-sharing harness check: `CLAUDE_CODE_SSE_PORT=verify-first ./.agents/bin/cargo.sh check -p sccache-wrapper` warmed the cache; `CLAUDE_CODE_SSE_PORT=verify-second ...` restored representative deps as hardlinks; a bare-cargo bypass build into `.agents-workspace/tmp/target-bypass-test` produced zero `.cachekey` sidecars and was correctly rejected by `.agents/bin/check-build-cache.sh`.

## Pitfalls

- Do not strip `-C metadata=...`; it is necessary for correctness even though it looks like local Cargo noise.
- Do not key parent crates only by `--extern` names. Dependency identity changes must invalidate parents.
- Do not assume `CARGO_INCREMENTAL=0` inside the wrapper disables incremental compilation; the rustc args must be rewritten.
- Do not rely on `SCCACHE_BASEDIRS` for Rust cross-worktree sharing.
- Do not run bare `cargo` with a hand-set `CARGO_TARGET_DIR`. That still bypasses the workspace wrapper and prevents cross-session cache sharing. Use `.agents/bin/cargo.sh` and run `.agents/bin/check-build-cache.sh` if a build unexpectedly recompiles warm dependencies.
- Do not debug long build hangs only at the current crate. First check whether the shared sccache server is responsive.
- Do not use `String::replace` to substitute the `extra-filename` suffix in stored output filenames or `.d` content. It is unanchored and will mangle names whenever the suffix appears mid-stem ( e.g. `stored_ef = "-d"` inside `libred-d.rlib` ). Use `rewrite_extra_filename` ( anchored to the end of the stem ) for filenames; the `.d` content path still uses `String::replace` and is tracked as `sccache-wrapper-d-file-rewrite` in TODO.md.
- Do not key `--extern` paths off the full path when the dep has no `.cachekey` sidecar ( proc-macros, target dirs outside the workspace ). Use the artefact basename, which embeds cargo's metadata hash. `extern_basename_key` is the canonical helper.
- Mixing wrapper feature sets ( e.g. flipping between `bundled` and prebuilt-libduckdb in the same session ) can resurrect stale `chrono` / `serde` artefacts and produce `E0463: can't find crate for serde` style errors. Workaround: wipe `.agents-workspace/tmp/winterbaume-rustc-cache/` and the session `target-*` directory when switching feature sets locally. CI is unaffected — each run starts from a clean cache.
- `sccache-wrapper --clear-cache` ( alias: `--reset` ) clears the configured cache directory and is the preferred recovery path for a poisoned cache. It is still destructive to the cache, so use it intentionally.
- `cargo check` against shared sccache state can report phantom errors like `can't find crate for arc_swap` in external dependencies. Forcing a `cargo build` of the same crate often resolves it. Treat single phantom-import errors as cache pollution unless they reproduce twice. The per-session `CARGO_TARGET_DIR` does not fully isolate when agents touch shared sccache state.
