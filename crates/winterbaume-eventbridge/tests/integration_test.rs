//! Integration tests for winterbaume EventBridge service.

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

#[tokio::test]
async fn test_put_rule() {
    let client = make_events_client().await;

    let resp = client
        .put_rule()
        .name("test-rule")
        .schedule_expression("rate(5 minutes)")
        .send()
        .await
        .expect("put_rule should succeed");

    assert!(resp.rule_arn().is_some());
    assert!(resp.rule_arn().unwrap().contains("test-rule"));
}

#[tokio::test]
async fn test_describe_rule() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("desc-rule")
        .event_pattern(r#"{"source": ["aws.ec2"]}"#)
        .description("My test rule")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_rule()
        .name("desc-rule")
        .send()
        .await
        .expect("describe_rule should succeed");

    assert_eq!(resp.name(), Some("desc-rule"));
    assert_eq!(resp.description(), Some("My test rule"));
    assert!(resp.event_pattern().is_some());
}

#[tokio::test]
async fn test_list_rules() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("rule-a")
        .schedule_expression("rate(1 minute)")
        .send()
        .await
        .unwrap();
    client
        .put_rule()
        .name("rule-b")
        .schedule_expression("rate(2 minutes)")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_rules()
        .send()
        .await
        .expect("list_rules should succeed");

    assert_eq!(resp.rules().len(), 2);
}

#[tokio::test]
async fn test_delete_rule() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("del-rule")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();

    client
        .delete_rule()
        .name("del-rule")
        .send()
        .await
        .expect("delete_rule should succeed");

    let resp = client.list_rules().send().await.unwrap();
    assert_eq!(resp.rules().len(), 0);
}

#[tokio::test]
async fn test_put_and_list_targets() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("target-rule")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();

    client
        .put_targets()
        .rule("target-rule")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("target-1")
                .arn("arn:aws:lambda:us-east-1:123456789012:function:my-func")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_targets should succeed");

    let resp = client
        .list_targets_by_rule()
        .rule("target-rule")
        .send()
        .await
        .expect("list_targets_by_rule should succeed");

    assert_eq!(resp.targets().len(), 1);
    assert_eq!(resp.targets()[0].id(), "target-1");
}

#[tokio::test]
async fn test_remove_targets() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("rm-target-rule")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();

    client
        .put_targets()
        .rule("rm-target-rule")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("t1")
                .arn("arn:aws:sqs:us-east-1:123456789012:my-queue")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .remove_targets()
        .rule("rm-target-rule")
        .ids("t1")
        .send()
        .await
        .expect("remove_targets should succeed");

    let resp = client
        .list_targets_by_rule()
        .rule("rm-target-rule")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.targets().len(), 0);
}

#[tokio::test]
async fn test_put_events() {
    let client = make_events_client().await;

    let resp = client
        .put_events()
        .entries(
            aws_sdk_eventbridge::types::PutEventsRequestEntry::builder()
                .source("my.app")
                .detail_type("MyEvent")
                .detail(r#"{"key": "value"}"#)
                .build(),
        )
        .send()
        .await
        .expect("put_events should succeed");

    assert_eq!(resp.failed_entry_count(), 0);
    assert_eq!(resp.entries().len(), 1);
}

#[tokio::test]
async fn test_delete_rule_with_targets_fails() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("has-targets")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();

    client
        .put_targets()
        .rule("has-targets")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("t1")
                .arn("arn:aws:sqs:us-east-1:123456789012:q")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let err = client.delete_rule().name("has-targets").send().await;

    assert!(err.is_err(), "delete rule with targets should fail");
}

// --- EventBus operations ---

#[tokio::test]
async fn test_create_event_bus() {
    let client = make_events_client().await;

    let resp = client
        .create_event_bus()
        .name("my-bus")
        .send()
        .await
        .expect("create_event_bus should succeed");

    assert!(resp.event_bus_arn().is_some());
    assert!(resp.event_bus_arn().unwrap().contains("my-bus"));
}

#[tokio::test]
async fn test_describe_event_bus() {
    let client = make_events_client().await;

    client
        .create_event_bus()
        .name("desc-bus")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_event_bus()
        .name("desc-bus")
        .send()
        .await
        .expect("describe_event_bus should succeed");

    assert_eq!(resp.name(), Some("desc-bus"));
    assert!(resp.arn().is_some());
}

#[tokio::test]
async fn test_delete_event_bus() {
    let client = make_events_client().await;

    client
        .create_event_bus()
        .name("del-bus")
        .send()
        .await
        .unwrap();

    client
        .delete_event_bus()
        .name("del-bus")
        .send()
        .await
        .expect("delete_event_bus should succeed");

    let resp = client.list_event_buses().send().await.unwrap();
    // Only the default bus should remain
    assert!(
        resp.event_buses()
            .iter()
            .all(|b| b.name() != Some("del-bus"))
    );
}

#[tokio::test]
async fn test_list_event_buses() {
    let client = make_events_client().await;

    client
        .create_event_bus()
        .name("bus-a")
        .send()
        .await
        .unwrap();
    client
        .create_event_bus()
        .name("bus-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_event_buses()
        .send()
        .await
        .expect("list_event_buses should succeed");

    // default + bus-a + bus-b
    assert!(resp.event_buses().len() >= 3);
}

#[tokio::test]
async fn test_put_permission_and_remove_permission() {
    let client = make_events_client().await;

    client
        .put_permission()
        .event_bus_name("default")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("put_permission should succeed");

    let resp = client
        .describe_event_bus()
        .name("default")
        .send()
        .await
        .unwrap();

    assert!(resp.policy().is_some());

    client
        .remove_permission()
        .event_bus_name("default")
        .send()
        .await
        .expect("remove_permission should succeed");
}

// --- Archive operations ---

#[tokio::test]
async fn test_create_archive() {
    let client = make_events_client().await;

    let resp = client
        .create_archive()
        .archive_name("my-archive")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
        .retention_days(30)
        .send()
        .await
        .expect("create_archive should succeed");

    assert!(resp.archive_arn().is_some());
}

#[tokio::test]
async fn test_describe_archive() {
    let client = make_events_client().await;

    client
        .create_archive()
        .archive_name("desc-archive")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_archive()
        .archive_name("desc-archive")
        .send()
        .await
        .expect("describe_archive should succeed");

    assert_eq!(resp.archive_name(), Some("desc-archive"));
    assert!(resp.archive_arn().is_some());
}

#[tokio::test]
async fn test_update_archive() {
    let client = make_events_client().await;

    client
        .create_archive()
        .archive_name("upd-archive")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
        .retention_days(30)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_archive()
        .archive_name("upd-archive")
        .retention_days(60)
        .send()
        .await
        .expect("update_archive should succeed");

    assert!(resp.archive_arn().is_some());
}

#[tokio::test]
async fn test_delete_archive() {
    let client = make_events_client().await;

    client
        .create_archive()
        .archive_name("del-archive")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
        .send()
        .await
        .unwrap();

    client
        .delete_archive()
        .archive_name("del-archive")
        .send()
        .await
        .expect("delete_archive should succeed");

    let resp = client.list_archives().send().await.unwrap();
    assert!(resp.archives().is_empty());
}

#[tokio::test]
async fn test_list_archives() {
    let client = make_events_client().await;

    client
        .create_archive()
        .archive_name("arc-1")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_archives()
        .send()
        .await
        .expect("list_archives should succeed");

    assert_eq!(resp.archives().len(), 1);
}

// --- Connection operations ---

#[tokio::test]
async fn test_create_connection() {
    let client = make_events_client().await;

    let resp = client
        .create_connection()
        .name("my-conn")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("x-api-key")
                        .api_key_value("secret")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("create_connection should succeed");

    assert!(resp.connection_arn().is_some());
}

#[tokio::test]
async fn test_describe_connection() {
    let client = make_events_client().await;

    client
        .create_connection()
        .name("desc-conn")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("x-api-key")
                        .api_key_value("secret")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_connection()
        .name("desc-conn")
        .send()
        .await
        .expect("describe_connection should succeed");

    assert_eq!(resp.name(), Some("desc-conn"));
}

#[tokio::test]
async fn test_delete_connection() {
    let client = make_events_client().await;

    client
        .create_connection()
        .name("del-conn")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("x-api-key")
                        .api_key_value("secret")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_connection()
        .name("del-conn")
        .send()
        .await
        .expect("delete_connection should succeed");

    assert!(resp.connection_arn().is_some());
}

#[tokio::test]
async fn test_update_connection() {
    let client = make_events_client().await;

    client
        .create_connection()
        .name("upd-conn")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("x-api-key")
                        .api_key_value("secret")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .update_connection()
        .name("upd-conn")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::Basic)
        .send()
        .await
        .expect("update_connection should succeed");

    assert!(resp.connection_arn().is_some());
}

