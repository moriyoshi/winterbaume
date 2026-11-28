//! Fetching and unpacking OCI images: just enough of the registry protocol
//! and the image layout to obtain the stock AWS Lambda base images.

mod layout;
mod registry;
mod unpack;

pub use layout::OciLayout;
pub use registry::RegistryClient;
pub use unpack::{apply_layer, resolve_in_root};

use serde::{Deserialize, Serialize};

/// A repository and tag in one registry, e.g. `lambda/nodejs` and `22`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageRef {
    pub repository: String,
    pub tag: String,
}

/// The CPU architecture of an image, in OCI platform terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    Amd64,
    Arm64,
}

impl Arch {
    /// The OCI `platform.architecture` value.
    pub fn oci_name(self) -> &'static str {
        match self {
            Arch::Amd64 => "amd64",
            Arch::Arm64 => "arm64",
        }
    }

    /// The Lambda `Architectures` value.
    pub fn lambda_name(self) -> &'static str {
        match self {
            Arch::Amd64 => "x86_64",
            Arch::Arm64 => "arm64",
        }
    }

    /// From a Lambda `Architectures` value.
    pub fn from_lambda_name(name: &str) -> Option<Self> {
        match name {
            "x86_64" => Some(Arch::Amd64),
            "arm64" => Some(Arch::Arm64),
            _ => None,
        }
    }

    /// The architecture this process runs on, if it is one of the two.
    pub fn host() -> Option<Self> {
        match std::env::consts::ARCH {
            "x86_64" => Some(Arch::Amd64),
            "aarch64" => Some(Arch::Arm64),
            _ => None,
        }
    }
}

/// One layer of an image manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Layer {
    /// `sha256:<hex>`.
    pub digest: String,
    pub media_type: String,
    pub size: u64,
}

/// A platform-specific image manifest, resolved from a tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedManifest {
    /// `sha256:<hex>` of the platform-specific manifest ( not of the index ).
    pub digest: String,
    /// `sha256:<hex>` of the image configuration blob.
    pub config_digest: String,
    /// In application order, lowest first.
    pub layers: Vec<Layer>,
}

/// The part of an image configuration that decides how a container starts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageConfig {
    /// `KEY=value` entries.
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default)]
    pub working_dir: Option<String>,
    #[serde(default)]
    pub entrypoint: Vec<String>,
    #[serde(default)]
    pub cmd: Vec<String>,
}

impl ImageConfig {
    /// Parse the `config` member of an OCI / Docker image configuration
    /// document.
    pub fn from_config_blob(blob: &[u8]) -> Result<Self, OciError> {
        #[derive(Deserialize)]
        struct Document {
            #[serde(default)]
            config: Option<Inner>,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Inner {
            #[serde(default)]
            env: Option<Vec<String>>,
            #[serde(default)]
            working_dir: Option<String>,
            #[serde(default)]
            entrypoint: Option<Vec<String>>,
            #[serde(default)]
            cmd: Option<Vec<String>>,
        }
        let document: Document = serde_json::from_slice(blob)
            .map_err(|e| OciError::Invalid(format!("image configuration: {e}")))?;
        let inner = document.config.unwrap_or(Inner {
            env: None,
            working_dir: None,
            entrypoint: None,
            cmd: None,
        });
        Ok(Self {
            env: inner.env.unwrap_or_default(),
            working_dir: inner.working_dir.filter(|w| !w.is_empty()),
            entrypoint: inner.entrypoint.unwrap_or_default(),
            cmd: inner.cmd.unwrap_or_default(),
        })
    }

    /// The environment as key / value pairs, in order.
    pub fn env_pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        self.env.iter().filter_map(|entry| entry.split_once('='))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OciError {
    /// The registry could not be reached or answered with an error.
    #[error("registry request failed: {0}")]
    Registry(String),
    /// The tag, or the requested platform, does not exist.
    #[error("not found: {0}")]
    NotFound(String),
    /// A document or blob is malformed, or its digest does not match.
    #[error("invalid image data: {0}")]
    Invalid(String),
    /// A layer entry would be written outside the root file system.
    #[error("unsafe layer entry: {0}")]
    Unsafe(String),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

/// Media types this module understands.
pub mod media_types {
    pub const OCI_INDEX: &str = "application/vnd.oci.image.index.v1+json";
    pub const OCI_MANIFEST: &str = "application/vnd.oci.image.manifest.v1+json";
    pub const DOCKER_LIST: &str = "application/vnd.docker.distribution.manifest.list.v2+json";
    pub const DOCKER_MANIFEST: &str = "application/vnd.docker.distribution.manifest.v2+json";
    pub const OCI_LAYER_GZIP: &str = "application/vnd.oci.image.layer.v1.tar+gzip";
    pub const OCI_LAYER_TAR: &str = "application/vnd.oci.image.layer.v1.tar";
    pub const DOCKER_LAYER_GZIP: &str = "application/vnd.docker.image.rootfs.diff.tar.gzip";
}
