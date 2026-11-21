//! Opt-in bucket policy enforcement ( `S3Service::with_bucket_policy_enforcement` ).
//!
//! Only explicit `Deny` statements are enforced, on data-plane operations.
//! Denials are asserted on the HTTP status and the S3 error code.

use std::sync::Arc;

use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::error::{ProvideErrorMetadata, SdkError};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{
    BucketVersioningStatus, Delete, ObjectIdentifier, ServerSideEncryption, VersioningConfiguration,
};
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use bytes::Bytes;
use winterbaume_core::{MockAws, MockRequest, MockService};
use winterbaume_s3::S3Service;

async fn client_for(mock: &MockAws, endpoint: Option<&str>) -> aws_sdk_s3::Client {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .load()
        .await;
    let mut builder = aws_sdk_s3::config::Builder::from(&config);
    if let Some(endpoint) = endpoint {
        builder = builder.endpoint_url(endpoint).force_path_style(true);
    }
    aws_sdk_s3::Client::from_conf(builder.build())
}

async fn client_with(service: S3Service) -> aws_sdk_s3::Client {
    let mock = MockAws::builder().with_service(service).build();
    client_for(&mock, None).await
}

async fn enforcing_client() -> aws_sdk_s3::Client {
    client_with(S3Service::new().with_bucket_policy_enforcement(true)).await
}

/// The request was refused with S3's `AccessDenied`: HTTP 403 and that code.
fn assert_access_denied<E>(err: &SdkError<E, HttpResponse>)
where
    E: ProvideErrorMetadata + std::fmt::Debug,
{
    let status = err.raw_response().map(|r| r.status().as_u16());
    assert_eq!(status, Some(403), "expected HTTP 403, got {err:?}");
    assert_eq!(err.code(), Some("AccessDenied"), "got {err:?}");
}

/// A `HEAD` denial: HTTP 403 with no body, hence no error code.
fn assert_forbidden_without_body<E: std::fmt::Debug>(err: &SdkError<E, HttpResponse>) {
    let response = err.raw_response().expect("an HTTP response");
    assert_eq!(response.status().as_u16(), 403, "got {err:?}");
    assert_eq!(response.body().bytes(), Some(&b""[..]));
}

async fn create_bucket(client: &aws_sdk_s3::Client, bucket: &str) {
    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .expect("create_bucket");
}

async fn put_policy(client: &aws_sdk_s3::Client, bucket: &str, policy: &str) {
    client
        .put_bucket_policy()
        .bucket(bucket)
        .policy(policy)
        .send()
        .await
        .expect("put_bucket_policy");
}

async fn put(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
) -> Result<
    aws_sdk_s3::operation::put_object::PutObjectOutput,
    SdkError<aws_sdk_s3::operation::put_object::PutObjectError, HttpResponse>,
> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from_static(b"hello\n"))
        .send()
        .await
}

async fn keys(client: &aws_sdk_s3::Client, bucket: &str) -> Vec<String> {
    client
        .list_objects_v2()
        .bucket(bucket)
        .send()
        .await
        .expect("list_objects_v2")
        .contents()
        .iter()
        .filter_map(|o| o.key().map(str::to_string))
        .collect()
}

/// The policy from issue #49.
const DENY_PUT: &str = r#"{"Version":"2012-10-17","Statement":[{"Sid":"DenyAll","Effect":"Deny","Principal":"*","Action":"s3:PutObject","Resource":"arn:aws:s3:::wb-pol/*"}]}"#;

#[tokio::test]
async fn enforcement_is_off_by_default_and_the_issue_scenario_succeeds() {
    assert!(!S3Service::new().bucket_policy_enforcement());
    for service in [
        S3Service::new(),
        S3Service::new().with_bucket_policy_enforcement(false),
    ] {
        let client = client_with(service).await;
        create_bucket(&client, "wb-pol").await;
        put_policy(&client, "wb-pol", DENY_PUT).await;

        let output = put(&client, "wb-pol", "k").await.expect("put_object");
        assert_eq!(output.e_tag(), Some("\"b1946ac92492d2347c6235b4d2611184\""));
        assert_eq!(keys(&client, "wb-pol").await, ["k"]);
    }
}

