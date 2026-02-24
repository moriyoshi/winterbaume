use aws_sdk_xray::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_xray::XRayService;

async fn make_client() -> aws_sdk_xray::Client {
    let mock = MockAws::builder().with_service(XRayService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_xray::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_xray::Client::new(&config)
}

#[tokio::test]
async fn test_put_trace_segments() {
    let client = make_client().await;

    let resp = client
        .put_trace_segments()
        .trace_segment_documents(r#"{"id":"abc123","trace_id":"1-abc-def"}"#)
        .send()
        .await
        .expect("put_trace_segments should succeed");

    assert!(resp.unprocessed_trace_segments().is_empty());
}

#[tokio::test]
async fn test_create_and_get_group() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .group_name("my-group")
        .filter_expression("service(\"my-service\")")
        .send()
        .await
        .expect("create_group should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(group.group_name(), Some("my-group"));
    assert_eq!(group.filter_expression(), Some("service(\"my-service\")"));

    let resp = client
        .get_group()
        .group_name("my-group")
        .send()
        .await
        .expect("get_group should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(group.group_name(), Some("my-group"));
}

#[tokio::test]
async fn test_delete_group() {
    let client = make_client().await;

    client
        .create_group()
        .group_name("delete-me")
        .send()
        .await
        .unwrap();

    client
        .delete_group()
        .group_name("delete-me")
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_group().group_name("delete-me").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_delete_then_get_group_not_found() {
    let client = make_client().await;

    client
        .create_group()
        .group_name("ephemeral-group")
        .send()
        .await
        .unwrap();

    client
        .delete_group()
        .group_name("ephemeral-group")
        .send()
        .await
        .unwrap();

    let result = client
        .get_group()
        .group_name("ephemeral-group")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_group() {
    let client = make_client().await;

    let result = client.get_group().group_name("no-such-group").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_telemetry_records() {
    // Port of moto test_put_telemetry
    let client = make_client().await;

    // PutTelemetryRecords should succeed (it's a fire-and-forget operation)
    client
        .put_telemetry_records()
        .telemetry_records(
            aws_sdk_xray::types::TelemetryRecord::builder()
                .timestamp(aws_smithy_types::DateTime::from_secs(1420070400))
                .segments_received_count(123)
                .segments_sent_count(123)
                .segments_spillover_count(123)
                .segments_rejected_count(123)
                .build()
                .unwrap(),
        )
        .ec2_instance_id("i-12345678")
        .hostname("myhost")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:instance/i-12345678")
        .send()
        .await
        .expect("put_telemetry_records should succeed");
}

#[tokio::test]
async fn test_put_multiple_trace_segments() {
    // Port of moto test_put_trace_segments - verify multiple documents can be put at once
    use serde_json::json;

    let client = make_client().await;

    let seg1 = json!({
        "name": "example.com",
        "id": "70de5b6f19ff9a0a",
        "start_time": 1478293361.271,
        "trace_id": "1-581cf771-a006649127e371903a2de979",
        "end_time": 1478293361.449
    })
    .to_string();

    let resp = client
        .put_trace_segments()
        .trace_segment_documents(seg1)
        .send()
        .await
        .expect("put_trace_segments should succeed");

    assert!(resp.unprocessed_trace_segments().is_empty());
}

#[tokio::test]
async fn test_create_group_with_insights() {
    // Port of moto create_group with InsightsConfiguration
    let client = make_client().await;

    let resp = client
        .create_group()
        .group_name("insights-group")
        .filter_expression("service(\"my-service\")")
        .insights_configuration(
            aws_sdk_xray::types::InsightsConfiguration::builder()
                .insights_enabled(true)
                .notifications_enabled(true)
                .build(),
        )
        .send()
        .await
        .expect("create_group with insights should succeed");

    let group = resp.group().unwrap();
    assert_eq!(group.group_name(), Some("insights-group"));
    assert_eq!(group.filter_expression(), Some("service(\"my-service\")"));
    let ic = group.insights_configuration().unwrap();
    assert_eq!(ic.insights_enabled(), Some(true));
    assert_eq!(ic.notifications_enabled(), Some(true));
}

#[tokio::test]
async fn test_create_duplicate_group_fails() {
    // Creating the same group name twice should fail
    let client = make_client().await;

    client
        .create_group()
        .group_name("dup-group")
        .send()
        .await
        .unwrap();

    let result = client.create_group().group_name("dup-group").send().await;
    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: AWS X-Ray
// ============================================================================

#[tokio::test]
async fn test_create_group_returns_arn() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .group_name("arn-check-group")
        .send()
        .await
        .expect("create_group should succeed");

    let group = resp.group().expect("should have group");
    let arn = group.group_arn().unwrap_or_default();
    assert!(!arn.is_empty(), "GroupARN should be non-empty");
    assert!(
        arn.contains("arn-check-group"),
        "ARN should contain the group name"
    );
}

#[tokio::test]
async fn test_get_group_returns_arn() {
    let client = make_client().await;

    client
        .create_group()
        .group_name("arn-get-group")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_group()
        .group_name("arn-get-group")
        .send()
        .await
        .expect("get_group should succeed");

    let group = resp.group().expect("should have group");
    let arn = group.group_arn().unwrap_or_default();
    assert!(
        !arn.is_empty(),
        "GroupARN should be non-empty in GetGroup response"
    );
}

#[tokio::test]
async fn test_get_group_by_arn() {
    let client = make_client().await;

    let create_resp = client
        .create_group()
        .group_name("by-arn-group")
        .send()
        .await
        .expect("create_group should succeed");

    let arn = create_resp
        .group()
        .unwrap()
        .group_arn()
        .unwrap()
        .to_string();

    let resp = client
        .get_group()
        .group_arn(&arn)
        .send()
        .await
        .expect("get_group by ARN should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(group.group_name(), Some("by-arn-group"));
}

#[tokio::test]
async fn test_delete_group_by_arn() {
    let client = make_client().await;

    let create_resp = client
        .create_group()
        .group_name("del-by-arn-group")
        .send()
        .await
        .expect("create_group should succeed");

    let arn = create_resp
        .group()
        .unwrap()
        .group_arn()
        .unwrap()
        .to_string();

    client
        .delete_group()
        .group_arn(&arn)
        .send()
        .await
        .expect("delete_group by ARN should succeed");

    let result = client
        .get_group()
        .group_name("del-by-arn-group")
        .send()
        .await;
    assert!(result.is_err(), "get_group should fail after delete");
}

#[tokio::test]
async fn test_delete_nonexistent_group() {
    let client = make_client().await;

    let result = client
        .delete_group()
        .group_name("no-such-group-for-delete")
        .send()
        .await;
    assert!(result.is_err(), "delete of nonexistent group should fail");
}

#[tokio::test]
async fn test_lifecycle_group() {
    // Full create -> get -> delete -> get-fails cycle
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_group()
        .group_name("lifecycle-group")
        .filter_expression("responsetime > 2")
        .send()
        .await
        .expect("create_group should succeed");

    let group = create_resp.group().expect("should have group");
    assert_eq!(group.group_name(), Some("lifecycle-group"));
    assert_eq!(group.filter_expression(), Some("responsetime > 2"));

    // Get
    let get_resp = client
        .get_group()
        .group_name("lifecycle-group")
        .send()
        .await
        .expect("get_group should succeed");
    assert_eq!(
        get_resp.group().unwrap().filter_expression(),
        Some("responsetime > 2")
    );

    // Delete
    client
        .delete_group()
        .group_name("lifecycle-group")
        .send()
        .await
        .expect("delete_group should succeed");

    // Verify gone
    let result = client
        .get_group()
        .group_name("lifecycle-group")
        .send()
        .await;
    assert!(result.is_err(), "get_group should fail after delete");
}

#[tokio::test]
async fn test_create_group_minimal_defaults() {
    // Create a group without filter expression and verify InsightsConfiguration defaults
    let client = make_client().await;

    let resp = client
        .create_group()
        .group_name("minimal-defaults-group")
        .send()
        .await
        .expect("create_group with no filter should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(group.group_name(), Some("minimal-defaults-group"));

    // InsightsConfiguration defaults should be false
    if let Some(ic) = group.insights_configuration() {
        assert_eq!(ic.insights_enabled(), Some(false));
        assert_eq!(ic.notifications_enabled(), Some(false));
    }
}

#[tokio::test]
async fn test_put_trace_segments_in_progress() {
    // Segment with in_progress flag (no end_time) should be accepted
    use serde_json::json;

    let client = make_client().await;

    let in_progress_seg = json!({
        "name": "my-service",
        "id": "abcdef1234567890",
        "trace_id": "1-581cf771-a006649127e371903a2de979",
        "start_time": 1478293361.271,
        "in_progress": true
    })
    .to_string();

    let resp = client
        .put_trace_segments()
        .trace_segment_documents(in_progress_seg)
        .send()
        .await
        .expect("put_trace_segments with in-progress segment should succeed");

    assert!(
        resp.unprocessed_trace_segments().is_empty(),
        "in-progress segment should not be unprocessed"
    );
}

#[tokio::test]
async fn test_create_group_with_filter_reflected_in_get() {
    let client = make_client().await;

    let filter = "service(\"api-service\") AND responsetime > 5";

    client
        .create_group()
        .group_name("filter-verify-group")
        .filter_expression(filter)
        .send()
        .await
        .expect("create_group should succeed");

    let resp = client
        .get_group()
        .group_name("filter-verify-group")
        .send()
        .await
        .expect("get_group should succeed");

    assert_eq!(
        resp.group().unwrap().filter_expression(),
        Some(filter),
        "filter expression should be preserved"
    );
}

#[tokio::test]
async fn test_get_group_by_arn_and_name_match() {
    // Fetch same group by ARN and by name; verify both return the same GroupARN and GroupName
    let client = make_client().await;

    let create_resp = client
        .create_group()
        .group_name("cross-lookup-group")
        .filter_expression("responsetime > 3")
        .send()
        .await
        .expect("create_group should succeed");

    let created_arn = create_resp
        .group()
        .unwrap()
        .group_arn()
        .unwrap()
        .to_string();

    // Fetch by name
    let by_name = client
        .get_group()
        .group_name("cross-lookup-group")
        .send()
        .await
        .expect("get_group by name should succeed");

    // Fetch by ARN
    let by_arn = client
        .get_group()
        .group_arn(&created_arn)
        .send()
        .await
        .expect("get_group by ARN should succeed");

    let g1 = by_name.group().unwrap();
    let g2 = by_arn.group().unwrap();

    assert_eq!(g1.group_name(), g2.group_name(), "group_name should match");
    assert_eq!(g1.group_arn(), g2.group_arn(), "group_arn should match");
    assert_eq!(
        g1.filter_expression(),
        g2.filter_expression(),
        "filter_expression should match"
    );
}

#[tokio::test]
async fn test_create_group_name_max_length() {
    // GroupName max length is 32 characters per AWS docs
    let client = make_client().await;

    let long_name = "a".repeat(32);

    let resp = client
        .create_group()
        .group_name(&long_name)
        .send()
        .await
        .expect("create_group with 32-char name should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(group.group_name(), Some(long_name.as_str()));
}

// ============================================================================
// Tests derived from AWS documentation: Groups (additional coverage)
// ============================================================================

#[tokio::test]
async fn test_get_groups_empty() {
    let client = make_client().await;

    let resp = client
        .get_groups()
        .send()
        .await
        .expect("get_groups should succeed on empty state");

    assert!(
        resp.groups().is_empty(),
        "groups list should be empty initially"
    );
}

#[tokio::test]
async fn test_get_groups_after_create() {
    let client = make_client().await;

    client
        .create_group()
        .group_name("group-a")
        .filter_expression("service(\"a\")")
        .send()
        .await
        .unwrap();

    client
        .create_group()
        .group_name("group-b")
        .filter_expression("service(\"b\")")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_groups()
        .send()
        .await
        .expect("get_groups should succeed");

    let groups = resp.groups();
    assert_eq!(groups.len(), 2, "should list 2 groups");

    let names: Vec<&str> = groups.iter().filter_map(|g| g.group_name()).collect();
    assert!(names.contains(&"group-a"), "should contain group-a");
    assert!(names.contains(&"group-b"), "should contain group-b");
}

#[tokio::test]
async fn test_update_group_filter() {
    let client = make_client().await;

    client
        .create_group()
        .group_name("update-filter-group")
        .filter_expression("responsetime > 1")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_group()
        .group_name("update-filter-group")
        .filter_expression("responsetime > 5")
        .send()
        .await
        .expect("update_group should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(
        group.filter_expression(),
        Some("responsetime > 5"),
        "filter should be updated"
    );

    // Verify via get_group
    let get_resp = client
        .get_group()
        .group_name("update-filter-group")
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.group().unwrap().filter_expression(),
        Some("responsetime > 5")
    );
}

#[tokio::test]
async fn test_update_group_insights() {
    let client = make_client().await;

    client
        .create_group()
        .group_name("update-insights-group")
        .insights_configuration(
            aws_sdk_xray::types::InsightsConfiguration::builder()
                .insights_enabled(false)
                .notifications_enabled(false)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .update_group()
        .group_name("update-insights-group")
        .insights_configuration(
            aws_sdk_xray::types::InsightsConfiguration::builder()
                .insights_enabled(true)
                .notifications_enabled(true)
                .build(),
        )
        .send()
        .await
        .expect("update_group insights should succeed");

    let group = resp.group().unwrap();
    let ic = group.insights_configuration().unwrap();
    assert_eq!(ic.insights_enabled(), Some(true));
    assert_eq!(ic.notifications_enabled(), Some(true));
}

#[tokio::test]
async fn test_update_group_by_arn() {
    let client = make_client().await;

    let create_resp = client
        .create_group()
        .group_name("update-by-arn-group")
        .filter_expression("responsetime > 1")
        .send()
        .await
        .unwrap();

    let arn = create_resp
        .group()
        .unwrap()
        .group_arn()
        .unwrap()
        .to_string();

    let resp = client
        .update_group()
        .group_arn(&arn)
        .filter_expression("responsetime > 10")
        .send()
        .await
        .expect("update_group by ARN should succeed");

    let group = resp.group().unwrap();
    assert_eq!(group.filter_expression(), Some("responsetime > 10"));
}

#[tokio::test]
async fn test_update_nonexistent_group() {
    let client = make_client().await;

    let result = client
        .update_group()
        .group_name("no-such-group-update")
        .filter_expression("responsetime > 1")
        .send()
        .await;

    assert!(result.is_err(), "update of nonexistent group should fail");
}

// ============================================================================
// Tests derived from AWS documentation: Sampling Rules
// ============================================================================

#[tokio::test]
async fn test_create_sampling_rule_basic() {
    let client = make_client().await;

    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("test-basic-rule")
        .resource_arn("*")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(1)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    let resp = client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await
        .expect("create_sampling_rule should succeed");

    let record = resp.sampling_rule_record().expect("should have record");
    let sr = record.sampling_rule().expect("should have sampling_rule");
    assert_eq!(sr.rule_name(), Some("test-basic-rule"));
    assert!(
        !sr.rule_arn().unwrap_or_default().is_empty(),
        "rule ARN should be non-empty"
    );
    assert_eq!(sr.priority(), 100);
    assert!((sr.fixed_rate() - 0.05).abs() < f64::EPSILON);
    assert_eq!(sr.reservoir_size(), 1);
    assert!(
        record.created_at().is_some(),
        "created_at should be present"
    );
    assert!(
        record.modified_at().is_some(),
        "modified_at should be present"
    );
}

#[tokio::test]
async fn test_create_sampling_rule_full_params() {
    let client = make_client().await;

    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("full-params-rule")
        .resource_arn("arn:aws:lambda:us-east-1:123456789012:function:my-func")
        .priority(50)
        .fixed_rate(0.1)
        .reservoir_size(10)
        .service_name("my-service")
        .service_type("AWS::Lambda::Function")
        .host("example.com")
        .http_method("GET")
        .url_path("/api/*")
        .version(1)
        .build()
        .unwrap();

    let resp = client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await
        .expect("create_sampling_rule with full params should succeed");

    let sr = resp
        .sampling_rule_record()
        .unwrap()
        .sampling_rule()
        .unwrap();
    assert_eq!(sr.rule_name(), Some("full-params-rule"));
    assert_eq!(sr.priority(), 50);
    assert!((sr.fixed_rate() - 0.1).abs() < f64::EPSILON);
    assert_eq!(sr.reservoir_size(), 10);
    assert_eq!(sr.service_name(), "my-service");
    assert_eq!(sr.service_type(), "AWS::Lambda::Function");
    assert_eq!(sr.host(), "example.com");
    assert_eq!(sr.http_method(), "GET");
    assert_eq!(sr.url_path(), "/api/*");
    assert_eq!(sr.version(), 1);
}

#[tokio::test]
async fn test_create_sampling_rule_duplicate() {
    let client = make_client().await;

    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("dup-rule")
        .resource_arn("*")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(1)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    client
        .create_sampling_rule()
        .sampling_rule(rule.clone())
        .send()
        .await
        .unwrap();

    let result = client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await;

    assert!(result.is_err(), "duplicate sampling rule should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("AlreadyExists") || err_str.contains("already exists"),
        "Expected already-exists error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_sampling_rules_empty() {
    let client = make_client().await;

    let resp = client
        .get_sampling_rules()
        .send()
        .await
        .expect("get_sampling_rules should succeed on empty state");

    assert!(
        resp.sampling_rule_records().is_empty(),
        "sampling rules should be empty initially"
    );
}

#[tokio::test]
async fn test_get_sampling_rules_after_create() {
    let client = make_client().await;

    let rule1 = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("list-rule-1")
        .resource_arn("*")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(1)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    let rule2 = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("list-rule-2")
        .resource_arn("*")
        .priority(200)
        .fixed_rate(0.1)
        .reservoir_size(5)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    client
        .create_sampling_rule()
        .sampling_rule(rule1)
        .send()
        .await
        .unwrap();

    client
        .create_sampling_rule()
        .sampling_rule(rule2)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_sampling_rules()
        .send()
        .await
        .expect("get_sampling_rules should succeed");

    let records = resp.sampling_rule_records();
    assert_eq!(records.len(), 2, "should have 2 sampling rules");

    let names: Vec<Option<&str>> = records
        .iter()
        .filter_map(|r| r.sampling_rule())
        .map(|sr| sr.rule_name())
        .collect();
    assert!(names.contains(&Some("list-rule-1")));
    assert!(names.contains(&Some("list-rule-2")));
}

#[tokio::test]
async fn test_delete_sampling_rule_by_name() {
    let client = make_client().await;

    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("del-rule")
        .resource_arn("*")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(1)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_sampling_rule()
        .rule_name("del-rule")
        .send()
        .await
        .expect("delete_sampling_rule should succeed");

    // Deleted rule is returned in the response
    let record = resp
        .sampling_rule_record()
        .expect("should have deleted record");
    let sr = record.sampling_rule().unwrap();
    assert_eq!(sr.rule_name(), Some("del-rule"));

    // Verify it's gone
    let list_resp = client.get_sampling_rules().send().await.unwrap();
    assert!(
        list_resp.sampling_rule_records().is_empty(),
        "sampling rules should be empty after delete"
    );
}

#[tokio::test]
async fn test_delete_sampling_rule_not_found() {
    let client = make_client().await;

    let result = client
        .delete_sampling_rule()
        .rule_name("nonexistent-rule")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete of nonexistent sampling rule should fail"
    );
}

#[tokio::test]
async fn test_update_sampling_rule() {
    let client = make_client().await;

    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("upd-rule")
        .resource_arn("*")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(1)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await
        .unwrap();

    let update = aws_sdk_xray::types::SamplingRuleUpdate::builder()
        .rule_name("upd-rule")
        .priority(50)
        .fixed_rate(0.2)
        .reservoir_size(10)
        .service_name("updated-service")
        .host("updated.example.com")
        .http_method("POST")
        .url_path("/updated/*")
        .build();

    let resp = client
        .update_sampling_rule()
        .sampling_rule_update(update)
        .send()
        .await
        .expect("update_sampling_rule should succeed");

    let sr = resp
        .sampling_rule_record()
        .unwrap()
        .sampling_rule()
        .unwrap();
    assert_eq!(sr.rule_name(), Some("upd-rule"));
    assert_eq!(sr.priority(), 50);
    assert!((sr.fixed_rate() - 0.2).abs() < f64::EPSILON);
    assert_eq!(sr.reservoir_size(), 10);
    assert_eq!(sr.service_name(), "updated-service");
    assert_eq!(sr.host(), "updated.example.com");
    assert_eq!(sr.http_method(), "POST");
    assert_eq!(sr.url_path(), "/updated/*");
}

#[tokio::test]
async fn test_update_sampling_rule_not_found() {
    let client = make_client().await;

    let update = aws_sdk_xray::types::SamplingRuleUpdate::builder()
        .rule_name("nonexistent-rule-upd")
        .priority(50)
        .build();

    let result = client
        .update_sampling_rule()
        .sampling_rule_update(update)
        .send()
        .await;

    assert!(
        result.is_err(),
        "update of nonexistent sampling rule should fail"
    );
}

#[tokio::test]
async fn test_lifecycle_sampling_rule() {
    let client = make_client().await;

    // Create
    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("lifecycle-rule")
        .resource_arn("*")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(1)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await
        .expect("create should succeed");

    // List and verify
    let list_resp = client.get_sampling_rules().send().await.unwrap();
    assert_eq!(list_resp.sampling_rule_records().len(), 1);

    // Update
    let update = aws_sdk_xray::types::SamplingRuleUpdate::builder()
        .rule_name("lifecycle-rule")
        .priority(25)
        .fixed_rate(0.5)
        .build();

    let upd_resp = client
        .update_sampling_rule()
        .sampling_rule_update(update)
        .send()
        .await
        .expect("update should succeed");

    let sr = upd_resp
        .sampling_rule_record()
        .unwrap()
        .sampling_rule()
        .unwrap();
    assert_eq!(sr.priority(), 25);
    assert!((sr.fixed_rate() - 0.5).abs() < f64::EPSILON);

    // Delete
    let del_resp = client
        .delete_sampling_rule()
        .rule_name("lifecycle-rule")
        .send()
        .await
        .expect("delete should succeed");
    assert_eq!(
        del_resp
            .sampling_rule_record()
            .unwrap()
            .sampling_rule()
            .unwrap()
            .rule_name(),
        Some("lifecycle-rule")
    );

    // Verify gone
    let list_resp = client.get_sampling_rules().send().await.unwrap();
    assert!(
        list_resp.sampling_rule_records().is_empty(),
        "rules should be empty after delete"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Resource Policies
// ============================================================================

#[tokio::test]
async fn test_put_resource_policy_basic() {
    let client = make_client().await;

    let resp = client
        .put_resource_policy()
        .policy_name("test-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("put_resource_policy should succeed");

    let policy = resp.resource_policy().expect("should have resource_policy");
    assert_eq!(policy.policy_name(), Some("test-policy"));
    assert_eq!(
        policy.policy_document(),
        Some(r#"{"Version":"2012-10-17","Statement":[]}"#)
    );
    assert!(
        policy.policy_revision_id().is_some(),
        "policy_revision_id should be present"
    );
    assert!(
        !policy.policy_revision_id().unwrap_or_default().is_empty(),
        "policy_revision_id should be non-empty"
    );
    assert!(
        policy.last_updated_time().is_some(),
        "last_updated_time should be present"
    );
}

#[tokio::test]
async fn test_put_resource_policy_update() {
    let client = make_client().await;

    // Create initial policy
    let resp1 = client
        .put_resource_policy()
        .policy_name("updatable-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    let rev1 = resp1
        .resource_policy()
        .unwrap()
        .policy_revision_id()
        .unwrap()
        .to_string();

    // Update with correct revision ID
    let resp2 = client
        .put_resource_policy()
        .policy_name("updatable-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow"}]}"#)
        .policy_revision_id(&rev1)
        .send()
        .await
        .expect("update with correct revision ID should succeed");

    let policy2 = resp2.resource_policy().unwrap();
    assert_eq!(
        policy2.policy_document(),
        Some(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow"}]}"#)
    );
    let rev2 = policy2.policy_revision_id().unwrap();
    assert_ne!(rev1.as_str(), rev2, "revision ID should change on update");
}

#[tokio::test]
async fn test_put_resource_policy_revision_mismatch() {
    let client = make_client().await;

    client
        .put_resource_policy()
        .policy_name("mismatch-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    // Try to update with wrong revision ID
    let result = client
        .put_resource_policy()
        .policy_name("mismatch-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Deny"}]}"#)
        .policy_revision_id("wrong-revision-id")
        .send()
        .await;

    assert!(result.is_err(), "update with wrong revision should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidPolicyRevisionId") || err_str.contains("revision"),
        "Expected revision mismatch error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_resource_policies_empty() {
    let client = make_client().await;

    let resp = client
        .list_resource_policies()
        .send()
        .await
        .expect("list_resource_policies should succeed on empty state");

    assert!(
        resp.resource_policies().is_empty(),
        "resource policies should be empty initially"
    );
}

#[tokio::test]
async fn test_list_resource_policies_after_create() {
    let client = make_client().await;

    client
        .put_resource_policy()
        .policy_name("policy-a")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    client
        .put_resource_policy()
        .policy_name("policy-b")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_resource_policies()
        .send()
        .await
        .expect("list_resource_policies should succeed");

    let policies = resp.resource_policies();
    assert_eq!(policies.len(), 2, "should have 2 resource policies");

    let names: Vec<Option<&str>> = policies.iter().map(|p| p.policy_name()).collect();
    assert!(names.contains(&Some("policy-a")));
    assert!(names.contains(&Some("policy-b")));
}

#[tokio::test]
async fn test_delete_resource_policy() {
    let client = make_client().await;

    let resp = client
        .put_resource_policy()
        .policy_name("del-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    let rev_id = resp
        .resource_policy()
        .unwrap()
        .policy_revision_id()
        .unwrap()
        .to_string();

    client
        .delete_resource_policy()
        .policy_name("del-policy")
        .policy_revision_id(&rev_id)
        .send()
        .await
        .expect("delete_resource_policy should succeed");

    // Verify it's gone
    let list_resp = client.list_resource_policies().send().await.unwrap();
    assert!(
        list_resp.resource_policies().is_empty(),
        "resource policies should be empty after delete"
    );
}

#[tokio::test]
async fn test_delete_resource_policy_not_found() {
    let client = make_client().await;

    let result = client
        .delete_resource_policy()
        .policy_name("nonexistent-policy")
        .policy_revision_id("some-rev-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete of nonexistent resource policy should fail"
    );
}

#[tokio::test]
async fn test_delete_resource_policy_revision_mismatch() {
    let client = make_client().await;

    client
        .put_resource_policy()
        .policy_name("rev-mismatch-del-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    let result = client
        .delete_resource_policy()
        .policy_name("rev-mismatch-del-policy")
        .policy_revision_id("wrong-revision")
        .send()
        .await;

    assert!(result.is_err(), "delete with wrong revision should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidPolicyRevisionId") || err_str.contains("revision"),
        "Expected revision mismatch error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_lifecycle_resource_policy() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .put_resource_policy()
        .policy_name("lifecycle-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("put should succeed");

    let rev1 = create_resp
        .resource_policy()
        .unwrap()
        .policy_revision_id()
        .unwrap()
        .to_string();

    // List and verify
    let list_resp = client.list_resource_policies().send().await.unwrap();
    assert_eq!(list_resp.resource_policies().len(), 1);
    assert_eq!(
        list_resp.resource_policies()[0].policy_name(),
        Some("lifecycle-policy")
    );

    // Update
    let update_resp = client
        .put_resource_policy()
        .policy_name("lifecycle-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow"}]}"#)
        .policy_revision_id(&rev1)
        .send()
        .await
        .expect("update should succeed");

    let rev2 = update_resp
        .resource_policy()
        .unwrap()
        .policy_revision_id()
        .unwrap()
        .to_string();
    assert_ne!(rev1, rev2);

    // Delete
    client
        .delete_resource_policy()
        .policy_name("lifecycle-policy")
        .policy_revision_id(&rev2)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let final_list = client.list_resource_policies().send().await.unwrap();
    assert!(
        final_list.resource_policies().is_empty(),
        "policies should be empty after delete"
    );
}

// ============================================================================
// Tests for stub handlers (no-state operations that return defaults)
// ============================================================================

#[tokio::test]
async fn test_batch_get_traces_stub() {
    let client = make_client().await;

    let resp = client
        .batch_get_traces()
        .trace_ids("1-581cf771-a006649127e371903a2de979")
        .send()
        .await
        .expect("batch_get_traces stub should succeed");

    assert!(resp.traces().is_empty(), "stub should return empty traces");
}

#[tokio::test]
async fn test_get_encryption_config_stub() {
    let client = make_client().await;

    let resp = client
        .get_encryption_config()
        .send()
        .await
        .expect("get_encryption_config stub should succeed");

    // Stub returns a default; encryption_config may or may not be present
    let _ = resp.encryption_config();
}

#[tokio::test]
async fn test_put_encryption_config_stub() {
    use aws_sdk_xray::types::EncryptionType;

    let client = make_client().await;

    let resp = client
        .put_encryption_config()
        .r#type(EncryptionType::None)
        .send()
        .await
        .expect("put_encryption_config stub should succeed");

    let _ = resp.encryption_config();
}

#[tokio::test]
async fn test_get_sampling_statistic_summaries_stub() {
    let client = make_client().await;

    let resp = client
        .get_sampling_statistic_summaries()
        .send()
        .await
        .expect("get_sampling_statistic_summaries stub should succeed");

    assert!(
        resp.sampling_statistic_summaries().is_empty(),
        "stub should return empty summaries"
    );
}

#[tokio::test]
async fn test_get_sampling_targets_stub() {
    let client = make_client().await;

    let doc = aws_sdk_xray::types::SamplingStatisticsDocument::builder()
        .client_id("test-client")
        .rule_name("Default")
        .request_count(100)
        .sampled_count(5)
        .borrow_count(0)
        .timestamp(aws_smithy_types::DateTime::from_secs(1420070400))
        .build()
        .unwrap();

    let resp = client
        .get_sampling_targets()
        .sampling_statistics_documents(doc)
        .send()
        .await
        .expect("get_sampling_targets stub should succeed");

    assert!(
        resp.sampling_target_documents().is_empty(),
        "stub should return empty targets"
    );
}

#[tokio::test]
async fn test_get_service_graph_stub() {
    let client = make_client().await;

    let resp = client
        .get_service_graph()
        .start_time(aws_smithy_types::DateTime::from_secs(1420070400))
        .end_time(aws_smithy_types::DateTime::from_secs(1420074000))
        .send()
        .await
        .expect("get_service_graph stub should succeed");

    assert!(
        resp.services().is_empty(),
        "stub should return empty services"
    );
}

#[tokio::test]
async fn test_get_trace_graph_stub() {
    let client = make_client().await;

    let resp = client
        .get_trace_graph()
        .trace_ids("1-581cf771-a006649127e371903a2de979")
        .send()
        .await
        .expect("get_trace_graph stub should succeed");

    assert!(
        resp.services().is_empty(),
        "stub should return empty services"
    );
}

#[tokio::test]
async fn test_get_trace_summaries_stub() {
    let client = make_client().await;

    let resp = client
        .get_trace_summaries()
        .start_time(aws_smithy_types::DateTime::from_secs(1420070400))
        .end_time(aws_smithy_types::DateTime::from_secs(1420074000))
        .send()
        .await
        .expect("get_trace_summaries stub should succeed");

    assert!(
        resp.trace_summaries().is_empty(),
        "stub should return empty trace summaries"
    );
}

#[tokio::test]
async fn test_get_time_series_service_statistics_stub() {
    let client = make_client().await;

    let resp = client
        .get_time_series_service_statistics()
        .start_time(aws_smithy_types::DateTime::from_secs(1420070400))
        .end_time(aws_smithy_types::DateTime::from_secs(1420074000))
        .send()
        .await
        .expect("get_time_series_service_statistics stub should succeed");

    assert!(
        resp.time_series_service_statistics().is_empty(),
        "stub should return empty time series"
    );
}

#[tokio::test]
async fn test_get_insight_summaries_stub() {
    let client = make_client().await;

    let resp = client
        .get_insight_summaries()
        .start_time(aws_smithy_types::DateTime::from_secs(1420070400))
        .end_time(aws_smithy_types::DateTime::from_secs(1420074000))
        .send()
        .await
        .expect("get_insight_summaries stub should succeed");

    assert!(
        resp.insight_summaries().is_empty(),
        "stub should return empty insight summaries"
    );
}

#[tokio::test]
async fn test_tag_resource_stub() {
    let client = make_client().await;

    let tag = aws_sdk_xray::types::Tag::builder()
        .key("env")
        .value("test")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_arn("arn:aws:xray:us-east-1:123456789012:group/my-group")
        .tags(tag)
        .send()
        .await
        .expect("tag_resource stub should succeed");
}

#[tokio::test]
async fn test_untag_resource_stub() {
    let client = make_client().await;

    client
        .untag_resource()
        .resource_arn("arn:aws:xray:us-east-1:123456789012:group/my-group")
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource stub should succeed");
}

#[tokio::test]
async fn test_list_tags_for_resource_stub() {
    let client = make_client().await;

    let resp = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:xray:us-east-1:123456789012:group/my-group")
        .send()
        .await
        .expect("list_tags_for_resource stub should succeed");

    assert!(resp.tags().is_empty(), "stub should return empty tags");
}

// ============================================================================
// Additional edge-case tests for sampling rules
// ============================================================================

#[tokio::test]
async fn test_delete_sampling_rule_by_arn() {
    let client = make_client().await;

    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("del-by-arn-rule")
        .resource_arn("*")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(1)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    let create_resp = client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp
        .sampling_rule_record()
        .unwrap()
        .sampling_rule()
        .unwrap()
        .rule_arn()
        .unwrap()
        .to_string();

    let del_resp = client
        .delete_sampling_rule()
        .rule_arn(&arn)
        .send()
        .await
        .expect("delete_sampling_rule by ARN should succeed");

    let sr = del_resp
        .sampling_rule_record()
        .unwrap()
        .sampling_rule()
        .unwrap();
    assert_eq!(sr.rule_name(), Some("del-by-arn-rule"));

    // Verify gone
    let list_resp = client.get_sampling_rules().send().await.unwrap();
    assert!(
        list_resp.sampling_rule_records().is_empty(),
        "rules should be empty after ARN delete"
    );
}

#[tokio::test]
async fn test_update_sampling_rule_by_arn() {
    let client = make_client().await;

    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("upd-by-arn-rule")
        .resource_arn("*")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(1)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    let create_resp = client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await
        .unwrap();

    let arn = create_resp
        .sampling_rule_record()
        .unwrap()
        .sampling_rule()
        .unwrap()
        .rule_arn()
        .unwrap()
        .to_string();

    let update = aws_sdk_xray::types::SamplingRuleUpdate::builder()
        .rule_arn(&arn)
        .priority(10)
        .fixed_rate(0.9)
        .build();

    let resp = client
        .update_sampling_rule()
        .sampling_rule_update(update)
        .send()
        .await
        .expect("update_sampling_rule by ARN should succeed");

    let sr = resp
        .sampling_rule_record()
        .unwrap()
        .sampling_rule()
        .unwrap();
    assert_eq!(sr.rule_name(), Some("upd-by-arn-rule"));
    assert_eq!(sr.priority(), 10);
    assert!((sr.fixed_rate() - 0.9).abs() < f64::EPSILON);
}

#[tokio::test]
async fn test_create_sampling_rule_arn_format() {
    let client = make_client().await;

    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("arn-fmt-rule")
        .resource_arn("*")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(1)
        .service_name("*")
        .service_type("*")
        .host("*")
        .http_method("*")
        .url_path("*")
        .version(1)
        .build()
        .unwrap();

    let resp = client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await
        .unwrap();

    let sr = resp
        .sampling_rule_record()
        .unwrap()
        .sampling_rule()
        .unwrap();
    let arn = sr.rule_arn().unwrap();
    assert!(
        arn.starts_with("arn:aws:xray:"),
        "ARN should start with arn:aws:xray:, got: {arn}"
    );
    assert!(
        arn.contains("sampling-rule/arn-fmt-rule"),
        "ARN should contain sampling-rule/rule-name, got: {arn}"
    );
}

// ============================================================================
// Additional edge-case tests for groups
// ============================================================================

#[tokio::test]
async fn test_get_groups_after_delete() {
    let client = make_client().await;

    client
        .create_group()
        .group_name("groups-del-a")
        .send()
        .await
        .unwrap();

    client
        .create_group()
        .group_name("groups-del-b")
        .send()
        .await
        .unwrap();

    client
        .delete_group()
        .group_name("groups-del-a")
        .send()
        .await
        .unwrap();

    let resp = client.get_groups().send().await.unwrap();
    let groups = resp.groups();
    assert_eq!(groups.len(), 1, "should have 1 group after delete");
    assert_eq!(groups[0].group_name(), Some("groups-del-b"));
}

#[tokio::test]
async fn test_update_group_partial_insights() {
    // Update only insights_enabled without touching notifications_enabled
    let client = make_client().await;

    client
        .create_group()
        .group_name("partial-insights-group")
        .insights_configuration(
            aws_sdk_xray::types::InsightsConfiguration::builder()
                .insights_enabled(false)
                .notifications_enabled(true)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .update_group()
        .group_name("partial-insights-group")
        .insights_configuration(
            aws_sdk_xray::types::InsightsConfiguration::builder()
                .insights_enabled(true)
                .build(),
        )
        .send()
        .await
        .expect("partial update should succeed");

    let ic = resp.group().unwrap().insights_configuration().unwrap();
    assert_eq!(
        ic.insights_enabled(),
        Some(true),
        "insights_enabled should be updated"
    );
}

// ============================================================================
// Tests for resource policy edge cases
// ============================================================================

#[tokio::test]
async fn test_put_resource_policy_overwrite_without_revision() {
    // PutResourcePolicy without revision ID should create or overwrite
    let client = make_client().await;

    client
        .put_resource_policy()
        .policy_name("overwrite-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    // Overwrite without providing revision ID (should succeed as new create)
    let resp = client
        .put_resource_policy()
        .policy_name("overwrite-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Deny"}]}"#)
        .send()
        .await
        .expect("overwrite without revision ID should succeed");

    assert_eq!(
        resp.resource_policy().unwrap().policy_document(),
        Some(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Deny"}]}"#)
    );
}

#[tokio::test]
async fn test_sampling_rule_update_preserves_unchanged_fields() {
    // Update only priority; other fields should remain unchanged
    let client = make_client().await;

    let rule = aws_sdk_xray::types::SamplingRule::builder()
        .rule_name("preserve-fields-rule")
        .resource_arn("arn:aws:lambda:us-east-1:123456789012:function:my-func")
        .priority(100)
        .fixed_rate(0.05)
        .reservoir_size(5)
        .service_name("my-svc")
        .service_type("AWS::Lambda::Function")
        .host("example.com")
        .http_method("GET")
        .url_path("/api/*")
        .version(1)
        .build()
        .unwrap();

    client
        .create_sampling_rule()
        .sampling_rule(rule)
        .send()
        .await
        .unwrap();

    let update = aws_sdk_xray::types::SamplingRuleUpdate::builder()
        .rule_name("preserve-fields-rule")
        .priority(1)
        .build();

    let resp = client
        .update_sampling_rule()
        .sampling_rule_update(update)
        .send()
        .await
        .expect("partial update should succeed");

    let sr = resp
        .sampling_rule_record()
        .unwrap()
        .sampling_rule()
        .unwrap();
    assert_eq!(sr.priority(), 1, "priority should be updated");
    assert!(
        (sr.fixed_rate() - 0.05).abs() < f64::EPSILON,
        "fixed_rate should be preserved"
    );
    assert_eq!(sr.reservoir_size(), 5, "reservoir_size should be preserved");
    assert_eq!(
        sr.service_name(),
        "my-svc",
        "service_name should be preserved"
    );
    assert_eq!(sr.host(), "example.com", "host should be preserved");
    assert_eq!(sr.http_method(), "GET", "http_method should be preserved");
    assert_eq!(sr.url_path(), "/api/*", "url_path should be preserved");
}

// ============================================================================
// BatchGetTraces tests
// ============================================================================

#[tokio::test]
async fn test_batch_get_traces_empty() {
    // BatchGetTraces with a trace ID that was never submitted returns it as unprocessed.
    let client = make_client().await;

    let resp = client
        .batch_get_traces()
        .trace_ids("1-00000000-000000000000000000000000")
        .send()
        .await
        .expect("batch_get_traces should succeed even for unknown IDs");

    assert!(
        resp.traces().is_empty(),
        "no traces should be found for unknown ID"
    );
    assert_eq!(
        resp.unprocessed_trace_ids().len(),
        1,
        "unknown trace ID should be unprocessed"
    );
}

#[tokio::test]
async fn test_batch_get_traces_returns_stored_segments() {
    use serde_json::json;

    let client = make_client().await;

    let trace_id = "1-581cf771-a006649127e371903a2de979";
    let seg = json!({
        "name": "example.com",
        "id": "70de5b6f19ff9a0a",
        "trace_id": trace_id,
        "start_time": 1478293361.271,
        "end_time": 1478293361.449
    })
    .to_string();

    client
        .put_trace_segments()
        .trace_segment_documents(seg)
        .send()
        .await
        .expect("put_trace_segments should succeed");

    let resp = client
        .batch_get_traces()
        .trace_ids(trace_id)
        .send()
        .await
        .expect("batch_get_traces should succeed");

    assert_eq!(resp.traces().len(), 1, "should find one trace");
    let trace = &resp.traces()[0];
    assert_eq!(trace.id(), Some(trace_id));
    assert!(!trace.segments().is_empty(), "trace should have segments");
    assert!(resp.unprocessed_trace_ids().is_empty());
}

// ============================================================================
// GetTraceSummaries tests
// ============================================================================

#[tokio::test]
async fn test_get_trace_summaries_empty_range() {
    // A time range that has no traces returns an empty list.
    use aws_smithy_types::DateTime;

    let client = make_client().await;

    let resp = client
        .get_trace_summaries()
        .start_time(DateTime::from_secs(0))
        .end_time(DateTime::from_secs(1))
        .send()
        .await
        .expect("get_trace_summaries should succeed");

    assert!(
        resp.trace_summaries().is_empty(),
        "no summaries in empty time range"
    );
    assert_eq!(resp.traces_processed_count(), Some(0));
}

#[tokio::test]
async fn test_get_trace_summaries_returns_finished_traces() {
    use aws_smithy_types::DateTime;
    use serde_json::json;

    let client = make_client().await;

    let trace_id = "1-581cf771-b006649127e371903a2de979";
    // start_time: 1478293361 = 2016-11-04T…
    // end_time: 1478293362
    let seg = json!({
        "name": "my-service",
        "id": "aaabbbccddeeff00",
        "trace_id": trace_id,
        "start_time": 1478293361.271,
        "end_time": 1478293362.449
    })
    .to_string();

    client
        .put_trace_segments()
        .trace_segment_documents(seg)
        .send()
        .await
        .expect("put should succeed");

    // Query window that covers both start and end of the trace
    let resp = client
        .get_trace_summaries()
        .start_time(DateTime::from_secs(1478293360))
        .end_time(DateTime::from_secs(1478293370))
        .send()
        .await
        .expect("get_trace_summaries should succeed");

    assert_eq!(
        resp.traces_processed_count(),
        Some(1),
        "should find one trace"
    );
    assert_eq!(resp.trace_summaries().len(), 1);
    let summary = &resp.trace_summaries()[0];
    assert_eq!(summary.id(), Some(trace_id));
    assert!(summary.duration().is_some());
}

#[tokio::test]
async fn test_get_trace_summaries_excludes_in_progress() {
    // A segment with no end_time is "in-progress" and should not appear in summaries.
    use aws_smithy_types::DateTime;
    use serde_json::json;

    let client = make_client().await;

    let trace_id = "1-581cf771-c006649127e371903a2de979";
    let seg = json!({
        "name": "my-service",
        "id": "1122334455667788",
        "trace_id": trace_id,
        "start_time": 1478293361.271
        // no end_time — in-progress
    })
    .to_string();

    client
        .put_trace_segments()
        .trace_segment_documents(seg)
        .send()
        .await
        .expect("put should succeed");

    let resp = client
        .get_trace_summaries()
        .start_time(DateTime::from_secs(1478293360))
        .end_time(DateTime::from_secs(1478293370))
        .send()
        .await
        .expect("get_trace_summaries should succeed");

    assert!(
        resp.trace_summaries().is_empty(),
        "in-progress traces should not appear in summaries"
    );
}

// ============================================================================
// GetServiceGraph tests
// ============================================================================

#[tokio::test]
async fn test_get_service_graph_empty_services() {
    use aws_smithy_types::DateTime;

    let client = make_client().await;

    let resp = client
        .get_service_graph()
        .start_time(DateTime::from_secs(1478293360))
        .end_time(DateTime::from_secs(1478293370))
        .send()
        .await
        .expect("get_service_graph should succeed");

    // Our implementation echoes back start/end and returns an empty services list
    assert!(
        resp.services().is_empty(),
        "no services in mock implementation"
    );
}

// ============================================================================
// GetTraceGraph tests
// ============================================================================

#[tokio::test]
async fn test_get_trace_graph_empty_services() {
    let client = make_client().await;

    let resp = client
        .get_trace_graph()
        .trace_ids("1-581cf771-a006649127e371903a2de979")
        .send()
        .await
        .expect("get_trace_graph should succeed");

    assert!(
        resp.services().is_empty(),
        "no services in mock trace graph"
    );
}

// ============================================================================
// GetSamplingTargets tests
// ============================================================================

#[tokio::test]
async fn test_get_sampling_targets_returns_known_rule() {
    use aws_sdk_xray::types::{SamplingRule, SamplingStatisticsDocument};

    let client = make_client().await;

    // Create a sampling rule first
    client
        .create_sampling_rule()
        .sampling_rule(
            SamplingRule::builder()
                .rule_name("test-rule-targets")
                .resource_arn("*")
                .priority(1000)
                .fixed_rate(0.05)
                .reservoir_size(5)
                .service_name("*")
                .service_type("*")
                .host("*")
                .http_method("*")
                .url_path("*")
                .version(1)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_sampling_rule should succeed");

    let stats_doc = SamplingStatisticsDocument::builder()
        .rule_name("test-rule-targets")
        .client_id("test-client")
        .timestamp(aws_smithy_types::DateTime::from_secs(0))
        .request_count(100)
        .sampled_count(5)
        .build()
        .unwrap();

    let resp = client
        .get_sampling_targets()
        .sampling_statistics_documents(stats_doc)
        .send()
        .await
        .expect("get_sampling_targets should succeed");

    assert_eq!(
        resp.sampling_target_documents().len(),
        1,
        "should return one target for the known rule"
    );
    let target = &resp.sampling_target_documents()[0];
    assert_eq!(target.rule_name(), Some("test-rule-targets"));
    // fixed_rate() returns f64 in the SDK, confirm it's non-negative
    assert!(target.fixed_rate() >= 0.0);
}

#[tokio::test]
async fn test_get_sampling_targets_unknown_rule_unprocessed() {
    use aws_sdk_xray::types::SamplingStatisticsDocument;

    let client = make_client().await;

    let stats_doc = SamplingStatisticsDocument::builder()
        .rule_name("nonexistent-rule")
        .client_id("test-client")
        .timestamp(aws_smithy_types::DateTime::from_secs(0))
        .request_count(10)
        .sampled_count(1)
        .build()
        .unwrap();

    let resp = client
        .get_sampling_targets()
        .sampling_statistics_documents(stats_doc)
        .send()
        .await
        .expect("get_sampling_targets should succeed even for unknown rules");

    assert!(
        resp.sampling_target_documents().is_empty(),
        "no targets for unknown rule"
    );
    assert_eq!(
        resp.unprocessed_statistics().len(),
        1,
        "unknown rule should be unprocessed"
    );
}

// ============================================================================
// PutTraceSegments bad JSON test
// ============================================================================

#[tokio::test]
async fn test_put_trace_segments_bad_json_is_unprocessed() {
    let client = make_client().await;

    // A valid JSON object but missing trace_id should be reported as unprocessed
    let bad_doc = r#"{"id":"aabb","name":"my-svc","start_time":1234567890.0}"#.to_string();

    let resp = client
        .put_trace_segments()
        .trace_segment_documents(bad_doc)
        .send()
        .await
        .expect("put_trace_segments should succeed even with bad segment");

    assert_eq!(
        resp.unprocessed_trace_segments().len(),
        1,
        "segment with missing trace_id should be unprocessed"
    );
}
