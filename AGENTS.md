# Documents for both humans and coding agents

* [README.md](./README.md)

# Documents for coding agents

* [./.agents/docs/OVERVIEW.md](./.agents/docs/OVERVIEW.md) ... project overview.
* [./.agents/docs/ARCHITECTURE.md](./.agents/docs/ARCHITECTURE.md) ... system architecture.
* [./.agents/docs/JOURNAL.md](./.agents/docs/JOURNAL.md) ... findings insights, and peer code review history.
* [./.agents/docs/LTM/INDEX.md](./.agents/docs/LTM/INDEX.md) ... long-term memory index for durable project knowledge under `./.agents/docs/LTM/`.
* [./.agents/docs/services/INDEX.md](./.agents/docs/services/INDEX.md) ... AWS service research index. Read the relevant `./.agents/docs/services/<service>.md` before adding or enhancing a service crate, porting tests, writing Terraform converters, or changing service-specific behaviour.
* [./.agents/docs/QUALITY_GATE.md](./.agents/docs/QUALITY_GATE.md) ... quality gate checklist and implementation conventions (stub handler format, coverage counting rules, error shaping, codegen rules).
* [./.agents/docs/TODO.md](./.agents/docs/TODO.md) ... open to-do items extracted from JOURNAL.md during `good-sleep` consolidation. Check and update this file when picking up or finishing work.

# Rules and protocols

## General

* ❌ Don't use mise. It's for humans.
* When working on an AWS service, first map the crate/service name through `./.agents/docs/services/INDEX.md` and read the matching `./.agents/docs/services/<service>.md`. Treat it as the local service dossier: Smithy model slug, AWS SDK for Rust slug, protocol, operation/resource inventory, AWS documentation research, Winterbaume LTM notes, and parity checklist. Use it to choose scope, identify service-specific pitfalls, and avoid rediscovering already-recorded research.

## File Management

* When you'd make summary documents for your work, be sure to write them under `./.agents/docs`, not under `/tmp`.
* Temporary files should be created under `./.agents-workspace/tmp`, not under `/tmp`.
* ❌ Do not randomly create a binary under the version controlled directory through `go build ./cmd/s3router`. Always put it under `./.agents-workspace/tmp`.
* ❌ Never delete user files without permission. Only safe to delete: files YOU created in THIS session that are in `./.agents-workspace/tmp/`. Always ask first if unsure. Assume all pre-existing files belong to user.

## Generated Source Files

* Any `wire.rs` or `model.rs` with the header `//! Do not edit manually. Regenerate with: smithy-codegen gen-serializers <service>` is **auto-generated**. Never edit these files directly.
* To fix a bug in generated output, edit the generator at `tools/smithy-codegen/src/gen_serializers.rs`, then regenerate:
  ```
  cargo run -p smithy-codegen -- gen-serializers <model-dir> \
      --output crates/winterbaume-<crate>/src/wire.rs \
      --model-output crates/winterbaume-<crate>/src/model.rs
  ```
* The model directory name is NOT always the same as the Rust crate name. Use `cargo run -p smithy-codegen -- list-services` to find the mapping (e.g., `elbv2` → `elastic-load-balancing-v2`, `route53` → `route-53`).
* After editing the generator, rebuild it first (`cargo build -p smithy-codegen`) and then regenerate all affected crates.

## Building

* We use `sccache` to optimise builds. As it's quite sensitive about the source file sums, make sure that `cargo fmt` is applied to all the source files changed before running `cargo check`, `cargo build`, and `cargo test`.
* `sccache-wrapper` (source in `tools/sccache-wrapper/`) provides two layers of build caching:
  1. **Cross-worktree cache** — normalises absolute paths in rustc arguments and hashes source file contents to produce a workspace-independent cache key. Cached artifacts are restored via hardlinks (zero-copy, no disk duplication). This layer is what makes builds in new worktrees fast.
  2. **sccache pass-through** — sets `SCCACHE_BASEDIRS` and strips `-C incremental=…` from rustc args (incremental compilation is incompatible with sccache), then delegates to the real `sccache` binary for same-tree rebuild caching.
