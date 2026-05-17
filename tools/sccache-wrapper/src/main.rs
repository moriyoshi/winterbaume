// sccache wrapper that provides cross-worktree Rust compilation caching
// and automatically sets SCCACHE_BASEDIRS
//
// The core problem: sccache's SCCACHE_BASEDIRS does not normalise paths for
// Rust compilations (only C/C++ preprocessor output).  This wrapper adds a
// path-normalising cache layer on top of sccache so that identical
// compilations in different worktrees share cached artifacts via hardlinks.
//
// Flow:
//   cargo → sccache-wrapper (RUSTC_WRAPPER) → sccache → rustc
//
//   1. Parse rustc args minimally.
//   2. If cacheable: normalise paths, compute hash, check cache.
//      - HIT:  hardlink artifacts to --out-dir, replay stdout, exit 0.
//      - MISS: singleflight — acquire a per-key lock. If already held,
//              wait for the leader then try the cache again.  Otherwise
//              run sccache as child, capture output, hardlink outputs
//              into cache, exit with child's code.
//   3. If not cacheable: exec sccache directly (zero overhead).

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::SystemTime;

use sha2::{Digest, Sha256};

mod scoreboard;

// ---------------------------------------------------------------------------
// File locking (singleflight) — uses fslock for cross-platform support
// ---------------------------------------------------------------------------

mod lock {
    use std::fs;
    use std::path::Path;

    pub struct FileLock {
        inner: fslock::LockFile,
        held: bool,
    }

    impl FileLock {
        /// Open (or create) a lock file.  Does NOT acquire the lock yet.
        pub fn open(path: &Path) -> Option<Self> {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let inner = fslock::LockFile::open(path).ok()?;
            Some(Self { inner, held: false })
        }

        /// Try to acquire an exclusive lock without blocking.
        /// Returns `true` if the lock was acquired.
        pub fn try_exclusive(&mut self) -> bool {
            match self.inner.try_lock() {
                Ok(true) => {
                    self.held = true;
                    true
                }
                _ => false,
            }
        }

        /// Block until the exclusive lock is acquired.
        pub fn exclusive(&mut self) -> bool {
            match self.inner.lock() {
                Ok(()) => {
                    self.held = true;
                    true
                }
                Err(_) => false,
            }
        }

        /// Release the lock.
        pub fn release(&mut self) {
            if self.held {
                let _ = self.inner.unlock();
                self.held = false;
            }
        }
    }

    impl Drop for FileLock {
        fn drop(&mut self) {
            self.release();
        }
    }
}

// ---------------------------------------------------------------------------
// Workspace root detection
// ---------------------------------------------------------------------------

fn detect_workspace_root() -> Option<PathBuf> {
    // Fast path: pre-computed env var (set in mise.toml).
    if let Ok(root) = env::var("WB_WORKSPACE_ROOT") {
        return Some(PathBuf::from(root));
    }
    // Slow path: ask git.
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(PathBuf::from(
        String::from_utf8(out.stdout).ok()?.trim().to_owned(),
    ))
}

// ---------------------------------------------------------------------------
// Find the real sccache binary
// ---------------------------------------------------------------------------

