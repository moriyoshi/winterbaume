use aws_sdk_lambda::config::BehaviorVersion;
use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::types::Runtime;
use winterbaume_core::MockAws;
use winterbaume_lambda::LambdaService;

async fn make_lambda_client() -> aws_sdk_lambda::Client {
    let mock = MockAws::builder()
        .with_service(LambdaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lambda::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_lambda::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_function() {
    let client = make_lambda_client().await;

    let resp = client
        .create_function()
        .function_name("test-func")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .expect("create_function should succeed");

    assert_eq!(resp.function_name(), Some("test-func"));
    assert!(resp.function_arn().is_some());

    let get_resp = client
        .get_function()
        .function_name("test-func")
        .send()
        .await
        .expect("get_function should succeed");

    let config = get_resp.configuration().expect("should have config");
    assert_eq!(config.function_name(), Some("test-func"));
    assert_eq!(config.handler(), Some("index.handler"));
}

#[tokio::test]
async fn test_list_functions() {
    let client = make_lambda_client().await;

    for name in ["func1", "func2", "func3"] {
        client
            .create_function()
            .function_name(name)
            .runtime(aws_sdk_lambda::types::Runtime::Python312)
            .handler("index.handler")
            .role("arn:aws:iam::123456789012:role/lambda-role")
            .code(
                aws_sdk_lambda::types::FunctionCode::builder()
                    .zip_file(Blob::new(vec![0u8; 10]))
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_functions()
        .send()
        .await
        .expect("list_functions should succeed");

    assert_eq!(resp.functions().len(), 3);
}

#[tokio::test]
async fn test_delete_function() {
    let client = make_lambda_client().await;

    client
        .create_function()
        .function_name("del-func")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_function()
        .function_name("del-func")
        .send()
        .await
        .expect("delete_function should succeed");

    let result = client.get_function().function_name("del-func").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_function_configuration() {
    let client = make_lambda_client().await;

    client
        .create_function()
        .function_name("upd-func")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .update_function_configuration()
        .function_name("upd-func")
        .description("Updated description")
        .memory_size(256)
        .timeout(30)
        .send()
        .await
        .expect("update_function_configuration should succeed");

    assert_eq!(resp.description(), Some("Updated description"));
    assert_eq!(resp.memory_size(), Some(256));
    assert_eq!(resp.timeout(), Some(30));
}

#[tokio::test]
async fn test_invoke() {
    let client = make_lambda_client().await;

    client
        .create_function()
        .function_name("invoke-func")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .invoke()
        .function_name("invoke-func")
        .send()
        .await
        .expect("invoke should succeed");

    assert_eq!(resp.status_code(), 200);
}

#[tokio::test]
async fn test_create_duplicate_fails() {
    let client = make_lambda_client().await;

    client
        .create_function()
        .function_name("dup-func")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .create_function()
        .function_name("dup-func")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_function_fails() {
    let client = make_lambda_client().await;

    let result = client
        .get_function()
        .function_name("nonexistent-func")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_function_fails() {
    let client = make_lambda_client().await;

    let result = client
        .delete_function()
        .function_name("nonexistent-func")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_functions_empty() {
    let client = make_lambda_client().await;

    let resp = client
        .list_functions()
        .send()
        .await
        .expect("list_functions should succeed");

    assert!(resp.functions().is_empty());
}

#[tokio::test]
async fn test_create_function_fields() {
    let client = make_lambda_client().await;

    let resp = client
        .create_function()
        .function_name("field-func")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .expect("create_function should succeed");

    assert_eq!(resp.function_name(), Some("field-func"));
    assert!(resp.function_arn().unwrap().contains("field-func"));
    assert_eq!(
        resp.runtime(),
        Some(&aws_sdk_lambda::types::Runtime::Python312)
    );
    assert_eq!(resp.handler(), Some("index.handler"));
}

#[tokio::test]
async fn test_update_function_configuration_fields() {
    let client = make_lambda_client().await;

    client
        .create_function()
        .function_name("upd-fields-func")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let update_resp = client
        .update_function_configuration()
        .function_name("upd-fields-func")
        .description("new desc")
        .memory_size(512)
        .timeout(60)
        .send()
        .await
        .expect("update_function_configuration should succeed");

    assert_eq!(update_resp.function_name(), Some("upd-fields-func"));
    assert_eq!(update_resp.description(), Some("new desc"));
    assert_eq!(update_resp.memory_size(), Some(512));
    assert_eq!(update_resp.timeout(), Some(60));

    let get_resp = client
        .get_function()
        .function_name("upd-fields-func")
        .send()
        .await
        .expect("get_function should succeed");

    let config = get_resp.configuration().expect("should have config");
    assert_eq!(config.function_name(), Some("upd-fields-func"));
    assert_eq!(config.description(), Some("new desc"));
    assert_eq!(config.memory_size(), Some(512));
    assert_eq!(config.timeout(), Some(60));
}

#[tokio::test]
async fn test_invoke_nonexistent_function_fails() {
    let client = make_lambda_client().await;

    let result = client
        .invoke()
        .function_name("nonexistent-func")
        .send()
        .await;

    assert!(result.is_err());
}

// ==================== Moto parity tests ====================

/// Helper to compute expected CodeSha256 (base64 of SHA-256) for a given byte slice.
fn compute_code_sha256(content: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content);
    let hash = hasher.finalize();
    base64::engine::general_purpose::STANDARD.encode(hash)
}

use base64::Engine as _;

/// Moto parity: test_create_function_from_zipfile
/// Verifies all fields returned by CreateFunction match moto's expected values.
#[tokio::test]
async fn test_moto_create_function_from_zipfile() {
    let client = make_lambda_client().await;
    let zip_content = vec![80, 75, 3, 4, 0, 0, 0, 0, 0, 0]; // minimal zip-like bytes

    let result = client
        .create_function()
        .function_name("moto-create-zip")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .description("test lambda function")
        .timeout(3)
        .memory_size(128)
        .send()
        .await
        .expect("create_function should succeed");

    // Exact value assertions matching moto behavior
    assert_eq!(result.function_name(), Some("moto-create-zip"));
    assert_eq!(
        result.function_arn(),
        Some("arn:aws:lambda:us-east-1:123456789012:function:moto-create-zip")
    );
    assert_eq!(result.runtime(), Some(&Runtime::Python311));
    assert_eq!(result.handler(), Some("lambda_function.lambda_handler"));
    assert_eq!(result.description(), Some("test lambda function"));
    assert_eq!(result.timeout(), Some(3));
    assert_eq!(result.memory_size(), Some(128));
    assert_eq!(
        result.code_sha256(),
        Some(compute_code_sha256(&zip_content).as_str())
    );
    assert_eq!(result.code_size(), zip_content.len() as i64);
    assert_eq!(result.version(), Some("$LATEST"));
    assert_eq!(result.state(), Some(&aws_sdk_lambda::types::State::Active));
    assert_eq!(
        result.package_type(),
        Some(&aws_sdk_lambda::types::PackageType::Zip)
    );

    // Architectures: ["x86_64"]
    let archs = result.architectures();
    assert_eq!(archs.len(), 1);
    assert_eq!(archs[0], aws_sdk_lambda::types::Architecture::X8664);

    // EphemeralStorage: { Size: 512 }
    let ephemeral = result
        .ephemeral_storage()
        .expect("should have ephemeral_storage");
    assert_eq!(ephemeral.size(), 512);

    // TracingConfig: { Mode: "PassThrough" }
    let tracing = result.tracing_config().expect("should have tracing_config");
    assert_eq!(
        tracing.mode(),
        Some(&aws_sdk_lambda::types::TracingMode::PassThrough)
    );

    // Layers: []
    assert!(result.layers().is_empty());

    // LastUpdateStatus: Successful
    assert_eq!(
        result.last_update_status(),
        Some(&aws_sdk_lambda::types::LastUpdateStatus::Successful)
    );
}

/// Moto parity: test_create_function_from_zipfile - defaults
/// When no timeout/memory specified, defaults should be 3 and 128.
#[tokio::test]
async fn test_moto_create_function_defaults() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 20];

    let result = client
        .create_function()
        .function_name("moto-defaults")
        .runtime(Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .send()
        .await
        .expect("create_function should succeed");

    // Default timeout is 3 (matches moto)
    assert_eq!(result.timeout(), Some(3));
    // Default memory_size is 128 (matches moto)
    assert_eq!(result.memory_size(), Some(128));
    // Default description is empty string
    assert_eq!(result.description(), Some(""));
    // Version should be $LATEST
    assert_eq!(result.version(), Some("$LATEST"));
}

/// Moto parity: test_get_function_created_with_zipfile
/// Tests that GetFunction returns proper Configuration and Code sections.
#[tokio::test]
async fn test_moto_get_function_created_with_zipfile() {
    let client = make_lambda_client().await;
    let zip_content = vec![80, 75, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    client
        .create_function()
        .function_name("moto-get-zip")
        .runtime(Runtime::Python311)
        .handler("lambda_function.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .description("test lambda function")
        .send()
        .await
        .unwrap();

    let response = client
        .get_function()
        .function_name("moto-get-zip")
        .send()
        .await
        .expect("get_function should succeed");

    // Code section
    let code = response.code().expect("should have code");
    assert_eq!(code.repository_type(), Some("S3"));
    // Location should be an S3 URL
    let location = code.location().expect("should have location");
    assert!(
        location.contains("awslambda"),
        "Location should reference awslambda S3 bucket"
    );

    // Configuration section
    let config = response.configuration().expect("should have config");
    assert_eq!(
        config.code_sha256(),
        Some(compute_code_sha256(&zip_content).as_str())
    );
    assert_eq!(config.code_size(), zip_content.len() as i64);
    assert_eq!(config.description(), Some("test lambda function"));
    assert_eq!(
        config.function_arn(),
        Some("arn:aws:lambda:us-east-1:123456789012:function:moto-get-zip")
    );
    assert_eq!(config.function_name(), Some("moto-get-zip"));
    assert_eq!(config.handler(), Some("lambda_function.handler"));
    assert_eq!(config.memory_size(), Some(128));
    assert_eq!(config.runtime(), Some(&Runtime::Python311));
    assert_eq!(config.timeout(), Some(3));
    assert_eq!(config.version(), Some("$LATEST"));
    assert_eq!(config.state(), Some(&aws_sdk_lambda::types::State::Active));
    assert!(config.layers().is_empty());
    assert_eq!(
        config.last_update_status(),
        Some(&aws_sdk_lambda::types::LastUpdateStatus::Successful)
    );

    // Tags on GetFunction response
    let tags = response.tags();
    // No tags were set, so should be empty
    assert!(tags.is_none() || tags.unwrap().is_empty());
}

/// Moto parity: test_create_function_with_already_exists
/// Duplicate creation should return ResourceConflictException.
#[tokio::test]
async fn test_moto_create_function_already_exists() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 10];

    client
        .create_function()
        .function_name("moto-dup")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .create_function()
        .function_name("moto-dup")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_conflict_exception());
}

/// Moto parity: test_get_unknown_function
#[tokio::test]
async fn test_moto_get_unknown_function() {
    let client = make_lambda_client().await;

    let result = client.get_function().function_name("junk").send().await;

    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

/// Moto parity: test_delete_unknown_function
#[tokio::test]
async fn test_moto_delete_unknown_function() {
    let client = make_lambda_client().await;

    let result = client
        .delete_function()
        .function_name("testFunctionThatDoesntExist")
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

/// Moto parity: test_delete_function
/// After deletion, list should not include the deleted function.
#[tokio::test]
async fn test_moto_delete_function_and_verify_list() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 15];

    client
        .create_function()
        .function_name("moto-del")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .description("test lambda function")
        .timeout(3)
        .memory_size(128)
        .send()
        .await
        .unwrap();

    client
        .delete_function()
        .function_name("moto-del")
        .send()
        .await
        .expect("delete should succeed");

    let func_list = client.list_functions().send().await.unwrap();
    let our_functions: Vec<_> = func_list
        .functions()
        .iter()
        .filter(|f| f.function_name() == Some("moto-del"))
        .collect();
    assert_eq!(our_functions.len(), 0);
}

/// Moto parity: test_delete_function_by_arn
/// Should be able to delete by ARN.
#[tokio::test]
async fn test_moto_delete_function_by_arn() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 15];

    let fnc = client
        .create_function()
        .function_name("moto-del-arn")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let arn = fnc.function_arn().unwrap().to_string();

    client
        .delete_function()
        .function_name(&arn)
        .send()
        .await
        .expect("delete by ARN should succeed");

    let func_list = client.list_functions().send().await.unwrap();
    let our_functions: Vec<_> = func_list
        .functions()
        .iter()
        .filter(|f| f.function_name() == Some("moto-del-arn"))
        .collect();
    assert_eq!(our_functions.len(), 0);
}

/// Moto parity: test_get_function_by_arn
/// Should be able to get function by its ARN.
#[tokio::test]
async fn test_moto_get_function_by_arn() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 15];

    let fnc = client
        .create_function()
        .function_name("moto-get-arn")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let arn = fnc.function_arn().unwrap().to_string();

    let result = client
        .get_function()
        .function_name(&arn)
        .send()
        .await
        .expect("get by ARN should succeed");

    let config = result.configuration().unwrap();
    assert_eq!(config.function_name(), Some("moto-get-arn"));
}

/// Moto parity: test_update_configuration
/// Verifies update_function_configuration updates only specified fields and preserves others.
#[tokio::test]
async fn test_moto_update_configuration() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 20];

    let fxn = client
        .create_function()
        .function_name("moto-upd-cfg")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .description("test lambda function")
        .timeout(3)
        .memory_size(128)
        .environment(
            aws_sdk_lambda::types::Environment::builder()
                .variables("test_old_environment", "test_old_value")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Verify initial values (moto parity)
    assert_eq!(fxn.description(), Some("test lambda function"));
    assert_eq!(fxn.handler(), Some("lambda_function.lambda_handler"));
    assert_eq!(fxn.memory_size(), Some(128));
    assert_eq!(fxn.runtime(), Some(&Runtime::Python311));
    assert_eq!(fxn.timeout(), Some(3));

    // Update configuration
    let updated_config = client
        .update_function_configuration()
        .function_name("moto-upd-cfg")
        .description("updated test lambda function")
        .handler("lambda_function.new_lambda_handler")
        .runtime(Runtime::Python312)
        .timeout(7)
        .environment(
            aws_sdk_lambda::types::Environment::builder()
                .variables("test_environment", "test_value")
                .build(),
        )
        .send()
        .await
        .expect("update_function_configuration should succeed");

    // Exact assertions matching moto
    assert_eq!(
        updated_config.description(),
        Some("updated test lambda function")
    );
    assert_eq!(
        updated_config.handler(),
        Some("lambda_function.new_lambda_handler")
    );
    // Memory should remain 128 since we didn't update it
    assert_eq!(updated_config.memory_size(), Some(128));
    assert_eq!(updated_config.runtime(), Some(&Runtime::Python312));
    assert_eq!(updated_config.timeout(), Some(7));

    // Environment should be updated (old vars replaced)
    let env = updated_config
        .environment()
        .expect("should have environment");
    let vars = env.variables().expect("should have variables");
    assert_eq!(
        vars.get("test_environment"),
        Some(&"test_value".to_string())
    );
    assert!(vars.get("test_old_environment").is_none());
}

