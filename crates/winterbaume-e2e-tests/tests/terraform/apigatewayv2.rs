use crate::harness::*;

// API Gateway V2 terraform resources tested here:
//   aws_apigatewayv2_api          (CreateApi, GetApis, GetApi, UpdateApi, DeleteApi)
//   aws_apigatewayv2_stage        (CreateStage, GetStages, GetStage, UpdateStage, DeleteStage)
//   aws_apigatewayv2_route        (CreateRoute, GetRoutes, GetRoute, UpdateRoute, DeleteRoute)
//   aws_apigatewayv2_integration  (CreateIntegration, GetIntegrations, GetIntegration,
//                                   UpdateIntegration, DeleteIntegration)
//   aws_apigatewayv2_authorizer   (CreateAuthorizer, GetAuthorizers, GetAuthorizer,
//                                   UpdateAuthorizer, DeleteAuthorizer)
//   aws_apigatewayv2_deployment   (CreateDeployment, GetDeployments, GetDeployment,
//                                   DeleteDeployment)
//   aws_apigatewayv2_domain_name  (CreateDomainName, GetDomainNames, GetDomainName,
//                                   DeleteDomainName)
//   aws_apigatewayv2_api_mapping  (CreateApiMapping, GetApiMappings, GetApiMapping,
//                                   DeleteApiMapping)
//   aws_apigatewayv2_vpc_link     (CreateVpcLink, GetVpcLinks, GetVpcLink, UpdateVpcLink,
//                                   DeleteVpcLink)

