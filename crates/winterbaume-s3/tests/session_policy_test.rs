//! Opt-in STS session policy enforcement
//! ( `S3Service::with_session_policy_enforcement`, issue #85 ).
//!
//! Requests signed with an access key the lookup knows are limited to what
//! its session policy allows; every other request is unaffected.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use aws_credential_types::Credentials;
use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::error::{ProvideErrorMetadata, SdkError};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{Delete, ObjectIdentifier};
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use bytes::Bytes;
use winterbaume_core::{MockAws, MockRequest, MockService};
use winterbaume_s3::{S3Service, SessionPolicyLookup};

const SCOPED_KEY: &str = "ASIASCOPEDEXAMPLE";
const DENYING_KEY: &str = "ASIADENYINGEXAMPLE";
const BROKEN_KEY: &str = "ASIABROKENEXAMPLE";
const ADMIN_KEY: &str = "AKIAADMINEXAMPLE";

/// Lakekeeper's shape: read / write / delete under one prefix, listing only
/// that prefix, plus multipart housekeeping.
const SCOPED_POLICY: &str = r#"{
    "Version": "2012-10-17",
    "Statement": [
        {"Effect": "Allow",
         "Action": ["s3:GetObject", "s3:GetObjectVersion", "s3:PutObject", "s3:DeleteObject",
                    "s3:AbortMultipartUpload", "s3:ListMultipartUploadParts"],
         "Resource": ["arn:aws:s3:::bkt/allowed/*"]},
        {"Effect": "Allow", "Action": "s3:ListBucket", "Resource": "arn:aws:s3:::bkt",
         "Condition": {"StringLike": {"s3:prefix": ["allowed/*"]}}},
        {"Effect": "Allow", "Action": "s3:GetBucketLocation", "Resource": "arn:aws:s3:::bkt"}
    ]
}"#;

/// Everything, except writes under `allowed/locked/`.
const DENYING_POLICY: &str = r#"{
    "Version": "2012-10-17",
    "Statement": [
        {"Effect": "Allow", "Action": "s3:*", "Resource": "*"},
        {"Effect": "Deny", "Action": "s3:PutObject", "Resource": "arn:aws:s3:::bkt/allowed/locked/*"}
    ]
}"#;

struct FixedPolicies(HashMap<&'static str, &'static str>);

impl SessionPolicyLookup for FixedPolicies {
    fn session_policy<'a>(
        &'a self,
        access_key_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move { self.0.get(access_key_id).map(|p| p.to_string()) })
    }
}

fn lookup() -> Arc<dyn SessionPolicyLookup> {
    Arc::new(FixedPolicies(HashMap::from([
        (SCOPED_KEY, SCOPED_POLICY),
        (DENYING_KEY, DENYING_POLICY),
        (BROKEN_KEY, "{ not a policy"),
    ])))
}

fn enforcing_service() -> Arc<S3Service> {
    Arc::new(S3Service::new().with_session_policy_enforcement(lookup()))
}

async fn client(service: &Arc<S3Service>, access_key: &str) -> aws_sdk_s3::Client {
    let mock = MockAws::builder().with_service(Arc::clone(service)).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(Credentials::new(
            access_key,
            "secret",
            Some("token".to_string()),
            None,
            "test",
        ))
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_s3::Client::new(&config)
}

/// A bucket with one object inside and one outside the allowed prefix,
/// created with credentials no session policy limits.
async fn setup(service: &Arc<S3Service>) -> aws_sdk_s3::Client {
    let admin = client(service, ADMIN_KEY).await;
    admin.create_bucket().bucket("bkt").send().await.unwrap();
    for key in ["allowed/a", "outside/b"] {
        put(&admin, key).await.unwrap();
    }
    admin
}

async fn put(
    client: &aws_sdk_s3::Client,
    key: &str,
) -> Result<
    aws_sdk_s3::operation::put_object::PutObjectOutput,
    SdkError<aws_sdk_s3::operation::put_object::PutObjectError, HttpResponse>,
> {
    client
        .put_object()
        .bucket("bkt")
        .key(key)
        .body(ByteStream::from_static(b"data"))
        .send()
        .await
}