/// Moto parity: test_list_create_list_get_delete_list
/// Full integration flow: list -> create -> list -> get -> delete -> list
#[tokio::test]
async fn test_moto_list_create_list_get_delete_list() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 25];

    // Initial list should be empty
    let initial_list = client.list_functions().send().await.unwrap();
    assert!(initial_list.functions().is_empty());

    // Create function
    client
        .create_function()
        .function_name("moto-flow")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .description("test lambda function")
        .timeout(3)
        .memory_size(128)
        .send()
        .await
        .unwrap();

    // List should now contain 1 function
    let functions = client.list_functions().send().await.unwrap();
    let func_names: Vec<_> = functions
        .functions()
        .iter()
        .filter_map(|f| f.function_name())
        .collect();
    assert!(func_names.contains(&"moto-flow"));

    // Verify ARN
    let func_arn = functions
        .functions()
        .iter()
        .find(|f| f.function_name() == Some("moto-flow"))
        .and_then(|f| f.function_arn())
        .unwrap();
    assert_eq!(
        func_arn,
        "arn:aws:lambda:us-east-1:123456789012:function:moto-flow"
    );

    // Get function
    let func = client
        .get_function()
        .function_name("moto-flow")
        .send()
        .await
        .unwrap();

    let config = func.configuration().unwrap();
    assert_eq!(
        config.function_arn(),
        Some("arn:aws:lambda:us-east-1:123456789012:function:moto-flow")
    );
    assert_eq!(config.function_name(), Some("moto-flow"));
    assert_eq!(config.handler(), Some("lambda_function.lambda_handler"));
    assert_eq!(config.memory_size(), Some(128));
    assert_eq!(config.runtime(), Some(&Runtime::Python311));
    assert_eq!(config.timeout(), Some(3));
    assert_eq!(config.version(), Some("$LATEST"));
    assert_eq!(config.state(), Some(&aws_sdk_lambda::types::State::Active));
    assert_eq!(config.description(), Some("test lambda function"));
    assert_eq!(
        config.code_sha256(),
        Some(compute_code_sha256(&zip_content).as_str())
    );
    assert_eq!(config.code_size(), zip_content.len() as i64);
    assert_eq!(
        config.package_type(),
        Some(&aws_sdk_lambda::types::PackageType::Zip)
    );

    // Delete function
    client
        .delete_function()
        .function_name("moto-flow")
        .send()
        .await
        .unwrap();

    // List should be empty again
    let functions = client.list_functions().send().await.unwrap();
    let func_names: Vec<_> = functions
        .functions()
        .iter()
        .filter_map(|f| f.function_name())
        .collect();
    assert!(!func_names.contains(&"moto-flow"));
}

/// Moto parity: test_create_function_with_tags
/// Tags should be returned in GetFunction response.
#[tokio::test]
async fn test_moto_create_function_with_tags() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 10];

    let function = client
        .create_function()
        .function_name("moto-tags")
        .runtime(Runtime::Python311)
        .handler("lambda_function.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    // Tags should be returned in get_function
    let result = client
        .get_function()
        .function_name("moto-tags")
        .send()
        .await
        .unwrap();

    let tags = result.tags().expect("should have tags");
    assert_eq!(tags.get("key1"), Some(&"val1".to_string()));
    assert_eq!(tags.get("key2"), Some(&"val2".to_string()));
    assert_eq!(tags.len(), 2);
}

/// Moto parity: test_create_function_with_environment
/// Environment variables should be in both create and get responses.
#[tokio::test]
async fn test_moto_create_function_with_environment() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 10];

    let result = client
        .create_function()
        .function_name("moto-env")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .environment(
            aws_sdk_lambda::types::Environment::builder()
                .variables("test_variable", "test_value")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Environment should be in create response
    let env = result
        .environment()
        .expect("should have environment in create response");
    let vars = env.variables().expect("should have variables");
    assert_eq!(vars.get("test_variable"), Some(&"test_value".to_string()));

    // And in get_function response
    let get_result = client
        .get_function()
        .function_name("moto-env")
        .send()
        .await
        .unwrap();

    let config = get_result.configuration().unwrap();
    let env = config
        .environment()
        .expect("should have environment in get response");
    let vars = env.variables().expect("should have variables");
    assert_eq!(vars.get("test_variable"), Some(&"test_value".to_string()));
}

/// Moto parity: test_update_function_zip (update_function_code)
/// CodeSha256 should change after update.
#[tokio::test]
async fn test_moto_update_function_code() {
    let client = make_lambda_client().await;
    let zip_content_one = vec![1u8; 10];

    let fxn = client
        .create_function()
        .function_name("moto-upd-code")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content_one.clone()))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let first_sha = fxn.code_sha256().unwrap().to_string();
    assert_eq!(first_sha, compute_code_sha256(&zip_content_one));

    // Update function code
    let update_result = client
        .update_function_code()
        .function_name("moto-upd-code")
        .send()
        .await
        .expect("update_function_code should succeed");

    // CodeSha256 should change after update
    let second_sha = update_result.code_sha256().unwrap().to_string();
    assert_ne!(second_sha, first_sha);

    // Function name should be preserved
    assert_eq!(update_result.function_name(), Some("moto-upd-code"));
}

/// Moto parity: test_invoke with payload
#[tokio::test]
async fn test_moto_invoke_returns_200() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 10];

    client
        .create_function()
        .function_name("moto-invoke")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .invoke()
        .function_name("moto-invoke")
        .send()
        .await
        .expect("invoke should succeed");

    assert_eq!(resp.status_code(), 200);
    assert_eq!(resp.executed_version(), Some("$LATEST"));
}

/// Moto parity: FunctionArn format verification
/// ARN should be arn:aws:lambda:{region}:{account_id}:function:{name}
#[tokio::test]
async fn test_moto_function_arn_format() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 10];

    let result = client
        .create_function()
        .function_name("arn-test")
        .runtime(Runtime::Python311)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content))
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Exact ARN format matching moto
    assert_eq!(
        result.function_arn(),
        Some("arn:aws:lambda:us-east-1:123456789012:function:arn-test")
    );
}

/// Moto parity: test_update_configuration preserves unspecified fields
/// When only description is updated, handler/runtime/memory/timeout should remain unchanged.
#[tokio::test]
async fn test_moto_update_configuration_preserves_fields() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 10];

    client
        .create_function()
        .function_name("moto-preserve")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content))
                .build(),
        )
        .timeout(5)
        .memory_size(256)
        .send()
        .await
        .unwrap();

    // Update only description
    let updated = client
        .update_function_configuration()
        .function_name("moto-preserve")
        .description("new description")
        .send()
        .await
        .unwrap();

    // Updated field
    assert_eq!(updated.description(), Some("new description"));
    // Preserved fields
    assert_eq!(updated.handler(), Some("lambda_function.lambda_handler"));
    assert_eq!(updated.runtime(), Some(&Runtime::Python311));
    assert_eq!(updated.timeout(), Some(5));
    assert_eq!(updated.memory_size(), Some(256));
    assert_eq!(updated.function_name(), Some("moto-preserve"));
}

/// Moto parity: verify list_functions returns correct fields for each function
#[tokio::test]
async fn test_moto_list_functions_fields() {
    let client = make_lambda_client().await;
    let zip_content = vec![1u8; 30];

    client
        .create_function()
        .function_name("moto-list-fields")
        .runtime(Runtime::Python311)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(zip_content.clone()))
                .build(),
        )
        .description("list test")
        .timeout(10)
        .memory_size(256)
        .send()
        .await
        .unwrap();

    let resp = client.list_functions().send().await.unwrap();
    assert_eq!(resp.functions().len(), 1);

    let func = &resp.functions()[0];
    assert_eq!(func.function_name(), Some("moto-list-fields"));
    assert_eq!(
        func.function_arn(),
        Some("arn:aws:lambda:us-east-1:123456789012:function:moto-list-fields")
    );
    assert_eq!(func.runtime(), Some(&Runtime::Python311));
    assert_eq!(func.handler(), Some("lambda_function.lambda_handler"));
    assert_eq!(func.description(), Some("list test"));
    assert_eq!(func.timeout(), Some(10));
    assert_eq!(func.memory_size(), Some(256));
    assert_eq!(
        func.code_sha256(),
        Some(compute_code_sha256(&zip_content).as_str())
    );
    assert_eq!(func.code_size(), zip_content.len() as i64);
    assert_eq!(func.version(), Some("$LATEST"));
    assert_eq!(func.state(), Some(&aws_sdk_lambda::types::State::Active));
    assert_eq!(
        func.package_type(),
        Some(&aws_sdk_lambda::types::PackageType::Zip)
    );
}

/// Moto parity: invoke nonexistent function should return ResourceNotFoundException
#[tokio::test]
async fn test_moto_invoke_nonexistent_resource_not_found() {
    let client = make_lambda_client().await;

    let result = client.invoke().function_name("nonexistent").send().await;

    let err = result.unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ==================== New operation tests ====================

async fn create_test_function(client: &aws_sdk_lambda::Client, name: &str) {
    client
        .create_function()
        .function_name(name)
        .runtime(Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();
}

// ========== Alias tests ==========

#[tokio::test]
async fn test_create_and_get_alias() {
    let client = make_lambda_client().await;
    create_test_function(&client, "alias-func").await;

    let resp = client
        .create_alias()
        .function_name("alias-func")
        .name("my-alias")
        .function_version("$LATEST")
        .description("test alias")
        .send()
        .await
        .expect("create_alias should succeed");

    assert_eq!(resp.name(), Some("my-alias"));
    assert_eq!(resp.function_version(), Some("$LATEST"));
    assert_eq!(resp.description(), Some("test alias"));
    assert!(resp.alias_arn().unwrap().contains("alias-func:my-alias"));

    let get_resp = client
        .get_alias()
        .function_name("alias-func")
        .name("my-alias")
        .send()
        .await
        .expect("get_alias should succeed");

    assert_eq!(get_resp.name(), Some("my-alias"));
    assert_eq!(get_resp.function_version(), Some("$LATEST"));
}

#[tokio::test]
async fn test_update_alias() {
    let client = make_lambda_client().await;
    create_test_function(&client, "upd-alias-func").await;

    client
        .create_alias()
        .function_name("upd-alias-func")
        .name("upd-alias")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_alias()
        .function_name("upd-alias-func")
        .name("upd-alias")
        .description("updated desc")
        .send()
        .await
        .expect("update_alias should succeed");

    assert_eq!(resp.description(), Some("updated desc"));
}

#[tokio::test]
async fn test_delete_alias() {
    let client = make_lambda_client().await;
    create_test_function(&client, "del-alias-func").await;

    client
        .create_alias()
        .function_name("del-alias-func")
        .name("del-alias")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap();

    client
        .delete_alias()
        .function_name("del-alias-func")
        .name("del-alias")
        .send()
        .await
        .expect("delete_alias should succeed");

    let result = client
        .get_alias()
        .function_name("del-alias-func")
        .name("del-alias")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_aliases() {
    let client = make_lambda_client().await;
    create_test_function(&client, "list-alias-func").await;

    for name in ["alias1", "alias2"] {
        client
            .create_alias()
            .function_name("list-alias-func")
            .name(name)
            .function_version("$LATEST")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_aliases()
        .function_name("list-alias-func")
        .send()
        .await
        .expect("list_aliases should succeed");

    assert_eq!(resp.aliases().len(), 2);
}

// ========== Event Source Mapping tests ==========

#[tokio::test]
async fn test_create_and_get_event_source_mapping() {
    let client = make_lambda_client().await;
    create_test_function(&client, "esm-func").await;

    let resp = client
        .create_event_source_mapping()
        .function_name("esm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:my-queue")
        .batch_size(10)
        .enabled(true)
        .send()
        .await
        .expect("create_event_source_mapping should succeed");

    let uuid = resp.uuid().unwrap().to_string();
    assert!(!uuid.is_empty());
    assert_eq!(resp.batch_size(), Some(10));
    assert_eq!(resp.state(), Some("Enabled"));

    let get_resp = client
        .get_event_source_mapping()
        .uuid(&uuid)
        .send()
        .await
        .expect("get_event_source_mapping should succeed");

    assert_eq!(get_resp.uuid(), Some(uuid.as_str()));
}

#[tokio::test]
async fn test_update_event_source_mapping() {
    let client = make_lambda_client().await;
    create_test_function(&client, "upd-esm-func").await;

    let resp = client
        .create_event_source_mapping()
        .function_name("upd-esm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:queue2")
        .batch_size(5)
        .send()
        .await
        .unwrap();

    let uuid = resp.uuid().unwrap().to_string();

    let upd = client
        .update_event_source_mapping()
        .uuid(&uuid)
        .batch_size(20)
        .enabled(false)
        .send()
        .await
        .expect("update_event_source_mapping should succeed");

    assert_eq!(upd.batch_size(), Some(20));
    assert_eq!(upd.state(), Some("Disabled"));
}

#[tokio::test]
async fn test_delete_event_source_mapping() {
    let client = make_lambda_client().await;
    create_test_function(&client, "del-esm-func").await;

    let resp = client
        .create_event_source_mapping()
        .function_name("del-esm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:queue3")
        .send()
        .await
        .unwrap();

    let uuid = resp.uuid().unwrap().to_string();

    client
        .delete_event_source_mapping()
        .uuid(&uuid)
        .send()
        .await
        .expect("delete_event_source_mapping should succeed");

    let result = client.get_event_source_mapping().uuid(&uuid).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_event_source_mappings() {
    let client = make_lambda_client().await;
    create_test_function(&client, "list-esm-func").await;

    client
        .create_event_source_mapping()
        .function_name("list-esm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:queue-list")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_event_source_mappings()
        .send()
        .await
        .expect("list_event_source_mappings should succeed");

    assert!(!resp.event_source_mappings().is_empty());
}

// ========== Function URL Config tests ==========

#[tokio::test]
async fn test_create_and_get_function_url_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "url-func").await;

    let resp = client
        .create_function_url_config()
        .function_name("url-func")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::None)
        .send()
        .await
        .expect("create_function_url_config should succeed");

    assert!(resp.function_url().contains("lambda-url"));
    assert_eq!(
        resp.auth_type(),
        &aws_sdk_lambda::types::FunctionUrlAuthType::None
    );

    let get_resp = client
        .get_function_url_config()
        .function_name("url-func")
        .send()
        .await
        .expect("get_function_url_config should succeed");

    assert_eq!(get_resp.function_url(), resp.function_url());
}

#[tokio::test]
async fn test_update_function_url_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "upd-url-func").await;

    client
        .create_function_url_config()
        .function_name("upd-url-func")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::None)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_function_url_config()
        .function_name("upd-url-func")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::AwsIam)
        .send()
        .await
        .expect("update_function_url_config should succeed");

    assert_eq!(
        resp.auth_type(),
        &aws_sdk_lambda::types::FunctionUrlAuthType::AwsIam
    );
}

