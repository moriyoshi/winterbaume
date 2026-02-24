use aws_sdk_apigatewayv2::config::BehaviorVersion;
use winterbaume_apigatewayv2::ApiGatewayV2Service;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_apigatewayv2::Client {
    let mock = MockAws::builder()
        .with_service(ApiGatewayV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apigatewayv2::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_apigatewayv2::Client::new(&config)
}

// -----------------------------------------------------------------------
// API tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_api() {
    let client = make_client().await;

    let create_resp = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = create_resp.api_id().expect("api_id should be present");
    assert_eq!(create_resp.name().unwrap_or_default(), "test-api");
    assert_eq!(
        create_resp.protocol_type().unwrap(),
        &aws_sdk_apigatewayv2::types::ProtocolType::Http
    );

    let get_resp = client
        .get_api()
        .api_id(api_id)
        .send()
        .await
        .expect("get_api should succeed");

    assert_eq!(get_resp.name().unwrap_or_default(), "test-api");
    assert_eq!(get_resp.api_id().unwrap_or_default(), api_id);
}

#[tokio::test]
async fn test_get_apis_empty() {
    let client = make_client().await;

    let resp = client
        .get_apis()
        .send()
        .await
        .expect("get_apis should succeed");

    assert!(resp.items().is_empty());
}

#[tokio::test]
async fn test_get_apis_with_entries() {
    let client = make_client().await;

    client
        .create_api()
        .name("api-1")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    client
        .create_api()
        .name("api-2")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Websocket)
        .send()
        .await
        .expect("create_api should succeed");

    let resp = client
        .get_apis()
        .send()
        .await
        .expect("get_apis should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_delete_api() {
    let client = make_client().await;

    let create_resp = client
        .create_api()
        .name("api-to-delete")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = create_resp.api_id().expect("api_id should be present");

    client
        .delete_api()
        .api_id(api_id)
        .send()
        .await
        .expect("delete_api should succeed");

    let get_result = client.get_api().api_id(api_id).send().await;
    assert!(get_result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_update_api() {
    let client = make_client().await;

    let create_resp = client
        .create_api()
        .name("original-name")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = create_resp.api_id().expect("api_id should be present");

    let update_resp = client
        .update_api()
        .api_id(api_id)
        .name("updated-name")
        .send()
        .await
        .expect("update_api should succeed");

    assert_eq!(update_resp.name().unwrap_or_default(), "updated-name");
}

#[tokio::test]
async fn test_get_nonexistent_api() {
    let client = make_client().await;

    let result = client.get_api().api_id("nonexistent-api-id").send().await;

    assert!(result.is_err());
}

// -----------------------------------------------------------------------
// Stage tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_stage() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_stage()
        .api_id(api_id)
        .stage_name("prod")
        .send()
        .await
        .expect("create_stage should succeed");

    assert_eq!(create_resp.stage_name().unwrap_or_default(), "prod");

    let get_resp = client
        .get_stage()
        .api_id(api_id)
        .stage_name("prod")
        .send()
        .await
        .expect("get_stage should succeed");

    assert_eq!(get_resp.stage_name().unwrap_or_default(), "prod");
}

#[tokio::test]
async fn test_get_stages() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    client
        .create_stage()
        .api_id(api_id)
        .stage_name("dev")
        .send()
        .await
        .expect("create_stage should succeed");

    client
        .create_stage()
        .api_id(api_id)
        .stage_name("prod")
        .send()
        .await
        .expect("create_stage should succeed");

    let resp = client
        .get_stages()
        .api_id(api_id)
        .send()
        .await
        .expect("get_stages should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_delete_stage() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    client
        .create_stage()
        .api_id(api_id)
        .stage_name("to-delete")
        .send()
        .await
        .expect("create_stage should succeed");

    client
        .delete_stage()
        .api_id(api_id)
        .stage_name("to-delete")
        .send()
        .await
        .expect("delete_stage should succeed");

    let result = client
        .get_stage()
        .api_id(api_id)
        .stage_name("to-delete")
        .send()
        .await;
    assert!(result.is_err());
}

// -----------------------------------------------------------------------
// Integration tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_integration() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_integration()
        .api_id(api_id)
        .integration_type(aws_sdk_apigatewayv2::types::IntegrationType::HttpProxy)
        .integration_uri("https://example.com")
        .send()
        .await
        .expect("create_integration should succeed");

    let integration_id = create_resp
        .integration_id()
        .expect("integration_id should be present");
    assert_eq!(
        create_resp.integration_type().unwrap(),
        &aws_sdk_apigatewayv2::types::IntegrationType::HttpProxy
    );

    let get_resp = client
        .get_integration()
        .api_id(api_id)
        .integration_id(integration_id)
        .send()
        .await
        .expect("get_integration should succeed");

    assert_eq!(
        get_resp.integration_id().unwrap_or_default(),
        integration_id
    );
}

#[tokio::test]
async fn test_get_integrations() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    let resp = client
        .get_integrations()
        .api_id(api_id)
        .send()
        .await
        .expect("get_integrations should succeed");

    assert!(resp.items().is_empty());
}