fn find_sccache() -> Option<PathBuf> {
    let wrapper_path = env::current_exe().ok()?;
    let path_var = env::var_os("PATH")?;
    for dir in env::split_paths(&path_var) {
        let candidate = dir.join("sccache");
        if candidate == wrapper_path {
            continue;
        }
        #[cfg(windows)]
        let candidate = if !candidate.exists() {
            dir.join("sccache.exe")
        } else {
            candidate
        };
        if candidate.exists() {
            if let (Ok(a), Ok(b)) = (candidate.canonicalize(), wrapper_path.canonicalize())
                && a == b
            {
                continue;
            }
            return Some(candidate);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Minimal rustc argument parser
// ---------------------------------------------------------------------------

struct ParsedArgs {
    crate_name: String,
    extra_filename: String, // e.g. "-df6723ed6d521f1f"
    out_dir: PathBuf,
    emit: Vec<String>,        // ["dep-info", "metadata", "link"]
    crate_types: Vec<String>, // ["lib"]
    source_file: PathBuf,     // e.g. crates/winterbaume-core/src/lib.rs
    is_test: bool,            // `--test` was passed: rustc emits a test-harness binary
}

/// Attempt to extract the fields we need.  Returns `None` for invocations
/// that should not be cached (--print, -vV, missing --crate-name, etc.).
fn parse_rustc_args(args: &[String]) -> Option<ParsedArgs> {
    // Quick reject: --print or -vV or -V anywhere in the args.
    for a in args {
        if a == "--print" || a == "-vV" || a == "-V" {
            return None;
        }
    }

    let mut crate_name: Option<String> = None;
    let mut extra_filename = String::new();
    let mut out_dir: Option<PathBuf> = None;
    let mut emit: Vec<String> = Vec::new();
    let mut crate_types: Vec<String> = Vec::new();
    let mut source_file: Option<PathBuf> = None;
    let mut is_test = false;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];

        // --crate-name <name> or --crate-name=<name>
        if arg == "--crate-name" {
            i += 1;
            crate_name = args.get(i).cloned();
        } else if let Some(val) = arg.strip_prefix("--crate-name=") {
            crate_name = Some(val.to_owned());
        }
        // --out-dir <path> or --out-dir=<path>
        else if arg == "--out-dir" {
            i += 1;
            out_dir = args.get(i).map(PathBuf::from);
        } else if let Some(val) = arg.strip_prefix("--out-dir=") {
            out_dir = Some(PathBuf::from(val));
        }
        // --emit=dep-info,metadata,link
        else if let Some(val) = arg.strip_prefix("--emit=") {
            emit = val.split(',').map(|s| s.to_owned()).collect();
        } else if arg == "--emit" {
            i += 1;
            if let Some(val) = args.get(i) {
                emit = val.split(',').map(|s| s.to_owned()).collect();
            }
        }
        // --crate-type <type>
        else if arg == "--crate-type" {
            i += 1;
            if let Some(val) = args.get(i) {
                crate_types.push(val.clone());
            }
        } else if let Some(val) = arg.strip_prefix("--crate-type=") {
            crate_types.push(val.to_owned());
        }
        // --test — rustc builds a test-harness binary instead of the
        // requested crate-type.  We still cache these; expected_output_files
        // and cache_store/cache_restore handle the bin-shaped artefacts.
        else if arg == "--test" {
            is_test = true;
        }
        // -C extra-filename=-abc123
        else if arg == "-C" {
            i += 1;
            if let Some(val) = args.get(i)
                && let Some(ef) = val.strip_prefix("extra-filename=")
            {
                extra_filename = ef.to_owned();
            }
        } else if let Some(rest) = arg.strip_prefix("-Cextra-filename=") {
            extra_filename = rest.to_owned();
        }
        // Source file: a non-flag arg ending in .rs
        else if !arg.starts_with('-') && arg.ends_with(".rs") {
            source_file = Some(PathBuf::from(arg));
        }

        i += 1;
    }

    let crate_name = crate_name?;
    let out_dir = out_dir?;
    let source_file = source_file?;

    if emit.is_empty() {
        return None;
    }

    // Cache lib / rlib / proc-macro crate types, plus `--test` harnesses.
    //
    // Proc-macros are cached deliberately even though rustc treats them as a
    // dynamic library output. Their dylib byte content is non-deterministic
    // across rustc invocations (hash-table iteration order, etc.), and the
    // SVH of the resulting dylib gets baked into the rmeta of every crate
    // that depends on the proc-macro. If two sessions build the same
    // proc-macro independently, downstream rmetas reference different SVHs
    // even though cargo's `--extern foo=…librustversion-<hash>.dylib`
    // filename is identical. Restoring such a downstream rmeta into a
    // session whose freshly-built proc-macro has a different SVH then
    // surfaces as `E0463: can't find crate for X` when rustc walks the dep
    // chain. Caching the proc-macro stabilises its dylib content across
    // sessions and keeps the SVH chain consistent.
    if !is_test
        && crate_types
            .iter()
            .any(|t| matches!(t.as_str(), "bin" | "cdylib" | "staticlib"))
    {
        return None;
    }

    Some(ParsedArgs {
        crate_name,
        extra_filename,
        out_dir,
        emit,
        crate_types,
        source_file,
        is_test,
    })
}

// ---------------------------------------------------------------------------
// Output filename determination
// ---------------------------------------------------------------------------

fn expected_output_files(parsed: &ParsedArgs) -> Vec<String> {
    let is_lib = parsed.crate_types.is_empty()
        || parsed.crate_types.iter().any(|t| t == "lib" || t == "rlib");
    let is_proc_macro = parsed.crate_types.iter().any(|t| t == "proc-macro");
    let stem_lib = format!("lib{}{}", parsed.crate_name, parsed.extra_filename);
    let stem_bare = format!("{}{}", parsed.crate_name, parsed.extra_filename);
    // Test-harness binaries have no `lib` prefix and, on Windows, an `.exe`
    // suffix.  rustc may also drop a companion `.pdb` next to the binary in
    // debug profiles on Windows; we don't track it explicitly today (cargo
    // tolerates its absence and the binary itself is what matters for `cargo
    // test`).
    let bin_suffix = if cfg!(windows) { ".exe" } else { "" };
    // Proc-macros build a host dynamic library. On macOS it's
    // `lib<crate>-<ef>.dylib`, on Linux `lib<crate>-<ef>.so`, and on Windows
    // `<crate>-<ef>.dll` (no `lib` prefix). Pick the right stem+suffix pair.
    let proc_macro_stem = if cfg!(windows) { &stem_bare } else { &stem_lib };
    let proc_macro_suffix = if cfg!(target_os = "macos") {
        ".dylib"
    } else if cfg!(windows) {
        ".dll"
    } else {
        ".so"
    };

    let mut files = Vec::new();
    for e in &parsed.emit {
        match e.as_str() {
            "metadata" => files.push(format!("{stem_lib}.rmeta")),
            "link" if parsed.is_test => files.push(format!("{stem_bare}{bin_suffix}")),
            "link" if is_proc_macro => files.push(format!("{proc_macro_stem}{proc_macro_suffix}")),
            "link" if is_lib => files.push(format!("{stem_lib}.rlib")),
            "dep-info" => files.push(format!("{stem_bare}.d")),
            _ => {}
        }
    }
    files
}

/// Rewrite an artefact filename to use a different `extra-filename` suffix.
///
/// Cached filenames look like `lib<crate><stored_ef>.<ext>` (or
/// `<crate><stored_ef>.<ext>`, or `<crate><stored_ef>` for unix test
/// binaries).  When restoring into a worktree where cargo computed a
/// different metadata hash, `<stored_ef>` must be substituted with the
/// caller's `<new_ef>`.
///
/// Crucially, the substitution is anchored to the end of the stem rather
/// than performed via [`str::replace`] — an unanchored replace would
/// corrupt names whose crate component happens to contain `stored_ef` as
/// a substring (e.g. `liborder-d.rlib` with `stored_ef = "-d"`), producing
/// a filename that doesn't exist on disk and surfacing as `E0463: can't
/// find crate` when the parent compilation tries to load it via `--extern`.
fn rewrite_extra_filename(filename: &str, stored_ef: &str, new_ef: &str) -> String {
    if stored_ef == new_ef || stored_ef.is_empty() {
        return filename.to_owned();
    }
    // Split off the extension (if any) so we can match `stored_ef` against
    // the stem suffix.  For unix test binaries with no extension, the whole
    // filename is the stem.
    let (stem, ext) = match filename.rfind('.') {
        Some(idx) => (&filename[..idx], &filename[idx..]),
        None => (filename, ""),
    };
    match stem.strip_suffix(stored_ef) {
        Some(prefix) => format!("{prefix}{new_ef}{ext}"),
        None => filename.to_owned(),
    }
}

/// Substitute extra-filename inside `.d` (dep-info) file content, anchored to
/// the `<crate><ef>` boundary.  Dep-info content references this crate's own
/// outputs as `<crate><ef>.<ext>` (and `lib<crate><ef>.<ext>` for libraries);
/// `<ef>` always begins with `-`, so the search key `<crate><ef>` is unambiguous.
///
/// The naive `content.replace(ef, _)` would over-replace if the hex sequence in
/// `<ef>` happened to occur elsewhere in the file (e.g. inside an absolute path
/// component).  Anchoring on the crate-name prefix prevents that.
fn rewrite_extra_filename_in_d_content(
    content: &str,
    crate_name: &str,
    stored_ef: &str,
    new_ef: &str,
) -> String {
    if stored_ef == new_ef || stored_ef.is_empty() || crate_name.is_empty() {
        return content.to_owned();
    }
    let stored_anchor = format!("{crate_name}{stored_ef}");
    let new_anchor = format!("{crate_name}{new_ef}");
    content.replace(&stored_anchor, &new_anchor)
}

// ---------------------------------------------------------------------------
// Source file content hashing
// ---------------------------------------------------------------------------

/// Hash all .rs files under the source file's parent directory (recursively).
/// This catches changes to non-root source files (e.g. mod.rs, helper.rs).
fn hash_source_tree(source_file: &Path) -> Option<Vec<u8>> {
    let src_dir = if source_file.is_relative() {
        env::current_dir().ok()?.join(source_file)
    } else {
        source_file.to_owned()
    };
    let src_dir = src_dir.parent()?;

    let mut paths = Vec::new();
    collect_rs_files(src_dir, &mut paths);
    paths.sort(); // deterministic order

    let mut hasher = Sha256::new();
    for p in &paths {
        if let Ok(contents) = fs::read(p) {
            // Include relative path in hash to distinguish files with same content.
            if let Ok(rel) = p.strip_prefix(src_dir) {
                hasher.update(rel.to_string_lossy().as_bytes());
            }
            hasher.update(b"\0");
            hasher.update(&contents);
            hasher.update(b"\0");
        }
    }
    Some(hasher.finalize().to_vec())
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

// ---------------------------------------------------------------------------
// Cache key computation
// ---------------------------------------------------------------------------

const PLACEHOLDER: &str = "@@WORKSPACE@@";
const EXTFN_PLACEHOLDER: &str = "@@EXTFN@@";

fn is_debug() -> bool {
    env::var_os("WB_RUSTC_CACHE_DEBUG").is_some()
}

/// Print diagnostic information for a cache entry: crate name, key, result,
/// and the stored received/emitted command lines.
fn print_diag(crate_name: &str, key: &str, result: &str, workspace_root: &str) {
    eprintln!(
        "sccache-wrapper: {result} for {crate_name} ({key_short})",
        key_short = &key[..12]
    );
    let entry = cache_entry_dir(key);
    if let Ok(raw) = fs::read_to_string(entry.join("args_received")) {
        let denorm = raw.replace(PLACEHOLDER, workspace_root);
        eprintln!("  received: {}", shell_join(denorm.lines()));
    }
    if let Ok(raw) = fs::read_to_string(entry.join("args_emitted")) {
        let denorm = raw.replace(PLACEHOLDER, workspace_root);
        eprintln!("  emitted:  {}", shell_join(denorm.lines()));
    }
}

/// Join args into a single line, quoting values that contain spaces.
fn shell_join<'a>(args: impl Iterator<Item = &'a str>) -> String {
    args.map(|a| {
        if a.contains(' ') || a.contains('"') || a.is_empty() {
            format!("'{a}'")
        } else {
            a.to_owned()
        }
    })
    .collect::<Vec<_>>()
    .join(" ")
}

fn normalise_arg(arg: &str, workspace_root: &str) -> String {
    arg.replace(workspace_root, PLACEHOLDER)
}

fn compiler_identity(rustc_path: &str) -> Vec<u8> {
    // Fast: use file metadata (size + mtime) instead of running a subprocess.
    if let Ok(meta) = fs::metadata(rustc_path) {
        let mut hasher = Sha256::new();
        hasher.update(rustc_path.as_bytes());
        hasher.update(b"\0");
        hasher.update(meta.len().to_le_bytes());
        let mtime = meta
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_le_bytes();
        hasher.update(mtime);
        return hasher.finalize().to_vec();
    }
    // Fallback: hash the path itself.
    Sha256::digest(rustc_path.as_bytes()).to_vec()
}

/// Check whether a `-C` flag value should be excluded from the cache key.
///
/// `-C extra-filename=` is purely a filename suffix derived from metadata and
/// does not affect compiled output.  `-C incremental=` is a workspace-local
/// path for incremental compilation state.
///
/// `-C metadata=` is deliberately **kept** — it is the crate disambiguator
/// that cargo uses to distinguish target-context from host-context
/// compilations of the same crate.  Stripping it causes cache pollution
/// when cargo compiles a crate twice in one build (e.g. parking_lot used
/// both as a regular dep and via a proc-macro dep).  The value is stable
/// across worktrees for the same compilation context.
fn is_display_only_c_flag(val: &str) -> bool {
    val.starts_with("extra-filename=") || val.starts_with("incremental=")
}

/// Compute the sidecar path that records the cache key for the artifact
/// referenced by `--extern name=PATH`.  PATH is e.g.
/// `…/target/debug/deps/libfoo-abc123.rmeta`; the sidecar lives next to
/// the artifact as `…/target/debug/deps/foo-abc123.cachekey`.
fn cachekey_sidecar_for_extern(extern_path: &Path) -> Option<PathBuf> {
    let parent = extern_path.parent()?;
    let stem = extern_path.file_stem()?.to_string_lossy();
    let bare = stem.strip_prefix("lib").unwrap_or(&stem);
    Some(parent.join(format!("{bare}.cachekey")))
}

/// Sidecar path for an artifact in the current build's `--out-dir`,
/// keyed off the parsed crate name and extra-filename.
fn cachekey_sidecar_for_output(parsed: &ParsedArgs) -> PathBuf {
    parsed.out_dir.join(format!(
        "{}{}.cachekey",
        parsed.crate_name, parsed.extra_filename
    ))
}

/// Cache-key fragment for an `--extern name=PATH` argument when no sidecar
/// is available.  Returns `name=<basename>` rather than `name=<full path>`
/// so that the same dep resolves to the same cache key across different
/// `CARGO_TARGET_DIR` settings.  The basename embeds cargo's metadata hash
/// (e.g. `libserde_derive-abc123.dylib`), which is the part that genuinely
/// disambiguates one compilation of a dep from another; the path prefix
/// just records where cargo happens to be writing today.
///
/// Falls back to the literal value when it doesn't have the expected
/// `name=path` shape — defensive only, cargo always uses that form.
fn extern_basename_key(val: &str) -> String {
    if let Some((name, path)) = val.split_once('=') {
        let basename = Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_owned());
        format!("{name}={basename}")
    } else {
        val.to_owned()
    }
}

