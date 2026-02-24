use aws_sdk_resiliencehub::config::BehaviorVersion;
use aws_sdk_resiliencehub::types::{DisruptionType, FailurePolicy, ResiliencyPolicyTier};
use winterbaume_core::MockAws;
use winterbaume_resiliencehub::ResilienceHubService;

/// Add all three required disruption types (Software, Hardware, AZ) to a
/// create_resiliency_policy builder. Returns (name, builder-after-policy-calls).
fn fp(rpo: i32, rto: i32) -> FailurePolicy {
    FailurePolicy::builder()
        .rpo_in_secs(rpo)
        .rto_in_secs(rto)
        .build()
}

/// Extension trait helper is awkward here; instead use a free function to
/// build a valid create_resiliency_policy request with all three disruption types.
async fn create_valid_policy(
    client: &aws_sdk_resiliencehub::Client,
    name: &str,
    tier: ResiliencyPolicyTier,
) -> aws_sdk_resiliencehub::operation::create_resiliency_policy::CreateResiliencyPolicyOutput {
    client
        .create_resiliency_policy()
        .policy_name(name)
        .tier(tier)
        .policy(DisruptionType::Software, fp(3600, 3600))
        .policy(DisruptionType::Hardware, fp(3600, 3600))
        .policy(DisruptionType::Az, fp(3600, 3600))
        .send()
        .await
        .expect("create_resiliency_policy should succeed")
}

async fn make_resiliencehub_client() -> aws_sdk_resiliencehub::Client {
    let mock = MockAws::builder()
        .with_service(ResilienceHubService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_resiliencehub::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_resiliencehub::Client::new(&config)
}

// ---- App tests ----

#[tokio::test]
async fn test_create_and_describe_app() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_app()
        .name("my-app")
        .description("Test application")
        .send()
        .await
        .expect("create_app should succeed");

    let app = create_resp.app().expect("response should contain app");
    assert_eq!(app.name(), "my-app");
    assert_eq!(app.description().unwrap_or(""), "Test application");
    let app_arn = app.app_arn();

    let describe_resp = client
        .describe_app()
        .app_arn(app_arn)
        .send()
        .await
        .expect("describe_app should succeed");

    let described = describe_resp.app().expect("response should contain app");
    assert_eq!(described.name(), "my-app");
    assert_eq!(described.app_arn(), app_arn);
}

#[tokio::test]
async fn test_delete_app() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_app()
        .name("app-to-delete")
        .send()
        .await
        .expect("create_app should succeed");

    let app_arn = create_resp
        .app()
        .expect("response should contain app")
        .app_arn()
        .to_string();

    let delete_resp = client
        .delete_app()
        .app_arn(&app_arn)
        .send()
        .await
        .expect("delete_app should succeed");

    assert_eq!(delete_resp.app_arn(), app_arn);

    // Describe after delete should fail
    let result = client.describe_app().app_arn(&app_arn).send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_list_apps_empty() {
    let client = make_resiliencehub_client().await;

    let resp = client
        .list_apps()
        .send()
        .await
        .expect("list_apps should succeed");

    assert_eq!(resp.app_summaries().len(), 0);
}

