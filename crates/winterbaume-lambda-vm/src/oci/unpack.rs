//! Applying image layers to a root file system tree on the host.
//!
//! The tree is later served to a guest as its root, so paths and links are
//! interpreted with the guest's semantics: an absolute symbolic link points
//! inside the tree, and `..` never leaves it.  Nothing here follows a link
//! out of the tree on the host.

use std::fs;
use std::io::{BufReader, Read};
use std::path::{Component, Path, PathBuf};

use super::OciError;

/// Maximum number of symbolic links followed while resolving one path.
const MAX_SYMLINK_HOPS: usize = 40;

const WHITEOUT_PREFIX: &str = ".wh.";
const OPAQUE_WHITEOUT: &str = ".wh..wh..opq";

/// Resolve `relative` inside `root` with chroot semantics.
///
/// Every existing component is inspected without following it; a symbolic
/// link met before the last component ( or at the last one, with
/// `follow_last` ) is replaced by its target, resolved from `root` when it
/// is absolute and from the directory holding the link otherwise.  `..`
/// stops at `root`.  Components that do not exist are appended as they are.
/// The result is `root` joined with normal components only.
pub fn resolve_in_root(
    root: &Path,
    relative: &Path,
    follow_last: bool,
) -> Result<PathBuf, OciError> {
    let mut resolved: Vec<std::ffi::OsString> = Vec::new();
    // Components still to process, last first ( a stack ).
    let mut pending: Vec<std::ffi::OsString> = Vec::new();
    push_components(&mut pending, relative);
    let mut hops = 0usize;

    while let Some(component) = pending.pop() {
        if component == ".." {
            resolved.pop();
            continue;
        }
        if component == "." || component.is_empty() {
            continue;
        }
        let is_last = pending.is_empty();
        let mut candidate = root.to_path_buf();
        candidate.extend(&resolved);
        candidate.push(&component);
        let follow = !is_last || follow_last;
        match fs::symlink_metadata(&candidate) {
            Ok(meta) if follow && meta.file_type().is_symlink() => {
                hops += 1;
                if hops > MAX_SYMLINK_HOPS {
                    return Err(OciError::Unsafe(format!(
                        "too many levels of symbolic links resolving {}",
                        relative.display()
                    )));
                }
                let target = fs::read_link(&candidate)?;
                if target.has_root() {
                    resolved.clear();
                }
                push_components(&mut pending, &target);
            }
            _ => resolved.push(component),
        }
    }

    let mut out = root.to_path_buf();
    out.extend(&resolved);
    Ok(out)
}

/// Push the components of `path` onto `stack` so that the first component
/// is popped first.  Root and prefix components are dropped: resolution is
/// always relative to the root.
fn push_components(stack: &mut Vec<std::ffi::OsString>, path: &Path) {
    let components: Vec<std::ffi::OsString> = path
        .components()
        .filter_map(|c| match c {
            Component::Normal(name) => Some(name.to_os_string()),
            Component::ParentDir => Some("..".into()),
            Component::CurDir | Component::RootDir | Component::Prefix(_) => None,
        })
        .collect();
    stack.extend(components.into_iter().rev());
}

/// Normalise an archive entry name to a relative path of normal
/// components, rejecting names that would climb above the root.
fn entry_relative(path: &Path) -> Result<PathBuf, OciError> {
    let mut out: Vec<&std::ffi::OsStr> = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(name) => out.push(name),
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {}
            Component::ParentDir => {
                if out.pop().is_none() {
                    return Err(OciError::Unsafe(format!(
                        "layer entry escapes the root: {}",
                        path.display()
                    )));
                }
            }
        }
    }
    Ok(out.iter().collect())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Compression {
    None,
    Gzip,
}

fn compression_of(layer: &Path, media_type: &str) -> Result<Compression, OciError> {
    if media_type.contains("zstd") {
        return Err(OciError::Invalid(
            "zstd layers are not supported".to_string(),
        ));
    }
    let mut magic = [0u8; 2];
    let mut file = fs::File::open(layer)?;
    let read = file.read(&mut magic)?;
    if read == 2 && magic == [0x1f, 0x8b] {
        return Ok(Compression::Gzip);
    }
    if media_type.ends_with("+gzip") || media_type.ends_with(".tar.gzip") {
        // Declared gzip but no gzip magic: let the decoder report it.
        return Ok(Compression::Gzip);
    }
    Ok(Compression::None)
}

