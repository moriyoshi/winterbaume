# Quality Gate Workflow and Recurring Findings

## Summary

The April 2026 quality-gate sweeps turned the repo's publication checklist into a repeatable maintenance workflow rather than a one-off audit. The durable knowledge is which failure classes recur across crates, which ones should be fixed in `smithy-codegen` instead of locally, how deferred gaps should flow into `TODO.md`, and which warnings are convention drift rather than real defects.

## Key Facts

- The quality-gate workflow is anchored in `.agents/docs/QUALITY_GATE.md`, but the journal sweep added practical triage rules about what to fix immediately, what to defer, and where to record follow-up work.
- The most common actionable failures were missing `STUB[...]` annotations, state-view fidelity gaps in `views.rs`, and hand-written response paths that bypass generated `wire::serialize_*` helpers.
- Generated-code hygiene problems belong in `tools/smithy-codegen`, not in hand-edited `model.rs` or `wire.rs`.
- Visibility differences such as `pub(crate) mod model;` versus `pub mod model;` were frequently noted, but were usually non-blocking convention drift rather than release blockers.
- Quality-gate findings should only become `TODO.md` entries when they represent real deferred work, not when they are already fixed during the gate pass or are harmless style deviations.
- The sweep established two important repo-wide deferred items: the EC2 `wire::serialize_*` migration and the EC2 state-view expansion.
- Workspace-wide sweeps should distinguish service crates from utility or infrastructure crates. `tests/integration_test.rs` is a service-crate requirement, not a blanket rule for every crate in the workspace.
- The static service-crate gate can produce useful false positives while the classifier evolves. Keep the audit script under `.agents-workspace/tmp`, tighten it against known benign patterns, and only promote real defects to durable docs or `TODO.md`.
- View-fixture drift is a recurring gate failure. Large `*StateView` structs need `Default` derives, and tests/converters should prefer builder helpers plus `..Default::default()` over raw all-field literals.
- Generated input and SDK output shapes are not uniformly required or optional across a service. Gate fixes should inspect each exact generated field or SDK accessor before applying `.is_empty()`, `.as_deref()`, `Some(...)`, or bare-value patterns.

## Details

### What the Sweep Actually Validated

The practical gate flow was:

1. build, clippy, and fmt on the target crate
2. inspect crate structure against the canonical module layout
3. check `state.rs` for domain-error discipline
4. inspect `views.rs` for snapshot, restore, and merge fidelity
5. audit intentional stubs for `STUB[...]` annotations
6. compare handler response construction against available generated serialisers
7. confirm integration-test or `FIX(terraform-e2e)` coverage where required

The journal sweep showed that the checklist works best when each finding is sorted into one of three buckets:

- fix now in the crate
- fix in the generator, then regenerate
- defer into `TODO.md` because the work is too large for the current pass

### Recurring Immediate Fixes

Three classes of issues recurred across many crates and were usually cheap to fix immediately:

- missing `// STUB[category]: reason` comments on intentional stub handlers
- `views.rs` fields that silently dropped durable collections on snapshot or restore
- small build or fmt failures caused by stale generated output or minor handler mistakes

Representative cases from the sweep:

- `winterbaume-cloudformation`, `winterbaume-redshift`, `winterbaume-ecs`, `winterbaume-s3control`, and many others only needed explicit `STUB[...]` annotations
- `winterbaume-autoscaling`, `winterbaume-apigatewayv2`, and `winterbaume-sesv2` needed view expansions so restore paths stopped zeroing real state
- `winterbaume-rds`, `winterbaume-sagemaker`, `winterbaume-codebuild`, and `winterbaume-bedrockagent` needed narrow remediation and then passed

### Generator-First Rule

The sweep reinforced one repo-wide rule: when the failure is emitted by generated files in more than one crate, the fix belongs in `smithy-codegen`.

Concrete generator-owned issues from this wave included:

- rpc-v2-cbor protocol support for CloudWatch
- rustfmt-clean output for generated awsQuery files
- `#![allow(dead_code)]` and other generated lint allowances
- request-deserialiser support and query-protocol helper generation

The durable maintenance rule is:

