---
name: write-tests
description: Research an AWS service via AWS Documentation MCP Server, plan a comprehensive test suite, write the plan to JOURNAL.md, and implement two complementary test files — per-operation integration tests in `tests/integration_test.rs` and end-to-end scenario tests in `tests/scenario_test.rs`.
argument-hint: <service-name>
user_invocable: true
---

# Write Tests from AWS Documentation

Research an AWS service using the AWS Documentation MCP Server, derive a comprehensive test plan from the API reference, record it in JOURNAL.md, and implement two layers of tests in the service crate:

1. **Per-operation integration tests** (`tests/integration_test.rs`) — exercise each handler in isolation: happy path, error paths, validation, defaults, FIX(terraform-e2e) coverage.
2. **Scenario tests** (`tests/scenario_test.rs`) — chain multiple operations into coherent end-to-end workflows that mirror how real applications use the service.

## Arguments

- `$0` — Service name matching the winterbaume crate suffix (e.g., `ecs`, `dynamodb`, `s3`, `kms`)

---

## Step 0: Resolve names and verify the crate exists

### 0a. Confirm the crate exists

```bash
ls crates/winterbaume-{service}/tests/integration_test.rs
```

If the file is missing, the service must be implemented first via the `add-service` skill.

### 0b. Identify the AWS SDK crate name

Use the mapping table from the `port-moto-tests` skill (e.g., `cognitoidp` → `aws-sdk-cognitoidentityprovider`, `elbv2` → `aws-sdk-elasticloadbalancingv2`, `logs` → `aws-sdk-cloudwatchlogs`). For anything not listed, the sdk crate name is `aws-sdk-{service}`.

### 0c. Identify which operations are implemented

```bash
grep -E 'handle_[a-z]' crates/winterbaume-{service}/src/handlers.rs
```

Build a set of **implemented operations**. Only write tests for operations that winterbaume handles.

---

## Step 1: Fetch AWS documentation using the AWS Documentation MCP Server

Use the AWS Documentation MCP Server to find and read documentation for the service. The goal is to understand:

1. **What the service does** — its purpose, core resources, and typical usage patterns
2. **Resource lifecycle** — how resources are created, configured, updated, and deleted
3. **Key constraints** — naming rules, limits, required fields, immutable fields
4. **Idempotency rules** — which operations are idempotent, which error on duplicate
5. **Error semantics** — what error codes are returned for common failure modes (not-found, conflict, validation)
6. **Relationships** — which resources depend on others (e.g., rules require listeners)

### 1a. Search for the service overview

Use the MCP Server to search for `{ServiceName} overview` and `{ServiceName} concepts`. Read the Developer Guide introduction.

### 1b. Read the API reference

Use the MCP Server to find the API Reference for the service. For each implemented operation, read:
- Input parameters (required vs optional, valid values, constraints)
- Response fields
- Error codes the operation can return

Prioritize reading operations in this order:
1. Core CRUD operations (Create, Get/Describe, List, Delete)
2. Update/modify operations
3. Tagging operations
4. Batch operations
5. Policy / permission operations

### 1c. Extract testable behaviors

For each implemented operation, derive the following test scenarios:

| Category | Description |
|----------|-------------|
| **Happy path** | Create a resource with minimum required fields; verify response fields |
| **Full params** | Create with all optional fields set; verify each field is reflected in response |
| **Full-input round-trip** | Set every documented input field, read the resource back via `Get`/`Describe`, and assert every input is reflected on read. This catches handlers that silently drop fields ( e.g. `CreateVolume` ignoring `Size` from `SnapshotId` when `Size` is omitted ). |
| **Per-call uniqueness** | For any handler that mints an id ( `Create*`, `Allocate*`, `Associate*`, `Attach*` ), call it twice with the same parent / input and assert the two ids differ. This catches counter-reuse bugs ( e.g. `AssociateAddress` minting `eipassoc-XXXX` from the EIP creation counter so re-association duplicates the id ). |
| **List** | List after create; verify the created resource appears |
| **Describe/Get** | Fetch by ID/name after create; verify fields match |
| **Update** | Modify a resource; verify changes are reflected |
| **Delete** | Delete a resource; verify it no longer exists |
| **Not-found error** | Get/Update/Delete a nonexistent resource; verify the correct error code |
| **Conflict error** | Create a resource twice with the same name; verify the correct error code |
| **Validation error** | Pass an invalid input (bad name, out-of-range value); verify the correct error code |
| **Lifecycle** | Full create → describe → update → delete → verify-gone cycle |
| **Tags** | TagResource, ListTagsForResource, UntagResource if the service supports tagging |
| **Pagination** | List with many items; verify pagination tokens work if pagination is implemented |

