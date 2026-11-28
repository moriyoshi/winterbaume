//! Opt-in execution of deployed function code.
//!
//! By default `Invoke` never runs anything: it answers with a fixed `{}`
//! payload.  [`LambdaService::with_code_execution`] plugs in a
//! [`LambdaExecutor`], which runs ZIP-packaged functions and returns what
//! they produced.  This crate defines the contract and the pieces every
//! executor shares ( the reserved environment, the log framing, the ZIP
//! unpacker ); the executor that ships with winterbaume, which boots the
//! stock AWS Lambda base images in microVMs, lives in the
//! `winterbaume-lambda-vm` crate.
//!
//! Every `Invoke` response says which of the two happened through the
//! [`EXECUTION_HEADER`] response header.
//!
//! [`LambdaService::with_code_execution`]: crate::LambdaService::with_code_execution

use std::collections::{BTreeMap, HashMap};
use std::future::Future;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use winterbaume_core::{BlobStore, VfsError};

/// Response header carried by every `Invoke` / `InvokeWithResponseStream`
/// response: [`EXECUTION_EXECUTED`] when the function's code was run,
/// [`EXECUTION_STUB`] when the payload is the fixed `{}` placeholder.
pub const EXECUTION_HEADER: &str = "x-winterbaume-lambda-execution";
/// [`EXECUTION_HEADER`] value: the deployed code was run.
pub const EXECUTION_EXECUTED: &str = "executed";
/// [`EXECUTION_HEADER`] value: nothing was run, the response is a stub.
pub const EXECUTION_STUB: &str = "stub";
/// Response header that accompanies a stub and says why the code was not
/// run; the values are those of [`StubReason::as_str`].
pub const STUB_REASON_HEADER: &str = "x-winterbaume-lambda-stub-reason";
/// Response header set when the function ran on another instruction set
/// architecture than the one it is configured for, as
/// `<configured>-><used>` ( for example `x86_64->arm64` ).  Only an executor
/// configured to allow that substitution sets it.
pub const ARCHITECTURE_SUBSTITUTED_HEADER: &str = "x-winterbaume-lambda-architecture-substituted";

/// How many trailing bytes of the execution log `LogType: Tail` returns.
pub const LOG_TAIL_BYTES: usize = 4096;
/// Upper bound for an unpacked deployment package ( the Lambda quota ).
pub const MAX_UNPACKED_BYTES: u64 = 262_144_000;

/// A boxed, sendable future, as returned by [`LambdaExecutor`] methods.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Why a function could not be run and is answered with the stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StubReason {
    /// No executor is configured.
    ExecutionDisabled,
    /// The runtime identifier is not one the executor can run.
    UnsupportedRuntime,
    /// The runtime is one AWS has deprecated and no longer publishes an
    /// image for.
    DeprecatedRuntime,
    /// No deployment package is stored for the function version ( code
    /// given as `S3Bucket` / `S3Key`, or state injected without code ).
    NoCode,
    /// The stored package cannot be unpacked.
    InvalidPackage,
    /// `PackageType: Image` functions are not run.
    ImageFunctionUnsupported,
    /// The function uses layers, whose content is not stored.
    LayersUnsupported,
    /// The function's architecture cannot be run on this host.
    ArchitectureUnsupported,
    /// The VM helper program cannot be found.
    VmHelperNotFound,
    /// The guest agent program cannot be found.
    VmAgentNotFound,
    /// libkrun cannot be loaded.
    LibkrunUnavailable,
    /// Hardware virtualisation is not available ( no `/dev/kvm` access, or
    /// no Hypervisor.framework entitlement ).
    VmUnavailable,
    /// The base image for the runtime cannot be obtained.
    ImageUnavailable,
    /// The image cache directory cannot be used.
    ImageCacheUnusable,
    /// `InvokeWithResponseStream` is not implemented.
    ResponseStreamUnsupported,
}