- do not hand-edit generated `model.rs` or `wire.rs`
- patch `tools/smithy-codegen`
- regenerate the affected crate
- then rerun the gate

### Hand-Written Response Debt

The sweep also made the response-serialisation debt more explicit.

There are two different situations:

- acceptable protocol-boundary hand work, especially for error shaping
- avoidable success-response hand construction where generated `wire::serialize_*` helpers already exist

The main deferred examples were:

- `winterbaume-ec2` still has roughly 100 non-stub handlers building XML manually despite available `wire::serialize_*` helpers
- `winterbaume-sesv2` still has 80+ hand-written JSON success bodies
- `winterbaume-cloudwatch` moved onto generated rpc-v2-cbor helpers after scalar and list timestamp support landed; later Terraform E2E work exposed a separate secondary-protocol adapter issue where CBOR Tag 1 had to become RFC 3339 in awsQuery XML.

The durable lesson is to distinguish protocol-specific exceptions from true generator-adoption gaps. Error helpers are often legitimate. Large success-response surfaces usually are not.

### State-View Fidelity as a Gate Class

By the later passes, `views.rs` fidelity had become one of the highest-signal quality-gate categories.

Important examples:

- `winterbaume-ec2` persists only a small subset of its durable collections and now carries two explicit deferred TODO items
- `winterbaume-bedrockagent` still omits many durable resource families from `BedrockAgentStateView`
- `winterbaume-rds` and `winterbaume-iam` surfaced view gaps that may not break current tests but still matter for snapshot and restore correctness

The gate established a durable rule: a crate is not really stateful in the repo sense unless `views.rs` preserves the durable resources implemented in `state.rs`.

### 2026-04-22 Full Gate Sweep (164 Services)

A workspace-wide quality-gate pass covering all 164 service crates completed on 2026-04-22.

Result: **PASS** (after one remediation).

| Category | Count |
|----------|-------|
| ✅ Pass | 14 |
| ⚠️ Warning | 4 |
| ❌ Fail (fixed) | 1 |
| ⏭️ Skipped | 5 |

Warning classes:

- QG-4.1: `amplify` and `apprunner` implement `StatefulService` in `lib.rs` instead of `views.rs` — functional but non-canonical.
- QG-4.6: 43 crates lack `notify_state_changed` — predates the convention.
- QG-7.1: `cloudwatch`, `sagemakerruntime`, `scheduler`, `sesv2` use `json!` without `wire::serialize_` (error responses or protocol edge cases; the latter three were resolved in the QG §7 batch above).
- QG-7.2: `neptune`, `route53`, `s3control` still use hand-written `xml_response()` (also resolved in the QG §7 batch).

The sole ❌ failure was `winterbaume-terraform` failing QG-1.1 ( compilation error in `converters/redshift.rs` ) after the upstream `ClusterStatus` enum refactor; fixed by making the inject side parse string → `ClusterStatus` via serde and the extract side use `.to_string()`.

Coverage snapshot at this pass: **6,413 / 10,456 operations across 164 services (61.3%)**.

### 2026-04-23 Workspace-Wide Gate Refresh (179 Crates)

A follow-up workspace-wide pass on 2026-04-23 re-ran the gate against the full crate set: **166 service crates plus 13 utility or infrastructure crates**.

Result: **PASS** (after one remediation).

Key durable outcomes:

- the only actual gate failure was a compile error in `crates/winterbaume-terraform/src/converters/redshift.rs`, where `serde_json::json!({...})` had been given bare block expressions for `cluster_nodes` and `cluster_type`; the durable fix was to compute those values in local `let` bindings before the macro call
- `cargo check --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo fmt --all -- --check` all passed after that fix
- the sweep clarified that utility crates such as `winterbaume-core`, `winterbaume-e2e-tests`, `winterbaume-sfn-asl-eval`, and the standalone evaluator crates are not violations just because they lack `tests/integration_test.rs`
- QG §7 broad response-serialisation debt was intentionally left as per-crate follow-up rather than turned into one oversized workspace TODO
- QG §8 still has one concrete open audit item: every `FIX(terraform-e2e)` marker should eventually be paired with a matching fast integration test

### 2026-04-24 Static Service-Crate Gate Remediation