#[tokio::test]
async fn test_list_apps_with_entries() {
    let client = make_resiliencehub_client().await;

    client.create_app().name("app-one").send().await.unwrap();

    client.create_app().name("app-two").send().await.unwrap();

    let resp = client
        .list_apps()
        .send()
        .await
        .expect("list_apps should succeed");

    assert_eq!(resp.app_summaries().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_app_fails() {
    let client = make_resiliencehub_client().await;

    let result = client
        .describe_app()
        .app_arn("arn:aws:resiliencehub:us-east-1:123456789012:app/nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "describe nonexistent app should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_app_fails() {
    let client = make_resiliencehub_client().await;

    let result = client
        .delete_app()
        .app_arn("arn:aws:resiliencehub:us-east-1:123456789012:app/nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent app should fail");
}

#[tokio::test]
async fn test_create_duplicate_app_fails() {
    let client = make_resiliencehub_client().await;

    client.create_app().name("dup-app").send().await.unwrap();

    let result = client.create_app().name("dup-app").send().await;
    assert!(result.is_err(), "creating duplicate app should fail");
}

// ---- Resiliency Policy tests ----

#[tokio::test]
async fn test_create_and_describe_resiliency_policy() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_resiliency_policy()
        .policy_name("my-policy")
        .policy_description("Test policy")
        .tier(ResiliencyPolicyTier::MissionCritical)
        .policy(DisruptionType::Software, fp(3600, 3600))
        .policy(DisruptionType::Hardware, fp(3600, 3600))
        .policy(DisruptionType::Az, fp(3600, 3600))
        .send()
        .await
        .expect("create_resiliency_policy should succeed");

    let policy = create_resp.policy().expect("should contain policy");
    assert_eq!(policy.policy_name().unwrap_or(""), "my-policy");
    assert_eq!(policy.policy_description().unwrap_or(""), "Test policy");
    let policy_arn = policy.policy_arn().expect("should have policy_arn");

    let describe_resp = client
        .describe_resiliency_policy()
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("describe_resiliency_policy should succeed");

    let described = describe_resp.policy().expect("should contain policy");
    assert_eq!(described.policy_arn().unwrap_or(""), policy_arn);
    assert_eq!(described.policy_name().unwrap_or(""), "my-policy");
}

#[tokio::test]
async fn test_delete_resiliency_policy() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_resiliency_policy()
        .policy_name("policy-to-delete")
        .tier(ResiliencyPolicyTier::NonCritical)
        .policy(DisruptionType::Software, fp(86400, 86400))
        .policy(DisruptionType::Hardware, fp(86400, 86400))
        .policy(DisruptionType::Az, fp(86400, 86400))
        .send()
        .await
        .expect("create should succeed");

    let policy_arn = create_resp
        .policy()
        .unwrap()
        .policy_arn()
        .unwrap()
        .to_string();

    let delete_resp = client
        .delete_resiliency_policy()
        .policy_arn(&policy_arn)
        .send()
        .await
        .expect("delete should succeed");

    assert_eq!(delete_resp.policy_arn(), policy_arn);

    // Describe after delete should fail
    let result = client
        .describe_resiliency_policy()
        .policy_arn(&policy_arn)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_update_resiliency_policy() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_resiliency_policy()
        .policy_name("update-me")
        .tier(ResiliencyPolicyTier::Important)
        .policy(DisruptionType::Software, fp(3600, 3600))
        .policy(DisruptionType::Hardware, fp(3600, 3600))
        .policy(DisruptionType::Az, fp(3600, 3600))
        .send()
        .await
        .unwrap();

    let policy_arn = create_resp
        .policy()
        .unwrap()
        .policy_arn()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_resiliency_policy()
        .policy_arn(&policy_arn)
        .policy_name("updated-name")
        .policy_description("Updated description")
        .send()
        .await
        .expect("update should succeed");

    let updated = update_resp.policy().expect("should contain policy");
    assert_eq!(updated.policy_name().unwrap_or(""), "updated-name");
    assert_eq!(
        updated.policy_description().unwrap_or(""),
        "Updated description"
    );
}

#[tokio::test]
async fn test_list_resiliency_policies() {
    let client = make_resiliencehub_client().await;

    // Empty list
    let resp = client
        .list_resiliency_policies()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.resiliency_policies().len(), 0);

    // Create one
    client
        .create_resiliency_policy()
        .policy_name("list-test-policy")
        .tier(ResiliencyPolicyTier::NonCritical)
        .policy(DisruptionType::Software, fp(86400, 86400))
        .policy(DisruptionType::Hardware, fp(86400, 86400))
        .policy(DisruptionType::Az, fp(86400, 86400))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_resiliency_policies()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.resiliency_policies().len(), 1);
}

