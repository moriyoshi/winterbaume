//! [`KrunExecutor`]: the [`LambdaExecutor`] that runs functions in microVMs.

use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use tokio::sync::OnceCell;
use winterbaume_lambda::execution::{BoxFuture, function_environment, log_stream_name};
use winterbaume_lambda::{
    FunctionTarget, InvocationOutcome, InvokeRequest, LambdaExecutor, StubReason,
};

use crate::cache::{BaseImage, ImageCache};
use crate::config::{ArchMismatch, HelperCommand, KrunExecutorConfig};
use crate::instance::{Call, Instance};
use crate::oci::Arch;
use crate::pool::{Pool, PoolKey};
use crate::proto::{HelperStatus, VmSpec};
use crate::runtimes;

/// Memory the guest kernel needs on top of the function's `MemorySize`.
const GUEST_OVERHEAD_MIB: u32 = 64;
/// Memory per vCPU, as Lambda allocates CPU: one vCPU at 1769 MB.
const MIB_PER_VCPU: u32 = 1769;
const MAX_VCPUS: u32 = 6;

/// Runs Lambda functions in libkrun microVMs booted from the stock AWS
/// Lambda base images.
///
/// Execution environments are kept warm and reused for later invocations
/// of the same function version, as on AWS: module-level state survives
/// between invocations of one environment.  An environment that failed,
/// timed out, or whose function was updated or deleted is discarded.
///
/// ```no_run
/// use std::sync::Arc;
/// use winterbaume_lambda::LambdaService;
/// use winterbaume_lambda_vm::{KrunExecutor, KrunExecutorConfig};
///
/// let executor = KrunExecutor::new(
///     KrunExecutorConfig::new().with_env("AWS_ENDPOINT_URL", "http://127.0.0.1:5555"),
/// );
/// let service = LambdaService::new().with_code_execution(Arc::new(executor));
/// # let _ = service;
/// ```
pub struct KrunExecutor {
    config: KrunExecutorConfig,
    helper: Option<HelperCommand>,
    agent: Option<PathBuf>,
    embedded_agent: OnceLock<Option<PathBuf>>,
    probe: OnceCell<Result<(), StubReason>>,
    cache: OnceLock<Result<Arc<ImageCache>, StubReason>>,
    pool: Arc<Pool>,
}

/// A function made runnable by [`KrunExecutor::prepare`].
struct Prepared {
    root: PathBuf,
    base: Arc<BaseImage>,
    bootstrap: Vec<&'static str>,
    substituted: Option<String>,
}

impl KrunExecutor {
    pub fn new(config: KrunExecutorConfig) -> Self {
        let helper = config.helper.clone().or_else(HelperCommand::locate);
        let agent = config
            .agent
            .clone()
            .or_else(|| locate_agent(helper.as_ref()));
        let pool = Pool::new(config.max_vms, config.max_vms_per_function, config.warm_ttl);
        Self {
            config,
            helper,
            agent,
            embedded_agent: OnceLock::new(),
            probe: OnceCell::new(),
            cache: OnceLock::new(),
            pool,
        }
    }

    /// The settings this executor was created with.
    pub fn config(&self) -> &KrunExecutorConfig {
        &self.config
    }

    /// Check, once, that a VM can be started here: the helper exists,
    /// libkrun loads and the hypervisor is usable.
    pub async fn probe(&self) -> Result<(), StubReason> {
        *self
            .probe
            .get_or_init(|| async {
                let Some(helper) = &self.helper else {
                    tracing::warn!(
                        "no Lambda VM helper found; set {} or install {}",
                        crate::config::HELPER_ENV,
                        crate::config::HELPER_PROGRAM
                    );
                    return Err(StubReason::VmHelperNotFound);
                };
                probe_helper(helper, self.config.libkrun.clone()).await
            })
            .await
    }

    /// Number of idle execution environments.
    pub fn idle_environments(&self) -> usize {
        self.pool.idle_count()
    }

