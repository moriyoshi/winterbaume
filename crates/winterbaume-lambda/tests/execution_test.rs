//! Opt-in execution of deployed function code on `Invoke` ( issue #48 ).
//!
//! These tests exercise the service side of execution: what reaches a
//! [`LambdaExecutor`], and how its outcome or refusal is answered.  They use
//! an in-process scripted executor, so they run everywhere.  Running real
//! functions in microVMs is tested in the `winterbaume-lambda-vm` crate.

use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use aws_sdk_lambda::config::interceptors::BeforeDeserializationInterceptorContextRef;
use aws_sdk_lambda::config::{BehaviorVersion, ConfigBag, Intercept, RuntimeComponents};
use aws_sdk_lambda::error::BoxError;
use aws_sdk_lambda::operation::invoke::InvokeOutput;
use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::types::{
    Architecture, Environment, FunctionCode, InvocationType, LogType, PackageType, Runtime,
};
use base64::Engine as _;
use bytes::Bytes;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use winterbaume_core::MockAws;
use winterbaume_lambda::execution::{
    ARCHITECTURE_SUBSTITUTED_HEADER, BoxFuture, EXECUTION_EXECUTED, EXECUTION_HEADER,
    EXECUTION_STUB, STUB_REASON_HEADER,
};
use winterbaume_lambda::{
    FunctionTarget, InvocationOutcome, InvokeRequest, LambdaExecutor, LambdaService, StubReason,
};

const ROLE: &str = "arn:aws:iam::123456789012:role/lambda-role";

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

/// What the scripted executor saw.
#[derive(Debug, Clone)]
struct Seen {
    target: FunctionTarget,
    request: InvokeRequest,
    code: Bytes,
}

/// An executor whose functions do what their handler string says:
///
/// - `echo`: return a JSON summary of the target and the request;
/// - `error`: fail with an `Unhandled` error document;
/// - `refuse`: refuse to run in `invoke` ( `prepare` accepts );
/// - `sleep`: wait for the function's timeout, so that a dropped request
///   can be observed.
///
/// A runtime named in `refuse_runtimes` is refused by `prepare`.
#[derive(Default)]
struct Scripted {
    seen: Mutex<Vec<Seen>>,
    retired: Mutex<Vec<String>>,
    prepared: Mutex<usize>,
    refuse_runtimes: HashMap<String, StubReason>,
    substitute: Option<String>,
    dropped: Arc<Mutex<usize>>,
}

struct CountOnDrop(Arc<Mutex<usize>>);

impl Drop for CountOnDrop {
    fn drop(&mut self) {
        *self.0.lock().unwrap() += 1;
    }
}

impl LambdaExecutor for Scripted {
    fn prepare<'a>(&'a self, target: &'a FunctionTarget) -> BoxFuture<'a, Result<(), StubReason>> {
        Box::pin(async move {
            *self.prepared.lock().unwrap() += 1;
            match self.refuse_runtimes.get(&target.runtime) {
                Some(reason) => Err(*reason),
                None => Ok(()),
            }
        })
    }

    fn invoke<'a>(
        &'a self,
        target: FunctionTarget,
        request: InvokeRequest,
    ) -> BoxFuture<'a, Result<InvocationOutcome, StubReason>> {
        Box::pin(async move {
            let code = target.code.load().await.expect("the package is stored");
            self.seen.lock().unwrap().push(Seen {
                target: target.clone(),
                request: request.clone(),
                code: code.clone(),
            });
            let log = format!("log of {}\n", request.request_id).into_bytes();
            let outcome = match target.handler.as_str() {
                "error" => InvocationOutcome::unhandled(
                    Bytes::from(json!({"errorType": "Error", "errorMessage": "boom"}).to_string()),
                    log,
                ),
                "refuse" => return Err(StubReason::VmUnavailable),
                "sleep" => {
                    let _guard = CountOnDrop(self.dropped.clone());
                    tokio::time::sleep(target.timeout).await;
                    std::mem::forget(_guard);
                    InvocationOutcome::success(Bytes::from_static(b"\"slept\""), log)
                }
                _ => InvocationOutcome::success(
                    Bytes::from(
                        json!({
                            "event": serde_json::from_slice::<Value>(&request.payload).unwrap(),
                            "function": target.function_name,
                            "version": target.function_version,
                            "invokedArn": request.invoked_function_arn,
                            "runtime": target.runtime,
                            "handler": target.handler,
                            "memory": target.memory_size,
                            "timeoutMs": target.timeout.as_millis() as u64,
                            "environment": target.environment,
                            "architecture": target.architecture,
                            "logGroup": target.log_group,
                            "codeSha256": target.code.sha256(),
                            "code": String::from_utf8_lossy(&code),
                            "clientContext": request.client_context,
                            "asynchronous": request.asynchronous,
                        })
                        .to_string(),
                    ),
                    log,
                ),
            };
            Ok(outcome.with_architecture_substituted(self.substitute.clone()))
        })
    }

    fn retire<'a>(&'a self, function_arn: &'a str) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            self.retired.lock().unwrap().push(function_arn.to_string());
        })
    }
}