/// Remove `-C incremental=…` from rustc arguments so that sccache sees
/// deterministic, non-incremental compilations.
fn strip_incremental(args: &[String]) -> Vec<String> {
    let mut out = Vec::with_capacity(args.len());
    let mut i = 0;
    while i < args.len() {
        if args[i] == "-C" {
            if let Some(next) = args.get(i + 1)
                && next.starts_with("incremental=")
            {
                i += 2;
                continue;
            }
        } else if args[i].starts_with("-Cincremental=") {
            i += 1;
            continue;
        }
        out.push(args[i].clone());
        i += 1;
    }
    out
}

/// Resolved cache key for the current compilation, plus the cache keys of
/// each `--extern` dependency that contributed to it (as `(name, key)`
/// pairs).  Returned together so callers don't have to rescan args.
struct CacheKey {
    key: String,
    deps: Vec<(String, String)>,
}

fn compute_cache_key(
    rustc_path: &str,
    args: &[String],
    parsed: &ParsedArgs,
    workspace_root: &str,
) -> Option<CacheKey> {
    let mut hasher = Sha256::new();
    let mut deps: Vec<(String, String)> = Vec::new();

    // 1. Compiler identity.
    hasher.update(compiler_identity(rustc_path));
    hasher.update(b"\x00");

    // 2. Normalised arguments — filtered to exclude values that cargo
    //    derives from the workspace path and that therefore differ across
    //    worktrees even for identical source.
    let debug = is_debug();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];

        // -C extra-filename= / incremental= — skip (see is_display_only_c_flag).
        if arg == "-C" {
            if let Some(next) = args.get(i + 1)
                && is_display_only_c_flag(next)
            {
                i += 2;
                continue;
            }
        } else if arg.starts_with("-C") && is_display_only_c_flag(&arg[2..]) {
            i += 1;
            continue;
        }

        // --out-dir — workspace-dependent, doesn't affect output content.
        if arg == "--out-dir" {
            i += 2;
            continue;
        }
        if arg.starts_with("--out-dir=") {
            i += 1;
            continue;
        }

        // Display-only flags — vary with terminal size / colour support
        // but never affect compiled output.
        if arg == "--diagnostic-width"
            || arg == "--color"
            || arg.starts_with("--diagnostic-width=")
            || arg.starts_with("--color=")
        {
            // Skip the flag itself.  For the two-arg form also skip the
            // following value.
            if !arg.contains('=') {
                i += 1;
            }
            i += 1;
            continue;
        }

        // -L — search paths are workspace-dependent and do not affect
        // output content (actual deps are pinned via --extern).
        if arg == "-L" {
            i += 2;
            continue;
        }
        if arg.starts_with("-L") {
            i += 1;
            continue;
        }

        // --extern name=PATH — prefer to identify the dep by its OWN cache
        // key (read from a sidecar file the wrapper writes next to each
        // restored/stored artifact).  This is workspace-independent, so
        // cross-worktree caching works even when cargo computes different
        // `-C metadata=` hashes for the dep in different trees.  Crucially,
        // the dep's cache key is derived from the dep's source content —
        // if the dep changes, the dep's key changes, and our key changes,
        // so we correctly miss rather than serve a stale parent .rlib that
        // internally references the wrong dep crate hash (E0460).
        //
        // Fallback: if no sidecar exists (e.g. dep is a proc-macro, which
        // the wrapper does not cache), key off the artifact's *basename*
        // rather than the full path.  The basename embeds cargo's metadata
        // hash and is target-dir-independent, so two compilations that
        // resolve the same dep version under different `CARGO_TARGET_DIR`
        // settings hash to the same cache key.  Using the full path here
        // (workspace-normalised or not) breaks cross-target-dir caching
        // whenever any extern lacks a sidecar — see the proc-macro case.
        if arg == "--extern" {
            i += 1;
            if let Some(val) = args.get(i) {
                hasher.update(b"--extern\x00");
                let mut used_sidecar = false;
                if let Some((name, path)) = val.split_once('=')
                    && let Some(sidecar) = cachekey_sidecar_for_extern(Path::new(path))
                    && let Ok(raw) = fs::read_to_string(&sidecar)
                {
                    let dep_key = raw.trim().to_owned();
                    hasher.update(name.as_bytes());
                    hasher.update(b"=");
                    hasher.update(dep_key.as_bytes());
                    if debug {
                        eprintln!(
                            "  cache-key arg[{i}]: --extern {name}=<dep_key:{}>",
                            &dep_key[..12.min(dep_key.len())]
                        );
                    }
                    deps.push((name.to_owned(), dep_key));
                    used_sidecar = true;
                }
                if !used_sidecar {
                    let key_part = extern_basename_key(val);
                    if debug {
                        eprintln!("  cache-key arg[{i}]: --extern {key_part}");
                    }
                    hasher.update(key_part.as_bytes());
                }
                hasher.update(b"\x00");
            }
            i += 1;
            continue;
        }

        // Everything else: normalise workspace root and include.
        let normalised = normalise_arg(arg, workspace_root);
        if debug {
            eprintln!("  cache-key arg[{i}]: {normalised}");
        }
        hasher.update(normalised.as_bytes());
        hasher.update(b"\x00");
        i += 1;
    }

    // 3. Source tree content hash.
    let src_hash = hash_source_tree(&parsed.source_file)?;
    hasher.update(&src_hash);

    let digest = hasher.finalize();
    let key: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    Some(CacheKey { key, deps })
}