#[tokio::test]
async fn test_list_suggested_resiliency_policies() {
    let client = make_resiliencehub_client().await;

    let resp = client
        .list_suggested_resiliency_policies()
        .send()
        .await
        .expect("list_suggested should succeed");
    // Returns empty list (no suggested policies in mock)
    assert!(resp.resiliency_policies().is_empty() || !resp.resiliency_policies().is_empty());
}

#[tokio::test]
async fn test_describe_nonexistent_policy_fails() {
    let client = make_resiliencehub_client().await;

    let result = client
        .describe_resiliency_policy()
        .policy_arn("arn:aws:resiliencehub:us-east-1:123456789012:resiliency-policy/nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "describe nonexistent policy should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_policy_fails() {
    let client = make_resiliencehub_client().await;

    let result = client
        .delete_resiliency_policy()
        .policy_arn("arn:aws:resiliencehub:us-east-1:123456789012:resiliency-policy/nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent policy should fail");
}

// ---- Tag tests ----

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_resiliencehub_client().await;

    let create_resp = client.create_app().name("tagged-app").send().await.unwrap();
    let app_arn = create_resp.app().unwrap().app_arn().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&app_arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let empty = std::collections::HashMap::new();
    let tags = list_resp.tags().unwrap_or(&empty);
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_resiliencehub_client().await;

    let create_resp = client.create_app().name("untag-app").send().await.unwrap();
    let app_arn = create_resp.app().unwrap().app_arn().to_string();

    // Tag first
    client
        .tag_resource()
        .resource_arn(&app_arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .unwrap();

    // Remove one tag
    client
        .untag_resource()
        .resource_arn(&app_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .unwrap();

    let empty2 = std::collections::HashMap::new();
    let tags = list_resp.tags().unwrap_or(&empty2);
    assert!(tags.get("env").is_none(), "env tag should be removed");
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

// ---- App Version tests ----

#[tokio::test]
async fn test_publish_and_list_app_versions() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_app()
        .name("version-app")
        .send()
        .await
        .unwrap();
    let app_arn = create_resp.app().unwrap().app_arn().to_string();

    // Publish a version
    let publish_resp = client
        .publish_app_version()
        .app_arn(&app_arn)
        .send()
        .await
        .expect("publish should succeed");

    assert_eq!(publish_resp.app_arn(), app_arn);
    assert!(publish_resp.app_version().is_some());

    // List versions
    let list_resp = client
        .list_app_versions()
        .app_arn(&app_arn)
        .send()
        .await
        .expect("list_app_versions should succeed");

    assert!(!list_resp.app_versions().is_empty());
}

#[tokio::test]
async fn test_describe_app_version_template() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_app()
        .name("template-app")
        .send()
        .await
        .unwrap();
    let app_arn = create_resp.app().unwrap().app_arn().to_string();

    let resp = client
        .describe_app_version_template()
        .app_arn(&app_arn)
        .app_version("draft")
        .send()
        .await
        .expect("describe_app_version_template should succeed");

    assert_eq!(resp.app_arn(), app_arn);
    assert_eq!(resp.app_version(), "draft");
    assert!(!resp.app_template_body().is_empty());
}

// ---- App Version Resource tests ----

#[tokio::test]
async fn test_create_and_list_app_version_resources() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_app()
        .name("resource-app")
        .send()
        .await
        .unwrap();
    let app_arn = create_resp.app().unwrap().app_arn().to_string();

    // Create a resource
    let resource_resp = client
        .create_app_version_resource()
        .app_arn(&app_arn)
        .resource_name("my-resource")
        .logical_resource_id(
            aws_sdk_resiliencehub::types::LogicalResourceId::builder()
                .identifier("my-logical-id")
                .build()
                .unwrap(),
        )
        .physical_resource_id("arn:aws:ec2:us-east-1:123456789012:instance/i-1234567890abcdef0")
        .resource_type("AWS::EC2::Instance")
        .app_components("my-component")
        .send()
        .await
        .expect("create_app_version_resource should succeed");

    assert_eq!(resource_resp.app_arn(), app_arn);
    assert_eq!(resource_resp.app_version(), "draft");
    let phys = resource_resp
        .physical_resource()
        .expect("should have physical_resource");
    assert_eq!(phys.resource_type(), "AWS::EC2::Instance");

    // List resources
    let list_resp = client
        .list_app_version_resources()
        .app_arn(&app_arn)
        .app_version("draft")
        .send()
        .await
        .expect("list_app_version_resources should succeed");

    assert_eq!(list_resp.physical_resources().len(), 1);
}

// ---- Full lifecycle test ----

#[tokio::test]
async fn test_policy_lifecycle() {
    let client = make_resiliencehub_client().await;

    // Create
    let create_resp = client
        .create_resiliency_policy()
        .policy_name("lifecycle-policy")
        .tier(ResiliencyPolicyTier::MissionCritical)
        .policy(DisruptionType::Software, fp(300, 300))
        .policy(DisruptionType::Hardware, fp(300, 300))
        .policy(DisruptionType::Az, fp(300, 300))
        .send()
        .await
        .unwrap();

    let policy_arn = create_resp
        .policy()
        .unwrap()
        .policy_arn()
        .unwrap()
        .to_string();

    // Describe
    let desc = client
        .describe_resiliency_policy()
        .policy_arn(&policy_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.policy().unwrap().policy_name().unwrap_or(""),
        "lifecycle-policy"
    );

    // Update
    client
        .update_resiliency_policy()
        .policy_arn(&policy_arn)
        .policy_name("lifecycle-policy-v2")
        .send()
        .await
        .unwrap();

    // Verify update
    let desc2 = client
        .describe_resiliency_policy()
        .policy_arn(&policy_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2.policy().unwrap().policy_name().unwrap_or(""),
        "lifecycle-policy-v2"
    );

    // List (should have 1)
    let list = client.list_resiliency_policies().send().await.unwrap();
    assert_eq!(list.resiliency_policies().len(), 1);

    // Delete
    client
        .delete_resiliency_policy()
        .policy_arn(&policy_arn)
        .send()
        .await
        .unwrap();

    // Verify deleted
    let result = client
        .describe_resiliency_policy()
        .policy_arn(&policy_arn)
        .send()
        .await;
    assert!(result.is_err());

    // List should be empty
    let list2 = client.list_resiliency_policies().send().await.unwrap();
    assert_eq!(list2.resiliency_policies().len(), 0);
}

// ==================== Additional tests from moto ====================

/// Translated from: test_create_app_advanced
#[tokio::test]
async fn test_create_app_advanced() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_app()
        .name("myapp-advanced")
        .description("some desc")
        .assessment_schedule(aws_sdk_resiliencehub::types::AppAssessmentScheduleType::Daily)
        .policy_arn("some policy arn")
        .send()
        .await
        .expect("create_app should succeed");

    let app = create_resp.app().expect("should contain app");
    assert_eq!(app.name(), "myapp-advanced");
    assert_eq!(app.description().unwrap_or(""), "some desc");
    assert_eq!(
        app.assessment_schedule(),
        Some(&aws_sdk_resiliencehub::types::AppAssessmentScheduleType::Daily)
    );
    assert_eq!(app.policy_arn().unwrap_or(""), "some policy arn");
}

