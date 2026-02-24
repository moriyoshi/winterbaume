use aws_sdk_cognitoidentity::config::BehaviorVersion;
use winterbaume_cognitoidentity::CognitoIdentityService;
use winterbaume_core::MockAws;

async fn make_client(region: &str) -> aws_sdk_cognitoidentity::Client {
    let mock = MockAws::builder()
        .with_service(CognitoIdentityService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cognitoidentity::config::Region::new(
            region.to_owned(),
        ))
        .load()
        .await;

    aws_sdk_cognitoidentity::Client::new(&config)
}

// --- test_create_identity_pool_invalid_name ---
#[tokio::test]
async fn test_create_identity_pool_invalid_name_hash() {
    let client = make_client("us-west-2").await;
    let result = client
        .create_identity_pool()
        .identity_pool_name("pool#name")
        .allow_unauthenticated_identities(false)
        .send()
        .await;
    assert!(result.is_err(), "should fail for invalid name 'pool#name'");
}

#[tokio::test]
async fn test_create_identity_pool_invalid_name_excl() {
    let client = make_client("us-west-2").await;
    let result = client
        .create_identity_pool()
        .identity_pool_name("with!excl")
        .allow_unauthenticated_identities(false)
        .send()
        .await;
    assert!(result.is_err(), "should fail for invalid name 'with!excl'");
}

#[tokio::test]
async fn test_create_identity_pool_invalid_name_quest() {
    let client = make_client("us-west-2").await;
    let result = client
        .create_identity_pool()
        .identity_pool_name("with?quest")
        .allow_unauthenticated_identities(false)
        .send()
        .await;
    assert!(result.is_err(), "should fail for invalid name 'with?quest'");
}

// --- test_create_identity_pool_valid_name ---
#[tokio::test]
async fn test_create_identity_pool_valid_name_x() {
    let client = make_client("us-west-2").await;
    client
        .create_identity_pool()
        .identity_pool_name("x")
        .allow_unauthenticated_identities(false)
        .send()
        .await
        .expect("should succeed for name 'x'");
}

#[tokio::test]
async fn test_create_identity_pool_valid_name_dash() {
    let client = make_client("us-west-2").await;
    client
        .create_identity_pool()
        .identity_pool_name("pool-")
        .allow_unauthenticated_identities(false)
        .send()
        .await
        .expect("should succeed for name 'pool-'");
}

#[tokio::test]
async fn test_create_identity_pool_valid_name_underscore() {
    let client = make_client("us-west-2").await;
    client
        .create_identity_pool()
        .identity_pool_name("pool_name")
        .allow_unauthenticated_identities(false)
        .send()
        .await
        .expect("should succeed for name 'pool_name'");
}

#[tokio::test]
async fn test_create_identity_pool_valid_name_space() {
    let client = make_client("us-west-2").await;
    client
        .create_identity_pool()
        .identity_pool_name("with space")
        .allow_unauthenticated_identities(false)
        .send()
        .await
        .expect("should succeed for name 'with space'");
}

// --- test_create_identity_pool (full) ---
#[tokio::test]
async fn test_create_identity_pool() {
    let client = make_client("us-west-2").await;

    let result = client
        .create_identity_pool()
        .identity_pool_name("TestPool")
        .allow_unauthenticated_identities(false)
        .supported_login_providers("graph.facebook.com", "123456789012345")
        .developer_provider_name("devname")
        .open_id_connect_provider_arns("arn:aws:rds:eu-west-2:123456789012:db:mysql-db")
        .cognito_identity_providers(
            aws_sdk_cognitoidentity::types::CognitoIdentityProvider::builder()
                .provider_name("testprovider")
                .client_id("CLIENT12345")
                .server_side_token_check(true)
                .build(),
        )
        .saml_provider_arns("arn:aws:rds:eu-west-2:123456789012:db:mysql-db")
        .send()
        .await
        .expect("create_identity_pool should succeed");

    assert!(
        !result.identity_pool_id().is_empty(),
        "IdentityPoolId should not be empty"
    );
}