The **Full-input round-trip** and **Per-call uniqueness** rows are mandatory whenever they apply to the operation; they cover per-handler completeness failures that scenario tests cannot catch ( the handler is wrong on its first call, not on a chain ).

### 1d. Invariant inventory ( cross-call invariants )

Per-operation tests cover what one handler does in isolation. **Cross-call invariants** are properties that connect *two* handlers — state field A written by handler X must be observable through handler Y. The wave-style implementation workflow ( per-handler scope, single-call round-trip ) is structurally blind to these. They surface only when scenario tests chain the right pair of calls. Examples that have produced real bugs in winterbaume:

- `EnableEbsEncryptionByDefault` flips a flag that subsequent `CreateVolume` calls must read for their default `Encrypted` value.
- `ModifyLaunchTemplate` with `SetDefaultVersion` updates `LaunchTemplate.default_version_number` *and* the per-version `LaunchTemplateVersion.default_version` boolean must be rewritten to match.
- `CreateVolume` with `SnapshotId` but no `Size` must inherit `Size` from the snapshot.
- `RunInstances` with `Placement.AvailabilityZone` must propagate the AZ to `Instance.placement`.

For this service, produce an **invariant inventory** table by reading the AWS *user guide* ( not just the API reference — the API ref describes one operation; cross-call invariants live in the developer guide / tutorials ). Append the table to the test plan in `JOURNAL.md` ( see Step 3 ). Every row's "Doc URL" cell is mandatory; it pins the claim to a specific AWS doc page so reviewers can spot-check.

| Category | Question to ask of each handler family | Doc URL |
|---|---|---|
| **Toggle propagation** | Does any `Enable*` / `Disable*` / `Modify*Default*` flip an account-wide flag that other handlers must read for their defaults? | required |
| **Modify rewrites sibling state** | Does any `Modify*` / `Update*` change one field in a way that requires updating sibling fields on related records ( e.g. flipping a default-version pointer requires rewriting per-version booleans )? | required |
| **Per-call uniqueness** | Are there id-minting handlers ( `Associate*`, `Allocate*`, `Attach*` ) where the AWS contract is "each call mints a fresh id, even for the same parent"? | required |
| **Default inheritance from parent** | Does any `Create*` inherit a default field from a referenced parent ( `CreateVolume` from `Snapshot.volume_size`; `RunInstances` from `Subnet.availability_zone` )? | required |
| **Lifecycle state transitions** | Are there state machines whose transitions ( `pending` → `available` → `deleting`, `running` → `stopped` → `terminated` ) must be observable through `Describe*` between calls and through previous/current state codes in mutation responses? | required |
| **Cross-resource references on read** | Do `Describe*` responses include reverse references back to other resources ( SG ingress rules referencing peer SGs by id, VPN connections listing both gateways, route tables listing associations )? | required |

For each row, fill one of:

- A short ( one-line ) statement of the specific invariant for this service, with a doc URL.
- The literal text `N/A — <one-line justification with doc URL>` if AWS doesn't document such an invariant for this service.

Empty rows are not allowed — every category must be either named or explicitly N/A. Genuinely stateless services ( `pricing`, `translate`, `comprehend`, `forecastquery`, `*-data` planes ) will have N/A on most rows; that's correct, but each row still needs the justification.

The author writes the inventory only to `JOURNAL.md`. Promotion into `.agents/docs/services/<service>.md` ( as a durable `## Cross-call invariants` section ) happens later via `good-sleep` / `deep-sleep`; do not edit the service dossier directly from this skill.

---

## Step 2: Read the existing test file

```bash
cat crates/winterbaume-{service}/tests/integration_test.rs
```

Identify:
1. **Existing test function names** — avoid duplicating covered scenarios
2. **`make_client()` helper** — reuse as-is
3. **Existing helpers** — resource creation helpers to reuse or extend
4. **Import patterns** — which SDK types are already imported

