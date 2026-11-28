//! The microVM executor end to end, through `LambdaService` and the AWS
//! SDK, with the fake VM helper standing in for libkrun: it plays a Lambda
//! runtime on the host against the executor's Runtime API.  Everything but
//! the VM itself is real: image resolution, root building, the Runtime API,
//! the warm pool, timeouts, process management and log framing.

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use aws_sdk_lambda::config::interceptors::BeforeDeserializationInterceptorContextRef;
use aws_sdk_lambda::config::{BehaviorVersion, ConfigBag, Intercept, RuntimeComponents};
use aws_sdk_lambda::error::BoxError;
use aws_sdk_lambda::operation::invoke::InvokeOutput;
use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::types::{Architecture, Environment, FunctionCode, LogType, Runtime};
use base64::Engine as _;
use serde_json::{Value, json};
use winterbaume_core::MockAws;
use winterbaume_lambda::LambdaService;
use winterbaume_lambda::execution::{
    ARCHITECTURE_SUBSTITUTED_HEADER, EXECUTION_EXECUTED, EXECUTION_HEADER, STUB_REASON_HEADER,
};
use winterbaume_lambda_vm::{
    ArchMismatch, AwsEndpoint, HelperCommand, KrunExecutor, KrunExecutorConfig,
};

const ROLE: &str = "arn:aws:iam::123456789012:role/lambda-role";
const FAKE_HELPER: &str = env!("CARGO_BIN_EXE_winterbaume-lambda-vm-fake-helper");

fn host_arch() -> &'static str {
    match std::env::consts::ARCH {
        "aarch64" => "arm64",
        _ => "x86_64",
    }
}

fn other_arch() -> Architecture {
    match host_arch() {
        "arm64" => Architecture::X8664,
        _ => Architecture::Arm64,
    }
}

fn host_architecture() -> Architecture {
    match host_arch() {
        "arm64" => Architecture::Arm64,
        _ => Architecture::X8664,
    }
}

/// An image directory with a minimal nodejs22.x root for the host
/// architecture, and its configuration.
fn image_dir() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path().join("nodejs22.x").join(host_arch());
    let rootfs = base.join("rootfs");
    std::fs::create_dir_all(rootfs.join("var/runtime")).unwrap();
    std::fs::create_dir_all(rootfs.join("var/task")).unwrap();
    std::fs::write(rootfs.join("var/runtime/bootstrap"), "#!/bin/sh\n").unwrap();
    std::fs::write(
        base.join("config.json"),
        r#"{"config":{"Env":["PATH=/var/lang/bin:/usr/bin","IMAGE_VAR=from-image","TZ=:/etc/localtime"],"WorkingDir":"/var/task"}}"#,
    )
    .unwrap();
    dir
}

struct Harness {
    client: aws_sdk_lambda::Client,
    headers: ResponseHeaders,
    _images: tempfile::TempDir,
    _cache: tempfile::TempDir,
}

fn config(images: &Path, cache: &Path) -> KrunExecutorConfig {
    KrunExecutorConfig::new()
        .with_helper(HelperCommand::program(FAKE_HELPER))
        // The fake does the agent's work itself; any file will do.
        .with_agent(FAKE_HELPER)
        .with_image_dir(images)
        .with_cache_dir(cache)
        .offline(true)
        .with_env("AWS_ENDPOINT_URL", "http://127.0.0.1:1")
}

async fn harness_with(adjust: impl FnOnce(KrunExecutorConfig) -> KrunExecutorConfig) -> Harness {
    let images = image_dir();
    let cache = tempfile::tempdir().unwrap();
    let executor = KrunExecutor::new(adjust(config(images.path(), cache.path())));
    let (client, headers) =
        client_for(LambdaService::new().with_code_execution(Arc::new(executor))).await;
    Harness {
        client,
        headers,
        _images: images,
        _cache: cache,
    }
}

async fn harness() -> Harness {
    harness_with(|c| c).await
}

/// A harness whose functions' AWS calls go to an STS service in the same
/// mock environment.
async fn harness_with_aws() -> Harness {
    let images = image_dir();
    let cache = tempfile::tempdir().unwrap();
    let endpoint = AwsEndpoint::new();
    let executor = KrunExecutor::new(
        KrunExecutorConfig::new()
            .with_helper(HelperCommand::program(FAKE_HELPER))
            .with_agent(FAKE_HELPER)
            .with_image_dir(images.path())
            .with_cache_dir(cache.path())
            .offline(true)
            .with_aws_endpoint(endpoint.clone()),
    );
    let mock = MockAws::builder()
        .with_service(LambdaService::new().with_code_execution(Arc::new(executor)))
        .with_service(winterbaume_sts::StsService::new())
        .build();
    assert!(endpoint.connect(mock.dispatcher()));
    let (client, headers) = client_for_mock(mock).await;
    Harness {
        client,
        headers,
        _images: images,
        _cache: cache,
    }
}