/// Records the headers of the most recent HTTP response, which the typed SDK
/// outputs do not expose.
#[derive(Debug, Clone, Default)]
struct ResponseHeaders(Arc<Mutex<HashMap<String, String>>>);

impl ResponseHeaders {
    fn get(&self, name: &str) -> Option<String> {
        self.0.lock().unwrap().get(name).cloned()
    }
}

impl Intercept for ResponseHeaders {
    fn name(&self) -> &'static str {
        "ResponseHeaders"
    }

    fn read_before_deserialization(
        &self,
        context: &BeforeDeserializationInterceptorContextRef<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let mut headers = self.0.lock().unwrap();
        headers.clear();
        for (name, value) in context.response().headers().iter() {
            headers.insert(name.to_ascii_lowercase(), value.to_string());
        }
        Ok(())
    }
}

async fn client_for(service: LambdaService) -> (aws_sdk_lambda::Client, ResponseHeaders) {
    let mock = MockAws::builder().with_service(service).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lambda::config::Region::new("us-east-1"))
        .load()
        .await;
    let headers = ResponseHeaders::default();
    let config = aws_sdk_lambda::config::Builder::from(&config)
        .interceptor(headers.clone())
        .build();
    (aws_sdk_lambda::Client::from_conf(config), headers)
}

/// A client whose service executes through `executor`.
async fn executing_client(executor: Arc<Scripted>) -> (aws_sdk_lambda::Client, ResponseHeaders) {
    client_for(LambdaService::new().with_code_execution(executor)).await
}

fn zip_of(files: &[(&str, &str)]) -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    for (name, content) in files {
        writer
            .start_file(*name, zip::write::SimpleFileOptions::default())
            .unwrap();
        writer.write_all(content.as_bytes()).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

fn sha256_base64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(Sha256::digest(bytes))
}

async fn create(
    client: &aws_sdk_lambda::Client,
    name: &str,
    handler: &str,
    code: &[u8],
    environment: &[(&str, &str)],
) {
    let mut env = Environment::builder();
    for (key, value) in environment {
        env = env.variables(*key, *value);
    }
    client
        .create_function()
        .function_name(name)
        .runtime(Runtime::Nodejs22x)
        .handler(handler)
        .role(ROLE)
        .timeout(9)
        .memory_size(256)
        .environment(env.build())
        .code(FunctionCode::builder().zip_file(Blob::new(code)).build())
        .send()
        .await
        .expect("create_function should succeed");
}

fn payload_json(output: &InvokeOutput) -> Value {
    let bytes = output.payload().expect("payload present").as_ref();
    serde_json::from_slice(bytes).unwrap_or_else(|e| {
        panic!(
            "payload is not JSON ( {e} ): {}",
            String::from_utf8_lossy(bytes)
        )
    })
}

async fn invoke(client: &aws_sdk_lambda::Client, name: &str, event: Value) -> InvokeOutput {
    client
        .invoke()
        .function_name(name)
        .payload(Blob::new(event.to_string()))
        .send()
        .await
        .expect("invoke should succeed")
}

