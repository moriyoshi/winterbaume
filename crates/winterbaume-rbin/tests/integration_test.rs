use std::sync::{Arc, Mutex};

use aws_sdk_rbin::config::BehaviorVersion;
use aws_sdk_rbin::types::{LockConfiguration, ResourceType, RetentionPeriod, UnlockDelay};
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_rbin::RbinService;

async fn make_client() -> aws_sdk_rbin::Client {
    let mock = MockAws::builder().with_service(RbinService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rbin::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_rbin::Client::new(&config)
}

#[tokio::test]
async fn test_rule_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_rule()
        .description("daily snapshots")
        .resource_type(ResourceType::EbsSnapshot)
        .retention_period(
            RetentionPeriod::builder()
                .retention_period_value(30)
                .retention_period_unit("DAYS".into())
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create");
    let id = create.identifier().expect("id").to_string();
    assert!(id.starts_with("rule-"));

    let got = client.get_rule().identifier(&id).send().await.expect("get");
    assert_eq!(got.description(), Some("daily snapshots"));

    client
        .update_rule()
        .identifier(&id)
        .description("weekly")
        .send()
        .await
        .expect("update");

    let list = client
        .list_rules()
        .resource_type(ResourceType::EbsSnapshot)
        .send()
        .await
        .expect("list");
    assert_eq!(list.rules().len(), 1);

    client
        .delete_rule()
        .identifier(&id)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_lock_unlock_rule() {
    let client = make_client().await;
    let create = client
        .create_rule()
        .resource_type(ResourceType::EbsSnapshot)
        .retention_period(
            RetentionPeriod::builder()
                .retention_period_value(7)
                .retention_period_unit("DAYS".into())
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create");
    let id = create.identifier().unwrap().to_string();

    let lock = client
        .lock_rule()
        .identifier(&id)
        .lock_configuration(
            LockConfiguration::builder()
                .unlock_delay(
                    UnlockDelay::builder()
                        .unlock_delay_value(7)
                        .unlock_delay_unit("DAYS".into())
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("lock");
    assert_eq!(lock.lock_state().map(|s| s.as_str()), Some("locked"));

    // Cannot delete a locked rule.
    let err = client
        .delete_rule()
        .identifier(&id)
        .send()
        .await
        .expect_err("locked");
    assert!(format!("{err:?}").contains("ConflictException"));

    let unlock = client
        .unlock_rule()
        .identifier(&id)
        .send()
        .await
        .expect("unlock");
    assert!(unlock.lock_end_time().is_some());
}

#[tokio::test]
async fn test_get_rule_not_found() {
    let client = make_client().await;
    let err = client
        .get_rule()
        .identifier("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = "arn:aws:rbin:us-east-1:123:rule/seed";
    client
        .tag_resource()
        .resource_arn(arn)
        .tags(
            aws_sdk_rbin::types::Tag::builder()
                .key("Env")
                .value("prod")
                .build()
                .unwrap(),
        )
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
    let svc = RbinService::new();
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