---

## Step 2b: Scan for handler fixes from E2E test sessions

Other agent sessions running terraform E2E tests (`write-e2e-tests` skill) discover and fix handler bugs, marking each fix with a `// FIX(terraform-e2e):` comment. These fixes often reveal behaviors that unit/integration tests didn't cover. Scan for them and ensure test coverage.

### 2b-i. Find all fix markers in the service crate

```bash
grep -rn "FIX(terraform-e2e)" crates/winterbaume-{service}/src/
```

### 2b-ii. For each fix, derive a test scenario

Read the fix comment and surrounding code to understand what was wrong. Common fix categories and the tests they imply:

| Fix category | Example fix comment | Test to add |
|---|---|---|
| **Missing response field** | "SqsManagedSseEnabled must be present in GetQueueAttributes" | Call the Get/Describe operation and assert the field is present with correct default value |
| **Missing default attribute** | "Added Owner, Policy, EffectiveDeliveryPolicy" | Create a resource, call Get/Describe, assert defaults are populated (e.g., Policy is valid JSON) |
| **Non-deterministic response** | "TableId must be deterministic" | Call Describe twice for the same resource; assert the field value is identical both times |
| **New operation added** | "Added SetTopicAttributes — terraform calls it after CreateTopic" | Call the operation; verify it succeeds and the attribute is reflected in subsequent Get |
| **Attribute parsing on create** | "Parse and store ContentBasedDeduplication for FIFO queues" | Create with the attribute set; Get and verify the attribute matches the input |
| **ARN normalization** | "ARN normalization in tag_resource" | Call TagResource with both ARN forms (with/without trailing suffix); verify tags are stored and retrievable |
| **Conditional response shape** | "PAY_PER_REQUEST tables must return ProvisionedThroughput with zeros" | Create with the alternative billing mode; Describe and assert the field is present with zero values |

### 2b-iii. Add fix-coverage tests to the plan

Include these derived test scenarios in the test plan table (Step 3) with the source marked as `FIX(terraform-e2e)` rather than "AWS Documentation". These tests have **high priority** because they cover real bugs that were found in production-like terraform workflows.

---

## Step 3: Write the test plan to JOURNAL.md

Append a new section to `.agents/docs/JOURNAL.md`:

```markdown
### Test plan: {service} (YYYY-MM-DD)

**Source**: AWS Documentation ({doc URL or "API Reference for {ServiceName}"})

**Service summary**: <1-2 sentences describing what the service does>

**Implemented operations**: <comma-separated list>

**Test scenarios planned**:

| # | Test function | Operation(s) | Scenario | Source | Expected outcome |
|---|---------------|--------------|----------|--------|-----------------|
| 1 | test_create_{resource}_basic | Create{Resource} | Minimum required fields | AWS docs | Returns resource with correct name/id |
| 2 | test_create_{resource}_duplicate | Create{Resource} | Create same name twice | AWS docs | {ErrorCode} error |
| 3 | test_get_{resource}_not_found | Get/Describe{Resource} | Nonexistent ID | AWS docs | {ErrorCode} error |
| 4 | test_list_{resource}s | List{Resource}s | After creating N items | AWS docs | All N appear in list |
| 5 | test_delete_{resource} | Delete{Resource} | Delete then get | AWS docs | {ErrorCode} on get |
| ... | ... | ... | ... | ... | ... |
| N | test_{fix_specific_scenario} | {Operation} | {scenario from FIX marker} | FIX(terraform-e2e) | {expected behaviour} |
...

**Key behaviors noted from docs**:
- <behaviour 1> (e.g., "Names must match regex `[a-zA-Z0-9_-]{1,64}`")
- <behaviour 2> (e.g., "Delete is idempotent — no error if resource doesn't exist")
- <behaviour 3> (e.g., "Status transitions: CREATING → ACTIVE → DELETING")

**Cross-call invariant inventory** ( from Step 1d — every row must be either named or explicitly N/A with a doc URL ):

| Category | This service's invariant | Doc URL |
|---|---|---|
| Toggle propagation | <one-line invariant or `N/A — <reason>`> | <required> |
| Modify rewrites sibling state | <one-line invariant or `N/A — <reason>`> | <required> |
| Per-call uniqueness | <one-line invariant or `N/A — <reason>`> | <required> |
| Default inheritance from parent | <one-line invariant or `N/A — <reason>`> | <required> |
| Lifecycle state transitions | <one-line invariant or `N/A — <reason>`> | <required> |
| Cross-resource references on read | <one-line invariant or `N/A — <reason>`> | <required> |

**Scenario tests planned** (see `tests/scenario_test.rs`):

| # | Scenario | Operations chained | What it proves |
|---|----------|--------------------|----------------|
| 1 | <use-case name, e.g., Batch job processing pipeline> | <e.g., CreateQueue, SendMessageBatch, ReceiveMessage loop, DeleteMessage> | <e.g., End-to-end producer/consumer drains the queue> |
| 2 | <use-case name> | <ops> | <invariant verified> |
| ... | ... | ... | ... |
```

