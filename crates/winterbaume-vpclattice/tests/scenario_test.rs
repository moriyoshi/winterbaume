//! Smoke tests for winterbaume VPC Lattice service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_vpclattice::config::BehaviorVersion;
use aws_sdk_vpclattice::types::{
    FixedResponseAction, ForwardAction, HttpMatch, ListenerProtocol, RuleAction, RuleMatch,
    TargetGroupConfig, TargetGroupProtocol, TargetGroupType, WeightedTargetGroup,
};
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

/// Scenario: end-to-end VPC Lattice service mesh.
///
/// A platform team wires up a complete service-mesh topology:
/// 1. CreateServiceNetwork — the mesh.
/// 2. CreateService — the application service joining the mesh.
/// 3. CreateTargetGroup — backends behind the service.
/// 4. CreateListener — HTTP/80 listener on the service, default-forwarding
///    to the target group.
/// 5. CreateRule — a path-based override rule on the listener.
/// 6. CreateServiceNetworkServiceAssociation — attach the service to the mesh.
/// 7. CreateServiceNetworkVpcAssociation — attach the mesh to a VPC.
/// 8. Tear down in reverse dependency order.
#[tokio::test]
async fn test_service_mesh_end_to_end() {
    let client = make_vpclattice_client().await;

    // 1. Service network.
    let sn = client
        .create_service_network()
        .name("orders-mesh")
        .send()
        .await
        .expect("create_service_network");
    let sn_id = sn.id().expect("sn id").to_string();
    assert_eq!(sn.name(), Some("orders-mesh"));

    // 2. Service.
    let svc = client
        .create_service()
        .name("orders-svc")
        .send()
        .await
        .expect("create_service");
    let svc_id = svc.id().expect("svc id").to_string();

    // 3. Target group.
    let tg = client
        .create_target_group()
        .name("orders-tg")
        .r#type(TargetGroupType::Instance)
        .config(
            TargetGroupConfig::builder()
                .port(80)
                .protocol(TargetGroupProtocol::Http)
                .vpc_identifier("vpc-meshtest")
                .build(),
        )
        .send()
        .await
        .expect("create_target_group");
    let tg_id = tg.id().expect("tg id").to_string();

    // 4. Listener — default action forwards to the target group.
    let forward_action = RuleAction::Forward(
        ForwardAction::builder()
            .target_groups(
                WeightedTargetGroup::builder()
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
        .name("http-80")
        .protocol(ListenerProtocol::Http)
        .port(80)
        .default_action(forward_action.clone())
        .send()
        .await
        .expect("create_listener");
    let listener_id = listener.id().expect("listener id").to_string();
    assert_eq!(listener.service_id(), Some(svc_id.as_str()));

    // 5. Path-rule override.
    let rule = client
        .create_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .name("orders-path")
        .priority(10)
        .action(forward_action.clone())
        .r#match(RuleMatch::HttpMatch(HttpMatch::builder().build()))
        .send()
        .await
        .expect("create_rule");
    let rule_id = rule.id().expect("rule id").to_string();
    assert_eq!(rule.name(), Some("orders-path"));

    // 6. Attach the service to the mesh.
    let svc_assoc = client
        .create_service_network_service_association()
        .service_network_identifier(&sn_id)
        .service_identifier(&svc_id)
        .send()
        .await
        .expect("create_service_network_service_association");
    let svc_assoc_id = svc_assoc.id().expect("svc assoc id").to_string();

    // 7. Attach the mesh to a VPC.
    let vpc_assoc = client
        .create_service_network_vpc_association()
        .service_network_identifier(&sn_id)
        .vpc_identifier("vpc-meshtest")
        .security_group_ids("sg-mesh")
        .send()
        .await
        .expect("create_service_network_vpc_association");
    let vpc_assoc_id = vpc_assoc.id().expect("vpc assoc id").to_string();

    // Spot-check the topology: svc association points back at our SN.
    let got_svc_assoc = client
        .get_service_network_service_association()
        .service_network_service_association_identifier(&svc_assoc_id)
        .send()
        .await
        .expect("get_service_network_service_association");
    assert_eq!(got_svc_assoc.service_network_id(), Some(sn_id.as_str()));
    assert_eq!(got_svc_assoc.id(), Some(svc_assoc_id.as_str()));

    let got_vpc_assoc = client
        .get_service_network_vpc_association()
        .service_network_vpc_association_identifier(&vpc_assoc_id)
        .send()
        .await
        .expect("get_service_network_vpc_association");
    assert_eq!(got_vpc_assoc.vpc_id(), Some("vpc-meshtest"));
    assert_eq!(got_vpc_assoc.service_network_id(), Some(sn_id.as_str()));

    // 8. Tear down in reverse order.
    client
        .delete_service_network_vpc_association()
        .service_network_vpc_association_identifier(&vpc_assoc_id)
        .send()
        .await
        .expect("delete_service_network_vpc_association");

    client
        .delete_service_network_service_association()
        .service_network_service_association_identifier(&svc_assoc_id)
        .send()
        .await
        .expect("delete_service_network_service_association");

    client
        .delete_rule()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .rule_identifier(&rule_id)
        .send()
        .await
        .expect("delete_rule");

    client
        .delete_listener()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .send()
        .await
        .expect("delete_listener");

    client
        .delete_target_group()
        .target_group_identifier(&tg_id)
        .send()
        .await
        .expect("delete_target_group");

    client
        .delete_service()
        .service_identifier(&svc_id)
        .send()
        .await
        .expect("delete_service");

    client
        .delete_service_network()
        .service_network_identifier(&sn_id)
        .send()
        .await
        .expect("delete_service_network");

    // Spot-check teardown landed.
    let after_listener = client
        .get_listener()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .send()
        .await;
    assert!(after_listener.is_err(), "listener should be gone");

    let after_sn = client
        .get_service_network()
        .service_network_identifier(&sn_id)
        .send()
        .await;
    assert!(after_sn.is_err(), "service network should be gone");
}

/// Scenario: switching a listener's default action via fixed-response.
///
/// A platform team initially fronts a service with a target group, then
/// puts the listener into "maintenance mode" by swapping its default
/// action to a fixed 503 response. Both states must be observable.
#[tokio::test]
async fn test_listener_maintenance_mode_swap() {
    let client = make_vpclattice_client().await;

    let svc = client
        .create_service()
        .name("billing-svc")
        .send()
        .await
        .expect("create_service");
    let svc_id = svc.id().unwrap().to_string();

    let tg = client
        .create_target_group()
        .name("billing-tg")
        .r#type(TargetGroupType::Lambda)
        .send()
        .await
        .expect("create_target_group");
    let tg_id = tg.id().unwrap().to_string();

    // Initial listener forwards to the target group.
    let forward_action = RuleAction::Forward(
        ForwardAction::builder()
            .target_groups(
                WeightedTargetGroup::builder()
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
        .name("front-door")
        .protocol(ListenerProtocol::Http)
        .port(80)
        .default_action(forward_action)
        .send()
        .await
        .expect("create_listener");
    let listener_id = listener.id().unwrap().to_string();

    // Maintenance mode — flip default to 503.
    let maintenance = RuleAction::FixedResponse(
        FixedResponseAction::builder()
            .status_code(503)
            .build()
            .unwrap(),
    );
    client
        .update_listener()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .default_action(maintenance)
        .send()
        .await
        .expect("update_listener default action");

    // Verify.
    let got = client
        .get_listener()
        .service_identifier(&svc_id)
        .listener_identifier(&listener_id)
        .send()
        .await
        .expect("get_listener");
    let action = got.default_action().expect("default_action present");
    match action {
        RuleAction::FixedResponse(fr) => {
            assert_eq!(fr.status_code(), 503);
        }
        other => panic!("expected FixedResponse default action, got {other:?}"),
    }
}
