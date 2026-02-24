# Service Implementation and Validation Synthesis

## Summary

Winterbaume's durable implementation workflow is now one service-hardening lifecycle rather than a loose set of separate practices. The stable sequence is: expand service coverage, generate or repair Smithy-backed wire support, validate behaviour through moto or AWS-doc-derived tests, validate provider-facing behaviour through Terraform E2E, promote tractable stubs into real state-backed behaviour, and finish with a crate-local quality-gate pass that separates immediate fixes from durable deferred work. CI-parity verification and generated-artefact maintenance belong in that same loop rather than in a later clean-up phase.

## Included Documents

| Document | Focus |
|----------|-------|
| [core-service-expansion-and-coverage.md](./core-service-expansion-and-coverage.md) | Service growth, protocol expansion, new-service rollout, and operation-coverage strategy |
| [smithy-codegen-and-wire-serialization.md](./smithy-codegen-and-wire-serialization.md) | Generated serialisers, model and wire modules, and codegen failure modes |
| [moto-parity-testing-and-behavioral-gaps.md](./moto-parity-testing-and-behavioral-gaps.md) | Behavioural validation against moto and the bug classes it exposes |
| [aws-doc-test-plan-catalog.md](./aws-doc-test-plan-catalog.md) | Docs-driven fallback planning, scenario catalogs, and one-off post-batch plan patterns |
| [terraform-e2e-harness-and-fix-coverage.md](./terraform-e2e-harness-and-fix-coverage.md) | Terraform-provider compatibility, E2E harness behaviour, and `FIX(terraform-e2e)` regression coverage |
| [terraform-resource-converters.md](./terraform-resource-converters.md) | Terraform inject and extract coverage, converter contract, and `StateView`-gated resource support |
| [state-error-shaping-and-handler-boundaries.md](./state-error-shaping-and-handler-boundaries.md) | Domain-error enums in `state.rs` and AWS-facing error shaping in `handlers.rs` |
| [stub-handler-audit-and-promotion.md](./stub-handler-audit-and-promotion.md) | Stub taxonomy, state-backed promotion patterns, and what should still stay deferred |
| [quality-gate-workflow-and-recurring-findings.md](./quality-gate-workflow-and-recurring-findings.md) | Repeatable crate-hardening workflow, recurring failure classes, and TODO triage rules |
| [new-service-implementation-patterns.md](./new-service-implementation-patterns.md) | Root-resource-first service scope, workspace wiring, SDK test gotchas, identifier helpers, and notifier coverage |
| [ec2-operation-expansion-and-invariants.md](./ec2-operation-expansion-and-invariants.md) | Large shared-state operation expansion, EC2 scenario invariants, and counter-reuse audit findings |

## Stable Knowledge