A 2026-04-24 workspace service-crate audit initially failed hard: 164 service crates audited, 1 passing, 163 failing, 493 failed checks, and 86 warnings. The first raw result was intentionally treated as a triage report rather than proof that every crate was broken.

The durable remediation pattern was:

- fix real workspace blockers first, especially fmt, check, and clippy failures
- bring dependent backends back into sync when a service trait changes
- correct obvious crate-structure issues, such as missing canonical modules or private generated `model` / `wire` modules
- move `StatefulService` implementations into `views.rs` where the gate convention expects them
- tighten the temporary static audit classifier for multiline generated allowances, empty derived views, populated `..Default::default()` response structs, request-body defaults, conversion helpers, function-level `STUB[...]` annotations, and payload-sensitive exceptions

The most important real fixes were:

- `winterbaume-dynamodb-redis` was updated for the expanded DynamoDB backend trait and persisted GSI/LSI definitions with backward-compatible serde defaults
- the Terraform DynamoDB converter now builds typed `SecondaryIndexView` values instead of opaque JSON blobs
- handler error shapers in several crates were normalised so messages flow through `err.to_string()`
- `winterbaume-rdsdata`, `winterbaume-securityhub`, CloudWatch, EC2, and EMR success responses moved further onto generated `wire::serialize_*` helpers
- Glacier `GetJobOutput` and IoT Data Plane `DeleteThingShadow` remained explicit QG-7 exceptions because generated serializers would wrap raw payload fields and change SDK-visible response bodies

Final static result: **164 service crates pass, 0 fail**. The remaining warnings are QG-11.6 manual-review notices for `serde_json::Value` view fields, not automatic failures.

The QG-8.4 `FIX(terraform-e2e)` marker audit also completed. Existing marker-bearing crates now have matching integration coverage comments or tests; the visible Redshift gap gained `test_describe_cluster_includes_provider_required_defaults`, covering the provider-required `ClusterNodes`, `ClusterParameterGroups`, and `MultiAZ` defaults.

### 2026-04-27 Full Gate Sweep ( 223 Service Crates )

A workspace-wide pass covering all 223 service crates ran in 23 batches of ~10 sonnet sub-agents. Each agent ran Phase A ( assessment ) and Phase B ( remediation ) for one crate. **Phase C ( per-crate JOURNAL writes ) was disabled** to avoid append-conflicts; per-crate reports were written to `.agents-workspace/tmp/qg-results/<crate>.md` ( 223 files ) and the workspace-level summary was consolidated into a single JOURNAL entry. This is the durable scaling pattern for future workspace-wide sweeps.

Result: **PASS ( 223 / 223 )**. Roughly 102 crates passed first-pass without remediation; ~121 needed Phase B remediation; a handful of items were genuinely deferred ( mostly missing `tests/scenario_test.rs` ).

Remediation themes from this sweep ( additive to the ones recorded earlier ):

