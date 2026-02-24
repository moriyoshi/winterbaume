//! Integration tests for winterbaume WorkSpaces service.
//!
//! These tests verify that aws-sdk-workspaces operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_workspaces::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_workspaces::WorkSpacesService;

/// Helper to create a configured WorkSpaces client backed by winterbaume.
async fn make_workspaces_client() -> aws_sdk_workspaces::Client {
    let mock = MockAws::builder()
        .with_service(WorkSpacesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_workspaces::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_workspaces::Client::new(&config)
}

#[tokio::test]
async fn test_create_workspaces() {
    let client = make_workspaces_client().await;

    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    let resp = client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .expect("create_workspaces should succeed");

    let pending = resp.pending_requests();
    assert_eq!(pending.len(), 1, "should have one pending workspace");
    let ws = &pending[0];
    assert!(ws.workspace_id().is_some(), "workspace should have an ID");
    assert_eq!(ws.directory_id(), Some("d-1234567890"));
    assert_eq!(ws.user_name(), Some("testuser"));
    assert_eq!(ws.bundle_id(), Some("wsb-abcdefgh1"));

    let failed = resp.failed_requests();
    assert!(failed.is_empty(), "should have no failed requests");
}

#[tokio::test]
async fn test_describe_workspaces() {
    let client = make_workspaces_client().await;

    // Create a workspace first
    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    let create_resp = client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .unwrap();

    let workspace_id = create_resp.pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string();

    // Describe by workspace ID
    let desc_resp = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .expect("describe_workspaces should succeed");

    let workspaces = desc_resp.workspaces();
    assert_eq!(workspaces.len(), 1, "should find one workspace");
    assert_eq!(workspaces[0].workspace_id(), Some(workspace_id.as_str()));
    assert_eq!(workspaces[0].user_name(), Some("testuser"));
}

#[tokio::test]
async fn test_terminate_workspaces() {
    let client = make_workspaces_client().await;

    // Create a workspace first
    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    let create_resp = client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .unwrap();

    let workspace_id = create_resp.pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string();

    // Terminate the workspace
    let terminate_request = aws_sdk_workspaces::types::TerminateRequest::builder()
        .workspace_id(&workspace_id)
        .build()
        .unwrap();

    let term_resp = client
        .terminate_workspaces()
        .terminate_workspace_requests(terminate_request)
        .send()
        .await
        .expect("terminate_workspaces should succeed");

    assert!(
        term_resp.failed_requests().is_empty(),
        "should have no failed termination requests"
    );

    // Verify the workspace is no longer found
    let desc_resp = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();

    assert!(
        desc_resp.workspaces().is_empty(),
        "terminated workspace should not appear in describe"
    );
}

#[tokio::test]
async fn test_describe_workspace_directories() {
    let client = make_workspaces_client().await;

    // Create a workspace to auto-register a directory
    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .unwrap();

    // Describe directories
    let dir_resp = client
        .describe_workspace_directories()
        .directory_ids("d-1234567890")
        .send()
        .await
        .expect("describe_workspace_directories should succeed");

    let directories = dir_resp.directories();
    assert_eq!(directories.len(), 1, "should find one directory");
    assert_eq!(directories[0].directory_id(), Some("d-1234567890"));
}

#[tokio::test]
async fn test_describe_workspaces_empty() {
    let client = make_workspaces_client().await;

    let resp = client
        .describe_workspaces()
        .send()
        .await
        .expect("describe_workspaces on empty state should succeed");

    assert!(
        resp.workspaces().is_empty(),
        "should return empty list when no workspaces exist"
    );
}

#[tokio::test]
async fn test_describe_workspace_directories_empty() {
    let client = make_workspaces_client().await;

    let resp = client
        .describe_workspace_directories()
        .send()
        .await
        .expect("describe_workspace_directories on empty state should succeed");

    assert!(
        resp.directories().is_empty(),
        "should return empty list when no directories exist"
    );
}

#[tokio::test]
async fn test_create_and_describe_tags() {
    let client = make_workspaces_client().await;

    // Create a workspace to get a resource ID
    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    let create_resp = client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .unwrap();

    let workspace_id = create_resp.pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string();

    // Create tags
    let tag1 = aws_sdk_workspaces::types::Tag::builder()
        .key("Environment")
        .value("Test")
        .build()
        .unwrap();
    let tag2 = aws_sdk_workspaces::types::Tag::builder()
        .key("Project")
        .value("Alpha")
        .build()
        .unwrap();

    client
        .create_tags()
        .resource_id(&workspace_id)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .expect("create_tags should succeed");

    // Describe tags
    let tags_resp = client
        .describe_tags()
        .resource_id(&workspace_id)
        .send()
        .await
        .expect("describe_tags should succeed");

    let tag_list = tags_resp.tag_list();
    assert_eq!(tag_list.len(), 2, "should have two tags");
    let keys: Vec<&str> = tag_list.iter().map(|t| t.key()).collect();
    assert!(keys.contains(&"Environment"));
    assert!(keys.contains(&"Project"));
}

#[tokio::test]
async fn test_create_workspace_image() {
    let client = make_workspaces_client().await;

    // Create a workspace first
    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    let create_resp = client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .unwrap();

    let workspace_id = create_resp.pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string();

    // Create workspace image
    let image_resp = client
        .create_workspace_image()
        .name("test-image")
        .description("A test image")
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("create_workspace_image should succeed");

    assert!(image_resp.image_id().is_some());
    assert_eq!(image_resp.name(), Some("test-image"));
    assert_eq!(image_resp.description(), Some("A test image"));
    assert_eq!(image_resp.state().map(|s| s.as_str()), Some("AVAILABLE"));
}

#[tokio::test]
async fn test_describe_workspace_images() {
    let client = make_workspaces_client().await;

    // Create a workspace and image
    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    let create_resp = client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .unwrap();

    let workspace_id = create_resp.pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string();

    let image_resp = client
        .create_workspace_image()
        .name("test-image")
        .description("A test image")
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();

    let image_id = image_resp.image_id().unwrap().to_string();

    // Describe images
    let desc_resp = client
        .describe_workspace_images()
        .image_ids(&image_id)
        .send()
        .await
        .expect("describe_workspace_images should succeed");

    let images = desc_resp.images();
    assert_eq!(images.len(), 1);
    assert_eq!(images[0].image_id(), Some(image_id.as_str()));
    assert_eq!(images[0].name(), Some("test-image"));
}

#[tokio::test]
async fn test_workspace_image_permissions() {
    let client = make_workspaces_client().await;

    // Create a workspace and image
    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    let create_resp = client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .unwrap();

    let workspace_id = create_resp.pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string();

    let image_resp = client
        .create_workspace_image()
        .name("test-image")
        .description("A test image")
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();

    let image_id = image_resp.image_id().unwrap().to_string();

    // Grant permission
    client
        .update_workspace_image_permission()
        .image_id(&image_id)
        .shared_account_id("123456789012")
        .allow_copy_image(true)
        .send()
        .await
        .expect("update_workspace_image_permission should succeed");

    // Describe permissions
    let perms_resp = client
        .describe_workspace_image_permissions()
        .image_id(&image_id)
        .send()
        .await
        .expect("describe_workspace_image_permissions should succeed");

    assert_eq!(perms_resp.image_id(), Some(image_id.as_str()));
    let perms = perms_resp.image_permissions();
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0].shared_account_id(), Some("123456789012"));

    // Revoke permission
    client
        .update_workspace_image_permission()
        .image_id(&image_id)
        .shared_account_id("123456789012")
        .allow_copy_image(false)
        .send()
        .await
        .unwrap();

    let perms_resp2 = client
        .describe_workspace_image_permissions()
        .image_id(&image_id)
        .send()
        .await
        .unwrap();

    assert!(
        perms_resp2.image_permissions().is_empty(),
        "permissions should be empty after revocation"
    );
}

