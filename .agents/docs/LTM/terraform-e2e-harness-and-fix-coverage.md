# Terraform E2E Harness and Fix Coverage

## Summary

Terraform compatibility exposed a separate class of problems from SDK integration tests. The AWS Terraform provider performs extra reads, waiter loops, refresh passes, and state reconciliation steps that require specific response fields and sub-resource handlers. Winterbaume's E2E harness, provider-compatibility fixes, and the later `FIX(terraform-e2e)` audit turned those discoveries into durable infrastructure.

## Key Facts

- Terraform E2E failures often reveal provider-specific behaviour that SDK integration tests do not exercise.
- `TF_LOG=TRACE` is a debugging tool, not a default runtime mode.
- `FIX(terraform-e2e)` became a cross-skill contract: provider-driven fixes should later gain fast integration-test coverage.
- Some services must remain isolated from batching because provider refresh or update paths can poison a shared wave.
- The old incremental shared-state batch design was retired. The durable design is fresh per-wave directories plus shared provider caching.
- Timeout cleanup must kill the whole Terraform process group, not only the parent Terraform process.
- Terraform E2E reporting depends on explicit resource-to-crate mapping tables in `generate_coverage.py`.
- The AWS provider can use a different protocol or URL style from the SDK integration tests. CloudWatch query-protocol requests and S3 virtual-hosted-style local URLs are reference examples.

## Details

### Why Terraform Finds Different Bugs

The provider is stricter and noisier than the SDK path:

- it performs waiter loops
- it re-reads resources repeatedly
- it expects some nested structs to exist even when empty
- it may refresh resources in a different order from the original create path

That produced recurring bug classes:

- omitted response fields
- wrong identifier formats
- bad filter semantics
- ARN normalisation mismatches
- read-after-write contract gaps

### Harness Foundations

The early durable fixes were:

- use `tokio::process::Command`
- run tests under a multi-threaded Tokio runtime
- cache Terraform provider plugins
- move harness logic into reusable helpers under `tests/e2e/terraform/`

This established the repo's normal E2E structure:

- `tests/e2e/terraform/main.rs`
- `tests/e2e/terraform/harness.rs`
- per-service Terraform modules

### TRACE Logging Discipline

Provider traces are often the only reliable way to discover missing sub-resource reads or waiter requirements.

The durable rule is:

- normal apply and plan paths run without TRACE
- smoke tests and timeout diagnostics can enable TRACE
- TRACE output should be preserved only when it helps explain a failure

### Batch Apply, Reworked

The original batch scheduler reused one growing Terraform state and one persistent `main.tf`. That reduced init overhead, but it created a worse long-run problem:

- wave after wave accumulated resources
- later refresh passes re-read the entire historical state
- one hanging resource could poison every later wave

The later redesign established the durable batching model:

- keep shared provider caching and batch coordination
- create a fresh working directory for each wave
- run Terraform against only the HCL for the current wave
- isolate failures to the current batch rather than the entire historical state

This keeps batching useful without letting state accumulation turn refresh into the dominant failure mode.

### Process-Group Cleanup

Timeout cleanup must kill the whole Terraform process group.

The durable mechanism is:

- spawn Terraform in its own process group
- kill the process group on timeout
- clean up stale lock files afterwards

The later shift from spawning `kill` as a subprocess to calling `libc::killpg` directly matters because it removed PATH dependence, extra forking, and race-prone shell behaviour from the timeout path.

### FIX(terraform-e2e) Loop

The repo later audited all `FIX(terraform-e2e)` markers to ensure they were not E2E-only knowledge.

The durable loop is:

1. provider failure exposes a handler or state bug
2. fix site gets a `FIX(terraform-e2e)` marker
3. a fast integration test is added for the same behaviour

That matters because E2E coverage is slower and more fragile than crate-local integration coverage.

### Provider Version Drift

The E2E harness originally pinned no AWS provider version (`source = "hashicorp/aws"` with no `version` constraint). Because the CI plugin cache is ephemeral, each runner download fetches the latest provider. Provider v5.x and later removed or renamed several attributes:

- `aws_redshift_cluster`: inline `logging` and `snapshot_copy` blocks removed (provider v5.45+) — must use separate `aws_redshift_logging` and `aws_redshift_snapshot_copy` resources.
- `aws_sfn_state_machine`: `log_destination` changed from a nested block to a plain string attribute.

