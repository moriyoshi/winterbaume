//! The on-disk cache of base images and the per-function root file systems
//! built from them.
//!
//! ```text
//! <cache>/<CACHE_FORMAT>/
//!   images/<manifest hex>/rootfs/          one unpacked base image
//!   images/<manifest hex>/image-config.json
//!   images/<manifest hex>/.complete        written last
//!   refs/<repository>/<tag>/<arch>         manifest digest a tag resolved to
//!   tmp/                                   pulls in progress
//!   run/<uuid>/<image>-<code>/             roots of this process's VMs
//! ```
//!
//! A pull is built under `tmp/` and renamed into place, so concurrent
//! processes sharing the cache never see a partial image.  A root is a
//! hard-link copy of the base image with the deployment package unpacked at
//! `/var/task` and `/tmp` pointing at the guest's tmpfs; the guest mounts it
//! read-only, so the shared files cannot be changed through it.  A process
//! removes its roots when its cache is dropped.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use tokio::sync::OnceCell;
use winterbaume_lambda::{CodeSource, StubReason};

use crate::oci::{
    Arch, ImageConfig, ImageRef, OciError, OciLayout, RegistryClient, ResolvedManifest,
};
use crate::runtimes::RuntimeImage;

/// Version of the cache layout and of how images are unpacked into it.  A
/// cache written by a build with another version is not used: images
/// unpacked before modification times were kept break the JVM, for one.
pub const CACHE_FORMAT: &str = "v2";

/// How long a tag's resolution is trusted before the registry is asked again.
const REF_TTL: Duration = Duration::from_secs(24 * 60 * 60);
/// `run/` directories of processes that went away without cleaning up are
/// removed after this long.
const STALE_RUN_DIR: Duration = Duration::from_secs(24 * 60 * 60);

/// An unpacked base image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseImage {
    /// Identifies the image's content, for root names.
    pub key: String,
    pub rootfs: PathBuf,
    pub config: ImageConfig,
    pub arch: Arch,
}

type TemplateCell = Arc<OnceCell<Result<PathBuf, StubReason>>>;

pub struct ImageCache {
    root: PathBuf,
    run_dir: PathBuf,
    registry: Option<RegistryClient>,
    image_dir: Option<PathBuf>,
    /// One pull at a time per image reference.
    pulls: Mutex<HashMap<(ImageRef, Arch), Arc<tokio::sync::Mutex<()>>>>,
    /// Images already resolved by this process, and when.
    resolved: Mutex<HashMap<(String, ImageRef, Arch), (Instant, Arc<BaseImage>)>>,
    templates: Mutex<HashMap<String, TemplateCell>>,
}

impl Drop for ImageCache {
    fn drop(&mut self) {
        let _ = remove_tree(&self.run_dir);
    }
}

impl ImageCache {
    /// Open ( and create ) the cache at `root`.  `registry` is the base URL
    /// to pull from, `None` when offline.
    pub fn open(
        root: &Path,
        registry: Option<&str>,
        image_dir: Option<&Path>,
    ) -> Result<Self, StubReason> {
        let root = &root.join(CACHE_FORMAT);
        let unusable = |what: &str, e: std::io::Error| {
            tracing::warn!(cache = %root.display(), error = %e, "{what}");
            StubReason::ImageCacheUnusable
        };
        for dir in ["images", "refs", "tmp", "run"] {
            std::fs::create_dir_all(root.join(dir))
                .map_err(|e| unusable("cannot create the Lambda image cache", e))?;
        }
        if !is_case_sensitive(root)
            .map_err(|e| unusable("cannot probe the Lambda image cache", e))?
        {
            tracing::warn!(
                cache = %root.display(),
                "the Lambda image cache is on a case-insensitive file system; \
                 use a case-sensitive volume ( on macOS: a case-sensitive APFS volume )"
            );
            return Err(StubReason::ImageCacheUnusable);
        }
        prune_stale_run_dirs(&root.join("run"));
        let run_dir = root
            .join("run")
            .join(uuid::Uuid::new_v4().simple().to_string());
        std::fs::create_dir_all(&run_dir)
            .map_err(|e| unusable("cannot create the Lambda root directory", e))?;
        Ok(Self {
            root: root.to_path_buf(),
            run_dir,
            registry: registry.map(RegistryClient::new),
            image_dir: image_dir.map(Path::to_path_buf),
            pulls: Mutex::new(HashMap::new()),
            resolved: Mutex::new(HashMap::new()),
            templates: Mutex::new(HashMap::new()),
        })
    }