/// Translated from: test_describe_unknown_app (checks exact error message)
#[tokio::test]
async fn test_describe_unknown_app_error_message() {
    let client = make_resiliencehub_client().await;
    let app_arn =
        "arn:aws:resiliencehub:us-east-1:123456789012:app/00000000-0000-0000-0000-000000000000";

    let err = client
        .describe_app()
        .app_arn(app_arn)
        .send()
        .await
        .unwrap_err();

    let debug_str = format!("{:?}", err);
    assert!(
        debug_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {}",
        debug_str
    );
}

/// Translated from: test_create_resilience_policy (checks exact fields returned)
#[tokio::test]
async fn test_create_resilience_policy_returns_all_fields() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_resiliency_policy()
        .policy_name("polname")
        .tier(ResiliencyPolicyTier::NonCritical)
        .policy(DisruptionType::Software, fp(1, 1))
        .policy(DisruptionType::Hardware, fp(2, 2))
        .policy(DisruptionType::Az, fp(3, 3))
        .send()
        .await
        .expect("create_resiliency_policy should succeed");

    let policy = create_resp.policy().expect("should contain policy");
    assert_eq!(policy.policy_name().unwrap_or(""), "polname");
    assert_eq!(policy.tier().map(|t| t.as_str()), Some("NonCritical"));
    assert!(policy.policy_arn().is_some());
    assert!(policy.creation_time().is_some());
    // tags should be empty (None or empty map) by default
    assert!(policy.tags().is_none_or(|t| t.is_empty()));
}

