use aws_sdk_cognitoidentityprovider::config::BehaviorVersion;
use winterbaume_cognitoidentityprovider::CognitoIdentityProviderService;
use winterbaume_core::MockAws;

async fn make_cognito_client() -> aws_sdk_cognitoidentityprovider::Client {
    let mock = MockAws::builder()
        .with_service(CognitoIdentityProviderService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cognitoidentityprovider::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    aws_sdk_cognitoidentityprovider::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_user_pool() {
    let client = make_cognito_client().await;

    let resp = client
        .create_user_pool()
        .pool_name("TestPool")
        .send()
        .await
        .expect("create_user_pool should succeed");

    let pool = resp.user_pool().expect("should have user pool");
    assert_eq!(pool.name(), Some("TestPool"));
    assert!(pool.id().is_some());
    assert!(pool.arn().is_some());

    let pool_id = pool.id().unwrap();

    let desc_resp = client
        .describe_user_pool()
        .user_pool_id(pool_id)
        .send()
        .await
        .expect("describe_user_pool should succeed");

    let pool = desc_resp.user_pool().expect("should have user pool");
    assert_eq!(pool.name(), Some("TestPool"));
}

#[tokio::test]
async fn test_delete_user_pool() {
    let client = make_cognito_client().await;

    let resp = client
        .create_user_pool()
        .pool_name("DelPool")
        .send()
        .await
        .unwrap();

    let pool_id = resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .delete_user_pool()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("delete_user_pool should succeed");

    let result = client
        .describe_user_pool()
        .user_pool_id(&pool_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_list_user_pools() {
    let client = make_cognito_client().await;

    client
        .create_user_pool()
        .pool_name("Pool1")
        .send()
        .await
        .unwrap();

    client
        .create_user_pool()
        .pool_name("Pool2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_user_pools()
        .max_results(10)
        .send()
        .await
        .expect("list_user_pools should succeed");

    assert_eq!(resp.user_pools().len(), 2);
}

#[tokio::test]
async fn test_create_user_pool_client() {
    let client = make_cognito_client().await;

    let pool_resp = client
        .create_user_pool()
        .pool_name("ClientPool")
        .send()
        .await
        .unwrap();

    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    let resp = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("test-client")
        .send()
        .await
        .expect("create_user_pool_client should succeed");

    let upc = resp
        .user_pool_client()
        .expect("should have user pool client");
    assert_eq!(upc.client_name(), Some("test-client"));
    assert!(upc.client_id().is_some());
    assert_eq!(upc.user_pool_id(), Some(pool_id.as_str()));
}

#[tokio::test]
async fn test_admin_create_and_get_user() {
    let client = make_cognito_client().await;

    let pool_resp = client
        .create_user_pool()
        .pool_name("UserPool")
        .send()
        .await
        .unwrap();

    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    let attr = aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
        .name("email")
        .value("test@example.com")
        .build()
        .unwrap();

    let resp = client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("testuser")
        .user_attributes(attr)
        .send()
        .await
        .expect("admin_create_user should succeed");

    let user = resp.user().expect("should have user");
    assert_eq!(user.username(), Some("testuser"));

    let get_resp = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("testuser")
        .send()
        .await
        .expect("admin_get_user should succeed");

    assert_eq!(get_resp.username(), "testuser");
}

#[tokio::test]
async fn test_admin_create_duplicate_user_fails() {
    let client = make_cognito_client().await;

    let pool_resp = client
        .create_user_pool()
        .pool_name("DupUserPool")
        .send()
        .await
        .unwrap();

    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("dupuser")
        .send()
        .await
        .unwrap();

    let result = client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("dupuser")
        .send()
        .await;
    assert!(result.is_err(), "duplicate user should fail");
}

#[tokio::test]
async fn test_admin_get_nonexistent_user_fails() {
    let client = make_cognito_client().await;

    let pool_resp = client
        .create_user_pool()
        .pool_name("NoUserPool")
        .send()
        .await
        .unwrap();

    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    let result = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent user should fail");
}

// --- Helper to create a pool + user + client for reuse ---

async fn setup_pool_with_user_and_client(
    client: &aws_sdk_cognitoidentityprovider::Client,
) -> (String, String) {
    let pool_resp = client
        .create_user_pool()
        .pool_name("TestPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("testuser")
        .send()
        .await
        .unwrap();

    let client_resp = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("test-app")
        .send()
        .await
        .unwrap();
    let client_id = client_resp
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    (pool_id, client_id)
}

// --- UpdateUserPool ---

#[tokio::test]
async fn test_update_user_pool() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("UpdPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .update_user_pool()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("update_user_pool should succeed");
}

// --- DescribeUserPoolClient / UpdateUserPoolClient / DeleteUserPoolClient / ListUserPoolClients ---

#[tokio::test]
async fn test_user_pool_client_lifecycle() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("ClientLifecycle")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    // Create
    let create_resp = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("my-app")
        .send()
        .await
        .unwrap();
    let client_id = create_resp
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    // Describe
    let desc_resp = client
        .describe_user_pool_client()
        .user_pool_id(&pool_id)
        .client_id(&client_id)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(
        desc_resp.user_pool_client().unwrap().client_name(),
        Some("my-app")
    );

    // Update
    let upd_resp = client
        .update_user_pool_client()
        .user_pool_id(&pool_id)
        .client_id(&client_id)
        .client_name("renamed-app")
        .send()
        .await
        .expect("update should succeed");
    assert_eq!(
        upd_resp.user_pool_client().unwrap().client_name(),
        Some("renamed-app")
    );

    // List
    let list_resp = client
        .list_user_pool_clients()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.user_pool_clients().len(), 1);

    // Delete
    client
        .delete_user_pool_client()
        .user_pool_id(&pool_id)
        .client_id(&client_id)
        .send()
        .await
        .expect("delete should succeed");

    // Verify deleted
    let result = client
        .describe_user_pool_client()
        .user_pool_id(&pool_id)
        .client_id(&client_id)
        .send()
        .await;
    assert!(result.is_err());
}

// --- AdminDeleteUser ---

#[tokio::test]
async fn test_admin_delete_user() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("DelUserPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("delme")
        .send()
        .await
        .unwrap();

    client
        .admin_delete_user()
        .user_pool_id(&pool_id)
        .username("delme")
        .send()
        .await
        .expect("admin_delete_user should succeed");

    let result = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("delme")
        .send()
        .await;
    assert!(result.is_err(), "user should be gone after delete");
}

// --- AdminDeleteUserAttributes ---

#[tokio::test]
async fn test_admin_delete_user_attributes() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("AttrPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    let attr = aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
        .name("custom:field")
        .value("val")
        .build()
        .unwrap();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("attruser")
        .user_attributes(attr)
        .send()
        .await
        .unwrap();

    client
        .admin_delete_user_attributes()
        .user_pool_id(&pool_id)
        .username("attruser")
        .user_attribute_names("custom:field")
        .send()
        .await
        .expect("delete attributes should succeed");
}

// --- AdminUpdateUserAttributes ---

#[tokio::test]
async fn test_admin_update_user_attributes() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("UpdAttrPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("upduser")
        .send()
        .await
        .unwrap();

    let attr = aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
        .name("email")
        .value("new@example.com")
        .build()
        .unwrap();

    client
        .admin_update_user_attributes()
        .user_pool_id(&pool_id)
        .username("upduser")
        .user_attributes(attr)
        .send()
        .await
        .expect("admin_update_user_attributes should succeed");
}

// --- AdminEnableUser / AdminDisableUser ---

#[tokio::test]
async fn test_admin_enable_disable_user() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("EnablePool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("toggleuser")
        .send()
        .await
        .unwrap();

    client
        .admin_disable_user()
        .user_pool_id(&pool_id)
        .username("toggleuser")
        .send()
        .await
        .expect("disable should succeed");

    let user = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("toggleuser")
        .send()
        .await
        .unwrap();
    assert!(!user.enabled());

    client
        .admin_enable_user()
        .user_pool_id(&pool_id)
        .username("toggleuser")
        .send()
        .await
        .expect("enable should succeed");

    let user = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("toggleuser")
        .send()
        .await
        .unwrap();
    assert!(user.enabled());
}

// --- AdminSetUserPassword ---

#[tokio::test]
async fn test_admin_set_user_password() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("PwdPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("pwduser")
        .send()
        .await
        .unwrap();

    client
        .admin_set_user_password()
        .user_pool_id(&pool_id)
        .username("pwduser")
        .password("NewP@ss123!")
        .permanent(true)
        .send()
        .await
        .expect("set password should succeed");
}

// --- AdminConfirmSignUp ---

#[tokio::test]
async fn test_admin_confirm_sign_up() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("ConfirmPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("confirmuser")
        .send()
        .await
        .unwrap();

    client
        .admin_confirm_sign_up()
        .user_pool_id(&pool_id)
        .username("confirmuser")
        .send()
        .await
        .expect("admin_confirm_sign_up should succeed");
}

// --- AdminResetUserPassword ---

#[tokio::test]
async fn test_admin_reset_user_password() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("ResetPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("resetuser")
        .send()
        .await
        .unwrap();

    client
        .admin_reset_user_password()
        .user_pool_id(&pool_id)
        .username("resetuser")
        .send()
        .await
        .expect("admin_reset_user_password should succeed");
}

// --- AdminSetUserMFAPreference ---

#[tokio::test]
async fn test_admin_set_user_mfa_preference() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("MFAPrefPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("mfauser")
        .send()
        .await
        .unwrap();

    client
        .admin_set_user_mfa_preference()
        .user_pool_id(&pool_id)
        .username("mfauser")
        .send()
        .await
        .expect("admin_set_user_mfa_preference should succeed");
}

// --- Group lifecycle: CreateGroup / GetGroup / UpdateGroup / DeleteGroup / ListGroups ---

#[tokio::test]
async fn test_group_lifecycle() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("GroupPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    // Create
    let create_resp = client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("admins")
        .description("Admin group")
        .precedence(1)
        .send()
        .await
        .expect("create_group should succeed");
    assert_eq!(create_resp.group().unwrap().group_name(), Some("admins"));

    // Get
    let get_resp = client
        .get_group()
        .user_pool_id(&pool_id)
        .group_name("admins")
        .send()
        .await
        .expect("get_group should succeed");
    assert_eq!(get_resp.group().unwrap().description(), Some("Admin group"));

    // Update
    let upd_resp = client
        .update_group()
        .user_pool_id(&pool_id)
        .group_name("admins")
        .description("Updated admins")
        .send()
        .await
        .expect("update_group should succeed");
    assert_eq!(
        upd_resp.group().unwrap().description(),
        Some("Updated admins")
    );

    // List
    let list_resp = client
        .list_groups()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("list_groups should succeed");
    assert_eq!(list_resp.groups().len(), 1);

    // Delete
    client
        .delete_group()
        .user_pool_id(&pool_id)
        .group_name("admins")
        .send()
        .await
        .expect("delete_group should succeed");

    // Verify deleted
    let result = client
        .get_group()
        .user_pool_id(&pool_id)
        .group_name("admins")
        .send()
        .await;
    assert!(result.is_err());
}

// --- AdminAddUserToGroup / AdminRemoveUserFromGroup / AdminListGroupsForUser / ListUsersInGroup ---

#[tokio::test]
async fn test_admin_user_group_operations() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("UserGroupPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("groupuser")
        .send()
        .await
        .unwrap();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("devs")
        .send()
        .await
        .unwrap();

    // Add user to group
    client
        .admin_add_user_to_group()
        .user_pool_id(&pool_id)
        .username("groupuser")
        .group_name("devs")
        .send()
        .await
        .expect("admin_add_user_to_group should succeed");

    // List groups for user
    let groups_resp = client
        .admin_list_groups_for_user()
        .user_pool_id(&pool_id)
        .username("groupuser")
        .send()
        .await
        .expect("admin_list_groups_for_user should succeed");
    assert_eq!(groups_resp.groups().len(), 1);
    assert_eq!(groups_resp.groups()[0].group_name(), Some("devs"));

    // List users in group
    let users_resp = client
        .list_users_in_group()
        .user_pool_id(&pool_id)
        .group_name("devs")
        .send()
        .await
        .expect("list_users_in_group should succeed");
    assert_eq!(users_resp.users().len(), 1);

    // Remove user from group
    client
        .admin_remove_user_from_group()
        .user_pool_id(&pool_id)
        .username("groupuser")
        .group_name("devs")
        .send()
        .await
        .expect("admin_remove_user_from_group should succeed");

    // Verify removed
    let groups_resp = client
        .admin_list_groups_for_user()
        .user_pool_id(&pool_id)
        .username("groupuser")
        .send()
        .await
        .unwrap();
    assert_eq!(groups_resp.groups().len(), 0);
}

// --- AdminInitiateAuth / AdminRespondToAuthChallenge ---

#[tokio::test]
async fn test_admin_initiate_auth() {
    let client = make_cognito_client().await;
    let (pool_id, client_id) = setup_pool_with_user_and_client(&client).await;

    // Set a password first
    client
        .admin_set_user_password()
        .user_pool_id(&pool_id)
        .username("testuser")
        .password("Test@123!")
        .permanent(true)
        .send()
        .await
        .unwrap();

    let auth_resp = client
        .admin_initiate_auth()
        .user_pool_id(&pool_id)
        .client_id(&client_id)
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::AdminUserPasswordAuth)
        .auth_parameters("USERNAME", "testuser")
        .auth_parameters("PASSWORD", "Test@123!")
        .send()
        .await
        .expect("admin_initiate_auth should succeed");

    let auth_result = auth_resp.authentication_result().unwrap();
    assert!(auth_result.access_token().is_some());
    assert!(auth_result.id_token().is_some());
}

#[tokio::test]
async fn test_admin_respond_to_auth_challenge() {
    let client = make_cognito_client().await;
    let (pool_id, client_id) = setup_pool_with_user_and_client(&client).await;

    let resp = client
        .admin_respond_to_auth_challenge()
        .user_pool_id(&pool_id)
        .client_id(&client_id)
        .challenge_name(
            aws_sdk_cognitoidentityprovider::types::ChallengeNameType::NewPasswordRequired,
        )
        .challenge_responses("USERNAME", "testuser")
        .challenge_responses("NEW_PASSWORD", "NewP@ss!")
        .send()
        .await
        .expect("admin_respond_to_auth_challenge should succeed");

    assert!(resp.authentication_result().is_some());
}

// --- AdminUserGlobalSignOut ---

#[tokio::test]
async fn test_admin_user_global_sign_out() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("SignOutPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("signoutuser")
        .send()
        .await
        .unwrap();

    client
        .admin_user_global_sign_out()
        .user_pool_id(&pool_id)
        .username("signoutuser")
        .send()
        .await
        .expect("admin_user_global_sign_out should succeed");
}

// --- SignUp / ConfirmSignUp ---

#[tokio::test]
async fn test_sign_up_and_confirm() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("SignUpPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    let client_resp = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("signup-app")
        .send()
        .await
        .unwrap();
    let app_client_id = client_resp
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    let signup_resp = client
        .sign_up()
        .client_id(&app_client_id)
        .username("newuser")
        .password("P@ssw0rd!")
        .send()
        .await
        .expect("sign_up should succeed");
    assert!(!signup_resp.user_confirmed());

    client
        .confirm_sign_up()
        .client_id(&app_client_id)
        .username("newuser")
        .confirmation_code("123456")
        .send()
        .await
        .expect("confirm_sign_up should succeed");
}

// --- ForgotPassword / ConfirmForgotPassword ---

#[tokio::test]
async fn test_forgot_and_confirm_forgot_password() {
    let client = make_cognito_client().await;
    let (_pool_id, client_id) = setup_pool_with_user_and_client(&client).await;

    client
        .forgot_password()
        .client_id(&client_id)
        .username("testuser")
        .send()
        .await
        .expect("forgot_password should succeed");

    client
        .confirm_forgot_password()
        .client_id(&client_id)
        .username("testuser")
        .password("NewP@ss123!")
        .confirmation_code("123456")
        .send()
        .await
        .expect("confirm_forgot_password should succeed");
}

// --- InitiateAuth / RespondToAuthChallenge ---

#[tokio::test]
async fn test_initiate_auth() {
    let client = make_cognito_client().await;
    let (pool_id, client_id) = setup_pool_with_user_and_client(&client).await;

    // Set password
    client
        .admin_set_user_password()
        .user_pool_id(&pool_id)
        .username("testuser")
        .password("MyP@ss123!")
        .permanent(true)
        .send()
        .await
        .unwrap();

    let auth_resp = client
        .initiate_auth()
        .client_id(&client_id)
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::UserPasswordAuth)
        .auth_parameters("USERNAME", "testuser")
        .auth_parameters("PASSWORD", "MyP@ss123!")
        .send()
        .await
        .expect("initiate_auth should succeed");

    assert!(auth_resp.authentication_result().is_some());
}

#[tokio::test]
async fn test_respond_to_auth_challenge() {
    let client = make_cognito_client().await;
    let (_pool_id, client_id) = setup_pool_with_user_and_client(&client).await;

    let resp = client
        .respond_to_auth_challenge()
        .client_id(&client_id)
        .challenge_name(
            aws_sdk_cognitoidentityprovider::types::ChallengeNameType::NewPasswordRequired,
        )
        .challenge_responses("USERNAME", "testuser")
        .challenge_responses("NEW_PASSWORD", "NewP@ss!")
        .send()
        .await
        .expect("respond_to_auth_challenge should succeed");

    assert!(resp.authentication_result().is_some());
}

// --- GlobalSignOut ---

#[tokio::test]
async fn test_global_sign_out() {
    let client = make_cognito_client().await;

    client
        .global_sign_out()
        .access_token("fake-token")
        .send()
        .await
        .expect("global_sign_out should succeed");
}

// --- ChangePassword ---

#[tokio::test]
async fn test_change_password() {
    let client = make_cognito_client().await;

    client
        .change_password()
        .access_token("fake-token")
        .previous_password("OldP@ss!")
        .proposed_password("NewP@ss!")
        .send()
        .await
        .expect("change_password should succeed");
}

// --- GetUser ---

#[tokio::test]
async fn test_get_user() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("GetUserPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("getme")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_user()
        .access_token("fake-token")
        .send()
        .await
        .expect("get_user should succeed");
    assert!(!resp.username().is_empty());
}

// --- UpdateUserAttributes ---

#[tokio::test]
async fn test_update_user_attributes() {
    let client = make_cognito_client().await;

    let attr = aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
        .name("nickname")
        .value("tester")
        .build()
        .unwrap();

    client
        .update_user_attributes()
        .access_token("fake-token")
        .user_attributes(attr)
        .send()
        .await
        .expect("update_user_attributes should succeed");
}

// --- SetUserMFAPreference ---

#[tokio::test]
async fn test_set_user_mfa_preference() {
    let client = make_cognito_client().await;

    client
        .set_user_mfa_preference()
        .access_token("fake-token")
        .send()
        .await
        .expect("set_user_mfa_preference should succeed");
}

// --- ListUsers ---

#[tokio::test]
async fn test_list_users() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("ListUsersPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("user1")
        .send()
        .await
        .unwrap();
    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("user2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_users()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("list_users should succeed");
    assert_eq!(resp.users().len(), 2);
}

// --- Identity Provider lifecycle ---

#[tokio::test]
async fn test_identity_provider_lifecycle() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("IdPPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    // Create
    let create_resp = client
        .create_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("Google")
        .provider_type(aws_sdk_cognitoidentityprovider::types::IdentityProviderTypeType::Google)
        .provider_details("client_id", "google-client-id")
        .provider_details("client_secret", "google-secret")
        .provider_details("authorize_scopes", "openid email")
        .send()
        .await
        .expect("create_identity_provider should succeed");
    assert_eq!(
        create_resp.identity_provider().unwrap().provider_name(),
        Some("Google")
    );

    // Describe
    let desc_resp = client
        .describe_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("Google")
        .send()
        .await
        .expect("describe should succeed");
    assert!(
        desc_resp
            .identity_provider()
            .unwrap()
            .provider_type()
            .is_some()
    );

    // Update
    let upd_resp = client
        .update_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("Google")
        .provider_details("client_id", "new-google-id")
        .send()
        .await
        .expect("update should succeed");
    assert!(upd_resp.identity_provider().is_some());

    // List
    let list_resp = client
        .list_identity_providers()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.providers().len(), 1);

    // Delete
    client
        .delete_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("Google")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("Google")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Resource Server lifecycle ---

#[tokio::test]
async fn test_resource_server_lifecycle() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("RSPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    let scope = aws_sdk_cognitoidentityprovider::types::ResourceServerScopeType::builder()
        .scope_name("read")
        .scope_description("Read access")
        .build()
        .unwrap();

    // Create
    let create_resp = client
        .create_resource_server()
        .user_pool_id(&pool_id)
        .identifier("https://api.example.com")
        .name("MyAPI")
        .scopes(scope)
        .send()
        .await
        .expect("create_resource_server should succeed");
    assert_eq!(create_resp.resource_server().unwrap().name(), Some("MyAPI"));

    // Describe
    let desc_resp = client
        .describe_resource_server()
        .user_pool_id(&pool_id)
        .identifier("https://api.example.com")
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc_resp.resource_server().unwrap().scopes().len(), 1);

    // List
    let list_resp = client
        .list_resource_servers()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.resource_servers().len(), 1);
}

// --- User Pool Domain lifecycle ---

#[tokio::test]
async fn test_user_pool_domain_lifecycle() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("DomainPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    // Create domain
    client
        .create_user_pool_domain()
        .user_pool_id(&pool_id)
        .domain("my-test-domain")
        .send()
        .await
        .expect("create_user_pool_domain should succeed");

    // Describe domain
    let desc_resp = client
        .describe_user_pool_domain()
        .domain("my-test-domain")
        .send()
        .await
        .expect("describe should succeed");
    let domain_desc = desc_resp.domain_description().unwrap();
    assert_eq!(domain_desc.domain(), Some("my-test-domain"));
    assert_eq!(domain_desc.user_pool_id(), Some(pool_id.as_str()));

    // Update domain
    client
        .update_user_pool_domain()
        .user_pool_id(&pool_id)
        .domain("my-test-domain")
        .send()
        .await
        .expect("update_user_pool_domain should succeed");

    // Delete domain
    client
        .delete_user_pool_domain()
        .user_pool_id(&pool_id)
        .domain("my-test-domain")
        .send()
        .await
        .expect("delete_user_pool_domain should succeed");
}

// --- AddCustomAttributes ---

#[tokio::test]
async fn test_add_custom_attributes() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("CustomAttrPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    let attr = aws_sdk_cognitoidentityprovider::types::SchemaAttributeType::builder()
        .name("custom:department")
        .attribute_data_type(aws_sdk_cognitoidentityprovider::types::AttributeDataType::String)
        .build();

    client
        .add_custom_attributes()
        .user_pool_id(&pool_id)
        .custom_attributes(attr)
        .send()
        .await
        .expect("add_custom_attributes should succeed");
}

// --- GetUserPoolMfaConfig / SetUserPoolMfaConfig ---

#[tokio::test]
async fn test_user_pool_mfa_config() {
    let client = make_cognito_client().await;
    let pool_resp = client
        .create_user_pool()
        .pool_name("MFAPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool_resp.user_pool().unwrap().id().unwrap().to_string();

    // Set MFA config
    let sw_config = aws_sdk_cognitoidentityprovider::types::SoftwareTokenMfaConfigType::builder()
        .enabled(true)
        .build();

    client
        .set_user_pool_mfa_config()
        .user_pool_id(&pool_id)
        .mfa_configuration(aws_sdk_cognitoidentityprovider::types::UserPoolMfaType::Optional)
        .software_token_mfa_configuration(sw_config)
        .send()
        .await
        .expect("set_user_pool_mfa_config should succeed");

    // Get MFA config
    let resp = client
        .get_user_pool_mfa_config()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("get_user_pool_mfa_config should succeed");
    assert_eq!(
        resp.mfa_configuration(),
        Some(&aws_sdk_cognitoidentityprovider::types::UserPoolMfaType::Optional)
    );
}

// --- AssociateSoftwareToken ---

#[tokio::test]
async fn test_associate_software_token() {
    let client = make_cognito_client().await;

    let resp = client
        .associate_software_token()
        .access_token("fake-token")
        .send()
        .await
        .expect("associate_software_token should succeed");
    assert!(resp.secret_code().is_some());
}

// --- VerifySoftwareToken ---

#[tokio::test]
async fn test_verify_software_token() {
    let client = make_cognito_client().await;

    let resp = client
        .verify_software_token()
        .access_token("fake-token")
        .user_code("123456")
        .send()
        .await
        .expect("verify_software_token should succeed");
    assert_eq!(
        resp.status(),
        Some(&aws_sdk_cognitoidentityprovider::types::VerifySoftwareTokenResponseType::Success)
    );
}

// ============================================================================
// Ported from moto: test_cognitoidp.py
// ============================================================================

// --- Admin user ops ---

// Ported from moto: test_cognitoidp.py::test_admin_create_user
#[tokio::test]
async fn test_moto_admin_create_user() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("AdminCreatePool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let attr = aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
        .name("thing")
        .value("myvalue")
        .build()
        .unwrap();

    let result = client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("testuser1")
        .user_attributes(attr)
        .send()
        .await
        .unwrap();

    let user = result.user().unwrap();
    assert_eq!(user.username(), Some("testuser1"));
    assert_eq!(
        user.user_status().map(|s| s.as_str()),
        Some("FORCE_CHANGE_PASSWORD")
    );
    assert!(user.enabled());

    // Should have "thing" attribute and "sub" auto-generated
    let attrs = user.attributes();
    let thing_attr: Vec<_> = attrs.iter().filter(|a| a.name() == "thing").collect();
    assert_eq!(thing_attr.len(), 1);
    assert_eq!(thing_attr[0].value(), Some("myvalue"));

    let sub_attr: Vec<_> = attrs.iter().filter(|a| a.name() == "sub").collect();
    assert_eq!(sub_attr.len(), 1);
    assert!(!sub_attr[0].value().unwrap_or("").is_empty());
}

// Ported from moto: test_cognitoidp.py::test_admin_create_existing_user
#[tokio::test]
async fn test_moto_admin_create_existing_user_error() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("DupPool2")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("dupuser2")
        .send()
        .await
        .unwrap();

    let err = client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("dupuser2")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("UsernameExistsException"),
        "Expected UsernameExistsException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_admin_delete_user
#[tokio::test]
async fn test_moto_admin_delete_user() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("DelPool3")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("delme3")
        .send()
        .await
        .unwrap();

    client
        .admin_delete_user()
        .user_pool_id(&pool_id)
        .username("delme3")
        .send()
        .await
        .unwrap();

    let err = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("delme3")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("UserNotFoundException"),
        "Expected UserNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_admin_disable_user
#[tokio::test]
async fn test_moto_admin_disable_user() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("DisablePool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("disuser")
        .send()
        .await
        .unwrap();

    client
        .admin_disable_user()
        .user_pool_id(&pool_id)
        .username("disuser")
        .send()
        .await
        .unwrap();

    let user = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("disuser")
        .send()
        .await
        .unwrap();
    assert!(!user.enabled());
}

// Ported from moto: test_cognitoidp.py::test_admin_enable_user
#[tokio::test]
async fn test_moto_admin_enable_user() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("EnablePool2")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("enuser")
        .send()
        .await
        .unwrap();

    client
        .admin_disable_user()
        .user_pool_id(&pool_id)
        .username("enuser")
        .send()
        .await
        .unwrap();

    client
        .admin_enable_user()
        .user_pool_id(&pool_id)
        .username("enuser")
        .send()
        .await
        .unwrap();

    let user = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("enuser")
        .send()
        .await
        .unwrap();
    assert!(user.enabled());
}

// Ported from moto: test_cognitoidp.py::test_admin_set_user_password
#[tokio::test]
async fn test_moto_admin_set_user_password() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("SetPwdPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let attr = aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
        .name("thing")
        .value("val1")
        .build()
        .unwrap();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("pwduser2")
        .user_attributes(attr)
        .send()
        .await
        .unwrap();

    client
        .admin_set_user_password()
        .user_pool_id(&pool_id)
        .username("pwduser2")
        .password("P2$$word")
        .permanent(true)
        .send()
        .await
        .unwrap();

    let result = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("pwduser2")
        .send()
        .await
        .unwrap();

    assert_eq!(result.username(), "pwduser2");
    // After permanent password set, status should be CONFIRMED
    assert_eq!(result.user_status().map(|s| s.as_str()), Some("CONFIRMED"));

    // Should still have the "thing" attribute
    let attrs = result.user_attributes();
    let thing_attr: Vec<_> = attrs.iter().filter(|a| a.name() == "thing").collect();
    assert_eq!(thing_attr.len(), 1);
    assert_eq!(thing_attr[0].value(), Some("val1"));
}

// --- Admin confirm sign up ---

// Ported from moto: test_cognitoidp.py::test_admin_confirm_sign_up
#[tokio::test]
async fn test_moto_admin_confirm_sign_up() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("ConfirmPool2")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let client_id = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("confirmapp")
        .send()
        .await
        .unwrap()
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    // Sign up a user
    client
        .sign_up()
        .client_id(&client_id)
        .username("signupconfuser")
        .password("Passw0rd!")
        .send()
        .await
        .unwrap();

    // User should be UNCONFIRMED
    let user = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("signupconfuser")
        .send()
        .await
        .unwrap();
    assert_eq!(user.user_status().map(|s| s.as_str()), Some("UNCONFIRMED"));

    // Admin confirm
    client
        .admin_confirm_sign_up()
        .user_pool_id(&pool_id)
        .username("signupconfuser")
        .send()
        .await
        .unwrap();

    // User should now be CONFIRMED
    let user = client
        .admin_get_user()
        .user_pool_id(&pool_id)
        .username("signupconfuser")
        .send()
        .await
        .unwrap();
    assert_eq!(user.user_status().map(|s| s.as_str()), Some("CONFIRMED"));
}