// --- test_describe_identity_pool ---
#[tokio::test]
async fn test_describe_identity_pool() {
    let client = make_client("us-west-2").await;

    let res = client
        .create_identity_pool()
        .identity_pool_name("TestPool")
        .allow_unauthenticated_identities(false)
        .supported_login_providers("graph.facebook.com", "123456789012345")
        .developer_provider_name("devname")
        .open_id_connect_provider_arns("arn:aws:rds:eu-west-2:123456789012:db:mysql-db")
        .cognito_identity_providers(
            aws_sdk_cognitoidentity::types::CognitoIdentityProvider::builder()
                .provider_name("testprovider")
                .client_id("CLIENT12345")
                .server_side_token_check(true)
                .build(),
        )
        .saml_provider_arns("arn:aws:rds:eu-west-2:123456789012:db:mysql-db")
        .send()
        .await
        .unwrap();

    let pool_id = res.identity_pool_id();

    let result = client
        .describe_identity_pool()
        .identity_pool_id(pool_id)
        .send()
        .await
        .expect("describe_identity_pool should succeed");

    assert_eq!(result.identity_pool_id(), pool_id);
    assert_eq!(
        result.allow_unauthenticated_identities(),
        res.allow_unauthenticated_identities()
    );

    // Check SupportedLoginProviders
    let slp = result.supported_login_providers().unwrap();
    assert_eq!(
        slp.get("graph.facebook.com"),
        Some(&"123456789012345".to_string())
    );

    // Check DeveloperProviderName
    assert_eq!(result.developer_provider_name(), Some("devname"));

    // Check OpenIdConnectProviderARNs
    let oidc = result.open_id_connect_provider_arns();
    assert_eq!(oidc.len(), 1);
    assert_eq!(oidc[0], "arn:aws:rds:eu-west-2:123456789012:db:mysql-db");

    // Check CognitoIdentityProviders
    let providers = result.cognito_identity_providers();
    assert_eq!(providers.len(), 1);
    assert_eq!(providers[0].provider_name(), Some("testprovider"));
    assert_eq!(providers[0].client_id(), Some("CLIENT12345"));
    assert_eq!(providers[0].server_side_token_check(), Some(true));

    // Check SamlProviderARNs
    let saml = result.saml_provider_arns();
    assert_eq!(saml.len(), 1);
    assert_eq!(saml[0], "arn:aws:rds:eu-west-2:123456789012:db:mysql-db");
}

// --- test_update_identity_pool ---
#[tokio::test]
async fn test_update_identity_pool_supported_login_providers_add() {
    let client = make_client("us-west-2").await;

    let res = client
        .create_identity_pool()
        .identity_pool_name("TestPool")
        .allow_unauthenticated_identities(false)
        .supported_login_providers("graph.facebook.com", "123456789012345")
        .send()
        .await
        .unwrap();

    let pool_id = res.identity_pool_id().to_string();

    // Verify initial value
    let first = client
        .describe_identity_pool()
        .identity_pool_id(&pool_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        first
            .supported_login_providers()
            .unwrap()
            .get("graph.facebook.com"),
        Some(&"123456789012345".to_string())
    );

    // Update
    let response = client
        .update_identity_pool()
        .identity_pool_id(&pool_id)
        .identity_pool_name("TestPool")
        .allow_unauthenticated_identities(false)
        .supported_login_providers("graph.facebook.com", "123456789012345")
        .supported_login_providers("graph.google.com", "00000000")
        .send()
        .await
        .unwrap();

    assert_eq!(
        response
            .supported_login_providers()
            .unwrap()
            .get("graph.facebook.com"),
        Some(&"123456789012345".to_string())
    );
    assert_eq!(
        response
            .supported_login_providers()
            .unwrap()
            .get("graph.google.com"),
        Some(&"00000000".to_string())
    );

    // Verify persistence
    let second = client
        .describe_identity_pool()
        .identity_pool_id(&pool_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        second
            .supported_login_providers()
            .unwrap()
            .get("graph.facebook.com"),
        Some(&"123456789012345".to_string())
    );
    assert_eq!(
        second
            .supported_login_providers()
            .unwrap()
            .get("graph.google.com"),
        Some(&"00000000".to_string())
    );
}