Durable rule: pin the provider version in `write_provider_tf()` to make upgrades explicit and prevent silent CI drift. The harness now intentionally targets provider `~> 6.0` because newer E2E fixtures use v6-only or v6-renamed schemas, including `aws_batch_compute_environment.name` and `aws_applicationcostprofiler_report_definition`.

### New E2E Module Wiring Checklist

A test module that compiles without errors can still silently fail at runtime if it is not wired into the harness. Every new E2E test module requires all three of:

1. `Cargo.toml` dev-dependency for the service crate.
2. `test_services()` registration entry in `harness.rs`.
3. Provider endpoint override in the harness configuration.

Compile-only verification is insufficient: missing runtime wiring causes requests to leak to real AWS endpoints, returning `403 InvalidClientTokenId` instead of a local mock response.

### Representative Compatibility Lessons

Several service-specific findings became general rules:

- S3 is not Terraform-compatible until bucket sub-resources such as ACL, policy, versioning, encryption, logging, website, ownership controls, public access block, and request payment all read back correctly.
- SQS needed queue attributes that are easy to miss from CRUD-only testing.
- CloudFront showed that some nested structs must be present with zero values because the provider dereferences them without nil checks.
- ELBv2 showed that waiter-backed reads must honour exact ARN or name filters.
- EKS showed that local harness URIs need explicit path-normalisation fallback, not only real AWS hostname handling.
- Transfer and Cognito IDP showed that identifier shape matters as much as the rest of the response body.
- Redshift provider reads `ClusterAvailabilityStatus` ( title-case `"Available"` ), not the fine-grained `ClusterStatus` field ( lowercase `"available"` ), so both fields must be populated by `cluster_to_wire`. The `ClusterStatus` type was refactored to an enum with `as_str()` (lowercase) and `availability_status()` (title-case) methods.
- Redshift provider accesses `ClusterParameterGroups[0]`, `ClusterNodes[0]`, and `MultiAZ` unconditionally. If any of those are absent or empty, the provider panics or rejects the response. `cluster_to_wire` must always return at least one parameter group, at least one node entry, and a non-empty `MultiAZ` string.
- Batch compute environment Terraform state uses the `name` attribute, not the AWS API's `compute_environment_name` field. Converter injection and extraction must follow provider schema names.
- CloudWatch's Terraform provider path uses awsQuery even though the Smithy primary protocol path in Winterbaume is rpc-v2-cbor. Local handlers must support the protocol the provider actually sends.
- ELBv2 listener rule responses must include full generated action fields such as `Order`; partial hand-written XML can crash the provider during readback.
- S3 provider v6 defaults to virtual-hosted-style requests. Local smoke configurations must set `s3_use_path_style = true` unless the mock host-routing layer explicitly supports names such as `bucket.localhost`.
- Provider v6 tightened some waiter enum comparisons. EC2 `NetworkInterfacePermission.permission_state` must return uppercase enum values such as `GRANTED`, not the lower-case strings older provider versions tolerated.
- ELBv2 listener-rule condition responses must populate both the legacy `Values` field and the matching `*Config.Values` block ( `HostHeaderConditionConfig`, `PathPatternConditionConfig`, `HttpRequestMethodConditionConfig`, `SourceIpConditionConfig` ). Provider v6 dereferences the config block that matches `Field` and can crash if it is absent.
- EC2 Describe handlers must filter aggressively when Terraform reads a resource by id. `tfresource.AssertSinglePtrResult` treats zero rows and more than one row as NotFound-class failures, so returning all rows is only safe when the resource family is guaranteed singleton in state.
- EC2 generic Describe operations must include every subtype sharing the id namespace. Transit Gateway attachments are the reference case: VPC, peering, and connect attachments all use `tgw-attach-...` ids.
- Some provider failures are upstream framework mapping bugs. `aws_ec2_capacity_block_reservation` in provider v6.43.0 uses AutoFlex without mapping `CapacityReservationArn`, `CreateDate`, and `TotalInstanceCount` to `ARN`, `CreatedDate`, and `InstanceCount`; the same mock response works for the legacy SDK-v2 capacity-reservation resource.

### 2026-04-24 Provider Failure Triage

CI run `24880947065` showed 21 Terraform E2E failures across Batch, CloudWatch, CodeCommit, and ELBv2. The durable root causes were not generic test flakiness:

| Service | Root cause | Durable fix |
|---------|------------|-------------|
| Batch | Converter read `compute_environment_name`; provider state uses `name` | Use `name` in inject and extract paths |
| CloudWatch | Provider sent awsQuery requests, but handler only spoke rpc-v2-cbor | Add dual-protocol dispatch, query body parsing, XML response conversion, and generated `_query` serializers |
| ELBv2 | Listener rule hand-written XML omitted provider-read fields like action `Order` | Use generated wire serializers and full domain-to-wire rule conversion |
| CodeCommit | Collateral failure from CloudWatch batch cancellation | Fix CloudWatch root cause |

The recurring lesson is to inspect the provider request/response trace before assuming a service handler is wrong in the obvious way. Provider crashes often come from a missing nested field, a protocol mismatch, or a Terraform schema attribute name that differs from the AWS API shape.

### 2026-04-30 Provider v6 Failure Triage

CI run `25089387163` reported 18 Terraform E2E failures, but they collapsed into four durable root causes:

| Bucket | Tests affected | Durable lesson |
|--------|----------------|----------------|
| Provider v5 vs v6 fixture drift | 11 | A single v6-only fixture inside a `batch_apply` wave can poison unrelated tests sharing the same Terraform apply. Check the resource Terraform reported, not only the Rust panic line. |
| CloudWatch CBOR Tag 1 to XML | 4 | Multi-protocol adapters must preserve protocol-specific sentinels. The CBOR path was correct; only the awsQuery XML conversion needed Tag 1 -> RFC 3339 formatting. |
| EC2 enum casing | 1 | Provider v6 waiter loops may compare Smithy enum spelling exactly. Store and return canonical enum values such as `GRANTED`. |
| ELBv2 nil-deref in provider | 2 | A provider plugin crash inside `resource*Read` usually points to an omitted optional field that the provider assumes is non-nil. Read the provider source at the panic line to identify the exact dereference. |

The local verification path after the fixes was per-crate fmt and clippy for `winterbaume-cloudwatch`, `winterbaume-ec2`, and `winterbaume-elasticloadbalancingv2`, plus focused integration tests: CloudWatch 48/48, ELBv2 76/76, EC2 357/357. The ignored E2E suite still needs a follow-up CI run for final validation.

### 2026-05-02 EC2 Provider v6.43 Rescue

CI run `25219922919` reported 8 EC2 Terraform E2E failures. Seven were mock-side gaps:

| Tests | Durable fix |
|-------|-------------|
| Local Gateway route and route-table VPC association | Auto-seed pre-existing Outposts-style Local Gateway route tables when a known id is first referenced. |
| Traffic mirror and network insights stacks | Filter `DescribeNetworkInterfaces` by `NetworkInterfaceId.N`. |
| Transit Gateway VPC attachment and connect stack | Include attachment `options`, `subnetIds`, and all attachment subtypes in generic reads. |
| IPAM full-stack and pool-CIDR allocation | Filter IPAM scopes by explicit ids and by standard filters, including `ipam-scope-id`, `ipam-id`, and `is-default`. |

The 8th failure, capacity block reservation, is an upstream provider issue. The mock emits the Smithy-modelled fields and the AWS SDK reader consumes them, but the framework resource model uses different field names and AutoFlex is not configured to bridge them. Keep this as a provider-compatibility TODO rather than distorting EC2 XML output.

### 2026-04-30 Batch-Wave Poisoning and Skill Hardening

CI run `25142638619` reported 13 failing Terraform tests, but triage collapsed them into two root causes plus one endpoint-wiring miss:

| Cluster | Durable lesson |
|---------|----------------|
| `applicationcostprofiler` wave | Verify every `aws_*` resource type exists in `hashicorp/terraform-provider-aws` before writing a test. `aws_applicationcostprofiler_report_definition` did not exist, and that invalid HCL poisoned unrelated tests in the same wave. |
| `inspector2` waves | Multiple tests in one batch must not contend on account-singleton resources. Use unique account IDs, isolate the server per test, or collapse the tests. |
| `appfabric` / `appflow` | Harness registration is not enough; provider endpoint overrides must include the service or requests leak to real AWS and fail with `UnrecognizedClientException`. |

The `write-e2e-tests` skill now requires resource-existence checks, full ignored-suite smoke testing with Rust libtest flags, singleton-contention review, and a pre-report self-check. A passing per-service filtered run is not enough because wave poisoning only appears when unrelated tests share one `terraform apply`.

### 2026-04-26 New E2E Modules

Three new E2E test modules landed alongside the AppFabric / AppFlow / Application Cost Profiler converters:

- `crates/winterbaume-e2e-tests/tests/terraform/appfabric.rs` -- basic AppBundle and AppBundle-with-CMK creation tests.
- `crates/winterbaume-e2e-tests/tests/terraform/appflow.rs` -- S3 -> S3 Flow apply ( S3 connector profiles are AWS-managed by default so no `CreateConnectorProfile` is needed; that operation is still a 501 stub in `winterbaume-appflow` ).
- `crates/winterbaume-e2e-tests/tests/terraform/applicationcostprofiler.rs` -- CSV-daily and Parquet-monthly report-definition creation tests.

A separate session pass added three EC2 PlacementGroup E2E tests in `tests/terraform/ec2.rs`: `test_ec2_placement_group_cluster_basic`, `test_ec2_placement_group_partition`, `test_ec2_placement_group_with_tags`. All pass against real `terraform apply` with provider `~> 5.0`.

For each new E2E module, three additional files are touched:

1. `crates/winterbaume-e2e-tests/Cargo.toml`: add `winterbaume-<name> = { workspace = true }` dev-dep.
2. `crates/winterbaume-e2e-tests/tests/terraform/main.rs`: add `mod <name>`.
3. `crates/winterbaume-e2e-tests/tests/terraform/harness.rs`: register the service in the `services()` vec so the in-process winterbaume HTTP server answers the SDK calls. Compile-only verification is insufficient: missing harness registration causes requests to leak to real AWS endpoints, returning `403 InvalidClientTokenId`.

### `extract_path()` "amazonaws.com" Antipattern

A workspace-wide antipattern surfaced when 12 newly-added services were wired into the E2E harness and four ( guardduty, inspector2, iot, mediapackage ) all failed the same way. Each crate had a local `extract_path()` ( or `extract_path_and_query()` ) helper that only stripped scheme/host when the URI literal contained `amazonaws.com`. Under terraform's custom endpoint ( `http://127.0.0.1:PORT/...` ) the helper returned the entire URI as the "path" so dispatch fell through to the catch-all 404. The shared `winterbaume_core::extract_path` is the correct general-purpose strip and must be used.

A grep across the workspace identified the same pattern still present in: `bedrock`, `vpclattice`, `synthetics`, `bedrockagent`, `ram`, `efs`, `sesv2`, `macie2`, `securityhub`, `osis`, `s3tables`. None are currently exercised by terraform E2E so they pass today, but they will fail the same way once exercised. Tracked in `TODO.md` as `systematic-extract-path-antipattern`.

The durable rule: any new service handler that does its own URL parsing should delegate to `winterbaume_core::extract_path` rather than rolling a local helper.

### Tag URL-Style Routing Variance

Tag operations route differently per service, and assuming the wrong shape gives silent 404s rather than loud failures. Reference patterns from the 8-service E2E batch:

- MediaLive uses URL-encoded ARN in the path ( `/prod/tags/{arn}` ) -- handler must split ARN on `:` or `/` and match by full-ARN, full-id, or trailing-id.
- MediaStore uses the awsJson `Resource` body field carrying the ARN.
- IoT uses POST-with-body ( `POST /tags`, `POST /untag`, `GET /tags` ) with ARN-keyed tag storage on the service state.
- DirectConnect uses `resourceArns: [...]` ( describe ) and `resourceArn` + `tags` / `tagKeys` ( tag / untag ) on the awsJson body.

Pick the pattern from the smithy model rather than assuming awsJson, and do not invent a new pattern for parity with sibling services.

### Provider Nil-Deref Crashes Look Like Mock Bugs

terraform-provider-aws v5 panics in `flattenCampaignHook` / `flattenCampaignLimits` / `flattenQuietTime` when those nested objects are absent from the response. The crash trace looks like a malformed winterbaume response, but the actual fix is on the mock side: always emit non-nil default-empty objects when the AWS shape allows it ( pinpoint `app_settings_to_wire` was the reference fix ). Worth checking other settings-style endpoints under v5.x.

### Multi-Resource Batch Waves Expose Per-Account State

Terraform applies that create multiple instances of the same resource in one wave can expose per-account state assumptions. Inspector2 had a single shared `BackendState` for all accounts; three `aws_inspector2_enabler` resources in the same wave caused drift between create and refresh. The durable fix is per-`(account, region)` state buckets when the AWS API is parameterised on `accountIds`. Auditing other services that take `accountIds` in their request body is worthwhile.

### Mediastore ARN Format Fix