#[tokio::test]
async fn enforcement_denies_the_issue_scenario_and_writes_nothing() {
    let client = enforcing_client().await;
    create_bucket(&client, "wb-pol").await;
    put_policy(&client, "wb-pol", DENY_PUT).await;

    let err = put(&client, "wb-pol", "k").await.expect_err("denied");
    assert_access_denied(&err);
    let message = err.message().unwrap_or_default();
    assert!(message.contains("s3:PutObject"), "{message}");
    assert!(message.contains("arn:aws:s3:::wb-pol/k"), "{message}");

    assert!(keys(&client, "wb-pol").await.is_empty());
    let err = client
        .get_object()
        .bucket("wb-pol")
        .key("k")
        .send()
        .await
        .expect_err("nothing was written");
    assert_eq!(err.raw_response().map(|r| r.status().as_u16()), Some(404));
    assert_eq!(err.code(), Some("NoSuchKey"));
}

#[tokio::test]
async fn builder_composes_with_the_vfs_constructors() {
    let vfs: Arc<dyn winterbaume_core::Vfs> = Arc::new(winterbaume_core::MemVfs::new());
    let service = S3Service::with_vfs(Arc::clone(&vfs)).with_bucket_policy_enforcement(true);
    assert!(service.bucket_policy_enforcement());
    let client = client_with(service).await;
    create_bucket(&client, "wb-pol").await;
    put_policy(&client, "wb-pol", DENY_PUT).await;
    assert_access_denied(&put(&client, "wb-pol", "k").await.expect_err("denied"));

    // The switch is per service instance, not persisted: a service recovered
    // from the same storage enforces the recovered policy only when asked to.
    let recovered = S3Service::recover_with_vfs(Arc::clone(&vfs))
        .await
        .expect("recover");
    assert!(!recovered.bucket_policy_enforcement());
    let recovered = S3Service::recover_with_vfs(vfs)
        .await
        .expect("recover")
        .with_bucket_policy_enforcement(true);
    let client = client_with(recovered).await;
    assert_access_denied(&put(&client, "wb-pol", "k").await.expect_err("denied"));
}

#[tokio::test]
async fn deny_scoped_to_a_prefix_denies_inside_and_allows_outside() {
    let client = enforcing_client().await;
    create_bucket(&client, "scoped").await;
    put_policy(
        &client,
        "scoped",
        r#"{"Version":"2012-10-17","Statement":[{"Effect":"Deny","Principal":{"AWS":"*"},
            "Action":["s3:PutObject","s3:DeleteObject"],
            "Resource":"arn:aws:s3:::scoped/locked/*"}]}"#,
    )
    .await;

    assert_access_denied(
        &put(&client, "scoped", "locked/a")
            .await
            .expect_err("inside"),
    );
    put(&client, "scoped", "open/a").await.expect("outside");
    put(&client, "scoped", "locked")
        .await
        .expect("not under locked/");
    assert_eq!(keys(&client, "scoped").await, ["locked", "open/a"]);
}

#[tokio::test]
async fn non_matching_actions_and_buckets_are_unaffected() {
    let client = enforcing_client().await;
    create_bucket(&client, "acts").await;
    create_bucket(&client, "other").await;
    put(&client, "acts", "k").await.expect("before the policy");
    put_policy(
        &client,
        "acts",
        r#"{"Statement":[{"Effect":"Deny","Principal":"*","Action":"s3:PutObject","Resource":"arn:aws:s3:::acts/*"}]}"#,
    )
    .await;

    assert_access_denied(&put(&client, "acts", "k2").await.expect_err("denied"));
    // Read, head, list and delete are different actions.
    let body = client
        .get_object()
        .bucket("acts")
        .key("k")
        .send()
        .await
        .expect("get_object")
        .body
        .collect()
        .await
        .unwrap()
        .into_bytes();
    assert_eq!(&body[..], b"hello\n");
    client
        .head_object()
        .bucket("acts")
        .key("k")
        .send()
        .await
        .expect("head_object");
    client
        .head_bucket()
        .bucket("acts")
        .send()
        .await
        .expect("head_bucket");
    assert_eq!(keys(&client, "acts").await, ["k"]);
    client
        .delete_object()
        .bucket("acts")
        .key("k")
        .send()
        .await
        .expect("delete_object");
    // A bucket without a policy is untouched.
    put(&client, "other", "k").await.expect("other bucket");
}