#[tokio::test]
async fn test_list_connections() {
    let client = make_events_client().await;

    client
        .create_connection()
        .name("list-conn")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("x-api-key")
                        .api_key_value("secret")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_connections()
        .send()
        .await
        .expect("list_connections should succeed");

    assert_eq!(resp.connections().len(), 1);
}

// --- ApiDestination operations ---

#[tokio::test]
async fn test_create_api_destination() {
    let client = make_events_client().await;

    let resp = client
        .create_api_destination()
        .name("my-dest")
        .connection_arn("arn:aws:events:us-east-1:123456789012:connection/my-conn")
        .invocation_endpoint("https://example.com/api")
        .http_method(aws_sdk_eventbridge::types::ApiDestinationHttpMethod::Post)
        .send()
        .await
        .expect("create_api_destination should succeed");

    assert!(resp.api_destination_arn().is_some());
}

#[tokio::test]
async fn test_describe_api_destination() {
    let client = make_events_client().await;

    client
        .create_api_destination()
        .name("desc-dest")
        .connection_arn("arn:aws:events:us-east-1:123456789012:connection/my-conn")
        .invocation_endpoint("https://example.com/api")
        .http_method(aws_sdk_eventbridge::types::ApiDestinationHttpMethod::Post)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_api_destination()
        .name("desc-dest")
        .send()
        .await
        .expect("describe_api_destination should succeed");

    assert_eq!(resp.name(), Some("desc-dest"));
}

#[tokio::test]
async fn test_update_api_destination() {
    let client = make_events_client().await;

    client
        .create_api_destination()
        .name("upd-dest")
        .connection_arn("arn:aws:events:us-east-1:123456789012:connection/my-conn")
        .invocation_endpoint("https://example.com/api")
        .http_method(aws_sdk_eventbridge::types::ApiDestinationHttpMethod::Post)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_api_destination()
        .name("upd-dest")
        .invocation_endpoint("https://example.com/v2/api")
        .send()
        .await
        .expect("update_api_destination should succeed");

    assert!(resp.api_destination_arn().is_some());
}

#[tokio::test]
async fn test_delete_api_destination() {
    let client = make_events_client().await;

    client
        .create_api_destination()
        .name("del-dest")
        .connection_arn("arn:aws:events:us-east-1:123456789012:connection/my-conn")
        .invocation_endpoint("https://example.com/api")
        .http_method(aws_sdk_eventbridge::types::ApiDestinationHttpMethod::Post)
        .send()
        .await
        .unwrap();

    client
        .delete_api_destination()
        .name("del-dest")
        .send()
        .await
        .expect("delete_api_destination should succeed");

    let resp = client.list_api_destinations().send().await.unwrap();
    assert!(resp.api_destinations().is_empty());
}

#[tokio::test]
async fn test_list_api_destinations() {
    let client = make_events_client().await;

    client
        .create_api_destination()
        .name("list-dest")
        .connection_arn("arn:aws:events:us-east-1:123456789012:connection/my-conn")
        .invocation_endpoint("https://example.com/api")
        .http_method(aws_sdk_eventbridge::types::ApiDestinationHttpMethod::Get)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_api_destinations()
        .send()
        .await
        .expect("list_api_destinations should succeed");

    assert_eq!(resp.api_destinations().len(), 1);
}

// --- Replay operations ---

#[tokio::test]
async fn test_start_replay() {
    let client = make_events_client().await;

    let resp = client
        .start_replay()
        .replay_name("my-replay")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("start_replay should succeed");

    assert!(resp.replay_arn().is_some());
}

#[tokio::test]
async fn test_cancel_replay() {
    let client = make_events_client().await;

    client
        .start_replay()
        .replay_name("cancel-replay")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .cancel_replay()
        .replay_name("cancel-replay")
        .send()
        .await
        .expect("cancel_replay should succeed");

    assert!(resp.replay_arn().is_some());
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::ReplayState::Cancelling)
    );
}

#[tokio::test]
async fn test_describe_replay() {
    let client = make_events_client().await;

    client
        .start_replay()
        .replay_name("desc-replay")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_replay()
        .replay_name("desc-replay")
        .send()
        .await
        .expect("describe_replay should succeed");

    assert_eq!(resp.replay_name(), Some("desc-replay"));
}

#[tokio::test]
async fn test_list_replays() {
    let client = make_events_client().await;

    client
        .start_replay()
        .replay_name("list-replay")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_replays()
        .send()
        .await
        .expect("list_replays should succeed");

    assert_eq!(resp.replays().len(), 1);
}

// --- PartnerEventSource operations ---

#[tokio::test]
async fn test_create_partner_event_source() {
    let client = make_events_client().await;

    let resp = client
        .create_partner_event_source()
        .name("aws.partner/example.com/test")
        .account("123456789012")
        .send()
        .await
        .expect("create_partner_event_source should succeed");

    assert!(resp.event_source_arn().is_some());
}

#[tokio::test]
async fn test_delete_partner_event_source() {
    let client = make_events_client().await;

    client
        .create_partner_event_source()
        .name("aws.partner/example.com/del")
        .account("123456789012")
        .send()
        .await
        .unwrap();

    client
        .delete_partner_event_source()
        .name("aws.partner/example.com/del")
        .account("123456789012")
        .send()
        .await
        .expect("delete_partner_event_source should succeed");
}

