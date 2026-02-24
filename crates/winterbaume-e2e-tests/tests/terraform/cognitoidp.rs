use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_user_pool_basic() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_pool_basic" {
  name = "terraform-test-pool"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-pool"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_user_pool_with_password_policy() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_pool_password" {
  name = "terraform-password-pool"

  password_policy {
    minimum_length                   = 12
    require_lowercase                = true
    require_numbers                  = true
    require_symbols                  = true
    require_uppercase                = true
    temporary_password_validity_days = 7
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-password-pool"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_user_pool_client_basic() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_client_pool" {
  name = "terraform-client-pool"
}

resource "aws_cognito_user_pool_client" "cognito_client_basic" {
  name         = "terraform-test-client"
  user_pool_id = aws_cognito_user_pool.cognito_client_pool.id

  explicit_auth_flows = [
    "ALLOW_USER_PASSWORD_AUTH",
    "ALLOW_REFRESH_TOKEN_AUTH",
  ]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-client-pool"));
    assert!(result.state.contains("terraform-test-client"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_user_pool_client_with_secret() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_secret_pool" {
  name = "terraform-secret-pool"
}

resource "aws_cognito_user_pool_client" "cognito_client_secret" {
  name                   = "terraform-secret-client"
  user_pool_id           = aws_cognito_user_pool.cognito_secret_pool.id
  generate_secret        = true
  refresh_token_validity = 30

  allowed_oauth_flows_user_pool_client = true
  allowed_oauth_flows                  = ["code", "implicit"]
  allowed_oauth_scopes                 = ["email", "openid", "profile"]
  callback_urls                        = ["https://example.com/callback"]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-secret-pool"));
    assert!(result.state.contains("terraform-secret-client"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_user_pool_domain() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_domain_pool" {
  name = "terraform-domain-pool"
}

resource "aws_cognito_user_pool_domain" "cognito_domain" {
  domain       = "terraform-test-domain"
  user_pool_id = aws_cognito_user_pool.cognito_domain_pool.id
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-domain-pool"));
    assert!(result.state.contains("terraform-test-domain"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_full_pool" {
  name = "cognito-full-stack-pool"

  password_policy {
    minimum_length = 8
    require_numbers = true
  }

  tags = {
    Stack = "full"
  }
}

resource "aws_cognito_user_pool_client" "cognito_full_web" {
  name         = "cognito-full-web-client"
  user_pool_id = aws_cognito_user_pool.cognito_full_pool.id

  explicit_auth_flows = [
    "ALLOW_USER_PASSWORD_AUTH",
    "ALLOW_REFRESH_TOKEN_AUTH",
  ]
}

resource "aws_cognito_user_pool_client" "cognito_full_mobile" {
  name            = "cognito-full-mobile-client"
  user_pool_id    = aws_cognito_user_pool.cognito_full_pool.id
  generate_secret = true

  explicit_auth_flows = [
    "ALLOW_REFRESH_TOKEN_AUTH",
  ]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("cognito-full-stack-pool"));
    assert!(result.state.contains("cognito-full-web-client"));
    assert!(result.state.contains("cognito-full-mobile-client"));
}

// ---------------------------------------------------------------------------
// aws_cognito_user_group
// ---------------------------------------------------------------------------

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_cognito_user_group_basic() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_user_group_pool" {
  name = "terraform-user-group-pool"
}

resource "aws_cognito_user_group" "cognito_user_group_basic" {
  name         = "terraform-test-group"
  user_pool_id = aws_cognito_user_pool.cognito_user_group_pool.id
  description  = "A test group"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-user-group-pool"));
    assert!(result.state.contains("terraform-test-group"));
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_cognito_user_group_with_precedence() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_user_group_prec_pool" {
  name = "terraform-user-group-prec-pool"
}

resource "aws_cognito_user_group" "cognito_user_group_with_precedence" {
  name         = "terraform-prec-group"
  user_pool_id = aws_cognito_user_pool.cognito_user_group_prec_pool.id
  description  = "A group with precedence"
  precedence   = 10
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-user-group-prec-pool"));
    assert!(result.state.contains("terraform-prec-group"));
}

// ---------------------------------------------------------------------------
// aws_cognito_resource_server
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_resource_server_basic() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_rs_pool" {
  name = "terraform-rs-pool"
}

resource "aws_cognito_resource_server" "cognito_rs_basic" {
  identifier   = "https://api.example.com"
  name         = "terraform-test-api"
  user_pool_id = aws_cognito_user_pool.cognito_rs_pool.id
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-rs-pool"));
    assert!(result.state.contains("https://api.example.com"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_resource_server_with_scopes() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_rs_scope_pool" {
  name = "terraform-rs-scope-pool"
}

resource "aws_cognito_resource_server" "cognito_rs_scopes" {
  identifier   = "https://api.scopes.example.com"
  name         = "terraform-scoped-api"
  user_pool_id = aws_cognito_user_pool.cognito_rs_scope_pool.id

  scope {
    scope_name        = "read"
    scope_description = "Read access"
  }

  scope {
    scope_name        = "write"
    scope_description = "Write access"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-rs-scope-pool"));
    assert!(result.state.contains("https://api.scopes.example.com"));
}

// ---------------------------------------------------------------------------
// aws_cognito_identity_provider
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_identity_provider_google() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_idp_pool" {
  name = "terraform-idp-pool"
}

resource "aws_cognito_identity_provider" "cognito_idp_google" {
  user_pool_id  = aws_cognito_user_pool.cognito_idp_pool.id
  provider_name = "Google"
  provider_type = "Google"

  provider_details = {
    client_id        = "test-google-client-id"
    client_secret    = "test-google-client-secret"
    authorize_scopes = "email profile openid"
  }

  attribute_mapping = {
    email    = "email"
    username = "sub"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-idp-pool"));
    assert!(result.state.contains("Google"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_identity_provider_oidc() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_oidc_pool" {
  name = "terraform-oidc-pool"
}

resource "aws_cognito_identity_provider" "cognito_idp_oidc" {
  user_pool_id  = aws_cognito_user_pool.cognito_oidc_pool.id
  provider_name = "TestOIDC"
  provider_type = "OIDC"

  provider_details = {
    client_id                = "test-oidc-client-id"
    client_secret            = "test-oidc-client-secret"
    authorize_scopes         = "openid email"
    oidc_issuer              = "https://idp.example.com"
    attributes_request_method = "GET"
  }

  attribute_mapping = {
    email    = "email"
    username = "sub"
  }

  idp_identifiers = ["testoidc"]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-oidc-pool"));
    assert!(result.state.contains("TestOIDC"));
}

// ---------------------------------------------------------------------------
// aws_cognito_user (AdminCreateUser / AdminGetUser / AdminDeleteUser)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_user_basic() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_user_pool" {
  name = "terraform-user-pool"
}

resource "aws_cognito_user" "cognito_user_basic" {
  user_pool_id = aws_cognito_user_pool.cognito_user_pool.id
  username     = "testuser"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-user-pool"));
    assert!(result.state.contains("testuser"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_user_with_attributes() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_user_attr_pool" {
  name = "terraform-user-attr-pool"

  schema {
    name                = "email"
    attribute_data_type = "String"
    mutable             = true
    required            = true
  }
}

resource "aws_cognito_user" "cognito_user_attrs" {
  user_pool_id = aws_cognito_user_pool.cognito_user_attr_pool.id
  username     = "testuser-attrs"

  attributes = {
    email          = "test@example.com"
    email_verified = "true"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-user-attr-pool"));
    assert!(result.state.contains("testuser-attrs"));
}

// ---------------------------------------------------------------------------
// Combined: multiple new resource types in one apply
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cognito_extended_stack() {
    let result = batch_apply(
        r#"
resource "aws_cognito_user_pool" "cognito_ext_pool" {
  name = "cognito-extended-stack-pool"
}

resource "aws_cognito_user_pool_client" "cognito_ext_client" {
  name         = "cognito-ext-client"
  user_pool_id = aws_cognito_user_pool.cognito_ext_pool.id

  explicit_auth_flows = [
    "ALLOW_USER_PASSWORD_AUTH",
    "ALLOW_REFRESH_TOKEN_AUTH",
  ]
}

resource "aws_cognito_resource_server" "cognito_ext_api" {
  identifier   = "https://api.ext.example.com"
  name         = "ext-api"
  user_pool_id = aws_cognito_user_pool.cognito_ext_pool.id

  scope {
    scope_name        = "read"
    scope_description = "Read access"
  }
}

resource "aws_cognito_identity_provider" "cognito_ext_google" {
  user_pool_id  = aws_cognito_user_pool.cognito_ext_pool.id
  provider_name = "Google"
  provider_type = "Google"

  provider_details = {
    client_id        = "ext-google-client-id"
    client_secret    = "ext-google-client-secret"
    authorize_scopes = "email"
  }

  attribute_mapping = {
    email    = "email"
    username = "sub"
  }
}

resource "aws_cognito_user" "cognito_ext_admin_user" {
  user_pool_id = aws_cognito_user_pool.cognito_ext_pool.id
  username     = "admin-user"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("cognito-extended-stack-pool"));
    assert!(result.state.contains("cognito-ext-client"));
    assert!(result.state.contains("https://api.ext.example.com"));
    assert!(result.state.contains("Google"));
    assert!(result.state.contains("admin-user"));
}
