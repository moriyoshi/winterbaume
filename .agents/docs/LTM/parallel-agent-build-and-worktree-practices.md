# Parallel Agent Build and Worktree Practices

## Summary

Once Winterbaume reached hundreds of crates and many parallel edits, durable operational knowledge became as important as service code. The project learned how to use worktree isolation safely, how to copy results back without clobbering concurrent work, and how to keep Rust builds fast enough for parallel agents by using `sccache-wrapper` — a checked-in Rust binary that provides both cross-worktree cache normalisation and sccache pass-through.

## Key Facts

- Worktree-isolated parallel agents are effective for independent service work, but merging generated files back requires care.
- `git apply --3way` can leave conflict markers in generated code and is often less reliable than copying specific files when write scopes do not overlap.
- Worktree results may be left uncommitted. In that case, the worktree directory still contains the changes even if no branch metadata was returned.
- Shell aliases in the user's environment make naive `cp`, `rm`, and redirect patterns unsafe for automation.
- `sccache-wrapper` (at `.agents-workspace/tmp/sccache-wrapper`) provides cross-worktree cache normalisation by rewriting absolute paths in rustc arguments to workspace-relative keys, then delegates to the real `sccache` binary. This makes cache hits work across worktrees without the old "Operation not permitted" failure mode.
- Bare `cargo` from any agent shell collides on `target/.cargo-lock` even when `CARGO_TARGET_DIR` is exported, because concurrent agents land in the same `$PWD` and pick the same target dir name. The repo ships `.agents/bin/cargo.sh` ( bash/zsh ) and `.agents/bin/cargo.ps1` ( PowerShell ) as the only sanctioned entry point: each agent gets a per-session target dir under `.agents-workspace/tmp/target-<session>/` and consistent `RUSTC_WRAPPER` / `WB_*` exports.
- Session id resolution order: `$CLAUDE_CODE_SSE_PORT`, then a `PPID` walk that looks for a `claude` or `codex` ancestor, then `$$`. The wrapper cleans nothing automatically, so target dirs accumulate in `.agents-workspace/tmp/` over time.
- Parallel sub-agents can stop mid-refactor when usage limits are hit. After any such abort, re-survey the affected crates and run a build before continuing; dispatch/handler signature mismatches often surface only at compile time.

## Details

### Worktree collaboration patterns

The journal established a durable workflow for parallel implementation:

- use worktree isolation for independent service or batch tasks
- keep ownership boundaries narrow
- expect generated files such as `wire.rs`, `model.rs`, and `lib.rs` to be merge hotspots
- re-read destination files before copyback because concurrent agents may have landed changes already

Common merge findings included:

- duplicate type definitions after parallel changes
- generated-file conflicts when both main and worktree branches regenerated serialisers
- hidden existing implementations in main that made direct copyback unsafe

### Safe copyback practice

When a worktree agent finishes without committing, the file content still exists in the worktree. The durable practice is:

- inspect the worktree output
- copy only the intended files
- avoid shell commands that may prompt or respect `noclobber`

The journal explicitly prefers `install -m 644` over `cp` for copying worktree files back into the main tree.

### Build acceleration

Parallel subagent work made cold Cargo builds too expensive. The durable build configuration is now entirely managed by `.agents/bin/cargo.sh`:

- `RUSTC_WRAPPER` points at `.agents-workspace/tmp/sccache-wrapper` (the checked-in Rust binary)
- `CARGO_INCREMENTAL=0` is always exported (incremental artefacts are incompatible with sccache caching)
- per-session `CARGO_TARGET_DIR` avoids lock contention between concurrent agents

Why the combination matters:

- `sccache-wrapper` normalises absolute paths in rustc arguments before hashing, so the same crate built from a worktree or the main checkout produces the same cache key
- disabling incremental compilation lets sccache serve whole-crate artefacts from cache, avoiding partial-rebuild mismatches
- per-session target directories mean agents never contend on `target/.cargo-lock`

There is no `.cargo/config.toml` in this repository; all build configuration is injected at invocation time by the cargo.sh wrapper.

### Per-session cargo wrapper ( `.agents/bin/cargo.{sh,ps1}` )

Multiple coding agents on the same machine ( Claude Code sessions and codex sessions, often several at once ) historically stalled on `target/.cargo-lock`. Two distinct failure modes were observed even after the `CARGO_TARGET_DIR="$PWD/target"` guidance landed:

1. A session that began Bash tool calls with `eval "$(mise env -s zsh 2>/dev/null)"` and then ran bare `cargo`. mise.toml does not export `CARGO_TARGET_DIR`, so cargo defaulted to the workspace `target/` and contended on the lock.
2. A session that correctly set `CARGO_TARGET_DIR=.agents-workspace/tmp/aiops-target` but then *reused that name* across unrelated tasks running in two sub-shells. `lsof` confirmed both held `aiops-target/debug/.cargo-lock` concurrently.

