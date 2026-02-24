---
name: quality-gate
description: Check whether a service crate passes the publication quality gate, then fix all failures. Reads the crate's source files, runs build/clippy/test, reports pass/fail for each gate check, creates a TODO list of failures, and works through them until the crate passes.
argument-hint: <service-name>
user_invocable: true
---

# Quality Gate Check and Fix

Evaluate a single service crate against the publication quality gate defined in `.agents/docs/QUALITY_GATE.md`, then fix every failure until the crate passes.

## Arguments

- `$0` — Service name matching the winterbaume crate suffix (e.g., `s3`, `iam`, `dynamodb`, `sqs`). The crate directory is `crates/winterbaume-{service}/`.

---

## Procedure

Work through the gate sections below in order. For each check, emit a verdict line:

```
✅ QG-{N}.{M}: {short description}
⚠️  QG-{N}.{M}: {short description} — {reason}
❌ QG-{N}.{M}: {short description} — {reason}
```

Use ✅ for pass, ⚠️ for non-blocking observation, ❌ for fail. At the end, print a summary table.

Some gate sections (9 Terraform E2E, 13 Coverage Reporting, 14 Server and Examples, 15 Workspace-Level) are workspace-wide and cannot be evaluated per-crate. Skip those sections with a note.

---

### Step 1: Read the quality gate document

Read `.agents/docs/QUALITY_GATE.md` to load the full gate criteria.

### Step 2: Read the crate source files

Read all of these files (some may not exist — that itself is a finding):

```
crates/winterbaume-{service}/src/lib.rs
crates/winterbaume-{service}/src/handlers.rs
crates/winterbaume-{service}/src/state.rs
crates/winterbaume-{service}/src/types.rs
crates/winterbaume-{service}/src/views.rs
crates/winterbaume-{service}/src/model.rs   (first 20 lines only — check header)
crates/winterbaume-{service}/src/wire.rs    (first 20 lines only — check header)
crates/winterbaume-{service}/src/backend.rs (if exists)
crates/winterbaume-{service}/Cargo.toml
```

Also check whether the integration test file exists:

```
crates/winterbaume-{service}/tests/integration_test.rs
```

### Step 3: Run build, clippy, and format checks (QG §1)

Run these commands and report results:

```bash
cargo check -p winterbaume-{service} 2>&1
cargo clippy -p winterbaume-{service} -- -D warnings 2>&1
cargo fmt -p winterbaume-{service} -- --check 2>&1
```

For generated files (`model.rs`, `wire.rs`):
- Check that the file header says `//! Do not edit manually.`
- Check that `model.rs` carries `#![allow(non_camel_case_types, clippy::upper_case_acronyms)]` if present.
- Check that `wire.rs` carries `#![allow(dead_code)]` if the crate has STUB handlers.

### Step 4: Crate structure conformance (QG §2)

Verify the canonical module layout in `lib.rs`:
- `pub mod handlers;`
- `pub mod state;`
- `pub mod types;`
- `pub mod views;`
- `pub mod model;` and `pub mod wire;` (when generated files exist)
- optionally `pub mod backend;`

#### §2 cross-call invariant inventory check

Per the canonical `QUALITY_GATE.md` §2 rule, `tests/scenario_test.rs` is mandatory and must contain at least one scenario per non-N/A row of the invariant inventory recorded by `/write-tests` ( Step 1d ).

1. Confirm `tests/scenario_test.rs` exists. If missing → ❌ FAIL ( fix: invoke `/write-tests {service}` ).
2. Locate the most recent `Test plan: {service}` section in `.agents/docs/JOURNAL.md`. Inside that section, find the **Cross-call invariant inventory** table.
   - If the inventory is missing → ❌ FAIL. The crate has not been processed by the current `/write-tests` skill version. Fix: re-run `/write-tests {service}` to generate it.
3. For every row of the inventory whose value is **not** `N/A — <reason>`, search `tests/scenario_test.rs` for a scenario whose doc comment names that row's category ( for example `Invariant: toggle propagation` ). Each non-N/A row must have ≥ 1 covering scenario.
   - Missing rows → ❌ FAIL. The fix is to author the scenarios; do **not** flip rows to N/A as a workaround unless the row is genuinely undocumented in AWS, in which case justify with a doc URL.
4. Stateless services may have all-N/A inventories; the inventory is still required, with one-line reasons + doc URLs per row. An empty / absent inventory is never acceptable.

Lifecycle tests in `tests/integration_test.rs` do **not** substitute. Invariant coverage is the property this gate enforces, and lifecycle coverage does not exercise cross-call invariants. The previous "5-resource lifecycle test may substitute" carve-out is removed.

### Step 5: State error shaping (QG §3)

Inspect `state.rs`:
- Check whether there is a `thiserror`-derived error enum.
- Search for `error_type:` or `status:` field constructions inside `state.rs` — these indicate the old anti-pattern.
- Search for `fn error_type` or `fn status` methods on the error enum.
- In `handlers.rs`, check that the error-shaping function uses a match on the enum and calls `err.to_string()`.