#[tokio::test]
async fn test_client_properties() {
    let client = make_workspaces_client().await;

    // Create a workspace to get a directory
    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .unwrap();

    // Modify client properties
    let cp = aws_sdk_workspaces::types::ClientProperties::builder()
        .reconnect_enabled(aws_sdk_workspaces::types::ReconnectEnum::Enabled)
        .build();

    client
        .modify_client_properties()
        .resource_id("d-1234567890")
        .client_properties(cp)
        .send()
        .await
        .expect("modify_client_properties should succeed");

    // Describe client properties
    let desc_resp = client
        .describe_client_properties()
        .resource_ids("d-1234567890")
        .send()
        .await
        .expect("describe_client_properties should succeed");

    let props_list = desc_resp.client_properties_list();
    assert_eq!(props_list.len(), 1);
    assert_eq!(props_list[0].resource_id(), Some("d-1234567890"));
}

#[tokio::test]
async fn test_modify_selfservice_permissions() {
    let client = make_workspaces_client().await;

    // Register a directory first
    client
        .register_workspace_directory()
        .directory_id("d-1234567890")
        .send()
        .await
        .expect("register should succeed");

    let perms = aws_sdk_workspaces::types::SelfservicePermissions::builder()
        .restart_workspace(aws_sdk_workspaces::types::ReconnectEnum::Enabled)
        .increase_volume_size(aws_sdk_workspaces::types::ReconnectEnum::Disabled)
        .build();

    client
        .modify_selfservice_permissions()
        .resource_id("d-1234567890")
        .selfservice_permissions(perms)
        .send()
        .await
        .expect("modify_selfservice_permissions should succeed");
}

#[tokio::test]
async fn test_modify_workspace_creation_properties() {
    let client = make_workspaces_client().await;

    // Register a directory first
    client
        .register_workspace_directory()
        .directory_id("d-1234567890")
        .send()
        .await
        .expect("register should succeed");

    let props = aws_sdk_workspaces::types::WorkspaceCreationProperties::builder()
        .enable_internet_access(true)
        .enable_maintenance_mode(true)
        .user_enabled_as_local_administrator(false)
        .build();

    client
        .modify_workspace_creation_properties()
        .resource_id("d-1234567890")
        .workspace_creation_properties(props)
        .send()
        .await
        .expect("modify_workspace_creation_properties should succeed");
}

#[tokio::test]
async fn test_register_and_deregister_workspace_directory() {
    let client = make_workspaces_client().await;

    // Register
    let reg_resp = client
        .register_workspace_directory()
        .directory_id("d-newdir1234")
        .send()
        .await
        .expect("register_workspace_directory should succeed");

    assert_eq!(reg_resp.directory_id(), Some("d-newdir1234"));
    assert_eq!(reg_resp.state().map(|s| s.as_str()), Some("REGISTERED"));

    // Verify it appears in describe
    let desc_resp = client
        .describe_workspace_directories()
        .directory_ids("d-newdir1234")
        .send()
        .await
        .unwrap();

    assert_eq!(desc_resp.directories().len(), 1);

    // Deregister
    client
        .deregister_workspace_directory()
        .directory_id("d-newdir1234")
        .send()
        .await
        .expect("deregister_workspace_directory should succeed");

    // Verify it's gone
    let desc_resp2 = client
        .describe_workspace_directories()
        .directory_ids("d-newdir1234")
        .send()
        .await
        .unwrap();

    assert!(
        desc_resp2.directories().is_empty(),
        "directory should be gone after deregister"
    );
}

