//! Scenario tests for the Amplify UI Builder service.
//!
//! These tests exercise end-to-end use-case workflows that chain multiple
//! operations, asserting business outcomes rather than per-API return shapes.
//!
//! NOTE: The Component, Form, Theme, and CodegenJob per-operation lifecycle
//! coverage lives in `integration_test.rs`.  The placeholder scenario test
//! below is left as a hook for a future end-to-end workflow once we have a
//! richer state view to assert against.

use aws_sdk_amplifyuibuilder::config::BehaviorVersion;
use aws_sdk_amplifyuibuilder::types::PutMetadataFlagBody;
use winterbaume_amplifyuibuilder::AmplifyUiBuilderService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_amplifyuibuilder::Client {
    let mock = MockAws::builder()
        .with_service(AmplifyUiBuilderService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amplifyuibuilder::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_amplifyuibuilder::Client::new(&config)
}

/// Scenario: feature-flag management workflow.
///
/// 1. An environment starts with no feature flags.
/// 2. A flag is enabled via PutMetadataFlag.
/// 3. GetMetadata confirms the flag is active.
/// 4. A second flag is set and both flags are visible.
#[tokio::test]
async fn test_feature_flag_management_workflow() {
    let client = make_client().await;
    let app_id = "app-scenario-1";
    let env = "production";

    // Step 1: confirm empty initial state.
    let initial = client
        .get_metadata()
        .app_id(app_id)
        .environment_name(env)
        .send()
        .await
        .expect("initial get_metadata");
    assert!(
        initial.features().is_empty(),
        "should start with no features"
    );

    // Step 2: enable the first flag.
    client
        .put_metadata_flag()
        .app_id(app_id)
        .environment_name(env)
        .feature_name("darkMode")
        .body(
            PutMetadataFlagBody::builder()
                .new_value("enabled")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put darkMode");

    // Step 3: confirm the flag is active.
    let after_first = client
        .get_metadata()
        .app_id(app_id)
        .environment_name(env)
        .send()
        .await
        .expect("get_metadata after first put");
    assert_eq!(
        after_first.features().get("darkMode"),
        Some(&"enabled".to_string()),
        "darkMode should be enabled"
    );

    // Step 4: set a second flag and confirm both are present.
    client
        .put_metadata_flag()
        .app_id(app_id)
        .environment_name(env)
        .feature_name("betaFeatures")
        .body(
            PutMetadataFlagBody::builder()
                .new_value("true")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put betaFeatures");

    let final_state = client
        .get_metadata()
        .app_id(app_id)
        .environment_name(env)
        .send()
        .await
        .expect("final get_metadata");
    assert_eq!(
        final_state.features().get("darkMode"),
        Some(&"enabled".to_string())
    );
    assert_eq!(
        final_state.features().get("betaFeatures"),
        Some(&"true".to_string())
    );
    assert_eq!(final_state.features().len(), 2, "exactly two flags set");
}

/// Scenario: token exchange and refresh workflow (stateless auth operations).
///
/// 1. ExchangeCodeForToken returns mock access and refresh tokens.
/// 2. RefreshToken uses the returned refresh token to obtain a new access token.
/// 3. An empty code is rejected with a ValidationException.
#[tokio::test]
async fn test_token_exchange_and_refresh_workflow() {
    use aws_sdk_amplifyuibuilder::types::{
        ExchangeCodeForTokenRequestBody, RefreshTokenRequestBody,
    };

    let client = make_client().await;

    // Step 1: exchange a code for tokens.
    let exchange = client
        .exchange_code_for_token()
        .provider("figma".into())
        .request(
            ExchangeCodeForTokenRequestBody::builder()
                .code("auth-code-abc")
                .redirect_uri("https://example.com/callback")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("exchange_code_for_token");
    let access = exchange.access_token().to_string();
    let refresh = exchange.refresh_token().to_string();
    assert!(!access.is_empty(), "access_token must be non-empty");
    assert!(!refresh.is_empty(), "refresh_token must be non-empty");

    // Step 2: use the refresh token.
    let refreshed = client
        .refresh_token()
        .provider("figma".into())
        .refresh_token_body(
            RefreshTokenRequestBody::builder()
                .token(refresh)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("refresh_token");
    assert!(
        !refreshed.access_token().is_empty(),
        "refreshed access token"
    );

    // Step 3: empty code is rejected.
    let err = client
        .exchange_code_for_token()
        .provider("figma".into())
        .request(
            ExchangeCodeForTokenRequestBody::builder()
                .code("")
                .redirect_uri("https://example.com/callback")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect_err("empty code should fail");
    assert!(
        format!("{err:?}").contains("ValidationException"),
        "expected ValidationException"
    );
}

/// Scenario: component / theme / form lifecycle workflows.
///
/// These are ignored because the SDK's strict `httpPayload` deserialisation
/// rejects mock responses that omit `@required` fields the current generator
/// does not emit. Tracked in TODO.md: fix smithy-codegen `@required` emission
/// for restJson httpPayload shapes.
#[tokio::test]
#[ignore = "blocked on smithy-codegen @required field emission for httpPayload shapes; see TODO.md"]
async fn test_component_scenario_create_update_delete() {
    // placeholder — will be filled when the generator fix lands
}