#[tokio::test]
async fn test_update_identity_pool_supported_login_providers_clear() {
    let client = make_client("us-west-2").await;

    let res = client
        .create_identity_pool()
        .identity_pool_name("TestPool")
        .allow_unauthenticated_identities(false)
        .supported_login_providers("graph.facebook.com", "123456789012345")
        .send()
        .await
        .unwrap();

    let pool_id = res.identity_pool_id().to_string();

    let response = client
        .update_identity_pool()
        .identity_pool_id(&pool_id)
        .identity_pool_name("TestPool")
        .allow_unauthenticated_identities(false)
        .set_supported_login_providers(Some(std::collections::HashMap::new()))
        .send()
        .await
        .unwrap();

    assert!(
        response.supported_login_providers().unwrap().is_empty(),
        "should be empty after clearing"
    );
}

#[tokio::test]
async fn test_update_identity_pool_developer_provider_name() {
    let client = make_client("us-west-2").await;

    let res = client
        .create_identity_pool()
        .identity_pool_name("TestPool")
        .allow_unauthenticated_identities(false)
        .developer_provider_name("dev1")
        .send()
        .await
        .unwrap();

    let pool_id = res.identity_pool_id().to_string();

    let first = client
        .describe_identity_pool()
        .identity_pool_id(&pool_id)
        .send()
        .await
        .unwrap();
    assert_eq!(first.developer_provider_name(), Some("dev1"));

    let response = client
        .update_identity_pool()
        .identity_pool_id(&pool_id)
        .identity_pool_name("TestPool")
        .allow_unauthenticated_identities(false)
        .developer_provider_name("dev2")
        .send()
        .await
        .unwrap();
    assert_eq!(response.developer_provider_name(), Some("dev2"));

    let second = client
        .describe_identity_pool()
        .identity_pool_id(&pool_id)
        .send()
        .await
        .unwrap();
    assert_eq!(second.developer_provider_name(), Some("dev2"));
}

// --- test_describe_identity_pool_with_invalid_id_raises_error ---
#[tokio::test]
async fn test_describe_identity_pool_with_invalid_id_raises_error() {
    let client = make_client("us-west-2").await;
    let result = client
        .describe_identity_pool()
        .identity_pool_id("us-west-2_non-existent")
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(
        service_err.is_resource_not_found_exception(),
        "expected ResourceNotFoundException"
    );
}

// --- test_get_id ---
#[tokio::test]
async fn test_get_id() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_identity_pool")
        .allow_unauthenticated_identities(true)
        .send()
        .await
        .unwrap();

    let result = client
        .get_id()
        .account_id("someaccount")
        .identity_pool_id(pool.identity_pool_id())
        .logins("someurl", "12345")
        .send()
        .await
        .expect("get_id should succeed");

    assert!(
        result.identity_id().unwrap().starts_with("us-west-2"),
        "IdentityId should start with region"
    );
}

// --- test_get_credentials_for_identity ---
#[tokio::test]
async fn test_get_credentials_for_identity() {
    let client = make_client("us-west-2").await;
    let result = client
        .get_credentials_for_identity()
        .identity_id("12345")
        .send()
        .await
        .expect("get_credentials_for_identity should succeed");

    assert!(result.credentials().is_some());
    assert_eq!(result.identity_id(), Some("12345"));
    assert!(result.credentials().unwrap().expiration().is_some());
}

// --- test_get_open_id_token_for_developer_identity ---
#[tokio::test]
async fn test_get_open_id_token_for_developer_identity() {
    let client = make_client("us-west-2").await;
    let result = client
        .get_open_id_token_for_developer_identity()
        .identity_pool_id("us-west-2:12345")
        .identity_id("12345")
        .logins("someurl", "12345")
        .token_duration(123)
        .send()
        .await
        .expect("should succeed");

    assert!(
        !result.token().unwrap().is_empty(),
        "Token should not be empty"
    );
    assert_eq!(result.identity_id(), Some("12345"));
}

