use aws_sdk_route53resolver::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53resolver::Route53ResolverService;

async fn make_route53resolver_client() -> aws_sdk_route53resolver::Client {
    let mock = MockAws::builder()
        .with_service(Route53ResolverService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53resolver::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_route53resolver::Client::new(&config)
}

// Helper to create a resolver endpoint
async fn create_test_endpoint(client: &aws_sdk_route53resolver::Client, name: &str) -> String {
    let resp = client
        .create_resolver_endpoint()
        .creator_request_id(format!("req-{name}"))
        .direction(aws_sdk_route53resolver::types::ResolverEndpointDirection::Inbound)
        .name(name)
        .security_group_ids("sg-12345678")
        .ip_addresses(
            aws_sdk_route53resolver::types::IpAddressRequest::builder()
                .subnet_id("subnet-12345678")
                .build()
                .unwrap(),
        )
        .ip_addresses(
            aws_sdk_route53resolver::types::IpAddressRequest::builder()
                .subnet_id("subnet-87654321")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    resp.resolver_endpoint().unwrap().id().unwrap().to_string()
}

// ============ Resolver Endpoint tests ============

#[tokio::test]
async fn test_create_resolver_endpoint() {
    let client = make_route53resolver_client().await;

    let resp = client
        .create_resolver_endpoint()
        .creator_request_id("test-request-1")
        .direction(aws_sdk_route53resolver::types::ResolverEndpointDirection::Inbound)
        .name("test-endpoint")
        .security_group_ids("sg-12345678")
        .ip_addresses(
            aws_sdk_route53resolver::types::IpAddressRequest::builder()
                .subnet_id("subnet-12345678")
                .build()
                .unwrap(),
        )
        .ip_addresses(
            aws_sdk_route53resolver::types::IpAddressRequest::builder()
                .subnet_id("subnet-87654321")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_resolver_endpoint should succeed");

    let endpoint = resp
        .resolver_endpoint()
        .expect("should have resolver endpoint");
    assert!(endpoint.id().is_some());
    assert!(endpoint.arn().unwrap().contains("arn:aws:route53resolver:"));
    assert_eq!(endpoint.name(), Some("test-endpoint"));
    assert_eq!(
        endpoint.direction(),
        Some(&aws_sdk_route53resolver::types::ResolverEndpointDirection::Inbound)
    );
    assert_eq!(endpoint.ip_address_count(), Some(2));
}

#[tokio::test]
async fn test_get_resolver_endpoint() {
    let client = make_route53resolver_client().await;
    let endpoint_id = create_test_endpoint(&client, "get-test").await;

    let get_resp = client
        .get_resolver_endpoint()
        .resolver_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("get_resolver_endpoint should succeed");

    let endpoint = get_resp.resolver_endpoint().unwrap();
    assert_eq!(endpoint.name(), Some("get-test"));
}

#[tokio::test]
async fn test_delete_resolver_endpoint() {
    let client = make_route53resolver_client().await;
    let endpoint_id = create_test_endpoint(&client, "delete-test").await;

    let delete_resp = client
        .delete_resolver_endpoint()
        .resolver_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("delete_resolver_endpoint should succeed");

    let endpoint = delete_resp.resolver_endpoint().unwrap();
    assert_eq!(
        endpoint.status(),
        Some(&aws_sdk_route53resolver::types::ResolverEndpointStatus::Deleting)
    );

    // Verify it's gone
    let result = client
        .get_resolver_endpoint()
        .resolver_endpoint_id(&endpoint_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_resolver_endpoints() {
    let client = make_route53resolver_client().await;

    for i in 1..=3 {
        create_test_endpoint(&client, &format!("list-test-{i}")).await;
    }

    let resp = client
        .list_resolver_endpoints()
        .send()
        .await
        .expect("list_resolver_endpoints should succeed");

    assert_eq!(resp.resolver_endpoints().len(), 3);
}

#[tokio::test]
async fn test_get_nonexistent_resolver_endpoint() {
    let client = make_route53resolver_client().await;
    let result = client
        .get_resolver_endpoint()
        .resolver_endpoint_id("rslvr-in-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_resolver_endpoint() {
    let client = make_route53resolver_client().await;
    let result = client
        .delete_resolver_endpoint()
        .resolver_endpoint_id("rslvr-in-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_resolver_endpoint() {
    let client = make_route53resolver_client().await;
    let endpoint_id = create_test_endpoint(&client, "update-test").await;

    let resp = client
        .update_resolver_endpoint()
        .resolver_endpoint_id(&endpoint_id)
        .name("updated-name")
        .send()
        .await
        .expect("update_resolver_endpoint should succeed");

    let endpoint = resp.resolver_endpoint().unwrap();
    assert_eq!(endpoint.name(), Some("updated-name"));
}

#[tokio::test]
async fn test_update_nonexistent_resolver_endpoint() {
    let client = make_route53resolver_client().await;
    let result = client
        .update_resolver_endpoint()
        .resolver_endpoint_id("rslvr-in-nonexistent")
        .name("test")
        .send()
        .await;
    assert!(result.is_err());
}

// ============ Associate/Disassociate IP Address tests ============

#[tokio::test]
async fn test_associate_resolver_endpoint_ip_address() {
    let client = make_route53resolver_client().await;
    let endpoint_id = create_test_endpoint(&client, "assoc-ip-test").await;

    let resp = client
        .associate_resolver_endpoint_ip_address()
        .resolver_endpoint_id(&endpoint_id)
        .ip_address(
            aws_sdk_route53resolver::types::IpAddressUpdate::builder()
                .subnet_id("subnet-newsubnet")
                .build(),
        )
        .send()
        .await
        .expect("associate_resolver_endpoint_ip_address should succeed");

    let endpoint = resp.resolver_endpoint().unwrap();
    assert_eq!(endpoint.ip_address_count(), Some(3));
}

#[tokio::test]
async fn test_list_resolver_endpoint_ip_addresses() {
    let client = make_route53resolver_client().await;
    let endpoint_id = create_test_endpoint(&client, "list-ip-test").await;

    let resp = client
        .list_resolver_endpoint_ip_addresses()
        .resolver_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("list_resolver_endpoint_ip_addresses should succeed");

    assert_eq!(resp.ip_addresses().len(), 2);
    let first = &resp.ip_addresses()[0];
    assert!(first.ip_id().is_some());
    assert!(first.subnet_id().is_some());
}

#[tokio::test]
async fn test_disassociate_resolver_endpoint_ip_address() {
    let client = make_route53resolver_client().await;
    let endpoint_id = create_test_endpoint(&client, "disassoc-ip-test").await;

    // Add a third IP so we can remove one (minimum 2)
    client
        .associate_resolver_endpoint_ip_address()
        .resolver_endpoint_id(&endpoint_id)
        .ip_address(
            aws_sdk_route53resolver::types::IpAddressUpdate::builder()
                .subnet_id("subnet-extra")
                .ip("10.0.5.5")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .disassociate_resolver_endpoint_ip_address()
        .resolver_endpoint_id(&endpoint_id)
        .ip_address(
            aws_sdk_route53resolver::types::IpAddressUpdate::builder()
                .subnet_id("subnet-extra")
                .ip("10.0.5.5")
                .build(),
        )
        .send()
        .await
        .expect("disassociate should succeed");

    let endpoint = resp.resolver_endpoint().unwrap();
    assert_eq!(endpoint.ip_address_count(), Some(2));
}

// ============ Resolver Rule tests ============

#[tokio::test]
async fn test_create_resolver_rule() {
    let client = make_route53resolver_client().await;

    let resp = client
        .create_resolver_rule()
        .creator_request_id("rule-req-1")
        .domain_name("example.com.")
        .name("test-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .target_ips(
            aws_sdk_route53resolver::types::TargetAddress::builder()
                .ip("10.0.0.1")
                .port(53)
                .build(),
        )
        .send()
        .await
        .expect("create_resolver_rule should succeed");

    let rule = resp.resolver_rule().unwrap();
    assert!(rule.id().is_some());
    assert_eq!(rule.name(), Some("test-rule"));
    assert_eq!(rule.domain_name(), Some("example.com."));
    assert_eq!(
        rule.rule_type(),
        Some(&aws_sdk_route53resolver::types::RuleTypeOption::Forward)
    );
    assert_eq!(
        rule.status(),
        Some(&aws_sdk_route53resolver::types::ResolverRuleStatus::Complete)
    );
}

#[tokio::test]
async fn test_get_resolver_rule() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_rule()
        .creator_request_id("rule-get-1")
        .domain_name("test.example.com.")
        .name("get-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .target_ips(
            aws_sdk_route53resolver::types::TargetAddress::builder()
                .ip("10.0.0.2")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let rule_id = create_resp
        .resolver_rule()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .get_resolver_rule()
        .resolver_rule_id(&rule_id)
        .send()
        .await
        .expect("get_resolver_rule should succeed");

    let rule = resp.resolver_rule().unwrap();
    assert_eq!(rule.name(), Some("get-rule"));
    assert_eq!(rule.domain_name(), Some("test.example.com."));
}

#[tokio::test]
async fn test_delete_resolver_rule() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_rule()
        .creator_request_id("rule-del-1")
        .domain_name("del.example.com.")
        .name("delete-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .target_ips(
            aws_sdk_route53resolver::types::TargetAddress::builder()
                .ip("10.0.0.3")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let rule_id = create_resp
        .resolver_rule()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .delete_resolver_rule()
        .resolver_rule_id(&rule_id)
        .send()
        .await
        .expect("delete_resolver_rule should succeed");

    let rule = resp.resolver_rule().unwrap();
    assert_eq!(
        rule.status(),
        Some(&aws_sdk_route53resolver::types::ResolverRuleStatus::Deleting)
    );

    // Verify deleted
    let result = client
        .get_resolver_rule()
        .resolver_rule_id(&rule_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_resolver_rules() {
    let client = make_route53resolver_client().await;

    for i in 1..=2 {
        client
            .create_resolver_rule()
            .creator_request_id(format!("rule-list-{i}"))
            .domain_name(format!("list{i}.example.com."))
            .name(format!("list-rule-{i}"))
            .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
            .target_ips(
                aws_sdk_route53resolver::types::TargetAddress::builder()
                    .ip("10.0.0.1")
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_resolver_rules()
        .send()
        .await
        .expect("list_resolver_rules should succeed");

    assert_eq!(resp.resolver_rules().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_resolver_rule() {
    let client = make_route53resolver_client().await;
    let result = client
        .get_resolver_rule()
        .resolver_rule_id("rslvr-rr-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ============ Resolver Rule Association tests ============

#[tokio::test]
async fn test_associate_resolver_rule() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_rule()
        .creator_request_id("rule-assoc-1")
        .domain_name("assoc.example.com.")
        .name("assoc-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .target_ips(
            aws_sdk_route53resolver::types::TargetAddress::builder()
                .ip("10.0.0.1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let rule_id = create_resp
        .resolver_rule()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .associate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-12345678")
        .name("my-assoc")
        .send()
        .await
        .expect("associate_resolver_rule should succeed");

    let assoc = resp.resolver_rule_association().unwrap();
    assert!(assoc.id().is_some());
    assert_eq!(assoc.resolver_rule_id(), Some(rule_id.as_str()));
    assert_eq!(assoc.vpc_id(), Some("vpc-12345678"));
    assert_eq!(
        assoc.status(),
        Some(&aws_sdk_route53resolver::types::ResolverRuleAssociationStatus::Complete)
    );
}

#[tokio::test]
async fn test_disassociate_resolver_rule() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_rule()
        .creator_request_id("rule-disassoc-1")
        .domain_name("disassoc.example.com.")
        .name("disassoc-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .target_ips(
            aws_sdk_route53resolver::types::TargetAddress::builder()
                .ip("10.0.0.1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let rule_id = create_resp
        .resolver_rule()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .associate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-disassoc")
        .name("disassoc-test")
        .send()
        .await
        .unwrap();

    let resp = client
        .disassociate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-disassoc")
        .send()
        .await
        .expect("disassociate_resolver_rule should succeed");

    let assoc = resp.resolver_rule_association().unwrap();
    assert_eq!(
        assoc.status(),
        Some(&aws_sdk_route53resolver::types::ResolverRuleAssociationStatus::Deleting)
    );
}

#[tokio::test]
async fn test_get_resolver_rule_association() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_rule()
        .creator_request_id("rule-get-assoc-1")
        .domain_name("getassoc.example.com.")
        .name("get-assoc-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .target_ips(
            aws_sdk_route53resolver::types::TargetAddress::builder()
                .ip("10.0.0.1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let rule_id = create_resp
        .resolver_rule()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let assoc_resp = client
        .associate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-getassoc")
        .name("get-assoc-test")
        .send()
        .await
        .unwrap();

    let assoc_id = assoc_resp
        .resolver_rule_association()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .get_resolver_rule_association()
        .resolver_rule_association_id(&assoc_id)
        .send()
        .await
        .expect("get_resolver_rule_association should succeed");

    let assoc = resp.resolver_rule_association().unwrap();
    assert_eq!(assoc.name(), Some("get-assoc-test"));
}

#[tokio::test]
async fn test_list_resolver_rule_associations() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_rule()
        .creator_request_id("rule-list-assoc-1")
        .domain_name("listassoc.example.com.")
        .name("list-assoc-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .target_ips(
            aws_sdk_route53resolver::types::TargetAddress::builder()
                .ip("10.0.0.1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let rule_id = create_resp
        .resolver_rule()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .associate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-list1")
        .name("list-1")
        .send()
        .await
        .unwrap();

    client
        .associate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-list2")
        .name("list-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_resolver_rule_associations()
        .send()
        .await
        .expect("list_resolver_rule_associations should succeed");

    assert_eq!(resp.resolver_rule_associations().len(), 2);
}

// ============ Query Log Config tests ============

#[tokio::test]
async fn test_create_resolver_query_log_config() {
    let client = make_route53resolver_client().await;

    let resp = client
        .create_resolver_query_log_config()
        .name("test-log-config")
        .destination_arn("arn:aws:s3:::my-query-logs")
        .creator_request_id("log-req-1")
        .send()
        .await
        .expect("create_resolver_query_log_config should succeed");

    let config = resp.resolver_query_log_config().unwrap();
    assert!(config.id().is_some());
    assert_eq!(config.name(), Some("test-log-config"));
    assert_eq!(config.destination_arn(), Some("arn:aws:s3:::my-query-logs"));
    assert_eq!(
        config.status(),
        Some(&aws_sdk_route53resolver::types::ResolverQueryLogConfigStatus::Created)
    );
}

#[tokio::test]
async fn test_get_resolver_query_log_config() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_query_log_config()
        .name("get-log-config")
        .destination_arn("arn:aws:s3:::my-query-logs-get")
        .creator_request_id("log-get-1")
        .send()
        .await
        .unwrap();

    let config_id = create_resp
        .resolver_query_log_config()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .get_resolver_query_log_config()
        .resolver_query_log_config_id(&config_id)
        .send()
        .await
        .expect("get_resolver_query_log_config should succeed");

    let config = resp.resolver_query_log_config().unwrap();
    assert_eq!(config.name(), Some("get-log-config"));
}

#[tokio::test]
async fn test_list_resolver_query_log_configs() {
    let client = make_route53resolver_client().await;

    client
        .create_resolver_query_log_config()
        .name("list-log-1")
        .destination_arn("arn:aws:s3:::logs-1")
        .creator_request_id("list-log-req-1")
        .send()
        .await
        .unwrap();

    client
        .create_resolver_query_log_config()
        .name("list-log-2")
        .destination_arn("arn:aws:s3:::logs-2")
        .creator_request_id("list-log-req-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_resolver_query_log_configs()
        .send()
        .await
        .expect("list_resolver_query_log_configs should succeed");

    assert_eq!(resp.resolver_query_log_configs().len(), 2);
    assert_eq!(resp.total_count(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_query_log_config() {
    let client = make_route53resolver_client().await;
    let result = client
        .get_resolver_query_log_config()
        .resolver_query_log_config_id("rqlc-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ============ Query Log Config Association tests ============

#[tokio::test]
async fn test_associate_resolver_query_log_config() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_query_log_config()
        .name("assoc-log-config")
        .destination_arn("arn:aws:s3:::logs-assoc")
        .creator_request_id("assoc-log-req")
        .send()
        .await
        .unwrap();

    let config_id = create_resp
        .resolver_query_log_config()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .associate_resolver_query_log_config()
        .resolver_query_log_config_id(&config_id)
        .resource_id("vpc-12345678")
        .send()
        .await
        .expect("associate_resolver_query_log_config should succeed");

    let assoc = resp.resolver_query_log_config_association().unwrap();
    assert!(assoc.id().is_some());
    assert_eq!(
        assoc.resolver_query_log_config_id(),
        Some(config_id.as_str())
    );
    assert_eq!(assoc.resource_id(), Some("vpc-12345678"));
    assert_eq!(
        assoc.status(),
        Some(&aws_sdk_route53resolver::types::ResolverQueryLogConfigAssociationStatus::Active)
    );
}

#[tokio::test]
async fn test_get_resolver_query_log_config_association() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_query_log_config()
        .name("get-assoc-log")
        .destination_arn("arn:aws:s3:::logs-get-assoc")
        .creator_request_id("get-assoc-log-req")
        .send()
        .await
        .unwrap();

    let config_id = create_resp
        .resolver_query_log_config()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let assoc_resp = client
        .associate_resolver_query_log_config()
        .resolver_query_log_config_id(&config_id)
        .resource_id("vpc-getassoc")
        .send()
        .await
        .unwrap();

    let assoc_id = assoc_resp
        .resolver_query_log_config_association()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .get_resolver_query_log_config_association()
        .resolver_query_log_config_association_id(&assoc_id)
        .send()
        .await
        .expect("get_resolver_query_log_config_association should succeed");

    let assoc = resp.resolver_query_log_config_association().unwrap();
    assert_eq!(assoc.resource_id(), Some("vpc-getassoc"));
}

#[tokio::test]
async fn test_list_resolver_query_log_config_associations() {
    let client = make_route53resolver_client().await;

    let create_resp = client
        .create_resolver_query_log_config()
        .name("list-assoc-log")
        .destination_arn("arn:aws:s3:::logs-list-assoc")
        .creator_request_id("list-assoc-log-req")
        .send()
        .await
        .unwrap();

    let config_id = create_resp
        .resolver_query_log_config()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .associate_resolver_query_log_config()
        .resolver_query_log_config_id(&config_id)
        .resource_id("vpc-list1")
        .send()
        .await
        .unwrap();

    client
        .associate_resolver_query_log_config()
        .resolver_query_log_config_id(&config_id)
        .resource_id("vpc-list2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_resolver_query_log_config_associations()
        .send()
        .await
        .expect("list_resolver_query_log_config_associations should succeed");

    assert_eq!(resp.resolver_query_log_config_associations().len(), 2);
    assert_eq!(resp.total_count(), 2);
}

// ============ DNSSEC Config tests ============

#[tokio::test]
async fn test_get_resolver_dnssec_config() {
    let client = make_route53resolver_client().await;

    let resp = client
        .get_resolver_dnssec_config()
        .resource_id("vpc-dnssec-test")
        .send()
        .await
        .expect("get_resolver_dnssec_config should succeed");

    let config = resp.resolver_dnssec_config().unwrap();
    assert!(config.id().is_some());
    assert_eq!(config.resource_id(), Some("vpc-dnssec-test"));
    assert_eq!(
        config.validation_status(),
        Some(&aws_sdk_route53resolver::types::ResolverDnssecValidationStatus::Disabled)
    );
}

#[tokio::test]
async fn test_update_resolver_dnssec_config() {
    let client = make_route53resolver_client().await;

    let resp = client
        .update_resolver_dnssec_config()
        .resource_id("vpc-dnssec-update")
        .validation(aws_sdk_route53resolver::types::Validation::Enable)
        .send()
        .await
        .expect("update_resolver_dnssec_config should succeed");

    let config = resp.resolver_dnssec_config().unwrap();
    assert_eq!(config.resource_id(), Some("vpc-dnssec-update"));
    assert_eq!(
        config.validation_status(),
        Some(&aws_sdk_route53resolver::types::ResolverDnssecValidationStatus::Enabled)
    );
}

#[tokio::test]
async fn test_list_resolver_dnssec_configs() {
    let client = make_route53resolver_client().await;

    // Create some dnssec configs by getting them
    client
        .get_resolver_dnssec_config()
        .resource_id("vpc-list-dnssec-1")
        .send()
        .await
        .unwrap();

    client
        .get_resolver_dnssec_config()
        .resource_id("vpc-list-dnssec-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_resolver_dnssec_configs()
        .send()
        .await
        .expect("list_resolver_dnssec_configs should succeed");

    assert!(resp.resolver_dnssec_configs().len() >= 2);
}

// ============ Tag tests ============

#[tokio::test]
async fn test_tag_resource_and_list_tags() {
    let client = make_route53resolver_client().await;

    let endpoint_id = create_test_endpoint(&client, "tag-test").await;
    let endpoint = client
        .get_resolver_endpoint()
        .resolver_endpoint_id(&endpoint_id)
        .send()
        .await
        .unwrap();
    let arn = endpoint
        .resolver_endpoint()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_route53resolver::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_route53resolver::types::Tag::builder()
                .key("project")
                .value("winterbaume")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);

    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.key(), t.value())).collect();
    assert_eq!(tag_map.get("env"), Some(&"test"));
    assert_eq!(tag_map.get("project"), Some(&"winterbaume"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_route53resolver_client().await;

    let endpoint_id = create_test_endpoint(&client, "untag-test").await;
    let endpoint = client
        .get_resolver_endpoint()
        .resolver_endpoint_id(&endpoint_id)
        .send()
        .await
        .unwrap();
    let arn = endpoint
        .resolver_endpoint()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    // Add tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_route53resolver::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_route53resolver::types::Tag::builder()
                .key("project")
                .value("winterbaume")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Remove one tag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "project");
}

// ============ Lifecycle tests ============

#[tokio::test]
async fn test_resolver_rule_lifecycle() {
    let client = make_route53resolver_client().await;

    // Create rule
    let create_resp = client
        .create_resolver_rule()
        .creator_request_id("lifecycle-1")
        .domain_name("lifecycle.example.com.")
        .name("lifecycle-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .target_ips(
            aws_sdk_route53resolver::types::TargetAddress::builder()
                .ip("10.0.0.1")
                .port(53)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let rule_id = create_resp
        .resolver_rule()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Associate with VPC
    let assoc_resp = client
        .associate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-lifecycle")
        .name("lifecycle-assoc")
        .send()
        .await
        .unwrap();

    let _assoc_id = assoc_resp
        .resolver_rule_association()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Cannot delete associated rule
    let delete_result = client
        .delete_resolver_rule()
        .resolver_rule_id(&rule_id)
        .send()
        .await;
    assert!(delete_result.is_err());

    // Disassociate
    client
        .disassociate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-lifecycle")
        .send()
        .await
        .unwrap();

    // Now deletion should succeed
    client
        .delete_resolver_rule()
        .resolver_rule_id(&rule_id)
        .send()
        .await
        .expect("delete should succeed after disassociation");

    // Verify deleted
    let result = client
        .get_resolver_rule()
        .resolver_rule_id(&rule_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_resolver_dnssec_config_lifecycle() {
    let client = make_route53resolver_client().await;

    let vpc_id = "vpc-dnssec-lifecycle";

    // Enable DNSSEC config
    let enable_resp = client
        .update_resolver_dnssec_config()
        .resource_id(vpc_id)
        .validation(aws_sdk_route53resolver::types::Validation::Enable)
        .send()
        .await
        .expect("enable dnssec should succeed");
    let config = enable_resp.resolver_dnssec_config().unwrap();
    assert_eq!(config.resource_id().unwrap_or(""), vpc_id);
    let config_id = config.id().unwrap().to_string();

    // Get DNSSEC config
    let get_resp = client
        .get_resolver_dnssec_config()
        .resource_id(vpc_id)
        .send()
        .await
        .expect("get dnssec config should succeed");
    let fetched = get_resp.resolver_dnssec_config().unwrap();
    assert_eq!(fetched.id().unwrap(), config_id);
    assert_eq!(fetched.resource_id().unwrap_or(""), vpc_id);

    // Disable DNSSEC config
    let disable_resp = client
        .update_resolver_dnssec_config()
        .resource_id(vpc_id)
        .validation(aws_sdk_route53resolver::types::Validation::Disable)
        .send()
        .await
        .expect("disable dnssec should succeed");
    let disabled = disable_resp.resolver_dnssec_config().unwrap();
    assert_eq!(disabled.resource_id().unwrap_or(""), vpc_id);
}

#[tokio::test]
async fn test_list_resolver_dnssec_configs_empty() {
    let client = make_route53resolver_client().await;

    let resp = client
        .list_resolver_dnssec_configs()
        .send()
        .await
        .expect("list dnssec configs should succeed");
    assert_eq!(resp.resolver_dnssec_configs().len(), 0);
}

#[tokio::test]
async fn test_get_resolver_rule_fields() {
    let client = make_route53resolver_client().await;

    let resp = client
        .create_resolver_rule()
        .creator_request_id("req-fields-test")
        .name("fields-test-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .domain_name("example.com")
        .send()
        .await
        .expect("create resolver rule should succeed");
    let rule = resp.resolver_rule().unwrap();
    let rule_id = rule.id().unwrap().to_string();

    let get_resp = client
        .get_resolver_rule()
        .resolver_rule_id(&rule_id)
        .send()
        .await
        .expect("get resolver rule should succeed");
    let fetched = get_resp.resolver_rule().unwrap();
    assert_eq!(fetched.id().unwrap(), rule_id);
    assert_eq!(fetched.name().unwrap_or(""), "fields-test-rule");
    assert_eq!(fetched.domain_name().unwrap_or(""), "example.com");
}

#[tokio::test]
async fn test_get_nonexistent_resolver_rule_by_id() {
    let client = make_route53resolver_client().await;

    let result: Result<_, _> = client
        .get_resolver_rule()
        .resolver_rule_id("rslvr-rr-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_resolver_rule_association_fields() {
    let client = make_route53resolver_client().await;

    let rule_resp = client
        .create_resolver_rule()
        .creator_request_id("req-assoc-fields")
        .name("assoc-fields-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .domain_name("assoc-fields.com")
        .send()
        .await
        .expect("create rule should succeed");
    let rule_id = rule_resp.resolver_rule().unwrap().id().unwrap().to_string();

    let assoc_resp = client
        .associate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-assoc-fields")
        .name("test-assoc")
        .send()
        .await
        .expect("associate rule should succeed");
    let assoc = assoc_resp.resolver_rule_association().unwrap();
    let assoc_id = assoc.id().unwrap().to_string();
    assert_eq!(assoc.resolver_rule_id().unwrap_or(""), rule_id);
    assert_eq!(assoc.vpc_id().unwrap_or(""), "vpc-assoc-fields");
    assert_eq!(assoc.name().unwrap_or(""), "test-assoc");

    let get_assoc_resp = client
        .get_resolver_rule_association()
        .resolver_rule_association_id(&assoc_id)
        .send()
        .await
        .expect("get association should succeed");
    let fetched_assoc = get_assoc_resp.resolver_rule_association().unwrap();
    assert_eq!(fetched_assoc.id().unwrap(), assoc_id);
    assert_eq!(fetched_assoc.resolver_rule_id().unwrap_or(""), rule_id);
}

#[tokio::test]
async fn test_get_nonexistent_resolver_endpoint_by_id() {
    let client = make_route53resolver_client().await;

    let result: Result<_, _> = client
        .get_resolver_endpoint()
        .resolver_endpoint_id("rslvr-in-nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_resolver_rule_association_fields() {
    let client = make_route53resolver_client().await;

    let rule_resp = client
        .create_resolver_rule()
        .creator_request_id("req-get-assoc-fields")
        .name("get-assoc-fields-rule")
        .rule_type(aws_sdk_route53resolver::types::RuleTypeOption::Forward)
        .domain_name("get-assoc-fields.com")
        .send()
        .await
        .expect("create rule should succeed");
    let rule_id = rule_resp.resolver_rule().unwrap().id().unwrap().to_string();

    let assoc_resp = client
        .associate_resolver_rule()
        .resolver_rule_id(&rule_id)
        .vpc_id("vpc-get-assoc")
        .name("get-assoc-test")
        .send()
        .await
        .expect("associate rule should succeed");
    let assoc_id = assoc_resp
        .resolver_rule_association()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let get_resp = client
        .get_resolver_rule_association()
        .resolver_rule_association_id(&assoc_id)
        .send()
        .await
        .expect("get rule association should succeed");
    let association = get_resp.resolver_rule_association().unwrap();
    assert_eq!(association.id().unwrap(), assoc_id);
    assert_eq!(association.resolver_rule_id().unwrap_or(""), rule_id);
    assert_eq!(association.vpc_id().unwrap_or(""), "vpc-get-assoc");
    assert_eq!(association.name().unwrap_or(""), "get-assoc-test");
}
