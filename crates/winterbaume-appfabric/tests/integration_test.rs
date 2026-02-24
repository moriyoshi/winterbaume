use std::sync::{Arc, Mutex};

use aws_sdk_appfabric::config::BehaviorVersion;
use winterbaume_appfabric::AppFabricService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_appfabric::Client {
    let mock = MockAws::builder()
        .with_service(AppFabricService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appfabric::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_appfabric::Client::new(&config)
}

#[tokio::test]
async fn test_app_bundle_lifecycle() {
    let client = make_client().await;
    let resp = client
        .create_app_bundle()
        .send()
        .await
        .expect("create_app_bundle");
    let arn = resp
        .app_bundle()
        .map(|b| b.arn())
        .expect("arn returned")
        .to_string();
    assert!(arn.contains("appbundle/"));

    let get = client
        .get_app_bundle()
        .app_bundle_identifier(&arn)
        .send()
        .await
        .expect("get_app_bundle");
    assert_eq!(get.app_bundle().map(|b| b.arn()), Some(arn.as_str()));

    client
        .delete_app_bundle()
        .app_bundle_identifier(&arn)
        .send()
        .await
        .expect("delete_app_bundle");

    let err = client
        .get_app_bundle()
        .app_bundle_identifier(&arn)
        .send()
        .await
        .expect_err("get after delete should fail");
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_create_with_cmk_and_tags() {
    let client = make_client().await;
    let resp = client
        .create_app_bundle()
        .customer_managed_key_identifier("arn:aws:kms:us-east-1:123456789012:key/my-key")
        .tags(
            aws_sdk_appfabric::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_app_bundle");
    let bundle = resp.app_bundle().expect("bundle");
    assert_eq!(
        bundle.customer_managed_key_arn(),
        Some("arn:aws:kms:us-east-1:123456789012:key/my-key")
    );
}

#[tokio::test]
async fn test_list_app_bundles() {
    let client = make_client().await;
    for _ in 0..3 {
        client.create_app_bundle().send().await.expect("create");
    }
    let resp = client.list_app_bundles().send().await.expect("list");
    assert_eq!(resp.app_bundle_summary_list().len(), 3);
}

#[tokio::test]
async fn test_get_app_bundle_not_found() {
    let client = make_client().await;
    let err = client
        .get_app_bundle()
        .app_bundle_identifier("arn:aws:appfabric:us-east-1:123456789012:appbundle/missing")
        .send()
        .await
        .expect_err("missing should fail");
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let create = client.create_app_bundle().send().await.expect("create");
    let arn = create
        .app_bundle()
        .map(|b| b.arn())
        .expect("arn")
        .to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_appfabric::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_appfabric::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags");
    let tags = resp.tags();
    assert_eq!(tags.len(), 2);

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags");
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "team");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = AppFabricService::new();
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
    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
}