    /// The directory holding this process's roots.
    pub fn run_dir(&self) -> &Path {
        &self.run_dir
    }

    /// The base image for `runtime`, from the override directory, the cache
    /// or the registry, in that order.
    pub async fn base_image(
        &self,
        runtime: &str,
        image: &RuntimeImage,
        arch: Arch,
    ) -> Result<Arc<BaseImage>, StubReason> {
        let reference = ImageRef {
            repository: image.repository.clone(),
            tag: image.tag.clone(),
        };
        let memo_key = (runtime.to_string(), reference.clone(), arch);
        if let Some((at, base)) = self.lock_resolved().get(&memo_key)
            && at.elapsed() < REF_TTL
        {
            return Ok(base.clone());
        }
        let lock = self
            .pulls
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .entry((reference.clone(), arch))
            .or_default()
            .clone();
        let _guard = lock.lock().await;

        let base = match self.lookup_image_dir(runtime, arch).await? {
            Some(base) => base,
            None => self.lookup_cache_or_registry(&reference, arch).await?,
        };
        let base = Arc::new(base);
        self.lock_resolved()
            .insert(memo_key, (Instant::now(), base.clone()));
        Ok(base)
    }

    fn lock_resolved(
        &self,
    ) -> std::sync::MutexGuard<'_, HashMap<(String, ImageRef, Arch), (Instant, Arc<BaseImage>)>>
    {
        self.resolved.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// `<image_dir>/<runtime>/<arch>/`: an unpacked `rootfs/`, or an OCI
    /// image layout.
    async fn lookup_image_dir(
        &self,
        runtime: &str,
        arch: Arch,
    ) -> Result<Option<BaseImage>, StubReason> {
        let Some(image_dir) = &self.image_dir else {
            return Ok(None);
        };
        let dir = image_dir
            .join(crate::runtimes::image_dir_name(runtime))
            .join(arch.lambda_name());
        let rootfs = dir.join("rootfs");
        if rootfs.is_dir() {
            let config = match std::fs::read(dir.join("config.json")) {
                Ok(blob) => ImageConfig::from_config_blob(&blob).map_err(|e| {
                    tracing::warn!(dir = %dir.display(), error = %e, "invalid image configuration");
                    StubReason::ImageUnavailable
                })?,
                Err(_) => default_config(),
            };
            use sha2::Digest as _;
            let key = format!(
                "dir-{:x}",
                sha2::Sha256::digest(rootfs.to_string_lossy().as_bytes())
            )[..20]
                .to_string();
            return Ok(Some(BaseImage {
                key,
                rootfs,
                config,
                arch,
            }));
        }
        if dir.join("index.json").is_file() {
            let layout = OciLayout::open(&dir).map_err(|e| {
                tracing::warn!(dir = %dir.display(), error = %e, "invalid OCI image layout");
                StubReason::ImageUnavailable
            })?;
            let manifest = layout.resolve(arch).map_err(|e| {
                tracing::warn!(dir = %dir.display(), error = %e, "no usable image in the OCI layout");
                StubReason::ImageUnavailable
            })?;
            if let Some(base) = self.cached(&manifest.digest, arch) {
                return Ok(Some(base));
            }
            let config = layout
                .read_blob(&manifest.config_digest)
                .and_then(|blob| ImageConfig::from_config_blob(&blob))
                .map_err(|e| {
                    tracing::warn!(dir = %dir.display(), error = %e, "invalid image configuration");
                    StubReason::ImageUnavailable
                })?;
            let layers = manifest
                .layers
                .iter()
                .map(|l| Ok((layout.blob_path(&l.digest)?, l.media_type.clone())))
                .collect::<Result<Vec<_>, OciError>>()
                .map_err(|e| {
                    tracing::warn!(error = %e, "invalid OCI image layout");
                    StubReason::ImageUnavailable
                })?;
            let base = self.install(&manifest, config, arch, layers, false).await?;
            return Ok(Some(base));
        }
        Ok(None)
    }

    async fn lookup_cache_or_registry(
        &self,
        reference: &ImageRef,
        arch: Arch,
    ) -> Result<BaseImage, StubReason> {
        let ref_file = self
            .root
            .join("refs")
            .join(reference.repository.replace('/', "_"))
            .join(&reference.tag)
            .join(arch.lambda_name());
        let known = std::fs::read_to_string(&ref_file)
            .ok()
            .map(|s| s.trim().to_string())
            .and_then(|digest| self.cached(&digest, arch).map(|base| (digest, base)));
        let fresh = std::fs::metadata(&ref_file)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| SystemTime::now().duration_since(t).ok())
            .is_some_and(|age| age < REF_TTL);
        if let Some((_, base)) = &known
            && (fresh || self.registry.is_none())
        {
            return Ok(base.clone());
        }
        let Some(registry) = &self.registry else {
            tracing::warn!(
                image = %format!("{}:{}", reference.repository, reference.tag),
                arch = arch.lambda_name(),
                "Lambda base image is not cached and pulling is off"
            );
            return Err(StubReason::ImageUnavailable);
        };
        match self.pull(registry, reference, arch).await {
            Ok(base) => {
                if let Some(parent) = ref_file.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let digest = base.key.clone();
                let _ = std::fs::write(&ref_file, format!("sha256:{digest}\n"));
                Ok(base)
            }
            Err(e) => {
                if let Some((_, base)) = known {
                    tracing::warn!(error = %e, "cannot refresh a Lambda base image; using the cached one");
                    // Do not ask again on every invocation.
                    let _ = touch(&ref_file);
                    return Ok(base);
                }
                tracing::warn!(
                    image = %format!("{}:{}", reference.repository, reference.tag),
                    arch = arch.lambda_name(),
                    error = %e,
                    "cannot pull the Lambda base image"
                );
                Err(StubReason::ImageUnavailable)
            }
        }
    }

