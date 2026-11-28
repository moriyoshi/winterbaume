//! Real functions in real microVMs, on the stock AWS Lambda base images.
//!
//! These tests need hardware virtualisation, libkrun and libkrunfw, and
//! either network access to `public.ecr.aws` or images in an image
//! directory.  They run only when `WB_LAMBDA_VM_TEST=1`; otherwise each
//! prints a `SKIP` line and passes.
//!
//! - `WB_LAMBDA_VM_TEST_LIBKRUN`: path of libkrun, when the dynamic loader
//!   does not find it ( libkrunfw next to it is loaded too ).
//! - `WB_LAMBDA_VM_TEST_CACHE`: image cache to use ( kept between runs; a
//!   temporary one by default, which pulls every image afresh ).
//! - `WB_LAMBDA_VM_TEST_IMAGE_DIR`: an image directory to use instead of
//!   pulling.
//! - `WB_LAMBDA_VM_AGENT`: the guest agent, built from
//!   `crates/winterbaume-lambda-vm-agent` for the host's architecture
//!   ( required ).

use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

use aws_sdk_lambda::config::BehaviorVersion;
use aws_sdk_lambda::operation::invoke::InvokeOutput;
use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::types::{Architecture, Environment, FunctionCode, LogType, Runtime};
use base64::Engine as _;
use serde_json::{Value, json};
use winterbaume_core::MockAws;
use winterbaume_lambda::LambdaService;
use winterbaume_lambda_vm::{AwsEndpoint, HelperCommand, KrunExecutor, KrunExecutorConfig};

const ROLE: &str = "arn:aws:iam::123456789012:role/lambda-role";
const HELPER: &str = env!("CARGO_BIN_EXE_winterbaume-lambda-vm-helper");

macro_rules! require_vm {
    () => {
        if std::env::var("WB_LAMBDA_VM_TEST").as_deref() != Ok("1") {
            fn here() {}
            let test = std::any::type_name_of_val(&here)
                .trim_end_matches("::here")
                .trim_end_matches("::{{closure}}");
            eprintln!("SKIP {test}: set WB_LAMBDA_VM_TEST=1 to run functions in microVMs");
            return;
        }
    };
}

fn host_architecture() -> Architecture {
    match std::env::consts::ARCH {
        "aarch64" => Architecture::Arm64,
        _ => Architecture::X8664,
    }
}

struct Vm {
    client: aws_sdk_lambda::Client,
    s3: aws_sdk_s3::Client,
    dynamodb: aws_sdk_dynamodb::Client,
    _cache: Option<tempfile::TempDir>,
}

async fn vm() -> Vm {
    // `RUST_LOG=winterbaume_lambda_vm=debug` shows why something did not run.
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_test_writer()
        .try_init();
    let endpoint = AwsEndpoint::new();
    let mut config = KrunExecutorConfig::new()
        .with_helper(HelperCommand::program(HELPER))
        .with_aws_endpoint(endpoint.clone());
    if let Some(lib) = std::env::var_os("WB_LAMBDA_VM_TEST_LIBKRUN") {
        config = config.with_libkrun(PathBuf::from(lib));
    }
    if let Some(dir) = std::env::var_os("WB_LAMBDA_VM_TEST_IMAGE_DIR") {
        config = config.with_image_dir(PathBuf::from(dir));
    }
    let temporary = match std::env::var_os("WB_LAMBDA_VM_TEST_CACHE") {
        Some(dir) => {
            config = config.with_cache_dir(PathBuf::from(dir));
            None
        }
        None => {
            let dir = tempfile::tempdir().unwrap();
            config = config.with_cache_dir(dir.path());
            Some(dir)
        }
    };
    let executor = KrunExecutor::new(config);
    executor
        .probe()
        .await
        .expect("WB_LAMBDA_VM_TEST=1 but a microVM cannot be started here");
    // Functions' AWS calls go to these, in the same mock environment.
    let mock = MockAws::builder()
        .with_service(LambdaService::new().with_code_execution(Arc::new(executor)))
        .with_service(winterbaume_sts::StsService::new())
        .with_service(winterbaume_s3::S3Service::new())
        .with_service(winterbaume_dynamodb::DynamoDbService::new())
        .build();
    endpoint.connect(mock.dispatcher());
    let sdk = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lambda::config::Region::new("us-east-1"))
        .load()
        .await;
    Vm {
        client: aws_sdk_lambda::Client::new(&sdk),
        s3: aws_sdk_s3::Client::from_conf(
            aws_sdk_s3::config::Builder::from(&sdk)
                .force_path_style(true)
                .build(),
        ),
        dynamodb: aws_sdk_dynamodb::Client::new(&sdk),
        _cache: temporary,
    }
}