* **Always invoke cargo via the agent wrapper, not bare `cargo`.** The wrapper mints a per-session `CARGO_TARGET_DIR` so concurrent agents do not contend on the same `target/.cargo-lock`, builds `sccache-wrapper` on first use, and exports `RUSTC_WRAPPER`, `WB_WORKSPACE_ROOT`, `WB_RUSTC_CACHE_DIR`, and `CARGO_INCREMENTAL=0` for you.
  * Unix (bash/zsh): `./.agents/bin/cargo.sh <args>`
  * Windows (PowerShell): `./.agents/bin/cargo.ps1 <args>`
* The wrapper resolves a session identifier from `$CLAUDE_CODE_SSE_PORT` (Claude Code) or by walking the parent-process chain to find a `claude` or `codex` ancestor, falling back to the shell PID. The resulting target directory is `.agents-workspace/tmp/target-<session>/`. Stale directories accumulate over time; clean `.agents-workspace/tmp/` manually when it grows.
* ❌ Never run bare `cargo` from an agent shell, even when you set `CARGO_TARGET_DIR=...` yourself to a fresh path. Bare `cargo` does NOT inherit `RUSTC_WRAPPER` / `WB_WORKSPACE_ROOT` / `WB_RUSTC_CACHE_DIR` from your shell unless `cargo.sh` set them, so rustc bypasses `sccache-wrapper` entirely: no entries in `winterbaume-rustc-cache/` are consulted, no `.cachekey` sidecars are written next to the produced `.rmeta`, and the resulting artefacts are NOT hardlinked from the shared cache. Concurrent agents will then redundantly rebuild byte-identical artefacts. The fix is always the same: invoke `./.agents/bin/cargo.sh ...` and let it mint the target dir for you. See the 2026-04-30 JOURNAL entry "sccache-wrapper cache not shared across agents" for the diagnostic walkthrough.
* **Self-check after a suspect build.** Run `./.agents/bin/check-build-cache.sh` ( or, manually, `ls "$CARGO_TARGET_DIR/debug/deps"/*.cachekey | wc -l` ). Zero `.cachekey` sidecars next to `.rmeta` files means the wrapper did not run for that target dir; rerun the build through `cargo.sh` before trusting timing or cache-hit observations.
* When a build appears to be stuck or running far longer than expected, inspect the shared scoreboard at `.agents-workspace/tmp/sccache-wrapper-scoreboard/` ( the agent cargo wrappers already export `WB_SCCACHE_WRAPPER_SCOREBOARD` to that path, so every session writes into the same directory and can see the others ). Each in-flight cache-miss compilation drops a `<crate>-<short_key>.json` file there with the rustc command line, build state ( `building` / `ready` ), elapsed-or-final duration, and the contending sessions ( each with the caller process tree like `caller -> cargo -> sccache-wrapper` ). Useful when you need to tell whether a session is genuinely compiling, waiting on another agent's singleflight lock, or has died abandoning a half-built crate.
  * Inspect live: `ls .agents-workspace/tmp/sccache-wrapper-scoreboard/` then `cat <file>.json`, or `.agents-workspace/tmp/sccache-wrapper --show-scoreboard` for a human-readable summary.
  * Stale `building` entries with no recent heartbeat ( > 30 s ) and `ready` entries older than 5 min are pruned automatically by the next session that touches the directory.

## Testing

* Make sure that regression tests are ready for your fix.

## Local Lint Gate (per-crate)

Before reporting any Rust code change as done — and this applies to subagents as well — you must run the per-crate clippy and rustfmt gate on every crate you modified:

```
./.agents/bin/cargo.sh clippy -p <crate> --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh fmt -p <crate> -- --check
```