// ============================================================================
// Tests derived from AWS documentation: WorkSpaces
// ============================================================================

/// Helper to create a workspace and return its ID.
async fn create_test_workspace(client: &aws_sdk_workspaces::Client) -> String {
    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .unwrap()
        .pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn test_create_workspaces_multiple() {
    let client = make_workspaces_client().await;

    let req1 = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("user1")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();
    let req2 = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("user2")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();
    let req3 = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("user3")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    let resp = client
        .create_workspaces()
        .workspaces(req1)
        .workspaces(req2)
        .workspaces(req3)
        .send()
        .await
        .expect("create_workspaces with 3 requests should succeed");

    let pending = resp.pending_requests();
    assert_eq!(pending.len(), 3, "should have 3 pending workspaces");
    assert!(resp.failed_requests().is_empty(), "should have no failures");

    // Verify all 3 appear in describe
    let ids: Vec<String> = pending
        .iter()
        .map(|ws| ws.workspace_id().unwrap().to_string())
        .collect();
    let desc_resp = client
        .describe_workspaces()
        .workspace_ids(ids[0].clone())
        .workspace_ids(ids[1].clone())
        .workspace_ids(ids[2].clone())
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc_resp.workspaces().len(),
        3,
        "all 3 workspaces should be found"
    );
}

#[tokio::test]
async fn test_create_workspaces_with_properties() {
    let client = make_workspaces_client().await;

    let props = aws_sdk_workspaces::types::WorkspaceProperties::builder()
        .running_mode(aws_sdk_workspaces::types::RunningMode::AlwaysOn)
        .root_volume_size_gib(100)
        .user_volume_size_gib(75)
        .build();

    let request = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("testuser")
        .bundle_id("wsb-abcdefgh1")
        .workspace_properties(props)
        .build()
        .unwrap();

    let create_resp = client
        .create_workspaces()
        .workspaces(request)
        .send()
        .await
        .expect("create_workspaces with properties should succeed");

    let workspace_id = create_resp.pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string();

    let desc_resp = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();

    let ws = &desc_resp.workspaces()[0];
    let wp = ws
        .workspace_properties()
        .expect("workspace_properties should be present");
    assert_eq!(wp.running_mode().map(|r| r.as_str()), Some("ALWAYS_ON"));
    assert_eq!(wp.root_volume_size_gib(), Some(100));
    assert_eq!(wp.user_volume_size_gib(), Some(75));
}

#[tokio::test]
async fn test_describe_workspaces_filter_by_directory() {
    let client = make_workspaces_client().await;

    // Create a workspace in directory A
    let req_a = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-aaaaaaaaaa")
        .user_name("userA")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();
    // Create a workspace in directory B
    let req_b = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-bbbbbbbbbb")
        .user_name("userB")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    client
        .create_workspaces()
        .workspaces(req_a)
        .send()
        .await
        .unwrap();
    client
        .create_workspaces()
        .workspaces(req_b)
        .send()
        .await
        .unwrap();

    // Filter by directory A
    let resp = client
        .describe_workspaces()
        .directory_id("d-aaaaaaaaaa")
        .send()
        .await
        .expect("describe by directory_id should succeed");

    assert_eq!(
        resp.workspaces().len(),
        1,
        "should find only workspace in directory A"
    );
    assert_eq!(resp.workspaces()[0].directory_id(), Some("d-aaaaaaaaaa"));
}

#[tokio::test]
async fn test_describe_workspaces_filter_by_user() {
    let client = make_workspaces_client().await;

    let req1 = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("alice")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();
    let req2 = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-1234567890")
        .user_name("bob")
        .bundle_id("wsb-abcdefgh1")
        .build()
        .unwrap();

    client
        .create_workspaces()
        .workspaces(req1)
        .send()
        .await
        .unwrap();
    client
        .create_workspaces()
        .workspaces(req2)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_workspaces()
        .directory_id("d-1234567890")
        .user_name("alice")
        .send()
        .await
        .expect("describe by user_name should succeed");

    assert_eq!(
        resp.workspaces().len(),
        1,
        "should find only alice's workspace"
    );
    assert_eq!(resp.workspaces()[0].user_name(), Some("alice"));
}