#[tokio::test]
async fn test_delete_integration() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    let integration = client
        .create_integration()
        .api_id(api_id)
        .integration_type(aws_sdk_apigatewayv2::types::IntegrationType::HttpProxy)
        .integration_uri("https://example.com")
        .send()
        .await
        .expect("create_integration should succeed");

    let integration_id = integration.integration_id().unwrap();

    client
        .delete_integration()
        .api_id(api_id)
        .integration_id(integration_id)
        .send()
        .await
        .expect("delete_integration should succeed");

    let result = client
        .get_integration()
        .api_id(api_id)
        .integration_id(integration_id)
        .send()
        .await;
    assert!(result.is_err());
}

// -----------------------------------------------------------------------
// Route tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_route() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_route()
        .api_id(api_id)
        .route_key("GET /items")
        .send()
        .await
        .expect("create_route should succeed");

    let route_id = create_resp.route_id().expect("route_id should be present");
    assert_eq!(create_resp.route_key().unwrap_or_default(), "GET /items");

    let get_resp = client
        .get_route()
        .api_id(api_id)
        .route_id(route_id)
        .send()
        .await
        .expect("get_route should succeed");

    assert_eq!(get_resp.route_id().unwrap_or_default(), route_id);
    assert_eq!(get_resp.route_key().unwrap_or_default(), "GET /items");
}

#[tokio::test]
async fn test_get_routes() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    client
        .create_route()
        .api_id(api_id)
        .route_key("GET /items")
        .send()
        .await
        .expect("create_route should succeed");

    client
        .create_route()
        .api_id(api_id)
        .route_key("POST /items")
        .send()
        .await
        .expect("create_route should succeed");

    let resp = client
        .get_routes()
        .api_id(api_id)
        .send()
        .await
        .expect("get_routes should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_delete_route() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    let route = client
        .create_route()
        .api_id(api_id)
        .route_key("GET /to-delete")
        .send()
        .await
        .expect("create_route should succeed");

    let route_id = route.route_id().unwrap();

    client
        .delete_route()
        .api_id(api_id)
        .route_id(route_id)
        .send()
        .await
        .expect("delete_route should succeed");

    let result = client
        .get_route()
        .api_id(api_id)
        .route_id(route_id)
        .send()
        .await;
    assert!(result.is_err());
}

// -----------------------------------------------------------------------
// Deployment tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_deployment() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_deployment()
        .api_id(api_id)
        .send()
        .await
        .expect("create_deployment should succeed");

    let deployment_id = create_resp
        .deployment_id()
        .expect("deployment_id should be present");

    let get_resp = client
        .get_deployment()
        .api_id(api_id)
        .deployment_id(deployment_id)
        .send()
        .await
        .expect("get_deployment should succeed");

    assert_eq!(get_resp.deployment_id().unwrap_or_default(), deployment_id);
}

#[tokio::test]
async fn test_get_deployments() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    client
        .create_deployment()
        .api_id(api_id)
        .send()
        .await
        .expect("create_deployment should succeed");

    let resp = client
        .get_deployments()
        .api_id(api_id)
        .send()
        .await
        .expect("get_deployments should succeed");

    assert_eq!(resp.items().len(), 1);
}

#[tokio::test]
async fn test_delete_deployment() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");

    let api_id = api.api_id().unwrap();

    let deployment = client
        .create_deployment()
        .api_id(api_id)
        .send()
        .await
        .expect("create_deployment should succeed");

    let deployment_id = deployment.deployment_id().unwrap();

    client
        .delete_deployment()
        .api_id(api_id)
        .deployment_id(deployment_id)
        .send()
        .await
        .expect("delete_deployment should succeed");

    let result = client
        .get_deployment()
        .api_id(api_id)
        .deployment_id(deployment_id)
        .send()
        .await;
    assert!(result.is_err());
}

// -----------------------------------------------------------------------
// Lifecycle tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_full_lifecycle() {
    let client = make_client().await;

    // Create API
    let api = client
        .create_api()
        .name("lifecycle-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    // Create integration
    let integration = client
        .create_integration()
        .api_id(api_id)
        .integration_type(aws_sdk_apigatewayv2::types::IntegrationType::HttpProxy)
        .integration_uri("https://backend.example.com")
        .send()
        .await
        .expect("create_integration should succeed");
    let integration_id = integration.integration_id().unwrap();

    // Create route targeting the integration
    let route = client
        .create_route()
        .api_id(api_id)
        .route_key("GET /api/items")
        .target(format!("integrations/{integration_id}"))
        .send()
        .await
        .expect("create_route should succeed");
    let route_id = route.route_id().unwrap();

    // Create stage
    let _stage = client
        .create_stage()
        .api_id(api_id)
        .stage_name("v1")
        .send()
        .await
        .expect("create_stage should succeed");

    // Create deployment
    let _deployment = client
        .create_deployment()
        .api_id(api_id)
        .send()
        .await
        .expect("create_deployment should succeed");

    // Verify everything exists
    let get_route = client
        .get_route()
        .api_id(api_id)
        .route_id(route_id)
        .send()
        .await
        .expect("get_route should succeed");
    assert_eq!(get_route.route_key().unwrap_or_default(), "GET /api/items");

    // Delete the API (should cascade)
    client
        .delete_api()
        .api_id(api_id)
        .send()
        .await
        .expect("delete_api should succeed");

    // Verify it's gone
    let result = client.get_api().api_id(api_id).send().await;
    assert!(result.is_err());
}

