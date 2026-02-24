use aws_sdk_cloudcontrol::config::BehaviorVersion;
use winterbaume_cloudcontrol::CloudControlService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_cloudcontrol::Client {
    let mock = MockAws::builder()
        .with_service(CloudControlService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudcontrol::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_cloudcontrol::Client::new(&config)
}

fn make_service() -> CloudControlService {
    CloudControlService::new()
}

// ---------------------------------------------------------------------------
// CreateResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_resource() {
    let client = make_client().await;
    let resp = client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"my-bucket"}"#)
        .send()
        .await
        .expect("create_resource should succeed");

    let event = resp.progress_event().expect("should have progress event");
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );
    assert_eq!(event.operation().map(|s| s.as_str()), Some("CREATE"));
    assert_eq!(event.type_name(), Some("AWS::S3::Bucket"));
    assert_eq!(event.identifier(), Some("my-bucket"));
    assert!(event.request_token().is_some());
}

// ---------------------------------------------------------------------------
// GetResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_resource_after_create() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"test-bucket"}"#)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .get_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("test-bucket")
        .send()
        .await
        .expect("get_resource should succeed");

    assert_eq!(resp.type_name(), Some("AWS::S3::Bucket"));
    let desc = resp
        .resource_description()
        .expect("should have description");
    assert_eq!(desc.identifier(), Some("test-bucket"));
    assert!(desc.properties().is_some());
}

#[tokio::test]
async fn test_get_resource_not_found() {
    let client = make_client().await;
    let err = client
        .get_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("nonexistent")
        .send()
        .await
        .expect_err("should fail for nonexistent resource");

    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ---------------------------------------------------------------------------
// DeleteResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_delete_resource() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"del-bucket"}"#)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .delete_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("del-bucket")
        .send()
        .await
        .expect("delete_resource should succeed");

    let event = resp.progress_event().expect("should have progress event");
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );
    assert_eq!(event.operation().map(|s| s.as_str()), Some("DELETE"));

    // Verify it's gone
    let err = client
        .get_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("del-bucket")
        .send()
        .await
        .expect_err("should fail after deletion");

    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

#[tokio::test]
async fn test_delete_resource_not_found() {
    let client = make_client().await;
    let err = client
        .delete_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("nonexistent")
        .send()
        .await
        .expect_err("should fail for nonexistent resource");

    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ---------------------------------------------------------------------------
// UpdateResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_update_resource() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"upd-bucket","Tags":[]}"#)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .update_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("upd-bucket")
        .patch_document(r#"[{"op":"add","path":"/Tags","value":[{"Key":"env","Value":"test"}]}]"#)
        .send()
        .await
        .expect("update_resource should succeed");

    let event = resp.progress_event().expect("should have progress event");
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );
    assert_eq!(event.operation().map(|s| s.as_str()), Some("UPDATE"));

    // Verify the update took effect
    let get_resp = client
        .get_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("upd-bucket")
        .send()
        .await
        .expect("get should succeed after update");

    let desc = get_resp.resource_description().unwrap();
    let props = desc.properties().unwrap();
    assert!(props.contains("env"));
}

#[tokio::test]
async fn test_update_resource_not_found() {
    let client = make_client().await;
    let err = client
        .update_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("nonexistent")
        .patch_document(r#"[{"op":"add","path":"/Tag","value":"x"}]"#)
        .send()
        .await
        .expect_err("should fail for nonexistent resource");

    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ---------------------------------------------------------------------------
// ListResources
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_resources() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::Lambda::Function")
        .desired_state(r#"{"FunctionName":"fn-a"}"#)
        .send()
        .await
        .expect("create a should succeed");

    client
        .create_resource()
        .type_name("AWS::Lambda::Function")
        .desired_state(r#"{"FunctionName":"fn-b"}"#)
        .send()
        .await
        .expect("create b should succeed");

    // Create a resource of a different type
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"bkt"}"#)
        .send()
        .await
        .expect("create bucket should succeed");

    let resp = client
        .list_resources()
        .type_name("AWS::Lambda::Function")
        .send()
        .await
        .expect("list_resources should succeed");

    assert_eq!(resp.type_name(), Some("AWS::Lambda::Function"));
    let descriptions = resp.resource_descriptions();
    assert_eq!(descriptions.len(), 2);
}

// ---------------------------------------------------------------------------
// GetResourceRequestStatus
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_resource_request_status() {
    let client = make_client().await;
    let create_resp = client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"status-test"}"#)
        .send()
        .await
        .expect("create should succeed");

    let token = create_resp
        .progress_event()
        .unwrap()
        .request_token()
        .unwrap()
        .to_string();

    let resp = client
        .get_resource_request_status()
        .request_token(&token)
        .send()
        .await
        .expect("get_resource_request_status should succeed");

    let event = resp.progress_event().expect("should have progress event");
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );
    assert_eq!(event.operation().map(|s| s.as_str()), Some("CREATE"));
    assert_eq!(event.request_token(), Some(token.as_str()));
}