#[tokio::test]
async fn test_terminate_nonexistent_workspace() {
    let client = make_workspaces_client().await;

    let terminate_req = aws_sdk_workspaces::types::TerminateRequest::builder()
        .workspace_id("ws-nonexistent1")
        .build()
        .unwrap();

    let resp = client
        .terminate_workspaces()
        .terminate_workspace_requests(terminate_req)
        .send()
        .await
        .expect("terminate of nonexistent workspace should return failed_requests, not HTTP error");

    let failed = resp.failed_requests();
    assert_eq!(failed.len(), 1, "should have one failed request");
    let err_str = format!("{:?}", failed[0].error_code());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_workspace_image_not_found() {
    let client = make_workspaces_client().await;

    let err = client
        .create_workspace_image()
        .name("img")
        .description("desc")
        .workspace_id("ws-nonexistent1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_workspace_image_permissions_not_found() {
    let client = make_workspaces_client().await;

    let err = client
        .describe_workspace_image_permissions()
        .image_id("wsi-nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_workspace_image_permission_not_found() {
    let client = make_workspaces_client().await;

    let err = client
        .update_workspace_image_permission()
        .image_id("wsi-nonexistent")
        .shared_account_id("123456789012")
        .allow_copy_image(true)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_register_workspace_directory_duplicate() {
    let client = make_workspaces_client().await;

    client
        .register_workspace_directory()
        .directory_id("d-unique12345")
        .send()
        .await
        .expect("first register should succeed");

    let err = client
        .register_workspace_directory()
        .directory_id("d-unique12345")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceAlreadyExistsException"),
        "expected ResourceAlreadyExistsException on duplicate register, got: {err_str}"
    );
}

#[tokio::test]
async fn test_deregister_nonexistent_directory() {
    let client = make_workspaces_client().await;

    let err = client
        .deregister_workspace_directory()
        .directory_id("d-nonexistent1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_modify_selfservice_permissions_not_found() {
    let client = make_workspaces_client().await;

    let perms = aws_sdk_workspaces::types::SelfservicePermissions::builder()
        .restart_workspace(aws_sdk_workspaces::types::ReconnectEnum::Enabled)
        .build();

    let err = client
        .modify_selfservice_permissions()
        .resource_id("d-nonexistent1")
        .selfservice_permissions(perms)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_modify_workspace_creation_properties_not_found() {
    let client = make_workspaces_client().await;

    let props = aws_sdk_workspaces::types::WorkspaceCreationProperties::builder()
        .enable_internet_access(true)
        .build();

    let err = client
        .modify_workspace_creation_properties()
        .resource_id("d-nonexistent1")
        .workspace_creation_properties(props)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_tags_override() {
    let client = make_workspaces_client().await;

    let workspace_id = create_test_workspace(&client).await;

    // Set initial tag
    let tag1 = aws_sdk_workspaces::types::Tag::builder()
        .key("Env")
        .value("Dev")
        .build()
        .unwrap();
    client
        .create_tags()
        .resource_id(&workspace_id)
        .tags(tag1)
        .send()
        .await
        .unwrap();

    // Override the same key with a new value
    let tag2 = aws_sdk_workspaces::types::Tag::builder()
        .key("Env")
        .value("Prod")
        .build()
        .unwrap();
    client
        .create_tags()
        .resource_id(&workspace_id)
        .tags(tag2)
        .send()
        .await
        .unwrap();

    let tags_resp = client
        .describe_tags()
        .resource_id(&workspace_id)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tag_list();
    let env_tags: Vec<_> = tags.iter().filter(|t| t.key() == "Env").collect();
    assert_eq!(
        env_tags.len(),
        1,
        "should have exactly one Env tag after override"
    );
    assert_eq!(
        env_tags[0].value(),
        Some("Prod"),
        "tag value should be updated to Prod"
    );
}

#[tokio::test]
async fn test_workspace_state_and_default_fields() {
    let client = make_workspaces_client().await;

    let workspace_id = create_test_workspace(&client).await;

    let desc_resp = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();

    let ws = &desc_resp.workspaces()[0];
    assert_eq!(
        ws.state().map(|s| s.as_str()),
        Some("AVAILABLE"),
        "state should be AVAILABLE"
    );
    assert!(
        ws.ip_address().map(|ip| !ip.is_empty()).unwrap_or(false),
        "ip_address should be set"
    );
    assert!(
        ws.computer_name().map(|c| !c.is_empty()).unwrap_or(false),
        "computer_name should be set"
    );
    assert!(
        ws.subnet_id().map(|s| !s.is_empty()).unwrap_or(false),
        "subnet_id should be set"
    );
}

#[tokio::test]
async fn test_workspace_image_multiple_permissions() {
    let client = make_workspaces_client().await;

    let workspace_id = create_test_workspace(&client).await;
    let image_resp = client
        .create_workspace_image()
        .name("multi-perm-image")
        .description("Test image")
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();

    let image_id = image_resp.image_id().unwrap().to_string();

    // Grant to two accounts
    client
        .update_workspace_image_permission()
        .image_id(&image_id)
        .shared_account_id("111111111111")
        .allow_copy_image(true)
        .send()
        .await
        .unwrap();

    client
        .update_workspace_image_permission()
        .image_id(&image_id)
        .shared_account_id("222222222222")
        .allow_copy_image(true)
        .send()
        .await
        .unwrap();

    let perms_resp = client
        .describe_workspace_image_permissions()
        .image_id(&image_id)
        .send()
        .await
        .unwrap();

    let perms = perms_resp.image_permissions();
    assert_eq!(perms.len(), 2, "should have permissions for 2 accounts");
    let account_ids: Vec<&str> = perms.iter().filter_map(|p| p.shared_account_id()).collect();
    assert!(
        account_ids.contains(&"111111111111"),
        "account 111111111111 should be present"
    );
    assert!(
        account_ids.contains(&"222222222222"),
        "account 222222222222 should be present"
    );
}

#[tokio::test]
async fn test_describe_workspace_images_all() {
    let client = make_workspaces_client().await;

    let workspace_id = create_test_workspace(&client).await;

    client
        .create_workspace_image()
        .name("image-one")
        .description("First image")
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();

    client
        .create_workspace_image()
        .name("image-two")
        .description("Second image")
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();

    let desc_resp = client
        .describe_workspace_images()
        .send()
        .await
        .expect("describe_workspace_images with no filter should succeed");

    assert_eq!(desc_resp.images().len(), 2, "should return both images");
    let names: Vec<Option<&str>> = desc_resp.images().iter().map(|i| i.name()).collect();
    assert!(
        names.contains(&Some("image-one")),
        "image-one should be present"
    );
    assert!(
        names.contains(&Some("image-two")),
        "image-two should be present"
    );
}

// ============================================================================
// Tests for previously untested operations
// ============================================================================

#[tokio::test]
async fn test_delete_tags() {
    let client = make_workspaces_client().await;
    let workspace_id = create_test_workspace(&client).await;

    // Create tags
    let tag1 = aws_sdk_workspaces::types::Tag::builder()
        .key("Env")
        .value("Dev")
        .build()
        .unwrap();
    let tag2 = aws_sdk_workspaces::types::Tag::builder()
        .key("Team")
        .value("Platform")
        .build()
        .unwrap();
    client
        .create_tags()
        .resource_id(&workspace_id)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .unwrap();

    // Delete one tag
    client
        .delete_tags()
        .resource_id(&workspace_id)
        .tag_keys("Env")
        .send()
        .await
        .expect("delete_tags should succeed");

    // Verify only Team remains
    let tags_resp = client
        .describe_tags()
        .resource_id(&workspace_id)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tag_list();
    assert_eq!(tags.len(), 1, "should have one tag remaining after delete");
    assert_eq!(tags[0].key(), "Team");
}

#[tokio::test]
async fn test_start_workspaces() {
    let client = make_workspaces_client().await;
    let workspace_id = create_test_workspace(&client).await;

    // Stop the workspace first
    let stop_req = aws_sdk_workspaces::types::StopRequest::builder()
        .workspace_id(&workspace_id)
        .build();
    client
        .stop_workspaces()
        .stop_workspace_requests(stop_req)
        .send()
        .await
        .unwrap();

    // Start the workspace
    let start_req = aws_sdk_workspaces::types::StartRequest::builder()
        .workspace_id(&workspace_id)
        .build();
    let resp = client
        .start_workspaces()
        .start_workspace_requests(start_req)
        .send()
        .await
        .expect("start_workspaces should succeed");

    assert!(
        resp.failed_requests().is_empty(),
        "should have no failed start requests"
    );

    // Verify workspace state is AVAILABLE
    let desc = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.workspaces()[0].state().map(|s| s.as_str()),
        Some("AVAILABLE")
    );
}

#[tokio::test]
async fn test_stop_workspaces() {
    let client = make_workspaces_client().await;
    let workspace_id = create_test_workspace(&client).await;

    let stop_req = aws_sdk_workspaces::types::StopRequest::builder()
        .workspace_id(&workspace_id)
        .build();
    let resp = client
        .stop_workspaces()
        .stop_workspace_requests(stop_req)
        .send()
        .await
        .expect("stop_workspaces should succeed");

    assert!(
        resp.failed_requests().is_empty(),
        "should have no failed stop requests"
    );

    // Verify workspace state is STOPPED
    let desc = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.workspaces()[0].state().map(|s| s.as_str()),
        Some("STOPPED")
    );
}

#[tokio::test]
async fn test_reboot_workspaces() {
    let client = make_workspaces_client().await;
    let workspace_id = create_test_workspace(&client).await;

    let reboot_req = aws_sdk_workspaces::types::RebootRequest::builder()
        .workspace_id(&workspace_id)
        .build()
        .unwrap();
    let resp = client
        .reboot_workspaces()
        .reboot_workspace_requests(reboot_req)
        .send()
        .await
        .expect("reboot_workspaces should succeed");

    assert!(
        resp.failed_requests().is_empty(),
        "should have no failed reboot requests"
    );

    // Verify workspace is still AVAILABLE after reboot
    let desc = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.workspaces()[0].state().map(|s| s.as_str()),
        Some("AVAILABLE")
    );
}

#[tokio::test]
async fn test_rebuild_workspaces() {
    let client = make_workspaces_client().await;
    let workspace_id = create_test_workspace(&client).await;

    let rebuild_req = aws_sdk_workspaces::types::RebuildRequest::builder()
        .workspace_id(&workspace_id)
        .build()
        .unwrap();
    let resp = client
        .rebuild_workspaces()
        .rebuild_workspace_requests(rebuild_req)
        .send()
        .await
        .expect("rebuild_workspaces should succeed");

    assert!(
        resp.failed_requests().is_empty(),
        "should have no failed rebuild requests"
    );
}

#[tokio::test]
async fn test_restore_workspace() {
    let client = make_workspaces_client().await;
    let workspace_id = create_test_workspace(&client).await;

    client
        .restore_workspace()
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("restore_workspace should succeed");

    // Verify workspace still exists and is AVAILABLE
    let desc = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.workspaces().len(), 1);
    assert_eq!(
        desc.workspaces()[0].state().map(|s| s.as_str()),
        Some("AVAILABLE")
    );
}

#[tokio::test]
async fn test_modify_workspace_properties() {
    let client = make_workspaces_client().await;
    let workspace_id = create_test_workspace(&client).await;

    let props = aws_sdk_workspaces::types::WorkspaceProperties::builder()
        .running_mode(aws_sdk_workspaces::types::RunningMode::AutoStop)
        .running_mode_auto_stop_timeout_in_minutes(120)
        .build();

    client
        .modify_workspace_properties()
        .workspace_id(&workspace_id)
        .workspace_properties(props)
        .send()
        .await
        .expect("modify_workspace_properties should succeed");

    // Verify the properties changed
    let desc = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();
    let ws = &desc.workspaces()[0];
    let wp = ws.workspace_properties().unwrap();
    assert_eq!(wp.running_mode().map(|r| r.as_str()), Some("AUTO_STOP"));
    assert_eq!(wp.running_mode_auto_stop_timeout_in_minutes(), Some(120));
}

#[tokio::test]
async fn test_modify_workspace_state() {
    let client = make_workspaces_client().await;
    let workspace_id = create_test_workspace(&client).await;

    client
        .modify_workspace_state()
        .workspace_id(&workspace_id)
        .workspace_state(aws_sdk_workspaces::types::TargetWorkspaceState::AdminMaintenance)
        .send()
        .await
        .expect("modify_workspace_state should succeed");

    // Verify state changed
    let desc = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.workspaces()[0].state().map(|s| s.as_str()),
        Some("ADMIN_MAINTENANCE")
    );
}

#[tokio::test]
async fn test_describe_workspaces_connection_status() {
    let client = make_workspaces_client().await;
    let workspace_id = create_test_workspace(&client).await;

    let resp = client
        .describe_workspaces_connection_status()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .expect("describe_workspaces_connection_status should succeed");

    let statuses = resp.workspaces_connection_status();
    assert_eq!(statuses.len(), 1, "should have one connection status entry");
    assert_eq!(statuses[0].workspace_id(), Some(workspace_id.as_str()));
    // AVAILABLE workspace should be CONNECTED
    assert_eq!(
        statuses[0].connection_state().map(|s| s.as_str()),
        Some("CONNECTED")
    );
}

#[tokio::test]
async fn test_create_and_describe_ip_group() {
    let client = make_workspaces_client().await;

    let rule = aws_sdk_workspaces::types::IpRuleItem::builder()
        .ip_rule("10.0.0.0/8")
        .rule_desc("Internal network")
        .build();

    let create_resp = client
        .create_ip_group()
        .group_name("test-ip-group")
        .group_desc("Test group")
        .user_rules(rule)
        .send()
        .await
        .expect("create_ip_group should succeed");

    let group_id = create_resp.group_id().unwrap().to_string();
    assert!(!group_id.is_empty());

    // Describe the group
    let desc_resp = client
        .describe_ip_groups()
        .group_ids(&group_id)
        .send()
        .await
        .expect("describe_ip_groups should succeed");

    let groups = desc_resp.result();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].group_id(), Some(group_id.as_str()));
    assert_eq!(groups[0].group_name(), Some("test-ip-group"));
    assert_eq!(groups[0].group_desc(), Some("Test group"));
    let rules = groups[0].user_rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].ip_rule(), Some("10.0.0.0/8"));
}