Documenting harder did not help. The sanctioned solution is a checked-in wrapper script that mints a per-session target dir from a session identifier:

| Wrapper | Resolves session id from |
|---------|--------------------------|
| `.agents/bin/cargo.sh` ( bash / zsh ) | `$CLAUDE_CODE_SSE_PORT` ( per-process unique under Claude Code ); fallback: `PPID` chain walk that looks for a `claude` or `codex` ancestor; final fallback: `$$`. |
| `.agents/bin/cargo.ps1` ( PowerShell ) | Same precedence as the bash version. |

The wrapper:

1. Builds `sccache-wrapper` to `.agents-workspace/tmp/sccache-wrapper` on first use ( or when `tools/sccache-wrapper/src/main.rs` or `Cargo.toml` is newer than the binary ). Bootstrap runs with `RUSTC_WRAPPER=` cleared and an isolated `CARGO_TARGET_DIR=.agents-workspace/tmp/sccache-wrapper-build` so it cannot recurse into itself.
2. Exports `CARGO_INCREMENTAL=0`, `RUSTC_WRAPPER` ( pointing at `.agents-workspace/tmp/sccache-wrapper` ), `WB_WORKSPACE_ROOT`, `WB_RUSTC_CACHE_DIR`, `WB_SCCACHE_WRAPPER_SCOREBOARD`, and the per-session `CARGO_TARGET_DIR=.agents-workspace/tmp/target-<session>/`.
3. `exec`s real cargo.

Discarded alternatives ( each rejected for one of: drift, silent failure, or violating the rule that CI keeps unmodified semantics ):

