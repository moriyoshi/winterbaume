use aws_sdk_elasticloadbalancingv2::config::BehaviorVersion;
use aws_sdk_elasticloadbalancingv2::types::{
    Action, ActionTypeEnum, ProtocolEnum, RuleCondition, RulePriorityPair, Tag, TargetDescription,
};
use winterbaume_core::MockAws;
use winterbaume_elasticloadbalancingv2::ElasticLoadBalancingV2Service;

async fn make_elbv2_client() -> aws_sdk_elasticloadbalancingv2::Client {
    let mock = MockAws::builder()
        .with_service(ElasticLoadBalancingV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticloadbalancingv2::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    aws_sdk_elasticloadbalancingv2::Client::new(&config)
}

// --- Load Balancer tests ---

#[tokio::test]
async fn test_create_and_describe_load_balancer() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_load_balancer()
        .name("test-alb")
        .send()
        .await
        .expect("create_load_balancer should succeed");

    let lbs = resp.load_balancers();
    assert_eq!(lbs.len(), 1);
    assert_eq!(lbs[0].load_balancer_name(), Some("test-alb"));
    assert!(lbs[0].load_balancer_arn().is_some());
    assert!(lbs[0].dns_name().is_some());

    let desc_resp = client
        .describe_load_balancers()
        .send()
        .await
        .expect("describe_load_balancers should succeed");

    assert_eq!(desc_resp.load_balancers().len(), 1);
    assert_eq!(
        desc_resp.load_balancers()[0].load_balancer_name(),
        Some("test-alb")
    );
}

#[tokio::test]
async fn test_create_duplicate_load_balancer_fails() {
    let client = make_elbv2_client().await;

    client
        .create_load_balancer()
        .name("dup-alb")
        .send()
        .await
        .unwrap();

    let result = client.create_load_balancer().name("dup-alb").send().await;
    assert!(result.is_err(), "duplicate ALB should fail");
}

#[tokio::test]
async fn test_delete_load_balancer() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_load_balancer()
        .name("del-alb")
        .send()
        .await
        .unwrap();

    let arn = resp.load_balancers()[0]
        .load_balancer_arn()
        .expect("should have ARN");

    client
        .delete_load_balancer()
        .load_balancer_arn(arn)
        .send()
        .await
        .expect("delete should succeed");

    let desc_resp = client.describe_load_balancers().send().await.unwrap();
    assert!(desc_resp.load_balancers().is_empty());
}

// --- Target Group tests ---

#[tokio::test]
async fn test_create_and_describe_target_group() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_target_group()
        .name("test-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .expect("create_target_group should succeed");

    let tgs = resp.target_groups();
    assert_eq!(tgs.len(), 1);
    assert_eq!(tgs[0].target_group_name(), Some("test-tg"));
    assert_eq!(tgs[0].port(), Some(8080));

    let desc_resp = client
        .describe_target_groups()
        .send()
        .await
        .expect("describe_target_groups should succeed");

    assert_eq!(desc_resp.target_groups().len(), 1);
}

#[tokio::test]
async fn test_delete_target_group() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_target_group()
        .name("del-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();

    let arn = resp.target_groups()[0]
        .target_group_arn()
        .expect("should have ARN");

    client
        .delete_target_group()
        .target_group_arn(arn)
        .send()
        .await
        .expect("delete_target_group should succeed");

    let desc_resp = client.describe_target_groups().send().await.unwrap();
    assert!(desc_resp.target_groups().is_empty());
}

#[tokio::test]
async fn test_create_duplicate_target_group_fails() {
    let client = make_elbv2_client().await;

    client
        .create_target_group()
        .name("dup-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();

    let result = client
        .create_target_group()
        .name("dup-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await;
    assert!(result.is_err(), "duplicate target group should fail");
}

#[tokio::test]
async fn test_modify_target_group() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_target_group()
        .name("mod-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();

    let arn = resp.target_groups()[0].target_group_arn().unwrap();

    let mod_resp = client
        .modify_target_group()
        .target_group_arn(arn)
        .health_check_path("/healthz")
        .send()
        .await
        .expect("modify_target_group should succeed");

    let tgs = mod_resp.target_groups();
    assert_eq!(tgs.len(), 1);
    assert_eq!(tgs[0].health_check_path(), Some("/healthz"));
}

#[tokio::test]
async fn test_modify_target_group_attributes() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_target_group()
        .name("attr-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();

    let arn = resp.target_groups()[0].target_group_arn().unwrap();

    let attr = aws_sdk_elasticloadbalancingv2::types::TargetGroupAttribute::builder()
        .key("deregistration_delay.timeout_seconds")
        .value("30")
        .build();

    let mod_resp = client
        .modify_target_group_attributes()
        .target_group_arn(arn)
        .attributes(attr)
        .send()
        .await
        .expect("modify_target_group_attributes should succeed");

    let attrs = mod_resp.attributes();
    assert!(!attrs.is_empty());
    assert!(attrs.iter().any(
        |a| a.key() == Some("deregistration_delay.timeout_seconds") && a.value() == Some("30")
    ));
}

// --- Listener tests ---

#[tokio::test]
async fn test_create_listener_and_describe() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("listener-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("listener-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action)
        .send()
        .await
        .expect("create_listener should succeed");

    let listeners = listener_resp.listeners();
    assert_eq!(listeners.len(), 1);
    assert_eq!(listeners[0].port(), Some(80));

    let desc_resp = client
        .describe_listeners()
        .load_balancer_arn(lb_arn)
        .send()
        .await
        .expect("describe_listeners should succeed");

    assert_eq!(desc_resp.listeners().len(), 1);
}

#[tokio::test]
async fn test_delete_listener() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("del-listener-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("del-listener-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action)
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    client
        .delete_listener()
        .listener_arn(listener_arn)
        .send()
        .await
        .expect("delete_listener should succeed");

    let desc_resp = client
        .describe_listeners()
        .load_balancer_arn(lb_arn)
        .send()
        .await
        .unwrap();
    assert!(desc_resp.listeners().is_empty());
}

#[tokio::test]
async fn test_modify_listener() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("mod-listener-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("mod-listener-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action)
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    let mod_resp = client
        .modify_listener()
        .listener_arn(listener_arn)
        .port(8080)
        .send()
        .await
        .expect("modify_listener should succeed");

    let listeners = mod_resp.listeners();
    assert_eq!(listeners.len(), 1);
    assert_eq!(listeners[0].port(), Some(8080));
}

#[tokio::test]
async fn test_describe_listener_attributes() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("listattr-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("listattr-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(443)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Https)
        .default_actions(action)
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    // Describe listener attributes (initially empty)
    let desc_resp = client
        .describe_listener_attributes()
        .listener_arn(listener_arn)
        .send()
        .await
        .expect("describe_listener_attributes should succeed");
    // Initially no attributes
    assert!(desc_resp.attributes().is_empty());
}

#[tokio::test]
async fn test_modify_listener_attributes() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("modlistattr-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("modlistattr-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(443)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Https)
        .default_actions(action)
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    let attr = aws_sdk_elasticloadbalancingv2::types::ListenerAttribute::builder()
        .key("routing.http.xff_header_processing.mode")
        .value("append")
        .build();

    let mod_resp = client
        .modify_listener_attributes()
        .listener_arn(listener_arn)
        .attributes(attr)
        .send()
        .await
        .expect("modify_listener_attributes should succeed");

    let attrs = mod_resp.attributes();
    assert!(!attrs.is_empty());
    assert!(attrs.iter().any(
        |a| a.key() == Some("routing.http.xff_header_processing.mode")
            && a.value() == Some("append")
    ));
}

// --- Listener Certificate tests ---

#[tokio::test]
async fn test_add_describe_remove_listener_certificates() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("cert-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("cert-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(443)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Https)
        .default_actions(action)
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    // Add certificate
    let cert = aws_sdk_elasticloadbalancingv2::types::Certificate::builder()
        .certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/abc123")
        .build();

    client
        .add_listener_certificates()
        .listener_arn(listener_arn)
        .certificates(cert)
        .send()
        .await
        .expect("add_listener_certificates should succeed");

    // Describe certificates
    let desc_resp = client
        .describe_listener_certificates()
        .listener_arn(listener_arn)
        .send()
        .await
        .expect("describe_listener_certificates should succeed");
    assert_eq!(desc_resp.certificates().len(), 1);
    assert_eq!(
        desc_resp.certificates()[0].certificate_arn(),
        Some("arn:aws:acm:us-east-1:123456789012:certificate/abc123")
    );

    // Remove certificate
    let cert_to_remove = aws_sdk_elasticloadbalancingv2::types::Certificate::builder()
        .certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/abc123")
        .build();

    client
        .remove_listener_certificates()
        .listener_arn(listener_arn)
        .certificates(cert_to_remove)
        .send()
        .await
        .expect("remove_listener_certificates should succeed");

    // Verify removed
    let desc_resp2 = client
        .describe_listener_certificates()
        .listener_arn(listener_arn)
        .send()
        .await
        .unwrap();
    assert!(desc_resp2.certificates().is_empty());
}