#[tokio::test]
async fn allow_only_policy_changes_nothing() {
    let client = enforcing_client().await;
    create_bucket(&client, "allow").await;
    // Allow statements are not evaluated: neither the listed nor the unlisted
    // actions are affected.
    put_policy(
        &client,
        "allow",
        r#"{"Statement":[{"Effect":"Allow","Principal":{"AWS":"arn:aws:iam::111122223333:root"},
            "Action":"s3:GetObject","Resource":"arn:aws:s3:::allow/*"}]}"#,
    )
    .await;

    put(&client, "allow", "k").await.expect("put_object");
    client
        .get_object()
        .bucket("allow")
        .key("k")
        .send()
        .await
        .expect("get_object");
    assert_eq!(keys(&client, "allow").await, ["k"]);
    client
        .delete_object()
        .bucket("allow")
        .key("k")
        .send()
        .await
        .expect("delete_object");
}

#[tokio::test]
async fn malformed_stored_policy_does_not_break_request_handling() {
    let client = enforcing_client().await;
    create_bucket(&client, "broken").await;
    for policy in [
        "this is not json",
        r#"{"Statement": 7}"#,
        r#"{"Statement":[{"Effect":"Maybe","Action":"s3:*","Resource":"*"}]}"#,
        "",
    ] {
        client
            .put_bucket_policy()
            .bucket("broken")
            .policy(policy)
            .send()
            .await
            .expect("put_bucket_policy stores the text as given");
        put(&client, "broken", "k").await.expect("put_object");
        client
            .get_object()
            .bucket("broken")
            .key("k")
            .send()
            .await
            .expect("get_object");
        assert_eq!(keys(&client, "broken").await, ["k"]);
    }
}

#[tokio::test]
async fn statements_for_specific_principals_are_skipped() {
    let client = enforcing_client().await;
    create_bucket(&client, "who").await;
    // The emulator cannot tell who is calling, so none of these apply.
    put_policy(
        &client,
        "who",
        r#"{"Statement":[
            {"Effect":"Deny","Principal":{"AWS":"arn:aws:iam::111122223333:user/mallory"},
             "Action":"s3:*","Resource":["arn:aws:s3:::who","arn:aws:s3:::who/*"]},
            {"Effect":"Deny","NotPrincipal":{"AWS":"arn:aws:iam::111122223333:role/app"},
             "Action":"s3:*","Resource":["arn:aws:s3:::who","arn:aws:s3:::who/*"]},
            {"Effect":"Deny","Principal":"*","Action":"s3:*",
             "Resource":["arn:aws:s3:::who","arn:aws:s3:::who/*"],
             "Condition":{"ArnNotLike":{"aws:PrincipalArn":"arn:aws:iam::111122223333:role/app"}}}
        ]}"#,
    )
    .await;

    put(&client, "who", "k").await.expect("put_object");
    assert_eq!(keys(&client, "who").await, ["k"]);
}

const DENY_INSECURE: &str = r#"{"Version":"2012-10-17","Statement":[{"Sid":"TlsOnly","Effect":"Deny","Principal":"*",
    "Action":"s3:*","Resource":["arn:aws:s3:::tls","arn:aws:s3:::tls/*"],
    "Condition":{"Bool":{"aws:SecureTransport":"false"}}}]}"#;

#[tokio::test]
async fn secure_transport_follows_the_request_scheme() {
    let mock = MockAws::builder()
        .with_service(S3Service::new().with_bucket_policy_enforcement(true))
        .build();
    let https = client_for(&mock, None).await;
    let http = client_for(&mock, Some("http://s3.us-east-1.amazonaws.com")).await;

    create_bucket(&https, "tls").await;
    put_policy(&https, "tls", DENY_INSECURE).await;

    // TLS: nothing is denied.
    put(&https, "tls", "k").await.expect("put over https");
    assert_eq!(keys(&https, "tls").await, ["k"]);

    // Plain HTTP: every data-plane action on the bucket is denied.
    assert_access_denied(&put(&http, "tls", "k2").await.expect_err("put over http"));
    let err = http
        .get_object()
        .bucket("tls")
        .key("k")
        .send()
        .await
        .expect_err("get over http");
    assert_access_denied(&err);
    let err = http
        .list_objects_v2()
        .bucket("tls")
        .send()
        .await
        .expect_err("list over http");
    assert_access_denied(&err);
    let err = http
        .head_object()
        .bucket("tls")
        .key("k")
        .send()
        .await
        .expect_err("head over http");
    assert_forbidden_without_body(&err);
    let err = http
        .head_bucket()
        .bucket("tls")
        .send()
        .await
        .expect_err("head bucket over http");
    assert_forbidden_without_body(&err);

    // The policy itself stays manageable over plain HTTP.
    http.get_bucket_policy()
        .bucket("tls")
        .send()
        .await
        .expect("get_bucket_policy");
    assert_eq!(keys(&https, "tls").await, ["k"]);
}