| Theme | Crates touched | Pattern |
|-------|----------------|---------|
| Missing `tests/scenario_test.rs` | ~70+ | 2-4 chained-operation scenario tests added per crate. Some crates with no plausible multi-step workflow ( pure inference APIs, AWS-managed corpora ) opted out and deferred the gap to TODO. |
| `*StateView` missing `#[derive(Default)]` | ~30+ | Terraform converters were constructing views by naming every field; adding `Default` lets future field additions stay backwards-compatible. Several converters in `winterbaume-terraform/src/converters/<service>.rs` were updated at the same time to use `..Default::default()`. |
| Unannotated stubs | ~40+ | Added `// STUB[no-engine]` / `STUB[no-telemetry]` / `STUB[no-state]` / `STUB[org-integration]` / `STUB[s3-integration]` annotations on the line before the `fn` to satisfy QG §5. Many were telemetry / forecast handlers ( CostExplorer forecast, Pricing/Pi/CodeGuru reports, ListAutoshifts, ListBuiltInIntents ). |
| Missing `src/types.rs` + `pub mod types;` | ~10 | Created placeholder `types.rs` and added the module declaration to `lib.rs` to satisfy the canonical crate layout. |
| Missing `#[serde(default)]` on `Option`/collection fields in state views | a handful ( iot, securityhub, servicediscovery ) | Forward-compat for older snapshot deserialisation. |
| Hand-written JSON / XML responses | braket, chimesdkmeetings, cognitosync, codegurureviewer, codegurusecurity, codestarnotifications, elasticloadbalancingv2 ( 27 handlers in one go ) | Refactored handlers to call generated wire serialisers; introduced `*_to_wire` helper functions where domain types didn't already match wire shapes. |
| Clippy errors after toolchain bumps | ~6 ( cloudfrontkeyvaluestore, codegurureviewer, codegurusecurity, connectparticipant, pcaconnectorscep, snowdevicemanagement, taxsettings ) | Mostly `redundant_closure`, `field_reassign_with_default`, `unnecessary_get_then_check`, `iter_kv_map`, `needless_borrows_for_generic_args`, `unnecessary_filter_map`. |
| State-view fidelity gaps | account, ecr, eventbridge, guardduty, networkmanager, iotdataplane, kinesis | Agents extended state views and `From`/`merge` impls. Some accidentally fixed adjacent bugs ( iotdataplane URL-decode topic on publish path; guardduty `ListFindings` defaulting `include_archived = true` ) -- flagged in per-crate reports. |
| Hard-coded ARN scope in state methods | applicationautoscaling, secretsmanager | Threaded the request scope into the state method signatures. The same antipattern is suspected in many other crates that use `gen_*_arn()` helpers ( cf. ssm `gen_ops_metadata_arn` finding ). |

Recurring observations that did NOT fail the gate but should be tracked:

- **`DEFAULT_ACCOUNT_ID` hardcoded in `dispatch()` in ~89 crates.** This is a workspace-wide pattern, not a per-crate defect, but every PASS verdict tacitly accepts that multi-account is not wired. Worth a coordinated cleanup, possibly via a shared `dispatch_with_scope()` helper in `winterbaume-core`.
- **Timestamps stored as `f64` epoch instead of RFC 3339.** Flagged on amplifyuibuilder ( wire-protocol-driven ), elasticbeanstalk, fsx ( `BackupView.creation_time` ), lexmodelsv2, mediapackagev2, panorama, workspaces. QG §11 prefers RFC 3339; non-blocking but inconsistent.
- **Fake "next-page" pagination tokens.** route53domains ( `"next-page"` ), s3vectors ( `"next"` ), scheduler ( always `None` ), cloudfrontkeyvaluestore, dax. Either implement real offset/cursor tokens or downgrade to single-page responses with an explanatory comment.
- **`test_full_lifecycle` in `integration_test.rs` substituting for `scenario_test.rs`.** Many crates ( cognitoidentity, cloudcontrol, datapipeline, simpledbv2 ) have a chained-4-6-op `test_full_lifecycle` that agents accepted as adequate scenario coverage. Convention worth codifying in QUALITY_GATE.md so future runs do not oscillate.

Operational notes for future workspace-wide sweeps:

- **Always set per-agent `CARGO_TARGET_DIR`.** Batch 1 ran without it; all 10 sub-agents shared one target directory and serialised on `target/.cargo-lock`. Took ~30 min vs ~5-8 min for subsequent batches that used `.agents-workspace/tmp/target-qg-<crate>/`. Cleanup of the `target-qg-*` directories is pending; `find .agents-workspace/tmp -maxdepth 1 -name 'target-qg-*' -mtime +1 -exec rm -rf {} +` reclaims the disk.
- **JOURNAL.md serialisation matters.** Skipping Phase C in agents and consolidating into one JOURNAL entry ( 223 -> 1 ) avoided the append-conflict failure mode entirely. Per-crate output files in `.agents-workspace/tmp/qg-results/` are the canonical record; agent-reply summaries are not.
- **One agent ( transcribe ) hung on cargo and never wrote its report file.** Re-dispatched explicitly with a "write the report file even if checks are still running" instruction. Bundle that into the default `quality-gate` skill prompt.
- **Trust-but-verify after parallel edits.** Two intermediate-state false alarms surfaced via the monitor stream ( `winterbaume-ebs blob_backed::roundtrip_snapshot_restore_with_blobs`; `winterbaume-transfer TransferStateView: Default not satisfied` ). Both were verified clean post-fix. A workspace-wide `cargo check --workspace` and `cargo test --workspace` after the sweep is advisable before treating the result as durable.