Keep the per-operation table rows in dependency order: creation first, then reads, updates, deletes, errors. The scenario table is separate because each row is a coherent story, not a step in a sequence. If no scenario tests fit the service (see Step 5), replace the scenario table with `**Scenario tests**: skipped — <one-line justification>.`

---

## Step 4: Implement the tests

Write the planned test functions and append them to `tests/integration_test.rs`.

### 4a. Section header

```rust
// ============================================================================
// Tests derived from AWS documentation: {ServiceName}
// ============================================================================
```

### 4b. Test function template

```rust
#[tokio::test]
async fn test_function_name() {
    let client = make_client().await;
    // ... test body ...
}
```

### 4c. Python-style assertions → Rust

| Goal | Rust pattern |
|------|-------------|
| Field equals string | `assert_eq!(response.name(), "expected")` (required field) |
| Optional field equals string | `assert_eq!(response.name(), Some("expected"))` |
| Field is present | `assert!(response.name().is_some())` |
| Field is absent | `assert!(response.name().is_none())` |
| List length | `assert_eq!(response.items().len(), 3)` |
| First item field | `assert_eq!(response.items()[0].name(), "expected")` |
| Non-empty ID | `assert!(!response.id().unwrap_or_default().is_empty())` |
| Timestamp present | `assert!(response.created_at().is_some())` |

### 4c-2. SDK getter return-type pitfalls

AWS SDK getters do not always return `Option<T>`; some return `T` directly ( e.g. `SshPublicKey::ssh_public_key_id() -> &str`, `FixedResponse::status_code() -> i32` ). `cargo build --tests` is the cheapest way to surface the mismatch — run it before claiming the test works.

### 4d. Error assertions

```rust
let err = client
    .some_operation()
    .some_field("bad-input")
    .send()
    .await
    .unwrap_err();

// Simple string check:
let err_str = format!("{:?}", err);
assert!(
    err_str.contains("ResourceNotFoundException") || err_str.contains("NoSuchResource"),
    "Expected not-found error, got: {err_str}"
);

// Or via typed service error if the SDK provides it:
let service_err = err.into_service_error();
assert!(service_err.is_resource_not_found_exception());
```

### 4e. Resource setup helpers

If multiple tests share the same precondition (e.g., "a cluster must exist"), create a helper after `make_client()`:

```rust
async fn create_test_{resource}(client: &aws_sdk_{sdkname}::Client) -> String {
    client
        .create_{resource}()
        .name("test-{resource}")
        .send()
        .await
        .unwrap()
        .{resource}()
        .unwrap()
        .{resource}_id()
        .unwrap()
        .to_string()
}
```

Always check if an equivalent helper already exists before adding one.

### 4f. FIX(terraform-e2e) coverage tests

If Step 2b found fix markers, append targeted tests under a separate section header:

```rust
// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================
```

Each test should reference the fix it covers:

```rust
// Covers FIX(terraform-e2e): {brief description of the fix}
#[tokio::test]
async fn test_{fix_scenario}() {
    let client = make_client().await;
    // ... verify the fixed behaviour ...
}
```

### 4g. Coverage expectations

- Every implemented operation must appear in at least one test.
- Every test in the plan table must be implemented (or explicitly skipped with a `// TODO` comment explaining why).
- Every `FIX(terraform-e2e)` marker must have at least one test covering the fixed behaviour.
- Lifecycle tests should cover the full create → read → update → delete cycle for each primary resource type.

