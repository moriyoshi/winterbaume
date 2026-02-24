---
name: port-moto-tests
description: Port moto's Python test cases for a given AWS service to winterbaume Rust integration tests. Reads moto tests from the moto GitHub repository, translates to aws-sdk-rust + MockAws style, and appends to existing integration_test.rs.
argument-hint: <service-name> [test-file-filter...]
user_invocable: true
---

# Port Moto Tests

Read moto's Python test suite for a given AWS service and translate the test cases into winterbaume Rust integration tests, appending them to the service crate's `tests/integration_test.rs`.

## Arguments

- `$0` — Service name matching the winterbaume crate suffix (e.g., `ecs`, `dynamodb`, `s3`, `kms`)
- `$1...` — (optional) Filter to specific moto test file(s) (e.g., `test_ecs.py`, `test_s3_lifecycle.py`). If omitted, port from the main test file (`test_{moto_name}.py`).

---

## Step 0: Resolve service names and locate files

### 0a. Winterbaume crate

```bash
ls crates/winterbaume-{service}/tests/integration_test.rs
```

If the crate or test file doesn't exist, abort — the service must already be implemented via the `add-service` skill first.

### 0b. Moto test directory

The moto test directory is `tests/test_{moto_name}/` in the [moto repository](https://github.com/getmoto/moto). The moto name usually matches the crate suffix but there are exceptions. Use this mapping:

| Crate suffix | Moto test dir |
|-------------|---------------|
| `cognitoidp` | `test_cognitoidp` |
| `cognitoidentity` | `test_cognitoidentity` |
| `config` | `test_config` |
| `elbv2` | `test_elbv2` |
| `events` | `test_events` |
| `logs` | `test_logs` |
| `secretsmanager` | `test_secretsmanager` |
| `lambda` | `test_awslambda` |
| `organizations` | `test_organizations` |
| `route53` | `test_route53` |
| `costexplorer` | `test_ce` |
| `dynamodbstreams` | `test_dynamodb` (subset) |

For anything not listed, try `test_{service}` first. If not found:

```bash
Use WebFetch to browse https://github.com/getmoto/moto/tree/master/tests and search for the service name.
```

### 0c. List moto test files

```bash
Use WebFetch to list files at https://github.com/getmoto/moto/tree/master/tests/test_{moto_name}/
```

If the user specified file filters, use those. Otherwise select the main test file (usually the largest `test_{moto_name}.py`).

---

## Step 1: Inventory existing winterbaume tests

Read the existing integration test file to identify:

1. **Already-ported test function names** — collect all `async fn test_*` names
2. **Implemented operations** — which SDK operations are exercised
3. **Helper functions** — `make_client()`, resource setup helpers, constants like `ACCOUNT_ID`
4. **Import patterns** — which SDK types are imported

```bash
grep 'async fn test_' crates/winterbaume-{service}/tests/integration_test.rs
```

Also read the handlers to know which operations are actually implemented:

```bash
grep -E '(handle_|=> self\.)' crates/winterbaume-{service}/src/handlers.rs
```

Build a set of **implemented operations** — only port tests for operations that winterbaume actually handles.

---

## Step 1b: Scan for handler fixes from E2E test sessions

Other agent sessions running terraform E2E tests (`write-e2e-tests` skill) discover and fix handler bugs, marking each fix with a `// FIX(terraform-e2e):` comment. These fixes reveal behaviors that existing tests didn't cover. Scan for them to ensure the ported test suite covers these fixes too.

### 1b-i. Find all fix markers in the service crate

```bash
grep -rn "FIX(terraform-e2e)" crates/winterbaume-{service}/src/
```

### 1b-ii. For each fix, check if moto tests already cover it

Cross-reference each fix with the moto test suite. If moto has a test exercising the same behaviour (e.g., testing that `ContentBasedDeduplication` is stored and returned for FIFO queues), prioritise porting that test.

### 1b-iii. If no moto test covers the fix, write a targeted test

When a `FIX(terraform-e2e)` marker addresses a behaviour that moto doesn't test (because it's terraform-specific, like response shape requirements), write a new test specifically for the fixed behaviour. Common patterns:

| Fix category | Test to add |
|---|---|
| **Missing response field** | Call Get/Describe, assert the field is present with the correct default value |
| **Missing default attribute** | Create a resource, call Get/Describe, assert defaults are populated |
| **Non-deterministic response** | Call Describe twice, assert the field value is identical |
| **New operation added** | Call the operation, verify success and that it persists |
| **Attribute parsing on create** | Create with the attribute set, Get and verify it matches input |
| **ARN normalization** | Call with variant ARN forms, verify correct behaviour |
| **Conditional response shape** | Create with alternate config, Describe and assert expected fields |

Mark these tests with a comment:

```rust
// Covers FIX(terraform-e2e): {brief description of the fix}
#[tokio::test]
async fn test_{fix_scenario}() { ... }
```

### 1b-iv. Append fix-coverage tests to the report

In the JOURNAL.md entry (Step 6), add a sub-section:

```markdown
- FIX(terraform-e2e) coverage: K tests added for handler fixes (list scenarios)
```

---

## Step 2: Read and analyse moto tests

Read the target moto test file(s). For each `def test_*` function, extract:

1. **Test name** — e.g., `test_create_cluster`
2. **Operations called** — e.g., `client.create_cluster(...)`, `client.describe_clusters(...)`
3. **Setup dependencies** — which other operations must succeed first
4. **Assertions** — response field checks, error expectations
5. **Skip conditions** — `@pytest.mark.parametrize`, `if settings.TEST_SERVER_MODE`, `SkipTest` → skip these

### Classification

For each moto test, classify:

- **SKIP**: Already ported (test name matches or equivalent logic exists in winterbaume tests)
- **SKIP**: Uses operations not implemented in winterbaume
- **SKIP**: Tests moto-specific internals (`moto_api`, `state_manager`, CloudFormation integration)
- **SKIP**: Tests server-mode-only features
- **PORT**: All operations used are implemented in winterbaume, and no equivalent test exists

### Ordering

Port tests in dependency order:
1. Basic CRUD (create, describe/get, list, delete)
2. Update/modify operations
3. Error cases (not-found, already-exists, validation)
4. Tag operations
5. Complex scenarios (multi-resource interactions)

---

## Step 3: Translate Python tests to Rust

For each test marked PORT, translate using these patterns:

### 3a. Test function skeleton

```rust
// Ported from moto: test_{moto_name}.py::test_function_name
#[tokio::test]
async fn test_function_name() {
    let client = make_client().await;
    // ... translated logic ...
}
```

### 3b. Python-to-Rust translation rules

| Python (boto3) | Rust (aws-sdk) |
|----------------|----------------|
| `client = boto3.client("svc", region_name="us-east-1")` | `let client = make_client().await;` (reuse existing helper) |
| `client.create_thing(Name="foo")` | `client.create_thing().name("foo").send().await.unwrap()` |
| `response["Thing"]["Name"]` | `response.thing().unwrap().name().unwrap()` or `response.thing().unwrap().name()` |
| `assert response["X"] == "Y"` | `assert_eq!(response.x(), Some("Y"))` or `assert_eq!(response.x(), "Y")` |
| `with pytest.raises(ClientError) as exc:` | `let err = client.op().send().await.unwrap_err();` |
| `exc.value.response["Error"]["Code"] == "NotFound"` | Check error variant: `assert!(format!("{:?}", err).contains("NotFound"))` or use `.into_service_error()` |
| `len(response["Items"])` | `response.items().len()` |
| `response["Items"][0]["Name"]` | `response.items()[0].name().unwrap()` |
| `assert ... is None` | `assert!(response.field().is_none())` |
| `datetime` comparisons | Generally skip exact timestamp assertions; assert `is_some()` instead |
| `uuid.UUID(response["Id"])` | `assert!(!response.id().unwrap().is_empty())` (verify non-empty) |
| `@pytest.mark.parametrize(...)` | Separate test functions, or a helper called with different args |
| `for item in items:` / list comprehension | `.iter()`, `.map()`, `.collect::<Vec<_>>()` |

### 3c. SDK type nuances

- **Required string fields**: `response.name()` returns `&str` — compare with `assert_eq!(r.name(), "expected")`
- **Optional string fields**: `response.name()` returns `Option<&str>` — compare with `assert_eq!(r.name(), Some("expected"))`
- **Enum fields**: Use `aws_sdk_xxx::types::EnumType::Variant` — e.g., `assert_eq!(status, &aws_sdk_ecs::types::ClusterStatus::Active)` . Tip: it's often easier to compare the string form, since winterbaume may return raw strings
- **Integer fields**: Usually `i32` or `i64`, not `Option` — `assert_eq!(r.count(), 5)`
- **Blob/Bytes fields**: `r.data().unwrap().as_ref()` to get `&[u8]`
- **Nested builders**: Use `.builder()...build().unwrap()` for complex input types