impl StubReason {
    /// The value of [`STUB_REASON_HEADER`] for this reason.
    pub fn as_str(self) -> &'static str {
        match self {
            StubReason::ExecutionDisabled => "execution-disabled",
            StubReason::UnsupportedRuntime => "unsupported-runtime",
            StubReason::DeprecatedRuntime => "deprecated-runtime",
            StubReason::NoCode => "no-code",
            StubReason::InvalidPackage => "invalid-package",
            StubReason::ImageFunctionUnsupported => "image-function-unsupported",
            StubReason::LayersUnsupported => "layers-unsupported",
            StubReason::ArchitectureUnsupported => "architecture-unsupported",
            StubReason::VmHelperNotFound => "vm-helper-not-found",
            StubReason::VmAgentNotFound => "vm-agent-not-found",
            StubReason::LibkrunUnavailable => "libkrun-unavailable",
            StubReason::VmUnavailable => "vm-unavailable",
            StubReason::ImageUnavailable => "image-unavailable",
            StubReason::ImageCacheUnusable => "image-cache-unusable",
            StubReason::ResponseStreamUnsupported => "response-stream-unsupported",
        }
    }
}

impl std::fmt::Display for StubReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where a function version's ZIP deployment package is stored.
///
/// The bytes are loaded only when an executor needs them, which for an
/// executor that keeps unpacked packages around is once per package.
#[derive(Clone)]
pub struct CodeSource {
    blobs: Arc<BlobStore>,
    key: String,
    sha256: String,
}

impl CodeSource {
    pub(crate) fn new(blobs: Arc<BlobStore>, sha256: &str) -> Self {
        Self {
            blobs,
            key: code_blob_key(sha256),
            sha256: sha256.to_string(),
        }
    }

    /// A source over bytes held in memory, for executor tests.
    pub async fn from_bytes(sha256: &str, bytes: Bytes) -> Self {
        let blobs = Arc::new(BlobStore::mem("lambda-code"));
        let key = code_blob_key(sha256);
        blobs
            .put(&key, bytes)
            .await
            .expect("an in-memory blob store accepts the package");
        Self {
            blobs,
            key,
            sha256: sha256.to_string(),
        }
    }

    /// The base64 SHA-256 of the package ( `CodeSha256` ).
    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    /// The SHA-256 as lowercase hex, safe as a file name.
    pub fn sha256_hex(&self) -> String {
        sha256_hex(&self.sha256)
    }

    /// Read the package.
    pub async fn load(&self) -> Result<Bytes, VfsError> {
        self.blobs.get(&self.key).await
    }
}

impl std::fmt::Debug for CodeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CodeSource")
            .field("sha256", &self.sha256)
            .finish_non_exhaustive()
    }
}

/// Blob store key of the package with base64 SHA-256 `sha256`.
pub(crate) fn code_blob_key(sha256: &str) -> String {
    format!("code/{}", sha256_hex(sha256))
}

/// Lowercase hex of the digest that `sha256` ( base64 ) encodes, or of its
/// bytes when it is not valid base64 ( mock digests ).
fn sha256_hex(sha256: &str) -> String {
    use base64::Engine as _;
    let raw = base64::engine::general_purpose::STANDARD
        .decode(sha256)
        .unwrap_or_else(|_| sha256.as_bytes().to_vec());
    raw.iter().map(|b| format!("{b:02x}")).collect()
}

/// One function version, with everything needed to run it, cloned out of
/// the service state so that no lock is held while it runs.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct FunctionTarget {
    pub function_name: String,
    /// Unqualified function ARN.
    pub function_arn: String,
    /// The version that runs: `$LATEST` or a published version number.
    pub function_version: String,
    pub account_id: String,
    pub region: String,
    pub runtime: String,
    pub handler: String,
    /// `MemorySize`, in MB.
    pub memory_size: u32,
    pub timeout: Duration,
    /// The function's own environment variables.
    pub environment: BTreeMap<String, String>,
    /// `x86_64` or `arm64`.
    pub architecture: String,
    /// The logging configuration's log group.
    pub log_group: String,
    pub code: CodeSource,
}

impl FunctionTarget {
    /// A target for executor tests.  Fields can be adjusted afterwards.
    pub fn for_test(function_name: &str, runtime: &str, handler: &str, code: CodeSource) -> Self {
        Self {
            function_name: function_name.to_string(),
            function_arn: format!("arn:aws:lambda:us-east-1:123456789012:function:{function_name}"),
            function_version: "$LATEST".to_string(),
            account_id: "123456789012".to_string(),
            region: "us-east-1".to_string(),
            runtime: runtime.to_string(),
            handler: handler.to_string(),
            memory_size: 128,
            timeout: Duration::from_secs(3),
            environment: BTreeMap::new(),
            architecture: crate::types::DEFAULT_ARCHITECTURE.to_string(),
            log_group: format!("/aws/lambda/{function_name}"),
            code,
        }
    }

