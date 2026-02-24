/// Scenario: Configuration Manager Full Lifecycle
///
/// Creates a configuration manager with two definitions, updates one
/// definition's parameters, retrieves configuration details by definition
/// ID, lists all configurations, updates the manager description, then
/// deletes the manager.  Asserts business outcomes at each step.
use aws_sdk_ssmquicksetup::config::BehaviorVersion;
use aws_sdk_ssmquicksetup::types::ConfigurationDefinitionInput;
use winterbaume_core::MockAws;
use winterbaume_ssmquicksetup::SsmQuickSetupService;

async fn make_client() -> aws_sdk_ssmquicksetup::Client {
    let mock = MockAws::builder()
        .with_service(SsmQuickSetupService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ssmquicksetup::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_ssmquicksetup::Client::new(&config)
}

#[tokio::test]
async fn test_configuration_manager_full_lifecycle() {
    let client = make_client().await;

    // --- Step 1: Create a manager with two definitions ---
    let def_a = ConfigurationDefinitionInput::builder()
        .r#type("AWSConfigSetup")
        .type_version("1.0")
        .parameters("ResourceTypes", "AWS::EC2::Instance")
        .build()
        .unwrap();
    let def_b = ConfigurationDefinitionInput::builder()
        .r#type("AWSPatchManager")
        .type_version("1.0")
        .parameters("PatchGroups", "prod")
        .build()
        .unwrap();
    let create = client
        .create_configuration_manager()
        .name("lifecycle-mgr")
        .description("full lifecycle test")
        .configuration_definitions(def_a)
        .configuration_definitions(def_b)
        .send()
        .await
        .expect("create");
    let arn = create.manager_arn().to_string();
    assert!(!arn.is_empty(), "manager ARN must be non-empty");

    // --- Step 2: Verify both definitions are present ---
    let get = client
        .get_configuration_manager()
        .manager_arn(&arn)
        .send()
        .await
        .expect("get after create");
    let defs = get.configuration_definitions();
    assert_eq!(defs.len(), 2, "expected two definitions after creation");
    let def_id = defs[0]
        .id()
        .expect("definition must have an id")
        .to_string();

    // --- Step 3: Update one definition's parameters ---
    client
        .update_configuration_definition()
        .manager_arn(&arn)
        .id(&def_id)
        .parameters("ResourceTypes", "AWS::S3::Bucket")
        .send()
        .await
        .expect("update definition");

    // --- Step 4: Retrieve configuration by definition ID ---
    let cfg = client
        .get_configuration()
        .configuration_id(&def_id)
        .send()
        .await
        .expect("get configuration");
    assert_eq!(
        cfg.configuration_definition_id(),
        Some(def_id.as_str()),
        "returned definition id must match"
    );
    assert_eq!(
        cfg.manager_arn(),
        Some(arn.as_str()),
        "returned manager ARN must match"
    );

    // --- Step 5: List configurations — both definitions appear ---
    let list_cfgs = client
        .list_configurations()
        .send()
        .await
        .expect("list configurations");
    assert_eq!(
        list_cfgs.configurations_list().len(),
        2,
        "both definitions must appear in list"
    );

    // --- Step 6: Update manager description ---
    client
        .update_configuration_manager()
        .manager_arn(&arn)
        .description("updated description")
        .send()
        .await
        .expect("update manager");
    let after_update = client
        .get_configuration_manager()
        .manager_arn(&arn)
        .send()
        .await
        .expect("get after update");
    assert_eq!(
        after_update.description(),
        Some("updated description"),
        "description must reflect update"
    );

    // --- Step 7: Tag and verify tags ---
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .send()
        .await
        .expect("tag resource");
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert_eq!(tags_resp.tags().len(), 1, "one tag must be returned");

    // --- Step 8: Delete manager — subsequent get must fail ---
    client
        .delete_configuration_manager()
        .manager_arn(&arn)
        .send()
        .await
        .expect("delete");
    let err = client
        .get_configuration_manager()
        .manager_arn(&arn)
        .send()
        .await
        .expect_err("get after delete must fail");
    assert!(
        format!("{err:?}").contains("ResourceNotFoundException"),
        "must surface ResourceNotFoundException after deletion, got: {err:?}"
    );
}