// -----------------------------------------------------------------------
// State view tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_state_snapshot_restore() {
    use std::collections::HashMap;

    use winterbaume_apigatewayv2::views::ApiView;
    use winterbaume_apigatewayv2::{ApiGatewayV2Service, ApiGatewayV2StateView};
    use winterbaume_core::StatefulService;

    let svc = ApiGatewayV2Service::new();
    let view = ApiGatewayV2StateView {
        apis: {
            let mut m = HashMap::new();
            m.insert(
                "test-api-id".to_string(),
                ApiView {
                    api_id: "test-api-id".to_string(),
                    name: "restored-api".to_string(),
                    protocol_type: "HTTP".to_string(),
                    route_selection_expression: None,
                    description: None,
                    api_endpoint: "https://test-api-id.execute-api.us-east-1.amazonaws.com"
                        .to_string(),
                    created_date: "2024-01-01T00:00:00Z".to_string(),
                    tags: HashMap::new(),
                },
            );
            m
        },
        ..Default::default()
    };

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snapshot.apis.contains_key("test-api-id"));
    assert_eq!(snapshot.apis["test-api-id"].name, "restored-api");
}

#[tokio::test]
async fn test_state_merge() {
    use std::collections::HashMap;

    use winterbaume_apigatewayv2::views::ApiView;
    use winterbaume_apigatewayv2::{ApiGatewayV2Service, ApiGatewayV2StateView};
    use winterbaume_core::StatefulService;

    let svc = ApiGatewayV2Service::new();

    // First restore with api-1
    let view1 = ApiGatewayV2StateView {
        apis: {
            let mut m = HashMap::new();
            m.insert(
                "api-1".to_string(),
                ApiView {
                    api_id: "api-1".to_string(),
                    name: "first-api".to_string(),
                    protocol_type: "HTTP".to_string(),
                    route_selection_expression: None,
                    description: None,
                    api_endpoint: "https://api-1.execute-api.us-east-1.amazonaws.com".to_string(),
                    created_date: "2024-01-01T00:00:00Z".to_string(),
                    tags: HashMap::new(),
                },
            );
            m
        },
        ..Default::default()
    };
    svc.restore("123456789012", "us-east-1", view1)
        .await
        .unwrap();

    // Then merge with api-2 — api-1 should still be present
    let view2 = ApiGatewayV2StateView {
        apis: {
            let mut m = HashMap::new();
            m.insert(
                "api-2".to_string(),
                ApiView {
                    api_id: "api-2".to_string(),
                    name: "second-api".to_string(),
                    protocol_type: "WEBSOCKET".to_string(),
                    route_selection_expression: None,
                    description: None,
                    api_endpoint: "https://api-2.execute-api.us-east-1.amazonaws.com".to_string(),
                    created_date: "2024-01-01T00:00:00Z".to_string(),
                    tags: HashMap::new(),
                },
            );
            m
        },
        ..Default::default()
    };
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot.apis.contains_key("api-1"),
        "api-1 should still be present after merge"
    );
    assert!(
        snapshot.apis.contains_key("api-2"),
        "api-2 should be present after merge"
    );
}

// -----------------------------------------------------------------------
// State change notification tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_apigatewayv2::ApiGatewayV2Service;
    use winterbaume_core::StatefulService;

    let svc = ApiGatewayV2Service::new();
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
        .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use winterbaume_apigatewayv2::views::ApiView;
    use winterbaume_apigatewayv2::{ApiGatewayV2Service, ApiGatewayV2StateView};
    use winterbaume_core::StatefulService;

    let svc = ApiGatewayV2Service::new();

    // Pre-seed with one API; ignore this first event
    let initial_view = ApiGatewayV2StateView {
        apis: {
            let mut m = HashMap::new();
            m.insert(
                "api-seed".to_string(),
                ApiView {
                    api_id: "api-seed".to_string(),
                    name: "seed-api".to_string(),
                    protocol_type: "HTTP".to_string(),
                    route_selection_expression: None,
                    description: None,
                    api_endpoint: "https://api-seed.execute-api.us-east-1.amazonaws.com"
                        .to_string(),
                    created_date: "2024-01-01T00:00:00Z".to_string(),
                    tags: HashMap::new(),
                },
            );
            m
        },
        ..Default::default()
    };
    svc.restore("123456789012", "us-east-1", initial_view)
        .await
        .unwrap();

    // Now register listener and do a second restore
    let snapshots: Arc<Mutex<Vec<ApiGatewayV2StateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    let new_view = ApiGatewayV2StateView {
        apis: {
            let mut m = HashMap::new();
            m.insert(
                "api-new".to_string(),
                ApiView {
                    api_id: "api-new".to_string(),
                    name: "new-api".to_string(),
                    protocol_type: "HTTP".to_string(),
                    route_selection_expression: None,
                    description: None,
                    api_endpoint: "https://api-new.execute-api.us-east-1.amazonaws.com".to_string(),
                    created_date: "2024-01-01T00:00:00Z".to_string(),
                    tags: HashMap::new(),
                },
            );
            m
        },
        ..Default::default()
    };
    svc.restore("123456789012", "us-east-1", new_view)
        .await
        .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(
        got[0].apis.contains_key("api-new"),
        "snapshot should contain the newly restored API"
    );
    assert!(
        !got[0].apis.contains_key("api-seed"),
        "snapshot should NOT contain the pre-seed API after restore (not merge)"
    );
}

