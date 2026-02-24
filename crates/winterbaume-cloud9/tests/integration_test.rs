use std::sync::{Arc, Mutex};

use aws_sdk_cloud9::config::BehaviorVersion;
use aws_sdk_cloud9::types::Tag;
use winterbaume_cloud9::Cloud9Service;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_cloud9::Client {
    let mock = MockAws::builder()
        .with_service(Cloud9Service::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloud9::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_cloud9::Client::new(&config)
}

#[tokio::test]
async fn test_environment_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_environment_ec2()
        .name("dev-env")
        .image_id("amazonlinux-2023-x86_64")
        .instance_type("t3.small")
        .description("My dev environment")
        .send()
        .await
        .expect("create");
    let id = create.environment_id().expect("id").to_string();

    let list = client.list_environments().send().await.expect("list");
    assert_eq!(list.environment_ids().len(), 1);

    let desc = client
        .describe_environments()
        .environment_ids(&id)
        .send()
        .await
        .expect("describe");
    assert_eq!(desc.environments().len(), 1);
    assert_eq!(desc.environments()[0].name(), Some("dev-env"));

    let status = client
        .describe_environment_status()
        .environment_id(&id)
        .send()
        .await
        .expect("status");
    assert_eq!(status.status().as_str(), "ready");

    client
        .update_environment()
        .environment_id(&id)
        .description("Updated description")
        .send()
        .await
        .expect("update");

    client
        .delete_environment()
        .environment_id(&id)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_membership_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_environment_ec2()
        .name("shared")
        .image_id("amazonlinux-2023-x86_64")
        .instance_type("t3.small")
        .send()
        .await
        .expect("create");
    let id = create.environment_id().unwrap().to_string();

    client
        .create_environment_membership()
        .environment_id(&id)
        .user_arn("arn:aws:iam::123:user/alice")
        .permissions("read-write".into())
        .send()
        .await
        .expect("create membership");

    let list = client
        .describe_environment_memberships()
        .environment_id(&id)
        .send()
        .await
        .expect("list memberships");
    // 2 memberships: owner (auto-added) + alice
    assert_eq!(list.memberships().len(), 2);

    client
        .update_environment_membership()
        .environment_id(&id)
        .user_arn("arn:aws:iam::123:user/alice")
        .permissions("read-only".into())
        .send()
        .await
        .expect("update membership");

    client
        .delete_environment_membership()
        .environment_id(&id)
        .user_arn("arn:aws:iam::123:user/alice")
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_describe_environment_status_not_found() {
    let client = make_client().await;
    let err = client
        .describe_environment_status()
        .environment_id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("NotFoundException"));
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = "arn:aws:cloud9:us-east-1:123:environment:abc";
    client
        .tag_resource()
        .resource_arn(arn)
        .tags(Tag::builder().key("Env").value("dev").build().unwrap())
        .send()
        .await
        .expect("tag");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list");
    assert_eq!(resp.tags().len(), 1);

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = Cloud9Service::new();
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
