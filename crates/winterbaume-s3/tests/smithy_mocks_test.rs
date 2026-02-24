//! Integration tests for winterbaume + aws-smithy-mocks interop.
//!
//! These tests verify that aws_smithy_mocks rule overrides work alongside
//! winterbaume's stateful S3 backend.

use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::error::NoSuchKey;
use aws_smithy_mocks::{RuleMode, mock};
use winterbaume_core::MockAws;
use winterbaume_s3::S3Service;

/// Helper to create a MockAws with S3 backend.
fn make_mock() -> MockAws {
    MockAws::builder().with_service(S3Service::new()).build()
}

#[tokio::test]
async fn test_sdk_config_convenience() {
    let mock = make_mock();
    let config = mock.sdk_config("us-east-1").await;
    let s3 = aws_sdk_s3::Client::new(&config);

    s3.create_bucket()
        .bucket("sdk-config-bucket")
        .send()
        .await
        .expect("create_bucket via sdk_config should succeed");

    s3.head_bucket()
        .bucket("sdk-config-bucket")
        .send()
        .await
        .expect("head_bucket via sdk_config should succeed");
}

#[tokio::test]
async fn test_rule_override_intercepts_matched_operation() {
    let mock = make_mock();

    // Rule: GetObject for key "always-fail" returns NoSuchKey error
    let fail_rule = mock!(aws_sdk_s3::Client::get_object)
        .then_error(|| GetObjectError::NoSuchKey(NoSuchKey::builder().build()));

    let interceptor = mock.mock_interceptor(RuleMode::MatchAny, &[&fail_rule]);

    let s3_config = aws_sdk_s3::Config::builder()
        .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .interceptor(interceptor)
        .build();
    let s3 = aws_sdk_s3::Client::from_conf(s3_config);

    // Create a bucket (passthrough to winterbaume)
    s3.create_bucket()
        .bucket("rule-bucket")
        .send()
        .await
        .expect("create_bucket should passthrough to winterbaume");

    // GetObject should be intercepted by the rule
    let result = s3
        .get_object()
        .bucket("rule-bucket")
        .key("any-key")
        .send()
        .await;

    assert!(result.is_err(), "get_object should fail via rule override");
    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(
        matches!(service_err, GetObjectError::NoSuchKey(_)),
        "error should be NoSuchKey from rule"
    );
}

#[tokio::test]
async fn test_stateful_ops_with_rule_override() {
    let mock = make_mock();

    // Rule: only match GetObject for key "intercepted"
    let intercept_rule = mock!(aws_sdk_s3::Client::get_object)
        .match_requests(|req| req.key() == Some("intercepted"))
        .then_error(|| GetObjectError::NoSuchKey(NoSuchKey::builder().build()));

    let interceptor = mock.mock_interceptor(RuleMode::MatchAny, &[&intercept_rule]);

    let s3_config = aws_sdk_s3::Config::builder()
        .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .interceptor(interceptor)
        .build();
    let s3 = aws_sdk_s3::Client::from_conf(s3_config);

    // Create bucket and put an object via winterbaume
    s3.create_bucket()
        .bucket("mixed-bucket")
        .send()
        .await
        .unwrap();

    s3.put_object()
        .bucket("mixed-bucket")
        .key("real-object")
        .body(ByteStream::from_static(b"real data"))
        .send()
        .await
        .unwrap();

    // GetObject for "real-object" should passthrough to winterbaume and succeed
    let resp = s3
        .get_object()
        .bucket("mixed-bucket")
        .key("real-object")
        .send()
        .await
        .expect("get_object for real-object should passthrough to winterbaume");

    let body = resp.body.collect().await.unwrap().into_bytes();
    assert_eq!(body.as_ref(), b"real data");

    // GetObject for "intercepted" should be caught by the rule
    let result = s3
        .get_object()
        .bucket("mixed-bucket")
        .key("intercepted")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_object for 'intercepted' key should fail via rule"
    );
}

#[tokio::test]
async fn test_rule_num_calls() {
    let mock = make_mock();

    let counted_rule = mock!(aws_sdk_s3::Client::get_object)
        .match_requests(|req| req.key() == Some("counted"))
        .then_error(|| GetObjectError::NoSuchKey(NoSuchKey::builder().build()));

    let interceptor = mock.mock_interceptor(RuleMode::MatchAny, &[&counted_rule]);

    let s3_config = aws_sdk_s3::Config::builder()
        .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .interceptor(interceptor)
        .build();
    let s3 = aws_sdk_s3::Client::from_conf(s3_config);

    s3.create_bucket()
        .bucket("count-bucket")
        .send()
        .await
        .unwrap();

    // Call GetObject for "counted" three times
    for _ in 0..3 {
        let _ = s3
            .get_object()
            .bucket("count-bucket")
            .key("counted")
            .send()
            .await;
    }

    assert_eq!(
        counted_rule.num_calls(),
        3,
        "rule should have been called 3 times"
    );
}