fn status_and_code(response: &winterbaume_core::MockResponse) -> (u16, bool) {
    let body = String::from_utf8_lossy(&response.body);
    (response.status, body.contains("<Code>AccessDenied</Code>"))
}

/// `winterbaume-server` hands every service an `https://` URI, so a front end
/// conveys the real transport with `X-Forwarded-Proto`.
#[tokio::test]
async fn secure_transport_honours_x_forwarded_proto() {
    let service = Arc::new(S3Service::new().with_bucket_policy_enforcement(true));
    let mock = MockAws::builder()
        .with_service(Arc::clone(&service))
        .build();
    let client = client_for(&mock, None).await;
    create_bucket(&client, "tls").await;
    put_policy(&client, "tls", DENY_INSECURE).await;
    put(&client, "tls", "k").await.expect("put over https");

    let get = |uri: &str, forwarded_proto: Option<&str>| {
        let mut headers = http::HeaderMap::new();
        if let Some(proto) = forwarded_proto {
            headers.insert("x-forwarded-proto", proto.parse().unwrap());
        }
        service.handle(MockRequest {
            method: "GET".to_string(),
            uri: uri.to_string(),
            headers,
            body: Bytes::new(),
        })
    };
    let https_uri = "https://s3.us-east-1.amazonaws.com/tls/k";
    let http_uri = "http://s3.us-east-1.amazonaws.com/tls/k";

    assert_eq!(status_and_code(&get(https_uri, None).await), (200, false));
    assert_eq!(status_and_code(&get(http_uri, None).await), (403, true));
    // The header overrides the URI scheme in both directions.
    assert_eq!(
        status_and_code(&get(https_uri, Some("http")).await),
        (403, true)
    );
    assert_eq!(
        status_and_code(&get(http_uri, Some("https")).await),
        (200, false)
    );
}

#[tokio::test]
async fn s3_prefix_condition_on_list() {
    let client = enforcing_client().await;
    create_bucket(&client, "lst").await;
    for key in ["home/alice/a", "home/bob/b", "top"] {
        put(&client, "lst", key).await.expect("put_object");
    }
    // Listing is confined to home/alice/; a request without a prefix has no
    // `s3:prefix` key, which a negated operator treats as a match.
    put_policy(
        &client,
        "lst",
        r#"{"Statement":[{"Effect":"Deny","Principal":"*","Action":"s3:ListBucket",
            "Resource":"arn:aws:s3:::lst",
            "Condition":{"StringNotLike":{"s3:prefix":"home/alice/*"}}}]}"#,
    )
    .await;

    let listed = client
        .list_objects_v2()
        .bucket("lst")
        .prefix("home/alice/")
        .send()
        .await
        .expect("allowed prefix");
    assert_eq!(listed.contents().len(), 1);
    assert_eq!(listed.contents()[0].key(), Some("home/alice/a"));
    client
        .list_objects()
        .bucket("lst")
        .prefix("home/alice/")
        .send()
        .await
        .expect("ListObjects v1 with the allowed prefix");

    let err = client
        .list_objects_v2()
        .bucket("lst")
        .prefix("home/bob/")
        .send()
        .await
        .expect_err("other prefix");
    assert_access_denied(&err);
    let err = client
        .list_objects_v2()
        .bucket("lst")
        .send()
        .await
        .expect_err("no prefix");
    assert_access_denied(&err);
    let err = client
        .list_objects()
        .bucket("lst")
        .prefix("home/")
        .send()
        .await
        .expect_err("ListObjects v1, shorter prefix");
    assert_access_denied(&err);

    // Object reads are a different action and resource.
    client
        .get_object()
        .bucket("lst")
        .key("home/bob/b")
        .send()
        .await
        .expect("get_object");
}

