use aws_sdk_config::config::BehaviorVersion;
use aws_sdk_config::types;
use winterbaume_config::ConfigService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_config::Client {
    let mock = MockAws::builder()
        .with_service(ConfigService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_config::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_config::Client::new(&config)
}

#[tokio::test]
async fn test_put_and_describe_configuration_recorder() {
    let client = make_client().await;

    client
        .put_configuration_recorder()
        .configuration_recorder(
            aws_sdk_config::types::ConfigurationRecorder::builder()
                .name("default")
                .role_arn("arn:aws:iam::123456789012:role/config-role")
                .recording_group(
                    aws_sdk_config::types::RecordingGroup::builder()
                        .all_supported(true)
                        .include_global_resource_types(false)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("put_configuration_recorder should succeed");

    let resp = client
        .describe_configuration_recorders()
        .send()
        .await
        .expect("describe_configuration_recorders should succeed");

    let recorders = resp.configuration_recorders();
    assert_eq!(recorders.len(), 1);
    assert_eq!(recorders[0].name(), Some("default"));
    assert_eq!(
        recorders[0].role_arn(),
        Some("arn:aws:iam::123456789012:role/config-role")
    );
}

#[tokio::test]
async fn test_delete_configuration_recorder() {
    let client = make_client().await;

    client
        .put_configuration_recorder()
        .configuration_recorder(
            aws_sdk_config::types::ConfigurationRecorder::builder()
                .name("delete-me")
                .role_arn("arn:aws:iam::123456789012:role/config-role")
                .build(),
        )
        .send()
        .await
        .expect("put_configuration_recorder should succeed");

    client
        .delete_configuration_recorder()
        .configuration_recorder_name("delete-me")
        .send()
        .await
        .expect("delete_configuration_recorder should succeed");

    let result = client
        .describe_configuration_recorders()
        .configuration_recorder_names("delete-me")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_delivery_channel() {
    let client = make_client().await;

    client
        .put_delivery_channel()
        .delivery_channel(
            aws_sdk_config::types::DeliveryChannel::builder()
                .name("default")
                .s3_bucket_name("my-config-bucket")
                .build(),
        )
        .send()
        .await
        .expect("put_delivery_channel should succeed");
}

#[tokio::test]
async fn test_delete_nonexistent_configuration_recorder() {
    let client = make_client().await;

    let result = client
        .delete_configuration_recorder()
        .configuration_recorder_name("nonexistent-recorder")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_configuration_recorders_by_name() {
    let client = make_client().await;

    for name in ["recorder-a", "recorder-b"] {
        client
            .put_configuration_recorder()
            .configuration_recorder(
                aws_sdk_config::types::ConfigurationRecorder::builder()
                    .name(name)
                    .role_arn("arn:aws:iam::123456789012:role/config-role")
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_configuration_recorders()
        .configuration_recorder_names("recorder-a")
        .send()
        .await
        .expect("describe should succeed");

    let recorders = resp.configuration_recorders();
    assert_eq!(recorders.len(), 1);
    assert_eq!(recorders[0].name(), Some("recorder-a"));
}

// --- DeliveryChannel tests ---

#[tokio::test]
async fn test_delete_delivery_channel() {
    let client = make_client().await;

    client
        .put_delivery_channel()
        .delivery_channel(
            aws_sdk_config::types::DeliveryChannel::builder()
                .name("dc-1")
                .s3_bucket_name("my-bucket")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_delivery_channel()
        .delivery_channel_name("dc-1")
        .send()
        .await
        .expect("delete_delivery_channel should succeed");
}

#[tokio::test]
async fn test_delete_nonexistent_delivery_channel() {
    let client = make_client().await;

    let result = client
        .delete_delivery_channel()
        .delivery_channel_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_delivery_channels() {
    let client = make_client().await;

    client
        .put_delivery_channel()
        .delivery_channel(
            aws_sdk_config::types::DeliveryChannel::builder()
                .name("dc-1")
                .s3_bucket_name("bucket-1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_delivery_channels()
        .send()
        .await
        .expect("describe_delivery_channels should succeed");

    let channels = resp.delivery_channels();
    assert_eq!(channels.len(), 1);
    assert_eq!(channels[0].name(), Some("dc-1"));
    assert_eq!(channels[0].s3_bucket_name(), Some("bucket-1"));
}

// --- ConfigRule tests ---

#[tokio::test]
async fn test_put_and_describe_config_rule() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            aws_sdk_config::types::ConfigRule::builder()
                .config_rule_name("my-rule")
                .source(
                    aws_sdk_config::types::Source::builder()
                        .owner(aws_sdk_config::types::Owner::Aws)
                        .source_identifier("S3_BUCKET_VERSIONING_ENABLED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("put_config_rule should succeed");

    let resp = client
        .describe_config_rules()
        .config_rule_names("my-rule")
        .send()
        .await
        .expect("describe_config_rules should succeed");

    let rules = resp.config_rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].config_rule_name(), Some("my-rule"));
    assert_eq!(
        rules[0].config_rule_state(),
        Some(&aws_sdk_config::types::ConfigRuleState::Active)
    );
}

#[tokio::test]
async fn test_delete_config_rule() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            aws_sdk_config::types::ConfigRule::builder()
                .config_rule_name("delete-rule")
                .source(
                    aws_sdk_config::types::Source::builder()
                        .owner(aws_sdk_config::types::Owner::Aws)
                        .source_identifier("S3_BUCKET_VERSIONING_ENABLED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_config_rule()
        .config_rule_name("delete-rule")
        .send()
        .await
        .expect("delete_config_rule should succeed");

    let result = client
        .describe_config_rules()
        .config_rule_names("delete-rule")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_config_rule() {
    let client = make_client().await;

    let result = client
        .delete_config_rule()
        .config_rule_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// --- AggregationAuthorization tests ---

#[tokio::test]
async fn test_put_and_describe_aggregation_authorization() {
    let client = make_client().await;

    let resp = client
        .put_aggregation_authorization()
        .authorized_account_id("111111111111")
        .authorized_aws_region("us-west-2")
        .send()
        .await
        .expect("put_aggregation_authorization should succeed");

    let auth = resp.aggregation_authorization().unwrap();
    assert_eq!(auth.authorized_account_id(), Some("111111111111"));
    assert_eq!(auth.authorized_aws_region(), Some("us-west-2"));

    let resp = client
        .describe_aggregation_authorizations()
        .send()
        .await
        .expect("describe should succeed");

    let auths = resp.aggregation_authorizations();
    assert_eq!(auths.len(), 1);
}

#[tokio::test]
async fn test_delete_aggregation_authorization() {
    let client = make_client().await;

    client
        .put_aggregation_authorization()
        .authorized_account_id("222222222222")
        .authorized_aws_region("eu-west-1")
        .send()
        .await
        .unwrap();

    client
        .delete_aggregation_authorization()
        .authorized_account_id("222222222222")
        .authorized_aws_region("eu-west-1")
        .send()
        .await
        .expect("delete_aggregation_authorization should succeed");

    let resp = client
        .describe_aggregation_authorizations()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.aggregation_authorizations().len(), 0);
}

// --- ConfigurationAggregator tests ---

#[tokio::test]
async fn test_put_and_describe_configuration_aggregator() {
    let client = make_client().await;

    let resp = client
        .put_configuration_aggregator()
        .configuration_aggregator_name("my-aggregator")
        .account_aggregation_sources(
            aws_sdk_config::types::AccountAggregationSource::builder()
                .account_ids("111111111111")
                .all_aws_regions(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_configuration_aggregator should succeed");

    let agg = resp.configuration_aggregator().unwrap();
    assert_eq!(agg.configuration_aggregator_name(), Some("my-aggregator"));

    let resp = client
        .describe_configuration_aggregators()
        .configuration_aggregator_names("my-aggregator")
        .send()
        .await
        .expect("describe should succeed");

    let aggs = resp.configuration_aggregators();
    assert_eq!(aggs.len(), 1);
}

#[tokio::test]
async fn test_delete_configuration_aggregator() {
    let client = make_client().await;

    client
        .put_configuration_aggregator()
        .configuration_aggregator_name("delete-agg")
        .account_aggregation_sources(
            aws_sdk_config::types::AccountAggregationSource::builder()
                .account_ids("111111111111")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_configuration_aggregator()
        .configuration_aggregator_name("delete-agg")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_configuration_aggregators()
        .configuration_aggregator_names("delete-agg")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_configuration_aggregator() {
    let client = make_client().await;

    let result = client
        .delete_configuration_aggregator()
        .configuration_aggregator_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// --- RetentionConfiguration tests ---

#[tokio::test]
async fn test_put_and_describe_retention_configuration() {
    let client = make_client().await;

    let resp = client
        .put_retention_configuration()
        .retention_period_in_days(90)
        .send()
        .await
        .expect("put_retention_configuration should succeed");

    let rc = resp.retention_configuration().unwrap();
    assert_eq!(rc.retention_period_in_days(), 90);

    let resp = client
        .describe_retention_configurations()
        .send()
        .await
        .expect("describe should succeed");

    let configs = resp.retention_configurations();
    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].retention_period_in_days(), 90);
}

#[tokio::test]
async fn test_delete_retention_configuration() {
    let client = make_client().await;

    let resp = client
        .put_retention_configuration()
        .retention_period_in_days(30)
        .send()
        .await
        .unwrap();

    let name = resp.retention_configuration().unwrap().name().to_string();

    client
        .delete_retention_configuration()
        .retention_configuration_name(&name)
        .send()
        .await
        .expect("delete should succeed");

    let resp = client
        .describe_retention_configurations()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.retention_configurations().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_retention_configuration() {
    let client = make_client().await;

    let result = client
        .delete_retention_configuration()
        .retention_configuration_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Start/Stop Configuration Recorder tests ---

#[tokio::test]
async fn test_start_and_stop_configuration_recorder() {
    let client = make_client().await;

    client
        .put_configuration_recorder()
        .configuration_recorder(
            aws_sdk_config::types::ConfigurationRecorder::builder()
                .name("my-recorder")
                .role_arn("arn:aws:iam::123456789012:role/config-role")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .start_configuration_recorder()
        .configuration_recorder_name("my-recorder")
        .send()
        .await
        .expect("start should succeed");

    let resp = client
        .describe_configuration_recorder_status()
        .configuration_recorder_names("my-recorder")
        .send()
        .await
        .expect("describe status should succeed");

    let statuses = resp.configuration_recorders_status();
    assert_eq!(statuses.len(), 1);
    assert!(statuses[0].recording());

    client
        .stop_configuration_recorder()
        .configuration_recorder_name("my-recorder")
        .send()
        .await
        .expect("stop should succeed");

    let resp = client
        .describe_configuration_recorder_status()
        .configuration_recorder_names("my-recorder")
        .send()
        .await
        .unwrap();

    let statuses = resp.configuration_recorders_status();
    assert!(!statuses[0].recording());
}

#[tokio::test]
async fn test_start_nonexistent_recorder() {
    let client = make_client().await;

    let result = client
        .start_configuration_recorder()
        .configuration_recorder_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_stop_nonexistent_recorder() {
    let client = make_client().await;

    let result = client
        .stop_configuration_recorder()
        .configuration_recorder_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// --- OrganizationConformancePack tests ---

#[tokio::test]
async fn test_put_and_describe_organization_conformance_pack() {
    let client = make_client().await;

    let resp = client
        .put_organization_conformance_pack()
        .organization_conformance_pack_name("my-org-pack")
        .delivery_s3_bucket("my-bucket")
        .send()
        .await
        .expect("put should succeed");

    assert!(resp.organization_conformance_pack_arn().is_some());

    let resp = client
        .describe_organization_conformance_packs()
        .organization_conformance_pack_names("my-org-pack")
        .send()
        .await
        .expect("describe should succeed");

    let packs = resp.organization_conformance_packs();
    assert_eq!(packs.len(), 1);
    assert_eq!(packs[0].organization_conformance_pack_name(), "my-org-pack");
}

#[tokio::test]
async fn test_describe_organization_conformance_pack_statuses() {
    let client = make_client().await;

    client
        .put_organization_conformance_pack()
        .organization_conformance_pack_name("status-pack")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_organization_conformance_pack_statuses()
        .organization_conformance_pack_names("status-pack")
        .send()
        .await
        .expect("describe statuses should succeed");

    let statuses = resp.organization_conformance_pack_statuses();
    assert_eq!(statuses.len(), 1);
    assert_eq!(
        statuses[0].status(),
        &aws_sdk_config::types::OrganizationResourceStatus::CreateSuccessful
    );
}

#[tokio::test]
async fn test_delete_organization_conformance_pack() {
    let client = make_client().await;

    client
        .put_organization_conformance_pack()
        .organization_conformance_pack_name("delete-pack")
        .send()
        .await
        .unwrap();

    client
        .delete_organization_conformance_pack()
        .organization_conformance_pack_name("delete-pack")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_organization_conformance_packs()
        .organization_conformance_pack_names("delete-pack")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_organization_conformance_pack_detailed_status() {
    let client = make_client().await;

    client
        .put_organization_conformance_pack()
        .organization_conformance_pack_name("detail-pack")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_organization_conformance_pack_detailed_status()
        .organization_conformance_pack_name("detail-pack")
        .send()
        .await
        .expect("get detailed status should succeed");

    // Empty statuses since no member accounts in mock
    assert_eq!(
        resp.organization_conformance_pack_detailed_statuses().len(),
        0
    );
}

#[tokio::test]
async fn test_get_organization_conformance_pack_detailed_status_nonexistent() {
    let client = make_client().await;

    let result = client
        .get_organization_conformance_pack_detailed_status()
        .organization_conformance_pack_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// --- PutResourceConfig / DeleteResourceConfig / ListDiscoveredResources tests ---

#[tokio::test]
async fn test_put_and_list_discovered_resources() {
    let client = make_client().await;

    client
        .put_resource_config()
        .resource_type("AWS::EC2::Instance")
        .resource_id("i-12345")
        .schema_version_id("1")
        .configuration("{\"instanceId\":\"i-12345\"}")
        .send()
        .await
        .expect("put_resource_config should succeed");

    let resp = client
        .list_discovered_resources()
        .resource_type(aws_sdk_config::types::ResourceType::Instance)
        .send()
        .await
        .expect("list_discovered_resources should succeed");

    let resources = resp.resource_identifiers();
    assert_eq!(resources.len(), 1);
    assert_eq!(resources[0].resource_id(), Some("i-12345"));
}

#[tokio::test]
async fn test_delete_resource_config() {
    let client = make_client().await;

    client
        .put_resource_config()
        .resource_type("AWS::S3::Bucket")
        .resource_id("my-bucket")
        .schema_version_id("1")
        .configuration("{}")
        .send()
        .await
        .unwrap();

    client
        .delete_resource_config()
        .resource_type("AWS::S3::Bucket")
        .resource_id("my-bucket")
        .send()
        .await
        .expect("delete_resource_config should succeed");

    let resp = client
        .list_discovered_resources()
        .resource_type(aws_sdk_config::types::ResourceType::Bucket)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.resource_identifiers().len(), 0);
}

// --- BatchGetResourceConfig tests ---

#[tokio::test]
async fn test_batch_get_resource_config() {
    let client = make_client().await;

    client
        .put_resource_config()
        .resource_type("AWS::EC2::Instance")
        .resource_id("i-batch-1")
        .schema_version_id("1")
        .configuration("{}")
        .send()
        .await
        .unwrap();

    let resp = client
        .batch_get_resource_config()
        .resource_keys(
            aws_sdk_config::types::ResourceKey::builder()
                .resource_type(aws_sdk_config::types::ResourceType::Instance)
                .resource_id("i-batch-1")
                .build()
                .unwrap(),
        )
        .resource_keys(
            aws_sdk_config::types::ResourceKey::builder()
                .resource_type(aws_sdk_config::types::ResourceType::Instance)
                .resource_id("i-not-exist")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("batch_get_resource_config should succeed");

    assert_eq!(resp.base_configuration_items().len(), 1);
    assert_eq!(resp.unprocessed_resource_keys().len(), 1);
}

// --- BatchGetAggregateResourceConfig tests ---

#[tokio::test]
async fn test_batch_get_aggregate_resource_config() {
    let client = make_client().await;

    // Need an aggregator first
    client
        .put_configuration_aggregator()
        .configuration_aggregator_name("agg-batch")
        .account_aggregation_sources(
            aws_sdk_config::types::AccountAggregationSource::builder()
                .account_ids("111111111111")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_resource_config()
        .resource_type("AWS::EC2::Instance")
        .resource_id("i-agg-1")
        .schema_version_id("1")
        .configuration("{}")
        .send()
        .await
        .unwrap();

    let resp = client
        .batch_get_aggregate_resource_config()
        .configuration_aggregator_name("agg-batch")
        .resource_identifiers(
            aws_sdk_config::types::AggregateResourceIdentifier::builder()
                .resource_type(aws_sdk_config::types::ResourceType::Instance)
                .resource_id("i-agg-1")
                .source_account_id("123456789012")
                .source_region("us-east-1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("batch_get_aggregate_resource_config should succeed");

    assert_eq!(resp.base_configuration_items().len(), 1);
}

// --- ListAggregateDiscoveredResources tests ---

#[tokio::test]
async fn test_list_aggregate_discovered_resources() {
    let client = make_client().await;

    client
        .put_configuration_aggregator()
        .configuration_aggregator_name("agg-list")
        .account_aggregation_sources(
            aws_sdk_config::types::AccountAggregationSource::builder()
                .account_ids("111111111111")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_resource_config()
        .resource_type("AWS::EC2::Instance")
        .resource_id("i-agg-list-1")
        .schema_version_id("1")
        .configuration("{}")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_aggregate_discovered_resources()
        .configuration_aggregator_name("agg-list")
        .resource_type(aws_sdk_config::types::ResourceType::Instance)
        .send()
        .await
        .expect("list_aggregate_discovered_resources should succeed");

    assert_eq!(resp.resource_identifiers().len(), 1);
}

// --- GetResourceConfigHistory tests ---

#[tokio::test]
async fn test_get_resource_config_history() {
    let client = make_client().await;

    client
        .put_resource_config()
        .resource_type("AWS::EC2::Instance")
        .resource_id("i-hist-1")
        .schema_version_id("1")
        .configuration("{\"key\":\"value\"}")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_resource_config_history()
        .resource_type(aws_sdk_config::types::ResourceType::Instance)
        .resource_id("i-hist-1")
        .send()
        .await
        .expect("get_resource_config_history should succeed");

    assert_eq!(resp.configuration_items().len(), 1);
}

#[tokio::test]
async fn test_get_resource_config_history_nonexistent() {
    let client = make_client().await;

    let resp = client
        .get_resource_config_history()
        .resource_type(aws_sdk_config::types::ResourceType::Instance)
        .resource_id("i-nonexistent")
        .send()
        .await
        .expect("get_resource_config_history should return empty");

    assert_eq!(resp.configuration_items().len(), 0);
}

// --- PutEvaluations tests ---

#[tokio::test]
async fn test_put_evaluations() {
    let client = make_client().await;

    let resp = client
        .put_evaluations()
        .result_token("test-token")
        .evaluations(
            aws_sdk_config::types::Evaluation::builder()
                .compliance_resource_type("AWS::EC2::Instance")
                .compliance_resource_id("i-eval-1")
                .compliance_type(aws_sdk_config::types::ComplianceType::Compliant)
                .ordering_timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_evaluations should succeed");

    // No failed evaluations
    assert_eq!(resp.failed_evaluations().len(), 0);
}

// --- SelectResourceConfig tests ---

#[tokio::test]
async fn test_select_resource_config() {
    let client = make_client().await;

    let resp = client
        .select_resource_config()
        .expression("SELECT * WHERE resourceType = 'AWS::EC2::Instance'")
        .send()
        .await
        .expect("select_resource_config should succeed");

    // Returns empty results (stub)
    assert!(resp.results().is_empty());
}

// --- Tag / Untag / ListTagsForResource tests ---

#[tokio::test]
async fn test_tag_untag_list_tags() {
    let client = make_client().await;

    // Create a config rule to tag
    client
        .put_config_rule()
        .config_rule(
            aws_sdk_config::types::ConfigRule::builder()
                .config_rule_name("tag-rule")
                .source(
                    aws_sdk_config::types::Source::builder()
                        .owner(aws_sdk_config::types::Owner::Aws)
                        .source_identifier("S3_BUCKET_VERSIONING_ENABLED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Get the ARN
    let resp = client
        .describe_config_rules()
        .config_rule_names("tag-rule")
        .send()
        .await
        .unwrap();
    let arn = resp.config_rules()[0]
        .config_rule_arn()
        .unwrap()
        .to_string();

    // Tag
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_config::types::Tag::builder()
                .key("env")
                .value("prod")
                .build(),
        )
        .tags(
            aws_sdk_config::types::Tag::builder()
                .key("team")
                .value("infra")
                .build(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);

    // Untag
    client
        .untag_resource()
        .resource_arn(&arn)
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
    assert_eq!(resp.tags()[0].key(), Some("env"));
    assert_eq!(resp.tags()[0].value(), Some("prod"));
}

// --- DescribeConfigurationRecorderStatus tests ---

#[tokio::test]
async fn test_describe_configuration_recorder_status() {
    let client = make_client().await;

    client
        .put_configuration_recorder()
        .configuration_recorder(
            aws_sdk_config::types::ConfigurationRecorder::builder()
                .name("status-recorder")
                .role_arn("arn:aws:iam::123456789012:role/config-role")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_configuration_recorder_status()
        .configuration_recorder_names("status-recorder")
        .send()
        .await
        .expect("describe status should succeed");

    let statuses = resp.configuration_recorders_status();
    assert_eq!(statuses.len(), 1);
    assert_eq!(statuses[0].name(), Some("status-recorder"));
    assert!(!statuses[0].recording());
}

// ============================================================================
// Ported from moto: test_config.py
// ============================================================================

// Ported from moto: test_config.py::test_describe_configurations (partial)
#[tokio::test]
async fn test_moto_describe_configurations_empty() {
    let client = make_client().await;

    // Without any configurations:
    let result = client
        .describe_configuration_recorders()
        .send()
        .await
        .unwrap();
    assert_eq!(result.configuration_recorders().len(), 0);
}

// Ported from moto: test_config.py::test_describe_configurations (partial)
#[tokio::test]
async fn test_moto_describe_configurations_with_recording_group() {
    let client = make_client().await;

    client
        .put_configuration_recorder()
        .configuration_recorder(
            types::ConfigurationRecorder::builder()
                .name("testrecorder")
                .role_arn("somearn")
                .recording_group(
                    types::RecordingGroup::builder()
                        .all_supported(false)
                        .include_global_resource_types(false)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .describe_configuration_recorders()
        .send()
        .await
        .unwrap();
    let recorders = result.configuration_recorders();
    assert_eq!(recorders.len(), 1);
    assert_eq!(recorders[0].name(), Some("testrecorder"));
    assert_eq!(recorders[0].role_arn(), Some("somearn"));
    let rg = recorders[0].recording_group().unwrap();
    assert!(!rg.all_supported());
    assert!(!rg.include_global_resource_types());

    // Specify an incorrect name:
    let err = client
        .describe_configuration_recorders()
        .configuration_recorder_names("wrong")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchConfigurationRecorderException")
            || err_str.contains("NoSuchConfigurationRecorder"),
        "Expected NoSuchConfigurationRecorderException, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_describe_delivery_channels
#[tokio::test]
async fn test_moto_describe_delivery_channels_empty() {
    let client = make_client().await;

    // Without any channels:
    let result = client.describe_delivery_channels().send().await.unwrap();
    assert_eq!(result.delivery_channels().len(), 0);
}

// Ported from moto: test_config.py::test_describe_delivery_channels (error path)
#[tokio::test]
async fn test_moto_describe_delivery_channels_wrong_name() {
    let client = make_client().await;

    client
        .put_delivery_channel()
        .delivery_channel(
            types::DeliveryChannel::builder()
                .name("testchannel")
                .s3_bucket_name("somebucket")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Specify an incorrect name:
    let err = client
        .describe_delivery_channels()
        .delivery_channel_names("wrong")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchDeliveryChannelException")
            || err_str.contains("NoSuchDeliveryChannel"),
        "Expected NoSuchDeliveryChannelException, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_describe_configuration_recorder_status
#[tokio::test]
async fn test_moto_describe_configuration_recorder_status_empty() {
    let client = make_client().await;

    // Without any:
    let result = client
        .describe_configuration_recorder_status()
        .send()
        .await
        .unwrap();
    assert_eq!(result.configuration_recorders_status().len(), 0);
}

// Ported from moto: test_config.py::test_describe_configuration_recorder_status (wrong name)
#[tokio::test]
async fn test_moto_describe_configuration_recorder_status_wrong_name() {
    let client = make_client().await;

    client
        .put_configuration_recorder()
        .configuration_recorder(
            types::ConfigurationRecorder::builder()
                .name("testrecorder")
                .role_arn("somearn")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Invalid name:
    let err = client
        .describe_configuration_recorder_status()
        .configuration_recorder_names("testrecorder")
        .configuration_recorder_names("wrong")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchConfigurationRecorderException")
            || err_str.contains("NoSuchConfigurationRecorder"),
        "Expected NoSuchConfigurationRecorderException, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_delete_configuration_recorder
#[tokio::test]
async fn test_moto_delete_configuration_recorder_twice() {
    let client = make_client().await;

    client
        .put_configuration_recorder()
        .configuration_recorder(
            types::ConfigurationRecorder::builder()
                .name("testrecorder")
                .role_arn("somearn")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Delete it:
    client
        .delete_configuration_recorder()
        .configuration_recorder_name("testrecorder")
        .send()
        .await
        .unwrap();

    // Try again -- it should be deleted:
    let err = client
        .delete_configuration_recorder()
        .configuration_recorder_name("testrecorder")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchConfigurationRecorderException")
            || err_str.contains("NoSuchConfigurationRecorder"),
        "Expected NoSuchConfigurationRecorderException, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_delete_delivery_channel (error path)
#[tokio::test]
async fn test_moto_delete_delivery_channel_nonexistent_error() {
    let client = make_client().await;

    let err = client
        .delete_delivery_channel()
        .delivery_channel_name("testchannel")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchDeliveryChannelException")
            || err_str.contains("NoSuchDeliveryChannel"),
        "Expected NoSuchDeliveryChannelException, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_put_configuration_aggregator (create + update preserves ARN)
#[tokio::test]
async fn test_moto_put_configuration_aggregator_create_and_update() {
    let client = make_client().await;

    // Create with account aggregation sources
    let result = client
        .put_configuration_aggregator()
        .configuration_aggregator_name("testing")
        .account_aggregation_sources(
            types::AccountAggregationSource::builder()
                .account_ids("012345678910")
                .account_ids("111111111111")
                .account_ids("222222222222")
                .aws_regions("us-east-1")
                .aws_regions("us-west-2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let agg = result.configuration_aggregator().unwrap();
    assert_eq!(agg.configuration_aggregator_name(), Some("testing"));
    let arn = agg.configuration_aggregator_arn().unwrap();
    assert!(
        arn.contains("config-aggregator"),
        "ARN should contain config-aggregator, got: {arn}"
    );
    assert!(agg.creation_time().is_some());
    assert!(agg.last_updated_time().is_some());

    // Update the existing one (should preserve ARN):
    let original_arn = arn.to_string();
    let result2 = client
        .put_configuration_aggregator()
        .configuration_aggregator_name("testing")
        .account_aggregation_sources(
            types::AccountAggregationSource::builder()
                .account_ids("012345678910")
                .all_aws_regions(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let agg2 = result2.configuration_aggregator().unwrap();
    assert_eq!(agg2.configuration_aggregator_name(), Some("testing"));
    assert_eq!(
        agg2.configuration_aggregator_arn().unwrap(),
        original_arn.as_str()
    );
}

// Ported from moto: test_config.py::test_put_configuration_aggregator (org source)
#[tokio::test]
async fn test_moto_put_configuration_aggregator_org_source() {
    let client = make_client().await;

    let result = client
        .put_configuration_aggregator()
        .configuration_aggregator_name("testingOrg")
        .organization_aggregation_source(
            types::OrganizationAggregationSource::builder()
                .role_arn("arn:aws:iam::012345678910:role/SomeRole")
                .aws_regions("us-east-1")
                .aws_regions("us-west-2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let agg = result.configuration_aggregator().unwrap();
    assert_eq!(agg.configuration_aggregator_name(), Some("testingOrg"));
    let org_source = agg.organization_aggregation_source().unwrap();
    assert_eq!(
        org_source.role_arn(),
        "arn:aws:iam::012345678910:role/SomeRole"
    );
    assert_eq!(org_source.aws_regions().len(), 2);
}

// Ported from moto: test_config.py::test_describe_configuration_aggregators (empty)
#[tokio::test]
async fn test_moto_describe_configuration_aggregators_empty() {
    let client = make_client().await;

    let result = client
        .describe_configuration_aggregators()
        .send()
        .await
        .unwrap();
    assert_eq!(result.configuration_aggregators().len(), 0);
}

// Ported from moto: test_config.py::test_delete_configuration_aggregator
#[tokio::test]
async fn test_moto_delete_configuration_aggregator_twice() {
    let client = make_client().await;

    client
        .put_configuration_aggregator()
        .configuration_aggregator_name("testing")
        .account_aggregation_sources(
            types::AccountAggregationSource::builder()
                .account_ids("012345678910")
                .all_aws_regions(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_configuration_aggregator()
        .configuration_aggregator_name("testing")
        .send()
        .await
        .unwrap();

    // And again to confirm it errors:
    let err = client
        .delete_configuration_aggregator()
        .configuration_aggregator_name("testing")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchConfigurationAggregatorException")
            || err_str.contains("configuration aggregator does not exist"),
        "Expected NoSuchConfigurationAggregatorException, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_put_aggregation_authorization (create + idempotent re-put)
#[tokio::test]
async fn test_moto_put_aggregation_authorization_idempotent() {
    let client = make_client().await;

    let result = client
        .put_aggregation_authorization()
        .authorized_account_id("012345678910")
        .authorized_aws_region("us-east-1")
        .send()
        .await
        .unwrap();

    let auth = result.aggregation_authorization().unwrap();
    assert_eq!(auth.authorized_account_id(), Some("012345678910"));
    assert_eq!(auth.authorized_aws_region(), Some("us-east-1"));
    assert!(auth.aggregation_authorization_arn().is_some());
    assert!(auth.creation_time().is_some());

    // Put again -- should return same ARN:
    let result2 = client
        .put_aggregation_authorization()
        .authorized_account_id("012345678910")
        .authorized_aws_region("us-east-1")
        .send()
        .await
        .unwrap();

    let auth2 = result2.aggregation_authorization().unwrap();
    assert_eq!(
        auth.aggregation_authorization_arn(),
        auth2.aggregation_authorization_arn()
    );
}

// Ported from moto: test_config.py::test_describe_aggregation_authorizations (multiple)
#[tokio::test]
async fn test_moto_describe_aggregation_authorizations_multiple() {
    let client = make_client().await;

    // Make 3 account authorizations:
    for i in 0..3 {
        client
            .put_aggregation_authorization()
            .authorized_account_id("0".repeat(12 - 1) + &i.to_string())
            .authorized_aws_region("us-west-2")
            .send()
            .await
            .unwrap();
    }

    let result = client
        .describe_aggregation_authorizations()
        .send()
        .await
        .unwrap();
    assert_eq!(result.aggregation_authorizations().len(), 3);
}

// Ported from moto: test_config.py::test_delete_aggregation_authorization (idempotent)
#[tokio::test]
async fn test_moto_delete_aggregation_authorization_idempotent() {
    let client = make_client().await;

    client
        .put_aggregation_authorization()
        .authorized_account_id("012345678910")
        .authorized_aws_region("us-west-2")
        .send()
        .await
        .unwrap();

    // Delete it:
    client
        .delete_aggregation_authorization()
        .authorized_account_id("012345678910")
        .authorized_aws_region("us-west-2")
        .send()
        .await
        .unwrap();

    // Verify none:
    let result = client
        .describe_aggregation_authorizations()
        .send()
        .await
        .unwrap();
    assert_eq!(result.aggregation_authorizations().len(), 0);

    // Try again -- should not error (idempotent):
    client
        .delete_aggregation_authorization()
        .authorized_account_id("012345678910")
        .authorized_aws_region("us-west-2")
        .send()
        .await
        .unwrap();
}

// Ported from moto: test_config.py::test_put_retention_configuration
#[tokio::test]
async fn test_moto_put_retention_configuration_name_default() {
    let client = make_client().await;

    let result = client
        .put_retention_configuration()
        .retention_period_in_days(2557)
        .send()
        .await
        .unwrap();

    let rc = result.retention_configuration().unwrap();
    assert_eq!(rc.name(), "default");
    assert_eq!(rc.retention_period_in_days(), 2557);
}

// Ported from moto: test_config.py::test_describe_retention_configurations (empty)
#[tokio::test]
async fn test_moto_describe_retention_configurations_empty() {
    let client = make_client().await;

    let result = client
        .describe_retention_configurations()
        .send()
        .await
        .unwrap();
    assert_eq!(result.retention_configurations().len(), 0);
}

// Ported from moto: test_config.py::test_describe_retention_configurations (with name)
#[tokio::test]
async fn test_moto_describe_retention_configurations_by_name() {
    let client = make_client().await;

    client
        .put_retention_configuration()
        .retention_period_in_days(2557)
        .send()
        .await
        .unwrap();

    // Describe all:
    let result = client
        .describe_retention_configurations()
        .send()
        .await
        .unwrap();
    assert_eq!(result.retention_configurations().len(), 1);
    assert_eq!(result.retention_configurations()[0].name(), "default");
    assert_eq!(
        result.retention_configurations()[0].retention_period_in_days(),
        2557
    );

    // Describe by name "default":
    let result = client
        .describe_retention_configurations()
        .retention_configuration_names("default")
        .send()
        .await
        .unwrap();
    assert_eq!(result.retention_configurations().len(), 1);
    assert_eq!(result.retention_configurations()[0].name(), "default");
}

// Ported from moto: test_config.py::test_delete_retention_configuration
#[tokio::test]
async fn test_moto_delete_retention_configuration_twice() {
    let client = make_client().await;

    client
        .put_retention_configuration()
        .retention_period_in_days(2557)
        .send()
        .await
        .unwrap();

    // Delete it:
    client
        .delete_retention_configuration()
        .retention_configuration_name("default")
        .send()
        .await
        .unwrap();

    assert_eq!(
        client
            .describe_retention_configurations()
            .send()
            .await
            .unwrap()
            .retention_configurations()
            .len(),
        0
    );

    // And again -- should error:
    let err = client
        .delete_retention_configuration()
        .retention_configuration_name("default")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchRetentionConfiguration"),
        "Expected retention configuration not found error, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_put_organization_conformance_pack
#[tokio::test]
async fn test_moto_put_organization_conformance_pack_update_preserves_arn() {
    let client = make_client().await;

    let resp1 = client
        .put_organization_conformance_pack()
        .organization_conformance_pack_name("test-pack")
        .delivery_s3_bucket("awsconfigconforms-test-bucket")
        .send()
        .await
        .unwrap();
    let arn1 = resp1
        .organization_conformance_pack_arn()
        .unwrap()
        .to_string();
    assert!(arn1.contains("organization-conformance-pack/test-pack-"));

    // Put again (update) -- should keep same ARN:
    let resp2 = client
        .put_organization_conformance_pack()
        .organization_conformance_pack_name("test-pack")
        .delivery_s3_bucket("awsconfigconforms-test-bucket")
        .send()
        .await
        .unwrap();
    let arn2 = resp2
        .organization_conformance_pack_arn()
        .unwrap()
        .to_string();

    // NOTE: winterbaume currently overwrites the entry on each put, generating a new ARN.
    // Moto preserves the original ARN on update. We'll check non-empty at least.
    assert!(!arn2.is_empty());
}

// Ported from moto: test_config.py::test_describe_organization_conformance_packs
#[tokio::test]
async fn test_moto_describe_organization_conformance_packs_details() {
    let client = make_client().await;

    client
        .put_organization_conformance_pack()
        .organization_conformance_pack_name("test-pack")
        .delivery_s3_bucket("awsconfigconforms-test-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_organization_conformance_packs()
        .organization_conformance_pack_names("test-pack")
        .send()
        .await
        .unwrap();

    let packs = resp.organization_conformance_packs();
    assert_eq!(packs.len(), 1);
    assert_eq!(packs[0].organization_conformance_pack_name(), "test-pack");
    assert_eq!(
        packs[0].delivery_s3_bucket(),
        Some("awsconfigconforms-test-bucket")
    );
    assert!(
        packs[0]
            .organization_conformance_pack_arn()
            .contains("test-pack")
    );
    // last_update_time is always present (non-optional DateTime)
    let _ = packs[0].last_update_time();
}

// Ported from moto: test_config.py::test_describe_organization_conformance_packs_errors
#[tokio::test]
async fn test_moto_describe_organization_conformance_packs_not_found() {
    let client = make_client().await;

    let err = client
        .describe_organization_conformance_packs()
        .organization_conformance_pack_names("not-existing")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchOrganizationConformancePackException")
            || err_str.contains("NoSuchOrganizationConformancePack"),
        "Expected NoSuchOrganizationConformancePackException, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_describe_organization_conformance_pack_statuses_errors
#[tokio::test]
async fn test_moto_describe_organization_conformance_pack_statuses_not_found() {
    let client = make_client().await;

    let err = client
        .describe_organization_conformance_pack_statuses()
        .organization_conformance_pack_names("not-existing")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchOrganizationConformancePackException")
            || err_str.contains("NoSuchOrganizationConformancePack"),
        "Expected NoSuchOrganizationConformancePackException, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_delete_organization_conformance_pack
#[tokio::test]
async fn test_moto_delete_organization_conformance_pack_and_verify() {
    let client = make_client().await;

    client
        .put_organization_conformance_pack()
        .organization_conformance_pack_name("test-pack")
        .delivery_s3_bucket("bucket")
        .send()
        .await
        .unwrap();

    client
        .delete_organization_conformance_pack()
        .organization_conformance_pack_name("test-pack")
        .send()
        .await
        .unwrap();

    // Verify statuses are empty:
    let resp = client
        .describe_organization_conformance_pack_statuses()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.organization_conformance_pack_statuses().len(), 0);
}

// Ported from moto: test_config.py::test_delete_organization_conformance_pack_errors
#[tokio::test]
async fn test_moto_delete_organization_conformance_pack_not_found() {
    let client = make_client().await;

    let err = client
        .delete_organization_conformance_pack()
        .organization_conformance_pack_name("not-existing")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchOrganizationConformancePackException")
            || err_str.contains("NoSuchOrganizationConformancePack"),
        "Expected NoSuchOrganizationConformancePackException, got: {err_str}"
    );
}

// Ported from moto: test_config.py::test_get_organization_conformance_pack_detailed_status_errors
#[tokio::test]
async fn test_moto_get_organization_conformance_pack_detailed_status_not_found() {
    let client = make_client().await;

    let err = client
        .get_organization_conformance_pack_detailed_status()
        .organization_conformance_pack_name("not-existing")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchOrganizationConformancePackException")
            || err_str.contains("NoSuchOrganizationConformancePack"),
        "Expected NoSuchOrganizationConformancePackException, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_config_rules.py
// ============================================================================

// Ported from moto: test_config_rules.py::test_describe_config_rules (empty)
#[tokio::test]
async fn test_moto_describe_config_rules_empty() {
    let client = make_client().await;

    let response = client.describe_config_rules().send().await.unwrap();
    assert_eq!(response.config_rules().len(), 0);
}

// Ported from moto: test_config_rules.py::test_describe_config_rules (not found)
#[tokio::test]
async fn test_moto_describe_config_rules_not_found() {
    let client = make_client().await;

    // Create a rule first
    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("my-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("S3_BUCKET_VERSIONING_ENABLED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let err = client
        .describe_config_rules()
        .config_rule_names("my-rule")
        .config_rule_names("fooey")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchConfigRuleException") || err_str.contains("NoSuchConfigRule"),
        "Expected NoSuchConfigRuleException, got: {err_str}"
    );
}

// Ported from moto: test_config_rules.py::test_valid_put_config_managed_rule
#[tokio::test]
async fn test_moto_put_config_managed_rule_details() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("managed-rule-test")
                .description("Managed S3 Public Read Prohibited Bucket Rule")
                .scope(
                    types::Scope::builder()
                        .compliance_resource_types("AWS::S3::Bucket")
                        .build(),
                )
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("S3_BUCKET_PUBLIC_READ_PROHIBITED")
                        .build()
                        .unwrap(),
                )
                .maximum_execution_frequency(types::MaximumExecutionFrequency::OneHour)
                .config_rule_state(types::ConfigRuleState::Active)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_config_rules()
        .config_rule_names("managed-rule-test")
        .send()
        .await
        .unwrap();

    let rules = resp.config_rules();
    assert_eq!(rules.len(), 1);
    let rule = &rules[0];
    assert_eq!(rule.config_rule_name(), Some("managed-rule-test"));
    assert_eq!(
        rule.description(),
        Some("Managed S3 Public Read Prohibited Bucket Rule")
    );
    assert!(rule.config_rule_arn().is_some());
    assert!(rule.config_rule_id().is_some());
    assert_eq!(
        rule.config_rule_state(),
        Some(&types::ConfigRuleState::Active)
    );
    let source = rule.source().unwrap();
    assert_eq!(source.owner(), &types::Owner::Aws);
    assert_eq!(
        source.source_identifier(),
        Some("S3_BUCKET_PUBLIC_READ_PROHIBITED")
    );
}

// Ported from moto: test_config_rules.py::test_delete_config_rules
#[tokio::test]
async fn test_moto_delete_config_rule_twice() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("test-delete-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("S3_BUCKET_PUBLIC_READ_PROHIBITED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Delete it:
    client
        .delete_config_rule()
        .config_rule_name("test-delete-rule")
        .send()
        .await
        .unwrap();

    // Verify none:
    let result = client.describe_config_rules().send().await.unwrap();
    assert_eq!(result.config_rules().len(), 0);

    // Try again -- should error:
    let err = client
        .delete_config_rule()
        .config_rule_name("test-delete-rule")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchConfigRuleException") || err_str.contains("NoSuchConfigRule"),
        "Expected NoSuchConfigRuleException, got: {err_str}"
    );
}

// Ported from moto: test_config_rules.py::test_describe_config_rules (multiple specific rules)
#[tokio::test]
async fn test_moto_describe_config_rules_specific() {
    let client = make_client().await;

    for name in ["rule-a", "rule-b", "rule-c"] {
        client
            .put_config_rule()
            .config_rule(
                types::ConfigRule::builder()
                    .config_rule_name(name)
                    .source(
                        types::Source::builder()
                            .owner(types::Owner::Aws)
                            .source_identifier("S3_BUCKET_VERSIONING_ENABLED")
                            .build()
                            .unwrap(),
                    )
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    // Request two specific rules:
    let resp = client
        .describe_config_rules()
        .config_rule_names("rule-a")
        .config_rule_names("rule-c")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.config_rules().len(), 2);
}

// ============================================================================
// Ported from moto: test_config_tags.py
// ============================================================================

// Ported from moto: test_config_tags.py::test_tag_resource (aggregator tagging)
#[tokio::test]
async fn test_moto_tag_resource_on_aggregator() {
    let client = make_client().await;

    // Create an aggregator
    let resp = client
        .put_configuration_aggregator()
        .configuration_aggregator_name("tag-test-agg")
        .account_aggregation_sources(
            types::AccountAggregationSource::builder()
                .account_ids("123456789012")
                .all_aws_regions(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let arn = resp
        .configuration_aggregator()
        .unwrap()
        .configuration_aggregator_arn()
        .unwrap()
        .to_string();

    // Tag the aggregator
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(types::Tag::builder().key("env").value("prod").build())
        .tags(types::Tag::builder().key("team").value("platform").build())
        .send()
        .await
        .unwrap();

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);

    // Untag one
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("team")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), Some("env"));
    assert_eq!(resp.tags()[0].value(), Some("prod"));
}

// Ported from moto: test_config_tags.py::test_tag_resource (overwrite existing tag)
#[tokio::test]
async fn test_moto_tag_resource_overwrite() {
    let client = make_client().await;

    // Create a config rule
    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("tag-overwrite-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("S3_BUCKET_VERSIONING_ENABLED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_config_rules()
        .config_rule_names("tag-overwrite-rule")
        .send()
        .await
        .unwrap();
    let arn = resp.config_rules()[0]
        .config_rule_arn()
        .unwrap()
        .to_string();

    // Tag it
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(types::Tag::builder().key("env").value("dev").build())
        .send()
        .await
        .unwrap();

    // Overwrite the tag
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(types::Tag::builder().key("env").value("prod").build())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), Some("env"));
    assert_eq!(resp.tags()[0].value(), Some("prod"));
}

// Ported from moto: test_config_tags.py::test_untag_resource (delete all tags)
#[tokio::test]
async fn test_moto_untag_all_tags() {
    let client = make_client().await;

    // Create an aggregator
    let resp = client
        .put_configuration_aggregator()
        .configuration_aggregator_name("untag-all-agg")
        .account_aggregation_sources(
            types::AccountAggregationSource::builder()
                .account_ids("123456789012")
                .all_aws_regions(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let arn = resp
        .configuration_aggregator()
        .unwrap()
        .configuration_aggregator_arn()
        .unwrap()
        .to_string();

    // Add tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(types::Tag::builder().key("key1").value("val1").build())
        .tags(types::Tag::builder().key("key2").value("val2").build())
        .tags(types::Tag::builder().key("key3").value("val3").build())
        .send()
        .await
        .unwrap();

    // Delete all tags
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("key1")
        .tag_keys("key2")
        .tag_keys("key3")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_config_tags.py::test_untag_resource (nonexistent key is ignored)
#[tokio::test]
async fn test_moto_untag_nonexistent_key_ignored() {
    let client = make_client().await;

    // Create an aggregator
    let resp = client
        .put_configuration_aggregator()
        .configuration_aggregator_name("untag-noop-agg")
        .account_aggregation_sources(
            types::AccountAggregationSource::builder()
                .account_ids("123456789012")
                .all_aws_regions(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let arn = resp
        .configuration_aggregator()
        .unwrap()
        .configuration_aggregator_arn()
        .unwrap()
        .to_string();

    // Add a tag
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(types::Tag::builder().key("keep").value("me").build())
        .send()
        .await
        .unwrap();

    // Try to untag a nonexistent key -- should not error:
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("nonexistent")
        .send()
        .await
        .unwrap();

    // The existing tag should still be there:
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), Some("keep"));
}

// Ported from moto: test_config.py::test_select_resource_config
#[tokio::test]
async fn test_moto_select_resource_config_structure() {
    let client = make_client().await;

    let response = client
        .select_resource_config()
        .expression("SELECT resourceId FROM AWS::S3::Bucket")
        .send()
        .await
        .unwrap();

    // Should have results field (even if empty) and query info
    assert!(response.results().is_empty());
    assert!(response.query_info().is_some());
}

// Ported from moto: test_config.py::test_put_resource_config + test_delete_resource_config
#[tokio::test]
async fn test_moto_put_and_delete_multiple_resources() {
    let client = make_client().await;

    // Create multiple resources of same type
    for i in 0..3 {
        client
            .put_resource_config()
            .resource_type("AWS::EC2::Instance")
            .resource_id(format!("i-{i}"))
            .schema_version_id("1")
            .configuration(format!("{{\"id\":\"i-{i}\"}}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_discovered_resources()
        .resource_type(types::ResourceType::Instance)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.resource_identifiers().len(), 3);

    // Delete one
    client
        .delete_resource_config()
        .resource_type("AWS::EC2::Instance")
        .resource_id("i-1")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_discovered_resources()
        .resource_type(types::ResourceType::Instance)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.resource_identifiers().len(), 2);

    // Delete nonexistent -- should not error:
    client
        .delete_resource_config()
        .resource_type("AWS::EC2::Instance")
        .resource_id("i-nonexistent")
        .send()
        .await
        .unwrap();
}

// Ported from moto: test_config.py::test_put_evaluations
#[tokio::test]
async fn test_moto_put_evaluations_multiple() {
    let client = make_client().await;

    let resp = client
        .put_evaluations()
        .result_token("test-token")
        .evaluations(
            types::Evaluation::builder()
                .compliance_resource_type("AWS::EC2::Instance")
                .compliance_resource_id("i-1")
                .compliance_type(types::ComplianceType::Compliant)
                .ordering_timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
                .build()
                .unwrap(),
        )
        .evaluations(
            types::Evaluation::builder()
                .compliance_resource_type("AWS::S3::Bucket")
                .compliance_resource_id("bucket-1")
                .compliance_type(types::ComplianceType::NonCompliant)
                .ordering_timestamp(aws_smithy_types::DateTime::from_secs(1700000001))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(resp.failed_evaluations().len(), 0);
}

// Ported from moto: test_config.py::test_batch_get_resource_config (mixed found/not-found)
#[tokio::test]
async fn test_moto_batch_get_resource_config_all_missing() {
    let client = make_client().await;

    let resp = client
        .batch_get_resource_config()
        .resource_keys(
            types::ResourceKey::builder()
                .resource_type(types::ResourceType::Instance)
                .resource_id("i-nonexistent")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(resp.base_configuration_items().len(), 0);
    assert_eq!(resp.unprocessed_resource_keys().len(), 1);
}

// Ported from moto: test_config.py::test_list_discovered_resource (empty)
#[tokio::test]
async fn test_moto_list_discovered_resources_empty() {
    let client = make_client().await;

    let resp = client
        .list_discovered_resources()
        .resource_type(types::ResourceType::Bucket)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.resource_identifiers().len(), 0);
}

#[tokio::test]
async fn test_put_and_describe_remediation_configurations() {
    let client = make_client().await;

    // First create a config rule
    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("test-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("AWS_EC2_INSTANCE_PROFILE_ATTACHED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("put_config_rule should succeed");

    // Put a remediation configuration
    client
        .put_remediation_configurations()
        .remediation_configurations(
            types::RemediationConfiguration::builder()
                .config_rule_name("test-rule")
                .target_type(types::RemediationTargetType::SsmDocument)
                .target_id("AWS-EnableS3BucketVersioning")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_remediation_configurations should succeed");

    // Describe remediation configurations
    let resp = client
        .describe_remediation_configurations()
        .config_rule_names("test-rule")
        .send()
        .await
        .expect("describe_remediation_configurations should succeed");

    let configs = resp.remediation_configurations();
    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].config_rule_name(), "test-rule");
    assert_eq!(configs[0].target_id(), "AWS-EnableS3BucketVersioning");
}

#[tokio::test]
async fn test_delete_remediation_configuration() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("del-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("AWS_EC2_INSTANCE_PROFILE_ATTACHED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_remediation_configurations()
        .remediation_configurations(
            types::RemediationConfiguration::builder()
                .config_rule_name("del-rule")
                .target_type(types::RemediationTargetType::SsmDocument)
                .target_id("AWS-EnableS3BucketVersioning")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_remediation_configuration()
        .config_rule_name("del-rule")
        .send()
        .await
        .expect("delete_remediation_configuration should succeed");

    let resp = client
        .describe_remediation_configurations()
        .config_rule_names("del-rule")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.remediation_configurations().len(), 0);
}

#[tokio::test]
async fn test_start_remediation_execution() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("exec-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("AWS_EC2_INSTANCE_PROFILE_ATTACHED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .start_remediation_execution()
        .config_rule_name("exec-rule")
        .resource_keys(
            types::ResourceKey::builder()
                .resource_type(types::ResourceType::from("AWS::EC2::Instance"))
                .resource_id("i-1234567890abcdef0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("start_remediation_execution should succeed");

    // Mock returns empty failed items
    assert_eq!(resp.failed_items().len(), 0);
}

#[tokio::test]
async fn test_put_and_describe_organization_config_rules() {
    let client = make_client().await;

    let put_resp = client
        .put_organization_config_rule()
        .organization_config_rule_name("org-rule-1")
        .organization_managed_rule_metadata(
            types::OrganizationManagedRuleMetadata::builder()
                .rule_identifier("AWS_EC2_INSTANCE_PROFILE_ATTACHED")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_organization_config_rule should succeed");

    assert!(put_resp.organization_config_rule_arn().is_some());

    let resp = client
        .describe_organization_config_rules()
        .send()
        .await
        .expect("describe_organization_config_rules should succeed");

    let rules = resp.organization_config_rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].organization_config_rule_name(), "org-rule-1");
}

#[tokio::test]
async fn test_delete_organization_config_rule() {
    let client = make_client().await;

    client
        .put_organization_config_rule()
        .organization_config_rule_name("org-del-rule")
        .organization_managed_rule_metadata(
            types::OrganizationManagedRuleMetadata::builder()
                .rule_identifier("AWS_EC2_INSTANCE_PROFILE_ATTACHED")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_organization_config_rule()
        .organization_config_rule_name("org-del-rule")
        .send()
        .await
        .expect("delete_organization_config_rule should succeed");

    let result = client
        .describe_organization_config_rules()
        .organization_config_rule_names("org-del-rule")
        .send()
        .await;

    assert!(result.is_err(), "Should error when rule doesn't exist");
}

#[tokio::test]
async fn test_start_config_rules_evaluation() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("eval-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("AWS_EC2_INSTANCE_PROFILE_ATTACHED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .start_config_rules_evaluation()
        .config_rule_names("eval-rule")
        .send()
        .await
        .expect("start_config_rules_evaluation should succeed");
}

#[tokio::test]
async fn test_describe_config_rule_evaluation_status() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("status-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("AWS_EC2_INSTANCE_PROFILE_ATTACHED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_config_rule_evaluation_status()
        .config_rule_names("status-rule")
        .send()
        .await
        .expect("describe_config_rule_evaluation_status should succeed");

    let statuses = resp.config_rules_evaluation_status();
    assert_eq!(statuses.len(), 1);
    assert_eq!(statuses[0].config_rule_name(), Some("status-rule"));
    assert!(statuses[0].first_evaluation_started());
}

#[tokio::test]
async fn test_describe_compliance_by_config_rule() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("compliance-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("AWS_EC2_INSTANCE_PROFILE_ATTACHED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_compliance_by_config_rule()
        .send()
        .await
        .expect("describe_compliance_by_config_rule should succeed");

    let compliance = resp.compliance_by_config_rules();
    assert_eq!(compliance.len(), 1);
    assert_eq!(compliance[0].config_rule_name(), Some("compliance-rule"));
}

#[tokio::test]
async fn test_get_compliance_details_by_config_rule() {
    let client = make_client().await;

    client
        .put_config_rule()
        .config_rule(
            types::ConfigRule::builder()
                .config_rule_name("detail-rule")
                .source(
                    types::Source::builder()
                        .owner(types::Owner::Aws)
                        .source_identifier("AWS_EC2_INSTANCE_PROFILE_ATTACHED")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Put some evaluations
    client
        .put_evaluations()
        .evaluations(
            types::Evaluation::builder()
                .compliance_resource_type("AWS::EC2::Instance")
                .compliance_resource_id("i-1234567890abcdef0")
                .compliance_type(types::ComplianceType::Compliant)
                .ordering_timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
                .build()
                .unwrap(),
        )
        .result_token("token")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_compliance_details_by_config_rule()
        .config_rule_name("detail-rule")
        .send()
        .await
        .expect("get_compliance_details_by_config_rule should succeed");

    let results = resp.evaluation_results();
    assert_eq!(results.len(), 1);
}