fn open_archive(
    layer: &Path,
    compression: Compression,
) -> Result<tar::Archive<Box<dyn Read>>, OciError> {
    let file = BufReader::new(fs::File::open(layer)?);
    let reader: Box<dyn Read> = match compression {
        Compression::Gzip => Box::new(flate2::read::GzDecoder::new(file)),
        Compression::None => Box::new(file),
    };
    Ok(tar::Archive::new(reader))
}

/// Remove whatever is at `path`, without following a symbolic link there.
fn remove_any(path: &Path) -> Result<(), OciError> {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.is_dir() => {
            make_tree_removable(path);
            fs::remove_dir_all(path)?;
        }
        Ok(_) => fs::remove_file(path)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }
    Ok(())
}

/// Give the owner write and search permission on every directory below
/// `path` ( directories are never shared between trees, unlike files ), so
/// that the tree can be removed.
fn make_tree_removable(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let Ok(meta) = fs::symlink_metadata(path) else {
            return;
        };
        if !meta.is_dir() {
            return;
        }
        let mode = meta.permissions().mode();
        if mode & 0o700 != 0o700 {
            let _ = fs::set_permissions(path, fs::Permissions::from_mode(mode | 0o700));
        }
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                make_tree_removable(&entry.path());
            }
        }
    }
    #[cfg(not(unix))]
    let _ = path;
}

/// Where an entry with normalised relative name `relative` lands: its
/// parent resolved inside the root, joined with its own name.
fn entry_target(rootfs: &Path, relative: &Path) -> Result<Option<PathBuf>, OciError> {
    let Some(name) = relative.file_name() else {
        // The root itself.
        return Ok(None);
    };
    let parent = relative.parent().unwrap_or(Path::new(""));
    let parent = resolve_in_root(rootfs, parent, true)?;
    Ok(Some(parent.join(name)))
}

#[cfg(unix)]
fn set_mode(path: &Path, mode: u32) -> Result<(), OciError> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(mode))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_mode(_path: &Path, _mode: u32) -> Result<(), OciError> {
    Ok(())
}