#[tokio::test]
async fn test_delete_ip_group() {
    let client = make_workspaces_client().await;

    let create_resp = client
        .create_ip_group()
        .group_name("to-delete-group")
        .send()
        .await
        .unwrap();

    let group_id = create_resp.group_id().unwrap().to_string();

    client
        .delete_ip_group()
        .group_id(&group_id)
        .send()
        .await
        .expect("delete_ip_group should succeed");

    // Verify it's gone
    let desc_resp = client
        .describe_ip_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    assert!(desc_resp.result().is_empty(), "group should be deleted");
}

#[tokio::test]
async fn test_authorize_and_revoke_ip_rules() {
    let client = make_workspaces_client().await;

    let create_resp = client
        .create_ip_group()
        .group_name("rules-test-group")
        .send()
        .await
        .unwrap();

    let group_id = create_resp.group_id().unwrap().to_string();

    // Authorize rules
    let rule1 = aws_sdk_workspaces::types::IpRuleItem::builder()
        .ip_rule("10.0.0.0/8")
        .rule_desc("Internal")
        .build();
    let rule2 = aws_sdk_workspaces::types::IpRuleItem::builder()
        .ip_rule("192.168.0.0/16")
        .rule_desc("Private")
        .build();

    client
        .authorize_ip_rules()
        .group_id(&group_id)
        .user_rules(rule1)
        .user_rules(rule2)
        .send()
        .await
        .expect("authorize_ip_rules should succeed");

    // Verify rules
    let desc = client
        .describe_ip_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.result()[0].user_rules().len(), 2);

    // Revoke one rule
    client
        .revoke_ip_rules()
        .group_id(&group_id)
        .user_rules("10.0.0.0/8")
        .send()
        .await
        .expect("revoke_ip_rules should succeed");

    // Verify only one rule remains
    let desc2 = client
        .describe_ip_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    let remaining_rules = desc2.result()[0].user_rules();
    assert_eq!(remaining_rules.len(), 1);
    assert_eq!(remaining_rules[0].ip_rule(), Some("192.168.0.0/16"));
}