// ---------------------------------------------------------------------------
// Cache directory helpers
// ---------------------------------------------------------------------------

fn cache_base_dir() -> PathBuf {
    if let Ok(dir) = env::var("WB_RUSTC_CACHE_DIR") {
        return PathBuf::from(dir);
    }
    env::temp_dir().join("winterbaume-rustc-cache")
}

fn cache_entry_dir(key: &str) -> PathBuf {
    cache_base_dir().join(&key[..2]).join(key)
}

fn cache_lock_path(key: &str) -> PathBuf {
    cache_base_dir()
        .join("locks")
        .join(&key[..2])
        .join(format!("{key}.lock"))
}

// ---------------------------------------------------------------------------
// Cache restore (on hit) — hardlinks where possible
// ---------------------------------------------------------------------------

fn cache_restore(key: &str, parsed: &ParsedArgs, workspace_root: &str) -> Option<i32> {
    // Files we have written into `out_dir`.  Tracked so a partial-restore
    // failure can be unwound, leaving rustc a clean slate to recompile into.
    let mut written: Vec<PathBuf> = Vec::new();

    let result = (|| -> io::Result<i32> {
        let entry = cache_entry_dir(key);
        let manifest = fs::read_to_string(entry.join("manifest"))?;

        let mut files: Vec<String> = Vec::new();
        let mut stored_extra_filename: Option<String> = None;
        // Default to 0 so legacy entries (written before the `exit:` field
        // existed) still replay correctly — those were only stored on success.
        let mut exit_code: i32 = 0;
        for line in manifest.lines() {
            if let Some(name) = line.strip_prefix("file:") {
                files.push(name.to_owned());
            } else if let Some(ef) = line.strip_prefix("extra-filename:") {
                stored_extra_filename = Some(ef.to_owned());
            } else if let Some(code) = line.strip_prefix("exit:") {
                exit_code = code.trim().parse().unwrap_or(0);
            }
        }

        // Verify all files exist in cache before restoring anything.
        for f in &files {
            if !entry.join(f).exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("cache entry missing file {f}"),
                ));
            }
        }

        fs::create_dir_all(&parsed.out_dir)?;

        // The cached filenames may use a different extra-filename suffix (from
        // a different worktree).  Map them to the names cargo expects now.
        let stored_ef = stored_extra_filename
            .as_deref()
            .unwrap_or(&parsed.extra_filename);

        for f in &files {
            let src = entry.join(f);
            let expected_name = rewrite_extra_filename(f, stored_ef, &parsed.extra_filename);
            let dst = parsed.out_dir.join(&expected_name);

            // Track before write so a partial write is included in cleanup.
            written.push(dst.clone());

            // Remove destination if it exists (hardlink requires no existing target).
            let _ = fs::remove_file(&dst);

            if f.ends_with(".d") {
                // Dep-info needs path rewriting — copy with substitution.
                let content = fs::read_to_string(&src)?;
                let path_rewritten = content.replace(PLACEHOLDER, workspace_root);
                let rewritten = rewrite_extra_filename_in_d_content(
                    &path_rewritten,
                    &parsed.crate_name,
                    EXTFN_PLACEHOLDER,
                    &parsed.extra_filename,
                );
                fs::write(&dst, rewritten)?;
            } else if fs::hard_link(&src, &dst).is_err() {
                // Hardlink failed (e.g. cross-filesystem) — fall back to copy.
                fs::copy(&src, &dst)?;
            }
        }

        // Drop a sidecar file recording this entry's cache key, so future
        // parent-crate compilations can identify this dep by its key rather
        // than by its workspace-dependent metadata-hash filename.  Sidecar
        // failure is non-fatal — it only degrades dep-resolution to the
        // path-based fallback.
        if let Err(e) = fs::write(cachekey_sidecar_for_output(parsed), key)
            && is_debug()
        {
            eprintln!(
                "sccache-wrapper: failed to write cache-key sidecar for {}: {e}",
                parsed.crate_name,
            );
        }

        // Replay stdout (with path rewriting) and stderr.  Output-forwarding
        // failures (e.g. broken pipe) are treated as non-fatal: the cache
        // restore itself still succeeded.
        if let Ok(stdout) = fs::read_to_string(entry.join("stdout")) {
            let rewritten = stdout.replace(PLACEHOLDER, workspace_root);
            let _ = io::stdout().write_all(rewritten.as_bytes());
        }
        if let Ok(stderr) = fs::read(entry.join("stderr")) {
            let _ = io::stderr().write_all(&stderr);
        }

        Ok(exit_code)
    })();

    match result {
        Ok(code) => Some(code),
        Err(e) => {
            if is_debug() {
                eprintln!(
                    "sccache-wrapper: cache restore failed for {} ({}): {e}",
                    parsed.crate_name,
                    &key[..12.min(key.len())],
                );
            }
            // Best-effort cleanup so the recompile that follows starts clean.
            for p in &written {
                let _ = fs::remove_file(p);
            }
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Cache store (on miss) — hardlinks from out_dir into cache
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn cache_store(
    key: &str,
    parsed: &ParsedArgs,
    workspace_root: &str,
    exit_code: i32,
    stdout: &[u8],
    stderr: &[u8],
    args_received: &[String],
    args_emitted: &[String],
    deps: &[(String, String)],
) {
    let entry = cache_entry_dir(key);
    let tmp = entry.with_extension(format!("tmp.{}", std::process::id()));

    // The closure below populates `tmp`, then renames it to `entry`.  Any
    // failure short-circuits via `?`, which is the only way to guarantee we
    // never publish a half-written cache entry into the live tree.
    let result = (|| -> io::Result<()> {
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;

        let expected = expected_output_files(parsed);
        let mut manifest_lines = Vec::new();

        // Record extra-filename so that restore can map filenames when the
        // metadata hash differs (cross-worktree hit).
        manifest_lines.push(format!("extra-filename:{}", parsed.extra_filename));

        // Record rustc's exit status so the cache replays it faithfully on
        // hit.  Today we only call cache_store on success (exit 0), but
        // recording the value keeps the door open for caching non-zero
        // exits later without a manifest format change.
        manifest_lines.push(format!("exit:{exit_code}"));

        // Record this entry's resolved dependencies (name → cache key of the
        // specific compilation we linked against), as resolved during cache
        // key computation.  Stored for diagnostic dumps.
        for (name, dep_key) in deps {
            manifest_lines.push(format!("dep:{name}={dep_key}"));
        }

        for filename in &expected {
            let src = parsed.out_dir.join(filename);
            if !src.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("expected output {filename} was not produced"),
                ));
            }

            let dst = tmp.join(filename);

            if filename.ends_with(".d") {
                // Dep-info: copy with path normalisation.  Also normalise the
                // extra-filename so the entry is worktree-independent.  The
                // extra-filename substitution is anchored to the `<crate><ef>`
                // boundary so an unrelated coincidental hex match elsewhere in
                // the file content is not corrupted.
                let content = fs::read_to_string(&src)?;
                let path_normalised = content.replace(workspace_root, PLACEHOLDER);
                let normalised = rewrite_extra_filename_in_d_content(
                    &path_normalised,
                    &parsed.crate_name,
                    &parsed.extra_filename,
                    EXTFN_PLACEHOLDER,
                );
                fs::write(&dst, normalised)?;
            } else if fs::hard_link(&src, &dst).is_err() {
                // Hardlink failed (e.g. cross-filesystem) — fall back to copy.
                fs::copy(&src, &dst)?;
            }
            manifest_lines.push(format!("file:{filename}"));
        }

        // Store the original and emitted command lines (normalised) for diagnostics.
        let received_normalised: Vec<String> = args_received
            .iter()
            .map(|a| normalise_arg(a, workspace_root))
            .collect();
        fs::write(tmp.join("args_received"), received_normalised.join("\n"))?;

        let emitted_normalised: Vec<String> = args_emitted
            .iter()
            .map(|a| normalise_arg(a, workspace_root))
            .collect();
        fs::write(tmp.join("args_emitted"), emitted_normalised.join("\n"))?;

        // Store stdout/stderr with normalised paths.
        let stdout_str = String::from_utf8_lossy(stdout);
        let normalised_stdout = stdout_str.replace(workspace_root, PLACEHOLDER);
        fs::write(tmp.join("stdout"), normalised_stdout.as_bytes())?;
        fs::write(tmp.join("stderr"), stderr)?;

        // Manifest is written last so that any earlier failure leaves an
        // obviously incomplete tmp directory rather than a manifest-without-
        // payload entry.
        let manifest = manifest_lines.join("\n") + "\n";
        fs::write(tmp.join("manifest"), manifest)?;

        if let Some(parent) = entry.parent() {
            fs::create_dir_all(parent)?;
        }

        // Atomic rename.  If a concurrent build of the same key already
        // populated `entry`, prefer the existing entry — its contents are
        // semantically equivalent — and discard ours.
        if let Err(e) = fs::rename(&tmp, &entry) {
            if entry.exists() {
                return Ok(());
            }
            return Err(e);
        }
        Ok(())
    })();

    // `tmp` is consumed by a successful rename, but the concurrent-loser
    // branch and every error path leave it behind.  Clean up unconditionally.
    let _ = fs::remove_dir_all(&tmp);

    if let Err(e) = result {
        if is_debug() {
            eprintln!(
                "sccache-wrapper: cache store failed for {} ({}): {e}",
                parsed.crate_name,
                &key[..12.min(key.len())],
            );
        }
        return;
    }

    // Drop a sidecar file in the deps dir recording this entry's cache key.
    // Future parent-crate compilations read it via `--extern` to identify
    // this dep by its workspace-independent key.  Sidecar failure is
    // non-fatal — it only degrades dep-resolution to the path-based fallback.
    if let Err(e) = fs::write(cachekey_sidecar_for_output(parsed), key)
        && is_debug()
    {
        eprintln!(
            "sccache-wrapper: failed to write cache-key sidecar for {}: {e}",
            parsed.crate_name,
        );
    }
}

