use aws_sdk_apigateway::config::BehaviorVersion;
use winterbaume_apigateway::ApiGatewayService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_apigateway::Client {
    let mock = MockAws::builder()
        .with_service(ApiGatewayService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apigateway::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_apigateway::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_rest_api() {
    let client = make_client().await;

    let resp = client
        .create_rest_api()
        .name("my-api")
        .description("Test API")
        .send()
        .await
        .expect("create_rest_api failed");

    let api_id = resp.id().expect("id should be set");
    assert_eq!(resp.name().unwrap_or_default(), "my-api");
    assert_eq!(resp.description().unwrap_or_default(), "Test API");
    assert!(resp.root_resource_id().is_some());

    let get_resp = client
        .get_rest_api()
        .rest_api_id(api_id)
        .send()
        .await
        .expect("get_rest_api failed");

    assert_eq!(get_resp.id().unwrap_or_default(), api_id);
    assert_eq!(get_resp.name().unwrap_or_default(), "my-api");
}

#[tokio::test]
async fn test_get_rest_apis() {
    let client = make_client().await;

    client
        .create_rest_api()
        .name("api-1")
        .send()
        .await
        .expect("create api-1 failed");

    client
        .create_rest_api()
        .name("api-2")
        .send()
        .await
        .expect("create api-2 failed");

    let resp = client
        .get_rest_apis()
        .send()
        .await
        .expect("get_rest_apis failed");

    let items = resp.items();
    assert_eq!(items.len(), 2);
    let names: Vec<&str> = items.iter().filter_map(|a| a.name()).collect();
    assert!(names.contains(&"api-1"));
    assert!(names.contains(&"api-2"));
}

#[tokio::test]
async fn test_delete_rest_api() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("to-delete")
        .send()
        .await
        .expect("create failed")
        .id()
        .unwrap()
        .to_string();

    client
        .delete_rest_api()
        .rest_api_id(&api_id)
        .send()
        .await
        .expect("delete_rest_api failed");

    let err = client
        .get_rest_api()
        .rest_api_id(&api_id)
        .send()
        .await
        .expect_err("should return NotFoundException");

    assert!(format!("{err:?}").contains("NotFoundException") || format!("{err:?}").contains("404"));
}

#[tokio::test]
async fn test_get_nonexistent_rest_api_returns_error() {
    let client = make_client().await;

    let err = client
        .get_rest_api()
        .rest_api_id("nonexistent123")
        .send()
        .await
        .expect_err("should fail with NotFoundException");

    assert!(format!("{err:?}").contains("NotFoundException") || format!("{err:?}").contains("404"));
}

#[tokio::test]
async fn test_create_resource() {
    let client = make_client().await;

    let api = client
        .create_rest_api()
        .name("resource-test-api")
        .send()
        .await
        .expect("create api failed");

    let api_id = api.id().unwrap();
    let root_resource_id = api.root_resource_id().unwrap();

    // Create a resource under root
    let resource = client
        .create_resource()
        .rest_api_id(api_id)
        .parent_id(root_resource_id)
        .path_part("items")
        .send()
        .await
        .expect("create_resource failed");

    assert_eq!(resource.path_part().unwrap_or_default(), "items");
    assert_eq!(resource.path().unwrap_or_default(), "/items");
    assert_eq!(resource.parent_id().unwrap_or_default(), root_resource_id);
    assert!(resource.id().is_some());
}

#[tokio::test]
async fn test_get_resources() {
    let client = make_client().await;

    let api = client
        .create_rest_api()
        .name("resources-list-test")
        .send()
        .await
        .expect("create api failed");

    let api_id = api.id().unwrap();
    let root_resource_id = api.root_resource_id().unwrap();

    // Create two child resources
    client
        .create_resource()
        .rest_api_id(api_id)
        .parent_id(root_resource_id)
        .path_part("users")
        .send()
        .await
        .expect("create users resource failed");

    client
        .create_resource()
        .rest_api_id(api_id)
        .parent_id(root_resource_id)
        .path_part("orders")
        .send()
        .await
        .expect("create orders resource failed");

    let resp = client
        .get_resources()
        .rest_api_id(api_id)
        .send()
        .await
        .expect("get_resources failed");

    let items = resp.items();
    // root + 2 children = 3
    assert_eq!(items.len(), 3);
    let paths: Vec<&str> = items.iter().filter_map(|r| r.path()).collect();
    assert!(paths.contains(&"/"));
    assert!(paths.contains(&"/users"));
    assert!(paths.contains(&"/orders"));
}

#[tokio::test]
async fn test_get_resource() {
    let client = make_client().await;

    let api = client
        .create_rest_api()
        .name("get-resource-test")
        .send()
        .await
        .expect("create api failed");

    let api_id = api.id().unwrap();
    let root_resource_id = api.root_resource_id().unwrap();

    let created = client
        .create_resource()
        .rest_api_id(api_id)
        .parent_id(root_resource_id)
        .path_part("pets")
        .send()
        .await
        .expect("create resource failed");

    let resource_id = created.id().unwrap();

    let got = client
        .get_resource()
        .rest_api_id(api_id)
        .resource_id(resource_id)
        .send()
        .await
        .expect("get_resource failed");

    assert_eq!(got.id().unwrap_or_default(), resource_id);
    assert_eq!(got.path_part().unwrap_or_default(), "pets");
    assert_eq!(got.path().unwrap_or_default(), "/pets");
}

#[tokio::test]
async fn test_delete_resource() {
    let client = make_client().await;

    let api = client
        .create_rest_api()
        .name("delete-resource-test")
        .send()
        .await
        .expect("create api failed");

    let api_id = api.id().unwrap();
    let root_resource_id = api.root_resource_id().unwrap();

    let resource_id = client
        .create_resource()
        .rest_api_id(api_id)
        .parent_id(root_resource_id)
        .path_part("temp")
        .send()
        .await
        .expect("create resource failed")
        .id()
        .unwrap()
        .to_string();

    client
        .delete_resource()
        .rest_api_id(api_id)
        .resource_id(&resource_id)
        .send()
        .await
        .expect("delete_resource failed");

    let err = client
        .get_resource()
        .rest_api_id(api_id)
        .resource_id(&resource_id)
        .send()
        .await
        .expect_err("should fail after deletion");

    assert!(format!("{err:?}").contains("NotFoundException") || format!("{err:?}").contains("404"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_apigateway::ApiGatewayStateView;
    use winterbaume_core::StatefulService;

    let svc = ApiGatewayService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);

    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", ApiGatewayStateView::default())
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
async fn test_state_view_restore_and_snapshot() {
    use std::collections::HashMap;

    use winterbaume_apigateway::views::{ApiGatewayStateView, RestApiView};
    use winterbaume_core::StatefulService;

    let svc = ApiGatewayService::new();

    let mut rest_apis = HashMap::new();
    rest_apis.insert(
        "abc1234567".to_string(),
        RestApiView {
            id: "abc1234567".to_string(),
            name: "restored-api".to_string(),
            description: None,
            version: None,
            created_date: 1700000000.0,
            endpoint_types: vec![],
            vpc_endpoint_ids: vec![],
            tags: HashMap::new(),
            disable_execute_api_endpoint: None,
            policy: None,
            api_key_source: None,
            binary_media_types: vec![],
            minimum_compression_size: None,
            root_resource_id: "root0000001".to_string(),
        },
    );

    let view = ApiGatewayStateView {
        rest_apis,
        resources: HashMap::new(),
        ..Default::default()
    };

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snapshot.rest_apis.contains_key("abc1234567"));
    assert_eq!(snapshot.rest_apis["abc1234567"].name, "restored-api");
}

// ---- Group 1: API Key operations ----

#[tokio::test]
async fn test_api_key_crud() {
    let client = make_client().await;

    let create_resp = client
        .create_api_key()
        .name("my-key")
        .description("test key")
        .enabled(true)
        .send()
        .await
        .expect("create_api_key failed");

    let key_id = create_resp.id().expect("id should be set").to_string();
    assert_eq!(create_resp.name().unwrap_or_default(), "my-key");
    assert_eq!(create_resp.description().unwrap_or_default(), "test key");
    assert!(create_resp.enabled());
    assert!(create_resp.value().is_some());

    let get_resp = client
        .get_api_key()
        .api_key(&key_id)
        .send()
        .await
        .expect("get_api_key failed");
    assert_eq!(get_resp.id().unwrap_or_default(), key_id);

    let list_resp = client
        .get_api_keys()
        .send()
        .await
        .expect("get_api_keys failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_api_key()
        .api_key(&key_id)
        .send()
        .await
        .expect("delete_api_key failed");

    client
        .get_api_key()
        .api_key(&key_id)
        .send()
        .await
        .expect_err("should be not found after delete");
}

// ---- Group 2: Domain Name and Base Path Mapping operations ----

#[tokio::test]
async fn test_domain_name_crud() {
    let client = make_client().await;

    client
        .create_domain_name()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("create_domain_name failed");

    let get_resp = client
        .get_domain_name()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("get_domain_name failed");
    assert_eq!(
        get_resp.domain_name().unwrap_or_default(),
        "api.example.com"
    );

    let list_resp = client
        .get_domain_names()
        .send()
        .await
        .expect("get_domain_names failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_domain_name()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("delete_domain_name failed");

    client
        .get_domain_name()
        .domain_name("api.example.com")
        .send()
        .await
        .expect_err("should be not found after delete");
}

#[tokio::test]
async fn test_base_path_mapping_crud() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("test-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    client
        .create_domain_name()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("create_domain_name failed");

    let create_resp = client
        .create_base_path_mapping()
        .domain_name("api.example.com")
        .rest_api_id(&api_id)
        .base_path("v1")
        .send()
        .await
        .expect("create_base_path_mapping failed");

    assert_eq!(create_resp.base_path().unwrap_or_default(), "v1");
    assert_eq!(create_resp.rest_api_id().unwrap_or_default(), api_id);

    let get_resp = client
        .get_base_path_mapping()
        .domain_name("api.example.com")
        .base_path("v1")
        .send()
        .await
        .expect("get_base_path_mapping failed");
    assert_eq!(get_resp.base_path().unwrap_or_default(), "v1");

    let list_resp = client
        .get_base_path_mappings()
        .domain_name("api.example.com")
        .send()
        .await
        .expect("get_base_path_mappings failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_base_path_mapping()
        .domain_name("api.example.com")
        .base_path("v1")
        .send()
        .await
        .expect("delete_base_path_mapping failed");
}

// ---- Group 4: Gateway Response operations ----

#[tokio::test]
async fn test_gateway_response_crud() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("gw-resp-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_apigateway::types::GatewayResponseType;
    client
        .put_gateway_response()
        .rest_api_id(&api_id)
        .response_type(GatewayResponseType::Default4Xx)
        .status_code("400")
        .send()
        .await
        .expect("put_gateway_response failed");

    let get_resp = client
        .get_gateway_response()
        .rest_api_id(&api_id)
        .response_type(GatewayResponseType::Default4Xx)
        .send()
        .await
        .expect("get_gateway_response failed");
    assert_eq!(get_resp.status_code().unwrap_or_default(), "400");

    let list_resp = client
        .get_gateway_responses()
        .rest_api_id(&api_id)
        .send()
        .await
        .expect("get_gateway_responses failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_gateway_response()
        .rest_api_id(&api_id)
        .response_type(GatewayResponseType::Default4Xx)
        .send()
        .await
        .expect("delete_gateway_response failed");
}

// ---- Group 5: Request Validator operations ----

#[tokio::test]
async fn test_request_validator_crud() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("rv-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_request_validator()
        .rest_api_id(&api_id)
        .name("body-validator")
        .validate_request_body(true)
        .validate_request_parameters(false)
        .send()
        .await
        .expect("create_request_validator failed");

    let validator_id = create_resp.id().expect("id should be set").to_string();
    assert_eq!(create_resp.name().unwrap_or_default(), "body-validator");
    assert!(create_resp.validate_request_body());
    assert!(!create_resp.validate_request_parameters());

    let get_resp = client
        .get_request_validator()
        .rest_api_id(&api_id)
        .request_validator_id(&validator_id)
        .send()
        .await
        .expect("get_request_validator failed");
    assert_eq!(get_resp.id().unwrap_or_default(), validator_id);

    let list_resp = client
        .get_request_validators()
        .rest_api_id(&api_id)
        .send()
        .await
        .expect("get_request_validators failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_request_validator()
        .rest_api_id(&api_id)
        .request_validator_id(&validator_id)
        .send()
        .await
        .expect("delete_request_validator failed");

    client
        .get_request_validator()
        .rest_api_id(&api_id)
        .request_validator_id(&validator_id)
        .send()
        .await
        .expect_err("should be not found after delete");
}

// ---- Group 6: Usage Plan operations ----

#[tokio::test]
async fn test_usage_plan_crud() {
    let client = make_client().await;

    let create_resp = client
        .create_usage_plan()
        .name("my-plan")
        .description("test plan")
        .send()
        .await
        .expect("create_usage_plan failed");

    let plan_id = create_resp.id().expect("id should be set").to_string();
    assert_eq!(create_resp.name().unwrap_or_default(), "my-plan");
    assert_eq!(create_resp.description().unwrap_or_default(), "test plan");

    let get_resp = client
        .get_usage_plan()
        .usage_plan_id(&plan_id)
        .send()
        .await
        .expect("get_usage_plan failed");
    assert_eq!(get_resp.id().unwrap_or_default(), plan_id);

    let list_resp = client
        .get_usage_plans()
        .send()
        .await
        .expect("get_usage_plans failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_usage_plan()
        .usage_plan_id(&plan_id)
        .send()
        .await
        .expect("delete_usage_plan failed");

    client
        .get_usage_plan()
        .usage_plan_id(&plan_id)
        .send()
        .await
        .expect_err("should be not found after delete");
}

#[tokio::test]
async fn test_usage_plan_key_crud() {
    let client = make_client().await;

    let key_id = client
        .create_api_key()
        .name("plan-key")
        .enabled(true)
        .send()
        .await
        .expect("create_api_key failed")
        .id()
        .unwrap()
        .to_string();

    let plan_id = client
        .create_usage_plan()
        .name("keyed-plan")
        .send()
        .await
        .expect("create_usage_plan failed")
        .id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_usage_plan_key()
        .usage_plan_id(&plan_id)
        .key_id(&key_id)
        .key_type("API_KEY")
        .send()
        .await
        .expect("create_usage_plan_key failed");
    assert_eq!(create_resp.id().unwrap_or_default(), key_id);

    let get_resp = client
        .get_usage_plan_key()
        .usage_plan_id(&plan_id)
        .key_id(&key_id)
        .send()
        .await
        .expect("get_usage_plan_key failed");
    assert_eq!(get_resp.id().unwrap_or_default(), key_id);

    let list_resp = client
        .get_usage_plan_keys()
        .usage_plan_id(&plan_id)
        .send()
        .await
        .expect("get_usage_plan_keys failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_usage_plan_key()
        .usage_plan_id(&plan_id)
        .key_id(&key_id)
        .send()
        .await
        .expect("delete_usage_plan_key failed");
}

// ---- Group 7: VPC Link operations ----

#[tokio::test]
async fn test_vpc_link_crud() {
    let client = make_client().await;

    let create_resp = client
        .create_vpc_link()
        .name("my-vpc-link")
        .target_arns("arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/net/my-nlb/1234567890abcdef")
        .send()
        .await
        .expect("create_vpc_link failed");

    let link_id = create_resp.id().expect("id should be set").to_string();
    assert_eq!(create_resp.name().unwrap_or_default(), "my-vpc-link");

    let get_resp = client
        .get_vpc_link()
        .vpc_link_id(&link_id)
        .send()
        .await
        .expect("get_vpc_link failed");
    assert_eq!(get_resp.id().unwrap_or_default(), link_id);

    let list_resp = client
        .get_vpc_links()
        .send()
        .await
        .expect("get_vpc_links failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_vpc_link()
        .vpc_link_id(&link_id)
        .send()
        .await
        .expect("delete_vpc_link failed");

    client
        .get_vpc_link()
        .vpc_link_id(&link_id)
        .send()
        .await
        .expect_err("should be not found after delete");
}

// ---- Group 8: Other operations ----

#[tokio::test]
async fn test_get_account() {
    let client = make_client().await;
    // Returns successfully even with default/empty state
    client
        .get_account()
        .send()
        .await
        .expect("get_account failed");
}

#[tokio::test]
async fn test_update_authorizer() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("auth-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_apigateway::types::AuthorizerType;
    let authorizer_id = client
        .create_authorizer()
        .rest_api_id(&api_id)
        .name("my-auth")
        .r#type(AuthorizerType::Token)
        .send()
        .await
        .expect("create_authorizer failed")
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_apigateway::types::Op;
    use aws_sdk_apigateway::types::builders::PatchOperationBuilder;
    let update_resp = client
        .update_authorizer()
        .rest_api_id(&api_id)
        .authorizer_id(&authorizer_id)
        .patch_operations(
            PatchOperationBuilder::default()
                .op(Op::Replace)
                .path("/name")
                .value("updated-auth")
                .build(),
        )
        .send()
        .await
        .expect("update_authorizer failed");

    assert_eq!(update_resp.id().unwrap_or_default(), authorizer_id);
}

#[tokio::test]
async fn test_update_rest_api() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("orig-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_apigateway::types::Op;
    use aws_sdk_apigateway::types::builders::PatchOperationBuilder;
    let update_resp = client
        .update_rest_api()
        .rest_api_id(&api_id)
        .patch_operations(
            PatchOperationBuilder::default()
                .op(Op::Replace)
                .path("/name")
                .value("new-name")
                .build(),
        )
        .send()
        .await
        .expect("update_rest_api failed");

    assert_eq!(update_resp.id().unwrap_or_default(), api_id);
}

// ---- New operations: Client Certificate ----

#[tokio::test]
async fn test_client_certificate_crud() {
    let client = make_client().await;

    let gen_resp = client
        .generate_client_certificate()
        .description("test cert")
        .send()
        .await
        .expect("generate_client_certificate failed");

    let cert_id = gen_resp
        .client_certificate_id()
        .expect("client_certificate_id should be set")
        .to_string();
    assert_eq!(gen_resp.description().unwrap_or_default(), "test cert");
    assert!(gen_resp.pem_encoded_certificate().is_some());

    let get_resp = client
        .get_client_certificate()
        .client_certificate_id(&cert_id)
        .send()
        .await
        .expect("get_client_certificate failed");
    assert_eq!(
        get_resp.client_certificate_id().unwrap_or_default(),
        cert_id
    );

    let list_resp = client
        .get_client_certificates()
        .send()
        .await
        .expect("get_client_certificates failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .update_client_certificate()
        .client_certificate_id(&cert_id)
        .send()
        .await
        .expect("update_client_certificate failed");

    client
        .delete_client_certificate()
        .client_certificate_id(&cert_id)
        .send()
        .await
        .expect("delete_client_certificate failed");

    client
        .get_client_certificate()
        .client_certificate_id(&cert_id)
        .send()
        .await
        .expect_err("should be not found after delete");
}

// ---- New operations: Documentation Part ----

#[tokio::test]
async fn test_documentation_part_crud() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("doc-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_apigateway::types::DocumentationPartType;
    use aws_sdk_apigateway::types::builders::DocumentationPartLocationBuilder;

    let create_resp = client
        .create_documentation_part()
        .rest_api_id(&api_id)
        .location(
            DocumentationPartLocationBuilder::default()
                .r#type(DocumentationPartType::Api)
                .build()
                .unwrap(),
        )
        .properties("{\"description\":\"my API\"}")
        .send()
        .await
        .expect("create_documentation_part failed");

    let part_id = create_resp.id().expect("id should be set").to_string();
    assert!(create_resp.properties().is_some());

    let get_resp = client
        .get_documentation_part()
        .rest_api_id(&api_id)
        .documentation_part_id(&part_id)
        .send()
        .await
        .expect("get_documentation_part failed");
    assert_eq!(get_resp.id().unwrap_or_default(), part_id);

    let list_resp = client
        .get_documentation_parts()
        .rest_api_id(&api_id)
        .send()
        .await
        .expect("get_documentation_parts failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_documentation_part()
        .rest_api_id(&api_id)
        .documentation_part_id(&part_id)
        .send()
        .await
        .expect("delete_documentation_part failed");

    client
        .get_documentation_part()
        .rest_api_id(&api_id)
        .documentation_part_id(&part_id)
        .send()
        .await
        .expect_err("should be not found after delete");
}

// ---- New operations: Documentation Version ----

#[tokio::test]
async fn test_documentation_version_crud() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("docver-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    let create_resp = client
        .create_documentation_version()
        .rest_api_id(&api_id)
        .documentation_version("1.0")
        .description("version one")
        .send()
        .await
        .expect("create_documentation_version failed");

    assert_eq!(create_resp.version().unwrap_or_default(), "1.0");
    assert_eq!(create_resp.description().unwrap_or_default(), "version one");

    let get_resp = client
        .get_documentation_version()
        .rest_api_id(&api_id)
        .documentation_version("1.0")
        .send()
        .await
        .expect("get_documentation_version failed");
    assert_eq!(get_resp.version().unwrap_or_default(), "1.0");

    let list_resp = client
        .get_documentation_versions()
        .rest_api_id(&api_id)
        .send()
        .await
        .expect("get_documentation_versions failed");
    assert_eq!(list_resp.items().len(), 1);

    client
        .delete_documentation_version()
        .rest_api_id(&api_id)
        .documentation_version("1.0")
        .send()
        .await
        .expect("delete_documentation_version failed");

    client
        .get_documentation_version()
        .rest_api_id(&api_id)
        .documentation_version("1.0")
        .send()
        .await
        .expect_err("should be not found after delete");
}

// ---- New operations: Tags ----

#[tokio::test]
async fn test_tag_operations() {
    let client = make_client().await;

    let arn = "arn:aws:apigateway:us-east-1::/restapis/abc123";

    client
        .tag_resource()
        .resource_arn(arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource failed");

    let get_resp = client
        .get_tags()
        .resource_arn(arn)
        .send()
        .await
        .expect("get_tags failed");

    let tags = get_resp.tags().expect("tags should be present");
    assert_eq!(tags.get("env").map(String::as_str), Some("prod"));
    assert_eq!(tags.get("team").map(String::as_str), Some("platform"));

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource failed");

    let get_resp2 = client
        .get_tags()
        .resource_arn(arn)
        .send()
        .await
        .expect("get_tags after untag failed");

    let tags2 = get_resp2
        .tags()
        .expect("tags should be present after untag");
    assert!(tags2.get("team").is_none(), "team tag should be removed");
    assert_eq!(tags2.get("env").map(String::as_str), Some("prod"));
}

// ---- New operations: SDK types ----

#[tokio::test]
async fn test_sdk_types() {
    let client = make_client().await;

    let list_resp = client
        .get_sdk_types()
        .send()
        .await
        .expect("get_sdk_types failed");
    assert!(!list_resp.items().is_empty());

    let get_resp = client
        .get_sdk_type()
        .id("javascript")
        .send()
        .await
        .expect("get_sdk_type failed");
    assert_eq!(get_resp.id().unwrap_or_default(), "javascript");
}

// ---- New operations: GetSdk ----

#[tokio::test]
async fn test_get_sdk() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("sdk-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    let deployment_id = client
        .create_deployment()
        .rest_api_id(&api_id)
        .send()
        .await
        .expect("create_deployment failed")
        .id()
        .unwrap()
        .to_string();

    client
        .create_stage()
        .rest_api_id(&api_id)
        .stage_name("prod")
        .deployment_id(&deployment_id)
        .send()
        .await
        .expect("create_stage failed");

    let sdk_resp = client
        .get_sdk()
        .rest_api_id(&api_id)
        .stage_name("prod")
        .sdk_type("javascript")
        .send()
        .await
        .expect("get_sdk failed");

    assert!(sdk_resp.body().is_some());
}

// ---- New operations: GetExport ----

#[tokio::test]
async fn test_get_export() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("export-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    let deployment_id = client
        .create_deployment()
        .rest_api_id(&api_id)
        .send()
        .await
        .expect("create_deployment failed")
        .id()
        .unwrap()
        .to_string();

    client
        .create_stage()
        .rest_api_id(&api_id)
        .stage_name("prod")
        .deployment_id(&deployment_id)
        .send()
        .await
        .expect("create_stage failed");

    let export_resp = client
        .get_export()
        .rest_api_id(&api_id)
        .stage_name("prod")
        .export_type("oas30")
        .send()
        .await
        .expect("get_export failed");

    assert!(export_resp.body().is_some());
}

// ---- New operations: FlushStageAuthorizersCache ----

#[tokio::test]
async fn test_flush_stage_authorizers_cache() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("flush-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    let deployment_id = client
        .create_deployment()
        .rest_api_id(&api_id)
        .send()
        .await
        .expect("create_deployment failed")
        .id()
        .unwrap()
        .to_string();

    client
        .create_stage()
        .rest_api_id(&api_id)
        .stage_name("prod")
        .deployment_id(&deployment_id)
        .send()
        .await
        .expect("create_stage failed");

    client
        .flush_stage_authorizers_cache()
        .rest_api_id(&api_id)
        .stage_name("prod")
        .send()
        .await
        .expect("flush_stage_authorizers_cache failed");
}

// ---- New operations: UpdateDeployment, UpdateVpcLink ----

#[tokio::test]
async fn test_update_deployment() {
    let client = make_client().await;

    let api_id = client
        .create_rest_api()
        .name("update-dep-api")
        .send()
        .await
        .expect("create_rest_api failed")
        .id()
        .unwrap()
        .to_string();

    let deployment_id = client
        .create_deployment()
        .rest_api_id(&api_id)
        .description("original")
        .send()
        .await
        .expect("create_deployment failed")
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_apigateway::types::Op;
    use aws_sdk_apigateway::types::builders::PatchOperationBuilder;
    let update_resp = client
        .update_deployment()
        .rest_api_id(&api_id)
        .deployment_id(&deployment_id)
        .patch_operations(
            PatchOperationBuilder::default()
                .op(Op::Replace)
                .path("/description")
                .value("updated")
                .build(),
        )
        .send()
        .await
        .expect("update_deployment failed");

    assert_eq!(update_resp.id().unwrap_or_default(), deployment_id);
}

#[tokio::test]
async fn test_update_vpc_link() {
    let client = make_client().await;

    let link_id = client
        .create_vpc_link()
        .name("my-link")
        .target_arns(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/net/my-nlb/abc",
        )
        .send()
        .await
        .expect("create_vpc_link failed")
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_apigateway::types::Op;
    use aws_sdk_apigateway::types::builders::PatchOperationBuilder;
    let update_resp = client
        .update_vpc_link()
        .vpc_link_id(&link_id)
        .patch_operations(
            PatchOperationBuilder::default()
                .op(Op::Replace)
                .path("/description")
                .value("updated link")
                .build(),
        )
        .send()
        .await
        .expect("update_vpc_link failed");

    assert_eq!(update_resp.id().unwrap_or_default(), link_id);
}

// ---- New operations: GetUsage ----

#[tokio::test]
async fn test_get_usage() {
    let client = make_client().await;

    let plan_id = client
        .create_usage_plan()
        .name("usage-plan")
        .send()
        .await
        .expect("create_usage_plan failed")
        .id()
        .unwrap()
        .to_string();

    let usage_resp = client
        .get_usage()
        .usage_plan_id(&plan_id)
        .start_date("2024-01-01")
        .end_date("2024-01-31")
        .send()
        .await
        .expect("get_usage failed");

    assert_eq!(usage_resp.usage_plan_id().unwrap_or_default(), plan_id);
}
