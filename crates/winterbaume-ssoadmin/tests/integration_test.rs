//! Integration tests for the SSO Admin service.

use aws_sdk_ssoadmin::config::BehaviorVersion;
use aws_sdk_ssoadmin::types::CustomerManagedPolicyReference;
use winterbaume_core::MockAws;
use winterbaume_ssoadmin::SsoAdminService;

const INSTANCE_ARN: &str = "arn:aws:sso:::instance/ssoins-0123456789abcdef";

async fn make_client() -> aws_sdk_ssoadmin::Client {
    let mock = MockAws::builder()
        .with_service(SsoAdminService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ssoadmin::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_ssoadmin::Client::new(&config)
}

// ---- ListInstances ----

#[tokio::test]
async fn test_list_instances() {
    let client = make_client().await;
    let resp = client
        .list_instances()
        .send()
        .await
        .expect("list_instances should succeed");
    let instances = resp.instances();
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].instance_arn(), Some(INSTANCE_ARN));
    assert_eq!(instances[0].identity_store_id(), Some("d-0123456789"));
}

// ---- PermissionSet lifecycle ----

#[tokio::test]
async fn test_create_and_describe_permission_set() {
    let client = make_client().await;
    let resp = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("TestPS")
        .description("A test permission set")
        .session_duration("PT4H")
        .send()
        .await
        .expect("create_permission_set should succeed");

    let ps = resp
        .permission_set()
        .expect("permission_set should be present");
    let arn = ps.permission_set_arn().expect("arn should be present");
    assert_eq!(ps.name(), Some("TestPS"));
    assert_eq!(ps.description(), Some("A test permission set"));

    let describe_resp = client
        .describe_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(arn)
        .send()
        .await
        .expect("describe_permission_set should succeed");
    let described = describe_resp
        .permission_set()
        .expect("permission_set present");
    assert_eq!(described.name(), Some("TestPS"));
    assert_eq!(described.session_duration(), Some("PT4H"));
}

#[tokio::test]
async fn test_list_permission_sets() {
    let client = make_client().await;
    client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("PS-List-1")
        .send()
        .await
        .expect("create_permission_set should succeed");
    client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("PS-List-2")
        .send()
        .await
        .expect("create_permission_set should succeed");

    let resp = client
        .list_permission_sets()
        .instance_arn(INSTANCE_ARN)
        .send()
        .await
        .expect("list_permission_sets should succeed");
    assert_eq!(resp.permission_sets().len(), 2);
}

#[tokio::test]
async fn test_delete_permission_set() {
    let client = make_client().await;
    let arn = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("ToDelete")
        .send()
        .await
        .expect("create_permission_set should succeed")
        .permission_set()
        .unwrap()
        .permission_set_arn()
        .unwrap()
        .to_string();

    client
        .delete_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .expect("delete_permission_set should succeed");

    let result = client
        .describe_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await;
    assert!(result.is_err(), "should fail after deletion");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("404"),
        "unexpected error: {err_str}"
    );
}

#[tokio::test]
async fn test_update_permission_set() {
    let client = make_client().await;
    let arn = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("UpdateMe")
        .send()
        .await
        .unwrap()
        .permission_set()
        .unwrap()
        .permission_set_arn()
        .unwrap()
        .to_string();

    client
        .update_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .description("Updated description")
        .session_duration("PT8H")
        .send()
        .await
        .expect("update_permission_set should succeed");

    let resp = client
        .describe_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .unwrap();
    let ps = resp.permission_set().unwrap();
    assert_eq!(ps.description(), Some("Updated description"));
    assert_eq!(ps.session_duration(), Some("PT8H"));
}

// ---- AccountAssignment lifecycle ----

