---
name: write-e2e-tests
description: Generate terraform E2E test suites for an AWS service, testing all terraform resource types winterbaume's handlers support via real `terraform apply` against an in-process server.
argument-hint: <service-name>
user_invocable: true
---

# Write Terraform E2E Tests

Generate a comprehensive terraform E2E test module for an AWS service under `crates/winterbaume-e2e-tests/tests/terraform/`. Each test runs `terraform apply` against an in-process winterbaume HTTP server to verify that real terraform workflows work.

Cargo test target name: `terraform` (runs as `cargo test --test terraform ...`).

## Arguments

- `$0` — Service name matching the winterbaume crate suffix (e.g., `s3`, `iam`, `sqs`, `dynamodb`, `lambda`)

---

## Principles (hard-won from real implementation)

### 1. The terraform provider is a black box — discover, don't assume

The terraform AWS provider makes many API calls beyond the obvious CRUD. You **cannot predict** what it will call from the AWS API contract alone. Examples from real implementation:

- **S3 `CreateBucket`** triggers 15+ sub-resource reads: `?versioning`, `?acl`, `?tagging`, `?encryption`, `?policy`, `?cors`, `?logging`, `?replication`, `?website`, `?object-lock`, `?accelerate`, `?request-payment`, `?ownershipControls`, `?publicAccessBlock`, `?policyStatus`
- **SQS `CreateQueue`** polls `GetQueueAttributes` until `SqsManagedSseEnabled` appears (25s hang without it). FIFO queues also need `ContentBasedDeduplication` to match the creation input.
- **SNS `CreateTopic`** immediately calls `SetTopicAttributes` and `GetTopicAttributes`. The `Policy` attribute must be valid JSON or terraform fails with "parsing policy: unexpected end of JSON input".
- **CloudWatch Logs `CreateLogGroup`** calls `ListTagsForResource` with an ARN that omits the `:*` suffix that winterbaume stores.
- **DynamoDB `CreateTable`** polls `DescribeTable` every 10s. If `TableId` changes between calls (e.g., random UUID), the waiter never settles.
- **StepFunctions `CreateStateMachine`** calls `ListStateMachineVersions` immediately after creation. Return `{"stateMachineVersions":[]}`.
- **Kinesis `CreateStream`** with encryption disabled: the provider stores `"NONE"` in state. During subsequent batch refreshes, the provider calls `StopStreamEncryption` with empty `EncryptionType`, which fails AWS SDK client-side validation before reaching the server. Use isolated servers for kinesis tests (see Principle 7).
- **CognitoIDP `CreateUserPool`**: the Go provider dereferences nested Go struct pointers (`Policies.PasswordPolicy`, `AdminCreateUserConfig`, `AccountRecoverySetting`). Return all nested objects with defaults or the provider panics with nil pointer dereference.
- **CognitoIDP `CreateUserPool`** calls `TagResource` when tags are specified (not `CreateUserPool` tag field). Return `{}` stub for `TagResource`, `UntagResource`, `ListTagsForResource`.
- **CognitoIDP `DescribeUserPoolClient`** must return all OAuth/auth fields that were passed in `CreateUserPoolClient` (`explicit_auth_flows`, `allowed_oauth_flows`, `allowed_oauth_scopes`, `callback_urls`, `logout_urls`, etc.) or the provider detects drift and fails with "Provider produced inconsistent result".
- **ECS `DescribeTaskDefinition` / `DeregisterTaskDefinition` / `DeleteTaskDefinitions`**: terraform calls all three with full ARNs (`arn:aws:ecs:...:task-definition/family:1`) but the common state key format is `family:revision`. All three operations need ARN prefix stripping — fixing only Describe is not sufficient. Omitting the fix on Deregister/Delete causes every subsequent batch wave to fail with "Unable to describe task definition: arn:..." cascade.
- **ECS `aws_ecs_service`**: the provider stores `networkConfiguration` in state. If `DescribeServices` does not echo back `networkConfiguration` on every read, terraform detects drift and calls `UpdateService` on subsequent batch waves, cascading failures. Use an isolated server for `aws_ecs_service` tests until the response round-trips correctly.
- **Route53 GetChange**: terraform calls `GET /2013-04-01/change/{id}` after every hosted zone and record set write. Missing handler → immediate 404 failure. Non-INSYNC status → infinite polling. Always return `INSYNC` immediately.
- **Route53 trailing dots**: `ListResourceRecordSets` must return record names WITH a trailing dot (e.g., `"www.example.com."`). The terraform provider normalizes state to trailing-dot form; returning without causes an "empty result" detection on the first refresh after create.
- **ACM certificate status**: `RequestCertificate` must return status `ISSUED` immediately. Returning `PENDING_VALIDATION` causes terraform to wait indefinitely for DNS or email validation that can never succeed in a mock.

