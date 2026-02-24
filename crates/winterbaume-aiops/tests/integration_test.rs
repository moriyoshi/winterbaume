use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use aws_sdk_aiops::config::BehaviorVersion;
use winterbaume_aiops::AIOpsService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_aiops::Client {
    let mock = MockAws::builder().with_service(AIOpsService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_aiops::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_aiops::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_investigation_group() {
    let client = make_client().await;

    let create_resp = client
        .create_investigation_group()
        .name("test-group")
        .role_arn("arn:aws:iam::123456789012:role/aiops-role")
        .retention_in_days(30)
        .send()
        .await
        .expect("create_investigation_group should succeed");

    let arn = create_resp.arn().expect("arn must be returned");
    assert!(arn.contains("investigation-group/test-group"));

    let get_resp = client
        .get_investigation_group()
        .identifier("test-group")
        .send()
        .await
        .expect("get_investigation_group should succeed");

    assert_eq!(get_resp.name(), Some("test-group"));
    assert_eq!(
        get_resp.role_arn(),
        Some("arn:aws:iam::123456789012:role/aiops-role")
    );
    assert_eq!(get_resp.retention_in_days(), Some(30));
}

#[tokio::test]
async fn test_create_duplicate_investigation_group_fails() {
    let client = make_client().await;
    client
        .create_investigation_group()
        .name("dup-group")
        .role_arn("arn:aws:iam::123456789012:role/aiops")
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_investigation_group()
        .name("dup-group")
        .role_arn("arn:aws:iam::123456789012:role/aiops")
        .send()
        .await
        .expect_err("second create should fail");
    let msg = format!("{:?}", err);
    assert!(msg.contains("ConflictException"), "got: {msg}");
}

#[tokio::test]
async fn test_get_investigation_group_not_found() {
    let client = make_client().await;
    let err = client
        .get_investigation_group()
        .identifier("missing")
        .send()
        .await
        .expect_err("should fail");
    let msg = format!("{:?}", err);
    assert!(msg.contains("ResourceNotFoundException"), "got: {msg}");
}

#[tokio::test]
async fn test_update_investigation_group() {
    let client = make_client().await;
    client
        .create_investigation_group()
        .name("upd-group")
        .role_arn("arn:aws:iam::123456789012:role/role-a")
        .send()
        .await
        .expect("create");

    client
        .update_investigation_group()
        .identifier("upd-group")
        .role_arn("arn:aws:iam::123456789012:role/role-b")
        .is_cloud_trail_event_history_enabled(true)
        .send()
        .await
        .expect("update");

    let resp = client
        .get_investigation_group()
        .identifier("upd-group")
        .send()
        .await
        .expect("get");
    assert_eq!(
        resp.role_arn(),
        Some("arn:aws:iam::123456789012:role/role-b")
    );
    assert_eq!(resp.is_cloud_trail_event_history_enabled(), Some(true));
}

#[tokio::test]
async fn test_delete_investigation_group_lifecycle() {
    let client = make_client().await;
    client
        .create_investigation_group()
        .name("del-group")
        .role_arn("arn:aws:iam::123456789012:role/aiops")
        .send()
        .await
        .expect("create");

    client
        .delete_investigation_group()
        .identifier("del-group")
        .send()
        .await
        .expect("delete");

    let err = client
        .get_investigation_group()
        .identifier("del-group")
        .send()
        .await
        .expect_err("get after delete should fail");
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_list_investigation_groups() {
    let client = make_client().await;
    for n in ["a-group", "b-group", "c-group"] {
        client
            .create_investigation_group()
            .name(n)
            .role_arn("arn:aws:iam::123456789012:role/aiops")
            .send()
            .await
            .expect("create");
    }

    let resp = client
        .list_investigation_groups()
        .send()
        .await
        .expect("list");
    let groups = resp.investigation_groups();
    assert_eq!(groups.len(), 3);
    let names: Vec<&str> = groups.iter().filter_map(|g| g.name()).collect();
    assert_eq!(names, vec!["a-group", "b-group", "c-group"]);
}

#[tokio::test]
async fn test_investigation_group_policy_lifecycle() {
    let client = make_client().await;
    client
        .create_investigation_group()
        .name("pol-group")
        .role_arn("arn:aws:iam::123456789012:role/aiops")
        .send()
        .await
        .expect("create");

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[]}"#;

    let put_resp = client
        .put_investigation_group_policy()
        .identifier("pol-group")
        .policy(policy_doc)
        .send()
        .await
        .expect("put policy");
    let arn = put_resp
        .investigation_group_arn()
        .expect("arn returned")
        .to_string();
    assert!(arn.contains("pol-group"));

    let get_resp = client
        .get_investigation_group_policy()
        .identifier("pol-group")
        .send()
        .await
        .expect("get policy");
    assert_eq!(get_resp.policy(), Some(policy_doc));

    client
        .delete_investigation_group_policy()
        .identifier("pol-group")
        .send()
        .await
        .expect("delete policy");

    let err = client
        .get_investigation_group_policy()
        .identifier("pol-group")
        .send()
        .await
        .expect_err("get after delete should fail");
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_tag_resource_lifecycle() {
    let client = make_client().await;
    let create_resp = client
        .create_investigation_group()
        .name("tag-group")
        .role_arn("arn:aws:iam::123456789012:role/aiops")
        .send()
        .await
        .expect("create");
    let arn = create_resp.arn().expect("arn").to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    let tags = resp.tags().expect("tags map");
    assert_eq!(tags.get("env"), Some(&"prod".to_string()));
    assert_eq!(tags.get("team"), Some(&"platform".to_string()));

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
        .expect("list tags");
    let tags = resp.tags().expect("tags map");
    assert!(tags.get("env").is_none());
    assert_eq!(tags.get("team"), Some(&"platform".to_string()));
}

#[tokio::test]
async fn test_get_by_arn() {
    let client = make_client().await;
    let create_resp = client
        .create_investigation_group()
        .name("arn-group")
        .role_arn("arn:aws:iam::123456789012:role/aiops")
        .send()
        .await
        .expect("create");
    let arn = create_resp.arn().expect("arn").to_string();

    let resp = client
        .get_investigation_group()
        .identifier(&arn)
        .send()
        .await
        .expect("get by arn");
    assert_eq!(resp.name(), Some("arn-group"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = AIOpsService::new();
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
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_snapshot_reflects_mutation() {
    use winterbaume_aiops::views::InvestigationGroupView;

    let svc = AIOpsService::new();

    let mut view = winterbaume_aiops::AIOpsStateView::default();
    view.investigation_groups.insert(
        "snap-group".to_string(),
        InvestigationGroupView {
            name: "snap-group".to_string(),
            arn: "arn:aws:aiops:us-east-1:123456789012:investigation-group/snap-group".to_string(),
            role_arn: "arn:aws:iam::123456789012:role/aiops".to_string(),
            encryption_configuration: None,
            retention_in_days: Some(7),
            chatbot_notification_channel: None,
            tag_key_boundaries: None,
            is_cloud_trail_event_history_enabled: None,
            cross_account_configurations: None,
            created_by: "snap-user".to_string(),
            created_at: 0,
            last_modified_by: "snap-user".to_string(),
            last_modified_at: 0,
            policy: None,
            tags: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("seed");

    let snapshots: Arc<Mutex<Vec<winterbaume_aiops::AIOpsStateView>>> =
        Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    let mut view2 = winterbaume_aiops::AIOpsStateView::default();
    view2.investigation_groups.insert(
        "snap-group-2".to_string(),
        InvestigationGroupView {
            name: "snap-group-2".to_string(),
            arn: "arn:aws:aiops:us-east-1:123456789012:investigation-group/snap-group-2"
                .to_string(),
            role_arn: "arn:aws:iam::123456789012:role/aiops".to_string(),
            encryption_configuration: None,
            retention_in_days: None,
            chatbot_notification_channel: None,
            tag_key_boundaries: None,
            is_cloud_trail_event_history_enabled: None,
            cross_account_configurations: None,
            created_by: "snap-user".to_string(),
            created_at: 0,
            last_modified_by: "snap-user".to_string(),
            last_modified_at: 0,
            policy: None,
            tags: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view2)
        .await
        .expect("restore2");

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].investigation_groups.contains_key("snap-group-2"));
}
