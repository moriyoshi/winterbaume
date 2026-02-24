//! End-to-end scenario tests for the EventBridge service.
//!
//! Each test exercises a multi-step use-case workflow rather than a single API operation.

use aws_sdk_eventbridge::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_eventbridge::EventBridgeService;

async fn make_events_client() -> aws_sdk_eventbridge::Client {
    let mock = MockAws::builder()
        .with_service(EventBridgeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_eventbridge::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_eventbridge::Client::new(&config)
}

/// Scenario: event bus custom routing pipeline.
///
/// 1. Create a custom event bus.
/// 2. Create a rule on that bus with an event pattern.
/// 3. Add a Lambda target to the rule.
/// 4. Verify the bus, rule, and target are all visible via describe/list.
/// 5. Remove the target and delete the rule; verify cleanup.
/// 6. Delete the event bus.
#[tokio::test]
async fn test_custom_bus_rule_target_pipeline() {
    let client = make_events_client().await;

    // Step 1: create custom bus
    let bus_resp = client
        .create_event_bus()
        .name("pipeline-bus")
        .send()
        .await
        .expect("create_event_bus should succeed");
    let bus_arn = bus_resp.event_bus_arn().expect("bus ARN must be present");
    assert!(bus_arn.contains("pipeline-bus"));

    // Step 2: create rule on the custom bus
    let rule_resp = client
        .put_rule()
        .name("pipeline-rule")
        .event_bus_name("pipeline-bus")
        .event_pattern(r#"{"source": ["com.example.app"]}"#)
        .send()
        .await
        .expect("put_rule should succeed");
    let rule_arn = rule_resp.rule_arn().expect("rule ARN must be present");
    assert!(rule_arn.contains("pipeline-rule"));

    // Step 3: add a Lambda target
    client
        .put_targets()
        .rule("pipeline-rule")
        .event_bus_name("pipeline-bus")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("lambda-target")
                .arn("arn:aws:lambda:us-east-1:123456789012:function:pipeline-fn")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_targets should succeed");

    // Step 4: verify via list APIs
    let targets_resp = client
        .list_targets_by_rule()
        .rule("pipeline-rule")
        .event_bus_name("pipeline-bus")
        .send()
        .await
        .expect("list_targets_by_rule should succeed");
    assert_eq!(targets_resp.targets().len(), 1);
    assert_eq!(targets_resp.targets()[0].id(), "lambda-target");

    let desc_bus = client
        .describe_event_bus()
        .name("pipeline-bus")
        .send()
        .await
        .expect("describe_event_bus should succeed");
    assert_eq!(desc_bus.name(), Some("pipeline-bus"));

    // Step 5: remove target, then delete rule
    client
        .remove_targets()
        .rule("pipeline-rule")
        .event_bus_name("pipeline-bus")
        .ids("lambda-target")
        .send()
        .await
        .expect("remove_targets should succeed");

    client
        .delete_rule()
        .name("pipeline-rule")
        .event_bus_name("pipeline-bus")
        .send()
        .await
        .expect("delete_rule should succeed");

    let rules = client
        .list_rules()
        .event_bus_name("pipeline-bus")
        .send()
        .await
        .expect("list_rules should succeed");
    assert_eq!(rules.rules().len(), 0);

    // Step 6: delete the bus
    client
        .delete_event_bus()
        .name("pipeline-bus")
        .send()
        .await
        .expect("delete_event_bus should succeed");
}

/// Scenario: connection and API destination lifecycle.
///
/// 1. Create a connection with BASIC auth.
/// 2. Create an API destination pointing at the connection.
/// 3. Update the API destination endpoint.
/// 4. Verify the updated state via describe.
/// 5. Delete the API destination and connection.
#[tokio::test]
async fn test_connection_and_api_destination_lifecycle() {
    let client = make_events_client().await;

    // Step 1: create connection
    let conn_resp = client
        .create_connection()
        .name("scenario-conn")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::Basic)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .basic_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionBasicAuthRequestParameters::builder()
                        .username("user")
                        .password("pass")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("create_connection should succeed");
    let conn_arn = conn_resp.connection_arn().expect("connection ARN required");

    // Step 2: create API destination
    let dest_resp = client
        .create_api_destination()
        .name("scenario-dest")
        .connection_arn(conn_arn)
        .invocation_endpoint("https://example.com/v1/events")
        .http_method(aws_sdk_eventbridge::types::ApiDestinationHttpMethod::Post)
        .send()
        .await
        .expect("create_api_destination should succeed");
    assert!(dest_resp.api_destination_arn().is_some());

    // Step 3: update the endpoint
    client
        .update_api_destination()
        .name("scenario-dest")
        .invocation_endpoint("https://example.com/v2/events")
        .send()
        .await
        .expect("update_api_destination should succeed");

    // Step 4: verify updated state
    let desc = client
        .describe_api_destination()
        .name("scenario-dest")
        .send()
        .await
        .expect("describe_api_destination should succeed");
    assert_eq!(
        desc.invocation_endpoint(),
        Some("https://example.com/v2/events")
    );

    // Step 5: delete destination then connection
    client
        .delete_api_destination()
        .name("scenario-dest")
        .send()
        .await
        .expect("delete_api_destination should succeed");

    client
        .delete_connection()
        .name("scenario-conn")
        .send()
        .await
        .expect("delete_connection should succeed");
}

/// Scenario: archive creation and tagging workflow.
///
/// 1. Create a custom event bus.
/// 2. Create an archive sourced from that bus.
/// 3. Tag both the bus and the archive.
/// 4. Verify tags are retrievable for both resources.
/// 5. Update archive description and verify.
/// 6. Clean up.
#[tokio::test]
async fn test_archive_and_tagging_workflow() {
    let client = make_events_client().await;

    // Step 1: bus
    let bus_resp = client
        .create_event_bus()
        .name("archive-bus")
        .send()
        .await
        .expect("create_event_bus should succeed");
    let bus_arn = bus_resp.event_bus_arn().expect("bus ARN must be present");

    // Step 2: archive
    let archive_resp = client
        .create_archive()
        .archive_name("archive-test")
        .event_source_arn(bus_arn.to_string())
        .description("initial description")
        .send()
        .await
        .expect("create_archive should succeed");
    let archive_arn = archive_resp.archive_arn().expect("archive ARN required");

    // Step 3: tag both resources
    client
        .tag_resource()
        .resource_arn(bus_arn)
        .tags(
            aws_sdk_eventbridge::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource (bus) should succeed");

    client
        .tag_resource()
        .resource_arn(archive_arn)
        .tags(
            aws_sdk_eventbridge::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource (archive) should succeed");

    // Step 4: verify tags
    let bus_tags = client
        .list_tags_for_resource()
        .resource_arn(bus_arn)
        .send()
        .await
        .expect("list_tags_for_resource (bus) should succeed");
    assert_eq!(bus_tags.tags().len(), 1);
    assert_eq!(bus_tags.tags()[0].key(), "env");

    let archive_tags = client
        .list_tags_for_resource()
        .resource_arn(archive_arn)
        .send()
        .await
        .expect("list_tags_for_resource (archive) should succeed");
    assert_eq!(archive_tags.tags().len(), 1);
    assert_eq!(archive_tags.tags()[0].key(), "team");

    // Step 5: update archive description
    client
        .update_archive()
        .archive_name("archive-test")
        .description("updated description")
        .send()
        .await
        .expect("update_archive should succeed");

    let desc = client
        .describe_archive()
        .archive_name("archive-test")
        .send()
        .await
        .expect("describe_archive should succeed");
    assert_eq!(desc.description(), Some("updated description"));

    // Step 6: clean up
    client
        .delete_archive()
        .archive_name("archive-test")
        .send()
        .await
        .expect("delete_archive should succeed");

    client
        .delete_event_bus()
        .name("archive-bus")
        .send()
        .await
        .expect("delete_event_bus should succeed");
}
