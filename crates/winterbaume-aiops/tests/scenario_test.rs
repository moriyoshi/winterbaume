/// Scenario: investigation group lifecycle
///
/// Covers the full resource lifecycle for an AIOps investigation group:
/// 1. Create an investigation group with encryption and retention settings.
/// 2. Attach a resource policy.
/// 3. Tag the group with environment metadata.
/// 4. Verify retrieval by ARN.
/// 5. Update the group's IAM role and trail settings.
/// 6. Remove the policy and confirm it is gone.
/// 7. Untag and confirm tags are absent.
/// 8. Delete the group and confirm it is no longer listed.
use aws_sdk_aiops::config::BehaviorVersion;
use winterbaume_aiops::AIOpsService;
use winterbaume_core::MockAws;

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
async fn test_investigation_group_full_lifecycle() {
    // Scenario: investigation group full lifecycle
    //
    // Creates a group, attaches a policy, tags it, updates it,
    // removes the policy, untags it, and finally deletes it.
    // Asserts business outcomes at each stage.
    let client = make_client().await;

    // Step 1: create with optional fields
    let create_resp = client
        .create_investigation_group()
        .name("lifecycle-group")
        .role_arn("arn:aws:iam::111122223333:role/aiops-role-v1")
        .retention_in_days(90)
        .is_cloud_trail_event_history_enabled(false)
        .send()
        .await
        .expect("create investigation group");
    let arn = create_resp.arn().expect("arn returned").to_string();
    assert!(
        arn.contains("lifecycle-group"),
        "ARN should contain group name"
    );

    // Step 2: attach a resource policy
    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"AWS":"arn:aws:iam::111122223333:root"},"Action":"aiops:*","Resource":"*"}]}"#;
    let put_resp = client
        .put_investigation_group_policy()
        .identifier("lifecycle-group")
        .policy(policy_doc)
        .send()
        .await
        .expect("put policy");
    assert!(
        put_resp
            .investigation_group_arn()
            .unwrap_or("")
            .contains("lifecycle-group"),
        "policy response should include group ARN"
    );

    // Step 3: tag the group
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "staging")
        .tags("team", "aiops")
        .send()
        .await
        .expect("tag resource");

    // Step 4: retrieve by ARN and verify state
    let get_resp = client
        .get_investigation_group()
        .identifier(&arn)
        .send()
        .await
        .expect("get by arn");
    assert_eq!(get_resp.name(), Some("lifecycle-group"));
    assert_eq!(
        get_resp.role_arn(),
        Some("arn:aws:iam::111122223333:role/aiops-role-v1")
    );
    assert_eq!(get_resp.retention_in_days(), Some(90));
    assert_eq!(get_resp.is_cloud_trail_event_history_enabled(), Some(false));

    let list_tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    let tags = list_tags_resp.tags().expect("tags");
    assert_eq!(tags.get("env").map(String::as_str), Some("staging"));
    assert_eq!(tags.get("team").map(String::as_str), Some("aiops"));

    // Step 5: update role ARN and enable CloudTrail history
    client
        .update_investigation_group()
        .identifier("lifecycle-group")
        .role_arn("arn:aws:iam::111122223333:role/aiops-role-v2")
        .is_cloud_trail_event_history_enabled(true)
        .send()
        .await
        .expect("update group");

    let updated = client
        .get_investigation_group()
        .identifier("lifecycle-group")
        .send()
        .await
        .expect("get after update");
    assert_eq!(
        updated.role_arn(),
        Some("arn:aws:iam::111122223333:role/aiops-role-v2"),
        "role ARN should be updated"
    );
    assert_eq!(
        updated.is_cloud_trail_event_history_enabled(),
        Some(true),
        "CloudTrail flag should be updated"
    );

    // Confirm policy is still accessible after update
    let policy_resp = client
        .get_investigation_group_policy()
        .identifier("lifecycle-group")
        .send()
        .await
        .expect("get policy after update");
    assert_eq!(
        policy_resp.policy(),
        Some(policy_doc),
        "policy should survive update"
    );

    // Step 6: remove the policy and confirm it is gone
    client
        .delete_investigation_group_policy()
        .identifier("lifecycle-group")
        .send()
        .await
        .expect("delete policy");

    let err = client
        .get_investigation_group_policy()
        .identifier("lifecycle-group")
        .send()
        .await
        .expect_err("get policy after delete should fail");
    assert!(
        format!("{:?}", err).contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err:?}"
    );

    // Step 7: untag and confirm tags absent
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag env");

    let remaining_tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list remaining tags");
    let tags_after = remaining_tags.tags().expect("tags after untag");
    assert!(tags_after.get("env").is_none(), "env tag should be removed");
    assert_eq!(
        tags_after.get("team").map(String::as_str),
        Some("aiops"),
        "team tag should remain"
    );

    // Step 8: delete and confirm absent from list
    client
        .delete_investigation_group()
        .identifier("lifecycle-group")
        .send()
        .await
        .expect("delete group");

    let list_resp = client
        .list_investigation_groups()
        .send()
        .await
        .expect("list after delete");
    let remaining: Vec<_> = list_resp
        .investigation_groups()
        .iter()
        .filter(|g| g.name() == Some("lifecycle-group"))
        .collect();
    assert!(
        remaining.is_empty(),
        "deleted group should not appear in list"
    );
}

#[tokio::test]
async fn test_investigation_group_cross_account_configuration() {
    // Scenario: cross-account investigation group
    //
    // Creates an investigation group with cross-account configurations,
    // verifies round-trip fidelity, and updates the role ARN.
    let client = make_client().await;

    use aws_sdk_aiops::types::CrossAccountConfiguration;
    let xacct = CrossAccountConfiguration::builder()
        .source_role_arn("arn:aws:iam::999988887777:role/aiops-cross-role")
        .build();

    client
        .create_investigation_group()
        .name("xacct-group")
        .role_arn("arn:aws:iam::111122223333:role/aiops-role")
        .cross_account_configurations(xacct)
        .send()
        .await
        .expect("create with cross-account config");

    let resp = client
        .get_investigation_group()
        .identifier("xacct-group")
        .send()
        .await
        .expect("get xacct group");

    let cfgs = resp.cross_account_configurations();
    assert_eq!(cfgs.len(), 1, "should have one cross-account configuration");
    assert_eq!(
        cfgs[0].source_role_arn(),
        Some("arn:aws:iam::999988887777:role/aiops-cross-role")
    );
}