#[tokio::test]
async fn test_get_resource_request_status_not_found() {
    let client = make_client().await;
    let err = client
        .get_resource_request_status()
        .request_token("nonexistent-token")
        .send()
        .await
        .expect_err("should fail for nonexistent token");

    let service_err = err.into_service_error();
    assert!(service_err.is_request_token_not_found_exception());
}

// ---------------------------------------------------------------------------
// ListResourceRequests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_resource_requests() {
    let client = make_client().await;

    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"req-list-1"}"#)
        .send()
        .await
        .expect("create should succeed");

    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"req-list-2"}"#)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .list_resource_requests()
        .send()
        .await
        .expect("list_resource_requests should succeed");

    let summaries = resp.resource_request_status_summaries();
    assert!(summaries.len() >= 2);
}

// ---------------------------------------------------------------------------
// CancelResourceRequest (error path - already completed)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_cancel_resource_request_already_complete() {
    let client = make_client().await;
    let create_resp = client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"cancel-test"}"#)
        .send()
        .await
        .expect("create should succeed");

    let token = create_resp
        .progress_event()
        .unwrap()
        .request_token()
        .unwrap()
        .to_string();

    // The operation completed synchronously (SUCCESS), so cancellation should fail.
    let err = client
        .cancel_resource_request()
        .request_token(&token)
        .send()
        .await
        .expect_err("should fail because op is already complete");

    // The SDK maps the error type; just check it's an error.
    assert!(err.into_service_error().meta().message().is_some());
}

// ---------------------------------------------------------------------------
// CreateResource - AlreadyExists
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_resource_already_exists() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"dup-bucket"}"#)
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"dup-bucket"}"#)
        .send()
        .await
        .expect_err("second create should fail");

    let service_err = err.into_service_error();
    assert!(service_err.is_already_exists_exception());
}

// ---------------------------------------------------------------------------
// Full lifecycle: create -> get -> update -> get -> delete -> verify gone
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_full_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_resource()
        .type_name("AWS::DynamoDB::Table")
        .desired_state(
            r#"{"TableName":"my-table","KeySchema":[{"AttributeName":"pk","KeyType":"HASH"}]}"#,
        )
        .send()
        .await
        .expect("create should succeed");
    let event = create_resp.progress_event().unwrap();
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );

    // Get
    let get_resp = client
        .get_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .send()
        .await
        .expect("get should succeed");
    let desc = get_resp.resource_description().unwrap();
    assert_eq!(desc.identifier(), Some("my-table"));

    // Update
    client
        .update_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .patch_document(r#"[{"op":"add","path":"/BillingMode","value":"PAY_PER_REQUEST"}]"#)
        .send()
        .await
        .expect("update should succeed");

    // Get after update
    let get_resp2 = client
        .get_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .send()
        .await
        .expect("get after update should succeed");
    let props = get_resp2
        .resource_description()
        .unwrap()
        .properties()
        .unwrap();
    assert!(props.contains("PAY_PER_REQUEST"));

    // Delete
    let del_resp = client
        .delete_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .send()
        .await
        .expect("delete should succeed");
    let del_event = del_resp.progress_event().unwrap();
    assert_eq!(
        del_event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );

    // Verify gone
    let err = client
        .get_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .send()
        .await
        .expect_err("should fail after deletion");
    assert!(err.into_service_error().is_resource_not_found_exception());
}