/// Records the headers of the most recent HTTP response.
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
    client_for_mock(MockAws::builder().with_service(service).build()).await
}

async fn client_for_mock(mock: MockAws) -> (aws_sdk_lambda::Client, ResponseHeaders) {
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

async fn create_with(
    h: &Harness,
    name: &str,
    handler: &str,
    runtime: Runtime,
    architecture: Architecture,
    timeout: i32,
) {
    h.client
        .create_function()
        .function_name(name)
        .runtime(runtime)
        .handler(handler)
        .role(ROLE)
        .timeout(timeout)
        .memory_size(256)
        .architectures(architecture)
        .environment(Environment::builder().variables("MINE", "yes").build())
        .code(
            FunctionCode::builder()
                .zip_file(Blob::new(zip_of(&[(
                    "index.mjs",
                    "export const handler = 1;",
                )])))
                .build(),
        )
        .send()
        .await
        .expect("create_function should succeed");
}

async fn create(h: &Harness, name: &str, handler: &str) {
    create_with(
        h,
        name,
        handler,
        Runtime::Nodejs22x,
        host_architecture(),
        10,
    )
    .await;
}

async fn invoke(h: &Harness, name: &str, event: Value) -> InvokeOutput {
    h.client
        .invoke()
        .function_name(name)
        .payload(Blob::new(event.to_string()))
        .log_type(LogType::Tail)
        .send()
        .await
        .expect("invoke should succeed")
}

fn payload(output: &InvokeOutput) -> Value {
    let bytes = output.payload().expect("payload").as_ref();
    serde_json::from_slice(bytes)
        .unwrap_or_else(|e| panic!("not JSON ( {e} ): {}", String::from_utf8_lossy(bytes)))
}

fn log_of(output: &InvokeOutput) -> String {
    String::from_utf8(
        base64::engine::general_purpose::STANDARD
            .decode(output.log_result().expect("LogType Tail returns the log"))
            .unwrap(),
    )
    .unwrap()
}

fn assert_executed(h: &Harness) {
    assert_eq!(
        h.headers.get(EXECUTION_HEADER).as_deref(),
        Some(EXECUTION_EXECUTED)
    );
}

fn assert_stub(h: &Harness, reason: &str) {
    assert_eq!(h.headers.get(EXECUTION_HEADER).as_deref(), Some("stub"));
    assert_eq!(h.headers.get(STUB_REASON_HEADER).as_deref(), Some(reason));
}

fn error_of(output: &InvokeOutput) -> Value {
    assert_eq!(output.status_code(), 200);
    assert_eq!(output.function_error(), Some("Unhandled"));
    payload(output)
}

// ---------------------------------------------------------------------------

#[tokio::test]
async fn the_runtime_gets_the_event_the_environment_and_the_invocation_headers() {
    let h = harness().await;
    create(&h, "echo-fn", "echo").await;
    let context = base64::engine::general_purpose::STANDARD.encode(r#"{"custom":{"k":"v"}}"#);
    let output = h
        .client
        .invoke()
        .function_name("echo-fn")
        .payload(Blob::new(r#"{"hello":"vm"}"#))
        .client_context(context)
        .send()
        .await
        .unwrap();
    assert_executed(&h);
    assert_eq!(output.function_error(), None);
    let result = payload(&output);
    assert_eq!(result["event"], json!({"hello": "vm"}));
    assert_eq!(
        result["arn"],
        "arn:aws:lambda:us-east-1:123456789012:function:echo-fn"
    );
    assert_eq!(result["clientContext"], r#"{"custom":{"k":"v"}}"#);
    assert!(result["traceId"].as_str().unwrap().starts_with("Root=1-"));
    let deadline: u64 = result["deadlineMs"].as_str().unwrap().parse().unwrap();
    let now = chrono::Utc::now().timestamp_millis() as u64;
    assert!(
        deadline > now && deadline <= now + 10_000,
        "deadline is the timeout from now"
    );

    let env = &result["env"];
    assert_eq!(env["MINE"], "yes");
    assert_eq!(
        env["IMAGE_VAR"], "from-image",
        "the image configuration applies"
    );
    assert_eq!(env["TZ"], ":UTC", "reserved variables win over the image's");
    assert_eq!(env["PATH"], "/var/lang/bin:/usr/bin");
    assert_eq!(env["AWS_ENDPOINT_URL"], "http://127.0.0.1:1");
    assert_eq!(env["_HANDLER"], "echo");
    assert_eq!(env["AWS_LAMBDA_FUNCTION_NAME"], "echo-fn");
    assert_eq!(env["AWS_LAMBDA_FUNCTION_MEMORY_SIZE"], "256");
    assert_eq!(env["AWS_EXECUTION_ENV"], "AWS_Lambda_nodejs22.x");
    assert_eq!(env["AWS_LAMBDA_RUNTIME_API"], "127.0.0.1:9001");
    assert_eq!(result["network"], false, "no network by default");
    assert_eq!(
        result["forwards"],
        json!([[9001, 1000]]),
        "no AWS endpoint without one"
    );
    assert_eq!(result["agent"], FAKE_HELPER);
    assert_eq!(result["ramMib"], 256 + 64);
    assert_eq!(result["vcpus"], 1);
    assert_eq!(result["workdir"], "/var/task");
    assert_eq!(result["bootstrap"], json!(["/var/runtime/bootstrap"]));
}

#[tokio::test]
async fn the_deployment_package_is_at_var_task_of_a_read_only_root() {
    let h = harness().await;
    create(&h, "file-fn", "file:index.mjs").await;
    let output = invoke(&h, "file-fn", json!({})).await;
    assert_eq!(payload(&output), json!("export const handler = 1;"));

    h.client
        .update_function_code()
        .function_name("file-fn")
        .zip_file(Blob::new(zip_of(&[("index.mjs", "second")])))
        .send()
        .await
        .unwrap();
    let output = invoke(&h, "file-fn", json!({})).await;
    assert_eq!(payload(&output), json!("second"));
}

#[tokio::test]
async fn environments_are_reused_until_the_function_changes() {
    let h = harness().await;
    create(&h, "count-fn", "count").await;
    assert_eq!(payload(&invoke(&h, "count-fn", json!({})).await), json!(1));
    assert_eq!(payload(&invoke(&h, "count-fn", json!({})).await), json!(2));
    h.client
        .update_function_configuration()
        .function_name("count-fn")
        .description("changed")
        .send()
        .await
        .unwrap();
    assert_eq!(
        payload(&invoke(&h, "count-fn", json!({})).await),
        json!(1),
        "a configuration change starts a new environment"
    );
}

#[tokio::test]
async fn the_log_is_framed_and_includes_the_function_output() {
    let h = harness().await;
    create(&h, "log-fn", "log:hello from the function").await;
    let output = invoke(&h, "log-fn", json!({})).await;
    let log = log_of(&output);
    let request_id = payload(&output)["requestId"].as_str().unwrap().to_string();
    assert!(
        log.starts_with(&format!("START RequestId: {request_id} Version: $LATEST\n")),
        "{log}"
    );
    assert!(
        log.contains("fake runtime starting"),
        "init output belongs to the first invocation: {log}"
    );
    assert!(log.contains("hello from the function\n"), "{log}");
    assert!(
        log.contains(&format!("END RequestId: {request_id}\n")),
        "{log}"
    );
    assert!(
        log.contains(&format!("REPORT RequestId: {request_id}\tDuration: ")),
        "{log}"
    );

    let second = log_of(&invoke(&h, "log-fn", json!({})).await);
    assert!(!second.contains("fake runtime starting"), "{second}");
    assert!(second.contains("hello from the function\n"), "{second}");
}

#[tokio::test]
async fn function_errors_are_unhandled_with_the_runtime_document() {
    let h = harness().await;
    create(&h, "err-fn", "error:TypeError").await;
    let error = error_of(&invoke(&h, "err-fn", json!({})).await);
    assert_eq!(error["errorType"], "TypeError");
    assert_eq!(error["errorMessage"], "fake failure");
    assert_executed(&h);
    // The environment survives a function error.
    let error = error_of(&invoke(&h, "err-fn", json!({})).await);
    assert_eq!(error["errorType"], "TypeError");
}

#[tokio::test]
async fn init_errors_crashes_and_missing_bootstraps() {
    let h = harness().await;
    create(&h, "init-fn", "init-error").await;
    let error = error_of(&invoke(&h, "init-fn", json!({})).await);
    assert_eq!(error["errorType"], "Runtime.ImportModuleError");

    create(&h, "exit-fn", "exit:3").await;
    let error = error_of(&invoke(&h, "exit-fn", json!({})).await);
    assert_eq!(error["errorType"], "Runtime.ExitError");
    assert!(
        error["errorMessage"]
            .as_str()
            .unwrap()
            .contains("exit status 3"),
        "{error}"
    );
    // A new environment is started for the next invocation.
    let error = error_of(&invoke(&h, "exit-fn", json!({})).await);
    assert_eq!(error["errorType"], "Runtime.ExitError");

    create(&h, "boot-fn", "no-bootstrap").await;
    let error = error_of(&invoke(&h, "boot-fn", json!({})).await);
    assert_eq!(error["errorType"], "Runtime.InvalidEntrypoint");
}

#[tokio::test]
async fn timeouts_kill_the_environment() {
    let h = harness().await;
    create_with(
        &h,
        "slow-fn",
        "sleep",
        Runtime::Nodejs22x,
        host_architecture(),
        1,
    )
    .await;
    let started = std::time::Instant::now();
    let output = invoke(&h, "slow-fn", json!({})).await;
    assert!(started.elapsed() < Duration::from_secs(8));
    let error = error_of(&output);
    assert_eq!(error["errorType"], "Sandbox.Timedout");
    assert!(
        error["errorMessage"]
            .as_str()
            .unwrap()
            .contains("Task timed out after 1.00 seconds"),
        "{error}"
    );
    let log = log_of(&output);
    assert!(log.contains("Task timed out after 1.00 seconds"), "{log}");
    let pid: u32 = log
        .split("pid=")
        .nth(1)
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|p| p.parse().ok())
        .expect("the fake runtime logs its pid");
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while Path::new(&format!("/proc/{pid}")).exists() && std::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    if cfg!(target_os = "linux") {
        assert!(
            !Path::new(&format!("/proc/{pid}")).exists(),
            "the helper was killed"
        );
    }
}

#[tokio::test]
async fn oversized_responses_are_a_function_error() {
    let h = harness().await;
    create(&h, "big-fn", "big").await;
    let error = error_of(&invoke(&h, "big-fn", json!({})).await);
    assert_eq!(error["errorType"], "Function.ResponseSizeTooLarge");
}

#[tokio::test]
async fn invocations_queue_for_the_per_function_limit() {
    let h = harness_with(|c| c.with_max_vms_per_function(1)).await;
    create(&h, "one-fn", "count").await;
    let mut tasks = Vec::new();
    for _ in 0..3 {
        let client = h.client.clone();
        tasks.push(tokio::spawn(async move {
            let out = client
                .invoke()
                .function_name("one-fn")
                .send()
                .await
                .unwrap();
            serde_json::from_slice::<u64>(out.payload().unwrap().as_ref()).unwrap()
        }));
    }
    let mut counts = Vec::new();
    for task in tasks {
        counts.push(task.await.unwrap());
    }
    counts.sort();
    assert_eq!(
        counts,
        vec![1, 2, 3],
        "one environment served them all, in turn"
    );
}

#[tokio::test]
async fn architecture_mismatch_is_a_stub_unless_relaxed() {
    let h = harness().await;
    create_with(&h, "arch-fn", "echo", Runtime::Nodejs22x, other_arch(), 10).await;
    invoke(&h, "arch-fn", json!({})).await;
    assert_stub(&h, "architecture-unsupported");

    let h = harness_with(|c| c.with_arch_mismatch(ArchMismatch::Native)).await;
    create_with(&h, "arch-fn", "echo", Runtime::Nodejs22x, other_arch(), 10).await;
    let output = invoke(&h, "arch-fn", json!({})).await;
    assert_executed(&h);
    assert_eq!(payload(&output)["event"], json!({}));
    let other = match other_arch() {
        Architecture::Arm64 => "arm64",
        _ => "x86_64",
    };
    assert_eq!(
        h.headers.get(ARCHITECTURE_SUBSTITUTED_HEADER),
        Some(format!("{other}->{}", host_arch()))
    );
}

#[tokio::test]
async fn runtimes_and_images_that_cannot_run_get_the_stub() {
    let h = harness().await;
    for (name, runtime, reason) in [
        ("py-fn", Runtime::Python313, "image-unavailable"),
        ("go-fn", Runtime::Go1x, "deprecated-runtime"),
        ("odd-fn", Runtime::from("cobol85"), "unsupported-runtime"),
    ] {
        create_with(&h, name, "echo", runtime, host_architecture(), 10).await;
        invoke(&h, name, json!({})).await;
        assert_stub(&h, reason);
    }
}

#[tokio::test]
async fn a_vm_that_cannot_start_is_a_stub() {
    let h = harness().await;
    create(&h, "fail-fn", "helper-fail:vm-unavailable").await;
    invoke(&h, "fail-fn", json!({})).await;
    assert_stub(&h, "vm-unavailable");

    let h = harness_with(|c| {
        c.with_helper(HelperCommand::with_args(
            FAKE_HELPER,
            ["--probe-fail=libkrun-unavailable"],
        ))
    })
    .await;
    create(&h, "nolib-fn", "echo").await;
    invoke(&h, "nolib-fn", json!({})).await;
    assert_stub(&h, "libkrun-unavailable");

    let h = harness_with(|c| c.with_helper(HelperCommand::program("/nonexistent/helper"))).await;
    create(&h, "nohelper-fn", "echo").await;
    invoke(&h, "nohelper-fn", json!({})).await;
    assert_stub(&h, "vm-helper-not-found");

    // A program that is not a helper at all.
    let h = harness_with(|c| c.with_helper(HelperCommand::with_args("/bin/sh", ["-c", "exit 0"])))
        .await;
    create(&h, "nothelper-fn", "echo").await;
    invoke(&h, "nothelper-fn", json!({})).await;
    assert_stub(&h, "vm-helper-not-found");
}

#[tokio::test]
async fn aws_calls_reach_the_services_of_the_same_mock_environment() {
    let h = harness_with_aws().await;
    create(&h, "aws-env-fn", "echo").await;
    let result = payload(&invoke(&h, "aws-env-fn", json!({})).await);
    assert_eq!(result["env"]["AWS_ENDPOINT_URL"], "http://127.0.0.1:4566");
    assert_eq!(result["forwards"], json!([[9001, 1000], [4566, 1001]]));

    create(&h, "aws-sts-fn", "aws:sts").await;
    let result = payload(&invoke(&h, "aws-sts-fn", json!({})).await);
    assert_eq!(result["status"], 200, "{result}");
    let body = result["body"].as_str().unwrap();
    assert!(body.contains("<Account>123456789012</Account>"), "{body}");
}

#[tokio::test]
async fn without_an_endpoint_there_is_no_aws_port() {
    let h = harness().await;
    create(&h, "noaws-fn", "aws:sts").await;
    let result = payload(&invoke(&h, "noaws-fn", json!({})).await);
    assert_eq!(result["body"], "no AWS endpoint");
}

#[tokio::test]
async fn the_network_setting_reaches_the_vm() {
    let h = harness_with(|c| c.with_network(true)).await;
    create(&h, "net-fn", "echo").await;
    assert_eq!(
        payload(&invoke(&h, "net-fn", json!({})).await)["network"],
        true
    );
}

#[tokio::test]
async fn a_missing_agent_is_a_stub() {
    let h = harness_with(|c| c.with_agent("/nonexistent/agent")).await;
    create(&h, "noagent-fn", "echo").await;
    invoke(&h, "noagent-fn", json!({})).await;
    assert_stub(&h, "vm-agent-not-found");
}

#[tokio::test]
async fn an_embedded_agent_is_used_when_none_is_on_disk() {
    if winterbaume_lambda_vm::config::embedded_agent().is_none() {
        eprintln!("SKIP: built without WINTERBAUME_LAMBDA_VM_AGENT");
        return;
    }
    // No agent configured; the variable and the executable's directory have
    // none either.
    let images = image_dir();
    let cache = tempfile::tempdir().unwrap();
    let executor = KrunExecutor::new(
        KrunExecutorConfig::new()
            .with_helper(HelperCommand::program(FAKE_HELPER))
            .with_image_dir(images.path())
            .with_cache_dir(cache.path())
            .offline(true),
    );
    let (client, headers) =
        client_for(LambdaService::new().with_code_execution(Arc::new(executor))).await;
    let h = Harness {
        client,
        headers,
        _images: images,
        _cache: cache,
    };
    create(&h, "embedded-fn", "echo").await;
    let result = payload(&invoke(&h, "embedded-fn", json!({})).await);
    assert_executed(&h);
    let agent = std::path::PathBuf::from(result["agent"].as_str().unwrap());
    assert!(agent.starts_with(h._cache.path()), "{}", agent.display());
    assert_eq!(
        std::fs::read(&agent).unwrap(),
        winterbaume_lambda_vm::config::embedded_agent().unwrap()
    );
}