fn assert_stub(headers: &ResponseHeaders, reason: &str) {
    assert_eq!(
        headers.get(EXECUTION_HEADER).as_deref(),
        Some(EXECUTION_STUB)
    );
    assert_eq!(headers.get(STUB_REASON_HEADER).as_deref(), Some(reason));
}

fn assert_executed(headers: &ResponseHeaders) {
    assert_eq!(
        headers.get(EXECUTION_HEADER).as_deref(),
        Some(EXECUTION_EXECUTED)
    );
    assert_eq!(headers.get(STUB_REASON_HEADER), None);
}

// ---------------------------------------------------------------------------
// Running through the executor
// ---------------------------------------------------------------------------

#[tokio::test]
async fn invoke_hands_the_function_to_the_executor() {
    let executor = Arc::new(Scripted::default());
    let (client, headers) = executing_client(executor.clone()).await;
    let code = zip_of(&[("index.mjs", "export const handler = () => 1;")]);
    create(&client, "fn-a", "echo", &code, &[("GREETING", "hi")]).await;

    let output = invoke(&client, "fn-a", json!({"n": 1})).await;
    assert_eq!(output.status_code(), 200);
    assert_eq!(output.executed_version(), Some("$LATEST"));
    assert_eq!(output.function_error(), None);
    assert_executed(&headers);

    let result = payload_json(&output);
    assert_eq!(result["event"], json!({"n": 1}));
    assert_eq!(result["function"], "fn-a");
    assert_eq!(result["version"], "$LATEST");
    assert_eq!(
        result["invokedArn"],
        "arn:aws:lambda:us-east-1:123456789012:function:fn-a"
    );
    assert_eq!(result["runtime"], "nodejs22.x");
    assert_eq!(result["memory"], 256);
    assert_eq!(result["timeoutMs"], 9000);
    assert_eq!(result["environment"], json!({"GREETING": "hi"}));
    assert_eq!(result["architecture"], "x86_64");
    assert_eq!(result["logGroup"], "/aws/lambda/fn-a");
    assert_eq!(result["codeSha256"], sha256_base64(&code));
    assert_eq!(
        result["code"].as_str().unwrap().as_bytes(),
        String::from_utf8_lossy(&code).as_bytes()
    );
    assert_eq!(result["asynchronous"], false);
    assert_eq!(*executor.prepared.lock().unwrap(), 1);
}

#[tokio::test]
async fn an_empty_payload_reaches_the_function_as_an_empty_object() {
    let executor = Arc::new(Scripted::default());
    let (client, _) = executing_client(executor.clone()).await;
    create(&client, "fn-empty", "echo", &zip_of(&[("a", "b")]), &[]).await;
    let output = client
        .invoke()
        .function_name("fn-empty")
        .send()
        .await
        .unwrap();
    assert_eq!(payload_json(&output)["event"], json!({}));
}

#[tokio::test]
async fn function_errors_and_log_tail() {
    let executor = Arc::new(Scripted::default());
    let (client, headers) = executing_client(executor.clone()).await;
    create(&client, "fn-err", "error", &zip_of(&[("a", "b")]), &[]).await;

    let output = client
        .invoke()
        .function_name("fn-err")
        .log_type(LogType::Tail)
        .send()
        .await
        .unwrap();
    assert_eq!(output.status_code(), 200, "function errors are HTTP 200");
    assert_eq!(output.function_error(), Some("Unhandled"));
    assert_eq!(payload_json(&output)["errorMessage"], "boom");
    assert_executed(&headers);
    let tail = base64::engine::general_purpose::STANDARD
        .decode(output.log_result().expect("LogType Tail returns the log"))
        .unwrap();
    let request_id = executor.seen.lock().unwrap()[0].request.request_id.clone();
    assert_eq!(
        String::from_utf8(tail).unwrap(),
        format!("log of {request_id}\n")
    );

    // Without `LogType: Tail` there is no log.
    let output = invoke(&client, "fn-err", json!({})).await;
    assert_eq!(output.log_result(), None);
}