### 3d. Error assertions

For error path tests, the simplest approach:

```rust
let err = client
    .some_operation()
    .some_field("bad-input")
    .send()
    .await
    .unwrap_err();

// Check error type through display/debug format:
let err_str = format!("{:?}", err);
assert!(
    err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
    "Expected not-found error, got: {err_str}"
);
```

Or if the SDK provides typed errors:

```rust
let service_err = err.into_service_error();
assert!(service_err.is_resource_not_found_exception());
```

### 3e. Setup helpers

If multiple tests need the same resource setup (e.g., "create a table before testing put_item"), create or reuse helper functions:

```rust
async fn create_test_cluster(client: &aws_sdk_ecs::Client) -> String {
    let resp = client
        .create_cluster()
        .cluster_name("test-cluster")
        .send()
        .await
        .unwrap();
    resp.cluster().unwrap().cluster_arn().unwrap().to_string()
}
```

Always check if an equivalent helper already exists in the test file before adding a new one.

### 3f. What to skip in translation

- **Moto-internal assertions**: `state_manager`, `moto_api._internal`, mock reset
- **CloudFormation integration tests**: `test_*_cloudformation` — these test CFN stack behaviour
- **Server-mode tests**: `if settings.TEST_SERVER_MODE` / `@mock_aws(config={"core": {"mock_credentials": False}})`
- **Pagination tokens**: If winterbaume doesn't implement pagination, skip pagination-specific tests. If it does, port them.
- **Account/credential-specific behaviour**: `mock.patch.dict(os.environ, ...)` — simplify or skip
- **Exact timestamp assertions**: Replace with `is_some()` checks
- **File I/O tests**: Tests that read/write actual files — adapt to use in-memory data

---

## Step 4: Append tests to integration_test.rs

### 4a. Add section header

Group ported tests under a clear comment block:

```rust
// ============================================================================
// Ported from moto: test_{moto_name}.py
// ============================================================================
```

### 4b. Add necessary imports

Check if new imports are needed (new SDK types, etc.) and add them at the top of the file. Merge with existing imports — don't duplicate.

### 4c. Append test functions

Append the translated tests after the existing tests. Maintain the existing code style:
- Same indentation (4 spaces)
- Same assertion style (`assert_eq!` / `assert!`)
- Same error handling style (`.unwrap()` vs `.expect("message")`)
- Same comment style

### 4c-ii. Append FIX(terraform-e2e) coverage tests

If Step 1b found fix markers with no corresponding moto test coverage, append the targeted tests under a separate section header:

```rust
// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================
```

### 4d. Add helper functions

If new setup helpers are needed, add them near the existing helpers (after `make_client()`, before test functions), not at the bottom.

---

## Prior Art and Lessons Learned

This skill continues work documented in JOURNAL.md (2026-03-22). Key findings from porting 816 tests across 32 services:

### Behavioral fixes are the main value

Many "working" handlers had subtly wrong behaviour that smoke tests couldn't catch. When ported moto tests fail, **fix the winterbaume handler/state code to match moto's behaviour** rather than weakening assertions. Common categories:

1. **Wrong defaults** — ACM status ISSUED vs PENDING_VALIDATION; ECS container cpu default 0 not 256; OpenSearch engine 2.5 not 2.11
2. **Missing response fields** — Lambda missing Architectures/EphemeralStorage/TracingConfig; KMS missing EncryptionAlgorithms
3. **Idempotency semantics** — SES/SNS/StepFunctions/CodeBuild delete operations should be idempotent (no error on missing resource)
4. **Error code/message mismatches** — IAM EntityAlreadyExists message format; MemoryDB type-specific fault names
5. **Missing validation** — CodeBuild name regex/length; SES sender identity; Secrets Manager recovery window range
6. **Data computation** — Lambda CodeSha256 must be real SHA-256; SQS message MD5 must match

### Moto tests are the authoritative behavioural spec