#[tokio::test]
async fn test_update_rules_of_ip_group() {
    let client = make_workspaces_client().await;

    let rule_initial = aws_sdk_workspaces::types::IpRuleItem::builder()
        .ip_rule("10.0.0.0/8")
        .build();

    let create_resp = client
        .create_ip_group()
        .group_name("update-rules-group")
        .user_rules(rule_initial)
        .send()
        .await
        .unwrap();

    let group_id = create_resp.group_id().unwrap().to_string();

    // Replace rules entirely
    let new_rule = aws_sdk_workspaces::types::IpRuleItem::builder()
        .ip_rule("172.16.0.0/12")
        .rule_desc("Replaced")
        .build();

    client
        .update_rules_of_ip_group()
        .group_id(&group_id)
        .user_rules(new_rule)
        .send()
        .await
        .expect("update_rules_of_ip_group should succeed");

    let desc = client
        .describe_ip_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    let rules = desc.result()[0].user_rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].ip_rule(), Some("172.16.0.0/12"));
    assert_eq!(rules[0].rule_desc(), Some("Replaced"));
}

#[tokio::test]
async fn test_associate_and_disassociate_ip_groups() {
    let client = make_workspaces_client().await;

    // Register a directory
    client
        .register_workspace_directory()
        .directory_id("d-ipgrp12345")
        .send()
        .await
        .unwrap();

    // Create an IP group
    let create_resp = client
        .create_ip_group()
        .group_name("assoc-group")
        .send()
        .await
        .unwrap();
    let group_id = create_resp.group_id().unwrap().to_string();

    // Associate
    client
        .associate_ip_groups()
        .directory_id("d-ipgrp12345")
        .group_ids(&group_id)
        .send()
        .await
        .expect("associate_ip_groups should succeed");

    // Disassociate
    client
        .disassociate_ip_groups()
        .directory_id("d-ipgrp12345")
        .group_ids(&group_id)
        .send()
        .await
        .expect("disassociate_ip_groups should succeed");
}

