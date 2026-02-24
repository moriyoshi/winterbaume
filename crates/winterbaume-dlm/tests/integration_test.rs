use std::sync::{Arc, Mutex};

use aws_sdk_dlm::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_dlm::DlmService;

async fn make_client() -> aws_sdk_dlm::Client {
    let mock = MockAws::builder().with_service(DlmService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dlm::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_dlm::Client::new(&config)
}

#[tokio::test]
async fn test_lifecycle_policy_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_lifecycle_policy()
        .description("daily snapshots")
        .execution_role_arn("arn:aws:iam::123:role/dlm-role")
        .send()
        .await
        .expect("create");
    let id = create.policy_id().expect("id").to_string();
    assert!(id.starts_with("policy-"));

    let got = client
        .get_lifecycle_policy()
        .policy_id(&id)
        .send()
        .await
        .expect("get");
    assert_eq!(got.policy().unwrap().description(), Some("daily snapshots"));

    client
        .update_lifecycle_policy()
        .policy_id(&id)
        .description("hourly")
        .send()
        .await
        .expect("update");

    let got2 = client
        .get_lifecycle_policy()
        .policy_id(&id)
        .send()
        .await
        .expect("get2");
    assert_eq!(got2.policy().unwrap().description(), Some("hourly"));

    let list = client.get_lifecycle_policies().send().await.expect("list");
    assert_eq!(list.policies().len(), 1);

    client
        .delete_lifecycle_policy()
        .policy_id(&id)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_get_lifecycle_policy_not_found() {
    let client = make_client().await;
    let err = client
        .get_lifecycle_policy()
        .policy_id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = "arn:aws:dlm:us-east-1:123:policy/seed";
    client
        .tag_resource()
        .resource_arn(arn)
        .tags("Env", "prod")
        .send()
        .await
        .expect("tag");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list");
    assert_eq!(resp.tags().unwrap().get("Env"), Some(&"prod".to_string()));

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
    let svc = DlmService::new();
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