They encode AWS-specific quirks (validation patterns, default values, idempotency) that can't be derived from the Smithy model. Prefer fixing winterbaume to match moto over adjusting assertions.

### Wire module serialisers

New handlers should use `wire::serialize_*_response()` from the generated wire module rather than hand-building JSON/XML. Check existing wire.rs for available serialisers and regenerate if needed.

---

## Step 5: Build, lint, and test

```bash
./.agents/bin/cargo.sh build -p winterbaume-{service}
./.agents/bin/cargo.sh clippy -p winterbaume-{service} --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh fmt -p winterbaume-{service} -- --check
./.agents/bin/cargo.sh test -p winterbaume-{service} -- --test-threads=1
```

Clippy and `fmt --check` are mandatory before reporting; if they fail, fix the violations and re-run. New test code is the most common source of `unnecessary_sort_by`, `collapsible_match`, and similar lints — running clippy here prevents those from reaching CI.

> **`--maxfail` is pytest, not cargo.** `--maxfail` is a pytest flag, not a Rust libtest flag. For Rust crate verification, use focused filters ( `cargo test <pattern>` ), `--test-threads=N`, `--no-fail-fast`, and per-crate gates instead. Do not pass `--maxfail` to cargo test.

If tests fail:

1. **Compilation errors** — Fix type mismatches, missing imports, wrong method names. Check the SDK docs by reading the generated model types in the aws-sdk crate.
2. **Assertion failures** — The winterbaume implementation may differ from moto's behaviour. Possible fixes:
   - Adjust the assertion to match winterbaume's current behaviour (if it's a valid alternative)
   - Fix the winterbaume handler/state to match the expected AWS behaviour
   - Add a `// TODO: winterbaume doesn't yet support X` comment and weaken the assertion
3. **501 Not Implemented** — The test uses an operation that's a stub. Remove the test or mark it with a `// TODO` and `#[ignore]`.

Run the tests iteratively — fix one test at a time:

```bash
cargo test -p winterbaume-{service} -- test_function_name
```

---

## Step 6: Report

After all tests pass, summarize:

1. **Tests ported**: Count and list of new test functions
2. **Tests skipped**: Count and brief reason for each skip category
3. **Issues found**: Any bugs discovered in winterbaume during porting (with or without fixes)
4. **Missing operations**: Operations that moto tests exercise but winterbaume hasn't implemented

### Append to JOURNAL.md

Add a brief entry to `.agents/docs/JOURNAL.md`:

```markdown
### Port moto tests: {service} (YYYY-MM-DD)

Ported N tests from moto's `tests/test_{moto_name}/test_{file}.py`.
- Tests added: N (list key scenarios)
- Tests skipped: M (operations not implemented / moto-internal / already covered)
- FIX(terraform-e2e) coverage: K tests added for handler fixes (list scenarios)
- Issues: (any bugs found and fixed)
```

---

## Reference: Service Name Mismatches

Common cases where the winterbaume crate name, moto test directory name, and SDK crate name don't trivially match:

| winterbaume crate | moto test dir | aws-sdk crate | Notes |
|-------------------|---------------|---------------|-------|
| `winterbaume-lambda` | `test_awslambda` | `aws-sdk-lambda` | moto uses `awslambda` to avoid Python keyword |
| `winterbaume-costexplorer` | `test_ce` | `aws-sdk-costexplorer` | moto uses `ce` |
| `winterbaume-cognitoidentityprovider` | `test_cognitoidp` | `aws-sdk-cognitoidentityprovider` | moto uses `cognitoidp` |
| `winterbaume-elasticloadbalancingv2` | `test_elbv2` | `aws-sdk-elasticloadbalancingv2` | moto uses `elbv2` |
| `winterbaume-eventbridge` | `test_events` | `aws-sdk-eventbridge` | moto uses `events` |
| `winterbaume-cloudwatchlogs` | `test_logs` | `aws-sdk-cloudwatchlogs` | moto uses `logs` |
| `winterbaume-config` | `test_config` | `aws-sdk-configservice` | SDK uses `configservice` |
| `winterbaume-directory` | `test_ds` | `aws-sdk-directoryservice` | moto uses `ds` |
| `winterbaume-route53` | `test_route53` | `aws-sdk-route53` | Matching |
| `winterbaume-secretsmanager` | `test_secretsmanager` | `aws-sdk-secretsmanager` | Matching |