#[tokio::test]
async fn invalid_json_payload_is_rejected_before_running() {
    let executor = Arc::new(Scripted::default());
    let (client, _) = executing_client(executor.clone()).await;
    create(&client, "fn-json", "echo", &zip_of(&[("a", "b")]), &[]).await;
    let err = client
        .invoke()
        .function_name("fn-json")
        .payload(Blob::new("{not json"))
        .send()
        .await
        .expect_err("invalid JSON is rejected");
    assert_eq!(
        err.into_service_error().meta().code(),
        Some("InvalidRequestContentException")
    );
    assert!(executor.seen.lock().unwrap().is_empty());
}

#[tokio::test]
async fn versions_and_aliases_keep_running_the_code_and_configuration_they_were_published_with() {
    let executor = Arc::new(Scripted::default());
    let (client, _) = executing_client(executor.clone()).await;
    let first = zip_of(&[("v", "first")]);
    let second = zip_of(&[("v", "second revision")]);
    create(&client, "fn-v", "echo", &first, &[("STAGE", "one")]).await;
    client
        .publish_version()
        .function_name("fn-v")
        .send()
        .await
        .unwrap();
    client
        .create_alias()
        .function_name("fn-v")
        .name("live")
        .function_version("1")
        .send()
        .await
        .unwrap();
    client
        .update_function_code()
        .function_name("fn-v")
        .zip_file(Blob::new(second.clone()))
        .send()
        .await
        .unwrap();
    client
        .update_function_configuration()
        .function_name("fn-v")
        .timeout(30)
        .environment(Environment::builder().variables("STAGE", "two").build())
        .send()
        .await
        .unwrap();

    let latest = payload_json(&invoke(&client, "fn-v", json!({})).await);
    assert_eq!(latest["codeSha256"], sha256_base64(&second));
    assert_eq!(latest["environment"]["STAGE"], "two");
    assert_eq!(latest["timeoutMs"], 30000);

    for qualifier in ["1", "live"] {
        let output = client
            .invoke()
            .function_name("fn-v")
            .qualifier(qualifier)
            .send()
            .await
            .unwrap();
        assert_eq!(output.executed_version(), Some("1"));
        let pinned = payload_json(&output);
        assert_eq!(pinned["version"], "1");
        assert_eq!(pinned["codeSha256"], sha256_base64(&first));
        assert_eq!(pinned["environment"]["STAGE"], "one");
        assert_eq!(pinned["timeoutMs"], 9000);
        assert_eq!(
            pinned["invokedArn"],
            format!("arn:aws:lambda:us-east-1:123456789012:function:fn-v:{qualifier}")
        );
    }
}

#[tokio::test]
async fn code_and_configuration_changes_retire_warm_state() {
    let executor = Arc::new(Scripted::default());
    let (client, _) = executing_client(executor.clone()).await;
    create(&client, "fn-r", "echo", &zip_of(&[("a", "1")]), &[]).await;
    let arn = "arn:aws:lambda:us-east-1:123456789012:function:fn-r";

    client
        .update_function_code()
        .function_name("fn-r")
        .zip_file(Blob::new(zip_of(&[("a", "2")])))
        .send()
        .await
        .unwrap();
    client
        .update_function_configuration()
        .function_name("fn-r")
        .timeout(5)
        .send()
        .await
        .unwrap();
    client
        .delete_function()
        .function_name("fn-r")
        .send()
        .await
        .unwrap();
    assert_eq!(*executor.retired.lock().unwrap(), vec![arn, arn, arn]);
}

#[tokio::test]
async fn architecture_and_logging_configuration_reach_the_executor() {
    let executor = Arc::new(Scripted {
        substitute: Some("arm64->x86_64".to_string()),
        ..Default::default()
    });
    let (client, headers) = executing_client(executor.clone()).await;
    client
        .create_function()
        .function_name("fn-arm")
        .runtime(Runtime::Nodejs22x)
        .handler("echo")
        .role(ROLE)
        .architectures(Architecture::Arm64)
        .logging_config(
            aws_sdk_lambda::types::LoggingConfig::builder()
                .log_group("/custom/group")
                .build(),
        )
        .code(
            FunctionCode::builder()
                .zip_file(Blob::new(zip_of(&[("a", "b")])))
                .build(),
        )
        .send()
        .await
        .unwrap();
    let result = payload_json(&invoke(&client, "fn-arm", json!({})).await);
    assert_eq!(result["architecture"], "arm64");
    assert_eq!(result["logGroup"], "/custom/group");
    assert_eq!(
        headers.get(ARCHITECTURE_SUBSTITUTED_HEADER).as_deref(),
        Some("arm64->x86_64")
    );
}