// --- Rule tests ---

#[tokio::test]
async fn test_create_describe_delete_rule() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("rule-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("rule-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    // Create rule
    let condition = aws_sdk_elasticloadbalancingv2::types::RuleCondition::builder()
        .field("path-pattern")
        .values("/api/*")
        .build();

    let rule_action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let rule_resp = client
        .create_rule()
        .listener_arn(listener_arn)
        .priority(10)
        .conditions(condition)
        .actions(rule_action)
        .send()
        .await
        .expect("create_rule should succeed");

    let rules = rule_resp.rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].priority(), Some("10"));
    let rule_arn = rules[0].rule_arn().unwrap();

    // Describe rules
    let desc_resp = client
        .describe_rules()
        .listener_arn(listener_arn)
        .send()
        .await
        .expect("describe_rules should succeed");
    assert_eq!(desc_resp.rules().len(), 1);

    // Delete rule
    client
        .delete_rule()
        .rule_arn(rule_arn)
        .send()
        .await
        .expect("delete_rule should succeed");

    let desc_resp2 = client
        .describe_rules()
        .listener_arn(listener_arn)
        .send()
        .await
        .unwrap();
    assert!(desc_resp2.rules().is_empty());
}

#[tokio::test]
async fn test_modify_rule() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("modrule-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("modrule-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    let condition = aws_sdk_elasticloadbalancingv2::types::RuleCondition::builder()
        .field("path-pattern")
        .values("/old/*")
        .build();

    let rule_resp = client
        .create_rule()
        .listener_arn(listener_arn)
        .priority(10)
        .conditions(condition)
        .actions(action)
        .send()
        .await
        .unwrap();
    let rule_arn = rule_resp.rules()[0].rule_arn().unwrap();

    // Modify rule conditions
    let new_condition = aws_sdk_elasticloadbalancingv2::types::RuleCondition::builder()
        .field("path-pattern")
        .values("/new/*")
        .build();

    let mod_resp = client
        .modify_rule()
        .rule_arn(rule_arn)
        .conditions(new_condition)
        .send()
        .await
        .expect("modify_rule should succeed");

    let rules = mod_resp.rules();
    assert_eq!(rules.len(), 1);
    let conditions = rules[0].conditions();
    assert!(!conditions.is_empty());
    assert!(conditions[0].values().iter().any(|v| v == "/new/*"));
}

#[tokio::test]
async fn test_set_rule_priorities() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("setprio-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("setprio-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    let condition = aws_sdk_elasticloadbalancingv2::types::RuleCondition::builder()
        .field("path-pattern")
        .values("/test/*")
        .build();

    let rule_resp = client
        .create_rule()
        .listener_arn(listener_arn)
        .priority(10)
        .conditions(condition)
        .actions(action)
        .send()
        .await
        .unwrap();
    let rule_arn = rule_resp.rules()[0].rule_arn().unwrap();

    let rule_prio = aws_sdk_elasticloadbalancingv2::types::RulePriorityPair::builder()
        .rule_arn(rule_arn)
        .priority(20)
        .build();

    let set_resp = client
        .set_rule_priorities()
        .rule_priorities(rule_prio)
        .send()
        .await
        .expect("set_rule_priorities should succeed");

    let rules = set_resp.rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].priority(), Some("20"));
}

// --- Target registration tests ---

#[tokio::test]
async fn test_register_deregister_targets() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("targets-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    // Register targets
    let target = aws_sdk_elasticloadbalancingv2::types::TargetDescription::builder()
        .id("i-1234567890abcdef0")
        .port(80)
        .build();

    client
        .register_targets()
        .target_group_arn(tg_arn)
        .targets(target)
        .send()
        .await
        .expect("register_targets should succeed");

    // Describe target health
    let health_resp = client
        .describe_target_health()
        .target_group_arn(tg_arn)
        .send()
        .await
        .expect("describe_target_health should succeed");
    assert_eq!(health_resp.target_health_descriptions().len(), 1);
    let target_desc = &health_resp.target_health_descriptions()[0];
    assert_eq!(
        target_desc.target().unwrap().id(),
        Some("i-1234567890abcdef0")
    );

    // Deregister targets
    let target_deregister = aws_sdk_elasticloadbalancingv2::types::TargetDescription::builder()
        .id("i-1234567890abcdef0")
        .port(80)
        .build();

    client
        .deregister_targets()
        .target_group_arn(tg_arn)
        .targets(target_deregister)
        .send()
        .await
        .expect("deregister_targets should succeed");

    let health_resp2 = client
        .describe_target_health()
        .target_group_arn(tg_arn)
        .send()
        .await
        .unwrap();
    assert!(health_resp2.target_health_descriptions().is_empty());
}

// --- Tag tests ---

