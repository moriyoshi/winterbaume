use aws_sdk_appsync::config::BehaviorVersion;
use winterbaume_appsync::AppSyncService;
use winterbaume_core::MockAws;

async fn make_appsync_client() -> aws_sdk_appsync::Client {
    let mock = MockAws::builder()
        .with_service(AppSyncService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appsync::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_appsync::Client::new(&config)
}

// ===================== GraphQL API (v1) Tests =====================

#[tokio::test]
async fn test_create_and_get_graphql_api() {
    let client = make_appsync_client().await;

    let resp = client
        .create_graphql_api()
        .name("TestApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .expect("create_graphql_api should succeed");

    let api = resp.graphql_api().expect("should have graphql_api");
    assert_eq!(api.name().unwrap(), "TestApi");
    assert_eq!(
        api.authentication_type().unwrap(),
        &aws_sdk_appsync::types::AuthenticationType::ApiKey
    );
    let api_id = api.api_id().unwrap().to_string();

    let get_resp = client
        .get_graphql_api()
        .api_id(&api_id)
        .send()
        .await
        .expect("get_graphql_api should succeed");

    let got_api = get_resp.graphql_api().expect("should have graphql_api");
    assert_eq!(got_api.name().unwrap(), "TestApi");
    assert_eq!(got_api.api_id().unwrap(), api_id);
}

#[tokio::test]
async fn test_delete_graphql_api() {
    let client = make_appsync_client().await;

    let resp = client
        .create_graphql_api()
        .name("DeleteMe")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();

    let api_id = resp.graphql_api().unwrap().api_id().unwrap().to_string();

    client
        .delete_graphql_api()
        .api_id(&api_id)
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_graphql_api().api_id(&api_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_graphql_apis() {
    let client = make_appsync_client().await;

    client
        .create_graphql_api()
        .name("Api1")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();

    client
        .create_graphql_api()
        .name("Api2")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::AwsIam)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_graphql_apis()
        .send()
        .await
        .expect("list_graphql_apis should succeed");

    assert_eq!(resp.graphql_apis().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_api_fails() {
    let client = make_appsync_client().await;

    let result = client
        .get_graphql_api()
        .api_id("nonexistent-api-id")
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent API should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_api_fails() {
    let client = make_appsync_client().await;

    let result = client
        .delete_graphql_api()
        .api_id("nonexistent-api-id")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent API should fail");
}

#[tokio::test]
async fn test_update_graphql_api() {
    let client = make_appsync_client().await;

    let resp = client
        .create_graphql_api()
        .name("OriginalName")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();

    let api_id = resp.graphql_api().unwrap().api_id().unwrap().to_string();

    let update_resp = client
        .update_graphql_api()
        .api_id(&api_id)
        .name("UpdatedName")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::AwsIam)
        .send()
        .await
        .expect("update_graphql_api should succeed");

    let updated = update_resp.graphql_api().unwrap();
    assert_eq!(updated.name().unwrap(), "UpdatedName");
    assert_eq!(
        updated.authentication_type().unwrap(),
        &aws_sdk_appsync::types::AuthenticationType::AwsIam
    );
}

// ===================== API Cache Tests =====================

#[tokio::test]
async fn test_create_and_get_api_cache() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("CacheApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    let cache_resp = client
        .create_api_cache()
        .api_id(&api_id)
        .api_caching_behavior(aws_sdk_appsync::types::ApiCachingBehavior::FullRequestCaching)
        .r#type(aws_sdk_appsync::types::ApiCacheType::T2Small)
        .ttl(300)
        .send()
        .await
        .expect("create_api_cache should succeed");

    let cache = cache_resp.api_cache().unwrap();
    assert_eq!(cache.ttl(), 300);

    let get_resp = client
        .get_api_cache()
        .api_id(&api_id)
        .send()
        .await
        .expect("get_api_cache should succeed");

    assert_eq!(get_resp.api_cache().unwrap().ttl(), 300);
}

#[tokio::test]
async fn test_delete_api_cache() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("CacheDeleteApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    client
        .create_api_cache()
        .api_id(&api_id)
        .api_caching_behavior(aws_sdk_appsync::types::ApiCachingBehavior::FullRequestCaching)
        .r#type(aws_sdk_appsync::types::ApiCacheType::T2Small)
        .ttl(300)
        .send()
        .await
        .unwrap();

    client
        .delete_api_cache()
        .api_id(&api_id)
        .send()
        .await
        .expect("delete_api_cache should succeed");

    let result = client.get_api_cache().api_id(&api_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_update_api_cache() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("CacheUpdateApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    client
        .create_api_cache()
        .api_id(&api_id)
        .api_caching_behavior(aws_sdk_appsync::types::ApiCachingBehavior::FullRequestCaching)
        .r#type(aws_sdk_appsync::types::ApiCacheType::T2Small)
        .ttl(300)
        .send()
        .await
        .unwrap();

    let update_resp = client
        .update_api_cache()
        .api_id(&api_id)
        .api_caching_behavior(aws_sdk_appsync::types::ApiCachingBehavior::PerResolverCaching)
        .r#type(aws_sdk_appsync::types::ApiCacheType::T2Medium)
        .ttl(600)
        .send()
        .await
        .expect("update_api_cache should succeed");

    assert_eq!(update_resp.api_cache().unwrap().ttl(), 600);
}

#[tokio::test]
async fn test_flush_api_cache() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("FlushCacheApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    client
        .create_api_cache()
        .api_id(&api_id)
        .api_caching_behavior(aws_sdk_appsync::types::ApiCachingBehavior::FullRequestCaching)
        .r#type(aws_sdk_appsync::types::ApiCacheType::T2Small)
        .ttl(300)
        .send()
        .await
        .unwrap();

    client
        .flush_api_cache()
        .api_id(&api_id)
        .send()
        .await
        .expect("flush_api_cache should succeed");
}

// ===================== API Key Tests =====================

#[tokio::test]
async fn test_create_and_list_api_keys() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("KeyApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    let key_resp = client
        .create_api_key()
        .api_id(&api_id)
        .description("test key")
        .expires(9999999999)
        .send()
        .await
        .expect("create_api_key should succeed");

    let key = key_resp.api_key().unwrap();
    assert_eq!(key.description().unwrap(), "test key");

    let list_resp = client
        .list_api_keys()
        .api_id(&api_id)
        .send()
        .await
        .expect("list_api_keys should succeed");

    assert_eq!(list_resp.api_keys().len(), 1);
}

#[tokio::test]
async fn test_delete_api_key() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("KeyDeleteApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    let key_resp = client
        .create_api_key()
        .api_id(&api_id)
        .expires(9999999999)
        .send()
        .await
        .unwrap();
    let key_id = key_resp.api_key().unwrap().id().unwrap().to_string();

    client
        .delete_api_key()
        .api_id(&api_id)
        .id(&key_id)
        .send()
        .await
        .expect("delete_api_key should succeed");

    let list_resp = client.list_api_keys().api_id(&api_id).send().await.unwrap();
    assert_eq!(list_resp.api_keys().len(), 0);
}

#[tokio::test]
async fn test_update_api_key() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("KeyUpdateApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    let key_resp = client
        .create_api_key()
        .api_id(&api_id)
        .description("original")
        .expires(9999999999)
        .send()
        .await
        .unwrap();
    let key_id = key_resp.api_key().unwrap().id().unwrap().to_string();

    let update_resp = client
        .update_api_key()
        .api_id(&api_id)
        .id(&key_id)
        .description("updated")
        .send()
        .await
        .expect("update_api_key should succeed");

    assert_eq!(
        update_resp.api_key().unwrap().description().unwrap(),
        "updated"
    );
}

// ===================== Schema Creation Tests =====================

#[tokio::test]
async fn test_start_and_get_schema_creation_status() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("SchemaApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    let start_resp = client
        .start_schema_creation()
        .api_id(&api_id)
        .definition(aws_sdk_appsync::primitives::Blob::new(
            b"type Query { hello: String }".to_vec(),
        ))
        .send()
        .await
        .expect("start_schema_creation should succeed");

    assert_eq!(
        start_resp.status().unwrap(),
        &aws_sdk_appsync::types::SchemaStatus::Success
    );

    let status_resp = client
        .get_schema_creation_status()
        .api_id(&api_id)
        .send()
        .await
        .expect("get_schema_creation_status should succeed");

    assert_eq!(
        status_resp.status().unwrap(),
        &aws_sdk_appsync::types::SchemaStatus::Success
    );
}

// ===================== Event API (v2) Tests =====================

#[tokio::test]
async fn test_create_and_get_api() {
    let client = make_appsync_client().await;

    let resp = client
        .create_api()
        .name("EventApi")
        .send()
        .await
        .expect("create_api should succeed");

    let api = resp.api().expect("should have api");
    assert_eq!(api.name().unwrap(), "EventApi");
    let api_id = api.api_id().unwrap().to_string();

    let get_resp = client
        .get_api()
        .api_id(&api_id)
        .send()
        .await
        .expect("get_api should succeed");

    assert_eq!(get_resp.api().unwrap().name().unwrap(), "EventApi");
}

#[tokio::test]
async fn test_delete_api() {
    let client = make_appsync_client().await;

    let resp = client
        .create_api()
        .name("DeleteEventApi")
        .send()
        .await
        .unwrap();
    let api_id = resp.api().unwrap().api_id().unwrap().to_string();

    client
        .delete_api()
        .api_id(&api_id)
        .send()
        .await
        .expect("delete_api should succeed");

    let result = client.get_api().api_id(&api_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_apis() {
    let client = make_appsync_client().await;

    client.create_api().name("EventApi1").send().await.unwrap();

    client.create_api().name("EventApi2").send().await.unwrap();

    let resp = client
        .list_apis()
        .send()
        .await
        .expect("list_apis should succeed");

    assert_eq!(resp.apis().len(), 2);
}

// ===================== Channel Namespace (v2) Tests =====================

#[tokio::test]
async fn test_create_and_list_channel_namespaces() {
    let client = make_appsync_client().await;

    let api_resp = client.create_api().name("ChannelApi").send().await.unwrap();
    let api_id = api_resp.api().unwrap().api_id().unwrap().to_string();

    let ns_resp = client
        .create_channel_namespace()
        .api_id(&api_id)
        .name("test-namespace")
        .send()
        .await
        .expect("create_channel_namespace should succeed");

    let ns = ns_resp.channel_namespace().unwrap();
    assert_eq!(ns.name().unwrap(), "test-namespace");

    let list_resp = client
        .list_channel_namespaces()
        .api_id(&api_id)
        .send()
        .await
        .expect("list_channel_namespaces should succeed");

    assert_eq!(list_resp.channel_namespaces().len(), 1);
}

#[tokio::test]
async fn test_delete_channel_namespace() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_api()
        .name("ChannelDeleteApi")
        .send()
        .await
        .unwrap();
    let api_id = api_resp.api().unwrap().api_id().unwrap().to_string();

    client
        .create_channel_namespace()
        .api_id(&api_id)
        .name("del-ns")
        .send()
        .await
        .unwrap();

    client
        .delete_channel_namespace()
        .api_id(&api_id)
        .name("del-ns")
        .send()
        .await
        .expect("delete_channel_namespace should succeed");

    let list_resp = client
        .list_channel_namespaces()
        .api_id(&api_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.channel_namespaces().len(), 0);
}

// ===================== Tag Tests =====================

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_appsync_client().await;

    let resp = client
        .create_graphql_api()
        .name("TagApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();

    let arn = resp.graphql_api().unwrap().arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "prod");
    assert_eq!(tags.get("team").unwrap(), "backend");
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_appsync_client().await;

    let resp = client
        .create_graphql_api()
        .name("UntagApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();

    let arn = resp.graphql_api().unwrap().arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tags().unwrap();
    assert!(tags.get("env").is_none(), "env tag should be removed");
    assert_eq!(tags.get("team").unwrap(), "backend");
}

// ===================== GetType Tests =====================

#[tokio::test]
async fn test_get_type_not_found() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("TypeApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    let result = client
        .get_type()
        .api_id(&api_id)
        .type_name("NonExistent")
        .format(aws_sdk_appsync::types::TypeDefinitionFormat::Sdl)
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent type should fail");
}

// ===================== Lifecycle Tests =====================

#[tokio::test]
async fn test_graphql_api_full_lifecycle() {
    let client = make_appsync_client().await;

    // Create
    let resp = client
        .create_graphql_api()
        .name("LifecycleApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await
        .unwrap();
    let api_id = resp.graphql_api().unwrap().api_id().unwrap().to_string();

    // Update
    client
        .update_graphql_api()
        .api_id(&api_id)
        .name("RenamedApi")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::AwsIam)
        .send()
        .await
        .unwrap();

    // Get
    let get_resp = client
        .get_graphql_api()
        .api_id(&api_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.graphql_api().unwrap().name().unwrap(),
        "RenamedApi"
    );

    // Create cache
    client
        .create_api_cache()
        .api_id(&api_id)
        .api_caching_behavior(aws_sdk_appsync::types::ApiCachingBehavior::FullRequestCaching)
        .r#type(aws_sdk_appsync::types::ApiCacheType::T2Small)
        .ttl(300)
        .send()
        .await
        .unwrap();

    // Create API key
    let key_resp = client
        .create_api_key()
        .api_id(&api_id)
        .description("lifecycle key")
        .expires(9999999999)
        .send()
        .await
        .unwrap();
    let key_id = key_resp.api_key().unwrap().id().unwrap().to_string();

    // List keys
    let keys = client.list_api_keys().api_id(&api_id).send().await.unwrap();
    assert_eq!(keys.api_keys().len(), 1);

    // Delete key
    client
        .delete_api_key()
        .api_id(&api_id)
        .id(&key_id)
        .send()
        .await
        .unwrap();

    // Delete API (should also clean up cache)
    client
        .delete_graphql_api()
        .api_id(&api_id)
        .send()
        .await
        .unwrap();

    // Verify gone
    assert!(
        client
            .get_graphql_api()
            .api_id(&api_id)
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_event_api_full_lifecycle() {
    let client = make_appsync_client().await;

    // Create API
    let resp = client
        .create_api()
        .name("EventLifecycle")
        .send()
        .await
        .unwrap();
    let api_id = resp.api().unwrap().api_id().unwrap().to_string();

    // Create channel namespace
    client
        .create_channel_namespace()
        .api_id(&api_id)
        .name("my-channel")
        .send()
        .await
        .unwrap();

    // List channel namespaces
    let nss = client
        .list_channel_namespaces()
        .api_id(&api_id)
        .send()
        .await
        .unwrap();
    assert_eq!(nss.channel_namespaces().len(), 1);

    // Delete channel namespace
    client
        .delete_channel_namespace()
        .api_id(&api_id)
        .name("my-channel")
        .send()
        .await
        .unwrap();

    // Delete API
    client.delete_api().api_id(&api_id).send().await.unwrap();

    // Verify gone
    assert!(client.get_api().api_id(&api_id).send().await.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: AWS AppSync
// ============================================================================

#[tokio::test]
async fn test_list_graphql_apis_empty() {
    let client = make_appsync_client().await;

    let resp = client
        .list_graphql_apis()
        .send()
        .await
        .expect("list_graphql_apis on empty store should succeed");

    assert_eq!(resp.graphql_apis().len(), 0);
}

#[tokio::test]
async fn test_update_graphql_api_not_found() {
    let client = make_appsync_client().await;

    let result = client
        .update_graphql_api()
        .api_id("nonexistent-api-id")
        .name("NewName")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
        .send()
        .await;
    assert!(result.is_err(), "update nonexistent api should fail");
}

#[tokio::test]
async fn test_list_apis_empty() {
    let client = make_appsync_client().await;

    let resp = client
        .list_apis()
        .send()
        .await
        .expect("list_apis on empty store should succeed");

    assert_eq!(resp.apis().len(), 0);
}

#[tokio::test]
async fn test_delete_channel_namespace_not_found() {
    let client = make_appsync_client().await;

    // Create an API first (channel namespace requires an event API)
    let api_resp = client
        .create_api()
        .name("event-api-for-ns-test")
        .event_config(
            aws_sdk_appsync::types::EventConfig::builder()
                .auth_providers(
                    aws_sdk_appsync::types::AuthProvider::builder()
                        .auth_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
                        .build()
                        .unwrap(),
                )
                .connection_auth_modes(
                    aws_sdk_appsync::types::AuthMode::builder()
                        .auth_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
                        .build()
                        .unwrap(),
                )
                .default_publish_auth_modes(
                    aws_sdk_appsync::types::AuthMode::builder()
                        .auth_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
                        .build()
                        .unwrap(),
                )
                .default_subscribe_auth_modes(
                    aws_sdk_appsync::types::AuthMode::builder()
                        .auth_type(aws_sdk_appsync::types::AuthenticationType::ApiKey)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let api_id = api_resp.api().unwrap().api_id().unwrap().to_string();

    let result = client
        .delete_channel_namespace()
        .api_id(&api_id)
        .name("nonexistent-namespace")
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete nonexistent channel namespace should fail"
    );
}

#[tokio::test]
async fn test_list_api_keys_empty() {
    let client = make_appsync_client().await;

    let api_resp = client
        .create_graphql_api()
        .name("no-keys-api")
        .authentication_type(aws_sdk_appsync::types::AuthenticationType::AwsIam)
        .send()
        .await
        .unwrap();
    let api_id = api_resp
        .graphql_api()
        .unwrap()
        .api_id()
        .unwrap()
        .to_string();

    let resp = client
        .list_api_keys()
        .api_id(&api_id)
        .send()
        .await
        .expect("list_api_keys on api with no keys should succeed");

    assert_eq!(resp.api_keys().len(), 0);
}