// -----------------------------------------------------------------------
// UpdateAuthorizer tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_update_authorizer() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_authorizer()
        .api_id(api_id)
        .name("my-authorizer")
        .authorizer_type(aws_sdk_apigatewayv2::types::AuthorizerType::Request)
        .identity_source("$request.header.Authorization")
        .send()
        .await
        .expect("create_authorizer should succeed");

    let authorizer_id = create_resp.authorizer_id().unwrap();
    assert_eq!(create_resp.name().unwrap(), "my-authorizer");

    let update_resp = client
        .update_authorizer()
        .api_id(api_id)
        .authorizer_id(authorizer_id)
        .name("updated-authorizer")
        .send()
        .await
        .expect("update_authorizer should succeed");

    assert_eq!(update_resp.name().unwrap(), "updated-authorizer");
    assert_eq!(update_resp.authorizer_id().unwrap(), authorizer_id);
}

// -----------------------------------------------------------------------
// UpdateModel tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_update_model() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_model()
        .api_id(api_id)
        .name("my-model")
        .schema("{}")
        .send()
        .await
        .expect("create_model should succeed");

    let model_id = create_resp.model_id().unwrap();
    assert_eq!(create_resp.name().unwrap(), "my-model");

    let update_resp = client
        .update_model()
        .api_id(api_id)
        .model_id(model_id)
        .name("updated-model")
        .description("updated description")
        .send()
        .await
        .expect("update_model should succeed");

    assert_eq!(update_resp.name().unwrap(), "updated-model");
    assert_eq!(update_resp.description().unwrap(), "updated description");
    assert_eq!(update_resp.model_id().unwrap(), model_id);
}

// -----------------------------------------------------------------------
// UpdateIntegrationResponse tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_update_integration_response() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let integration = client
        .create_integration()
        .api_id(api_id)
        .integration_type(aws_sdk_apigatewayv2::types::IntegrationType::Http)
        .send()
        .await
        .expect("create_integration should succeed");
    let integration_id = integration.integration_id().unwrap();

    let create_resp = client
        .create_integration_response()
        .api_id(api_id)
        .integration_id(integration_id)
        .integration_response_key("200")
        .send()
        .await
        .expect("create_integration_response should succeed");
    let response_id = create_resp.integration_response_id().unwrap();
    assert_eq!(create_resp.integration_response_key().unwrap(), "200");

    let update_resp = client
        .update_integration_response()
        .api_id(api_id)
        .integration_id(integration_id)
        .integration_response_id(response_id)
        .integration_response_key("201")
        .send()
        .await
        .expect("update_integration_response should succeed");

    assert_eq!(update_resp.integration_response_key().unwrap(), "201");
    assert_eq!(update_resp.integration_response_id().unwrap(), response_id);
}

// -----------------------------------------------------------------------
// UpdateVpcLink tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_update_vpc_link() {
    let client = make_client().await;

    let create_resp = client
        .create_vpc_link()
        .name("my-vpc-link")
        .subnet_ids("subnet-12345")
        .send()
        .await
        .expect("create_vpc_link should succeed");

    let vpc_link_id = create_resp.vpc_link_id().unwrap();
    assert_eq!(create_resp.name().unwrap(), "my-vpc-link");

    let update_resp = client
        .update_vpc_link()
        .vpc_link_id(vpc_link_id)
        .name("updated-vpc-link")
        .send()
        .await
        .expect("update_vpc_link should succeed");

    assert_eq!(update_resp.name().unwrap(), "updated-vpc-link");
    assert_eq!(update_resp.vpc_link_id().unwrap(), vpc_link_id);
}

// -----------------------------------------------------------------------
// UpdateStage tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_update_stage() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    client
        .create_stage()
        .api_id(api_id)
        .stage_name("dev")
        .description("original desc")
        .send()
        .await
        .expect("create_stage should succeed");

    let update_resp = client
        .update_stage()
        .api_id(api_id)
        .stage_name("dev")
        .description("updated desc")
        .send()
        .await
        .expect("update_stage should succeed");

    assert_eq!(update_resp.stage_name().unwrap_or_default(), "dev");
    assert_eq!(update_resp.description().unwrap(), "updated desc");
}