- Service work in Winterbaume succeeds when done in layers: implement handlers and state, generate or refresh serialiser support, add SDK integration coverage, then add higher-fidelity parity or Terraform coverage as needed.
- New service crates should start with root-resource lifecycle coverage: create, get or describe, list, delete, update when cheap, and tags when SDK or Terraform read-back expects them. Small services should reach full operation coverage immediately; larger hierarchies should make deferred child-resource families explicit and return 501 rather than fabricating behaviour.
- Adding a service is not complete until every workspace touch point is wired: root workspace members and dependencies, root optional dependency and feature flag, `full` feature list, root dev-dependencies, `winterbaume-server` dependency and registration, example binary, integration tests, and TODO service status.
- Operation-count coverage is useful for prioritisation, but it is not a correctness signal on its own.
- `smithy-codegen` is core infrastructure, not optional tooling. If generated outputs are wrong, fix the generator rather than hand-editing generated files.
- Moto parity tests are the best behavioural spec where moto has meaningful coverage, but docs-driven plans are the default fallback once moto becomes weak, absent, or too narrow.
- Terraform E2E tests expose a separate compatibility layer: provider read-after-write expectations, waiter loops, missing default fields, sub-resource reads, exact filter semantics, provider schema field names, provider-version schema drift, and sometimes different protocol choices from SDK tests.
- Terraform provider reads often require stricter filtering than SDK smoke tests. EC2 is the reference: Describe handlers that return all rows can pass while only one resource exists, then fail provider refresh once two resources share a family. Parse explicit `<Id>.N` lists and standard `Filter.N` entries, and make generic Describe handlers iterate every subtype sharing the same id namespace.
- Some Terraform failures are upstream provider bugs rather than mock bugs. EC2 capacity block reservation in provider v6.43.0 uses AutoFlex without mapping SDK field names `CapacityReservationArn`, `CreateDate`, and `TotalInstanceCount` to framework fields `ARN`, `CreatedDate`, and `InstanceCount`; emitting non-Smithy XML names from Winterbaume is not the right fix.
- Terraform resource converters are part of the same lifecycle, not a separate integration toy. They rely on `StateView` fidelity and `merge()` semantics, so converter support should follow view support rather than bypass it.
- Nested Terraform-block work is a full vertical slice, not a converter-only clean-up. When a resource needs nested blocks such as `logging_configuration`, `encryption_configuration`, `vpc_config`, or `snapshot_copy`, the durable fix usually spans service types, handlers, `views.rs`, converter inject or extract logic, and Terraform E2E no-drift checks.
- Coverage reporting itself is part of the workflow, not just a report artefact. False negatives from handler-shape heuristics or Terraform resource-to-crate mappings are normal maintenance work at repo scale.
- XML, awsQuery, and ec2Query services need extra care around response wrappers, list serialization, and protocol-specific request or response structure.
- Coverage reporting can undercount implemented operations when handlers use generic or non-standard dispatch patterns; treat coverage mismatches as hypotheses to investigate, not as proof.
- Local verification should match CI scope. If generated code, tests, or examples changed, `cargo clippy --workspace --all-targets` is the reliable check; library-only clippy runs miss example and test warnings.
- Generator changes can create cascading breakage outside the crate that first exposed the bug. Stale `model.rs` or `wire.rs`, empty-shape syntax changes, and generated banner updates should be treated as workspace maintenance, not as one-crate fixes.
- `state.rs` should raise domain errors, not protocol-facing wire error structs; handlers own the final AWS error envelope.
- Stub work should be classified before it is fixed. `STUB[no-state]` usually means add durable state and handler wiring, while `STUB[no-engine]` means the missing work belongs in a dedicated evaluator, parser, or validator. As of 2026-04-19, every `STUB[no-state]` annotation has been eliminated from the codebase (~400 handlers across 45+ crates). The remaining stub categories (`no-engine`, `s3-integration`, `org-integration`, `no-telemetry`, `delegation-api`) require external subsystems and are intentionally deferred.
- Hard-coded mock analysis/detection stubs should be replaced with input-seeded deterministic generation: hash the input to produce a seed, then derive varying but reproducible results from it. This avoids both static fake data and non-deterministic randomness. The pattern was applied across Rekognition (6 handlers), Polly, and Signer.
- Durable regressions recur in a few classes: missing fields, wrong defaults, wrong error codes, bad idempotency, invalid serialization shape, and provider-specific read-back mismatches.
- The Terraform AWS provider may access response list fields at index 0 without nil checks. Redshift clusters are the reference case: `ClusterParameterGroups[0]`, `ClusterNodes[0]`, and `MultiAZ` must always be populated in `cluster_to_wire`, or the provider panics.
- The AWS provider's `ClusterAvailabilityStatus` (title-case) is the field polled by Redshift waiters, not the fine-grained `ClusterStatus` (lowercase). Both must be populated independently; using a typed enum with `as_str()` and `availability_status()` methods is the durable approach.
- Provider attribute removals and renames can silently break E2E tests whenever the CI runner fetches a new provider release. Pin the provider version in `write_provider_tf()` and make upgrades explicit. The harness now intentionally targets provider `~> 6.0`; concrete drift already seen includes Redshift inline `logging`/`snapshot_copy` removals ( provider v5.45+ ), SFN `log_destination` block-to-string change, Batch `name` vs `compute_environment_name`, Application Cost Profiler schema availability, S3 path-style requirements under v6, and EC2 waiter enum casing.
- QG §7 response debt is now best treated as a precise exception audit rather than a broad grep failure. CloudWatch moved its timestamp-sensitive CBOR success paths onto generated serializers after the rpc-v2-cbor scalar/list timestamp helper work. Legitimate payload-sensitive exceptions remain, such as Glacier `GetJobOutput` and IoT Data Plane `DeleteThingShadow`, where generated serializers would wrap raw payload bytes and change SDK-visible bodies.
- restJson1 `@httpPayload` output members must serialise as just the inner shape's value, not as the wrapping struct. The pre-2026-04-28 generator emitted `{"<member>": {...}}` and the SDK silently filled missing required fields with defaults via `*_correct_errors`, surfacing as "client receives empty struct". Fix is at `tools/smithy-codegen/src/gen_serializers.rs::generate_rest_json_serializer`. The restXml generator at `gen_serializers.rs:972-1023` already handles this correctly by emitting a function signature that takes `&PayloadType` directly, so no parallel restXml bug exists.
- JSON request deserialisation is now generator-owned for `RestJson1`, `AwsJson1_0`, and `AwsJson1_1`, alongside restXml, awsQuery, ec2Query, and rpc-v2-cbor. The 2026-05 adoption sweep moved 166 / 168 crates to `body.get = 0`; remaining manual body inspection needs a documented hybrid reason ( API Gateway PATCH-style flat scalar fields ) or a separate multi-protocol recipe ( CloudWatch awsQuery + rpc-v2-cbor ).
- awsQuery struct fields with nested-struct member types need a `ResolvedType::Structure(_)` arm in `generate_aws_query_member_deser`, otherwise the deserialiser body is empty and the field is silently dropped. ELB `ModifyLoadBalancerAttributes` was the reference case ( 4 nested config blocks dropped ); fix at `gen_serializers.rs:1700-1721`.
- Generator-fix blast-radius asymmetry is the standing maintenance shape: a generator change produces correct emitted code, but only the regenerated crate gets the fix; every other crate using the same pattern still ships the broken output until separately regenerated. Maintain a "regen sweep" task in TODO.md whenever a generator fix lands. The 2026-04-28 restJson1 `@httpPayload` framing fix surfaced 8 silent SDK-visible bugs across `bedrock`, `glacier`, `iotdataplane`, `kinesisvideoarchivedmedia`, `lambda`, `mediastoredata`, `pinpoint`, `sagemakerruntime` after the loop-iteration regen. Regenerating any service typically also reorders `model.rs` harmlessly because the generator's struct emission order is non-deterministic ( HashMap iteration ); do not treat that diff as a red flag.
- Multi-protocol services need provider-driven validation. CloudWatch is the reference case: Smithy primary protocol selection chose rpc-v2-cbor, but the Terraform AWS provider sent awsQuery requests, so the handler and generator needed secondary-protocol response support.
- Multi-protocol adapters must preserve protocol-specific sentinels while converting responses. CloudWatch's CBOR Tag 1 timestamp path was correct, but the awsQuery XML conversion needed RFC 3339 formatting for provider refresh.
- Provider crashes are often nil-dereferences on optional fields that the provider assumes are populated. ELBv2 listener-rule conditions must emit both legacy `Values` and matching `*Config.Values` blocks; Redshift clusters and Pinpoint app settings are similar examples where absent nested defaults crash or poison readback.
- The static service-crate gate reached a zero-failure result after classifier tightening and real remediation: 164 service crates passing, 0 failing, with remaining warnings limited to manual-review `serde_json::Value` notices.
- As of 2026-04-22, overall API coverage stands at 6,413 / 10,456 operations across 164 services (61.3%).
- When SDK builders move from flat request fields to nested request structs, paired handlers need to accept both shapes consistently. Updating only the first operation exposed by a test usually leaves the sibling read or delete path broken.
- Integration-test generation has one recurring failure mode that is now explicit repo guidance: create the prerequisite resource first and capture its real ARN or identifier before testing a read or describe API. Plausible hard-coded ARNs are not a substitute for state setup.
- AWS SDK Rust service errors should be asserted with debug formatting, not `Display`, because `Display` often collapses to `service error` and hides the exception type.
- SDK getters reflect Smithy `@required`: required members return `&T`, while optional members return `Option<&T>`. Do not infer SDK getter optionality from generated wire response structs.
- SDK getter optionality can vary between sibling outputs inside one service. S3 Files exposed adjacent `&str` and `Option<&str>` accessors across file-system, policy, and mount-target responses; compile tests before codifying assertion shapes in templates.
- Generated sibling shapes can use different field names for the same concept. Check the exact generated output struct before writing conversions.
- REST JSON handlers that parse request bodies as `serde_json::Value` should treat an empty body as `{}` so valid GET requests do not fail before routing.
- REST JSON handlers should reuse `winterbaume_core` URL helpers before writing local query parsing. `parse_query_string` currently collapses repeated keys, so list query params such as `?tagKeys=a&tagKeys=b` may still need a narrow local collector until core grows a multi-value helper; do not copy whole percent-decoder/helper blocks into new crates.
- Brand-new services can be useful as explicit partial scaffolds when the service surface is new and Terraform has no resources yet. S3 Files is the current reference for the full path: root FileSystem lifecycle plus tags landed first, then a second pass brought mount targets, access points, file-system policy, sync configuration, and mount-target update to 21 / 21 operation coverage. `/write-tests` remains the publication-readiness follow-up for scenario inventory even when per-operation coverage is complete.
- Stateful new services should include a notifier regression test that subscribes, calls `restore(account, region, Default::default())`, and verifies the emitted scope.
- Path identifiers that can be either bare IDs or ARNs should use one helper that strips the trailing slash component for ARN inputs and returns bare IDs unchanged.
- Terraform converter work is not complete when the file compiles. Registration in the injector build path is part of the definition of done.
- Terraform extraction now has a multi-scope path for opted-in regional services: `ConversionContext`, scoped `ExtractedResource` metadata, `BackendState::scopes_with_state()`, and `TerraformInjector::extract_all()` with post-collection collision handling.
- Parallel worktree batches can overstate what landed. Journal or agent summaries must be checked against the main working tree before promoting a claimed converter or nested-block completion into durable guidance.
- The quality-gate pass is not an afterthought. It is the final check that the crate adopted generated wire helpers where possible, preserved state-view fidelity, annotated intentional stubs clearly, and recorded only the real deferred work in `TODO.md`.
- Docs-driven planning in this repository follows a stable structure: source URLs, a short service summary, implemented operations, candidate scenarios, and explicit expected outcomes.
- The same workflow scales from batch work to single-service pushes. The EC2 expansion, S3 Tables completion, stub-promotion waves, converter growth, and quality-gate sweep all reused the same implementation -> codegen -> validation -> hardening sequence.
- Mature service crates ship two test files: `tests/integration_test.rs` for per-operation coverage and `tests/scenario_test.rs` for chained 3+-operation use cases. The `/write-tests` skill produces both; `/add-service` delegates to it after the per-operation tests pass. Stateless metadata-only services ( Translate, Pricing, Forecast Query ) explicitly justify skipping scenario tests instead of authoring synthetic ones. `#[ignore]` is acceptable for scenarios that depend on unimplemented features, with a *mandatory* cross-link to `TODO.md` so the test becomes a forcing function once the capability lands.
- Large stateful services need a cross-call invariant inventory before scenario design. The canonical rows are toggle propagation, modify rewrites sibling state, per-call uniqueness, default inheritance from parent, lifecycle state transitions, and cross-resource references on read. Each non-`N/A` row needs at least one scenario in `tests/scenario_test.rs`.
- EC2 is the reference large-service workflow: sequential family batches are appropriate when every batch touches shared `state.rs`, `views.rs`, and `handlers.rs`, and invariant audits should specifically look for reused counters across distinct ID families.
- EC2 generated code must always be regenerated with `--features-map tools/smithy-codegen/ec2-features.toml`; omitting it strips feature gates from generated `model.rs` and `wire.rs` and undoes the split-crate compile-time work.
- Some services have no public `Create*` API for their primary resource because the content is AWS-curated ( e.g. AWS Artifact's compliance reports, customer agreements ). Tests cannot seed via SDK calls. The convention is to construct the service's `*StateView` directly and call `service.restore(...)` or `service.merge(...)` before exercising the SDK. This doubles as a Terraform-converter contract test, since converters use the same path. Note this in the crate `README.md` so future contributors do not search for a missing "create" API.
- Click-through agreement flows ( AWS Artifact's `GetTermForReport` -> `GetReport` ) model the gate with a one-shot rotated token: store the most recently issued token per resource id in `state.term_tokens`, validate the supplied token before serving, and rotate on each issue. This is the smallest-surface pattern that produces realistic SDK-level errors when tokens are missing or stale.
- Deeply-nested discriminated-union request fields ( AppFlow's 17-variant `SourceConnectorProperties`; autoscalingplans's `ApplicationSource` and `ScalingInstructions` ) are most reliably stored as opaque `serde_json::Value` and round-tripped via `serde_json::from_value(v).ok()` when emitting the response. Trade-off: no inspection of union contents in handlers ( which the mock typically does not need ); win: bounded crate compile time and no per-service union modelling.
- `aws-sdk-*` enum variant naming is inconsistent across services: `ScalingMetricType::AsgAverageCpuUtilization` vs older Smithy models exporting `ASGAverageCPUUtilization`. The SDK builder applies acronym-aware capitalisation but the rule is not predictable from the model name alone. Always run `cargo test --no-run` first and read the compiler's "help: there is a variant with a similar name" suggestions; do not infer variant names from Smithy JSON.
- When a Smithy model marks a member `smithy.api#required`, the generated SDK getter elides the `Option`. `assert_eq!(plan.scaling_plan_name(), "test-plan")` is correct; `assert_eq!(plan.scaling_plan_name(), Some("test-plan"))` does not compile. This surfaces in newer aws-sdk-rust crates ( appfabric `bundle.arn()`, autoscalingplans `plan.scaling_plan_name()` ).
- A material fraction of new service crates have no `aws_<service>_*` resources in the upstream Terraform AWS provider and are skipped at Step 7 / Step 10 of the `add-service` skill. Recent skips: aiops ( brand-new ), amplifybackend ( deprecated Gen-1 ), appconfigdata ( runtime data API ), AWS Artifact ( `artifact`, read-mostly compliance-doc service ), autoscalingplans ( resource removed in current provider versions ). Confirm by checking the hashicorp/aws Terraform provider documentation index before authoring a converter.
- The Terraform converter error variant is `ConversionError::MissingAttribute`, not `MissingRequiredAttribute`. Caught at compile time on every newly authored converter.
- Round-trip bugs in Terraform converters fall into two named classes worth watching for: ( a ) **JSON-shape mismatch** -- the converter copies provider-schema-shaped JSON ( singleton block arrays, snake_case keys ) into a state view that the handler later decodes as AWS REST JSON ( camelCase ); the decode silently returns `None` and the resource loses fields when described. AppFlow is the reference case. ( b ) **Identifier id/name swap** -- extract emits `"id"` as the name and `"placement_group_id"` as the real `pg-...` id, but inject reads `"id"` first; round-tripped resources end up with the wrong identifier. EC2 placement-group is the reference case. Both classes need a converter test that injects realistic Terraform state then exercises the corresponding SDK describe/list filter.
- Provider schema names trump AWS API field names in Terraform state. Batch compute environments use `name`, not `compute_environment_name`; similar fixes should start by checking the provider schema and trace before editing the mock API shape.
- The `/tackle-todos` multi-round sweep complements the per-crate quality-gate workflow. Operationally: 4-ish parallel agents per round, each owning 1-2 thematically related TODO units, no worktree isolation ( agents share the main checkout ). After each round, verify by compiling the touched crates plus `winterbaume-server` ( catches cross-crate breakage from view-shape changes ); the day-summary `cargo check` was insufficient on its own. Cascade-fix surface scales with state-view expansion: adding a field to `*StateView` breaks every literal that lacks `..Default::default()`, which in this codebase often means `winterbaume-terraform/src/converters/<service>.rs` and the per-crate Redis backend wrapper. Inline cascade fixes during verification beat re-dispatching agents.
- Recurring failure modes from /tackle-todos sweeps worth bundling into skill prompts: missing `chrono = { workspace = true }` Cargo.toml dep when an agent calls `chrono::Utc::now()`; SDK getter signature drift when scenario-test agents assume `Option<T>` for getters that return `T` ( e.g. `SshPublicKey::ssh_public_key_id() -> &str` ); state-view literal drift ( derive `Default` on every `*StateView` and component view; always use `..Default::default()` in literals ); state-shape changes breaking terraform converters that mirror the type ( per-crate gates do not catch this; only `winterbaume-terraform` integration tests do ); stale cargo cache from concurrent agents reporting phantom `can't find crate for X` errors ( fall back to `cargo build` ); generator-fix blast-radius asymmetry ( only regenerated crate gets the fix ); and `model.rs` reorders that accompany regens are harmless ( generator HashMap iteration nondeterminism ).
- Per-worktree handler-fix agents land their edits on `worktree-agent-<id>` branches based on whatever commit the worktree was forked from. Because the main checkout typically has uncommitted work and the agent branches have stale bases, `git merge` rarely applies cleanly and the CLAUDE.md ban on `git checkout` / `git restore` / `git stash` rules out the obvious workarounds. Durable procedure: for each worktree, extract the agent's intent via `git diff HEAD` from inside the worktree, inspect the current state of the corresponding files in the main checkout, apply the agent's edits in place via `Edit` against absolute paths, skip changes already landed independently, `cargo fmt -p <crate>` + `cargo test -p <crate>` per crate, then `git worktree unlock` ( if the agent harness still holds the lock ) + `git worktree remove --force` + `git branch -D worktree-agent-<id>`. Worktree-drift gotcha: `cd <worktree-path>` invocations persist across subsequent shell commands, so anchor `git status` / `git rev-parse` with absolute paths or `git -C <path> …`; edits via absolute file paths are unaffected.
- Patch-style worktree merge-back needs an additional verification pass: list untracked files in every source worktree, inspect every `.rej` file, confirm lock-file hunks landed in the intended package block, then run `cargo check -p <crate> --tests` and any newly-added tests by name in the main checkout. A successful `patch` command can still drop new files, apply a dependency hunk to a wrong package block, or leave integration tests uncompiled.
- When agent intent diverges architecturally from the rest of the codebase, prefer the codebase's convention even if the agent's worktree version "worked". Concrete example: the inspector2 worktree refactored multi-account handlers to call `self.state.get(acct, region)` from inside the handler, dropping the `&state` parameter that every other handler took. The cleaner fix that landed in main reshaped state ( `Inspector2State::enabled_resource_types` from `HashSet<String>` to `HashMap<String, HashSet<String>>` keyed by account id ) so handlers kept the conventional `&state` signature and multi-account semantics live in the handler body iterating `extract_account_ids(body)`. Preserves the dispatcher-owns-state-lookup convention used elsewhere.

## Operational Guidance

When adding or expanding a service, use this order:

1. Confirm the service's protocol family, endpoint shape, and existing crate state.
2. Choose the operation slice deliberately. Prefer root-resource lifecycle and tags first; implement full coverage only when the service surface is small enough to finish coherently.
3. Implement or extend `handlers.rs` and `state.rs` with coherent behaviour.
4. For new services, wire every workspace surface before calling the crate complete: Cargo workspace lists, dependencies, root features, server registration, examples, tests, and TODO status.
5. Regenerate `model.rs` and `wire.rs` if the service relies on Smithy output shapes.
6. If the generator changed, bulk-regenerate every affected crate rather than only the first failing one, then rerun `cargo fmt --all --check` and workspace clippy with CI-equivalent flags.
7. Add or refresh SDK integration tests, including debug-formatted service-error assertions and notifier coverage for stateful services.
8. For stateful services, write or refresh the cross-call invariant inventory before scenario tests. Explicitly justify `N/A` rows instead of relying on lifecycle smoke tests.
9. If moto has real tests, port those before adding large numbers of new operations.
10. If moto is absent or incomplete, write a docs-driven test plan that records source links, implemented operations, and exact expected behaviours before adding test cases.
11. Keep domain validation and business failures in `state.rs`, then shape them into AWS-facing errors only in `handlers.rs`.
12. If Terraform resources depend on the service, run or extend Terraform E2E coverage and record any `FIX(terraform-e2e)` markers with fast integration-test follow-up.
13. If Terraform resource converters are relevant, extend `views.rs` first, then add inject or extract coverage against the converter contract rather than mutating private state directly.
14. If provider traces show a protocol or URL style different from SDK tests, implement the provider-facing path rather than assuming SDK compatibility covers it.
15. For nested Terraform blocks, treat the work as full-stack. Update service types and handlers where needed, then `views.rs`, then the converter, then add a no-drift Terraform E2E assertion rather than stopping at converter integration tests.
16. Register new converters where the injector is constructed. Converter code that exists only on disk is not finished.
17. For scoped Terraform extraction, prefer service-level `scopes_with_state()` plus injector `register_with_scopes()` over converter access to private service fields.
18. All `STUB[no-state]` handlers have been promoted. For any new handlers, ensure they are state-backed from the start rather than returning empty/default responses. Defer only operations that truly need a separate engine or external integration.
19. Finish with a quality-gate pass and sort findings into: fix now in the crate, fix in `smithy-codegen`, or defer into `TODO.md` because the remaining work is concrete and still open.
20. Re-check `API_COVERAGE.md`, but treat mismatches as a prompt for local inspection of handler-detection heuristics, integration-test parsing, and Terraform resource-to-crate mappings.
21. If examples or test-only helpers changed, run the same `--all-targets` clippy path CI uses before considering the work clean.
22. If the work was done through parallel agents or worktrees, verify the claimed landing state in the main checkout before writing the result up as durable project knowledge.

Use the strongest available spec source:

- moto tests for behaviour
- AWS docs for uncovered services, operations, or validation rules
- Terraform traces for provider behaviour

For docs-driven work, prefer the stable plan format:

- source URLs
- short service summary
- implemented-operation inventory
- candidate scenarios with explicit expected outcomes
- notes about omissions, idempotency, and validation rules

When the work is architectural rather than service-specific:

- keep the state-layer contract domain-oriented
- put protocol or HTTP shaping at the handler boundary
- update the relevant synthesis or source LTM note when the change establishes a repo-wide pattern rather than a one-off service fix

## Files

- `crates/winterbaume-*/src/handlers.rs`: request routing, error shaping, protocol dispatch, and serialiser calls.
- `crates/winterbaume-*/src/state.rs`: state transitions, validation, and durable semantics.
- `crates/winterbaume-*/src/views.rs`: state view, restore, merge, and notifier wiring for stateful services.
- `crates/winterbaume-*/src/model.rs`: generated Smithy model structs where supported.
- `crates/winterbaume-*/src/wire.rs`: generated serialiser functions and re-exported wire types.
- `crates/winterbaume-*/tests/integration_test.rs`: primary SDK and parity regression layer.
- `crates/winterbaume-*/tests/scenario_test.rs`: chained scenario layer for cross-call invariants.
- `tests/e2e/terraform/harness.rs`: Terraform E2E harness and smoke-test helpers.
- `crates/winterbaume-terraform/src/converters/*.rs`: Terraform inject and extract support built on `StateView`
- `crates/winterbaume-terraform/src/injector.rs`: converter registration, scope providers, and `extract_all()`
- `tools/smithy-codegen/src/model.rs`: service-model parsing, timestamp logic, and resource-operation discovery.
- `tools/smithy-codegen/src/gen_serializers.rs`: serialiser generation and XML, awsQuery, and ec2Query behaviour.
- `.agents/docs/API_COVERAGE.md`: coverage report, useful but imperfect.
- `.agents/docs/QUALITY_GATE.md`: canonical publication and crate-hardening checklist
- `.agents/docs/TODO.md`: deferred quality-gate, stub, and view-fidelity follow-up work
- `.agents/skills/api-coverage/scripts/generate_coverage.py`: coverage and validation reporting, including integration and Terraform E2E layers.

## Tests

Regression layers should be chosen deliberately:

- SDK integration tests for request and response compatibility with `aws-sdk-rust`
- moto parity tests for exact field, error, and idempotency behaviour
- docs-driven tests when moto is absent or incomplete
- Terraform E2E tests for provider compatibility and sub-resource read behaviour
- Terraform converter integration tests for inject or extract fidelity against `StateView`
- quality-gate checks for crate-local build, clippy, fmt, state-view fidelity, and stub annotation hygiene
- focused per-crate build or test runs for architectural refactors such as error-shaping changes

Representative commands mentioned across the source documents:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test -p winterbaume-iam -- --test-threads=1
cargo test -p winterbaume-s3
cargo test -p winterbaume-ec2 -- --test-threads=1
cargo test -p winterbaume-s3tables
cargo test --test e2e_terraform -- --ignored
```

For generated-code regressions, compile and rerun the affected service suite after regeneration rather than trusting generation output by inspection.

## Pitfalls

- Do not treat passing smoke tests as proof of behavioural correctness.
- Do not assert SDK service errors with `Display`; use debug formatting so the AWS exception type is visible.
- Do not infer SDK getter optionality from generated wire structs. Smithy `@required` changes the SDK getter shape.
- Do not leave deferred operation families implicit in partial service crates. Document the scope and return explicit 501 responses.
- Do not copy sibling query-string parsers into new restJson1 crates when shared helpers already exist in `winterbaume-core`.
- Do not hand-edit generated `model.rs` or `wire.rs` as a lasting fix.
- Do not assume Terraform behaviour matches SDK behaviour; provider read paths are often stricter.
- Do not assume Terraform provider traffic uses the same protocol or URL shape as SDK clients.
- Do not treat "too many results" provider refresh failures as flaky E2E noise. They often mean a Describe handler ignored id or filter params.
- Do not write Terraform converters against private runtime state when `views.rs` is the real durable contract.
- Do not treat top-level Terraform converter coverage as proof that nested blocks round-trip correctly.
- Do not trust `API_COVERAGE.md` blindly when generic dispatch or stale detection may be involved.
- Do not let state modules grow HTTP or AWS wire concerns just because the first handler mapping feels repetitive.
- Do not introduce new `STUB[no-state]` handlers — the codebase has zero remaining and new handlers should be state-backed from the start.
- When fixing incorrect API parameter names or handler bugs, verify against the official AWS documentation rather than inferring from code alone. Hallucinated parameter names (e.g. `StackNameOrId` instead of `StackName`) are a known class of LLM-generated bug that reads plausibly but fails at runtime.
- Do not use hard-coded ARNs or identifiers in describe/read integration tests when the API under test requires a real pre-created resource.
- Do not ignore XML, awsQuery, or ec2Query serialization edge cases.
- Do not treat generated serializers as universally correct for raw payload APIs; payload-sensitive exceptions must remain explicit and documented.
- Do not treat `cargo clippy --workspace` as sufficient when CI runs `--all-targets`.
- Do not stop after regenerating one crate if the underlying generator change affects shared emitted patterns.
- Do not turn every quality-gate warning into a TODO. Keep deferred work concrete and still open.
- Keep contradictions visible: operation coverage may look complete while behavioural parity is still weak, and a service may satisfy parity tests while still missing Terraform-specific defaults or sub-resource handlers.
- Do not treat a parallel-agent batch summary as authoritative without checking the main working tree for the claimed state-shape, converter, and test changes.
- Do not regenerate one crate after a generator change and assume the fix has landed for the workspace. The fix only takes effect for the regenerated `wire.rs`; every other restJson1 / restXml / awsQuery crate that uses the same generator path still ships its old `wire.rs` until separately regenerated.
- Do not assume `@httpPayload` outputs work just because the model declares the trait. Pre-2026-04-28 generated serialisers wrapped them; the SDK's `*_correct_errors` then filled missing required fields with defaults and the bug presented as "client receives empty struct". Test against `aws-sdk-rust` clients, not just the wire bytes.
- Do not blindly merge agent-worktree refactors that change handler signatures away from the codebase convention. The codebase's `dispatcher → &state → handler` pattern is load-bearing; rework state shape to match the new requirement instead of pushing state lookup into the handler body.
- Do not rely on prior `cd <worktree-path>` for `git status` / `git rev-parse` calls — the CWD persists across shell commands and the output describes the drifted worktree, not the main checkout. Anchor with absolute paths or `git -C <path> …`.
- Do not treat create/list/delete lifecycle coverage as a substitute for scenario tests. Cross-call invariants often fail only after a second create, modify, associate, or read operation.
- Do not reuse one counter across distinct AWS ID families unless the service's namespace really is shared. Helper names such as `next_*_id` still need review to confirm they increment the matching counter.