    async fn pull(
        &self,
        registry: &RegistryClient,
        reference: &ImageRef,
        arch: Arch,
    ) -> Result<BaseImage, OciError> {
        let manifest = registry.resolve(reference, arch).await?;
        if let Some(base) = self.cached(&manifest.digest, arch) {
            return Ok(base);
        }
        tracing::info!(
            image = %format!("{}:{}", reference.repository, reference.tag),
            arch = arch.lambda_name(),
            digest = %manifest.digest,
            "pulling Lambda base image"
        );
        let config_blob = registry
            .fetch_blob(reference, &manifest.config_digest)
            .await?;
        let config = ImageConfig::from_config_blob(&config_blob)?;
        let downloads = self
            .root
            .join("tmp")
            .join(format!("layers-{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&downloads)?;
        let _cleanup = RemoveOnDrop(downloads.clone());
        let mut layers = Vec::new();
        for (index, layer) in manifest.layers.iter().enumerate() {
            let path = downloads.join(format!("{index}"));
            registry
                .download_blob(reference, &layer.digest, &path)
                .await?;
            layers.push((path, layer.media_type.clone()));
        }
        self.install(&manifest, config, arch, layers, true)
            .await
            .map_err(|_| OciError::Invalid("cannot unpack the image".to_string()))
    }

    /// The complete cached image with manifest `digest`, if there is one.
    fn cached(&self, digest: &str, arch: Arch) -> Option<BaseImage> {
        let hex = digest.strip_prefix("sha256:").unwrap_or(digest);
        if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        let dir = self.root.join("images").join(hex);
        if !dir.join(".complete").is_file() {
            return None;
        }
        let config = std::fs::read(dir.join("image-config.json"))
            .ok()
            .and_then(|raw| serde_json::from_slice(&raw).ok())
            .unwrap_or_else(default_config);
        Some(BaseImage {
            key: hex.to_string(),
            rootfs: dir.join("rootfs"),
            config,
            arch,
        })
    }

    /// Unpack `layers` into a new cached image and move it into place.
    async fn install(
        &self,
        manifest: &ResolvedManifest,
        config: ImageConfig,
        arch: Arch,
        layers: Vec<(PathBuf, String)>,
        remove_layers: bool,
    ) -> Result<BaseImage, StubReason> {
        let hex = manifest
            .digest
            .strip_prefix("sha256:")
            .unwrap_or(&manifest.digest)
            .to_string();
        let root = self.root.clone();
        let staged_config = config.clone();
        let result = tokio::task::spawn_blocking(move || -> Result<(), OciError> {
            let staging = root
                .join("tmp")
                .join(format!("image-{}", uuid::Uuid::new_v4().simple()));
            let cleanup = RemoveOnDrop(staging.clone());
            let rootfs = staging.join("rootfs");
            std::fs::create_dir_all(&rootfs)?;
            for (layer, media_type) in &layers {
                crate::oci::apply_layer(&rootfs, layer, media_type)?;
                if remove_layers {
                    let _ = std::fs::remove_file(layer);
                }
            }
            std::fs::write(
                staging.join("image-config.json"),
                serde_json::to_vec(&staged_config).map_err(|e| OciError::Invalid(e.to_string()))?,
            )?;
            std::fs::write(staging.join(".complete"), b"")?;
            let dest = root.join("images").join(&hex);
            match std::fs::rename(&staging, &dest) {
                Ok(()) => {
                    std::mem::forget(cleanup);
                    Ok(())
                }
                // Another process finished the same image first.
                Err(_) if dest.join(".complete").is_file() => Ok(()),
                Err(e) => Err(e.into()),
            }
        })
        .await;
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                tracing::warn!(digest = %manifest.digest, error = %e, "cannot unpack the Lambda base image");
                return Err(StubReason::ImageUnavailable);
            }
            Err(e) => {
                tracing::warn!(error = %e, "unpacking the Lambda base image panicked");
                return Err(StubReason::ImageUnavailable);
            }
        }
        self.cached(&manifest.digest, arch)
            .map(|mut base| {
                base.config = config;
                base
            })
            .ok_or(StubReason::ImageUnavailable)
    }

