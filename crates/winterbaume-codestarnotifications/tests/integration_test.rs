use std::sync::{Arc, Mutex};

use aws_sdk_codestarnotifications::config::BehaviorVersion;
use aws_sdk_codestarnotifications::types::{DetailType, NotificationRuleStatus, Target};
use winterbaume_codestarnotifications::CodeStarNotificationsService;
use winterbaume_core::{MockAws, StatefulService};

const RESOURCE_ARN: &str = "arn:aws:codecommit:us-east-1:123456789012:my-repo";
const TOPIC_ARN: &str = "arn:aws:sns:us-east-1:123456789012:notify-topic";

async fn make_client() -> aws_sdk_codestarnotifications::Client {
    let mock = MockAws::builder()
        .with_service(CodeStarNotificationsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codestarnotifications::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_codestarnotifications::Client::new(&config)
}

async fn create_rule(client: &aws_sdk_codestarnotifications::Client) -> String {
    let target = Target::builder()
        .target_type("SNS")
        .target_address(TOPIC_ARN)
        .build();
    let resp = client
        .create_notification_rule()
        .name("my-rule")
        .resource(RESOURCE_ARN)
        .detail_type(DetailType::Basic)
        .event_type_ids("codecommit-repository-pull-request-created")
        .targets(target)
        .status(NotificationRuleStatus::Enabled)
        .send()
        .await
        .expect("create");
    resp.arn().expect("arn").to_string()
}

#[tokio::test]
async fn test_create_describe_list() {
    let client = make_client().await;
    let arn = create_rule(&client).await;
    let described = client
        .describe_notification_rule()
        .arn(&arn)
        .send()
        .await
        .expect("describe");
    assert_eq!(described.arn(), arn.as_str());
    assert_eq!(described.name(), Some("my-rule"));
    assert_eq!(described.resource(), Some(RESOURCE_ARN));
    assert_eq!(described.targets().len(), 1);
    assert_eq!(described.targets()[0].target_address(), Some(TOPIC_ARN));

    let listed = client.list_notification_rules().send().await.expect("list");
    assert_eq!(listed.notification_rules().len(), 1);
    assert_eq!(listed.notification_rules()[0].arn(), Some(arn.as_str()));
}

#[tokio::test]
async fn test_subscribe_and_unsubscribe() {
    let client = make_client().await;
    let arn = create_rule(&client).await;

    let target = Target::builder()
        .target_type("SNS")
        .target_address("arn:aws:sns:us-east-1:123456789012:topic-2")
        .build();
    let resp = client
        .subscribe()
        .arn(&arn)
        .target(target)
        .send()
        .await
        .expect("subscribe");
    assert_eq!(resp.arn(), Some(arn.as_str()));

    let described = client
        .describe_notification_rule()
        .arn(&arn)
        .send()
        .await
        .expect("describe");
    assert_eq!(described.targets().len(), 2);

    client
        .unsubscribe()
        .arn(&arn)
        .target_address("arn:aws:sns:us-east-1:123456789012:topic-2")
        .send()
        .await
        .expect("unsubscribe");
    let described = client
        .describe_notification_rule()
        .arn(&arn)
        .send()
        .await
        .expect("describe");
    assert_eq!(described.targets().len(), 1);
}

#[tokio::test]
async fn test_update_rule() {
    let client = make_client().await;
    let arn = create_rule(&client).await;
    client
        .update_notification_rule()
        .arn(&arn)
        .name("renamed-rule")
        .status(NotificationRuleStatus::Disabled)
        .send()
        .await
        .expect("update");
    let described = client
        .describe_notification_rule()
        .arn(&arn)
        .send()
        .await
        .expect("describe");
    assert_eq!(described.name(), Some("renamed-rule"));
    assert_eq!(described.status().map(|s| s.as_str()), Some("DISABLED"));
}

#[tokio::test]
async fn test_delete_rule() {
    let client = make_client().await;
    let arn = create_rule(&client).await;
    client
        .delete_notification_rule()
        .arn(&arn)
        .send()
        .await
        .expect("delete");
    let err = client
        .describe_notification_rule()
        .arn(&arn)
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_list_event_types_seeded() {
    let client = make_client().await;
    let resp = client.list_event_types().send().await.expect("list");
    assert!(!resp.event_types().is_empty());
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = create_rule(&client).await;
    client
        .tag_resource()
        .arn(&arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("tag");
    let listed = client
        .list_tags_for_resource()
        .arn(&arn)
        .send()
        .await
        .expect("list");
    assert_eq!(
        listed.tags().and_then(|t| t.get("env")),
        Some(&"prod".to_string())
    );

    client
        .untag_resource()
        .arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag");
    let listed = client
        .list_tags_for_resource()
        .arn(&arn)
        .send()
        .await
        .expect("list");
    assert!(listed.tags().map(|t| t.is_empty()).unwrap_or(true));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = CodeStarNotificationsService::new();
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
