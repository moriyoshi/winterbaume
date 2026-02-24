//! Smoke tests for winterbaume S3 service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{
    BucketVersioningStatus, CompletedMultipartUpload, CompletedPart, VersioningConfiguration,
};
use winterbaume_core::MockAws;
use winterbaume_s3::S3Service;

async fn make_s3_client_with_service(svc: S3Service) -> aws_sdk_s3::Client {
    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_s3::Client::new(&config)
}

async fn make_s3_client() -> aws_sdk_s3::Client {
    make_s3_client_with_service(S3Service::new()).await
}

/// Scenario: document storage service.
///
/// A web application stores user-uploaded documents in S3 under per-user
/// prefixes. It uploads several files, lists the "folder", downloads a
/// specific document, overwrites one, and finally deletes it.
#[tokio::test]
async fn test_document_storage_workflow() {
    let client = make_s3_client().await;
    let bucket = "docs-bucket";

    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .expect("create_bucket");

    // Upload three documents under a user prefix.
    let docs = [
        ("users/alice/report.pdf", b"PDF content v1" as &[u8]),
        ("users/alice/summary.txt", b"Summary text"),
        ("users/bob/notes.txt", b"Bob's notes"),
    ];
    for (key, body) in docs {
        client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from_static(body))
            .send()
            .await
            .unwrap_or_else(|e| panic!("put_object {key}: {e}"));
    }

    // List Alice's documents via prefix.
    let list = client
        .list_objects_v2()
        .bucket(bucket)
        .prefix("users/alice/")
        .send()
        .await
        .expect("list_objects_v2");
    assert_eq!(list.key_count(), Some(2), "alice should have 2 documents");

    // Download and verify a specific document.
    let get = client
        .get_object()
        .bucket(bucket)
        .key("users/alice/report.pdf")
        .send()
        .await
        .expect("get_object report.pdf");
    let body = get.body.collect().await.unwrap().into_bytes();
    assert_eq!(body.as_ref(), b"PDF content v1");

    // Overwrite the document with new content.
    client
        .put_object()
        .bucket(bucket)
        .key("users/alice/report.pdf")
        .body(ByteStream::from_static(b"PDF content v2"))
        .send()
        .await
        .expect("put_object overwrite");

    let get2 = client
        .get_object()
        .bucket(bucket)
        .key("users/alice/report.pdf")
        .send()
        .await
        .expect("get_object after overwrite");
    let body2 = get2.body.collect().await.unwrap().into_bytes();
    assert_eq!(body2.as_ref(), b"PDF content v2");

    // Delete the overwritten document and confirm it is gone.
    client
        .delete_object()
        .bucket(bucket)
        .key("users/alice/report.pdf")
        .send()
        .await
        .expect("delete_object");

    let list_after = client
        .list_objects_v2()
        .bucket(bucket)
        .prefix("users/alice/")
        .send()
        .await
        .expect("list after delete");
    assert_eq!(
        list_after.key_count(),
        Some(1),
        "only summary.txt should remain"
    );
}