#[tokio::test]
async fn list_conditions_on_max_keys_and_delimiter() {
    let client = enforcing_client().await;
    create_bucket(&client, "page").await;
    put(&client, "page", "a/b").await.expect("put_object");
    put_policy(
        &client,
        "page",
        r#"{"Statement":[
            {"Effect":"Deny","Principal":"*","Action":"s3:ListBucket","Resource":"arn:aws:s3:::page",
             "Condition":{"NumericGreaterThan":{"s3:max-keys":"10"}}},
            {"Effect":"Deny","Principal":"*","Action":"s3:ListBucket","Resource":"arn:aws:s3:::page",
             "Condition":{"StringEquals":{"s3:delimiter":"|"}}}
        ]}"#,
    )
    .await;

    client
        .list_objects_v2()
        .bucket("page")
        .max_keys(10)
        .delimiter("/")
        .send()
        .await
        .expect("within limits");
    // No max-keys / delimiter sent: the keys are absent and positive
    // operators do not match.
    client
        .list_objects_v2()
        .bucket("page")
        .send()
        .await
        .expect("no listing parameters");
    let err = client
        .list_objects_v2()
        .bucket("page")
        .max_keys(11)
        .send()
        .await
        .expect_err("max-keys too large");
    assert_access_denied(&err);
    let err = client
        .list_objects_v2()
        .bucket("page")
        .delimiter("|")
        .send()
        .await
        .expect_err("denied delimiter");
    assert_access_denied(&err);
}

#[tokio::test]
async fn header_conditions_and_the_missing_key_rule() {
    let client = enforcing_client().await;
    create_bucket(&client, "hdr").await;
    put_policy(
        &client,
        "hdr",
        r#"{"Statement":[
            {"Sid":"RequireKms","Effect":"Deny","Principal":"*","Action":"s3:PutObject",
             "Resource":"arn:aws:s3:::hdr/enc/*",
             "Condition":{"StringNotEquals":{"s3:x-amz-server-side-encryption":"aws:kms"}}},
            {"Sid":"NoPublicAcl","Effect":"Deny","Principal":"*","Action":"s3:PutObject",
             "Resource":"arn:aws:s3:::hdr/*",
             "Condition":{"StringEquals":{"s3:x-amz-acl":["public-read","public-read-write"]}}},
            {"Sid":"NoGlacier","Effect":"Deny","Principal":"*","Action":"s3:PutObject",
             "Resource":"arn:aws:s3:::hdr/*",
             "Condition":{"StringEquals":{"s3:x-amz-storage-class":"GLACIER"}}},
            {"Sid":"IfExists","Effect":"Deny","Principal":"*","Action":"s3:PutObject",
             "Resource":"arn:aws:s3:::hdr/ifx/*",
             "Condition":{"StringEqualsIfExists":{"s3:x-amz-storage-class":"REDUCED_REDUNDANCY"}}},
            {"Sid":"CannotSupply","Effect":"Deny","Principal":"*","Action":"s3:GetObject",
             "Resource":"arn:aws:s3:::hdr/*",
             "Condition":{"StringEquals":{"s3:ExistingObjectTag/classification":"secret"}}}
        ]}"#,
    )
    .await;
    let body = || ByteStream::from_static(b"x");

    // RequireKms: the header is missing, and a negated operator matches a
    // missing key, so the Deny applies.
    assert_access_denied(
        &put(&client, "hdr", "enc/a")
            .await
            .expect_err("no SSE header"),
    );
    let err = client
        .put_object()
        .bucket("hdr")
        .key("enc/a")
        .server_side_encryption(ServerSideEncryption::Aes256)
        .body(body())
        .send()
        .await
        .expect_err("wrong SSE");
    assert_access_denied(&err);
    client
        .put_object()
        .bucket("hdr")
        .key("enc/a")
        .server_side_encryption(ServerSideEncryption::AwsKms)
        .body(body())
        .send()
        .await
        .expect("SSE-KMS");

    // NoPublicAcl: a positive operator does not match a missing key.
    put(&client, "hdr", "plain").await.expect("no ACL header");
    let err = client
        .put_object()
        .bucket("hdr")
        .key("pub")
        .acl(aws_sdk_s3::types::ObjectCannedAcl::PublicRead)
        .body(body())
        .send()
        .await
        .expect_err("public ACL");
    assert_access_denied(&err);
    client
        .put_object()
        .bucket("hdr")
        .key("priv")
        .acl(aws_sdk_s3::types::ObjectCannedAcl::Private)
        .body(body())
        .send()
        .await
        .expect("private ACL");

    // NoGlacier: the storage class requested in the header.
    let err = client
        .put_object()
        .bucket("hdr")
        .key("cold")
        .storage_class(aws_sdk_s3::types::StorageClass::Glacier)
        .body(body())
        .send()
        .await
        .expect_err("GLACIER");
    assert_access_denied(&err);
    client
        .put_object()
        .bucket("hdr")
        .key("warm")
        .storage_class(aws_sdk_s3::types::StorageClass::StandardIa)
        .body(body())
        .send()
        .await
        .expect("STANDARD_IA");

    // IfExists: holds when the key is missing, is tested when it is present.
    assert_access_denied(&put(&client, "hdr", "ifx/a").await.expect_err("no header"));
    client
        .put_object()
        .bucket("hdr")
        .key("ifx/a")
        .storage_class(aws_sdk_s3::types::StorageClass::StandardIa)
        .body(body())
        .send()
        .await
        .expect("a different storage class");

    // CannotSupply: a key the emulator does not provide never satisfies a
    // positive operator, so the statement does not apply.
    client
        .get_object()
        .bucket("hdr")
        .key("plain")
        .send()
        .await
        .expect("get_object");
}