---

## Step 5: Add scenario tests for realistic workflows

Per-operation tests prove that each handler works in isolation. **Scenario tests** prove that the service supports coherent end-to-end use cases — the way real applications actually call it. They live in a separate file (`tests/scenario_test.rs`) and chain multiple operations to model a single story.

Reference scenario suites already in the tree (read at least one before writing your own):

- `crates/winterbaume-sqs/tests/scenario_test.rs` — batch job pipeline, DLQ configuration, visibility-timeout heartbeat, FIFO ordering
- `crates/winterbaume-sns/tests/scenario_test.rs` — multi-subscriber notification hub, batch publishing, attribute lifecycle, subscription filter policy
- `crates/winterbaume-dynamodb/tests/scenario_test.rs` — optimistic locking, leaderboard query/scan, GSI query, shopping cart with transact-write
- `crates/winterbaume-s3/tests/scenario_test.rs` — document storage workflow, versioned asset pipeline, multipart upload, batch prefix lifecycle, blob recovery

### 5a. Decide whether scenario tests fit this service

Add scenario tests when **at least one** of the following is true:

- The service exposes resources that obviously combine in a workflow (queue + DLQ; topic + subscriptions; table + items; bucket + objects)
- The service has stateful semantics that emerge only across multiple calls (FIFO ordering, optimistic locking, version history, multipart assembly, condition expressions)
- A realistic application would chain 3+ operations in a single handler (publish/subscribe; create + put + get + delete; transactional writes)

Skip scenario tests when:

- Every operation is independent and stateless (e.g. `Translate.TranslateText`, `Pricing.GetProducts`) — the integration tests already cover everything
- The service is a thin metadata API with no sequencing semantics

If unsure, prefer to add 2-3 scenario tests. Under-coverage is the more common failure mode.

### 5b. Identify candidate scenarios

Pick scenarios from the service's documented **use cases** (the developer-guide "common patterns" / "tutorials" / "getting started" sections), not from the API reference. Useful prompts to ask yourself:

- "What is the canonical first thing a developer does with this service?"
- "Which two or three resources are designed to be used together?"
- "What stateful invariant would surprise a user if violated?" (e.g. FIFO order, condition expressions, version IDs, idempotency tokens)
- "What does the service's example code in the AWS docs actually do?"

A good scenario suite has 3-5 tests, each modelling a distinct realistic story. Aim for breadth across the service surface rather than re-testing the same path.

### 5c. Create `tests/scenario_test.rs`

Use this skeleton (adapt from `crates/winterbaume-sqs/tests/scenario_test.rs`):

```rust
//! Smoke tests for winterbaume {ServiceName} service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_{sdkname}::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_{service}::{ServiceName}Service;

async fn make_{service}_client() -> aws_sdk_{sdkname}::Client {
    let mock = MockAws::builder().with_service({ServiceName}Service::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_{sdkname}::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_{sdkname}::Client::new(&config)
}
```

The client helper is local to this file — do not share with `integration_test.rs`. Each scenario test should be self-contained: create the resources it needs, exercise them, assert the end-state.

### 5d. Test naming and structure

Each scenario test:

- Is named after the **use case**, not the API: `test_batch_job_processing_pipeline`, not `test_send_message_batch`
- Has a doc comment opening with `/// Scenario: <one-line summary>.` followed by a paragraph describing the user story
- Uses **plausible names** that match the scenario (`jobs-queue`, `order-events`, `users-table`, `analytics-bucket`) — not `test-1`, `tmp-foo`
- Performs **end-to-end work** — produce, consume, verify final state
- Asserts the **business outcome** (queue empty after processing; all subscribers received the event; version history matches; conditional update was rejected) rather than re-asserting individual response fields already covered in `integration_test.rs`
- Uses `expect("<helpful message>")` or `unwrap_or_else(|e| panic!(...))` so a failing intermediate step points at the offending operation

### 5e. Handling unimplemented dependencies

If a scenario depends on a feature that is not yet implemented in winterbaume (e.g. S3 versioning, SNS message filtering), still write the test, mark it `#[ignore]`, and add a doc comment that links to a tracked entry in `.agents/docs/TODO.md`. This makes the test a forcing function that flips on automatically when the underlying feature lands.