    /// The root file system for running `code` on `base`, built once.
    pub async fn template(
        &self,
        base: &BaseImage,
        code: &CodeSource,
    ) -> Result<PathBuf, StubReason> {
        let name = format!("{}-{}", base.key, code.sha256_hex());
        let cell = self
            .templates
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .entry(name.clone())
            .or_default()
            .clone();
        cell.get_or_init(|| async {
            let package = code.load().await.map_err(|e| {
                tracing::warn!(error = %e, "cannot read the Lambda deployment package");
                StubReason::NoCode
            })?;
            let rootfs = base.rootfs.clone();
            let dest = self.run_dir.join(&name);
            tokio::task::spawn_blocking(move || build_root(&rootfs, &package, &dest).map(|()| dest))
                .await
                .map_err(|_| StubReason::InvalidPackage)?
        })
        .await
        .clone()
    }
}

/// A conservative image configuration for an image directory without one.
fn default_config() -> ImageConfig {
    ImageConfig {
        env: vec![
            "PATH=/var/lang/bin:/usr/local/bin:/usr/bin/:/bin:/opt/bin".to_string(),
            "LD_LIBRARY_PATH=/var/lang/lib:/lib64:/usr/lib64:/var/runtime:/var/runtime/lib:/var/task:/var/task/lib:/opt/lib".to_string(),
            "LANG=en_US.UTF-8".to_string(),
            "LAMBDA_TASK_ROOT=/var/task".to_string(),
            "LAMBDA_RUNTIME_DIR=/var/runtime".to_string(),
        ],
        working_dir: Some("/var/task".to_string()),
        entrypoint: vec!["/lambda-entrypoint.sh".to_string()],
        cmd: Vec::new(),
    }
}

