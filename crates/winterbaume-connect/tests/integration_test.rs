use aws_sdk_connect::config::BehaviorVersion;
use winterbaume_connect::ConnectService;
use winterbaume_core::MockAws;

async fn make_connect_client() -> aws_sdk_connect::Client {
    let mock = MockAws::builder()
        .with_service(ConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connect::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_connect::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_instance() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("test-instance")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .expect("create_instance should succeed");

    let instance_id = create_resp.id().expect("should have instance id");
    assert!(!instance_id.is_empty());
    assert!(create_resp.arn().is_some());

    let describe_resp = client
        .describe_instance()
        .instance_id(instance_id)
        .send()
        .await
        .expect("describe_instance should succeed");

    let instance = describe_resp.instance().expect("should have instance");
    assert_eq!(instance.id().unwrap(), instance_id);
    assert_eq!(instance.instance_alias().unwrap(), "test-instance");
}

#[tokio::test]
async fn test_list_instances() {
    let client = make_connect_client().await;

    for alias in ["inst1", "inst2", "inst3"] {
        client
            .create_instance()
            .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
            .instance_alias(alias)
            .inbound_calls_enabled(true)
            .outbound_calls_enabled(true)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_instances()
        .send()
        .await
        .expect("list_instances should succeed");

    assert_eq!(resp.instance_summary_list().len(), 3);
}

#[tokio::test]
async fn test_delete_instance() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("del-instance")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(false)
        .send()
        .await
        .unwrap();

    let instance_id = create_resp.id().unwrap();

    client
        .delete_instance()
        .instance_id(instance_id)
        .send()
        .await
        .expect("delete_instance should succeed");

    let result = client
        .describe_instance()
        .instance_id(instance_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_instances_empty() {
    let client = make_connect_client().await;

    let resp = client
        .list_instances()
        .send()
        .await
        .expect("list_instances should succeed");

    assert!(resp.instance_summary_list().is_empty());
}

#[tokio::test]
async fn test_describe_nonexistent_instance_fails() {
    let client = make_connect_client().await;

    let result = client
        .describe_instance()
        .instance_id("nonexistent-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_instance_fails() {
    let client = make_connect_client().await;

    let result = client
        .delete_instance()
        .instance_id("nonexistent-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_associate_and_list_analytics_data_set() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("analytics-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let instance_id = create_resp.id().unwrap();

    // Associate a data set
    let assoc_resp = client
        .associate_analytics_data_set()
        .instance_id(instance_id)
        .data_set_id("ds-12345")
        .send()
        .await
        .expect("associate_analytics_data_set should succeed");

    assert_eq!(assoc_resp.data_set_id().unwrap(), "ds-12345");
    assert!(assoc_resp.resource_share_id().is_some());
    assert!(assoc_resp.resource_share_arn().is_some());

    // Associate another data set
    client
        .associate_analytics_data_set()
        .instance_id(instance_id)
        .data_set_id("ds-67890")
        .send()
        .await
        .expect("second associate should succeed");

    // List associations
    let list_resp = client
        .list_analytics_data_associations()
        .instance_id(instance_id)
        .send()
        .await
        .expect("list_analytics_data_associations should succeed");

    assert_eq!(list_resp.results().len(), 2);
}

#[tokio::test]
async fn test_disassociate_analytics_data_set() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("disassoc-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let instance_id = create_resp.id().unwrap();

    client
        .associate_analytics_data_set()
        .instance_id(instance_id)
        .data_set_id("ds-remove-me")
        .send()
        .await
        .unwrap();

    // Disassociate
    client
        .disassociate_analytics_data_set()
        .instance_id(instance_id)
        .data_set_id("ds-remove-me")
        .send()
        .await
        .expect("disassociate should succeed");

    // List should be empty
    let list_resp = client
        .list_analytics_data_associations()
        .instance_id(instance_id)
        .send()
        .await
        .unwrap();

    assert!(list_resp.results().is_empty());
}

#[tokio::test]
async fn test_disassociate_nonexistent_analytics_data_set_fails() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("disassoc-fail-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let instance_id = create_resp.id().unwrap();

    let result = client
        .disassociate_analytics_data_set()
        .instance_id(instance_id)
        .data_set_id("nonexistent-ds")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("tag-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().unwrap().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "prod");
    assert_eq!(tags.get("team").unwrap(), "backend");
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("untag-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().unwrap().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .tags("version", "1")
        .send()
        .await
        .unwrap();

    // Untag 'team'
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    // List tags - should have env and version, not team
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "prod");
    assert_eq!(tags.get("version").unwrap(), "1");
    assert!(!tags.contains_key("team"));
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Connect
// Source: https://docs.aws.amazon.com/connect/latest/APIReference/
// ============================================================================

#[tokio::test]
async fn test_create_instance_response_fields() {
    let client = make_connect_client().await;

    let resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("resp-fields-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .expect("create_instance should succeed");

    let id = resp.id().expect("response should have Id");
    assert!(!id.is_empty(), "Id should not be empty");

    let arn = resp.arn().expect("response should have Arn");
    assert!(!arn.is_empty(), "Arn should not be empty");
    assert!(
        arn.contains(id),
        "Arn '{arn}' should contain instance id '{id}'"
    );
}

#[tokio::test]
async fn test_create_instance_with_tags() {
    let client = make_connect_client().await;

    let resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("tagged-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .tags("project", "winterbaume")
        .tags("env", "test")
        .send()
        .await
        .expect("create_instance with tags should succeed");

    let arn = resp.arn().expect("should have arn").to_string();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().expect("tags map should be present");
    assert_eq!(
        tags.get("project").map(|s| s.as_str()),
        Some("winterbaume"),
        "project tag should be reflected"
    );
    assert_eq!(
        tags.get("env").map(|s| s.as_str()),
        Some("test"),
        "env tag should be reflected"
    );
}

#[tokio::test]
async fn test_create_instance_duplicate_alias() {
    let client = make_connect_client().await;

    client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("dup-alias-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .expect("first create_instance should succeed");

    let err = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("dup-alias-inst")
        .inbound_calls_enabled(false)
        .outbound_calls_enabled(false)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceConflictException"),
        "Expected ResourceConflictException for duplicate alias, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_instance_fields() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("fields-check-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(false)
        .send()
        .await
        .unwrap();

    let instance_id = create_resp.id().unwrap().to_string();

    let describe_resp = client
        .describe_instance()
        .instance_id(&instance_id)
        .send()
        .await
        .expect("describe_instance should succeed");

    let inst = describe_resp.instance().expect("should have instance");

    assert_eq!(inst.id().unwrap(), instance_id.as_str());
    assert_eq!(
        inst.instance_alias().unwrap(),
        "fields-check-inst",
        "instance_alias should match"
    );
    assert_eq!(
        inst.identity_management_type().unwrap(),
        &aws_sdk_connect::types::DirectoryType::ConnectManaged,
        "identity_management_type should be CONNECT_MANAGED"
    );
    assert_eq!(
        inst.instance_status().unwrap(),
        &aws_sdk_connect::types::InstanceStatus::Active,
        "instance_status should be ACTIVE"
    );
    assert!(
        inst.inbound_calls_enabled().unwrap(),
        "inbound_calls_enabled should be true"
    );
    assert!(
        !inst.outbound_calls_enabled().unwrap(),
        "outbound_calls_enabled should be false"
    );
    assert!(inst.created_time().is_some(), "created_time should be set");
}

#[tokio::test]
async fn test_describe_instance_arn_format() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("arn-format-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let instance_id = create_resp.id().unwrap().to_string();

    let describe_resp = client
        .describe_instance()
        .instance_id(&instance_id)
        .send()
        .await
        .unwrap();

    let inst = describe_resp.instance().unwrap();
    let arn = inst.arn().expect("arn should be present");
    assert!(
        arn.starts_with("arn:aws:connect:"),
        "ARN should start with arn:aws:connect:, got: {arn}"
    );
    assert!(
        arn.contains(&instance_id),
        "ARN should contain instance id, got: {arn}"
    );
}

#[tokio::test]
async fn test_create_instance_saml_type() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::Saml)
        .instance_alias("saml-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .expect("create_instance with SAML type should succeed");

    let instance_id = create_resp.id().unwrap().to_string();

    let describe_resp = client
        .describe_instance()
        .instance_id(&instance_id)
        .send()
        .await
        .unwrap();

    let inst = describe_resp.instance().unwrap();
    assert_eq!(
        inst.identity_management_type().unwrap(),
        &aws_sdk_connect::types::DirectoryType::Saml,
        "identity_management_type should be SAML"
    );
}

#[tokio::test]
async fn test_lifecycle_create_describe_delete() {
    let client = make_connect_client().await;

    // Create
    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("lifecycle-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .expect("create should succeed");

    let instance_id = create_resp.id().unwrap().to_string();

    // Describe
    let describe_resp = client
        .describe_instance()
        .instance_id(&instance_id)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(
        describe_resp.instance().unwrap().id().unwrap(),
        instance_id.as_str()
    );

    // Delete
    client
        .delete_instance()
        .instance_id(&instance_id)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .describe_instance()
        .instance_id(&instance_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should return error");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_associate_with_target_account() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("target-acct-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let instance_id = create_resp.id().unwrap().to_string();

    // Associate with explicit target account
    let assoc_resp = client
        .associate_analytics_data_set()
        .instance_id(&instance_id)
        .data_set_id("ds-target-001")
        .target_account_id("111122223333")
        .send()
        .await
        .expect("associate with target account should succeed");

    assert_eq!(assoc_resp.data_set_id().unwrap(), "ds-target-001");
    assert_eq!(
        assoc_resp.target_account_id().unwrap(),
        "111122223333",
        "target_account_id should be reflected in response"
    );

    // Verify it appears in list
    let list_resp = client
        .list_analytics_data_associations()
        .instance_id(&instance_id)
        .send()
        .await
        .unwrap();

    let results = list_resp.results();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].data_set_id().unwrap(), "ds-target-001");
    assert_eq!(results[0].target_account_id().unwrap(), "111122223333");
}

#[tokio::test]
async fn test_associate_duplicate_dataset() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("dup-dataset-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let instance_id = create_resp.id().unwrap().to_string();

    client
        .associate_analytics_data_set()
        .instance_id(&instance_id)
        .data_set_id("ds-dup-001")
        .send()
        .await
        .expect("first associate should succeed");

    let err = client
        .associate_analytics_data_set()
        .instance_id(&instance_id)
        .data_set_id("ds-dup-001")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceConflictException"),
        "Expected ResourceConflictException for duplicate dataset, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_analytics_data_associations_nonexistent_instance() {
    let client = make_connect_client().await;

    let result = client
        .list_analytics_data_associations()
        .instance_id("nonexistent-instance-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "should return error for nonexistent instance"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_disassociate_nonexistent_instance() {
    let client = make_connect_client().await;

    let result = client
        .disassociate_analytics_data_set()
        .instance_id("nonexistent-instance-id")
        .data_set_id("ds-any")
        .send()
        .await;

    assert!(
        result.is_err(),
        "should return error when instance does not exist"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_tags_empty() {
    let client = make_connect_client().await;

    // List tags for an ARN that was never tagged
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:connect:us-east-1:123456789012:instance/no-such-inst")
        .send()
        .await
        .expect("list_tags_for_resource for untagged ARN should succeed without error");

    let tags = tags_resp.tags();
    let is_empty = tags.map(|t| t.is_empty()).unwrap_or(true);
    assert!(is_empty, "tags should be empty for untagged resource");
}

#[tokio::test]
async fn test_untag_nonexistent_key() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("untag-missing-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().unwrap().to_string();

    // Untag a key that was never set - should succeed idempotently
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("nonexistent-key")
        .send()
        .await
        .expect("untag_resource for nonexistent key should succeed idempotently");
}

#[tokio::test]
async fn test_tag_resource_overwrite() {
    let client = make_connect_client().await;

    let create_resp = client
        .create_instance()
        .identity_management_type(aws_sdk_connect::types::DirectoryType::ConnectManaged)
        .instance_alias("overwrite-tag-inst")
        .inbound_calls_enabled(true)
        .outbound_calls_enabled(true)
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().unwrap().to_string();

    // First tag
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("color", "blue")
        .send()
        .await
        .unwrap();

    // Overwrite same key with different value
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("color", "green")
        .send()
        .await
        .expect("second tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().expect("tags should be present");
    assert_eq!(
        tags.get("color").map(|s| s.as_str()),
        Some("green"),
        "tag value should be overwritten by second call"
    );
}