    /// The qualified ARN when the function was invoked through a qualifier.
    pub fn qualified_arn(&self, qualifier: Option<&str>) -> String {
        match qualifier.filter(|q| !q.is_empty()) {
            Some(q) => format!("{}:{q}", self.function_arn),
            None => self.function_arn.clone(),
        }
    }
}

/// One request to run a function.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct InvokeRequest {
    /// The `RequestId` of this invocation.
    pub request_id: String,
    /// The ARN the caller invoked, qualified when a qualifier was given.
    pub invoked_function_arn: String,
    /// The event; `{}` when the request carried no payload.
    pub payload: Bytes,
    /// The raw `X-Amz-Client-Context` header value ( base64 JSON ).
    pub client_context: Option<String>,
    /// `InvocationType: Event`.
    pub asynchronous: bool,
}

impl InvokeRequest {
    pub fn new(invoked_function_arn: impl Into<String>, payload: Bytes) -> Self {
        Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            invoked_function_arn: invoked_function_arn.into(),
            payload: if payload.is_empty() {
                Bytes::from_static(b"{}")
            } else {
                payload
            },
            client_context: None,
            asynchronous: false,
        }
    }

    pub fn with_client_context(mut self, client_context: Option<String>) -> Self {
        self.client_context = client_context;
        self
    }

    pub fn asynchronous(mut self, asynchronous: bool) -> Self {
        self.asynchronous = asynchronous;
        self
    }
}

/// What an invocation produced.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct InvocationOutcome {
    /// The handler's result, or the JSON error document.
    pub payload: Bytes,
    /// `Some("Unhandled")` when the payload is an error document.
    pub function_error: Option<String>,
    /// The last [`LOG_TAIL_BYTES`] of the execution log.
    pub log_tail: Vec<u8>,
    /// `<configured>-><used>` when the function ran on another architecture.
    pub architecture_substituted: Option<String>,
}

impl InvocationOutcome {
    pub fn success(payload: Bytes, log_tail: Vec<u8>) -> Self {
        Self {
            payload,
            function_error: None,
            log_tail,
            architecture_substituted: None,
        }
    }

    pub fn unhandled(payload: Bytes, log_tail: Vec<u8>) -> Self {
        Self {
            payload,
            function_error: Some("Unhandled".to_string()),
            log_tail,
            architecture_substituted: None,
        }
    }

    pub fn with_architecture_substituted(mut self, substituted: Option<String>) -> Self {
        self.architecture_substituted = substituted;
        self
    }
}

/// Runs deployed function code for a [`LambdaService`](crate::LambdaService).
///
/// The service validates the request, rejects what no executor can run (
/// container images, functions with layers, versions without a stored
/// package ) and releases its state lock before calling the executor.
pub trait LambdaExecutor: Send + Sync + 'static {
    /// Make `target` runnable, or say why it cannot be run.
    ///
    /// Called before every invocation, and before `Event` invocations are
    /// acknowledged with 202, so that a function that cannot run is answered
    /// with the stub instead.  Expensive work ( fetching an image, unpacking
    /// a package ) should be done once and remembered.
    fn prepare<'a>(&'a self, target: &'a FunctionTarget) -> BoxFuture<'a, Result<(), StubReason>>;

    /// Run one invocation to completion.
    ///
    /// Failures of the function or its runtime ( an exception, a timeout, a
    /// crash ) are an [`InvocationOutcome`] with `function_error` set.  `Err`
    /// means nothing was run, and the caller answers with the stub.
    ///
    /// Dropping the future abandons the invocation; the executor should
    /// stop whatever was running it.
    fn invoke<'a>(
        &'a self,
        target: FunctionTarget,
        request: InvokeRequest,
    ) -> BoxFuture<'a, Result<InvocationOutcome, StubReason>>;

    /// Forget warm state for a function: its code or configuration changed,
    /// or it was deleted.  `function_arn` is unqualified.
    fn retire<'a>(&'a self, function_arn: &'a str) -> BoxFuture<'a, ()> {
        let _ = function_arn;
        Box::pin(async {})
    }
}