#[tokio::test]
async fn test_delete_function_url_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "del-url-func").await;

    client
        .create_function_url_config()
        .function_name("del-url-func")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::None)
        .send()
        .await
        .unwrap();

    client
        .delete_function_url_config()
        .function_name("del-url-func")
        .send()
        .await
        .expect("delete_function_url_config should succeed");

    let result = client
        .get_function_url_config()
        .function_name("del-url-func")
        .send()
        .await;
    assert!(result.is_err());
}

// ========== Permission tests ==========

#[tokio::test]
async fn test_add_and_get_permission() {
    let client = make_lambda_client().await;
    create_test_function(&client, "perm-func").await;

    let resp = client
        .add_permission()
        .function_name("perm-func")
        .statement_id("stmt1")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .send()
        .await
        .expect("add_permission should succeed");

    assert!(resp.statement().is_some());

    let policy = client
        .get_policy()
        .function_name("perm-func")
        .send()
        .await
        .expect("get_policy should succeed");

    assert!(policy.policy().unwrap().contains("stmt1"));
}

#[tokio::test]
async fn test_remove_permission() {
    let client = make_lambda_client().await;
    create_test_function(&client, "rm-perm-func").await;

    client
        .add_permission()
        .function_name("rm-perm-func")
        .statement_id("stmt-rm")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .send()
        .await
        .unwrap();

    client
        .remove_permission()
        .function_name("rm-perm-func")
        .statement_id("stmt-rm")
        .send()
        .await
        .expect("remove_permission should succeed");

    let result = client
        .get_policy()
        .function_name("rm-perm-func")
        .send()
        .await;
    assert!(result.is_err());
}

// ========== RevisionId optimistic-concurrency tests ==========
//
// AddPermission and RemovePermission both model an optional RevisionId.
// Prior to the fix, the handler dropped the value and the GetPolicy response
// minted a fresh uuid on every call, making the RevisionId useless for
// optimistic concurrency.  The fix maintains a per-policy revision id that
// bumps on every successful mutation; passing a stale id must surface as
// PreconditionFailedException (HTTP 412).

#[tokio::test]
async fn test_get_policy_revision_id_is_stable_across_reads() {
    let client = make_lambda_client().await;
    create_test_function(&client, "rev-stable-func").await;
    client
        .add_permission()
        .function_name("rev-stable-func")
        .statement_id("stmt-stable")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .send()
        .await
        .unwrap();

    let first = client
        .get_policy()
        .function_name("rev-stable-func")
        .send()
        .await
        .unwrap();
    let second = client
        .get_policy()
        .function_name("rev-stable-func")
        .send()
        .await
        .unwrap();

    let r1 = first.revision_id().unwrap();
    let r2 = second.revision_id().unwrap();
    assert!(!r1.is_empty(), "revision_id should be populated");
    assert_eq!(
        r1, r2,
        "GetPolicy must return a stable revision id across reads, got {r1} vs {r2}",
    );
}

#[tokio::test]
async fn test_add_permission_bumps_revision_id() {
    let client = make_lambda_client().await;
    create_test_function(&client, "rev-bump-func").await;

    client
        .add_permission()
        .function_name("rev-bump-func")
        .statement_id("stmt-a")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .send()
        .await
        .unwrap();
    let rev1 = client
        .get_policy()
        .function_name("rev-bump-func")
        .send()
        .await
        .unwrap()
        .revision_id()
        .unwrap()
        .to_string();

    client
        .add_permission()
        .function_name("rev-bump-func")
        .statement_id("stmt-b")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .revision_id(&rev1)
        .send()
        .await
        .expect("AddPermission with matching RevisionId must succeed");
    let rev2 = client
        .get_policy()
        .function_name("rev-bump-func")
        .send()
        .await
        .unwrap()
        .revision_id()
        .unwrap()
        .to_string();

    assert_ne!(
        rev1, rev2,
        "RevisionId must change after a successful AddPermission",
    );
}

#[tokio::test]
async fn test_add_permission_with_stale_revision_id_is_rejected() {
    use aws_sdk_lambda::operation::add_permission::AddPermissionError;

    let client = make_lambda_client().await;
    create_test_function(&client, "rev-stale-add-func").await;

    client
        .add_permission()
        .function_name("rev-stale-add-func")
        .statement_id("stmt-1")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .send()
        .await
        .unwrap();

    let err = client
        .add_permission()
        .function_name("rev-stale-add-func")
        .statement_id("stmt-2")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .revision_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await
        .expect_err("AddPermission with stale RevisionId must be rejected");

    let svc_err = err.into_service_error();
    assert!(
        matches!(svc_err, AddPermissionError::PreconditionFailedException(_)),
        "expected PreconditionFailedException, got {svc_err:?}",
    );

    // The rejected call must not have mutated state -- the second statement
    // must still be absent.
    let policy = client
        .get_policy()
        .function_name("rev-stale-add-func")
        .send()
        .await
        .unwrap();
    assert!(policy.policy().unwrap().contains("stmt-1"));
    assert!(
        !policy.policy().unwrap().contains("stmt-2"),
        "stmt-2 must not be present after the rejected AddPermission",
    );
}

#[tokio::test]
async fn test_remove_permission_with_stale_revision_id_is_rejected() {
    use aws_sdk_lambda::operation::remove_permission::RemovePermissionError;

    let client = make_lambda_client().await;
    create_test_function(&client, "rev-stale-remove-func").await;

    client
        .add_permission()
        .function_name("rev-stale-remove-func")
        .statement_id("stmt-keep")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .send()
        .await
        .unwrap();

    let err = client
        .remove_permission()
        .function_name("rev-stale-remove-func")
        .statement_id("stmt-keep")
        .revision_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await
        .expect_err("RemovePermission with stale RevisionId must be rejected");

    let svc_err = err.into_service_error();
    assert!(
        matches!(
            svc_err,
            RemovePermissionError::PreconditionFailedException(_)
        ),
        "expected PreconditionFailedException, got {svc_err:?}",
    );

    // The rejected call must not have mutated state -- the statement must
    // still be present.
    let policy = client
        .get_policy()
        .function_name("rev-stale-remove-func")
        .send()
        .await
        .unwrap();
    assert!(policy.policy().unwrap().contains("stmt-keep"));
}

#[tokio::test]
async fn test_remove_permission_with_matching_revision_id_succeeds() {
    let client = make_lambda_client().await;
    create_test_function(&client, "rev-match-remove-func").await;

    client
        .add_permission()
        .function_name("rev-match-remove-func")
        .statement_id("stmt-bye")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .send()
        .await
        .unwrap();

    let rev = client
        .get_policy()
        .function_name("rev-match-remove-func")
        .send()
        .await
        .unwrap()
        .revision_id()
        .unwrap()
        .to_string();

    client
        .remove_permission()
        .function_name("rev-match-remove-func")
        .statement_id("stmt-bye")
        .revision_id(&rev)
        .send()
        .await
        .expect("RemovePermission with matching RevisionId must succeed");

    // Policy is now empty -- AWS returns ResourceNotFoundException.
    let result = client
        .get_policy()
        .function_name("rev-match-remove-func")
        .send()
        .await;
    assert!(result.is_err());
}

// ========== Concurrency tests ==========

#[tokio::test]
async fn test_put_and_get_function_concurrency() {
    let client = make_lambda_client().await;
    create_test_function(&client, "conc-func").await;

    let resp = client
        .put_function_concurrency()
        .function_name("conc-func")
        .reserved_concurrent_executions(100)
        .send()
        .await
        .expect("put_function_concurrency should succeed");

    assert_eq!(resp.reserved_concurrent_executions(), Some(100));

    let get_resp = client
        .get_function_concurrency()
        .function_name("conc-func")
        .send()
        .await
        .expect("get_function_concurrency should succeed");

    assert_eq!(get_resp.reserved_concurrent_executions(), Some(100));
}

#[tokio::test]
async fn test_delete_function_concurrency() {
    let client = make_lambda_client().await;
    create_test_function(&client, "del-conc-func").await;

    client
        .put_function_concurrency()
        .function_name("del-conc-func")
        .reserved_concurrent_executions(50)
        .send()
        .await
        .unwrap();

    client
        .delete_function_concurrency()
        .function_name("del-conc-func")
        .send()
        .await
        .expect("delete_function_concurrency should succeed");

    let get_resp = client
        .get_function_concurrency()
        .function_name("del-conc-func")
        .send()
        .await
        .unwrap();

    assert!(get_resp.reserved_concurrent_executions().is_none());
}

// ========== Function Event Invoke Config tests ==========

#[tokio::test]
async fn test_put_and_get_function_event_invoke_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "eic-func").await;

    let resp = client
        .put_function_event_invoke_config()
        .function_name("eic-func")
        .maximum_retry_attempts(1)
        .maximum_event_age_in_seconds(3600)
        .send()
        .await
        .expect("put_function_event_invoke_config should succeed");

    assert_eq!(resp.maximum_retry_attempts(), Some(1));
    assert_eq!(resp.maximum_event_age_in_seconds(), Some(3600));

    let get_resp = client
        .get_function_event_invoke_config()
        .function_name("eic-func")
        .send()
        .await
        .expect("get_function_event_invoke_config should succeed");

    assert_eq!(get_resp.maximum_retry_attempts(), Some(1));
}

#[tokio::test]
async fn test_update_function_event_invoke_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "upd-eic-func").await;

    client
        .put_function_event_invoke_config()
        .function_name("upd-eic-func")
        .maximum_retry_attempts(2)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_function_event_invoke_config()
        .function_name("upd-eic-func")
        .maximum_retry_attempts(0)
        .send()
        .await
        .expect("update_function_event_invoke_config should succeed");

    assert_eq!(resp.maximum_retry_attempts(), Some(0));
}

