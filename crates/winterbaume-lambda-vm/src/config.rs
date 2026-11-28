//! Configuration of a [`KrunExecutor`](crate::KrunExecutor).

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The public registry the stock AWS Lambda base images are pulled from.
pub const DEFAULT_REGISTRY: &str = "https://public.ecr.aws";
/// File name of the stand-alone VM helper program.
pub const HELPER_PROGRAM: &str = "winterbaume-lambda-vm-helper";
/// Environment variable naming the VM helper program.
pub const HELPER_ENV: &str = "WB_LAMBDA_VM_HELPER";
/// File name of the guest agent.
pub const AGENT_PROGRAM: &str = "winterbaume-lambda-vm-agent";
/// Environment variable naming the guest agent.
pub const AGENT_ENV: &str = "WB_LAMBDA_VM_AGENT";

/// The guest agent built into this binary, when it was built with
/// `WINTERBAUME_LAMBDA_VM_AGENT` naming one ( release builds are ).
pub fn embedded_agent() -> Option<&'static [u8]> {
    #[cfg(embedded_agent)]
    {
        Some(include_bytes!(env!("WINTERBAUME_EMBEDDED_AGENT")))
    }
    #[cfg(not(embedded_agent))]
    {
        None
    }
}

/// What to do with a function whose `Architectures` differs from the host's.
///
/// libkrun runs guests on the host's own instruction set; it cannot emulate
/// another one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArchMismatch {
    /// Answer with the `architecture-unsupported` stub.
    #[default]
    Stub,
    /// Run the function on the host's architecture anyway, with the base
    /// image for that architecture ( the `lambda-run-on-host-architecture`
    /// relaxation ), and say so in the
    /// `x-winterbaume-lambda-architecture-substituted` response header.
    /// Fine for interpreted code; native modules built for the configured
    /// architecture will fail to load.
    Native,
}

impl ArchMismatch {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "stub" => Some(Self::Stub),
            "native" => Some(Self::Native),
            _ => None,
        }
    }
}

/// How to start the VM helper: a program and the arguments that put it in
/// helper mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelperCommand {
    pub program: PathBuf,
    pub args: Vec<OsString>,
}

impl HelperCommand {
    /// A stand-alone helper program, run without arguments.
    pub fn program(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
        }
    }

    /// A program that acts as the helper when given `args`, such as a server
    /// binary that calls [`crate::helper::helper_main`] for a hidden
    /// subcommand.
    pub fn with_args(
        program: impl Into<PathBuf>,
        args: impl IntoIterator<Item = impl Into<OsString>>,
    ) -> Self {
        Self {
            program: program.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }

    /// The helper from [`HELPER_ENV`], next to the running executable, or on
    /// `PATH`, in that order.
    pub fn locate() -> Option<Self> {
        if let Some(explicit) = std::env::var_os(HELPER_ENV).filter(|v| !v.is_empty()) {
            let path = PathBuf::from(explicit);
            return path.is_file().then(|| Self::program(path));
        }
        let name = format!("{HELPER_PROGRAM}{}", std::env::consts::EXE_SUFFIX);
        if let Some(sibling) = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|dir| dir.join(&name)))
            .filter(|p| p.is_file())
        {
            return Some(Self::program(sibling));
        }
        std::env::split_paths(&std::env::var_os("PATH")?)
            .map(|dir| dir.join(&name))
            .find(|candidate| candidate.is_file())
            .map(Self::program)
    }
}

/// Settings of a [`KrunExecutor`](crate::KrunExecutor).
#[derive(Debug, Clone)]
pub struct KrunExecutorConfig {
    pub(crate) helper: Option<HelperCommand>,
    pub(crate) libkrun: Option<PathBuf>,
    pub(crate) cache_dir: Option<PathBuf>,
    pub(crate) image_dir: Option<PathBuf>,
    pub(crate) offline: bool,
    pub(crate) registry: String,
    pub(crate) warm_ttl: Duration,
    pub(crate) max_vms: usize,
    pub(crate) max_vms_per_function: usize,
    pub(crate) arch_mismatch: ArchMismatch,
    pub(crate) env: BTreeMap<String, String>,
    pub(crate) init_timeout: Duration,
    pub(crate) agent: Option<PathBuf>,
    pub(crate) network: bool,
    pub(crate) aws_endpoint: Option<crate::AwsEndpoint>,
}

impl Default for KrunExecutorConfig {
    fn default() -> Self {
        let cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        Self {
            helper: None,
            libkrun: None,
            cache_dir: None,
            image_dir: None,
            offline: false,
            registry: DEFAULT_REGISTRY.to_string(),
            warm_ttl: Duration::from_secs(300),
            max_vms: cpus.max(2),
            max_vms_per_function: cpus.max(2),
            arch_mismatch: ArchMismatch::default(),
            env: BTreeMap::new(),
            init_timeout: Duration::from_secs(30),
            agent: None,
            network: false,
            aws_endpoint: None,
        }
    }
}

