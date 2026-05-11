# Repo Maintenance and Agent Workflows Synthesis

## Summary

Winterbaume's repo-level operational knowledge spans several areas that future agents repeatedly need together: safe parallel worktree collaboration, workspace-wide documentation and example maintenance, self-contained skill maintenance, GitHub Actions security and release hardening, and the journal-to-LTM memory workflows. The stable pattern is to treat these as one operator layer around the codebase: how to work safely, how to refresh generated docs, how to keep CI and the release path resistant to supply-chain compromise, how to publish crates in dependency order without hitting first-launch quota limits, how to match local verification to CI, how to keep repo skills portable and drift-checked, and how to keep project memory coherent without destroying traceability.

## Included Documents

| Document | Focus |
|----------|-------|
| [parallel-agent-build-and-worktree-practices.md](./parallel-agent-build-and-worktree-practices.md) | Parallel worktree workflow, copyback rules, and `sccache` caveats |
| [sccache-wrapper-cross-worktree-cache.md](./sccache-wrapper-cross-worktree-cache.md) | Custom Rust cache wrapper, cross-worktree cache keys, singleflight locking, diagnostics, and stale-server recovery |
| [workspace-readmes-and-service-examples.md](./workspace-readmes-and-service-examples.md) | Coverage and README regeneration, examples layout, and crate-name mapping pitfalls |
| [crate-publishing-and-release.md](./crate-publishing-and-release.md) | Crate publishing order, cargo-dist binary releases, per-crate READMEs, and metadata corrections |
| [winterbaume-skill-maintenance.md](./winterbaume-skill-maintenance.md) | Self-contained Winterbaume skills, embedded issue-template contracts, slug snapshots, drift CI, and Markdown template hygiene |
| [github-issue-triage-and-automation.md](./github-issue-triage-and-automation.md) | Service labels, issue forms, AI triage, stale feedback handling, and GitHub-native triage memory |
| [repo-security-and-supply-chain.md](./repo-security-and-supply-chain.md) | GHA action SHA pinning, cargo-dist installer SHA-256 verification, `GITHUB_TOKEN` scoping, and `workflow_run` chain documentation |
| [journal-ltm-maintenance-workflows.md](./journal-ltm-maintenance-workflows.md) | `good-sleep`, `deep-sleep`, `reconcile-journal-ltm`, and the canonical consolidation record rule |

## Stable Knowledge