#[tokio::test]
async fn test_create_and_list_account_assignments() {
    let client = make_client().await;
    let arn = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("PS-AA")
        .send()
        .await
        .unwrap()
        .permission_set()
        .unwrap()
        .permission_set_arn()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_account_assignment()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .principal_type(aws_sdk_ssoadmin::types::PrincipalType::User)
        .principal_id("user-001")
        .target_id("123456789012")
        .target_type(aws_sdk_ssoadmin::types::TargetType::AwsAccount)
        .send()
        .await
        .expect("create_account_assignment should succeed");

    let status = create_resp
        .account_assignment_creation_status()
        .expect("status present");
    assert_eq!(status.status().map(|s| s.as_str()), Some("SUCCEEDED"));
    let request_id = status.request_id().unwrap().to_string();

    // DescribeAccountAssignmentCreationStatus
    let describe_resp = client
        .describe_account_assignment_creation_status()
        .instance_arn(INSTANCE_ARN)
        .account_assignment_creation_request_id(&request_id)
        .send()
        .await
        .expect("describe status should succeed");
    let described = describe_resp.account_assignment_creation_status().unwrap();
    assert_eq!(described.status().map(|s| s.as_str()), Some("SUCCEEDED"));

    // ListAccountAssignments
    let list_resp = client
        .list_account_assignments()
        .instance_arn(INSTANCE_ARN)
        .account_id("123456789012")
        .permission_set_arn(&arn)
        .send()
        .await
        .expect("list_account_assignments should succeed");
    assert_eq!(list_resp.account_assignments().len(), 1);
    assert_eq!(
        list_resp.account_assignments()[0].principal_id(),
        Some("user-001")
    );
}

#[tokio::test]
async fn test_delete_account_assignment() {
    let client = make_client().await;
    let arn = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("PS-Del-AA")
        .send()
        .await
        .unwrap()
        .permission_set()
        .unwrap()
        .permission_set_arn()
        .unwrap()
        .to_string();

    client
        .create_account_assignment()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .principal_type(aws_sdk_ssoadmin::types::PrincipalType::User)
        .principal_id("user-to-remove")
        .target_id("111111111111")
        .target_type(aws_sdk_ssoadmin::types::TargetType::AwsAccount)
        .send()
        .await
        .unwrap();

    let del_resp = client
        .delete_account_assignment()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .principal_type(aws_sdk_ssoadmin::types::PrincipalType::User)
        .principal_id("user-to-remove")
        .target_id("111111111111")
        .target_type(aws_sdk_ssoadmin::types::TargetType::AwsAccount)
        .send()
        .await
        .expect("delete_account_assignment should succeed");

    let status = del_resp.account_assignment_deletion_status().unwrap();
    assert_eq!(status.status().map(|s| s.as_str()), Some("SUCCEEDED"));

    // Verify it's gone
    let list_resp = client
        .list_account_assignments()
        .instance_arn(INSTANCE_ARN)
        .account_id("111111111111")
        .permission_set_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(list_resp.account_assignments().is_empty());
}

#[tokio::test]
async fn test_list_accounts_for_provisioned_permission_set() {
    let client = make_client().await;
    let arn = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("PS-Accounts")
        .send()
        .await
        .unwrap()
        .permission_set()
        .unwrap()
        .permission_set_arn()
        .unwrap()
        .to_string();

    for account in ["222222222222", "333333333333"] {
        client
            .create_account_assignment()
            .instance_arn(INSTANCE_ARN)
            .permission_set_arn(&arn)
            .principal_type(aws_sdk_ssoadmin::types::PrincipalType::User)
            .principal_id("userX")
            .target_id(account)
            .target_type(aws_sdk_ssoadmin::types::TargetType::AwsAccount)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_accounts_for_provisioned_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .expect("list_accounts_for_provisioned_permission_set should succeed");
    let ids = resp.account_ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&"222222222222".to_string()));
    assert!(ids.contains(&"333333333333".to_string()));
}

#[tokio::test]
async fn test_list_permission_sets_provisioned_to_account() {
    let client = make_client().await;
    let acct = "444444444444";

    for ps_name in ["PS-Prov-1", "PS-Prov-2"] {
        let arn = client
            .create_permission_set()
            .instance_arn(INSTANCE_ARN)
            .name(ps_name)
            .send()
            .await
            .unwrap()
            .permission_set()
            .unwrap()
            .permission_set_arn()
            .unwrap()
            .to_string();

        client
            .create_account_assignment()
            .instance_arn(INSTANCE_ARN)
            .permission_set_arn(&arn)
            .principal_type(aws_sdk_ssoadmin::types::PrincipalType::User)
            .principal_id("u1")
            .target_id(acct)
            .target_type(aws_sdk_ssoadmin::types::TargetType::AwsAccount)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_permission_sets_provisioned_to_account()
        .instance_arn(INSTANCE_ARN)
        .account_id(acct)
        .send()
        .await
        .expect("list_permission_sets_provisioned_to_account should succeed");
    assert_eq!(resp.permission_sets().len(), 2);
}

// ---- Managed Policy operations ----

#[tokio::test]
async fn test_attach_detach_managed_policy() {
    let client = make_client().await;
    let arn = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("PS-MP")
        .send()
        .await
        .unwrap()
        .permission_set()
        .unwrap()
        .permission_set_arn()
        .unwrap()
        .to_string();

    let policy_arn = "arn:aws:iam::aws:policy/ReadOnlyAccess";

    client
        .attach_managed_policy_to_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .managed_policy_arn(policy_arn)
        .send()
        .await
        .expect("attach_managed_policy should succeed");

    let list_resp = client
        .list_managed_policies_in_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .expect("list_managed_policies should succeed");
    let policies = list_resp.attached_managed_policies();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].arn(), Some(policy_arn));

    client
        .detach_managed_policy_from_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .managed_policy_arn(policy_arn)
        .send()
        .await
        .expect("detach_managed_policy should succeed");

    let list_resp2 = client
        .list_managed_policies_in_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(list_resp2.attached_managed_policies().is_empty());
}

