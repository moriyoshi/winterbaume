use aws_sdk_fis::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_fis::{FisService, FisStateView};

async fn make_client() -> (aws_sdk_fis::Client, FisService) {
    let svc = FisService::new();
    let mock = MockAws::builder().with_service(FisService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_fis::config::Region::new("us-east-1"))
        .load()
        .await;
    (aws_sdk_fis::Client::new(&config), svc)
}

async fn make_simple_client() -> aws_sdk_fis::Client {
    let mock = MockAws::builder().with_service(FisService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_fis::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_fis::Client::new(&config)
}

// -------------------------------------------------------------------------
// Experiment Template tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_experiment_template() {
    let client = make_simple_client().await;

    let create_resp = client
        .create_experiment_template()
        .description("test template")
        .role_arn("arn:aws:iam::123456789012:role/FISRole")
        .actions(
            "action1",
            aws_sdk_fis::types::CreateExperimentTemplateActionInput::builder()
                .action_id("aws:ec2:stop-instances")
                .build()
                .unwrap(),
        )
        .stop_conditions(
            aws_sdk_fis::types::CreateExperimentTemplateStopConditionInput::builder()
                .source("none")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_experiment_template should succeed");

    let template = create_resp
        .experiment_template()
        .expect("template should be present");
    assert_eq!(template.description(), Some("test template"));
    assert!(template.id().unwrap_or("").starts_with("EXT"));

    // Get
    let id = template.id().unwrap();
    let get_resp = client
        .get_experiment_template()
        .id(id)
        .send()
        .await
        .expect("get_experiment_template should succeed");

    let got = get_resp
        .experiment_template()
        .expect("template should be present");
    assert_eq!(got.id(), template.id());
    assert_eq!(got.description(), Some("test template"));
    assert_eq!(
        got.role_arn(),
        Some("arn:aws:iam::123456789012:role/FISRole")
    );
}

#[tokio::test]
async fn test_list_experiment_templates() {
    let client = make_simple_client().await;

    // Create two templates
    client
        .create_experiment_template()
        .description("template 1")
        .role_arn("arn:aws:iam::123456789012:role/FISRole")
        .actions(
            "action1",
            aws_sdk_fis::types::CreateExperimentTemplateActionInput::builder()
                .action_id("aws:ec2:stop-instances")
                .build()
                .unwrap(),
        )
        .stop_conditions(
            aws_sdk_fis::types::CreateExperimentTemplateStopConditionInput::builder()
                .source("none")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create should succeed");

    client
        .create_experiment_template()
        .description("template 2")
        .role_arn("arn:aws:iam::123456789012:role/FISRole")
        .actions(
            "action1",
            aws_sdk_fis::types::CreateExperimentTemplateActionInput::builder()
                .action_id("aws:ec2:reboot-instances")
                .build()
                .unwrap(),
        )
        .stop_conditions(
            aws_sdk_fis::types::CreateExperimentTemplateStopConditionInput::builder()
                .source("none")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create should succeed");

    let list_resp = client
        .list_experiment_templates()
        .send()
        .await
        .expect("list_experiment_templates should succeed");

    assert_eq!(list_resp.experiment_templates().len(), 2);
}

#[tokio::test]
async fn test_delete_experiment_template() {
    let client = make_simple_client().await;

    let create_resp = client
        .create_experiment_template()
        .description("to delete")
        .role_arn("arn:aws:iam::123456789012:role/FISRole")
        .actions(
            "action1",
            aws_sdk_fis::types::CreateExperimentTemplateActionInput::builder()
                .action_id("aws:ec2:stop-instances")
                .build()
                .unwrap(),
        )
        .stop_conditions(
            aws_sdk_fis::types::CreateExperimentTemplateStopConditionInput::builder()
                .source("none")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let id = create_resp
        .experiment_template()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Delete
    let del_resp = client
        .delete_experiment_template()
        .id(&id)
        .send()
        .await
        .expect("delete should succeed");

    let deleted = del_resp.experiment_template().unwrap();
    assert_eq!(deleted.id(), Some(id.as_str()));

    // Verify gone
    let err = client.get_experiment_template().id(&id).send().await;
    assert!(err.is_err(), "should fail to get deleted template");
}

#[tokio::test]
async fn test_update_experiment_template() {
    let client = make_simple_client().await;

    let create_resp = client
        .create_experiment_template()
        .description("original")
        .role_arn("arn:aws:iam::123456789012:role/FISRole")
        .actions(
            "action1",
            aws_sdk_fis::types::CreateExperimentTemplateActionInput::builder()
                .action_id("aws:ec2:stop-instances")
                .build()
                .unwrap(),
        )
        .stop_conditions(
            aws_sdk_fis::types::CreateExperimentTemplateStopConditionInput::builder()
                .source("none")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let id = create_resp
        .experiment_template()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_experiment_template()
        .id(&id)
        .description("updated description")
        .role_arn("arn:aws:iam::123456789012:role/NewRole")
        .send()
        .await
        .expect("update should succeed");

    let updated = update_resp.experiment_template().unwrap();
    assert_eq!(updated.description(), Some("updated description"));
    assert_eq!(
        updated.role_arn(),
        Some("arn:aws:iam::123456789012:role/NewRole")
    );
}

// -------------------------------------------------------------------------
// Lifecycle test
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_experiment_template_lifecycle() {
    let client = make_simple_client().await;

    // Create
    let create_resp = client
        .create_experiment_template()
        .description("lifecycle test")
        .role_arn("arn:aws:iam::123456789012:role/FISRole")
        .actions(
            "stop-ec2",
            aws_sdk_fis::types::CreateExperimentTemplateActionInput::builder()
                .action_id("aws:ec2:stop-instances")
                .description("Stop EC2 instances")
                .build()
                .unwrap(),
        )
        .targets(
            "instances",
            aws_sdk_fis::types::CreateExperimentTemplateTargetInput::builder()
                .resource_type("aws:ec2:instance")
                .selection_mode("ALL")
                .build()
                .unwrap(),
        )
        .stop_conditions(
            aws_sdk_fis::types::CreateExperimentTemplateStopConditionInput::builder()
                .source("none")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let template = create_resp.experiment_template().unwrap();
    let id = template.id().unwrap().to_string();
    assert_eq!(template.description(), Some("lifecycle test"));

    // Describe
    let get_resp = client
        .get_experiment_template()
        .id(&id)
        .send()
        .await
        .unwrap();
    let got = get_resp.experiment_template().unwrap();
    assert_eq!(got.description(), Some("lifecycle test"));

    // Update
    client
        .update_experiment_template()
        .id(&id)
        .description("updated lifecycle")
        .send()
        .await
        .unwrap();

    // Verify update
    let get2 = client
        .get_experiment_template()
        .id(&id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get2.experiment_template().unwrap().description(),
        Some("updated lifecycle")
    );

    // Delete
    client
        .delete_experiment_template()
        .id(&id)
        .send()
        .await
        .unwrap();

    // Verify gone
    assert!(
        client
            .get_experiment_template()
            .id(&id)
            .send()
            .await
            .is_err()
    );
}

// -------------------------------------------------------------------------
// Error path tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_get_nonexistent_template() {
    let client = make_simple_client().await;

    let err = client
        .get_experiment_template()
        .id("EXTnonexistent")
        .send()
        .await;
    assert!(err.is_err(), "should fail for nonexistent template");
}

#[tokio::test]
async fn test_delete_nonexistent_template() {
    let client = make_simple_client().await;

    let err = client
        .delete_experiment_template()
        .id("EXTnonexistent")
        .send()
        .await;
    assert!(err.is_err(), "should fail for nonexistent template");
}

#[tokio::test]
async fn test_update_nonexistent_template() {
    let client = make_simple_client().await;

    let err = client
        .update_experiment_template()
        .id("EXTnonexistent")
        .description("nope")
        .send()
        .await;
    assert!(err.is_err(), "should fail for nonexistent template");
}

// -------------------------------------------------------------------------
// Tag tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_tags_lifecycle() {
    let client = make_simple_client().await;

    // Create with tags
    let create_resp = client
        .create_experiment_template()
        .description("tag test")
        .role_arn("arn:aws:iam::123456789012:role/FISRole")
        .actions(
            "action1",
            aws_sdk_fis::types::CreateExperimentTemplateActionInput::builder()
                .action_id("aws:ec2:stop-instances")
                .build()
                .unwrap(),
        )
        .stop_conditions(
            aws_sdk_fis::types::CreateExperimentTemplateStopConditionInput::builder()
                .source("none")
                .build()
                .unwrap(),
        )
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .unwrap();

    let template = create_resp.experiment_template().unwrap();
    let arn = template.arn().unwrap().to_string();
    let tags = template.tags().unwrap();
    assert_eq!(tags.get("env"), Some(&"test".to_string()));
    assert_eq!(tags.get("team"), Some(&"platform".to_string()));

    // List tags
    let list_tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let listed = list_tags_resp.tags().unwrap();
    assert_eq!(listed.get("env"), Some(&"test".to_string()));

    // Add tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("new-tag", "new-value")
        .send()
        .await
        .expect("tag_resource should succeed");

    let list2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        list2.tags().unwrap().get("new-tag"),
        Some(&"new-value".to_string())
    );

    // Remove tags
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag should succeed");

    let list3 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(list3.tags().unwrap().get("env").is_none());
    assert_eq!(
        list3.tags().unwrap().get("team"),
        Some(&"platform".to_string())
    );
}

// -------------------------------------------------------------------------
// State view tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_state_view_snapshot_and_restore() {
    use winterbaume_core::StatefulService;

    let svc = FisService::new();

    // Start with empty state
    let empty = svc.snapshot("123456789012", "us-east-1").await;
    assert!(empty.experiment_templates.is_empty());

    // Restore with a view that has a template
    let mut view = FisStateView::default();
    view.experiment_templates.insert(
        "EXTtest123".to_string(),
        winterbaume_fis::views::ExperimentTemplateView {
            id: "EXTtest123".to_string(),
            arn: "arn:aws:fis:us-east-1:123456789012:experiment-template/EXTtest123".to_string(),
            description: "restored template".to_string(),
            role_arn: "arn:aws:iam::123456789012:role/FISRole".to_string(),
            targets: Default::default(),
            actions: Default::default(),
            stop_conditions: vec![],
            tags: Default::default(),
            creation_time: "2025-01-01T00:00:00+00:00".to_string(),
            last_update_time: "2025-01-01T00:00:00+00:00".to_string(),
        },
    );

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot.experiment_templates.len(), 1);
    assert!(snapshot.experiment_templates.contains_key("EXTtest123"));
    assert_eq!(
        snapshot.experiment_templates["EXTtest123"].description,
        "restored template"
    );
}

#[tokio::test]
async fn test_state_view_merge() {
    use winterbaume_core::StatefulService;

    let svc = FisService::new();

    // Restore initial state
    let mut view1 = FisStateView::default();
    view1.experiment_templates.insert(
        "EXTexisting".to_string(),
        winterbaume_fis::views::ExperimentTemplateView {
            id: "EXTexisting".to_string(),
            arn: "arn:aws:fis:us-east-1:123456789012:experiment-template/EXTexisting".to_string(),
            description: "existing template".to_string(),
            role_arn: "arn:aws:iam::123456789012:role/FISRole".to_string(),
            targets: Default::default(),
            actions: Default::default(),
            stop_conditions: vec![],
            tags: Default::default(),
            creation_time: "2025-01-01T00:00:00+00:00".to_string(),
            last_update_time: "2025-01-01T00:00:00+00:00".to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view1)
        .await
        .unwrap();

    // Merge a new template
    let mut view2 = FisStateView::default();
    view2.experiment_templates.insert(
        "EXTmerged".to_string(),
        winterbaume_fis::views::ExperimentTemplateView {
            id: "EXTmerged".to_string(),
            arn: "arn:aws:fis:us-east-1:123456789012:experiment-template/EXTmerged".to_string(),
            description: "merged template".to_string(),
            role_arn: "arn:aws:iam::123456789012:role/FISRole".to_string(),
            targets: Default::default(),
            actions: Default::default(),
            stop_conditions: vec![],
            tags: Default::default(),
            creation_time: "2025-01-01T00:00:00+00:00".to_string(),
            last_update_time: "2025-01-01T00:00:00+00:00".to_string(),
        },
    );
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    // Both should be present
    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot.experiment_templates.len(), 2);
    assert!(snapshot.experiment_templates.contains_key("EXTexisting"));
    assert!(snapshot.experiment_templates.contains_key("EXTmerged"));
}

// -------------------------------------------------------------------------
// State change notification tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = FisService::new();
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

    use winterbaume_core::StatefulService;

    let svc = FisService::new();

    // Pre-seed
    let mut view = FisStateView::default();
    view.experiment_templates.insert(
        "EXTseed".to_string(),
        winterbaume_fis::views::ExperimentTemplateView {
            id: "EXTseed".to_string(),
            arn: "arn:aws:fis:us-east-1:123456789012:experiment-template/EXTseed".to_string(),
            description: "seed".to_string(),
            role_arn: "arn:aws:iam::123456789012:role/FISRole".to_string(),
            targets: Default::default(),
            actions: Default::default(),
            stop_conditions: vec![],
            tags: Default::default(),
            creation_time: "2025-01-01T00:00:00+00:00".to_string(),
            last_update_time: "2025-01-01T00:00:00+00:00".to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Register listener
    let snapshots: Arc<Mutex<Vec<FisStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    // Trigger mutation via restore with different state
    let mut view2 = FisStateView::default();
    view2.experiment_templates.insert(
        "EXTnew".to_string(),
        winterbaume_fis::views::ExperimentTemplateView {
            id: "EXTnew".to_string(),
            arn: "arn:aws:fis:us-east-1:123456789012:experiment-template/EXTnew".to_string(),
            description: "new".to_string(),
            role_arn: "arn:aws:iam::123456789012:role/FISRole".to_string(),
            targets: Default::default(),
            actions: Default::default(),
            stop_conditions: vec![],
            tags: Default::default(),
            creation_time: "2025-01-01T00:00:00+00:00".to_string(),
            last_update_time: "2025-01-01T00:00:00+00:00".to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view2)
        .await
        .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].experiment_templates.contains_key("EXTnew"));
    assert!(!got[0].experiment_templates.contains_key("EXTseed"));
}