#[tokio::test]
async fn test_add_describe_remove_tags() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("tags-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    // Add tags
    let tag = aws_sdk_elasticloadbalancingv2::types::Tag::builder()
        .key("env")
        .value("test")
        .build();

    client
        .add_tags()
        .resource_arns(&lb_arn)
        .tags(tag)
        .send()
        .await
        .expect("add_tags should succeed");

    // Describe tags
    let desc_resp = client
        .describe_tags()
        .resource_arns(&lb_arn)
        .send()
        .await
        .expect("describe_tags should succeed");

    let tag_descs = desc_resp.tag_descriptions();
    assert_eq!(tag_descs.len(), 1);
    assert_eq!(tag_descs[0].resource_arn(), Some(lb_arn.as_str()));
    let tags = tag_descs[0].tags();
    assert!(
        tags.iter()
            .any(|t| t.key() == Some("env") && t.value() == Some("test"))
    );

    // Remove tags
    client
        .remove_tags()
        .resource_arns(&lb_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("remove_tags should succeed");

    // Verify removed
    let desc_resp2 = client
        .describe_tags()
        .resource_arns(&lb_arn)
        .send()
        .await
        .unwrap();
    let tag_descs2 = desc_resp2.tag_descriptions();
    assert_eq!(tag_descs2.len(), 1);
    assert!(tag_descs2[0].tags().is_empty());
}

// --- Load Balancer Attributes tests ---

#[tokio::test]
async fn test_describe_modify_load_balancer_attributes() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("attrs-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    // Describe (initially empty)
    let desc_resp = client
        .describe_load_balancer_attributes()
        .load_balancer_arn(lb_arn)
        .send()
        .await
        .expect("describe_load_balancer_attributes should succeed");
    assert!(desc_resp.attributes().is_empty());

    // Modify
    let attr = aws_sdk_elasticloadbalancingv2::types::LoadBalancerAttribute::builder()
        .key("idle_timeout.timeout_seconds")
        .value("120")
        .build();

    let mod_resp = client
        .modify_load_balancer_attributes()
        .load_balancer_arn(lb_arn)
        .attributes(attr)
        .send()
        .await
        .expect("modify_load_balancer_attributes should succeed");

    let attrs = mod_resp.attributes();
    assert!(!attrs.is_empty());
    assert!(
        attrs
            .iter()
            .any(|a| a.key() == Some("idle_timeout.timeout_seconds") && a.value() == Some("120"))
    );
}

// --- Set operations tests ---

#[tokio::test]
async fn test_set_ip_address_type() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("iptype-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let resp = client
        .set_ip_address_type()
        .load_balancer_arn(lb_arn)
        .ip_address_type(aws_sdk_elasticloadbalancingv2::types::IpAddressType::Dualstack)
        .send()
        .await
        .expect("set_ip_address_type should succeed");

    assert_eq!(
        resp.ip_address_type(),
        Some(&aws_sdk_elasticloadbalancingv2::types::IpAddressType::Dualstack)
    );
}

#[tokio::test]
async fn test_set_security_groups() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("sg-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let resp = client
        .set_security_groups()
        .load_balancer_arn(lb_arn)
        .security_groups("sg-aaaa1111")
        .security_groups("sg-bbbb2222")
        .send()
        .await
        .expect("set_security_groups should succeed");

    let sg_ids = resp.security_group_ids();
    assert_eq!(sg_ids.len(), 2);
    assert!(sg_ids.contains(&"sg-aaaa1111".to_string()));
    assert!(sg_ids.contains(&"sg-bbbb2222".to_string()));
}

#[tokio::test]
async fn test_set_subnets() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("subnet-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let resp = client
        .set_subnets()
        .load_balancer_arn(lb_arn)
        .subnets("subnet-aaa111")
        .subnets("subnet-bbb222")
        .send()
        .await
        .expect("set_subnets should succeed");

    let azs = resp.availability_zones();
    assert_eq!(azs.len(), 2);
}

// --- Lifecycle tests ---

#[tokio::test]
async fn test_full_lifecycle() {
    let client = make_elbv2_client().await;

    // Create LB
    let lb_resp = client
        .create_load_balancer()
        .name("lifecycle-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    // Create TG
    let tg_resp = client
        .create_target_group()
        .name("lifecycle-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0]
        .target_group_arn()
        .unwrap()
        .to_string();

    // Register target
    let target = aws_sdk_elasticloadbalancingv2::types::TargetDescription::builder()
        .id("i-abc123")
        .port(80)
        .build();

    client
        .register_targets()
        .target_group_arn(&tg_arn)
        .targets(target)
        .send()
        .await
        .unwrap();

    // Create Listener
    let action = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(aws_sdk_elasticloadbalancingv2::types::ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(&lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0]
        .listener_arn()
        .unwrap()
        .to_string();

    // Create Rule
    let condition = aws_sdk_elasticloadbalancingv2::types::RuleCondition::builder()
        .field("path-pattern")
        .values("/api/*")
        .build();

    let rule_resp = client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(10)
        .conditions(condition)
        .actions(action)
        .send()
        .await
        .unwrap();
    let rule_arn = rule_resp.rules()[0].rule_arn().unwrap().to_string();

    // Add tags
    let tag = aws_sdk_elasticloadbalancingv2::types::Tag::builder()
        .key("app")
        .value("lifecycle")
        .build();

    client
        .add_tags()
        .resource_arns(&lb_arn)
        .tags(tag)
        .send()
        .await
        .unwrap();

    // Verify everything exists
    assert_eq!(
        client
            .describe_load_balancers()
            .send()
            .await
            .unwrap()
            .load_balancers()
            .len(),
        1
    );
    assert_eq!(
        client
            .describe_listeners()
            .load_balancer_arn(&lb_arn)
            .send()
            .await
            .unwrap()
            .listeners()
            .len(),
        1
    );
    assert_eq!(
        client
            .describe_rules()
            .listener_arn(&listener_arn)
            .send()
            .await
            .unwrap()
            .rules()
            .len(),
        1
    );

    // Delete rule
    client
        .delete_rule()
        .rule_arn(&rule_arn)
        .send()
        .await
        .unwrap();

    // Delete listener
    client
        .delete_listener()
        .listener_arn(&listener_arn)
        .send()
        .await
        .unwrap();

    // Deregister target
    let target_dereg = aws_sdk_elasticloadbalancingv2::types::TargetDescription::builder()
        .id("i-abc123")
        .port(80)
        .build();

    client
        .deregister_targets()
        .target_group_arn(&tg_arn)
        .targets(target_dereg)
        .send()
        .await
        .unwrap();

    // Delete TG
    client
        .delete_target_group()
        .target_group_arn(&tg_arn)
        .send()
        .await
        .unwrap();

    // Delete LB
    client
        .delete_load_balancer()
        .load_balancer_arn(&lb_arn)
        .send()
        .await
        .unwrap();

    // Verify everything is gone
    assert!(
        client
            .describe_load_balancers()
            .send()
            .await
            .unwrap()
            .load_balancers()
            .is_empty()
    );
    assert!(
        client
            .describe_target_groups()
            .send()
            .await
            .unwrap()
            .target_groups()
            .is_empty()
    );
}

// ============================================================================
// Ported from moto: test_elbv2.py, test_elbv2_target_groups.py
// ============================================================================

// Helper: create LB + TG + listener and return (client, lb_arn, tg_arn, listener_arn)
async fn setup_lb_tg_listener() -> (
    aws_sdk_elasticloadbalancingv2::Client,
    String,
    String,
    String,
) {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("setup-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    let tg_resp = client
        .create_target_group()
        .name("setup-tg")
        .protocol(ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0]
        .target_group_arn()
        .unwrap()
        .to_string();

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(&lb_arn)
        .port(80)
        .protocol(ProtocolEnum::Http)
        .default_actions(action)
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0]
        .listener_arn()
        .unwrap()
        .to_string();

    (client, lb_arn, tg_arn, listener_arn)
}

// Ported from moto: test_elbv2.py::test_describe_load_balancers
#[tokio::test]
async fn test_describe_load_balancers_with_filters() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_load_balancer()
        .name("filter-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    // Describe all
    let desc = client.describe_load_balancers().send().await.unwrap();
    assert_eq!(desc.load_balancers().len(), 1);
    assert_eq!(
        desc.load_balancers()[0].load_balancer_name(),
        Some("filter-alb")
    );

    // Describe by ARN
    let desc_by_arn = client
        .describe_load_balancers()
        .load_balancer_arns(&lb_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc_by_arn.load_balancers()[0].load_balancer_name(),
        Some("filter-alb")
    );

    // Describe by name
    let desc_by_name = client
        .describe_load_balancers()
        .names("filter-alb")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc_by_name.load_balancers()[0].load_balancer_name(),
        Some("filter-alb")
    );

    // Describe with invalid ARN should fail
    let err = client
        .describe_load_balancers()
        .load_balancer_arns("not-a/real/arn")
        .send()
        .await;
    assert!(err.is_err(), "invalid ARN should fail");

    // Describe with invalid name should fail
    let err = client.describe_load_balancers().names("nope").send().await;
    assert!(err.is_err(), "invalid name should fail");
}

// Ported from moto: test_elbv2.py::test_describe_listeners (validation)
#[tokio::test]
async fn test_describe_listeners_requires_arn() {
    let client = make_elbv2_client().await;

    // Calling describe_listeners with no parameters should fail
    let err = client.describe_listeners().send().await;
    assert!(err.is_err(), "describe_listeners with no args should fail");
    let err_str = format!("{:?}", err.unwrap_err());
    assert!(
        err_str.contains("ValidationError"),
        "Expected ValidationError, got: {err_str}"
    );
}

// Ported from moto: test_elbv2.py::test_describe_listeners (by ListenerArns)
#[tokio::test]
async fn test_describe_listeners_by_listener_arns() {
    let (client, lb_arn, tg_arn, _) = setup_lb_tg_listener().await;

    // Create a second listener (HTTPS on 443)
    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn)
        .build();
    let resp2 = client
        .create_listener()
        .load_balancer_arn(&lb_arn)
        .port(443)
        .protocol(ProtocolEnum::Https)
        .default_actions(action)
        .send()
        .await
        .unwrap();
    let https_listener_arn = resp2.listeners()[0].listener_arn().unwrap().to_string();

    // Describe all by LB ARN
    let desc = client
        .describe_listeners()
        .load_balancer_arn(&lb_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.listeners().len(), 2);

    // Describe by specific listener ARN
    let desc_one = client
        .describe_listeners()
        .listener_arns(&https_listener_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_one.listeners().len(), 1);
    assert_eq!(desc_one.listeners()[0].port(), Some(443));
}

// Ported from moto: test_elbv2.py::test_create_rule_priority_in_use
#[tokio::test]
async fn test_create_rule_priority_in_use() {
    let (client, _lb_arn, tg_arn, listener_arn) = setup_lb_tg_listener().await;

    let condition = RuleCondition::builder()
        .field("path-pattern")
        .values("/first/*")
        .build();
    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn)
        .build();

    // Create rule at priority 100
    client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(100)
        .conditions(condition.clone())
        .actions(action.clone())
        .send()
        .await
        .unwrap();

    // Try to create another rule at same priority
    let err = client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(100)
        .conditions(condition)
        .actions(action)
        .send()
        .await;
    assert!(err.is_err(), "duplicate priority should fail");
    let err_str = format!("{:?}", err.unwrap_err());
    assert!(
        err_str.contains("PriorityInUse"),
        "Expected PriorityInUse, got: {err_str}"
    );
}

// Ported from moto: test_elbv2.py::test_modify_rule_conditions
#[tokio::test]
async fn test_modify_rule_conditions_and_actions_independently() {
    let (client, _lb_arn, tg_arn, listener_arn) = setup_lb_tg_listener().await;

    // Create a rule with no conditions/actions (empty)
    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn)
        .build();

    let condition = RuleCondition::builder()
        .field("path-pattern")
        .values("/test/*")
        .build();

    let rule_resp = client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(100)
        .conditions(condition.clone())
        .actions(action.clone())
        .send()
        .await
        .unwrap();
    let rule_arn = rule_resp.rules()[0].rule_arn().unwrap().to_string();

    // Modify rule: update conditions only (new condition)
    let new_condition = RuleCondition::builder()
        .field("host-header")
        .values("example.com")
        .build();

    let mod_resp = client
        .modify_rule()
        .rule_arn(&rule_arn)
        .conditions(new_condition)
        .send()
        .await
        .unwrap();
    let rule = &mod_resp.rules()[0];
    // Conditions should be updated
    assert_eq!(rule.conditions().len(), 1);
    assert!(
        rule.conditions()[0]
            .values()
            .iter()
            .any(|v| v == "example.com")
    );
    // Actions should remain unchanged
    assert_eq!(rule.actions().len(), 1);

    // Modify rule: update actions only (add second action)
    let action2 = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn)
        .build();
    let mod_resp2 = client
        .modify_rule()
        .rule_arn(&rule_arn)
        .actions(action.clone())
        .actions(action2)
        .send()
        .await
        .unwrap();
    let rule2 = &mod_resp2.rules()[0];
    assert_eq!(rule2.actions().len(), 2);
    // Conditions should remain from previous modification
    assert_eq!(rule2.conditions().len(), 1);
}