#[tokio::test]
async fn test_delete_function_event_invoke_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "del-eic-func").await;

    client
        .put_function_event_invoke_config()
        .function_name("del-eic-func")
        .maximum_retry_attempts(1)
        .send()
        .await
        .unwrap();

    client
        .delete_function_event_invoke_config()
        .function_name("del-eic-func")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_function_event_invoke_config()
        .function_name("del-eic-func")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_function_event_invoke_configs() {
    let client = make_lambda_client().await;
    create_test_function(&client, "list-eic-func").await;

    client
        .put_function_event_invoke_config()
        .function_name("list-eic-func")
        .maximum_retry_attempts(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_function_event_invoke_configs()
        .function_name("list-eic-func")
        .send()
        .await
        .expect("list_function_event_invoke_configs should succeed");

    assert_eq!(resp.function_event_invoke_configs().len(), 1);
}

// ========== Tag tests ==========

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_lambda_client().await;
    create_test_function(&client, "tag-func").await;

    let arn = "arn:aws:lambda:us-east-1:123456789012:function:tag-func";

    client
        .tag_resource()
        .resource(arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags()
        .resource(arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env"), Some(&"prod".to_string()));
    assert_eq!(tags.get("team"), Some(&"backend".to_string()));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_lambda_client().await;
    create_test_function(&client, "untag-func").await;

    let arn = "arn:aws:lambda:us-east-1:123456789012:function:untag-func";

    client
        .tag_resource()
        .resource(arn)
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource(arn)
        .tag_keys("key1")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client.list_tags().resource(arn).send().await.unwrap();

    let tags = resp.tags().unwrap();
    assert!(tags.get("key1").is_none());
    assert_eq!(tags.get("key2"), Some(&"val2".to_string()));
}

#[tokio::test]
async fn test_tag_resource_url_encoded_arn_roundtrip() {
    // Regression test: the SDK URL-encodes colons (:) and other reserved
    // characters when interpolating an ARN into a path-style URI like
    // /2017-03-31/tags/{Resource+}. The handler must URL-decode the
    // remaining path segments before using them as a state-map key.
    let client = make_lambda_client().await;
    create_test_function(&client, "url-encoded-tag-func").await;

    let arn = "arn:aws:lambda:us-east-1:123456789012:function:url-encoded-tag-func";

    client
        .tag_resource()
        .resource(arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("tag_resource should succeed despite URL-encoded path ARN");

    let resp = client
        .list_tags()
        .resource(arn)
        .send()
        .await
        .expect("list_tags should succeed and find tags after URL-encoded lookup");

    let tags = resp.tags().unwrap();
    assert_eq!(
        tags.get("env"),
        Some(&"prod".to_string()),
        "tags must round-trip when ARN is URL-encoded by the SDK"
    );

    client
        .untag_resource()
        .resource(arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed despite URL-encoded path ARN");

    let resp = client.list_tags().resource(arn).send().await.unwrap();
    let tags = resp.tags().unwrap();
    assert!(tags.get("env").is_none());
}

// ========== Layer tests ==========

#[tokio::test]
async fn test_publish_and_get_layer_version() {
    let client = make_lambda_client().await;

    let resp = client
        .publish_layer_version()
        .layer_name("my-layer")
        .description("test layer")
        .compatible_runtimes(aws_sdk_lambda::types::Runtime::Python312)
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .expect("publish_layer_version should succeed");

    assert_eq!(resp.version(), 1);
    assert!(resp.layer_arn().unwrap().contains("my-layer"));
    assert_eq!(resp.description(), Some("test layer"));

    let get_resp = client
        .get_layer_version()
        .layer_name("my-layer")
        .version_number(1)
        .send()
        .await
        .expect("get_layer_version should succeed");

    assert_eq!(get_resp.version(), 1);
    assert_eq!(get_resp.description(), Some("test layer"));
}

#[tokio::test]
async fn test_delete_layer_version() {
    let client = make_lambda_client().await;

    client
        .publish_layer_version()
        .layer_name("del-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_layer_version()
        .layer_name("del-layer")
        .version_number(1)
        .send()
        .await
        .expect("delete_layer_version should succeed");
}

#[tokio::test]
async fn test_list_layer_versions() {
    let client = make_lambda_client().await;

    for _ in 0..2 {
        client
            .publish_layer_version()
            .layer_name("list-layer")
            .content(
                aws_sdk_lambda::types::LayerVersionContentInput::builder()
                    .zip_file(Blob::new(vec![0u8; 10]))
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_layer_versions()
        .layer_name("list-layer")
        .send()
        .await
        .expect("list_layer_versions should succeed");

    assert_eq!(resp.layer_versions().len(), 2);
}

#[tokio::test]
async fn test_list_layers() {
    let client = make_lambda_client().await;

    client
        .publish_layer_version()
        .layer_name("layers-test")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_layers()
        .send()
        .await
        .expect("list_layers should succeed");

    assert!(!resp.layers().is_empty());
}

// ========== Layer permission tests ==========

#[tokio::test]
async fn test_add_and_get_layer_version_policy() {
    let client = make_lambda_client().await;

    client
        .publish_layer_version()
        .layer_name("perm-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .add_layer_version_permission()
        .layer_name("perm-layer")
        .version_number(1)
        .statement_id("stmt1")
        .action("lambda:GetLayerVersion")
        .principal("*")
        .send()
        .await
        .expect("add_layer_version_permission should succeed");

    assert!(resp.statement().is_some());

    let policy = client
        .get_layer_version_policy()
        .layer_name("perm-layer")
        .version_number(1)
        .send()
        .await
        .expect("get_layer_version_policy should succeed");

    assert!(policy.policy().unwrap().contains("stmt1"));
}

#[tokio::test]
async fn test_remove_layer_version_permission() {
    let client = make_lambda_client().await;

    client
        .publish_layer_version()
        .layer_name("rm-perm-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .add_layer_version_permission()
        .layer_name("rm-perm-layer")
        .version_number(1)
        .statement_id("stmt-rm")
        .action("lambda:GetLayerVersion")
        .principal("*")
        .send()
        .await
        .unwrap();

    client
        .remove_layer_version_permission()
        .layer_name("rm-perm-layer")
        .version_number(1)
        .statement_id("stmt-rm")
        .send()
        .await
        .expect("remove_layer_version_permission should succeed");
}

// ========== Version tests ==========

#[tokio::test]
async fn test_publish_version() {
    let client = make_lambda_client().await;
    create_test_function(&client, "ver-func").await;

    let resp = client
        .publish_version()
        .function_name("ver-func")
        .description("v1")
        .send()
        .await
        .expect("publish_version should succeed");

    assert_eq!(resp.version(), Some("1"));
    assert_eq!(resp.description(), Some("v1"));
}

#[tokio::test]
async fn test_list_versions_by_function() {
    let client = make_lambda_client().await;
    create_test_function(&client, "list-ver-func").await;

    client
        .publish_version()
        .function_name("list-ver-func")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_versions_by_function()
        .function_name("list-ver-func")
        .send()
        .await
        .expect("list_versions_by_function should succeed");

    // Should have $LATEST + 1 published version
    assert_eq!(resp.versions().len(), 2);
}

// ========== GetFunctionCodeSigningConfig test ==========

#[tokio::test]
async fn test_get_function_code_signing_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "csc-func").await;

    let resp = client
        .get_function_code_signing_config()
        .function_name("csc-func")
        .send()
        .await
        .expect("get_function_code_signing_config should succeed");

    assert_eq!(resp.function_name(), "csc-func");
}

// ========== Lifecycle test ==========

#[tokio::test]
async fn test_alias_lifecycle() {
    let client = make_lambda_client().await;
    create_test_function(&client, "lifecycle-func").await;

    // Create
    client
        .create_alias()
        .function_name("lifecycle-func")
        .name("live")
        .function_version("$LATEST")
        .description("initial")
        .send()
        .await
        .unwrap();

    // Get
    let alias = client
        .get_alias()
        .function_name("lifecycle-func")
        .name("live")
        .send()
        .await
        .unwrap();
    assert_eq!(alias.description(), Some("initial"));

    // Update
    let updated = client
        .update_alias()
        .function_name("lifecycle-func")
        .name("live")
        .description("updated")
        .send()
        .await
        .unwrap();
    assert_eq!(updated.description(), Some("updated"));

    // Delete
    client
        .delete_alias()
        .function_name("lifecycle-func")
        .name("live")
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .get_alias()
        .function_name("lifecycle-func")
        .name("live")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Ported from moto: test_lambda_alias.py
// ============================================================================

// Ported from moto: test_lambda_alias.py::test_create_alias
#[tokio::test]
async fn test_moto_create_alias() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-ca-func").await;

    let resp = client
        .create_alias()
        .function_name("moto-ca-func")
        .name("alias1")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap();

    assert!(resp.alias_arn().unwrap().contains("moto-ca-func:alias1"));
    assert_eq!(resp.name(), Some("alias1"));
    assert_eq!(resp.function_version(), Some("$LATEST"));
    assert_eq!(resp.description(), Some(""));
    assert!(resp.revision_id().is_some());
    assert!(resp.routing_config().is_none());
}

// Ported from moto: test_lambda_alias.py::test_create_alias_with_routing_config
#[tokio::test]
async fn test_moto_create_alias_with_routing_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-carc-func").await;

    let routing = aws_sdk_lambda::types::AliasRoutingConfiguration::builder()
        .additional_version_weights("2", 0.5)
        .build();

    let resp = client
        .create_alias()
        .function_name("moto-carc-func")
        .name("alias1")
        .function_version("$LATEST")
        .description("desc")
        .routing_config(routing)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("alias1"));
    assert_eq!(resp.description(), Some("desc"));
    let rc = resp.routing_config().unwrap();
    let weights = rc.additional_version_weights().unwrap();
    assert_eq!(weights.get("2"), Some(&0.5));
}

// Ported from moto: test_lambda_alias.py::test_create_alias_using_function_arn
#[tokio::test]
async fn test_moto_create_alias_using_function_arn() {
    let client = make_lambda_client().await;

    let fn_resp = client
        .create_function()
        .function_name("moto-caarn-func")
        .runtime(Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let fn_arn = fn_resp.function_arn().unwrap();

    let resp = client
        .create_alias()
        .function_name(fn_arn)
        .name("alias1")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap();

    assert!(resp.alias_arn().unwrap().contains("moto-caarn-func:alias1"));
    assert_eq!(resp.name(), Some("alias1"));
    assert_eq!(resp.function_version(), Some("$LATEST"));
}

// Ported from moto: test_lambda_alias.py::test_delete_alias
#[tokio::test]
async fn test_moto_delete_alias() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-da-func").await;

    client
        .create_alias()
        .function_name("moto-da-func")
        .name("alias1")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap();

    client
        .delete_alias()
        .function_name("moto-da-func")
        .name("alias1")
        .send()
        .await
        .unwrap();

    let err = client
        .get_alias()
        .function_name("moto-da-func")
        .name("alias1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_lambda_alias.py::test_get_alias
#[tokio::test]
async fn test_moto_get_alias() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-ga-func").await;

    client
        .create_alias()
        .function_name("moto-ga-func")
        .name("alias1")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_alias()
        .function_name("moto-ga-func")
        .name("alias1")
        .send()
        .await
        .unwrap();

    assert!(resp.alias_arn().unwrap().contains("moto-ga-func:alias1"));
    assert_eq!(resp.name(), Some("alias1"));
    assert_eq!(resp.function_version(), Some("$LATEST"));
    assert_eq!(resp.description(), Some(""));
    assert!(resp.revision_id().is_some());
}

// Ported from moto: test_lambda_alias.py::test_get_unknown_alias
#[tokio::test]
async fn test_moto_get_unknown_alias() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-gua-func").await;

    let err = client
        .get_alias()
        .function_name("moto-gua-func")
        .name("unknown")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
    assert!(
        err_str.contains("Cannot find alias arn"),
        "Expected 'Cannot find alias arn', got: {err_str}"
    );
}

// Ported from moto: test_lambda_alias.py::test_aliases_are_unique_per_function
#[tokio::test]
async fn test_moto_aliases_are_unique_per_function() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-uniq1").await;
    create_test_function(&client, "moto-uniq2").await;

    // Same alias name on different functions should work
    client
        .create_alias()
        .function_name("moto-uniq1")
        .name("alias1")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap();

    client
        .create_alias()
        .function_name("moto-uniq2")
        .name("alias1")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap();

    // Duplicate alias on the same function should fail
    let err = client
        .create_alias()
        .function_name("moto-uniq1")
        .name("alias1")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException, got: {err_str}"
    );
    assert!(
        err_str.contains("Alias already exists"),
        "Expected 'Alias already exists', got: {err_str}"
    );
}

// Ported from moto: test_lambda_alias.py::test_update_alias
#[tokio::test]
async fn test_moto_update_alias_with_version() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-ua-func").await;

    client
        .create_alias()
        .function_name("moto-ua-func")
        .name("alias1")
        .function_version("$LATEST")
        .send()
        .await
        .unwrap();

    // Publish a version
    client
        .publish_version()
        .function_name("moto-ua-func")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_alias()
        .function_name("moto-ua-func")
        .name("alias1")
        .function_version("1")
        .description("updated desc")
        .send()
        .await
        .unwrap();

    assert!(resp.alias_arn().unwrap().contains("moto-ua-func:alias1"));
    assert_eq!(resp.name(), Some("alias1"));
    assert_eq!(resp.function_version(), Some("1"));
    assert_eq!(resp.description(), Some("updated desc"));
    assert!(resp.revision_id().is_some());
}

// Ported from moto: test_lambda_alias.py::test_update_alias_routingconfig
#[tokio::test]
async fn test_moto_update_alias_routingconfig() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-uarc-func").await;

    client
        .create_alias()
        .function_name("moto-uarc-func")
        .name("alias1")
        .function_version("$LATEST")
        .description("desc")
        .send()
        .await
        .unwrap();

    let routing = aws_sdk_lambda::types::AliasRoutingConfiguration::builder()
        .additional_version_weights("2", 0.5)
        .build();

    let resp = client
        .update_alias()
        .function_name("moto-uarc-func")
        .name("alias1")
        .routing_config(routing)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("alias1"));
    assert_eq!(resp.function_version(), Some("$LATEST"));
    assert_eq!(resp.description(), Some("desc"));
    let rc = resp.routing_config().unwrap();
    let weights = rc.additional_version_weights().unwrap();
    assert_eq!(weights.get("2"), Some(&0.5));
}

// ============================================================================
// Ported from moto: test_lambda_tags.py
// ============================================================================