// -----------------------------------------------------------------------
// UpdateIntegration tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_update_integration() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let integration = client
        .create_integration()
        .api_id(api_id)
        .integration_type(aws_sdk_apigatewayv2::types::IntegrationType::HttpProxy)
        .integration_uri("https://example.com")
        .send()
        .await
        .expect("create_integration should succeed");
    let integration_id = integration.integration_id().unwrap();

    let update_resp = client
        .update_integration()
        .api_id(api_id)
        .integration_id(integration_id)
        .description("updated integration")
        .send()
        .await
        .expect("update_integration should succeed");

    assert_eq!(update_resp.integration_id().unwrap(), integration_id);
    assert_eq!(update_resp.description().unwrap(), "updated integration");
}

// -----------------------------------------------------------------------
// UpdateRoute tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_update_route() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let route = client
        .create_route()
        .api_id(api_id)
        .route_key("GET /items")
        .send()
        .await
        .expect("create_route should succeed");
    let route_id = route.route_id().unwrap();

    let update_resp = client
        .update_route()
        .api_id(api_id)
        .route_id(route_id)
        .route_key("POST /items")
        .send()
        .await
        .expect("update_route should succeed");

    assert_eq!(update_resp.route_id().unwrap(), route_id);
    assert_eq!(update_resp.route_key().unwrap(), "POST /items");
}

// -----------------------------------------------------------------------
// Authorizer CRUD tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_authorizers() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    client
        .create_authorizer()
        .api_id(api_id)
        .name("auth-1")
        .authorizer_type(aws_sdk_apigatewayv2::types::AuthorizerType::Request)
        .identity_source("$request.header.Authorization")
        .send()
        .await
        .expect("create_authorizer should succeed");

    let resp = client
        .get_authorizers()
        .api_id(api_id)
        .send()
        .await
        .expect("get_authorizers should succeed");

    assert_eq!(resp.items().len(), 1);
}

#[tokio::test]
async fn test_get_authorizer() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_authorizer()
        .api_id(api_id)
        .name("my-auth")
        .authorizer_type(aws_sdk_apigatewayv2::types::AuthorizerType::Request)
        .identity_source("$request.header.Authorization")
        .send()
        .await
        .expect("create_authorizer should succeed");
    let authorizer_id = create_resp.authorizer_id().unwrap();

    let get_resp = client
        .get_authorizer()
        .api_id(api_id)
        .authorizer_id(authorizer_id)
        .send()
        .await
        .expect("get_authorizer should succeed");

    assert_eq!(get_resp.authorizer_id().unwrap(), authorizer_id);
    assert_eq!(get_resp.name().unwrap(), "my-auth");
}

#[tokio::test]
async fn test_delete_authorizer() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_authorizer()
        .api_id(api_id)
        .name("to-delete")
        .authorizer_type(aws_sdk_apigatewayv2::types::AuthorizerType::Request)
        .identity_source("$request.header.Authorization")
        .send()
        .await
        .expect("create_authorizer should succeed");
    let authorizer_id = create_resp.authorizer_id().unwrap();

    client
        .delete_authorizer()
        .api_id(api_id)
        .authorizer_id(authorizer_id)
        .send()
        .await
        .expect("delete_authorizer should succeed");

    let result = client
        .get_authorizer()
        .api_id(api_id)
        .authorizer_id(authorizer_id)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// -----------------------------------------------------------------------
// Model CRUD tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_models() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    client
        .create_model()
        .api_id(api_id)
        .name("model-1")
        .schema("{}")
        .send()
        .await
        .expect("create_model should succeed");

    let resp = client
        .get_models()
        .api_id(api_id)
        .send()
        .await
        .expect("get_models should succeed");

    assert_eq!(resp.items().len(), 1);
}

#[tokio::test]
async fn test_get_model() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_model()
        .api_id(api_id)
        .name("my-model")
        .schema("{}")
        .send()
        .await
        .expect("create_model should succeed");
    let model_id = create_resp.model_id().unwrap();

    let get_resp = client
        .get_model()
        .api_id(api_id)
        .model_id(model_id)
        .send()
        .await
        .expect("get_model should succeed");

    assert_eq!(get_resp.model_id().unwrap(), model_id);
    assert_eq!(get_resp.name().unwrap(), "my-model");
}

#[tokio::test]
async fn test_delete_model() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let create_resp = client
        .create_model()
        .api_id(api_id)
        .name("to-delete")
        .schema("{}")
        .send()
        .await
        .expect("create_model should succeed");
    let model_id = create_resp.model_id().unwrap();

    client
        .delete_model()
        .api_id(api_id)
        .model_id(model_id)
        .send()
        .await
        .expect("delete_model should succeed");

    let result = client
        .get_model()
        .api_id(api_id)
        .model_id(model_id)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// -----------------------------------------------------------------------
// VPC Link CRUD tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_vpc_links() {
    let client = make_client().await;

    // Initially empty
    let resp = client
        .get_vpc_links()
        .send()
        .await
        .expect("get_vpc_links should succeed");
    assert!(resp.items().is_empty());

    // Create one
    client
        .create_vpc_link()
        .name("vpc-link-1")
        .subnet_ids("subnet-12345")
        .send()
        .await
        .expect("create_vpc_link should succeed");

    let resp = client
        .get_vpc_links()
        .send()
        .await
        .expect("get_vpc_links should succeed");
    assert_eq!(resp.items().len(), 1);
}