#[tokio::test]
async fn test_create_and_describe_connection_alias() {
    let client = make_workspaces_client().await;

    let create_resp = client
        .create_connection_alias()
        .connection_string("www.example.com")
        .send()
        .await
        .expect("create_connection_alias should succeed");

    let alias_id = create_resp.alias_id().unwrap().to_string();
    assert!(!alias_id.is_empty());

    // Describe
    let desc_resp = client
        .describe_connection_aliases()
        .alias_ids(&alias_id)
        .send()
        .await
        .expect("describe_connection_aliases should succeed");

    let aliases = desc_resp.connection_aliases();
    assert_eq!(aliases.len(), 1);
    assert_eq!(aliases[0].alias_id(), Some(alias_id.as_str()));
    assert_eq!(aliases[0].connection_string(), Some("www.example.com"));
    assert_eq!(aliases[0].state().map(|s| s.as_str()), Some("CREATED"));
}

#[tokio::test]
async fn test_delete_connection_alias() {
    let client = make_workspaces_client().await;

    let create_resp = client
        .create_connection_alias()
        .connection_string("delete-me.example.com")
        .send()
        .await
        .unwrap();

    let alias_id = create_resp.alias_id().unwrap().to_string();

    client
        .delete_connection_alias()
        .alias_id(&alias_id)
        .send()
        .await
        .expect("delete_connection_alias should succeed");

    // Verify it's gone
    let desc_resp = client
        .describe_connection_aliases()
        .alias_ids(&alias_id)
        .send()
        .await
        .unwrap();
    assert!(
        desc_resp.connection_aliases().is_empty(),
        "alias should be deleted"
    );
}

#[tokio::test]
async fn test_describe_connection_alias_permissions() {
    let client = make_workspaces_client().await;

    let create_resp = client
        .create_connection_alias()
        .connection_string("perm-test.example.com")
        .send()
        .await
        .unwrap();

    let alias_id = create_resp.alias_id().unwrap().to_string();

    // Update permissions
    let perm = aws_sdk_workspaces::types::ConnectionAliasPermission::builder()
        .shared_account_id("111111111111")
        .allow_association(true)
        .build()
        .unwrap();

    client
        .update_connection_alias_permission()
        .alias_id(&alias_id)
        .connection_alias_permission(perm)
        .send()
        .await
        .expect("update_connection_alias_permission should succeed");

    // Describe permissions
    let desc_resp = client
        .describe_connection_alias_permissions()
        .alias_id(&alias_id)
        .send()
        .await
        .expect("describe_connection_alias_permissions should succeed");

    assert_eq!(desc_resp.alias_id(), Some(alias_id.as_str()));
    let perms = desc_resp.connection_alias_permissions();
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0].shared_account_id(), "111111111111");
}

#[tokio::test]
async fn test_create_and_describe_workspace_bundle() {
    let client = make_workspaces_client().await;

    // Create an image first (needed for bundle)
    let workspace_id = create_test_workspace(&client).await;
    let image_resp = client
        .create_workspace_image()
        .name("bundle-source-image")
        .description("Image for bundle test")
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();
    let image_id = image_resp.image_id().unwrap().to_string();

    // Create bundle
    let compute_type = aws_sdk_workspaces::types::ComputeType::builder()
        .name(aws_sdk_workspaces::types::Compute::Standard)
        .build();
    let user_storage = aws_sdk_workspaces::types::UserStorage::builder()
        .capacity("50")
        .build()
        .unwrap();

    let create_resp = client
        .create_workspace_bundle()
        .bundle_name("test-bundle")
        .bundle_description("A test bundle")
        .image_id(&image_id)
        .compute_type(compute_type)
        .user_storage(user_storage)
        .send()
        .await
        .expect("create_workspace_bundle should succeed");

    let bundle = create_resp.workspace_bundle().unwrap();
    let bundle_id = bundle.bundle_id().unwrap().to_string();
    assert_eq!(bundle.name(), Some("test-bundle"));

    // Describe bundles
    let desc_resp = client
        .describe_workspace_bundles()
        .bundle_ids(&bundle_id)
        .send()
        .await
        .expect("describe_workspace_bundles should succeed");

    let bundles = desc_resp.bundles();
    assert_eq!(bundles.len(), 1);
    assert_eq!(bundles[0].bundle_id(), Some(bundle_id.as_str()));
    assert_eq!(bundles[0].name(), Some("test-bundle"));
}

#[tokio::test]
async fn test_delete_workspace_bundle() {
    let client = make_workspaces_client().await;

    // Create image + bundle
    let workspace_id = create_test_workspace(&client).await;
    let image_resp = client
        .create_workspace_image()
        .name("bundle-del-image")
        .description("Image for delete bundle test")
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();
    let image_id = image_resp.image_id().unwrap().to_string();

    let compute_type = aws_sdk_workspaces::types::ComputeType::builder()
        .name(aws_sdk_workspaces::types::Compute::Value)
        .build();
    let user_storage = aws_sdk_workspaces::types::UserStorage::builder()
        .capacity("50")
        .build()
        .unwrap();

    let create_resp = client
        .create_workspace_bundle()
        .bundle_name("del-bundle")
        .image_id(&image_id)
        .compute_type(compute_type)
        .user_storage(user_storage)
        .send()
        .await
        .unwrap();

    let bundle_id = create_resp
        .workspace_bundle()
        .unwrap()
        .bundle_id()
        .unwrap()
        .to_string();

    client
        .delete_workspace_bundle()
        .bundle_id(&bundle_id)
        .send()
        .await
        .expect("delete_workspace_bundle should succeed");

    // Verify gone
    let desc = client
        .describe_workspace_bundles()
        .bundle_ids(&bundle_id)
        .send()
        .await
        .unwrap();
    assert!(desc.bundles().is_empty(), "bundle should be deleted");
}