// Ported from moto: test_lambda_tags.py::test_tags
#[tokio::test]
async fn test_moto_tags_integration() {
    let client = make_lambda_client().await;

    let func = client
        .create_function()
        .function_name("moto-tags-func")
        .runtime(Runtime::Python312)
        .handler("lambda_function.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let fn_arn = func.function_arn().unwrap();

    // List tags when there are none
    let tags = client.list_tags().resource(fn_arn).send().await.unwrap();
    let empty = std::collections::HashMap::new();
    let tag_map = tags.tags().unwrap_or(&empty);
    assert!(tag_map.is_empty());

    // Tag resource
    client
        .tag_resource()
        .resource(fn_arn)
        .tags("spam", "eggs")
        .send()
        .await
        .unwrap();

    let tags = client.list_tags().resource(fn_arn).send().await.unwrap();
    let tag_map = tags.tags().unwrap();
    assert_eq!(tag_map.get("spam"), Some(&"eggs".to_string()));

    // Add another tag
    client
        .tag_resource()
        .resource(fn_arn)
        .tags("foo", "bar")
        .send()
        .await
        .unwrap();

    let tags = client.list_tags().resource(fn_arn).send().await.unwrap();
    let tag_map = tags.tags().unwrap();
    assert_eq!(tag_map.len(), 2);
    assert_eq!(tag_map.get("spam"), Some(&"eggs".to_string()));
    assert_eq!(tag_map.get("foo"), Some(&"bar".to_string()));

    // Untag resource (including a nonexistent key - should not error)
    client
        .untag_resource()
        .resource(fn_arn)
        .tag_keys("spam")
        .tag_keys("trolls")
        .send()
        .await
        .unwrap();

    let tags = client.list_tags().resource(fn_arn).send().await.unwrap();
    let tag_map = tags.tags().unwrap();
    assert_eq!(tag_map.len(), 1);
    assert_eq!(tag_map.get("foo"), Some(&"bar".to_string()));

    // Untag a key that does not exist (no error)
    client
        .untag_resource()
        .resource(fn_arn)
        .tag_keys("spam")
        .send()
        .await
        .unwrap();
}

// Ported from moto: test_lambda_tags.py::test_create_function_with_tags
#[tokio::test]
async fn test_moto_create_function_with_tags_and_get() {
    let client = make_lambda_client().await;

    let func = client
        .create_function()
        .function_name("moto-tag-create-func")
        .runtime(Runtime::Python312)
        .handler("lambda_function.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    let fn_arn = func.function_arn().unwrap();

    let tags = client.list_tags().resource(fn_arn).send().await.unwrap();
    let tag_map = tags.tags().unwrap();
    assert_eq!(tag_map.get("key1"), Some(&"val1".to_string()));
    assert_eq!(tag_map.get("key2"), Some(&"val2".to_string()));

    // get_function should also return tags
    let result = client
        .get_function()
        .function_name("moto-tag-create-func")
        .send()
        .await
        .unwrap();
    let fn_tags = result.tags().unwrap();
    assert_eq!(fn_tags.get("key1"), Some(&"val1".to_string()));
    assert_eq!(fn_tags.get("key2"), Some(&"val2".to_string()));
}

// Ported from moto: test_lambda_tags.py::test_tags_not_found
#[tokio::test]
async fn test_moto_tags_not_found() {
    let client = make_lambda_client().await;

    let bad_arn = "arn:aws:lambda:us-east-1:123456789012:function:not-found";

    let err = client
        .list_tags()
        .resource(bad_arn)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("not found"),
        "Expected error for list_tags on nonexistent, got: {err_str}"
    );

    let err = client
        .tag_resource()
        .resource(bad_arn)
        .tags("spam", "eggs")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("not found"),
        "Expected error for tag_resource on nonexistent, got: {err_str}"
    );

    let err = client
        .untag_resource()
        .resource(bad_arn)
        .tag_keys("spam")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("not found"),
        "Expected error for untag_resource on nonexistent, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_lambda_policy.py
// ============================================================================

// Ported from moto: test_lambda_policy.py::test_add_function_permission
#[tokio::test]
async fn test_moto_add_function_permission() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-perm-func").await;

    let resp = client
        .add_permission()
        .function_name("moto-perm-func")
        .statement_id("1")
        .action("lambda:InvokeFunction")
        .principal("432143214321")
        .source_arn("arn:aws:lambda:us-west-2:account-id:function:helloworld")
        .send()
        .await
        .unwrap();

    assert!(resp.statement().is_some());
    let stmt: serde_json::Value = serde_json::from_str(resp.statement().unwrap()).unwrap();
    assert_eq!(stmt["Action"], "lambda:InvokeFunction");
    assert_eq!(
        stmt["Condition"]["ArnLike"]["AWS:SourceArn"],
        "arn:aws:lambda:us-west-2:account-id:function:helloworld"
    );
}

// Ported from moto: test_lambda_policy.py::test_get_function_policy
#[tokio::test]
async fn test_moto_get_function_policy() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-gp-func").await;

    client
        .add_permission()
        .function_name("moto-gp-func")
        .statement_id("2")
        .action("lambda:InvokeFunction")
        .principal("lambda.amazonaws.com")
        .source_arn("arn:aws:lambda:us-west-2:123456789012:function:helloworld")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_policy()
        .function_name("moto-gp-func")
        .send()
        .await
        .unwrap();

    assert!(resp.policy().is_some());
    let policy: serde_json::Value = serde_json::from_str(resp.policy().unwrap()).unwrap();
    assert_eq!(policy["Statement"][0]["Action"], "lambda:InvokeFunction");
    assert_eq!(
        policy["Statement"][0]["Principal"]["Service"],
        "lambda.amazonaws.com"
    );
    // Resource should be the function ARN
    let resource = policy["Statement"][0]["Resource"].as_str().unwrap();
    assert!(
        resource.contains("moto-gp-func"),
        "Resource should contain function name"
    );
}

// Ported from moto: test_lambda_policy.py::test_remove_function_permission
#[tokio::test]
async fn test_moto_remove_function_permission() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-rp-func").await;

    client
        .add_permission()
        .function_name("moto-rp-func")
        .statement_id("1")
        .action("lambda:InvokeFunction")
        .principal("432143214321")
        .source_arn("arn:aws:lambda:us-west-2:account-id:function:helloworld")
        .send()
        .await
        .unwrap();

    client
        .remove_permission()
        .function_name("moto-rp-func")
        .statement_id("1")
        .send()
        .await
        .unwrap();

    // get_policy should now fail because there are no permissions
    let err = client
        .get_policy()
        .function_name("moto-rp-func")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
    assert!(
        err_str.contains("The resource you requested does not exist"),
        "Expected correct message, got: {err_str}"
    );
}

// Ported from moto: test_lambda_policy.py::test_get_unknown_policy
#[tokio::test]
async fn test_moto_get_unknown_policy() {
    let client = make_lambda_client().await;

    let err = client
        .get_policy()
        .function_name("unknown")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
    assert!(
        err_str.contains("Function not found"),
        "Expected 'Function not found' message, got: {err_str}"
    );
}

// Ported from moto: test_lambda_policy.py::test_policy_error_if_blank_resource_policy
#[tokio::test]
async fn test_moto_policy_error_if_blank_resource_policy() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-blank-policy-func").await;

    let err = client
        .get_policy()
        .function_name("moto-blank-policy-func")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
    assert!(
        err_str.contains("The resource you requested does not exist"),
        "Expected correct message, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_lambda_concurrency.py
// ============================================================================

// Ported from moto: test_lambda_concurrency.py::test_put_function_concurrency
#[tokio::test]
async fn test_moto_put_function_concurrency() {
    let client = make_lambda_client().await;

    let func = client
        .create_function()
        .function_name("moto-conc-func")
        .runtime(Runtime::Python312)
        .handler("lambda_function.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    // By FunctionName
    let result = client
        .put_function_concurrency()
        .function_name("moto-conc-func")
        .reserved_concurrent_executions(15)
        .send()
        .await
        .unwrap();
    assert_eq!(result.reserved_concurrent_executions(), Some(15));

    // By FunctionArn
    let fn_arn = func.function_arn().unwrap();
    let result = client
        .put_function_concurrency()
        .function_name(fn_arn)
        .reserved_concurrent_executions(25)
        .send()
        .await
        .unwrap();
    assert_eq!(result.reserved_concurrent_executions(), Some(25));
}

// Ported from moto: test_lambda_concurrency.py::test_delete_function_concurrency
#[tokio::test]
async fn test_moto_delete_function_concurrency() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-delconc-func").await;

    client
        .put_function_concurrency()
        .function_name("moto-delconc-func")
        .reserved_concurrent_executions(15)
        .send()
        .await
        .unwrap();

    client
        .delete_function_concurrency()
        .function_name("moto-delconc-func")
        .send()
        .await
        .unwrap();

    let result = client
        .get_function()
        .function_name("moto-delconc-func")
        .send()
        .await
        .unwrap();

    // Concurrency field should not be present after deletion
    assert!(
        result.concurrency().is_none()
            || result
                .concurrency()
                .unwrap()
                .reserved_concurrent_executions()
                .is_none()
    );
}

// Ported from moto: test_lambda_concurrency.py::test_get_function_concurrency
#[tokio::test]
async fn test_moto_get_function_concurrency() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-getconc-func").await;

    client
        .put_function_concurrency()
        .function_name("moto-getconc-func")
        .reserved_concurrent_executions(15)
        .send()
        .await
        .unwrap();

    let result = client
        .get_function_concurrency()
        .function_name("moto-getconc-func")
        .send()
        .await
        .unwrap();

    assert_eq!(result.reserved_concurrent_executions(), Some(15));
}

// ============================================================================
// Ported from moto: test_lambda_function_urls.py
// ============================================================================

// Ported from moto: test_lambda_function_urls.py::test_create_function_url_config
#[tokio::test]
async fn test_moto_create_function_url_config_by_name_and_arn() {
    let client = make_lambda_client().await;

    let func = client
        .create_function()
        .function_name("moto-url-func")
        .runtime(Runtime::Python312)
        .handler("lambda_function.lambda_handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let fn_arn = func.function_arn().unwrap();

    let resp = client
        .create_function_url_config()
        .function_name("moto-url-func")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::AwsIam)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.function_arn(), fn_arn);
    assert_eq!(
        resp.auth_type(),
        &aws_sdk_lambda::types::FunctionUrlAuthType::AwsIam
    );
    assert!(!resp.function_url().is_empty());

    let get_resp = client
        .get_function_url_config()
        .function_name("moto-url-func")
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.function_arn(), fn_arn);
    assert_eq!(
        get_resp.auth_type(),
        &aws_sdk_lambda::types::FunctionUrlAuthType::AwsIam
    );
    assert!(!get_resp.function_url().is_empty());
}

// Ported from moto: test_lambda_function_urls.py::test_create_function_url_config_with_cors
#[tokio::test]
async fn test_moto_create_function_url_config_with_cors() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-url-cors-func").await;

    let cors = aws_sdk_lambda::types::Cors::builder()
        .allow_credentials(true)
        .allow_headers("date")
        .allow_headers("keep-alive")
        .allow_methods("*")
        .allow_origins("*")
        .expose_headers("date")
        .expose_headers("keep-alive")
        .max_age(86400)
        .build();

    let resp = client
        .create_function_url_config()
        .function_name("moto-url-cors-func")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::AwsIam)
        .cors(cors)
        .send()
        .await
        .unwrap();

    let cors_resp = resp.cors().unwrap();
    assert_eq!(cors_resp.allow_credentials(), Some(true));
    let headers = cors_resp.allow_headers();
    assert!(headers.contains(&"date".to_string()));
    assert!(headers.contains(&"keep-alive".to_string()));
    let methods = cors_resp.allow_methods();
    assert!(methods.contains(&"*".to_string()));
    let origins = cors_resp.allow_origins();
    assert!(origins.contains(&"*".to_string()));
    let expose = cors_resp.expose_headers();
    assert!(expose.contains(&"date".to_string()));
    assert!(expose.contains(&"keep-alive".to_string()));
    assert_eq!(cors_resp.max_age(), Some(86400));
}

// Ported from moto: test_lambda_function_urls.py::test_update_function_url_config_with_cors
#[tokio::test]
async fn test_moto_update_function_url_config_with_cors() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-url-upcors-func").await;

    let cors = aws_sdk_lambda::types::Cors::builder()
        .allow_credentials(true)
        .allow_headers("date")
        .allow_headers("keep-alive")
        .allow_methods("*")
        .allow_origins("*")
        .expose_headers("date")
        .expose_headers("keep-alive")
        .max_age(86400)
        .build();

    client
        .create_function_url_config()
        .function_name("moto-url-upcors-func")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::AwsIam)
        .cors(cors)
        .send()
        .await
        .unwrap();

    let updated_cors = aws_sdk_lambda::types::Cors::builder()
        .allow_credentials(false)
        .build();

    let resp = client
        .update_function_url_config()
        .function_name("moto-url-upcors-func")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::None)
        .cors(updated_cors)
        .send()
        .await
        .unwrap();

    let cors_resp = resp.cors().unwrap();
    assert_eq!(cors_resp.allow_credentials(), Some(false));
}

// Ported from moto: test_lambda_function_urls.py::test_delete_function_url_config
#[tokio::test]
async fn test_moto_delete_function_url_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-url-del-func").await;

    client
        .create_function_url_config()
        .function_name("moto-url-del-func")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::AwsIam)
        .send()
        .await
        .unwrap();

    client
        .delete_function_url_config()
        .function_name("moto-url-del-func")
        .send()
        .await
        .unwrap();

    let err = client
        .get_function_url_config()
        .function_name("moto-url-del-func")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("not found"),
        "Expected error after deletion, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_lambda_layers.py
// ============================================================================

// Ported from moto: test_lambda_layers.py::test_get_layer_version
#[tokio::test]
async fn test_moto_get_layer_version() {
    let client = make_lambda_client().await;

    let resp = client
        .publish_layer_version()
        .layer_name("moto-glv-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .compatible_runtimes(aws_sdk_lambda::types::Runtime::Python36)
        .license_info("MIT")
        .compatible_architectures(aws_sdk_lambda::types::Architecture::X8664)
        .send()
        .await
        .unwrap();

    let version = resp.version();

    let get_resp = client
        .get_layer_version()
        .layer_name("moto-glv-layer")
        .version_number(version)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.version(), 1);
    assert_eq!(
        get_resp.compatible_architectures(),
        &[aws_sdk_lambda::types::Architecture::X8664]
    );
    assert_eq!(
        get_resp.compatible_runtimes(),
        &[aws_sdk_lambda::types::Runtime::Python36]
    );
    assert_eq!(get_resp.license_info(), Some("MIT"));
}