/// Build a root at `dest`: `rootfs` hard-linked, `package` unpacked at
/// `/var/task`, `/tmp` on the guest's tmpfs, the host's DNS configuration.
fn build_root(rootfs: &Path, package: &[u8], dest: &Path) -> Result<(), StubReason> {
    let staging = dest.with_extension(format!("tmp-{}", uuid::Uuid::new_v4().simple()));
    let cleanup = RemoveOnDrop(staging.clone());
    let failed = |what: &str, e: std::io::Error| {
        tracing::warn!(root = %staging.display(), error = %e, "{what}");
        StubReason::ImageCacheUnusable
    };
    link_tree(rootfs, &staging, Path::new(""))
        .map_err(|e| failed("cannot build the function root", e))?;
    for dir in ["dev", "proc", "sys", "var", "opt"] {
        std::fs::create_dir_all(staging.join(dir))
            .map_err(|e| failed("cannot build the function root", e))?;
    }
    // Lambda's only writable path; the guest's /dev/shm is a tmpfs.
    #[cfg(unix)]
    std::os::unix::fs::symlink("/dev/shm", staging.join("tmp"))
        .map_err(|e| failed("cannot build the function root", e))?;
    if let Ok(resolv) = std::fs::read("/etc/resolv.conf") {
        let etc = staging.join("etc");
        let _ = std::fs::create_dir_all(&etc);
        let _ = std::fs::remove_file(etc.join("resolv.conf"));
        let _ = std::fs::write(etc.join("resolv.conf"), resolv);
    }
    let task = staging.join("var/task");
    let _ = std::fs::remove_dir_all(&task);
    winterbaume_lambda::execution::unpack_zip(package, &task).map_err(|e| {
        tracing::debug!(error = %e, "Lambda deployment package cannot be unpacked");
        StubReason::InvalidPackage
    })?;
    match std::fs::rename(&staging, dest) {
        Ok(()) => {
            std::mem::forget(cleanup);
            Ok(())
        }
        Err(_) if dest.is_dir() => Ok(()),
        Err(e) => Err(failed("cannot build the function root", e)),
    }
}

/// Recreate the tree at `src` under `dst`: directories anew, files as hard
/// links, symbolic links as they are.  `/tmp` and the contents of
/// `/var/task` of the image are left out.
fn link_tree(src: &Path, dst: &Path, relative: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::symlink_metadata(src)?.permissions().mode() & 0o7777;
        std::fs::set_permissions(dst, std::fs::Permissions::from_mode(mode | 0o700))?;
    }
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        let rel = relative.join(&name);
        if rel == Path::new("tmp") || rel == Path::new("var/task") {
            if rel == Path::new("var/task") {
                std::fs::create_dir_all(dst.join(&name))?;
            }
            continue;
        }
        let from = entry.path();
        let to = dst.join(&name);
        let kind = entry.file_type()?;
        if kind.is_dir() {
            link_tree(&from, &to, &rel)?;
        } else if kind.is_symlink() {
            #[cfg(unix)]
            std::os::unix::fs::symlink(std::fs::read_link(&from)?, &to)?;
        } else if kind.is_file() {
            // An image directory override may be on another file system.
            if std::fs::hard_link(&from, &to).is_err() {
                std::fs::copy(&from, &to)?;
            }
        }
    }
    Ok(())
}