#[tokio::test]
async fn client_context_is_passed_for_synchronous_invocations_only() {
    let executor = Arc::new(Scripted::default());
    let (client, _) = executing_client(executor.clone()).await;
    create(&client, "fn-cc", "echo", &zip_of(&[("a", "b")]), &[]).await;
    let context = base64::engine::general_purpose::STANDARD.encode(r#"{"custom":{"k":"v"}}"#);

    let output = client
        .invoke()
        .function_name("fn-cc")
        .client_context(&context)
        .send()
        .await
        .unwrap();
    assert_eq!(payload_json(&output)["clientContext"], context);

    client
        .invoke()
        .function_name("fn-cc")
        .client_context(&context)
        .invocation_type(InvocationType::Event)
        .send()
        .await
        .unwrap();
    wait_for(|| executor.seen.lock().unwrap().len() == 2).await;
    let seen = executor.seen.lock().unwrap();
    assert!(seen[1].request.asynchronous);
    assert_eq!(seen[1].request.client_context, None);
}

async fn wait_for(mut condition: impl FnMut() -> bool) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while !condition() {
        assert!(
            tokio::time::Instant::now() < deadline,
            "condition not reached in time"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[tokio::test]
async fn event_invocations_return_202_and_run_in_the_background() {
    let executor = Arc::new(Scripted::default());
    let (client, headers) = executing_client(executor.clone()).await;
    create(&client, "fn-ev", "echo", &zip_of(&[("a", "b")]), &[]).await;
    let output = client
        .invoke()
        .function_name("fn-ev")
        .invocation_type(InvocationType::Event)
        .payload(Blob::new(r#"{"later":true}"#))
        .send()
        .await
        .unwrap();
    assert_eq!(output.status_code(), 202);
    assert!(output.payload().is_none_or(|p| p.as_ref().is_empty()));
    assert_executed(&headers);
    wait_for(|| executor.seen.lock().unwrap().len() == 1).await;
    assert_eq!(
        executor.seen.lock().unwrap()[0].request.payload,
        Bytes::from_static(br#"{"later":true}"#)
    );
}

#[tokio::test]
async fn dry_run_returns_204_without_running() {
    let executor = Arc::new(Scripted::default());
    let (client, _) = executing_client(executor.clone()).await;
    create(&client, "fn-dry", "echo", &zip_of(&[("a", "b")]), &[]).await;
    let output = client
        .invoke()
        .function_name("fn-dry")
        .invocation_type(InvocationType::DryRun)
        .send()
        .await
        .unwrap();
    assert_eq!(output.status_code(), 204);
    assert_eq!(output.executed_version(), Some("$LATEST"));
    assert_eq!(*executor.prepared.lock().unwrap(), 0);
    assert!(executor.seen.lock().unwrap().is_empty());
}

#[tokio::test]
async fn abandoned_invocation_drops_the_executor_future() {
    let executor = Arc::new(Scripted::default());
    let dropped = executor.dropped.clone();
    let service = LambdaService::new().with_code_execution(executor.clone());
    let (client, _) = client_for(service).await;
    create(&client, "fn-sleep", "sleep", &zip_of(&[("a", "b")]), &[]).await;
    let pending = tokio::spawn({
        let client = client.clone();
        async move { client.invoke().function_name("fn-sleep").send().await }
    });
    wait_for(|| executor.seen.lock().unwrap().len() == 1).await;
    pending.abort();
    let _ = pending.await;
    wait_for(|| *dropped.lock().unwrap() == 1).await;
}

// ---------------------------------------------------------------------------
// The marked stub
// ---------------------------------------------------------------------------

#[tokio::test]
async fn execution_disabled_returns_the_marked_stub() {
    let (client, headers) = client_for(LambdaService::new()).await;
    create(&client, "fn-off", "echo", &zip_of(&[("a", "b")]), &[]).await;

    let output = invoke(&client, "fn-off", json!({"x": 1})).await;
    assert_eq!(output.status_code(), 200);
    assert_eq!(output.payload().unwrap().as_ref(), b"{}");
    assert_stub(&headers, "execution-disabled");

    // Every invocation type is answered by the stub; DryRun included.
    for invocation_type in [InvocationType::Event, InvocationType::DryRun] {
        let output = client
            .invoke()
            .function_name("fn-off")
            .invocation_type(invocation_type)
            .send()
            .await
            .unwrap();
        assert_eq!(output.status_code(), 200);
        assert_stub(&headers, "execution-disabled");
    }
}

#[tokio::test]
async fn functions_that_cannot_run_get_the_marked_stub() {
    let executor = Arc::new(Scripted {
        refuse_runtimes: HashMap::from([
            ("python3.13".to_string(), StubReason::ImageUnavailable),
            ("nodejs16.x".to_string(), StubReason::DeprecatedRuntime),
        ]),
        ..Default::default()
    });
    let (client, headers) = executing_client(executor.clone()).await;
    let code = zip_of(&[("a", "b")]);

    // Refused by the executor's `prepare`.
    for (name, runtime, reason) in [
        ("fn-py", Runtime::Python313, "image-unavailable"),
        ("fn-old", Runtime::Nodejs16x, "deprecated-runtime"),
    ] {
        client
            .create_function()
            .function_name(name)
            .runtime(runtime)
            .handler("echo")
            .role(ROLE)
            .code(
                FunctionCode::builder()
                    .zip_file(Blob::new(code.clone()))
                    .build(),
            )
            .send()
            .await
            .unwrap();
        let output = invoke(&client, name, json!({})).await;
        assert_eq!(output.status_code(), 200);
        assert_eq!(output.payload().unwrap().as_ref(), b"{}");
        assert_stub(&headers, reason);

        // An Event invocation is answered by the stub too, with 202.
        let output = client
            .invoke()
            .function_name(name)
            .invocation_type(InvocationType::Event)
            .send()
            .await
            .unwrap();
        assert_eq!(output.status_code(), 202);
        assert_stub(&headers, reason);
    }

    // Refused by the executor's `invoke`.
    create(&client, "fn-refuse", "refuse", &code, &[]).await;
    invoke(&client, "fn-refuse", json!({})).await;
    assert_stub(&headers, "vm-unavailable");

    // Code given as an S3 location is not fetched.
    client
        .create_function()
        .function_name("fn-s3")
        .runtime(Runtime::Nodejs22x)
        .handler("echo")
        .role(ROLE)
        .code(
            FunctionCode::builder()
                .s3_bucket("bucket")
                .s3_key("fn.zip")
                .build(),
        )
        .send()
        .await
        .unwrap();
    invoke(&client, "fn-s3", json!({})).await;
    assert_stub(&headers, "no-code");

    // Container images are not run.
    client
        .create_function()
        .function_name("fn-image")
        .package_type(PackageType::Image)
        .role(ROLE)
        .code(
            FunctionCode::builder()
                .image_uri("123456789012.dkr.ecr.us-east-1.amazonaws.com/repo:latest")
                .build(),
        )
        .send()
        .await
        .unwrap();
    invoke(&client, "fn-image", json!({})).await;
    assert_stub(&headers, "image-function-unsupported");

    // Layer content is not stored, so a function with layers is not run
    // without them.
    client
        .create_function()
        .function_name("fn-layers")
        .runtime(Runtime::Nodejs22x)
        .handler("echo")
        .role(ROLE)
        .layers("arn:aws:lambda:us-east-1:123456789012:layer:shared:1")
        .code(
            FunctionCode::builder()
                .zip_file(Blob::new(code.clone()))
                .build(),
        )
        .send()
        .await
        .unwrap();
    invoke(&client, "fn-layers", json!({})).await;
    assert_stub(&headers, "layers-unsupported");

    assert!(
        executor
            .seen
            .lock()
            .unwrap()
            .iter()
            .all(|s| s.target.function_name == "fn-refuse"),
        "only the function refused by invoke reached it"
    );
}

#[tokio::test]
async fn response_stream_invocations_get_the_marked_stub() {
    let executor = Arc::new(Scripted::default());
    let (client, _) = executing_client(executor.clone()).await;
    create(&client, "fn-stream", "echo", &zip_of(&[("a", "b")]), &[]).await;
    // The SDK's event-stream output does not expose raw headers through the
    // interceptor reliably; the call must at least not run the function.
    let _ = client
        .invoke_with_response_stream()
        .function_name("fn-stream")
        .send()
        .await;
    assert!(executor.seen.lock().unwrap().is_empty());
}

#[tokio::test]
async fn api_errors_with_execution_enabled() {
    let executor = Arc::new(Scripted::default());
    let (client, _) = executing_client(executor.clone()).await;
    let err = client
        .invoke()
        .function_name("missing")
        .send()
        .await
        .expect_err("unknown function");
    assert_eq!(
        err.into_service_error().meta().code(),
        Some("ResourceNotFoundException")
    );
    create(&client, "fn-q", "echo", &zip_of(&[("a", "b")]), &[]).await;
    let err = client
        .invoke()
        .function_name("fn-q")
        .qualifier("7")
        .send()
        .await
        .expect_err("unknown version");
    assert_eq!(
        err.into_service_error().meta().code(),
        Some("ResourceNotFoundException")
    );
    assert!(executor.seen.lock().unwrap().is_empty());
}

// ---------------------------------------------------------------------------
// Code storage
// ---------------------------------------------------------------------------

#[tokio::test]
async fn the_executor_sees_the_exact_uploaded_archive() {
    let executor = Arc::new(Scripted::default());
    let (client, _) = executing_client(executor.clone()).await;
    let code = zip_of(&[("index.mjs", "export const handler = async () => 1;")]);
    create(&client, "fn-bytes", "echo", &code, &[]).await;
    invoke(&client, "fn-bytes", json!({})).await;
    assert_eq!(executor.seen.lock().unwrap()[0].code, Bytes::from(code));
}

#[tokio::test]
async fn code_sha256_and_code_size_are_those_of_the_uploaded_archive() {
    let (client, _) = client_for(LambdaService::new()).await;
    let first = zip_of(&[("index.mjs", "export const handler = async () => 1;")]);
    let second = zip_of(&[(
        "index.mjs",
        "export const handler = async () => 'a rather longer second revision';",
    )]);
    assert_ne!(first.len(), second.len());

    let created = client
        .create_function()
        .function_name("hashed")
        .runtime(Runtime::Nodejs22x)
        .handler("index.handler")
        .role(ROLE)
        .code(
            FunctionCode::builder()
                .zip_file(Blob::new(first.clone()))
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(created.code_sha256(), Some(sha256_base64(&first).as_str()));
    assert_eq!(created.code_size(), first.len() as i64);

    let published = client
        .publish_version()
        .function_name("hashed")
        .send()
        .await
        .unwrap();
    assert_eq!(
        published.code_sha256(),
        Some(sha256_base64(&first).as_str())
    );

    let updated = client
        .update_function_code()
        .function_name("hashed")
        .zip_file(Blob::new(second.clone()))
        .send()
        .await
        .unwrap();
    assert_eq!(updated.code_sha256(), Some(sha256_base64(&second).as_str()));
    assert_eq!(updated.code_size(), second.len() as i64);

    let configuration = client
        .get_function_configuration()
        .function_name("hashed")
        .send()
        .await
        .unwrap();
    assert_eq!(
        configuration.code_sha256(),
        Some(sha256_base64(&second).as_str())
    );
    assert_eq!(configuration.code_size(), second.len() as i64);

    // The published version still reports the archive it was published with.
    let versions = client
        .list_versions_by_function()
        .function_name("hashed")
        .send()
        .await
        .unwrap();
    let version_one = versions
        .versions()
        .iter()
        .find(|v| v.version() == Some("1"))
        .expect("version 1 is listed");
    assert_eq!(
        version_one.code_sha256(),
        Some(sha256_base64(&first).as_str())
    );
    assert_eq!(version_one.code_size(), first.len() as i64);
}
