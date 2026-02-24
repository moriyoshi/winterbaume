use aws_sdk_apigatewaymanagement::config::BehaviorVersion;
use winterbaume_apigatewaymanagement::ApiGatewayManagementService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_apigatewaymanagement::Client {
    let mock = MockAws::builder()
        .with_service(ApiGatewayManagementService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apigatewaymanagement::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_apigatewaymanagement::Client::new(&config)
}

#[tokio::test]
async fn test_get_connection_auto_creates() {
    let client = make_client().await;

    let resp = client
        .get_connection()
        .connection_id("conn-abc123")
        .send()
        .await
        .expect("get_connection should succeed");

    assert!(resp.connected_at().is_some());
    assert!(resp.last_active_at().is_some());
    let identity = resp.identity().expect("identity should be set");
    assert!(!identity.source_ip().unwrap_or_default().is_empty());
    assert!(!identity.user_agent().unwrap_or_default().is_empty());
}

#[tokio::test]
async fn test_post_to_connection() {
    let client = make_client().await;

    client
        .post_to_connection()
        .connection_id("conn-post1")
        .data(aws_smithy_types::Blob::new(b"hello world".to_vec()))
        .send()
        .await
        .expect("post_to_connection should succeed");
}

#[tokio::test]
async fn test_post_then_get_updates_last_active() {
    let client = make_client().await;

    let before = client
        .get_connection()
        .connection_id("conn-ts1")
        .send()
        .await
        .expect("get_connection failed");

    client
        .post_to_connection()
        .connection_id("conn-ts1")
        .data(aws_smithy_types::Blob::new(b"data".to_vec()))
        .send()
        .await
        .expect("post_to_connection failed");

    let after = client
        .get_connection()
        .connection_id("conn-ts1")
        .send()
        .await
        .expect("get_connection failed");

    // last_active_at should be >= connected_at after a post
    assert!(after.last_active_at() >= before.last_active_at());
}

#[tokio::test]
async fn test_delete_connection() {
    let client = make_client().await;

    // Ensure connection exists
    client
        .get_connection()
        .connection_id("conn-del1")
        .send()
        .await
        .expect("get_connection failed");

    // Delete it
    client
        .delete_connection()
        .connection_id("conn-del1")
        .send()
        .await
        .expect("delete_connection should succeed");
}

#[tokio::test]
async fn test_delete_and_get_recreates() {
    let client = make_client().await;

    client
        .get_connection()
        .connection_id("conn-recreate1")
        .send()
        .await
        .expect("initial get failed");

    client
        .delete_connection()
        .connection_id("conn-recreate1")
        .send()
        .await
        .expect("delete failed");

    // Getting again auto-creates a fresh connection
    let resp = client
        .get_connection()
        .connection_id("conn-recreate1")
        .send()
        .await
        .expect("get after delete should succeed (auto-creates)");

    assert!(resp.connected_at().is_some());
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = ApiGatewayManagementService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore(
        "123456789012",
        "us-east-1",
        winterbaume_apigatewaymanagement::ApiGatewayManagementApiStateView::default(),
    )
    .await
    .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, "123456789012");
    assert_eq!(got[0].1, "us-east-1");
}

#[tokio::test]
async fn test_state_restore_and_snapshot() {
    use winterbaume_apigatewaymanagement::views::ConnectionView;
    use winterbaume_core::StatefulService;

    let svc = ApiGatewayManagementService::new();

    let mut view = winterbaume_apigatewaymanagement::ApiGatewayManagementApiStateView::default();
    view.connections.insert(
        "conn-seeded".to_string(),
        ConnectionView {
            connected_at: chrono::Utc::now(),
            last_active_at: chrono::Utc::now(),
            source_ip: "10.0.0.1".to_string(),
            user_agent: "test-agent".to_string(),
            data: b"payload".to_vec(),
        },
    );

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.connections.contains_key("conn-seeded"));
    let conn = &snap.connections["conn-seeded"];
    assert_eq!(conn.source_ip, "10.0.0.1");
    assert_eq!(conn.user_agent, "test-agent");
    assert_eq!(conn.data, b"payload");
}

#[tokio::test]
async fn test_state_merge_is_additive() {
    use winterbaume_apigatewaymanagement::views::ConnectionView;
    use winterbaume_core::StatefulService;

    let svc = ApiGatewayManagementService::new();

    let mut view1 = winterbaume_apigatewaymanagement::ApiGatewayManagementApiStateView::default();
    view1.connections.insert(
        "conn-A".to_string(),
        ConnectionView {
            connected_at: chrono::Utc::now(),
            last_active_at: chrono::Utc::now(),
            source_ip: "1.1.1.1".to_string(),
            user_agent: "agent-A".to_string(),
            data: vec![],
        },
    );
    svc.restore("123456789012", "us-east-1", view1)
        .await
        .unwrap();

    let mut view2 = winterbaume_apigatewaymanagement::ApiGatewayManagementApiStateView::default();
    view2.connections.insert(
        "conn-B".to_string(),
        ConnectionView {
            connected_at: chrono::Utc::now(),
            last_active_at: chrono::Utc::now(),
            source_ip: "2.2.2.2".to_string(),
            user_agent: "agent-B".to_string(),
            data: vec![],
        },
    );
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snap.connections.contains_key("conn-A"),
        "conn-A should remain after merge"
    );
    assert!(
        snap.connections.contains_key("conn-B"),
        "conn-B should be added by merge"
    );
}