fn assert_access_denied<E>(err: &SdkError<E, HttpResponse>, reason: &str)
where
    E: ProvideErrorMetadata + std::fmt::Debug,
{
    let status = err.raw_response().map(|r| r.status().as_u16());
    assert_eq!(status, Some(403), "expected HTTP 403, got {err:?}");
    assert_eq!(err.code(), Some("AccessDenied"), "got {err:?}");
    let message = err.message().unwrap_or_default();
    assert!(
        message.contains(reason),
        "message {message:?} lacks {reason:?}"
    );
}

/// The issue's reproduction: inside the prefix succeeds, outside is refused.
#[tokio::test]
async fn writes_outside_the_session_policy_are_denied() {
    let service = enforcing_service();
    setup(&service).await;
    let scoped = client(&service, SCOPED_KEY).await;

    put(&scoped, "allowed/new").await.unwrap();
    let err = put(&scoped, "outside/new").await.unwrap_err();
    assert_access_denied(
        &err,
        "because no session policy allows the s3:PutObject action",
    );
    let message = err.message().unwrap_or_default().to_string();
    assert!(
        message.contains("arn:aws:s3:::bkt/outside/new"),
        "{message}"
    );
}

#[tokio::test]
async fn reads_deletes_and_versions_follow_the_policy() {
    let service = enforcing_service();
    setup(&service).await;
    let scoped = client(&service, SCOPED_KEY).await;

    scoped
        .get_object()
        .bucket("bkt")
        .key("allowed/a")
        .send()
        .await
        .unwrap();
    let err = scoped
        .get_object()
        .bucket("bkt")
        .key("outside/b")
        .send()
        .await
        .unwrap_err();
    assert_access_denied(&err, "s3:GetObject");

    // HEAD maps to s3:GetObject too, and a denied HEAD has no body.
    scoped
        .head_object()
        .bucket("bkt")
        .key("allowed/a")
        .send()
        .await
        .unwrap();
    let err = scoped
        .head_object()
        .bucket("bkt")
        .key("outside/b")
        .send()
        .await
        .unwrap_err();
    let response = err.raw_response().expect("an HTTP response");
    assert_eq!(response.status().as_u16(), 403);
    assert_eq!(response.body().bytes(), Some(&b""[..]));

    scoped
        .delete_object()
        .bucket("bkt")
        .key("allowed/a")
        .send()
        .await
        .unwrap();
    let err = scoped
        .delete_object()
        .bucket("bkt")
        .key("outside/b")
        .send()
        .await
        .unwrap_err();
    assert_access_denied(&err, "s3:DeleteObject");
}

/// `s3:ListBucket` with an `s3:prefix` condition, as Lakekeeper uses it.
#[tokio::test]
async fn listing_is_limited_to_the_allowed_prefix() {
    let service = enforcing_service();
    setup(&service).await;
    let scoped = client(&service, SCOPED_KEY).await;

    let listed = scoped
        .list_objects_v2()
        .bucket("bkt")
        .prefix("allowed/")
        .send()
        .await
        .unwrap();
    let keys: Vec<_> = listed.contents().iter().filter_map(|o| o.key()).collect();
    assert_eq!(keys, vec!["allowed/a"]);

    let err = scoped
        .list_objects_v2()
        .bucket("bkt")
        .prefix("outside/")
        .send()
        .await
        .unwrap_err();
    assert_access_denied(&err, "s3:ListBucket");

    // No prefix at all: the condition cannot hold.
    let err = scoped
        .list_objects_v2()
        .bucket("bkt")
        .send()
        .await
        .unwrap_err();
    assert_access_denied(&err, "s3:ListBucket");
}

#[tokio::test]
async fn multipart_uploads_follow_the_policy() {
    let service = enforcing_service();
    setup(&service).await;
    let scoped = client(&service, SCOPED_KEY).await;

    // s3:PutObject covers CreateMultipartUpload and UploadPart.
    let upload = scoped
        .create_multipart_upload()
        .bucket("bkt")
        .key("allowed/mpu")
        .send()
        .await
        .unwrap();
    let upload_id = upload.upload_id().unwrap();
    scoped
        .upload_part()
        .bucket("bkt")
        .key("allowed/mpu")
        .upload_id(upload_id)
        .part_number(1)
        .body(ByteStream::from_static(b"part"))
        .send()
        .await
        .unwrap();
    scoped
        .list_parts()
        .bucket("bkt")
        .key("allowed/mpu")
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();
    scoped
        .abort_multipart_upload()
        .bucket("bkt")
        .key("allowed/mpu")
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();

    let err = scoped
        .create_multipart_upload()
        .bucket("bkt")
        .key("outside/mpu")
        .send()
        .await
        .unwrap_err();
    assert_access_denied(&err, "s3:PutObject");
}

