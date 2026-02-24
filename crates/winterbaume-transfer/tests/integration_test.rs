use aws_sdk_transfer::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_transfer::TransferService;

async fn make_transfer_client() -> aws_sdk_transfer::Client {
    let mock = MockAws::builder()
        .with_service(TransferService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_transfer::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_transfer::Client::new(&config)
}

#[tokio::test]
async fn test_create_server() {
    let client = make_transfer_client().await;

    let resp = client
        .create_server()
        .send()
        .await
        .expect("create_server should succeed");

    let server_id = resp.server_id();
    assert!(server_id.starts_with("s-"));
}

#[tokio::test]
async fn test_describe_server() {
    let client = make_transfer_client().await;

    let resp = client.create_server().send().await.unwrap();
    let server_id = resp.server_id().to_string();

    let desc = client
        .describe_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("describe_server should succeed");

    let server = desc.server().expect("should have server");
    assert_eq!(server.server_id(), Some(server_id.as_str()));
    assert_eq!(
        server.state(),
        Some(&aws_sdk_transfer::types::State::Online)
    );
    assert_eq!(
        server.endpoint_type(),
        Some(&aws_sdk_transfer::types::EndpointType::Public)
    );
}

#[tokio::test]
async fn test_list_servers() {
    let client = make_transfer_client().await;

    for _ in 0..3 {
        client.create_server().send().await.unwrap();
    }

    let resp = client
        .list_servers()
        .send()
        .await
        .expect("list_servers should succeed");

    assert_eq!(resp.servers().len(), 3);
}

#[tokio::test]
async fn test_delete_server() {
    let client = make_transfer_client().await;

    let resp = client.create_server().send().await.unwrap();
    let server_id = resp.server_id().to_string();

    client
        .delete_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("delete_server should succeed");

    let result = client.describe_server().server_id(&server_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_server() {
    let client = make_transfer_client().await;

    let result = client
        .delete_server()
        .server_id("s-nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

// --- User operations ---

async fn create_server_and_user(client: &aws_sdk_transfer::Client) -> (String, String) {
    let server = client.create_server().send().await.unwrap();
    let server_id = server.server_id().to_string();

    client
        .create_user()
        .server_id(&server_id)
        .user_name("testuser")
        .role("arn:aws:iam::123456789012:role/transfer-role")
        .home_directory("/bucket/home/testuser")
        .send()
        .await
        .expect("create_user should succeed");

    (server_id, "testuser".to_string())
}

#[tokio::test]
async fn test_create_and_describe_user() {
    let client = make_transfer_client().await;
    let (server_id, user_name) = create_server_and_user(&client).await;

    let resp = client
        .describe_user()
        .server_id(&server_id)
        .user_name(&user_name)
        .send()
        .await
        .expect("describe_user should succeed");

    assert_eq!(resp.server_id(), &server_id);
    let user = resp.user().unwrap();
    assert_eq!(user.user_name(), Some("testuser"));
    assert!(user.arn().contains("testuser"));
    assert_eq!(user.home_directory(), Some("/bucket/home/testuser"));
}

#[tokio::test]
async fn test_delete_user() {
    let client = make_transfer_client().await;
    let (server_id, user_name) = create_server_and_user(&client).await;

    client
        .delete_user()
        .server_id(&server_id)
        .user_name(&user_name)
        .send()
        .await
        .expect("delete_user should succeed");

    let result = client
        .describe_user()
        .server_id(&server_id)
        .user_name(&user_name)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_import_and_delete_ssh_public_key() {
    let client = make_transfer_client().await;
    let (server_id, user_name) = create_server_and_user(&client).await;

    let resp = client
        .import_ssh_public_key()
        .server_id(&server_id)
        .user_name(&user_name)
        .ssh_public_key_body("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC... test@example.com")
        .send()
        .await
        .expect("import_ssh_public_key should succeed");

    let key_id = resp.ssh_public_key_id().to_string();
    assert!(key_id.starts_with("key-"));

    let user_resp = client
        .describe_user()
        .server_id(&server_id)
        .user_name(&user_name)
        .send()
        .await
        .unwrap();
    let user = user_resp.user().unwrap();
    assert_eq!(user.ssh_public_keys().len(), 1);

    client
        .delete_ssh_public_key()
        .server_id(&server_id)
        .user_name(&user_name)
        .ssh_public_key_id(&key_id)
        .send()
        .await
        .expect("delete_ssh_public_key should succeed");

    let user_resp = client
        .describe_user()
        .server_id(&server_id)
        .user_name(&user_name)
        .send()
        .await
        .unwrap();
    let user = user_resp.user().unwrap();
    assert!(user.ssh_public_keys().is_empty());
}

#[tokio::test]
async fn test_describe_nonexistent_user() {
    let client = make_transfer_client().await;
    let server = client.create_server().send().await.unwrap();

    let result = client
        .describe_user()
        .server_id(server.server_id())
        .user_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: Transfer Family
// ============================================================================

#[tokio::test]
async fn test_create_server_with_options() {
    let client = make_transfer_client().await;

    let resp = client
        .create_server()
        .endpoint_type(aws_sdk_transfer::types::EndpointType::Public)
        .domain(aws_sdk_transfer::types::Domain::S3)
        .protocols(aws_sdk_transfer::types::Protocol::Sftp)
        .send()
        .await
        .expect("create_server with options should succeed");

    let server_id = resp.server_id().to_string();
    assert!(server_id.starts_with("s-"), "ServerId must start with s-");

    let desc = client
        .describe_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("describe_server should succeed");

    let server = desc.server().expect("should have server");
    assert_eq!(
        server.endpoint_type(),
        Some(&aws_sdk_transfer::types::EndpointType::Public)
    );
    assert_eq!(server.domain(), Some(&aws_sdk_transfer::types::Domain::S3));
    let protocols = server.protocols();
    assert_eq!(protocols.len(), 1);
    assert_eq!(protocols[0], aws_sdk_transfer::types::Protocol::Sftp);
}

#[tokio::test]
async fn test_create_server_with_tags() {
    let client = make_transfer_client().await;

    let tag = aws_sdk_transfer::types::Tag::builder()
        .key("Environment")
        .value("test")
        .build()
        .unwrap();

    let resp = client
        .create_server()
        .tags(tag)
        .send()
        .await
        .expect("create_server with tags should succeed");

    let server_id = resp.server_id().to_string();

    let desc = client
        .describe_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("describe_server should succeed");

    let server = desc.server().expect("should have server");
    let tags = server.tags();
    assert_eq!(tags.len(), 1, "Expected 1 tag");
    assert_eq!(tags[0].key(), "Environment");
    assert_eq!(tags[0].value(), "test");
}

#[tokio::test]
async fn test_describe_server_arn_format() {
    let client = make_transfer_client().await;

    let resp = client.create_server().send().await.unwrap();
    let server_id = resp.server_id().to_string();

    let desc = client
        .describe_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("describe_server should succeed");

    let server = desc.server().expect("should have server");
    let arn = server.arn();
    assert!(
        arn.starts_with("arn:aws:transfer:"),
        "ARN must start with arn:aws:transfer: got: {arn}"
    );
    assert!(
        arn.contains(&server_id),
        "ARN must contain the ServerId, got: {arn}"
    );
    assert!(
        arn.contains(":server/"),
        "ARN must contain :server/, got: {arn}"
    );
}

#[tokio::test]
async fn test_describe_server_not_found() {
    let client = make_transfer_client().await;

    let result = client
        .describe_server()
        .server_id("s-00000000000000000")
        .send()
        .await;

    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_servers_empty() {
    let client = make_transfer_client().await;

    let resp = client
        .list_servers()
        .send()
        .await
        .expect("list_servers should succeed");

    assert_eq!(resp.servers().len(), 0, "Expected empty list");
}

#[tokio::test]
async fn test_list_servers_fields() {
    let client = make_transfer_client().await;

    let create_resp = client.create_server().send().await.unwrap();
    let server_id = create_resp.server_id().to_string();

    let resp = client
        .list_servers()
        .send()
        .await
        .expect("list_servers should succeed");

    assert_eq!(resp.servers().len(), 1);
    let entry = &resp.servers()[0];

    assert_eq!(
        entry.server_id(),
        Some(server_id.as_str()),
        "ServerId must match"
    );
    assert_eq!(
        entry.state(),
        Some(&aws_sdk_transfer::types::State::Online),
        "State must be ONLINE"
    );
    assert_eq!(
        entry.endpoint_type(),
        Some(&aws_sdk_transfer::types::EndpointType::Public),
        "EndpointType must be PUBLIC"
    );
    assert!(!entry.arn().is_empty(), "ARN must not be empty");
}

#[tokio::test]
async fn test_server_full_lifecycle() {
    let client = make_transfer_client().await;

    // Create
    let resp = client.create_server().send().await.unwrap();
    let server_id = resp.server_id().to_string();

    // Describe
    let desc = client
        .describe_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("describe should succeed after create");
    assert_eq!(desc.server().unwrap().server_id(), Some(server_id.as_str()));

    // Delete
    client
        .delete_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client.describe_server().server_id(&server_id).send().await;
    assert!(result.is_err(), "describe after delete should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_user_duplicate() {
    let client = make_transfer_client().await;
    let server = client.create_server().send().await.unwrap();
    let server_id = server.server_id().to_string();

    // First creation succeeds
    client
        .create_user()
        .server_id(&server_id)
        .user_name("dupuser")
        .role("arn:aws:iam::123456789012:role/transfer-role")
        .send()
        .await
        .expect("first create_user should succeed");

    // Second creation with same name must fail
    let result = client
        .create_user()
        .server_id(&server_id)
        .user_name("dupuser")
        .role("arn:aws:iam::123456789012:role/transfer-role")
        .send()
        .await;

    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceExistsException"),
        "Expected ResourceExistsException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_user_nonexistent_server() {
    let client = make_transfer_client().await;

    let result = client
        .create_user()
        .server_id("s-00000000000000000")
        .user_name("someuser")
        .role("arn:aws:iam::123456789012:role/transfer-role")
        .send()
        .await;

    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_user_role_field() {
    let client = make_transfer_client().await;
    let server = client.create_server().send().await.unwrap();
    let server_id = server.server_id().to_string();
    let role_arn = "arn:aws:iam::123456789012:role/my-transfer-role";

    client
        .create_user()
        .server_id(&server_id)
        .user_name("roleuser")
        .role(role_arn)
        .send()
        .await
        .expect("create_user should succeed");

    let resp = client
        .describe_user()
        .server_id(&server_id)
        .user_name("roleuser")
        .send()
        .await
        .expect("describe_user should succeed");

    let user = resp.user().expect("should have user");
    assert_eq!(user.role(), Some(role_arn), "Role must match");
}

#[tokio::test]
async fn test_delete_user_not_found() {
    let client = make_transfer_client().await;
    let server = client.create_server().send().await.unwrap();

    let result = client
        .delete_user()
        .server_id(server.server_id())
        .user_name("ghostuser")
        .send()
        .await;

    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_import_ssh_public_key_not_found_server() {
    let client = make_transfer_client().await;

    let result = client
        .import_ssh_public_key()
        .server_id("s-00000000000000000")
        .user_name("anyuser")
        .ssh_public_key_body("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC... test@example.com")
        .send()
        .await;

    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_import_ssh_public_key_not_found_user() {
    let client = make_transfer_client().await;
    let server = client.create_server().send().await.unwrap();

    let result = client
        .import_ssh_public_key()
        .server_id(server.server_id())
        .user_name("nosuchuser")
        .ssh_public_key_body("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC... test@example.com")
        .send()
        .await;

    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_ssh_public_key_not_found() {
    let client = make_transfer_client().await;
    let (server_id, user_name) = create_server_and_user(&client).await;

    let result = client
        .delete_ssh_public_key()
        .server_id(&server_id)
        .user_name(&user_name)
        .ssh_public_key_id("key-00000000000000000")
        .send()
        .await;

    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_import_multiple_ssh_keys() {
    let client = make_transfer_client().await;
    let (server_id, user_name) = create_server_and_user(&client).await;

    let key1_resp = client
        .import_ssh_public_key()
        .server_id(&server_id)
        .user_name(&user_name)
        .ssh_public_key_body("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC... key1@example.com")
        .send()
        .await
        .expect("first import should succeed");
    let key1_id = key1_resp.ssh_public_key_id().to_string();

    let key2_resp = client
        .import_ssh_public_key()
        .server_id(&server_id)
        .user_name(&user_name)
        .ssh_public_key_body("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQD... key2@example.com")
        .send()
        .await
        .expect("second import should succeed");
    let key2_id = key2_resp.ssh_public_key_id().to_string();

    assert_ne!(key1_id, key2_id, "Two imported keys must have distinct IDs");
    assert!(key1_id.starts_with("key-"), "Key ID must start with key-");
    assert!(key2_id.starts_with("key-"), "Key ID must start with key-");

    let desc = client
        .describe_user()
        .server_id(&server_id)
        .user_name(&user_name)
        .send()
        .await
        .expect("describe_user should succeed");

    let user = desc.user().expect("should have user");
    assert_eq!(
        user.ssh_public_keys().len(),
        2,
        "User should have 2 SSH keys"
    );
}

#[tokio::test]
async fn test_user_full_lifecycle() {
    let client = make_transfer_client().await;

    // Create server
    let server_resp = client.create_server().send().await.unwrap();
    let server_id = server_resp.server_id().to_string();

    // Create user
    client
        .create_user()
        .server_id(&server_id)
        .user_name("lifecycleuser")
        .role("arn:aws:iam::123456789012:role/transfer-role")
        .home_directory("/bucket/home/lifecycleuser")
        .send()
        .await
        .expect("create_user should succeed");

    // Describe
    let desc = client
        .describe_user()
        .server_id(&server_id)
        .user_name("lifecycleuser")
        .send()
        .await
        .expect("describe_user after create should succeed");
    assert_eq!(
        desc.user().unwrap().user_name(),
        Some("lifecycleuser"),
        "UserName must match"
    );

    // Delete
    client
        .delete_user()
        .server_id(&server_id)
        .user_name("lifecycleuser")
        .send()
        .await
        .expect("delete_user should succeed");

    // Verify gone
    let result = client
        .describe_user()
        .server_id(&server_id)
        .user_name("lifecycleuser")
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// UpdateServer tests
// ============================================================================

#[tokio::test]
async fn test_update_server_protocols() {
    let client = make_transfer_client().await;

    let resp = client.create_server().send().await.unwrap();
    let server_id = resp.server_id().to_string();

    client
        .update_server()
        .server_id(&server_id)
        .protocols(aws_sdk_transfer::types::Protocol::Sftp)
        .protocols(aws_sdk_transfer::types::Protocol::Ftps)
        .send()
        .await
        .expect("update_server should succeed");

    let desc = client
        .describe_server()
        .server_id(&server_id)
        .send()
        .await
        .unwrap();
    let server = desc.server().unwrap();
    let protocols = server.protocols();
    assert_eq!(protocols.len(), 2, "Should have 2 protocols after update");
}

#[tokio::test]
async fn test_update_server_not_found() {
    let client = make_transfer_client().await;

    let result = client
        .update_server()
        .server_id("s-00000000000000000")
        .send()
        .await;

    assert!(result.is_err(), "update on nonexistent server should fail");
}

// ============================================================================
// ListUsers tests
// ============================================================================

#[tokio::test]
async fn test_list_users() {
    let client = make_transfer_client().await;

    let server = client.create_server().send().await.unwrap();
    let server_id = server.server_id().to_string();

    for i in 0..3 {
        client
            .create_user()
            .server_id(&server_id)
            .user_name(format!("user{i}"))
            .role("arn:aws:iam::123456789012:role/transfer-role")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_users()
        .server_id(&server_id)
        .send()
        .await
        .expect("list_users should succeed");

    assert_eq!(resp.server_id(), &server_id);
    assert_eq!(resp.users().len(), 3, "Should have 3 users");
}

#[tokio::test]
async fn test_list_users_empty() {
    let client = make_transfer_client().await;
    let server = client.create_server().send().await.unwrap();

    let resp = client
        .list_users()
        .server_id(server.server_id())
        .send()
        .await
        .expect("list_users should succeed");

    assert_eq!(resp.users().len(), 0, "Should have 0 users");
}

// ============================================================================
// UpdateUser tests
// ============================================================================

#[tokio::test]
async fn test_update_user_home_directory() {
    let client = make_transfer_client().await;
    let (server_id, user_name) = create_server_and_user(&client).await;

    client
        .update_user()
        .server_id(&server_id)
        .user_name(&user_name)
        .home_directory("/new-bucket/home/testuser")
        .send()
        .await
        .expect("update_user should succeed");

    let desc = client
        .describe_user()
        .server_id(&server_id)
        .user_name(&user_name)
        .send()
        .await
        .unwrap();
    let user = desc.user().unwrap();
    assert_eq!(
        user.home_directory(),
        Some("/new-bucket/home/testuser"),
        "HomeDirectory should be updated"
    );
}

#[tokio::test]
async fn test_update_user_not_found() {
    let client = make_transfer_client().await;
    let server = client.create_server().send().await.unwrap();

    let result = client
        .update_user()
        .server_id(server.server_id())
        .user_name("nonexistent")
        .home_directory("/foo")
        .send()
        .await;

    assert!(result.is_err(), "update on nonexistent user should fail");
}

// ============================================================================
// Agreement tests
// ============================================================================

async fn create_profiles_and_agreement(client: &aws_sdk_transfer::Client) -> (String, String) {
    let server = client.create_server().send().await.unwrap();
    let server_id = server.server_id().to_string();

    let local_profile = client
        .create_profile()
        .profile_type(aws_sdk_transfer::types::ProfileType::Local)
        .as2_id("local-as2-id")
        .send()
        .await
        .unwrap();
    let local_profile_id = local_profile.profile_id().to_string();

    let partner_profile = client
        .create_profile()
        .profile_type(aws_sdk_transfer::types::ProfileType::Partner)
        .as2_id("partner-as2-id")
        .send()
        .await
        .unwrap();
    let partner_profile_id = partner_profile.profile_id().to_string();

    let agreement = client
        .create_agreement()
        .server_id(&server_id)
        .local_profile_id(&local_profile_id)
        .partner_profile_id(&partner_profile_id)
        .base_directory("/bucket/base")
        .access_role("arn:aws:iam::123456789012:role/agreement-role")
        .send()
        .await
        .expect("create_agreement should succeed");

    let agreement_id = agreement.agreement_id().to_string();
    (server_id, agreement_id)
}

#[tokio::test]
async fn test_create_and_describe_agreement() {
    let client = make_transfer_client().await;
    let (_server_id, agreement_id) = create_profiles_and_agreement(&client).await;

    assert!(
        agreement_id.starts_with("a-"),
        "AgreementId must start with a-"
    );

    let desc = client
        .describe_agreement()
        .agreement_id(&agreement_id)
        .server_id(&_server_id)
        .send()
        .await
        .expect("describe_agreement should succeed");

    let agreement = desc.agreement().unwrap();
    assert_eq!(agreement.agreement_id(), Some(agreement_id.as_str()));
    assert_eq!(agreement.base_directory(), Some("/bucket/base"));
}

#[tokio::test]
async fn test_list_agreements() {
    let client = make_transfer_client().await;
    let (server_id, _agreement_id) = create_profiles_and_agreement(&client).await;

    let resp = client
        .list_agreements()
        .server_id(&server_id)
        .send()
        .await
        .expect("list_agreements should succeed");

    assert_eq!(resp.agreements().len(), 1);
}

#[tokio::test]
async fn test_update_agreement() {
    let client = make_transfer_client().await;
    let (server_id, agreement_id) = create_profiles_and_agreement(&client).await;

    client
        .update_agreement()
        .agreement_id(&agreement_id)
        .server_id(&server_id)
        .base_directory("/new-bucket/base")
        .send()
        .await
        .expect("update_agreement should succeed");

    let desc = client
        .describe_agreement()
        .agreement_id(&agreement_id)
        .server_id(&server_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.agreement().unwrap().base_directory(),
        Some("/new-bucket/base")
    );
}

#[tokio::test]
async fn test_delete_agreement() {
    let client = make_transfer_client().await;
    let (server_id, agreement_id) = create_profiles_and_agreement(&client).await;

    client
        .delete_agreement()
        .agreement_id(&agreement_id)
        .server_id(&server_id)
        .send()
        .await
        .expect("delete_agreement should succeed");

    let result = client
        .describe_agreement()
        .agreement_id(&agreement_id)
        .server_id(&server_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ============================================================================
// Certificate tests
// ============================================================================

#[tokio::test]
async fn test_import_and_describe_certificate() {
    let client = make_transfer_client().await;

    let resp = client
        .import_certificate()
        .usage(aws_sdk_transfer::types::CertificateUsageType::Signing)
        .certificate("-----BEGIN CERTIFICATE-----\nMIIBtest\n-----END CERTIFICATE-----")
        .send()
        .await
        .expect("import_certificate should succeed");

    let cert_id = resp.certificate_id().to_string();
    assert!(
        cert_id.starts_with("cert-"),
        "CertificateId must start with cert-"
    );

    let desc = client
        .describe_certificate()
        .certificate_id(&cert_id)
        .send()
        .await
        .expect("describe_certificate should succeed");

    let cert = desc.certificate().unwrap();
    assert_eq!(cert.certificate_id(), Some(cert_id.as_str()));
    assert_eq!(
        cert.usage(),
        Some(&aws_sdk_transfer::types::CertificateUsageType::Signing)
    );
}

#[tokio::test]
async fn test_list_certificates() {
    let client = make_transfer_client().await;

    client
        .import_certificate()
        .usage(aws_sdk_transfer::types::CertificateUsageType::Signing)
        .certificate("-----BEGIN CERTIFICATE-----\nMIIBtest1\n-----END CERTIFICATE-----")
        .send()
        .await
        .unwrap();

    client
        .import_certificate()
        .usage(aws_sdk_transfer::types::CertificateUsageType::Encryption)
        .certificate("-----BEGIN CERTIFICATE-----\nMIIBtest2\n-----END CERTIFICATE-----")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_certificates()
        .send()
        .await
        .expect("list_certificates should succeed");

    assert_eq!(resp.certificates().len(), 2);
}

#[tokio::test]
async fn test_update_certificate() {
    let client = make_transfer_client().await;

    let resp = client
        .import_certificate()
        .usage(aws_sdk_transfer::types::CertificateUsageType::Signing)
        .certificate("-----BEGIN CERTIFICATE-----\nMIIBtest\n-----END CERTIFICATE-----")
        .description("original")
        .send()
        .await
        .unwrap();
    let cert_id = resp.certificate_id().to_string();

    client
        .update_certificate()
        .certificate_id(&cert_id)
        .description("updated")
        .send()
        .await
        .expect("update_certificate should succeed");

    let desc = client
        .describe_certificate()
        .certificate_id(&cert_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.certificate().unwrap().description(), Some("updated"));
}

#[tokio::test]
async fn test_delete_certificate() {
    let client = make_transfer_client().await;

    let resp = client
        .import_certificate()
        .usage(aws_sdk_transfer::types::CertificateUsageType::Signing)
        .certificate("-----BEGIN CERTIFICATE-----\nMIIBtest\n-----END CERTIFICATE-----")
        .send()
        .await
        .unwrap();
    let cert_id = resp.certificate_id().to_string();

    client
        .delete_certificate()
        .certificate_id(&cert_id)
        .send()
        .await
        .expect("delete_certificate should succeed");

    let result = client
        .describe_certificate()
        .certificate_id(&cert_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ============================================================================
// Connector tests
// ============================================================================

#[tokio::test]
async fn test_create_and_describe_connector() {
    let client = make_transfer_client().await;

    let resp = client
        .create_connector()
        .url("sftp://example.com")
        .access_role("arn:aws:iam::123456789012:role/connector-role")
        .send()
        .await
        .expect("create_connector should succeed");

    let connector_id = resp.connector_id().to_string();
    assert!(
        connector_id.starts_with("c-"),
        "ConnectorId must start with c-"
    );

    let desc = client
        .describe_connector()
        .connector_id(&connector_id)
        .send()
        .await
        .expect("describe_connector should succeed");

    let connector = desc.connector().unwrap();
    assert_eq!(connector.connector_id(), Some(connector_id.as_str()));
    assert_eq!(connector.url(), Some("sftp://example.com"));
}

#[tokio::test]
async fn test_list_connectors() {
    let client = make_transfer_client().await;

    client
        .create_connector()
        .url("sftp://example1.com")
        .access_role("arn:aws:iam::123456789012:role/connector-role")
        .send()
        .await
        .unwrap();

    client
        .create_connector()
        .url("sftp://example2.com")
        .access_role("arn:aws:iam::123456789012:role/connector-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_connectors()
        .send()
        .await
        .expect("list_connectors should succeed");

    assert_eq!(resp.connectors().len(), 2);
}

#[tokio::test]
async fn test_update_connector() {
    let client = make_transfer_client().await;

    let resp = client
        .create_connector()
        .url("sftp://example.com")
        .access_role("arn:aws:iam::123456789012:role/connector-role")
        .send()
        .await
        .unwrap();
    let connector_id = resp.connector_id().to_string();

    client
        .update_connector()
        .connector_id(&connector_id)
        .url("sftp://updated.example.com")
        .send()
        .await
        .expect("update_connector should succeed");

    let desc = client
        .describe_connector()
        .connector_id(&connector_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.connector().unwrap().url(),
        Some("sftp://updated.example.com")
    );
}

#[tokio::test]
async fn test_delete_connector() {
    let client = make_transfer_client().await;

    let resp = client
        .create_connector()
        .url("sftp://example.com")
        .access_role("arn:aws:iam::123456789012:role/connector-role")
        .send()
        .await
        .unwrap();
    let connector_id = resp.connector_id().to_string();

    client
        .delete_connector()
        .connector_id(&connector_id)
        .send()
        .await
        .expect("delete_connector should succeed");

    let result = client
        .describe_connector()
        .connector_id(&connector_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ============================================================================
// Profile tests
// ============================================================================

#[tokio::test]
async fn test_create_and_describe_profile() {
    let client = make_transfer_client().await;

    let resp = client
        .create_profile()
        .profile_type(aws_sdk_transfer::types::ProfileType::Local)
        .as2_id("my-as2-id")
        .send()
        .await
        .expect("create_profile should succeed");

    let profile_id = resp.profile_id().to_string();
    assert!(profile_id.starts_with("p-"), "ProfileId must start with p-");

    let desc = client
        .describe_profile()
        .profile_id(&profile_id)
        .send()
        .await
        .expect("describe_profile should succeed");

    let profile = desc.profile().unwrap();
    assert_eq!(profile.profile_id(), Some(profile_id.as_str()));
    assert_eq!(profile.as2_id(), Some("my-as2-id"));
    assert_eq!(
        profile.profile_type(),
        Some(&aws_sdk_transfer::types::ProfileType::Local)
    );
}

#[tokio::test]
async fn test_list_profiles() {
    let client = make_transfer_client().await;

    client
        .create_profile()
        .profile_type(aws_sdk_transfer::types::ProfileType::Local)
        .as2_id("local-id")
        .send()
        .await
        .unwrap();

    client
        .create_profile()
        .profile_type(aws_sdk_transfer::types::ProfileType::Partner)
        .as2_id("partner-id")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_profiles()
        .send()
        .await
        .expect("list_profiles should succeed");

    assert_eq!(resp.profiles().len(), 2);
}

#[tokio::test]
async fn test_update_profile() {
    let client = make_transfer_client().await;

    let cert_resp = client
        .import_certificate()
        .usage(aws_sdk_transfer::types::CertificateUsageType::Signing)
        .certificate("-----BEGIN CERTIFICATE-----\nMIIBtest\n-----END CERTIFICATE-----")
        .send()
        .await
        .unwrap();
    let cert_id = cert_resp.certificate_id().to_string();

    let resp = client
        .create_profile()
        .profile_type(aws_sdk_transfer::types::ProfileType::Local)
        .as2_id("my-as2-id")
        .send()
        .await
        .unwrap();
    let profile_id = resp.profile_id().to_string();

    client
        .update_profile()
        .profile_id(&profile_id)
        .certificate_ids(&cert_id)
        .send()
        .await
        .expect("update_profile should succeed");

    let desc = client
        .describe_profile()
        .profile_id(&profile_id)
        .send()
        .await
        .unwrap();
    let profile = desc.profile().unwrap();
    assert_eq!(profile.certificate_ids().len(), 1);
    assert_eq!(profile.certificate_ids()[0], cert_id);
}

#[tokio::test]
async fn test_delete_profile() {
    let client = make_transfer_client().await;

    let resp = client
        .create_profile()
        .profile_type(aws_sdk_transfer::types::ProfileType::Local)
        .as2_id("my-as2-id")
        .send()
        .await
        .unwrap();
    let profile_id = resp.profile_id().to_string();

    client
        .delete_profile()
        .profile_id(&profile_id)
        .send()
        .await
        .expect("delete_profile should succeed");

    let result = client
        .describe_profile()
        .profile_id(&profile_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ============================================================================
// WebApp tests
// ============================================================================

#[tokio::test]
async fn test_create_and_describe_web_app() {
    let client = make_transfer_client().await;

    let resp = client
        .create_web_app()
        .identity_provider_details(
            aws_sdk_transfer::types::WebAppIdentityProviderDetails::IdentityCenterConfig(
                aws_sdk_transfer::types::IdentityCenterConfig::builder()
                    .instance_arn("arn:aws:sso:::instance/ssoins-0000000000000000")
                    .role("arn:aws:iam::123456789012:role/webapp-role")
                    .build(),
            ),
        )
        .send()
        .await
        .expect("create_web_app should succeed");

    let web_app_id = resp.web_app_id().to_string();
    assert!(
        web_app_id.starts_with("webapp-"),
        "WebAppId must start with webapp-"
    );

    let desc = client
        .describe_web_app()
        .web_app_id(&web_app_id)
        .send()
        .await
        .expect("describe_web_app should succeed");

    let web_app = desc.web_app().unwrap();
    assert_eq!(web_app.web_app_id(), web_app_id.as_str());
    assert!(web_app.arn().contains("webapp"));
}

#[tokio::test]
async fn test_list_web_apps() {
    let client = make_transfer_client().await;

    client
        .create_web_app()
        .identity_provider_details(
            aws_sdk_transfer::types::WebAppIdentityProviderDetails::IdentityCenterConfig(
                aws_sdk_transfer::types::IdentityCenterConfig::builder()
                    .instance_arn("arn:aws:sso:::instance/ssoins-0000000000000000")
                    .role("arn:aws:iam::123456789012:role/webapp-role")
                    .build(),
            ),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_web_apps()
        .send()
        .await
        .expect("list_web_apps should succeed");

    assert_eq!(resp.web_apps().len(), 1);
}

#[tokio::test]
async fn test_delete_web_app() {
    let client = make_transfer_client().await;

    let resp = client
        .create_web_app()
        .identity_provider_details(
            aws_sdk_transfer::types::WebAppIdentityProviderDetails::IdentityCenterConfig(
                aws_sdk_transfer::types::IdentityCenterConfig::builder()
                    .instance_arn("arn:aws:sso:::instance/ssoins-0000000000000000")
                    .role("arn:aws:iam::123456789012:role/webapp-role")
                    .build(),
            ),
        )
        .send()
        .await
        .unwrap();
    let web_app_id = resp.web_app_id().to_string();

    client
        .delete_web_app()
        .web_app_id(&web_app_id)
        .send()
        .await
        .expect("delete_web_app should succeed");

    let result = client
        .describe_web_app()
        .web_app_id(&web_app_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ============================================================================
// Workflow tests
// ============================================================================

#[tokio::test]
async fn test_create_and_describe_workflow() {
    let client = make_transfer_client().await;

    let resp = client
        .create_workflow()
        .description("test workflow")
        .send()
        .await
        .expect("create_workflow should succeed");

    let workflow_id = resp.workflow_id().to_string();
    assert!(
        workflow_id.starts_with("w-"),
        "WorkflowId must start with w-"
    );

    let desc = client
        .describe_workflow()
        .workflow_id(&workflow_id)
        .send()
        .await
        .expect("describe_workflow should succeed");

    let workflow = desc.workflow().unwrap();
    assert_eq!(workflow.workflow_id(), Some(workflow_id.as_str()));
    assert_eq!(workflow.description(), Some("test workflow"));
}

#[tokio::test]
async fn test_list_workflows() {
    let client = make_transfer_client().await;

    client
        .create_workflow()
        .description("wf1")
        .send()
        .await
        .unwrap();

    client
        .create_workflow()
        .description("wf2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_workflows()
        .send()
        .await
        .expect("list_workflows should succeed");

    assert_eq!(resp.workflows().len(), 2);
}

#[tokio::test]
async fn test_delete_workflow() {
    let client = make_transfer_client().await;

    let resp = client
        .create_workflow()
        .description("to-delete")
        .send()
        .await
        .unwrap();
    let workflow_id = resp.workflow_id().to_string();

    client
        .delete_workflow()
        .workflow_id(&workflow_id)
        .send()
        .await
        .expect("delete_workflow should succeed");

    let result = client
        .describe_workflow()
        .workflow_id(&workflow_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

/// Covers FIX(terraform-e2e): create_connector must set status to "ACTIVE"
/// (not "ONLINE") because the Terraform provider waiter expects "ACTIVE".
#[tokio::test]
async fn test_connector_status_is_active_on_create() {
    let client = make_transfer_client().await;

    let resp = client
        .create_connector()
        .url("sftp://example.com")
        .access_role("arn:aws:iam::123456789012:role/connector-role")
        .send()
        .await
        .expect("create_connector should succeed");

    let connector_id = resp.connector_id().to_string();

    let desc = client
        .describe_connector()
        .connector_id(&connector_id)
        .send()
        .await
        .expect("describe_connector should succeed");

    let connector = desc.connector().expect("should have connector");
    assert_eq!(
        connector.status(),
        &aws_sdk_transfer::types::ConnectorStatus::Active,
        "Connector status must be ACTIVE after creation (FIX: was ONLINE before)"
    );
}

/// Covers FIX(terraform-e2e): server state must be ONLINE after creation,
/// which is the value the Terraform provider expects for servers.
#[tokio::test]
async fn test_server_state_is_online_on_create() {
    let client = make_transfer_client().await;

    let resp = client
        .create_server()
        .send()
        .await
        .expect("create_server should succeed");

    let server_id = resp.server_id().to_string();

    let desc = client
        .describe_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("describe_server should succeed");

    let server = desc.server().expect("should have server");
    assert_eq!(
        server.state(),
        Some(&aws_sdk_transfer::types::State::Online),
        "Server state must be ONLINE after creation"
    );
}

/// Covers FIX(terraform-e2e) in views.rs: connector_view_to_connector must
/// preserve "ACTIVE" status through a snapshot/restore round-trip.
#[tokio::test]
async fn test_connector_status_survives_snapshot_restore() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_transfer::views::{ConnectorView, TransferStateView};

    let svc = TransferService::new();

    // Restore a state view that contains a connector with explicit "ACTIVE" status
    let mut connectors = HashMap::new();
    connectors.insert(
        "c-test00000000001".to_string(),
        ConnectorView {
            connector_id: "c-test00000000001".to_string(),
            arn: "arn:aws:transfer:us-east-1:123456789012:connector/c-test00000000001".to_string(),
            url: Some("sftp://snapshot-test.example.com".to_string()),
            as2_config: None,
            sftp_config: None,
            access_role: "arn:aws:iam::123456789012:role/connector-role".to_string(),
            logging_role: None,
            status: "ACTIVE".to_string(),
            tags: vec![],
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        TransferStateView {
            connectors,
            ..Default::default()
        },
    )
    .await
    .expect("restore should succeed");

    // Snapshot and verify the status round-trips correctly
    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    let connector = snapshot
        .connectors
        .get("c-test00000000001")
        .expect("connector should exist in snapshot");
    assert_eq!(
        connector.status, "ACTIVE",
        "Connector status must be ACTIVE after snapshot round-trip"
    );
}

/// Covers FIX(terraform-e2e) in views.rs: connector_view_to_connector must
/// default an empty status string to "ACTIVE" during restore.
#[tokio::test]
async fn test_connector_empty_status_defaults_to_active() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_transfer::views::{ConnectorView, TransferStateView};

    let svc = TransferService::new();

    // Restore a connector view with an empty status string (simulates old state)
    let mut connectors = HashMap::new();
    connectors.insert(
        "c-test00000000002".to_string(),
        ConnectorView {
            connector_id: "c-test00000000002".to_string(),
            arn: "arn:aws:transfer:us-east-1:123456789012:connector/c-test00000000002".to_string(),
            url: Some("sftp://empty-status.example.com".to_string()),
            as2_config: None,
            sftp_config: None,
            access_role: "arn:aws:iam::123456789012:role/connector-role".to_string(),
            logging_role: None,
            status: "".to_string(),
            tags: vec![],
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        TransferStateView {
            connectors,
            ..Default::default()
        },
    )
    .await
    .expect("restore should succeed");

    // After restore, snapshot and verify empty status was defaulted to ACTIVE
    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    let connector = snapshot
        .connectors
        .get("c-test00000000002")
        .expect("connector should exist in snapshot");
    assert_eq!(
        connector.status, "ACTIVE",
        "Empty connector status must default to ACTIVE after restore (FIX in views.rs)"
    );
}
