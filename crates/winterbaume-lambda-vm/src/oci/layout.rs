//! Reading an OCI image layout directory ( `oci-layout`, `index.json`,
//! `blobs/sha256/<hex>` ), as written by `skopeo copy ... oci:DIR`.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::{Arch, Layer, OciError, ResolvedManifest, media_types};

/// An OCI image layout on disk.
#[derive(Debug, Clone)]
pub struct OciLayout {
    root: PathBuf,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Descriptor {
    #[serde(default)]
    pub media_type: Option<String>,
    pub digest: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub platform: Option<Platform>,
}

#[derive(Deserialize)]
pub(crate) struct Platform {
    #[serde(default)]
    pub os: String,
    #[serde(default)]
    pub architecture: String,
}

/// An index ( manifest list ) or a manifest; which members are present
/// tells them apart when `mediaType` is missing.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Document {
    #[serde(default)]
    pub media_type: Option<String>,
    #[serde(default)]
    pub manifests: Option<Vec<Descriptor>>,
    #[serde(default)]
    pub config: Option<Descriptor>,
    #[serde(default)]
    pub layers: Option<Vec<Descriptor>>,
}

impl Document {
    pub(crate) fn parse(bytes: &[u8]) -> Result<Self, OciError> {
        serde_json::from_slice(bytes).map_err(|e| OciError::Invalid(format!("manifest: {e}")))
    }

    pub(crate) fn is_index(&self) -> bool {
        match self.media_type.as_deref() {
            Some(media_types::OCI_INDEX | media_types::DOCKER_LIST) => true,
            Some(_) => false,
            None => self.manifests.is_some() && self.config.is_none(),
        }
    }

    /// The manifest descriptor for `linux/<arch>`, ignoring attestation
    /// entries ( platform `unknown/unknown` ).
    pub(crate) fn select(&self, arch: Arch) -> Option<&Descriptor> {
        self.manifests.as_deref()?.iter().find(|d| {
            d.platform
                .as_ref()
                .is_some_and(|p| p.os == "linux" && p.architecture == arch.oci_name())
        })
    }

    pub(crate) fn into_resolved(self, digest: String) -> Result<ResolvedManifest, OciError> {
        let config = self
            .config
            .ok_or_else(|| OciError::Invalid("manifest has no config".to_string()))?;
        let layers = self
            .layers
            .unwrap_or_default()
            .into_iter()
            .map(|l| Layer {
                digest: l.digest,
                media_type: l
                    .media_type
                    .unwrap_or_else(|| media_types::OCI_LAYER_GZIP.to_string()),
                size: l.size,
            })
            .collect();
        Ok(ResolvedManifest {
            digest,
            config_digest: config.digest,
            layers,
        })
    }
}

/// `sha256:<hex>` of `bytes`.
pub(crate) fn sha256_digest(bytes: &[u8]) -> String {
    let hash = Sha256::digest(bytes);
    let hex: String = hash.iter().map(|b| format!("{b:02x}")).collect();
    format!("sha256:{hex}")
}

/// The hex part of a `sha256:<64 lowercase hex>` digest, or an error for
/// anything else.
pub(crate) fn digest_hex(digest: &str) -> Result<&str, OciError> {
    let hex = digest
        .strip_prefix("sha256:")
        .ok_or_else(|| OciError::Invalid(format!("unsupported digest: {digest}")))?;
    if hex.len() == 64 && hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        Ok(hex)
    } else {
        Err(OciError::Invalid(format!("malformed digest: {digest}")))
    }
}

/// How deeply nested indexes may be.
const MAX_INDEX_DEPTH: usize = 4;

impl OciLayout {
    /// Open the layout at `dir`.
    pub fn open(dir: &Path) -> Result<Self, OciError> {
        if !dir.join("index.json").is_file() {
            return Err(OciError::NotFound(format!(
                "{} is not an OCI image layout ( no index.json )",
                dir.display()
            )));
        }
        Ok(Self {
            root: dir.to_path_buf(),
        })
    }

    /// The `linux/<arch>` image of the layout.
    pub fn resolve(&self, arch: Arch) -> Result<ResolvedManifest, OciError> {
        let index = std::fs::read(self.root.join("index.json"))?;
        let mut document = Document::parse(&index)?;
        for _ in 0..MAX_INDEX_DEPTH {
            let manifests = document.manifests.as_deref().unwrap_or_default();
            let chosen = match document.select(arch) {
                Some(d) => d,
                // A single manifest without a platform is taken as is.
                None if manifests.len() == 1 && manifests[0].platform.is_none() => &manifests[0],
                None => {
                    return Err(OciError::NotFound(format!(
                        "no linux/{} image in {}",
                        arch.oci_name(),
                        self.root.display()
                    )));
                }
            };
            let digest = chosen.digest.clone();
            let bytes = self.read_blob(&digest)?;
            let next = Document::parse(&bytes)?;
            if next.is_index() {
                document = next;
                continue;
            }
            return next.into_resolved(digest);
        }
        Err(OciError::Invalid(
            "image indexes nested too deeply".to_string(),
        ))
    }

