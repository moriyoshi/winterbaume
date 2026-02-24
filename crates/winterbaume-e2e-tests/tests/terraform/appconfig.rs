use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_appconfig_application
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appconfig_application_basic() {
    let result = batch_apply(
        r#"
resource "aws_appconfig_application" "appconfig_app_basic" {
  name        = "test-appconfig-app"
  description = "Basic AppConfig application for E2E testing"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-appconfig-app"));
}

// ---------------------------------------------------------------------------
// aws_appconfig_environment
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appconfig_environment() {
    let result = batch_apply(
        r#"
resource "aws_appconfig_application" "appconfig_app_for_env" {
  name = "app-for-env-test"
}

resource "aws_appconfig_environment" "appconfig_env" {
  name           = "test-environment"
  description    = "Test environment for E2E"
  application_id = aws_appconfig_application.appconfig_app_for_env.id

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-environment"));
}

// ---------------------------------------------------------------------------
// aws_appconfig_configuration_profile
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appconfig_configuration_profile() {
    let result = batch_apply(
        r#"
resource "aws_appconfig_application" "appconfig_app_for_profile" {
  name = "app-for-profile-test"
}

resource "aws_appconfig_configuration_profile" "appconfig_profile" {
  application_id = aws_appconfig_application.appconfig_app_for_profile.id
  name           = "test-config-profile"
  description    = "Test configuration profile"
  location_uri   = "hosted"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-config-profile"));
}

// ---------------------------------------------------------------------------
// aws_appconfig_deployment_strategy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appconfig_deployment_strategy() {
    let result = batch_apply(
        r#"
resource "aws_appconfig_deployment_strategy" "appconfig_deploy_strategy" {
  name                           = "test-deployment-strategy"
  description                    = "Test deployment strategy"
  deployment_duration_in_minutes = 0
  final_bake_time_in_minutes     = 0
  growth_factor                  = 100
  growth_type                    = "LINEAR"
  replicate_to                   = "NONE"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-deployment-strategy"));
}

// ---------------------------------------------------------------------------
// aws_appconfig_hosted_configuration_version
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appconfig_hosted_configuration_version() {
    let result = batch_apply(
        r#"
resource "aws_appconfig_application" "appconfig_app_for_hosted" {
  name = "app-for-hosted-config"
}

resource "aws_appconfig_configuration_profile" "appconfig_profile_for_hosted" {
  application_id = aws_appconfig_application.appconfig_app_for_hosted.id
  name           = "profile-for-hosted"
  location_uri   = "hosted"
}

resource "aws_appconfig_hosted_configuration_version" "appconfig_hosted_ver" {
  application_id           = aws_appconfig_application.appconfig_app_for_hosted.id
  configuration_profile_id = aws_appconfig_configuration_profile.appconfig_profile_for_hosted.configuration_profile_id
  content_type             = "application/json"
  content                  = jsonencode({ feature_enabled = true, max_items = 10 })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("app-for-hosted-config"));
    assert!(result.state.contains("application/json"));
}

// ---------------------------------------------------------------------------
// aws_appconfig_deployment (full pipeline)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appconfig_deployment() {
    let result = batch_apply(
        r#"
resource "aws_appconfig_application" "appconfig_app_for_deploy" {
  name = "app-for-deployment"
}

resource "aws_appconfig_environment" "appconfig_env_for_deploy" {
  name           = "env-for-deployment"
  application_id = aws_appconfig_application.appconfig_app_for_deploy.id
}

resource "aws_appconfig_configuration_profile" "appconfig_profile_for_deploy" {
  application_id = aws_appconfig_application.appconfig_app_for_deploy.id
  name           = "profile-for-deployment"
  location_uri   = "hosted"
}

resource "aws_appconfig_deployment_strategy" "appconfig_strategy_for_deploy" {
  name                           = "strategy-for-deployment"
  deployment_duration_in_minutes = 0
  final_bake_time_in_minutes     = 0
  growth_factor                  = 100
  growth_type                    = "LINEAR"
  replicate_to                   = "NONE"
}

resource "aws_appconfig_hosted_configuration_version" "appconfig_hosted_for_deploy" {
  application_id           = aws_appconfig_application.appconfig_app_for_deploy.id
  configuration_profile_id = aws_appconfig_configuration_profile.appconfig_profile_for_deploy.configuration_profile_id
  content_type             = "application/json"
  content                  = jsonencode({ enabled = true })
}

resource "aws_appconfig_deployment" "appconfig_deployment" {
  application_id           = aws_appconfig_application.appconfig_app_for_deploy.id
  environment_id           = aws_appconfig_environment.appconfig_env_for_deploy.environment_id
  configuration_profile_id = aws_appconfig_configuration_profile.appconfig_profile_for_deploy.configuration_profile_id
  configuration_version    = aws_appconfig_hosted_configuration_version.appconfig_hosted_for_deploy.version_number
  deployment_strategy_id   = aws_appconfig_deployment_strategy.appconfig_strategy_for_deploy.id

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("app-for-deployment"));
    assert!(result.state.contains("env-for-deployment"));
}

// ---------------------------------------------------------------------------
// aws_appconfig_extension
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appconfig_extension() {
    let result = batch_apply(
        r#"
resource "aws_sns_topic" "appconfig_ext_topic" {
  name = "appconfig-extension-topic"
}

resource "aws_appconfig_extension" "appconfig_extension" {
  name        = "test-extension"
  description = "Test AppConfig extension"

  action_point {
    point = "ON_DEPLOYMENT_COMPLETE"
    action {
      name     = "SendNotification"
      role_arn = "arn:aws:iam::123456789012:role/test-role"
      uri      = aws_sns_topic.appconfig_ext_topic.arn
    }
  }

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-extension"));
}

// ---------------------------------------------------------------------------
// Full stack: application + environment + profile + strategy + config + deploy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appconfig_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_appconfig_application" "appconfig_full" {
  name        = "full-stack-appconfig-app"
  description = "Full stack test"

  tags = {
    Name = "full-stack"
  }
}

resource "aws_appconfig_environment" "appconfig_full" {
  name           = "full-stack-env"
  description    = "Full stack environment"
  application_id = aws_appconfig_application.appconfig_full.id
}

resource "aws_appconfig_configuration_profile" "appconfig_full" {
  application_id = aws_appconfig_application.appconfig_full.id
  name           = "full-stack-profile"
  location_uri   = "hosted"
}

resource "aws_appconfig_deployment_strategy" "appconfig_full" {
  name                           = "full-stack-strategy"
  deployment_duration_in_minutes = 0
  final_bake_time_in_minutes     = 0
  growth_factor                  = 100
  growth_type                    = "LINEAR"
  replicate_to                   = "NONE"
}

resource "aws_appconfig_hosted_configuration_version" "appconfig_full" {
  application_id           = aws_appconfig_application.appconfig_full.id
  configuration_profile_id = aws_appconfig_configuration_profile.appconfig_full.configuration_profile_id
  content_type             = "application/json"
  content                  = jsonencode({ version = "1.0", feature_flags = { dark_mode = true } })
}

resource "aws_appconfig_deployment" "appconfig_full" {
  application_id           = aws_appconfig_application.appconfig_full.id
  environment_id           = aws_appconfig_environment.appconfig_full.environment_id
  configuration_profile_id = aws_appconfig_configuration_profile.appconfig_full.configuration_profile_id
  configuration_version    = aws_appconfig_hosted_configuration_version.appconfig_full.version_number
  deployment_strategy_id   = aws_appconfig_deployment_strategy.appconfig_full.id
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("full-stack-appconfig-app"));
    assert!(result.state.contains("full-stack-env"));
    assert!(result.state.contains("full-stack-profile"));
    assert!(result.state.contains("full-stack-strategy"));
}