#[tokio::test]
async fn copy_object_checks_source_read_and_destination_write() {
    let client = enforcing_client().await;
    for bucket in ["src", "dst"] {
        create_bucket(&client, bucket).await;
    }
    put(&client, "src", "open/a").await.expect("put_object");
    put(&client, "src", "secret/a").await.expect("put_object");
    put_policy(
        &client,
        "src",
        r#"{"Statement":[{"Effect":"Deny","Principal":"*","Action":"s3:GetObject","Resource":"arn:aws:s3:::src/secret/*"}]}"#,
    )
    .await;
    put_policy(
        &client,
        "dst",
        r#"{"Statement":[{"Effect":"Deny","Principal":"*","Action":"s3:PutObject","Resource":"arn:aws:s3:::dst/frozen/*"}]}"#,
    )
    .await;

    client
        .copy_object()
        .bucket("dst")
        .key("in/a")
        .copy_source("src/open/a")
        .send()
        .await
        .expect("readable source, writable destination");

    // The source bucket's policy denies the read.
    let err = client
        .copy_object()
        .bucket("dst")
        .key("in/b")
        .copy_source("src/secret/a")
        .send()
        .await
        .expect_err("source read denied");
    assert_access_denied(&err);
    let message = err.message().unwrap_or_default();
    assert!(message.contains("s3:GetObject"), "{message}");
    assert!(message.contains("arn:aws:s3:::src/secret/a"), "{message}");

    // The destination bucket's policy denies the write.
    let err = client
        .copy_object()
        .bucket("dst")
        .key("frozen/a")
        .copy_source("src/open/a")
        .send()
        .await
        .expect_err("destination write denied");
    assert_access_denied(&err);
    let message = err.message().unwrap_or_default();
    assert!(message.contains("s3:PutObject"), "{message}");
    assert!(message.contains("arn:aws:s3:::dst/frozen/a"), "{message}");

    assert_eq!(keys(&client, "dst").await, ["in/a"]);
}

#[tokio::test]
async fn delete_objects_reports_denied_keys_and_deletes_the_rest() {
    let client = enforcing_client().await;
    create_bucket(&client, "bulk").await;
    for key in ["keep/a", "keep/b", "tmp/a", "tmp/b"] {
        put(&client, "bulk", key).await.expect("put_object");
    }
    put_policy(
        &client,
        "bulk",
        r#"{"Statement":[{"Effect":"Deny","Principal":"*","Action":"s3:DeleteObject","Resource":"arn:aws:s3:::bulk/keep/*"}]}"#,
    )
    .await;

    let delete = Delete::builder()
        .set_objects(Some(
            ["keep/a", "tmp/a", "keep/b", "tmp/b"]
                .into_iter()
                .map(|k| ObjectIdentifier::builder().key(k).build().unwrap())
                .collect(),
        ))
        .build()
        .unwrap();
    let output = client
        .delete_objects()
        .bucket("bulk")
        .delete(delete)
        .send()
        .await
        .expect("the call itself succeeds");

    let deleted: Vec<_> = output.deleted().iter().filter_map(|d| d.key()).collect();
    assert_eq!(deleted, ["tmp/a", "tmp/b"]);
    let errors: Vec<_> = output
        .errors()
        .iter()
        .map(|e| (e.key(), e.code(), e.message()))
        .collect();
    assert_eq!(
        errors,
        [
            (Some("keep/a"), Some("AccessDenied"), Some("Access Denied")),
            (Some("keep/b"), Some("AccessDenied"), Some("Access Denied")),
        ]
    );
    assert_eq!(keys(&client, "bulk").await, ["keep/a", "keep/b"]);

    // Single-object delete of a protected key fails outright.
    let err = client
        .delete_object()
        .bucket("bulk")
        .key("keep/a")
        .send()
        .await
        .expect_err("denied");
    assert_access_denied(&err);
    assert_eq!(keys(&client, "bulk").await, ["keep/a", "keep/b"]);
}

