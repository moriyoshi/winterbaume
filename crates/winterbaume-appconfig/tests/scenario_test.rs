/// Scenario: Full configuration deployment pipeline
///
/// Tests the end-to-end lifecycle of deploying a configuration:
/// create application -> create environment -> create deployment strategy ->
/// create configuration profile -> create hosted configuration version ->
/// start deployment -> verify deployment -> stop deployment.
use aws_sdk_appconfig::config::BehaviorVersion;
use winterbaume_appconfig::AppConfigService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_appconfig::Client {
    let mock = MockAws::builder()
        .with_service(AppConfigService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appconfig::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_appconfig::Client::new(&config)
}

/// Scenario: Configuration deployment pipeline
///
/// Chains application creation, environment, deployment strategy, configuration
/// profile, hosted configuration version, and a full deployment lifecycle
/// including stop. Asserts business outcomes rather than individual API shapes.
#[tokio::test]
async fn test_configuration_deployment_pipeline() {
    let client = make_client().await;

    // Step 1: create application
    let app = client
        .create_application()
        .name("pipeline-app")
        .description("Pipeline scenario app")
        .send()
        .await
        .expect("create_application should succeed");
    let app_id = app.id().expect("app must have id").to_string();

    // Step 2: create deployment strategy
    let strategy = client
        .create_deployment_strategy()
        .name("fast-rollout")
        .deployment_duration_in_minutes(0)
        .final_bake_time_in_minutes(0)
        .growth_factor(100.0)
        .send()
        .await
        .expect("create_deployment_strategy should succeed");
    let strategy_id = strategy.id().expect("strategy must have id").to_string();

    // Step 3: create environment
    let env = client
        .create_environment()
        .application_id(&app_id)
        .name("prod")
        .description("Production environment")
        .send()
        .await
        .expect("create_environment should succeed");
    let env_id = env.id().expect("env must have id").to_string();

    // Step 4: create configuration profile
    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("feature-flags")
        .location_uri("hosted")
        .send()
        .await
        .expect("create_configuration_profile should succeed");
    let profile_id = profile.id().expect("profile must have id").to_string();

    // Step 5: create hosted configuration version
    let version = client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content_type("application/json")
        .content(aws_sdk_appconfig::primitives::Blob::new(
            br#"{"feature_enabled": true}"#,
        ))
        .send()
        .await
        .expect("create_hosted_configuration_version should succeed");
    let version_number = version.version_number();
    assert_eq!(version_number, 1, "first version should be 1");

    // Step 6: start deployment
    let deployment = client
        .start_deployment()
        .application_id(&app_id)
        .environment_id(&env_id)
        .deployment_strategy_id(&strategy_id)
        .configuration_profile_id(&profile_id)
        .configuration_version(version_number.to_string())
        .send()
        .await
        .expect("start_deployment should succeed");
    let deployment_number = deployment.deployment_number();
    assert_eq!(deployment_number, 1, "first deployment should be 1");
    assert_eq!(
        deployment.state().map(|s| s.as_str()),
        Some("COMPLETE"),
        "mock deployments complete immediately"
    );

    // Step 7: verify deployment appears in list
    let deployments = client
        .list_deployments()
        .application_id(&app_id)
        .environment_id(&env_id)
        .send()
        .await
        .expect("list_deployments should succeed");
    assert_eq!(
        deployments.items().len(),
        1,
        "one deployment should be listed"
    );

    // Step 8: stop deployment (sets state to REVERTED)
    let stopped = client
        .stop_deployment()
        .application_id(&app_id)
        .environment_id(&env_id)
        .deployment_number(deployment_number)
        .send()
        .await
        .expect("stop_deployment should succeed");
    assert_eq!(
        stopped.state().map(|s| s.as_str()),
        Some("REVERTED"),
        "stopped deployment should be REVERTED"
    );
}

/// Scenario: Extension association lifecycle
///
/// Creates an extension, associates it with a resource (application ARN), then
/// lists and deletes the association.
#[tokio::test]
async fn test_extension_association_lifecycle() {
    let client = make_client().await;

    // Create application to associate the extension with
    let app = client
        .create_application()
        .name("ext-assoc-app")
        .send()
        .await
        .expect("create_application should succeed");
    let app_id = app.id().expect("app must have id").to_string();

    // Create extension
    let ext = client
        .create_extension()
        .name("my-extension")
        .send()
        .await
        .expect("create_extension should succeed");
    let ext_arn = ext.arn().expect("extension must have arn").to_string();

    // Associate extension with the application ARN
    let resource_arn = format!("arn:aws:appconfig:us-east-1:123456789012:application/{app_id}");
    let assoc = client
        .create_extension_association()
        .extension_identifier(&ext_arn)
        .resource_identifier(&resource_arn)
        .send()
        .await
        .expect("create_extension_association should succeed");
    let assoc_id = assoc.id().expect("association must have id").to_string();

    // List associations and confirm it appears
    let list_resp = client
        .list_extension_associations()
        .send()
        .await
        .expect("list_extension_associations should succeed");
    assert_eq!(
        list_resp.items().len(),
        1,
        "one association should be present"
    );

    // Delete association
    client
        .delete_extension_association()
        .extension_association_id(&assoc_id)
        .send()
        .await
        .expect("delete_extension_association should succeed");

    // Confirm association is gone
    let list_after = client
        .list_extension_associations()
        .send()
        .await
        .expect("list after delete should succeed");
    assert_eq!(
        list_after.items().len(),
        0,
        "association should be removed after delete"
    );
}

/// Scenario: Multi-version configuration management
///
/// Creates multiple hosted configuration versions on the same profile and
/// verifies listing, retrieval by version number, and deletion of individual
/// versions.
#[tokio::test]
async fn test_multi_version_configuration_management() {
    let client = make_client().await;

    // Setup application and profile
    let app = client
        .create_application()
        .name("multiversion-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("my-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    // Create 3 versions
    for i in 1u8..=3 {
        client
            .create_hosted_configuration_version()
            .application_id(&app_id)
            .configuration_profile_id(&profile_id)
            .content_type("application/json")
            .content(aws_sdk_appconfig::primitives::Blob::new(
                format!(r#"{{"version": {i}}}"#).as_bytes(),
            ))
            .send()
            .await
            .unwrap_or_else(|_| panic!("version {i} creation should succeed"));
    }

    // List versions and confirm 3 exist
    let versions = client
        .list_hosted_configuration_versions()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .send()
        .await
        .expect("list_hosted_configuration_versions should succeed");
    assert_eq!(versions.items().len(), 3, "should have 3 versions");

    // Fetch version 2 specifically and verify version number
    let v2 = client
        .get_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .version_number(2)
        .send()
        .await
        .expect("get version 2 should succeed");
    assert_eq!(v2.version_number(), 2);

    // Delete version 1
    client
        .delete_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .version_number(1)
        .send()
        .await
        .expect("delete version 1 should succeed");

    // Confirm only 2 versions remain
    let remaining = client
        .list_hosted_configuration_versions()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        remaining.items().len(),
        2,
        "should have 2 versions after delete"
    );
}