Mediastore initially emitted `arn:aws:mediastore:container:<name>` ( missing region/account, `:` instead of `/` before name ). Terraform's `ListTagsForResource` round-trips the ARN, so the server could not look up the container. The standard ARN shape is `arn:aws:mediastore:<region>:<account>:container/<name>`. Companion handler fix: ARN-by-suffix lookup ( split on `/`, take trailing component as the container name ).

Also: AWS provider validates mediastore container names client-side as `[A-Za-z0-9_]+`. Hyphens are rejected; tests and Terraform configs must use underscores. Generic rule: when authoring E2E test resource names, prefer underscore-only identifiers unless the smithy model documents `[A-Za-z0-9_-]+`.

### Worktree Merge-Back Procedure

Per-worktree handler-fix agents land their edits on `worktree-agent-<id>` branches based on whatever commit the worktree was forked from. After the agents return, those branches must be merged back into the main checkout. Since the main checkout typically has uncommitted work and the agent branches have stale bases, `git merge` rarely applies cleanly, and the CLAUDE.md ban on `git checkout` / `git restore` / `git stash` rules out the obvious workarounds.

The durable procedure that worked for the 2026-04-28 8-worktree batch:

1. For each worktree, run `git diff HEAD` from inside the worktree to extract the agent's intent ( changes against the worktree's stale base ).
2. Inspect the current state of the corresponding files in the main checkout.
3. Apply the agent's edits in place via `Edit` against absolute paths in the main checkout. Skip any change that has already landed independently in main; merge the rest semantically.
4. `cargo fmt -p <crate>` and `cargo test -p <crate>` per crate before moving on.
5. After all worktrees are merged, remove them with `git worktree unlock <path>` ( if the agent harness still holds the worktree lock — pid in the lock file ), then `git worktree remove --force <path>`, then `git branch -D worktree-agent-<id>`.
6. `git worktree list` should show only the main checkout when done.

**Worktree-drift gotcha:** when a `cd /path/to/.claude/worktrees/agent-XXX` invocation runs to inspect a per-worktree diff, the CWD persists across subsequent shell commands. `git status` and `git rev-parse` from the drifted CWD report the worktree's branch and modified-file list, even when all `Edit` calls target absolute paths in the main checkout. The mismatch only surfaces when worktree-removal fails with "cannot remove a locked working tree". Lesson: when running tools that print branch / status from `cwd`, do not rely on prior `cd` to a worktree — always anchor with an explicit absolute path for status checks, or use `git -C <path> …`. Edits via absolute file paths remain correct regardless of CWD, but interpretive commands need the right anchor.

When agent intent diverges architecturally from the rest of the codebase, prefer the codebase's convention even if the agent's worktree version "worked". Concrete example: the 2026-04-28 inspector2 worktree refactored multi-account handlers to call `self.state.get(acct, region)` from inside the handler, dropping the `&state` parameter that every other handler in the file took. The cleaner fix that landed in main:

- Changed `Inspector2State::enabled_resource_types` from `HashSet<String>` to `HashMap<String, HashSet<String>>` keyed by account id. The state is now genuinely cross-account ( one bucket per region rather than per-(account, region) pair for this field ).
- `enable` / `disable` / `get_account_status` already took an `account_id: &str` parameter, so they were updated to read / write the matching map entry. `build_resource_status` and `compute_account_status` now also take `account_id`.
- `Inspector2StateView::enabled_resource_types` and the `From` round-trip / `merge` paths in `views.rs` were updated to the new shape.
- Handlers keep the conventional `&state` signature. Multi-account semantics live in the handler body: it iterates `extract_account_ids(body)`, takes a single write/read lock on the shared state, and accumulates per-account `wire::Account` entries into the response.

This preserves the dispatcher-owns-state-lookup convention used elsewhere in the codebase while giving Inspector2 the multi-account write semantics that terraform's `aws_inspector2_enabler` requires.

### 12-Service Wiring Batch + 8-Service Parallel-Worktree Batch

Two scaling patterns proved out:

- **File-only generation in parallel, serial registration.** 12 parallel agents generated test files ( one per service ); a coordinator then performed a single serial registration pass updating `Cargo.toml` dev-deps, `tests/terraform/main.rs` ( `mod <service>;` ), and `harness.rs` ( `test_services()` + `write_provider_tf()` endpoints ). This avoided the `Cargo.toml`-merge-conflict footgun that parallel registrations would have caused.
- **Per-worktree handler-fix agents.** 8 parallel agents, each in its own isolated worktree, tackled one failing service's handler bug. Each agent ran focused ignored Terraform E2E tests through `./.agents/bin/cargo.sh test -p winterbaume-e2e-tests --test main -- --ignored <service> --test-threads=1` and reported its pass count. Do not pass pytest-only flags such as `--maxfail` through Rust libtest. The runtime auto-cleans worktrees with no changes; agents that report file edits but no worktree path likely ran in the main checkout or the cleanup heuristic mis-fired -- worth a manual `git status` check before merging.

Worktree changes are NOT auto-merged back to the main checkout. Cherry-picking or `git merge --squash` per branch is a follow-up step requiring user approval; record the branch names in the JOURNAL entry so the merge step has the list.

### Isolation Rules

Some services should not use shared batch waves.

Reference cases:

- Kinesis, because provider paths can fail client-side before the mock sees the malformed request
- ECS, because shared refreshes can induce drift-sensitive update paths
- other tightly coupled provider graphs where one refresh failure can poison unrelated resources in the same wave

The durable rule is to isolate services whose provider behaviour is unusually stateful or whose malformed-refresh paths are outside the mock's control.

## Files

- `tests/e2e/terraform/harness.rs` - shared server startup, Terraform invocation, batching, timeout handling, and diagnostics
- `tests/e2e/terraform/main.rs` - E2E test entry point
- `tests/e2e/terraform/*.rs` - per-service Terraform E2E modules
- `.agents/skills/api-coverage/scripts/generate_coverage.py` - Terraform resource-to-crate coverage mapping
- `.agents/docs/TODO.md` - provider-driven deferred work surfaced by the harness
- `crates/winterbaume-terraform/src/converters/batch.rs` - provider-schema attribute-name fix for compute environments
- `crates/winterbaume-cloudwatch/src/handlers.rs` - dual rpc-v2-cbor / awsQuery request handling
- `crates/winterbaume-elasticloadbalancingv2/src/handlers.rs` - generated listener-rule response serialization

## Test Coverage

- the harness later reached a full-suite milestone of 315 passing Terraform E2E tests after the fresh-wave batching redesign
- the `FIX(terraform-e2e)` audit added a substantial set of fast integration tests across affected crates
- later coverage-report work made Terraform E2E depth visible in `.agents/docs/API_COVERAGE.md`
- focused Terraform and converter integration tests were used to validate Batch provider schema fixes, CloudWatch protocol fixes, and ELBv2 listener-rule response shape fixes

## Pitfalls

- Do not leave TRACE enabled by default.
- Do not batch services whose provider refresh paths can poison the whole wave.
- Do not kill only the Terraform parent process on timeout.
- Do not treat E2E-only fixes as finished until crate-local integration tests cover the same behaviour.
- Do not trust missing Terraform coverage in the report until you have checked the mapping tables.
- Do not leave the AWS provider unpinned. Ephemeral CI caches combined with an unpinned provider mean any HashiCorp release can silently break tests.
- Do not register a new E2E test module with only `mod <name>;`. The full wiring checklist ( Cargo.toml dev-dep, `test_services()` registration, endpoint override ) is required.
- Do not leave optional response fields unpopulated for Redshift clusters. The provider dereferences `ClusterParameterGroups[0]`, `ClusterNodes[0]`, and `MultiAZ` without defensive checks.
- Do not assume SDK protocol traffic matches Terraform provider protocol traffic.
- Do not assume S3 local Terraform smoke tests use path-style requests unless `s3_use_path_style = true` is set.
- Do not run `cd <worktree-path>` and then trust subsequent `git status` / `git rev-parse` output to describe the main checkout. The CWD persists across shell commands; use absolute paths or `git -C <path> …` for status checks. Edits via absolute file paths are unaffected.
- Do not blindly merge agent-worktree refactors that change handler signatures away from the codebase convention. The codebase's `dispatcher → &state → handler` pattern is load-bearing; rework state shape to match the new requirement instead of pushing state lookup into the handler body. Inspector2's `Vec<String>` → `HashMap<account, HashSet<region>>` rework is the canonical example.
- When converters or backend wrappers ( e.g. `winterbaume-dynamodb-redis` ) construct `*StateView` literals, always include `..Default::default()`. State views are append-only — every new field added in another agent's session breaks every literal that lacks the spread. Per-crate quality gates do not catch this; only `winterbaume-terraform` integration tests do.