#[tokio::test]
async fn delete_objects_without_denials_is_unchanged() {
    let client = enforcing_client().await;
    create_bucket(&client, "bulk").await;
    put(&client, "bulk", "a").await.expect("put_object");
    let delete = Delete::builder()
        .objects(ObjectIdentifier::builder().key("a").build().unwrap())
        .build()
        .unwrap();
    let output = client
        .delete_objects()
        .bucket("bulk")
        .delete(delete)
        .send()
        .await
        .expect("delete_objects");
    assert_eq!(output.deleted().len(), 1);
    assert!(output.errors().is_empty());
}

#[tokio::test]
async fn versioned_reads_use_their_own_action() {
    let client = enforcing_client().await;
    create_bucket(&client, "ver").await;
    client
        .put_bucket_versioning()
        .bucket("ver")
        .versioning_configuration(
            VersioningConfiguration::builder()
                .status(BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("put_bucket_versioning");
    let version_id = put(&client, "ver", "k")
        .await
        .expect("put_object")
        .version_id()
        .expect("a version id")
        .to_string();
    put_policy(
        &client,
        "ver",
        r#"{"Statement":[
            {"Effect":"Deny","Principal":"*","Action":["s3:GetObjectVersion","s3:DeleteObjectVersion"],
             "Resource":"arn:aws:s3:::ver/*"},
            {"Effect":"Deny","Principal":"*","Action":"s3:ListBucketVersions","Resource":"arn:aws:s3:::ver"}
        ]}"#,
    )
    .await;

    client
        .get_object()
        .bucket("ver")
        .key("k")
        .send()
        .await
        .expect("current version: s3:GetObject");
    let err = client
        .get_object()
        .bucket("ver")
        .key("k")
        .version_id(&version_id)
        .send()
        .await
        .expect_err("specific version: s3:GetObjectVersion");
    assert_access_denied(&err);
    let err = client
        .head_object()
        .bucket("ver")
        .key("k")
        .version_id(&version_id)
        .send()
        .await
        .expect_err("head of a specific version");
    assert_forbidden_without_body(&err);
    let err = client
        .delete_object()
        .bucket("ver")
        .key("k")
        .version_id(&version_id)
        .send()
        .await
        .expect_err("s3:DeleteObjectVersion");
    assert_access_denied(&err);
    let err = client
        .list_object_versions()
        .bucket("ver")
        .send()
        .await
        .expect_err("s3:ListBucketVersions");
    assert_access_denied(&err);
    // ListObjectsV2 is s3:ListBucket, which is not denied.
    assert_eq!(keys(&client, "ver").await, ["k"]);
}