    /// The agent to give VMs: the configured one; else one found through
    /// `WB_LAMBDA_VM_AGENT` or next to the helper; else the one built into
    /// this binary, written once into the cache directory ( a helper other
    /// than this binary does not have it built in ).
    fn agent_path(&self) -> Option<PathBuf> {
        if let Some(path) = self.agent.as_deref().filter(|p| p.is_file()) {
            return Some(path.to_path_buf());
        }
        // An agent configured explicitly must be that one.
        if self.config.agent.is_some() {
            return None;
        }
        let bytes = crate::config::embedded_agent()?;
        self.embedded_agent
            .get_or_init(|| {
                use sha2::Digest as _;
                let dir = self
                    .config
                    .effective_cache_dir()?
                    .join(crate::cache::CACHE_FORMAT)
                    .join("agent");
                let hex: String = sha2::Sha256::digest(bytes)[..8]
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect();
                let path = dir.join(format!("{}-{hex}", crate::config::AGENT_PROGRAM));
                if !path.is_file() {
                    std::fs::create_dir_all(&dir).ok()?;
                    let staging = dir.join(format!(".{}", uuid::Uuid::new_v4().simple()));
                    std::fs::write(&staging, bytes).ok()?;
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(&staging, std::fs::Permissions::from_mode(0o755))
                            .ok()?;
                    }
                    if std::fs::rename(&staging, &path).is_err() {
                        let _ = std::fs::remove_file(&staging);
                    }
                }
                path.is_file().then_some(path)
            })
            .clone()
    }

    fn cache(&self) -> Result<Arc<ImageCache>, StubReason> {
        self.cache
            .get_or_init(|| {
                let Some(dir) = self.config.effective_cache_dir() else {
                    tracing::warn!("no cache directory for Lambda base images; set one");
                    return Err(StubReason::ImageCacheUnusable);
                };
                let registry = (!self.config.offline).then_some(self.config.registry.as_str());
                ImageCache::open(&dir, registry, self.config.image_dir.as_deref()).map(Arc::new)
            })
            .clone()
    }

    async fn resolve(&self, target: &FunctionTarget) -> Result<Prepared, StubReason> {
        self.probe().await?;
        if self.agent_path().is_none() {
            tracing::warn!(
                "no Lambda VM guest agent found; set {} or build {}",
                crate::config::AGENT_ENV,
                crate::config::AGENT_PROGRAM
            );
            return Err(StubReason::VmAgentNotFound);
        }
        let image = runtimes::image_for(&target.runtime)?;
        let wanted = Arch::from_lambda_name(&target.architecture)
            .ok_or(StubReason::ArchitectureUnsupported)?;
        let host = Arch::host().ok_or(StubReason::VmUnavailable)?;
        let substituted = if wanted == host {
            None
        } else {
            match self.config.arch_mismatch {
                ArchMismatch::Stub => return Err(StubReason::ArchitectureUnsupported),
                ArchMismatch::Native => {
                    Some(format!("{}->{}", wanted.lambda_name(), host.lambda_name()))
                }
            }
        };
        let cache = self.cache()?;
        let base = cache.base_image(&target.runtime, &image, host).await?;
        let root = cache.template(&base, &target.code).await?;
        Ok(Prepared {
            root,
            base,
            bootstrap: image.bootstrap,
            substituted,
        })
    }

    fn vm_spec(&self, target: &FunctionTarget, prepared: &Prepared) -> VmSpec {
        let mut env: BTreeMap<String, String> = prepared
            .base
            .config
            .env_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let log_stream = log_stream_name(&target.function_version);
        env.extend(function_environment(target, &self.config.env, &log_stream));
        let cpus = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(1);
        let vcpus = target
            .memory_size
            .div_ceil(MIB_PER_VCPU)
            .clamp(1, MAX_VCPUS.min(cpus.max(1)));
        VmSpec {
            libkrun: self.config.libkrun.clone(),
            root: prepared.root.clone(),
            vcpus: vcpus as u8,
            ram_mib: target.memory_size + GUEST_OVERHEAD_MIB,
            env,
            bootstrap: prepared.bootstrap.iter().map(|b| b.to_string()).collect(),
            workdir: prepared
                .base
                .config
                .working_dir
                .clone()
                .unwrap_or_else(|| "/var/task".to_string()),
            log_level: 0,
            probe: false,
            agent: self.agent_path(),
            // Added by the environment, with its sockets.
            forwards: Vec::new(),
            network: self.config.network,
        }
    }

    async fn run(
        &self,
        target: FunctionTarget,
        request: InvokeRequest,
    ) -> Result<InvocationOutcome, StubReason> {
        let prepared = self.resolve(&target).await?;
        let spec = self.vm_spec(&target, &prepared);
        let aws = self
            .config
            .aws_endpoint
            .as_ref()
            .and_then(|endpoint| endpoint.dispatcher());
        let key = PoolKey {
            function_arn: target.function_arn.clone(),
            function_version: target.function_version.clone(),
            fingerprint: fingerprint(&spec, aws.is_some()),
        };
        let helper = self.helper.clone().ok_or(StubReason::VmHelperNotFound)?;
        let init_timeout = self.config.init_timeout;
        let mut lease = self
            .pool
            .acquire(key, |generation, permits| async move {
                Instance::start(&helper, spec, aws, init_timeout, generation, permits).await
            })
            .await?;
        let outcome = lease
            .instance
            .invoke(Call {
                request_id: request.request_id,
                invoked_function_arn: request.invoked_function_arn,
                payload: request.payload,
                client_context: request.client_context,
                timeout: target.timeout,
                function_version: target.function_version,
                memory_size: target.memory_size,
            })
            .await;
        self.pool.release(lease);
        Ok(outcome.with_architecture_substituted(prepared.substituted))
    }
}

