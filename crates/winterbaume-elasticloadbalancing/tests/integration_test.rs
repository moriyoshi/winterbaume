use aws_sdk_elasticloadbalancing::config::BehaviorVersion;
use aws_sdk_elasticloadbalancing::types::{Listener as SdkListener, Tag};
use winterbaume_core::MockAws;
use winterbaume_elasticloadbalancing::ElasticLoadBalancingService;

async fn make_elb_client() -> aws_sdk_elasticloadbalancing::Client {
    let mock = MockAws::builder()
        .with_service(ElasticLoadBalancingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticloadbalancing::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    aws_sdk_elasticloadbalancing::Client::new(&config)
}

fn make_listener(lb_port: i32, instance_port: i32, protocol: &str) -> SdkListener {
    SdkListener::builder()
        .load_balancer_port(lb_port)
        .instance_port(instance_port)
        .protocol(protocol)
        .build()
        .unwrap()
}

// --- CreateLoadBalancer / DescribeLoadBalancers ---

#[tokio::test]
async fn test_create_and_describe_load_balancer() {
    let client = make_elb_client().await;

    let resp = client
        .create_load_balancer()
        .load_balancer_name("test-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 8080, "HTTP"))
        .send()
        .await
        .expect("create_load_balancer should succeed");

    assert!(resp.dns_name().is_some());
    let dns = resp.dns_name().unwrap();
    assert!(dns.contains("test-lb"));

    let desc = client
        .describe_load_balancers()
        .send()
        .await
        .expect("describe_load_balancers should succeed");

    let lbs = desc.load_balancer_descriptions();
    assert_eq!(lbs.len(), 1);
    assert_eq!(lbs[0].load_balancer_name(), Some("test-lb"));
    assert!(lbs[0].dns_name().is_some());
}

#[tokio::test]
async fn test_describe_specific_load_balancer() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("my-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    let desc = client
        .describe_load_balancers()
        .load_balancer_names("my-lb")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(desc.load_balancer_descriptions().len(), 1);
    assert_eq!(
        desc.load_balancer_descriptions()[0].load_balancer_name(),
        Some("my-lb")
    );
}

#[tokio::test]
async fn test_describe_nonexistent_load_balancer_fails() {
    let client = make_elb_client().await;

    let err = client
        .describe_load_balancers()
        .load_balancer_names("nonexistent-lb")
        .send()
        .await;

    assert!(err.is_err(), "Should fail for nonexistent LB");
}

#[tokio::test]
async fn test_create_duplicate_load_balancer_fails() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("dup-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_load_balancer()
        .load_balancer_name("dup-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await;

    assert!(err.is_err(), "Duplicate create should fail");
}

// --- DeleteLoadBalancer ---

#[tokio::test]
async fn test_delete_load_balancer() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("lb-to-delete")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    client
        .delete_load_balancer()
        .load_balancer_name("lb-to-delete")
        .send()
        .await
        .expect("delete should succeed");

    let desc = client
        .describe_load_balancers()
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(desc.load_balancer_descriptions().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_load_balancer_succeeds() {
    // Classic ELB silently ignores deleting a non-existent LB
    let client = make_elb_client().await;

    client
        .delete_load_balancer()
        .load_balancer_name("does-not-exist")
        .send()
        .await
        .expect("delete nonexistent should succeed (silently ignored)");
}

// --- RegisterInstancesWithLoadBalancer / DeregisterInstancesFromLoadBalancer ---

#[tokio::test]
async fn test_register_and_deregister_instances() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("inst-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    let register_resp = client
        .register_instances_with_load_balancer()
        .load_balancer_name("inst-lb")
        .instances(
            aws_sdk_elasticloadbalancing::types::Instance::builder()
                .instance_id("i-12345678")
                .build(),
        )
        .send()
        .await
        .expect("register should succeed");

    let instances = register_resp.instances();
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].instance_id(), Some("i-12345678"));

    let deregister_resp = client
        .deregister_instances_from_load_balancer()
        .load_balancer_name("inst-lb")
        .instances(
            aws_sdk_elasticloadbalancing::types::Instance::builder()
                .instance_id("i-12345678")
                .build(),
        )
        .send()
        .await
        .expect("deregister should succeed");

    assert_eq!(deregister_resp.instances().len(), 0);
}

// --- DescribeInstanceHealth ---

#[tokio::test]
async fn test_describe_instance_health() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("health-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    client
        .register_instances_with_load_balancer()
        .load_balancer_name("health-lb")
        .instances(
            aws_sdk_elasticloadbalancing::types::Instance::builder()
                .instance_id("i-aaaa1111")
                .build(),
        )
        .send()
        .await
        .expect("register should succeed");

    let health_resp = client
        .describe_instance_health()
        .load_balancer_name("health-lb")
        .send()
        .await
        .expect("describe_instance_health should succeed");

    let states = health_resp.instance_states();
    assert_eq!(states.len(), 1);
    assert_eq!(states[0].instance_id(), Some("i-aaaa1111"));
    assert_eq!(states[0].state(), Some("InService"));
}

// --- ConfigureHealthCheck ---

#[tokio::test]
async fn test_configure_health_check() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("hc-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    let hc_resp = client
        .configure_health_check()
        .load_balancer_name("hc-lb")
        .health_check(
            aws_sdk_elasticloadbalancing::types::HealthCheck::builder()
                .target("HTTP:80/health")
                .interval(20)
                .timeout(5)
                .unhealthy_threshold(3)
                .healthy_threshold(2)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("configure_health_check should succeed");

    let hc = hc_resp
        .health_check()
        .expect("health check should be present");
    assert_eq!(hc.target(), "HTTP:80/health");
    assert_eq!(hc.interval(), 20);
}

// --- CreateLoadBalancerListeners / DeleteLoadBalancerListeners ---

#[tokio::test]
async fn test_create_and_delete_listeners() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("listener-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    client
        .create_load_balancer_listeners()
        .load_balancer_name("listener-lb")
        .listeners(make_listener(443, 8443, "HTTPS"))
        .send()
        .await
        .expect("create_listeners should succeed");

    let desc = client
        .describe_load_balancers()
        .load_balancer_names("listener-lb")
        .send()
        .await
        .expect("describe should succeed");

    let listeners = desc.load_balancer_descriptions()[0].listener_descriptions();
    assert_eq!(listeners.len(), 2);

    client
        .delete_load_balancer_listeners()
        .load_balancer_name("listener-lb")
        .load_balancer_ports(443)
        .send()
        .await
        .expect("delete_listeners should succeed");

    let desc2 = client
        .describe_load_balancers()
        .load_balancer_names("listener-lb")
        .send()
        .await
        .expect("describe should succeed");

    let listeners2 = desc2.load_balancer_descriptions()[0].listener_descriptions();
    assert_eq!(listeners2.len(), 1);
    assert_eq!(listeners2[0].listener().unwrap().load_balancer_port(), 80);
}

// --- DescribeLoadBalancerPolicies ---

#[tokio::test]
async fn test_describe_load_balancer_policies_empty() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("policy-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .describe_load_balancer_policies()
        .load_balancer_name("policy-lb")
        .send()
        .await
        .expect("describe_policies should succeed");

    assert_eq!(resp.policy_descriptions().len(), 0);
}

// --- AttachLoadBalancerToSubnets / DetachLoadBalancerFromSubnets ---

#[tokio::test]
async fn test_attach_and_detach_subnets() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("subnet-lb")
        .subnets("subnet-00000001")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    let attach_resp = client
        .attach_load_balancer_to_subnets()
        .load_balancer_name("subnet-lb")
        .subnets("subnet-00000002")
        .send()
        .await
        .expect("attach_subnets should succeed");

    assert!(
        attach_resp
            .subnets()
            .contains(&"subnet-00000002".to_string())
    );

    let detach_resp = client
        .detach_load_balancer_from_subnets()
        .load_balancer_name("subnet-lb")
        .subnets("subnet-00000002")
        .send()
        .await
        .expect("detach_subnets should succeed");

    assert!(
        !detach_resp
            .subnets()
            .contains(&"subnet-00000002".to_string())
    );
}

// --- ApplySecurityGroupsToLoadBalancer ---

#[tokio::test]
async fn test_apply_security_groups() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("sg-lb")
        .subnets("subnet-00000001")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    let sg_resp = client
        .apply_security_groups_to_load_balancer()
        .load_balancer_name("sg-lb")
        .security_groups("sg-12345678")
        .send()
        .await
        .expect("apply_security_groups should succeed");

    assert!(
        sg_resp
            .security_groups()
            .contains(&"sg-12345678".to_string())
    );
}

// --- AddTags / DescribeTags / RemoveTags ---

#[tokio::test]
async fn test_add_describe_remove_tags() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("tagged-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    client
        .add_tags()
        .load_balancer_names("tagged-lb")
        .tags(Tag::builder().key("env").value("test").build().unwrap())
        .tags(Tag::builder().key("team").value("infra").build().unwrap())
        .send()
        .await
        .expect("add_tags should succeed");

    let tag_desc = client
        .describe_tags()
        .load_balancer_names("tagged-lb")
        .send()
        .await
        .expect("describe_tags should succeed");

    let tag_descriptions = tag_desc.tag_descriptions();
    assert_eq!(tag_descriptions.len(), 1);
    let tags = tag_descriptions[0].tags();
    assert_eq!(tags.len(), 2);

    let tag_map: std::collections::HashMap<&str, &str> = tags
        .iter()
        .map(|t| (t.key(), t.value().unwrap_or("")))
        .collect();
    assert_eq!(tag_map.get("env"), Some(&"test"));
    assert_eq!(tag_map.get("team"), Some(&"infra"));

    client
        .remove_tags()
        .load_balancer_names("tagged-lb")
        .tags(
            aws_sdk_elasticloadbalancing::types::TagKeyOnly::builder()
                .key("env")
                .build(),
        )
        .send()
        .await
        .expect("remove_tags should succeed");

    let tag_desc2 = client
        .describe_tags()
        .load_balancer_names("tagged-lb")
        .send()
        .await
        .expect("describe_tags after remove should succeed");

    let tags2 = tag_desc2.tag_descriptions()[0].tags();
    assert_eq!(tags2.len(), 1);
    assert_eq!(tags2[0].key(), "team");
}

// --- EnableAvailabilityZonesForLoadBalancer / DisableAvailabilityZonesForLoadBalancer ---

#[tokio::test]
async fn test_enable_and_disable_availability_zones() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("az-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    let enable_resp = client
        .enable_availability_zones_for_load_balancer()
        .load_balancer_name("az-lb")
        .availability_zones("us-east-1b")
        .send()
        .await
        .expect("enable_azs should succeed");

    let azs = enable_resp.availability_zones();
    assert!(azs.contains(&"us-east-1a".to_string()));
    assert!(azs.contains(&"us-east-1b".to_string()));

    let disable_resp = client
        .disable_availability_zones_for_load_balancer()
        .load_balancer_name("az-lb")
        .availability_zones("us-east-1b")
        .send()
        .await
        .expect("disable_azs should succeed");

    let azs2 = disable_resp.availability_zones();
    assert!(azs2.contains(&"us-east-1a".to_string()));
    assert!(!azs2.contains(&"us-east-1b".to_string()));
}

// --- SetLoadBalancerPoliciesOfListener / CreateLoadBalancerPolicy ---

#[tokio::test]
async fn test_create_policy_and_set_listener() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("pol-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    client
        .create_load_balancer_policy()
        .load_balancer_name("pol-lb")
        .policy_name("my-ssl-policy")
        .policy_type_name("SSLNegotiationPolicyType")
        .send()
        .await
        .expect("create_policy should succeed");

    client
        .set_load_balancer_policies_of_listener()
        .load_balancer_name("pol-lb")
        .load_balancer_port(80)
        .policy_names("my-ssl-policy")
        .send()
        .await
        .expect("set_policies_of_listener should succeed");

    let desc = client
        .describe_load_balancer_policies()
        .load_balancer_name("pol-lb")
        .send()
        .await
        .expect("describe_policies should succeed");

    assert_eq!(desc.policy_descriptions().len(), 1);
    assert_eq!(
        desc.policy_descriptions()[0].policy_name(),
        Some("my-ssl-policy")
    );
}

// --- ModifyLoadBalancerAttributes / DescribeLoadBalancerAttributes ---

#[tokio::test]
async fn test_modify_and_describe_attributes() {
    let client = make_elb_client().await;

    client
        .create_load_balancer()
        .load_balancer_name("attrs-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 80, "HTTP"))
        .send()
        .await
        .expect("create should succeed");

    let desc_attrs = client
        .describe_load_balancer_attributes()
        .load_balancer_name("attrs-lb")
        .send()
        .await
        .expect("describe_attributes should succeed");

    let attrs = desc_attrs.load_balancer_attributes().unwrap();
    assert!(!attrs.cross_zone_load_balancing().unwrap().enabled());
    assert_eq!(attrs.connection_settings().unwrap().idle_timeout(), 60);

    client
        .modify_load_balancer_attributes()
        .load_balancer_name("attrs-lb")
        .load_balancer_attributes(
            aws_sdk_elasticloadbalancing::types::LoadBalancerAttributes::builder()
                .cross_zone_load_balancing(
                    aws_sdk_elasticloadbalancing::types::CrossZoneLoadBalancing::builder()
                        .enabled(true)
                        .build(),
                )
                .connection_settings(
                    aws_sdk_elasticloadbalancing::types::ConnectionSettings::builder()
                        .idle_timeout(120)
                        .build()
                        .unwrap(),
                )
                .connection_draining(
                    aws_sdk_elasticloadbalancing::types::ConnectionDraining::builder()
                        .enabled(true)
                        .timeout(30)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("modify_attributes should succeed");

    let desc_attrs2 = client
        .describe_load_balancer_attributes()
        .load_balancer_name("attrs-lb")
        .send()
        .await
        .expect("describe_attributes after modify should succeed");

    let attrs2 = desc_attrs2.load_balancer_attributes().unwrap();
    assert!(attrs2.cross_zone_load_balancing().unwrap().enabled());
    assert_eq!(attrs2.connection_settings().unwrap().idle_timeout(), 120);
    assert!(attrs2.connection_draining().unwrap().enabled());
}

// --- State view tests ---

#[tokio::test]
async fn test_state_snapshot_restore() {
    use winterbaume_core::StatefulService;
    use winterbaume_elasticloadbalancing::{ElasticLoadBalancingService, ElbStateView};

    let svc = ElasticLoadBalancingService::new();
    let mock = MockAws::builder()
        .with_service(ElasticLoadBalancingService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticloadbalancing::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    let client = aws_sdk_elasticloadbalancing::Client::new(&config);

    // We need to use the same service for state operations, so let's test directly
    let _ = client; // client uses a different service instance

    // Create a snapshot of an empty state
    let empty_view = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(empty_view.load_balancers.len(), 0);

    // Restore a view with a load balancer
    let mut view = ElbStateView::default();
    view.load_balancers.insert(
        "restored-lb".to_string(),
        winterbaume_elasticloadbalancing::views::ElbLoadBalancerView {
            name: "restored-lb".to_string(),
            dns_name: "restored-lb-12345678.us-east-1.elb.amazonaws.com".to_string(),
            scheme: "internet-facing".to_string(),
            availability_zones: vec!["us-east-1a".to_string()],
            subnets: vec![],
            security_groups: vec![],
            instances: vec![],
            listeners: vec![],
            health_check: winterbaume_elasticloadbalancing::views::ElbHealthCheckView {
                target: "TCP:80".to_string(),
                interval: 30,
                timeout: 5,
                unhealthy_threshold: 2,
                healthy_threshold: 10,
            },
            tags: std::collections::HashMap::new(),
            policies: vec![],
            attributes: winterbaume_elasticloadbalancing::views::ElbAttributesView {
                cross_zone_load_balancing_enabled: false,
                access_log_enabled: false,
                access_log_emit_interval: None,
                access_log_s3_bucket_name: None,
                access_log_s3_bucket_prefix: None,
                connection_draining_enabled: false,
                connection_draining_timeout: None,
                connection_idle_timeout: 60,
            },
            vpc_id: None,
        },
    );

    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore should succeed");

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot.load_balancers.len(), 1);
    assert!(snapshot.load_balancers.contains_key("restored-lb"));
}

#[tokio::test]
async fn test_state_merge() {
    use winterbaume_core::StatefulService;
    use winterbaume_elasticloadbalancing::{ElasticLoadBalancingService, ElbStateView};

    let svc = ElasticLoadBalancingService::new();

    // Restore initial state with lb1
    let mut view1 = ElbStateView::default();
    view1.load_balancers.insert(
        "lb1".to_string(),
        winterbaume_elasticloadbalancing::views::ElbLoadBalancerView {
            name: "lb1".to_string(),
            dns_name: "lb1.elb.amazonaws.com".to_string(),
            scheme: "internet-facing".to_string(),
            availability_zones: vec![],
            subnets: vec![],
            security_groups: vec![],
            instances: vec![],
            listeners: vec![],
            health_check: winterbaume_elasticloadbalancing::views::ElbHealthCheckView {
                target: "TCP:80".to_string(),
                interval: 30,
                timeout: 5,
                unhealthy_threshold: 2,
                healthy_threshold: 10,
            },
            tags: std::collections::HashMap::new(),
            policies: vec![],
            attributes: winterbaume_elasticloadbalancing::views::ElbAttributesView {
                cross_zone_load_balancing_enabled: false,
                access_log_enabled: false,
                access_log_emit_interval: None,
                access_log_s3_bucket_name: None,
                access_log_s3_bucket_prefix: None,
                connection_draining_enabled: false,
                connection_draining_timeout: None,
                connection_idle_timeout: 60,
            },
            vpc_id: None,
        },
    );
    svc.restore("123456789012", "us-east-1", view1)
        .await
        .expect("restore should succeed");

    // Merge adds lb2 without removing lb1
    let mut view2 = ElbStateView::default();
    view2.load_balancers.insert(
        "lb2".to_string(),
        winterbaume_elasticloadbalancing::views::ElbLoadBalancerView {
            name: "lb2".to_string(),
            dns_name: "lb2.elb.amazonaws.com".to_string(),
            scheme: "internal".to_string(),
            availability_zones: vec![],
            subnets: vec![],
            security_groups: vec![],
            instances: vec![],
            listeners: vec![],
            health_check: winterbaume_elasticloadbalancing::views::ElbHealthCheckView {
                target: "TCP:80".to_string(),
                interval: 30,
                timeout: 5,
                unhealthy_threshold: 2,
                healthy_threshold: 10,
            },
            tags: std::collections::HashMap::new(),
            policies: vec![],
            attributes: winterbaume_elasticloadbalancing::views::ElbAttributesView {
                cross_zone_load_balancing_enabled: false,
                access_log_enabled: false,
                access_log_emit_interval: None,
                access_log_s3_bucket_name: None,
                access_log_s3_bucket_prefix: None,
                connection_draining_enabled: false,
                connection_draining_timeout: None,
                connection_idle_timeout: 60,
            },
            vpc_id: None,
        },
    );
    svc.merge("123456789012", "us-east-1", view2)
        .await
        .expect("merge should succeed");

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot.load_balancers.len(), 2);
    assert!(snapshot.load_balancers.contains_key("lb1"));
    assert!(snapshot.load_balancers.contains_key("lb2"));
}

// --- State change notification tests ---

#[tokio::test]
async fn test_state_change_listener_fires_on_restore() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_elasticloadbalancing::{ElasticLoadBalancingService, ElbStateView};

    let svc = ElasticLoadBalancingService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);

    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", ElbStateView::default())
        .await
        .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, "123456789012");
    assert_eq!(got[0].1, "us-east-1");
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_elasticloadbalancing::{ElasticLoadBalancingService, ElbStateView};

    let svc = ElasticLoadBalancingService::new();

    // Pre-seed state so that a restore fires (ignore that event)
    svc.restore("123456789012", "us-east-1", ElbStateView::default())
        .await
        .unwrap();

    // Register listener to capture snapshots
    let snapshots: Arc<Mutex<Vec<ElbStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    // Restore a view with a load balancer
    let mut view = ElbStateView::default();
    view.load_balancers.insert(
        "notify-lb".to_string(),
        winterbaume_elasticloadbalancing::views::ElbLoadBalancerView {
            name: "notify-lb".to_string(),
            dns_name: "notify-lb.elb.amazonaws.com".to_string(),
            scheme: "internet-facing".to_string(),
            availability_zones: vec![],
            subnets: vec![],
            security_groups: vec![],
            instances: vec![],
            listeners: vec![],
            health_check: winterbaume_elasticloadbalancing::views::ElbHealthCheckView {
                target: "TCP:80".to_string(),
                interval: 30,
                timeout: 5,
                unhealthy_threshold: 2,
                healthy_threshold: 10,
            },
            tags: std::collections::HashMap::new(),
            policies: vec![],
            attributes: winterbaume_elasticloadbalancing::views::ElbAttributesView {
                cross_zone_load_balancing_enabled: false,
                access_log_enabled: false,
                access_log_emit_interval: None,
                access_log_s3_bucket_name: None,
                access_log_s3_bucket_prefix: None,
                connection_draining_enabled: false,
                connection_draining_timeout: None,
                connection_idle_timeout: 60,
            },
            vpc_id: None,
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].load_balancers.contains_key("notify-lb"));
}