#[tokio::test]
async fn multipart_tagging_and_acl_operations_are_enforced() {
    let client = enforcing_client().await;
    create_bucket(&client, "ops").await;
    put(&client, "ops", "k").await.expect("put_object");
    let upload_id = client
        .create_multipart_upload()
        .bucket("ops")
        .key("mp")
        .send()
        .await
        .expect("create_multipart_upload")
        .upload_id()
        .expect("upload id")
        .to_string();
    put_policy(
        &client,
        "ops",
        r#"{"Statement":[
            {"Effect":"Deny","Principal":"*",
             "Action":["s3:PutObject","s3:ListMultipartUploadParts","s3:GetObjectTagging",
                       "s3:PutObjectTagging","s3:DeleteObjectTagging","s3:GetObjectAcl","s3:PutObjectAcl"],
             "Resource":"arn:aws:s3:::ops/*"},
            {"Effect":"Deny","Principal":"*","Action":"s3:ListBucketMultipartUploads","Resource":"arn:aws:s3:::ops"}
        ]}"#,
    )
    .await;

    let err = client
        .create_multipart_upload()
        .bucket("ops")
        .key("mp2")
        .send()
        .await
        .expect_err("CreateMultipartUpload is s3:PutObject");
    assert_access_denied(&err);
    let err = client
        .upload_part()
        .bucket("ops")
        .key("mp")
        .upload_id(&upload_id)
        .part_number(1)
        .body(ByteStream::from_static(b"part"))
        .send()
        .await
        .expect_err("UploadPart is s3:PutObject");
    assert_access_denied(&err);
    let err = client
        .list_parts()
        .bucket("ops")
        .key("mp")
        .upload_id(&upload_id)
        .send()
        .await
        .expect_err("ListParts");
    assert_access_denied(&err);
    let err = client
        .list_multipart_uploads()
        .bucket("ops")
        .send()
        .await
        .expect_err("ListMultipartUploads");
    assert_access_denied(&err);
    let err = client
        .get_object_tagging()
        .bucket("ops")
        .key("k")
        .send()
        .await
        .expect_err("GetObjectTagging");
    assert_access_denied(&err);
    let err = client
        .delete_object_tagging()
        .bucket("ops")
        .key("k")
        .send()
        .await
        .expect_err("DeleteObjectTagging");
    assert_access_denied(&err);
    let err = client
        .get_object_acl()
        .bucket("ops")
        .key("k")
        .send()
        .await
        .expect_err("GetObjectAcl");
    assert_access_denied(&err);
    let err = client
        .put_object_acl()
        .bucket("ops")
        .key("k")
        .acl(aws_sdk_s3::types::ObjectCannedAcl::Private)
        .send()
        .await
        .expect_err("PutObjectAcl");
    assert_access_denied(&err);

    // s3:AbortMultipartUpload is not in the Deny list.
    client
        .abort_multipart_upload()
        .bucket("ops")
        .key("mp")
        .upload_id(&upload_id)
        .send()
        .await
        .expect("abort_multipart_upload");
    // Bucket-level tagging is control-plane and is never enforced.
    let err = client
        .get_bucket_tagging()
        .bucket("ops")
        .send()
        .await
        .expect_err("no tags set");
    assert_eq!(err.code(), Some("NoSuchTagSet"));
}

#[tokio::test]
async fn policy_can_be_replaced_and_deleted_under_a_deny_all() {
    let client = enforcing_client().await;
    create_bucket(&client, "locked").await;
    put(&client, "locked", "k").await.expect("put_object");
    let deny_all =
        r#"{"Statement":[{"Effect":"Deny","Principal":"*","Action":"*","Resource":"*"}]}"#;
    put_policy(&client, "locked", deny_all).await;

    assert_access_denied(&put(&client, "locked", "k2").await.expect_err("denied"));
    let err = client
        .list_objects_v2()
        .bucket("locked")
        .send()
        .await
        .expect_err("denied");
    assert_access_denied(&err);

    // Control-plane operations are not enforced: the policy can be read and
    // replaced, and other bucket configuration still works.
    let stored = client
        .get_bucket_policy()
        .bucket("locked")
        .send()
        .await
        .expect("get_bucket_policy");
    assert_eq!(stored.policy(), Some(deny_all));
    client
        .get_bucket_versioning()
        .bucket("locked")
        .send()
        .await
        .expect("get_bucket_versioning");
    put_policy(
        &client,
        "locked",
        r#"{"Statement":[{"Effect":"Deny","Principal":"*","Action":"s3:DeleteObject","Resource":"arn:aws:s3:::locked/*"}]}"#,
    )
    .await;
    put(&client, "locked", "k2").await.expect("replaced policy");
    let err = client
        .delete_object()
        .bucket("locked")
        .key("k2")
        .send()
        .await
        .expect_err("the replacement is enforced");
    assert_access_denied(&err);

    put_policy(&client, "locked", deny_all).await;
    client
        .delete_bucket_policy()
        .bucket("locked")
        .send()
        .await
        .expect("delete_bucket_policy");
    client
        .delete_object()
        .bucket("locked")
        .key("k2")
        .send()
        .await
        .expect("no policy, no denial");
    assert_eq!(keys(&client, "locked").await, ["k"]);
}