#[tokio::test]
async fn test_get_open_id_token_for_developer_identity_no_explicit_identity_id() {
    let client = make_client("us-west-2").await;
    let result = client
        .get_open_id_token_for_developer_identity()
        .identity_pool_id("us-west-2:12345")
        .logins("someurl", "12345")
        .token_duration(123)
        .send()
        .await
        .expect("should succeed");

    assert!(
        !result.token().unwrap().is_empty(),
        "Token should not be empty"
    );
    assert!(
        !result.identity_id().unwrap().is_empty(),
        "IdentityId should not be empty"
    );
}

// --- test_get_open_id_token ---
#[tokio::test]
async fn test_get_open_id_token() {
    let client = make_client("us-west-2").await;
    let result = client
        .get_open_id_token()
        .identity_id("12345")
        .logins("someurl", "12345")
        .send()
        .await
        .expect("should succeed");

    assert!(
        !result.token().unwrap().is_empty(),
        "Token should not be empty"
    );
    assert_eq!(result.identity_id(), Some("12345"));
}

// --- test_list_identities ---
#[tokio::test]
async fn test_list_identities() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_identity_pool")
        .allow_unauthenticated_identities(true)
        .send()
        .await
        .unwrap();

    let pool_id = pool.identity_pool_id().to_string();

    let identity = client
        .get_id()
        .account_id("someaccount")
        .identity_pool_id(&pool_id)
        .logins("someurl", "12345")
        .send()
        .await
        .unwrap();

    let identity_id = identity.identity_id().unwrap().to_string();

    let identities = client
        .list_identities()
        .identity_pool_id(&pool_id)
        .max_results(123)
        .send()
        .await
        .expect("list_identities should succeed");

    assert_eq!(identities.identity_pool_id(), Some(pool_id.as_str()));
    assert!(!identities.identities().is_empty());

    let ids: Vec<&str> = identities
        .identities()
        .iter()
        .filter_map(|i| i.identity_id.as_deref())
        .collect();
    assert!(
        ids.contains(&identity_id.as_str()),
        "created identity should be in list"
    );
}

// --- test_list_identity_pools ---
#[tokio::test]
async fn test_list_identity_pools() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_identity_pool")
        .allow_unauthenticated_identities(true)
        .send()
        .await
        .unwrap();

    let pool_id = pool.identity_pool_id().to_string();

    let result = client
        .list_identity_pools()
        .max_results(10)
        .send()
        .await
        .expect("list_identity_pools should succeed");

    assert_eq!(result.identity_pools().len(), 1);
    assert_eq!(
        result.identity_pools()[0]
            .identity_pool_id
            .as_deref()
            .unwrap(),
        pool_id
    );
    assert_eq!(
        result.identity_pools()[0]
            .identity_pool_name
            .as_deref()
            .unwrap(),
        "test_identity_pool"
    );
}

// --- test_delete_identities ---
#[tokio::test]
async fn test_delete_identities() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_pool")
        .allow_unauthenticated_identities(true)
        .send()
        .await
        .unwrap();

    let pool_id = pool.identity_pool_id().to_string();

    // Create an identity via get_id
    let id_res = client
        .get_id()
        .account_id("someaccount")
        .identity_pool_id(&pool_id)
        .send()
        .await
        .unwrap();
    let identity_id = id_res.identity_id().unwrap().to_string();

    // Verify identity exists
    let before = client
        .list_identities()
        .identity_pool_id(&pool_id)
        .max_results(10)
        .send()
        .await
        .unwrap();
    assert!(
        !before.identities().is_empty(),
        "should have at least one identity"
    );

    // Delete identities
    let result = client
        .delete_identities()
        .identity_ids_to_delete(&identity_id)
        .send()
        .await
        .expect("delete_identities should succeed");

    assert!(
        result.unprocessed_identity_ids().is_empty(),
        "no unprocessed ids expected"
    );
}

// --- test_describe_identity ---
#[tokio::test]
async fn test_describe_identity() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_pool")
        .allow_unauthenticated_identities(true)
        .send()
        .await
        .unwrap();

    let pool_id = pool.identity_pool_id().to_string();

    let id_res = client
        .get_id()
        .account_id("someaccount")
        .identity_pool_id(&pool_id)
        .send()
        .await
        .unwrap();
    let identity_id = id_res.identity_id().unwrap().to_string();

    let result = client
        .describe_identity()
        .identity_id(&identity_id)
        .send()
        .await
        .expect("describe_identity should succeed");

    assert_eq!(result.identity_id(), Some(identity_id.as_str()));
    assert!(result.creation_date().is_some());
    assert!(result.last_modified_date().is_some());
}