// ---------------------------------------------------------------------------
// SCCACHE_BASEDIRS computation (kept for non-cached pass-through path)
// ---------------------------------------------------------------------------

fn compute_basedirs() -> Option<String> {
    let toplevel_output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !toplevel_output.status.success() {
        return None;
    }
    let toplevel = PathBuf::from(
        String::from_utf8(toplevel_output.stdout)
            .ok()?
            .trim()
            .to_owned(),
    );

    let common_output = Command::new("git")
        .args(["rev-parse", "--git-common-dir"])
        .current_dir(&toplevel)
        .output()
        .ok()?;
    if !common_output.status.success() {
        return None;
    }
    let common_relative = String::from_utf8(common_output.stdout).ok()?;
    let common_relative = common_relative.trim();

    let common_abs = if PathBuf::from(common_relative).is_absolute() {
        PathBuf::from(common_relative)
    } else {
        toplevel.join(common_relative)
    };
    let common_abs = common_abs.canonicalize().ok()?;
    let main_root = common_abs.parent()?;

    let separator = if cfg!(windows) { ";" } else { ":" };

    if toplevel == main_root {
        Some(main_root.to_string_lossy().into_owned())
    } else {
        let toplevel = toplevel.canonicalize().ok()?;
        Some(format!(
            "{}{}{}",
            main_root.display(),
            separator,
            toplevel.display()
        ))
    }
}

// ---------------------------------------------------------------------------
// Set up environment for the sccache child process
// ---------------------------------------------------------------------------

fn setup_sccache_env(cmd: &mut Command) {
    if env::var_os("SCCACHE_BASEDIRS").is_none()
        && let Some(basedirs) = compute_basedirs()
    {
        cmd.env("SCCACHE_BASEDIRS", basedirs);
    }
}

// ---------------------------------------------------------------------------
// Compilation + cache-store helper
// ---------------------------------------------------------------------------

fn compile_and_cache(
    cache_key: &str,
    parsed: &ParsedArgs,
    workspace_root: &str,
    sccache: &Path,
    args_received: &[String],
    args_emitted: &[String],
    deps: &[(String, String)],
) -> ExitCode {
    let mut cmd = Command::new(sccache);
    cmd.args(args_emitted);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    setup_sccache_env(&mut cmd);

    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("sccache-wrapper: failed to run sccache: {e}");
            return ExitCode::from(126);
        }
    };

    // Forward stdout/stderr to the real streams.
    let _ = io::stdout().write_all(&output.stdout);
    let _ = io::stderr().write_all(&output.stderr);

    let exit_code = output.status.code().unwrap_or(1);

    // Cache successful compilations only.  We deliberately do not store
    // failed builds (negative caching): rustc errors are deterministic for
    // the same source + args + compiler, so caching them would be valid in
    // principle, but stderr is not path-normalised today and a cached
    // failure would replay the original worktree's paths.  Keep the gate
    // here so the rest of the pipeline (manifest fields, cache_restore exit
    // replay) remains future-proof for whenever we decide to flip it.
    if exit_code == 0 {
        cache_store(
            cache_key,
            parsed,
            workspace_root,
            exit_code,
            &output.stdout,
            &output.stderr,
            args_received,
            args_emitted,
            deps,
        );
    }

    ExitCode::from(exit_code as u8)
}

// ---------------------------------------------------------------------------
// GC: remove stale entries (older entries when a newer one exists for
// the same crate + compilation context)
// ---------------------------------------------------------------------------

/// Extract (program_kind, crate_name, metadata) from normalised `args_received` text.
/// `args_received` is one arg per line (same format as stored by cache_store);
/// the first line is the compiler path, and `program_kind` is its file stem
/// (e.g. `rustc` vs `clippy-driver`), suffixed with `-test` when `--test` is
/// present. Two entries with matching crate_name and metadata but differing
/// programs are independent artefacts and must not supersede each other
/// during GC: rustc-vs-clippy produce different content for the same input,
/// and rustc-vs-rustc-test produce different artefact shapes (`.rlib` vs
/// test-harness binary) that are not interchangeable.
fn extract_entry_identity(args_received: &str) -> (String, String, String) {
    let lines: Vec<&str> = args_received.lines().collect();
    let mut program_kind = lines
        .first()
        .map(|line| {
            Path::new(line)
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| (*line).to_owned())
        })
        .unwrap_or_else(|| "?".to_owned());
    let mut crate_name = String::from("?");
    let mut metadata = String::new();
    let mut is_test = false;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line == "--crate-name" {
            if let Some(&next) = lines.get(i + 1) {
                crate_name = next.to_owned();
                i += 2;
                continue;
            }
        } else if let Some(name) = line.strip_prefix("--crate-name=") {
            crate_name = name.to_owned();
        } else if line == "--test" {
            is_test = true;
        } else if line == "-C" {
            if let Some(&next) = lines.get(i + 1)
                && let Some(val) = next.strip_prefix("metadata=")
            {
                metadata = val.to_owned();
                i += 2;
                continue;
            }
        } else if let Some(val) = line.strip_prefix("-Cmetadata=") {
            metadata = val.to_owned();
        }
        i += 1;
    }

    if is_test {
        program_kind.push_str("-test");
    }

    (program_kind, crate_name, metadata)
}

/// Compute total byte size of regular files directly inside `path` (non-recursive).
fn dir_size(path: &Path) -> u64 {
    let Ok(rd) = fs::read_dir(path) else {
        return 0;
    };
    rd.flatten()
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}

