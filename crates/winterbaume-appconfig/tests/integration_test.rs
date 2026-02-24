use aws_sdk_appconfig::config::BehaviorVersion;
use aws_sdk_appconfig::primitives::Blob;
use winterbaume_appconfig::AppConfigService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_appconfig::Client {
    let mock = MockAws::builder()
        .with_service(AppConfigService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appconfig::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_appconfig::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_application() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("my-app")
        .description("Test application")
        .send()
        .await
        .expect("create_application should succeed");

    let app_id = resp.id().expect("should have id");
    assert!(!app_id.is_empty());
    assert_eq!(resp.name(), Some("my-app"));

    let get_resp = client
        .get_application()
        .application_id(app_id)
        .send()
        .await
        .expect("get_application should succeed");

    assert_eq!(get_resp.name(), Some("my-app"));
    assert_eq!(get_resp.description(), Some("Test application"));
}

#[tokio::test]
async fn test_list_applications() {
    let client = make_client().await;

    for name in ["app-a", "app-b"] {
        client.create_application().name(name).send().await.unwrap();
    }

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_delete_application() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("delete-me")
        .send()
        .await
        .unwrap();
    let app_id = resp.id().unwrap().to_string();

    client
        .delete_application()
        .application_id(&app_id)
        .send()
        .await
        .expect("delete_application should succeed");

    let result = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_delete_then_list_empty() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("ephemeral-app")
        .description("Will be deleted")
        .send()
        .await
        .unwrap();
    let app_id = resp.id().unwrap().to_string();

    // List should show 1
    let list_resp = client.list_applications().send().await.unwrap();
    assert_eq!(list_resp.items().len(), 1);

    // Delete it
    client
        .delete_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    // List should now be empty
    let list_resp = client.list_applications().send().await.unwrap();
    assert_eq!(list_resp.items().len(), 0);
}

#[tokio::test]
async fn test_get_nonexistent_application() {
    let client = make_client().await;

    let result = client
        .get_application()
        .application_id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- UpdateApplication ----

#[tokio::test]
async fn test_update_application() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("original-name")
        .description("original-desc")
        .send()
        .await
        .unwrap();
    let app_id = resp.id().unwrap().to_string();

    let updated = client
        .update_application()
        .application_id(&app_id)
        .name("new-name")
        .description("new-desc")
        .send()
        .await
        .expect("update_application should succeed");

    assert_eq!(updated.name(), Some("new-name"));
    assert_eq!(updated.description(), Some("new-desc"));

    // Verify via get
    let get_resp = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.name(), Some("new-name"));
    assert_eq!(get_resp.description(), Some("new-desc"));
}

// ---- ConfigurationProfile CRUD ----

#[tokio::test]
async fn test_create_and_get_configuration_profile() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("profile-test-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap();

    let profile = client
        .create_configuration_profile()
        .application_id(app_id)
        .name("my-profile")
        .location_uri("hosted")
        .description("Test profile")
        .r#type("AWS.Freeform")
        .send()
        .await
        .expect("create_configuration_profile should succeed");

    let profile_id = profile.id().expect("should have id");
    assert!(!profile_id.is_empty());
    assert_eq!(profile.name(), Some("my-profile"));
    assert_eq!(profile.application_id(), Some(app_id));
    assert_eq!(profile.location_uri(), Some("hosted"));

    let get_resp = client
        .get_configuration_profile()
        .application_id(app_id)
        .configuration_profile_id(profile_id)
        .send()
        .await
        .expect("get_configuration_profile should succeed");

    assert_eq!(get_resp.name(), Some("my-profile"));
    assert_eq!(get_resp.description(), Some("Test profile"));
}