fn is_case_sensitive(dir: &Path) -> std::io::Result<bool> {
    let probe = dir.join(format!(".case-probe-{}-A", std::process::id()));
    std::fs::write(&probe, b"")?;
    let other = dir.join(format!(".case-probe-{}-a", std::process::id()));
    let sensitive = !other.exists();
    let _ = std::fs::remove_file(&probe);
    Ok(sensitive)
}

fn prune_stale_run_dirs(run: &Path) {
    let Ok(entries) = std::fs::read_dir(run) else {
        return;
    };
    for entry in entries.flatten() {
        let stale = entry
            .metadata()
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| SystemTime::now().duration_since(t).ok())
            .is_some_and(|age| age > STALE_RUN_DIR);
        if stale {
            let _ = remove_tree(&entry.path());
        }
    }
}

fn touch(path: &Path) -> std::io::Result<()> {
    let contents = std::fs::read(path)?;
    std::fs::write(path, contents)
}

/// Remove a tree, making its directories writable first ( images contain
/// read-only directories ).  Files are never chmod-ed: they may be hard
/// links shared with the cached image.
pub(crate) fn remove_tree(path: &Path) -> std::io::Result<()> {
    if std::fs::symlink_metadata(path).is_err() {
        return Ok(());
    }
    make_dirs_writable(path);
    std::fs::remove_dir_all(path)
}

fn make_dirs_writable(path: &Path) {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return;
    };
    if !meta.is_dir() {
        return;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = meta.permissions().mode();
        if mode & 0o700 != 0o700 {
            let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode | 0o700));
        }
    }
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|t| t.is_dir()) {
                make_dirs_writable(&entry.path());
            }
        }
    }
}

struct RemoveOnDrop(PathBuf);

