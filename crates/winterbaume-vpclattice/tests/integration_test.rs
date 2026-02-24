use aws_sdk_vpclattice::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_vpclattice::VpcLatticeService;

async fn make_vpclattice_client() -> aws_sdk_vpclattice::Client {
    let mock = MockAws::builder()
        .with_service(VpcLatticeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_vpclattice::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_vpclattice::Client::new(&config)
}

// ── ServiceNetwork tests ────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_service_network() {
    let client = make_vpclattice_client().await;

    let create_resp = client
        .create_service_network()
        .name("test-network")
        .send()
        .await
        .expect("create_service_network should succeed");

    assert_eq!(create_resp.name(), Some("test-network"));
    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());

    let id = create_resp.id().unwrap();

    let get_resp = client
        .get_service_network()
        .service_network_identifier(id)
        .send()
        .await
        .expect("get_service_network should succeed");

    assert_eq!(get_resp.name(), Some("test-network"));
    assert_eq!(get_resp.id(), Some(id));
}

#[tokio::test]
async fn test_list_service_networks() {
    let client = make_vpclattice_client().await;

    for name in ["net1", "net2", "net3"] {
        client
            .create_service_network()
            .name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_service_networks()
        .send()
        .await
        .expect("list_service_networks should succeed");

    assert_eq!(resp.items().len(), 3);
}

#[tokio::test]
async fn test_list_service_networks_empty() {
    let client = make_vpclattice_client().await;

    let resp = client
        .list_service_networks()
        .send()
        .await
        .expect("list_service_networks should succeed");

    assert!(resp.items().is_empty());
}

#[tokio::test]
async fn test_delete_service_network() {
    let client = make_vpclattice_client().await;

    let create_resp = client
        .create_service_network()
        .name("delete-me")
        .send()
        .await
        .unwrap();

    let id = create_resp.id().unwrap().to_string();

    client
        .delete_service_network()
        .service_network_identifier(&id)
        .send()
        .await
        .expect("delete_service_network should succeed");

    let result = client
        .get_service_network()
        .service_network_identifier(&id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_duplicate_service_network_fails() {
    let client = make_vpclattice_client().await;

    client
        .create_service_network()
        .name("dup-net")
        .send()
        .await
        .unwrap();

    let result = client.create_service_network().name("dup-net").send().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_service_network_fails() {
    let client = make_vpclattice_client().await;

    let result = client
        .get_service_network()
        .service_network_identifier("nonexistent-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_service_network_fails() {
    let client = make_vpclattice_client().await;

    let result = client
        .delete_service_network()
        .service_network_identifier("nonexistent-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_service_network_with_auth_type() {
    let client = make_vpclattice_client().await;

    let resp = client
        .create_service_network()
        .name("auth-network")
        .auth_type(aws_sdk_vpclattice::types::AuthType::AwsIam)
        .send()
        .await
        .expect("create_service_network with auth_type should succeed");

    assert_eq!(resp.name(), Some("auth-network"));
    assert_eq!(
        resp.auth_type(),
        Some(&aws_sdk_vpclattice::types::AuthType::AwsIam)
    );
}

// ── AccessLogSubscription tests ─────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_access_log_subscription() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("log-net")
        .send()
        .await
        .unwrap();
    let sn_arn = sn.arn().unwrap().to_string();

    let create_resp = client
        .create_access_log_subscription()
        .resource_identifier(&sn_arn)
        .destination_arn("arn:aws:s3:::my-log-bucket")
        .send()
        .await
        .expect("create_access_log_subscription should succeed");

    assert!(!create_resp.id().is_empty());
    assert!(!create_resp.arn().is_empty());

    let sub_id = create_resp.id().to_string();

    let get_resp = client
        .get_access_log_subscription()
        .access_log_subscription_identifier(&sub_id)
        .send()
        .await
        .expect("get_access_log_subscription should succeed");

    assert_eq!(get_resp.id(), sub_id.as_str());
    assert_eq!(get_resp.destination_arn(), "arn:aws:s3:::my-log-bucket");
}

#[tokio::test]
async fn test_delete_access_log_subscription() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("del-log-net")
        .send()
        .await
        .unwrap();

    let sub = client
        .create_access_log_subscription()
        .resource_identifier(sn.arn().unwrap())
        .destination_arn("arn:aws:s3:::bucket")
        .send()
        .await
        .unwrap();

    let sub_id = sub.id().to_string();

    client
        .delete_access_log_subscription()
        .access_log_subscription_identifier(&sub_id)
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_access_log_subscription()
        .access_log_subscription_identifier(&sub_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_access_log_subscriptions() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("list-log-net")
        .send()
        .await
        .unwrap();
    let sn_arn = sn.arn().unwrap().to_string();

    client
        .create_access_log_subscription()
        .resource_identifier(&sn_arn)
        .destination_arn("arn:aws:s3:::bucket1")
        .send()
        .await
        .unwrap();

    client
        .create_access_log_subscription()
        .resource_identifier(&sn_arn)
        .destination_arn("arn:aws:s3:::bucket2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_access_log_subscriptions()
        .resource_identifier(&sn_arn)
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_access_log_subscription_fails() {
    let client = make_vpclattice_client().await;

    let result = client
        .get_access_log_subscription()
        .access_log_subscription_identifier("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── ServiceNetworkServiceAssociation tests ──────────────────────────

#[tokio::test]
async fn test_create_and_get_sn_service_association() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("assoc-net")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    let create_resp = client
        .create_service_network_service_association()
        .service_network_identifier(&sn_id)
        .service_identifier("svc-12345678901234567")
        .send()
        .await
        .expect("create_service_network_service_association should succeed");

    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());

    let assoc_id = create_resp.id().unwrap().to_string();

    let get_resp = client
        .get_service_network_service_association()
        .service_network_service_association_identifier(&assoc_id)
        .send()
        .await
        .expect("get should succeed");

    assert_eq!(get_resp.id(), Some(assoc_id.as_str()));
    assert_eq!(get_resp.service_network_id(), Some(sn_id.as_str()));
}

#[tokio::test]
async fn test_delete_sn_service_association() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("del-assoc-net")
        .send()
        .await
        .unwrap();

    let assoc = client
        .create_service_network_service_association()
        .service_network_identifier(sn.id().unwrap())
        .service_identifier("svc-del")
        .send()
        .await
        .unwrap();

    let assoc_id = assoc.id().unwrap().to_string();

    let del_resp = client
        .delete_service_network_service_association()
        .service_network_service_association_identifier(&assoc_id)
        .send()
        .await
        .expect("delete should succeed");

    assert_eq!(del_resp.id(), Some(assoc_id.as_str()));

    let result = client
        .get_service_network_service_association()
        .service_network_service_association_identifier(&assoc_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_sn_service_associations() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("list-svc-net")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    client
        .create_service_network_service_association()
        .service_network_identifier(&sn_id)
        .service_identifier("svc-aaa")
        .send()
        .await
        .unwrap();

    client
        .create_service_network_service_association()
        .service_network_identifier(&sn_id)
        .service_identifier("svc-bbb")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_service_network_service_associations()
        .service_network_identifier(&sn_id)
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_sn_service_association_fails() {
    let client = make_vpclattice_client().await;

    let result = client
        .get_service_network_service_association()
        .service_network_service_association_identifier("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── ServiceNetworkVpcAssociation tests ──────────────────────────────

#[tokio::test]
async fn test_create_and_get_sn_vpc_association() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("vpc-assoc-net")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    let create_resp = client
        .create_service_network_vpc_association()
        .service_network_identifier(&sn_id)
        .vpc_identifier("vpc-12345")
        .security_group_ids("sg-111")
        .security_group_ids("sg-222")
        .send()
        .await
        .expect("create_service_network_vpc_association should succeed");

    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());

    let assoc_id = create_resp.id().unwrap().to_string();

    let get_resp = client
        .get_service_network_vpc_association()
        .service_network_vpc_association_identifier(&assoc_id)
        .send()
        .await
        .expect("get should succeed");

    assert_eq!(get_resp.id(), Some(assoc_id.as_str()));
    assert_eq!(get_resp.vpc_id(), Some("vpc-12345"));
    assert_eq!(get_resp.service_network_id(), Some(sn_id.as_str()));
    assert_eq!(get_resp.security_group_ids().len(), 2);
}

#[tokio::test]
async fn test_delete_sn_vpc_association() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("del-vpc-net")
        .send()
        .await
        .unwrap();

    let assoc = client
        .create_service_network_vpc_association()
        .service_network_identifier(sn.id().unwrap())
        .vpc_identifier("vpc-del")
        .send()
        .await
        .unwrap();

    let assoc_id = assoc.id().unwrap().to_string();

    let del_resp = client
        .delete_service_network_vpc_association()
        .service_network_vpc_association_identifier(&assoc_id)
        .send()
        .await
        .expect("delete should succeed");

    assert_eq!(del_resp.id(), Some(assoc_id.as_str()));

    let result = client
        .get_service_network_vpc_association()
        .service_network_vpc_association_identifier(&assoc_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_sn_vpc_associations() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("list-vpc-net")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    client
        .create_service_network_vpc_association()
        .service_network_identifier(&sn_id)
        .vpc_identifier("vpc-aaa")
        .send()
        .await
        .unwrap();

    client
        .create_service_network_vpc_association()
        .service_network_identifier(&sn_id)
        .vpc_identifier("vpc-bbb")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_service_network_vpc_associations()
        .service_network_identifier(&sn_id)
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_sn_vpc_association_fails() {
    let client = make_vpclattice_client().await;

    let result = client
        .get_service_network_vpc_association()
        .service_network_vpc_association_identifier("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── TargetGroup tests ───────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_target_group() {
    let client = make_vpclattice_client().await;

    let create_resp = client
        .create_target_group()
        .name("my-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .config(
            aws_sdk_vpclattice::types::TargetGroupConfig::builder()
                .port(80)
                .protocol(aws_sdk_vpclattice::types::TargetGroupProtocol::Http)
                .vpc_identifier("vpc-tg")
                .build(),
        )
        .send()
        .await
        .expect("create_target_group should succeed");

    assert_eq!(create_resp.name(), Some("my-tg"));
    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());

    let tg_id = create_resp.id().unwrap().to_string();

    let get_resp = client
        .get_target_group()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .expect("get_target_group should succeed");

    assert_eq!(get_resp.id(), Some(tg_id.as_str()));
    assert_eq!(get_resp.name(), Some("my-tg"));
    assert!(get_resp.config().is_some());
    let cfg = get_resp.config().unwrap();
    assert_eq!(cfg.port(), Some(80));
}

#[tokio::test]
async fn test_delete_target_group() {
    let client = make_vpclattice_client().await;

    let tg = client
        .create_target_group()
        .name("del-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();

    let tg_id = tg.id().unwrap().to_string();

    let del_resp = client
        .delete_target_group()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .expect("delete should succeed");

    assert_eq!(del_resp.id(), Some(tg_id.as_str()));

    let result = client
        .get_target_group()
        .target_group_identifier(&tg_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_target_groups() {
    let client = make_vpclattice_client().await;

    client
        .create_target_group()
        .name("tg-1")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();

    client
        .create_target_group()
        .name("tg-2")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Lambda)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_target_groups()
        .send()
        .await
        .expect("list_target_groups should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_list_target_groups_empty() {
    let client = make_vpclattice_client().await;

    let resp = client
        .list_target_groups()
        .send()
        .await
        .expect("list_target_groups should succeed");

    assert!(resp.items().is_empty());
}

#[tokio::test]
async fn test_get_nonexistent_target_group_fails() {
    let client = make_vpclattice_client().await;

    let result = client
        .get_target_group()
        .target_group_identifier("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_duplicate_target_group_fails() {
    let client = make_vpclattice_client().await;

    client
        .create_target_group()
        .name("dup-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();

    let result = client
        .create_target_group()
        .name("dup-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await;
    assert!(result.is_err());
}

// ── Register/Deregister/List Targets tests ──────────────────────────

#[tokio::test]
async fn test_register_and_list_targets() {
    let client = make_vpclattice_client().await;

    let tg = client
        .create_target_group()
        .name("reg-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    let reg_resp = client
        .register_targets()
        .target_group_identifier(&tg_id)
        .targets(
            aws_sdk_vpclattice::types::Target::builder()
                .id("i-1234567890abcdef0")
                .port(80)
                .build()
                .unwrap(),
        )
        .targets(
            aws_sdk_vpclattice::types::Target::builder()
                .id("i-0987654321fedcba0")
                .port(8080)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("register_targets should succeed");

    assert_eq!(reg_resp.successful().len(), 2);
    assert!(reg_resp.unsuccessful().is_empty());

    let list_resp = client
        .list_targets()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .expect("list_targets should succeed");

    assert_eq!(list_resp.items().len(), 2);
}

#[tokio::test]
async fn test_deregister_targets() {
    let client = make_vpclattice_client().await;

    let tg = client
        .create_target_group()
        .name("dereg-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    client
        .register_targets()
        .target_group_identifier(&tg_id)
        .targets(
            aws_sdk_vpclattice::types::Target::builder()
                .id("i-aaa")
                .port(80)
                .build()
                .unwrap(),
        )
        .targets(
            aws_sdk_vpclattice::types::Target::builder()
                .id("i-bbb")
                .port(80)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let dereg_resp = client
        .deregister_targets()
        .target_group_identifier(&tg_id)
        .targets(
            aws_sdk_vpclattice::types::Target::builder()
                .id("i-aaa")
                .port(80)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("deregister_targets should succeed");

    assert_eq!(dereg_resp.successful().len(), 1);

    let list_resp = client
        .list_targets()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.items().len(), 1);
    assert_eq!(list_resp.items()[0].id(), Some("i-bbb"));
}

#[tokio::test]
async fn test_register_targets_nonexistent_tg_fails() {
    let client = make_vpclattice_client().await;

    let result = client
        .register_targets()
        .target_group_identifier("nonexistent")
        .targets(
            aws_sdk_vpclattice::types::Target::builder()
                .id("i-xxx")
                .build()
                .unwrap(),
        )
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_targets_nonexistent_tg_fails() {
    let client = make_vpclattice_client().await;

    let result = client
        .list_targets()
        .target_group_identifier("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── Lifecycle tests ─────────────────────────────────────────────────

#[tokio::test]
async fn test_target_group_lifecycle() {
    let client = make_vpclattice_client().await;

    // Create
    let tg = client
        .create_target_group()
        .name("lifecycle-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .config(
            aws_sdk_vpclattice::types::TargetGroupConfig::builder()
                .port(443)
                .protocol(aws_sdk_vpclattice::types::TargetGroupProtocol::Https)
                .vpc_identifier("vpc-lifecycle")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    // Register targets
    client
        .register_targets()
        .target_group_identifier(&tg_id)
        .targets(
            aws_sdk_vpclattice::types::Target::builder()
                .id("i-lifecycle")
                .port(443)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Get - verify
    let get = client
        .get_target_group()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get.name(), Some("lifecycle-tg"));

    // List targets
    let targets = client
        .list_targets()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .unwrap();
    assert_eq!(targets.items().len(), 1);

    // Deregister
    client
        .deregister_targets()
        .target_group_identifier(&tg_id)
        .targets(
            aws_sdk_vpclattice::types::Target::builder()
                .id("i-lifecycle")
                .port(443)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let targets = client
        .list_targets()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .unwrap();
    assert!(targets.items().is_empty());

    // Delete
    client
        .delete_target_group()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .get_target_group()
        .target_group_identifier(&tg_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_network_association_lifecycle() {
    let client = make_vpclattice_client().await;

    // Create service network
    let sn = client
        .create_service_network()
        .name("lifecycle-net")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    // Create VPC association
    let vpc_assoc = client
        .create_service_network_vpc_association()
        .service_network_identifier(&sn_id)
        .vpc_identifier("vpc-lifecycle")
        .send()
        .await
        .unwrap();
    let vpc_assoc_id = vpc_assoc.id().unwrap().to_string();

    // Create service association
    let svc_assoc = client
        .create_service_network_service_association()
        .service_network_identifier(&sn_id)
        .service_identifier("svc-lifecycle")
        .send()
        .await
        .unwrap();
    let svc_assoc_id = svc_assoc.id().unwrap().to_string();

    // Verify counts on get
    let sn_get = client
        .get_service_network()
        .service_network_identifier(&sn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(sn_get.number_of_associated_services(), Some(1));
    assert_eq!(sn_get.number_of_associated_vpcs(), Some(1));

    // Delete associations
    client
        .delete_service_network_service_association()
        .service_network_service_association_identifier(&svc_assoc_id)
        .send()
        .await
        .unwrap();

    client
        .delete_service_network_vpc_association()
        .service_network_vpc_association_identifier(&vpc_assoc_id)
        .send()
        .await
        .unwrap();

    // Verify counts decreased
    let sn_get = client
        .get_service_network()
        .service_network_identifier(&sn_id)
        .send()
        .await
        .unwrap();
    assert_eq!(sn_get.number_of_associated_services(), Some(0));
    assert_eq!(sn_get.number_of_associated_vpcs(), Some(0));

    // Verify associations gone
    assert!(
        client
            .get_service_network_service_association()
            .service_network_service_association_identifier(&svc_assoc_id)
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .get_service_network_vpc_association()
            .service_network_vpc_association_identifier(&vpc_assoc_id)
            .send()
            .await
            .is_err()
    );
}

// ── Additional tests from moto ───────────────────────────────────────

/// Translated from moto test_get_service_network - lookup by ARN
#[tokio::test]
async fn test_get_service_network_by_arn() {
    let client = make_vpclattice_client().await;

    let create_resp = client
        .create_service_network()
        .name("arn-lookup-network")
        .auth_type(aws_sdk_vpclattice::types::AuthType::None)
        .send()
        .await
        .expect("create_service_network should succeed");

    let arn = create_resp.arn().unwrap().to_string();
    let id = create_resp.id().unwrap().to_string();

    // Lookup by ARN
    let by_arn = client
        .get_service_network()
        .service_network_identifier(&arn)
        .send()
        .await
        .expect("get_service_network by ARN should succeed");

    assert_eq!(by_arn.id(), Some(id.as_str()));
    assert_eq!(by_arn.arn(), Some(arn.as_str()));
    assert_eq!(by_arn.name(), Some("arn-lookup-network"));
}

/// Translated from moto test_create_service_network - sharingConfig in response
#[tokio::test]
async fn test_create_service_network_with_auth_and_sharing() {
    let client = make_vpclattice_client().await;

    let resp = client
        .create_service_network()
        .name("auth-sharing-network")
        .auth_type(aws_sdk_vpclattice::types::AuthType::None)
        .send()
        .await
        .expect("create should succeed");

    assert_eq!(resp.name(), Some("auth-sharing-network"));
    assert!(resp.arn().is_some());
    assert!(resp.id().unwrap().starts_with("sn-"));
    assert_eq!(
        resp.auth_type(),
        Some(&aws_sdk_vpclattice::types::AuthType::None)
    );
}

/// Translated from moto test_get_access_log_subscription - lookup by id and by arn
#[tokio::test]
async fn test_get_access_log_subscription_by_arn() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("sn-for-als-arn-test")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    let sub = client
        .create_access_log_subscription()
        .resource_identifier(&sn_id)
        .destination_arn("arn:aws:s3:::my-log-bucket")
        .send()
        .await
        .expect("create_access_log_subscription should succeed");

    let sub_id = sub.id().to_string();
    let sub_arn = sub.arn().to_string();

    // Lookup by ID
    let by_id = client
        .get_access_log_subscription()
        .access_log_subscription_identifier(&sub_id)
        .send()
        .await
        .expect("get by ID should succeed");
    assert_eq!(by_id.arn(), sub_arn.as_str());

    // Lookup by ARN
    let by_arn = client
        .get_access_log_subscription()
        .access_log_subscription_identifier(&sub_arn)
        .send()
        .await
        .expect("get by ARN should succeed");
    assert_eq!(by_arn.id(), sub_id.as_str());
    assert_eq!(by_arn.destination_arn(), "arn:aws:s3:::my-log-bucket");
}

/// Translated from moto test_get_access_log_subscription_not_found
#[tokio::test]
async fn test_get_access_log_subscription_not_found_message() {
    let client = make_vpclattice_client().await;

    let err = client
        .get_access_log_subscription()
        .access_log_subscription_identifier("als-invalid-id123")
        .send()
        .await;
    assert!(err.is_err(), "get nonexistent ALS should fail");
}

/// Translated from moto test_delete_access_log_subscription_not_found
#[tokio::test]
async fn test_delete_access_log_subscription_not_found_message() {
    let client = make_vpclattice_client().await;

    let err = client
        .delete_access_log_subscription()
        .access_log_subscription_identifier("als-invalid-id123")
        .send()
        .await;
    assert!(err.is_err(), "delete nonexistent ALS should fail");
}

/// Translated from moto test_create_access_log_subscription_resources_not_found -
/// sn- prefix with invalid ID returns ResourceNotFoundException
#[tokio::test]
async fn test_create_access_log_subscription_sn_not_found() {
    let client = make_vpclattice_client().await;

    let err = client
        .create_access_log_subscription()
        .resource_identifier("sn-invalid-sn-id14")
        .destination_arn("arn:aws:s3:::my-log-bucket")
        .send()
        .await;
    assert!(
        err.is_err(),
        "create ALS with invalid sn- ID should fail with ResourceNotFoundException"
    );
}

/// Translated from moto test_create_access_log_subscription_invalid_resourceIdentifier -
/// non sn-/svc- prefix returns ValidationException
#[tokio::test]
async fn test_create_access_log_subscription_invalid_prefix() {
    let client = make_vpclattice_client().await;

    let err = client
        .create_access_log_subscription()
        .resource_identifier("invalid-sn-id1234")
        .destination_arn("arn:aws:s3:::my-log-bucket")
        .send()
        .await;
    assert!(
        err.is_err(),
        "create ALS with invalid prefix should fail with ValidationException"
    );
}

/// Translated from moto test_list_access_log_subscriptions - with maxResults
#[tokio::test]
async fn test_list_access_log_subscriptions_max_results() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("sn-for-als-list-test")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    // Create 5 subscriptions
    for i in 0..5 {
        client
            .create_access_log_subscription()
            .resource_identifier(&sn_id)
            .destination_arn(format!("arn:aws:s3:::my-bucket-{i}"))
            .send()
            .await
            .unwrap();
    }

    // List with maxResults=3
    let resp = client
        .list_access_log_subscriptions()
        .resource_identifier(&sn_id)
        .max_results(3)
        .send()
        .await
        .expect("list with maxResults should succeed");
    assert_eq!(resp.items().len(), 3);
    assert!(
        resp.items()
            .iter()
            .all(|s| s.resource_id() == sn_id.as_str())
    );

    // List without maxResults: all 5
    let resp_all = client
        .list_access_log_subscriptions()
        .resource_identifier(&sn_id)
        .send()
        .await
        .expect("list all should succeed");
    assert_eq!(resp_all.items().len(), 5);
}

/// Translated from moto test_list_access_log_subscriptions - different resource isolation
#[tokio::test]
async fn test_list_access_log_subscriptions_resource_isolation() {
    let client = make_vpclattice_client().await;

    let sn1 = client
        .create_service_network()
        .name("sn-isolation-1")
        .send()
        .await
        .unwrap();
    let sn1_id = sn1.id().unwrap().to_string();

    let sn2 = client
        .create_service_network()
        .name("sn-isolation-2")
        .send()
        .await
        .unwrap();
    let sn2_id = sn2.id().unwrap().to_string();

    // Create 2 subs for sn1, 1 sub for sn2
    for i in 0..2 {
        client
            .create_access_log_subscription()
            .resource_identifier(&sn1_id)
            .destination_arn(format!("arn:aws:s3:::sn1-bucket-{i}"))
            .send()
            .await
            .unwrap();
    }
    client
        .create_access_log_subscription()
        .resource_identifier(&sn2_id)
        .destination_arn("arn:aws:s3:::sn2-bucket")
        .send()
        .await
        .unwrap();

    // sn1 should have 2 subscriptions
    let sn1_subs = client
        .list_access_log_subscriptions()
        .resource_identifier(&sn1_id)
        .send()
        .await
        .unwrap();
    assert_eq!(sn1_subs.items().len(), 2);

    // sn2 should have 1 subscription
    let sn2_subs = client
        .list_access_log_subscriptions()
        .resource_identifier(&sn2_id)
        .send()
        .await
        .unwrap();
    assert_eq!(sn2_subs.items().len(), 1);
}

// ── Service tests ────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_service() {
    let client = make_vpclattice_client().await;

    let create_resp = client
        .create_service()
        .name("test-service")
        .send()
        .await
        .expect("create_service should succeed");

    assert_eq!(create_resp.name(), Some("test-service"));
    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());

    let id = create_resp.id().unwrap().to_string();

    let get_resp = client
        .get_service()
        .service_identifier(&id)
        .send()
        .await
        .expect("get_service should succeed");

    assert_eq!(get_resp.name(), Some("test-service"));
    assert_eq!(get_resp.id(), Some(id.as_str()));
}

#[tokio::test]
async fn test_delete_service() {
    let client = make_vpclattice_client().await;

    let create_resp = client
        .create_service()
        .name("delete-svc")
        .send()
        .await
        .unwrap();

    let id = create_resp.id().unwrap().to_string();

    client
        .delete_service()
        .service_identifier(&id)
        .send()
        .await
        .expect("delete_service should succeed");

    let result = client.get_service().service_identifier(&id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_services() {
    let client = make_vpclattice_client().await;

    for name in ["svc1", "svc2"] {
        client.create_service().name(name).send().await.unwrap();
    }

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");

    assert_eq!(resp.items().len(), 2);
}

// ── Auth policy tests ─────────────────────────────────────────────────

#[tokio::test]
async fn test_put_get_delete_auth_policy() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("policy-net")
        .send()
        .await
        .unwrap();

    let sn_id = sn.id().unwrap();

    let put_resp = client
        .put_auth_policy()
        .resource_identifier(sn_id)
        .policy("{\"Version\":\"2012-10-17\"}")
        .send()
        .await
        .expect("put_auth_policy should succeed");

    assert!(put_resp.policy().is_some());

    let get_resp = client
        .get_auth_policy()
        .resource_identifier(sn_id)
        .send()
        .await
        .expect("get_auth_policy should succeed");

    assert_eq!(get_resp.policy(), Some("{\"Version\":\"2012-10-17\"}"));

    client
        .delete_auth_policy()
        .resource_identifier(sn_id)
        .send()
        .await
        .expect("delete_auth_policy should succeed");

    let result = client
        .get_auth_policy()
        .resource_identifier(sn_id)
        .send()
        .await;
    assert!(result.is_err());
}

// ── Resource policy tests ─────────────────────────────────────────────

#[tokio::test]
async fn test_put_get_delete_resource_policy() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("res-policy-net")
        .send()
        .await
        .unwrap();

    let sn_arn = sn.arn().unwrap();

    client
        .put_resource_policy()
        .resource_arn(sn_arn)
        .policy("{\"Version\":\"2012-10-17\"}")
        .send()
        .await
        .expect("put_resource_policy should succeed");

    let get_resp = client
        .get_resource_policy()
        .resource_arn(sn_arn)
        .send()
        .await
        .expect("get_resource_policy should succeed");

    assert_eq!(get_resp.policy(), Some("{\"Version\":\"2012-10-17\"}"));

    client
        .delete_resource_policy()
        .resource_arn(sn_arn)
        .send()
        .await
        .expect("delete_resource_policy should succeed");
}

// ── Tags tests ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_tag_untag_list_tags() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("tagged-net")
        .send()
        .await
        .unwrap();

    let sn_arn = sn.arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&sn_arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&sn_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().expect("tags should be present");
    assert!(tags.contains_key("env"));
    assert!(tags.contains_key("team"));

    client
        .untag_resource()
        .resource_arn(&sn_arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&sn_arn)
        .send()
        .await
        .unwrap();

    let tags2 = list_resp2.tags().expect("tags should be present");
    assert!(!tags2.contains_key("team"));
    assert!(tags2.contains_key("env"));
}

// ── UpdateAccessLogSubscription test ─────────────────────────────────

#[tokio::test]
async fn test_update_access_log_subscription() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("update-als-net")
        .send()
        .await
        .unwrap();

    let sn_id = sn.id().unwrap().to_string();

    let als = client
        .create_access_log_subscription()
        .resource_identifier(&sn_id)
        .destination_arn("arn:aws:logs:us-east-1:123456789012:log-group:original")
        .send()
        .await
        .unwrap();

    let als_id = als.id().to_string();

    let update_resp = client
        .update_access_log_subscription()
        .access_log_subscription_identifier(&als_id)
        .destination_arn("arn:aws:logs:us-east-1:123456789012:log-group:updated")
        .send()
        .await
        .expect("update_access_log_subscription should succeed");

    assert_eq!(
        update_resp.destination_arn(),
        "arn:aws:logs:us-east-1:123456789012:log-group:updated"
    );

    let get_resp = client
        .get_access_log_subscription()
        .access_log_subscription_identifier(&als_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        get_resp.destination_arn(),
        "arn:aws:logs:us-east-1:123456789012:log-group:updated"
    );
}

// ── UpdateService tests ───────────────────────────────────────────────

#[tokio::test]
async fn test_update_service() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("update-svc")
        .auth_type(aws_sdk_vpclattice::types::AuthType::None)
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let update_resp = client
        .update_service()
        .service_identifier(&svc_id)
        .auth_type(aws_sdk_vpclattice::types::AuthType::AwsIam)
        .send()
        .await
        .expect("update_service should succeed");

    assert_eq!(update_resp.id(), Some(svc_id.as_str()));
    assert_eq!(
        update_resp.auth_type(),
        Some(&aws_sdk_vpclattice::types::AuthType::AwsIam)
    );

    let get_resp = client
        .get_service()
        .service_identifier(&svc_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.auth_type(),
        Some(&aws_sdk_vpclattice::types::AuthType::AwsIam)
    );
}

#[tokio::test]
async fn test_update_service_not_found() {
    let client = make_vpclattice_client().await;

    let result = client
        .update_service()
        .service_identifier("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── UpdateServiceNetwork tests ────────────────────────────────────────

#[tokio::test]
async fn test_update_service_network() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("update-sn")
        .auth_type(aws_sdk_vpclattice::types::AuthType::None)
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    let update_resp = client
        .update_service_network()
        .service_network_identifier(&sn_id)
        .auth_type(aws_sdk_vpclattice::types::AuthType::AwsIam)
        .send()
        .await
        .expect("update_service_network should succeed");

    assert_eq!(update_resp.id(), Some(sn_id.as_str()));
    assert_eq!(
        update_resp.auth_type(),
        Some(&aws_sdk_vpclattice::types::AuthType::AwsIam)
    );
}

// ── UpdateTargetGroup tests ───────────────────────────────────────────

#[tokio::test]
async fn test_update_target_group() {
    let client = make_vpclattice_client().await;

    let tg = client
        .create_target_group()
        .name("update-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    let update_resp = client
        .update_target_group()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .expect("update_target_group should succeed");

    assert_eq!(update_resp.id(), Some(tg_id.as_str()));
    assert_eq!(update_resp.name(), Some("update-tg"));
}

// ── UpdateServiceNetworkVpcAssociation tests ──────────────────────────

#[tokio::test]
async fn test_update_service_network_vpc_association() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("update-snva-net")
        .send()
        .await
        .unwrap();

    let assoc = client
        .create_service_network_vpc_association()
        .service_network_identifier(sn.id().unwrap())
        .vpc_identifier("vpc-abc123")
        .security_group_ids("sg-old")
        .send()
        .await
        .unwrap();
    let assoc_id = assoc.id().unwrap().to_string();

    let update_resp = client
        .update_service_network_vpc_association()
        .service_network_vpc_association_identifier(&assoc_id)
        .security_group_ids("sg-new1")
        .security_group_ids("sg-new2")
        .send()
        .await
        .expect("update_service_network_vpc_association should succeed");

    assert_eq!(update_resp.id(), Some(assoc_id.as_str()));
    assert_eq!(update_resp.security_group_ids().len(), 2);
}

// ── Listener tests ────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_listener() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("listener-test-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let default_action = aws_sdk_vpclattice::types::RuleAction::FixedResponse(
        aws_sdk_vpclattice::types::FixedResponseAction::builder()
            .status_code(404)
            .build()
            .unwrap(),
    );

    let create_resp = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("test-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(default_action)
        .send()
        .await
        .expect("create_listener should succeed");

    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());
    assert_eq!(create_resp.name(), Some("test-listener"));
    assert_eq!(create_resp.service_id(), Some(svc_id.as_str()));

    let listener_id = create_resp.id().unwrap().to_string();

    let get_resp = client
        .get_listener()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .send()
        .await
        .expect("get_listener should succeed");

    assert_eq!(get_resp.id(), Some(listener_id.as_str()));
    assert_eq!(get_resp.name(), Some("test-listener"));
    assert_eq!(get_resp.service_id(), Some(svc_id.as_str()));
}

#[tokio::test]
async fn test_list_listeners() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("list-listener-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let default_action = aws_sdk_vpclattice::types::RuleAction::FixedResponse(
        aws_sdk_vpclattice::types::FixedResponseAction::builder()
            .status_code(200)
            .build()
            .unwrap(),
    );

    for name in ["listener-a", "listener-b"] {
        client
            .create_listener()
            .service_identifier(&svc_id)
            .name(name)
            .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
            .port(80)
            .default_action(default_action.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_listeners()
        .service_identifier(&svc_id)
        .send()
        .await
        .expect("list_listeners should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_delete_listener() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("del-listener-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let default_action = aws_sdk_vpclattice::types::RuleAction::FixedResponse(
        aws_sdk_vpclattice::types::FixedResponseAction::builder()
            .status_code(200)
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("delete-me")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(default_action)
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    client
        .delete_listener()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .send()
        .await
        .expect("delete_listener should succeed");

    let result = client
        .get_listener()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .send()
        .await;
    assert!(result.is_err(), "listener should be gone after delete");
}

#[tokio::test]
async fn test_get_listener_not_found() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("listener-notfound-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let result = client
        .get_listener()
        .service_identifier(&svc_id)
        .listener_identifier("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── Rule tests ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_rule() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("rule-test-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let tg = client
        .create_target_group()
        .name("rule-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    let default_action = aws_sdk_vpclattice::types::RuleAction::Forward(
        aws_sdk_vpclattice::types::ForwardAction::builder()
            .target_groups(
                aws_sdk_vpclattice::types::WeightedTargetGroup::builder()
                    .target_group_identifier(&tg_id)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("rule-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(default_action.clone())
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    let rule_action = aws_sdk_vpclattice::types::RuleAction::Forward(
        aws_sdk_vpclattice::types::ForwardAction::builder()
            .target_groups(
                aws_sdk_vpclattice::types::WeightedTargetGroup::builder()
                    .target_group_identifier(&tg_id)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );

    let create_resp = client
        .create_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .name("path-rule")
        .priority(10)
        .action(rule_action)
        .r#match(aws_sdk_vpclattice::types::RuleMatch::HttpMatch(
            aws_sdk_vpclattice::types::HttpMatch::builder().build(),
        ))
        .send()
        .await
        .expect("create_rule should succeed");

    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());
    assert_eq!(create_resp.name(), Some("path-rule"));
    assert_eq!(create_resp.priority(), Some(10));

    let rule_id = create_resp.id().unwrap().to_string();

    let get_resp = client
        .get_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .rule_identifier(&rule_id)
        .send()
        .await
        .expect("get_rule should succeed");

    assert_eq!(get_resp.id(), Some(rule_id.as_str()));
    assert_eq!(get_resp.name(), Some("path-rule"));
    assert_eq!(get_resp.priority(), Some(10));
}

#[tokio::test]
async fn test_list_rules() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("list-rules-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let tg = client
        .create_target_group()
        .name("list-rules-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    let forward_action = aws_sdk_vpclattice::types::RuleAction::Forward(
        aws_sdk_vpclattice::types::ForwardAction::builder()
            .target_groups(
                aws_sdk_vpclattice::types::WeightedTargetGroup::builder()
                    .target_group_identifier(&tg_id)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("list-rules-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(forward_action.clone())
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    for priority in [10, 20, 30] {
        client
            .create_rule()
            .service_identifier(&svc_id)
            .listener_identifier(&listener_id)
            .name(format!("rule-{priority}"))
            .priority(priority)
            .action(forward_action.clone())
            .r#match(aws_sdk_vpclattice::types::RuleMatch::HttpMatch(
                aws_sdk_vpclattice::types::HttpMatch::builder().build(),
            ))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_rules()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .send()
        .await
        .expect("list_rules should succeed");

    assert_eq!(resp.items().len(), 3);
}

#[tokio::test]
async fn test_delete_rule() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("del-rule-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let tg = client
        .create_target_group()
        .name("del-rule-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    let forward_action = aws_sdk_vpclattice::types::RuleAction::Forward(
        aws_sdk_vpclattice::types::ForwardAction::builder()
            .target_groups(
                aws_sdk_vpclattice::types::WeightedTargetGroup::builder()
                    .target_group_identifier(&tg_id)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("del-rule-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(forward_action.clone())
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    let rule = client
        .create_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .name("delete-rule")
        .priority(5)
        .action(forward_action)
        .r#match(aws_sdk_vpclattice::types::RuleMatch::HttpMatch(
            aws_sdk_vpclattice::types::HttpMatch::builder().build(),
        ))
        .send()
        .await
        .unwrap();
    let rule_id = rule.id().unwrap().to_string();

    client
        .delete_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .rule_identifier(&rule_id)
        .send()
        .await
        .expect("delete_rule should succeed");

    let result = client
        .get_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .rule_identifier(&rule_id)
        .send()
        .await;
    assert!(result.is_err(), "rule should be gone after delete");
}

#[tokio::test]
async fn test_create_rule_invalid_listener() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("rule-invalid-listener-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let rule_action = aws_sdk_vpclattice::types::RuleAction::FixedResponse(
        aws_sdk_vpclattice::types::FixedResponseAction::builder()
            .status_code(200)
            .build()
            .unwrap(),
    );

    let result = client
        .create_rule()
        .service_identifier(&svc_id)
        .listener_identifier("nonexistent-listener")
        .name("bad-rule")
        .priority(1)
        .action(rule_action)
        .r#match(aws_sdk_vpclattice::types::RuleMatch::HttpMatch(
            aws_sdk_vpclattice::types::HttpMatch::builder().build(),
        ))
        .send()
        .await;
    assert!(
        result.is_err(),
        "creating rule with invalid listener should fail"
    );
}

// ── UpdateListener tests ──────────────────────────────────────────────

#[tokio::test]
async fn test_update_listener() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("update-listener-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let default_action = aws_sdk_vpclattice::types::RuleAction::FixedResponse(
        aws_sdk_vpclattice::types::FixedResponseAction::builder()
            .status_code(200)
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("updatable-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(default_action)
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    let new_action = aws_sdk_vpclattice::types::RuleAction::FixedResponse(
        aws_sdk_vpclattice::types::FixedResponseAction::builder()
            .status_code(404)
            .build()
            .unwrap(),
    );

    let update_resp = client
        .update_listener()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .default_action(new_action)
        .send()
        .await
        .expect("update_listener should succeed");

    assert_eq!(update_resp.id(), Some(listener_id.as_str()));
    assert_eq!(update_resp.name(), Some("updatable-listener"));
}

// ── UpdateRule tests ──────────────────────────────────────────────────

#[tokio::test]
async fn test_update_rule() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("update-rule-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let tg = client
        .create_target_group()
        .name("update-rule-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    let forward_action = aws_sdk_vpclattice::types::RuleAction::Forward(
        aws_sdk_vpclattice::types::ForwardAction::builder()
            .target_groups(
                aws_sdk_vpclattice::types::WeightedTargetGroup::builder()
                    .target_group_identifier(&tg_id)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("update-rule-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(forward_action.clone())
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    let rule = client
        .create_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .name("update-me")
        .priority(10)
        .action(forward_action.clone())
        .r#match(aws_sdk_vpclattice::types::RuleMatch::HttpMatch(
            aws_sdk_vpclattice::types::HttpMatch::builder().build(),
        ))
        .send()
        .await
        .unwrap();
    let rule_id = rule.id().unwrap().to_string();

    let update_resp = client
        .update_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .rule_identifier(&rule_id)
        .priority(20)
        .send()
        .await
        .expect("update_rule should succeed");

    assert_eq!(update_resp.id(), Some(rule_id.as_str()));
    assert_eq!(update_resp.priority(), Some(20));
}

// ── BatchUpdateRule tests ─────────────────────────────────────────────

#[tokio::test]
async fn test_batch_update_rule() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("batch-rule-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let tg = client
        .create_target_group()
        .name("batch-rule-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    let forward_action = aws_sdk_vpclattice::types::RuleAction::Forward(
        aws_sdk_vpclattice::types::ForwardAction::builder()
            .target_groups(
                aws_sdk_vpclattice::types::WeightedTargetGroup::builder()
                    .target_group_identifier(&tg_id)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("batch-rule-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(forward_action.clone())
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    let rule1 = client
        .create_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .name("batch-r1")
        .priority(10)
        .action(forward_action.clone())
        .r#match(aws_sdk_vpclattice::types::RuleMatch::HttpMatch(
            aws_sdk_vpclattice::types::HttpMatch::builder().build(),
        ))
        .send()
        .await
        .unwrap();
    let rule1_id = rule1.id().unwrap().to_string();

    let rule2 = client
        .create_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .name("batch-r2")
        .priority(20)
        .action(forward_action.clone())
        .r#match(aws_sdk_vpclattice::types::RuleMatch::HttpMatch(
            aws_sdk_vpclattice::types::HttpMatch::builder().build(),
        ))
        .send()
        .await
        .unwrap();
    let rule2_id = rule2.id().unwrap().to_string();

    let batch_resp = client
        .batch_update_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .rules(
            aws_sdk_vpclattice::types::RuleUpdate::builder()
                .rule_identifier(&rule1_id)
                .priority(30)
                .build()
                .unwrap(),
        )
        .rules(
            aws_sdk_vpclattice::types::RuleUpdate::builder()
                .rule_identifier(&rule2_id)
                .priority(40)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("batch_update_rule should succeed");

    assert_eq!(batch_resp.successful().len(), 2);
    assert!(batch_resp.unsuccessful().is_empty());
}

// ── ResourceConfiguration tests ───────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_resource_configuration() {
    let client = make_vpclattice_client().await;

    let create_resp = client
        .create_resource_configuration()
        .name("test-rc")
        .r#type(aws_sdk_vpclattice::types::ResourceConfigurationType::Single)
        .send()
        .await
        .expect("create_resource_configuration should succeed");

    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());
    assert_eq!(create_resp.name(), Some("test-rc"));

    let rc_id = create_resp.id().unwrap().to_string();

    let get_resp = client
        .get_resource_configuration()
        .resource_configuration_identifier(&rc_id)
        .send()
        .await
        .expect("get_resource_configuration should succeed");

    assert_eq!(get_resp.id(), Some(rc_id.as_str()));
    assert_eq!(get_resp.name(), Some("test-rc"));
}

#[tokio::test]
async fn test_delete_resource_configuration() {
    let client = make_vpclattice_client().await;

    let rc = client
        .create_resource_configuration()
        .name("delete-me-rc")
        .r#type(aws_sdk_vpclattice::types::ResourceConfigurationType::Single)
        .send()
        .await
        .unwrap();
    let rc_id = rc.id().unwrap().to_string();

    client
        .delete_resource_configuration()
        .resource_configuration_identifier(&rc_id)
        .send()
        .await
        .expect("delete_resource_configuration should succeed");

    let result = client
        .get_resource_configuration()
        .resource_configuration_identifier(&rc_id)
        .send()
        .await;
    assert!(
        result.is_err(),
        "resource configuration should be gone after delete"
    );
}

#[tokio::test]
async fn test_list_resource_configurations() {
    let client = make_vpclattice_client().await;

    for name in ["rc-a", "rc-b"] {
        client
            .create_resource_configuration()
            .name(name)
            .r#type(aws_sdk_vpclattice::types::ResourceConfigurationType::Single)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_resource_configurations()
        .send()
        .await
        .expect("list_resource_configurations should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_update_resource_configuration() {
    let client = make_vpclattice_client().await;

    let rc = client
        .create_resource_configuration()
        .name("update-rc")
        .r#type(aws_sdk_vpclattice::types::ResourceConfigurationType::Single)
        .port_ranges("80")
        .send()
        .await
        .unwrap();
    let rc_id = rc.id().unwrap().to_string();

    let update_resp = client
        .update_resource_configuration()
        .resource_configuration_identifier(&rc_id)
        .port_ranges("443")
        .send()
        .await
        .expect("update_resource_configuration should succeed");

    assert_eq!(update_resp.id(), Some(rc_id.as_str()));
}

// ── ResourceGateway tests ─────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_resource_gateway() {
    let client = make_vpclattice_client().await;

    let create_resp = client
        .create_resource_gateway()
        .name("test-rg")
        .vpc_identifier("vpc-12345")
        .subnet_ids("subnet-abc")
        .send()
        .await
        .expect("create_resource_gateway should succeed");

    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());
    assert_eq!(create_resp.name(), Some("test-rg"));

    let rg_id = create_resp.id().unwrap().to_string();

    let get_resp = client
        .get_resource_gateway()
        .resource_gateway_identifier(&rg_id)
        .send()
        .await
        .expect("get_resource_gateway should succeed");

    assert_eq!(get_resp.id(), Some(rg_id.as_str()));
    assert_eq!(get_resp.name(), Some("test-rg"));
}

#[tokio::test]
async fn test_delete_resource_gateway() {
    let client = make_vpclattice_client().await;

    let rg = client
        .create_resource_gateway()
        .name("delete-me-rg")
        .vpc_identifier("vpc-12345")
        .subnet_ids("subnet-abc")
        .send()
        .await
        .unwrap();
    let rg_id = rg.id().unwrap().to_string();

    client
        .delete_resource_gateway()
        .resource_gateway_identifier(&rg_id)
        .send()
        .await
        .expect("delete_resource_gateway should succeed");

    let result = client
        .get_resource_gateway()
        .resource_gateway_identifier(&rg_id)
        .send()
        .await;
    assert!(
        result.is_err(),
        "resource gateway should be gone after delete"
    );
}

#[tokio::test]
async fn test_list_resource_gateways() {
    let client = make_vpclattice_client().await;

    for name in ["rg-a", "rg-b"] {
        client
            .create_resource_gateway()
            .name(name)
            .vpc_identifier("vpc-12345")
            .subnet_ids("subnet-abc")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_resource_gateways()
        .send()
        .await
        .expect("list_resource_gateways should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_update_resource_gateway() {
    let client = make_vpclattice_client().await;

    let rg = client
        .create_resource_gateway()
        .name("update-rg")
        .vpc_identifier("vpc-12345")
        .subnet_ids("subnet-abc")
        .security_group_ids("sg-111")
        .send()
        .await
        .unwrap();
    let rg_id = rg.id().unwrap().to_string();

    let update_resp = client
        .update_resource_gateway()
        .resource_gateway_identifier(&rg_id)
        .security_group_ids("sg-222")
        .send()
        .await
        .expect("update_resource_gateway should succeed");

    assert_eq!(update_resp.id(), Some(rg_id.as_str()));
}

// ── ServiceNetworkResourceAssociation tests ───────────────────────────

#[tokio::test]
async fn test_create_and_get_service_network_resource_association() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("snra-test-sn")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    let rc = client
        .create_resource_configuration()
        .name("snra-test-rc")
        .r#type(aws_sdk_vpclattice::types::ResourceConfigurationType::Single)
        .send()
        .await
        .unwrap();
    let rc_id = rc.id().unwrap().to_string();

    let create_resp = client
        .create_service_network_resource_association()
        .service_network_identifier(&sn_id)
        .resource_configuration_identifier(&rc_id)
        .send()
        .await
        .expect("create_service_network_resource_association should succeed");

    assert!(create_resp.id().is_some());
    assert!(create_resp.arn().is_some());

    let assoc_id = create_resp.id().unwrap().to_string();

    let get_resp = client
        .get_service_network_resource_association()
        .service_network_resource_association_identifier(&assoc_id)
        .send()
        .await
        .expect("get_service_network_resource_association should succeed");

    assert_eq!(get_resp.id(), Some(assoc_id.as_str()));
}

#[tokio::test]
async fn test_delete_service_network_resource_association() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("snra-del-sn")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    let rc = client
        .create_resource_configuration()
        .name("snra-del-rc")
        .r#type(aws_sdk_vpclattice::types::ResourceConfigurationType::Single)
        .send()
        .await
        .unwrap();
    let rc_id = rc.id().unwrap().to_string();

    let assoc = client
        .create_service_network_resource_association()
        .service_network_identifier(&sn_id)
        .resource_configuration_identifier(&rc_id)
        .send()
        .await
        .unwrap();
    let assoc_id = assoc.id().unwrap().to_string();

    client
        .delete_service_network_resource_association()
        .service_network_resource_association_identifier(&assoc_id)
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_service_network_resource_association()
        .service_network_resource_association_identifier(&assoc_id)
        .send()
        .await;
    assert!(result.is_err(), "association should be gone after delete");
}

#[tokio::test]
async fn test_list_service_network_resource_associations() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("snra-list-sn")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    let rc = client
        .create_resource_configuration()
        .name("snra-list-rc")
        .r#type(aws_sdk_vpclattice::types::ResourceConfigurationType::Single)
        .send()
        .await
        .unwrap();
    let rc_id = rc.id().unwrap().to_string();

    client
        .create_service_network_resource_association()
        .service_network_identifier(&sn_id)
        .resource_configuration_identifier(&rc_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_service_network_resource_associations()
        .service_network_identifier(&sn_id)
        .send()
        .await
        .expect("list_service_network_resource_associations should succeed");

    assert_eq!(resp.items().len(), 1);
}

// ── ResourceEndpointAssociation tests (stubs) ─────────────────────────

#[tokio::test]
async fn test_list_resource_endpoint_associations() {
    let client = make_vpclattice_client().await;

    let resp = client
        .list_resource_endpoint_associations()
        .resource_configuration_identifier("rc-dummy")
        .send()
        .await
        .expect("list_resource_endpoint_associations should succeed");

    assert!(resp.items().is_empty());
}

// ── ServiceNetworkVpcEndpointAssociation tests (stub) ─────────────────

#[tokio::test]
async fn test_list_service_network_vpc_endpoint_associations() {
    let client = make_vpclattice_client().await;

    let resp = client
        .list_service_network_vpc_endpoint_associations()
        .service_network_identifier("sn-dummy")
        .send()
        .await
        .expect("list_service_network_vpc_endpoint_associations should succeed");

    assert!(resp.items().is_empty());
}

// ── Coverage for FIX(terraform-e2e) handler fixes ─────────────────────

/// FIX(terraform-e2e): tag_resource / untag_resource / list_tags_for_resource must work
/// on listener ARNs. Previously only service-network, service, and target-group ARNs
/// were handled, causing TagResourceNotFound for listeners.
#[tokio::test]
async fn test_tag_untag_list_tags_on_listener_arn() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("tag-listener-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let default_action = aws_sdk_vpclattice::types::RuleAction::FixedResponse(
        aws_sdk_vpclattice::types::FixedResponseAction::builder()
            .status_code(200)
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("tag-test-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(default_action)
        .send()
        .await
        .unwrap();
    let listener_arn = listener.arn().unwrap().to_string();

    // tag_resource on listener ARN should succeed
    client
        .tag_resource()
        .resource_arn(&listener_arn)
        .tags("env", "prod")
        .tags("team", "infra")
        .send()
        .await
        .expect("tag_resource on listener ARN should succeed");

    // list_tags_for_resource should return the tags we just set
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&listener_arn)
        .send()
        .await
        .expect("list_tags_for_resource on listener ARN should succeed");

    let tags = tags_resp.tags().expect("tags should be present");
    assert_eq!(tags.get("env"), Some(&"prod".to_string()));
    assert_eq!(tags.get("team"), Some(&"infra".to_string()));

    // untag_resource should remove the specified key
    client
        .untag_resource()
        .resource_arn(&listener_arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource on listener ARN should succeed");

    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&listener_arn)
        .send()
        .await
        .unwrap();

    let tags2 = tags_resp2.tags().expect("tags should be present");
    assert!(!tags2.contains_key("team"), "untagged key should be gone");
    assert!(tags2.contains_key("env"), "remaining key should persist");
}

/// FIX(terraform-e2e): tag_resource / untag_resource / list_tags_for_resource must work
/// on ServiceNetworkServiceAssociation ARNs.
#[tokio::test]
async fn test_tag_untag_list_tags_on_sn_service_association_arn() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("tag-assoc-net")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    let assoc = client
        .create_service_network_service_association()
        .service_network_identifier(&sn_id)
        .service_identifier("svc-tagassoctest1234")
        .send()
        .await
        .unwrap();
    let assoc_arn = assoc.arn().unwrap().to_string();

    // tag_resource on association ARN should succeed
    client
        .tag_resource()
        .resource_arn(&assoc_arn)
        .tags("purpose", "testing")
        .send()
        .await
        .expect("tag_resource on sn-service association ARN should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&assoc_arn)
        .send()
        .await
        .expect("list_tags_for_resource on sn-service association ARN should succeed");

    let tags = tags_resp.tags().expect("tags should be present");
    assert_eq!(tags.get("purpose"), Some(&"testing".to_string()));

    // untag
    client
        .untag_resource()
        .resource_arn(&assoc_arn)
        .tag_keys("purpose")
        .send()
        .await
        .expect("untag_resource on sn-service association ARN should succeed");

    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&assoc_arn)
        .send()
        .await
        .unwrap();

    // After untagging the only key, the tags map may be None or empty
    let tags2 = tags_resp2.tags();
    assert!(
        tags2.is_none() || !tags2.unwrap().contains_key("purpose"),
        "untagged key should be removed"
    );
}

/// FIX(terraform-e2e): tag_resource / untag_resource / list_tags_for_resource must work
/// on ServiceNetworkVpcAssociation ARNs.
#[tokio::test]
async fn test_tag_untag_list_tags_on_sn_vpc_association_arn() {
    let client = make_vpclattice_client().await;

    let sn = client
        .create_service_network()
        .name("tag-vpc-assoc-net")
        .send()
        .await
        .unwrap();
    let sn_id = sn.id().unwrap().to_string();

    let assoc = client
        .create_service_network_vpc_association()
        .service_network_identifier(&sn_id)
        .vpc_identifier("vpc-tagtest123")
        .send()
        .await
        .unwrap();
    let assoc_arn = assoc.arn().unwrap().to_string();

    // tag_resource on VPC association ARN should succeed
    client
        .tag_resource()
        .resource_arn(&assoc_arn)
        .tags("cost-centre", "ops")
        .send()
        .await
        .expect("tag_resource on sn-vpc association ARN should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&assoc_arn)
        .send()
        .await
        .expect("list_tags_for_resource on sn-vpc association ARN should succeed");

    let tags = tags_resp.tags().expect("tags should be present");
    assert_eq!(tags.get("cost-centre"), Some(&"ops".to_string()));

    // untag
    client
        .untag_resource()
        .resource_arn(&assoc_arn)
        .tag_keys("cost-centre")
        .send()
        .await
        .expect("untag_resource on sn-vpc association ARN should succeed");

    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&assoc_arn)
        .send()
        .await
        .unwrap();

    // After untagging the only key, the tags map may be None or empty
    let tags2 = tags_resp2.tags();
    assert!(
        tags2.is_none() || !tags2.unwrap().contains_key("cost-centre"),
        "untagged key should be removed"
    );
}

/// FIX(terraform-e2e): rules do not carry tags in the mock model, but the Terraform
/// provider calls ListTagsForResource on rule ARNs after create. The mock must return
/// empty tags (not 404) for rule ARNs. tag_resource/untag_resource should also silently
/// accept rule ARNs without error.
#[tokio::test]
async fn test_list_tags_for_rule_returns_empty_not_404() {
    let client = make_vpclattice_client().await;

    // Set up service -> listener -> rule
    let svc = client
        .create_service()
        .name("rule-tag-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let tg = client
        .create_target_group()
        .name("rule-tag-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    let forward_action = aws_sdk_vpclattice::types::RuleAction::Forward(
        aws_sdk_vpclattice::types::ForwardAction::builder()
            .target_groups(
                aws_sdk_vpclattice::types::WeightedTargetGroup::builder()
                    .target_group_identifier(&tg_id)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("rule-tag-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(forward_action.clone())
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    let rule = client
        .create_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .name("tag-rule")
        .priority(10)
        .action(forward_action)
        .r#match(aws_sdk_vpclattice::types::RuleMatch::HttpMatch(
            aws_sdk_vpclattice::types::HttpMatch::builder().build(),
        ))
        .send()
        .await
        .unwrap();
    let rule_arn = rule.arn().unwrap().to_string();

    // ListTagsForResource on a rule ARN should return empty tags, not 404
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&rule_arn)
        .send()
        .await
        .expect("list_tags_for_resource on rule ARN should succeed (not 404)");

    let tags = tags_resp.tags();
    assert!(
        tags.is_none() || tags.unwrap().is_empty(),
        "rule tags should be empty or absent"
    );

    // tag_resource on rule ARN should silently succeed (no-op)
    client
        .tag_resource()
        .resource_arn(&rule_arn)
        .tags("ignored", "value")
        .send()
        .await
        .expect("tag_resource on rule ARN should silently succeed");

    // untag_resource on rule ARN should silently succeed (no-op)
    client
        .untag_resource()
        .resource_arn(&rule_arn)
        .tag_keys("ignored")
        .send()
        .await
        .expect("untag_resource on rule ARN should silently succeed");

    // Tags should still be empty after the no-op tag/untag
    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&rule_arn)
        .send()
        .await
        .unwrap();

    let tags2 = tags_resp2.tags();
    assert!(
        tags2.is_none() || tags2.unwrap().is_empty(),
        "rule tags should remain empty or absent"
    );
}

/// FIX(terraform-e2e): Terraform AWS provider sets the listener resource id as a
/// composite "{service_id}/{listener_id}" string. When that composite is subsequently
/// used as listener_identifier (e.g. in aws_vpclattice_listener_rule), resolve_listener
/// must strip the service prefix and look up by the plain listener ID.
#[tokio::test]
async fn test_resolve_listener_with_composite_service_listener_id() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("composite-id-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let default_action = aws_sdk_vpclattice::types::RuleAction::FixedResponse(
        aws_sdk_vpclattice::types::FixedResponseAction::builder()
            .status_code(200)
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("composite-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(default_action)
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    // Simulate the composite identifier that Terraform sets: "svc-xxx/listener-yyy"
    let composite_id = format!("{}/{}", svc_id, listener_id);

    // get_listener with composite listener_identifier should resolve correctly
    let get_resp = client
        .get_listener()
        .service_identifier(&svc_id)
        .listener_identifier(&composite_id)
        .send()
        .await
        .expect("get_listener with composite svc-id/listener-id should succeed");

    assert_eq!(get_resp.id(), Some(listener_id.as_str()));
    assert_eq!(get_resp.name(), Some("composite-listener"));
}

/// FIX(terraform-e2e): create_rule must also accept the composite listener identifier
/// "svc-xxx/listener-yyy" that Terraform passes through.
#[tokio::test]
async fn test_create_rule_with_composite_listener_id() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("composite-rule-svc")
        .send()
        .await
        .unwrap();
    let svc_id = svc.id().unwrap().to_string();

    let tg = client
        .create_target_group()
        .name("composite-rule-tg")
        .r#type(aws_sdk_vpclattice::types::TargetGroupType::Instance)
        .send()
        .await
        .unwrap();
    let tg_id = tg.id().unwrap().to_string();

    let forward_action = aws_sdk_vpclattice::types::RuleAction::Forward(
        aws_sdk_vpclattice::types::ForwardAction::builder()
            .target_groups(
                aws_sdk_vpclattice::types::WeightedTargetGroup::builder()
                    .target_group_identifier(&tg_id)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );

    let listener = client
        .create_listener()
        .service_identifier(&svc_id)
        .name("composite-rule-listener")
        .protocol(aws_sdk_vpclattice::types::ListenerProtocol::Http)
        .port(80)
        .default_action(forward_action.clone())
        .send()
        .await
        .unwrap();
    let listener_id = listener.id().unwrap().to_string();

    // Use composite "svc-xxx/listener-yyy" as listener_identifier
    let composite_listener_id = format!("{}/{}", svc_id, listener_id);

    let rule = client
        .create_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&composite_listener_id)
        .name("composite-rule")
        .priority(20)
        .action(forward_action)
        .r#match(aws_sdk_vpclattice::types::RuleMatch::HttpMatch(
            aws_sdk_vpclattice::types::HttpMatch::builder().build(),
        ))
        .send()
        .await
        .expect("create_rule with composite listener identifier should succeed");

    assert!(rule.id().is_some());
    assert_eq!(rule.name(), Some("composite-rule"));
    assert_eq!(rule.priority(), Some(20));
}