#[tokio::test]
async fn test_describe_event_source() {
    let client = make_events_client().await;

    client
        .create_partner_event_source()
        .name("aws.partner/example.com/desc")
        .account("123456789012")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_event_source()
        .name("aws.partner/example.com/desc")
        .send()
        .await
        .expect("describe_event_source should succeed");

    assert_eq!(resp.name(), Some("aws.partner/example.com/desc"));
}

#[tokio::test]
async fn test_describe_partner_event_source() {
    let client = make_events_client().await;

    client
        .create_partner_event_source()
        .name("aws.partner/example.com/pdesc")
        .account("123456789012")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_partner_event_source()
        .name("aws.partner/example.com/pdesc")
        .send()
        .await
        .expect("describe_partner_event_source should succeed");

    assert_eq!(resp.name(), Some("aws.partner/example.com/pdesc"));
}

// --- DisableRule / EnableRule ---

#[tokio::test]
async fn test_disable_and_enable_rule() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("toggle-rule")
        .schedule_expression("rate(5 minutes)")
        .send()
        .await
        .unwrap();

    client
        .disable_rule()
        .name("toggle-rule")
        .send()
        .await
        .expect("disable_rule should succeed");

    let resp = client
        .describe_rule()
        .name("toggle-rule")
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::RuleState::Disabled)
    );

    client
        .enable_rule()
        .name("toggle-rule")
        .send()
        .await
        .expect("enable_rule should succeed");

    let resp = client
        .describe_rule()
        .name("toggle-rule")
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::RuleState::Enabled)
    );
}

// --- ListRuleNamesByTarget ---

#[tokio::test]
async fn test_list_rule_names_by_target() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("target-match-rule")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();

    let target_arn = "arn:aws:lambda:us-east-1:123456789012:function:my-target";

    client
        .put_targets()
        .rule("target-match-rule")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("t1")
                .arn(target_arn)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_rule_names_by_target()
        .target_arn(target_arn)
        .send()
        .await
        .expect("list_rule_names_by_target should succeed");

    assert_eq!(resp.rule_names().len(), 1);
    assert_eq!(resp.rule_names()[0], "target-match-rule");
}

// --- Tag operations ---

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_events_client().await;

    let resp = client
        .put_rule()
        .name("tagged-rule")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();

    let arn = resp.rule_arn().unwrap();

    client
        .tag_resource()
        .resource_arn(arn)
        .tags(
            aws_sdk_eventbridge::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "env");
    assert_eq!(resp.tags()[0].value(), "test");
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_events_client().await;

    let resp = client
        .put_rule()
        .name("untag-rule")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();

    let arn = resp.rule_arn().unwrap();

    client
        .tag_resource()
        .resource_arn(arn)
        .tags(
            aws_sdk_eventbridge::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();

    assert!(resp.tags().is_empty());
}

// --- PutPartnerEvents ---

#[tokio::test]
async fn test_put_partner_events() {
    let client = make_events_client().await;

    let resp = client
        .put_partner_events()
        .entries(
            aws_sdk_eventbridge::types::PutPartnerEventsRequestEntry::builder()
                .source("partner.app")
                .detail_type("PartnerEvent")
                .detail(r#"{"key": "value"}"#)
                .build(),
        )
        .send()
        .await
        .expect("put_partner_events should succeed");

    assert_eq!(resp.failed_entry_count(), 0);
    assert_eq!(resp.entries().len(), 1);
}

// --- TestEventPattern ---

#[tokio::test]
async fn test_test_event_pattern() {
    let client = make_events_client().await;

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"source": ["aws.ec2"]}"#)
        .event(r#"{"source": "aws.ec2", "detail-type": "EC2 Instance State-change Notification"}"#)
        .send()
        .await
        .expect("test_event_pattern should succeed");

    assert!(resp.result());
}

// ============================================================================
// Ported from moto: test_events.py
// ============================================================================

// --- Event Bus tests ported from moto ---

// Ported from moto: test_events.py::test_create_event_bus_errors
#[tokio::test]
async fn test_create_event_bus_duplicate_error() {
    let client = make_events_client().await;

    client
        .create_event_bus()
        .name("test-bus")
        .send()
        .await
        .unwrap();

    let err = client
        .create_event_bus()
        .name("test-bus")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Event bus test-bus already exists"),
        "Expected already exists error, got: {err_str}"
    );

    // Default bus already exists
    let err = client
        .create_event_bus()
        .name("default")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Event bus default already exists"),
        "Expected already exists error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_describe_event_bus
#[tokio::test]
async fn test_describe_default_event_bus() {
    let client = make_events_client().await;

    let resp = client.describe_event_bus().send().await.unwrap();
    assert_eq!(resp.name(), Some("default"));
    assert!(resp.arn().unwrap().contains("event-bus/default"));
    assert!(resp.policy().is_none());
}

// Ported from moto: test_events.py::test_describe_event_bus_errors
#[tokio::test]
async fn test_describe_event_bus_not_found() {
    let client = make_events_client().await;

    let err = client
        .describe_event_bus()
        .name("non-existing")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Event bus non-existing does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_list_event_buses
#[tokio::test]
async fn test_list_event_buses_with_prefix() {
    let client = make_events_client().await;

    client
        .create_event_bus()
        .name("test-bus-1")
        .send()
        .await
        .unwrap();
    client
        .create_event_bus()
        .name("test-bus-2")
        .send()
        .await
        .unwrap();
    client
        .create_event_bus()
        .name("other-bus-1")
        .send()
        .await
        .unwrap();
    client
        .create_event_bus()
        .name("other-bus-2")
        .send()
        .await
        .unwrap();

    let resp = client.list_event_buses().send().await.unwrap();
    assert_eq!(resp.event_buses().len(), 5); // default + 4 custom

    let resp = client
        .list_event_buses()
        .name_prefix("other-bus")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.event_buses().len(), 2);
    for bus in resp.event_buses() {
        assert!(bus.name().unwrap().starts_with("other-bus"));
    }
}

// Ported from moto: test_events.py::test_delete_event_bus
#[tokio::test]
async fn test_delete_event_bus_nonexisting_succeeds() {
    let client = make_events_client().await;
    // Deleting non-existing event bus should succeed silently
    client
        .delete_event_bus()
        .name("non-existing")
        .send()
        .await
        .unwrap();
}

// Ported from moto: test_events.py::test_delete_event_bus_errors
#[tokio::test]
async fn test_delete_default_event_bus_fails() {
    let client = make_events_client().await;

    let err = client
        .delete_event_bus()
        .name("default")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Cannot delete event bus default"),
        "Expected validation error, got: {err_str}"
    );
}

// --- Permission tests ported from moto ---