// Ported from moto: test_cognitoidp.py::test_admin_confirm_sign_up_non_existing_user
#[tokio::test]
async fn test_moto_admin_confirm_sign_up_non_existing_user() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("ConfirmNEPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let err = client
        .admin_confirm_sign_up()
        .user_pool_id(&pool_id)
        .username("nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("UserNotFoundException"),
        "Expected UserNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_admin_confirm_sign_up_non_existing_pool
#[tokio::test]
async fn test_moto_admin_confirm_sign_up_non_existing_pool() {
    let client = make_cognito_client().await;

    let err = client
        .admin_confirm_sign_up()
        .user_pool_id("us-east-1_aaaaaaaa")
        .username("anyone")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// --- Groups ---

// Ported from moto: test_cognitoidp.py::test_create_group
#[tokio::test]
async fn test_moto_create_group() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("GrpPool1")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let result = client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("test-group")
        .description("A test group")
        .role_arn("arn:aws:iam:::role/my-iam-role")
        .precedence(42)
        .send()
        .await
        .unwrap();

    let group = result.group().unwrap();
    assert_eq!(group.group_name(), Some("test-group"));
    assert_eq!(group.user_pool_id(), Some(pool_id.as_str()));
    assert_eq!(group.description(), Some("A test group"));
    assert_eq!(group.role_arn(), Some("arn:aws:iam:::role/my-iam-role"));
    assert_eq!(group.precedence(), Some(42));
    assert!(group.creation_date().is_some());
    assert!(group.last_modified_date().is_some());
}

// Ported from moto: test_cognitoidp.py::test_create_group_with_duplicate_name_raises_error
#[tokio::test]
async fn test_moto_create_group_duplicate_error() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("GrpDupPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("dupgroup")
        .send()
        .await
        .unwrap();

    let err = client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("dupgroup")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("GroupExistsException"),
        "Expected GroupExistsException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_get_group
#[tokio::test]
async fn test_moto_get_group() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("GetGrpPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("getgrp")
        .send()
        .await
        .unwrap();

    let result = client
        .get_group()
        .user_pool_id(&pool_id)
        .group_name("getgrp")
        .send()
        .await
        .unwrap();

    let group = result.group().unwrap();
    assert_eq!(group.group_name(), Some("getgrp"));
    assert_eq!(group.user_pool_id(), Some(pool_id.as_str()));
    assert!(group.creation_date().is_some());
    assert!(group.last_modified_date().is_some());
}

// Ported from moto: test_cognitoidp.py::test_update_group
#[tokio::test]
async fn test_moto_update_group() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("UpdGrpPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("updgrp")
        .description("old desc")
        .role_arn("arn:aws:iam:::role/old-role")
        .precedence(10)
        .send()
        .await
        .unwrap();

    let result = client
        .update_group()
        .user_pool_id(&pool_id)
        .group_name("updgrp")
        .description("new desc")
        .role_arn("arn:aws:iam:::role/new-role")
        .precedence(20)
        .send()
        .await
        .unwrap();

    let group = result.group().unwrap();
    assert_eq!(group.group_name(), Some("updgrp"));
    assert_eq!(group.user_pool_id(), Some(pool_id.as_str()));
    assert_eq!(group.description(), Some("new desc"));
    assert_eq!(group.role_arn(), Some("arn:aws:iam:::role/new-role"));
    assert_eq!(group.precedence(), Some(20));
}

// Ported from moto: test_cognitoidp.py::test_delete_group
#[tokio::test]
async fn test_moto_delete_group() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("DelGrpPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("delgrp")
        .send()
        .await
        .unwrap();

    client
        .delete_group()
        .user_pool_id(&pool_id)
        .group_name("delgrp")
        .send()
        .await
        .unwrap();

    let err = client
        .get_group()
        .user_pool_id(&pool_id)
        .group_name("delgrp")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_admin_add_user_to_group_again_is_noop
#[tokio::test]
async fn test_moto_admin_add_user_to_group_noop() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("AddGrpNoopPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("noopgrp")
        .send()
        .await
        .unwrap();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("noopuser")
        .send()
        .await
        .unwrap();

    // Add twice - should not fail
    client
        .admin_add_user_to_group()
        .user_pool_id(&pool_id)
        .username("noopuser")
        .group_name("noopgrp")
        .send()
        .await
        .unwrap();

    client
        .admin_add_user_to_group()
        .user_pool_id(&pool_id)
        .username("noopuser")
        .group_name("noopgrp")
        .send()
        .await
        .unwrap();

    // Should only appear once in group
    let resp = client
        .list_users_in_group()
        .user_pool_id(&pool_id)
        .group_name("noopgrp")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.users().len(), 1);
}

// Ported from moto: test_cognitoidp.py::test_list_users_in_group_ignores_deleted_user
#[tokio::test]
async fn test_moto_list_users_in_group_ignores_deleted_user() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("DelUsrGrpPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("grp1")
        .send()
        .await
        .unwrap();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("user_a")
        .send()
        .await
        .unwrap();
    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("user_b")
        .send()
        .await
        .unwrap();

    client
        .admin_add_user_to_group()
        .user_pool_id(&pool_id)
        .username("user_a")
        .group_name("grp1")
        .send()
        .await
        .unwrap();
    client
        .admin_add_user_to_group()
        .user_pool_id(&pool_id)
        .username("user_b")
        .group_name("grp1")
        .send()
        .await
        .unwrap();

    // Delete user_a
    client
        .admin_delete_user()
        .user_pool_id(&pool_id)
        .username("user_a")
        .send()
        .await
        .unwrap();

    let result = client
        .list_users_in_group()
        .user_pool_id(&pool_id)
        .group_name("grp1")
        .send()
        .await
        .unwrap();

    assert_eq!(result.users().len(), 1);
    assert_eq!(result.users()[0].username(), Some("user_b"));
}

// Ported from moto: test_cognitoidp.py::test_admin_list_groups_for_user_ignores_deleted_group
#[tokio::test]
async fn test_moto_admin_list_groups_for_user_ignores_deleted_group() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("DelGrpUsrPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("grp_a")
        .send()
        .await
        .unwrap();
    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("grp_b")
        .send()
        .await
        .unwrap();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("grpuser")
        .send()
        .await
        .unwrap();

    client
        .admin_add_user_to_group()
        .user_pool_id(&pool_id)
        .username("grpuser")
        .group_name("grp_a")
        .send()
        .await
        .unwrap();
    client
        .admin_add_user_to_group()
        .user_pool_id(&pool_id)
        .username("grpuser")
        .group_name("grp_b")
        .send()
        .await
        .unwrap();

    // Delete grp_a
    client
        .delete_group()
        .user_pool_id(&pool_id)
        .group_name("grp_a")
        .send()
        .await
        .unwrap();

    let result = client
        .admin_list_groups_for_user()
        .user_pool_id(&pool_id)
        .username("grpuser")
        .send()
        .await
        .unwrap();

    assert_eq!(result.groups().len(), 1);
    assert_eq!(result.groups()[0].group_name(), Some("grp_b"));
}

// Ported from moto: test_cognitoidp.py::test_admin_remove_user_from_group
#[tokio::test]
async fn test_moto_admin_remove_user_from_group() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("RemGrpPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("remgrp")
        .send()
        .await
        .unwrap();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("remuser")
        .send()
        .await
        .unwrap();

    client
        .admin_add_user_to_group()
        .user_pool_id(&pool_id)
        .username("remuser")
        .group_name("remgrp")
        .send()
        .await
        .unwrap();

    client
        .admin_remove_user_from_group()
        .user_pool_id(&pool_id)
        .username("remuser")
        .group_name("remgrp")
        .send()
        .await
        .unwrap();

    let users = client
        .list_users_in_group()
        .user_pool_id(&pool_id)
        .group_name("remgrp")
        .send()
        .await
        .unwrap();
    assert_eq!(users.users().len(), 0);

    let groups = client
        .admin_list_groups_for_user()
        .user_pool_id(&pool_id)
        .username("remuser")
        .send()
        .await
        .unwrap();
    assert_eq!(groups.groups().len(), 0);
}

// Ported from moto: test_cognitoidp.py::test_list_users_in_group
#[tokio::test]
async fn test_moto_list_users_in_group() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("ListUigPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_group()
        .user_pool_id(&pool_id)
        .group_name("listgrp")
        .send()
        .await
        .unwrap();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("listuser")
        .send()
        .await
        .unwrap();

    client
        .admin_add_user_to_group()
        .user_pool_id(&pool_id)
        .username("listuser")
        .group_name("listgrp")
        .send()
        .await
        .unwrap();

    let result = client
        .list_users_in_group()
        .user_pool_id(&pool_id)
        .group_name("listgrp")
        .send()
        .await
        .unwrap();

    assert_eq!(result.users().len(), 1);
    assert_eq!(result.users()[0].username(), Some("listuser"));
}

// --- Identity Providers ---

// Ported from moto: test_cognitoidp.py::test_create_identity_provider
#[tokio::test]
async fn test_moto_create_identity_provider() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("IdPCreatePool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let result = client
        .create_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("Facebook")
        .provider_type(aws_sdk_cognitoidentityprovider::types::IdentityProviderTypeType::Facebook)
        .provider_details("thing", "myvalue")
        .send()
        .await
        .unwrap();

    let idp = result.identity_provider().unwrap();
    assert_eq!(idp.user_pool_id(), Some(pool_id.as_str()));
    assert_eq!(idp.provider_name(), Some("Facebook"));
    assert_eq!(idp.provider_type().map(|t| t.as_str()), Some("Facebook"));
    assert_eq!(
        idp.provider_details()
            .unwrap()
            .get("thing")
            .map(|s| s.as_str()),
        Some("myvalue")
    );
}

// Ported from moto: test_cognitoidp.py::test_describe_identity_providers
#[tokio::test]
async fn test_moto_describe_identity_provider() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("IdPDescPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("FB2")
        .provider_type(aws_sdk_cognitoidentityprovider::types::IdentityProviderTypeType::Facebook)
        .provider_details("thing", "val2")
        .send()
        .await
        .unwrap();

    let result = client
        .describe_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("FB2")
        .send()
        .await
        .unwrap();

    let idp = result.identity_provider().unwrap();
    assert_eq!(idp.user_pool_id(), Some(pool_id.as_str()));
    assert_eq!(idp.provider_name(), Some("FB2"));
    assert_eq!(idp.provider_type().map(|t| t.as_str()), Some("Facebook"));
    assert_eq!(
        idp.provider_details()
            .unwrap()
            .get("thing")
            .map(|s| s.as_str()),
        Some("val2")
    );
}

// Ported from moto: test_cognitoidp.py::test_update_identity_provider
#[tokio::test]
async fn test_moto_update_identity_provider() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("IdPUpdPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("FB3")
        .provider_type(aws_sdk_cognitoidentityprovider::types::IdentityProviderTypeType::Facebook)
        .provider_details("thing", "old_val")
        .send()
        .await
        .unwrap();

    let result = client
        .update_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("FB3")
        .provider_details("thing", "new_val")
        .attribute_mapping("email", "email")
        .attribute_mapping("username", "sub")
        .send()
        .await
        .unwrap();

    let idp = result.identity_provider().unwrap();
    assert_eq!(idp.user_pool_id(), Some(pool_id.as_str()));
    assert_eq!(idp.provider_name(), Some("FB3"));
    assert_eq!(idp.provider_type().map(|t| t.as_str()), Some("Facebook"));
    assert_eq!(
        idp.provider_details()
            .unwrap()
            .get("thing")
            .map(|s| s.as_str()),
        Some("new_val")
    );
    let mapping = idp.attribute_mapping().unwrap();
    assert_eq!(mapping.get("email").map(|s| s.as_str()), Some("email"));
    assert_eq!(mapping.get("username").map(|s| s.as_str()), Some("sub"));
}

// Ported from moto: test_cognitoidp.py::test_update_identity_provider_no_user_pool
#[tokio::test]
async fn test_moto_update_identity_provider_no_pool() {
    let client = make_cognito_client().await;

    let err = client
        .update_identity_provider()
        .user_pool_id("foo")
        .provider_name("bar")
        .provider_details("thing", "val")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_update_identity_provider_no_identity_provider
#[tokio::test]
async fn test_moto_update_identity_provider_no_idp() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("IdPNoIdpPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let err = client
        .update_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("nonexistent")
        .provider_details("thing", "val")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_delete_identity_providers
#[tokio::test]
async fn test_moto_delete_identity_provider() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("IdPDelPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("FB4")
        .provider_type(aws_sdk_cognitoidentityprovider::types::IdentityProviderTypeType::Facebook)
        .provider_details("thing", "val")
        .send()
        .await
        .unwrap();

    client
        .delete_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("FB4")
        .send()
        .await
        .unwrap();

    let err = client
        .describe_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("FB4")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_list_identity_providers
#[tokio::test]
async fn test_moto_list_identity_providers() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("IdPListPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_identity_provider()
        .user_pool_id(&pool_id)
        .provider_name("FB5")
        .provider_type(aws_sdk_cognitoidentityprovider::types::IdentityProviderTypeType::Facebook)
        .provider_details("k", "v")
        .send()
        .await
        .unwrap();

    let result = client
        .list_identity_providers()
        .user_pool_id(&pool_id)
        .max_results(10)
        .send()
        .await
        .unwrap();

    assert_eq!(result.providers().len(), 1);
    assert_eq!(result.providers()[0].provider_name(), Some("FB5"));
    assert_eq!(
        result.providers()[0].provider_type().map(|t| t.as_str()),
        Some("Facebook")
    );
}

// --- Resource Servers ---

// Ported from moto: test_cognitoidp.py::test_create_resource_server (duplicate error)
#[tokio::test]
async fn test_moto_create_resource_server_duplicate_error() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("RSDupPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let scope = aws_sdk_cognitoidentityprovider::types::ResourceServerScopeType::builder()
        .scope_name("app:write")
        .scope_description("write scope")
        .build()
        .unwrap();

    client
        .create_resource_server()
        .user_pool_id(&pool_id)
        .identifier("http://localhost.localdomain")
        .name("local server")
        .scopes(scope.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_resource_server()
        .user_pool_id(&pool_id)
        .identifier("http://localhost.localdomain")
        .name("local server")
        .scopes(scope)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidParameterException"),
        "Expected InvalidParameterException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_create_resource_server_with_no_scopes
#[tokio::test]
async fn test_moto_create_resource_server_no_scopes() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("RSNoScopePool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let result = client
        .create_resource_server()
        .user_pool_id(&pool_id)
        .identifier("http://localhost.localdomain")
        .name("local server")
        .send()
        .await
        .unwrap();

    let rs = result.resource_server().unwrap();
    assert_eq!(rs.user_pool_id(), Some(pool_id.as_str()));
    assert_eq!(rs.identifier(), Some("http://localhost.localdomain"));
    assert_eq!(rs.name(), Some("local server"));
}

// Ported from moto: test_cognitoidp.py::test_describe_resource_server
#[tokio::test]
async fn test_moto_describe_resource_server() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("RSDescPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let scope1 = aws_sdk_cognitoidentityprovider::types::ResourceServerScopeType::builder()
        .scope_name("app:write")
        .scope_description("write scope")
        .build()
        .unwrap();
    let scope2 = aws_sdk_cognitoidentityprovider::types::ResourceServerScopeType::builder()
        .scope_name("app:read")
        .scope_description("read scope")
        .build()
        .unwrap();

    client
        .create_resource_server()
        .user_pool_id(&pool_id)
        .identifier("my_server")
        .name("new_remote_server")
        .scopes(scope1)
        .scopes(scope2)
        .send()
        .await
        .unwrap();

    let result = client
        .describe_resource_server()
        .user_pool_id(&pool_id)
        .identifier("my_server")
        .send()
        .await
        .unwrap();

    let rs = result.resource_server().unwrap();
    assert_eq!(rs.user_pool_id(), Some(pool_id.as_str()));
    assert_eq!(rs.identifier(), Some("my_server"));
    assert_eq!(rs.name(), Some("new_remote_server"));
    assert_eq!(rs.scopes().len(), 2);

    // Non-existent server should fail
    let err = client
        .describe_resource_server()
        .user_pool_id(&pool_id)
        .identifier("non_existent_server")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_list_resource_servers_empty_set
#[tokio::test]
async fn test_moto_list_resource_servers_empty() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("RSEmptyPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let result = client
        .list_resource_servers()
        .user_pool_id(&pool_id)
        .max_results(50)
        .send()
        .await
        .unwrap();

    assert_eq!(result.resource_servers().len(), 0);
}

// --- User Pool Domains ---

// Ported from moto: test_cognitoidp.py::test_describe_user_pool_domain
#[tokio::test]
async fn test_moto_describe_user_pool_domain() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("DomDescPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_user_pool_domain()
        .user_pool_id(&pool_id)
        .domain("my-domain-1")
        .send()
        .await
        .unwrap();

    let result = client
        .describe_user_pool_domain()
        .domain("my-domain-1")
        .send()
        .await
        .unwrap();

    let desc = result.domain_description().unwrap();
    assert_eq!(desc.domain(), Some("my-domain-1"));
    assert_eq!(desc.user_pool_id(), Some(pool_id.as_str()));
}

// Ported from moto: test_cognitoidp.py::test_delete_user_pool_domain (describe after delete returns empty)
#[tokio::test]
async fn test_moto_delete_user_pool_domain_then_describe_empty() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("DomDelPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .create_user_pool_domain()
        .user_pool_id(&pool_id)
        .domain("my-domain-2")
        .send()
        .await
        .unwrap();

    client
        .delete_user_pool_domain()
        .user_pool_id(&pool_id)
        .domain("my-domain-2")
        .send()
        .await
        .unwrap();

    // Describing a missing domain should return 200 with empty DomainDescription
    let result = client
        .describe_user_pool_domain()
        .domain("my-domain-2")
        .send()
        .await
        .unwrap();

    // The domain description should essentially be empty (no domain field)
    let desc = result.domain_description();
    // It should either be None or have no domain
    if let Some(d) = desc {
        assert!(d.domain().is_none() || d.domain() == Some(""));
    }
}

// --- Auth flows ---

// Ported from moto: test_cognitoidp.py::test_login_denied_if_account_disabled
#[tokio::test]
async fn test_moto_login_denied_if_account_disabled() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("DisabledAuthPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let app_client_id = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("auth-app")
        .send()
        .await
        .unwrap()
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    // Create user and set password
    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("disauthuser")
        .send()
        .await
        .unwrap();

    client
        .admin_set_user_password()
        .user_pool_id(&pool_id)
        .username("disauthuser")
        .password("P2$Sword")
        .permanent(true)
        .send()
        .await
        .unwrap();

    // Disable the user
    client
        .admin_disable_user()
        .user_pool_id(&pool_id)
        .username("disauthuser")
        .send()
        .await
        .unwrap();

    // InitiateAuth should fail with NotAuthorizedException
    let err = client
        .initiate_auth()
        .client_id(&app_client_id)
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::UserPasswordAuth)
        .auth_parameters("USERNAME", "disauthuser")
        .auth_parameters("PASSWORD", "P2$Sword")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotAuthorizedException"),
        "Expected NotAuthorizedException, got: {err_str}"
    );

    // AdminInitiateAuth should also fail with NotAuthorizedException
    let err = client
        .admin_initiate_auth()
        .user_pool_id(&pool_id)
        .client_id(&app_client_id)
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::AdminNoSrpAuth)
        .auth_parameters("USERNAME", "disauthuser")
        .auth_parameters("PASSWORD", "P2$Sword")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotAuthorizedException"),
        "Expected NotAuthorizedException for admin auth, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_initiate_auth_USER_PASSWORD_AUTH_user_not_found
#[tokio::test]
async fn test_moto_initiate_auth_user_not_found() {
    let client = make_cognito_client().await;
    let (pool_id, app_client_id) = setup_pool_with_user_and_client(&client).await;

    // Set password for the existing user
    client
        .admin_set_user_password()
        .user_pool_id(&pool_id)
        .username("testuser")
        .password("P2$Sword")
        .permanent(true)
        .send()
        .await
        .unwrap();

    let err = client
        .initiate_auth()
        .client_id(&app_client_id)
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::UserPasswordAuth)
        .auth_parameters("USERNAME", "INVALIDUSER")
        .auth_parameters("PASSWORD", "P2$Sword")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("UserNotFoundException"),
        "Expected UserNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_initiate_auth_USER_PASSWORD_AUTH_user_incorrect_password
#[tokio::test]
async fn test_moto_initiate_auth_incorrect_password() {
    let client = make_cognito_client().await;
    let (pool_id, app_client_id) = setup_pool_with_user_and_client(&client).await;

    client
        .admin_set_user_password()
        .user_pool_id(&pool_id)
        .username("testuser")
        .password("P2$Sword")
        .permanent(true)
        .send()
        .await
        .unwrap();

    let err = client
        .initiate_auth()
        .client_id(&app_client_id)
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::UserPasswordAuth)
        .auth_parameters("USERNAME", "testuser")
        .auth_parameters("PASSWORD", "WrongPassword")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotAuthorizedException"),
        "Expected NotAuthorizedException, got: {err_str}"
    );
}

// Ported from moto: test_cognitoidp.py::test_admin_initiate_auth (ADMIN_NO_SRP_AUTH and ADMIN_USER_PASSWORD_AUTH)
#[tokio::test]
async fn test_moto_admin_initiate_auth_flows() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("AuthFlowPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let app_client_id = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("auth-client")
        .send()
        .await
        .unwrap()
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("authuser")
        .send()
        .await
        .unwrap();

    client
        .admin_set_user_password()
        .user_pool_id(&pool_id)
        .username("authuser")
        .password("Test@123!")
        .permanent(true)
        .send()
        .await
        .unwrap();

    // ADMIN_NO_SRP_AUTH
    let resp = client
        .admin_initiate_auth()
        .user_pool_id(&pool_id)
        .client_id(&app_client_id)
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::AdminNoSrpAuth)
        .auth_parameters("USERNAME", "authuser")
        .auth_parameters("PASSWORD", "Test@123!")
        .send()
        .await
        .unwrap();

    let auth_result = resp.authentication_result().unwrap();
    assert!(auth_result.access_token().is_some());
    assert!(auth_result.id_token().is_some());
    assert!(auth_result.refresh_token().is_some());
    assert_eq!(auth_result.token_type(), Some("Bearer"));

    // ADMIN_USER_PASSWORD_AUTH
    let resp = client
        .admin_initiate_auth()
        .user_pool_id(&pool_id)
        .client_id(&app_client_id)
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::AdminUserPasswordAuth)
        .auth_parameters("USERNAME", "authuser")
        .auth_parameters("PASSWORD", "Test@123!")
        .send()
        .await
        .unwrap();

    let auth_result = resp.authentication_result().unwrap();
    assert!(auth_result.access_token().is_some());
    assert!(auth_result.id_token().is_some());
}

// --- MFA Config ---

// Ported from moto: test_cognitoidp.py::test_set_user_pool_mfa_config (enable software token, then disable)
#[tokio::test]
async fn test_moto_set_user_pool_mfa_config() {
    let client = make_cognito_client().await;

    let pool_id = client
        .create_user_pool()
        .pool_name("MFACfgPool")
        .send()
        .await
        .unwrap()
        .user_pool()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Enable software token MFA
    let sw_config = aws_sdk_cognitoidentityprovider::types::SoftwareTokenMfaConfigType::builder()
        .enabled(true)
        .build();

    let mfa_result = client
        .set_user_pool_mfa_config()
        .user_pool_id(&pool_id)
        .mfa_configuration(aws_sdk_cognitoidentityprovider::types::UserPoolMfaType::On)
        .software_token_mfa_configuration(sw_config)
        .send()
        .await
        .unwrap();

    assert_eq!(
        mfa_result.mfa_configuration(),
        Some(&aws_sdk_cognitoidentityprovider::types::UserPoolMfaType::On)
    );
    assert!(mfa_result.software_token_mfa_configuration().is_some());
    assert!(
        mfa_result
            .software_token_mfa_configuration()
            .unwrap()
            .enabled()
    );

    // Get MFA config should match
    let get_result = client
        .get_user_pool_mfa_config()
        .user_pool_id(&pool_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_result.mfa_configuration(),
        Some(&aws_sdk_cognitoidentityprovider::types::UserPoolMfaType::On)
    );

    // Disable MFA
    let mfa_result = client
        .set_user_pool_mfa_config()
        .user_pool_id(&pool_id)
        .mfa_configuration(aws_sdk_cognitoidentityprovider::types::UserPoolMfaType::Off)
        .send()
        .await
        .unwrap();

    assert_eq!(
        mfa_result.mfa_configuration(),
        Some(&aws_sdk_cognitoidentityprovider::types::UserPoolMfaType::Off)
    );

    // Get should reflect OFF
    let get_result = client
        .get_user_pool_mfa_config()
        .user_pool_id(&pool_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_result.mfa_configuration(),
        Some(&aws_sdk_cognitoidentityprovider::types::UserPoolMfaType::Off)
    );
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers TagResource/UntagResource/ListTagsForResource for user pools.
#[tokio::test]
async fn test_tag_and_untag_user_pool() {
    let client = make_cognito_client().await;

    let resp = client
        .create_user_pool()
        .pool_name("TagPool")
        .send()
        .await
        .expect("create_user_pool should succeed");

    let pool_arn = resp.user_pool().unwrap().arn().unwrap().to_string();

    // TagResource should succeed
    client
        .tag_resource()
        .resource_arn(&pool_arn)
        .tags("env", "test")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    // ListTagsForResource should return the tags we just set
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&pool_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "test");
    assert_eq!(tags.get("team").unwrap(), "backend");

    // UntagResource should remove the specified tag
    client
        .untag_resource()
        .resource_arn(&pool_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify the tag was removed
    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&pool_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed after untag");

    let tags2 = tags_resp2.tags().unwrap();
    assert!(tags2.get("env").is_none(), "env tag should be removed");
    assert_eq!(
        tags2.get("team").unwrap(),
        "backend",
        "team tag should remain"
    );
}

// Covers FIX(terraform-e2e): DescribeUserPool returns default nested objects
// (handlers.rs:1763) — Default nested objects (AdminCreateUserConfig, etc.) in
// DescribeUserPool to prevent nil pointer dereference.
#[tokio::test]
async fn test_describe_user_pool_returns_default_nested_objects() {
    let client = make_cognito_client().await;

    let resp = client
        .create_user_pool()
        .pool_name("NestedObjPool")
        .send()
        .await
        .expect("create_user_pool should succeed");

    let pool_id = resp.user_pool().unwrap().id().unwrap().to_string();

    let desc_resp = client
        .describe_user_pool()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("describe_user_pool should succeed");

    let pool = desc_resp.user_pool().expect("should have user pool");

    // Policies should be present with a password policy
    let policies = pool.policies().expect("policies should be present");
    let pw_policy = policies
        .password_policy()
        .expect("password_policy should be present");
    assert!(
        pw_policy.minimum_length() > Some(0),
        "minimum_length should be set"
    );

    // AdminCreateUserConfig should be present
    let admin_config = pool
        .admin_create_user_config()
        .expect("admin_create_user_config should be present");
    // allow_admin_create_user_only should be set (false by default)
    let _ = admin_config.allow_admin_create_user_only();

    // AccountRecoverySetting should be present
    let recovery = pool
        .account_recovery_setting()
        .expect("account_recovery_setting should be present");
    assert!(
        !recovery.recovery_mechanisms().is_empty(),
        "recovery_mechanisms should not be empty"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// New coverage tests for untested operations
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_delete_user() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("DelUserPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    let pool_client = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("DelUserClient")
        .explicit_auth_flows(
            aws_sdk_cognitoidentityprovider::types::ExplicitAuthFlowsType::AllowUserPasswordAuth,
        )
        .send()
        .await
        .unwrap();
    let client_id = pool_client
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    client
        .sign_up()
        .client_id(&client_id)
        .username("deluser")
        .password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .admin_confirm_sign_up()
        .user_pool_id(&pool_id)
        .username("deluser")
        .send()
        .await
        .unwrap();

    let auth = client
        .initiate_auth()
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::UserPasswordAuth)
        .client_id(&client_id)
        .auth_parameters("USERNAME", "deluser")
        .auth_parameters("PASSWORD", "Password1!")
        .send()
        .await
        .unwrap();

    let access_token = auth
        .authentication_result()
        .unwrap()
        .access_token()
        .unwrap();

    client
        .delete_user()
        .access_token(access_token)
        .send()
        .await
        .expect("delete_user should succeed");
}

#[tokio::test]
async fn test_get_user_auth_factors() {
    let client = make_cognito_client().await;

    let resp = client
        .get_user_auth_factors()
        .access_token("fake-token")
        .send()
        .await
        .expect("get_user_auth_factors should succeed");

    // Should return an empty list of configured auth factors
    assert!(resp.configured_user_auth_factors().is_empty());
}

#[tokio::test]
async fn test_delete_user_attributes_token_based() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("DelAttrsPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    let pool_client = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("DelAttrsClient")
        .explicit_auth_flows(
            aws_sdk_cognitoidentityprovider::types::ExplicitAuthFlowsType::AllowUserPasswordAuth,
        )
        .send()
        .await
        .unwrap();
    let client_id = pool_client
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    client
        .sign_up()
        .client_id(&client_id)
        .username("attruser")
        .password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .admin_confirm_sign_up()
        .user_pool_id(&pool_id)
        .username("attruser")
        .send()
        .await
        .unwrap();

    let auth = client
        .initiate_auth()
        .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::UserPasswordAuth)
        .client_id(&client_id)
        .auth_parameters("USERNAME", "attruser")
        .auth_parameters("PASSWORD", "Password1!")
        .send()
        .await
        .unwrap();

    let access_token = auth
        .authentication_result()
        .unwrap()
        .access_token()
        .unwrap();

    client
        .delete_user_attributes()
        .access_token(access_token)
        .user_attribute_names("custom:test")
        .send()
        .await
        .expect("delete_user_attributes should succeed");
}

#[tokio::test]
async fn test_get_user_attribute_verification_code() {
    let client = make_cognito_client().await;

    let resp = client
        .get_user_attribute_verification_code()
        .access_token("fake-token")
        .attribute_name("email")
        .send()
        .await
        .expect("get_user_attribute_verification_code should succeed");

    let details = resp
        .code_delivery_details()
        .expect("should have code delivery details");
    assert_eq!(details.attribute_name(), Some("email"));
}

#[tokio::test]
async fn test_verify_user_attribute() {
    let client = make_cognito_client().await;

    client
        .verify_user_attribute()
        .access_token("fake-token")
        .attribute_name("email")
        .code("123456")
        .send()
        .await
        .expect("verify_user_attribute should succeed");
}

#[tokio::test]
async fn test_revoke_token() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("RevokePool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    let pool_client = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("RevokeClient")
        .send()
        .await
        .unwrap();
    let client_id = pool_client
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    client
        .revoke_token()
        .token("some-refresh-token")
        .client_id(&client_id)
        .send()
        .await
        .expect("revoke_token should succeed");
}

#[tokio::test]
async fn test_describe_and_set_risk_configuration() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("RiskPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    // Describe risk configuration (should return defaults)
    let desc_resp = client
        .describe_risk_configuration()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("describe_risk_configuration should succeed");
    let risk_config = desc_resp
        .risk_configuration()
        .expect("should have risk configuration");
    assert_eq!(risk_config.user_pool_id(), Some(pool_id.as_str()));

    // Set risk configuration
    let set_resp = client
        .set_risk_configuration()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("set_risk_configuration should succeed");
    let risk_config = set_resp
        .risk_configuration()
        .expect("should have risk configuration");
    assert_eq!(risk_config.user_pool_id(), Some(pool_id.as_str()));
}

#[tokio::test]
async fn test_set_and_get_ui_customization() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("UIPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    // Set UI customization
    let set_resp = client
        .set_ui_customization()
        .user_pool_id(&pool_id)
        .css("body { background: red; }")
        .send()
        .await
        .expect("set_ui_customization should succeed");
    let ui = set_resp
        .ui_customization()
        .expect("should have ui customization");
    assert_eq!(ui.user_pool_id(), Some(pool_id.as_str()));
    assert_eq!(ui.css(), Some("body { background: red; }"));

    // Get UI customization
    let get_resp = client
        .get_ui_customization()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("get_ui_customization should succeed");
    let ui = get_resp.ui_customization();
    assert!(ui.is_some());
}

#[tokio::test]
async fn test_get_and_set_log_delivery_configuration() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("LogPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    // Get log delivery configuration (should return empty)
    let get_resp = client
        .get_log_delivery_configuration()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("get_log_delivery_configuration should succeed");
    let log_config = get_resp
        .log_delivery_configuration()
        .expect("should have log delivery configuration");
    assert!(
        log_config.user_pool_id().contains(&pool_id),
        "user_pool_id should match"
    );

    // Set log delivery configuration
    let set_resp = client
        .set_log_delivery_configuration()
        .user_pool_id(&pool_id)
        .log_configurations(
            aws_sdk_cognitoidentityprovider::types::LogConfigurationType::builder()
                .log_level(aws_sdk_cognitoidentityprovider::types::LogLevel::Error)
                .event_source(
                    aws_sdk_cognitoidentityprovider::types::EventSourceName::UserAuthEvents,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("set_log_delivery_configuration should succeed");
    let log_config = set_resp
        .log_delivery_configuration()
        .expect("should have log delivery configuration");
    assert!(
        log_config.user_pool_id().contains(&pool_id),
        "user_pool_id should match"
    );
}

#[tokio::test]
async fn test_delete_resource_server() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("DelRSPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    client
        .create_resource_server()
        .user_pool_id(&pool_id)
        .identifier("my-api")
        .name("My API")
        .send()
        .await
        .unwrap();

    client
        .delete_resource_server()
        .user_pool_id(&pool_id)
        .identifier("my-api")
        .send()
        .await
        .expect("delete_resource_server should succeed");

    // Verify it's gone
    let result = client
        .describe_resource_server()
        .user_pool_id(&pool_id)
        .identifier("my-api")
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_update_resource_server() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("UpdRSPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    client
        .create_resource_server()
        .user_pool_id(&pool_id)
        .identifier("my-api")
        .name("My API")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_resource_server()
        .user_pool_id(&pool_id)
        .identifier("my-api")
        .name("My Updated API")
        .send()
        .await
        .expect("update_resource_server should succeed");

    let rs = resp.resource_server().expect("should have resource server");
    assert_eq!(rs.name(), Some("My Updated API"));
}

#[tokio::test]
async fn test_user_import_job_lifecycle() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("ImportPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    // Create user import job
    let create_resp = client
        .create_user_import_job()
        .user_pool_id(&pool_id)
        .job_name("TestJob")
        .cloud_watch_logs_role_arn("arn:aws:iam::123456789012:role/test")
        .send()
        .await
        .expect("create_user_import_job should succeed");

    let job = create_resp
        .user_import_job()
        .expect("should have import job");
    assert_eq!(job.job_name(), Some("TestJob"));
    let job_id = job.job_id().unwrap().to_string();

    // Describe user import job
    let desc_resp = client
        .describe_user_import_job()
        .user_pool_id(&pool_id)
        .job_id(&job_id)
        .send()
        .await
        .expect("describe_user_import_job should succeed");
    let desc_job = desc_resp.user_import_job().expect("should have import job");
    assert_eq!(desc_job.job_name(), Some("TestJob"));

    // List user import jobs
    let list_resp = client
        .list_user_import_jobs()
        .user_pool_id(&pool_id)
        .max_results(10)
        .send()
        .await
        .expect("list_user_import_jobs should succeed");
    assert!(
        !list_resp.user_import_jobs().is_empty(),
        "should have at least one import job"
    );

    // Start the import job
    let start_resp = client
        .start_user_import_job()
        .user_pool_id(&pool_id)
        .job_id(&job_id)
        .send()
        .await
        .expect("start_user_import_job should succeed");
    assert!(start_resp.user_import_job().is_some());

    // Stop the import job
    let stop_resp = client
        .stop_user_import_job()
        .user_pool_id(&pool_id)
        .job_id(&job_id)
        .send()
        .await
        .expect("stop_user_import_job should succeed");
    assert!(stop_resp.user_import_job().is_some());
}

#[tokio::test]
async fn test_get_csv_header() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("CSVPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    let resp = client
        .get_csv_header()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("get_csv_header should succeed");

    assert_eq!(resp.user_pool_id(), Some(pool_id.as_str()));
    let headers = resp.csv_header();
    assert!(!headers.is_empty(), "CSV header should not be empty");
    assert!(
        headers.contains(&"cognito:username".to_string()),
        "CSV header should contain cognito:username"
    );
}

#[tokio::test]
async fn test_admin_list_user_auth_events() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("AuthEventsPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("eventuser")
        .send()
        .await
        .unwrap();

    let resp = client
        .admin_list_user_auth_events()
        .user_pool_id(&pool_id)
        .username("eventuser")
        .max_results(10)
        .send()
        .await
        .expect("admin_list_user_auth_events should succeed");

    // Should return an empty list (no auth events recorded in mock)
    assert!(resp.auth_events().is_empty());
}

#[tokio::test]
async fn test_get_signing_certificate() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("CertPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    let resp = client
        .get_signing_certificate()
        .user_pool_id(&pool_id)
        .send()
        .await
        .expect("get_signing_certificate should succeed");

    let cert = resp.certificate().expect("should have certificate");
    assert!(
        cert.contains("BEGIN CERTIFICATE"),
        "certificate should contain PEM header"
    );
}

#[tokio::test]
async fn test_resend_confirmation_code() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("ResendPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    let pool_client = client
        .create_user_pool_client()
        .user_pool_id(&pool_id)
        .client_name("ResendClient")
        .explicit_auth_flows(
            aws_sdk_cognitoidentityprovider::types::ExplicitAuthFlowsType::AllowUserPasswordAuth,
        )
        .send()
        .await
        .unwrap();
    let client_id = pool_client
        .user_pool_client()
        .unwrap()
        .client_id()
        .unwrap()
        .to_string();

    client
        .sign_up()
        .client_id(&client_id)
        .username("resenduser")
        .password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .resend_confirmation_code()
        .client_id(&client_id)
        .username("resenduser")
        .send()
        .await
        .expect("resend_confirmation_code should succeed");

    let details = resp
        .code_delivery_details()
        .expect("should have code delivery details");
    assert_eq!(details.attribute_name(), Some("email"));
}

#[tokio::test]
async fn test_confirm_device() {
    let client = make_cognito_client().await;

    client
        .confirm_device()
        .access_token("fake-token")
        .device_key("device-key-123")
        .send()
        .await
        .expect("confirm_device should succeed");
}

#[tokio::test]
async fn test_list_devices() {
    let client = make_cognito_client().await;

    let resp = client
        .list_devices()
        .access_token("fake-token")
        .send()
        .await
        .expect("list_devices should succeed");

    assert!(resp.devices().is_empty());
}

#[tokio::test]
async fn test_admin_list_devices() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("AdminDevPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("devuser")
        .send()
        .await
        .unwrap();

    let resp = client
        .admin_list_devices()
        .user_pool_id(&pool_id)
        .username("devuser")
        .send()
        .await
        .expect("admin_list_devices should succeed");

    assert!(resp.devices().is_empty());
}

#[tokio::test]
async fn test_admin_set_user_settings() {
    let client = make_cognito_client().await;

    let pool = client
        .create_user_pool()
        .pool_name("SettingsPool")
        .send()
        .await
        .unwrap();
    let pool_id = pool.user_pool().unwrap().id().unwrap().to_string();

    client
        .admin_create_user()
        .user_pool_id(&pool_id)
        .username("settingsuser")
        .send()
        .await
        .unwrap();

    client
        .admin_set_user_settings()
        .user_pool_id(&pool_id)
        .username("settingsuser")
        .mfa_options(
            aws_sdk_cognitoidentityprovider::types::MfaOptionType::builder()
                .delivery_medium(aws_sdk_cognitoidentityprovider::types::DeliveryMediumType::Sms)
                .attribute_name("phone_number")
                .build(),
        )
        .send()
        .await
        .expect("admin_set_user_settings should succeed");
}