    /// Where the blob with `digest` is stored.
    pub fn blob_path(&self, digest: &str) -> Result<PathBuf, OciError> {
        let hex = digest_hex(digest)?;
        Ok(self.root.join("blobs").join("sha256").join(hex))
    }

    /// Read a blob, checking its digest.
    pub fn read_blob(&self, digest: &str) -> Result<Vec<u8>, OciError> {
        let path = self.blob_path(digest)?;
        let bytes = std::fs::read(&path).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => OciError::NotFound(format!("blob {digest}")),
            _ => OciError::Io(e),
        })?;
        let actual = sha256_digest(&bytes);
        if actual != digest {
            return Err(OciError::Invalid(format!(
                "blob {digest} has digest {actual}"
            )));
        }
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn put(root: &Path, bytes: &[u8]) -> String {
        let digest = sha256_digest(bytes);
        let dir = root.join("blobs/sha256");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(digest_hex(&digest).unwrap()), bytes).unwrap();
        digest
    }

    fn manifest(root: &Path, label: &str) -> (String, usize, String) {
        let config = put(
            root,
            json!({"config": {"Env": [format!("ARCH={label}")]}})
                .to_string()
                .as_bytes(),
        );
        let layer = put(root, label.as_bytes());
        let body = json!({
            "schemaVersion": 2,
            "mediaType": media_types::OCI_MANIFEST,
            "config": {"mediaType": "application/vnd.oci.image.config.v1+json", "digest": config, "size": 1},
            "layers": [{"mediaType": media_types::OCI_LAYER_GZIP, "digest": layer, "size": label.len()}],
        })
        .to_string();
        (put(root, body.as_bytes()), body.len(), config)
    }

    #[test]
    fn resolves_the_requested_architecture_through_nested_indexes() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let (amd, amd_len, _) = manifest(root, "amd64");
        let (arm, arm_len, arm_config) = manifest(root, "arm64");
        let inner = json!({
            "schemaVersion": 2,
            "mediaType": media_types::OCI_INDEX,
            "manifests": [
                {"mediaType": media_types::OCI_MANIFEST, "digest": amd, "size": amd_len,
                 "platform": {"os": "linux", "architecture": "amd64"}},
                {"mediaType": media_types::OCI_MANIFEST, "digest": arm, "size": arm_len,
                 "platform": {"os": "linux", "architecture": "arm64", "variant": "v8"}},
                {"mediaType": media_types::OCI_MANIFEST, "digest": amd, "size": amd_len,
                 "platform": {"os": "unknown", "architecture": "unknown"}},
            ],
        })
        .to_string();
        let inner_digest = put(root, inner.as_bytes());
        std::fs::write(
            root.join("index.json"),
            json!({"schemaVersion": 2, "manifests": [
                {"mediaType": media_types::OCI_INDEX, "digest": inner_digest, "size": inner.len()}
            ]})
            .to_string(),
        )
        .unwrap();
        std::fs::write(root.join("oci-layout"), r#"{"imageLayoutVersion":"1.0.0"}"#).unwrap();

        let layout = OciLayout::open(root).unwrap();
        let resolved = layout.resolve(Arch::Arm64).unwrap();
        assert_eq!(resolved.digest, arm);
        assert_eq!(resolved.config_digest, arm_config);
        assert_eq!(resolved.layers.len(), 1);
        let config =
            super::super::ImageConfig::from_config_blob(&layout.read_blob(&arm_config).unwrap())
                .unwrap();
        assert_eq!(config.env, vec!["ARCH=arm64".to_string()]);
        assert_eq!(layout.resolve(Arch::Amd64).unwrap().digest, amd);
    }

    #[test]
    fn a_lone_manifest_without_platform_is_accepted() {
        let dir = tempfile::tempdir().unwrap();
        let (digest, len, _) = manifest(dir.path(), "only");
        std::fs::write(
            dir.path().join("index.json"),
            json!({"schemaVersion": 2, "manifests": [
                {"mediaType": media_types::OCI_MANIFEST, "digest": digest, "size": len}
            ]})
            .to_string(),
        )
        .unwrap();
        let layout = OciLayout::open(dir.path()).unwrap();
        assert_eq!(layout.resolve(Arch::Amd64).unwrap().digest, digest);
    }

    #[test]
    fn digests_are_validated() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("index.json"), "{}").unwrap();
        let layout = OciLayout::open(dir.path()).unwrap();
        for bad in [
            "sha256:../../etc/passwd",
            "sha512:00",
            "sha256:ABCDEF",
            &format!("sha256:{}", "A".repeat(64)),
        ] {
            assert!(layout.blob_path(bad).is_err(), "{bad}");
        }
        let digest = put(dir.path(), b"content");
        std::fs::write(
            dir.path()
                .join("blobs/sha256")
                .join(digest_hex(&digest).unwrap()),
            b"tampered",
        )
        .unwrap();
        assert!(matches!(
            layout.read_blob(&digest).unwrap_err(),
            OciError::Invalid(_)
        ));
        assert!(OciLayout::open(&dir.path().join("missing")).is_err());
    }
}
