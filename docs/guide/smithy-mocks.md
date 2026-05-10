# Smithy Mocks Integration

Winterbaume can be combined with [`aws-smithy-mocks`](https://crates.io/crates/aws-smithy-mocks) to layer per-operation rule overrides on top of stateful backends. Winterbaume handles all state; rules intercept specific calls and return custom responses.

## Enabling the feature

Add the `smithy-mocks` feature to `winterbaume-core` in your `Cargo.toml`:

```toml
[dev-dependencies]
winterbaume-core = { version = "0.1", features = ["smithy-mocks"] }
aws-smithy-mocks = "0.x"
```

## How it works

`mock.mock_interceptor(rule_mode, rules)` creates an `aws_smithy_mocks::MockResponseInterceptor` with `allow_passthrough()` enabled. Rules are checked first; if no rule matches the request, it falls through to winterbaume's stateful handler.

## Example

```rust
use aws_smithy_mocks::{mock, RuleMode};
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::types::error::NoSuchKey;
use winterbaume_core::MockAws;
use winterbaume_s3::S3Service;

#[tokio::test]
async fn test_get_object_error_override() {
    let mock = MockAws::builder()
        .with_service(S3Service::new())
        .build();

    // This rule makes GetObject return NoSuchKey for a specific key
    let fail_rule = mock!(aws_sdk_s3::Client::get_object)
        .match_requests(|req| req.key() == Some("always-fail"))
        .then_error(|| {
            GetObjectError::NoSuchKey(NoSuchKey::builder().build())
        });

    let interceptor = mock.mock_interceptor(RuleMode::MatchAny, &[&fail_rule]);

    let s3_config = aws_sdk_s3::Config::builder()
        .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .interceptor(interceptor)
        .build();

    let s3 = aws_sdk_s3::Client::from_conf(s3_config);

    // Stateful operations work normally
    s3.create_bucket().bucket("test-bucket").send().await.unwrap();

    // The rule fires for "always-fail" — winterbaume state is never consulted
    let err = s3
        .get_object()
        .bucket("test-bucket")
        .key("always-fail")
        .send()
        .await
        .unwrap_err();

    assert!(matches!(
        err.into_service_error(),
        GetObjectError::NoSuchKey(_)
    ));
}
```

## Rule modes

`aws_smithy_mocks` supports two rule modes:

| Mode | Behavior |
|---|---|
| `RuleMode::MatchAny` | First matching rule wins. Unmatched requests pass through to winterbaume. |
| `RuleMode::Sequential` | Rules fire in order, one per call. Unmatched requests pass through. |

## Use cases

- Simulate transient errors (throttling, service unavailable) for specific operations
- Return error responses for edge cases that are hard to reproduce with real state
- Override a single operation's output while leaving all other operations stateful