// ---------------------------------------------------------------------------
// aws_apigatewayv2_api — HTTP API
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_api_http_basic() {
    let result = batch_apply(
        r#"
resource "aws_apigatewayv2_api" "apigwv2_api_http_basic" {
  name          = "apigwv2-http-basic"
  protocol_type = "HTTP"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("apigwv2-http-basic"),
        "state missing HTTP API"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_api_http_with_cors() {
    let result = batch_apply(
        r#"
resource "aws_apigatewayv2_api" "apigwv2_api_http_cors" {
  name          = "apigwv2-http-cors"
  protocol_type = "HTTP"
  description   = "HTTP API with CORS"

  cors_configuration {
    allow_headers = ["Content-Type", "Authorization"]
    allow_methods = ["GET", "POST", "OPTIONS"]
    allow_origins = ["https://example.com"]
    max_age       = 300
  }

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("apigwv2-http-cors"),
        "state missing HTTP API with CORS"
    );
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_api — WebSocket API
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_api_websocket() {
    let result = batch_apply(
        r#"
resource "aws_apigatewayv2_api" "apigwv2_api_ws" {
  name                       = "apigwv2-websocket"
  protocol_type              = "WEBSOCKET"
  route_selection_expression = "$request.body.action"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("apigwv2-websocket"),
        "state missing WebSocket API"
    );
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_stage
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_stage_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-stage-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_stage_api" {
  name          = "apigwv2-stage-api"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "apigwv2_stage_basic" {
  api_id = aws_apigatewayv2_api.apigwv2_stage_api.id
  name   = "dev"

  tags = {
    Environment = "dev"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_apigatewayv2_stage"),
        "state missing stage"
    );
    assert!(state.contains("\"dev\""), "state missing stage name");

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_stage_auto_deploy() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-stage-auto");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_stage_auto_api" {
  name          = "apigwv2-stage-auto-api"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "apigwv2_stage_auto" {
  api_id      = aws_apigatewayv2_api.apigwv2_stage_auto_api.id
  name        = "$default"
  auto_deploy = true

  stage_variables = {
    env = "staging"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_apigatewayv2_stage"),
        "state missing stage"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_integration
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_integration_http_proxy() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-integ-http");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_integ_api" {
  name          = "apigwv2-integ-api"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_integration" "apigwv2_integ_http" {
  api_id             = aws_apigatewayv2_api.apigwv2_integ_api.id
  integration_type   = "HTTP_PROXY"
  integration_method = "GET"
  integration_uri    = "https://httpbin.org/get"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_apigatewayv2_integration"),
        "state missing integration"
    );
    assert!(
        state.contains("HTTP_PROXY"),
        "state missing integration type"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_integration_lambda() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-integ-lambda");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_integ_lambda_api" {
  name          = "apigwv2-integ-lambda-api"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_integration" "apigwv2_integ_lambda" {
  api_id                 = aws_apigatewayv2_api.apigwv2_integ_lambda_api.id
  integration_type       = "AWS_PROXY"
  integration_uri        = "arn:aws:lambda:us-east-1:123456789012:function:my-function"
  payload_format_version = "2.0"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_apigatewayv2_integration"),
        "state missing integration"
    );
    assert!(
        state.contains("AWS_PROXY"),
        "state missing integration type"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_route
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_route_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-route-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_route_api" {
  name          = "apigwv2-route-api"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_integration" "apigwv2_route_integ" {
  api_id             = aws_apigatewayv2_api.apigwv2_route_api.id
  integration_type   = "HTTP_PROXY"
  integration_method = "GET"
  integration_uri    = "https://httpbin.org/get"
}

resource "aws_apigatewayv2_route" "apigwv2_route_basic" {
  api_id    = aws_apigatewayv2_api.apigwv2_route_api.id
  route_key = "GET /items"
  target    = "integrations/${aws_apigatewayv2_integration.apigwv2_route_integ.id}"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_apigatewayv2_route"),
        "state missing route"
    );
    assert!(state.contains("GET /items"), "state missing route key");

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_authorizer
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_authorizer_jwt() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-auth-jwt");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_auth_api" {
  name          = "apigwv2-auth-api"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_authorizer" "apigwv2_auth_jwt" {
  api_id           = aws_apigatewayv2_api.apigwv2_auth_api.id
  authorizer_type  = "JWT"
  identity_sources = ["$request.header.Authorization"]
  name             = "jwt-authorizer"

  jwt_configuration {
    audience = ["https://api.example.com"]
    issuer   = "https://auth.example.com/"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_apigatewayv2_authorizer"),
        "state missing authorizer"
    );
    assert!(
        state.contains("jwt-authorizer"),
        "state missing authorizer name"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_deployment
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_deployment_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-deploy-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_deploy_api" {
  name          = "apigwv2-deploy-api"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_deployment" "apigwv2_deploy_basic" {
  api_id      = aws_apigatewayv2_api.apigwv2_deploy_api.id
  description = "test deployment"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_apigatewayv2_deployment"),
        "state missing deployment"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_domain_name
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_domain_name_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-domain-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_domain_name" "apigwv2_domain_basic" {
  domain_name = "api.example.com"

  domain_name_configuration {
    certificate_arn = "arn:aws:acm:us-east-1:123456789012:certificate/12345678-abcd-efgh-ijkl-123456789012"
    endpoint_type   = "REGIONAL"
    security_policy = "TLS_1_2"
  }

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_apigatewayv2_domain_name"),
        "state missing domain name"
    );
    assert!(
        state.contains("api.example.com"),
        "state missing domain name value"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_api_mapping
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_api_mapping_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-mapping-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_mapping_api" {
  name          = "apigwv2-mapping-api"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "apigwv2_mapping_stage" {
  api_id = aws_apigatewayv2_api.apigwv2_mapping_api.id
  name   = "prod"
}

resource "aws_apigatewayv2_domain_name" "apigwv2_mapping_domain" {
  domain_name = "mapping.example.com"

  domain_name_configuration {
    certificate_arn = "arn:aws:acm:us-east-1:123456789012:certificate/12345678-abcd-efgh-ijkl-123456789012"
    endpoint_type   = "REGIONAL"
    security_policy = "TLS_1_2"
  }
}

resource "aws_apigatewayv2_api_mapping" "apigwv2_mapping_basic" {
  api_id      = aws_apigatewayv2_api.apigwv2_mapping_api.id
  domain_name = aws_apigatewayv2_domain_name.apigwv2_mapping_domain.id
  stage       = aws_apigatewayv2_stage.apigwv2_mapping_stage.id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_apigatewayv2_api_mapping"),
        "state missing API mapping"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_vpc_link
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_vpc_link_basic() {
    let result = batch_apply(
        r#"
resource "aws_apigatewayv2_vpc_link" "apigwv2_vpc_link_basic" {
  name               = "apigwv2-vpc-link-basic"
  subnet_ids         = ["subnet-12345678", "subnet-87654321"]
  security_group_ids = ["sg-12345678"]

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("apigwv2-vpc-link-basic"),
        "state missing VPC link"
    );
}

// ---------------------------------------------------------------------------
// Full HTTP API lifecycle: api + integration + route + stage
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_full_http_api() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-full-http");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_full_api" {
  name          = "apigwv2-full-http-api"
  protocol_type = "HTTP"
  description   = "Full HTTP API E2E test"
}

resource "aws_apigatewayv2_integration" "apigwv2_full_integ" {
  api_id             = aws_apigatewayv2_api.apigwv2_full_api.id
  integration_type   = "HTTP_PROXY"
  integration_method = "ANY"
  integration_uri    = "https://httpbin.org/{proxy}"
}

resource "aws_apigatewayv2_route" "apigwv2_full_route" {
  api_id    = aws_apigatewayv2_api.apigwv2_full_api.id
  route_key = "ANY /{proxy+}"
  target    = "integrations/${aws_apigatewayv2_integration.apigwv2_full_integ.id}"
}

resource "aws_apigatewayv2_stage" "apigwv2_full_stage" {
  api_id      = aws_apigatewayv2_api.apigwv2_full_api.id
  name        = "$default"
  auto_deploy = true
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("apigwv2-full-http-api"), "state missing API");
    assert!(
        state.contains("aws_apigatewayv2_integration"),
        "state missing integration"
    );
    assert!(
        state.contains("aws_apigatewayv2_route"),
        "state missing route"
    );
    assert!(
        state.contains("aws_apigatewayv2_stage"),
        "state missing stage"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_api — modify in place
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_apigatewayv2_api_modify_in_place() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apigwv2-api-modify");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_api_modify" {
  name          = "apigwv2-api-modify"
  protocol_type = "HTTP"
  description   = "initial description"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apigatewayv2_api" "apigwv2_api_modify" {
  name          = "apigwv2-api-modify"
  protocol_type = "HTTP"
  description   = "updated description"
}
"#,
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "Expected in-place update:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}