// ---- Customer-managed policy operations ----

#[tokio::test]
async fn test_attach_detach_customer_managed_policy() {
    let client = make_client().await;
    let arn = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("PS-CMP")
        .send()
        .await
        .unwrap()
        .permission_set()
        .unwrap()
        .permission_set_arn()
        .unwrap()
        .to_string();

    let cmp_ref = CustomerManagedPolicyReference::builder()
        .name("MyCustomPolicy")
        .path("/custom/")
        .build()
        .unwrap();

    client
        .attach_customer_managed_policy_reference_to_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .customer_managed_policy_reference(cmp_ref.clone())
        .send()
        .await
        .expect("attach_customer_managed_policy should succeed");

    let list_resp = client
        .list_customer_managed_policy_references_in_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .expect("list_customer_managed_policies should succeed");
    let refs = list_resp.customer_managed_policy_references();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].name(), "MyCustomPolicy");

    client
        .detach_customer_managed_policy_reference_from_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .customer_managed_policy_reference(cmp_ref)
        .send()
        .await
        .expect("detach_customer_managed_policy should succeed");

    let list_resp2 = client
        .list_customer_managed_policy_references_in_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(list_resp2.customer_managed_policy_references().is_empty());
}

// ---- Inline Policy operations ----

#[tokio::test]
async fn test_inline_policy_lifecycle() {
    let client = make_client().await;
    let arn = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("PS-Inline")
        .send()
        .await
        .unwrap()
        .permission_set()
        .unwrap()
        .permission_set_arn()
        .unwrap()
        .to_string();

    let policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;

    client
        .put_inline_policy_to_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .inline_policy(policy)
        .send()
        .await
        .expect("put_inline_policy should succeed");

    let get_resp = client
        .get_inline_policy_for_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .expect("get_inline_policy should succeed");
    assert_eq!(get_resp.inline_policy(), Some(policy));

    client
        .delete_inline_policy_from_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .expect("delete_inline_policy should succeed");

    let get_resp2 = client
        .get_inline_policy_for_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(get_resp2.inline_policy().is_none());
}

// ---- Tag operations ----

#[tokio::test]
async fn test_tag_resource_lifecycle() {
    let client = make_client().await;
    let arn = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("PS-Tags")
        .send()
        .await
        .unwrap()
        .permission_set()
        .unwrap()
        .permission_set_arn()
        .unwrap()
        .to_string();

    let tag = aws_sdk_ssoadmin::types::Tag::builder()
        .key("Env")
        .value("test")
        .build()
        .unwrap();

    client
        .tag_resource()
        .instance_arn(INSTANCE_ARN)
        .resource_arn(&arn)
        .tags(tag)
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .instance_arn(INSTANCE_ARN)
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    let tags = list_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "Env");
    assert_eq!(tags[0].value(), "test");

    client
        .untag_resource()
        .instance_arn(INSTANCE_ARN)
        .resource_arn(&arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp2 = client
        .list_tags_for_resource()
        .instance_arn(INSTANCE_ARN)
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(list_resp2.tags().is_empty());
}

// ---- Error paths ----

#[tokio::test]
async fn test_describe_nonexistent_permission_set() {
    let client = make_client().await;
    let result = client
        .describe_permission_set()
        .instance_arn(INSTANCE_ARN)
        .permission_set_arn("arn:aws:sso:::permissionSet/nonexistent/abc123")
        .send()
        .await;
    assert!(
        result.is_err(),
        "should fail for nonexistent permission set"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("404"),
        "unexpected error: {err_str}"
    );
}