/// Scenario: versioned asset pipeline.
///
/// A CI/CD pipeline stores build artefacts in a versioned bucket. It uploads
/// artefact v1, then v2, then retrieves the specific v1 version to roll back,
/// and finally deletes the v2 version leaving v1 as current.
///
/// NOTE: this test is marked `#[ignore]` because S3 object versioning
/// (per-key version history, version-specific GetObject/DeleteObject, and
/// version IDs in PutObject responses) is not yet implemented.
/// See TODO.md: "s3: implement object versioning".
#[tokio::test]
async fn test_versioned_asset_pipeline() {
    let client = make_s3_client().await;
    let bucket = "artefacts-bucket";

    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .expect("create_bucket");

    // Enable versioning.
    client
        .put_bucket_versioning()
        .bucket(bucket)
        .versioning_configuration(
            VersioningConfiguration::builder()
                .status(BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("put_bucket_versioning");

    let key = "app/binary";

    // Upload version 1 — response should carry x-amz-version-id.
    let put_v1 = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from_static(b"binary-v1"))
        .send()
        .await
        .expect("put_object v1");
    let version_id_v1 = put_v1.version_id().expect("v1 version_id").to_string();

    // Upload version 2.
    let put_v2 = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from_static(b"binary-v2"))
        .send()
        .await
        .expect("put_object v2");
    let version_id_v2 = put_v2.version_id().expect("v2 version_id").to_string();

    assert_ne!(version_id_v1, version_id_v2, "version IDs must differ");

    // Current (latest) get should return v2.
    let current = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .expect("get_object current");
    let current_body = current.body.collect().await.unwrap().into_bytes();
    assert_eq!(current_body.as_ref(), b"binary-v2");

    // Roll back: retrieve v1 by explicit version ID.
    let rollback = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .version_id(&version_id_v1)
        .send()
        .await
        .expect("get_object v1");
    let rollback_body = rollback.body.collect().await.unwrap().into_bytes();
    assert_eq!(rollback_body.as_ref(), b"binary-v1");

    // List versions: both v1 and v2 should be present.
    let versions = client
        .list_object_versions()
        .bucket(bucket)
        .prefix(key)
        .send()
        .await
        .expect("list_object_versions");
    assert_eq!(
        versions.versions().len(),
        2,
        "both versions should be listed"
    );

    // Delete only v2, leaving v1 as the surviving version.
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .version_id(&version_id_v2)
        .send()
        .await
        .expect("delete_object v2");

    let versions_after = client
        .list_object_versions()
        .bucket(bucket)
        .prefix(key)
        .send()
        .await
        .expect("list_object_versions after delete");
    assert_eq!(
        versions_after.versions().len(),
        1,
        "only v1 should remain after deleting v2"
    );
}

/// Scenario: large-file upload via multipart.
///
/// An ETL job uploads a large CSV split into three parts via the S3 multipart
/// upload API, then downloads and verifies the assembled object.
#[tokio::test]
async fn test_bulk_data_multipart_upload() {
    let client = make_s3_client().await;
    let bucket = "etl-bucket";
    let key = "data/export.csv";

    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .expect("create_bucket");

    let mpu = client
        .create_multipart_upload()
        .bucket(bucket)
        .key(key)
        .content_type("text/csv")
        .send()
        .await
        .expect("create_multipart_upload");
    let upload_id = mpu.upload_id().expect("upload_id").to_string();

    // Upload three parts (in real usage each would be >= 5 MB).
    let part_data: [&[u8]; 3] = [b"id,name,value\n", b"1,alpha,100\n", b"2,beta,200\n"];
    let mut completed_parts = Vec::new();
    for (i, data) in part_data.iter().enumerate() {
        let part_number = (i + 1) as i32;
        let resp = client
            .upload_part()
            .bucket(bucket)
            .key(key)
            .upload_id(&upload_id)
            .part_number(part_number)
            .body(ByteStream::from_static(data))
            .send()
            .await
            .unwrap_or_else(|e| panic!("upload_part {part_number}: {e}"));
        completed_parts.push(
            CompletedPart::builder()
                .part_number(part_number)
                .e_tag(resp.e_tag().unwrap_or_default())
                .build(),
        );
    }

    // Verify listed parts count before completing.
    let listed = client
        .list_parts()
        .bucket(bucket)
        .key(key)
        .upload_id(&upload_id)
        .send()
        .await
        .expect("list_parts");
    assert_eq!(listed.parts().len(), 3);

    client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .upload_id(&upload_id)
        .multipart_upload(
            CompletedMultipartUpload::builder()
                .set_parts(Some(completed_parts))
                .build(),
        )
        .send()
        .await
        .expect("complete_multipart_upload");

    // Download the assembled object and verify content.
    let get = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .expect("get_object after multipart");
    let body = get.body.collect().await.unwrap().into_bytes();
    let expected = b"id,name,value\n1,alpha,100\n2,beta,200\n";
    assert_eq!(body.as_ref(), expected);
    assert_eq!(get.content_type.as_deref(), Some("text/csv"));
}

