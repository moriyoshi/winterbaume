use aws_sdk_auditmanager::config::BehaviorVersion;
use winterbaume_auditmanager::AuditManagerService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_auditmanager::Client {
    let mock = MockAws::builder()
        .with_service(AuditManagerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_auditmanager::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_auditmanager::Client::new(&config)
}

// ---- Account status tests ----

#[tokio::test]
async fn test_get_account_status_initial() {
    let client = make_client().await;

    let resp = client
        .get_account_status()
        .send()
        .await
        .expect("get_account_status should succeed");

    assert_eq!(resp.status().map(|s| s.as_str()), Some("INACTIVE"));
}

#[tokio::test]
async fn test_register_and_deregister_account() {
    let client = make_client().await;

    let resp = client
        .register_account()
        .send()
        .await
        .expect("register_account should succeed");
    assert_eq!(resp.status().map(|s| s.as_str()), Some("ACTIVE"));

    let resp = client
        .get_account_status()
        .send()
        .await
        .expect("get_account_status should succeed");
    assert_eq!(resp.status().map(|s| s.as_str()), Some("ACTIVE"));

    let resp = client
        .deregister_account()
        .send()
        .await
        .expect("deregister_account should succeed");
    assert_eq!(resp.status().map(|s| s.as_str()), Some("INACTIVE"));
}

// ---- Control tests ----

#[tokio::test]
async fn test_create_and_get_control() {
    let client = make_client().await;

    let resp = client
        .create_control()
        .name("test-control")
        .description("A test control")
        .send()
        .await
        .expect("create_control should succeed");

    let control = resp.control().expect("should have control");
    assert_eq!(control.name(), Some("test-control"));
    let id = control.id().expect("should have id");

    let resp = client
        .get_control()
        .control_id(id)
        .send()
        .await
        .expect("get_control should succeed");

    let control = resp.control().expect("should have control");
    assert_eq!(control.name(), Some("test-control"));
    assert_eq!(control.description(), Some("A test control"));
}

#[tokio::test]
async fn test_list_controls() {
    let client = make_client().await;

    for name in ["ctrl-a", "ctrl-b", "ctrl-c"] {
        client
            .create_control()
            .name(name)
            .send()
            .await
            .expect("create_control should succeed");
    }

    let resp = client
        .list_controls()
        .control_type(aws_sdk_auditmanager::types::ControlType::Custom)
        .send()
        .await
        .expect("list_controls should succeed");

    assert_eq!(resp.control_metadata_list().len(), 3);
}

#[tokio::test]
async fn test_delete_control() {
    let client = make_client().await;

    let resp = client
        .create_control()
        .name("to-delete")
        .send()
        .await
        .expect("create_control should succeed");
    let id = resp.control().unwrap().id().unwrap().to_string();

    client
        .delete_control()
        .control_id(&id)
        .send()
        .await
        .expect("delete_control should succeed");

    let result = client.get_control().control_id(&id).send().await;
    assert!(result.is_err(), "should error for deleted control");
}

// ---- Framework tests ----

#[tokio::test]
async fn test_create_and_get_assessment_framework() {
    let client = make_client().await;

    let resp = client
        .create_assessment_framework()
        .name("test-framework")
        .description("A test framework")
        .send()
        .await
        .expect("create_assessment_framework should succeed");

    let fw = resp.framework().expect("should have framework");
    assert_eq!(fw.name(), Some("test-framework"));
    let id = fw.id().expect("should have id");

    let resp = client
        .get_assessment_framework()
        .framework_id(id)
        .send()
        .await
        .expect("get_assessment_framework should succeed");

    let fw = resp.framework().expect("should have framework");
    assert_eq!(fw.name(), Some("test-framework"));
}

#[tokio::test]
async fn test_delete_assessment_framework() {
    let client = make_client().await;

    let resp = client
        .create_assessment_framework()
        .name("to-delete-fw")
        .send()
        .await
        .expect("create_assessment_framework should succeed");
    let id = resp.framework().unwrap().id().unwrap().to_string();

    client
        .delete_assessment_framework()
        .framework_id(&id)
        .send()
        .await
        .expect("delete_assessment_framework should succeed");

    let result = client
        .get_assessment_framework()
        .framework_id(&id)
        .send()
        .await;
    assert!(result.is_err(), "should error for deleted framework");
}

// ---- Assessment tests ----

#[tokio::test]
async fn test_create_and_get_assessment() {
    let client = make_client().await;

    // First create a framework
    let fw_resp = client
        .create_assessment_framework()
        .name("fw-for-assessment")
        .send()
        .await
        .expect("create_assessment_framework should succeed");
    let fw_id = fw_resp.framework().unwrap().id().unwrap().to_string();

    // Create assessment scope
    use aws_sdk_auditmanager::types::{
        AssessmentReportDestinationType, AssessmentReportsDestination, AwsAccount, Scope,
    };
    let dest = AssessmentReportsDestination::builder()
        .destination("s3://my-bucket")
        .destination_type(AssessmentReportDestinationType::S3)
        .build();
    let scope = Scope::builder()
        .aws_accounts(AwsAccount::builder().id("123456789012").build())
        .build();

    let resp = client
        .create_assessment()
        .name("test-assessment")
        .description("A test assessment")
        .framework_id(&fw_id)
        .assessment_reports_destination(dest)
        .scope(scope)
        .roles(
            aws_sdk_auditmanager::types::Role::builder()
                .role_type(aws_sdk_auditmanager::types::RoleType::ProcessOwner)
                .role_arn("arn:aws:iam::123456789012:role/test")
                .build()
                .expect("role build should succeed"),
        )
        .send()
        .await
        .expect("create_assessment should succeed");

    let assessment = resp.assessment().expect("should have assessment");
    let metadata = assessment.metadata().expect("should have metadata");
    assert_eq!(metadata.name(), Some("test-assessment"));
    let id = metadata.id().expect("should have id");

    let resp = client
        .get_assessment()
        .assessment_id(id)
        .send()
        .await
        .expect("get_assessment should succeed");

    let assessment = resp.assessment().expect("should have assessment");
    let metadata = assessment.metadata().expect("should have metadata");
    assert_eq!(metadata.name(), Some("test-assessment"));
}

// ---- State view tests ----

#[tokio::test]
async fn test_state_view_snapshot_restore() {
    use std::collections::HashMap;

    use winterbaume_auditmanager::views::{AuditManagerStateView, ControlView};
    use winterbaume_core::StatefulService;

    let svc = AuditManagerService::new();

    // Seed via restore
    let mut controls = HashMap::new();
    controls.insert(
        "ctrl-1".to_string(),
        ControlView {
            id: "ctrl-1".to_string(),
            name: "snap-ctrl".to_string(),
            description: None,
            control_type: "Custom".to_string(),
            created_at: 0.0,
            tags: HashMap::new(),
            control_mapping_sources: vec![],
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        AuditManagerStateView {
            controls,
            ..Default::default()
        },
    )
    .await
    .expect("restore should succeed");

    let view = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view.controls.len(), 1);
    assert!(view.controls.contains_key("ctrl-1"));

    // Restore into new service
    let svc2 = AuditManagerService::new();
    svc2.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore should succeed");

    let view2 = svc2.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view2.controls.len(), 1);
}