// Ported from moto: test_lambda_layers.py::test_get_layer_version__unknown
#[tokio::test]
async fn test_moto_get_layer_version_unknown() {
    let client = make_lambda_client().await;

    // Layer that never existed
    let err = client
        .get_layer_version()
        .layer_name("nonexistent-layer")
        .version_number(1)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );

    // Publish then try nonexistent version
    client
        .publish_layer_version()
        .layer_name("moto-glvunk-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .compatible_runtimes(aws_sdk_lambda::types::Runtime::Python36)
        .license_info("MIT")
        .send()
        .await
        .unwrap();

    let err = client
        .get_layer_version()
        .layer_name("moto-glvunk-layer")
        .version_number(999)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_lambda_layers.py::test_delete_layer_version
#[tokio::test]
async fn test_moto_delete_layer_version() {
    let client = make_lambda_client().await;

    let resp = client
        .publish_layer_version()
        .layer_name("moto-dlv-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .compatible_runtimes(aws_sdk_lambda::types::Runtime::Python36)
        .license_info("MIT")
        .send()
        .await
        .unwrap();

    let version = resp.version();

    // Verify it exists
    client
        .get_layer_version()
        .layer_name("moto-dlv-layer")
        .version_number(version)
        .send()
        .await
        .unwrap();

    // Delete
    client
        .delete_layer_version()
        .layer_name("moto-dlv-layer")
        .version_number(version)
        .send()
        .await
        .unwrap();

    // Verify empty
    let result = client
        .list_layer_versions()
        .layer_name("moto-dlv-layer")
        .send()
        .await
        .unwrap();
    assert_eq!(result.layer_versions().len(), 0);
}

// Ported from moto: test_lambda_layers.py::test_list_lambda_layers_with_unknown_name
#[tokio::test]
async fn test_moto_list_layer_versions_unknown_name() {
    let client = make_lambda_client().await;

    let result = client
        .list_layer_versions()
        .layer_name("nonexistent-layer")
        .send()
        .await
        .unwrap();

    assert_eq!(result.layer_versions().len(), 0);
}

// Ported from moto: test_lambda_layers.py::test_get_layer_with_no_layer_versions
#[tokio::test]
async fn test_moto_get_layer_with_no_layer_versions() {
    let client = make_lambda_client().await;

    // Publish v1
    client
        .publish_layer_version()
        .layer_name("moto-glnlv-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Check list_layers shows it with version 1
    let layers = client.list_layers().send().await.unwrap();
    let our_layer = layers
        .layers()
        .iter()
        .find(|l| l.layer_name() == Some("moto-glnlv-layer"));
    assert!(our_layer.is_some());
    assert_eq!(
        our_layer
            .unwrap()
            .latest_matching_version()
            .unwrap()
            .version(),
        1
    );

    // Publish v2
    client
        .publish_layer_version()
        .layer_name("moto-glnlv-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let layers = client.list_layers().send().await.unwrap();
    let our_layer = layers
        .layers()
        .iter()
        .find(|l| l.layer_name() == Some("moto-glnlv-layer"));
    assert_eq!(
        our_layer
            .unwrap()
            .latest_matching_version()
            .unwrap()
            .version(),
        2
    );

    // Delete v2 -> should fall back to v1
    client
        .delete_layer_version()
        .layer_name("moto-glnlv-layer")
        .version_number(2)
        .send()
        .await
        .unwrap();
    let layers = client.list_layers().send().await.unwrap();
    let our_layer = layers
        .layers()
        .iter()
        .find(|l| l.layer_name() == Some("moto-glnlv-layer"));
    assert_eq!(
        our_layer
            .unwrap()
            .latest_matching_version()
            .unwrap()
            .version(),
        1
    );

    // Delete v1 -> layer should disappear from list_layers
    client
        .delete_layer_version()
        .layer_name("moto-glnlv-layer")
        .version_number(1)
        .send()
        .await
        .unwrap();
    let layers = client.list_layers().send().await.unwrap();
    let our_layer = layers
        .layers()
        .iter()
        .find(|l| l.layer_name() == Some("moto-glnlv-layer"));
    assert!(our_layer.is_none());

    // Publish again -> should be version 3, not 1
    let resp = client
        .publish_layer_version()
        .layer_name("moto-glnlv-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.version(), 3);

    let layers = client.list_layers().send().await.unwrap();
    let our_layer = layers
        .layers()
        .iter()
        .find(|l| l.layer_name() == Some("moto-glnlv-layer"));
    assert_eq!(
        our_layer
            .unwrap()
            .latest_matching_version()
            .unwrap()
            .version(),
        3
    );
}

// Ported from moto: test_lambda_layers.py::test_add_layer_version_permission
#[tokio::test]
async fn test_moto_add_layer_version_permission() {
    let client = make_lambda_client().await;

    let resp = client
        .publish_layer_version()
        .layer_name("moto-alvp-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .compatible_runtimes(aws_sdk_lambda::types::Runtime::Python36)
        .license_info("MIT")
        .compatible_architectures(aws_sdk_lambda::types::Architecture::X8664)
        .send()
        .await
        .unwrap();

    let version = resp.version();

    let perm_resp = client
        .add_layer_version_permission()
        .layer_name("moto-alvp-layer")
        .version_number(version)
        .statement_id("xaccount")
        .action("lambda:GetLayerVersion")
        .principal("432143214321")
        .organization_id("o-123456")
        .send()
        .await
        .unwrap();

    assert!(perm_resp.revision_id().is_some());
    assert!(perm_resp.statement().is_some());
    let stmt: serde_json::Value = serde_json::from_str(perm_resp.statement().unwrap()).unwrap();
    assert_eq!(stmt["Action"], "lambda:GetLayerVersion");
}

// Ported from moto: test_lambda_layers.py::test_get_layer_version_policy
#[tokio::test]
async fn test_moto_get_layer_version_policy() {
    let client = make_lambda_client().await;

    let resp = client
        .publish_layer_version()
        .layer_name("moto-glvp-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .compatible_runtimes(aws_sdk_lambda::types::Runtime::Python36)
        .license_info("MIT")
        .compatible_architectures(aws_sdk_lambda::types::Architecture::X8664)
        .send()
        .await
        .unwrap();

    let version = resp.version();

    client
        .add_layer_version_permission()
        .layer_name("moto-glvp-layer")
        .version_number(version)
        .statement_id("xaccount")
        .action("lambda:GetLayerVersion")
        .principal("432143214321")
        .send()
        .await
        .unwrap();

    let policy_resp = client
        .get_layer_version_policy()
        .layer_name("moto-glvp-layer")
        .version_number(version)
        .send()
        .await
        .unwrap();

    assert!(policy_resp.policy().is_some());
    assert!(policy_resp.revision_id().is_some());
    let policy: serde_json::Value = serde_json::from_str(policy_resp.policy().unwrap()).unwrap();
    assert_eq!(policy["Statement"][0]["Action"], "lambda:GetLayerVersion");
    let resource = policy["Statement"][0]["Resource"].as_str().unwrap();
    assert!(
        resource.contains("moto-glvp-layer:1"),
        "Resource should contain layer name and version"
    );
}

// Ported from moto: test_lambda_layers.py::test_remove_layer_version_permission
#[tokio::test]
async fn test_moto_remove_layer_version_permission() {
    let client = make_lambda_client().await;

    let resp = client
        .publish_layer_version()
        .layer_name("moto-rlvp-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .compatible_runtimes(aws_sdk_lambda::types::Runtime::Python36)
        .license_info("MIT")
        .compatible_architectures(aws_sdk_lambda::types::Architecture::X8664)
        .send()
        .await
        .unwrap();

    let version = resp.version();

    client
        .add_layer_version_permission()
        .layer_name("moto-rlvp-layer")
        .version_number(version)
        .statement_id("xaccount")
        .action("lambda:GetLayerVersion")
        .principal("432143214321")
        .send()
        .await
        .unwrap();

    // Verify policy exists
    client
        .get_layer_version_policy()
        .layer_name("moto-rlvp-layer")
        .version_number(version)
        .send()
        .await
        .unwrap();

    // Remove
    client
        .remove_layer_version_permission()
        .layer_name("moto-rlvp-layer")
        .version_number(version)
        .statement_id("xaccount")
        .send()
        .await
        .unwrap();

    // Verify policy is gone
    let err = client
        .get_layer_version_policy()
        .layer_name("moto-rlvp-layer")
        .version_number(version)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
    assert!(
        err_str.contains("The resource you requested does not exist"),
        "Expected correct message, got: {err_str}"
    );
}

// Ported from moto: test_lambda_layers.py::test_list_lambda_layers (simplified)
#[tokio::test]
async fn test_moto_list_layer_versions_ordering() {
    let client = make_lambda_client().await;

    // Publish 3 versions
    for _ in 0..3 {
        client
            .publish_layer_version()
            .layer_name("moto-llvo-layer")
            .content(
                aws_sdk_lambda::types::LayerVersionContentInput::builder()
                    .zip_file(Blob::new(vec![0u8; 10]))
                    .build(),
            )
            .compatible_runtimes(aws_sdk_lambda::types::Runtime::Python313)
            .license_info("MIT")
            .send()
            .await
            .unwrap();
    }

    let result = client
        .list_layer_versions()
        .layer_name("moto-llvo-layer")
        .send()
        .await
        .unwrap();

    let versions: Vec<i64> = result
        .layer_versions()
        .iter()
        .map(|v| v.version())
        .collect();
    // Should be in descending order
    assert_eq!(versions, vec![3, 2, 1]);
}

// ============================================================================
// Ported from moto: test_lambda.py (publish_version, list_versions_by_function)
// ============================================================================

// Ported from moto: test_lambda.py::test_publish_version_unknown_function
#[tokio::test]
async fn test_moto_publish_version_unknown_function() {
    let client = make_lambda_client().await;

    let err = client
        .publish_version()
        .function_name("bad_function_name")
        .description("v2")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
    assert!(
        err_str.contains("Function not found"),
        "Expected 'Function not found' message, got: {err_str}"
    );
}

// Ported from moto: test_lambda.py::test_publish (simplified)
#[tokio::test]
async fn test_moto_publish_version_and_list() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-pub-func").await;

    // Before publish: only $LATEST
    let versions = client
        .list_versions_by_function()
        .function_name("moto-pub-func")
        .send()
        .await
        .unwrap();
    assert_eq!(versions.versions().len(), 1);

    // Publish
    let resp = client
        .publish_version()
        .function_name("moto-pub-func")
        .send()
        .await
        .unwrap();

    assert!(resp.function_arn().unwrap().contains("moto-pub-func:1"));

    // After publish: $LATEST + version 1
    let versions = client
        .list_versions_by_function()
        .function_name("moto-pub-func")
        .send()
        .await
        .unwrap();
    assert_eq!(versions.versions().len(), 2);
}

// Ported from moto: test_lambda.py::test_list_versions_by_function
#[tokio::test]
async fn test_moto_list_versions_by_function() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-lvf-func").await;

    // Publish twice
    client
        .publish_version()
        .function_name("moto-lvf-func")
        .send()
        .await
        .unwrap();
    client
        .publish_version()
        .function_name("moto-lvf-func")
        .send()
        .await
        .unwrap();

    let versions = client
        .list_versions_by_function()
        .function_name("moto-lvf-func")
        .send()
        .await
        .unwrap();

    assert_eq!(versions.versions().len(), 3); // $LATEST + 1 + 2
    let arns: Vec<&str> = versions
        .versions()
        .iter()
        .map(|v| v.function_arn().unwrap_or(""))
        .collect();
    assert!(arns[0].ends_with(":$LATEST"));
    assert!(arns[1].ends_with(":1"));
    assert!(arns[2].ends_with(":2"));
}

// Ported from moto: test_lambda.py::test_list_versions_by_function_for_nonexistent_function
#[tokio::test]
async fn test_moto_list_versions_by_function_nonexistent() {
    let client = make_lambda_client().await;

    let versions = client
        .list_versions_by_function()
        .function_name("nonexistent-func")
        .send()
        .await
        .unwrap();

    assert_eq!(versions.versions().len(), 0);
}

// ============================================================================
// Ported from moto: test_lambda_eventsourcemapping.py (simplified)
// ============================================================================

// Ported from moto: test_lambda_eventsourcemapping.py::test_create_event_source_mapping (simplified)
#[tokio::test]
async fn test_moto_create_event_source_mapping_basic() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-esm-func").await;

    let resp = client
        .create_event_source_mapping()
        .function_name("moto-esm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:test-queue")
        .batch_size(1)
        .enabled(true)
        .starting_position(aws_sdk_lambda::types::EventSourcePosition::AtTimestamp)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.event_source_arn(),
        Some("arn:aws:sqs:us-east-1:123456789012:test-queue")
    );
    assert!(resp.function_arn().is_some());
    assert_eq!(resp.state(), Some("Enabled"));
    assert_eq!(resp.batch_size(), Some(1));
    assert_eq!(
        resp.starting_position().map(|s| s.as_str()),
        Some("AT_TIMESTAMP")
    );
    assert!(resp.uuid().is_some());
}

// Ported: test_lambda_eventsourcemapping - update
#[tokio::test]
async fn test_moto_update_event_source_mapping_basic() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-uesm-func").await;

    let resp = client
        .create_event_source_mapping()
        .function_name("moto-uesm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:update-queue")
        .batch_size(5)
        .enabled(true)
        .send()
        .await
        .unwrap();

    let uuid = resp.uuid().unwrap().to_string();

    let updated = client
        .update_event_source_mapping()
        .uuid(&uuid)
        .batch_size(20)
        .enabled(false)
        .send()
        .await
        .unwrap();

    assert_eq!(updated.batch_size(), Some(20));
    assert_eq!(updated.state(), Some("Disabled"));
}

// Ported: test_lambda_eventsourcemapping - delete
#[tokio::test]
async fn test_moto_delete_event_source_mapping_basic() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-desm-func").await;

    let resp = client
        .create_event_source_mapping()
        .function_name("moto-desm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:del-queue")
        .send()
        .await
        .unwrap();

    let uuid = resp.uuid().unwrap().to_string();

    client
        .delete_event_source_mapping()
        .uuid(&uuid)
        .send()
        .await
        .unwrap();

    let err = client
        .get_event_source_mapping()
        .uuid(&uuid)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported: test_lambda_eventsourcemapping - list
#[tokio::test]
async fn test_moto_list_event_source_mappings_basic() {
    let client = make_lambda_client().await;
    create_test_function(&client, "moto-lesm-func").await;

    client
        .create_event_source_mapping()
        .function_name("moto-lesm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:list-queue-1")
        .send()
        .await
        .unwrap();

    client
        .create_event_source_mapping()
        .function_name("moto-lesm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:list-queue-2")
        .send()
        .await
        .unwrap();

    let resp = client.list_event_source_mappings().send().await.unwrap();

    assert!(resp.event_source_mappings().len() >= 2);
}

// ============================================================================
// Additional tests derived from AWS documentation
// ============================================================================

#[tokio::test]
async fn test_create_function_invalid_memory_too_low() {
    let client = make_lambda_client().await;

    let result = client
        .create_function()
        .function_name("invalid-mem-low")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .memory_size(64)
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidParameterValueException"),
        "Expected InvalidParameterValueException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_function_invalid_memory_too_high() {
    let client = make_lambda_client().await;

    let result = client
        .create_function()
        .function_name("invalid-mem-high")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .memory_size(10241)
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidParameterValueException"),
        "Expected InvalidParameterValueException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_function_invalid_timeout() {
    let client = make_lambda_client().await;

    let result = client
        .create_function()
        .function_name("invalid-timeout")
        .runtime(aws_sdk_lambda::types::Runtime::Python312)
        .handler("index.handler")
        .role("arn:aws:iam::123456789012:role/lambda-role")
        .code(
            aws_sdk_lambda::types::FunctionCode::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .timeout(901)
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidParameterValueException"),
        "Expected InvalidParameterValueException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_function_configuration_nonexistent() {
    let client = make_lambda_client().await;

    let result = client
        .update_function_configuration()
        .function_name("does-not-exist")
        .description("should fail")
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_function_configuration_invalid_memory() {
    let client = make_lambda_client().await;
    create_test_function(&client, "upd-invmem-func").await;

    let result = client
        .update_function_configuration()
        .function_name("upd-invmem-func")
        .memory_size(64)
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidParameterValueException"),
        "Expected InvalidParameterValueException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_multiple_permissions_independent() {
    let client = make_lambda_client().await;
    create_test_function(&client, "multi-perm-func").await;

    client
        .add_permission()
        .function_name("multi-perm-func")
        .statement_id("stmt-a")
        .action("lambda:InvokeFunction")
        .principal("events.amazonaws.com")
        .send()
        .await
        .unwrap();

    client
        .add_permission()
        .function_name("multi-perm-func")
        .statement_id("stmt-b")
        .action("lambda:InvokeFunction")
        .principal("s3.amazonaws.com")
        .send()
        .await
        .unwrap();

    // Verify both statements are present
    let policy_resp = client
        .get_policy()
        .function_name("multi-perm-func")
        .send()
        .await
        .expect("get_policy should succeed");

    let policy_str = policy_resp.policy().unwrap();
    assert!(
        policy_str.contains("stmt-a"),
        "Policy should contain stmt-a"
    );
    assert!(
        policy_str.contains("stmt-b"),
        "Policy should contain stmt-b"
    );

    // Remove one and verify the other remains
    client
        .remove_permission()
        .function_name("multi-perm-func")
        .statement_id("stmt-a")
        .send()
        .await
        .expect("remove_permission should succeed");

    let policy_resp = client
        .get_policy()
        .function_name("multi-perm-func")
        .send()
        .await
        .expect("get_policy should still succeed after removing one statement");

    let policy_str = policy_resp.policy().unwrap();
    assert!(
        !policy_str.contains("stmt-a"),
        "Policy should no longer contain stmt-a"
    );
    assert!(
        policy_str.contains("stmt-b"),
        "Policy should still contain stmt-b"
    );
}

#[tokio::test]
async fn test_publish_version_sequential_numbering() {
    let client = make_lambda_client().await;
    create_test_function(&client, "seq-ver-func").await;

    let v1 = client
        .publish_version()
        .function_name("seq-ver-func")
        .description("first version")
        .send()
        .await
        .expect("first publish_version should succeed");

    assert_eq!(v1.version(), Some("1"));
    assert_eq!(v1.description(), Some("first version"));
    assert!(v1.function_arn().unwrap().ends_with(":1"));

    let v2 = client
        .publish_version()
        .function_name("seq-ver-func")
        .description("second version")
        .send()
        .await
        .expect("second publish_version should succeed");

    assert_eq!(v2.version(), Some("2"));
    assert_eq!(v2.description(), Some("second version"));
    assert!(v2.function_arn().unwrap().ends_with(":2"));

    let v3 = client
        .publish_version()
        .function_name("seq-ver-func")
        .send()
        .await
        .expect("third publish_version should succeed");

    assert_eq!(v3.version(), Some("3"));
}

#[tokio::test]
async fn test_layer_version_arn_format() {
    let client = make_lambda_client().await;

    let resp = client
        .publish_layer_version()
        .layer_name("arn-format-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .expect("publish_layer_version should succeed");

    let layer_arn = resp.layer_arn().expect("should have layer_arn");
    let layer_version_arn = resp
        .layer_version_arn()
        .expect("should have layer_version_arn");

    // layer ARN: arn:aws:lambda:{region}:{account}:layer:{name}
    assert!(
        layer_arn.starts_with("arn:aws:lambda:"),
        "layer_arn should start with arn:aws:lambda:"
    );
    assert!(
        layer_arn.contains(":layer:arn-format-layer"),
        "layer_arn should contain layer name"
    );

    // layer version ARN: arn:aws:lambda:{region}:{account}:layer:{name}:{version}
    assert!(
        layer_version_arn.ends_with(":1"),
        "layer_version_arn should end with version :1"
    );
    assert!(
        layer_version_arn.contains(":layer:arn-format-layer:"),
        "layer_version_arn should contain layer name"
    );
}

#[tokio::test]
async fn test_create_alias_on_nonexistent_function_fails() {
    let client = make_lambda_client().await;

    let result = client
        .create_alias()
        .function_name("no-such-func")
        .name("my-alias")
        .function_version("$LATEST")
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_function_event_invoke_config_with_destination() {
    let client = make_lambda_client().await;
    create_test_function(&client, "eic-dest-func").await;

    let dest_config = aws_sdk_lambda::types::DestinationConfig::builder()
        .on_success(
            aws_sdk_lambda::types::OnSuccess::builder()
                .destination("arn:aws:sqs:us-east-1:123456789012:success-queue")
                .build(),
        )
        .on_failure(
            aws_sdk_lambda::types::OnFailure::builder()
                .destination("arn:aws:sqs:us-east-1:123456789012:failure-queue")
                .build(),
        )
        .build();

    let resp = client
        .put_function_event_invoke_config()
        .function_name("eic-dest-func")
        .maximum_retry_attempts(2)
        .maximum_event_age_in_seconds(7200)
        .destination_config(dest_config)
        .send()
        .await
        .expect("put_function_event_invoke_config with destinations should succeed");

    assert_eq!(resp.maximum_retry_attempts(), Some(2));
    assert_eq!(resp.maximum_event_age_in_seconds(), Some(7200));

    let dest = resp
        .destination_config()
        .expect("should have destination_config");
    let on_success = dest.on_success().expect("should have on_success");
    assert_eq!(
        on_success.destination(),
        Some("arn:aws:sqs:us-east-1:123456789012:success-queue")
    );
    let on_failure = dest.on_failure().expect("should have on_failure");
    assert_eq!(
        on_failure.destination(),
        Some("arn:aws:sqs:us-east-1:123456789012:failure-queue")
    );
}

#[tokio::test]
async fn test_esm_multiple_sources_same_function() {
    let client = make_lambda_client().await;
    create_test_function(&client, "multi-esm-func").await;

    let esm1 = client
        .create_event_source_mapping()
        .function_name("multi-esm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:queue-src-1")
        .batch_size(5)
        .send()
        .await
        .expect("first create_event_source_mapping should succeed");

    let esm2 = client
        .create_event_source_mapping()
        .function_name("multi-esm-func")
        .event_source_arn("arn:aws:sqs:us-east-1:123456789012:queue-src-2")
        .batch_size(10)
        .send()
        .await
        .expect("second create_event_source_mapping should succeed");

    let uuid1 = esm1.uuid().unwrap().to_string();
    let uuid2 = esm2.uuid().unwrap().to_string();
    assert_ne!(uuid1, uuid2, "Each ESM should get a unique UUID");

    // Both retrievable individually
    let get1 = client
        .get_event_source_mapping()
        .uuid(&uuid1)
        .send()
        .await
        .expect("get first ESM should succeed");
    assert_eq!(
        get1.event_source_arn(),
        Some("arn:aws:sqs:us-east-1:123456789012:queue-src-1")
    );

    let get2 = client
        .get_event_source_mapping()
        .uuid(&uuid2)
        .send()
        .await
        .expect("get second ESM should succeed");
    assert_eq!(
        get2.event_source_arn(),
        Some("arn:aws:sqs:us-east-1:123456789012:queue-src-2")
    );

    // Both appear in list
    let list = client
        .list_event_source_mappings()
        .send()
        .await
        .expect("list_event_source_mappings should succeed");

    let uuids: Vec<&str> = list
        .event_source_mappings()
        .iter()
        .filter_map(|m| m.uuid())
        .collect();
    assert!(uuids.contains(&uuid1.as_str()), "List should contain uuid1");
    assert!(uuids.contains(&uuid2.as_str()), "List should contain uuid2");
}

#[tokio::test]
async fn test_send_durable_execution_callback_failure() {
    let client = make_lambda_client().await;

    // Sending a callback failure with a valid-looking callback ID should succeed (204 No Content)
    let result = client
        .send_durable_execution_callback_failure()
        .callback_id("cb-123456789")
        .send()
        .await;
    assert!(
        result.is_ok(),
        "send_durable_execution_callback_failure should succeed: {result:?}"
    );
}

#[tokio::test]
async fn test_send_durable_execution_callback_heartbeat() {
    let client = make_lambda_client().await;

    let result = client
        .send_durable_execution_callback_heartbeat()
        .callback_id("cb-123456789")
        .send()
        .await;
    assert!(
        result.is_ok(),
        "send_durable_execution_callback_heartbeat should succeed: {result:?}"
    );
}

#[tokio::test]
async fn test_send_durable_execution_callback_success() {
    let client = make_lambda_client().await;

    let result = client
        .send_durable_execution_callback_success()
        .callback_id("cb-123456789")
        .send()
        .await;
    assert!(
        result.is_ok(),
        "send_durable_execution_callback_success should succeed: {result:?}"
    );
}

// ========== ListFunctionUrlConfigs tests ==========

#[tokio::test]
async fn test_list_function_url_configs_empty() {
    let client = make_lambda_client().await;
    create_test_function(&client, "list-url-func-empty").await;

    let resp = client
        .list_function_url_configs()
        .function_name("list-url-func-empty")
        .send()
        .await
        .expect("list_function_url_configs should succeed");

    assert_eq!(resp.function_url_configs().len(), 0);
}

#[tokio::test]
async fn test_list_function_url_configs_after_create() {
    let client = make_lambda_client().await;
    create_test_function(&client, "list-url-func-create").await;

    client
        .create_function_url_config()
        .function_name("list-url-func-create")
        .auth_type(aws_sdk_lambda::types::FunctionUrlAuthType::None)
        .send()
        .await
        .expect("create_function_url_config should succeed");

    let resp = client
        .list_function_url_configs()
        .function_name("list-url-func-create")
        .send()
        .await
        .expect("list_function_url_configs should succeed");

    assert_eq!(resp.function_url_configs().len(), 1);
    let config = &resp.function_url_configs()[0];
    assert_eq!(
        config.auth_type(),
        &aws_sdk_lambda::types::FunctionUrlAuthType::None
    );
    assert!(!config.function_url().is_empty());
}

// ========== GetFunctionConfiguration tests ==========

#[tokio::test]
async fn test_get_function_configuration() {
    let client = make_lambda_client().await;
    create_test_function(&client, "config-func").await;

    let resp = client
        .get_function_configuration()
        .function_name("config-func")
        .send()
        .await
        .expect("get_function_configuration should succeed");

    assert_eq!(resp.function_name(), Some("config-func"));
    assert_eq!(resp.handler(), Some("index.handler"));
    assert!(resp.function_arn().is_some());
    assert_eq!(
        resp.runtime(),
        Some(&aws_sdk_lambda::types::Runtime::Python312)
    );
}

#[tokio::test]
async fn test_get_function_configuration_nonexistent() {
    let client = make_lambda_client().await;

    let err = client
        .get_function_configuration()
        .function_name("no-such-func")
        .send()
        .await;

    assert!(err.is_err(), "should fail for nonexistent function");
}

// ========== CodeSigningConfig CRUD tests ==========

#[tokio::test]
async fn test_create_and_get_code_signing_config() {
    let client = make_lambda_client().await;

    let create_resp = client
        .create_code_signing_config()
        .description("test CSC")
        .allowed_publishers(
            aws_sdk_lambda::types::AllowedPublishers::builder()
                .signing_profile_version_arns(
                    "arn:aws:signer:us-east-1:123456789012:/signing-profiles/MyProfile",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_code_signing_config should succeed");

    let csc = create_resp
        .code_signing_config()
        .expect("should have code_signing_config");
    let csc_arn = csc.code_signing_config_arn();
    assert!(csc_arn.contains("csc-"));
    assert_eq!(csc.description(), Some("test CSC"));

    // Extract the CSC ID from the ARN
    let csc_id = csc.code_signing_config_id();

    let get_resp = client
        .get_code_signing_config()
        .code_signing_config_arn(csc_arn)
        .send()
        .await
        .expect("get_code_signing_config should succeed");

    let got = get_resp
        .code_signing_config()
        .expect("should have code_signing_config");
    assert_eq!(got.code_signing_config_id(), csc_id);
    assert_eq!(got.description(), Some("test CSC"));
}

#[tokio::test]
async fn test_list_code_signing_configs() {
    let client = make_lambda_client().await;

    // Create two configs
    client
        .create_code_signing_config()
        .allowed_publishers(
            aws_sdk_lambda::types::AllowedPublishers::builder()
                .signing_profile_version_arns(
                    "arn:aws:signer:us-east-1:123456789012:/signing-profiles/P1",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_code_signing_config()
        .allowed_publishers(
            aws_sdk_lambda::types::AllowedPublishers::builder()
                .signing_profile_version_arns(
                    "arn:aws:signer:us-east-1:123456789012:/signing-profiles/P2",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_code_signing_configs()
        .send()
        .await
        .expect("list_code_signing_configs should succeed");

    let configs = resp.code_signing_configs();
    assert!(configs.len() >= 2, "should list at least 2 configs");
}

#[tokio::test]
async fn test_delete_code_signing_config() {
    let client = make_lambda_client().await;

    let create_resp = client
        .create_code_signing_config()
        .allowed_publishers(
            aws_sdk_lambda::types::AllowedPublishers::builder()
                .signing_profile_version_arns(
                    "arn:aws:signer:us-east-1:123456789012:/signing-profiles/P1",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let csc_arn = create_resp
        .code_signing_config()
        .unwrap()
        .code_signing_config_arn()
        .to_string();

    client
        .delete_code_signing_config()
        .code_signing_config_arn(&csc_arn)
        .send()
        .await
        .expect("delete_code_signing_config should succeed");

    // Verify it's gone
    let err = client
        .get_code_signing_config()
        .code_signing_config_arn(&csc_arn)
        .send()
        .await;

    assert!(err.is_err(), "should fail after deletion");
}

#[tokio::test]
async fn test_update_code_signing_config() {
    let client = make_lambda_client().await;

    let create_resp = client
        .create_code_signing_config()
        .description("original desc")
        .allowed_publishers(
            aws_sdk_lambda::types::AllowedPublishers::builder()
                .signing_profile_version_arns(
                    "arn:aws:signer:us-east-1:123456789012:/signing-profiles/P1",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let csc_arn = create_resp
        .code_signing_config()
        .unwrap()
        .code_signing_config_arn()
        .to_string();

    // Extract the CSC ID from the ARN for the update call
    let csc_id = csc_arn.rsplit(':').next().unwrap_or(&csc_arn).to_string();

    let update_resp = client
        .update_code_signing_config()
        .code_signing_config_arn(&csc_arn)
        .description("updated desc")
        .send()
        .await
        .expect("update_code_signing_config should succeed");

    let updated = update_resp
        .code_signing_config()
        .expect("should have code_signing_config");
    assert_eq!(updated.description(), Some("updated desc"));
    assert_eq!(updated.code_signing_config_id(), csc_id.as_str());
}

// ========== InvokeAsync test ==========

#[tokio::test]
#[allow(deprecated)]
async fn test_invoke_async() {
    let client = make_lambda_client().await;
    create_test_function(&client, "invoke-async-func").await;

    let resp = client
        .invoke_async()
        .function_name("invoke-async-func")
        .invoke_args(aws_sdk_lambda::primitives::ByteStream::from_static(b"{}"))
        .send()
        .await
        .expect("invoke_async should succeed");

    assert_eq!(resp.status(), 202);
}

// ========== PutFunctionCodeSigningConfig / DeleteFunctionCodeSigningConfig tests ==========

#[tokio::test]
async fn test_put_and_get_function_code_signing_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "csc-put-func").await;

    // Create a code signing config first
    let csc_resp = client
        .create_code_signing_config()
        .allowed_publishers(
            aws_sdk_lambda::types::AllowedPublishers::builder()
                .signing_profile_version_arns(
                    "arn:aws:signer:us-east-1:123456789012:/signing-profiles/P1",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let csc_arn = csc_resp
        .code_signing_config()
        .unwrap()
        .code_signing_config_arn()
        .to_string();

    // Put the code signing config on the function
    let put_resp = client
        .put_function_code_signing_config()
        .function_name("csc-put-func")
        .code_signing_config_arn(&csc_arn)
        .send()
        .await
        .expect("put_function_code_signing_config should succeed");

    assert_eq!(put_resp.code_signing_config_arn(), csc_arn.as_str());
    assert_eq!(put_resp.function_name(), "csc-put-func");

    // Get it back
    let get_resp = client
        .get_function_code_signing_config()
        .function_name("csc-put-func")
        .send()
        .await
        .expect("get_function_code_signing_config should succeed");

    assert_eq!(get_resp.code_signing_config_arn(), csc_arn.as_str());
}

#[tokio::test]
async fn test_delete_function_code_signing_config() {
    let client = make_lambda_client().await;
    create_test_function(&client, "csc-del-func").await;

    // Create and put a code signing config
    let csc_resp = client
        .create_code_signing_config()
        .allowed_publishers(
            aws_sdk_lambda::types::AllowedPublishers::builder()
                .signing_profile_version_arns(
                    "arn:aws:signer:us-east-1:123456789012:/signing-profiles/P1",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let csc_arn = csc_resp
        .code_signing_config()
        .unwrap()
        .code_signing_config_arn()
        .to_string();

    client
        .put_function_code_signing_config()
        .function_name("csc-del-func")
        .code_signing_config_arn(&csc_arn)
        .send()
        .await
        .unwrap();

    // Delete the function code signing config
    client
        .delete_function_code_signing_config()
        .function_name("csc-del-func")
        .send()
        .await
        .expect("delete_function_code_signing_config should succeed");

    // Verify it's gone - get should return empty ARN or error
    let get_resp = client
        .get_function_code_signing_config()
        .function_name("csc-del-func")
        .send()
        .await;
    // After deletion the config is removed
    if let Ok(resp) = get_resp {
        assert!(
            resp.code_signing_config_arn().is_empty(),
            "code_signing_config_arn should be empty after deletion"
        );
    }
}

// ========== GetLayerVersionByArn test ==========

#[tokio::test]
async fn test_get_layer_version_by_arn() {
    let client = make_lambda_client().await;

    // First publish a layer version
    let pub_resp = client
        .publish_layer_version()
        .layer_name("by-arn-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .expect("publish_layer_version should succeed");

    let layer_version_arn = pub_resp
        .layer_version_arn()
        .expect("should have layer_version_arn")
        .to_string();

    // Get the layer version by ARN
    let get_resp = client
        .get_layer_version_by_arn()
        .arn(&layer_version_arn)
        .send()
        .await
        .expect("get_layer_version_by_arn should succeed");

    assert_eq!(
        get_resp.layer_version_arn(),
        Some(layer_version_arn.as_str())
    );
}

// ========== InvokeAsync edge-case tests ==========

#[tokio::test]
#[allow(deprecated)]
async fn test_invoke_async_nonexistent_function() {
    let client = make_lambda_client().await;

    let result = client
        .invoke_async()
        .function_name("nonexistent-func")
        .invoke_args(aws_sdk_lambda::primitives::ByteStream::from_static(b"{}"))
        .send()
        .await;

    assert!(
        result.is_err(),
        "invoke_async on nonexistent function should fail"
    );
}

// ========== PutFunctionCodeSigningConfig edge-case tests ==========

#[tokio::test]
async fn test_put_function_code_signing_config_nonexistent_function() {
    let client = make_lambda_client().await;

    let result = client
        .put_function_code_signing_config()
        .function_name("nonexistent-func")
        .code_signing_config_arn(
            "arn:aws:lambda:us-east-1:123456789012:code-signing-config:csc-fake",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "put_function_code_signing_config on nonexistent function should fail"
    );
}

#[tokio::test]
async fn test_delete_function_code_signing_config_nonexistent_function() {
    let client = make_lambda_client().await;

    let result = client
        .delete_function_code_signing_config()
        .function_name("nonexistent-func")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_function_code_signing_config on nonexistent function should fail"
    );
}

#[tokio::test]
async fn test_get_function_code_signing_config_no_config_set() {
    let client = make_lambda_client().await;
    create_test_function(&client, "csc-none-func").await;

    // Get code signing config for function that has none set
    let resp = client
        .get_function_code_signing_config()
        .function_name("csc-none-func")
        .send()
        .await
        .expect("get_function_code_signing_config should succeed even with no config");

    assert!(
        resp.code_signing_config_arn().is_empty(),
        "code_signing_config_arn should be empty when none is set"
    );
}

// ========== GetLayerVersionByArn edge-case tests ==========

#[tokio::test]
async fn test_get_layer_version_by_arn_nonexistent() {
    let client = make_lambda_client().await;

    let result = client
        .get_layer_version_by_arn()
        .arn("arn:aws:lambda:us-east-1:123456789012:layer:nonexistent-layer:1")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_layer_version_by_arn with nonexistent ARN should fail"
    );
}

// ========== Code signing config lifecycle test ==========

#[tokio::test]
async fn test_code_signing_config_full_lifecycle() {
    let client = make_lambda_client().await;
    create_test_function(&client, "csc-lifecycle-func").await;

    // Step 1: Create a code signing config
    let csc_resp = client
        .create_code_signing_config()
        .description("lifecycle test")
        .allowed_publishers(
            aws_sdk_lambda::types::AllowedPublishers::builder()
                .signing_profile_version_arns(
                    "arn:aws:signer:us-east-1:123456789012:/signing-profiles/P1",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let csc_arn = csc_resp
        .code_signing_config()
        .unwrap()
        .code_signing_config_arn()
        .to_string();

    // Step 2: Put it on the function
    client
        .put_function_code_signing_config()
        .function_name("csc-lifecycle-func")
        .code_signing_config_arn(&csc_arn)
        .send()
        .await
        .expect("put should succeed");

    // Step 3: Verify it's set
    let get_resp = client
        .get_function_code_signing_config()
        .function_name("csc-lifecycle-func")
        .send()
        .await
        .expect("get should succeed");

    assert_eq!(get_resp.code_signing_config_arn(), csc_arn.as_str());

    // Step 4: Delete it from the function
    client
        .delete_function_code_signing_config()
        .function_name("csc-lifecycle-func")
        .send()
        .await
        .expect("delete should succeed");

    // Step 5: Verify it's gone
    let get_after = client
        .get_function_code_signing_config()
        .function_name("csc-lifecycle-func")
        .send()
        .await;

    if let Ok(resp) = get_after {
        assert!(
            resp.code_signing_config_arn().is_empty(),
            "should be empty after removal"
        );
    }

    // Step 6: Delete the CSC itself
    client
        .delete_code_signing_config()
        .code_signing_config_arn(&csc_arn)
        .send()
        .await
        .expect("delete CSC should succeed");
}

// ========== Multiple layer versions with get-by-arn test ==========

#[tokio::test]
async fn test_get_layer_version_by_arn_multiple_versions() {
    let client = make_lambda_client().await;

    // Publish two layer versions
    let pub_resp1 = client
        .publish_layer_version()
        .layer_name("multi-ver-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![0u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let arn1 = pub_resp1.layer_version_arn().unwrap().to_string();
    assert_eq!(pub_resp1.version(), 1);

    let pub_resp2 = client
        .publish_layer_version()
        .layer_name("multi-ver-layer")
        .content(
            aws_sdk_lambda::types::LayerVersionContentInput::builder()
                .zip_file(Blob::new(vec![1u8; 10]))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let arn2 = pub_resp2.layer_version_arn().unwrap().to_string();
    assert_eq!(pub_resp2.version(), 2);
    assert_ne!(arn1, arn2);

    // Get each version by ARN and verify they're different
    let get1 = client
        .get_layer_version_by_arn()
        .arn(&arn1)
        .send()
        .await
        .expect("get version 1 by ARN should succeed");

    assert_eq!(get1.version(), 1);

    let get2 = client
        .get_layer_version_by_arn()
        .arn(&arn2)
        .send()
        .await
        .expect("get version 2 by ARN should succeed");

    assert_eq!(get2.version(), 2);
}

// ========== Coverage for FIX(terraform-e2e) handler fixes ==========

#[tokio::test]
async fn test_list_tags_with_code_signing_config_arn() {
    let client = make_lambda_client().await;

    // Create a code-signing config so its ARN exists in state
    let create_resp = client
        .create_code_signing_config()
        .description("csc for list-tags test")
        .allowed_publishers(
            aws_sdk_lambda::types::AllowedPublishers::builder()
                .signing_profile_version_arns(
                    "arn:aws:signer:us-east-1:123456789012:/signing-profiles/MyProfile",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_code_signing_config should succeed");

    let csc = create_resp
        .code_signing_config()
        .expect("should have code_signing_config");
    let csc_arn = csc.code_signing_config_arn().to_string();

    // ListTags on a code-signing-config ARN should succeed and return empty tags
    let resp = client
        .list_tags()
        .resource(&csc_arn)
        .send()
        .await
        .expect("list_tags on code-signing-config ARN should succeed");

    let tags = resp.tags().unwrap();
    assert!(tags.is_empty(), "code-signing-config tags should be empty");
}

#[tokio::test]
async fn test_list_tags_with_nonexistent_code_signing_config_arn() {
    let client = make_lambda_client().await;

    // ListTags on a code-signing-config ARN that does not exist should return an error
    let fake_arn = "arn:aws:lambda:us-east-1:123456789012:code-signing-config:csc-nonexistent";
    let err = client.list_tags().resource(fake_arn).send().await;

    assert!(
        err.is_err(),
        "list_tags on nonexistent code-signing-config ARN should fail"
    );
}