#[tokio::test]
async fn test_get_vpc_link() {
    let client = make_client().await;

    let create_resp = client
        .create_vpc_link()
        .name("my-vpc-link")
        .subnet_ids("subnet-12345")
        .send()
        .await
        .expect("create_vpc_link should succeed");
    let vpc_link_id = create_resp.vpc_link_id().unwrap();

    let get_resp = client
        .get_vpc_link()
        .vpc_link_id(vpc_link_id)
        .send()
        .await
        .expect("get_vpc_link should succeed");

    assert_eq!(get_resp.vpc_link_id().unwrap(), vpc_link_id);
    assert_eq!(get_resp.name().unwrap(), "my-vpc-link");
}

#[tokio::test]
async fn test_delete_vpc_link() {
    let client = make_client().await;

    let create_resp = client
        .create_vpc_link()
        .name("to-delete")
        .subnet_ids("subnet-12345")
        .send()
        .await
        .expect("create_vpc_link should succeed");
    let vpc_link_id = create_resp.vpc_link_id().unwrap();

    client
        .delete_vpc_link()
        .vpc_link_id(vpc_link_id)
        .send()
        .await
        .expect("delete_vpc_link should succeed");

    let result = client.get_vpc_link().vpc_link_id(vpc_link_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

// -----------------------------------------------------------------------
// Domain Name CRUD tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_domain_name() {
    let client = make_client().await;

    let create_resp = client
        .create_domain_name()
        .domain_name("example.com")
        .send()
        .await
        .expect("create_domain_name should succeed");

    assert_eq!(create_resp.domain_name().unwrap(), "example.com");

    let get_resp = client
        .get_domain_name()
        .domain_name("example.com")
        .send()
        .await
        .expect("get_domain_name should succeed");

    assert_eq!(get_resp.domain_name().unwrap(), "example.com");
}

#[tokio::test]
async fn test_get_domain_names() {
    let client = make_client().await;

    let resp = client
        .get_domain_names()
        .send()
        .await
        .expect("get_domain_names should succeed");
    assert!(resp.items().is_empty());

    client
        .create_domain_name()
        .domain_name("example.com")
        .send()
        .await
        .expect("create_domain_name should succeed");

    let resp = client
        .get_domain_names()
        .send()
        .await
        .expect("get_domain_names should succeed");
    assert_eq!(resp.items().len(), 1);
}

#[tokio::test]
async fn test_delete_domain_name() {
    let client = make_client().await;

    client
        .create_domain_name()
        .domain_name("to-delete.example.com")
        .send()
        .await
        .expect("create_domain_name should succeed");

    client
        .delete_domain_name()
        .domain_name("to-delete.example.com")
        .send()
        .await
        .expect("delete_domain_name should succeed");

    let result = client
        .get_domain_name()
        .domain_name("to-delete.example.com")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// -----------------------------------------------------------------------
// Integration Response CRUD tests (Get/Delete, Create already tested)
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_integration_responses() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let integration = client
        .create_integration()
        .api_id(api_id)
        .integration_type(aws_sdk_apigatewayv2::types::IntegrationType::Http)
        .send()
        .await
        .expect("create_integration should succeed");
    let integration_id = integration.integration_id().unwrap();

    // Initially empty
    let resp = client
        .get_integration_responses()
        .api_id(api_id)
        .integration_id(integration_id)
        .send()
        .await
        .expect("get_integration_responses should succeed");
    assert!(resp.items().is_empty());

    // Create one
    client
        .create_integration_response()
        .api_id(api_id)
        .integration_id(integration_id)
        .integration_response_key("200")
        .send()
        .await
        .expect("create_integration_response should succeed");

    let resp = client
        .get_integration_responses()
        .api_id(api_id)
        .integration_id(integration_id)
        .send()
        .await
        .expect("get_integration_responses should succeed");
    assert_eq!(resp.items().len(), 1);
}

#[tokio::test]
async fn test_get_integration_response() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let integration = client
        .create_integration()
        .api_id(api_id)
        .integration_type(aws_sdk_apigatewayv2::types::IntegrationType::Http)
        .send()
        .await
        .expect("create_integration should succeed");
    let integration_id = integration.integration_id().unwrap();

    let create_resp = client
        .create_integration_response()
        .api_id(api_id)
        .integration_id(integration_id)
        .integration_response_key("200")
        .send()
        .await
        .expect("create_integration_response should succeed");
    let response_id = create_resp.integration_response_id().unwrap();

    let get_resp = client
        .get_integration_response()
        .api_id(api_id)
        .integration_id(integration_id)
        .integration_response_id(response_id)
        .send()
        .await
        .expect("get_integration_response should succeed");

    assert_eq!(get_resp.integration_response_id().unwrap(), response_id);
    assert_eq!(get_resp.integration_response_key().unwrap(), "200");
}

