//! Scenario tests for winterbaume Direct Connect service.
//!
//! These tests exercise end-to-end use-case scenarios that chain multiple
//! Direct Connect operations together, asserting business outcomes rather
//! than individual API shapes.

use aws_sdk_directconnect::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_directconnect::DirectConnectService;

/// Helper to create a configured Direct Connect client backed by winterbaume.
async fn make_dc_client() -> aws_sdk_directconnect::Client {
    let mock = MockAws::builder()
        .with_service(DirectConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_directconnect::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_directconnect::Client::new(&config)
}

/// Scenario: Connection lifecycle — create, describe, and delete a connection.
///
/// Verifies the full durable lifecycle of a Direct Connect connection:
/// provisioning a new connection, observing it in describe output, and
/// finally deleting it so the slot is released.
#[tokio::test]
async fn test_connection_lifecycle() {
    // Scenario: Connection lifecycle
    // 1. Create a connection.
    // 2. Verify it appears in describe output.
    // 3. Delete the connection and confirm the response carries Deleted state.
    // 4. Verify the connection no longer appears in describe output.

    let client = make_dc_client().await;

    // Step 1 — provision.
    let create = client
        .create_connection()
        .connection_name("lifecycle-conn")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .tags(
            aws_sdk_directconnect::types::Tag::builder()
                .key("scenario")
                .value("lifecycle")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_connection should succeed");

    let conn_id = create.connection_id().expect("must have ID").to_string();
    assert_eq!(create.connection_name(), Some("lifecycle-conn"));
    assert!(
        conn_id.starts_with("dxcon-"),
        "connection ID must use dxcon- prefix"
    );

    // Step 2 — observe.
    let describe = client
        .describe_connections()
        .connection_id(&conn_id)
        .send()
        .await
        .expect("describe_connections should succeed");

    let connections = describe.connections();
    assert_eq!(connections.len(), 1, "must see exactly the one connection");
    assert_eq!(connections[0].connection_id(), Some(conn_id.as_str()));
    assert_eq!(connections[0].bandwidth(), Some("1Gbps"));
    assert_eq!(
        connections[0].tags().len(),
        1,
        "tag must survive round-trip"
    );

    // Step 3 — delete.
    let delete = client
        .delete_connection()
        .connection_id(&conn_id)
        .send()
        .await
        .expect("delete_connection should succeed");

    assert_eq!(
        delete.connection_state(),
        Some(&aws_sdk_directconnect::types::ConnectionState::Deleted),
        "deleted connection must report Deleted state"
    );

    // Step 4 — confirm removal.
    let after = client
        .describe_connections()
        .connection_id(&conn_id)
        .send()
        .await;

    assert!(
        after.is_err(),
        "describing a deleted connection should return an error"
    );
}

/// Scenario: Multi-connection portfolio — independent connections coexist and
/// can be selectively removed.
///
/// Validates that the service correctly handles a portfolio of connections
/// provisioned at different locations, and that deleting one does not affect
/// the others.
#[tokio::test]
async fn test_multi_connection_portfolio() {
    // Scenario: Multi-connection portfolio
    // 1. Create three connections at two different locations.
    // 2. Verify all three appear in the global describe list.
    // 3. Delete one connection.
    // 4. Verify the list shrinks to two and the correct connection was removed.

    let client = make_dc_client().await;

    // Step 1 — provision three connections.
    let mut ids: Vec<String> = Vec::new();
    for (name, loc, bw) in [
        ("portfolio-a", "EqDC2", "1Gbps"),
        ("portfolio-b", "EqDC2", "10Gbps"),
        ("portfolio-c", "EqSe2", "1Gbps"),
    ] {
        let r = client
            .create_connection()
            .connection_name(name)
            .location(loc)
            .bandwidth(bw)
            .send()
            .await
            .expect("create_connection should succeed");
        ids.push(r.connection_id().unwrap().to_string());
    }

    assert_eq!(ids.len(), 3);
    // All IDs must be distinct.
    let mut unique = ids.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), 3, "all connection IDs must be unique");

    // Step 2 — confirm global list.
    let all = client
        .describe_connections()
        .send()
        .await
        .expect("describe_connections should succeed");
    assert_eq!(
        all.connections().len(),
        3,
        "global list must show all three"
    );

    // Step 3 — delete the second connection.
    client
        .delete_connection()
        .connection_id(&ids[1])
        .send()
        .await
        .expect("delete_connection should succeed");

    // Step 4 — verify portfolio shrank and correct one is gone.
    let remaining = client
        .describe_connections()
        .send()
        .await
        .expect("describe_connections should succeed");

    let remaining_ids: Vec<&str> = remaining
        .connections()
        .iter()
        .filter_map(|c| c.connection_id())
        .collect();

    assert_eq!(
        remaining_ids.len(),
        2,
        "exactly two connections must remain"
    );
    assert!(
        remaining_ids.contains(&ids[0].as_str()),
        "first connection must still exist"
    );
    assert!(
        !remaining_ids.contains(&ids[1].as_str()),
        "deleted connection must be absent"
    );
    assert!(
        remaining_ids.contains(&ids[2].as_str()),
        "third connection must still exist"
    );
}

/// Scenario: Location discovery informs connection provisioning.
///
/// Validates that a client can discover available locations and then
/// successfully provision a connection at one of them, ensuring the
/// DescribeLocations → CreateConnection workflow is coherent.
#[tokio::test]
async fn test_location_discovery_then_provision() {
    // Scenario: Location discovery informs provisioning
    // 1. Discover available locations.
    // 2. Pick the first location's code.
    // 3. Provision a connection there.
    // 4. Confirm the returned connection carries the chosen location.

    let client = make_dc_client().await;

    // Step 1 — discover locations.
    let locs = client
        .describe_locations()
        .send()
        .await
        .expect("describe_locations should succeed");

    let locations = locs.locations();
    assert!(
        !locations.is_empty(),
        "at least one location must be available"
    );

    // Step 2 — pick a location.
    let chosen_code = locations[0]
        .location_code()
        .expect("location must have a code")
        .to_string();

    let speeds = locations[0].available_port_speeds();
    assert!(!speeds.is_empty(), "location must advertise port speeds");
    let chosen_speed = speeds[0].clone();

    // Step 3 — provision.
    let create = client
        .create_connection()
        .connection_name("discovered-location-conn")
        .location(&chosen_code)
        .bandwidth(&chosen_speed)
        .send()
        .await
        .expect("create_connection at discovered location should succeed");

    // Step 4 — verify coherence.
    assert_eq!(
        create.location(),
        Some(chosen_code.as_str()),
        "connection location must match the chosen location code"
    );
    assert_eq!(
        create.bandwidth(),
        Some(chosen_speed.as_str()),
        "connection bandwidth must match the chosen port speed"
    );
}