// --- test_get_identity_pool_roles / test_set_identity_pool_roles ---
#[tokio::test]
async fn test_set_and_get_identity_pool_roles() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_pool")
        .allow_unauthenticated_identities(true)
        .send()
        .await
        .unwrap();

    let pool_id = pool.identity_pool_id().to_string();

    // Set roles
    let mut roles = std::collections::HashMap::new();
    roles.insert(
        "authenticated".to_string(),
        "arn:aws:iam::123456789012:role/AuthRole".to_string(),
    );
    roles.insert(
        "unauthenticated".to_string(),
        "arn:aws:iam::123456789012:role/UnauthRole".to_string(),
    );

    client
        .set_identity_pool_roles()
        .identity_pool_id(&pool_id)
        .set_roles(Some(roles.clone()))
        .send()
        .await
        .expect("set_identity_pool_roles should succeed");

    // Get roles
    let result = client
        .get_identity_pool_roles()
        .identity_pool_id(&pool_id)
        .send()
        .await
        .expect("get_identity_pool_roles should succeed");

    assert_eq!(result.identity_pool_id(), Some(pool_id.as_str()));
    let returned_roles = result.roles().unwrap();
    assert_eq!(
        returned_roles.get("authenticated"),
        Some(&"arn:aws:iam::123456789012:role/AuthRole".to_string())
    );
    assert_eq!(
        returned_roles.get("unauthenticated"),
        Some(&"arn:aws:iam::123456789012:role/UnauthRole".to_string())
    );
}

// --- test_get_principal_tag_attribute_map / test_set_principal_tag_attribute_map ---
#[tokio::test]
async fn test_set_and_get_principal_tag_attribute_map() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_pool")
        .allow_unauthenticated_identities(true)
        .send()
        .await
        .unwrap();

    let pool_id = pool.identity_pool_id().to_string();

    // Set principal tag attribute map
    let mut tags = std::collections::HashMap::new();
    tags.insert("department".to_string(), "engineering".to_string());

    let set_result = client
        .set_principal_tag_attribute_map()
        .identity_pool_id(&pool_id)
        .identity_provider_name("graph.facebook.com")
        .use_defaults(false)
        .set_principal_tags(Some(tags))
        .send()
        .await
        .expect("set_principal_tag_attribute_map should succeed");

    assert_eq!(set_result.identity_pool_id(), Some(pool_id.as_str()));
    assert_eq!(
        set_result.identity_provider_name(),
        Some("graph.facebook.com")
    );
    assert_eq!(set_result.use_defaults(), Some(false));

    // Get principal tag attribute map
    let get_result = client
        .get_principal_tag_attribute_map()
        .identity_pool_id(&pool_id)
        .identity_provider_name("graph.facebook.com")
        .send()
        .await
        .expect("get_principal_tag_attribute_map should succeed");

    assert_eq!(get_result.identity_pool_id(), Some(pool_id.as_str()));
    assert_eq!(
        get_result.identity_provider_name(),
        Some("graph.facebook.com")
    );
    assert_eq!(get_result.use_defaults(), Some(false));
    let ptags = get_result.principal_tags().unwrap();
    assert_eq!(ptags.get("department"), Some(&"engineering".to_string()));
}

// --- test_tag_resource / test_untag_resource / test_list_tags_for_resource ---
#[tokio::test]
async fn test_tag_untag_list_tags_for_resource() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_pool")
        .allow_unauthenticated_identities(true)
        .send()
        .await
        .unwrap();

    let pool_id = pool.identity_pool_id().to_string();
    let resource_arn = format!(
        "arn:aws:cognito-identity:us-west-2:123456789012:identitypool/{}",
        pool_id
    );

    // Tag resource
    let mut tags = std::collections::HashMap::new();
    tags.insert("env".to_string(), "test".to_string());
    tags.insert("team".to_string(), "backend".to_string());

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let list_result = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let returned_tags = list_result.tags().unwrap();
    assert_eq!(returned_tags.get("env"), Some(&"test".to_string()));
    assert_eq!(returned_tags.get("team"), Some(&"backend".to_string()));

    // Untag resource
    client
        .untag_resource()
        .resource_arn(&resource_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify tag was removed
    let after = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .unwrap();

    let after_tags = after.tags().unwrap();
    assert!(!after_tags.contains_key("env"), "env tag should be removed");
    assert_eq!(
        after_tags.get("team"),
        Some(&"backend".to_string()),
        "team tag should remain"
    );
}