// Ported from moto: test_events.py::test_permissions
#[tokio::test]
async fn test_permissions_with_statements() {
    let client = make_events_client().await;

    client
        .put_permission()
        .action("events:PutEvents")
        .principal("111111111111")
        .statement_id("Account1")
        .send()
        .await
        .unwrap();

    client
        .put_permission()
        .action("events:PutEvents")
        .principal("222222222222")
        .statement_id("Account2")
        .send()
        .await
        .unwrap();

    let resp = client.describe_event_bus().send().await.unwrap();
    let policy_str = resp.policy().expect("policy should be set");
    let policy: serde_json::Value = serde_json::from_str(policy_str).unwrap();
    assert_eq!(policy["Statement"].as_array().unwrap().len(), 2);

    client
        .remove_permission()
        .statement_id("Account2")
        .send()
        .await
        .unwrap();

    let resp = client.describe_event_bus().send().await.unwrap();
    let policy_str = resp.policy().expect("policy should still be set");
    let policy: serde_json::Value = serde_json::from_str(policy_str).unwrap();
    let statements = policy["Statement"].as_array().unwrap();
    assert_eq!(statements.len(), 1);
    assert_eq!(statements[0]["Sid"].as_str().unwrap(), "Account1");
}

// Ported from moto: test_events.py::test_permission_policy
#[tokio::test]
async fn test_put_permission_with_policy() {
    let client = make_events_client().await;

    let policy = r#"{"Statement":[{"Sid":"asdf","Action":"events:PutEvents","Principal":"111111111111","StatementId":"Account1","Effect":"n/a","Resource":"n/a"}]}"#;
    client.put_permission().policy(policy).send().await.unwrap();

    let resp = client.describe_event_bus().send().await.unwrap();
    let policy_str = resp.policy().expect("policy should be set");
    let policy_doc: serde_json::Value = serde_json::from_str(policy_str).unwrap();
    let statements = policy_doc["Statement"].as_array().unwrap();
    assert_eq!(statements.len(), 1);
    assert_eq!(statements[0]["Sid"].as_str().unwrap(), "asdf");
}

// Ported from moto: test_events.py::test_put_permission_errors
#[tokio::test]
async fn test_put_permission_errors() {
    let client = make_events_client().await;
    client
        .create_event_bus()
        .name("test-bus")
        .send()
        .await
        .unwrap();

    // Non-existing event bus
    let err = client
        .put_permission()
        .event_bus_name("non-existing")
        .action("events:PutEvents")
        .principal("111111111111")
        .statement_id("test")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Event bus non-existing does not exist"),
        "Expected not found error, got: {err_str}"
    );

    // Invalid action
    let err = client
        .put_permission()
        .event_bus_name("test-bus")
        .action("events:PutPermission")
        .principal("111111111111")
        .statement_id("test")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Provided value in parameter 'action' is not supported"),
        "Expected action validation error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_remove_permission_errors
#[tokio::test]
async fn test_remove_permission_errors() {
    let client = make_events_client().await;
    client
        .create_event_bus()
        .name("test-bus")
        .send()
        .await
        .unwrap();

    // Non-existing event bus
    let err = client
        .remove_permission()
        .event_bus_name("non-existing")
        .statement_id("test")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Event bus non-existing does not exist"),
        "Expected not found error, got: {err_str}"
    );

    // No policy on bus
    let err = client
        .remove_permission()
        .event_bus_name("test-bus")
        .statement_id("test")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("EventBus does not have a policy"),
        "Expected no policy error, got: {err_str}"
    );

    // Add a permission first
    client
        .put_permission()
        .event_bus_name("test-bus")
        .action("events:PutEvents")
        .principal("111111111111")
        .statement_id("test")
        .send()
        .await
        .unwrap();

    // Non-existing statement
    let err = client
        .remove_permission()
        .event_bus_name("test-bus")
        .statement_id("non-existing")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Statement with the provided id does not exist"),
        "Expected statement not found error, got: {err_str}"
    );
}

// --- Archive tests ported from moto ---