### `/tackle-todos` Multi-Round Sweep Pattern ( 2026-04-28 )

The /tackle-todos skill complements the quality-gate sweep by going through `TODO.md` in batches of 4-ish parallel agents per round. The 2026-04-28 four-round day cleared 39 of 269 unchecked items. Operationally:

- Each round dispatched 4 parallel agents, each owning 1-2 thematically related TODO units. No worktree isolation; agents share the main checkout. Acceptable for content edits but exposes file-mtime races: the `Edit` tool rejects batched edits with "modified since read" when independent agents touch the same file concurrently. Workaround: serialise Edit calls ( one Read → one Edit at a time ) when the file is hot.
- After each round, run a verification pass that compiles the touched crates plus `winterbaume-server` ( catches cross-crate breakage from view-shape changes ). `cargo check` is faster but reports phantom "can't find crate for X" errors on shared sccache state — fall back to `cargo build` when that happens.
- Cascade-fix surface area scales with state-view expansion. Adding a field to `*StateView` breaks every literal that lacks `..Default::default()`; in this codebase that often means `winterbaume-terraform/src/converters/<service>.rs` and the per-crate Redis backend wrapper. The 2026-04-28 round 3 verification pass surfaced 27 such cascading errors and had to add `..Default::default()` to view literals plus `#[derive(Default)]` to `EventBusView`, `BackupStateView`, `DetectorView`.
- Inline cascade fixes during verification beat re-dispatching agents. Examples from the 2026-04-28 sweep: U4 transfer / vpclattice SDK getter signature mismatches ( `Some(key_id.as_str())` → `key_id`, `Some(status_code)` → `status_code` ); W4 panorama / workspaces missing `chrono = { workspace = true }` Cargo.toml dep; medialive `let (k, v) = p.split_once('=')` ( returns `Option`, not a tuple — needs `?` inside `filter_map` ).
- Workspace-wide trust-but-verify is the open carry-forward. The day-summary verification only ran `cargo build -p winterbaume-server`, which is necessary but not sufficient. A full `cargo build --workspace` plus the E2E and integration suite is still needed before any release.

Recurring failure modes catalogued during the day, worth bundling into skill prompts ( see TODO.md `skill-*` entries ):

1. **Missing `chrono` Cargo.toml dep.** Agents add `chrono::Utc::now()` calls without verifying the crate's `Cargo.toml`. Recurring across U2 ( elasticbeanstalk ) and W4 ( panorama, workspaces ).
2. **SDK getter signature drift.** Scenario-test agents assume `Option<T>` for getters that return `T` ( e.g. `SshPublicKey::ssh_public_key_id() -> &str`, `FixedResponse::status_code() -> i32` ). Caught at `cargo build --tests` time after the agent returns.
3. **State-view literal drift.** Adding fields to a `*StateView` struct breaks every literal construction. Most state-view types lacked `Default` derives until the 2026-04-28 sweep made them mandatory by convention. Always derive `Default` on every `*StateView` struct ( and component view struct ); always use `..Default::default()` in struct literals when constructing them in tests, converters, or backend wrappers.
4. **State-shape changes break terraform converters.** When a state field's type changes ( e.g. inspector2's `Vec<String>` → `HashMap<account, HashSet<region>>` ), terraform converters that mirror that type must also be updated. Per-crate quality gates do not catch this; only `winterbaume-terraform` integration tests do.
5. **TaskCompleted hook misconfiguration ( this project ).** The hook's `condition` field is the literal imperative "Write down the findings and work summary to JOURNAL.md", which the LLM evaluator cannot reduce to a boolean. Net effect: every TaskUpdate to `completed` is silently rejected, leaving most round tasks stuck at `in_progress` even though the underlying work is done. Settings-level fix needed.
6. **Stale cargo cache from concurrent agents.** `cargo check` reports phantom errors like `can't find crate for arc_swap` in external dependencies. Forcing a `cargo build` of the same crate resolves it. The per-session `CARGO_TARGET_DIR` does not fully isolate when agents touch shared sccache state. Workaround: prefer `cargo build` over `cargo check` for verification, and treat single phantom-import errors as cache pollution unless they reproduce twice.
7. **Generator-fix blast-radius asymmetry.** A generator change at `tools/smithy-codegen/src/gen_serializers.rs` produces correct emitted code, but only the regenerated crate gets the fix. All other crates that use the same generator pattern still ship the broken output until they are separately regenerated. Maintain a "regen sweep" task in TODO.md whenever a generator fix lands. The 2026-04-28 restJson1 `@httpPayload` framing fix surfaced 8 silent SDK-visible bugs across `bedrock`, `glacier`, `iotdataplane`, `kinesisvideoarchivedmedia`, `lambda`, `mediastoredata`, `pinpoint`, `sagemakerruntime` after the loop-iteration regen sweep.
8. **Regen reorders `model.rs` harmlessly.** The generator's struct emission order is non-deterministic ( HashMap iteration ). Regenerating any service's `wire.rs` typically also produces a `model.rs` reorder with no functional change. Do not treat the diff as a red flag.