// --- test_lookup_developer_identity ---
#[tokio::test]
async fn test_lookup_developer_identity() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_pool")
        .allow_unauthenticated_identities(true)
        .developer_provider_name("devprovider")
        .send()
        .await
        .expect("create_identity_pool should succeed");
    let pool_id = pool.identity_pool_id();

    // Merge creates a developer identity link we can then look up
    let merge_result = client
        .merge_developer_identities()
        .source_user_identifier("src-user")
        .destination_user_identifier("dst-user")
        .developer_provider_name("devprovider")
        .identity_pool_id(pool_id)
        .send()
        .await
        .expect("merge should succeed");

    let result = client
        .lookup_developer_identity()
        .identity_pool_id(pool_id)
        .developer_user_identifier("dst-user")
        .max_results(10)
        .send()
        .await
        .expect("lookup_developer_identity should succeed");

    assert_eq!(result.identity_id(), merge_result.identity_id());
    assert!(!result.developer_user_identifier_list().is_empty());
}

// --- test_merge_developer_identities ---
#[tokio::test]
async fn test_merge_developer_identities() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_pool")
        .allow_unauthenticated_identities(true)
        .developer_provider_name("devprovider")
        .send()
        .await
        .expect("create_identity_pool should succeed");
    let pool_id = pool.identity_pool_id();

    let result = client
        .merge_developer_identities()
        .source_user_identifier("src-user")
        .destination_user_identifier("dst-user")
        .developer_provider_name("devprovider")
        .identity_pool_id(pool_id)
        .send()
        .await
        .expect("merge_developer_identities should succeed");

    assert!(result.identity_id().is_some());
}

// --- test_unlink_developer_identity ---
#[tokio::test]
async fn test_unlink_developer_identity() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_pool")
        .allow_unauthenticated_identities(true)
        .developer_provider_name("devprovider")
        .send()
        .await
        .expect("create_identity_pool should succeed");
    let pool_id = pool.identity_pool_id();

    let result = client
        .unlink_developer_identity()
        .identity_id("12345")
        .identity_pool_id(pool_id)
        .developer_provider_name("devprovider")
        .developer_user_identifier("dev-user")
        .send()
        .await;

    assert!(result.is_ok(), "unlink_developer_identity should succeed");
}

// --- test_unlink_identity ---
#[tokio::test]
async fn test_unlink_identity() {
    let client = make_client("us-west-2").await;

    let mut logins = std::collections::HashMap::new();
    logins.insert("someurl".to_string(), "12345".to_string());

    let result = client
        .unlink_identity()
        .identity_id("12345")
        .set_logins(Some(logins))
        .logins_to_remove("someurl")
        .send()
        .await;

    assert!(result.is_ok(), "unlink_identity should succeed");
}

// --- test_delete_identity_pool ---
#[tokio::test]
async fn test_delete_identity_pool() {
    let client = make_client("us-west-2").await;
    let pool = client
        .create_identity_pool()
        .identity_pool_name("test_identity_pool")
        .allow_unauthenticated_identities(true)
        .send()
        .await
        .unwrap();

    let pool_id = pool.identity_pool_id().to_string();

    let before = client
        .list_identity_pools()
        .max_results(10)
        .send()
        .await
        .unwrap();
    assert_eq!(before.identity_pools().len(), 1);

    client
        .delete_identity_pool()
        .identity_pool_id(&pool_id)
        .send()
        .await
        .expect("delete should succeed");

    let after = client
        .list_identity_pools()
        .max_results(10)
        .send()
        .await
        .unwrap();
    assert_eq!(after.identity_pools().len(), 0);
}