// ---------------------------------------------------------------------------
// State views: snapshot, restore, merge
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_state_snapshot_restore() {
    use winterbaume_cloudcontrol::views::{CloudControlStateView, ResourceView};
    use winterbaume_core::StatefulService;

    let svc = make_service();

    // Seed state via restore
    let mut view = CloudControlStateView::default();
    view.resources.insert(
        "AWS::S3::Bucket|snap-bucket".to_string(),
        ResourceView {
            type_name: "AWS::S3::Bucket".to_string(),
            identifier: "snap-bucket".to_string(),
            resource_model: r#"{"BucketName":"snap-bucket"}"#.to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Snapshot
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.resources.contains_key("AWS::S3::Bucket|snap-bucket"));

    // Restore to empty
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();
    let empty = svc.snapshot("123456789012", "us-east-1").await;
    assert!(empty.resources.is_empty());

    // Restore from snapshot
    svc.restore("123456789012", "us-east-1", snap)
        .await
        .unwrap();
    let restored = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        restored
            .resources
            .contains_key("AWS::S3::Bucket|snap-bucket")
    );
}

#[tokio::test]
async fn test_state_merge_additive() {
    use winterbaume_cloudcontrol::views::{CloudControlStateView, ResourceView};
    use winterbaume_core::StatefulService;

    let svc = make_service();

    // Seed existing resource
    let mut initial = CloudControlStateView::default();
    initial.resources.insert(
        "AWS::S3::Bucket|existing-bucket".to_string(),
        ResourceView {
            type_name: "AWS::S3::Bucket".to_string(),
            identifier: "existing-bucket".to_string(),
            resource_model: r#"{"BucketName":"existing-bucket"}"#.to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", initial)
        .await
        .unwrap();

    // Merge a new resource
    let mut merge_view = CloudControlStateView::default();
    merge_view.resources.insert(
        "AWS::Lambda::Function|merged-fn".to_string(),
        ResourceView {
            type_name: "AWS::Lambda::Function".to_string(),
            identifier: "merged-fn".to_string(),
            resource_model: r#"{"FunctionName":"merged-fn"}"#.to_string(),
        },
    );
    svc.merge("123456789012", "us-east-1", merge_view)
        .await
        .unwrap();

    // Both should exist
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snap.resources
            .contains_key("AWS::S3::Bucket|existing-bucket")
    );
    assert!(
        snap.resources
            .contains_key("AWS::Lambda::Function|merged-fn")
    );
}

// ---------------------------------------------------------------------------
// State change notifications
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = CloudControlService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::sync::{Arc, Mutex};

    use winterbaume_cloudcontrol::views::{CloudControlStateView, ResourceView};
    use winterbaume_core::StatefulService;

    let svc = CloudControlService::new();

    // Pre-seed with a resource
    let mut view = CloudControlStateView::default();
    view.resources.insert(
        "AWS::S3::Bucket|notify-bucket".to_string(),
        ResourceView {
            type_name: "AWS::S3::Bucket".to_string(),
            identifier: "notify-bucket".to_string(),
            resource_model: r#"{"BucketName":"notify-bucket"}"#.to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Subscribe and capture snapshot
    let snapshots: Arc<Mutex<Vec<CloudControlStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    // Merge another resource
    let mut view2 = CloudControlStateView::default();
    view2.resources.insert(
        "AWS::Lambda::Function|notify-fn".to_string(),
        ResourceView {
            type_name: "AWS::Lambda::Function".to_string(),
            identifier: "notify-fn".to_string(),
            resource_model: r#"{"FunctionName":"notify-fn"}"#.to_string(),
        },
    );
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(
        got[0]
            .resources
            .contains_key("AWS::Lambda::Function|notify-fn")
    );
    assert!(
        got[0]
            .resources
            .contains_key("AWS::S3::Bucket|notify-bucket")
    );
}