The S3 Files full-coverage pass reinforced the same view-fixture rule on a new-service crate: adding `policy`, `synchronization_configuration`, `mount_targets`, and `access_points` broke every raw `FileSystemView` / `S3FilesStateView` literal in tests. A small `fs_view(id, bucket)` builder helper plus `..Default::default()` on top-level views contained the churn. This should be treated as an `add-service` template issue, not only a one-off crate fix.

The wire-deserialiser sweep added a second recurring shape class:

- generated request fields that represent the same Smithy concept may be `String` in one input shape and `Option<String>` in another; WAFv2 broke when `.is_empty()` was copied from a required shape onto an optional one
- SDK response accessors can similarly vary between `&str` and `Option<&str>` in one service; S3 Files exposed this across file-system, policy, and mount-target responses

The gate response is pragmatic: compile tests early, let compiler errors reveal exact SDK accessor shapes, and prefer local match patterns such as `match input.field.as_deref() { Some(s) if !s.is_empty() => s, _ => ... }` when generated input fields are optional.

### `/tackle-todos` Round 5 and Round 6 Lessons

The 2026-04-29 / 2026-04-30 follow-up sweeps added two durable operating rules:

- Stale TODOs should be verified before dispatching agents. In round 5, `partiql-cleanup-orphans` and the URL-decode audit targets were already resolved by the time the work started, so the useful action was cross-checking rather than editing.
- Rust tests do not understand pytest flags. One agent tried `cargo test ... -- --maxfail=5`; `--maxfail` belongs to pytest, not libtest. Rust verification should use focused test filters, `--test-threads`, and `--no-fail-fast` where appropriate.

Round 6 deliberately stayed in the main checkout after worktree merge-back drift in round 5. That was the right shape for small single-crate edits: ACM and Support validation tightened to AWS behaviour, unreachable typed-SDK required-field tests were rewritten as explanatory comments, and the PartiQL lexer gained SQL comment support. The verification set was per-crate fmt / clippy plus focused tests for ACM, Support, PartiQL, Polly, and Kinesis Video Archived Media; a final grep showed zero `// TODO` / `// FIXME` comments under `crates/**/*.rs`.

### 2026-05-01 / 2026-05-02 Workflow Hardening

The next `/tackle-todos` batches closed stale or duplicated backlog items and converted recurring mistakes into skill-prompt guardrails:

- verify stale TODO items before dispatching implementation agents.
- use Rust libtest flags (`--no-fail-fast`, focused filters, `--test-threads`) rather than pytest-only flags such as `--maxfail`.
- derive `Default` on every `*StateView` and component view struct; use `..Default::default()` in literals.
- verify `chrono = { workspace = true }` before adding `chrono::` calls.
- assume AWS SDK getters may return `T`, `&T`, or `Option<T>`; let `cargo build --tests` settle the signature.
- remember state-shape changes can break Terraform converters even when the service crate passes.
- write quality-gate report files before long final cargo invocations so the parent can distinguish slow builds from crashed agents.
- register every new service crate in `CRATE_TO_MODEL` and run the three-counts canary: coverage service crates, per-crate READMEs, and docs service pages.