// ---- State change listener tests ----

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = AuditManagerService::new();
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

// ============================================================================
// Additional coverage tests
// ============================================================================

#[tokio::test]
async fn test_list_assessment_frameworks() {
    let client = make_client().await;
    for name in ["fw-1", "fw-2", "fw-3"] {
        client
            .create_assessment_framework()
            .name(name)
            .send()
            .await
            .unwrap();
    }
    let resp = client
        .list_assessment_frameworks()
        .framework_type(aws_sdk_auditmanager::types::FrameworkType::Custom)
        .send()
        .await
        .expect("list_assessment_frameworks should succeed");
    assert_eq!(resp.framework_metadata_list().len(), 3);
}

#[tokio::test]
async fn test_list_assessments() {
    let client = make_client().await;

    // Create framework first
    let fw = client
        .create_assessment_framework()
        .name("fw-for-list")
        .send()
        .await
        .unwrap();
    let fw_id = fw.framework().unwrap().id().unwrap().to_string();

    use aws_sdk_auditmanager::types::{
        AssessmentReportDestinationType, AssessmentReportsDestination, AwsAccount, Scope,
    };
    let dest = AssessmentReportsDestination::builder()
        .destination("s3://bucket")
        .destination_type(AssessmentReportDestinationType::S3)
        .build();
    let scope = Scope::builder()
        .aws_accounts(AwsAccount::builder().id("123456789012").build())
        .build();
    let role = aws_sdk_auditmanager::types::Role::builder()
        .role_type(aws_sdk_auditmanager::types::RoleType::ProcessOwner)
        .role_arn("arn:aws:iam::123456789012:role/test")
        .build()
        .unwrap();

    for name in ["assess-1", "assess-2"] {
        client
            .create_assessment()
            .name(name)
            .framework_id(&fw_id)
            .assessment_reports_destination(dest.clone())
            .scope(scope.clone())
            .roles(role.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_assessments()
        .send()
        .await
        .expect("list_assessments should succeed");
    assert_eq!(resp.assessment_metadata().len(), 2);
}

#[tokio::test]
async fn test_delete_assessment() {
    let client = make_client().await;

    let fw = client
        .create_assessment_framework()
        .name("fw-for-del")
        .send()
        .await
        .unwrap();
    let fw_id = fw.framework().unwrap().id().unwrap().to_string();

    use aws_sdk_auditmanager::types::{
        AssessmentReportDestinationType, AssessmentReportsDestination, AwsAccount, Scope,
    };
    let dest = AssessmentReportsDestination::builder()
        .destination("s3://bucket")
        .destination_type(AssessmentReportDestinationType::S3)
        .build();
    let scope = Scope::builder()
        .aws_accounts(AwsAccount::builder().id("123456789012").build())
        .build();
    let role = aws_sdk_auditmanager::types::Role::builder()
        .role_type(aws_sdk_auditmanager::types::RoleType::ProcessOwner)
        .role_arn("arn:aws:iam::123456789012:role/test")
        .build()
        .unwrap();

    let resp = client
        .create_assessment()
        .name("to-del-assessment")
        .framework_id(&fw_id)
        .assessment_reports_destination(dest)
        .scope(scope)
        .roles(role)
        .send()
        .await
        .unwrap();
    let id = resp
        .assessment()
        .unwrap()
        .metadata()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .delete_assessment()
        .assessment_id(&id)
        .send()
        .await
        .expect("delete_assessment should succeed");

    let result = client.get_assessment().assessment_id(&id).send().await;
    assert!(result.is_err(), "should error for deleted assessment");
}

#[tokio::test]
async fn test_get_nonexistent_control() {
    let client = make_client().await;
    let result = client
        .get_control()
        .control_id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_framework() {
    let client = make_client().await;
    let result = client
        .get_assessment_framework()
        .framework_id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_assessment() {
    let client = make_client().await;
    let result = client
        .get_assessment()
        .assessment_id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_control_full_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_control()
        .name("lifecycle-ctrl")
        .description("test lifecycle")
        .send()
        .await
        .unwrap();
    let id = create_resp.control().unwrap().id().unwrap().to_string();

    // Get
    let get_resp = client.get_control().control_id(&id).send().await.unwrap();
    assert_eq!(get_resp.control().unwrap().name(), Some("lifecycle-ctrl"));

    // List
    let list_resp = client
        .list_controls()
        .control_type(aws_sdk_auditmanager::types::ControlType::Custom)
        .send()
        .await
        .unwrap();
    assert!(!list_resp.control_metadata_list().is_empty());

    // Delete
    client
        .delete_control()
        .control_id(&id)
        .send()
        .await
        .unwrap();

    // Verify gone
    assert!(client.get_control().control_id(&id).send().await.is_err());
}
