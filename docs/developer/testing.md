# Testing

Winterbaume uses three complementary test layers. Each layer catches different classes of bugs; together they give strong confidence that service behaviour matches real AWS.

## Test layers

| Layer | Location | What it catches |
|---|---|---|
| SDK integration tests | `crates/winterbaume-{svc}/tests/integration_test.rs` | API shape compatibility, state transitions, error responses |
| Moto parity ports | Same file | Behavioral correctness against a proven reference |
| Terraform E2E | `crates/winterbaume-e2e-tests/tests/terraform/` | Provider-level compatibility, waiters, sub-resource reads |

## SDK integration tests

Every service crate has an `integration_test.rs` that exercises the service through real `aws-sdk-rust` clients routed through a `MockAws` instance:

```rust
use aws_sdk_sqs as sqs;
use winterbaume_core::MockAws;
use winterbaume_sqs::SqsService;
use winterbaume_sts::StsService;

async fn build() -> (sqs::Client, MockAws) {
    let mock = MockAws::builder()
        .with_service(StsService::new())
        .with_service(SqsService::new())
        .build();
    let config = mock.sdk_config("us-east-1").await;
    (sqs::Client::new(&config), mock)
}

#[tokio::test]
async fn test_send_and_receive() {
    let (client, _mock) = build().await;

    let q = client.create_queue().queue_name("test").send().await.unwrap();
    let url = q.queue_url().unwrap();

    client.send_message()
        .queue_url(url)
        .message_body("hello")
        .send().await.unwrap();

    let recv = client.receive_message()
        .queue_url(url)
        .send().await.unwrap();

    assert_eq!(recv.messages().len(), 1);
    assert_eq!(recv.messages()[0].body().unwrap(), "hello");
}
```

### Running service tests

```sh
# All tests for one service
cargo test -p winterbaume-sqs

# A specific test by name
cargo test -p winterbaume-sqs test_send_and_receive

# Stop after first 5 failures
cargo test -p winterbaume-sqs -- --test-threads=1 2>&1 | head -100
```

Do not run the entire workspace test suite at once without a `--maxfail` guard — it takes a long time and produces too much output to read. Run per-service or per-feature.

## Moto parity ports

The strongest behavioural specs are the Python tests in [moto's test suite](https://github.com/getmoto/moto/tree/master/tests). Port these to Rust to verify that winterbaume produces the same behaviour as moto.

### Porting workflow

1. Read the moto test in `tests/test_{service}/test_{service}.py` from the [moto repository](https://github.com/getmoto/moto).
2. Translate each test function to a `#[tokio::test]` in `integration_test.rs`, using the same setup, inputs, and assertions.
3. Substitute boto3 calls with the equivalent `aws-sdk-rust` calls.

Example — original moto test:

```python
@mock_aws
def test_create_queue():
    sqs = boto3.client("sqs", region_name="us-east-1")
    sqs.create_queue(QueueName="test-queue")
    queues = sqs.list_queues()["QueueUrls"]
    assert len(queues) == 1
    assert "test-queue" in queues[0]
```

Ported to Rust:

```rust
#[tokio::test]
async fn test_create_queue() {
    let (client, _mock) = build().await;
    client.create_queue().queue_name("test-queue").send().await.unwrap();
    let resp = client.list_queues().send().await.unwrap();
    assert_eq!(resp.queue_urls().len(), 1);
    assert!(resp.queue_urls()[0].contains("test-queue"));
}
```

The `/port-moto-tests` skill automates this workflow for a given service.

## Terraform E2E tests

`crates/winterbaume-e2e-tests/tests/terraform/` contains per-service HCL configurations and a Rust test harness that:

1. Starts `winterbaume-server` in-process.
2. Runs `terraform init` + `terraform apply` against it.
3. Asserts on resource creation, read-backs, and provider waiter behaviour.
4. Runs `terraform destroy` to verify deletion paths.

These tests exercise patterns that SDK tests miss: provider-level `read` callbacks after `create`, computed attributes, waiter retry loops, and sub-resource ordering.

### Running E2E tests

```sh
# Run a single service suite
cargo test -p winterbaume-e2e-tests -- sqs

# Run all E2E tests (slow)
cargo test -p winterbaume-e2e-tests
```

Stale `.terraform.tfstate.lock.info` files will block re-runs. Always `rm -rf` the working directory before retrying a failed E2E run.

### `FIX(terraform-e2e)` convention

When a handler or state bug is discovered through an E2E test failure, the fix site is marked:

```rust
// FIX(terraform-e2e): ELBv2 DescribeLoadBalancers must return DNSName in response
// Provider's read callback always re-reads after create and asserts on this field.
```

A matching fast integration test should be added so the fix does not remain E2E-only. Grep for `FIX(terraform-e2e)` to find all current sites and verify they each have integration-test coverage.

## Coverage counting

The `/api-coverage` skill generates coverage reports by counting implemented operations, excluding handlers with a `// STUB[...]` comment. Coverage counts are a prioritisation signal, not an acceptance criterion — behavioural correctness and provider compatibility are what matter.

## Test organisation guidelines

- Each test function should cover one meaningful behaviour, not one API call.
- Tests that exercise error conditions are as valuable as happy-path tests.
- Use `#[tokio::test]` with no shared global state — construct a fresh `MockAws` per test.
- Long test suites should use helper functions like `build()` to keep setup DRY.
- When a bug is found and fixed, add a regression test that would have caught it.