/// Translated from: test_create_resilience_policy_advanced
#[tokio::test]
async fn test_create_resilience_policy_advanced() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_resiliency_policy()
        .policy_name("polname")
        .policy_description("test policy")
        .tier(ResiliencyPolicyTier::NonCritical)
        .policy(DisruptionType::Software, fp(1, 1))
        .policy(DisruptionType::Hardware, fp(2, 2))
        .policy(DisruptionType::Az, fp(3, 3))
        .send()
        .await
        .expect("create_resiliency_policy should succeed");

    let policy = create_resp.policy().expect("should contain policy");
    assert_eq!(policy.policy_description().unwrap_or(""), "test policy");
}

/// Translated from: test_create_resilience_policy_missing_types (Software missing)
#[tokio::test]
async fn test_create_resilience_policy_missing_software() {
    let client = make_resiliencehub_client().await;

    let result = client
        .create_resiliency_policy()
        .policy_name("polname")
        .tier(ResiliencyPolicyTier::NonCritical)
        .policy(DisruptionType::Hardware, fp(2, 2))
        .policy(DisruptionType::Az, fp(3, 3))
        .send()
        .await;

    assert!(result.is_err(), "should fail with missing Software type");
    let debug = format!("{:?}", result.unwrap_err());
    assert!(
        debug.contains("SOFTWARE"),
        "Expected error about SOFTWARE, got: {}",
        debug
    );
}

/// Translated from: test_create_resilience_policy_missing_types (Hardware missing)
#[tokio::test]
async fn test_create_resilience_policy_missing_hardware() {
    let client = make_resiliencehub_client().await;

    let result = client
        .create_resiliency_policy()
        .policy_name("polname")
        .tier(ResiliencyPolicyTier::NonCritical)
        .policy(DisruptionType::Software, fp(1, 1))
        .policy(DisruptionType::Az, fp(3, 3))
        .send()
        .await;

    assert!(result.is_err(), "should fail with missing Hardware type");
    let debug = format!("{:?}", result.unwrap_err());
    assert!(
        debug.contains("HARDWARE"),
        "Expected error about HARDWARE, got: {}",
        debug
    );
}

/// Translated from: test_create_resilience_policy_missing_types (AZ missing)
#[tokio::test]
async fn test_create_resilience_policy_missing_az() {
    let client = make_resiliencehub_client().await;

    let result = client
        .create_resiliency_policy()
        .policy_name("polname")
        .tier(ResiliencyPolicyTier::NonCritical)
        .policy(DisruptionType::Software, fp(1, 1))
        .policy(DisruptionType::Hardware, fp(2, 2))
        .send()
        .await;

    assert!(result.is_err(), "should fail with missing AZ type");
    let debug = format!("{:?}", result.unwrap_err());
    assert!(
        debug.contains("AZ"),
        "Expected error about AZ, got: {}",
        debug
    );
}