- Worktree-isolated parallel agents are useful, but generated files and shared docs are merge hotspots that require narrow ownership and careful copyback.
- When the main repo has significant uncommitted changes and `isolation: "worktree"` is used, the worktree starts from a stale snapshot. Service-crate files modified in the worktree may conflict with newer main-repo versions on copyback. Safe practice: accept copyback of registration files ( `main.rs`, `Cargo.toml`, `mod.rs` ) but diff service-crate source files against the main-repo state before accepting them.
- `sccache` can serve stale object files after source edits in some environments. If build results look wrong after a code change, run `RUSTC_WRAPPER="" cargo clean -p <crate>` before retrying.
- Shell aliases and `noclobber` behaviour in the user's environment make naive `cp`, `rm`, and redirect patterns unsafe for automation.
- Shared Cargo targets plus `sccache` and `incremental = false` are the preferred build setup, but worktree-local `sccache` runs may fail on this macOS setup and should be re-run from the primary checkout.
- `tools/sccache-wrapper` is now the preferred Rust cache entry point. It normalises cross-worktree cache keys, strips incremental flags, records diagnostics, writes `.cachekey` sidecars for dependency identity, and uses singleflight locks to prevent duplicate compiles.
- The wrapper caches `--test` test-harness compilations: `is_test` is part of `ParsedArgs`, expected output files include `<name><ef>` ( `.exe` on Windows ) for `link`, and the manifest stores `exit:N` so `cache_restore` replays rustc's exit status faithfully ( though the `if exit_code == 0` gate still keeps negative caching off until stderr path-normalisation lands ). GC grouping suffixes `program_kind` with `-test` so lib and test units cannot collide even when cargo hands them the same `-Cmetadata=`.
- Output-filename rewriting in `cache_restore` uses the anchored helper `rewrite_extra_filename( filename, stored_ef, new_ef )` ( strip suffix from the end of the stem ), not `String::replace`. The original unanchored `String::replace` mangled crate names like `libred-d.rlib` when `stored_ef = "-d"`, producing `E0463: can't find crate for X` against unrelated crates. The same unanchored pattern still exists for `.d` content rewriting at the `EXTFN_PLACEHOLDER` substitution sites and is tracked as `sccache-wrapper-d-file-rewrite` in TODO.md.
- Cross-`CARGO_TARGET_DIR` cache keys: the wrapper now uses `extern_basename_key( val )` instead of `normalise_arg( val, workspace_root )` when a dep has no `.cachekey` sidecar. The basename embeds cargo's metadata hash ( e.g. `libtracing_attributes-4d22862bb3dd8ae3.dylib` ), which disambiguates compilations regardless of target-dir prefix. Without this, every consumer of a proc-macro dep ( basically every crate, since derive macros are ubiquitous ) hit the path-based fallback and rebuilt from scratch under each new `CARGO_TARGET_DIR`. If cargo ever starts hashing in `CARGO_TARGET_DIR` itself, the strategy will need to mine the metadata hash directly from the filename rather than trusting the whole basename.
- Mixing wrapper feature sets ( e.g. flipping between `bundled` and prebuilt-libduckdb in the same local session ) can resurrect stale `chrono` / `serde` artefacts producing `E0463`. Workaround: wipe `.agents-workspace/tmp/winterbaume-rustc-cache/` and the session `target-*` directory when switching feature sets locally. CI is unaffected — each run starts from a clean cache.
- `sccache-wrapper --clear-cache` ( alias: `--reset` ) clears the configured cache directory and is the preferred recovery path for a poisoned cache. Use it intentionally because it is still a destructive cache reset.
- `.agents/bin/check-build-cache.sh` is the quick doctor when a target dir unexpectedly recompiles warm dependencies. Zero `.cachekey` sidecars in `<target>/debug/deps/` means the workspace wrapper did not run, usually because bare `cargo` was invoked with a hand-set `CARGO_TARGET_DIR`. Hand-setting `CARGO_TARGET_DIR` is not a substitute for `.agents/bin/cargo.sh`; it does not export `RUSTC_WRAPPER`, `WB_WORKSPACE_ROOT`, `WB_RUSTC_CACHE_DIR`, or the shared scoreboard path.
- `cargo check` against shared sccache state can report phantom errors like `can't find crate for arc_swap` in external dependencies. Forcing a `cargo build` of the same crate often resolves it. Treat single phantom-import errors as cache pollution unless they reproduce twice; prefer `cargo build` over `cargo check` for verification when concurrent agents are active.
- If builds routed through the wrapper hang across agents, check the shared sccache server with `sccache --show-stats`; a stale server may need `pkill -f sccache` and `sccache --start-server`.
- Workspace documentation is generated at scale. API coverage is produced by `.agents/skills/api-coverage/scripts/generate_coverage.py`, and README regeneration is handled by `.agents/skills/update-readme/scripts/update_readme.py`.
- Generated per-service README content can now coexist with manual guidance through `<!-- BEGIN AUTO ... -->` / `<!-- END AUTO -->` blocks. Manual provider notes belong outside the generated block so regeneration preserves them.
- Service crate README generation can transcribe selected service-dossier sections by Smithy model slug. The current transcribed section is `## Current Network Resource Stub Semantics`; keep the service dossier as the source of truth and verify source-section count against generated README-section count after regeneration.
- `.agents/docs/API_COVERAGE.md` now spans operation coverage, integration-test coverage, and Terraform E2E coverage, so maintenance work includes parser heuristics and mapping-table upkeep as well as content regeneration.
- The supported-services pipeline no longer depends on `winterbaume-stubs`; coverage and README reporting now derive from real workspace crates plus explicit mapping tables and parser heuristics.
- The supported example layout is the root package's `examples/` directory, not a separate examples workspace crate.
- Crate renames are never "done" after Cargo builds. They also require updates to scripts, examples, server wiring, and any hardcoded name-mapping tables. The 2026-04-28 `winterbaume-databasemigrationservice` → `winterbaume-databasemigration` rename ( driven by the `aws-sdk-databasemigrationservice` v0.0.0 placeholder mistakenly being chosen over the published `aws-sdk-databasemigration` ) is the reference checklist: directory `git mv`, package name in the crate's `Cargo.toml`, workspace `Cargo.toml` ( members list appears twice, `workspace.dependencies`, root dev-deps on both winterbaume crate and SDK crate ), every consumer crate, the crate's own README and tests, top-level `README.md` + `docs/services/<slug>.md` + `docs/reference/services.md`, `.agents/docs/API_COVERAGE.md`, both active and historical `TODO.md` entries with cross-references back to the old name, skill scripts ( `generate_coverage.py` crate-to-model map, `update_readme.py` crate-to-name map + example-slug alias + server-mode CLI map + `_CRATE_NOTES` placeholder ), and any LTM documents that propagated the old slug. When picking a new `winterbaume-<service>` crate name, cross-check the published `aws-sdk-<slug>` on crates.io and verify version > 0.0.0 — v0.0.0 placeholder crates are misleading.
- The README "Protocol" column is driven by a hand-maintained `CRATE_DISPLAY_INFO` dict in `.agents/skills/update-readme/scripts/update_readme.py:22`. Crates missing from the dict get the lowercase short name and a literal `?`. Resolve missing entries by reading the `aws.protocols#...` trait directly from each service's Smithy model under `vendor/api-models-aws/models/<slug>/service.json`. Mind that the model directory name is not always the `winterbaume-<x>` suffix ( e.g. `applicationdiscoveryservice` → `application-discovery-service`, `arczonalshift` → `arc-zonal-shift`, `pcaconnectorscep` → `pca-connector-scep`, `route53recoverycluster` → `route53-recovery-cluster`, `autoscalingplans` → `auto-scaling-plans` ). Long-term, derive `(display_name, protocol)` from the Smithy model at generation time rather than hand-curating.
- Local verification must match CI invocations. `cargo clippy --workspace --all-targets` catches warnings in examples and tests that library-only runs miss.
- CI Clippy often aborts on the first crate error, so each fix cycle should be followed by another full local `--all-targets` run to expose the next hidden layer rather than assuming the previous failure was the only one.
- Examples that call handlers directly or bypass the AWS SDK may need root-package `[dev-dependencies]` updates for crates such as `bytes`, `http`, or `serde_json`.
- Terraform E2E tests in `crates/winterbaume-e2e-tests` should carry `#[ignore]` so the normal CI test job can compile them without requiring Terraform; the dedicated E2E job is the place that runs ignored tests.
- Coverage-parser upkeep includes upstream-specific heuristics and cache invalidation. The kumo integration is the reference case: public HTTP handler methods, not only `handleXxx` helpers, define the real operation surface.
- Some recurring CI fixes are intentionally local rather than architectural: targeted `#[allow(clippy::result_large_err)]` on `Result<Value, MockResponse>` parsing helpers, and structural assertions instead of hard-coded thresholds for counts that shrink as services graduate from stubs or placeholders.
- Clippy guidance is pattern-sensitive. For example, `sort_by_key` is only the right rewrite when the sort key is `Copy` or trivially extractable; do not bulk-convert string-field `sort_by` calls that would require cloning.
- `JOURNAL.md` is the chronological source log, `.agents/docs/TODO.md` holds unresolved follow-up work, and `.agents/docs/LTM/INDEX.md` is the index of durable long-term memory.
- The memory workflows have a strict split: `good-sleep` distills journal entries into topic docs, `deep-sleep` synthesizes overlapping LTM docs, and `reconcile-journal-ltm` audits coverage and normalizes the journal's consolidation metadata.
- `JOURNAL.md` should keep one canonical `## LTM Consolidation Record`; any refresh or addendum sections are temporary and should later be folded back through reconciliation.
- In this repo's current state, `JOURNAL.md` has intentionally been cleaned down to the title plus the canonical consolidation record. Skill-required append-only notes such as a `deep-sleep` record are temporary metadata; the next `reconcile-journal-ltm` pass should fold them back into the canonical record once their durable content is represented in LTM.
- When the LTM tree already has a sensible synthesis layer, prefer refreshing the existing synthesis documents and leaving some synthesis notes intentionally standalone rather than adding another summary tier.
- Short documentation rewrite records should update the existing documentation map and workflow memory rather than create standalone topic notes. `OVERVIEW.md` owns orientation, `ARCHITECTURE.md` owns implementation detail, and `QUALITY_GATE.md` owns the flat checklist.
- Cross-skill script imports are a maintenance smell. Skills may share concepts and output formats, but parser logic should generally live inside the skill that executes it.
- Crate publishing must follow dependency-graph order: `winterbaume-core` first (no internal deps), then standalone library crates, then service crates, then the umbrella `winterbaume` and `winterbaume-server` last. For first public release, use `tools/release-batch/`; it reads `cargo metadata`, drops `publish = false` packages, topologically sorts publishable crates, and runs cargo-release in quota-sized chunks.
- Crates.io checks the `publish_new` quota before cargo-release reaches any per-crate `pre-release-hook`, so hooks cannot throttle a first launch with hundreds of new crates. Either get a quota raise from `help@crates.io` or keep each `release-batch` chunk at or below the current quota (5 new crates in the May 2026 audit) with a full rate-window sleep between chunks.
- The first-launch release-batch shape validated in May 2026 is 240 publishable crates, chunk size 5, and 48 chunks. The driver defaults to a 660 second sleep, accepts `--cargo <path>` or `WB_CARGO` for executable injection, and keeps printed per-chunk commands in stub form as `$ cargo ...` while reporting the resolved executable once in the startup banner.
- `release-batch` recovery must account for partial side effects. 429s and `is already published to crates.io` failures can mean some crates uploaded before cargo-release reached its tag step. The driver prunes already-published crates from the retry chunk and backfills missing `<crate>-v<version>` tags for pruned crates.
- The crates.io HTTP API can lag cargo's registry-index view after a publish. Prefer cargo's `is already published to crates.io` output as the authoritative immediate retry signal, with HTTP probes as best-effort support.
- Plan-only `release-batch` runs do not need a version. Execute runs require a version argument, but it may be a concrete semver or a cargo-release level such as `patch`, `minor`, `major`, `release`, `alpha`, `beta`, or `rc`. Crates.io resumability checks only run for concrete semver values.
- Binary releases for `winterbaume-server` use cargo-dist, triggered by version tags. The release workflow gates on CI via a reusable workflow call — no artefacts build unless CI passes.
- The umbrella `winterbaume` root package must keep anchored `include` patterns ( for example `/Cargo.toml`, `/README.md`, `/src/**/*.rs` ) because bare patterns match nested files and can package `.agents/`, vendored Smithy models, docs, examples, and other repository content. Keep `autoexamples = false` while root `examples/` is intentionally excluded from the package whitelist.
- The umbrella package should not carry unused root dev-dependencies. crates.io counts dev-dependencies in the published manifest dependency limit; the May 2026 release removed 455 unused dev-dependencies because root examples were not packaged or auto-discovered.
- Each crate has its own `README.md` (no workspace-level `readme` inheritance). Service crate READMEs are generated by `update_readme.py` and include an umbrella crate reference and inline code examples.
- Public documentation should now show registry dependencies and `cargo install winterbaume-server` where appropriate. Local `path = "..."` placeholders are contributor-only guidance.
- `docs/services/*.md` is generated from per-crate READMEs. Fix generated service docs by changing README generation inputs, not by editing the generated VitePress pages.
- Winterbaume-specific skills should be self-contained when advertised that way. Embed issue-body templates, labeller regexes, and slug snapshots in the skill directory, then enforce drift with CI instead of reading `.github` files at skill runtime.
- `winterbaume-server` emits structured access logs through `tracing::info!` fields (`remote_addr`, `method`, `path`, `status`, `elapsed_ms`) rather than free-form access strings, so JSON log output can be enabled later without changing request handling.
- GitHub issue triage is now service-label driven. Bug and feature forms require an AWS SDK service slug, auto-labelling maps it to `service:<slug>`, and AI triage writes machine memory to the orphan `memory/triage` branch rather than external storage.
- The workspace license is `Apache-2.0` (not dual MIT/Apache). Only one `LICENSE` file exists at the root.
- The repo ships `.agents/bin/cargo.sh` ( bash/zsh ) and `.agents/bin/cargo.ps1` ( PowerShell ) as the only sanctioned `cargo` entry point for agents. Each agent gets a per-session `CARGO_TARGET_DIR=.agents-workspace/tmp/target-<session>/` and consistent `RUSTC_WRAPPER` / `WB_*` / `WB_SCCACHE_WRAPPER_SCOREBOARD` exports. Bare `cargo` from any agent shell collides on `target/.cargo-lock` even when `CARGO_TARGET_DIR` is exported, because concurrent agents land in the same `$PWD` and pick the same target dir name. Session id is resolved from `$CLAUDE_CODE_SSE_PORT`, then a `PPID` walk for a `claude`/`codex` ancestor, then `$$`. The wrapper does not GC; `.agents-workspace/tmp/target-<session>/` directories accumulate and need manual cleanup.
- The sccache-wrapper publishes a per-crate scoreboard at `WB_SCCACHE_WRAPPER_SCOREBOARD` ( default `.agents-workspace/tmp/sccache-wrapper-scoreboard/` ) so external observers can see in-flight compilations, contenders ( pid, role, session id, process tree ), and per-crate state ( `building` / `ready` ). Per-file fslock locks scope contention per-crate. `sccache-wrapper --show-scoreboard` renders a human-readable summary. Useful when a build looks stuck and you need to distinguish "genuinely compiling", "waiting on another agent's singleflight lock", and "abandoned by a dead session".
- The triage-bug workflow has a prompt-injection guardrail step *before* the AI triage call. The guardrail uses the strongest model exposed by `actions/ai-inference@v2` ( `openai/gpt-5` ), with a system prompt that enumerates concrete attack-family names ( Direct manipulation, Indirect / data-as-instruction, Encoded / obfuscated payloads including stylised-alphabet substitutions, Translation laundering including compositional / derivational construction, Fake delimiters, Tool / agent hijacking, Authority impersonation, Verdict / output coercion, Hidden-instruction surfaces ). Reply is one line of compact JSON `{"verdict":"safe"|"abuse","reason":...}`.
- The guardrail exposes a three-valued verdict on `steps.guardrail.outputs.verdict`: `safe` ( triage runs, no label ), `abuse` ( skip triage, apply `possible-abuse` label ), `inconclusive` ( skip triage, no label ). This is fail-closed for triage but fail-open for the label, because auto-labelling on every transient model error would cascade into sibling-workflow lockouts. Sibling workflows ( `auto-label-service`, `clear-feedback-label`, `record-outcome`, `stale-feedback` ) all gate on the absence of `possible-abuse` so a flagged issue stops driving downstream automation. A regex heuristic step was tried and removed: modern attackers easily evade keyword filters via paraphrase, transliteration, multi-language phrasing, and role-play framing.
- Every `uses:` reference across the nine workflow files is pinned to a commit SHA with the version tag in an inline comment. The pinning is auditable only as long as the SHA and the tag comment move together; renewals follow a documented bump procedure ( resolve the tag's commit via `gh api repos/<owner>/<repo>/git/refs/tags/<tag>`, update SHA and comment in the same commit, read the upstream changelog before any major bump ).
- `release.yml`'s `Install dist` step downloads the cargo-dist installer to a temp file, verifies it against a hardcoded `CARGO_DIST_INSTALLER_SHA256`, and only then executes it. Pipe-to-`sh` patterns were removed; `cargo install cargo-dist --locked` and vendoring were rejected ( compile cost, churn ). `CARGO_DIST_INSTALLER_URL` and `CARGO_DIST_INSTALLER_SHA256` must always move together. Recompute the digest with `curl --proto '=https' --tlsv1.2 -LsSf <url> | sha256sum` when bumping.
- `GITHUB_TOKEN` cannot be scoped below repository level. `record-outcome.yml` and `record-triage.yml` carry `contents: write` with an accepted-risk comment; the structural mitigation is branch protection on `main` so the bot can only push to `memory/triage`. Third-party token consumers ( e.g. `cloudflare/wrangler-action` ) get the smallest possible scope at the workflow's top-level `permissions:` block with an inline justification.
- `workflow_run` chains create an implicit dependency edge invisible at the consumer file. Each consumer ( e.g. `record-triage.yml` ) carries an inline comment naming the producing workflow and artefact, the blast radius ( append-only JSONL writes to `memory/triage` ), and why artefact contents are stored verbatim and never `eval`'d / sourced / fed to a shell.
- The `Install Rust non-interactively` step in `release.yml` deliberately curl-pipes `https://sh.rustup.rs`. This is rust-lang.org's canonical install path and only runs in container matrix entries; do not "fix" it without coordinating with the upstream guidance.
- The 2026-04-27 audit was driven by an LLM panel ( Backdoor Hunter, Supply Chain Inspector, Integrity Analyst ). Only consolidated, multi-reviewer-confirmed findings ( the cargo-dist `remote_fetch_execute` finding ) were treated as real defects. `binary_change` notes from the panel were artefacts of patch-size truncation and are not actionable.

## Operational Guidance

When doing repo-maintenance work:

1. Confirm whether the task touches code, generated docs, or project memory, because each has different safe-write rules.
2. If working in parallel, keep write scopes narrow and assume generated files, `lib.rs`, and repo-authored docs may have concurrent edits in the main tree.
3. Prefer inspecting and copying specific files from worktrees rather than applying broad patches or conflict-prone generated diffs.
4. If a worktree build hits an `sccache` permission failure, copy the changes back and verify from the main checkout.
5. If a wrapper-backed build hangs, validate sccache server health before assuming the current crate is the cause.
6. After service additions or coverage changes, refresh API coverage and regenerate the workspace and per-crate READMEs together. Keep manual per-crate README guidance outside the auto-generated block.
7. If a new Terraform E2E service or integration-test pattern is missing from `API_COVERAGE.md`, inspect the crate-mapping tables, upstream-parser heuristics, and cache state before assuming the tests are absent.
8. If the supported-services table shows duplicate rows, placeholder metadata, or missing services, inspect `CRATE_DISPLAY_INFO`, crate-to-model mappings, and report parser logic rather than looking for retired stub-crate metadata.
9. Keep handler-level `STUB[...]` work mentally separate from repo-level supported-service discovery. Stub annotations still matter for implementation and quality-gate work, but they no longer drive server registration or README rows.
10. If CI failures only appear under `--all-targets`, inspect examples and integration tests first, then verify that root-package `[dev-dependencies]` still cover any direct handler-driving examples.
11. Keep Terraform E2E tests ignored by default unless the normal unit-test job can satisfy their runtime dependencies; use the dedicated ignored-test job for real Terraform execution.
12. After touching examples, generated code, or test-only helpers, rerun the same CI-shaped checks locally, especially `cargo clippy --workspace --all-targets`.
13. Keep examples under `examples/` and verify that any crate-name or SDK-name mappings are still correct.
14. Always run `cargo` through `.agents/bin/cargo.sh` ( or `.ps1` on Windows ). Never invoke bare `cargo` from an agent shell, even with a custom `CARGO_TARGET_DIR`; that bypasses the workspace `RUSTC_WRAPPER` / `WB_*` exports, prevents cross-session cache sharing, and leaves zero `.cachekey` sidecars for `.agents/bin/check-build-cache.sh` to find. Never reuse a sticky `CARGO_TARGET_DIR` name across unrelated tasks; the wrapper's session-derived suffix is precisely the contention fix.
15. When a build looks stuck and you suspect cross-agent contention, inspect `.agents-workspace/tmp/sccache-wrapper-scoreboard/` directly or run `.agents-workspace/tmp/sccache-wrapper --show-scoreboard`. Live `building` entries with non-empty `contenders` indicate queueing on a singleflight lock; stale `building` entries past 30 s without a heartbeat indicate an abandoned session.
16. For triage-bug workflow changes, treat the prompt-injection guardrail as the security boundary. The hostile body must never reach the triage assistant; the classifier runs as a separate `actions/ai-inference@v2` step. Do not move detection into the triage system prompt, and do not pair the AI guardrail with a regex heuristic without a specific named attack family the model is known to miss.
17. For workflow-security work, treat action SHA pinning, cargo-dist installer SHA-256 verification, and `GITHUB_TOKEN` scoping as one bundle. Bump SHAs and tag comments together, recompute installer digests when the URL moves, and revisit branch protection on `main` whenever a recorder workflow's permission scope widens. Document `workflow_run` consumer edges inline so the trust boundary stays visible to future maintainers.
18. For memory maintenance:
   - use `good-sleep` when durable journal content is missing from LTM
   - use `deep-sleep` when many LTM docs overlap and need synthesis
   - when running `deep-sleep`, update the current synthesis layer first and only add a new synthesis tier if it materially reduces overlap
   - use `reconcile-journal-ltm` when the journal needs a coverage audit and one canonical consolidation record
   - preserve the current "canonical record only" journal shape unless the user explicitly asks to resume chronological logging
   - fold documentation rewrite records into the existing synthesis/workflow notes instead of making separate LTM files for each rewrite

## Files

- `.agents/bin/cargo.sh` and `.agents/bin/cargo.ps1`: per-session cargo wrappers (only sanctioned cargo entry point for agents); inject all build env vars and bootstrap `sccache-wrapper` — there is no `.cargo/config.toml`
- `.agents-workspace/tmp/target-<session>/`: per-session target dirs minted by the wrapper
- `.agents-workspace/tmp/sccache-wrapper-scoreboard/`: shared scoreboard directory for in-flight compilations
- `.agents/docs/JOURNAL.md`: canonical consolidation record; the chronological body has been removed at user request
- `.agents/docs/LTM/INDEX.md`: synthesis and source-topic inventory
- `.agents/docs/TODO.md`: unresolved follow-up items extracted from journal work
- `tools/sccache-wrapper/`: cross-worktree Rust cache wrapper and diagnostics
- `.agents/skills/api-coverage/scripts/generate_coverage.py`: coverage generation
- `.agents/skills/test-coverage/scripts/test_coverage.py`: narrower integration-test coverage tooling
- `.agents/skills/update-readme/scripts/update_readme.py`: root and per-crate README regeneration
- `.agents/skills/update-readme/SKILL.md`: README generator contract, including dossier-section transcription
- `skills/winterbaume-bug/SKILL.md` and `skills/winterbaume-bug/references/service-slugs.txt`: self-contained bug-report skill and service-slug snapshot
- `.github/workflows/skill-slug-drift.yml`: CI guard for the bug skill's embedded slug snapshot
- `crates/winterbaume-server/src/main.rs`: structured access logging and request timing
- `tools/release-batch/`: dependency-ordered chunked cargo-release driver for first public crates.io release
- `scripts/publish-all.py`: older dependency-ordered batch publish script
- `RELEASE.md`: release checklist and first-launch release-batch operator guidance
- `dist-workspace.toml`: cargo-dist configuration for binary releases
- `.github/workflows/release.yml`: cargo-dist release workflow
- `.github/workflows/triage-bug.yml`, `record-triage.yml`, `record-outcome.yml`, `stale-feedback.yml`, and `clear-feedback-label.yml`: GitHub-native triage automation
- `.github/workflows/*.yml`: every `uses:` reference is SHA-pinned with an inline tag comment; `release.yml` carries the cargo-dist installer SHA-256 pin; `record-outcome.yml` and `record-triage.yml` carry the accepted-risk `contents: write` comments; `deploy-docs.yml` carries the `cloudflare/wrangler-action` scope justification
- `.github/ISSUE_TEMPLATE/bug_report.yml`, `feature_request.yml`, and `kb-entry.yml`: issue intake and knowledge-base forms
- `.agents/docs/TRIAGE.md`: AI triage protocol
- `README.md`: workspace overview and supported-services table
- `examples/*.rs`: per-service example binaries in the supported root-package layout

## Tests

This area is mostly operational, but useful verification points include:

```bash
cargo build --examples -p winterbaume
cargo clippy --workspace --all-targets
rg '^## LTM Consolidation Record' .agents/docs/JOURNAL.md
```

Success criteria are mostly document- and workflow-level:

- worktree changes copy back without clobbering unrelated edits
- generated README updates reflect current coverage and crate names
- manually-authored README sections outside auto markers survive regeneration
- coverage reports still reflect newly added integration and Terraform suites after mapping and parser changes
- `JOURNAL.md` has one canonical consolidation record after reconciliation
- LTM syntheses and source-topic docs stay aligned in `INDEX.md`

## Pitfalls

- Do not assume a worktree agent committed its changes. Check the worktree contents directly.
- Do not overwrite generated files or repo-authored docs blindly if the main tree may have concurrent edits.
- Do not create a separate examples workspace crate when the root package can host `examples/` directly.
- Do not trust old journal consolidation tables without checking the referenced LTM documents.
- Do not scatter new memory-maintenance metadata across multiple journal sections when a canonical record already exists.
- Do not add a new synthesis layer by reflex when a small refresh to the existing synthesis documents would capture the same durable guidance.
- Do not assume missing coverage rows mean missing tests; stale parser heuristics or crate-mapping tables are common operator-side failure modes.
- Do not give the LLM-driven triage workflow direct contents write permissions; recorder workflows own GitHub branch writes.
- Do not treat a stale shared sccache server as a crate-local compile problem.
- Do not look for `winterbaume-stubs` when debugging supported-services reporting; that crate and its stub-row path were retired.
- Do not rely on the first CI Clippy failure as the whole problem set; later crates may still fail once the first error is removed.
- Do not leave Terraform E2E tests unignored in the normal test job unless that job also installs Terraform.
- Do not use hard-coded lower bounds for dynamic registry sizes when the real structural invariant can be asserted directly.
- Do not assume examples compile just because the service crate builds; `--all-targets` is what exposes example and test drift.
- Do not publish crates out of dependency order; downstream crates will fail to resolve unpublished internal deps.
- Do not rely on cargo-release hooks to beat crates.io's first-launch `publish_new` quota check; use `tools/release-batch/` or obtain a quota raise.
- Do not assume a published crate was tagged if cargo-release aborted mid-chunk. Check and backfill tags when retry pruning skips crates that landed in an earlier attempt.
- Do not let the umbrella crate package the whole repository. Root package include patterns must stay anchored, and `autoexamples = false` must stay in place while examples are excluded.
- Do not let unused root-package dev-dependencies push the umbrella crate over crates.io's dependency limit.
- Do not hand-edit generated per-crate READMEs; regenerate via `update_readme.py` instead.
- Do not hand-edit generated `docs/services/*.md`; fix the per-crate README source and regenerate.
- Do not put manual README guidance inside the generated auto block.
- Do not make a self-contained skill read repository templates or workflow files during normal execution. Mirror the contract inside the skill and test drift separately.
- Do not parse server access logs from message text when structured tracing fields are available.
- The DMS example uses raw `MockRequest`/`MockService` calls. Note that `aws-sdk-databasemigrationservice` ( with trailing `service` ) is a v0.0.0 placeholder crate; the real AWS SDK Rust client is published as `aws-sdk-databasemigration` and the winterbaume crate was renamed to `winterbaume-databasemigration` to match. With `aws-sdk-databasemigration` already in dev-deps as a real v1.x SDK, the QG-8.1 TODO ( "tests cannot be ported until AWS ships a real Rust DMS SDK" ) is now actionable.
- Do not assume `?` in the README Protocol column means coverage tooling cannot detect the protocol. It almost always means a missing entry in `CRATE_DISPLAY_INFO`. Read the `aws.protocols#...` trait directly from `vendor/api-models-aws/models/<service>/service.json` to determine the right value, then append the entry.
- Do not use `String::replace` to substitute the `extra-filename` suffix in stored output filenames or `.d` content. It is unanchored and will mangle names whenever the suffix appears mid-stem ( e.g. `stored_ef = "-d"` inside `libred-d.rlib` ). Use `rewrite_extra_filename` ( anchored to the end of the stem ) for filenames; the `.d` content path still uses `String::replace` and is tracked as `sccache-wrapper-d-file-rewrite` in TODO.md.
- Do not key `--extern` paths off the full path when the dep has no `.cachekey` sidecar ( proc-macros, target dirs outside the workspace ). Use `extern_basename_key` instead — the artefact basename embeds cargo's metadata hash and is workspace-independent.
- When fixing incorrect API parameter names or handler bugs, always verify against the official AWS documentation (via the AWS Documentation MCP Server or the API Reference) rather than relying on inference from the code alone. Hallucinated parameter names that read plausibly (e.g. `StackNameOrId` instead of the real `StackName`) are a known class of LLM-generated bug.
- When writing integration tests that call APIs requiring a pre-existing resource, always create the resource first. Missing setup steps are a common LLM generation error, especially in batch-generated test suites.
- Do not run bare `cargo` from any agent shell. The wrapper at `.agents/bin/cargo.sh` ( `.ps1` on Windows ) is the only sanctioned entry point; hand-setting `CARGO_TARGET_DIR` is not a substitute.
- Do not reuse a sticky `CARGO_TARGET_DIR` name across unrelated tasks; the wrapper's per-session suffix exists to prevent two sub-shells of the same session from colliding.
- Do not move guardrail detection into the AI triage system prompt. The hostile issue body must never reach the triage assistant; a separate classifier step is the only sound design.
- Do not fail-closed for the `possible-abuse` label on transient model errors. Auto-labelling on every API outage cascades into sibling-workflow lockouts; the `inconclusive` verdict skips triage *without* the label so a maintainer can review.
- Do not bump an action's `uses:` SHA without also updating the inline tag comment; the comment is what keeps the pinning auditable.
- Do not bump cargo-dist's installer URL without recomputing the SHA-256 digest in the same commit. The pinning is only useful when the two values move together.
- Do not widen `GITHUB_TOKEN` permissions casually. Branch protection is the only structural mitigation for `contents: write` on the recorder workflows; widening the scope without revisiting branch protection is a regression.
- Do not assume the LLM-audit panel's `binary_change` notes are actionable; they are an artefact of patch-size truncation. Treat only multi-reviewer-confirmed findings as real defects.
- Do not "fix" the rustup curl-pipe in `release.yml`; it is the documented install path for rust-lang.org and only runs in container matrix entries.