#[tokio::test]
async fn test_delete_configuration_profile() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("del-profile-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("del-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    client
        .delete_configuration_profile()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .send()
        .await
        .expect("delete_configuration_profile should succeed");

    let result = client
        .get_configuration_profile()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_configuration_profiles() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("list-profile-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    for name in ["profile-a", "profile-b"] {
        client
            .create_configuration_profile()
            .application_id(&app_id)
            .name(name)
            .location_uri("hosted")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_configuration_profiles()
        .application_id(&app_id)
        .send()
        .await
        .expect("list_configuration_profiles should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_update_configuration_profile() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("upd-profile-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("orig-profile")
        .location_uri("hosted")
        .description("orig-desc")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    let updated = client
        .update_configuration_profile()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .name("updated-profile")
        .description("updated-desc")
        .send()
        .await
        .expect("update_configuration_profile should succeed");

    assert_eq!(updated.name(), Some("updated-profile"));
    assert_eq!(updated.description(), Some("updated-desc"));
}

// ---- HostedConfigurationVersion CRUD ----

#[tokio::test]
async fn test_create_and_get_hosted_configuration_version() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("hcv-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("hcv-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    let version = client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content(Blob::new(b"{\"key\":\"value\"}".to_vec()))
        .content_type("application/json")
        .description("v1")
        .send()
        .await
        .expect("create_hosted_configuration_version should succeed");

    assert_eq!(version.version_number(), 1);
    assert_eq!(version.application_id(), Some(app_id.as_str()));
    assert_eq!(
        version.configuration_profile_id(),
        Some(profile_id.as_str())
    );

    let get_resp = client
        .get_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .version_number(1)
        .send()
        .await
        .expect("get_hosted_configuration_version should succeed");

    assert_eq!(get_resp.version_number(), 1);
    assert_eq!(get_resp.description(), Some("v1"));
}

#[tokio::test]
async fn test_delete_hosted_configuration_version() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("del-hcv-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("del-hcv-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content(Blob::new(b"test".to_vec()))
        .content_type("text/plain")
        .send()
        .await
        .unwrap();

    client
        .delete_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .version_number(1)
        .send()
        .await
        .expect("delete_hosted_configuration_version should succeed");

    let result = client
        .get_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .version_number(1)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_hosted_configuration_version_auto_increments() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("incr-hcv-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("incr-hcv-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    let v1 = client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content(Blob::new(b"v1".to_vec()))
        .content_type("text/plain")
        .send()
        .await
        .unwrap();
    assert_eq!(v1.version_number(), 1);

    let v2 = client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content(Blob::new(b"v2".to_vec()))
        .content_type("text/plain")
        .send()
        .await
        .unwrap();
    assert_eq!(v2.version_number(), 2);
}

// ---- Tag operations ----

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_client().await;

    let arn = "arn:aws:appconfig:us-east-1:123456789012:application/test-app-id";

    client
        .tag_resource()
        .resource_arn(arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags().expect("should have tags map");
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let arn = "arn:aws:appconfig:us-east-1:123456789012:application/test-untag";

    client
        .tag_resource()
        .resource_arn(arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .tags("owner", "alice")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("team")
        .tag_keys("owner")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().expect("should have tags map");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
}

#[tokio::test]
async fn test_list_tags_empty() {
    let client = make_client().await;

    let arn = "arn:aws:appconfig:us-east-1:123456789012:application/no-tags";

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed for untagged resource");

    let tags = resp.tags();
    assert!(tags.is_none() || tags.unwrap().is_empty());
}

// ---- Moto-ported tests ----

#[tokio::test]
async fn test_tag_application_at_creation() {
    // Ported from moto test_tag_application: tags passed at create_application time
    let client = make_client().await;

    let app = client
        .create_application()
        .name("testapp-tagged")
        .tags("k1", "v1")
        .send()
        .await
        .expect("create_application with tags should succeed");

    let app_id = app.id().expect("should have id");
    let app_arn = format!("arn:aws:appconfig:us-east-1:123456789012:application/{app_id}");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().expect("should have tags");
    assert_eq!(tags.get("k1").map(|s| s.as_str()), Some("v1"));
}

#[tokio::test]
async fn test_tag_application_then_add_and_remove() {
    // Ported from moto test_tag_application: multi-step tag workflow
    let client = make_client().await;

    let app = client
        .create_application()
        .name("testapp")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();
    let app_arn = format!("arn:aws:appconfig:us-east-1:123456789012:application/{app_id}");

    // Initially empty
    let tags = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .unwrap();
    assert!(tags.tags().is_none() || tags.tags().unwrap().is_empty());

    // Add tags
    client
        .tag_resource()
        .resource_arn(&app_arn)
        .tags("k1", "v1")
        .send()
        .await
        .unwrap();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("k1").map(|s| s.as_str()), Some("v1"));

    // Add more tags and then remove
    client
        .tag_resource()
        .resource_arn(&app_arn)
        .tags("k2", "v2")
        .tags("k3", "v3")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&app_arn)
        .tag_keys("k2")
        .send()
        .await
        .unwrap();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags.contains_key("k1"));
    assert!(tags.contains_key("k3"));
    assert!(!tags.contains_key("k2"));
}

#[tokio::test]
async fn test_configuration_profile_with_retrieval_role_arn() {
    // Ported from moto test_create_configuration_profile: RetrievalRoleArn field
    let client = make_client().await;

    let app = client
        .create_application()
        .name("app-with-role")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("profile-with-role")
        .location_uri("hosted")
        .retrieval_role_arn("rrarn:rrarn:rrarn:rrarn")
        .send()
        .await
        .expect("create_configuration_profile with retrieval_role_arn should succeed");

    let profile_id = profile.id().unwrap().to_string();
    assert_eq!(
        profile.retrieval_role_arn(),
        Some("rrarn:rrarn:rrarn:rrarn")
    );

    // Verify via get
    let get_resp = client
        .get_configuration_profile()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .send()
        .await
        .expect("get_configuration_profile should succeed");
    assert_eq!(
        get_resp.retrieval_role_arn(),
        Some("rrarn:rrarn:rrarn:rrarn")
    );
}

#[tokio::test]
async fn test_update_configuration_profile_retrieval_role() {
    // Ported from moto test_create_configuration_profile: update RetrievalRoleArn
    let client = make_client().await;

    let app = client
        .create_application()
        .name("app-update-role")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("profile-for-update")
        .location_uri("hosted")
        .retrieval_role_arn("rrarn:rrarn:rrarn:rrarn")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    let updated = client
        .update_configuration_profile()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .name("new-name")
        .retrieval_role_arn("rrarn:rrarn:rrarn:222")
        .send()
        .await
        .expect("update_configuration_profile should succeed");

    assert_eq!(updated.name(), Some("new-name"));
    assert_eq!(updated.retrieval_role_arn(), Some("rrarn:rrarn:rrarn:222"));
}

#[tokio::test]
async fn test_tag_configuration_profile_at_creation() {
    // Ported from moto test_tag_config_profile: tags at creation time
    let client = make_client().await;

    let app = client
        .create_application()
        .name("app-for-profile-tags")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("profile-with-tags")
        .location_uri("hosted")
        .tags("k1", "v1")
        .send()
        .await
        .expect("create_configuration_profile with tags should succeed");

    let profile_id = profile.id().unwrap().to_string();
    let profile_arn = format!(
        "arn:aws:appconfig:us-east-1:123456789012:application/{app_id}/configurationprofile/{profile_id}"
    );

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&profile_arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().expect("should have tags");
    assert_eq!(tags.get("k1").map(|s| s.as_str()), Some("v1"));
}

#[tokio::test]
async fn test_hosted_config_version_with_content() {
    // Ported from moto TestHostedConfigurationVersions: content body is stored and retrievable
    let client = make_client().await;

    let app = client
        .create_application()
        .name("hcv-content-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("hcv-content-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    // Create with content and description
    let version = client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content(aws_sdk_appconfig::primitives::Blob::new(b"asdf".to_vec()))
        .content_type("text/xml")
        .description("desc")
        .send()
        .await
        .expect("create_hosted_configuration_version should succeed");

    assert_eq!(version.version_number(), 1);
    assert_eq!(version.application_id(), Some(app_id.as_str()));
    assert_eq!(
        version.configuration_profile_id(),
        Some(profile_id.as_str())
    );
    assert_eq!(version.description(), Some("desc"));

    // Create second version
    let v2 = client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content(aws_sdk_appconfig::primitives::Blob::new(b"asdf".to_vec()))
        .content_type("text/xml")
        .send()
        .await
        .unwrap();
    assert_eq!(v2.version_number(), 2);
}

#[tokio::test]
async fn test_delete_application_error_on_missing() {
    // Ported from moto test_create_application: ResourceNotFoundException on deleted app
    let client = make_client().await;

    let app = client
        .create_application()
        .name("to-be-deleted")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    client
        .delete_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();

    let result = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let svc_err = err.as_service_error().expect("should be a service error");
    assert!(svc_err.is_resource_not_found_exception());
}

#[tokio::test]
async fn test_delete_configuration_profile_error_on_missing() {
    // Ported from moto test_create_configuration_profile
    let client = make_client().await;

    let app = client
        .create_application()
        .name("app-for-profile-delete")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("del-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    client
        .delete_configuration_profile()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .send()
        .await
        .unwrap();

    let result = client
        .get_configuration_profile()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_hosted_config_version_delete_error_on_missing() {
    // Ported from moto TestHostedConfigurationVersions.test_delete_hosted_configuration_version
    let client = make_client().await;

    let app = client
        .create_application()
        .name("app-hcv-del-err")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("profile-hcv-del-err")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content(aws_sdk_appconfig::primitives::Blob::new(b"data".to_vec()))
        .content_type("text/plain")
        .send()
        .await
        .unwrap();

    client
        .delete_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .version_number(1)
        .send()
        .await
        .unwrap();

    let result = client
        .get_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .version_number(1)
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: AWS AppConfig
// ============================================================================

#[tokio::test]
async fn test_update_application_name_only() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("orig-name")
        .description("orig-description")
        .send()
        .await
        .unwrap();
    let app_id = resp.id().unwrap().to_string();

    client
        .update_application()
        .application_id(&app_id)
        .name("new-name")
        .send()
        .await
        .expect("update_application name-only should succeed");

    let get = client
        .get_application()
        .application_id(&app_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get.name(), Some("new-name"));
    // description should be preserved
    assert_eq!(get.description(), Some("orig-description"));
}

#[tokio::test]
async fn test_create_configuration_profile_nonexistent_application() {
    let client = make_client().await;

    let result = client
        .create_configuration_profile()
        .application_id("nonexistent-app-id")
        .name("my-profile")
        .location_uri("hosted")
        .send()
        .await;
    assert!(
        result.is_err(),
        "create profile for nonexistent app should fail"
    );
}

#[tokio::test]
async fn test_get_nonexistent_configuration_profile() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("app-for-profile-test")
        .send()
        .await
        .unwrap();
    let app_id = resp.id().unwrap().to_string();

    let result = client
        .get_configuration_profile()
        .application_id(&app_id)
        .configuration_profile_id("nonexistent-profile-id")
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent profile should fail");
}

#[tokio::test]
async fn test_create_hosted_configuration_version_nonexistent_profile() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("app-for-hcv-test")
        .send()
        .await
        .unwrap();
    let app_id = resp.id().unwrap().to_string();

    let result = client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id("nonexistent-profile-id")
        .content(Blob::new(b"config-data".as_ref()))
        .content_type("application/json")
        .send()
        .await;
    assert!(
        result.is_err(),
        "create hcv for nonexistent profile should fail"
    );
}

#[tokio::test]
async fn test_get_nonexistent_hosted_configuration_version() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("app-for-hcv-get-test")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("profile-for-hcv-test")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    let result = client
        .get_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .version_number(999)
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent hcv version should fail");
}

// ============================================================================
// Environment CRUD tests
// ============================================================================

#[tokio::test]
async fn test_create_and_get_environment() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("env-test-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let env = client
        .create_environment()
        .application_id(&app_id)
        .name("staging")
        .description("Staging environment")
        .send()
        .await
        .expect("create_environment should succeed");

    let env_id = env.id().expect("should have id");
    assert!(!env_id.is_empty());
    assert_eq!(env.name(), Some("staging"));
    assert_eq!(env.application_id(), Some(app_id.as_str()));

    let get_resp = client
        .get_environment()
        .application_id(&app_id)
        .environment_id(env_id)
        .send()
        .await
        .expect("get_environment should succeed");

    assert_eq!(get_resp.name(), Some("staging"));
    assert_eq!(get_resp.description(), Some("Staging environment"));
}

#[tokio::test]
async fn test_list_environments() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("list-env-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    for name in ["dev", "staging", "prod"] {
        client
            .create_environment()
            .application_id(&app_id)
            .name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_environments()
        .application_id(&app_id)
        .send()
        .await
        .expect("list_environments should succeed");

    assert_eq!(resp.items().len(), 3);
}

#[tokio::test]
async fn test_delete_environment() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("del-env-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let env = client
        .create_environment()
        .application_id(&app_id)
        .name("to-delete")
        .send()
        .await
        .unwrap();
    let env_id = env.id().unwrap().to_string();

    client
        .delete_environment()
        .application_id(&app_id)
        .environment_id(&env_id)
        .send()
        .await
        .expect("delete_environment should succeed");

    let result = client
        .get_environment()
        .application_id(&app_id)
        .environment_id(&env_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_environment() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("upd-env-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let env = client
        .create_environment()
        .application_id(&app_id)
        .name("orig-env")
        .description("orig-desc")
        .send()
        .await
        .unwrap();
    let env_id = env.id().unwrap().to_string();

    let updated = client
        .update_environment()
        .application_id(&app_id)
        .environment_id(&env_id)
        .name("updated-env")
        .description("updated-desc")
        .send()
        .await
        .expect("update_environment should succeed");

    assert_eq!(updated.name(), Some("updated-env"));
    assert_eq!(updated.description(), Some("updated-desc"));
}

// ============================================================================
// DeploymentStrategy CRUD tests
// ============================================================================

#[tokio::test]
async fn test_create_and_get_deployment_strategy() {
    let client = make_client().await;

    let ds = client
        .create_deployment_strategy()
        .name("my-strategy")
        .deployment_duration_in_minutes(10)
        .growth_factor(20.0)
        .description("Test strategy")
        .final_bake_time_in_minutes(5)
        .growth_type(aws_sdk_appconfig::types::GrowthType::Linear)
        .replicate_to(aws_sdk_appconfig::types::ReplicateTo::None)
        .send()
        .await
        .expect("create_deployment_strategy should succeed");

    let ds_id = ds.id().expect("should have id");
    assert!(!ds_id.is_empty());
    assert_eq!(ds.name(), Some("my-strategy"));
    assert_eq!(ds.deployment_duration_in_minutes(), 10);
    assert_eq!(ds.growth_factor(), Some(20.0_f32));

    let get_resp = client
        .get_deployment_strategy()
        .deployment_strategy_id(ds_id)
        .send()
        .await
        .expect("get_deployment_strategy should succeed");

    assert_eq!(get_resp.name(), Some("my-strategy"));
    assert_eq!(get_resp.description(), Some("Test strategy"));
    assert_eq!(get_resp.final_bake_time_in_minutes(), 5);
}

#[tokio::test]
async fn test_list_deployment_strategies() {
    let client = make_client().await;

    for name in ["strat-a", "strat-b"] {
        client
            .create_deployment_strategy()
            .name(name)
            .deployment_duration_in_minutes(5)
            .growth_factor(10.0)
            .replicate_to(aws_sdk_appconfig::types::ReplicateTo::None)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_deployment_strategies()
        .send()
        .await
        .expect("list_deployment_strategies should succeed");

    assert!(resp.items().len() >= 2);
}

#[tokio::test]
async fn test_delete_deployment_strategy() {
    let client = make_client().await;

    let ds = client
        .create_deployment_strategy()
        .name("del-strat")
        .deployment_duration_in_minutes(5)
        .growth_factor(10.0)
        .replicate_to(aws_sdk_appconfig::types::ReplicateTo::None)
        .send()
        .await
        .unwrap();
    let ds_id = ds.id().unwrap().to_string();

    client
        .delete_deployment_strategy()
        .deployment_strategy_id(&ds_id)
        .send()
        .await
        .expect("delete_deployment_strategy should succeed");

    let result = client
        .get_deployment_strategy()
        .deployment_strategy_id(&ds_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_deployment_strategy() {
    let client = make_client().await;

    let ds = client
        .create_deployment_strategy()
        .name("orig-strat")
        .deployment_duration_in_minutes(5)
        .growth_factor(10.0)
        .replicate_to(aws_sdk_appconfig::types::ReplicateTo::None)
        .send()
        .await
        .unwrap();
    let ds_id = ds.id().unwrap().to_string();

    let updated = client
        .update_deployment_strategy()
        .deployment_strategy_id(&ds_id)
        .deployment_duration_in_minutes(20)
        .growth_factor(50.0)
        .final_bake_time_in_minutes(10)
        .send()
        .await
        .expect("update_deployment_strategy should succeed");

    assert_eq!(updated.deployment_duration_in_minutes(), 20);
    assert_eq!(updated.growth_factor(), Some(50.0_f32));
    assert_eq!(updated.final_bake_time_in_minutes(), 10);
}

// ============================================================================
// Deployment tests (StartDeployment, GetDeployment, ListDeployments)
// ============================================================================

#[tokio::test]
async fn test_start_and_get_deployment() {
    let client = make_client().await;

    // Setup: app, env, profile, hosted config version, deployment strategy
    let app = client
        .create_application()
        .name("deploy-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let env = client
        .create_environment()
        .application_id(&app_id)
        .name("deploy-env")
        .send()
        .await
        .unwrap();
    let env_id = env.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("deploy-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content(Blob::new(b"{\"key\":\"val\"}".to_vec()))
        .content_type("application/json")
        .send()
        .await
        .unwrap();

    let ds = client
        .create_deployment_strategy()
        .name("deploy-strat")
        .deployment_duration_in_minutes(0)
        .growth_factor(100.0)
        .replicate_to(aws_sdk_appconfig::types::ReplicateTo::None)
        .send()
        .await
        .unwrap();
    let ds_id = ds.id().unwrap().to_string();

    let dep = client
        .start_deployment()
        .application_id(&app_id)
        .environment_id(&env_id)
        .deployment_strategy_id(&ds_id)
        .configuration_profile_id(&profile_id)
        .configuration_version("1")
        .description("first deployment")
        .send()
        .await
        .expect("start_deployment should succeed");

    assert_eq!(dep.deployment_number(), 1);
    assert_eq!(dep.application_id(), Some(app_id.as_str()));
    assert_eq!(dep.environment_id(), Some(env_id.as_str()));

    // GetDeployment
    let get_dep = client
        .get_deployment()
        .application_id(&app_id)
        .environment_id(&env_id)
        .deployment_number(1)
        .send()
        .await
        .expect("get_deployment should succeed");

    assert_eq!(get_dep.deployment_number(), 1);
    assert_eq!(get_dep.configuration_version(), Some("1"));
}

#[tokio::test]
async fn test_list_deployments() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("list-deploy-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let env = client
        .create_environment()
        .application_id(&app_id)
        .name("list-deploy-env")
        .send()
        .await
        .unwrap();
    let env_id = env.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("list-deploy-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    client
        .create_hosted_configuration_version()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .content(Blob::new(b"v1".to_vec()))
        .content_type("text/plain")
        .send()
        .await
        .unwrap();

    let ds = client
        .create_deployment_strategy()
        .name("list-deploy-strat")
        .deployment_duration_in_minutes(0)
        .growth_factor(100.0)
        .replicate_to(aws_sdk_appconfig::types::ReplicateTo::None)
        .send()
        .await
        .unwrap();
    let ds_id = ds.id().unwrap().to_string();

    client
        .start_deployment()
        .application_id(&app_id)
        .environment_id(&env_id)
        .deployment_strategy_id(&ds_id)
        .configuration_profile_id(&profile_id)
        .configuration_version("1")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_deployments()
        .application_id(&app_id)
        .environment_id(&env_id)
        .send()
        .await
        .expect("list_deployments should succeed");

    assert_eq!(resp.items().len(), 1);
}

// ============================================================================
// Extension CRUD tests
// ============================================================================

#[tokio::test]
async fn test_create_and_get_extension() {
    let client = make_client().await;

    let ext = client
        .create_extension()
        .name("my-extension")
        .description("Test extension")
        .actions(
            aws_sdk_appconfig::types::ActionPoint::OnDeploymentStart,
            vec![
                aws_sdk_appconfig::types::Action::builder()
                    .name("my-action")
                    .uri("arn:aws:lambda:us-east-1:123456789012:function:my-func")
                    .role_arn("arn:aws:iam::123456789012:role/my-role")
                    .build(),
            ],
        )
        .send()
        .await
        .expect("create_extension should succeed");

    let ext_id = ext.id().expect("should have id");
    assert!(!ext_id.is_empty());
    assert_eq!(ext.name(), Some("my-extension"));

    let get_resp = client
        .get_extension()
        .extension_identifier(ext_id)
        .send()
        .await
        .expect("get_extension should succeed");

    assert_eq!(get_resp.name(), Some("my-extension"));
    assert_eq!(get_resp.description(), Some("Test extension"));
}

#[tokio::test]
async fn test_list_extensions() {
    let client = make_client().await;

    for name in ["ext-a", "ext-b"] {
        client
            .create_extension()
            .name(name)
            .actions(
                aws_sdk_appconfig::types::ActionPoint::OnDeploymentStart,
                vec![
                    aws_sdk_appconfig::types::Action::builder()
                        .name("action")
                        .uri("arn:aws:lambda:us-east-1:123456789012:function:f")
                        .build(),
                ],
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_extensions()
        .send()
        .await
        .expect("list_extensions should succeed");

    assert!(resp.items().len() >= 2);
}

#[tokio::test]
async fn test_delete_extension() {
    let client = make_client().await;

    let ext = client
        .create_extension()
        .name("del-ext")
        .actions(
            aws_sdk_appconfig::types::ActionPoint::OnDeploymentStart,
            vec![
                aws_sdk_appconfig::types::Action::builder()
                    .name("action")
                    .uri("arn:aws:lambda:us-east-1:123456789012:function:f")
                    .build(),
            ],
        )
        .send()
        .await
        .unwrap();
    let ext_id = ext.id().unwrap().to_string();

    client
        .delete_extension()
        .extension_identifier(&ext_id)
        .send()
        .await
        .expect("delete_extension should succeed");

    let result = client
        .get_extension()
        .extension_identifier(&ext_id)
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// ListHostedConfigurationVersions test
// ============================================================================

#[tokio::test]
async fn test_list_hosted_configuration_versions() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("list-hcv-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("list-hcv-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    for content in [b"v1".as_ref(), b"v2".as_ref()] {
        client
            .create_hosted_configuration_version()
            .application_id(&app_id)
            .configuration_profile_id(&profile_id)
            .content(Blob::new(content.to_vec()))
            .content_type("text/plain")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_hosted_configuration_versions()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .send()
        .await
        .expect("list_hosted_configuration_versions should succeed");

    assert_eq!(resp.items().len(), 2);
}

// ============================================================================
// ValidateConfiguration test
// ============================================================================

#[tokio::test]
async fn test_validate_configuration() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("validate-app")
        .send()
        .await
        .unwrap();
    let app_id = app.id().unwrap().to_string();

    let profile = client
        .create_configuration_profile()
        .application_id(&app_id)
        .name("validate-profile")
        .location_uri("hosted")
        .send()
        .await
        .unwrap();
    let profile_id = profile.id().unwrap().to_string();

    let result = client
        .validate_configuration()
        .application_id(&app_id)
        .configuration_profile_id(&profile_id)
        .configuration_version("1")
        .send()
        .await;
    assert!(result.is_ok(), "validate_configuration should succeed");
}

// ============================================================================
// AccountSettings tests
// ============================================================================

#[tokio::test]
async fn test_get_account_settings() {
    let client = make_client().await;

    let resp = client
        .get_account_settings()
        .send()
        .await
        .expect("get_account_settings should succeed");

    // Initial settings should exist (may have default values)
    let _ = resp.deletion_protection();
}

#[tokio::test]
async fn test_update_account_settings() {
    let client = make_client().await;

    let resp = client
        .update_account_settings()
        .deletion_protection(
            aws_sdk_appconfig::types::DeletionProtectionSettings::builder()
                .enabled(true)
                .protection_period_in_minutes(60)
                .build(),
        )
        .send()
        .await
        .expect("update_account_settings should succeed");

    let dp = resp
        .deletion_protection()
        .expect("should have deletion protection");
    assert_eq!(dp.enabled(), Some(true));
    assert_eq!(dp.protection_period_in_minutes(), Some(60));

    // Verify via get
    let get_resp = client.get_account_settings().send().await.unwrap();

    let dp = get_resp
        .deletion_protection()
        .expect("should have deletion protection");
    assert_eq!(dp.enabled(), Some(true));
    assert_eq!(dp.protection_period_in_minutes(), Some(60));
}