/// Apply one image layer ( a tar archive, optionally gzip-compressed ) to
/// the tree at `rootfs`.
///
/// Whiteouts are applied first, so that they only affect what lower layers
/// left behind, then the other entries are extracted.  Device nodes and
/// FIFOs are skipped, ownership is not applied, and setuid / setgid bits are
/// dropped.
pub fn apply_layer(rootfs: &Path, layer: &Path, media_type: &str) -> Result<(), OciError> {
    let compression = compression_of(layer, media_type)?;

    // Pass 1: whiteouts.
    let mut archive = open_archive(layer, compression)?;
    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?.into_owned();
        let relative = entry_relative(&path)?;
        let Some(name) = relative.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !name.starts_with(WHITEOUT_PREFIX) {
            continue;
        }
        let dir = relative.parent().unwrap_or(Path::new(""));
        let dir = resolve_in_root(rootfs, dir, true)?;
        if name == OPAQUE_WHITEOUT {
            if fs::symlink_metadata(&dir).is_ok_and(|m| m.is_dir()) {
                for child in fs::read_dir(&dir)? {
                    remove_any(&child?.path())?;
                }
            }
        } else {
            let hidden = &name[WHITEOUT_PREFIX.len()..];
            if hidden.is_empty() || hidden == "." || hidden == ".." || hidden.contains('/') {
                continue;
            }
            remove_any(&dir.join(hidden))?;
        }
    }

    // Pass 2: everything else.
    let mut archive = open_archive(layer, compression)?;
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        let relative = entry_relative(&path)?;
        if relative
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with(WHITEOUT_PREFIX))
        {
            continue;
        }
        let entry_type = entry.header().entry_type();
        let mode = entry.header().mode().unwrap_or(0o644);
        let Some(target) = entry_target(rootfs, &relative)? else {
            // An entry for the root directory itself: nothing to create.
            continue;
        };
        match entry_type {
            tar::EntryType::Regular | tar::EntryType::Continuous => {
                remove_any(&target)?;
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut file = fs::File::create(&target)?;
                std::io::copy(&mut entry, &mut file)?;
                // Keep the image's modification times: the JVM's class data
                // sharing archive, for one, refuses jars whose time changed.
                if let Ok(mtime) = entry.header().mtime() {
                    let _ = file.set_modified(
                        std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(mtime),
                    );
                }
                drop(file);
                set_mode(&target, (mode & 0o1777) | 0o400)?;
            }
            tar::EntryType::Directory => {
                if fs::symlink_metadata(&target).is_ok_and(|m| !m.is_dir()) {
                    remove_any(&target)?;
                }
                fs::create_dir_all(&target)?;
                // Directories are never shared with other trees, so changing
                // their mode is safe.
                set_mode(&target, (mode & 0o1777) | 0o700)?;
            }
            tar::EntryType::Symlink => {
                let Some(link) = entry.link_name()? else {
                    return Err(OciError::Invalid(format!(
                        "symbolic link without a target: {}",
                        path.display()
                    )));
                };
                let link = link.into_owned();
                remove_any(&target)?;
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                #[cfg(unix)]
                std::os::unix::fs::symlink(&link, &target)?;
                #[cfg(not(unix))]
                let _ = link;
            }
            tar::EntryType::Link => {
                let Some(link) = entry.link_name()? else {
                    return Err(OciError::Invalid(format!(
                        "hard link without a target: {}",
                        path.display()
                    )));
                };
                let link_relative = entry_relative(&link)?;
                let source = resolve_in_root(rootfs, &link_relative, false)?;
                let is_file = fs::symlink_metadata(&source).is_ok_and(|m| m.is_file());
                if !is_file || !source.starts_with(rootfs) {
                    return Err(OciError::Unsafe(format!(
                        "hard link {} does not point at a file inside the image: {}",
                        path.display(),
                        link.display()
                    )));
                }
                if source == target {
                    continue;
                }
                remove_any(&target)?;
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::hard_link(&source, &target)?;
            }
            // Devices, FIFOs, and metadata-only entries.
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    enum Item<'a> {
        File(&'a str, &'a [u8], u32),
        Dir(&'a str),
        Symlink(&'a str, &'a str),
        Hardlink(&'a str, &'a str),
        Char(&'a str),
    }

    fn header(kind: tar::EntryType, mode: u32, size: u64) -> tar::Header {
        let mut header = tar::Header::new_gnu();
        header.set_entry_type(kind);
        header.set_mode(mode);
        header.set_size(size);
        header.set_uid(0);
        header.set_gid(0);
        header
    }

    /// Build a layer file; `raw_names` writes names verbatim ( bypassing
    /// the `tar` crate's own `..` checks ) to test hostile archives.
    fn layer(dir: &Path, name: &str, items: &[Item<'_>], gzip: bool) -> PathBuf {
        let mut builder = tar::Builder::new(Vec::new());
        for item in items {
            match item {
                Item::File(path, data, mode) => {
                    let mut h = header(tar::EntryType::Regular, *mode, data.len() as u64);
                    set_raw_name(&mut h, path);
                    h.set_cksum();
                    builder.append(&h, *data).unwrap();
                }
                Item::Dir(path) => {
                    let mut h = header(tar::EntryType::Directory, 0o755, 0);
                    set_raw_name(&mut h, path);
                    h.set_cksum();
                    builder.append(&h, std::io::empty()).unwrap();
                }
                Item::Symlink(path, target) => {
                    let mut h = header(tar::EntryType::Symlink, 0o777, 0);
                    set_raw_name(&mut h, path);
                    set_raw_link(&mut h, target);
                    h.set_cksum();
                    builder.append(&h, std::io::empty()).unwrap();
                }
                Item::Hardlink(path, target) => {
                    let mut h = header(tar::EntryType::Link, 0o644, 0);
                    set_raw_name(&mut h, path);
                    set_raw_link(&mut h, target);
                    h.set_cksum();
                    builder.append(&h, std::io::empty()).unwrap();
                }
                Item::Char(path) => {
                    let mut h = header(tar::EntryType::Char, 0o666, 0);
                    set_raw_name(&mut h, path);
                    h.set_cksum();
                    builder.append(&h, std::io::empty()).unwrap();
                }
            }
        }
        let bytes = builder.into_inner().unwrap();
        let out = dir.join(name);
        if gzip {
            let mut encoder =
                flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
            encoder.write_all(&bytes).unwrap();
            fs::write(&out, encoder.finish().unwrap()).unwrap();
        } else {
            fs::write(&out, bytes).unwrap();
        }
        out
    }

    fn set_raw_name(header: &mut tar::Header, name: &str) {
        let gnu = header.as_gnu_mut().unwrap();
        gnu.name = [0; 100];
        gnu.name[..name.len()].copy_from_slice(name.as_bytes());
    }

    fn set_raw_link(header: &mut tar::Header, name: &str) {
        let gnu = header.as_gnu_mut().unwrap();
        gnu.linkname = [0; 100];
        gnu.linkname[..name.len()].copy_from_slice(name.as_bytes());
    }

    #[cfg(unix)]
    fn mode_of(path: &Path) -> u32 {
        use std::os::unix::fs::PermissionsExt;
        fs::symlink_metadata(path).unwrap().permissions().mode() & 0o7777
    }

    fn setup() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("rootfs");
        fs::create_dir(&root).unwrap();
        (dir, root)
    }

    #[test]
    fn layers_stack_with_whiteouts() {
        let (dir, root) = setup();
        let base = layer(
            dir.path(),
            "base.tar.gz",
            &[
                Item::Dir("etc/"),
                Item::File("etc/hostname", b"base", 0o644),
                Item::File("etc/gone", b"x", 0o644),
                Item::Dir("opt/app/"),
                Item::File("opt/app/old", b"old", 0o644),
                Item::File("bin/tool", b"#!", 0o4755),
                Item::Symlink("bin/sh", "busybox"),
                Item::Hardlink("bin/tool2", "bin/tool"),
                Item::Char("dev/null"),
            ],
            true,
        );
        apply_layer(&root, &base, crate::oci::media_types::OCI_LAYER_GZIP).unwrap();
        assert_eq!(fs::read(root.join("etc/hostname")).unwrap(), b"base");
        assert_eq!(
            fs::read_link(root.join("bin/sh")).unwrap(),
            PathBuf::from("busybox")
        );
        assert!(!root.join("dev/null").exists(), "device nodes are skipped");
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            assert_eq!(mode_of(&root.join("bin/tool")), 0o755, "setuid is dropped");
            assert_eq!(
                fs::metadata(root.join("bin/tool")).unwrap().ino(),
                fs::metadata(root.join("bin/tool2")).unwrap().ino()
            );
        }

        let upper = layer(
            dir.path(),
            "upper.tar",
            &[
                Item::File("etc/hostname", b"upper", 0o644),
                Item::File("etc/.wh.gone", b"", 0o644),
                // Same-layer content written before the opaque marker stays.
                Item::File("opt/app/new", b"new", 0o644),
                Item::File("opt/app/.wh..wh..opq", b"", 0o644),
            ],
            false,
        );
        apply_layer(&root, &upper, crate::oci::media_types::OCI_LAYER_TAR).unwrap();
        assert_eq!(fs::read(root.join("etc/hostname")).unwrap(), b"upper");
        assert!(!root.join("etc/gone").exists());
        assert!(
            !root.join("opt/app/old").exists(),
            "opaque dir hides lower content"
        );
        assert_eq!(fs::read(root.join("opt/app/new")).unwrap(), b"new");
        assert!(!root.join("opt/app/.wh..wh..opq").exists());
    }

    #[test]
    fn entries_cannot_escape_the_root() {
        let (dir, root) = setup();
        let escape = layer(
            dir.path(),
            "escape.tar",
            &[Item::File("../x", b"x", 0o644)],
            false,
        );
        assert!(matches!(
            apply_layer(&root, &escape, "").unwrap_err(),
            OciError::Unsafe(_)
        ));
        assert!(!dir.path().join("x").exists());

        // Through an absolute symbolic link: lands inside the root.
        let through = layer(
            dir.path(),
            "through.tar",
            &[
                Item::Symlink("evil", "/"),
                Item::File("evil/etc/passwd", b"root:x", 0o644),
                Item::Symlink("up", "../../.."),
                Item::File("up/marker", b"m", 0o644),
            ],
            false,
        );
        apply_layer(&root, &through, "").unwrap();
        assert_eq!(fs::read(root.join("etc/passwd")).unwrap(), b"root:x");
        assert_eq!(fs::read(root.join("marker")).unwrap(), b"m");
        assert!(!dir.path().join("marker").exists());

        // Hard links must point at files inside the root.
        fs::write(dir.path().join("outside"), b"secret").unwrap();
        let hard = layer(
            dir.path(),
            "hard.tar",
            &[Item::Hardlink("stolen", "../../outside")],
            false,
        );
        assert!(apply_layer(&root, &hard, "").is_err());
        assert!(!root.join("stolen").exists());

        let hard_via_link = layer(
            dir.path(),
            "hard2.tar",
            &[
                Item::Symlink("out", "/../.."),
                Item::Hardlink("stolen", "out/outside"),
            ],
            false,
        );
        assert!(apply_layer(&root, &hard_via_link, "").is_err());
    }

    #[test]
    fn writes_through_relative_directory_links_stay_inside() {
        let (dir, root) = setup();
        let l = layer(
            dir.path(),
            "lib.tar.gz",
            &[
                Item::Dir("usr/lib64/"),
                Item::Symlink("lib64", "usr/lib64"),
                Item::File("lib64/libc.so", b"elf", 0o755),
            ],
            true,
        );
        apply_layer(&root, &l, crate::oci::media_types::DOCKER_LAYER_GZIP).unwrap();
        assert_eq!(fs::read(root.join("usr/lib64/libc.so")).unwrap(), b"elf");
        assert!(
            fs::symlink_metadata(root.join("lib64"))
                .unwrap()
                .file_type()
                .is_symlink()
        );
    }

    #[test]
    fn symlink_loops_are_an_error() {
        let (dir, root) = setup();
        let l = layer(
            dir.path(),
            "loop.tar",
            &[
                Item::Symlink("a", "b"),
                Item::Symlink("b", "a"),
                Item::File("a/file", b"x", 0o644),
            ],
            false,
        );
        assert!(matches!(
            apply_layer(&root, &l, "").unwrap_err(),
            OciError::Unsafe(_)
        ));
    }

    #[test]
    fn zstd_is_rejected() {
        let (dir, root) = setup();
        let l = layer(dir.path(), "z.tar", &[Item::Dir("a/")], false);
        assert!(matches!(
            apply_layer(&root, &l, "application/vnd.oci.image.layer.v1.tar+zstd").unwrap_err(),
            OciError::Invalid(_)
        ));
    }

    #[test]
    fn resolve_in_root_re_roots_and_clamps() {
        let (_dir, root) = setup();
        fs::create_dir_all(root.join("usr/lib")).unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/usr/lib", root.join("abs")).unwrap();
            std::os::unix::fs::symlink("../../../usr", root.join("usr/lib/rel")).unwrap();
        }
        assert_eq!(
            resolve_in_root(&root, Path::new("abs/x"), false).unwrap(),
            root.join("usr/lib/x")
        );
        assert_eq!(
            resolve_in_root(&root, Path::new("abs"), false).unwrap(),
            root.join("abs")
        );
        assert_eq!(
            resolve_in_root(&root, Path::new("abs"), true).unwrap(),
            root.join("usr/lib")
        );
        assert_eq!(
            resolve_in_root(&root, Path::new("usr/lib/rel/lib"), true).unwrap(),
            root.join("usr/lib")
        );
        assert_eq!(
            resolve_in_root(&root, Path::new("../../etc/passwd"), true).unwrap(),
            root.join("etc/passwd")
        );
        assert_eq!(
            resolve_in_root(&root, Path::new("/missing/./a/../b"), true).unwrap(),
            root.join("missing/b")
        );
    }

    #[test]
    fn modification_times_are_kept() {
        let (dir, root) = setup();
        let mut builder = tar::Builder::new(Vec::new());
        let mut h = header(tar::EntryType::Regular, 0o644, 3);
        h.set_mtime(1_600_000_000);
        builder
            .append_data(&mut h, "lib/core.jar", &b"jar"[..])
            .unwrap();
        let path = dir.path().join("mtime.tar");
        fs::write(&path, builder.into_inner().unwrap()).unwrap();
        apply_layer(&root, &path, crate::oci::media_types::OCI_LAYER_TAR).unwrap();
        let modified = fs::metadata(root.join("lib/core.jar"))
            .unwrap()
            .modified()
            .unwrap();
        assert_eq!(
            modified,
            std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_600_000_000)
        );
    }

    #[test]
    fn directory_modes_stay_manageable() {
        let (dir, root) = setup();
        let mut builder = tar::Builder::new(Vec::new());
        let mut h = header(tar::EntryType::Directory, 0o500, 0);
        set_raw_name(&mut h, "locked/");
        h.set_cksum();
        builder.append(&h, std::io::empty()).unwrap();
        let mut h = header(tar::EntryType::Regular, 0o000, 1);
        set_raw_name(&mut h, "locked/f");
        h.set_cksum();
        builder.append(&h, &b"z"[..]).unwrap();
        let path = dir.path().join("modes.tar");
        fs::write(&path, builder.into_inner().unwrap()).unwrap();
        apply_layer(&root, &path, "").unwrap();
        #[cfg(unix)]
        {
            assert_eq!(mode_of(&root.join("locked")), 0o700);
            assert_eq!(mode_of(&root.join("locked/f")), 0o400);
        }
        // Whiteout of a locked tree works.
        let wh = layer(
            dir.path(),
            "wh.tar",
            &[Item::File(".wh.locked", b"", 0o644)],
            false,
        );
        apply_layer(&root, &wh, "").unwrap();
        assert!(!root.join("locked").exists());
    }
}
