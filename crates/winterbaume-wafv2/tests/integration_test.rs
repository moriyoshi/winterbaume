use aws_sdk_wafv2::config::BehaviorVersion;
use aws_sdk_wafv2::types::{
    AllowAction, BlockAction, CaptchaConfig as SdkCaptchaConfig,
    ChallengeConfig as SdkChallengeConfig, CountAction, CustomResponseBody, DefaultAction,
    ImmunityTimeProperty, IpAddressVersion, Regex, Rule, RuleAction, Scope, Statement, Tag,
    VisibilityConfig,
};
use winterbaume_core::MockAws;
use winterbaume_wafv2::WafV2Service;

async fn make_client() -> aws_sdk_wafv2::Client {
    let mock = MockAws::builder().with_service(WafV2Service::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_wafv2::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_wafv2::Client::new(&config)
}

fn allow_action() -> DefaultAction {
    DefaultAction::builder()
        .allow(AllowAction::builder().build())
        .build()
}

fn block_action() -> DefaultAction {
    DefaultAction::builder()
        .block(BlockAction::builder().build())
        .build()
}

fn vis_config() -> VisibilityConfig {
    VisibilityConfig::builder()
        .sampled_requests_enabled(true)
        .cloud_watch_metrics_enabled(true)
        .metric_name("metric")
        .build()
        .unwrap()
}

// ── WebACL tests ──

#[tokio::test]
async fn test_create_and_get_web_acl() {
    let client = make_client().await;

    let create_resp = client
        .create_web_acl()
        .name("my-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .description("Test ACL")
        .send()
        .await
        .expect("create_web_acl should succeed");

    let summary = create_resp.summary().expect("should have summary");
    let id = summary.id().expect("should have id");
    assert_eq!(summary.name().unwrap(), "my-acl");

    let get_resp = client
        .get_web_acl()
        .name("my-acl")
        .scope(Scope::Regional)
        .id(id)
        .send()
        .await
        .expect("get_web_acl should succeed");

    let acl = get_resp.web_acl().expect("should have web acl");
    assert_eq!(acl.name(), "my-acl");
    assert_eq!(acl.description().unwrap(), "Test ACL");
}

#[tokio::test]
async fn test_list_web_acls() {
    let client = make_client().await;

    for name in ["acl-a", "acl-b"] {
        client
            .create_web_acl()
            .name(name)
            .scope(Scope::Regional)
            .default_action(allow_action())
            .visibility_config(vis_config())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_web_acls()
        .scope(Scope::Regional)
        .send()
        .await
        .expect("list_web_acls should succeed");

    assert_eq!(resp.web_acls().len(), 2);
}

#[tokio::test]
async fn test_delete_web_acl() {
    let client = make_client().await;

    let create_resp = client
        .create_web_acl()
        .name("delete-me")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();
    let id = summary.id().unwrap();
    let lock_token = summary.lock_token().unwrap();

    client
        .delete_web_acl()
        .name("delete-me")
        .scope(Scope::Regional)
        .id(id)
        .lock_token(lock_token)
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_web_acl()
        .name("delete-me")
        .scope(Scope::Regional)
        .id(id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_web_acl() {
    let client = make_client().await;

    let create_resp = client
        .create_web_acl()
        .name("update-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .description("original")
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();
    let id = summary.id().unwrap();
    let lock_token = summary.lock_token().unwrap();

    let update_resp = client
        .update_web_acl()
        .name("update-acl")
        .scope(Scope::Regional)
        .id(id)
        .lock_token(lock_token)
        .default_action(block_action())
        .visibility_config(vis_config())
        .description("updated")
        .send()
        .await
        .expect("update should succeed");

    assert!(update_resp.next_lock_token().is_some());
}

#[tokio::test]
async fn test_get_nonexistent_web_acl() {
    let client = make_client().await;

    let result = client
        .get_web_acl()
        .name("no-such-acl")
        .scope(Scope::Regional)
        .id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_delete_then_list_empty() {
    let client = make_client().await;

    let create_resp = client
        .create_web_acl()
        .name("ephemeral-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();
    let id = summary.id().unwrap();
    let lock_token = summary.lock_token().unwrap();

    let list = client
        .list_web_acls()
        .scope(Scope::Regional)
        .send()
        .await
        .unwrap();
    assert_eq!(list.web_acls().len(), 1);

    client
        .delete_web_acl()
        .name("ephemeral-acl")
        .scope(Scope::Regional)
        .id(id)
        .lock_token(lock_token)
        .send()
        .await
        .unwrap();

    let list = client
        .list_web_acls()
        .scope(Scope::Regional)
        .send()
        .await
        .unwrap();
    assert_eq!(list.web_acls().len(), 0);
}

// ── IPSet tests ──

#[tokio::test]
async fn test_create_and_get_ip_set() {
    let client = make_client().await;

    let create_resp = client
        .create_ip_set()
        .name("my-ipset")
        .scope(Scope::Regional)
        .ip_address_version(IpAddressVersion::Ipv4)
        .addresses("10.0.0.0/8")
        .addresses("192.168.0.0/16")
        .description("Test IP set")
        .send()
        .await
        .expect("create_ip_set should succeed");

    let summary = create_resp.summary().expect("should have summary");
    let id = summary.id().expect("should have id");
    assert_eq!(summary.name().unwrap(), "my-ipset");

    let get_resp = client
        .get_ip_set()
        .name("my-ipset")
        .scope(Scope::Regional)
        .id(id)
        .send()
        .await
        .expect("get_ip_set should succeed");

    let ip_set = get_resp.ip_set().expect("should have ip set");
    assert_eq!(ip_set.name(), "my-ipset");
    assert_eq!(ip_set.addresses().len(), 2);
    assert_eq!(ip_set.ip_address_version().as_str(), "IPV4");
}

#[tokio::test]
async fn test_delete_ip_set() {
    let client = make_client().await;

    let create_resp = client
        .create_ip_set()
        .name("delete-ipset")
        .scope(Scope::Regional)
        .ip_address_version(IpAddressVersion::Ipv4)
        .addresses("10.0.0.0/8")
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();

    client
        .delete_ip_set()
        .name("delete-ipset")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .lock_token(summary.lock_token().unwrap())
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_ip_set()
        .name("delete-ipset")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_ip_set() {
    let client = make_client().await;

    let create_resp = client
        .create_ip_set()
        .name("update-ipset")
        .scope(Scope::Regional)
        .ip_address_version(IpAddressVersion::Ipv4)
        .addresses("10.0.0.0/8")
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();

    let update_resp = client
        .update_ip_set()
        .name("update-ipset")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .lock_token(summary.lock_token().unwrap())
        .addresses("192.168.0.0/16")
        .addresses("172.16.0.0/12")
        .description("updated")
        .send()
        .await
        .expect("update should succeed");

    assert!(update_resp.next_lock_token().is_some());

    let get_resp = client
        .get_ip_set()
        .name("update-ipset")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .send()
        .await
        .unwrap();

    let ip_set = get_resp.ip_set().unwrap();
    assert_eq!(ip_set.addresses().len(), 2);
    assert!(ip_set.addresses().contains(&"192.168.0.0/16".to_string()));
}

#[tokio::test]
async fn test_list_ip_sets() {
    let client = make_client().await;

    for name in ["ipset-a", "ipset-b"] {
        client
            .create_ip_set()
            .name(name)
            .scope(Scope::Regional)
            .ip_address_version(IpAddressVersion::Ipv4)
            .addresses("10.0.0.0/8")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_ip_sets()
        .scope(Scope::Regional)
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.ip_sets().len(), 2);
}

// ── RegexPatternSet tests ──

#[tokio::test]
async fn test_create_and_get_regex_pattern_set() {
    let client = make_client().await;

    let create_resp = client
        .create_regex_pattern_set()
        .name("my-regex")
        .scope(Scope::Regional)
        .regular_expression_list(Regex::builder().regex_string("^foo").build())
        .regular_expression_list(Regex::builder().regex_string("bar$").build())
        .description("Test regex set")
        .send()
        .await
        .expect("create should succeed");

    let summary = create_resp.summary().expect("should have summary");
    let id = summary.id().expect("should have id");

    let get_resp = client
        .get_regex_pattern_set()
        .name("my-regex")
        .scope(Scope::Regional)
        .id(id)
        .send()
        .await
        .expect("get should succeed");

    let rps = get_resp.regex_pattern_set().expect("should have set");
    assert_eq!(rps.name().unwrap(), "my-regex");
    assert_eq!(rps.regular_expression_list().len(), 2);
}

#[tokio::test]
async fn test_delete_regex_pattern_set() {
    let client = make_client().await;

    let create_resp = client
        .create_regex_pattern_set()
        .name("del-regex")
        .scope(Scope::Regional)
        .regular_expression_list(Regex::builder().regex_string("x").build())
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();

    client
        .delete_regex_pattern_set()
        .name("del-regex")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .lock_token(summary.lock_token().unwrap())
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_regex_pattern_set()
        .name("del-regex")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_regex_pattern_set() {
    let client = make_client().await;

    let create_resp = client
        .create_regex_pattern_set()
        .name("upd-regex")
        .scope(Scope::Regional)
        .regular_expression_list(Regex::builder().regex_string("old").build())
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();

    let update_resp = client
        .update_regex_pattern_set()
        .name("upd-regex")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .lock_token(summary.lock_token().unwrap())
        .regular_expression_list(Regex::builder().regex_string("new1").build())
        .regular_expression_list(Regex::builder().regex_string("new2").build())
        .send()
        .await
        .expect("update should succeed");

    assert!(update_resp.next_lock_token().is_some());
}

#[tokio::test]
async fn test_list_regex_pattern_sets() {
    let client = make_client().await;

    for name in ["regex-a", "regex-b"] {
        client
            .create_regex_pattern_set()
            .name(name)
            .scope(Scope::Regional)
            .regular_expression_list(Regex::builder().regex_string("x").build())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_regex_pattern_sets()
        .scope(Scope::Regional)
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.regex_pattern_sets().len(), 2);
}

// ── RuleGroup tests ──

#[tokio::test]
async fn test_create_and_get_rule_group() {
    let client = make_client().await;

    let create_resp = client
        .create_rule_group()
        .name("my-rg")
        .scope(Scope::Regional)
        .capacity(100)
        .visibility_config(vis_config())
        .description("Test rule group")
        .send()
        .await
        .expect("create should succeed");

    let summary = create_resp.summary().expect("should have summary");
    let id = summary.id().expect("should have id");

    let get_resp = client
        .get_rule_group()
        .name("my-rg")
        .scope(Scope::Regional)
        .id(id)
        .send()
        .await
        .expect("get should succeed");

    let rg = get_resp.rule_group().expect("should have rule group");
    assert_eq!(rg.name(), "my-rg");
    assert_eq!(rg.capacity(), 100);
}

#[tokio::test]
async fn test_delete_rule_group() {
    let client = make_client().await;

    let create_resp = client
        .create_rule_group()
        .name("del-rg")
        .scope(Scope::Regional)
        .capacity(50)
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();

    client
        .delete_rule_group()
        .name("del-rg")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .lock_token(summary.lock_token().unwrap())
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_rule_group()
        .name("del-rg")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_rule_group() {
    let client = make_client().await;

    let create_resp = client
        .create_rule_group()
        .name("upd-rg")
        .scope(Scope::Regional)
        .capacity(100)
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();

    let update_resp = client
        .update_rule_group()
        .name("upd-rg")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .lock_token(summary.lock_token().unwrap())
        .visibility_config(vis_config())
        .description("updated rg")
        .send()
        .await
        .expect("update should succeed");

    assert!(update_resp.next_lock_token().is_some());
}

#[tokio::test]
async fn test_list_rule_groups() {
    let client = make_client().await;

    for name in ["rg-a", "rg-b"] {
        client
            .create_rule_group()
            .name(name)
            .scope(Scope::Regional)
            .capacity(10)
            .visibility_config(vis_config())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_rule_groups()
        .scope(Scope::Regional)
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.rule_groups().len(), 2);
}

// ── LoggingConfiguration tests ──

#[tokio::test]
async fn test_put_get_delete_logging_configuration() {
    let client = make_client().await;

    // First create a web ACL so we have a valid resource ARN
    let create_resp = client
        .create_web_acl()
        .name("log-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let acl_arn = create_resp.summary().unwrap().arn().unwrap().to_string();

    // Put logging config
    use aws_sdk_wafv2::types::LoggingConfiguration;
    let put_resp = client
        .put_logging_configuration()
        .logging_configuration(
            LoggingConfiguration::builder()
                .resource_arn(&acl_arn)
                .log_destination_configs(
                    "arn:aws:firehose:us-east-1:123456789012:deliverystream/aws-waf-logs-test",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put should succeed");

    let lc = put_resp
        .logging_configuration()
        .expect("should have config");
    assert_eq!(lc.resource_arn(), &acl_arn);

    // Get logging config
    let get_resp = client
        .get_logging_configuration()
        .resource_arn(&acl_arn)
        .send()
        .await
        .expect("get should succeed");

    assert_eq!(
        get_resp.logging_configuration().unwrap().resource_arn(),
        &acl_arn
    );

    // Delete logging config
    client
        .delete_logging_configuration()
        .resource_arn(&acl_arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .get_logging_configuration()
        .resource_arn(&acl_arn)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_logging_configurations() {
    let client = make_client().await;

    let create_resp = client
        .create_web_acl()
        .name("list-log-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let acl_arn = create_resp.summary().unwrap().arn().unwrap().to_string();

    use aws_sdk_wafv2::types::LoggingConfiguration;
    client
        .put_logging_configuration()
        .logging_configuration(
            LoggingConfiguration::builder()
                .resource_arn(&acl_arn)
                .log_destination_configs(
                    "arn:aws:firehose:us-east-1:123456789012:deliverystream/aws-waf-logs-test",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_logging_configurations()
        .scope(Scope::Regional)
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.logging_configurations().len(), 1);
}

// ── Associate/Disassociate WebACL tests ──

#[tokio::test]
async fn test_associate_and_get_web_acl_for_resource() {
    let client = make_client().await;

    let create_resp = client
        .create_web_acl()
        .name("assoc-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let acl_arn = create_resp.summary().unwrap().arn().unwrap().to_string();
    let resource_arn =
        "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/1234567890";

    client
        .associate_web_acl()
        .web_acl_arn(&acl_arn)
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("associate should succeed");

    let get_resp = client
        .get_web_acl_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("get_web_acl_for_resource should succeed");

    let acl = get_resp.web_acl().expect("should have web acl");
    assert_eq!(acl.name(), "assoc-acl");
}

#[tokio::test]
async fn test_disassociate_web_acl() {
    let client = make_client().await;

    let create_resp = client
        .create_web_acl()
        .name("disassoc-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let acl_arn = create_resp.summary().unwrap().arn().unwrap().to_string();
    let resource_arn =
        "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/9999";

    client
        .associate_web_acl()
        .web_acl_arn(&acl_arn)
        .resource_arn(resource_arn)
        .send()
        .await
        .unwrap();

    client
        .disassociate_web_acl()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let get_resp = client
        .get_web_acl_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("get should succeed but return no acl");

    assert!(get_resp.web_acl().is_none());
}

#[tokio::test]
async fn test_get_web_acl_for_unassociated_resource() {
    let client = make_client().await;

    let resp = client
        .get_web_acl_for_resource()
        .resource_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/0000",
        )
        .send()
        .await
        .expect("should succeed with no acl");

    assert!(resp.web_acl().is_none());
}

// ── Tag tests ──

#[tokio::test]
async fn test_tag_untag_list_tags() {
    let client = make_client().await;

    let create_resp = client
        .create_web_acl()
        .name("tag-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let acl_arn = create_resp.summary().unwrap().arn().unwrap().to_string();

    // Tag
    client
        .tag_resource()
        .resource_arn(&acl_arn)
        .tags(Tag::builder().key("env").value("prod").build().unwrap())
        .tags(
            Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag should succeed");

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&acl_arn)
        .send()
        .await
        .expect("list tags should succeed");

    let tag_info = resp.tag_info_for_resource().expect("should have tag info");
    assert_eq!(tag_info.tag_list().len(), 2);

    // Untag
    client
        .untag_resource()
        .resource_arn(&acl_arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag should succeed");

    // Verify
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&acl_arn)
        .send()
        .await
        .unwrap();

    let tag_info = resp.tag_info_for_resource().unwrap();
    assert_eq!(tag_info.tag_list().len(), 1);
    assert_eq!(tag_info.tag_list()[0].key(), "env");
    assert_eq!(tag_info.tag_list()[0].value(), "prod");
}

#[tokio::test]
async fn test_tag_ip_set() {
    let client = make_client().await;

    let create_resp = client
        .create_ip_set()
        .name("tag-ipset")
        .scope(Scope::Regional)
        .ip_address_version(IpAddressVersion::Ipv4)
        .addresses("10.0.0.0/8")
        .send()
        .await
        .unwrap();

    let arn = create_resp.summary().unwrap().arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("env").value("test").build().unwrap())
        .send()
        .await
        .expect("tag should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tag_info = resp.tag_info_for_resource().unwrap();
    assert_eq!(tag_info.tag_list().len(), 1);
}

// ============================================================================
// Ported from moto: test_wafv2.py
// ============================================================================

// Ported from moto: test_wafv2.py::test_create_web_acl
#[tokio::test]
async fn test_moto_create_web_acl_duplicate_error() {
    let client = make_client().await;

    client
        .create_web_acl()
        .name("John")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    // Duplicate name - should raise error
    let err = client
        .create_web_acl()
        .name("John")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("DuplicateItem") || err_str.contains("duplicate"),
        "Expected duplicate error, got: {err_str}"
    );
}

// Ported from moto: test_wafv2.py::test_create_web_acl (CLOUDFRONT scope)
#[tokio::test]
async fn test_moto_create_web_acl_cloudfront_scope() {
    let client = make_client().await;

    let resp = client
        .create_web_acl()
        .name("Carl")
        .scope(Scope::Cloudfront)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let summary = resp.summary().unwrap();
    let arn = summary.arn().unwrap();
    // CLOUDFRONT scope ACLs should have global in the ARN
    assert!(
        arn.contains(":global/webacl/Carl/"),
        "ARN should contain global path: {arn}"
    );
}

// Ported from moto: test_wafv2.py::test_create_web_acl_with_all_arguments
#[tokio::test]
async fn test_moto_create_web_acl_with_all_arguments() {
    let client = make_client().await;

    let rule_1 = Rule::builder()
        .action(
            RuleAction::builder()
                .allow(AllowAction::builder().build())
                .build(),
        )
        .name("tf-acc-test-rule-1")
        .priority(10)
        .statement(
            Statement::builder()
                .geo_match_statement(
                    aws_sdk_wafv2::types::GeoMatchStatement::builder()
                        .country_codes(aws_sdk_wafv2::types::CountryCode::Us)
                        .country_codes(aws_sdk_wafv2::types::CountryCode::Nl)
                        .build(),
                )
                .build(),
        )
        .visibility_config(
            VisibilityConfig::builder()
                .cloud_watch_metrics_enabled(false)
                .metric_name("tf-acc-test-rule-1")
                .sampled_requests_enabled(false)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let rule_2 = Rule::builder()
        .action(
            RuleAction::builder()
                .count(CountAction::builder().build())
                .build(),
        )
        .name("tf-acc-test-rule-2")
        .priority(5)
        .statement(
            Statement::builder()
                .size_constraint_statement(
                    aws_sdk_wafv2::types::SizeConstraintStatement::builder()
                        .comparison_operator(aws_sdk_wafv2::types::ComparisonOperator::Lt)
                        .field_to_match(
                            aws_sdk_wafv2::types::FieldToMatch::builder()
                                .query_string(aws_sdk_wafv2::types::QueryString::builder().build())
                                .build(),
                        )
                        .size(50)
                        .text_transformations(
                            aws_sdk_wafv2::types::TextTransformation::builder()
                                .priority(2)
                                .r#type(aws_sdk_wafv2::types::TextTransformationType::CmdLine)
                                .build()
                                .unwrap(),
                        )
                        .text_transformations(
                            aws_sdk_wafv2::types::TextTransformation::builder()
                                .priority(5)
                                .r#type(aws_sdk_wafv2::types::TextTransformationType::None)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .visibility_config(
            VisibilityConfig::builder()
                .cloud_watch_metrics_enabled(false)
                .metric_name("tf-acc-test-rule-2")
                .sampled_requests_enabled(false)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let create_resp = client
        .create_web_acl()
        .name("test")
        .scope(Scope::Cloudfront)
        .default_action(allow_action())
        .description("test desc")
        .visibility_config(
            VisibilityConfig::builder()
                .sampled_requests_enabled(false)
                .cloud_watch_metrics_enabled(false)
                .metric_name("idk")
                .build()
                .unwrap(),
        )
        .captcha_config(
            SdkCaptchaConfig::builder()
                .immunity_time_property(
                    ImmunityTimeProperty::builder()
                        .immunity_time(60)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .challenge_config(
            SdkChallengeConfig::builder()
                .immunity_time_property(
                    ImmunityTimeProperty::builder()
                        .immunity_time(60)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .token_domains("test1.com")
        .token_domains("test2.com")
        .tags(
            Tag::builder()
                .key("TestKey")
                .value("TestValue")
                .build()
                .unwrap(),
        )
        .rules(rule_1)
        .rules(rule_2)
        .custom_response_bodies(
            "Test",
            CustomResponseBody::builder()
                .content("test")
                .content_type(aws_sdk_wafv2::types::ResponseContentType::TextPlain)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let web_acl_id = create_resp.summary().unwrap().id().unwrap().to_string();

    let wacl = client
        .get_web_acl()
        .name("test")
        .scope(Scope::Cloudfront)
        .id(&web_acl_id)
        .send()
        .await
        .unwrap();

    let acl = wacl.web_acl().unwrap();
    assert_eq!(acl.description().unwrap(), "test desc");
    assert_eq!(acl.rules().len(), 2);
    assert!(acl.token_domains().contains(&"test1.com".to_string()));
    assert!(acl.token_domains().contains(&"test2.com".to_string()));
    assert!(acl.captcha_config().is_some());
    assert!(acl.challenge_config().is_some());
    assert!(acl.custom_response_bodies().is_some());
}

// Ported from moto: test_wafv2.py::test_get_web_acl (LabelNamespace)
#[tokio::test]
async fn test_moto_get_web_acl_label_namespace() {
    let client = make_client().await;

    let create_resp = client
        .create_web_acl()
        .name("John")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let web_acl_id = create_resp.summary().unwrap().id().unwrap().to_string();

    let wacl = client
        .get_web_acl()
        .name("John")
        .scope(Scope::Regional)
        .id(&web_acl_id)
        .send()
        .await
        .unwrap();

    let acl = wacl.web_acl().unwrap();
    assert_eq!(acl.name(), "John");
    assert_eq!(acl.id(), &web_acl_id);
    // Check LabelNamespace matches moto's format
    let label_ns = acl.label_namespace().unwrap();
    assert!(
        label_ns.contains("webacl:John:"),
        "LabelNamespace should contain 'webacl:John:', got: {label_ns}"
    );
}

// Ported from moto: test_wafv2.py::test_list_web_acl (scope filtering)
#[tokio::test]
async fn test_moto_list_web_acl_scope_filtering() {
    let client = make_client().await;

    client
        .create_web_acl()
        .name("Daphne")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    client
        .create_web_acl()
        .name("Penelope")
        .scope(Scope::Cloudfront)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    for idx in 0..5 {
        client
            .create_web_acl()
            .name(format!("Sarah {idx}"))
            .scope(Scope::Regional)
            .default_action(allow_action())
            .visibility_config(vis_config())
            .send()
            .await
            .unwrap();
    }

    let res = client
        .list_web_acls()
        .scope(Scope::Regional)
        .send()
        .await
        .unwrap();
    assert_eq!(res.web_acls().len(), 6);

    let res = client
        .list_web_acls()
        .scope(Scope::Cloudfront)
        .send()
        .await
        .unwrap();
    assert_eq!(res.web_acls().len(), 1);
    assert_eq!(res.web_acls()[0].name().unwrap(), "Penelope");
}

// Ported from moto: test_wafv2.py::test_delete_web_acl (lock token check + error verification)
#[tokio::test]
async fn test_moto_delete_web_acl_lock_token_check() {
    let client = make_client().await;

    let wacl = client
        .create_web_acl()
        .name("Daphne")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let summary = wacl.summary().unwrap();
    let id = summary.id().unwrap();

    // Wrong lock token should fail
    let err = client
        .delete_web_acl()
        .name("Daphne")
        .scope(Scope::Regional)
        .id(id)
        .lock_token("123")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("OptimisticLock"),
        "Expected OptimisticLock error, got: {err_str}"
    );

    // Correct lock token should succeed
    client
        .delete_web_acl()
        .name("Daphne")
        .scope(Scope::Regional)
        .id(id)
        .lock_token(summary.lock_token().unwrap())
        .send()
        .await
        .unwrap();

    let res = client
        .list_web_acls()
        .scope(Scope::Regional)
        .send()
        .await
        .unwrap();
    assert_eq!(res.web_acls().len(), 0);

    // Get after delete should fail with NonexistentItem
    let err = client
        .get_web_acl()
        .name("Daphne")
        .scope(Scope::Regional)
        .id(id)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Nonexistent"),
        "Expected Nonexistent error, got: {err_str}"
    );
}

// Ported from moto: test_wafv2.py::test_update_web_acl (detailed verification)
#[tokio::test]
async fn test_moto_update_web_acl_detailed() {
    let client = make_client().await;

    let wacl = client
        .create_web_acl()
        .name("Daphne")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let summary = wacl.summary().unwrap();
    let id = summary.id().unwrap();

    let resp = client
        .update_web_acl()
        .name("Daphne")
        .scope(Scope::Regional)
        .id(id)
        .default_action(
            DefaultAction::builder()
                .block(BlockAction::builder().build())
                .build(),
        )
        .description("updated_desc")
        .rules(
            Rule::builder()
                .name("rule1")
                .priority(456)
                .statement(Statement::builder().build())
                .visibility_config(
                    VisibilityConfig::builder()
                        .sampled_requests_enabled(true)
                        .cloud_watch_metrics_enabled(true)
                        .metric_name("updated")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .lock_token(summary.lock_token().unwrap())
        .visibility_config(
            VisibilityConfig::builder()
                .sampled_requests_enabled(true)
                .cloud_watch_metrics_enabled(true)
                .metric_name("updated")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert!(resp.next_lock_token().is_some());

    // Wrong lock token should fail
    let err = client
        .update_web_acl()
        .name("Daphne")
        .scope(Scope::Regional)
        .id(id)
        .default_action(block_action())
        .lock_token("123")
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("OptimisticLock"),
        "Expected OptimisticLock error, got: {err_str}"
    );

    // Verify the update was applied
    let acl = client
        .get_web_acl()
        .name("Daphne")
        .scope(Scope::Regional)
        .id(id)
        .send()
        .await
        .unwrap();

    let web_acl = acl.web_acl().unwrap();
    assert_eq!(web_acl.description().unwrap(), "updated_desc");
    assert!(web_acl.default_action().unwrap().block().is_some());
    assert_eq!(web_acl.rules().len(), 1);
    assert_eq!(web_acl.rules()[0].name(), "rule1");
    assert_eq!(web_acl.rules()[0].priority(), 456);
    assert!(
        web_acl
            .visibility_config()
            .unwrap()
            .sampled_requests_enabled()
    );
}

// ============================================================================
// Ported from moto: test_wafv2.py - IP set tests
// ============================================================================

// Ported from moto: test_wafv2.py::test_ip_set_crud
#[tokio::test]
async fn test_moto_ip_set_crud() {
    let client = make_client().await;

    let create_resp = client
        .create_ip_set()
        .name("test-ip-set")
        .scope(Scope::Cloudfront)
        .description("Test IP set")
        .ip_address_version(IpAddressVersion::Ipv4)
        .addresses("192.168.0.1/32")
        .addresses("10.0.0.0/8")
        .tags(
            Tag::builder()
                .key("Environment")
                .value("Test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();
    assert!(!summary.id().unwrap().is_empty());
    assert!(!summary.lock_token().unwrap().is_empty());
    assert!(!summary.arn().unwrap().is_empty());
    assert_eq!(summary.name().unwrap(), "test-ip-set");

    let get_resp = client
        .get_ip_set()
        .name("test-ip-set")
        .scope(Scope::Cloudfront)
        .id(summary.id().unwrap())
        .send()
        .await
        .unwrap();

    let ip_set = get_resp.ip_set().unwrap();
    assert_eq!(ip_set.ip_address_version().as_str(), "IPV4");
    assert!(get_resp.lock_token().is_some());

    // Update with invalid lock token should fail
    let err = client
        .update_ip_set()
        .name("test-ip-set")
        .scope(Scope::Cloudfront)
        .id(summary.id().unwrap())
        .addresses("10.0.0.0/8")
        .lock_token("aaaaaaaaaaaaaaaaaa")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("OptimisticLock"),
        "Expected OptimisticLock error, got: {err_str}"
    );

    // Update with valid lock token
    let update_resp = client
        .update_ip_set()
        .name("test-ip-set")
        .scope(Scope::Cloudfront)
        .id(summary.id().unwrap())
        .description("Updated test IP set")
        .addresses("192.168.1.0/24")
        .addresses("10.0.0.0/8")
        .lock_token(get_resp.lock_token().unwrap())
        .send()
        .await
        .unwrap();

    assert!(update_resp.next_lock_token().is_some());

    let updated_get = client
        .get_ip_set()
        .name("test-ip-set")
        .scope(Scope::Cloudfront)
        .id(summary.id().unwrap())
        .send()
        .await
        .unwrap();

    let updated_ip_set = updated_get.ip_set().unwrap();
    assert_eq!(updated_ip_set.description().unwrap(), "Updated test IP set");
    assert_eq!(
        updated_get.lock_token().unwrap(),
        update_resp.next_lock_token().unwrap()
    );
    assert!(
        updated_ip_set
            .addresses()
            .contains(&"192.168.1.0/24".to_string())
    );
    assert!(
        updated_ip_set
            .addresses()
            .contains(&"10.0.0.0/8".to_string())
    );

    // List
    let list_resp = client
        .list_ip_sets()
        .scope(Scope::Cloudfront)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.ip_sets().len(), 1);

    // Delete
    client
        .delete_ip_set()
        .name("test-ip-set")
        .scope(Scope::Cloudfront)
        .id(summary.id().unwrap())
        .lock_token(updated_get.lock_token().unwrap())
        .send()
        .await
        .unwrap();

    // Get after delete should fail
    let err = client
        .get_ip_set()
        .name("test-ip-set")
        .scope(Scope::Cloudfront)
        .id(summary.id().unwrap())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Nonexistent"),
        "Expected NonexistentItem error, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_regex_pattern_sets.py
// ============================================================================

// Ported from moto: test_regex_pattern_sets.py::test_regex_pattern_set_crud
#[tokio::test]
async fn test_moto_regex_pattern_set_crud() {
    let client = make_client().await;

    let create_resp = client
        .create_regex_pattern_set()
        .name("test-regex-pattern-set")
        .scope(Scope::Regional)
        .description("Test regex pattern set")
        .regular_expression_list(Regex::builder().regex_string("test.*pattern").build())
        .tags(
            Tag::builder()
                .key("test-key")
                .value("test-value")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();
    assert!(!summary.id().unwrap().is_empty());
    assert!(!summary.lock_token().unwrap().is_empty());
    assert!(!summary.arn().unwrap().is_empty());
    assert_eq!(summary.name().unwrap(), "test-regex-pattern-set");

    // Get
    let get_resp = client
        .get_regex_pattern_set()
        .name("test-regex-pattern-set")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .send()
        .await
        .unwrap();

    let pattern_set = get_resp.regex_pattern_set().unwrap();
    assert_eq!(pattern_set.regular_expression_list().len(), 1);
    assert_eq!(
        pattern_set.regular_expression_list()[0]
            .regex_string()
            .unwrap(),
        "test.*pattern"
    );
    assert!(get_resp.lock_token().is_some());

    // Update with invalid lock token
    let err = client
        .update_regex_pattern_set()
        .name("test-regex-pattern-set")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .description("Updated description")
        .regular_expression_list(Regex::builder().regex_string("updated.*pattern").build())
        .lock_token("invalid-lock-token")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(err_str.contains("OptimisticLock"));

    // Update with valid lock token
    let update_resp = client
        .update_regex_pattern_set()
        .name("test-regex-pattern-set")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .description("Updated description")
        .regular_expression_list(Regex::builder().regex_string("updated.*pattern").build())
        .lock_token(get_resp.lock_token().unwrap())
        .send()
        .await
        .unwrap();

    assert!(update_resp.next_lock_token().is_some());

    // Verify update
    let updated_get = client
        .get_regex_pattern_set()
        .name("test-regex-pattern-set")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .send()
        .await
        .unwrap();

    let updated_set = updated_get.regex_pattern_set().unwrap();
    assert_eq!(updated_set.description().unwrap(), "Updated description");
    assert_eq!(updated_set.regular_expression_list().len(), 1);
    assert_eq!(
        updated_set.regular_expression_list()[0]
            .regex_string()
            .unwrap(),
        "updated.*pattern"
    );
    assert_eq!(
        updated_get.lock_token().unwrap(),
        update_resp.next_lock_token().unwrap()
    );

    // List
    let list_resp = client
        .list_regex_pattern_sets()
        .scope(Scope::Regional)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.regex_pattern_sets().len(), 1);

    // Delete
    client
        .delete_regex_pattern_set()
        .name("test-regex-pattern-set")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .lock_token(updated_get.lock_token().unwrap())
        .send()
        .await
        .unwrap();

    // Verify deletion
    let err = client
        .get_regex_pattern_set()
        .name("test-regex-pattern-set")
        .scope(Scope::Regional)
        .id(summary.id().unwrap())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(err_str.contains("Nonexistent"));
}

// Ported from moto: test_regex_pattern_sets.py::test_duplicate_regex_pattern_set
#[tokio::test]
async fn test_moto_duplicate_regex_pattern_set() {
    let client = make_client().await;

    client
        .create_regex_pattern_set()
        .name("test-regex-pattern-set")
        .scope(Scope::Regional)
        .description("Test regex pattern set")
        .regular_expression_list(Regex::builder().regex_string("test.*pattern").build())
        .send()
        .await
        .unwrap();

    let err = client
        .create_regex_pattern_set()
        .name("test-regex-pattern-set")
        .scope(Scope::Regional)
        .description("Duplicate regex pattern set")
        .regular_expression_list(Regex::builder().regex_string("duplicate.*pattern").build())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("DuplicateItem") || err_str.contains("duplicate"),
        "Expected duplicate error, got: {err_str}"
    );
}

// Ported from moto: test_regex_pattern_sets.py::test_cloudfront_scope
#[tokio::test]
async fn test_moto_regex_cloudfront_scope() {
    let client = make_client().await;

    let create_resp = client
        .create_regex_pattern_set()
        .name("test-cloudfront-regex-set")
        .scope(Scope::Cloudfront)
        .description("Test CloudFront regex pattern set")
        .regular_expression_list(Regex::builder().regex_string("test.*pattern").build())
        .send()
        .await
        .unwrap();

    let summary = create_resp.summary().unwrap();
    let arn = summary.arn().unwrap();
    assert!(
        arn.contains(":us-east-1:"),
        "ARN should contain region: {arn}"
    );

    // List only CLOUDFRONT regex pattern sets
    let list_resp = client
        .list_regex_pattern_sets()
        .scope(Scope::Cloudfront)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.regex_pattern_sets().len(), 1);
}

// ============================================================================
// Ported from moto: test_wafv2_rules.py
// ============================================================================

// Ported from moto: test_wafv2_rules.py::test_create_rule_group
#[tokio::test]
async fn test_moto_create_rule_group() {
    let client = make_client().await;

    let resp = client
        .create_rule_group()
        .capacity(100)
        .name("test-group")
        .scope(Scope::Regional)
        .visibility_config(
            VisibilityConfig::builder()
                .sampled_requests_enabled(false)
                .cloud_watch_metrics_enabled(false)
                .metric_name("test-group")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let summary = resp.summary().unwrap();
    assert!(
        summary
            .arn()
            .unwrap()
            .contains(":regional/rulegroup/test-group/")
    );
    assert_eq!(summary.name().unwrap(), "test-group");
    assert!(!summary.id().unwrap().is_empty());
    assert!(!summary.lock_token().unwrap().is_empty());

    // Duplicate should fail
    let err = client
        .create_rule_group()
        .capacity(100)
        .name("test-group")
        .scope(Scope::Regional)
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("DuplicateItem") || err_str.contains("duplicate"),
        "Expected duplicate error, got: {err_str}"
    );
}

// Ported from moto: test_wafv2_rules.py::test_update_rule_group
#[tokio::test]
async fn test_moto_update_rule_group() {
    let client = make_client().await;

    let group = client
        .create_rule_group()
        .capacity(100)
        .name("test-group")
        .scope(Scope::Regional)
        .visibility_config(
            VisibilityConfig::builder()
                .sampled_requests_enabled(false)
                .cloud_watch_metrics_enabled(false)
                .metric_name("test-group")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let summary = group.summary().unwrap();
    let id = summary.id().unwrap();
    let lock_token = summary.lock_token().unwrap();

    let test_rule = Rule::builder()
        .name("test-rule")
        .priority(0)
        .statement(
            Statement::builder()
                .label_match_statement(
                    aws_sdk_wafv2::types::LabelMatchStatement::builder()
                        .scope(aws_sdk_wafv2::types::LabelMatchScope::Label)
                        .key("testlabelmatch")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .action(
            RuleAction::builder()
                .allow(AllowAction::builder().build())
                .build(),
        )
        .visibility_config(
            VisibilityConfig::builder()
                .sampled_requests_enabled(true)
                .cloud_watch_metrics_enabled(true)
                .metric_name("test-rule")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let resp = client
        .update_rule_group()
        .name("test-group")
        .scope(Scope::Regional)
        .id(id)
        .rules(test_rule)
        .lock_token(lock_token)
        .visibility_config(
            VisibilityConfig::builder()
                .sampled_requests_enabled(true)
                .cloud_watch_metrics_enabled(true)
                .metric_name("new-metric")
                .build()
                .unwrap(),
        )
        .description("test")
        .custom_response_bodies(
            "Test",
            CustomResponseBody::builder()
                .content("ohmylookatthiscontent")
                .content_type(aws_sdk_wafv2::types::ResponseContentType::TextPlain)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let next_lock = resp.next_lock_token().unwrap();
    assert!(!next_lock.is_empty());
    assert_ne!(next_lock, lock_token);

    // Verify the update by getting
    let updated = client
        .get_rule_group()
        .name("test-group")
        .scope(Scope::Regional)
        .id(id)
        .send()
        .await
        .unwrap();

    let rule_group = updated.rule_group().unwrap();
    assert!(
        rule_group
            .visibility_config()
            .unwrap()
            .sampled_requests_enabled()
    );
    assert!(
        rule_group
            .visibility_config()
            .unwrap()
            .cloud_watch_metrics_enabled()
    );
    assert_eq!(
        rule_group.visibility_config().unwrap().metric_name(),
        "new-metric"
    );
    assert_eq!(rule_group.description().unwrap(), "test");
    assert!(rule_group.custom_response_bodies().is_some());
    assert_eq!(rule_group.rules().len(), 1);
    assert_eq!(rule_group.rules()[0].name(), "test-rule");

    // Wrong lock token
    let err = client
        .update_rule_group()
        .name("test-group")
        .scope(Scope::Regional)
        .id(id)
        .lock_token("123")
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(err_str.contains("OptimisticLock"));

    // Wrong name
    let err = client
        .update_rule_group()
        .name("bad-group")
        .scope(Scope::Regional)
        .id(id)
        .lock_token(lock_token)
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(err_str.contains("Nonexistent"));
}

// Ported from moto: test_wafv2_rules.py::test_delete_rule_group
#[tokio::test]
async fn test_moto_delete_rule_group() {
    let client = make_client().await;

    let group = client
        .create_rule_group()
        .capacity(100)
        .name("test-group-1")
        .scope(Scope::Regional)
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    let summary = group.summary().unwrap();
    let id = summary.id().unwrap();
    let lock_token = summary.lock_token().unwrap();

    // Wrong scope should fail
    let err = client
        .delete_rule_group()
        .name("test-group-1")
        .scope(Scope::Cloudfront)
        .id(id)
        .lock_token(lock_token)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(err_str.contains("Nonexistent"));

    // Wrong lock token should fail
    let err = client
        .delete_rule_group()
        .name("test-group-1")
        .scope(Scope::Regional)
        .id(id)
        .lock_token("1234567890")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(err_str.contains("OptimisticLock"));

    // Correct delete
    assert_eq!(
        client
            .list_rule_groups()
            .scope(Scope::Regional)
            .send()
            .await
            .unwrap()
            .rule_groups()
            .len(),
        1
    );

    client
        .delete_rule_group()
        .name("test-group-1")
        .scope(Scope::Regional)
        .id(id)
        .lock_token(lock_token)
        .send()
        .await
        .unwrap();

    assert_eq!(
        client
            .list_rule_groups()
            .scope(Scope::Regional)
            .send()
            .await
            .unwrap()
            .rule_groups()
            .len(),
        0
    );
}

// Ported from moto: test_wafv2_rules.py::test_list_rule_groups (scope filtering)
#[tokio::test]
async fn test_moto_list_rule_groups_scope_filtering() {
    let client = make_client().await;

    assert_eq!(
        client
            .list_rule_groups()
            .scope(Scope::Regional)
            .send()
            .await
            .unwrap()
            .rule_groups()
            .len(),
        0
    );

    client
        .create_rule_group()
        .capacity(100)
        .name("test-group-1")
        .scope(Scope::Regional)
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    for idx in 2..10 {
        client
            .create_rule_group()
            .capacity(100)
            .name(format!("test-group-{idx}"))
            .scope(Scope::Cloudfront)
            .visibility_config(vis_config())
            .send()
            .await
            .unwrap();
    }

    let reg_groups = client
        .list_rule_groups()
        .scope(Scope::Regional)
        .send()
        .await
        .unwrap();
    assert_eq!(reg_groups.rule_groups().len(), 1);

    let cf_groups = client
        .list_rule_groups()
        .scope(Scope::Cloudfront)
        .send()
        .await
        .unwrap();
    assert_eq!(cf_groups.rule_groups().len(), 8);
}

// ============================================================================
// Ported from moto: test_wafv2_logging_configuration.py
// ============================================================================

// Ported from moto: test_wafv2_logging_configuration.py::test_logging_configuration_crud
#[tokio::test]
async fn test_moto_logging_configuration_crud() {
    let client = make_client().await;

    let create_web_acl_resp = client
        .create_web_acl()
        .name("TestWebACL")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(
            VisibilityConfig::builder()
                .sampled_requests_enabled(true)
                .cloud_watch_metrics_enabled(true)
                .metric_name("TestWebACLMetric")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let web_acl_arn = create_web_acl_resp
        .summary()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    use aws_sdk_wafv2::types::LoggingConfiguration;
    let create_resp = client
        .put_logging_configuration()
        .logging_configuration(
            LoggingConfiguration::builder()
                .resource_arn(&web_acl_arn)
                .log_destination_configs(
                    "arn:aws:logs:us-east-1:123456789012:log-group:aws-waf-logs-test",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let lc = create_resp.logging_configuration().unwrap();
    assert_eq!(lc.resource_arn(), &web_acl_arn);

    let get_resp = client
        .get_logging_configuration()
        .resource_arn(&web_acl_arn)
        .send()
        .await
        .unwrap();
    assert!(get_resp.logging_configuration().is_some());

    client
        .delete_logging_configuration()
        .resource_arn(&web_acl_arn)
        .send()
        .await
        .unwrap();

    let err = client
        .get_logging_configuration()
        .resource_arn(&web_acl_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(err_str.contains("Nonexistent"));
}

// ============================================================================
// Ported from moto: test_wafv2_integration.py
// ============================================================================

// Ported from moto: test_wafv2_integration.py::test_associate_with_unknown_resource
#[tokio::test]
async fn test_moto_associate_with_unknown_resource() {
    let client = make_client().await;

    let wacl_arn = client
        .create_web_acl()
        .name("John")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap()
        .summary()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    // We can associate with any resource ARN
    let random_arn = "arn:aws:cognito-idp:us-east-1:123456789012:userpool/us-east-1_test";
    client
        .associate_web_acl()
        .web_acl_arn(&wacl_arn)
        .resource_arn(random_arn)
        .send()
        .await
        .unwrap();

    let wacl = client
        .get_web_acl_for_resource()
        .resource_arn(random_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(wacl.web_acl().unwrap().name(), "John");

    // Non-existent WebACL ARN should fail
    let err = client
        .associate_web_acl()
        .web_acl_arn(format!("{wacl_arn}2"))
        .resource_arn(random_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Nonexistent"),
        "Expected Nonexistent error, got: {err_str}"
    );
}

// Ported from moto: test_wafv2_integration.py::test_disassociate_unknown_resource
#[tokio::test]
async fn test_moto_disassociate_unknown_resource() {
    let client = make_client().await;

    client
        .create_web_acl()
        .name("John")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap();

    // Nothing happens when disassociating unknown resource
    client
        .disassociate_web_acl()
        .resource_arn("unknownarnwithlength20")
        .send()
        .await
        .unwrap();
}

// ============================================================================
// Ported from moto: test_wafv2_tags.py
// ============================================================================

// Ported from moto: test_wafv2_tags.py::test_list_tags_for_resource__none_supplied
#[tokio::test]
async fn test_moto_list_tags_none_supplied() {
    let client = make_client().await;

    let arn = client
        .create_web_acl()
        .name("John")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .unwrap()
        .summary()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    let tag_info = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap()
        .tag_info_for_resource()
        .unwrap()
        .clone();

    assert_eq!(tag_info.tag_list().len(), 0);
}

// Ported from moto: test_wafv2_tags.py::test_list_tags_for_resource
#[tokio::test]
async fn test_moto_list_tags_for_resource_with_tags() {
    let client = make_client().await;

    let mut builder = client
        .create_web_acl()
        .name("test")
        .scope(Scope::Cloudfront)
        .default_action(allow_action())
        .visibility_config(vis_config());

    for idx in 0..10 {
        builder = builder.tags(
            Tag::builder()
                .key(format!("k{idx}"))
                .value("v1")
                .build()
                .unwrap(),
        );
    }

    let arn = builder
        .send()
        .await
        .unwrap()
        .summary()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    let tag_info = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap()
        .tag_info_for_resource()
        .unwrap()
        .clone();

    assert_eq!(tag_info.tag_list().len(), 10);
}

// Ported from moto: test_wafv2_tags.py::test_tag_resource
#[tokio::test]
async fn test_moto_tag_resource() {
    let client = make_client().await;

    let arn = client
        .create_web_acl()
        .name("test")
        .scope(Scope::Cloudfront)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .tags(Tag::builder().key("k1").value("v1").build().unwrap())
        .send()
        .await
        .unwrap()
        .summary()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("k2").value("v2").build().unwrap())
        .send()
        .await
        .unwrap();

    let tag_info = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap()
        .tag_info_for_resource()
        .unwrap()
        .clone();

    assert_eq!(tag_info.tag_list().len(), 2);
    assert_eq!(tag_info.tag_list()[0].key(), "k1");
    assert_eq!(tag_info.tag_list()[0].value(), "v1");
    assert_eq!(tag_info.tag_list()[1].key(), "k2");
    assert_eq!(tag_info.tag_list()[1].value(), "v2");
}

// Ported from moto: test_wafv2_tags.py::test_untag_resource
#[tokio::test]
async fn test_moto_untag_resource() {
    let client = make_client().await;

    let arn = client
        .create_web_acl()
        .name("test")
        .scope(Scope::Cloudfront)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .tags(Tag::builder().key("k1").value("v1").build().unwrap())
        .tags(Tag::builder().key("k2").value("v2").build().unwrap())
        .send()
        .await
        .unwrap()
        .summary()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("k1")
        .send()
        .await
        .unwrap();

    let tag_info = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap()
        .tag_info_for_resource()
        .unwrap()
        .clone();

    assert_eq!(tag_info.tag_list().len(), 1);
    assert_eq!(tag_info.tag_list()[0].key(), "k2");
    assert_eq!(tag_info.tag_list()[0].value(), "v2");
}

// ── CheckCapacity tests ──

/// A single GeoMatch rule costs 1 WCU.
#[tokio::test]
async fn test_check_capacity_geo_match() {
    let client = make_client().await;

    let rule = Rule::builder()
        .name("geo-rule")
        .priority(0)
        .statement(
            Statement::builder()
                .geo_match_statement(
                    aws_sdk_wafv2::types::GeoMatchStatement::builder()
                        .country_codes(aws_sdk_wafv2::types::CountryCode::Us)
                        .build(),
                )
                .build(),
        )
        .action(
            RuleAction::builder()
                .block(BlockAction::builder().build())
                .build(),
        )
        .visibility_config(vis_config())
        .build()
        .unwrap();

    let resp = client
        .check_capacity()
        .scope(Scope::Regional)
        .rules(rule)
        .send()
        .await
        .expect("check_capacity should succeed");

    assert_eq!(resp.capacity(), 1);
}

/// A non-trivial ruleset: GeoMatch (1 WCU) + SqliMatch on JSON body with two
/// text transformations (20 × 2 = 40 base + 2 × 10 = 60 WCU) = 61 WCU total.
/// This asserts that CheckCapacity returns > 1 for complex rules and that the
/// JSON body doubling and transform costs are included.
#[tokio::test]
async fn test_check_capacity_complex_ruleset_returns_correct_wcu() {
    let client = make_client().await;

    let rule_geo = Rule::builder()
        .name("geo-rule")
        .priority(0)
        .statement(
            Statement::builder()
                .geo_match_statement(
                    aws_sdk_wafv2::types::GeoMatchStatement::builder()
                        .country_codes(aws_sdk_wafv2::types::CountryCode::Us)
                        .build(),
                )
                .build(),
        )
        .action(
            RuleAction::builder()
                .count(CountAction::builder().build())
                .build(),
        )
        .visibility_config(vis_config())
        .build()
        .unwrap();

    let rule_sqli = Rule::builder()
        .name("sqli-json-rule")
        .priority(1)
        .statement(
            Statement::builder()
                .sqli_match_statement(
                    aws_sdk_wafv2::types::SqliMatchStatement::builder()
                        .field_to_match(
                            aws_sdk_wafv2::types::FieldToMatch::builder()
                                .json_body(
                                    aws_sdk_wafv2::types::JsonBody::builder()
                                        .match_pattern(
                                            aws_sdk_wafv2::types::JsonMatchPattern::builder()
                                                .all(aws_sdk_wafv2::types::All::builder().build())
                                                .build(),
                                        )
                                        .match_scope(aws_sdk_wafv2::types::JsonMatchScope::All)
                                        .build()
                                        .unwrap(),
                                )
                                .build(),
                        )
                        .text_transformations(
                            aws_sdk_wafv2::types::TextTransformation::builder()
                                .priority(0)
                                .r#type(aws_sdk_wafv2::types::TextTransformationType::UrlDecode)
                                .build()
                                .unwrap(),
                        )
                        .text_transformations(
                            aws_sdk_wafv2::types::TextTransformation::builder()
                                .priority(1)
                                .r#type(
                                    aws_sdk_wafv2::types::TextTransformationType::HtmlEntityDecode,
                                )
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .action(
            RuleAction::builder()
                .block(BlockAction::builder().build())
                .build(),
        )
        .visibility_config(vis_config())
        .build()
        .unwrap();

    let resp = client
        .check_capacity()
        .scope(Scope::Regional)
        .rules(rule_geo)
        .rules(rule_sqli)
        .send()
        .await
        .expect("check_capacity should succeed");

    // GeoMatch: 1 WCU
    // SqliMatch on JsonBody with 2 transforms: 20 × 2 (JSON body doubles base)
    //   + 2 × 10 (transforms) = 60 WCU
    // Total: 1 + 60 = 61 WCU
    assert_eq!(resp.capacity(), 61);
    assert!(
        resp.capacity() > 1,
        "expected WCU > 1 for complex ruleset, got {}",
        resp.capacity()
    );
}
