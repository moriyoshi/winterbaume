//! Integration tests for winterbaume Direct Connect service.
//!
//! These tests verify that aws-sdk-directconnect operations work end-to-end
//! through the winterbaume mock infrastructure.

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

#[tokio::test]
async fn test_create_connection() {
    let client = make_dc_client().await;

    let resp = client
        .create_connection()
        .connection_name("my-connection")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .expect("create_connection should succeed");

    assert!(resp.connection_id().is_some(), "should have connection ID");
    assert_eq!(resp.connection_name(), Some("my-connection"));
    assert_eq!(resp.location(), Some("EqDC2"));
    assert_eq!(resp.bandwidth(), Some("1Gbps"));
    assert_eq!(
        resp.connection_state(),
        Some(&aws_sdk_directconnect::types::ConnectionState::Requested)
    );
}

#[tokio::test]
async fn test_describe_connections_all() {
    let client = make_dc_client().await;

    // Create two connections
    client
        .create_connection()
        .connection_name("conn-1")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    client
        .create_connection()
        .connection_name("conn-2")
        .location("EqSe2")
        .bandwidth("10Gbps")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_connections()
        .send()
        .await
        .expect("describe_connections should succeed");

    let connections = resp.connections();
    assert_eq!(connections.len(), 2, "should have 2 connections");
}

#[tokio::test]
async fn test_describe_connections_by_id() {
    let client = make_dc_client().await;

    let create_resp = client
        .create_connection()
        .connection_name("specific-conn")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    let conn_id = create_resp.connection_id().unwrap().to_string();

    let resp = client
        .describe_connections()
        .connection_id(&conn_id)
        .send()
        .await
        .expect("describe_connections by ID should succeed");

    let connections = resp.connections();
    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].connection_name(), Some("specific-conn"));
}

#[tokio::test]
async fn test_delete_connection() {
    let client = make_dc_client().await;

    let create_resp = client
        .create_connection()
        .connection_name("to-delete")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    let conn_id = create_resp.connection_id().unwrap().to_string();

    let del_resp = client
        .delete_connection()
        .connection_id(&conn_id)
        .send()
        .await
        .expect("delete_connection should succeed");

    assert_eq!(
        del_resp.connection_state(),
        Some(&aws_sdk_directconnect::types::ConnectionState::Deleted)
    );

    // Verify it's gone
    let desc_result = client
        .describe_connections()
        .connection_id(&conn_id)
        .send()
        .await;

    assert!(
        desc_result.is_err(),
        "describe_connections for deleted connection should fail"
    );
}

#[tokio::test]
async fn test_describe_locations() {
    let client = make_dc_client().await;

    let resp = client
        .describe_locations()
        .send()
        .await
        .expect("describe_locations should succeed");

    let locations = resp.locations();
    assert!(!locations.is_empty(), "should have at least one location");

    let first = &locations[0];
    assert!(first.location_code().is_some());
    assert!(first.location_name().is_some());
    assert!(first.region().is_some());
}