impl LambdaExecutor for KrunExecutor {
    fn prepare<'a>(&'a self, target: &'a FunctionTarget) -> BoxFuture<'a, Result<(), StubReason>> {
        Box::pin(async move { self.resolve(target).await.map(|_| ()) })
    }

    fn invoke<'a>(
        &'a self,
        target: FunctionTarget,
        request: InvokeRequest,
    ) -> BoxFuture<'a, Result<InvocationOutcome, StubReason>> {
        Box::pin(self.run(target, request))
    }

    fn retire<'a>(&'a self, function_arn: &'a str) -> BoxFuture<'a, ()> {
        self.pool.retire(function_arn);
        Box::pin(async {})
    }
}

/// Everything an environment was started with, except the per-environment
/// log stream name.
fn fingerprint(spec: &VmSpec, aws_endpoint: bool) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    aws_endpoint.hash(&mut hasher);
    spec.network.hash(&mut hasher);
    spec.root.hash(&mut hasher);
    spec.vcpus.hash(&mut hasher);
    spec.ram_mib.hash(&mut hasher);
    spec.bootstrap.hash(&mut hasher);
    spec.workdir.hash(&mut hasher);
    for (k, v) in &spec.env {
        if k != "AWS_LAMBDA_LOG_STREAM_NAME" {
            k.hash(&mut hasher);
            v.hash(&mut hasher);
        }
    }
    hasher.finish()
}

/// The guest agent from [`crate::config::AGENT_ENV`], or next to the helper
/// program or the running executable.
fn locate_agent(helper: Option<&HelperCommand>) -> Option<PathBuf> {
    if let Some(explicit) = std::env::var_os(crate::config::AGENT_ENV).filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(explicit));
    }
    let beside = |program: &std::path::Path| {
        program
            .parent()
            .map(|dir| dir.join(crate::config::AGENT_PROGRAM))
            .filter(|p| p.is_file())
    };
    helper
        .and_then(|h| beside(&h.program))
        .or_else(|| std::env::current_exe().ok().and_then(|exe| beside(&exe)))
}

/// Run the helper in probe mode.
async fn probe_helper(helper: &HelperCommand, libkrun: Option<PathBuf>) -> Result<(), StubReason> {
    use tokio::io::AsyncWriteExt;
    let mut child = tokio::process::Command::new(&helper.program)
        .args(&helper.args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| {
            tracing::warn!(helper = %helper.program.display(), error = %e, "cannot start the Lambda VM helper");
            StubReason::VmHelperNotFound
        })?;
    if let Some(mut stdin) = child.stdin.take() {
        let spec = serde_json::to_vec(&VmSpec::probe(libkrun)).expect("a VM spec serialises");
        let _ = stdin.write_all(&spec).await;
        let _ = stdin.shutdown().await;
    }
    let output = tokio::time::timeout(Duration::from_secs(15), child.wait_with_output())
        .await
        .map_err(|_| StubReason::VmHelperNotFound)?
        .map_err(|_| StubReason::VmHelperNotFound)?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let status = stderr.lines().find_map(HelperStatus::parse);
    match status {
        Some(HelperStatus::Booting) => Ok(()),
        Some(HelperStatus::Failed { reason, message }) => {
            tracing::warn!(%reason, %message, "Lambda functions cannot run in microVMs here");
            Err(match reason.as_str() {
                "libkrun-unavailable" => StubReason::LibkrunUnavailable,
                _ => StubReason::VmUnavailable,
            })
        }
        None => {
            tracing::warn!(
                helper = %helper.program.display(),
                output = %stderr,
                "the program is not a Lambda VM helper"
            );
            Err(StubReason::VmHelperNotFound)
        }
    }
}