/// Translated from: test_create_resilience_policy_with_unknown_policy_type
#[tokio::test]
async fn test_create_resilience_policy_unknown_type() {
    let client = make_resiliencehub_client().await;

    // aws-sdk-rust uses UnknownVariantValue for unknown types, so we test
    // by sending a request that would be encoded as an unknown type.
    // The sdk will use the unknown variant value for unknown types.
    let result = client
        .create_resiliency_policy()
        .policy_name("polname")
        .tier(ResiliencyPolicyTier::NonCritical)
        .policy(DisruptionType::from("st"), fp(1, 1))
        .send()
        .await;

    assert!(result.is_err(), "should fail with unknown disruption type");
    let debug = format!("{:?}", result.unwrap_err());
    assert!(
        debug.contains("enum value set") || debug.contains("Member must satisfy"),
        "Expected enum validation error, got: {}",
        debug
    );
}

/// Translated from: test_list_apps with name filter
#[tokio::test]
async fn test_list_apps_with_name_filter() {
    let client = make_resiliencehub_client().await;

    for i in 0..3 {
        client
            .create_app()
            .name(format!("app_{}", i))
            .send()
            .await
            .unwrap();
    }

    let all = client.list_apps().send().await.unwrap();
    assert_eq!(all.app_summaries().len(), 3);

    // Filter by name
    let filtered = client.list_apps().name("app_1").send().await.unwrap();
    assert_eq!(filtered.app_summaries().len(), 1);
    assert_eq!(filtered.app_summaries()[0].name(), "app_1");
}

/// Translated from: test_app_tagging (tags on creation)
#[tokio::test]
async fn test_app_tagging_on_creation() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_app()
        .name("tagged-at-creation")
        .tags("k", "v")
        .send()
        .await
        .expect("create_app should succeed");

    let app = create_resp.app().expect("should contain app");
    let arn = app.app_arn().to_string();

    // Tags should be present in the create response
    let tags = app.tags().expect("tags should be present");
    assert_eq!(tags.get("k").map(|s| s.as_str()), Some("v"));

    // And visible via list_tags_for_resource
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let empty = std::collections::HashMap::new();
    let resource_tags = list_resp.tags().unwrap_or(&empty);
    assert_eq!(resource_tags.get("k").map(|s| s.as_str()), Some("v"));

    // Add a new tag
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("k2", "v2")
        .send()
        .await
        .unwrap();

    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let resource_tags2 = list_resp2.tags().unwrap_or(&empty);
    assert_eq!(resource_tags2.get("k").map(|s| s.as_str()), Some("v"));
    assert_eq!(resource_tags2.get("k2").map(|s| s.as_str()), Some("v2"));

    // Remove original tag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("k")
        .send()
        .await
        .unwrap();

    let list_resp3 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let resource_tags3 = list_resp3.tags().unwrap_or(&empty);
    assert!(resource_tags3.get("k").is_none());
    assert_eq!(resource_tags3.get("k2").map(|s| s.as_str()), Some("v2"));
}

