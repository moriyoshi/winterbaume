use std::sync::{Arc, Mutex};

use aws_sdk_supportapp::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_supportapp::SupportAppService;

async fn make_client() -> aws_sdk_supportapp::Client {
    let mock = MockAws::builder()
        .with_service(SupportAppService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_supportapp::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_supportapp::Client::new(&config)
}

#[tokio::test]
async fn test_account_alias_round_trip() {
    let client = make_client().await;
    client
        .put_account_alias()
        .account_alias("my-account")
        .send()
        .await
        .expect("put");

    let resp = client.get_account_alias().send().await.expect("get");
    assert_eq!(resp.account_alias(), Some("my-account"));

    client.delete_account_alias().send().await.expect("delete");
    let resp = client.get_account_alias().send().await.expect("get2");
    assert_eq!(resp.account_alias(), None);
}

#[tokio::test]
async fn test_channel_lifecycle() {
    let client = make_client().await;
    client
        .create_slack_channel_configuration()
        .team_id("T1")
        .channel_id("C1")
        .channel_name("ops")
        .channel_role_arn("arn:aws:iam::123:role/SupportRole")
        .notify_on_case_severity("all".into())
        .notify_on_create_or_reopen_case(true)
        .send()
        .await
        .expect("create");

    let list = client
        .list_slack_channel_configurations()
        .send()
        .await
        .expect("list");
    assert_eq!(list.slack_channel_configurations().len(), 1);
    let cfg = &list.slack_channel_configurations()[0];
    assert_eq!(cfg.channel_id(), "C1");
    assert_eq!(cfg.team_id(), "T1");

    let updated = client
        .update_slack_channel_configuration()
        .team_id("T1")
        .channel_id("C1")
        .notify_on_resolve_case(true)
        .send()
        .await
        .expect("update");
    assert_eq!(updated.notify_on_resolve_case(), Some(true));

    client
        .delete_slack_channel_configuration()
        .team_id("T1")
        .channel_id("C1")
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_duplicate_channel_returns_conflict() {
    let client = make_client().await;
    let do_create = || async {
        client
            .create_slack_channel_configuration()
            .team_id("T2")
            .channel_id("C2")
            .channel_role_arn("arn:aws:iam::123:role/SupportRole")
            .notify_on_case_severity("none".into())
            .send()
            .await
    };
    do_create().await.expect("create 1");
    let err = do_create().await.expect_err("dup");
    assert!(format!("{err:?}").contains("ConflictException"));
}

#[tokio::test]
async fn test_workspace_register_and_delete() {
    let client = make_client().await;
    client
        .register_slack_workspace_for_organization()
        .team_id("T3")
        .send()
        .await
        .expect("register");

    let list = client
        .list_slack_workspace_configurations()
        .send()
        .await
        .expect("list");
    assert_eq!(list.slack_workspace_configurations().len(), 1);

    client
        .delete_slack_workspace_configuration()
        .team_id("T3")
        .send()
        .await
        .expect("delete");

    let list = client
        .list_slack_workspace_configurations()
        .send()
        .await
        .expect("list2");
    assert_eq!(list.slack_workspace_configurations().len(), 0);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = SupportAppService::new();
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