**Rule**: Always smoke-test before writing any test. One smoke test reveals more than an hour of reading API docs.

### 2. Debug from terraform's side, not the server's

When a test hangs, the answer is **always** in terraform's trace. Server-side logging is nearly useless because:

- When the test deadlocks (tokio issue), no requests reach the server.
- When requests succeed (200), you can't see why terraform is unsatisfied.
- The provider's retry/polling/waiter logic is invisible from the server.

**Preferred debugging path** — use `smoke_test_terraform` from the harness:

```rust
let result = smoke_test_terraform(
    test_services(),
    r#"resource "aws_{resource_type}" "test" { ... }"#,
    std::time::Duration::from_secs(60),
).await;
eprintln!("{result}");
eprintln!("API calls: {:?}", result.api_calls);
eprintln!("Missing ops: {:?}", result.missing_operations);
eprintln!("Failed ops: {:?}", result.failed_operations);
```

This parses the TF_LOG=TRACE output and returns structured data without you touching any files.

**Manual debugging** (when smoke_test isn't enough):

Use `run_terraform_with_timeout` with a short timeout — on timeout it saves the trace automatically and returns a diagnostic summary in `stderr`:

```rust
let (ok, _stdout, stderr) = run_terraform_with_timeout(
    &dir,
    &["apply", "-auto-approve", "-no-color"],
    std::time::Duration::from_secs(60),
).await;
eprintln!("{stderr}");  // contains "API calls: XxxOp x7, ..." on timeout
```

If you need the raw trace file, find it under `.agents-workspace/tmp/tf-logs/`. Search for:
- `"request failed with unretryable error"` — unretryable failures
- `"produced an unexpected new value"` — shape mismatch between plan and apply
- `"retrying request"` — the provider is polling
- Lines starting with `  | ` — response bodies

### 3. Mark every handler fix

When you fix a handler to make terraform work, **always** add a `// FIX(terraform-e2e):` comment at the fix site explaining what was wrong and why. This lets other agent sessions:
- Discover fixes via `grep -r "FIX(terraform-e2e)"` across the codebase
- Verify the fix is correct
- Write proper unit tests covering the fixed behaviour

### 4. Response shape matters more than response content

The terraform provider checks response shape (field presence, types, JSON validity) more than values. A response that has all the right values but is missing a single expected field will cause an infinite polling loop. Key patterns:

| Service | Must-have in response |
|---|---|
| SQS | `SqsManagedSseEnabled` in GetQueueAttributes |
| SQS (FIFO) | `ContentBasedDeduplication`, `DeduplicationScope`, `FifoThroughputLimit` |
| DynamoDB | `ProvisionedThroughput` (even for PAY_PER_REQUEST with zeros), `DeletionProtectionEnabled`, stable `TableId` |
| SNS | `Policy` (valid JSON), `Owner`, `EffectiveDeliveryPolicy` in GetTopicAttributes |
| S3 | Proper 404 XML error codes for each unset sub-resource (e.g., `NoSuchTagSet`, `NoSuchBucketPolicy`) |
| StepFunctions | `stateMachineVersions: []` in `ListStateMachineVersions` |
| Kinesis | `encryption_type = "NONE"` always present in `DescribeStream`/`DescribeStreamSummary` (see Principle 7 for batch_apply caveat) |
| CognitoIDP | `UserPoolType` nested objects: `Policies` (with `PasswordPolicy`), `AdminCreateUserConfig`, `AccountRecoverySetting` must be present with non-null values |
| CognitoIDP | `UserPoolClient` must echo back all fields passed on create (`explicit_auth_flows`, `allowed_oauth_flows`, `allowed_oauth_scopes`, `callback_urls`, `logout_urls`, `refresh_token_validity`, `supported_identity_providers`) |
| Route53 | Record names in `ListResourceRecordSets` must include trailing dot (e.g., `"www.example.com."`) — terraform normalizes to trailing-dot form in state and detects drift if the API returns without |
| Route53 | Must implement `GET /2013-04-01/change/{id}` returning `INSYNC` immediately — terraform polls this after every zone and record change |
| ACM | `RequestCertificate` must return status `ISSUED` immediately (not `PENDING_VALIDATION`) — terraform waits for ISSUED and hangs forever if the mock returns pending |
| ECS | Task definition ARN normalization: `DescribeTaskDefinition`, `DeregisterTaskDefinition`, and `DeleteTaskDefinitions` must all strip the `arn:aws:ecs:...:task-definition/` prefix before looking up the `family:revision` key in state |

### 5. Non-deterministic responses cause waiter loops

If a response field changes between calls (like a random UUID for `TableId`), the provider's state waiter may treat it as "still transitioning" and poll forever. All ID/ARN fields in describe/get responses must be deterministic.

### 6. Computed fields returned by the server must round-trip through the provider's state

When the provider stores a computed field in state, it stores the value returned by the API (possibly normalized). On every subsequent `batch_apply` wave, terraform refreshes all existing resources. If the refreshed value differs from what is stored in state, terraform detects drift and tries to "fix" it — which can involve operations that fail.

**The normalization trap**: some values are normalized in state but not in the API response. Example:
- Kinesis `encryption_type`: API returns `"NONE"`, provider normalizes to `"NONE"` in state. Returning `""` on refresh causes drift `"NONE" vs ""` → `StopStreamEncryption`.
- CognitoIDP `mfa_configuration`: returning `"OFF"` when the user didn't set it causes drift `null vs "OFF"` → provider fails with "inconsistent result after apply".

**Rule**: return exactly what the provider will store in state. If you're not sure, run the test once, check `terraform.tfstate` to see what was stored, and make sure DescribeX returns a value that maps to the same stored value on subsequent calls.

### 7. Services with SDK-validated client-side operations must use isolated servers

Some terraform providers call AWS SDK operations where the SDK itself validates required fields BEFORE sending the request. If the provider calls such an operation with missing required fields (a provider bug), our server never receives the request, and no server-side fix is possible.

**Known case — Kinesis**: the terraform AWS provider calls `StopStreamEncryption` during certain update paths (any stream modification), passing an empty `EncryptionType`. The Go AWS SDK v2 validates `EncryptionType` as required, so the call fails with "missing required field, StopStreamEncryptionInput.EncryptionType" before reaching the server.

**When this breaks batch_apply**: kinesis streams created in batch wave 1 sit in the shared terraform state. When batch wave 2 runs (for any other service), terraform refreshes the kinesis streams. If the provider's refresh path triggers the problematic operation on the EXISTING streams, the entire batch 2 apply fails — cascading across all services in that wave.

**Fix**: convert tests for such services to isolated servers. Each test starts its own fresh server with empty state, eliminating cross-batch contamination:

```rust
// Use this pattern instead of batch_apply for services with SDK-validated update paths
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kinesis_stream_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("kinesis-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(dir.join("main.tf"), r#"..."#).unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    // ... assertions ...
    cleanup_tf_dir(&dir);
}
```

Add a comment explaining WHY isolated servers are needed so future agents don't revert to batch_apply.

**How to diagnose**: the symptom is that `test_kms_key_basic` (or any unrelated service) reports "Error: stopping Kinesis Stream X encryption..." even though you're testing KMS. The cascade means batch wave N failed because batch wave N-1 left kinesis streams in the shared state.

### 8. Never enable TF_LOG=TRACE for normal apply calls

`TF_LOG=TRACE` generates 50+ MB of output per test run. When 40+ tests run in parallel, this overwhelms disk I/O and CPU, hanging the machine.

The harness has three terraform runners with different trace behaviors:

| Function | Trace enabled | When to use |
|---|---|---|
| `terraform_apply` / `run_terraform` | **No** | All normal E2E tests |
| `run_terraform_with_timeout` | **Yes** (kept on timeout, deleted on success) | Tests for services with known polling waiters |
| `smoke_test_terraform` | **Yes** (parsed, deleted on success) | Debugging: discovers what API calls the provider makes |

Never call `run_terraform_raw` directly with `enable_trace=true` in a regular test — use `run_terraform_with_timeout` instead.

### 9. Multiple tests in a batch wave must not contend on the same singleton API resource

`batch_apply` collects HCL from every concurrent test into a single `main.tf` and runs ONE `terraform apply`. Two resources in the same wave that target the same underlying account/global identity collide — even if their HCL local names differ.

Concrete cases to watch for:

- **`aws_inspector2_enabler`** is keyed by `account_id`. Two enablers for the same account with different `resource_types` cannot coexist: after apply, the API reports the union of enabled types, each enabler then sees drift and triggers Update→Disable→`BatchGetAccountStatus` polling, which deadlines out at 5 min and poisons the entire wave (every other test in the same wave fails too — iam, iot, etc.).
- **`aws_iam_account_password_policy`**, **`aws_organizations_organization`**, **`aws_inspector2_organization_configuration`**, **`aws_securityhub_account`** — all account-singleton resources.
- **Anything that takes a literal account_id as input** — when several tests reuse `"123456789012"`, they share state in the in-process server.

Fixes (in order of preference):

1. **Use unique account_ids per test**: `"111111111111"`, `"222222222222"`, … for tests that take an explicit `account_ids = [...]`.
2. **Move the service's tests to isolated servers** (Principle 7 pattern) so each test gets a fresh state.
3. **Collapse to one test** if the resource is a true singleton with no per-test variation.

How to spot this before you ship: when `cargo test -- --ignored --test-threads=4` reports failures in a service whose code you didn't touch (e.g. you added inspector2 tests and now iam/iot tests fail), the new service is the cascading culprit — see Step 4d.

---

## Step 0: Verify prerequisites

### 0a. Confirm the crate exists

```bash
ls crates/winterbaume-{service}/src/handlers.rs
```

### 0b. Identify implemented operations

```bash
grep -E '"[A-Z][a-zA-Z]+"' crates/winterbaume-{service}/src/handlers.rs | head -40
```

Only test operations that are actually dispatched (not unmatched actions).

### 0b.1. Identify the service's registered TF converters

The terraform converters are spec-driven. The authoritative list of TF resource types the service handles lives in `crates/winterbaume-terraform/specs/<service>.toml`:

```bash
grep -E "^type = " crates/winterbaume-terraform/specs/<service>.toml | sort -u
```

Each `[[resource]]` block's `type = "aws_..."` is a candidate for E2E coverage. The corresponding hand-written converter at `crates/winterbaume-terraform/src/converters/<service>.rs` orchestrates inject/extract — open it to confirm the resource is genuinely wired, not just spec'd. Each converter must also be registered in `crates/winterbaume-server/src/main.rs` for the in-process server to route Terraform requests through it.

If a TF resource is not in the spec, the converter doesn't exist and the E2E test will fail with the in-process server returning empty state. Add the converter first via the `terraform-converter` skill, then write the E2E test.

The per-service resource-type coverage report ( `.agents/docs/TERRAFORM_RESOURCE_COVERAGE.md`, regenerated via `python3 .agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py` ) shows exactly which resources are handled vs missing.

### 0c. Identify the terraform provider endpoint name

The terraform AWS provider endpoint names **do not always match** winterbaume crate names. Use the wrong name and requests silently go to the real AWS (or nowhere).

Known mappings:

| winterbaume crate | Terraform endpoint | Gotcha |
|---|---|---|
| winterbaume-s3 | `s3` | |
| winterbaume-iam | `iam` | |
| winterbaume-sqs | `sqs` | |
| winterbaume-sts | `sts` | |
| winterbaume-dynamodb | `dynamodb` | |
| winterbaume-kms | `kms` | |
| winterbaume-sns | `sns` | |
| winterbaume-lambda | `lambda` | |
| winterbaume-secretsmanager | `secretsmanager` | |
| winterbaume-logs | `cloudwatchlogs` | **NOT `logs`** |
| winterbaume-ssm | `ssm` | |
| winterbaume-ecr | `ecr` | |
| winterbaume-events | `eventbridge` | **NOT `events`** |
| winterbaume-cloudwatch | `cloudwatch` | |
| winterbaume-eks | `eks` | |
| winterbaume-ecs | `ecs` | |
| winterbaume-organizations | `organizations` | |
| winterbaume-cognitoidp | `cognitoidp` | |
| winterbaume-route53 | `route53` | |
| winterbaume-acm | `acm` | |
| winterbaume-kinesis | `kinesis` | |
| winterbaume-cloudfront | `cloudfront` | |
| winterbaume-efs | `efs` | |
| winterbaume-appfabric | `appfabric` | |
| winterbaume-appflow | `appflow` | |
| winterbaume-inspector2 | `inspector2` | |
| winterbaume-iot | `iot` | |

If not in this table, look it up in the provider's reference list:

```bash
gh api 'repos/hashicorp/terraform-provider-aws/contents/website/docs/guides/custom-service-endpoints.html.markdown' --jq '.content' | base64 -d | grep -i "<your-service>"
```

The endpoint name to use in `write_provider_tf` is the second column (the lower-case identifier). Forgetting this is a silent failure: requests bypass the in-process server and go to real AWS, which rejects them with `UnrecognizedClientException`.

### 0d. Identify terraform resource types

The canonical list is the spec file ( see Step 0b.1 ). Sub-resource categories:

| Category | Example | What it tests |
|---|---|---|
| Primary resource | `aws_dynamodb_table` | Full CRUD lifecycle |
| Sub-resource | `aws_s3_bucket_versioning` | Separate PUT/GET/DELETE on sub-resource API |
| Attachment | `aws_iam_role_policy_attachment` | Relationship between two existing resources |
| Inline | `aws_iam_role_policy` | Child resource embedded in parent |
| Policy | `aws_sqs_queue_policy` | SetAttributes with Policy field |
| Exclusive | `aws_iam_role_policies_exclusive` | Replaces the full set of inline/attached policies on the entity |
| Multi-target | `aws_iam_policy_attachment` | Single TF resource attaching one policy to multiple entities; extract intentionally returns empty |

### 0d.1. Verify each `aws_*` resource type actually exists in `hashicorp/aws`

A winterbaume terraform converter being registered for `aws_foo_bar` does **not** prove `aws_foo_bar` exists in `hashicorp/terraform-provider-aws`. Some AWS services were never given a terraform resource (e.g. discontinued services like `applicationcostprofiler`), and writing tests that reference a non-existent resource type fails with `Error: Invalid resource type` — which then poisons the entire batch wave for unrelated tests.

For every `aws_*` resource type you plan to use, confirm it exists in the provider before writing the test:

```bash
gh api 'search/code?q=aws_<resource_type>+repo:hashicorp/terraform-provider-aws' --jq '.total_count'
```

A non-zero count means the string appears in the provider source. Zero hits means the resource does not exist — do not write a test for it. Instead, leave a `// TODO: aws_<resource_type> is not in hashicorp/aws (as of YYYY-MM-DD)` comment in the test module and skip it.

If `gh` is not available, fetch `https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/<resource_type_without_aws_prefix>` — a 404 confirms the resource does not exist.

### 0e. Register the service in the harness

Check `crates/winterbaume-e2e-tests/tests/terraform/harness.rs`:

1. **`test_services()`** — add `Arc::new(winterbaume_{service}::{Service}Service::new())`
2. **`write_provider_tf()`** — add `{endpoint} = "{server_url}"` to the endpoints block
3. **Root `Cargo.toml` `[dev-dependencies]`** — add `winterbaume-{service} = { workspace = true }`

---

## Step 1: Smoke test against the in-process server

### 1a. Write a minimal test and run it

Write a single test function with the simplest possible terraform resource. **Run it immediately** — do not write the full suite first.

```bash
cargo test --test terraform {service}::test_{service}_{resource}_basic -- --ignored --nocapture
```

### 1b. If it passes on the first try

Congratulations — this is rare. Proceed to Step 2 and write the full suite.

### 1c. If it hangs (no output for 60+ seconds)

The provider is stuck on an API call. Debug with `smoke_test_terraform`:

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn debug_{service}_smoke() {
    let result = smoke_test_terraform(
        test_services(),
        r#"resource "aws_{resource}" "test" { name = "test" }"#,
        std::time::Duration::from_secs(60),
    ).await;
    eprintln!("{result}");
    for call in &result.api_calls {
        eprintln!("  {} -> {}", call.operation, call.status_code);
    }
    assert!(result.success);
}
```

Run it and read the output. It tells you:
- Which API operations the provider called and how many times each
- Which operations failed with unretryable errors
- Which operations were not found in the dispatch table

**Common causes and fixes**:

| Symptom | What to look for | Fix |
|---|---|---|
| Provider polls Describe/Get forever | Same operation called 10+ times, all 200 | A response field is missing, changes between calls, or has wrong format |
| Provider polls forever after Create | DescribeX called repeatedly but retries continue | A response field is non-deterministic (e.g., random UUID). Make it stable. |
| 25s hang then succeeds | GetXxxAttributes called 7-8 times then finally accepted | An optional attribute the provider checks for is missing. Add it. |
| Instant failure after Create | Failed op in `result.failed_operations` | The provider calls a follow-up operation that doesn't exist. Implement it. |
| "parsing policy: unexpected end of JSON input" | Response contains empty or missing Policy field | Add a valid default JSON policy on resource creation. |

### 1d. If it fails with an error

Read the error message. Common errors:

- `"Could not find operation X for Y"` → Operation X not in the dispatch table. Add it.
- `"parsing policy: unexpected end of JSON input"` → A JSON attribute is empty. Add a valid default.
- `"unexpected new value: .field was null, but now cty.StringVal("")"` → Handler returns `""` where terraform expected `null`. Remove or match the field.

### 1e. Fix and re-test iteratively

For every fix:
1. Add `// FIX(terraform-e2e):` comment at the fix site
2. Rebuild: `cargo test --test terraform --no-run`
3. Re-run: `cargo test --test terraform {service}::test_{service}_{resource}_basic -- --ignored --nocapture`
4. Repeat until the basic test passes

---

## Step 2: Write the full test module

### 2a. Create the module file

Create `crates/winterbaume-e2e-tests/tests/terraform/{service}.rs`:

```rust
use crate::harness::*;
```

### 2b. Test function template

**Default: use `batch_apply`** — collects HCL from all concurrent tests into a single `terraform apply`, dramatically reducing test run time.

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_{service}_{what}() {
    let result = batch_apply(r#"
resource "aws_{resource_type}" "{service}_{what}" {
  // ... attributes with unique content values ...
}
"#).await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("unique-content-value"));
}
```

`batch_apply` returns `BatchResult { success, stdout, stderr, state }` where `state` is the full `terraform.tfstate` JSON. Tests assert on unique content values in the state (bucket names, parameter paths, etc.).

**When to use isolated apply instead** (own server + dir):
- Tests that run `terraform apply` **twice** (modify-in-place, idempotency checks)
- Tests that use `terraform plan` after apply
- Tests for services where the provider calls SDK-validated operations with missing fields during any update/refresh path (e.g., Kinesis `StopStreamEncryption` — see Principle 7)

For those, use the traditional pattern:
```rust
async fn test_{service}_{what}_modify() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("{service}-{what}");
    write_provider_tf(&dir, &url);
    // ... write main.tf, terraform_init, terraform_apply x2, cleanup ...
}
```

### 2c. Test design per resource type

Design tests based on what the service supports. Coverage goals:

| Test pattern | When to use | Example |
|---|---|---|
| `test_{svc}_{resource}_basic` | Every primary resource | `test_kms_key_basic` |
| `test_{svc}_{resource}_with_tags` | If service supports tags | `test_sns_topic_with_tags` |
| `test_{svc}_{resource}_with_{option}` | For each important optional config | `test_ecr_repository_with_settings` |
| `test_{svc}_{sub_resource}` | Each sub-resource type | `test_s3_bucket_versioning` |
| `test_{svc}_{resource}_policy` | Policy sub-resources | `test_ecr_repository_policy` |
| `test_{svc}_{resource}_modify_in_place` | If update is supported | `test_ssm_parameter_modify_in_place` |
| `test_{svc}_full_stack` | Combine all resource types | `test_iam_full_stack` |

### 2d. Critical rules

1. **`#[tokio::test(flavor = "multi_thread", worker_threads = 2)]`** — Always. Single-threaded tokio deadlocks when `tokio::process::Command` awaits while the in-process server needs to handle requests.

2. **`#[ignore]`** — Always. Tests need `terraform` CLI and network (for first `init`).

3. **`create_tf_dir` names must be globally unique** — Tests run in parallel. Use `"{service}-{descriptive-slug}"`. Never reuse across modules.

4. **Assert on both stdout and tfstate** — Apply may report success but write wrong state.

5. **Use `jsonencode()` in HCL for JSON attributes** — Avoids formatting drift.

6. **Multi-line HCL blocks** — Single-line `attribute { name = "x"; type = "S" }` fails in terraform. Always use multi-line.

7. **Never test `terraform destroy` for services with polling delete waiters** — SQS destroy takes 60+ seconds. If a service hangs on destroy, skip it.

8. **Remove tests for unimplemented operations** — Don't leave failing tests with `#[ignore = "reason"]` because `cargo test -- --ignored` runs them too. Delete the function entirely and leave a `// TODO:` comment instead.

9. **Use `terraform_apply` (not `run_terraform_with_timeout`) for normal tests** — `terraform_apply` does not enable TF_LOG=TRACE, which keeps runs fast and avoids overwhelming disk I/O when tests run in parallel. Only use `run_terraform_with_timeout` when the test specifically verifies timeout/waiter behaviour.

### 2e. Sub-resource pattern

```hcl
resource "aws_{service}_{parent}" "test" {
  name = "test-parent"
}

resource "aws_{service}_{parent}_{sub_resource}" "test" {
  {parent}_id = aws_{service}_{parent}.test.id
  // ... sub-resource config ...
}
```

### 2f. Full-stack test

The last test should combine multiple resources with cross-references to verify the whole service works together:

```rust
async fn test_{service}_full_stack() {
    // Create N resources with dependencies
    // Assert all N created
    // Verify tfstate has all expected resources
}
```

---

## Step 3: Register the module

1. Add `mod {service};` to `crates/winterbaume-e2e-tests/tests/terraform/main.rs`
2. Update the doc comment:
   ```rust
   //!   cargo test --test terraform {service} -- --ignored
   ```
3. Add `winterbaume-{service} = { workspace = true }` to root `Cargo.toml` `[dev-dependencies]` if not already present
4. Add the service to `test_services()` in `harness.rs` if not already present
5. Add the endpoint to `write_provider_tf()` in `harness.rs` if not already present

---

## Step 4: Build, lint, and run

### 4a. Build and lint

```bash
./.agents/bin/cargo.sh test --test terraform --no-run
./.agents/bin/cargo.sh clippy -p winterbaume-e2e-tests --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh fmt -p winterbaume-e2e-tests -- --check
```

The clippy and fmt gates are mandatory before reporting; if they fail, fix the violations and re-run. New harness/test code is the most common source of `unnecessary_sort_by`, `collapsible_match`, and similar lints — running clippy here prevents those from reaching CI.

### 4b. Run the simplest test first

```bash
cargo test --test terraform {service}::test_{service}_{resource}_basic -- --ignored --nocapture
```

If this hangs, go back to Step 1c. Do not write more tests until the basic one passes.

### 4c. Run the full service suite

```bash
cargo test --test terraform {service} -- --ignored --nocapture
```

### 4d. Run the full E2E suite for regressions

**Mandatory — this is the gate that catches batch-wave cross-contamination.** A new service's tests can poison unrelated services in the same wave (Principle 9). The per-service run in 4c does not catch this; only the full suite does.

```bash
cargo test --test terraform -- --ignored --test-threads=4
```

The full suite takes ~3-4 minutes at 4 threads for ~84 tests; longer as the suite grows. Adjust the thread count based on available cores and observed CPU/disk load.

**Acceptance criterion**: the final line must read

```
test result: ok. N passed; 0 failed; 0 ignored; ...
```

Anything else — even one failure, even in a service you did not touch — means you are not done. Common cascades:

| Failure pattern | Cause | Fix |
|---|---|---|
| `Error: Invalid resource type` in MULTIPLE service stdouts that all reference one resource type | One test references an `aws_*` resource that doesn't exist in `hashicorp/aws`. The bad HCL poisons the whole wave. | Re-run Step 0d.1; delete the offending test. |
| `Error: stopping Kinesis Stream encryption` in tests for unrelated services | Kinesis tests left streams in shared state; provider triggers SDK-validated `StopStreamEncryption` on refresh in later waves. | Convert kinesis tests to isolated servers (Principle 7). |
| `timeout while waiting for resource to be gone` followed by 5+ minutes of dead time | Multiple tests in the same wave manage the same singleton API resource (account-scoped, etc.). | Apply Principle 9 (unique account_ids or isolated servers). |
| `Error acquiring the state lock` in every wave after the first failing one | A previous wave's terraform was SIGKILLed and left a stale lock. Should be auto-cleaned by the harness — if not, the harness has regressed. | Investigate `run_terraform_raw` lock-cleanup logic. |

**If you see phantom compile errors** ("missing field X" when the field is clearly present in source), a concurrent agent may have modified the crate mid-compilation, leaving stale incremental artifacts. Run `cargo clean -p {crate}` and rebuild.

---

## Step 5: Report

### 5a. Append to JOURNAL.md

```markdown
## YYYY-MM-DD: Terraform E2E Tests — {ServiceName}

### Test coverage

| Test | Terraform resources | Duration |
|---|---|---|
| `test_{service}_{resource}_basic` | `aws_{resource}` | ~Ns |
| ... | ... | ... |

### Handler fixes applied

| File | Fix | Comment marker |
|---|---|---|
| `crates/winterbaume-{service}/src/handlers.rs` | Added XxxOperation handler | `FIX(terraform-e2e)` |
| `crates/winterbaume-{service}/src/state.rs` | Added default Policy attribute | `FIX(terraform-e2e)` |
| (or "None — all tests passed without handler changes") | | |

### Resource types NOT tested

- `aws_{resource_type}` — reason (e.g., "requires GetSubscriptionAttributes not implemented")
```

### 5b. Report to the user

1. Number of tests written and passing
2. Terraform resource types covered
3. Handler fixes applied (with `FIX(terraform-e2e)` markers)
4. Resource types skipped and why
5. Any TF resource types found in `crates/winterbaume-terraform/specs/<service>.toml` that didn't make it into the test module (with a brief reason — non-trivial deps, no handler-side support, etc.) — these become candidate follow-up work

### 5c. Pre-report self-check (do not skip)

Before declaring done, answer all of these honestly. If you cannot say "yes" to every line, you are not done:

- [ ] I ran `cargo test --test terraform -- --ignored --test-threads=4` end-to-end (not just `--no-run`, not just per-service).
- [ ] The final line said `0 failed; 0 ignored`. (Filtered runs with `0 ignored` because of a name-pattern do not count — the suite-wide run is the gate.)
- [ ] I verified each `aws_*` resource type my tests reference exists in `hashicorp/terraform-provider-aws` (Step 0d.1).
- [ ] If my service's resources are account-scoped or otherwise singleton, each test uses a unique account_id OR the suite uses isolated servers (Principle 9).
- [ ] CI's `E2E (Terraform)` job will run the same `--ignored` suite I just ran — there is no way for a test to be "skipped on CI but counted as passing locally". If I `#[ignore = "..."]` a test, I deleted it instead and left a `// TODO:` comment.

A passing per-service run (Step 4c) is **not** sufficient evidence that the suite is healthy — batch-wave cross-contamination only appears in the full suite (Principle 9 / Step 4d).