#[tokio::test]
async fn test_delete_integration_response() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let integration = client
        .create_integration()
        .api_id(api_id)
        .integration_type(aws_sdk_apigatewayv2::types::IntegrationType::Http)
        .send()
        .await
        .expect("create_integration should succeed");
    let integration_id = integration.integration_id().unwrap();

    let create_resp = client
        .create_integration_response()
        .api_id(api_id)
        .integration_id(integration_id)
        .integration_response_key("200")
        .send()
        .await
        .expect("create_integration_response should succeed");
    let response_id = create_resp.integration_response_id().unwrap();

    client
        .delete_integration_response()
        .api_id(api_id)
        .integration_id(integration_id)
        .integration_response_id(response_id)
        .send()
        .await
        .expect("delete_integration_response should succeed");

    let result = client
        .get_integration_response()
        .api_id(api_id)
        .integration_id(integration_id)
        .integration_response_id(response_id)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// -----------------------------------------------------------------------
// Route Response CRUD tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_route_response() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let route = client
        .create_route()
        .api_id(api_id)
        .route_key("GET /items")
        .send()
        .await
        .expect("create_route should succeed");
    let route_id = route.route_id().unwrap();

    let create_resp = client
        .create_route_response()
        .api_id(api_id)
        .route_id(route_id)
        .route_response_key("$default")
        .send()
        .await
        .expect("create_route_response should succeed");
    let response_id = create_resp
        .route_response_id()
        .expect("route_response_id should be present");

    assert_eq!(create_resp.route_response_key().unwrap(), "$default");

    let get_resp = client
        .get_route_response()
        .api_id(api_id)
        .route_id(route_id)
        .route_response_id(response_id)
        .send()
        .await
        .expect("get_route_response should succeed");

    assert_eq!(get_resp.route_response_id().unwrap(), response_id);
    assert_eq!(get_resp.route_response_key().unwrap(), "$default");
}

#[tokio::test]
async fn test_get_route_responses() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let route = client
        .create_route()
        .api_id(api_id)
        .route_key("GET /items")
        .send()
        .await
        .expect("create_route should succeed");
    let route_id = route.route_id().unwrap();

    // Initially empty
    let resp = client
        .get_route_responses()
        .api_id(api_id)
        .route_id(route_id)
        .send()
        .await
        .expect("get_route_responses should succeed");
    assert!(resp.items().is_empty());

    // Create one
    client
        .create_route_response()
        .api_id(api_id)
        .route_id(route_id)
        .route_response_key("$default")
        .send()
        .await
        .expect("create_route_response should succeed");

    let resp = client
        .get_route_responses()
        .api_id(api_id)
        .route_id(route_id)
        .send()
        .await
        .expect("get_route_responses should succeed");
    assert_eq!(resp.items().len(), 1);
}

#[tokio::test]
async fn test_delete_route_response() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let route = client
        .create_route()
        .api_id(api_id)
        .route_key("GET /items")
        .send()
        .await
        .expect("create_route should succeed");
    let route_id = route.route_id().unwrap();

    let create_resp = client
        .create_route_response()
        .api_id(api_id)
        .route_id(route_id)
        .route_response_key("$default")
        .send()
        .await
        .expect("create_route_response should succeed");
    let response_id = create_resp.route_response_id().unwrap();

    client
        .delete_route_response()
        .api_id(api_id)
        .route_id(route_id)
        .route_response_id(response_id)
        .send()
        .await
        .expect("delete_route_response should succeed");

    let result = client
        .get_route_response()
        .api_id(api_id)
        .route_id(route_id)
        .route_response_id(response_id)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// -----------------------------------------------------------------------
// API Mapping CRUD tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_api_mapping() {
    let client = make_client().await;

    // Prerequisites: API, stage, and domain name
    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    client
        .create_stage()
        .api_id(api_id)
        .stage_name("prod")
        .send()
        .await
        .expect("create_stage should succeed");

    client
        .create_domain_name()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("create_domain_name should succeed");

    let create_resp = client
        .create_api_mapping()
        .domain_name("api.example.com")
        .api_id(api_id)
        .stage("prod")
        .send()
        .await
        .expect("create_api_mapping should succeed");

    let mapping_id = create_resp
        .api_mapping_id()
        .expect("api_mapping_id should be present");
    assert_eq!(create_resp.api_id().unwrap(), api_id);
    assert_eq!(create_resp.stage().unwrap(), "prod");

    let get_resp = client
        .get_api_mapping()
        .domain_name("api.example.com")
        .api_mapping_id(mapping_id)
        .send()
        .await
        .expect("get_api_mapping should succeed");

    assert_eq!(get_resp.api_mapping_id().unwrap(), mapping_id);
    assert_eq!(get_resp.api_id().unwrap(), api_id);
}