#[tokio::test]
async fn test_delete_workspace_image() {
    let client = make_workspaces_client().await;

    let workspace_id = create_test_workspace(&client).await;
    let image_resp = client
        .create_workspace_image()
        .name("to-delete-image")
        .description("Will be deleted")
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();
    let image_id = image_resp.image_id().unwrap().to_string();

    client
        .delete_workspace_image()
        .image_id(&image_id)
        .send()
        .await
        .expect("delete_workspace_image should succeed");

    // Verify gone
    let desc = client
        .describe_workspace_images()
        .image_ids(&image_id)
        .send()
        .await
        .unwrap();
    assert!(desc.images().is_empty(), "image should be deleted");
}

#[tokio::test]
async fn test_create_and_describe_workspaces_pool() {
    let client = make_workspaces_client().await;

    // Register directory first
    client
        .register_workspace_directory()
        .directory_id("d-pool123456")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_workspaces_pool()
        .pool_name("test-pool")
        .description("A test pool")
        .bundle_id("wsb-abcdefgh1")
        .directory_id("d-pool123456")
        .send()
        .await
        .expect("create_workspaces_pool should succeed");

    let pool = create_resp.workspaces_pool().unwrap();
    let pool_id = pool.pool_id().to_string();
    assert_eq!(pool.pool_name(), "test-pool");
    assert_eq!(pool.description(), Some("A test pool"));

    // Describe pools
    let desc_resp = client
        .describe_workspaces_pools()
        .pool_ids(&pool_id)
        .send()
        .await
        .expect("describe_workspaces_pools should succeed");

    let pools = desc_resp.workspaces_pools();
    assert_eq!(pools.len(), 1);
    assert_eq!(pools[0].pool_id(), pool_id.as_str());
    assert_eq!(pools[0].pool_name(), "test-pool");
}

#[tokio::test]
async fn test_terminate_workspaces_pool() {
    let client = make_workspaces_client().await;

    client
        .register_workspace_directory()
        .directory_id("d-poolterm12")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_workspaces_pool()
        .pool_name("term-pool")
        .bundle_id("wsb-abcdefgh1")
        .directory_id("d-poolterm12")
        .send()
        .await
        .unwrap();

    let pool_id = create_resp.workspaces_pool().unwrap().pool_id().to_string();

    client
        .terminate_workspaces_pool()
        .pool_id(&pool_id)
        .send()
        .await
        .expect("terminate_workspaces_pool should succeed");

    // Verify gone
    let desc = client
        .describe_workspaces_pools()
        .pool_ids(&pool_id)
        .send()
        .await
        .unwrap();
    assert!(
        desc.workspaces_pools().is_empty(),
        "pool should be terminated"
    );
}

#[tokio::test]
async fn test_update_workspaces_pool() {
    let client = make_workspaces_client().await;

    client
        .register_workspace_directory()
        .directory_id("d-poolupd123")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_workspaces_pool()
        .pool_name("upd-pool")
        .bundle_id("wsb-abcdefgh1")
        .directory_id("d-poolupd123")
        .send()
        .await
        .unwrap();

    let pool_id = create_resp.workspaces_pool().unwrap().pool_id().to_string();

    // Update description
    let upd_resp = client
        .update_workspaces_pool()
        .pool_id(&pool_id)
        .description("Updated description")
        .send()
        .await
        .expect("update_workspaces_pool should succeed");

    let updated_pool = upd_resp.workspaces_pool().unwrap();
    assert_eq!(updated_pool.description(), Some("Updated description"));
}

#[tokio::test]
async fn test_start_and_stop_workspaces_pool() {
    let client = make_workspaces_client().await;

    client
        .register_workspace_directory()
        .directory_id("d-poolss1234")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_workspaces_pool()
        .pool_name("startstop-pool")
        .bundle_id("wsb-abcdefgh1")
        .directory_id("d-poolss1234")
        .send()
        .await
        .unwrap();

    let pool_id = create_resp.workspaces_pool().unwrap().pool_id().to_string();

    // Stop pool
    client
        .stop_workspaces_pool()
        .pool_id(&pool_id)
        .send()
        .await
        .expect("stop_workspaces_pool should succeed");

    // Start pool
    client
        .start_workspaces_pool()
        .pool_id(&pool_id)
        .send()
        .await
        .expect("start_workspaces_pool should succeed");
}

#[tokio::test]
async fn test_modify_workspace_properties_not_found() {
    let client = make_workspaces_client().await;

    let props = aws_sdk_workspaces::types::WorkspaceProperties::builder()
        .running_mode(aws_sdk_workspaces::types::RunningMode::AlwaysOn)
        .build();

    let err = client
        .modify_workspace_properties()
        .workspace_id("ws-nonexistent1")
        .workspace_properties(props)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_restore_workspace_not_found() {
    let client = make_workspaces_client().await;

    let err = client
        .restore_workspace()
        .workspace_id("ws-nonexistent1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}