// Ported from moto: test_elbv2.py::test_register_targets (multiple targets, deregister one)
#[tokio::test]
async fn test_register_multiple_targets_deregister_one() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("multi-targets-tg")
        .protocol(ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    // No targets yet
    let health = client
        .describe_target_health()
        .target_group_arn(tg_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(health.target_health_descriptions().len(), 0);

    // Register two targets with different ports
    let t1 = TargetDescription::builder()
        .id("i-1111111111111111")
        .port(5060)
        .build();
    let t2 = TargetDescription::builder()
        .id("i-2222222222222222")
        .port(4030)
        .build();

    client
        .register_targets()
        .target_group_arn(tg_arn)
        .targets(t1)
        .targets(t2)
        .send()
        .await
        .unwrap();

    let health = client
        .describe_target_health()
        .target_group_arn(tg_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(health.target_health_descriptions().len(), 2);

    // Deregister one target (without specifying port, should match by ID)
    let t2_dereg = TargetDescription::builder()
        .id("i-2222222222222222")
        .port(4030)
        .build();
    client
        .deregister_targets()
        .target_group_arn(tg_arn)
        .targets(t2_dereg)
        .send()
        .await
        .unwrap();

    let health = client
        .describe_target_health()
        .target_group_arn(tg_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(health.target_health_descriptions().len(), 1);
    assert_eq!(
        health.target_health_descriptions()[0]
            .target()
            .unwrap()
            .id(),
        Some("i-1111111111111111")
    );
}

// Ported from moto: test_elbv2_target_groups.py::test_create_target_group_with_tags
#[tokio::test]
async fn test_target_group_tags() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("tagged-tg")
        .protocol(ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0]
        .target_group_arn()
        .unwrap()
        .to_string();

    // Add tags
    let tag1 = Tag::builder().key("key1").value("val1").build();
    let tag2 = Tag::builder().key("key2").value("val2").build();
    client
        .add_tags()
        .resource_arns(&tg_arn)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .unwrap();

    // Verify tags
    let desc = client
        .describe_tags()
        .resource_arns(&tg_arn)
        .send()
        .await
        .unwrap();
    let tag_descs = desc.tag_descriptions();
    assert_eq!(tag_descs.len(), 1);
    assert_eq!(tag_descs[0].resource_arn(), Some(tg_arn.as_str()));
    let tags = tag_descs[0].tags();
    assert_eq!(tags.len(), 2);
    assert!(
        tags.iter()
            .any(|t| t.key() == Some("key1") && t.value() == Some("val1"))
    );
    assert!(
        tags.iter()
            .any(|t| t.key() == Some("key2") && t.value() == Some("val2"))
    );

    // Remove one tag
    client
        .remove_tags()
        .resource_arns(&tg_arn)
        .tag_keys("key1")
        .send()
        .await
        .unwrap();

    let desc2 = client
        .describe_tags()
        .resource_arns(&tg_arn)
        .send()
        .await
        .unwrap();
    let tags2 = desc2.tag_descriptions()[0].tags();
    assert_eq!(tags2.len(), 1);
    assert!(
        tags2
            .iter()
            .any(|t| t.key() == Some("key2") && t.value() == Some("val2"))
    );
}

// Ported from moto: test_elbv2.py::test_add_remove_tags (tag update + multiple tags)
#[tokio::test]
async fn test_tags_update_existing_value() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_load_balancer()
        .name("tagupdate-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    // Add tag
    let tag1 = Tag::builder().key("env").value("staging").build();
    client
        .add_tags()
        .resource_arns(&lb_arn)
        .tags(tag1)
        .send()
        .await
        .unwrap();

    // Update same tag key with new value
    let tag2 = Tag::builder().key("env").value("production").build();
    client
        .add_tags()
        .resource_arns(&lb_arn)
        .tags(tag2)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_tags()
        .resource_arns(&lb_arn)
        .send()
        .await
        .unwrap();
    let tags = desc.tag_descriptions()[0].tags();
    // Should have only one "env" tag with updated value
    let env_tags: Vec<_> = tags.iter().filter(|t| t.key() == Some("env")).collect();
    assert_eq!(env_tags.len(), 1);
    assert_eq!(env_tags[0].value(), Some("production"));
}

// Ported from moto: test_elbv2.py::test_describe_tags for untagged resources
#[tokio::test]
async fn test_describe_tags_untagged_resource() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_load_balancer()
        .name("notag-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    // Describe tags for a resource that has never had tags added
    let desc = client
        .describe_tags()
        .resource_arns(&lb_arn)
        .send()
        .await
        .unwrap();
    let tag_descs = desc.tag_descriptions();
    assert_eq!(tag_descs.len(), 1);
    assert_eq!(tag_descs[0].resource_arn(), Some(lb_arn.as_str()));
    assert!(tag_descs[0].tags().is_empty());
}

// Ported from moto: test_elbv2_target_groups.py::test_create_target_group_and_listeners
// (comprehensive TG + listener lifecycle with delete cascade)
#[tokio::test]
async fn test_target_group_and_listener_lifecycle() {
    let client = make_elbv2_client().await;

    // Create LB
    let lb_resp = client
        .create_load_balancer()
        .name("lifecycle2-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    // Create TG
    let tg_resp = client
        .create_target_group()
        .name("lifecycle2-tg")
        .protocol(ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0]
        .target_group_arn()
        .unwrap()
        .to_string();

    // Check it's in describe_target_groups
    let desc_tg = client.describe_target_groups().send().await.unwrap();
    assert_eq!(desc_tg.target_groups().len(), 1);

    // Create HTTP listener
    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn)
        .build();
    let l_resp = client
        .create_listener()
        .load_balancer_arn(&lb_arn)
        .port(80)
        .protocol(ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let http_listener_arn = l_resp.listeners()[0].listener_arn().unwrap().to_string();
    assert_eq!(l_resp.listeners()[0].port(), Some(80));

    // Create HTTPS listener
    let l_resp2 = client
        .create_listener()
        .load_balancer_arn(&lb_arn)
        .port(443)
        .protocol(ProtocolEnum::Https)
        .default_actions(action)
        .send()
        .await
        .unwrap();
    let https_listener_arn = l_resp2.listeners()[0].listener_arn().unwrap().to_string();
    assert_eq!(l_resp2.listeners()[0].port(), Some(443));

    // Describe listeners by LB ARN
    let desc_l = client
        .describe_listeners()
        .load_balancer_arn(&lb_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_l.listeners().len(), 2);

    // Describe listeners by ListenerArns
    let desc_one = client
        .describe_listeners()
        .listener_arns(&https_listener_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_one.listeners().len(), 1);
    assert_eq!(desc_one.listeners()[0].port(), Some(443));

    // Describe both listeners by ARN
    let desc_both = client
        .describe_listeners()
        .listener_arns(&http_listener_arn)
        .listener_arns(&https_listener_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_both.listeners().len(), 2);

    // Delete HTTP listener
    client
        .delete_listener()
        .listener_arn(&http_listener_arn)
        .send()
        .await
        .unwrap();
    let desc_l2 = client
        .describe_listeners()
        .load_balancer_arn(&lb_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_l2.listeners().len(), 1);

    // Delete LB (should cascade delete remaining listener)
    client
        .delete_load_balancer()
        .load_balancer_arn(&lb_arn)
        .send()
        .await
        .unwrap();

    // Verify LB is gone
    let desc_lbs = client.describe_load_balancers().send().await.unwrap();
    assert!(desc_lbs.load_balancers().is_empty());

    // Verify remaining listener was cascade-deleted
    let err = client
        .describe_listeners()
        .listener_arns(&https_listener_arn)
        .send()
        .await;
    assert!(err.is_err(), "listener should have been cascade-deleted");

    // TG should still exist
    let desc_tg2 = client.describe_target_groups().send().await.unwrap();
    assert_eq!(desc_tg2.target_groups().len(), 1);

    // Clean up TG
    client
        .delete_target_group()
        .target_group_arn(&tg_arn)
        .send()
        .await
        .unwrap();
    let desc_tg3 = client.describe_target_groups().send().await.unwrap();
    assert!(desc_tg3.target_groups().is_empty());
}

// Ported from moto: test_elbv2.py::test_handle_listener_rules (comprehensive rules test)
#[tokio::test]
async fn test_handle_listener_rules_comprehensive() {
    let (client, _lb_arn, tg_arn, listener_arn) = setup_lb_tg_listener().await;

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn)
        .build();

    // Create first rule at priority 100
    let cond1 = RuleCondition::builder()
        .field("host-header")
        .values("xxx.example.com")
        .build();
    let cond2 = RuleCondition::builder()
        .field("path-pattern")
        .values("foobar")
        .build();

    let rule_resp = client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(100)
        .conditions(cond1)
        .conditions(cond2)
        .actions(action.clone())
        .send()
        .await
        .unwrap();
    let first_rule_arn = rule_resp.rules()[0].rule_arn().unwrap().to_string();
    assert_eq!(rule_resp.rules()[0].priority(), Some("100"));

    // Create second rule at priority 500
    let cond3 = RuleCondition::builder()
        .field("host-header")
        .values("yyy.example.com")
        .build();
    client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(500)
        .conditions(cond3)
        .actions(action.clone())
        .send()
        .await
        .unwrap();

    // Test PriorityInUse
    let cond_dup = RuleCondition::builder()
        .field("path-pattern")
        .values("/dup/*")
        .build();
    let err = client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(500)
        .conditions(cond_dup)
        .actions(action.clone())
        .send()
        .await;
    assert!(err.is_err(), "duplicate priority should fail");

    // Describe rules for listener
    let desc = client
        .describe_rules()
        .listener_arn(&listener_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.rules().len(), 2);

    // Describe by specific rule ARN
    let desc_one = client
        .describe_rules()
        .rule_arns(&first_rule_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_one.rules().len(), 1);
    assert_eq!(
        desc_one.rules()[0].rule_arn(),
        Some(first_rule_arn.as_str())
    );

    // Modify rule conditions
    let new_cond = RuleCondition::builder()
        .field("host-header")
        .values("new.example.com")
        .build();
    let mod_resp = client
        .modify_rule()
        .rule_arn(&first_rule_arn)
        .conditions(new_cond)
        .send()
        .await
        .unwrap();
    let mod_rule = &mod_resp.rules()[0];
    assert!(
        mod_rule.conditions()[0]
            .values()
            .iter()
            .any(|v| v == "new.example.com")
    );
    // Actions unchanged
    assert_eq!(mod_rule.actions().len(), 1);

    // Modify priority
    let prio = RulePriorityPair::builder()
        .rule_arn(&first_rule_arn)
        .priority(99)
        .build();
    let prio_resp = client
        .set_rule_priorities()
        .rule_priorities(prio)
        .send()
        .await
        .unwrap();
    assert_eq!(prio_resp.rules()[0].priority(), Some("99"));
}

// Ported from moto: test_elbv2.py::test_register_targets (register + describe + deregister)
#[tokio::test]
async fn test_register_targets_describe_health() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("health-tg")
        .protocol(ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0]
        .target_group_arn()
        .unwrap()
        .to_string();

    // No targets initially
    let health = client
        .describe_target_health()
        .target_group_arn(&tg_arn)
        .send()
        .await
        .unwrap();
    assert!(health.target_health_descriptions().is_empty());

    // Register targets
    let t1 = TargetDescription::builder()
        .id("i-aaaa1111")
        .port(5060)
        .build();
    let t2 = TargetDescription::builder()
        .id("i-bbbb2222")
        .port(4030)
        .build();
    client
        .register_targets()
        .target_group_arn(&tg_arn)
        .targets(t1)
        .targets(t2)
        .send()
        .await
        .unwrap();

    // Verify both registered
    let health2 = client
        .describe_target_health()
        .target_group_arn(&tg_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(health2.target_health_descriptions().len(), 2);

    // All targets should report healthy
    for thd in health2.target_health_descriptions() {
        assert!(thd.target_health().is_some());
    }

    // Deregister one
    let t2_dereg = TargetDescription::builder()
        .id("i-bbbb2222")
        .port(4030)
        .build();
    client
        .deregister_targets()
        .target_group_arn(&tg_arn)
        .targets(t2_dereg)
        .send()
        .await
        .unwrap();

    let health3 = client
        .describe_target_health()
        .target_group_arn(&tg_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(health3.target_health_descriptions().len(), 1);
    assert_eq!(
        health3.target_health_descriptions()[0]
            .target()
            .unwrap()
            .id(),
        Some("i-aaaa1111")
    );
}

// Ported from moto: test for adding multiple tags and updating values
#[tokio::test]
async fn test_add_multiple_tags_and_update() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_load_balancer()
        .name("multitag-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    // Add several tags at once
    let tags: Vec<Tag> = ["a", "b", "c", "d", "e"]
        .iter()
        .map(|k| Tag::builder().key(*k).value("val").build())
        .collect();

    let mut req = client.add_tags().resource_arns(&lb_arn);
    for t in tags {
        req = req.tags(t);
    }
    req.send().await.unwrap();

    // Verify 5 tags
    let desc = client
        .describe_tags()
        .resource_arns(&lb_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.tag_descriptions()[0].tags().len(), 5);

    // Update one tag
    let update = Tag::builder().key("c").value("updated").build();
    client
        .add_tags()
        .resource_arns(&lb_arn)
        .tags(update)
        .send()
        .await
        .unwrap();

    let desc2 = client
        .describe_tags()
        .resource_arns(&lb_arn)
        .send()
        .await
        .unwrap();
    let tags2 = desc2.tag_descriptions()[0].tags();
    // Still 5 tags (not 6), because "c" was updated in-place
    assert_eq!(tags2.len(), 5);
    let c_tag = tags2.iter().find(|t| t.key() == Some("c")).unwrap();
    assert_eq!(c_tag.value(), Some("updated"));

    // Remove "a" tag
    client
        .remove_tags()
        .resource_arns(&lb_arn)
        .tag_keys("a")
        .send()
        .await
        .unwrap();

    let desc3 = client
        .describe_tags()
        .resource_arns(&lb_arn)
        .send()
        .await
        .unwrap();
    let tags3 = desc3.tag_descriptions()[0].tags();
    assert_eq!(tags3.len(), 4);
    assert!(!tags3.iter().any(|t| t.key() == Some("a")));
}

// Ported from moto: test describe_rules by RuleArns
#[tokio::test]
async fn test_describe_rules_by_rule_arns() {
    let (client, _lb_arn, tg_arn, listener_arn) = setup_lb_tg_listener().await;

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn)
        .build();

    let cond = RuleCondition::builder()
        .field("path-pattern")
        .values("/a/*")
        .build();
    let r1 = client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(10)
        .conditions(cond)
        .actions(action.clone())
        .send()
        .await
        .unwrap();
    let r1_arn = r1.rules()[0].rule_arn().unwrap().to_string();

    let cond2 = RuleCondition::builder()
        .field("path-pattern")
        .values("/b/*")
        .build();
    let r2 = client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(20)
        .conditions(cond2)
        .actions(action)
        .send()
        .await
        .unwrap();
    let r2_arn = r2.rules()[0].rule_arn().unwrap().to_string();

    // Describe all rules for listener
    let all = client
        .describe_rules()
        .listener_arn(&listener_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(all.rules().len(), 2);

    // Describe by specific rule ARN
    let one = client
        .describe_rules()
        .rule_arns(&r1_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(one.rules().len(), 1);
    assert_eq!(one.rules()[0].priority(), Some("10"));

    // Describe both rule ARNs
    let both = client
        .describe_rules()
        .rule_arns(&r1_arn)
        .rule_arns(&r2_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(both.rules().len(), 2);
}

// Ported from moto: test LB attributes with multiple keys
#[tokio::test]
async fn test_load_balancer_attributes_multiple() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_load_balancer()
        .name("multiattr-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = resp.load_balancers()[0].load_balancer_arn().unwrap();

    // Initially empty
    let desc = client
        .describe_load_balancer_attributes()
        .load_balancer_arn(lb_arn)
        .send()
        .await
        .unwrap();
    assert!(desc.attributes().is_empty());

    // Set multiple attributes
    let a1 = aws_sdk_elasticloadbalancingv2::types::LoadBalancerAttribute::builder()
        .key("idle_timeout.timeout_seconds")
        .value("120")
        .build();
    let a2 = aws_sdk_elasticloadbalancingv2::types::LoadBalancerAttribute::builder()
        .key("routing.http2.enabled")
        .value("true")
        .build();

    client
        .modify_load_balancer_attributes()
        .load_balancer_arn(lb_arn)
        .attributes(a1)
        .attributes(a2)
        .send()
        .await
        .unwrap();

    // Verify both are present
    let desc2 = client
        .describe_load_balancer_attributes()
        .load_balancer_arn(lb_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.attributes().len(), 2);
    assert!(
        desc2
            .attributes()
            .iter()
            .any(|a| a.key() == Some("idle_timeout.timeout_seconds") && a.value() == Some("120"))
    );
    assert!(
        desc2
            .attributes()
            .iter()
            .any(|a| a.key() == Some("routing.http2.enabled") && a.value() == Some("true"))
    );
}

// Ported from moto: test registering duplicate target (idempotent)
#[tokio::test]
async fn test_register_target_idempotent() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("idemp-tg")
        .protocol(ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let target = TargetDescription::builder()
        .id("i-aaaa1111")
        .port(80)
        .build();

    // Register same target twice
    client
        .register_targets()
        .target_group_arn(tg_arn)
        .targets(target.clone())
        .send()
        .await
        .unwrap();
    client
        .register_targets()
        .target_group_arn(tg_arn)
        .targets(target)
        .send()
        .await
        .unwrap();

    // Should only have one target
    let health = client
        .describe_target_health()
        .target_group_arn(tg_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(health.target_health_descriptions().len(), 1);
}

// Ported from moto: test multiple certificates on a listener
#[tokio::test]
async fn test_multiple_listener_certificates() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("multicert-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("multicert-tg")
        .protocol(ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(443)
        .protocol(ProtocolEnum::Https)
        .default_actions(action)
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    // Add two certificates
    let cert1 = aws_sdk_elasticloadbalancingv2::types::Certificate::builder()
        .certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/cert-1")
        .build();
    let cert2 = aws_sdk_elasticloadbalancingv2::types::Certificate::builder()
        .certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/cert-2")
        .build();

    client
        .add_listener_certificates()
        .listener_arn(listener_arn)
        .certificates(cert1)
        .certificates(cert2)
        .send()
        .await
        .unwrap();

    // Describe should show 2
    let desc = client
        .describe_listener_certificates()
        .listener_arn(listener_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.certificates().len(), 2);

    // Adding the same cert again should be idempotent
    let cert1_dup = aws_sdk_elasticloadbalancingv2::types::Certificate::builder()
        .certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/cert-1")
        .build();
    client
        .add_listener_certificates()
        .listener_arn(listener_arn)
        .certificates(cert1_dup)
        .send()
        .await
        .unwrap();

    let desc2 = client
        .describe_listener_certificates()
        .listener_arn(listener_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2.certificates().len(),
        2,
        "duplicate cert should not add another"
    );

    // Remove one
    let cert_rm = aws_sdk_elasticloadbalancingv2::types::Certificate::builder()
        .certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/cert-1")
        .build();
    client
        .remove_listener_certificates()
        .listener_arn(listener_arn)
        .certificates(cert_rm)
        .send()
        .await
        .unwrap();

    let desc3 = client
        .describe_listener_certificates()
        .listener_arn(listener_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc3.certificates().len(), 1);
    assert_eq!(
        desc3.certificates()[0].certificate_arn(),
        Some("arn:aws:acm:us-east-1:123456789012:certificate/cert-2")
    );
}

#[tokio::test]
async fn test_describe_load_balancers_by_arn() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_load_balancer()
        .name("filter-arn-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = resp.load_balancers()[0].load_balancer_arn().unwrap();

    // Create a second LB to ensure filtering works
    client
        .create_load_balancer()
        .name("filter-arn-alb2")
        .send()
        .await
        .unwrap();

    let desc_resp = client
        .describe_load_balancers()
        .load_balancer_arns(lb_arn)
        .send()
        .await
        .expect("describe_load_balancers by ARN should succeed");

    assert_eq!(desc_resp.load_balancers().len(), 1);
    assert_eq!(
        desc_resp.load_balancers()[0].load_balancer_arn(),
        Some(lb_arn)
    );
}

#[tokio::test]
async fn test_describe_load_balancers_by_name() {
    let client = make_elbv2_client().await;

    client
        .create_load_balancer()
        .name("byname-alb1")
        .send()
        .await
        .unwrap();

    client
        .create_load_balancer()
        .name("byname-alb2")
        .send()
        .await
        .unwrap();

    let desc_resp = client
        .describe_load_balancers()
        .names("byname-alb1")
        .send()
        .await
        .expect("describe_load_balancers by name should succeed");

    assert_eq!(desc_resp.load_balancers().len(), 1);
    assert_eq!(
        desc_resp.load_balancers()[0].load_balancer_name(),
        Some("byname-alb1")
    );
}

#[tokio::test]
async fn test_describe_load_balancers_nonexistent_arn_fails() {
    let client = make_elbv2_client().await;

    let result = client
        .describe_load_balancers()
        .load_balancer_arns(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/no-such/abcdef",
        )
        .send()
        .await;

    assert!(result.is_err(), "describe with nonexistent ARN should fail");
}

#[tokio::test]
async fn test_describe_load_balancers_nonexistent_name_fails() {
    let client = make_elbv2_client().await;

    let result = client
        .describe_load_balancers()
        .names("no-such-alb")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe with nonexistent name should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_load_balancer_fails() {
    let client = make_elbv2_client().await;

    let result = client
        .delete_load_balancer()
        .load_balancer_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/no-such/abcdef",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "deleting a nonexistent load balancer should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_target_group_fails() {
    let client = make_elbv2_client().await;

    let result = client
        .delete_target_group()
        .target_group_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/no-such/abcdef",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "deleting a nonexistent target group should fail"
    );
}

#[tokio::test]
async fn test_create_listener_on_nonexistent_lb_fails() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("noalb-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let result = client
        .create_listener()
        .load_balancer_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/no-such/abcdef",
        )
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action)
        .send()
        .await;

    assert!(
        result.is_err(),
        "create_listener on nonexistent LB should fail"
    );
}

#[tokio::test]
async fn test_describe_listeners_by_listener_arn() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("listbyarn-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("listbyarn-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    // Also create a second listener on a different port so we can verify filtering
    client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(8080)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action)
        .send()
        .await
        .unwrap();

    let desc_resp = client
        .describe_listeners()
        .listener_arns(listener_arn)
        .send()
        .await
        .expect("describe_listeners by listener ARN should succeed");

    assert_eq!(desc_resp.listeners().len(), 1);
    assert_eq!(desc_resp.listeners()[0].listener_arn(), Some(listener_arn));
    assert_eq!(desc_resp.listeners()[0].port(), Some(80));
}

#[tokio::test]
async fn test_describe_listeners_without_filter_fails() {
    let client = make_elbv2_client().await;

    // describe_listeners requires either lb ARN or listener ARNs; calling with neither should fail
    let result = client.describe_listeners().send().await;
    assert!(
        result.is_err(),
        "describe_listeners without any filter should fail"
    );
}

#[tokio::test]
async fn test_describe_listeners_nonexistent_listener_arn_fails() {
    let client = make_elbv2_client().await;

    let result = client
        .describe_listeners()
        .listener_arns(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:listener/app/no-such/abcdef",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_listeners with nonexistent listener ARN should fail"
    );
}

#[tokio::test]
async fn test_create_rule_duplicate_priority_fails() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("dupprio-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("dupprio-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    let condition1 = RuleCondition::builder()
        .field("path-pattern")
        .values("/first/*")
        .build();

    client
        .create_rule()
        .listener_arn(listener_arn)
        .priority(10)
        .conditions(condition1)
        .actions(action.clone())
        .send()
        .await
        .unwrap();

    let condition2 = RuleCondition::builder()
        .field("path-pattern")
        .values("/second/*")
        .build();

    let result = client
        .create_rule()
        .listener_arn(listener_arn)
        .priority(10)
        .conditions(condition2)
        .actions(action)
        .send()
        .await;

    assert!(
        result.is_err(),
        "duplicate priority on same listener should fail"
    );
}

#[tokio::test]
async fn test_delete_listener_removes_rules() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("delrules-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("delrules-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    let condition = RuleCondition::builder()
        .field("path-pattern")
        .values("/check/*")
        .build();

    client
        .create_rule()
        .listener_arn(listener_arn)
        .priority(5)
        .conditions(condition)
        .actions(action)
        .send()
        .await
        .unwrap();

    // Verify rule exists
    let desc_before = client
        .describe_rules()
        .listener_arn(listener_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_before.rules().len(), 1);

    // Delete the listener
    client
        .delete_listener()
        .listener_arn(listener_arn)
        .send()
        .await
        .expect("delete_listener should succeed");

    // Rules for that listener should no longer exist
    let desc_after = client.describe_rules().send().await.unwrap();
    assert!(
        desc_after.rules().is_empty(),
        "rules should be removed when listener is deleted"
    );
}

#[tokio::test]
async fn test_delete_load_balancer_removes_listeners_and_rules() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("cascade-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    let tg_resp = client
        .create_target_group()
        .name("cascade-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(&lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0]
        .listener_arn()
        .unwrap()
        .to_string();

    let condition = RuleCondition::builder()
        .field("path-pattern")
        .values("/cascade/*")
        .build();

    client
        .create_rule()
        .listener_arn(&listener_arn)
        .priority(15)
        .conditions(condition)
        .actions(action)
        .send()
        .await
        .unwrap();

    // Delete the load balancer
    client
        .delete_load_balancer()
        .load_balancer_arn(&lb_arn)
        .send()
        .await
        .expect("delete_load_balancer should succeed");

    // No load balancers remain
    let lbs = client.describe_load_balancers().send().await.unwrap();
    assert!(lbs.load_balancers().is_empty());

    // No rules remain (cascaded delete via listener)
    let rules = client.describe_rules().send().await.unwrap();
    assert!(
        rules.rules().is_empty(),
        "rules should be cascade-deleted with the load balancer"
    );
}

#[tokio::test]
async fn test_register_targets_idempotent() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("idem-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let target = TargetDescription::builder()
        .id("i-idempotent0000001")
        .port(80)
        .build();

    client
        .register_targets()
        .target_group_arn(tg_arn)
        .targets(target.clone())
        .send()
        .await
        .expect("first register_targets should succeed");

    // Register the same target again; should not duplicate it
    client
        .register_targets()
        .target_group_arn(tg_arn)
        .targets(target)
        .send()
        .await
        .expect("second register_targets should also succeed");

    let health_resp = client
        .describe_target_health()
        .target_group_arn(tg_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(
        health_resp.target_health_descriptions().len(),
        1,
        "re-registering the same target should not create a duplicate"
    );
}

#[tokio::test]
async fn test_register_multiple_targets() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("multi-target-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let target1 = TargetDescription::builder()
        .id("i-multi0000000001")
        .port(80)
        .build();
    let target2 = TargetDescription::builder()
        .id("i-multi0000000002")
        .port(80)
        .build();
    let target3 = TargetDescription::builder()
        .id("i-multi0000000003")
        .port(8080)
        .build();

    client
        .register_targets()
        .target_group_arn(tg_arn)
        .targets(target1)
        .targets(target2)
        .targets(target3)
        .send()
        .await
        .expect("register_targets with multiple targets should succeed");

    let health_resp = client
        .describe_target_health()
        .target_group_arn(tg_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(health_resp.target_health_descriptions().len(), 3);
}

#[tokio::test]
async fn test_tags_on_target_group() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("tagged-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0]
        .target_group_arn()
        .unwrap()
        .to_string();

    let tag = Tag::builder().key("team").value("backend").build();

    client
        .add_tags()
        .resource_arns(&tg_arn)
        .tags(tag)
        .send()
        .await
        .expect("add_tags on target group should succeed");

    let desc_resp = client
        .describe_tags()
        .resource_arns(&tg_arn)
        .send()
        .await
        .expect("describe_tags on target group should succeed");

    let tag_descs = desc_resp.tag_descriptions();
    assert_eq!(tag_descs.len(), 1);
    assert!(
        tag_descs[0]
            .tags()
            .iter()
            .any(|t| t.key() == Some("team") && t.value() == Some("backend")),
        "tag should be present on target group"
    );
}

#[tokio::test]
async fn test_describe_rules_by_rule_arn() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("rulearn-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp = client
        .create_target_group()
        .name("rulearn-tg")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let action = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(tg_arn)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action.clone())
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    let condition1 = RuleCondition::builder()
        .field("path-pattern")
        .values("/a/*")
        .build();
    let rule1_resp = client
        .create_rule()
        .listener_arn(listener_arn)
        .priority(1)
        .conditions(condition1)
        .actions(action.clone())
        .send()
        .await
        .unwrap();
    let rule1_arn = rule1_resp.rules()[0].rule_arn().unwrap();

    let condition2 = RuleCondition::builder()
        .field("path-pattern")
        .values("/b/*")
        .build();
    client
        .create_rule()
        .listener_arn(listener_arn)
        .priority(2)
        .conditions(condition2)
        .actions(action)
        .send()
        .await
        .unwrap();

    // Describe only the first rule by its ARN
    let desc_resp = client
        .describe_rules()
        .rule_arns(rule1_arn)
        .send()
        .await
        .expect("describe_rules by rule ARN should succeed");

    assert_eq!(desc_resp.rules().len(), 1);
    assert_eq!(desc_resp.rules()[0].rule_arn(), Some(rule1_arn));
    assert_eq!(desc_resp.rules()[0].priority(), Some("1"));
}

#[tokio::test]
async fn test_describe_load_balancer_attributes_nonexistent_fails() {
    let client = make_elbv2_client().await;

    let result = client
        .describe_load_balancer_attributes()
        .load_balancer_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/no-such/abcdef",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_load_balancer_attributes on nonexistent LB should fail"
    );
}

#[tokio::test]
async fn test_create_load_balancer_internal_scheme() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_load_balancer()
        .name("internal-alb")
        .scheme(aws_sdk_elasticloadbalancingv2::types::LoadBalancerSchemeEnum::Internal)
        .send()
        .await
        .expect("create internal load balancer should succeed");

    let lbs = resp.load_balancers();
    assert_eq!(lbs.len(), 1);
    assert_eq!(
        lbs[0].scheme(),
        Some(&aws_sdk_elasticloadbalancingv2::types::LoadBalancerSchemeEnum::Internal)
    );
}

#[tokio::test]
async fn test_modify_listener_default_action() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("modaction-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let tg_resp1 = client
        .create_target_group()
        .name("modaction-tg1")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn1 = tg_resp1.target_groups()[0].target_group_arn().unwrap();

    let tg_resp2 = client
        .create_target_group()
        .name("modaction-tg2")
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn2 = tg_resp2.target_groups()[0]
        .target_group_arn()
        .unwrap()
        .to_string();

    let action1 = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(tg_arn1)
        .build();

    let listener_resp = client
        .create_listener()
        .load_balancer_arn(lb_arn)
        .port(80)
        .protocol(aws_sdk_elasticloadbalancingv2::types::ProtocolEnum::Http)
        .default_actions(action1)
        .send()
        .await
        .unwrap();
    let listener_arn = listener_resp.listeners()[0].listener_arn().unwrap();

    // Modify the default action to point at a different target group
    let action2 = Action::builder()
        .r#type(ActionTypeEnum::Forward)
        .target_group_arn(&tg_arn2)
        .build();

    let mod_resp = client
        .modify_listener()
        .listener_arn(listener_arn)
        .default_actions(action2)
        .send()
        .await
        .expect("modify_listener default action should succeed");

    let listeners = mod_resp.listeners();
    assert_eq!(listeners.len(), 1);
    let default_actions = listeners[0].default_actions();
    assert!(
        default_actions
            .iter()
            .any(|a| a.target_group_arn() == Some(tg_arn2.as_str())),
        "default action should point to the new target group"
    );
}

#[tokio::test]
async fn test_add_multiple_tags_and_remove_one() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("multitag-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0]
        .load_balancer_arn()
        .unwrap()
        .to_string();

    let tag1 = Tag::builder().key("env").value("prod").build();
    let tag2 = Tag::builder().key("owner").value("platform").build();
    let tag3 = Tag::builder().key("cost-center").value("eng-001").build();

    client
        .add_tags()
        .resource_arns(&lb_arn)
        .tags(tag1)
        .tags(tag2)
        .tags(tag3)
        .send()
        .await
        .expect("adding multiple tags should succeed");

    // Remove just one tag
    client
        .remove_tags()
        .resource_arns(&lb_arn)
        .tag_keys("owner")
        .send()
        .await
        .expect("remove_tags should succeed");

    let desc_resp = client
        .describe_tags()
        .resource_arns(&lb_arn)
        .send()
        .await
        .unwrap();

    let tags = desc_resp.tag_descriptions()[0].tags();
    assert_eq!(tags.len(), 2, "only two tags should remain");
    assert!(tags.iter().any(|t| t.key() == Some("env")));
    assert!(tags.iter().any(|t| t.key() == Some("cost-center")));
    assert!(
        !tags.iter().any(|t| t.key() == Some("owner")),
        "removed tag should not be present"
    );
}

#[tokio::test]
async fn test_modify_target_group_nonexistent_fails() {
    let client = make_elbv2_client().await;

    let result = client
        .modify_target_group()
        .target_group_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/no-such/abcdef",
        )
        .health_check_path("/ping")
        .send()
        .await;

    assert!(
        result.is_err(),
        "modify_target_group on nonexistent target group should fail"
    );
}

#[tokio::test]
async fn test_describe_target_health_nonexistent_target_group_fails() {
    let client = make_elbv2_client().await;

    let result = client
        .describe_target_health()
        .target_group_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/no-such/abcdef",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_target_health on nonexistent target group should fail"
    );
}

#[tokio::test]
async fn test_set_rule_priorities_nonexistent_rule_fails() {
    let client = make_elbv2_client().await;

    let pair = RulePriorityPair::builder()
        .rule_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:listener-rule/app/no-such/abcdef",
        )
        .priority(99)
        .build();

    let result = client
        .set_rule_priorities()
        .rule_priorities(pair)
        .send()
        .await;

    assert!(
        result.is_err(),
        "set_rule_priorities with nonexistent rule ARN should fail"
    );
}

// --- DescribeAccountLimits ---

#[tokio::test]
async fn test_describe_account_limits() {
    let client = make_elbv2_client().await;

    let resp = client
        .describe_account_limits()
        .send()
        .await
        .expect("describe_account_limits should succeed");

    let limits = resp.limits();
    assert!(
        !limits.is_empty(),
        "should return at least one account limit"
    );
    // Check for a well-known limit
    assert!(
        limits
            .iter()
            .any(|l| l.name() == Some("application-load-balancers")),
        "should include application-load-balancers limit"
    );
}

// --- DescribeSSLPolicies ---

#[tokio::test]
async fn test_describe_ssl_policies() {
    let client = make_elbv2_client().await;

    let resp = client
        .describe_ssl_policies()
        .send()
        .await
        .expect("describe_ssl_policies should succeed");

    let policies = resp.ssl_policies();
    assert!(
        !policies.is_empty(),
        "should return at least one SSL policy"
    );
    let policy = &policies[0];
    assert!(policy.name().is_some(), "SSL policy should have a name");
    assert!(
        !policy.ssl_protocols().is_empty(),
        "SSL policy should have protocols"
    );
    assert!(
        !policy.ciphers().is_empty(),
        "SSL policy should have ciphers"
    );
}

// --- GetResourcePolicy ---

#[tokio::test]
async fn test_get_resource_policy() {
    let client = make_elbv2_client().await;

    let resp = client
        .get_resource_policy()
        .resource_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/test/abc",
        )
        .send()
        .await
        .expect("get_resource_policy should succeed");

    // The mock returns no policy by default
    assert!(
        resp.policy().is_none() || resp.policy() == Some(""),
        "should return empty or no policy"
    );
}

// --- DescribeTargetGroupAttributes ---

#[tokio::test]
async fn test_describe_target_group_attributes() {
    let client = make_elbv2_client().await;

    let tg_resp = client
        .create_target_group()
        .name("tgattr-tg")
        .protocol(ProtocolEnum::Http)
        .port(80)
        .vpc_id("vpc-12345678")
        .send()
        .await
        .unwrap();
    let tg_arn = tg_resp.target_groups()[0].target_group_arn().unwrap();

    let resp = client
        .describe_target_group_attributes()
        .target_group_arn(tg_arn)
        .send()
        .await
        .expect("describe_target_group_attributes should succeed");

    let attrs = resp.attributes();
    assert!(
        !attrs.is_empty(),
        "should return default target group attributes"
    );
    // Should include well-known defaults
    assert!(
        attrs
            .iter()
            .any(|a| a.key() == Some("deregistration_delay.timeout_seconds")),
        "should include deregistration_delay.timeout_seconds"
    );
    assert!(
        attrs.iter().any(|a| a.key() == Some("stickiness.enabled")),
        "should include stickiness.enabled"
    );
}

// --- DescribeCapacityReservation ---

#[tokio::test]
async fn test_describe_capacity_reservation() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("capreserv-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let resp = client
        .describe_capacity_reservation()
        .load_balancer_arn(lb_arn)
        .send()
        .await
        .expect("describe_capacity_reservation should succeed");

    // Check for well-formed response
    assert!(
        !resp.capacity_reservation_state().is_empty(),
        "should return capacity reservation state"
    );
}

// --- ModifyCapacityReservation ---

#[tokio::test]
async fn test_modify_capacity_reservation() {
    let client = make_elbv2_client().await;

    let lb_resp = client
        .create_load_balancer()
        .name("modcapreserv-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let resp = client
        .modify_capacity_reservation()
        .load_balancer_arn(lb_arn)
        .send()
        .await
        .expect("modify_capacity_reservation should succeed");

    assert!(
        !resp.capacity_reservation_state().is_empty(),
        "should return capacity reservation state"
    );
}

// --- ModifyIpPools ---

#[tokio::test]
async fn test_modify_ip_pools() {
    let client = make_elbv2_client().await;

    // Create a load balancer first — the handler requires LoadBalancerArn
    let lb_resp = client
        .create_load_balancer()
        .name("ip-pools-alb")
        .send()
        .await
        .unwrap();
    let lb_arn = lb_resp.load_balancers()[0].load_balancer_arn().unwrap();

    let resp = client
        .modify_ip_pools()
        .load_balancer_arn(lb_arn)
        .send()
        .await
        .expect("modify_ip_pools should succeed");

    let _ = resp;
}

// --- TrustStore CRUD ---

#[tokio::test]
async fn test_create_describe_delete_trust_store() {
    let client = make_elbv2_client().await;

    // Create trust store
    let create_resp = client
        .create_trust_store()
        .name("test-ts")
        .ca_certificates_bundle_s3_bucket("my-bucket")
        .ca_certificates_bundle_s3_key("ca-bundle.pem")
        .send()
        .await
        .expect("create_trust_store should succeed");

    let trust_stores = create_resp.trust_stores();
    assert_eq!(trust_stores.len(), 1);
    assert_eq!(trust_stores[0].name(), Some("test-ts"));
    let ts_arn = trust_stores[0].trust_store_arn().unwrap().to_string();

    // Describe trust stores
    let desc_resp = client
        .describe_trust_stores()
        .send()
        .await
        .expect("describe_trust_stores should succeed");
    assert_eq!(desc_resp.trust_stores().len(), 1);
    assert_eq!(desc_resp.trust_stores()[0].name(), Some("test-ts"));

    // Delete trust store
    client
        .delete_trust_store()
        .trust_store_arn(&ts_arn)
        .send()
        .await
        .expect("delete_trust_store should succeed");

    // Verify deleted
    let desc_resp2 = client.describe_trust_stores().send().await.unwrap();
    assert!(desc_resp2.trust_stores().is_empty());
}

// --- Coverage for FIX(terraform-e2e) handler fixes ---

/// DescribeTargetGroups must filter by ARN so the provider's waiter gets back
/// exactly the TG it requested rather than every TG in the account.
#[tokio::test]
async fn test_describe_target_groups_filter_by_arn() {
    let client = make_elbv2_client().await;

    let resp_a = client
        .create_target_group()
        .name("tg-alpha")
        .protocol(ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-11111111")
        .send()
        .await
        .expect("create tg-alpha should succeed");
    let arn_a = resp_a.target_groups()[0]
        .target_group_arn()
        .expect("should have ARN")
        .to_string();

    let resp_b = client
        .create_target_group()
        .name("tg-beta")
        .protocol(ProtocolEnum::Http)
        .port(9090)
        .vpc_id("vpc-22222222")
        .send()
        .await
        .expect("create tg-beta should succeed");
    let arn_b = resp_b.target_groups()[0]
        .target_group_arn()
        .expect("should have ARN")
        .to_string();

    // Without any filter we get both
    let all = client
        .describe_target_groups()
        .send()
        .await
        .expect("describe all should succeed");
    assert_eq!(all.target_groups().len(), 2);

    // Filter by a single ARN returns only that TG
    let by_arn_a = client
        .describe_target_groups()
        .target_group_arns(&arn_a)
        .send()
        .await
        .expect("describe by arn_a should succeed");
    assert_eq!(by_arn_a.target_groups().len(), 1);
    assert_eq!(
        by_arn_a.target_groups()[0].target_group_name(),
        Some("tg-alpha")
    );

    // Filter by the other ARN
    let by_arn_b = client
        .describe_target_groups()
        .target_group_arns(&arn_b)
        .send()
        .await
        .expect("describe by arn_b should succeed");
    assert_eq!(by_arn_b.target_groups().len(), 1);
    assert_eq!(
        by_arn_b.target_groups()[0].target_group_name(),
        Some("tg-beta")
    );
}

/// DescribeTargetGroups must filter by name so only matching TGs are returned.
#[tokio::test]
async fn test_describe_target_groups_filter_by_name() {
    let client = make_elbv2_client().await;

    client
        .create_target_group()
        .name("tg-one")
        .protocol(ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-11111111")
        .send()
        .await
        .expect("create tg-one should succeed");

    client
        .create_target_group()
        .name("tg-two")
        .protocol(ProtocolEnum::Http)
        .port(9090)
        .vpc_id("vpc-22222222")
        .send()
        .await
        .expect("create tg-two should succeed");

    // Filter by name returns only the matching TG
    let by_name = client
        .describe_target_groups()
        .names("tg-one")
        .send()
        .await
        .expect("describe by name should succeed");
    assert_eq!(by_name.target_groups().len(), 1);
    assert_eq!(
        by_name.target_groups()[0].target_group_name(),
        Some("tg-one")
    );
}

/// When both ARN and name filters are empty, all TGs are returned.
/// When both are specified, results must satisfy both predicates.
#[tokio::test]
async fn test_describe_target_groups_filter_by_arn_and_name() {
    let client = make_elbv2_client().await;

    let resp = client
        .create_target_group()
        .name("tg-combined")
        .protocol(ProtocolEnum::Http)
        .port(8080)
        .vpc_id("vpc-11111111")
        .send()
        .await
        .expect("create tg-combined should succeed");
    let arn = resp.target_groups()[0]
        .target_group_arn()
        .expect("should have ARN")
        .to_string();

    client
        .create_target_group()
        .name("tg-other")
        .protocol(ProtocolEnum::Http)
        .port(9090)
        .vpc_id("vpc-22222222")
        .send()
        .await
        .expect("create tg-other should succeed");

    // ARN matches tg-combined, name matches tg-combined => 1 result
    let both_match = client
        .describe_target_groups()
        .target_group_arns(&arn)
        .names("tg-combined")
        .send()
        .await
        .expect("describe with matching arn+name should succeed");
    assert_eq!(both_match.target_groups().len(), 1);
    assert_eq!(
        both_match.target_groups()[0].target_group_name(),
        Some("tg-combined")
    );

    // ARN matches tg-combined, name matches tg-other => 0 results (AND logic)
    let mismatch = client
        .describe_target_groups()
        .target_group_arns(&arn)
        .names("tg-other")
        .send()
        .await
        .expect("describe with mismatched arn+name should succeed");
    assert_eq!(mismatch.target_groups().len(), 0);
}