fn zip_of(files: &[(&str, &str, u32)]) -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    for (name, content, mode) in files {
        writer
            .start_file(
                *name,
                zip::write::SimpleFileOptions::default().unix_permissions(*mode),
            )
            .unwrap();
        writer.write_all(content.as_bytes()).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

async fn create(
    vm: &Vm,
    name: &str,
    runtime: Runtime,
    handler: &str,
    files: &[(&str, &str, u32)],
    timeout: i32,
) {
    vm.client
        .create_function()
        .function_name(name)
        .runtime(runtime)
        .handler(handler)
        .role(ROLE)
        .timeout(timeout)
        .architectures(host_architecture())
        .environment(
            Environment::builder()
                .variables("GREETING", "hello")
                .build(),
        )
        .code(
            FunctionCode::builder()
                .zip_file(Blob::new(zip_of(files)))
                .build(),
        )
        .send()
        .await
        .expect("create_function should succeed");
}

async fn invoke(vm: &Vm, name: &str, event: Value) -> InvokeOutput {
    vm.client
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
    serde_json::from_slice(bytes).unwrap_or_else(|e| {
        panic!(
            "not JSON ( {e} ): {} / log: {}",
            String::from_utf8_lossy(bytes),
            log_of(output)
        )
    })
}

fn log_of(output: &InvokeOutput) -> String {
    output
        .log_result()
        .and_then(|l| base64::engine::general_purpose::STANDARD.decode(l).ok())
        .map(|b| String::from_utf8_lossy(&b).to_string())
        .unwrap_or_default()
}

const NODE_ESM: &str = r#"
import fs from "node:fs";
let invocations = 0;
export const handler = async (event, context) => {
  invocations += 1;
  fs.writeFileSync("/tmp/scratch", "ok");
  let taskWritable = true;
  try { fs.writeFileSync("/var/task/nope", "x"); } catch (e) { taskWritable = e.code; }
  console.log("handling", JSON.stringify(event));
  return {
    event,
    invocations,
    greeting: process.env.GREETING,
    functionName: context.functionName,
    requestId: context.awsRequestId,
    remaining: context.getRemainingTimeInMillis(),
    memory: context.memoryLimitInMB,
    tmp: fs.readFileSync("/tmp/scratch", "utf8"),
    taskWritable,
    node: process.version,
    arch: process.arch,
  };
};
"#;

#[tokio::test]
async fn nodejs_esm_handler_runs_on_the_stock_image() {
    require_vm!();
    let vm = vm().await;
    create(
        &vm,
        "node-esm",
        Runtime::Nodejs22x,
        "index.handler",
        &[("index.mjs", NODE_ESM, 0o644)],
        30,
    )
    .await;

    let output = invoke(&vm, "node-esm", json!({"n": 1})).await;
    assert_eq!(output.function_error(), None, "log: {}", log_of(&output));
    let result = payload(&output);
    assert_eq!(result["event"], json!({"n": 1}));
    assert_eq!(result["greeting"], "hello");
    assert_eq!(result["functionName"], "node-esm");
    assert_eq!(result["memory"], "128");
    assert_eq!(result["tmp"], "ok");
    assert_eq!(result["taskWritable"], "EROFS", "/var/task is read-only");
    assert!(result["remaining"].as_u64().unwrap() > 20_000);
    assert!(result["node"].as_str().unwrap().starts_with("v22."));
    assert!(
        log_of(&output).contains("handling {\"n\":1}"),
        "{}",
        log_of(&output)
    );

    // A warm environment keeps module state.
    let again = payload(&invoke(&vm, "node-esm", json!({"n": 2})).await);
    assert_eq!(again["invocations"], 2);
}

#[tokio::test]
async fn nodejs_commonjs_callback_and_errors_use_the_aws_runtime() {
    require_vm!();
    let vm = vm().await;
    create(
        &vm,
        "node-cjs",
        Runtime::Nodejs22x,
        "lib/app.handler",
        &[(
            "lib/app.js",
            "exports.handler = (event, context, callback) => callback(null, { cjs: true, got: event.x });",
            0o644,
        )],
        30,
    )
    .await;
    let result = payload(&invoke(&vm, "node-cjs", json!({"x": 5})).await);
    assert_eq!(result, json!({"cjs": true, "got": 5}));

    create(
        &vm,
        "node-throw",
        Runtime::Nodejs22x,
        "index.handler",
        &[(
            "index.mjs",
            "export const handler = async () => { throw new TypeError('bad input'); };",
            0o644,
        )],
        30,
    )
    .await;
    let output = invoke(&vm, "node-throw", json!({})).await;
    assert_eq!(output.function_error(), Some("Unhandled"));
    let error = payload(&output);
    assert_eq!(error["errorType"], "TypeError");
    assert_eq!(error["errorMessage"], "bad input");
    assert!(error["trace"].is_array(), "{error}");

    create(
        &vm,
        "node-missing",
        Runtime::Nodejs22x,
        "nope.handler",
        &[("index.mjs", "export const handler = 1;", 0o644)],
        30,
    )
    .await;
    let output = invoke(&vm, "node-missing", json!({})).await;
    assert_eq!(output.function_error(), Some("Unhandled"));
    assert_eq!(payload(&output)["errorType"], "Runtime.ImportModuleError");

    create(
        &vm,
        "node-nohandler",
        Runtime::Nodejs22x,
        "index.other",
        &[("index.mjs", "export const handler = 1;", 0o644)],
        30,
    )
    .await;
    let output = invoke(&vm, "node-nohandler", json!({})).await;
    assert_eq!(output.function_error(), Some("Unhandled"));
    assert_eq!(payload(&output)["errorType"], "Runtime.HandlerNotFound");
}

#[tokio::test]
async fn timeouts_stop_the_vm() {
    require_vm!();
    let vm = vm().await;
    create(
        &vm,
        "node-slow",
        Runtime::Nodejs22x,
        "index.handler",
        // A pending timer keeps the event loop busy.  A promise that never
        // settles on an idle loop makes the Node.js runtime answer `null`
        // instead, as on AWS.
        &[(
            "index.mjs",
            "export const handler = () => new Promise(() => setInterval(() => {}, 1000));",
            0o644,
        )],
        2,
    )
    .await;
    for _ in 0..2 {
        // The second invocation gets a fresh environment and times out too.
        let output = invoke(&vm, "node-slow", json!({})).await;
        assert_eq!(output.function_error(), Some("Unhandled"));
        assert_eq!(payload(&output)["errorType"], "Sandbox.Timedout");
        assert!(log_of(&output).contains("Task timed out after 2.00 seconds"));
    }
}

#[tokio::test]
async fn python_handler_runs_on_the_stock_image() {
    require_vm!();
    let vm = vm().await;
    create(
        &vm,
        "py-fn",
        Runtime::Python313,
        "app.handler",
        &[(
            "app.py",
            "import os, sys\n\ndef handler(event, context):\n    return {'event': event, 'greeting': os.environ['GREETING'], 'python': sys.version.split()[0], 'name': context.function_name}\n",
            0o644,
        )],
        30,
    )
    .await;
    let output = invoke(&vm, "py-fn", json!({"p": true})).await;
    assert_eq!(output.function_error(), None, "log: {}", log_of(&output));
    let result = payload(&output);
    assert_eq!(result["event"], json!({"p": true}));
    assert_eq!(result["greeting"], "hello");
    assert_eq!(result["name"], "py-fn");
    assert!(result["python"].as_str().unwrap().starts_with("3.13"));
}

/// A custom runtime: the function's own `bootstrap`, a shell script that
/// speaks the Runtime API with curl.
const SHELL_BOOTSTRAP: &str = r#"#!/bin/sh
set -eu
API="http://${AWS_LAMBDA_RUNTIME_API}/2018-06-01/runtime/invocation"
while true; do
  HEADERS="$(mktemp)"
  EVENT="$(curl -sS -LD "$HEADERS" "$API/next")"
  ID="$(grep -i Lambda-Runtime-Aws-Request-Id "$HEADERS" | tr -d '[:space:]' | cut -d: -f2)"
  curl -sS -X POST "$API/$ID/response" -d "{\"custom\":true,\"handler\":\"$_HANDLER\",\"event\":$EVENT}" >/dev/null
done
"#;

#[tokio::test]
async fn custom_runtime_runs_the_function_bootstrap() {
    require_vm!();
    let vm = vm().await;
    create(
        &vm,
        "custom-fn",
        Runtime::Providedal2023,
        "function.handler",
        &[("bootstrap", SHELL_BOOTSTRAP, 0o755)],
        30,
    )
    .await;
    let output = invoke(&vm, "custom-fn", json!({"c": 1})).await;
    assert_eq!(output.function_error(), None, "log: {}", log_of(&output));
    assert_eq!(
        payload(&output),
        json!({"custom": true, "handler": "function.handler", "event": {"c": 1}})
    );
}

// ---------------------------------------------------------------------------
// AWS SDK calls from inside the function, to the same mock environment
// ---------------------------------------------------------------------------

/// Create the bucket and table the SDK tests use.
async fn aws_fixtures(vm: &Vm, bucket: &str, table: &str) {
    vm.s3.create_bucket().bucket(bucket).send().await.unwrap();
    vm.dynamodb
        .create_table()
        .table_name(table)
        .attribute_definitions(
            aws_sdk_dynamodb::types::AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(aws_sdk_dynamodb::types::ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .key_schema(
            aws_sdk_dynamodb::types::KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(aws_sdk_dynamodb::types::KeyType::Hash)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();
}

/// What the function wrote is visible from the test, and the reverse.
async fn assert_aws_round_trip(vm: &Vm, output: &InvokeOutput, bucket: &str, table: &str) {
    assert_eq!(output.function_error(), None, "log: {}", log_of(output));
    let result = payload(output);
    assert_eq!(result["account"], "123456789012", "{result}");
    assert_eq!(result["object"], "from the test", "{result}");
    assert_eq!(result["item"], "from the test", "{result}");
    let object = vm
        .s3
        .get_object()
        .bucket(bucket)
        .key("written-by-function")
        .send()
        .await
        .expect("the function's object exists");
    let body = object.body.collect().await.unwrap().into_bytes();
    assert_eq!(&body[..], b"from the function");
    let item = vm
        .dynamodb
        .get_item()
        .table_name(table)
        .key(
            "id",
            aws_sdk_dynamodb::types::AttributeValue::S("written-by-function".into()),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(
        item.item()
            .and_then(|i| i.get("value"))
            .and_then(|v| v.as_s().ok()),
        Some(&"from the function".to_string())
    );
}

async fn seed(vm: &Vm, bucket: &str, table: &str) {
    vm.s3
        .put_object()
        .bucket(bucket)
        .key("written-by-test")
        .body(aws_sdk_s3::primitives::ByteStream::from_static(
            b"from the test",
        ))
        .send()
        .await
        .unwrap();
    vm.dynamodb
        .put_item()
        .table_name(table)
        .item(
            "id",
            aws_sdk_dynamodb::types::AttributeValue::S("written-by-test".into()),
        )
        .item(
            "value",
            aws_sdk_dynamodb::types::AttributeValue::S("from the test".into()),
        )
        .send()
        .await
        .unwrap();
}

const NODE_SDK: &str = r#"
import { STSClient, GetCallerIdentityCommand } from "@aws-sdk/client-sts";
import { S3Client, PutObjectCommand, GetObjectCommand } from "@aws-sdk/client-s3";
import { DynamoDBClient, PutItemCommand, GetItemCommand } from "@aws-sdk/client-dynamodb";
export const handler = async (event) => {
  const id = await new STSClient({}).send(new GetCallerIdentityCommand({}));
  const s3 = new S3Client({});
  await s3.send(new PutObjectCommand({ Bucket: event.bucket, Key: "written-by-function", Body: "from the function" }));
  const got = await s3.send(new GetObjectCommand({ Bucket: event.bucket, Key: "written-by-test" }));
  const ddb = new DynamoDBClient({});
  await ddb.send(new PutItemCommand({ TableName: event.table, Item: { id: { S: "written-by-function" }, value: { S: "from the function" } } }));
  const item = await ddb.send(new GetItemCommand({ TableName: event.table, Key: { id: { S: "written-by-test" } } }));
  return { account: id.Account, object: await got.Body.transformToString(), item: item.Item?.value?.S };
};
"#;

#[tokio::test]
async fn nodejs_sdk_calls_reach_the_mock_environment() {
    require_vm!();
    let vm = vm().await;
    aws_fixtures(&vm, "node-bucket", "node-table").await;
    seed(&vm, "node-bucket", "node-table").await;
    create(
        &vm,
        "node-sdk",
        Runtime::Nodejs22x,
        "index.handler",
        &[("index.mjs", NODE_SDK, 0o644)],
        30,
    )
    .await;
    let output = invoke(
        &vm,
        "node-sdk",
        json!({"bucket": "node-bucket", "table": "node-table"}),
    )
    .await;
    assert_aws_round_trip(&vm, &output, "node-bucket", "node-table").await;
}

const PYTHON_SDK: &str = r#"
import boto3

def handler(event, context):
    account = boto3.client("sts").get_caller_identity()["Account"]
    s3 = boto3.client("s3")
    s3.put_object(Bucket=event["bucket"], Key="written-by-function", Body=b"from the function")
    got = s3.get_object(Bucket=event["bucket"], Key="written-by-test")["Body"].read().decode()
    ddb = boto3.client("dynamodb")
    ddb.put_item(TableName=event["table"], Item={"id": {"S": "written-by-function"}, "value": {"S": "from the function"}})
    item = ddb.get_item(TableName=event["table"], Key={"id": {"S": "written-by-test"}})["Item"]["value"]["S"]
    return {"account": account, "object": got, "item": item}
"#;

#[tokio::test]
async fn python_sdk_calls_reach_the_mock_environment() {
    require_vm!();
    let vm = vm().await;
    aws_fixtures(&vm, "py-bucket", "py-table").await;
    seed(&vm, "py-bucket", "py-table").await;
    create(
        &vm,
        "py-sdk",
        Runtime::Python313,
        "app.handler",
        &[("app.py", PYTHON_SDK, 0o644)],
        30,
    )
    .await;
    let output = invoke(
        &vm,
        "py-sdk",
        json!({"bucket": "py-bucket", "table": "py-table"}),
    )
    .await;
    assert_aws_round_trip(&vm, &output, "py-bucket", "py-table").await;
}

#[tokio::test]
async fn the_guest_has_no_network_by_default() {
    require_vm!();
    let vm = vm().await;
    let probe = r#"
import net from "node:net";
const attempt = (host, port) => new Promise((resolve) => {
  const s = net.connect({ host, port, timeout: 2000 });
  s.on("connect", () => { s.destroy(); resolve("connected"); });
  s.on("timeout", () => { s.destroy(); resolve("timeout"); });
  s.on("error", (e) => resolve(e.code));
});
export const handler = async () => ({
  internet: await attempt("1.1.1.1", 443),
  hostLoopback: await attempt("127.0.0.1", 22),
  runtimeApi: await attempt("127.0.0.1", 9001),
});
"#;
    create(
        &vm,
        "node-net",
        Runtime::Nodejs22x,
        "index.handler",
        &[("index.mjs", probe, 0o644)],
        30,
    )
    .await;
    let result = payload(&invoke(&vm, "node-net", json!({})).await);
    assert_ne!(result["internet"], "connected", "{result}");
    assert_ne!(
        result["hostLoopback"], "connected",
        "the host's ports are not reachable: {result}"
    );
    assert_eq!(result["runtimeApi"], "connected", "{result}");
}

// ---------------------------------------------------------------------------
// Every runtime family, on its stock base image
// ---------------------------------------------------------------------------

const NODE_ECHO: &str = "export const handler = async (event) => ({ runtime: 'nodejs', version: process.versions.node.split('.')[0], event });";
const PYTHON_ECHO: &str = "import sys\n\ndef handler(event, context):\n    return {'runtime': 'python', 'version': '%d.%d' % sys.version_info[:2], 'event': event}\n";
const RUBY_ECHO: &str = "def handler(event:, context:)\n  { 'runtime' => 'ruby', 'version' => RUBY_VERSION.split('.')[0, 2].join('.'), 'event' => event }\nend\n";
const JAVA_CLASS: &[u8] = include_bytes!("fixtures/java/Hello.class");
/// One package per framework: the .NET runtime starts with the function's
/// `runtimeconfig.json`, so a net8.0 build does not run on dotnet10.
const DOTNET8_ZIP: &[u8] = include_bytes!("fixtures/dotnet/hello-net8.zip");
const DOTNET10_ZIP: &[u8] = include_bytes!("fixtures/dotnet/hello-net10.zip");
/// A custom runtime in a shell script, speaking the Runtime API with curl.
const PROVIDED_BOOTSTRAP: &str = r#"#!/bin/sh
set -eu
API="http://${AWS_LAMBDA_RUNTIME_API}/2018-06-01/runtime/invocation"
while true; do
  HEADERS="$(mktemp)"
  EVENT="$(curl -sS -LD "$HEADERS" "$API/next")"
  ID="$(grep -i Lambda-Runtime-Aws-Request-Id "$HEADERS" | tr -d '[:space:]' | cut -d: -f2)"
  curl -sS -X POST "$API/$ID/response" -d "{\"runtime\":\"provided\",\"event\":$EVENT}" >/dev/null
done
"#;

fn zip_bytes(files: &[(&str, &[u8], u32)]) -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    for (name, content, mode) in files {
        writer
            .start_file(
                *name,
                zip::write::SimpleFileOptions::default().unix_permissions(*mode),
            )
            .unwrap();
        writer.write_all(content).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

/// Deploy `package` on `runtime`, invoke it and check what it says about
/// itself.
async fn run_on(
    runtime: &str,
    handler: &str,
    package: Vec<u8>,
    family: &str,
    version: Option<&str>,
) {
    let vm = vm().await;
    let name = format!("matrix-{}", runtime.replace('.', "-"));
    vm.client
        .create_function()
        .function_name(&name)
        .runtime(Runtime::from(runtime))
        .handler(handler)
        .role(ROLE)
        .timeout(60)
        .memory_size(512)
        .architectures(host_architecture())
        .code(FunctionCode::builder().zip_file(Blob::new(package)).build())
        .send()
        .await
        .expect("create_function should succeed");
    let output = invoke(&vm, &name, json!({"matrix": runtime})).await;
    assert_eq!(
        output.function_error(),
        None,
        "{runtime}: {} / log: {}",
        String::from_utf8_lossy(output.payload().map(|p| p.as_ref()).unwrap_or_default()),
        log_of(&output)
    );
    let result = payload(&output);
    assert_eq!(result["runtime"], family, "{runtime}: {result}");
    assert_eq!(
        result["event"],
        json!({"matrix": runtime}),
        "{runtime}: {result}"
    );
    if let Some(version) = version {
        assert_eq!(result["version"], version, "{runtime}: {result}");
    }
}

macro_rules! runtime_matrix {
    ($($test:ident: $runtime:literal => $kind:ident $(, $version:literal)?;)*) => {
        $(
            #[tokio::test]
            async fn $test() {
                require_vm!();
                runtime_matrix!(@run $kind, $runtime $(, $version)?);
            }
        )*
    };
    (@run node, $runtime:literal, $version:literal) => {
        run_on($runtime, "index.handler", zip_bytes(&[("index.mjs", NODE_ECHO.as_bytes(), 0o644)]), "nodejs", Some($version)).await
    };
    (@run python, $runtime:literal, $version:literal) => {
        run_on($runtime, "app.handler", zip_bytes(&[("app.py", PYTHON_ECHO.as_bytes(), 0o644)]), "python", Some($version)).await
    };
    (@run ruby, $runtime:literal, $version:literal) => {
        run_on($runtime, "function.handler", zip_bytes(&[("function.rb", RUBY_ECHO.as_bytes(), 0o644)]), "ruby", Some($version)).await
    };
    (@run java, $runtime:literal, $version:literal) => {
        run_on($runtime, "Hello::handleRequest", zip_bytes(&[("Hello.class", JAVA_CLASS, 0o644)]), "java", Some($version)).await
    };
    (@run dotnet, $runtime:literal, $version:literal) => {
        run_on(
            $runtime,
            "Hello::Hello.Function::Handle",
            if $version == "8" { DOTNET8_ZIP } else { DOTNET10_ZIP }.to_vec(),
            "dotnet",
            Some($version),
        )
        .await
    };
    (@run provided, $runtime:literal) => {
        run_on($runtime, "function.handler", zip_bytes(&[("bootstrap", PROVIDED_BOOTSTRAP.as_bytes(), 0o755)]), "provided", None).await
    };
}

runtime_matrix! {
    matrix_nodejs18: "nodejs18.x" => node, "18";
    matrix_nodejs20: "nodejs20.x" => node, "20";
    matrix_nodejs22: "nodejs22.x" => node, "22";
    matrix_nodejs24: "nodejs24.x" => node, "24";
    matrix_nodejs26: "nodejs26.x" => node, "26";
    matrix_python39: "python3.9" => python, "3.9";
    matrix_python310: "python3.10" => python, "3.10";
    matrix_python311: "python3.11" => python, "3.11";
    matrix_python312: "python3.12" => python, "3.12";
    matrix_python313: "python3.13" => python, "3.13";
    matrix_python314: "python3.14" => python, "3.14";
    matrix_ruby32: "ruby3.2" => ruby, "3.2";
    matrix_ruby33: "ruby3.3" => ruby, "3.3";
    matrix_ruby34: "ruby3.4" => ruby, "3.4";
    matrix_ruby40: "ruby4.0" => ruby, "4.0";
    matrix_java8_al2: "java8.al2" => java, "1.8";
    matrix_java11: "java11" => java, "11";
    matrix_java17: "java17" => java, "17";
    matrix_java21: "java21" => java, "21";
    matrix_java25: "java25" => java, "25";
    matrix_dotnet8: "dotnet8" => dotnet, "8";
    matrix_dotnet10: "dotnet10" => dotnet, "10";
    matrix_provided_al2: "provided.al2" => provided;
    matrix_provided_al2023: "provided.al2023" => provided;
}