```bash
rg 'error_type:|status:\s*\d' crates/winterbaume-{service}/src/state.rs
rg 'fn error_type|fn status\b' crates/winterbaume-{service}/src/state.rs
rg 'err\.to_string\(\)' crates/winterbaume-{service}/src/handlers.rs
```

### Step 6: StatefulService contract and state views (QG §4)

Inspect `views.rs`:
- Confirm a `*StateView` type exists and implements `Serialize + DeserializeOwned`.
- Confirm `StatefulService` is implemented (`snapshot`, `restore`, `merge`).
- Check for `#[serde(default)]` on collection fields.
- In `handlers.rs`, search for `.await` calls that might hold lock guards:

```bash
rg 'notify_state_changed' crates/winterbaume-{service}/src/handlers.rs
```

### Step 7: Stub handler discipline (QG §5)

Search for stub patterns:
```bash
rg 'STUB\[' crates/winterbaume-{service}/src/handlers.rs --count
rg 'Default::default\(\)' crates/winterbaume-{service}/src/handlers.rs -n
rg 'next-token-placeholder' crates/winterbaume-{service}/src/handlers.rs
```

- Every `Default::default()` in a handler response should either be preceded by a `// STUB[category]` comment or be part of a `_to_model` conversion.
- No placeholder pagination tokens.

### Step 8: Pluggable backend conformance (QG §6)

Only applicable if `src/backend.rs` exists. If it does:
- Confirm the trait is object-safe (returns `Pin<Box<dyn Future<...> + Send>>`).
- Confirm `new()` uses the in-memory adapter.
- Confirm `with_backend(...)` or `with_query_backend(...)` exists.

### Step 9: Smithy codegen integrity (QG §7)

- Verify `model.rs` and `wire.rs` have the `//! Do not edit manually.` header.
- Search `handlers.rs` for hand-written JSON or XML response construction:

```bash
rg 'json!\(\{' crates/winterbaume-{service}/src/handlers.rs
rg 'xml_response\(' crates/winterbaume-{service}/src/handlers.rs
rg 'format!.*<.*Response>' crates/winterbaume-{service}/src/handlers.rs
```

Report any remaining hand-written responses where generated `wire::serialize_*` functions should be used instead.

### Step 10: Integration test coverage (QG §8)

- Confirm `tests/integration_test.rs` exists and is non-empty.
- Search for `FIX(terraform-e2e)` markers in handler and state code and confirm corresponding tests exist:

```bash
rg 'FIX\(terraform-e2e\)' crates/winterbaume-{service}/src/ -n
```

- Run the integration tests:

```bash
cargo test -p winterbaume-{service} 2>&1
```

Report the test count and pass/fail status.

### Step 11: Blob-backed service correctness (QG §10)

Only applicable if the service uses `BlobStore` (check for `blob_store` or `BlobStore` in the source). If it does:
- Verify state views reference blobs by key, not by embedding content.
- Verify `BlobBackedService` is implemented.

### Step 12: State view fidelity (QG §11)

Inspect `views.rs` for:
- Whether views capture the full durable resource configuration or only a narrow subset.
- Proper serialisation conventions (`Vec<u8>` as hex, `DateTime` as RFC 3339, enums as stable strings).
- Whether `serde_json::Value` fields are justified (opaque wire-shaped blobs) or should be typed.

### Step 13: Skip workspace-wide sections

The following sections are workspace-wide and not evaluated per-crate. Note them as skipped:

- §9 Terraform E2E Coverage
- §12 Rule Evaluator and Validator Crates (unless the crate IS one)
- §13 Coverage Reporting and Documentation
- §14 Server and Examples
- §15 Workspace-Level Checks

---

## Phase A Output: Initial Report

After all checks, produce a summary:

```
# Quality Gate Report: winterbaume-{service}

## Summary

| Result | Count |
|--------|-------|
| ✅ Pass | N |
| ⚠️ Warning | N |
| ❌ Fail | N |
| ⏭️ Skipped | N |

## Verdict: PASS / FAIL

## Details

(all verdict lines from above, grouped by section)
```

---

## Phase B: Remediation

If there are any ❌ failures, work through them until the crate passes.

### Step 14: Create a TODO list

Create tasks for every ❌ failure, ordered by dependency (e.g., fix state error shaping before fixing handlers, fix compilation before running tests). Each task should be a concrete, actionable item:

```
TODO-1: [QG-3.1] Replace {Service}Error struct with thiserror enum in state.rs
TODO-2: [QG-3.2] Update error-shaping function in handlers.rs to match on enum
TODO-3: [QG-5.1] Add STUB[no-state] annotation to handle_foo, handle_bar
TODO-4: [QG-1.2] Fix clippy warning: unused variable in handle_baz
...
```

⚠️ warnings are informational and do not require fixes, but include them as low-priority items if they represent real improvements.

### Step 15: Work through the TODO list

For each TODO item, in order:

1. **Read** the relevant source file(s) to understand the current state.
2. **Fix** the issue by editing the source file(s). Follow the conventions in `QUALITY_GATE.md`:
   - For state error shaping: follow the `/refactor-state-errors` pattern (thiserror enum in state.rs, exhaustive match in handlers.rs, `err.to_string()` for messages).
   - For stub annotations: add `// STUB[category]: reason` on the line before the `fn` declaration.
   - For clippy/fmt: apply the minimal fix.
   - For missing integration tests: add targeted tests that cover the gap.
   - For generated file issues: **do not hand-edit** `model.rs` or `wire.rs`. Fix the generator or re-run codegen.
3. **Verify** the fix compiles:
   ```bash
   cargo check -p winterbaume-{service} 2>&1
   ```
4. **Mark the TODO as done** and move to the next item.

### Step 16: Re-run the gate

After all TODO items are resolved, re-run the full gate check (Steps 3–12) to confirm the crate now passes. If new failures emerge from the fixes, add them to the TODO list and repeat.

### Step 17: Final verification

Run the full test suite one final time:

```bash
cargo clippy -p winterbaume-{service} -- -D warnings 2>&1
cargo test -p winterbaume-{service} 2>&1
```

Both must pass cleanly.

---

## Phase B Output: Remediation Report

After all fixes, produce the final report:

```
# Quality Gate Remediation: winterbaume-{service}

## Changes Made

| TODO | Gate Check | Action Taken |
|------|-----------|--------------|
| TODO-1 | QG-3.1 | Replaced SesError struct with thiserror enum (5 variants) |
| TODO-2 | QG-3.2 | Updated ses_error_response with exhaustive match |
| ... | ... | ... |

## Final Gate Status

| Result | Count |
|--------|-------|
| ✅ Pass | N |
| ⚠️ Warning | N |
| ❌ Fail | 0 |
| ⏭️ Skipped | N |

## Verdict: PASS
```

If any ❌ failures remain that genuinely cannot be fixed (e.g., they require workspace-wide generator changes or upstream fixes), list them explicitly under **Deferred Items** with a reason and add them to `.agents/docs/TODO.md`.

---

## Phase C: Record to JOURNAL.md

After Phase A (and Phase B if remediation was needed), **append** a dated entry to `.agents/docs/JOURNAL.md`. Do not edit existing sections — only append.

### Step 18: Write the journal entry

Append a section to the end of `.agents/docs/JOURNAL.md` in this format:

```markdown
## {YYYY-MM-DD} — Quality Gate: winterbaume-{service}

### Result: {PASS / FAIL (with N deferred)}

| Metric | Value |
|--------|-------|
| ✅ Pass | N |
| ⚠️ Warning | N |
| ❌ Fail | N |
| ⏭️ Skipped | N |
| Integration tests | N passed |

### Findings

(Bullet list of all ⚠️ and ❌ items with their QG codes and short descriptions. If the crate passed cleanly, write "No issues found.")

### Remediation

(If Phase B ran: summary table of changes made. If no remediation was needed: "No remediation required — crate passed on first check.")

### Deferred Items

(If any ❌ items were deferred to TODO.md: list them here with reasons. Otherwise omit this subsection.)
```

Use today's date. Keep the entry concise — the journal is a log, not a full report. The detailed verdict lines were already shown to the user in Phase A output.

---

## Recurring failure modes

These are real, repeated failure patterns observed in past quality-gate sweeps. Check them explicitly during Phase A and add TODO items for any that apply.

- **Missing `chrono` dependency.** If any source file in the crate calls `chrono::Utc::now()` or any other `chrono::` API, verify the crate's `Cargo.toml` declares `chrono = { workspace = true }` before declaring the change done. Recurring failure mode caught manually in the 2026-04-28 sweep ( elasticbeanstalk, panorama, workspaces ) — agents added the call without the dep, cascading to compile-time errors only after they returned.
- **State-view literal drift.** Every `*StateView` struct ( and its component view structs ) must `#[derive(Default)]`, and any literal construction in tests, converters, or backend wrappers should use `..Default::default()`. State-view literal drift is the largest cascade source in `/tackle-todos` sweeps; per-crate quality gates do not catch the breakage, only `winterbaume-terraform` integration tests do.
- **State-shape change cascade.** When a state-field type changes ( e.g. `Vec<String>` → `HashMap<K, HashSet<V>>` for `inspector2`'s `enabled_resource_types` ), terraform converters that mirror that field also need updates. Per-crate quality gates do not catch the breakage; only `winterbaume-terraform` integration tests do. After changing a state-field type, grep the workspace for cross-crate consumers.
- **`--maxfail` is pytest, not cargo.** `--maxfail` is a pytest flag, not a Rust libtest flag. For Rust crate verification, use focused filters ( `cargo test <pattern>` ), `--test-threads=N`, `--no-fail-fast`, and per-crate gates instead. Do not pass `--maxfail` to cargo test.
- **Write the report file before the final cargo invocation finishes.** Always write your report file to disk before the final cargo invocation finishes, even if checks are still running. One agent ( `transcribe`, 2026-04-27 sweep ) hung on cargo and never wrote its report file, requiring a re-dispatch. The report's content can be amended in a final pass once the cargo command resolves; the file's mere existence is what tells the parent the agent didn't crash.
