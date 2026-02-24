use std::sync::{Arc, Mutex};

use aws_sdk_route53recoverycluster::config::BehaviorVersion;
use aws_sdk_route53recoverycluster::types::{RoutingControlState, UpdateRoutingControlStateEntry};
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_route53recoverycluster::RecoveryClusterService;

const PRIMARY_ARN: &str =
    "arn:aws:route53-recovery-control::123456789012:controlpanel/default-cp/routingcontrol/primary";
const SECONDARY_ARN: &str = "arn:aws:route53-recovery-control::123456789012:controlpanel/default-cp/routingcontrol/secondary";

async fn make_client() -> aws_sdk_route53recoverycluster::Client {
    let mock = MockAws::builder()
        .with_service(RecoveryClusterService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53recoverycluster::config::Region::new(
            "us-east-1",
        ))
        .endpoint_url("https://route53-recovery-cluster.us-east-1.amazonaws.com")
        .load()
        .await;
    aws_sdk_route53recoverycluster::Client::new(&config)
}

#[tokio::test]
async fn test_list_seeded_controls() {
    let client = make_client().await;
    let resp = client.list_routing_controls().send().await.expect("list");
    assert_eq!(resp.routing_controls().len(), 2);
}

#[tokio::test]
async fn test_get_routing_control_state() {
    let client = make_client().await;
    let resp = client
        .get_routing_control_state()
        .routing_control_arn(PRIMARY_ARN)
        .send()
        .await
        .expect("get");
    assert_eq!(resp.routing_control_state().as_str(), "On");
}

#[tokio::test]
async fn test_update_single_state() {
    let client = make_client().await;
    client
        .update_routing_control_state()
        .routing_control_arn(SECONDARY_ARN)
        .routing_control_state(RoutingControlState::On)
        .send()
        .await
        .expect("update");
    let after = client
        .get_routing_control_state()
        .routing_control_arn(SECONDARY_ARN)
        .send()
        .await
        .expect("after");
    assert_eq!(after.routing_control_state().as_str(), "On");
}

#[tokio::test]
async fn test_update_states_batch() {
    let client = make_client().await;
    let entry1 = UpdateRoutingControlStateEntry::builder()
        .routing_control_arn(PRIMARY_ARN)
        .routing_control_state(RoutingControlState::Off)
        .build()
        .unwrap();
    let entry2 = UpdateRoutingControlStateEntry::builder()
        .routing_control_arn(SECONDARY_ARN)
        .routing_control_state(RoutingControlState::On)
        .build()
        .unwrap();
    client
        .update_routing_control_states()
        .update_routing_control_state_entries(entry1)
        .update_routing_control_state_entries(entry2)
        .send()
        .await
        .expect("update batch");

    let primary = client
        .get_routing_control_state()
        .routing_control_arn(PRIMARY_ARN)
        .send()
        .await
        .expect("primary");
    assert_eq!(primary.routing_control_state().as_str(), "Off");
    let secondary = client
        .get_routing_control_state()
        .routing_control_arn(SECONDARY_ARN)
        .send()
        .await
        .expect("secondary");
    assert_eq!(secondary.routing_control_state().as_str(), "On");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = RecoveryClusterService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore");
    assert_eq!(events.lock().unwrap().len(), 1);
}