#[tokio::test]
async fn explicit_deny_in_the_session_policy() {
    let service = enforcing_service();
    setup(&service).await;
    let denying = client(&service, DENYING_KEY).await;

    put(&denying, "outside/anything").await.unwrap();
    let err = put(&denying, "allowed/locked/x").await.unwrap_err();
    assert_access_denied(&err, "with an explicit deny in a session policy");
}

#[tokio::test]
async fn delete_objects_is_authorised_per_key() {
    let service = enforcing_service();
    setup(&service).await;
    let scoped = client(&service, SCOPED_KEY).await;

    let delete = Delete::builder()
        .objects(
            ObjectIdentifier::builder()
                .key("allowed/a")
                .build()
                .unwrap(),
        )
        .objects(
            ObjectIdentifier::builder()
                .key("outside/b")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    let result = scoped
        .delete_objects()
        .bucket("bkt")
        .delete(delete)
        .send()
        .await
        .unwrap();
    let deleted: Vec<_> = result.deleted().iter().filter_map(|d| d.key()).collect();
    assert_eq!(deleted, vec!["allowed/a"]);
    let errors: Vec<_> = result
        .errors()
        .iter()
        .map(|e| (e.key().unwrap_or_default(), e.code().unwrap_or_default()))
        .collect();
    assert_eq!(errors, vec![("outside/b", "AccessDenied")]);
}

/// A session policy that cannot be parsed allows nothing.
#[tokio::test]
async fn unparseable_session_policy_allows_nothing() {
    let service = enforcing_service();
    setup(&service).await;
    let broken = client(&service, BROKEN_KEY).await;
    let err = put(&broken, "allowed/a").await.unwrap_err();
    assert_access_denied(&err, "because no session policy allows");
}

/// Credentials without a session policy, and every caller when enforcement
/// is off, are not restricted.
#[tokio::test]
async fn unrestricted_callers_and_enforcement_off() {
    let service = enforcing_service();
    let admin = setup(&service).await;
    put(&admin, "outside/admin").await.unwrap();
    admin.list_objects_v2().bucket("bkt").send().await.unwrap();

    let service = Arc::new(S3Service::new());
    assert!(!service.session_policy_enforcement());
    setup(&service).await;
    let scoped = client(&service, SCOPED_KEY).await;
    put(&scoped, "outside/new").await.unwrap();
}

/// Bucket creation and configuration are not checked: a scoped caller can
/// still create a bucket its policy says nothing about.
#[tokio::test]
async fn control_plane_is_not_checked() {
    let service = enforcing_service();
    let scoped = client(&service, SCOPED_KEY).await;
    scoped.create_bucket().bucket("other").send().await.unwrap();
    scoped
        .get_bucket_location()
        .bucket("other")
        .send()
        .await
        .unwrap();
}

/// A presigned URL carries the access key in `X-Amz-Credential`.
#[tokio::test]
async fn presigned_requests_are_checked() {
    let service = enforcing_service();
    setup(&service).await;
    let get = |key: &str| {
        let uri = format!(
            "http://s3.us-east-1.amazonaws.com/bkt/{key}?X-Amz-Algorithm=AWS4-HMAC-SHA256\
             &X-Amz-Credential={SCOPED_KEY}%2F20260101%2Fus-east-1%2Fs3%2Faws4_request\
             &X-Amz-Date=20260101T000000Z&X-Amz-Expires=300&X-Amz-SignedHeaders=host\
             &X-Amz-Signature=0000"
        );
        let service = Arc::clone(&service);
        async move {
            service
                .handle(MockRequest {
                    method: "GET".to_string(),
                    uri,
                    headers: http::HeaderMap::new(),
                    body: Bytes::new(),
                })
                .await
        }
    };
    assert_eq!(get("allowed/a").await.status, 200);
    assert_eq!(get("outside/b").await.status, 403);
}