// --- Full lifecycle test ---

#[tokio::test]
async fn test_full_lifecycle() {
    let client = make_elb_client().await;

    // Create
    let create_resp = client
        .create_load_balancer()
        .load_balancer_name("lifecycle-lb")
        .availability_zones("us-east-1a")
        .listeners(make_listener(80, 8080, "HTTP"))
        .listeners(make_listener(443, 8443, "HTTPS"))
        .send()
        .await
        .expect("create should succeed");

    assert!(create_resp.dns_name().is_some());

    // Register instances
    let reg_resp = client
        .register_instances_with_load_balancer()
        .load_balancer_name("lifecycle-lb")
        .instances(
            aws_sdk_elasticloadbalancing::types::Instance::builder()
                .instance_id("i-00000001")
                .build(),
        )
        .instances(
            aws_sdk_elasticloadbalancing::types::Instance::builder()
                .instance_id("i-00000002")
                .build(),
        )
        .send()
        .await
        .expect("register should succeed");
    assert_eq!(reg_resp.instances().len(), 2);

    // Add tags
    client
        .add_tags()
        .load_balancer_names("lifecycle-lb")
        .tags(
            Tag::builder()
                .key("Name")
                .value("lifecycle-lb")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_tags should succeed");

    // Describe
    let desc = client
        .describe_load_balancers()
        .load_balancer_names("lifecycle-lb")
        .send()
        .await
        .expect("describe should succeed");

    let lb = &desc.load_balancer_descriptions()[0];
    assert_eq!(lb.load_balancer_name(), Some("lifecycle-lb"));
    assert_eq!(lb.instances().len(), 2);
    assert_eq!(lb.listener_descriptions().len(), 2);
    assert_eq!(lb.availability_zones(), &["us-east-1a"]);

    // Delete
    client
        .delete_load_balancer()
        .load_balancer_name("lifecycle-lb")
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let desc2 = client
        .describe_load_balancers()
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc2.load_balancer_descriptions().len(), 0);
}