struct EntryInfo {
    path: PathBuf,
    key: String,
    program_kind: String,
    crate_name: String,
    metadata: String,
    mtime: SystemTime,
}

/// Walk the cache directory and identify duplicate entries that GC would
/// evict. Returns `(infos, to_remove_indices)` so callers can either remove
/// the targets ( `--gc` ) or just summarise them ( the dry-run report
/// embedded in `--dump-cache` ). Returns `Err(io::Error)` if the cache base
/// dir is unreadable; an empty cache is `Ok((vec![], vec![]))`.
fn gc_scan() -> io::Result<(Vec<EntryInfo>, Vec<usize>)> {
    let base = cache_base_dir();
    let prefixes = fs::read_dir(&base)?;

    let mut infos: Vec<EntryInfo> = Vec::new();

    for prefix in prefixes.flatten() {
        let prefix_path = prefix.path();
        if !prefix_path.is_dir() || prefix.file_name() == "locks" {
            continue;
        }
        let Ok(rd) = fs::read_dir(&prefix_path) else {
            continue;
        };
        for entry in rd.flatten() {
            let entry_path = entry.path();
            if !entry_path.is_dir() {
                continue;
            }
            let manifest_path = entry_path.join("manifest");
            if !manifest_path.exists() {
                continue;
            }

            let key = entry_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();

            let mtime = fs::metadata(&manifest_path)
                .and_then(|m| m.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH);

            let (program_kind, crate_name, metadata) =
                fs::read_to_string(entry_path.join("args_received"))
                    .map(|raw| extract_entry_identity(&raw))
                    .unwrap_or_else(|_| ("?".to_owned(), "?".to_owned(), String::new()));

            infos.push(EntryInfo {
                path: entry_path,
                key,
                program_kind,
                crate_name,
                metadata,
                mtime,
            });
        }
    }

    // Group by (program_kind, crate_name, metadata). The program kind keeps
    // rustc, clippy-driver, and `--test` rustc invocations (`rustc-test`) in
    // separate buckets — they may share crate_name+metadata but produce
    // non-interchangeable artefacts (rustc emits an `.rlib`, clippy emits a
    // diagnostics-only `.rmeta`, and rustc-test emits a binary harness),
    // so a newer entry of one kind must not evict an older entry of another.
    use std::collections::HashMap;
    let mut groups: HashMap<(String, String, String), Vec<usize>> = HashMap::new();
    for (i, info) in infos.iter().enumerate() {
        groups
            .entry((
                info.program_kind.clone(),
                info.crate_name.clone(),
                info.metadata.clone(),
            ))
            .or_default()
            .push(i);
    }

    // Within each group, keep the newest entry (highest mtime) and mark the rest for removal.
    let mut to_remove: Vec<usize> = Vec::new();
    for indices in groups.values() {
        if indices.len() <= 1 {
            continue;
        }
        let mut sorted = indices.clone();
        sorted.sort_by(|&a, &b| infos[b].mtime.cmp(&infos[a].mtime));
        to_remove.extend_from_slice(&sorted[1..]);
    }

    Ok((infos, to_remove))
}

fn run_gc(dry_run: bool) -> ExitCode {
    let (infos, to_remove) = match gc_scan() {
        Ok(out) => out,
        Err(e) => {
            eprintln!(
                "sccache-wrapper: cannot read cache dir {}: {e}",
                cache_base_dir().display()
            );
            return ExitCode::from(1);
        }
    };

    if to_remove.is_empty() {
        println!("sccache-wrapper: nothing to collect (no stale entries found)");
        return ExitCode::SUCCESS;
    }

    let mut removed = 0usize;
    let mut bytes_freed = 0u64;

    for &idx in &to_remove {
        let info = &infos[idx];
        let size = dir_size(&info.path);
        let short_key = &info.key[..12.min(info.key.len())];
        if dry_run {
            println!(
                "would remove: {} [{}] ({short_key}) ~{size} bytes",
                info.crate_name, info.program_kind,
            );
            bytes_freed += size;
        } else {
            match fs::remove_dir_all(&info.path) {
                Ok(()) => {
                    println!(
                        "removed: {} [{}] ({short_key})",
                        info.crate_name, info.program_kind,
                    );
                    removed += 1;
                    bytes_freed += size;
                    // Also clean up the lock file if it exists.
                    let lock = cache_lock_path(&info.key);
                    let _ = fs::remove_file(&lock);
                }
                Err(e) => {
                    eprintln!(
                        "sccache-wrapper: failed to remove {}: {e}",
                        info.path.display()
                    );
                }
            }
        }
    }

    if dry_run {
        println!(
            "\n{} entries would be removed (~{bytes_freed} bytes freed)",
            to_remove.len(),
        );
    } else {
        println!("\n{removed} entries removed ({bytes_freed} bytes freed)");
    }

    ExitCode::SUCCESS
}

// ---------------------------------------------------------------------------
// Diagnostic: dump cache contents
// ---------------------------------------------------------------------------