The EC2 invariant work changed the gate shape for stateful services. A multi-resource lifecycle test inside `integration_test.rs` is no longer a substitute for `tests/scenario_test.rs`. Large stateful services need a cross-call invariant inventory with row-by-row `N/A` justifications, and scenario tests must cover each non-`N/A` row.

The helper `.agents/bin/audit-state-fields.sh` is only a heuristic worklist. It flags likely toggle-without-consumer and counter-reuse patterns, then humans or `/write-tests` agents decide which rows need scenario coverage.

The first fleet dry-run showed the script is currently a high-precision EC2 backstop rather than a fleet-wide checker. It flagged only EC2 because the counter heuristic looks for EC2-style `self.counters.*` references; most other crates mint IDs through different shapes. EC2 produced 12 flags, 11 of which were helper/caller or same-id-family false positives, and one real bug: duplicate ENI attachment ids. A future script revision could prune the dominant false-positive class by ignoring pairs where one co-reference is a `next_*_id` helper and the other body calls that helper.

### How Deferred Work Should Be Recorded

The sweep was useful precisely because it did not treat every warning as a TODO.

Good `TODO.md` candidates:

- large, concrete, still-open work with a clear owner surface
- generator-adoption gaps such as EC2 or SESv2 response migration
- state-view fidelity gaps with clear missing collections
- audit tasks with a bounded scope, such as verifying that every `FIX(terraform-e2e)` marker has a matching fast integration test

Bad `TODO.md` candidates:

- convention-only notes such as `pub(crate)` versus `pub`
- warnings already fixed during the same quality-gate pass
- observations that are already accepted architectural exceptions
- a giant "many crates still have hand-written responses" umbrella item when the practical follow-up is crate-local and should be handled in smaller passes

This is why the post-sweep TODO set stayed relatively small even though the journal contains many warning notes.

## Files

- [.agents/docs/QUALITY_GATE.md](/Users/moriyoshi/Source/winterbaume/.agents/docs/QUALITY_GATE.md) - canonical checklist and implementation conventions
- [.agents/docs/TODO.md](/Users/moriyoshi/Source/winterbaume/.agents/docs/TODO.md) - deferred quality-gate follow-up work
- [tools/smithy-codegen/src/gen_serializers.rs](/Users/moriyoshi/Source/winterbaume/tools/smithy-codegen/src/gen_serializers.rs) - generator fixes triggered by gate failures
- [crates/winterbaume-ec2/src/handlers.rs](/Users/moriyoshi/Source/winterbaume/crates/winterbaume-ec2/src/handlers.rs) - reference deferred `wire::serialize_*` migration
- [crates/winterbaume-ec2/src/views.rs](/Users/moriyoshi/Source/winterbaume/crates/winterbaume-ec2/src/views.rs) - reference deferred state-view expansion

## Test Coverage

- The sweep repeatedly used crate-local build, clippy, fmt, and integration-test checks rather than a single workspace-wide gate run.
- Quality-gate fixes often ended with focused verification such as `cargo check -p <crate>`, `cargo clippy -p <crate>`, and crate-local integration tests.
- `FIX(terraform-e2e)` findings were treated as incomplete until the crate also had a fast integration test for the same behaviour.

## Pitfalls

- Do not promote every warning into `TODO.md`. Keep the follow-up list small and actionable.
- Do not patch generated files directly just to make one crate pass.
- Do not treat view-fidelity warnings as cosmetic. They are persistence bugs.
- Do not accept hand-written success responses as "good enough" when generated serialisers already exist.
- Do not let quality-gate notes live only in `JOURNAL.md` once they represent ongoing deferred work. Move them into `TODO.md`.
- Do not declare a stateful crate done solely from per-operation integration tests. Cross-call invariants need scenario coverage or explicit `N/A` inventory rows.
- Do not treat the invariant audit script as a compile gate. It is a triage aid whose findings must be reviewed.
- Do not assume the invariant audit script has fleet-wide recall. Its initial version is EC2-shaped and misses crates that do not use a `self.counters` substruct.
- Do not copy SDK accessor or generated input-field optionality patterns across sibling operations without checking the exact generated type.
- Do not let large state-view literals spread through tests. Builder helpers plus `..Default::default()` are part of the quality-gate maintenance pattern.
