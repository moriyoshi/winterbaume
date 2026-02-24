use std::sync::{Arc, Mutex};

use aws_sdk_s3outposts::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_s3outposts::S3OutpostsService;

async fn make_client() -> aws_sdk_s3outposts::Client {
    let mock = MockAws::builder()
        .with_service(S3OutpostsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3outposts::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_s3outposts::Client::new(&config)
}

#[tokio::test]
async fn test_endpoint_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_endpoint()
        .outpost_id("op-default")
        .subnet_id("subnet-aaa")
        .security_group_id("sg-bbb")
        .send()
        .await
        .expect("create");
    let arn = create.endpoint_arn().expect("arn").to_string();
    assert!(arn.contains("/endpoint/"));
    let endpoint_id = arn.rsplit('/').next().unwrap().to_string();

    let list = client.list_endpoints().send().await.expect("list");
    assert_eq!(list.endpoints().len(), 1);

    client
        .delete_endpoint()
        .endpoint_id(&endpoint_id)
        .outpost_id("op-default")
        .send()
        .await
        .expect("delete");

    let list2 = client.list_endpoints().send().await.expect("list2");
    assert_eq!(list2.endpoints().len(), 0);
}

#[tokio::test]
async fn test_delete_endpoint_not_found() {
    let client = make_client().await;
    let err = client
        .delete_endpoint()
        .endpoint_id("missing")
        .outpost_id("op-default")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_list_outposts_seeded_default() {
    let client = make_client().await;
    let resp = client
        .list_outposts_with_s3()
        .send()
        .await
        .expect("list_outposts");
    assert_eq!(resp.outposts().len(), 1);
    assert_eq!(resp.outposts()[0].outpost_id(), Some("op-default"));
}

#[tokio::test]
async fn test_list_shared_endpoints_empty() {
    let client = make_client().await;
    let resp = client
        .list_shared_endpoints()
        .outpost_id("op-default")
        .send()
        .await
        .expect("list_shared");
    assert_eq!(resp.endpoints().len(), 0);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = S3OutpostsService::new();
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