#[tokio::test]
async fn test_delete_nonexistent_connection_fails() {
    let client = make_dc_client().await;

    let result = client
        .delete_connection()
        .connection_id("dxcon-nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_connection for nonexistent ID should fail"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_connection_fails() {
    let client = make_dc_client().await;

    let result = client
        .describe_connections()
        .connection_id("dxcon-nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_connections for nonexistent ID should fail"
    );
}

#[tokio::test]
async fn test_create_connection_with_tags() {
    let client = make_dc_client().await;

    let resp = client
        .create_connection()
        .connection_name("tagged-connection")
        .location("EqDC2")
        .bandwidth("10Gbps")
        .tags(
            aws_sdk_directconnect::types::Tag::builder()
                .key("environment")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_directconnect::types::Tag::builder()
                .key("owner")
                .value("team-a")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_connection with tags should succeed");

    assert!(resp.connection_id().is_some());
    assert_eq!(resp.connection_name(), Some("tagged-connection"));
    assert_eq!(resp.tags().len(), 2);

    let tag_keys: Vec<&str> = resp.tags().iter().map(|t| t.key()).collect();
    assert!(tag_keys.contains(&"environment"));
    assert!(tag_keys.contains(&"owner"));
}

#[tokio::test]
async fn test_describe_connections_empty() {
    let client = make_dc_client().await;

    let resp = client
        .describe_connections()
        .send()
        .await
        .expect("describe_connections on empty state should succeed");

    assert_eq!(resp.connections().len(), 0);
}

#[tokio::test]
async fn test_create_multiple_connections_same_location() {
    let client = make_dc_client().await;

    client
        .create_connection()
        .connection_name("conn-a")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    client
        .create_connection()
        .connection_name("conn-b")
        .location("EqDC2")
        .bandwidth("10Gbps")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_connections()
        .send()
        .await
        .expect("describe_connections should succeed");

    assert_eq!(resp.connections().len(), 2);

    // Connection IDs should be different
    let ids: Vec<Option<&str>> = resp
        .connections()
        .iter()
        .map(|c| c.connection_id())
        .collect();
    assert_ne!(ids[0], ids[1]);
}

#[tokio::test]
async fn test_create_connection_unique_ids() {
    let client = make_dc_client().await;

    let resp1 = client
        .create_connection()
        .connection_name("conn-unique-1")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    let resp2 = client
        .create_connection()
        .connection_name("conn-unique-2")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    assert_ne!(
        resp1.connection_id(),
        resp2.connection_id(),
        "each connection should receive a distinct ID"
    );
}

#[tokio::test]
async fn test_create_connection_id_prefix() {
    let client = make_dc_client().await;

    let resp = client
        .create_connection()
        .connection_name("prefix-check")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    let id = resp.connection_id().expect("should have connection ID");
    assert!(
        id.starts_with("dxcon-"),
        "connection ID should start with 'dxcon-', got: {id}"
    );
}

#[tokio::test]
async fn test_create_connection_response_has_region() {
    let client = make_dc_client().await;

    let resp = client
        .create_connection()
        .connection_name("region-check")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.region(),
        Some("us-east-1"),
        "response should carry the region the client was configured with"
    );
}

#[tokio::test]
async fn test_create_connection_response_has_owner_account() {
    let client = make_dc_client().await;

    let resp = client
        .create_connection()
        .connection_name("owner-check")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    assert!(
        resp.owner_account().is_some(),
        "response should include ownerAccount"
    );
}

#[tokio::test]
async fn test_delete_connection_removes_from_list() {
    let client = make_dc_client().await;

    // Create two connections so we can verify only one is deleted.
    let keep_resp = client
        .create_connection()
        .connection_name("keep-me")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();
    let keep_id = keep_resp.connection_id().unwrap().to_string();

    let del_resp = client
        .create_connection()
        .connection_name("delete-me")
        .location("EqSe2")
        .bandwidth("10Gbps")
        .send()
        .await
        .unwrap();
    let del_id = del_resp.connection_id().unwrap().to_string();

    client
        .delete_connection()
        .connection_id(&del_id)
        .send()
        .await
        .expect("delete_connection should succeed");

    let list = client
        .describe_connections()
        .send()
        .await
        .expect("describe_connections should succeed");

    let ids: Vec<&str> = list
        .connections()
        .iter()
        .filter_map(|c| c.connection_id())
        .collect();

    assert!(
        ids.contains(&keep_id.as_str()),
        "kept connection should still be present"
    );
    assert!(
        !ids.contains(&del_id.as_str()),
        "deleted connection should not appear in the list"
    );
}

#[tokio::test]
async fn test_double_delete_fails() {
    let client = make_dc_client().await;

    let create_resp = client
        .create_connection()
        .connection_name("double-delete")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    let conn_id = create_resp.connection_id().unwrap().to_string();

    client
        .delete_connection()
        .connection_id(&conn_id)
        .send()
        .await
        .expect("first delete should succeed");

    let second = client
        .delete_connection()
        .connection_id(&conn_id)
        .send()
        .await;

    assert!(
        second.is_err(),
        "second delete of the same connection should fail"
    );
}

#[tokio::test]
async fn test_describe_locations_has_available_port_speeds() {
    let client = make_dc_client().await;

    let resp = client
        .describe_locations()
        .send()
        .await
        .expect("describe_locations should succeed");

    for loc in resp.locations() {
        assert!(
            !loc.available_port_speeds().is_empty(),
            "location {} should expose at least one port speed",
            loc.location_code().unwrap_or("<unknown>")
        );
    }
}

#[tokio::test]
async fn test_describe_locations_has_available_providers() {
    let client = make_dc_client().await;

    let resp = client
        .describe_locations()
        .send()
        .await
        .expect("describe_locations should succeed");

    for loc in resp.locations() {
        assert!(
            !loc.available_providers().is_empty(),
            "location {} should expose at least one provider",
            loc.location_code().unwrap_or("<unknown>")
        );
    }
}

#[tokio::test]
async fn test_update_connection_returns_not_implemented() {
    let client = make_dc_client().await;

    let create_resp = client
        .create_connection()
        .connection_name("update-target")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .unwrap();

    let conn_id = create_resp.connection_id().unwrap().to_string();

    let result = client
        .update_connection()
        .connection_id(&conn_id)
        .connection_name("renamed-connection")
        .send()
        .await;

    assert!(
        result.is_err(),
        "UpdateConnection is not yet implemented and should return an error"
    );
}

#[tokio::test]
async fn test_create_connection_with_single_tag() {
    let client = make_dc_client().await;

    let resp = client
        .create_connection()
        .connection_name("single-tag-conn")
        .location("EqSe2")
        .bandwidth("10Gbps")
        .tags(
            aws_sdk_directconnect::types::Tag::builder()
                .key("project")
                .value("winterbaume")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_connection with a single tag should succeed");

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "project");
    assert_eq!(resp.tags()[0].value(), Some("winterbaume"));
}

#[tokio::test]
async fn test_describe_connections_count_after_multiple_creates_and_delete() {
    let client = make_dc_client().await;

    let mut ids = Vec::new();
    for i in 0..3 {
        let r = client
            .create_connection()
            .connection_name(format!("batch-conn-{i}"))
            .location("EqDC2")
            .bandwidth("1Gbps")
            .send()
            .await
            .unwrap();
        ids.push(r.connection_id().unwrap().to_string());
    }

    // Delete the middle one.
    client
        .delete_connection()
        .connection_id(&ids[1])
        .send()
        .await
        .unwrap();

    let list = client
        .describe_connections()
        .send()
        .await
        .expect("describe_connections should succeed");

    assert_eq!(
        list.connections().len(),
        2,
        "should have 2 connections after deleting one of three"
    );
}

#[tokio::test]
async fn test_describe_tags_returns_existing_tags() {
    let client = make_dc_client().await;

    let resp = client
        .create_connection()
        .connection_name("tagged-conn")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .tags(
            aws_sdk_directconnect::types::Tag::builder()
                .key("environment")
                .value("production")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_connection should succeed");
    let connection_id = resp.connection_id().unwrap();
    let arn = format!("arn:aws:directconnect:us-east-1:123456789012:dxcon/{connection_id}");

    let described = client
        .describe_tags()
        .resource_arns(&arn)
        .send()
        .await
        .expect("describe_tags should succeed");

    let resource_tags = described.resource_tags();
    assert_eq!(resource_tags.len(), 1);
    assert_eq!(resource_tags[0].resource_arn(), Some(arn.as_str()));
    let tags = resource_tags[0].tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "environment");
    assert_eq!(tags[0].value(), Some("production"));
}

#[tokio::test]
async fn test_tag_resource_then_describe_tags() {
    let client = make_dc_client().await;

    let resp = client
        .create_connection()
        .connection_name("untagged-conn")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .send()
        .await
        .expect("create_connection should succeed");
    let connection_id = resp.connection_id().unwrap();
    let arn = format!("arn:aws:directconnect:us-east-1:123456789012:dxcon/{connection_id}");

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_directconnect::types::Tag::builder()
                .key("team")
                .value("infra")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let described = client
        .describe_tags()
        .resource_arns(&arn)
        .send()
        .await
        .expect("describe_tags should succeed");
    let tags = described.resource_tags()[0].tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "team");
    assert_eq!(tags[0].value(), Some("infra"));
}

#[tokio::test]
async fn test_untag_resource_removes_tags() {
    let client = make_dc_client().await;

    let resp = client
        .create_connection()
        .connection_name("multi-tag-conn")
        .location("EqDC2")
        .bandwidth("1Gbps")
        .tags(
            aws_sdk_directconnect::types::Tag::builder()
                .key("team")
                .value("infra")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_directconnect::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_connection should succeed");
    let connection_id = resp.connection_id().unwrap();
    let arn = format!("arn:aws:directconnect:us-east-1:123456789012:dxcon/{connection_id}");

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    let described = client
        .describe_tags()
        .resource_arns(&arn)
        .send()
        .await
        .expect("describe_tags should succeed");
    let tags = described.resource_tags()[0].tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value(), Some("prod"));
}