fn clear_cache() -> ExitCode {
    let base = cache_base_dir();
    if !base.exists() {
        eprintln!(
            "sccache-wrapper: cache directory does not exist: {}",
            base.display()
        );
        return ExitCode::SUCCESS;
    }
    match fs::remove_dir_all(&base) {
        Ok(()) => {
            eprintln!("sccache-wrapper: cleared cache at {}", base.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!(
                "sccache-wrapper: failed to clear cache at {}: {e}",
                base.display()
            );
            ExitCode::from(1)
        }
    }
}

fn dump_cache() -> ExitCode {
    let base = cache_base_dir();
    let workspace_root = detect_workspace_root()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let mut entries: Vec<PathBuf> = Vec::new();
    // Walk <base>/<2-char prefix>/<hash>/manifest
    let prefixes = match fs::read_dir(&base) {
        Ok(rd) => rd,
        Err(e) => {
            eprintln!(
                "sccache-wrapper: cannot read cache dir {}: {e}",
                base.display()
            );
            return ExitCode::from(1);
        }
    };
    for prefix in prefixes.flatten() {
        if !prefix.path().is_dir() || prefix.file_name() == "locks" {
            continue;
        }
        if let Ok(rd) = fs::read_dir(prefix.path()) {
            for entry in rd.flatten() {
                if entry.path().is_dir() && entry.path().join("manifest").exists() {
                    entries.push(entry.path());
                }
            }
        }
    }
    entries.sort();

    for entry in &entries {
        let key = entry
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let manifest = match fs::read_to_string(entry.join("manifest")) {
            Ok(m) => m,
            Err(_) => continue,
        };

        // Extract crate name from args_received (first --crate-name value).
        let crate_name = fs::read_to_string(entry.join("args_received"))
            .ok()
            .and_then(|raw| {
                let lines: Vec<&str> = raw.lines().collect();
                lines
                    .windows(2)
                    .find(|w| w[0] == "--crate-name")
                    .map(|w| w[1].to_owned())
            })
            .unwrap_or_else(|| {
                // Fall back to parsing the manifest filenames.
                manifest
                    .lines()
                    .find_map(|l| l.strip_prefix("file:"))
                    .and_then(|f| {
                        let stem = f.strip_prefix("lib").unwrap_or(f);
                        stem.split('-').next().map(|s| s.to_owned())
                    })
                    .unwrap_or_else(|| "?".to_owned())
            });

        let extra_fn = manifest
            .lines()
            .find_map(|l| l.strip_prefix("extra-filename:"))
            .unwrap_or("");

        println!(
            "--- {crate_name} ({}) ef={extra_fn} ---",
            &key[..12.min(key.len())]
        );

        if let Ok(raw) = fs::read_to_string(entry.join("args_received")) {
            let denorm = raw.replace(PLACEHOLDER, &workspace_root);
            println!("  received: {}", shell_join(denorm.lines()));
        } else {
            println!("  received: (not stored)");
        }
        if let Ok(raw) = fs::read_to_string(entry.join("args_emitted")) {
            let denorm = raw.replace(PLACEHOLDER, &workspace_root);
            println!("  emitted:  {}", shell_join(denorm.lines()));
        } else {
            println!("  emitted:  (not stored)");
        }

        let files: Vec<&str> = manifest
            .lines()
            .filter_map(|l| l.strip_prefix("file:"))
            .collect();
        if !files.is_empty() {
            println!("  files:    {}", files.join(", "));
        }

        let deps: Vec<&str> = manifest
            .lines()
            .filter_map(|l| l.strip_prefix("dep:"))
            .collect();
        if !deps.is_empty() {
            println!("  deps:");
            for d in &deps {
                if let Some((name, dep_key)) = d.split_once('=') {
                    println!("    {name} → {}", &dep_key[..12.min(dep_key.len())]);
                } else {
                    println!("    {d}");
                }
            }
        }
    }

    println!("\n{} cache entries in {}", entries.len(), base.display());

    // Surface a one-line GC hint so the user can see when duplicate entries
    // have accumulated without having to run `--gc --dry-run` separately.
    if let Ok((infos, to_remove)) = gc_scan()
        && !to_remove.is_empty()
    {
        let bytes: u64 = to_remove.iter().map(|&i| dir_size(&infos[i].path)).sum();
        println!(
            "GC would reclaim {} duplicate entries (~{bytes} bytes); run `sccache-wrapper --gc` to apply.",
            to_remove.len(),
        );
    }

    ExitCode::SUCCESS
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> ExitCode {
    // Standalone diagnostic mode: sccache-wrapper --dump-cache
    if env::args().nth(1).as_deref() == Some("--dump-cache") {
        return dump_cache();
    }
    if env::args().nth(1).as_deref() == Some("--gc") {
        let dry_run = env::args().any(|a| a == "--dry-run");
        return run_gc(dry_run);
    }
    if env::args().nth(1).as_deref() == Some("--show-scoreboard") {
        return match scoreboard::show_scoreboard() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("sccache-wrapper: {e}");
                ExitCode::from(1)
            }
        };
    }
    if matches!(
        env::args().nth(1).as_deref(),
        Some("--clear-cache" | "--reset")
    ) {
        return clear_cache();
    }

    // Honour kill-switch.
    if env::var("WB_RUSTC_CACHE").as_deref() == Ok("0") {
        return passthrough_exec();
    }

    let sccache = match find_sccache() {
        Some(p) => p,
        None => {
            if is_debug() {
                eprintln!("sccache-wrapper: could not find sccache in PATH");
            }
            return ExitCode::from(127);
        }
    };

    // args[0] = rustc path, args[1..] = rustc flags.
    let all_args: Vec<String> = env::args().skip(1).map(|a| a.to_string()).collect();
    if all_args.is_empty() {
        return exec_sccache(&sccache, &all_args);
    }

    let rustc_path = &all_args[0];
    let rustc_args = &all_args[1..];

    // Detect workspace root for path normalisation.
    let workspace_root = detect_workspace_root();

    // Try to parse as a cacheable compilation.
    let parsed = parse_rustc_args(rustc_args);

    let cacheable = parsed.is_some() && workspace_root.is_some();

    if !cacheable {
        // Non-cacheable: exec sccache directly (no overhead).
        return exec_sccache(&sccache, &all_args);
    }

    let workspace_root = workspace_root.unwrap();
    let workspace_root_str = workspace_root.to_string_lossy().to_string();
    let parsed = parsed.unwrap();

    // Strip `-C incremental=…` from the args passed to sccache/rustc.
    // Incremental compilation is incompatible with sccache (non-deterministic
    // outputs) and cargo already set the flag based on CARGO_INCREMENTAL —
    // removing it here is the only effective way to disable it since
    // CARGO_INCREMENTAL is a cargo-level variable that has no effect on the
    // rustc subprocess.
    let emitted_args = strip_incremental(&all_args);

    // Compute cache key (and collect resolved dep cache keys).
    let CacheKey {
        key: cache_key,
        deps,
    } = match compute_cache_key(rustc_path, rustc_args, &parsed, &workspace_root_str) {
        Some(k) => k,
        None => return exec_sccache(&sccache, &emitted_args),
    };

    let debug = is_debug();

    // Try cache hit (fast path — no locking, no scoreboard registration).
    if let Some(exit_code) = cache_restore(&cache_key, &parsed, &workspace_root_str) {
        if debug {
            print_diag(
                &parsed.crate_name,
                &cache_key,
                "cache HIT",
                &workspace_root_str,
            );
        }
        return ExitCode::from(exit_code as u8);
    }

    // Cache miss — register on the scoreboard (if configured) so external
    // observers can see this build is in flight.  Initial role is `Waiting`;
    // we promote to `Building` once we hold the singleflight lock.  Drop
    // unregisters us automatically.
    let scoreboard_session = scoreboard::Tracker::from_env().map(|tracker| {
        scoreboard::Session::start(
            tracker,
            &cache_key,
            &parsed.crate_name,
            &parsed.extra_filename,
            &all_args,
            scoreboard::Role::Waiting,
        )
    });

    // Cache miss — use singleflight to avoid redundant concurrent builds.
    {
        if let Some(mut flock) = lock::FileLock::open(&cache_lock_path(&cache_key)) {
            if !flock.try_exclusive() {
                // Another process is compiling the same key — wait for it.
                if debug {
                    eprintln!(
                        "sccache-wrapper: waiting for concurrent build of {} ({})",
                        parsed.crate_name,
                        &cache_key[..12]
                    );
                }
                if !flock.exclusive() && debug {
                    // Lock acquisition itself failed — log and proceed
                    // unsynchronised rather than wedge.  The cache_store call
                    // below remains race-safe because the rename is atomic.
                    eprintln!(
                        "sccache-wrapper: failed to acquire singleflight lock for {} ({}); proceeding unsynchronised",
                        parsed.crate_name,
                        &cache_key[..12]
                    );
                }

                // Leader finished — try the cache again.
                if let Some(exit_code) = cache_restore(&cache_key, &parsed, &workspace_root_str) {
                    if debug {
                        print_diag(
                            &parsed.crate_name,
                            &cache_key,
                            "cache HIT (singleflight)",
                            &workspace_root_str,
                        );
                    }
                    return ExitCode::from(exit_code as u8);
                }
                // Leader must have failed — fall through and compile ourselves.
            }

            // We hold the lock (either acquired immediately or as fallback).
            if let Some(s) = scoreboard_session.as_ref() {
                s.promote_to_builder();
            }
            if debug {
                eprintln!(
                    "sccache-wrapper: cache MISS for {} ({})",
                    parsed.crate_name,
                    &cache_key[..12]
                );
            }
            let result = compile_and_cache(
                &cache_key,
                &parsed,
                &workspace_root_str,
                &sccache,
                &all_args,
                &emitted_args,
                &deps,
            );
            if debug {
                print_diag(
                    &parsed.crate_name,
                    &cache_key,
                    "stored",
                    &workspace_root_str,
                );
            }
            if let Some(s) = scoreboard_session.as_ref() {
                s.mark_ready();
            }
            // Lock released on drop.
            drop(flock);
            return result;
        }
    }

    // Fallback (lock open failure): compile without singleflight.
    if debug {
        eprintln!(
            "sccache-wrapper: failed to open singleflight lock at {}; compiling unsynchronised",
            cache_lock_path(&cache_key).display(),
        );
    }
    if let Some(s) = scoreboard_session.as_ref() {
        s.promote_to_builder();
    }
    if debug {
        eprintln!(
            "sccache-wrapper: cache MISS for {} ({})",
            parsed.crate_name,
            &cache_key[..12]
        );
    }
    let result = compile_and_cache(
        &cache_key,
        &parsed,
        &workspace_root_str,
        &sccache,
        &all_args,
        &emitted_args,
        &deps,
    );
    if debug {
        print_diag(
            &parsed.crate_name,
            &cache_key,
            "stored",
            &workspace_root_str,
        );
    }
    if let Some(s) = scoreboard_session.as_ref() {
        s.mark_ready();
    }
    result
}