/// The environment variables the Lambda runtime gives a function's
/// execution environment, other than those the base image sets.
///
/// In increasing order of precedence: placeholder credentials ( so that an
/// AWS SDK inside the function does not go looking for an instance
/// profile ), `extra` ( executor configuration, such as `AWS_ENDPOINT_URL` ),
/// the function's own variables, and the variables Lambda reserves.
pub fn function_environment(
    target: &FunctionTarget,
    extra: &BTreeMap<String, String>,
    log_stream: &str,
) -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();
    let mut set = |key: &str, value: &str| {
        env.insert(key.to_string(), value.to_string());
    };
    set("AWS_ACCESS_KEY_ID", "winterbaume");
    set("AWS_SECRET_ACCESS_KEY", "winterbaume");
    set("AWS_SESSION_TOKEN", "winterbaume");
    for (key, value) in extra {
        set(key, value);
    }
    for (key, value) in &target.environment {
        set(key, value);
    }
    // Reserved by the Lambda runtime; not overridable.
    set("TZ", ":UTC");
    set("AWS_REGION", &target.region);
    set("AWS_DEFAULT_REGION", &target.region);
    set("AWS_LAMBDA_FUNCTION_NAME", &target.function_name);
    set("AWS_LAMBDA_FUNCTION_VERSION", &target.function_version);
    set(
        "AWS_LAMBDA_FUNCTION_MEMORY_SIZE",
        &target.memory_size.to_string(),
    );
    set("AWS_LAMBDA_INITIALIZATION_TYPE", "on-demand");
    set("AWS_LAMBDA_LOG_GROUP_NAME", &target.log_group);
    set("AWS_LAMBDA_LOG_STREAM_NAME", log_stream);
    set(
        "AWS_EXECUTION_ENV",
        &format!("AWS_Lambda_{}", target.runtime),
    );
    set("_HANDLER", &target.handler);
    set("LAMBDA_TASK_ROOT", "/var/task");
    set("LAMBDA_RUNTIME_DIR", "/var/runtime");
    env
}

/// A CloudWatch log stream name for a new execution environment.
pub fn log_stream_name(function_version: &str) -> String {
    format!(
        "{}/[{}]{}",
        chrono::Utc::now().format("%Y/%m/%d"),
        function_version,
        uuid::Uuid::new_v4().simple()
    )
}

/// The last few kilobytes of what a function wrote.
#[derive(Debug, Default, Clone)]
pub struct LogTail {
    bytes: Vec<u8>,
}