```rust
/// Scenario: versioned asset pipeline.
///
/// NOTE: this test is marked `#[ignore]` because S3 object versioning is not
/// yet implemented. See TODO.md: "s3: implement object versioning".
#[tokio::test]
#[ignore]
async fn test_versioned_asset_pipeline() {
    // ... full scenario as if the feature worked ...
}
```

If you mark anything `#[ignore]`, also append (or update) the matching TODO entry under `.agents/docs/TODO.md` so the gap stays visible.

### 5f. Coverage expectations

- Every scenario in the JOURNAL plan table (see Step 3) must be implemented (or explicitly skipped via `#[ignore]` with a TODO link).
- Each scenario should exercise at least 3 distinct operations — if you only need one or two operations to tell the story, it belongs in `integration_test.rs`, not `scenario_test.rs`.
- Do not duplicate per-operation assertions from `integration_test.rs` — focus on the **interactions** between operations, the **stateful invariants**, and the **end-state** of the workflow.
- **For every non-N/A row of the Step 1d invariant inventory, at least one scenario must explicitly exercise that invariant.** Mark the row each scenario covers in a doc comment ( `/// Invariant: toggle propagation — EnableEbsEncryptionByDefault → CreateVolume default.` ). A row with no covering scenario fails the publication gate ( see `quality-gate` §2 ).

---

## Step 6: Build, lint, and test

```bash
./.agents/bin/cargo.sh build -p winterbaume-{service}
./.agents/bin/cargo.sh clippy -p winterbaume-{service} --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh fmt -p winterbaume-{service} -- --check
./.agents/bin/cargo.sh test -p winterbaume-{service} -- --test-threads=1
```

The clippy and fmt gates are mandatory before reporting; if they fail, fix the violations and re-run. New test code is the most common source of `unnecessary_sort_by`, `collapsible_match`, and similar lints — running clippy here prevents those from reaching CI.

> **`--maxfail` is pytest, not cargo.** `--maxfail` is a pytest flag, not a Rust libtest flag. For Rust crate verification, use focused filters ( `cargo test <pattern>` ), `--test-threads=N`, `--no-fail-fast`, and per-crate gates instead. Do not pass `--maxfail` to cargo test.

This runs both `integration_test.rs` and `scenario_test.rs`. To target one file:

```bash
cargo test -p winterbaume-{service} --test scenario_test
cargo test -p winterbaume-{service} --test integration_test
```

If tests fail:

1. **Compilation errors** — Fix type mismatches, missing imports, wrong method names. Check the SDK model types under `~/.cargo/registry`.
2. **Assertion failures** — The winterbaume implementation may differ from AWS docs. Either:
   - Fix the winterbaume handler/state to match the documented behaviour
   - Weaken the assertion with a `// TODO: winterbaume returns X, AWS returns Y` comment
3. **501 Not Implemented** — The operation is a stub. Remove the test or mark it `#[ignore]` with a TODO comment.
4. **Scenario test fails partway through a chain** — A scenario test that fails on a later step often points to a missing or wrong response field that an earlier integration test did not catch. Fix the handler, then add a targeted integration-test assertion so the gap is caught at that layer too.

Run tests iteratively — fix one at a time:

```bash
cargo test -p winterbaume-{service} -- test_function_name
```

---

## Step 7: Report

After all tests pass, append a completion note under the same JOURNAL.md section:

```markdown
**Implementation result** (YYYY-MM-DD):
- Integration tests implemented: N (in `tests/integration_test.rs`)
- Scenario tests implemented: S (in `tests/scenario_test.rs`) — or "skipped (justification)" if Step 5a determined none fit
- Tests skipped (`#[ignore]`): M (with TODO refs)
- FIX(terraform-e2e) coverage: K tests added for handler fixes
- Bugs found and fixed: <list or "none">
- `cargo test -p winterbaume-{service}` — all (N + S) tests pass
```

Then report to the user:
1. Number of integration tests and scenario tests written and passing
2. Operations covered
3. Any bugs found in winterbaume during testing
4. Any planned tests that were skipped (or scenario tests intentionally not added) and why