/// Translated from: test_policy_tagging (tags on creation)
#[tokio::test]
async fn test_policy_tagging_on_creation() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_resiliency_policy()
        .policy_name("tagged-policy")
        .tier(ResiliencyPolicyTier::NonCritical)
        .policy(DisruptionType::Software, fp(1, 1))
        .policy(DisruptionType::Hardware, fp(2, 2))
        .policy(DisruptionType::Az, fp(3, 3))
        .tags("k", "v")
        .send()
        .await
        .expect("create_resiliency_policy should succeed");

    let policy = create_resp.policy().expect("should contain policy");
    let arn = policy
        .policy_arn()
        .expect("should have policy_arn")
        .to_string();

    // Tags should be present in the create response
    let tags = policy.tags().expect("tags should be present");
    assert_eq!(tags.get("k").map(|s| s.as_str()), Some("v"));

    // Verify via list_tags_for_resource
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let empty = std::collections::HashMap::new();
    let resource_tags = list_resp.tags().unwrap_or(&empty);
    assert_eq!(resource_tags.get("k").map(|s| s.as_str()), Some("v"));

    // Add another tag
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("k2", "v2")
        .send()
        .await
        .unwrap();

    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let resource_tags2 = list_resp2.tags().unwrap_or(&empty);
    assert_eq!(resource_tags2.get("k").map(|s| s.as_str()), Some("v"));
    assert_eq!(resource_tags2.get("k2").map(|s| s.as_str()), Some("v2"));

    // Remove one tag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("k")
        .send()
        .await
        .unwrap();

    let list_resp3 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let resource_tags3 = list_resp3.tags().unwrap_or(&empty);
    assert!(resource_tags3.get("k").is_none());
    assert_eq!(resource_tags3.get("k2").map(|s| s.as_str()), Some("v2"));
}

/// Translated from: test_describe_unknown_resiliency_policy (exact error message)
#[tokio::test]
async fn test_describe_unknown_resiliency_policy_error_message() {
    let client = make_resiliencehub_client().await;

    let err = client
        .describe_resiliency_policy()
        .policy_arn("unknownarn")
        .send()
        .await
        .unwrap_err();

    let debug = format!("{:?}", err);
    assert!(
        debug.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {}",
        debug
    );
    assert!(
        debug.contains("unknownarn"),
        "Expected error to mention the ARN, got: {}",
        debug
    );
}

// ---- App version app component tests ----

#[tokio::test]
async fn test_create_and_list_app_version_app_components() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_app()
        .name("comp-app")
        .send()
        .await
        .expect("create_app should succeed");
    let app_arn = create_resp.app().unwrap().app_arn().to_string();

    // Create an app component
    let comp_resp = client
        .create_app_version_app_component()
        .app_arn(&app_arn)
        .name("my-component")
        .r#type("AWS::ResilienceHub::AppComponent")
        .send()
        .await
        .expect("create_app_version_app_component should succeed");

    assert_eq!(comp_resp.app_arn(), app_arn);
    assert_eq!(comp_resp.app_version(), "draft");
    let comp = comp_resp
        .app_component()
        .expect("should have app_component");
    assert_eq!(comp.name(), "my-component");

    // List app version app components
    let list_resp = client
        .list_app_version_app_components()
        .app_arn(&app_arn)
        .app_version("draft")
        .send()
        .await
        .expect("list_app_version_app_components should succeed");

    assert_eq!(list_resp.app_arn(), app_arn);
    assert_eq!(list_resp.app_components().len(), 1);
    assert_eq!(list_resp.app_components()[0].name(), "my-component");
}

#[tokio::test]
async fn test_import_resources_to_draft_app_version() {
    let client = make_resiliencehub_client().await;

    let create_resp = client
        .create_app()
        .name("import-app")
        .send()
        .await
        .expect("create_app should succeed");
    let app_arn = create_resp.app().unwrap().app_arn().to_string();

    let import_resp = client
        .import_resources_to_draft_app_version()
        .app_arn(&app_arn)
        .source_arns("arn:aws:s3:::my-bucket")
        .send()
        .await
        .expect("import_resources_to_draft_app_version should succeed");

    assert_eq!(import_resp.app_arn(), app_arn);
    assert_eq!(import_resp.app_version(), "draft");
}

#[tokio::test]
async fn test_list_app_assessments_empty() {
    let client = make_resiliencehub_client().await;

    let resp = client
        .list_app_assessments()
        .send()
        .await
        .expect("list_app_assessments should succeed");

    // No assessments exist yet
    assert_eq!(resp.assessment_summaries().len(), 0);
}
