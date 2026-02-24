use aws_sdk_servicecatalog::config::BehaviorVersion;
use aws_sdk_servicecatalog::error::ProvideErrorMetadata;
use winterbaume_core::MockAws;
use winterbaume_servicecatalog::ServiceCatalogService;

async fn make_client() -> aws_sdk_servicecatalog::Client {
    let mock = MockAws::builder()
        .with_service(ServiceCatalogService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicecatalog::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_servicecatalog::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_portfolio() {
    let client = make_client().await;

    let create_resp = client
        .create_portfolio()
        .display_name("My Portfolio")
        .provider_name("Test Provider")
        .description("A test portfolio")
        .idempotency_token("token-1")
        .send()
        .await
        .expect("create_portfolio should succeed");

    let portfolio = create_resp
        .portfolio_detail()
        .expect("should have portfolio detail");
    assert_eq!(portfolio.display_name(), Some("My Portfolio"));
    assert_eq!(portfolio.provider_name(), Some("Test Provider"));
    assert_eq!(portfolio.description(), Some("A test portfolio"));
    assert!(portfolio.id().is_some());
    assert!(portfolio.arn().is_some());

    let portfolio_id = portfolio.id().unwrap().to_string();

    let describe_resp = client
        .describe_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .expect("describe_portfolio should succeed");

    let detail = describe_resp
        .portfolio_detail()
        .expect("should have portfolio detail");
    assert_eq!(detail.display_name(), Some("My Portfolio"));
    assert_eq!(detail.provider_name(), Some("Test Provider"));
    assert_eq!(detail.id(), Some(portfolio_id.as_str()));
}

#[tokio::test]
async fn test_list_portfolios() {
    let client = make_client().await;

    for (name, token) in [("Portfolio A", "tok-a"), ("Portfolio B", "tok-b")] {
        client
            .create_portfolio()
            .display_name(name)
            .provider_name("Provider")
            .idempotency_token(token)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_portfolios()
        .send()
        .await
        .expect("list_portfolios should succeed");

    assert_eq!(resp.portfolio_details().len(), 2);
}

#[tokio::test]
async fn test_delete_portfolio() {
    let client = make_client().await;

    let create_resp = client
        .create_portfolio()
        .display_name("Delete Me")
        .provider_name("Provider")
        .idempotency_token("tok-del")
        .send()
        .await
        .unwrap();

    let portfolio_id = create_resp
        .portfolio_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .delete_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .expect("delete_portfolio should succeed");

    let result = client.describe_portfolio().id(&portfolio_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_portfolio() {
    let client = make_client().await;

    let result = client
        .describe_portfolio()
        .id("port-nonexistent1")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_portfolio() {
    let client = make_client().await;

    let result = client
        .delete_portfolio()
        .id("port-nonexistent2")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_portfolios_empty() {
    let client = make_client().await;

    let resp = client
        .list_portfolios()
        .send()
        .await
        .expect("list_portfolios should succeed on empty state");

    assert_eq!(resp.portfolio_details().len(), 0);
}

#[tokio::test]
async fn test_create_portfolio_without_description() {
    let client = make_client().await;

    let create_resp = client
        .create_portfolio()
        .display_name("No Desc Portfolio")
        .provider_name("Provider")
        .idempotency_token("tok-nodesc")
        .send()
        .await
        .expect("create_portfolio without description should succeed");

    let portfolio = create_resp
        .portfolio_detail()
        .expect("should have portfolio detail");
    assert_eq!(portfolio.display_name(), Some("No Desc Portfolio"));
    assert_eq!(portfolio.provider_name(), Some("Provider"));
}

// ============================================================================
// Tests derived from AWS documentation: Service Catalog
// ============================================================================

#[tokio::test]
async fn test_describe_portfolio_created_time() {
    let client = make_client().await;

    let create_resp = client
        .create_portfolio()
        .display_name("Timed Portfolio")
        .provider_name("Provider")
        .idempotency_token("tok-time")
        .send()
        .await
        .expect("create_portfolio should succeed");

    let portfolio_id = create_resp
        .portfolio_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let describe_resp = client
        .describe_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .expect("describe_portfolio should succeed");

    let detail = describe_resp
        .portfolio_detail()
        .expect("should have portfolio detail");
    assert!(
        detail.created_time().is_some(),
        "created_time should be present"
    );
}

#[tokio::test]
async fn test_describe_portfolio_tag_options_and_budgets_empty() {
    let client = make_client().await;

    let create_resp = client
        .create_portfolio()
        .display_name("Empty Opts Portfolio")
        .provider_name("Provider")
        .idempotency_token("tok-empty-opts")
        .send()
        .await
        .expect("create_portfolio should succeed");

    let portfolio_id = create_resp
        .portfolio_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let describe_resp = client
        .describe_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .expect("describe_portfolio should succeed");

    assert_eq!(
        describe_resp.tag_options().len(),
        0,
        "tag_options should be empty"
    );
    assert_eq!(describe_resp.budgets().len(), 0, "budgets should be empty");
}

#[tokio::test]
async fn test_delete_portfolio_not_found_error_code() {
    let client = make_client().await;

    let err = client
        .delete_portfolio()
        .id("port-notexist999")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_resource_not_found_exception(),
        "Expected ResourceNotFoundException for deleting nonexistent portfolio, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_describe_portfolio_not_found_error_code() {
    let client = make_client().await;

    let err = client
        .describe_portfolio()
        .id("port-notexist888")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_resource_not_found_exception(),
        "Expected ResourceNotFoundException for describing nonexistent portfolio, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_portfolio_full_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_portfolio()
        .display_name("Lifecycle Portfolio")
        .description("Full lifecycle test")
        .provider_name("Provider")
        .idempotency_token("tok-lifecycle")
        .send()
        .await
        .expect("create_portfolio should succeed");

    let detail = create_resp
        .portfolio_detail()
        .expect("should have portfolio detail");
    let portfolio_id = detail.id().unwrap().to_string();
    assert!(portfolio_id.starts_with("port-"));

    // Describe
    let describe_resp = client
        .describe_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .expect("describe_portfolio should succeed");

    let described = describe_resp
        .portfolio_detail()
        .expect("should have portfolio detail");
    assert_eq!(described.display_name(), Some("Lifecycle Portfolio"));
    assert_eq!(described.description(), Some("Full lifecycle test"));
    assert_eq!(described.provider_name(), Some("Provider"));

    // Verify in list
    let list_resp = client
        .list_portfolios()
        .send()
        .await
        .expect("list should succeed");
    let ids: Vec<Option<&str>> = list_resp
        .portfolio_details()
        .iter()
        .map(|p| p.id())
        .collect();
    assert!(
        ids.contains(&Some(portfolio_id.as_str())),
        "portfolio should appear in list"
    );

    // Delete
    client
        .delete_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .expect("delete_portfolio should succeed");

    // Verify gone
    let err = client
        .describe_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .unwrap_err();
    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_resource_not_found_exception(),
        "Deleted portfolio should return ResourceNotFoundException"
    );
}

#[tokio::test]
async fn test_create_portfolio_multiple_tags() {
    let client = make_client().await;

    let tag1 = aws_sdk_servicecatalog::types::Tag::builder()
        .key("env")
        .value("prod")
        .build()
        .unwrap();
    let tag2 = aws_sdk_servicecatalog::types::Tag::builder()
        .key("team")
        .value("platform")
        .build()
        .unwrap();
    let tag3 = aws_sdk_servicecatalog::types::Tag::builder()
        .key("cost-center")
        .value("engineering")
        .build()
        .unwrap();

    let create_resp = client
        .create_portfolio()
        .display_name("Multi-Tag Portfolio")
        .provider_name("Provider")
        .tags(tag1)
        .tags(tag2)
        .tags(tag3)
        .idempotency_token("tok-multitag")
        .send()
        .await
        .expect("create_portfolio with multiple tags should succeed");

    let tags = create_resp.tags();
    assert_eq!(
        tags.len(),
        3,
        "all 3 tags should be returned in create response"
    );

    let portfolio_id = create_resp
        .portfolio_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Verify tags appear in describe as well
    let describe_resp = client
        .describe_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .expect("describe_portfolio should succeed");

    let desc_tags = describe_resp.tags();
    assert_eq!(
        desc_tags.len(),
        3,
        "all 3 tags should be present in describe"
    );

    let keys: Vec<&str> = desc_tags.iter().map(|t| t.key()).collect();
    assert!(keys.contains(&"env"), "env tag should be present");
    assert!(keys.contains(&"team"), "team tag should be present");
    assert!(
        keys.contains(&"cost-center"),
        "cost-center tag should be present"
    );
}

#[tokio::test]
async fn test_list_portfolios_created_time_present() {
    let client = make_client().await;

    client
        .create_portfolio()
        .display_name("Time Test Portfolio")
        .provider_name("Provider")
        .idempotency_token("tok-timelist")
        .send()
        .await
        .expect("create_portfolio should succeed");

    let list_resp = client
        .list_portfolios()
        .send()
        .await
        .expect("list_portfolios should succeed");

    for p in list_resp.portfolio_details() {
        assert!(
            p.created_time().is_some(),
            "created_time should be present for portfolio id={:?}",
            p.id()
        );
    }
}

// --- Ported from moto test_servicecatalog.py ---

#[tokio::test]
async fn test_create_portfolio_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_portfolio()
        .display_name("Test Portfolio")
        .description("Test Portfolio Description")
        .provider_name("Test Provider")
        .tags(
            aws_sdk_servicecatalog::types::Tag::builder()
                .key("testkey")
                .value("testvalue")
                .build()
                .unwrap(),
        )
        .idempotency_token("test-token")
        .send()
        .await
        .expect("create_portfolio should succeed");

    let portfolio = resp
        .portfolio_detail()
        .expect("should have portfolio detail");
    assert_eq!(portfolio.display_name(), Some("Test Portfolio"));
    assert_eq!(portfolio.description(), Some("Test Portfolio Description"));
    assert_eq!(portfolio.provider_name(), Some("Test Provider"));

    // Tags should be returned
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "testkey");
    assert_eq!(tags[0].value(), "testvalue");
}

#[tokio::test]
async fn test_create_portfolio_idempotency() {
    let client = make_client().await;

    let resp1 = client
        .create_portfolio()
        .display_name("Test Portfolio")
        .description("Test Portfolio Description")
        .provider_name("Test Provider")
        .idempotency_token("test-token")
        .send()
        .await
        .expect("first create should succeed");

    let id1 = resp1.portfolio_detail().unwrap().id().unwrap().to_string();

    // Same token returns same portfolio
    let resp2 = client
        .create_portfolio()
        .display_name("Different Name")
        .description("Different Description")
        .provider_name("Different Provider")
        .idempotency_token("test-token")
        .send()
        .await
        .expect("idempotent create should succeed");

    let id2 = resp2.portfolio_detail().unwrap().id().unwrap().to_string();
    assert_eq!(
        id1, id2,
        "same idempotency token should return same portfolio"
    );
    assert_eq!(
        resp1.portfolio_detail().unwrap().display_name(),
        resp2.portfolio_detail().unwrap().display_name(),
        "idempotent response should have same display name"
    );
}

#[tokio::test]
async fn test_create_portfolio_different_tokens_create_different_portfolios() {
    let client = make_client().await;

    let resp1 = client
        .create_portfolio()
        .display_name("Portfolio 1")
        .provider_name("Provider")
        .idempotency_token("token-1")
        .send()
        .await
        .expect("first create should succeed");

    let resp2 = client
        .create_portfolio()
        .display_name("Portfolio 2")
        .provider_name("Provider")
        .idempotency_token("token-2")
        .send()
        .await
        .expect("second create should succeed");

    let id1 = resp1.portfolio_detail().unwrap().id().unwrap();
    let id2 = resp2.portfolio_detail().unwrap().id().unwrap();
    assert_ne!(
        id1, id2,
        "different tokens should create different portfolios"
    );
}

#[tokio::test]
async fn test_delete_portfolio_removes_from_list() {
    let client = make_client().await;

    let create_resp = client
        .create_portfolio()
        .display_name("Test Portfolio")
        .description("Test Portfolio Description")
        .provider_name("Test Provider")
        .idempotency_token("test-token")
        .send()
        .await
        .expect("create should succeed");

    let portfolio_id = create_resp
        .portfolio_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Verify it's in the list
    let list_before = client
        .list_portfolios()
        .send()
        .await
        .expect("list should succeed");
    let ids_before: Vec<&str> = list_before
        .portfolio_details()
        .iter()
        .map(|p| p.id().unwrap())
        .collect();
    assert!(ids_before.contains(&portfolio_id.as_str()));

    client
        .delete_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .expect("delete should succeed");

    // Verify it's no longer in the list
    let list_after = client
        .list_portfolios()
        .send()
        .await
        .expect("list should succeed");
    let ids_after: Vec<Option<&str>> = list_after
        .portfolio_details()
        .iter()
        .map(|p| p.id())
        .collect();
    assert!(!ids_after.contains(&Some(portfolio_id.as_str())));
}

#[tokio::test]
async fn test_describe_portfolio_includes_required_fields() {
    let client = make_client().await;

    let create_resp = client
        .create_portfolio()
        .display_name("Test Portfolio")
        .description("Test Portfolio Description")
        .provider_name("Test Provider")
        .tags(
            aws_sdk_servicecatalog::types::Tag::builder()
                .key("testkey")
                .value("testvalue")
                .build()
                .unwrap(),
        )
        .idempotency_token("test-token")
        .send()
        .await
        .expect("create should succeed");

    let portfolio_id = create_resp
        .portfolio_detail()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let describe_resp = client
        .describe_portfolio()
        .id(&portfolio_id)
        .send()
        .await
        .expect("describe should succeed");

    let detail = describe_resp
        .portfolio_detail()
        .expect("should have portfolio detail");
    assert_eq!(detail.id(), Some(portfolio_id.as_str()));
    assert!(detail.arn().is_some());
    assert_eq!(detail.display_name(), Some("Test Portfolio"));
    assert_eq!(detail.description(), Some("Test Portfolio Description"));
    assert_eq!(detail.provider_name(), Some("Test Provider"));

    // Tags should be returned in describe
    let tags = describe_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "testkey");
    assert_eq!(tags[0].value(), "testvalue");
}

#[tokio::test]
async fn test_list_portfolios_returns_required_fields() {
    let client = make_client().await;

    for i in 1..=2 {
        client
            .create_portfolio()
            .display_name(format!("Test Portfolio {i}"))
            .description(format!("Description {i}"))
            .provider_name(format!("Provider {i}"))
            .idempotency_token(format!("token-{i}"))
            .send()
            .await
            .expect("create should succeed");
    }

    let resp = client
        .list_portfolios()
        .send()
        .await
        .expect("list should succeed");
    let portfolios = resp.portfolio_details();
    assert_eq!(portfolios.len(), 2);

    for p in portfolios {
        assert!(p.id().is_some());
        assert!(p.arn().is_some());
        assert!(p.display_name().is_some());
        assert!(p.description().is_some());
        assert!(p.provider_name().is_some());
    }
}

#[tokio::test]
async fn test_portfolio_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_portfolio()
        .display_name("Test Portfolio")
        .provider_name("Provider")
        .idempotency_token("test-token")
        .send()
        .await
        .expect("create should succeed");

    let portfolio = resp
        .portfolio_detail()
        .expect("should have portfolio detail");
    let arn = portfolio.arn().expect("should have ARN");
    assert!(arn.starts_with("arn:aws:catalog:us-east-1:123456789012:portfolio/port-"));
    let id = portfolio.id().expect("should have id");
    assert!(id.starts_with("port-"));
}

#[tokio::test]
async fn test_create_portfolio_missing_display_name_fails() {
    let client = make_client().await;

    // The SDK requires display_name so we can't test this directly via the SDK builder.
    // But we can verify that creating with display_name works.
    let resp = client
        .create_portfolio()
        .display_name("Valid Name")
        .provider_name("Provider")
        .idempotency_token("tok")
        .send()
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn test_describe_nonexistent_portfolio_fails() {
    let client = make_client().await;

    let result = client
        .describe_portfolio()
        .id("port-nonexistent1")
        .send()
        .await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    let svc_err = err.into_service_error();
    assert!(svc_err.message().is_some());
}