impl Drop for RemoveOnDrop {
    fn drop(&mut self) {
        let _ = remove_tree(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

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

    /// An image directory with an unpacked rootfs for nodejs22.x / arm64.
    fn image_dir() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let rootfs = dir.path().join("nodejs22.x/arm64/rootfs");
        std::fs::create_dir_all(rootfs.join("var/runtime")).unwrap();
        std::fs::create_dir_all(rootfs.join("var/task")).unwrap();
        std::fs::create_dir_all(rootfs.join("tmp")).unwrap();
        std::fs::create_dir_all(rootfs.join("usr/lib64")).unwrap();
        std::fs::write(rootfs.join("var/runtime/bootstrap"), "#!/bin/sh\n").unwrap();
        std::fs::write(rootfs.join("var/task/leftover"), "x").unwrap();
        std::os::unix::fs::symlink("usr/lib64", rootfs.join("lib64")).unwrap();
        std::fs::create_dir_all(rootfs.join("root")).unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(rootfs.join("root"), std::fs::Permissions::from_mode(0o550))
            .unwrap();
        std::fs::write(
            dir.path().join("nodejs22.x/arm64/config.json"),
            r#"{"config":{"Env":["PATH=/custom/bin","LANG=C"],"WorkingDir":"/var/task"}}"#,
        )
        .unwrap();
        dir
    }

    fn node22() -> RuntimeImage {
        crate::runtimes::image_for("nodejs22.x").unwrap()
    }

    #[tokio::test]
    async fn image_dir_rootfs_is_used_and_roots_are_built_from_it() {
        let images = image_dir();
        let cache_dir = tempfile::tempdir().unwrap();
        let cache = ImageCache::open(cache_dir.path(), None, Some(images.path())).unwrap();

        let base = cache
            .base_image("nodejs22.x", &node22(), Arch::Arm64)
            .await
            .unwrap();
        assert_eq!(base.config.env, vec!["PATH=/custom/bin", "LANG=C"]);
        assert!(base.key.starts_with("dir-"));

        let code = CodeSource::from_bytes(
            "q83vEjRWeJA=",
            zip_of(&[("index.mjs", "export const handler = 1;")]).into(),
        )
        .await;
        let root = cache.template(&base, &code).await.unwrap();
        assert!(root.starts_with(cache.run_dir()));
        assert_eq!(
            std::fs::read_to_string(root.join("var/task/index.mjs")).unwrap(),
            "export const handler = 1;"
        );
        assert!(
            !root.join("var/task/leftover").exists(),
            "the image's /var/task is replaced"
        );
        assert_eq!(
            std::fs::read_link(root.join("tmp")).unwrap(),
            Path::new("/dev/shm")
        );
        assert_eq!(
            std::fs::read_link(root.join("lib64")).unwrap(),
            Path::new("usr/lib64")
        );
        for dir in ["dev", "proc", "sys"] {
            assert!(root.join(dir).is_dir());
        }
        use std::os::unix::fs::MetadataExt;
        assert_eq!(
            std::fs::metadata(root.join("var/runtime/bootstrap"))
                .unwrap()
                .ino(),
            std::fs::metadata(base.rootfs.join("var/runtime/bootstrap"))
                .unwrap()
                .ino(),
            "image files are hard links"
        );

        // Built once.
        assert_eq!(cache.template(&base, &code).await.unwrap(), root);

        // Removed with the cache, read-only directories included; the image
        // stays.
        let run_dir = cache.run_dir().to_path_buf();
        drop(cache);
        assert!(!run_dir.exists());
        assert!(
            images
                .path()
                .join("nodejs22.x/arm64/rootfs/var/runtime/bootstrap")
                .is_file()
        );
    }

    #[tokio::test]
    async fn a_bad_package_is_invalid_and_stays_so() {
        let images = image_dir();
        let cache_dir = tempfile::tempdir().unwrap();
        let cache = ImageCache::open(cache_dir.path(), None, Some(images.path())).unwrap();
        let base = cache
            .base_image("nodejs22.x", &node22(), Arch::Arm64)
            .await
            .unwrap();
        let code =
            CodeSource::from_bytes("bm90emlw", bytes::Bytes::from_static(b"not a zip")).await;
        assert_eq!(
            cache.template(&base, &code).await,
            Err(StubReason::InvalidPackage)
        );
        assert_eq!(
            cache.template(&base, &code).await,
            Err(StubReason::InvalidPackage)
        );
        let leftovers: Vec<_> = std::fs::read_dir(cache.run_dir()).unwrap().collect();
        assert!(leftovers.is_empty(), "a failed build leaves nothing behind");
    }

    #[tokio::test]
    async fn offline_without_an_image_is_unavailable() {
        let cache_dir = tempfile::tempdir().unwrap();
        let cache = ImageCache::open(cache_dir.path(), None, None).unwrap();
        assert_eq!(
            cache.base_image("nodejs22.x", &node22(), Arch::Amd64).await,
            Err(StubReason::ImageUnavailable)
        );
        // An unreachable registry, too.
        let cache = ImageCache::open(cache_dir.path(), Some("http://127.0.0.1:9"), None).unwrap();
        assert_eq!(
            cache.base_image("nodejs22.x", &node22(), Arch::Amd64).await,
            Err(StubReason::ImageUnavailable)
        );
    }

    #[test]
    fn stale_run_directories_are_pruned() {
        let cache_dir = tempfile::tempdir().unwrap();
        let old = cache_dir.path().join(CACHE_FORMAT).join("run/old");
        std::fs::create_dir_all(old.join("x")).unwrap();
        let past = SystemTime::now() - Duration::from_secs(2 * 24 * 60 * 60);
        std::fs::File::open(&old)
            .unwrap()
            .set_modified(past)
            .unwrap();
        let cache = ImageCache::open(cache_dir.path(), None, None).unwrap();
        assert!(!old.exists());
        assert!(cache.run_dir().is_dir());
    }
}