/// Exec sccache directly, replacing this process.  Used for non-cacheable
/// invocations to avoid any overhead.
fn exec_sccache(sccache: &Path, args: &[String]) -> ExitCode {
    let mut cmd = Command::new(sccache);
    cmd.args(args.iter().map(OsString::from));
    setup_sccache_env(&mut cmd);

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let err = cmd.exec();
        if is_debug() {
            eprintln!("sccache-wrapper: exec failed: {err}");
        }
        ExitCode::from(126)
    }

    #[cfg(not(unix))]
    {
        match cmd.status() {
            Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
            Err(e) => {
                if is_debug() {
                    eprintln!("sccache-wrapper: failed to run sccache: {e}");
                }
                ExitCode::from(126)
            }
        }
    }
}

/// Kill-switch: just exec the real sccache without any cache logic.
fn passthrough_exec() -> ExitCode {
    let sccache = match find_sccache() {
        Some(p) => p,
        None => {
            if is_debug() {
                eprintln!("sccache-wrapper: could not find sccache in PATH");
            }
            return ExitCode::from(127);
        }
    };
    let all_args: Vec<String> = env::args().skip(1).collect();
    exec_sccache(&sccache, &all_args)
}

#[cfg(test)]
mod tests {
    use super::{extern_basename_key, rewrite_extra_filename, rewrite_extra_filename_in_d_content};

    #[test]
    fn extern_basename_strips_workspace_target_dir() {
        // The default cargo target dir, inside the workspace.
        assert_eq!(
            extern_basename_key("serde=/Users/me/repo/target/debug/deps/libserde-abc123.rmeta"),
            "serde=libserde-abc123.rmeta",
        );
    }

    #[test]
    fn extern_basename_strips_per_session_target_dir() {
        // A per-session CARGO_TARGET_DIR — different prefix from the default,
        // but same dep version → must produce the same key.
        let a = extern_basename_key(
            "serde=/Users/me/repo/.agents-workspace/tmp/target-abc/debug/deps/libserde-abc123.rmeta",
        );
        let b = extern_basename_key(
            "serde=/Users/me/repo/.agents-workspace/tmp/target-xyz/debug/deps/libserde-abc123.rmeta",
        );
        assert_eq!(a, b);
        assert_eq!(a, "serde=libserde-abc123.rmeta");
    }

    #[test]
    fn extern_basename_strips_target_dir_outside_workspace() {
        // CARGO_TARGET_DIR set to a path entirely outside the workspace —
        // pre-fix, normalise_arg couldn't help here because the workspace
        // root substitution doesn't apply.
        let inside =
            extern_basename_key("serde=/Users/me/repo/target/debug/deps/libserde-abc123.rmeta");
        let outside = extern_basename_key("serde=/tmp/target-tmp/debug/deps/libserde-abc123.rmeta");
        assert_eq!(inside, outside);
    }

    #[test]
    fn extern_basename_preserves_proc_macro_dylib() {
        // Proc-macros surface as .dylib/.so/.dll — must round-trip too.
        assert_eq!(
            extern_basename_key("serde_derive=/path/to/deps/libserde_derive-def456.dylib"),
            "serde_derive=libserde_derive-def456.dylib",
        );
    }

    #[test]
    fn extern_basename_falls_back_when_not_name_equals_path() {
        // Defensive: cargo always uses name=path, but if something else
        // shows up, return it verbatim rather than producing garbage.
        assert_eq!(extern_basename_key("noequals"), "noequals");
    }

    #[test]
    fn rewrites_rlib_suffix() {
        assert_eq!(
            rewrite_extra_filename("libfoo-abc123.rlib", "-abc123", "-def456"),
            "libfoo-def456.rlib",
        );
    }

    #[test]
    fn rewrites_rmeta_suffix() {
        assert_eq!(
            rewrite_extra_filename("libfoo-abc123.rmeta", "-abc123", "-def456"),
            "libfoo-def456.rmeta",
        );
    }

    #[test]
    fn rewrites_dep_info_suffix() {
        assert_eq!(
            rewrite_extra_filename("foo-abc123.d", "-abc123", "-def456"),
            "foo-def456.d",
        );
    }

    #[test]
    fn rewrites_unix_test_binary_with_no_extension() {
        assert_eq!(
            rewrite_extra_filename("foo-abc123", "-abc123", "-def456"),
            "foo-def456",
        );
    }

    #[test]
    fn rewrites_windows_test_binary_with_exe_extension() {
        assert_eq!(
            rewrite_extra_filename("foo-abc123.exe", "-abc123", "-def456"),
            "foo-def456.exe",
        );
    }

    #[test]
    fn does_not_corrupt_when_stored_ef_appears_inside_crate_name() {
        // Regression test: an unanchored `str::replace` here would mangle
        // the crate-name component (e.g. `liborder` contains `-d` would not
        // match, but `libred-d` matches `-d` twice).  Anchored stem-suffix
        // replacement must only touch the trailing extra-filename.
        assert_eq!(
            rewrite_extra_filename("libred-d.rlib", "-d", "-deadbeef"),
            "libred-deadbeef.rlib",
        );
    }

    #[test]
    fn returns_input_when_suffix_does_not_match() {
        // Defensive: if the stored manifest is somehow inconsistent, leave
        // the filename alone rather than producing garbage.  The caller
        // will then notice via the existence check and report a miss.
        assert_eq!(
            rewrite_extra_filename("libfoo-abc.rlib", "-xyz", "-def"),
            "libfoo-abc.rlib",
        );
    }

    #[test]
    fn returns_input_when_stored_and_new_match() {
        assert_eq!(
            rewrite_extra_filename("libfoo-abc.rlib", "-abc", "-abc"),
            "libfoo-abc.rlib",
        );
    }

    #[test]
    fn returns_input_when_stored_ef_empty() {
        // An empty `stored_ef` cannot be anchored to anything; refuse to
        // rewrite rather than splatting `new_ef` between every char (which
        // is what `String::replace("", _)` would do).
        assert_eq!(
            rewrite_extra_filename("libfoo.rlib", "", "-def"),
            "libfoo.rlib",
        );
    }

    #[test]
    fn d_content_rewrites_anchored_to_crate_name() {
        let content = "/out/libfoo-abc.rlib /out/foo-abc.d: src/lib.rs\n";
        assert_eq!(
            rewrite_extra_filename_in_d_content(content, "foo", "-abc", "-deadbeef"),
            "/out/libfoo-deadbeef.rlib /out/foo-deadbeef.d: src/lib.rs\n",
        );
    }

    #[test]
    fn d_content_does_not_corrupt_coincidental_hex_in_paths() {
        // Regression: an unanchored `.replace("-abc", "@@EXTFN@@")` here
        // would also rewrite the unrelated `-abc` inside the source path,
        // producing a corrupted dep-info file.
        let content = "/out/libfoo-abc.rlib: /home/u/proj-abc/src/lib.rs\n";
        assert_eq!(
            rewrite_extra_filename_in_d_content(content, "foo", "-abc", "@@EXTFN@@"),
            "/out/libfoo@@EXTFN@@.rlib: /home/u/proj-abc/src/lib.rs\n",
        );
    }

    #[test]
    fn d_content_returns_input_when_stored_ef_empty() {
        let content = "/out/libfoo.rlib: src/lib.rs\n";
        assert_eq!(
            rewrite_extra_filename_in_d_content(content, "foo", "", "-def"),
            content,
        );
    }

    #[test]
    fn d_content_returns_input_when_stored_and_new_match() {
        let content = "/out/libfoo-abc.rlib: src/lib.rs\n";
        assert_eq!(
            rewrite_extra_filename_in_d_content(content, "foo", "-abc", "-abc"),
            content,
        );
    }

    #[test]
    fn d_content_returns_input_when_crate_name_empty() {
        let content = "/out/libfoo-abc.rlib: src/lib.rs\n";
        assert_eq!(
            rewrite_extra_filename_in_d_content(content, "", "-abc", "-def"),
            content,
        );
    }
}