// Ported from moto: test_events.py::test_create_archive_error_duplicate
#[tokio::test]
async fn test_create_archive_duplicate_error() {
    let client = make_events_client().await;

    client
        .create_archive()
        .archive_name("test-archive")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
        .send()
        .await
        .unwrap();

    let err = client
        .create_archive()
        .archive_name("test-archive")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Archive test-archive already exists"),
        "Expected already exists error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_describe_archive
#[tokio::test]
async fn test_describe_archive_detailed() {
    let client = make_events_client().await;

    let source_arn = "arn:aws:events:us-east-1:123456789012:event-bus/default";
    let event_pattern = r#"{"key": ["value"]}"#;
    client
        .create_archive()
        .archive_name("test-archive")
        .event_source_arn(source_arn)
        .description("test archive")
        .event_pattern(event_pattern)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_archive()
        .archive_name("test-archive")
        .send()
        .await
        .unwrap();

    assert!(resp.archive_arn().unwrap().contains("archive/test-archive"));
    assert_eq!(resp.archive_name(), Some("test-archive"));
    assert!(resp.creation_time().is_some());
    assert_eq!(resp.description(), Some("test archive"));
    assert_eq!(resp.event_pattern(), Some(event_pattern));
    assert_eq!(resp.event_source_arn(), Some(source_arn));
    assert_eq!(resp.retention_days(), Some(0));
    assert_eq!(resp.size_bytes(), 0);
    assert_eq!(resp.state().unwrap().as_str(), "ENABLED");
}

// Ported from moto: test_events.py::test_describe_archive_error_unknown_archive
#[tokio::test]
async fn test_describe_archive_not_found() {
    let client = make_events_client().await;

    let err = client
        .describe_archive()
        .archive_name("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Archive unknown does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_list_archives_with_name_prefix
#[tokio::test]
async fn test_list_archives_with_name_prefix() {
    let client = make_events_client().await;

    let source_arn = "arn:aws:events:us-east-1:123456789012:event-bus/default";
    client
        .create_archive()
        .archive_name("test")
        .event_source_arn(source_arn)
        .send()
        .await
        .unwrap();
    client
        .create_archive()
        .archive_name("test-archive")
        .event_source_arn(source_arn)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_archives()
        .name_prefix("test-")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.archives().len(), 1);
    assert_eq!(resp.archives()[0].archive_name(), Some("test-archive"));
}

// Ported from moto: test_events.py::test_update_archive
#[tokio::test]
async fn test_update_archive_detailed() {
    let client = make_events_client().await;

    let source_arn = "arn:aws:events:us-east-1:123456789012:event-bus/default";
    client
        .create_archive()
        .archive_name("upd-archive2")
        .event_source_arn(source_arn)
        .send()
        .await
        .unwrap();

    let event_pattern = r#"{"key": ["value"]}"#;
    client
        .update_archive()
        .archive_name("upd-archive2")
        .description("test archive")
        .event_pattern(event_pattern)
        .retention_days(14)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_archive()
        .archive_name("upd-archive2")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.description(), Some("test archive"));
    assert_eq!(resp.event_pattern(), Some(event_pattern));
    assert_eq!(resp.retention_days(), Some(14));
    assert_eq!(resp.state().unwrap().as_str(), "ENABLED");
}

// Ported from moto: test_events.py::test_update_archive_error_unknown_archive
#[tokio::test]
async fn test_update_archive_not_found() {
    let client = make_events_client().await;

    let err = client
        .update_archive()
        .archive_name("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Archive unknown does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_delete_archive_error_unknown_archive
#[tokio::test]
async fn test_delete_archive_not_found() {
    let client = make_events_client().await;

    let err = client
        .delete_archive()
        .archive_name("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Archive unknown does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// --- Connection tests ported from moto ---

// Ported from moto: test_events.py::test_create_and_list_connections
#[tokio::test]
async fn test_connection_arn_includes_uuid() {
    let client = make_events_client().await;

    let resp = client
        .create_connection()
        .name("test-conn-uuid")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("x-api-key")
                        .api_key_value("secret")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    // ARN should include UUID suffix like: .../connection/test-conn-uuid/<uuid>
    let arn = resp.connection_arn().unwrap();
    assert!(arn.contains("connection/test-conn-uuid/"));
}

// Ported from moto: test_events.py::test_create_and_describe_connection
#[tokio::test]
async fn test_describe_connection_detailed() {
    let client = make_events_client().await;

    client
        .create_connection()
        .name("desc-conn2")
        .description("test description")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("x-api-key")
                        .api_key_value("secret")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_connection()
        .name("desc-conn2")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("desc-conn2"));
    assert_eq!(resp.description(), Some("test description"));
    assert_eq!(
        resp.authorization_type(),
        Some(&aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
    );
    assert_eq!(
        resp.connection_state(),
        Some(&aws_sdk_eventbridge::types::ConnectionState::Authorized)
    );
    assert!(resp.creation_time().is_some());
}

// Ported from moto: test_events.py::test_create_and_update_connection
#[tokio::test]
async fn test_update_connection_description() {
    let client = make_events_client().await;

    client
        .create_connection()
        .name("upd-conn2")
        .description("original desc")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("x-api-key")
                        .api_key_value("secret")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .update_connection()
        .name("upd-conn2")
        .description("updated desc")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_connection()
        .name("upd-conn2")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("upd-conn2"));
    assert_eq!(resp.description(), Some("updated desc"));
    assert_eq!(
        resp.authorization_type(),
        Some(&aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
    );
}

// Ported from moto: test_events.py::test_update_unknown_connection
#[tokio::test]
async fn test_update_unknown_connection() {
    let client = make_events_client().await;

    let err = client
        .update_connection()
        .name("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Connection 'unknown' does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_delete_connection
#[tokio::test]
async fn test_delete_connection_lifecycle() {
    let client = make_events_client().await;

    let conns = client.list_connections().send().await.unwrap();
    assert_eq!(conns.connections().len(), 0);

    client
        .create_connection()
        .name("del-conn2")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("x-api-key")
                        .api_key_value("secret")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let conns = client.list_connections().send().await.unwrap();
    assert_eq!(conns.connections().len(), 1);

    client
        .delete_connection()
        .name("del-conn2")
        .send()
        .await
        .unwrap();

    let conns = client.list_connections().send().await.unwrap();
    assert_eq!(conns.connections().len(), 0);
}

// Ported from moto: test_events.py::test_describe_connection_not_present
#[tokio::test]
async fn test_describe_connection_not_found() {
    let client = make_events_client().await;

    let err = client
        .describe_connection()
        .name("nonexistent")
        .send()
        .await
        .unwrap_err();
    assert!(err.into_service_error().is_resource_not_found_exception());
}

// Ported from moto: test_events.py::test_delete_connection_not_present
#[tokio::test]
async fn test_delete_connection_not_found() {
    let client = make_events_client().await;

    let err = client
        .delete_connection()
        .name("nonexistent")
        .send()
        .await
        .unwrap_err();
    assert!(err.into_service_error().is_resource_not_found_exception());
}

// --- API Destination tests ported from moto ---

// Ported from moto: test_events.py::test_create_and_list_api_destinations
#[tokio::test]
async fn test_api_destination_full_lifecycle() {
    let client = make_events_client().await;

    let conn_resp = client
        .create_connection()
        .name("api-dest-conn")
        .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
        .auth_parameters(
            aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                .api_key_auth_parameters(
                    aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                        .api_key_name("test")
                        .api_key_value("test")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();
    let conn_arn = conn_resp.connection_arn().unwrap();

    let dest_resp = client
        .create_api_destination()
        .name("test-dest2")
        .description("test-description")
        .connection_arn(conn_arn)
        .invocation_endpoint("https://example.com")
        .http_method(aws_sdk_eventbridge::types::ApiDestinationHttpMethod::Get)
        .send()
        .await
        .unwrap();

    // ARN should include UUID suffix
    let arn = dest_resp.api_destination_arn().unwrap();
    assert!(arn.contains("api-destination/test-dest2/"));

    let desc = client
        .describe_api_destination()
        .name("test-dest2")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.name(), Some("test-dest2"));
    assert_eq!(desc.description(), Some("test-description"));
    assert!(
        desc.api_destination_arn()
            .unwrap()
            .contains("api-destination/test-dest2/")
    );

    let list = client.list_api_destinations().send().await.unwrap();
    assert_eq!(list.api_destinations().len(), 1);
    assert_eq!(list.api_destinations()[0].name(), Some("test-dest2"));
}

// Ported from moto: test_events.py::test_describe_unknown_api_destination
#[tokio::test]
async fn test_describe_unknown_api_destination() {
    let client = make_events_client().await;

    let err = client
        .describe_api_destination()
        .name("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("An api-destination 'unknown' does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_delete_unknown_api_destination
#[tokio::test]
async fn test_delete_unknown_api_destination() {
    let client = make_events_client().await;

    let err = client
        .delete_api_destination()
        .name("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("An api-destination 'unknown' does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// --- Replay tests ported from moto ---

// Ported from moto: test_events.py::test_start_replay
#[tokio::test]
async fn test_start_replay_returns_starting() {
    let client = make_events_client().await;

    let resp = client
        .start_replay()
        .replay_name("starting-replay")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert!(
        resp.replay_arn()
            .unwrap()
            .contains("replay/starting-replay")
    );
    // Start replay should return STARTING
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::ReplayState::Starting)
    );
}

// Ported from moto: test_events.py::test_describe_replay
#[tokio::test]
async fn test_describe_replay_detailed() {
    let client = make_events_client().await;

    client
        .start_replay()
        .replay_name("detailed-replay")
        .description("test replay")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1612137600))
        .event_end_time(aws_smithy_types::DateTime::from_secs(1612224000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_replay()
        .replay_name("detailed-replay")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.replay_name(), Some("detailed-replay"));
    assert_eq!(resp.description(), Some("test replay"));
    assert!(
        resp.replay_arn()
            .unwrap()
            .contains("replay/detailed-replay")
    );
    assert_eq!(
        resp.event_source_arn(),
        Some("arn:aws:events:us-east-1:123456789012:archive/my-archive")
    );
    // Completed because mock is synchronous
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::ReplayState::Completed)
    );
    assert!(resp.replay_start_time().is_some());
    assert!(resp.replay_end_time().is_some());
    assert!(resp.destination().is_some());
    assert_eq!(
        resp.destination().unwrap().arn(),
        "arn:aws:events:us-east-1:123456789012:event-bus/default"
    );
}

// Ported from moto: test_events.py::test_describe_replay_error_unknown_replay
#[tokio::test]
async fn test_describe_replay_not_found() {
    let client = make_events_client().await;

    let err = client
        .describe_replay()
        .replay_name("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Replay unknown does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_start_replay_error_duplicate
#[tokio::test]
async fn test_start_replay_duplicate() {
    let client = make_events_client().await;

    client
        .start_replay()
        .replay_name("dup-replay")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let err = client
        .start_replay()
        .replay_name("dup-replay")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Replay dup-replay already exists"),
        "Expected already exists error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_cancel_replay
#[tokio::test]
async fn test_cancel_replay_detailed() {
    let client = make_events_client().await;

    client
        .start_replay()
        .replay_name("cancel-replay2")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .cancel_replay()
        .replay_name("cancel-replay2")
        .send()
        .await
        .unwrap();

    assert!(resp.replay_arn().unwrap().contains("replay/cancel-replay2"));
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::ReplayState::Cancelling)
    );

    // After cancel, describe should show CANCELLED
    let resp = client
        .describe_replay()
        .replay_name("cancel-replay2")
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::ReplayState::Cancelled)
    );
}

// Ported from moto: test_events.py::test_cancel_replay_error_unknown_replay
#[tokio::test]
async fn test_cancel_replay_not_found() {
    let client = make_events_client().await;

    let err = client
        .cancel_replay()
        .replay_name("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Replay unknown does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_cancel_replay_error_illegal_state
#[tokio::test]
async fn test_cancel_replay_illegal_state() {
    let client = make_events_client().await;

    client
        .start_replay()
        .replay_name("illegal-cancel")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Cancel once - should succeed
    client
        .cancel_replay()
        .replay_name("illegal-cancel")
        .send()
        .await
        .unwrap();

    // Cancel again - should fail with IllegalStatusException
    let err = client
        .cancel_replay()
        .replay_name("illegal-cancel")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("IllegalStatus") || err_str.contains("not in a valid state"),
        "Expected illegal state error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_list_replays_with_name_prefix
#[tokio::test]
async fn test_list_replays_with_name_prefix() {
    let client = make_events_client().await;

    client
        .start_replay()
        .replay_name("test")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(1000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(2000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .start_replay()
        .replay_name("test-replay")
        .event_source_arn("arn:aws:events:us-east-1:123456789012:archive/my-archive")
        .event_start_time(aws_smithy_types::DateTime::from_secs(3000))
        .event_end_time(aws_smithy_types::DateTime::from_secs(4000))
        .destination(
            aws_sdk_eventbridge::types::ReplayDestination::builder()
                .arn("arn:aws:events:us-east-1:123456789012:event-bus/default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_replays()
        .name_prefix("test-")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.replays().len(), 1);
    assert_eq!(resp.replays()[0].replay_name(), Some("test-replay"));
}

// --- Rule enable/disable tests ported from moto ---

// Ported from moto: test_events.py::test_enable_disable_rule
#[tokio::test]
async fn test_enable_disable_rule_with_error() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("toggle-rule2")
        .schedule_expression("rate(5 minutes)")
        .send()
        .await
        .unwrap();

    // Should start enabled
    let resp = client
        .describe_rule()
        .name("toggle-rule2")
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::RuleState::Enabled)
    );

    client
        .disable_rule()
        .name("toggle-rule2")
        .send()
        .await
        .unwrap();
    let resp = client
        .describe_rule()
        .name("toggle-rule2")
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::RuleState::Disabled)
    );

    client
        .enable_rule()
        .name("toggle-rule2")
        .send()
        .await
        .unwrap();
    let resp = client
        .describe_rule()
        .name("toggle-rule2")
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.state(),
        Some(&aws_sdk_eventbridge::types::RuleState::Enabled)
    );

    // Test enable unknown rule
    let err = client.enable_rule().name("junk").send().await.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected not found error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_disable_unknown_rule
#[tokio::test]
async fn test_disable_unknown_rule() {
    let client = make_events_client().await;

    let err = client
        .disable_rule()
        .name("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Rule unknown does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// --- Delete rule tests ported from moto ---

// Ported from moto: test_events.py::test_delete_rule_with_targets
#[tokio::test]
async fn test_delete_rule_with_targets_error_message() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("has-targets2")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();

    client
        .put_targets()
        .rule("has-targets2")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("t1")
                .arn("arn:aws:sqs:us-east-1:123456789012:q")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let err = client
        .delete_rule()
        .name("has-targets2")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Rule can't be deleted since it has targets"),
        "Expected validation error about targets, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_delete_unknown_rule
#[tokio::test]
async fn test_delete_unknown_rule_succeeds() {
    let client = make_events_client().await;
    // Deleting unknown rule should succeed silently (verified against AWS)
    client.delete_rule().name("unknown").send().await.unwrap();
}

// --- Tag tests ported from moto ---

// Ported from moto: test_events.py::test_rule_tagging_happy
#[tokio::test]
async fn test_rule_tagging_add_and_remove() {
    let client = make_events_client().await;

    let resp = client
        .put_rule()
        .name("tag-test-rule")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();
    let arn = resp.rule_arn().unwrap();

    // Add two tags
    client
        .tag_resource()
        .resource_arn(arn)
        .tags(
            aws_sdk_eventbridge::types::Tag::builder()
                .key("key1")
                .value("value1")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_eventbridge::types::Tag::builder()
                .key("key2")
                .value("value2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);

    // Remove one tag
    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("key1")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "key2");
    assert_eq!(resp.tags()[0].value(), "value2");
}

// --- PutRule update keeps targets ---

// Ported from moto: test_events.py::test_update_rule_with_targets
#[tokio::test]
async fn test_update_rule_preserves_targets() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("rule-with-targets")
        .schedule_expression("rate(5 minutes)")
        .send()
        .await
        .unwrap();

    client
        .put_targets()
        .rule("rule-with-targets")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("t1")
                .arn("arn:aws:lambda:us-east-1:111111111111:function:test-function-1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let targets = client
        .list_targets_by_rule()
        .rule("rule-with-targets")
        .send()
        .await
        .unwrap();
    assert_eq!(targets.targets().len(), 1);

    // Update rule schedule - targets should be preserved
    client
        .put_rule()
        .name("rule-with-targets")
        .schedule_expression("rate(1 minute)")
        .send()
        .await
        .unwrap();

    let targets = client
        .list_targets_by_rule()
        .rule("rule-with-targets")
        .send()
        .await
        .unwrap();
    assert_eq!(targets.targets().len(), 1);
    assert_eq!(targets.targets()[0].id(), "t1");
}

// --- TestEventPattern tests ported from moto ---

// Ported from moto: test_event_pattern.py::test_event_pattern_with_allowed_values_event_filter
#[tokio::test]
async fn test_event_pattern_matching() {
    let client = make_events_client().await;

    // Match by source
    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"source": ["foo", "bar"]}"#)
        .event(r#"{"source": "foo"}"#)
        .send()
        .await
        .unwrap();
    assert!(resp.result());

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"source": ["foo", "bar"]}"#)
        .event(r#"{"source": "bar"}"#)
        .send()
        .await
        .unwrap();
    assert!(resp.result());

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"source": ["foo", "bar"]}"#)
        .event(r#"{"source": "baz"}"#)
        .send()
        .await
        .unwrap();
    assert!(!resp.result());
}

// Ported from moto: test_event_pattern.py::test_event_pattern_with_nested_event_filter
#[tokio::test]
async fn test_event_pattern_nested() {
    let client = make_events_client().await;

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": ["bar"]}}"#)
        .event(r#"{"detail": {"foo": "bar"}}"#)
        .send()
        .await
        .unwrap();
    assert!(resp.result());

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": ["bar"]}}"#)
        .event(r#"{"detail": {"foo": "baz"}}"#)
        .send()
        .await
        .unwrap();
    assert!(!resp.result());
}

// Ported from moto: test_event_pattern.py::test_event_pattern_with_prefix_event_filter
#[tokio::test]
async fn test_event_pattern_prefix() {
    let client = make_events_client().await;

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"prefix": "bar"}]}}"#)
        .event(r#"{"detail": {"foo": "bar"}}"#)
        .send()
        .await
        .unwrap();
    assert!(resp.result());

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"prefix": "bar"}]}}"#)
        .event(r#"{"detail": {"foo": "bar!"}}"#)
        .send()
        .await
        .unwrap();
    assert!(resp.result());

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"prefix": "bar"}]}}"#)
        .event(r#"{"detail": {"foo": "ba"}}"#)
        .send()
        .await
        .unwrap();
    assert!(!resp.result());
}

// Ported from moto: test_event_pattern.py::test_event_pattern_with_single_numeric_event_filter
#[tokio::test]
async fn test_event_pattern_numeric() {
    let client = make_events_client().await;

    // > 1
    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"numeric": [">", 1]}]}}"#)
        .event(r#"{"detail": {"foo": 2}}"#)
        .send()
        .await
        .unwrap();
    assert!(resp.result());

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"numeric": [">", 1]}]}}"#)
        .event(r#"{"detail": {"foo": 0}}"#)
        .send()
        .await
        .unwrap();
    assert!(!resp.result());

    // Range: >= 1 and < 3
    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"numeric": [">=", 1, "<", 3]}]}}"#)
        .event(r#"{"detail": {"foo": 2}}"#)
        .send()
        .await
        .unwrap();
    assert!(resp.result());

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"numeric": [">=", 1, "<", 3]}]}}"#)
        .event(r#"{"detail": {"foo": 3}}"#)
        .send()
        .await
        .unwrap();
    assert!(!resp.result());
}