impl LogTail {
    pub fn push(&mut self, chunk: &[u8]) {
        self.bytes.extend_from_slice(chunk);
        if self.bytes.len() > 4 * LOG_TAIL_BYTES {
            let excess = self.bytes.len() - LOG_TAIL_BYTES;
            self.bytes.drain(..excess);
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// The execution log of one invocation as Lambda frames it, `START` /
/// `END` / `REPORT` around what the function wrote, cut to its last
/// [`LOG_TAIL_BYTES`].
pub fn frame_log(
    request_id: &str,
    function_version: &str,
    output: &[u8],
    duration: Duration,
    memory_size: u32,
) -> Vec<u8> {
    let duration_ms = duration.as_secs_f64() * 1000.0;
    let mut text =
        format!("START RequestId: {request_id} Version: {function_version}\n").into_bytes();
    text.extend_from_slice(output);
    if text.last() != Some(&b'\n') {
        text.push(b'\n');
    }
    text.extend_from_slice(
        format!(
            "END RequestId: {request_id}\nREPORT RequestId: {request_id}\tDuration: {duration_ms:.2} ms\tBilled Duration: {} ms\tMemory Size: {memory_size} MB\tMax Memory Used: {memory_size} MB\t\n",
            duration_ms.ceil() as u64,
        )
        .as_bytes(),
    );
    text[text.len().saturating_sub(LOG_TAIL_BYTES)..].to_vec()
}

/// An error document for a failure of the runtime rather than of the handler
/// ( `Sandbox.Timedout`, `Runtime.ExitError`, ... ).
pub fn runtime_error(request_id: &str, error_type: &str, message: &str) -> Bytes {
    Bytes::from(
        serde_json::json!({
            "errorType": error_type,
            "errorMessage": format!("RequestId: {request_id} Error: {message}"),
        })
        .to_string(),
    )
}

/// Unpack a ZIP deployment package into `dest`.
///
/// Refuses archives with entries that would land outside `dest` ( absolute
/// paths, `..` components ) and archives larger than the Lambda quota.
/// Symbolic links are created last and only when they stay inside `dest`, so
/// that no entry can be written through one.  On failure nothing is left at
/// `dest`.
pub fn unpack_zip(bytes: &[u8], dest: &Path) -> Result<(), String> {
    let result = unpack_zip_into(bytes, dest);
    if result.is_err() {
        let _ = std::fs::remove_dir_all(dest);
    }
    result
}

fn unpack_zip_into(bytes: &[u8], dest: &Path) -> Result<(), String> {
    let mut archive =
        zip::ZipArchive::new(std::io::Cursor::new(bytes)).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;

    let mut remaining = MAX_UNPACKED_BYTES;
    let mut links: Vec<(PathBuf, PathBuf)> = Vec::new();
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).map_err(|e| e.to_string())?;
        let relative = entry
            .enclosed_name()
            .ok_or_else(|| format!("archive entry escapes the package: {}", entry.name()))?;
        let target = dest.join(&relative);
        if entry.is_dir() {
            std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
            continue;
        }
        if entry.is_symlink() {
            let mut link = String::new();
            entry
                .by_ref()
                .take(4096)
                .read_to_string(&mut link)
                .map_err(|e| e.to_string())?;
            links.push((relative, PathBuf::from(link)));
            continue;
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut file = std::fs::File::create(&target).map_err(|e| e.to_string())?;
        let written = std::io::copy(&mut entry.by_ref().take(remaining + 1), &mut file)
            .map_err(|e| e.to_string())?;
        if written > remaining {
            return Err("unpacked deployment package exceeds 262144000 bytes".to_string());
        }
        remaining -= written;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            // Lambda makes package files world-readable; executables keep
            // their execute bits.  setuid / setgid / sticky are dropped.
            let mode = entry.unix_mode().unwrap_or(0o644) & 0o777 | 0o444;
            let _ = file.set_permissions(std::fs::Permissions::from_mode(mode));
        }
    }

    for (relative, link) in links {
        if !link_stays_inside(&relative, &link) {
            tracing::debug!(entry = %relative.display(), "skipping symbolic link that leaves the package");
            continue;
        }
        let target = dest.join(&relative);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        #[cfg(unix)]
        let _ = std::os::unix::fs::symlink(&link, &target);
        #[cfg(not(unix))]
        let _ = (&link, &target);
    }
    Ok(())
}

/// Whether a symbolic link at `relative` ( inside the package ) pointing at
/// `link` resolves, lexically, to somewhere inside the package.
fn link_stays_inside(relative: &Path, link: &Path) -> bool {
    let mut depth: i64 = relative.components().count() as i64 - 1;
    for component in link.components() {
        match component {
            Component::Normal(_) => depth += 1,
            Component::CurDir => {}
            Component::ParentDir => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            Component::RootDir | Component::Prefix(_) => return false,
        }
    }
    true
}

/// Decode an `X-Amz-Client-Context` header ( base64 JSON ).
pub fn decode_client_context(header: Option<&str>) -> Option<serde_json::Value> {
    use base64::Engine as _;
    header
        .and_then(|raw| base64::engine::general_purpose::STANDARD.decode(raw).ok())
        .and_then(|json| serde_json::from_slice(&json).ok())
}

/// Convert a `HashMap` environment to the ordered map executors use.
pub(crate) fn ordered(env: &HashMap<String, String>) -> BTreeMap<String, String> {
    env.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn zip_with(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        for (name, content) in entries {
            writer
                .start_file(*name, zip::write::SimpleFileOptions::default())
                .unwrap();
            writer.write_all(content).unwrap();
        }
        writer.finish().unwrap().into_inner()
    }

    #[test]
    fn unpacks_nested_entries() {
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("pkg");
        unpack_zip(
            &zip_with(&[("index.mjs", b"a"), ("dist/lib/x.js", b"bc")]),
            &dest,
        )
        .unwrap();
        assert_eq!(std::fs::read(dest.join("index.mjs")).unwrap(), b"a");
        assert_eq!(std::fs::read(dest.join("dist/lib/x.js")).unwrap(), b"bc");
    }

    #[test]
    fn rejects_entries_that_escape_the_package() {
        for name in ["../evil.txt", "a/../../evil.txt", "/abs/evil.txt"] {
            let dir = tempfile::tempdir().unwrap();
            let dest = dir.path().join("root").join("pkg");
            let result = unpack_zip(&zip_with(&[("ok.txt", b"ok"), (name, b"evil")]), &dest);
            if name.starts_with('/') {
                // The zip crate strips a leading `/`, which keeps the entry
                // inside the package; either outcome is safe.
                if result.is_ok() {
                    assert!(dest.join("abs/evil.txt").is_file());
                }
            } else {
                assert!(result.is_err(), "{name} must be rejected");
                assert!(!dest.exists(), "a rejected package leaves nothing behind");
            }
            assert!(!dir.path().join("evil.txt").exists());
            assert!(!dir.path().join("root").join("evil.txt").exists());
        }
    }

    #[test]
    fn rejects_garbage() {
        let dir = tempfile::tempdir().unwrap();
        assert!(unpack_zip(&[1u8; 10], &dir.path().join("pkg")).is_err());
    }

    #[test]
    fn symlinks_must_stay_inside() {
        assert!(link_stays_inside(
            Path::new("node_modules/.bin/tool"),
            Path::new("../tool/cli.js")
        ));
        assert!(link_stays_inside(Path::new("a"), Path::new("b/c")));
        assert!(!link_stays_inside(Path::new("a"), Path::new("../b")));
        assert!(!link_stays_inside(
            Path::new("a/b"),
            Path::new("../../etc/passwd")
        ));
        assert!(!link_stays_inside(Path::new("a"), Path::new("/etc/passwd")));
    }

    #[test]
    fn log_tail_keeps_only_the_end() {
        let mut tail = LogTail::default();
        for i in 0..10_000u32 {
            tail.push(format!("line {i}\n").as_bytes());
        }
        assert!(tail.as_bytes().len() <= 4 * LOG_TAIL_BYTES);
        assert!(tail.as_bytes().ends_with(b"line 9999\n"));
    }

    #[test]
    fn frame_log_wraps_output_and_keeps_the_report() {
        let framed = frame_log("r-1", "$LATEST", b"hello", Duration::from_millis(12), 256);
        let text = String::from_utf8(framed).unwrap();
        assert!(
            text.starts_with("START RequestId: r-1 Version: $LATEST\nhello\nEND RequestId: r-1\n")
        );
        assert!(text.contains("REPORT RequestId: r-1\tDuration: 12.00 ms"));
        assert!(text.contains("Memory Size: 256 MB"));

        let long = vec![b'x'; 3 * LOG_TAIL_BYTES];
        let framed = frame_log("r-2", "1", &long, Duration::ZERO, 128);
        assert_eq!(framed.len(), LOG_TAIL_BYTES);
        assert!(String::from_utf8_lossy(&framed).contains("REPORT RequestId: r-2"));
    }

    #[tokio::test]
    async fn reserved_variables_win_over_the_function_environment() {
        let code = CodeSource::from_bytes("abc", Bytes::new()).await;
        let mut target = FunctionTarget::for_test("f", "nodejs22.x", "index.handler", code);
        target
            .environment
            .insert("AWS_REGION".to_string(), "mars-1".to_string());
        target
            .environment
            .insert("MINE".to_string(), "1".to_string());
        let extra = BTreeMap::from([
            (
                "AWS_ENDPOINT_URL".to_string(),
                "http://127.0.0.1:1".to_string(),
            ),
            ("MINE".to_string(), "0".to_string()),
        ]);
        let env = function_environment(&target, &extra, "stream");
        assert_eq!(env["AWS_REGION"], "us-east-1");
        assert_eq!(env["MINE"], "1");
        assert_eq!(env["AWS_ENDPOINT_URL"], "http://127.0.0.1:1");
        assert_eq!(env["_HANDLER"], "index.handler");
        assert_eq!(env["AWS_LAMBDA_LOG_STREAM_NAME"], "stream");
        assert_eq!(env["AWS_EXECUTION_ENV"], "AWS_Lambda_nodejs22.x");
    }

    #[tokio::test]
    async fn code_source_round_trips_bytes() {
        let source = CodeSource::from_bytes("q83vEjRWeJA=", Bytes::from_static(b"zip")).await;
        assert_eq!(source.sha256_hex(), "abcdef1234567890");
        let loaded = source.load().await.unwrap();
        assert_eq!(&loaded[..], b"zip");
    }
}