* If clippy fails, fix the violations and re-run until it passes. Do not declare the change complete with outstanding clippy errors.
* If `fmt --check` fails, run `./.agents/bin/cargo.sh fmt -p <crate>` to apply the formatter.
* When a change spans several crates, run the gate for each touched crate; cargo will only re-check what changed, so the cost stays small.
* The workspace-wide gate (`cargo clippy --workspace ...`) is expensive (20+ minutes) and is the CI's job. Don't reach for it as part of routine code-change verification — the per-crate gate is sufficient there. It is appropriate during quality-assurance tasks (e.g., the `quality-gate` and `verify-publish-ready` skills, or before tagging a release) where workspace-level coverage is the explicit goal.
* When delegating Rust changes to a subagent, instruct it to run this same gate before returning, and have it include the result in its final report.

## Python

* If there's a `pyproject.toml` file, try to run the tests with `uv run pytest ...` and arbitrary scripts with `uv run python ...`.
  * ❌ If there's no `pyproject.toml`, never run a bare `pip install` out of a venv. Always use `uv pip ...` in combination with `uv venv`.
* For typed Python tests:
  * Keep fixture return types as `Iterator[Type]` for yield fixtures.
  * Avoid `dict[str, object]` and `list[object]` in annotations; use `dict[str, Any]`/`list[Any]` for mixed payloads and real TypedDicts for structured shapes.
* ❌ You shouldn't run the entire integration test suites at once. Or if you can spare them 2+ minutes, be patient with it. You should always specify `--maxfail=n` (n should be a number less than 10), and also be sure to specify `--lf` as well when you want to run the last failing tests.

## Shell Pitfalls (prezto defaults)

The user's shell uses prezto, which sets aliases and options that break non-interactive scripts:

* ❌ `cp src dst` prompts interactively when `dst` exists (prezto aliases `cp` to `cp -i`). Always `rm -f dst` before `cp`. Also kill any process using the destination file first (e.g., `pkill -f winterbaume-server` before replacing the binary).
* ❌ `cat > file <<'EOF'` and `echo > file` fail with `file exists` when the target exists (prezto enables `NO_CLOBBER`). Workaround: `rm -f file` before writing, or use `tee` / `/bin/cat`.
* ❌ `rm file` prompts for confirmation on some files (prezto aliases `rm` to `rm -i`). Always use `rm -f` for non-interactive deletion.
* When running ad-hoc shell scripts that create terraform working directories, always `rm -rf` the entire directory before retrying — stale `.terraform.tfstate.lock.info` files will lock out new runs.

## Git Workflow

* ❌ Neither do `git checkout` nor `git restore`. The other coding agent is concurrently working on the same directory.
* ❌ Never make discretionary commits.

## Documentation

* Try to write your work summary to one of the existing documents.
* ❌ Avoid editing any existing sections of JOURNAL.md. You should rather just append texts to it. ( The sole exception is the `reconcile-journal-ltm` skill, which may remove entries that have already been consolidated into `.agents/docs/LTM/` per the canonical `## LTM Consolidation Record` table. )
* As the project name derives from German, use **British English** spellings throughout repo-authored documentation (e.g., "behaviour", "colour", "organise", "analyse", "centre", "modelled", "cancelled", "initialisation", "standardise", "stylised"). Technical programming terms that are conventionally American (e.g., `serialize` in Rust/serde context, API identifiers like `BehaviorVersion`) and AWS service names (e.g., "Organizations") are exempt.
* ❌ For repo-authored documentation only (e.g., `AGENTS.md`, `README.md`, `.agents/docs/**`), never use full-width parentheses (`（` `)`). Instead, use half-width parentheses (`(` `)`) with a half-width space being put before/after an open/close parenthesis when it's preceded/followed by a non-white-space character. This rule does **not** apply to generated or third-party reference files under `skills/**/references/**`.
* ❌ For repo-authored documentation only (e.g., `AGENTS.md`, `README.md`, `.agents/docs/**`), never use full-width colons (`：`). Instead, use a half-width colon followed by a half-width space. This rule does **not** apply to generated or third-party reference files under `skills/**/references/**`.
