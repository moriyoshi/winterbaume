use std::sync::{Arc, Mutex};

use aws_sdk_cognitosync::config::BehaviorVersion;
use aws_sdk_cognitosync::types::{Operation as PatchOp, Platform, PushSync, RecordPatch};
use winterbaume_cognitosync::CognitoSyncService;
use winterbaume_core::{MockAws, StatefulService};

const POOL: &str = "us-east-1:abc12345-6789-0abc-def0-123456789012";
const IDENTITY: &str = "us-east-1:abc12345-6789-0abc-def0-aaaaaaaaaaaa";
const DATASET: &str = "my-dataset";

async fn make_client() -> aws_sdk_cognitosync::Client {
    let mock = MockAws::builder()
        .with_service(CognitoSyncService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cognitosync::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_cognitosync::Client::new(&config)
}

#[tokio::test]
async fn test_describe_unseeded_pool() {
    let client = make_client().await;
    let resp = client
        .describe_identity_pool_usage()
        .identity_pool_id(POOL)
        .send()
        .await
        .expect("describe");
    let usage = resp.identity_pool_usage().expect("usage");
    assert_eq!(usage.identity_pool_id(), Some(POOL));
}

#[tokio::test]
async fn test_update_then_list_records() {
    let client = make_client().await;
    client
        .update_records()
        .identity_pool_id(POOL)
        .identity_id(IDENTITY)
        .dataset_name(DATASET)
        .sync_session_token("session-1")
        .record_patches(
            RecordPatch::builder()
                .op(PatchOp::Replace)
                .key("score")
                .value("42")
                .sync_count(0)
                .build()
                .expect("patch"),
        )
        .send()
        .await
        .expect("update");
    let resp = client
        .list_records()
        .identity_pool_id(POOL)
        .identity_id(IDENTITY)
        .dataset_name(DATASET)
        .send()
        .await
        .expect("list");
    let records = resp.records();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].key(), Some("score"));
    assert_eq!(records[0].value(), Some("42"));
    assert_eq!(records[0].sync_count(), Some(1));
    assert!(resp.dataset_exists());
    assert_eq!(resp.dataset_sync_count(), Some(1));
}

#[tokio::test]
async fn test_describe_dataset_after_update() {
    let client = make_client().await;
    client
        .update_records()
        .identity_pool_id(POOL)
        .identity_id(IDENTITY)
        .dataset_name(DATASET)
        .sync_session_token("session-1")
        .record_patches(
            RecordPatch::builder()
                .op(PatchOp::Replace)
                .key("k")
                .value("v")
                .sync_count(0)
                .build()
                .expect("patch"),
        )
        .send()
        .await
        .expect("update");
    let resp = client
        .describe_dataset()
        .identity_pool_id(POOL)
        .identity_id(IDENTITY)
        .dataset_name(DATASET)
        .send()
        .await
        .expect("describe");
    let d = resp.dataset().expect("dataset");
    assert_eq!(d.dataset_name(), Some(DATASET));
    assert_eq!(d.num_records(), Some(1));
}

#[tokio::test]
async fn test_describe_unknown_dataset() {
    let client = make_client().await;
    let err = client
        .describe_dataset()
        .identity_pool_id(POOL)
        .identity_id(IDENTITY)
        .dataset_name("ghost")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_register_device_and_subscribe() {
    let client = make_client().await;
    let resp = client
        .register_device()
        .identity_pool_id(POOL)
        .identity_id(IDENTITY)
        .platform(Platform::Adm)
        .token("push-token")
        .send()
        .await
        .expect("register");
    let device_id = resp.device_id().expect("dev").to_string();
    client
        .update_records()
        .identity_pool_id(POOL)
        .identity_id(IDENTITY)
        .dataset_name(DATASET)
        .sync_session_token("s")
        .record_patches(
            RecordPatch::builder()
                .op(PatchOp::Replace)
                .key("k")
                .value("v")
                .sync_count(0)
                .build()
                .expect("patch"),
        )
        .send()
        .await
        .expect("update");
    client
        .subscribe_to_dataset()
        .identity_pool_id(POOL)
        .identity_id(IDENTITY)
        .dataset_name(DATASET)
        .device_id(&device_id)
        .send()
        .await
        .expect("subscribe");
    client
        .unsubscribe_from_dataset()
        .identity_pool_id(POOL)
        .identity_id(IDENTITY)
        .dataset_name(DATASET)
        .device_id(&device_id)
        .send()
        .await
        .expect("unsubscribe");
}

#[tokio::test]
async fn test_pool_configuration_round_trip() {
    let client = make_client().await;
    let push = PushSync::builder()
        .role_arn("arn:aws:iam::123456789012:role/cognito-push")
        .application_arns("arn:aws:sns:us-east-1:123456789012:app/GCM/myapp")
        .build();
    let resp = client
        .set_identity_pool_configuration()
        .identity_pool_id(POOL)
        .push_sync(push)
        .send()
        .await
        .expect("set");
    assert_eq!(resp.identity_pool_id(), Some(POOL));
    let got = client
        .get_identity_pool_configuration()
        .identity_pool_id(POOL)
        .send()
        .await
        .expect("get");
    assert_eq!(got.identity_pool_id(), Some(POOL));
    let push = got.push_sync().expect("push");
    assert_eq!(
        push.role_arn(),
        Some("arn:aws:iam::123456789012:role/cognito-push")
    );
}

#[tokio::test]
async fn test_bulk_publish_lifecycle() {
    let client = make_client().await;
    let resp = client
        .bulk_publish()
        .identity_pool_id(POOL)
        .send()
        .await
        .expect("bulk");
    assert_eq!(resp.identity_pool_id(), Some(POOL));
    let details = client
        .get_bulk_publish_details()
        .identity_pool_id(POOL)
        .send()
        .await
        .expect("details");
    assert_eq!(
        details.bulk_publish_status().map(|s| s.as_str()),
        Some("SUCCEEDED")
    );
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = CognitoSyncService::new();
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
        .expect("restore");
    assert_eq!(events.lock().unwrap().len(), 1);
}