/// Scenario: batch namespace management.
///
/// An analytics platform stores per-day partitioned data under structured
/// prefixes. It uploads 10 objects, verifies listing with pagination, then
/// bulk-deletes them all via delete_objects.
#[tokio::test]
async fn test_batch_prefix_lifecycle() {
    let client = make_s3_client().await;
    let bucket = "analytics-bucket";

    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .expect("create_bucket");

    // Upload 10 daily partition objects.
    let mut keys = Vec::new();
    for day in 1u32..=10 {
        let key = format!("events/2024-01-{day:02}/part-00000.json");
        client
            .put_object()
            .bucket(bucket)
            .key(&key)
            .body(ByteStream::from_static(b"{\"count\":42}"))
            .content_type("application/json")
            .send()
            .await
            .unwrap_or_else(|e| panic!("put {key}: {e}"));
        keys.push(key);
    }

    // List with small page size to exercise pagination.
    let mut total = 0usize;
    let mut continuation: Option<String> = None;
    loop {
        let mut req = client
            .list_objects_v2()
            .bucket(bucket)
            .prefix("events/")
            .max_keys(3);
        if let Some(ref tok) = continuation {
            req = req.continuation_token(tok);
        }
        let page = req.send().await.expect("list_objects_v2 page");
        total += page.contents().len();
        if page.next_continuation_token().is_none() {
            break;
        }
        continuation = page.next_continuation_token().map(str::to_string);
    }
    assert_eq!(total, 10, "all 10 partitions should be listed across pages");

    // Bulk delete all 10 objects in one request.
    let delete_objects: Vec<_> = keys
        .iter()
        .map(|k| {
            aws_sdk_s3::types::ObjectIdentifier::builder()
                .key(k)
                .build()
                .unwrap()
        })
        .collect();
    let del_resp = client
        .delete_objects()
        .bucket(bucket)
        .delete(
            aws_sdk_s3::types::Delete::builder()
                .set_objects(Some(delete_objects))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("delete_objects");
    assert_eq!(del_resp.deleted().len(), 10);
    assert!(del_resp.errors().is_empty(), "no delete errors expected");

    // Confirm the bucket is now empty.
    let after = client
        .list_objects_v2()
        .bucket(bucket)
        .prefix("events/")
        .send()
        .await
        .expect("list after bulk delete");
    assert_eq!(after.key_count(), Some(0));
}

// ---------------------------------------------------------------------------
// Recovery tests — reconstruct state solely from BlobStore sidecars
// ---------------------------------------------------------------------------

/// Build a client + service that share a VFS; after operations are done,
/// construct a fresh `S3Service` over the same VFS and call `recover_with_vfs()`.
/// The sidecars written during the operations are the sole source of truth.
async fn make_recoverable_client() -> (aws_sdk_s3::Client, winterbaume_core::MockAws) {
    let mock = MockAws::builder().build();
    // Share the mock's VFS with the registered S3 service so sidecars survive
    // the original service instance being dropped.
    let svc = S3Service::with_vfs(mock.vfs());
    let mock = MockAws::builder().vfs(mock.vfs()).with_service(svc).build();
    let config = aws_config::defaults(aws_sdk_s3::config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_s3::Client::new(&config);
    (client, mock)
}

/// Scenario: basic recovery — CreateBucket + PutObject; discard state; recover.
#[tokio::test]
async fn test_recover_from_blobs_basic() {
    let (client, mock) = make_recoverable_client().await;

    client
        .create_bucket()
        .bucket("recovery-bucket")
        .send()
        .await
        .expect("create_bucket");

    client
        .put_object()
        .bucket("recovery-bucket")
        .key("hello.txt")
        .body(ByteStream::from_static(b"hello world"))
        .content_type("text/plain")
        .send()
        .await
        .expect("put_object");

    // Build a fresh service over the same VFS and recover state from sidecars.
    let recovery_svc = S3Service::recover_with_vfs(mock.vfs()).await.unwrap();
    let client = make_s3_client_with_service(recovery_svc).await;

    let get = client
        .get_object()
        .bucket("recovery-bucket")
        .key("hello.txt")
        .send()
        .await
        .expect("get_object");
    assert_eq!(get.content_type, Some("text/plain".to_string()));
    assert_eq!(get.content_length, Some(11));
}

/// Scenario: versioned history + delete marker recovery.
#[tokio::test]
async fn test_recover_versioned_history() {
    let (client, mock) = make_recoverable_client().await;

    client
        .create_bucket()
        .bucket("versioned-bucket")
        .send()
        .await
        .expect("create_bucket");

    client
        .put_bucket_versioning()
        .bucket("versioned-bucket")
        .versioning_configuration(
            VersioningConfiguration::builder()
                .status(BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("enable versioning");

    client
        .put_object()
        .bucket("versioned-bucket")
        .key("doc.txt")
        .body(ByteStream::from_static(b"version 1"))
        .send()
        .await
        .expect("put v1");

    client
        .put_object()
        .bucket("versioned-bucket")
        .key("doc.txt")
        .body(ByteStream::from_static(b"version 2"))
        .send()
        .await
        .expect("put v2");

    // Delete without version_id → creates a delete marker.
    client
        .delete_object()
        .bucket("versioned-bucket")
        .key("doc.txt")
        .send()
        .await
        .expect("delete (creates marker)");

    let recovery_svc = S3Service::recover_with_vfs(mock.vfs()).await.unwrap();
    let client = make_s3_client_with_service(recovery_svc).await;

    // History: two versions present.
    let history = client
        .list_object_versions()
        .bucket("versioned-bucket")
        .prefix("doc.txt")
        .send()
        .await
        .expect("list_object_versions");
    assert_eq!(
        history.versions.unwrap().len(),
        2,
        "expected two versions in history"
    );
    assert_eq!(
        history.delete_markers.unwrap().len(),
        1,
        "expected one delete marker"
    );
    let b = client
        .get_bucket_versioning()
        .bucket("versioned-bucket")
        .send()
        .await
        .expect("get_bucket_versioning");
    assert_eq!(
        b.status,
        Some(BucketVersioningStatus::Enabled),
        "versioning status preserved"
    );
}

/// Scenario: bucket config fields are preserved across recovery.
#[tokio::test]
async fn test_recover_bucket_config() {
    use aws_sdk_s3::types::Tag;

    let (client, mock) = make_recoverable_client().await;

    client
        .create_bucket()
        .bucket("config-bucket")
        .send()
        .await
        .expect("create_bucket");

    client
        .put_bucket_versioning()
        .bucket("config-bucket")
        .versioning_configuration(
            VersioningConfiguration::builder()
                .status(BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("enable versioning");

    client
        .put_bucket_tagging()
        .bucket("config-bucket")
        .tagging(
            aws_sdk_s3::types::Tagging::builder()
                .tag_set(Tag::builder().key("env").value("test").build().unwrap())
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_tagging");

    let recovery_svc = S3Service::recover_with_vfs(mock.vfs()).await.unwrap();
    let client = make_s3_client_with_service(recovery_svc).await;

    let b = client
        .get_bucket_versioning()
        .bucket("config-bucket")
        .send()
        .await
        .expect("get_bucket_versioning");
    assert_eq!(
        b.status,
        Some(BucketVersioningStatus::Enabled),
        "versioning status preserved"
    );
    let b = client
        .get_bucket_tagging()
        .bucket("config-bucket")
        .send()
        .await
        .expect("get_bucket_tagging");
    assert_eq!(
        b.tag_set(),
        vec![Tag::builder().key("env").value("test").build().unwrap()],
        "tags preserved"
    );
}