// Ported from moto: test_event_pattern.py::test_event_pattern_with_exists_event_filter
#[tokio::test]
async fn test_event_pattern_exists() {
    let client = make_events_client().await;

    // exists: true
    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"exists": true}]}}"#)
        .event(r#"{"detail": {"foo": "bar"}}"#)
        .send()
        .await
        .unwrap();
    assert!(resp.result());

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"exists": true}]}}"#)
        .event(r#"{"detail": {}}"#)
        .send()
        .await
        .unwrap();
    assert!(!resp.result());

    // exists: false
    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"exists": false}]}}"#)
        .event(r#"{"detail": {"foo": "bar"}}"#)
        .send()
        .await
        .unwrap();
    assert!(!resp.result());

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"detail": {"foo": [{"exists": false}]}}"#)
        .event(r#"{"detail": {}}"#)
        .send()
        .await
        .unwrap();
    assert!(resp.result());
}

// --- Partner Event Source tests ported from moto ---

// Ported from moto: test existing partner tests with error cases
#[tokio::test]
async fn test_partner_event_source_not_found() {
    let client = make_events_client().await;

    let err = client
        .describe_event_source()
        .name("aws.partner/nonexistent/test")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// --- PutTargets/RemoveTargets error tests ---

// Ported from moto: test_events.py::test_put_targets_error_unknown_rule
#[tokio::test]
async fn test_put_targets_unknown_rule() {
    let client = make_events_client().await;

    let err = client
        .put_targets()
        .rule("unknown")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("s3")
                .arn("arn:aws:s3:::test-bucket")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound") || err_str.contains("does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

// Ported from moto: test_events.py::test_remove_targets_error_unknown_rule
#[tokio::test]
async fn test_remove_targets_unknown_rule() {
    let client = make_events_client().await;

    let err = client
        .remove_targets()
        .rule("unknown")
        .ids("something")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound") || err_str.contains("does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_rules_with_name_prefix() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("prod-rule-1")
        .schedule_expression("rate(1 minute)")
        .send()
        .await
        .unwrap();
    client
        .put_rule()
        .name("prod-rule-2")
        .schedule_expression("rate(2 minutes)")
        .send()
        .await
        .unwrap();
    client
        .put_rule()
        .name("dev-rule-1")
        .schedule_expression("rate(5 minutes)")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_rules()
        .name_prefix("prod-")
        .send()
        .await
        .expect("list_rules with prefix should succeed");

    assert_eq!(resp.rules().len(), 2);
    for rule in resp.rules() {
        assert!(rule.name().unwrap().starts_with("prod-"));
    }
}

#[tokio::test]
async fn test_describe_rule_not_found() {
    let client = make_events_client().await;

    let err = client
        .describe_rule()
        .name("nonexistent-rule")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound") || err_str.contains("does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_rule_nonexistent_succeeds() {
    // AWS EventBridge silently succeeds when deleting a non-existent rule
    let client = make_events_client().await;

    client
        .delete_rule()
        .name("nonexistent-rule")
        .send()
        .await
        .expect("delete_rule for non-existent rule should succeed silently");
}

#[tokio::test]
async fn test_put_rule_with_disabled_state() {
    let client = make_events_client().await;

    let resp = client
        .put_rule()
        .name("disabled-rule")
        .schedule_expression("rate(1 hour)")
        .state(aws_sdk_eventbridge::types::RuleState::Disabled)
        .send()
        .await
        .expect("put_rule with DISABLED state should succeed");

    assert!(resp.rule_arn().is_some());

    let desc = client
        .describe_rule()
        .name("disabled-rule")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.state(),
        Some(&aws_sdk_eventbridge::types::RuleState::Disabled)
    );
}

#[tokio::test]
async fn test_put_rule_is_idempotent_and_updates() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("idem-rule")
        .schedule_expression("rate(1 minute)")
        .description("original description")
        .send()
        .await
        .unwrap();

    // Overwrite with new description and different schedule
    client
        .put_rule()
        .name("idem-rule")
        .schedule_expression("rate(10 minutes)")
        .description("updated description")
        .send()
        .await
        .expect("second put_rule on same name should succeed");

    let resp = client
        .describe_rule()
        .name("idem-rule")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.description(), Some("updated description"));
    assert_eq!(resp.schedule_expression(), Some("rate(10 minutes)"));

    // Only one rule should exist
    let list = client.list_rules().send().await.unwrap();
    assert_eq!(list.rules().len(), 1);
}