#[tokio::test]
async fn test_duplicate_permission_set_name_fails() {
    let client = make_client().await;
    client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("DupPS")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_permission_set()
        .instance_arn(INSTANCE_ARN)
        .name("DupPS")
        .send()
        .await;
    assert!(result.is_err(), "duplicate name should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ConflictException") || err_str.contains("409"),
        "unexpected error: {err_str}"
    );
}

// ---- State view tests ----

#[tokio::test]
async fn test_snapshot_and_restore() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_ssoadmin::SsoAdminStateView;
    use winterbaume_ssoadmin::views::PermissionSetView;

    let svc = SsoAdminService::new();

    // Pre-seed a permission set via restore
    let mut ps_map = HashMap::new();
    ps_map.insert(
        "arn:aws:sso:::permissionSet/test/001".to_string(),
        PermissionSetView {
            permission_set_arn: "arn:aws:sso:::permissionSet/test/001".to_string(),
            name: "Restored".to_string(),
            description: None,
            session_duration: None,
            relay_state: None,
            inline_policy: None,
            managed_policies: vec![],
            customer_managed_policies: vec![],
            tags: HashMap::new(),
            created_date: 0.0,
        },
    );
    let view = SsoAdminStateView {
        permission_sets: ps_map,
        account_assignments: vec![],
        assignment_statuses: HashMap::new(),
    };
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot
            .permission_sets
            .contains_key("arn:aws:sso:::permissionSet/test/001")
    );
    assert_eq!(
        snapshot.permission_sets["arn:aws:sso:::permissionSet/test/001"].name,
        "Restored"
    );
}

#[tokio::test]
async fn test_merge_is_additive() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_ssoadmin::SsoAdminStateView;
    use winterbaume_ssoadmin::views::PermissionSetView;

    let svc = SsoAdminService::new();

    // Restore first PS
    let mut ps_map1 = HashMap::new();
    ps_map1.insert(
        "arn:aws:sso:::permissionSet/test/001".to_string(),
        PermissionSetView {
            permission_set_arn: "arn:aws:sso:::permissionSet/test/001".to_string(),
            name: "First".to_string(),
            description: None,
            session_duration: None,
            relay_state: None,
            inline_policy: None,
            managed_policies: vec![],
            customer_managed_policies: vec![],
            tags: HashMap::new(),
            created_date: 0.0,
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        SsoAdminStateView {
            permission_sets: ps_map1,
            account_assignments: vec![],
            assignment_statuses: HashMap::new(),
        },
    )
    .await
    .unwrap();

    // Merge second PS
    let mut ps_map2 = HashMap::new();
    ps_map2.insert(
        "arn:aws:sso:::permissionSet/test/002".to_string(),
        PermissionSetView {
            permission_set_arn: "arn:aws:sso:::permissionSet/test/002".to_string(),
            name: "Second".to_string(),
            description: None,
            session_duration: None,
            relay_state: None,
            inline_policy: None,
            managed_policies: vec![],
            customer_managed_policies: vec![],
            tags: HashMap::new(),
            created_date: 0.0,
        },
    );
    svc.merge(
        "123456789012",
        "us-east-1",
        SsoAdminStateView {
            permission_sets: ps_map2,
            account_assignments: vec![],
            assignment_statuses: HashMap::new(),
        },
    )
    .await
    .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(
        snapshot.permission_sets.len(),
        2,
        "merge should preserve existing PS"
    );
}

// ---- StateChangeNotifier tests ----

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = SsoAdminService::new();
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
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_ssoadmin::SsoAdminStateView;
    use winterbaume_ssoadmin::views::PermissionSetView;

    let svc = SsoAdminService::new();

    // Pre-seed
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();

    let snapshots: Arc<Mutex<Vec<SsoAdminStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    let mut ps_map = HashMap::new();
    ps_map.insert(
        "arn:aws:sso:::permissionSet/test/snap".to_string(),
        PermissionSetView {
            permission_set_arn: "arn:aws:sso:::permissionSet/test/snap".to_string(),
            name: "SnapPS".to_string(),
            description: None,
            session_duration: None,
            relay_state: None,
            inline_policy: None,
            managed_policies: vec![],
            customer_managed_policies: vec![],
            tags: HashMap::new(),
            created_date: 0.0,
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        SsoAdminStateView {
            permission_sets: ps_map,
            account_assignments: vec![],
            assignment_statuses: HashMap::new(),
        },
    )
    .await
    .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(
        got[0]
            .permission_sets
            .contains_key("arn:aws:sso:::permissionSet/test/snap"),
        "snapshot should reflect restored state"
    );
}