- per-agent worktree wrapper ( redundant with Claude Code's `isolation: worktree`; codex does not auto-create worktrees ).
- checked-in `.cargo/config.toml` ( silently routes every CI cargo invocation through the agent wrapper ).
- gitignored generated `.cargo/config.toml` ( drifts; silent failure on missing setup step ).
- documenting harder ( agents diverge from rules; even when followed, name collisions remained ).

**Open follow-up**: no automatic GC for `.agents-workspace/tmp/target-<session>/`. Cleanup is currently manual; the dirs accumulate as sessions come and go.

`CLAUDE_CODE_SSE_PORT` is not a contractual interface. If a future Claude Code release stops setting it, the `PPID` walk fallback degrades the session id to one that may collide between two long-lived shells under the same `claude` parent, but does not regress to the original shared-target contention.

### Environment-specific operational knowledge

The user's shell and local setup introduce real automation constraints:

- `cp` may prompt on overwrite
- bare redirect can fail on existing files
- stale Terraform lock files can block retries
- replacing a running binary can silently hang if the process still holds it

These are not incidental annoyances. They are durable constraints for agent automation in this repository.

### Mid-refactor sub-agent abort recovery

The 2026-05 wire-deserialiser adoption sweep exposed a failure mode specific to long parallel refactors: agents that hit usage limits can leave a crate halfway between old and new handler signatures. The repository may look plausible in text search, but `dispatch()` and handler functions no longer agree.

Recovery procedure:

1. Re-survey the assigned crate before editing. If the intended pattern is already complete ( for example `body.get = 0` ), verify and move on.
2. Build the directly affected crates before launching more edits. During the wire-deserialiser sweep, `winterbaume-greengrass`, `winterbaume-wafv2`, `winterbaume-backup`, and `winterbaume-macie2` all needed cleanup because earlier agents stopped after changing only part of the call graph.
3. Treat downstream failures in umbrella crates such as `winterbaume-server` and `winterbaume-terraform` as dependency symptoms until the direct service crate is clean.
4. Prompts for recovery agents should explicitly allow the case where a previous parallel run already completed most of the task: "If the count is already zero, just run the per-crate gate and report it."

This makes "trust but verify" concrete for shared-checkout parallelism: text-search deltas are not enough after an interrupted refactor; compile the crate that owns the signature boundary.

## Files

- `.agents/bin/cargo.sh`: the only sanctioned entry point for agent cargo invocations; exports all build env vars and bootstraps `sccache-wrapper`.
- `tools/sccache-wrapper/src/main.rs`: source of the sccache-wrapper binary; edit here to fix caching bugs, then the wrapper script rebuilds it automatically on next use.
- `.agents/bin/cargo.sh`: bash/zsh wrapper that mints a per-session `CARGO_TARGET_DIR` and bootstraps `sccache-wrapper`.
- `.agents/bin/cargo.ps1`: PowerShell variant of the same wrapper.
- `.agents-workspace/tmp/target-<session>/`: per-session target directories created by the wrapper.
- generated service files under `crates/winterbaume-*/src/`: frequent merge hotspots during parallel work.
- `.agents/docs/JOURNAL.md`: worktree-management findings and operational caveats.

## Test Coverage

This topic is mostly operational, but its success criteria show up in:

- reduced rebuild time during parallel service work
- successful cargo test runs after copyback from worktrees
- fewer manual rebuild cycles caused by generated-file merge mistakes

The sccache setup was validated by observed cache hits and reuse across repeated builds.

### Stale-Worktree Hazard

When the main repo has significant uncommitted changes and a worktree is created from the initial commit, the worktree starts from a stale snapshot. Two failure modes follow:

1. **Missing files**: uncommitted files (e.g., new converter `.rs` files) exist only in the main repo. They must be copied into the worktree manually before it can compile.
2. **Stale service crate versions**: a subagent may "fix" compilation failures by editing the worktree's old `state.rs`/`types.rs`/`views.rs`, then bulk-copying them back overwrites the current (newer) main-repo versions, introducing regressions.

The durable rule is: when `isolation: "worktree"` is used and the main repo has uncommitted changes, do not let worktree changes propagate back to service-crate implementation files without first diffing against the main-repo state. Changes to `main.rs`, `Cargo.toml`, and `mod.rs` registration files are generally safe to copy back; changes to service-crate source files are not.

Also: a stale build cache can produce phantom compile errors that disappear after `cargo clean`. When a worktree reports impossible failures, `cargo clean -p <crate>` in the main checkout is the first debugging step.

### Patch-Style Merge-Back Hazards

Patch-style merge-back from agent worktrees has its own failure modes, even when the patch appears to apply:

1. Unified diffs do not reliably create previously untracked files unless the diff has the right new-file headers. Always run `git -C <worktree> status --short` and explicitly copy every `??` file before declaring a merge complete.
2. `patch` can apply a hunk to the wrong repeated context. A real case added `aws-sdk-ec2` to the `winterbaume-iam` package block in `Cargo.lock` instead of `winterbaume-terraform` because the surrounding dependency context was non-unique.
3. `.rej` files must be read end-to-end even when the target file appears to contain the intended edit. A rejected `Cargo.toml` hunk may pair with a wrongly-applied `Cargo.lock` hunk.
4. Agent-side test reports are not enough after context drift. Re-run `cargo check -p <crate> --tests` and any newly-added tests by name in the main checkout after merge-back.
5. Do not trust a report that says a helper was implemented until the main-tree test proves the contract. The AppFlow reshape helper was initially only a key cameliser even though the report described singleton-block lifting; the SDK round-trip test caught the mismatch.

For mechanical merge-back, prefer `git apply` over `patch -p1` when possible because it fails atomically per file and refuses some ambiguous contexts. If the working tree state prevents that, treat the patch as intent and apply it semantically in the main checkout, then verify.

## Pitfalls

- Never assume a worktree agent committed its changes. Check the worktree contents directly.
- Do not overwrite generated files blindly if the main tree may have concurrent edits.
- Shared target directories are safe but can serialize identical crate builds. This is expected.
- If `incremental` is left enabled, `sccache` loses much of its value.
- If `sccache-wrapper` or sccache serves stale artefacts after a source edit, run `.agents/bin/cargo.sh clean -p <crate>` to force a rebuild of that crate.
- Do not let a worktree subagent edit service-crate implementation files when the main repo has uncommitted changes to those same files. Use `git diff` or `git status` to confirm no concurrent edits before accepting any copyback.
- Do not run bare `cargo` from any agent shell. Always go through `.agents/bin/cargo.sh` ( or `.ps1` ). Bare cargo defaults to the workspace `target/` and immediately contends on the lock with every other agent.
- Do not pick a sticky `CARGO_TARGET_DIR` name and reuse it across unrelated tasks. The wrapper's session-derived suffix exists precisely so that two sub-shells of the same session do not collide.
- A `cargo check` or `cargo build` started before a `Cargo.toml` edit holds the *old* manifest in memory. Mid-flight stale-manifest produces "unresolved module" errors that look like missing imports. Cancel the running build and rerun after edits land.
- Do not use patch-process success as proof that a worktree merge-back landed correctly. Check untracked files, rejected hunks, lock-file package blocks, and `--tests` compile output in the main checkout.
- Do not continue a parallel refactor after a usage-limit abort without first compiling the affected crate. Partial signature migrations are easy to miss in search output and cheap to catch with the per-crate build.