#[tokio::test]
async fn test_put_targets_on_nonexistent_rule_fails() {
    let client = make_events_client().await;

    let err = client
        .put_targets()
        .rule("ghost-rule")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("t1")
                .arn("arn:aws:sqs:us-east-1:123456789012:my-queue")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound") || err_str.contains("does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_targets_by_rule_nonexistent_fails() {
    let client = make_events_client().await;

    let err = client
        .list_targets_by_rule()
        .rule("ghost-rule")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound") || err_str.contains("does not exist"),
        "Expected not found error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_events_multiple_entries() {
    let client = make_events_client().await;

    let resp = client
        .put_events()
        .entries(
            aws_sdk_eventbridge::types::PutEventsRequestEntry::builder()
                .source("my.app")
                .detail_type("EventTypeA")
                .detail(r#"{"seq": 1}"#)
                .build(),
        )
        .entries(
            aws_sdk_eventbridge::types::PutEventsRequestEntry::builder()
                .source("my.app")
                .detail_type("EventTypeB")
                .detail(r#"{"seq": 2}"#)
                .build(),
        )
        .entries(
            aws_sdk_eventbridge::types::PutEventsRequestEntry::builder()
                .source("my.app")
                .detail_type("EventTypeC")
                .detail(r#"{"seq": 3}"#)
                .build(),
        )
        .send()
        .await
        .expect("put_events with multiple entries should succeed");

    assert_eq!(resp.failed_entry_count(), 0);
    assert_eq!(resp.entries().len(), 3);
    // Each entry should have an event_id
    for entry in resp.entries() {
        assert!(
            entry.event_id().is_some(),
            "each entry should have an event_id"
        );
    }
}

#[tokio::test]
async fn test_test_event_pattern_no_match() {
    let client = make_events_client().await;

    let resp = client
        .test_event_pattern()
        .event_pattern(r#"{"source": ["aws.s3"]}"#)
        .event(r#"{"source": "aws.ec2", "detail-type": "EC2 Instance State-change Notification"}"#)
        .send()
        .await
        .expect("test_event_pattern should succeed");

    assert!(!resp.result());
}

#[tokio::test]
async fn test_list_connections_with_name_prefix() {
    let client = make_events_client().await;

    for name in &["prod-conn-1", "prod-conn-2", "staging-conn-1"] {
        client
            .create_connection()
            .name(*name)
            .authorization_type(aws_sdk_eventbridge::types::ConnectionAuthorizationType::ApiKey)
            .auth_parameters(
                aws_sdk_eventbridge::types::CreateConnectionAuthRequestParameters::builder()
                    .api_key_auth_parameters(
                        aws_sdk_eventbridge::types::CreateConnectionApiKeyAuthRequestParameters::builder()
                            .api_key_name("x-api-key")
                            .api_key_value("secret")
                            .build()
                            .unwrap(),
                    )
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_connections()
        .name_prefix("prod-")
        .send()
        .await
        .expect("list_connections with prefix should succeed");

    assert_eq!(resp.connections().len(), 2);
    for conn in resp.connections() {
        assert!(conn.name().unwrap().starts_with("prod-"));
    }
}

#[tokio::test]
async fn test_list_api_destinations_with_name_prefix() {
    let client = make_events_client().await;

    for (name, endpoint) in &[
        ("prod-dest-1", "https://example.com/prod/1"),
        ("prod-dest-2", "https://example.com/prod/2"),
        ("dev-dest-1", "https://example.com/dev/1"),
    ] {
        client
            .create_api_destination()
            .name(*name)
            .connection_arn("arn:aws:events:us-east-1:123456789012:connection/my-conn")
            .invocation_endpoint(*endpoint)
            .http_method(aws_sdk_eventbridge::types::ApiDestinationHttpMethod::Post)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_api_destinations()
        .name_prefix("prod-")
        .send()
        .await
        .expect("list_api_destinations with prefix should succeed");

    assert_eq!(resp.api_destinations().len(), 2);
    for dest in resp.api_destinations() {
        assert!(dest.name().unwrap().starts_with("prod-"));
    }
}

#[tokio::test]
async fn test_multiple_tags_and_partial_untag() {
    let client = make_events_client().await;

    let resp = client
        .put_rule()
        .name("multi-tag-rule")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();
    let arn = resp.rule_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_eventbridge::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_eventbridge::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_eventbridge::types::Tag::builder()
                .key("cost-center")
                .value("123")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource with multiple tags should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 3);

    // Remove only two of the three tags
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "cost-center");
    assert_eq!(resp.tags()[0].value(), "123");
}

#[tokio::test]
async fn test_list_rule_names_by_target_no_match() {
    let client = make_events_client().await;

    let resp = client
        .list_rule_names_by_target()
        .target_arn("arn:aws:lambda:us-east-1:123456789012:function:nobody-uses-this")
        .send()
        .await
        .expect("list_rule_names_by_target should succeed with no matches");

    assert_eq!(resp.rule_names().len(), 0);
}

#[tokio::test]
async fn test_put_rule_with_event_pattern_only() {
    let client = make_events_client().await;

    let resp = client
        .put_rule()
        .name("pattern-only-rule")
        .event_pattern(r#"{"source": ["aws.s3"], "detail-type": ["Object Created"]}"#)
        .send()
        .await
        .expect("put_rule with event_pattern only should succeed");

    assert!(resp.rule_arn().is_some());

    let desc = client
        .describe_rule()
        .name("pattern-only-rule")
        .send()
        .await
        .unwrap();
    assert!(desc.event_pattern().is_some());
    assert!(desc.schedule_expression().is_none());
}

#[tokio::test]
async fn test_put_multiple_targets_and_remove_one() {
    let client = make_events_client().await;

    client
        .put_rule()
        .name("multi-target-rule")
        .schedule_expression("rate(1 hour)")
        .send()
        .await
        .unwrap();

    client
        .put_targets()
        .rule("multi-target-rule")
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("t1")
                .arn("arn:aws:sqs:us-east-1:123456789012:queue-1")
                .build()
                .unwrap(),
        )
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("t2")
                .arn("arn:aws:sqs:us-east-1:123456789012:queue-2")
                .build()
                .unwrap(),
        )
        .targets(
            aws_sdk_eventbridge::types::Target::builder()
                .id("t3")
                .arn("arn:aws:lambda:us-east-1:123456789012:function:handler")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_targets with multiple targets should succeed");

    let list = client
        .list_targets_by_rule()
        .rule("multi-target-rule")
        .send()
        .await
        .unwrap();
    assert_eq!(list.targets().len(), 3);

    // Remove one target
    client
        .remove_targets()
        .rule("multi-target-rule")
        .ids("t2")
        .send()
        .await
        .expect("remove_targets should succeed");

    let list = client
        .list_targets_by_rule()
        .rule("multi-target-rule")
        .send()
        .await
        .unwrap();
    assert_eq!(list.targets().len(), 2);
    let ids: Vec<&str> = list.targets().iter().map(|t| t.id()).collect();
    assert!(ids.contains(&"t1"));
    assert!(ids.contains(&"t3"));
    assert!(!ids.contains(&"t2"));
}