#[tokio::test]
async fn test_get_api_mappings() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    client
        .create_stage()
        .api_id(api_id)
        .stage_name("prod")
        .send()
        .await
        .expect("create_stage should succeed");

    client
        .create_domain_name()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("create_domain_name should succeed");

    // Initially empty
    let resp = client
        .get_api_mappings()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("get_api_mappings should succeed");
    assert!(resp.items().is_empty());

    // Create one
    client
        .create_api_mapping()
        .domain_name("api.example.com")
        .api_id(api_id)
        .stage("prod")
        .send()
        .await
        .expect("create_api_mapping should succeed");

    let resp = client
        .get_api_mappings()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("get_api_mappings should succeed");
    assert_eq!(resp.items().len(), 1);
}

#[tokio::test]
async fn test_delete_api_mapping() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("test-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    client
        .create_stage()
        .api_id(api_id)
        .stage_name("prod")
        .send()
        .await
        .expect("create_stage should succeed");

    client
        .create_domain_name()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("create_domain_name should succeed");

    let create_resp = client
        .create_api_mapping()
        .domain_name("api.example.com")
        .api_id(api_id)
        .stage("prod")
        .send()
        .await
        .expect("create_api_mapping should succeed");
    let mapping_id = create_resp.api_mapping_id().unwrap();

    client
        .delete_api_mapping()
        .domain_name("api.example.com")
        .api_mapping_id(mapping_id)
        .send()
        .await
        .expect("delete_api_mapping should succeed");

    let result = client
        .get_api_mapping()
        .domain_name("api.example.com")
        .api_mapping_id(mapping_id)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// -----------------------------------------------------------------------
// Tag tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_tag_and_get_tags() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("tagged-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    // Build the ARN for the API resource
    let resource_arn = format!("arn:aws:apigateway:us-east-1::/apis/{api_id}");

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "test")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .get_tags()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("get_tags should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("backend"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let api = client
        .create_api()
        .name("tagged-api")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed");
    let api_id = api.api_id().unwrap();

    let resource_arn = format!("arn:aws:apigateway:us-east-1::/apis/{api_id}");

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags("env", "test")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    client
        .untag_resource()
        .resource_arn(&resource_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .get_tags()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("get_tags should succeed");

    let tags = resp.tags().unwrap();
    assert!(
        tags.get("env").is_none(),
        "env tag should have been removed"
    );
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("backend"));
}

// -----------------------------------------------------------------------
// Coverage for FIX(e2e) handler fixes
// -----------------------------------------------------------------------
//
// The Go SDK signs API Gateway V2 requests with "apigateway" (not
// "apigatewayv2") in the SigV4 credential scope.  When a provider uses a
// custom endpoint such as http://127.0.0.1:{port}, the mock router cannot
// match the request by URL host, so it falls back to service-name matching
// using the value returned by `MockService::service_name()`.  The fix in
// `ApiGatewayV2Service::service_name()` returns "apigateway" to align with
// what the Go SDK sends.
//
// The Rust AWS SDK, which these integration tests use, sends requests to
// the canonical `apigateway.{region}.amazonaws.com` host and always
// includes the `/v2/` path prefix, so the URL-pattern route
// (`r"https?://apigateway\.[^/]+\.amazonaws\.com/v2/"`) matches before
// the service-name fallback is reached.  This means the credential-scope
// signing name is not exercised in in-process integration tests.
//
// The test below verifies two things that are directly observable without
// an out-of-process proxy:
//   1. `service_name()` returns "apigateway" (unit-level assertion), and
//   2. A basic round-trip through the handler still succeeds, confirming
//      that the URL-pattern routing with the `/v2/` discriminator works
//      correctly alongside the service-name fix.
//
// Full end-to-end coverage of the Go SDK signing path requires running
// a Terraform/Go test against a real listener (see the e2e test suite).

#[tokio::test]
async fn test_service_name_returns_apigateway_for_go_sdk_compat() {
    use winterbaume_core::MockService;

    let svc = ApiGatewayV2Service::new();
    // The Go SDK signs requests with "apigateway" in the credential scope,
    // so the mock service must advertise the same name for fallback routing.
    assert_eq!(
        svc.service_name(),
        "apigateway",
        "service_name() must be 'apigateway' so the router matches \
         Go SDK SigV4 credentials that use 'apigateway' instead of 'apigatewayv2'"
    );
}

#[tokio::test]
async fn test_round_trip_with_apigateway_service_name() {
    // Smoke test: confirm that a create/get round-trip succeeds even though
    // service_name() is "apigateway" rather than "apigatewayv2".  This
    // exercises the URL-pattern route that distinguishes v1 from v2 via the
    // /v2/ path prefix.
    let client = make_client().await;

    let create_resp = client
        .create_api()
        .name("go-sdk-compat-test")
        .protocol_type(aws_sdk_apigatewayv2::types::ProtocolType::Http)
        .send()
        .await
        .expect("create_api should succeed with 'apigateway' service_name");

    let api_id = create_resp.api_id().expect("api_id should be present");

    let get_resp = client
        .get_api()
        .api_id(api_id)
        .send()
        .await
        .expect("get_api should succeed with 'apigateway' service_name");

    assert_eq!(get_resp.name().unwrap_or_default(), "go-sdk-compat-test");
}