impl KrunExecutorConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// The VM helper to start.  Defaults to [`HelperCommand::locate`].
    pub fn with_helper(mut self, helper: HelperCommand) -> Self {
        self.helper = Some(helper);
        self
    }

    /// An explicit libkrun library, instead of the dynamic loader's search
    /// ( which on macOS does not include Homebrew's prefix ).
    pub fn with_libkrun(mut self, path: impl Into<PathBuf>) -> Self {
        self.libkrun = Some(path.into());
        self
    }

    /// Where pulled images are unpacked and kept between runs.  Defaults to
    /// `$XDG_CACHE_HOME/winterbaume/lambda` ( `~/.cache/...` ) on Linux and
    /// `~/Library/Caches/winterbaume/lambda` on macOS.  It must be on a
    /// case-sensitive file system.
    pub fn with_cache_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.cache_dir = Some(dir.into());
        self
    }

    /// A directory of images to use instead of pulling them:
    /// `<dir>/<runtime>/<arch>/` holding either an unpacked `rootfs/` ( with
    /// an optional OCI image `config.json` beside it ) or an OCI image layout
    /// ( `index.json`, `blobs/` ).  `<arch>` is `x86_64` or `arm64`.
    pub fn with_image_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.image_dir = Some(dir.into());
        self
    }

    /// Never contact a registry; use only cached and [`Self::with_image_dir`]
    /// images.
    pub fn offline(mut self, offline: bool) -> Self {
        self.offline = offline;
        self
    }

    /// The registry to pull base images from ( a mirror of
    /// `public.ecr.aws` ), as a base URL.
    pub fn with_registry(mut self, base_url: impl Into<String>) -> Self {
        self.registry = base_url.into().trim_end_matches('/').to_string();
        self
    }

    /// How long an idle execution environment is kept for reuse.
    pub fn with_warm_ttl(mut self, ttl: Duration) -> Self {
        self.warm_ttl = ttl;
        self
    }

    /// The most microVMs running at once, over all functions.
    pub fn with_max_vms(mut self, max: usize) -> Self {
        self.max_vms = max.max(1);
        self
    }

    /// The most microVMs running at once for one function.
    pub fn with_max_vms_per_function(mut self, max: usize) -> Self {
        self.max_vms_per_function = max.max(1);
        self
    }

    pub fn with_arch_mismatch(mut self, policy: ArchMismatch) -> Self {
        self.arch_mismatch = policy;
        self
    }

    /// Apply the Lambda relaxations in `relaxations`: with
    /// [`RUN_ON_HOST_ARCHITECTURE`](winterbaume_lambda::RUN_ON_HOST_ARCHITECTURE),
    /// [`ArchMismatch::Native`].
    pub fn with_relaxations(mut self, relaxations: &winterbaume_core::Relaxations) -> Self {
        if relaxations.is_enabled(winterbaume_lambda::RUN_ON_HOST_ARCHITECTURE) {
            self.arch_mismatch = ArchMismatch::Native;
        }
        self
    }

    /// Set an environment variable in every function's execution
    /// environment, such as `AWS_ENDPOINT_URL` pointing back at the emulator.
    /// A function's own variables and those Lambda reserves take precedence.
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    /// The guest agent binary: a static Linux executable for the host's
    /// architecture, built from the `winterbaume-lambda-vm-agent` crate.
    /// Defaults to [`AGENT_ENV`], then `winterbaume-lambda-vm-agent` next to
    /// the VM helper or the running executable.
    pub fn with_agent(mut self, path: impl Into<PathBuf>) -> Self {
        self.agent = Some(path.into());
        self
    }

    /// Give functions the host's network, as the user running the emulator.
    /// Off by default: functions reach only the Runtime API and the
    /// [`AwsEndpoint`](crate::AwsEndpoint).
    pub fn with_network(mut self, network: bool) -> Self {
        self.network = network;
        self
    }

    /// The services that functions' AWS SDK calls go to: functions get
    /// `AWS_ENDPOINT_URL` pointing at them once the endpoint is connected.
    pub fn with_aws_endpoint(mut self, endpoint: crate::AwsEndpoint) -> Self {
        self.aws_endpoint = Some(endpoint);
        self
    }

    /// How long a new execution environment may take to boot and initialise
    /// before its first invocation fails.
    pub fn with_init_timeout(mut self, timeout: Duration) -> Self {
        self.init_timeout = timeout;
        self
    }

    pub(crate) fn effective_cache_dir(&self) -> Option<PathBuf> {
        if let Some(dir) = &self.cache_dir {
            return Some(dir.clone());
        }
        default_cache_dir()
    }

    pub fn helper(&self) -> Option<&HelperCommand> {
        self.helper.as_ref()
    }

    pub fn libkrun(&self) -> Option<&Path> {
        self.libkrun.as_deref()
    }
}

fn default_cache_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    if cfg!(target_os = "macos") {
        return home.map(|h| h.join("Library/Caches/winterbaume/lambda"));
    }
    std::env::var_os("XDG_CACHE_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| home.map(|h| h.join(".cache")))
        .map(|base| base.join("winterbaume/lambda"))
}
