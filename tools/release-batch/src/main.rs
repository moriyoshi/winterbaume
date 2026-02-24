//! Chunked cargo-release driver for the crates.io publish_new rate limit.
//!
//! Cargo rejects a multi-crate batch upfront when the planned new-crate count
//! exceeds the account's publish_new quota (default: 5 per 10 minutes). This
//! tool splits a workspace release into successive cargo-release invocations
//! of at most --chunk-size crates each, sleeping --sleep seconds between
//! invocations so the rate window slides.

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::thread;
use std::time::Duration;

use clap::Parser;
use serde::Deserialize;

/// Environment variable that overrides the cargo executable used by the driver.
/// Falls back to `--cargo`, then to `cargo` on PATH.
const CARGO_ENV: &str = "WB_CARGO";

#[derive(Parser, Debug)]
#[command(
    about = "Chunked cargo-release driver for the crates.io publish_new rate limit",
    long_about = "Splits a workspace release into successive cargo-release invocations of \
                  at most --chunk-size crates each, sleeping --sleep seconds between \
                  invocations so the rate window slides. Without --execute, prints the \
                  planned chunks as a dry run."
)]
struct Args {
    /// Version (e.g. 0.1.0) or level (patch/minor/major/release/alpha/beta/rc) passed
    /// to cargo-release as its first positional argument. Required when --execute is set;
    /// optional for plan-only runs (the topological chunking is independent of the value).
    /// The crates.io resumability check runs only when this is a concrete semver.
    #[arg(long)]
    version: Option<String>,

    /// Maximum new crates per cargo-release invocation.
    #[arg(long, default_value_t = 5)]
    chunk_size: usize,

    /// Seconds to sleep between chunks (default: 600s window + 60s buffer).
    #[arg(long, default_value_t = 660)]
    sleep: u64,

    /// Actually run `cargo release --execute`. Without this flag, prints the plan only.
    #[arg(long)]
    execute: bool,

    /// Pass --sign to cargo release for GPG-signed commits and tags.
    #[arg(long)]
    sign: bool,

    /// Pass --no-confirm to cargo release for unattended runs.
    #[arg(long)]
    no_confirm: bool,

    /// Do not query crates.io to skip already-published versions.
    #[arg(long)]
    skip_version_check: bool,

    /// Cargo executable to invoke for `metadata`, `locate-project`, and `release`.
    /// Defaults to the WB_CARGO environment variable, then `cargo` on PATH.
    /// Useful for forcing the project's wrapper script (e.g. `.agents/bin/cargo.sh`).
    #[arg(long)]
    cargo: Option<OsString>,
}

/// Where the resolved cargo executable came from, so the plan output can
/// document it in the form most useful to the operator.
enum CargoSource {
    /// `--cargo <path>` on the CLI.
    Cli,
    /// `WB_CARGO` environment variable.
    Env,
    /// Default `cargo` on PATH.
    Default,
}

struct CargoExe {
    path: OsString,
    source: CargoSource,
}

fn cargo_exe(args: &Args) -> CargoExe {
    if let Some(c) = &args.cargo {
        return CargoExe {
            path: c.clone(),
            source: CargoSource::Cli,
        };
    }
    if let Some(c) = env::var_os(CARGO_ENV) {
        return CargoExe {
            path: c,
            source: CargoSource::Env,
        };
    }
    CargoExe {
        path: OsString::from("cargo"),
        source: CargoSource::Default,
    }
}

#[derive(Deserialize)]
struct Metadata {
    workspace_members: Vec<String>,
    packages: Vec<Package>,
    resolve: Resolve,
}

#[derive(Deserialize)]
struct Package {
    id: String,
    name: String,
    /// `null` (publishable to all registries), `[]` (publish=false), or a list of registries.
    publish: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Resolve {
    nodes: Vec<Node>,
}

#[derive(Deserialize)]
struct Node {
    id: String,
    deps: Vec<NodeDep>,
}

#[derive(Deserialize)]
struct NodeDep {
    pkg: String,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("cargo {cmd:?} failed with status {status}: {stderr}")]
    CargoFailed {
        cmd: &'static str,
        status: String,
        stderr: String,
    },
    #[error("cycle in workspace topological sort: {0:?}")]
    Cycle(Vec<String>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

fn workspace_root(cargo: &CargoExe) -> Result<PathBuf, Error> {
    let out = Command::new(&cargo.path)
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()?;
    if !out.status.success() {
        return Err(Error::CargoFailed {
            cmd: "locate-project",
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    let mut path = PathBuf::from(String::from_utf8(out.stdout)?.trim());
    path.pop();
    Ok(path)
}

fn cargo_metadata(cargo: &CargoExe, root: &Path) -> Result<Metadata, Error> {
    let out = Command::new(&cargo.path)
        .args(["metadata", "--format-version", "1", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .output()?;
    if !out.status.success() {
        return Err(Error::CargoFailed {
            cmd: "metadata",
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(serde_json::from_slice(&out.stdout)?)
}

fn publishable_members(meta: &Metadata) -> Vec<&Package> {
    let members: BTreeSet<&str> = meta.workspace_members.iter().map(|s| s.as_str()).collect();
    meta.packages
        .iter()
        .filter(|p| {
            members.contains(p.id.as_str())
                && !matches!(p.publish.as_ref(), Some(v) if v.is_empty())
        })
        .collect()
}

fn topo_sort(meta: &Metadata, members: &[&Package]) -> Result<Vec<String>, Error> {
    let member_ids: BTreeSet<&str> = members.iter().map(|p| p.id.as_str()).collect();
    let id_to_name: HashMap<&str, &str> = meta
        .packages
        .iter()
        .map(|p| (p.id.as_str(), p.name.as_str()))
        .collect();

    let mut deps: HashMap<String, BTreeSet<String>> = members
        .iter()
        .map(|p| (p.name.clone(), BTreeSet::new()))
        .collect();

    for node in &meta.resolve.nodes {
        if !member_ids.contains(node.id.as_str()) {
            continue;
        }
        let src = id_to_name[node.id.as_str()];
        for d in &node.deps {
            if member_ids.contains(d.pkg.as_str()) {
                let dep_name = id_to_name[d.pkg.as_str()];
                if dep_name == src {
                    continue;
                }
                deps.get_mut(src).unwrap().insert(dep_name.to_string());
            }
        }
    }

    let mut indeg: HashMap<String, usize> =
        deps.iter().map(|(n, d)| (n.clone(), d.len())).collect();
    let mut rev: HashMap<String, BTreeSet<String>> = HashMap::new();
    for (n, ds) in &deps {
        for d in ds {
            rev.entry(d.clone()).or_default().insert(n.clone());
        }
    }

    let mut queue: VecDeque<String> = {
        let mut zero: Vec<String> = indeg
            .iter()
            .filter(|&(_, &d)| d == 0)
            .map(|(n, _)| n.clone())
            .collect();
        zero.sort();
        zero.into()
    };

    let mut order = Vec::with_capacity(deps.len());
    while let Some(n) = queue.pop_front() {
        if let Some(ms) = rev.get(&n) {
            for m in ms {
                let entry = indeg.get_mut(m).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    queue.push_back(m.clone());
                }
            }
        }
        order.push(n);
    }

    if order.len() != deps.len() {
        let missing: Vec<String> = deps
            .keys()
            .filter(|n| !order.contains(n))
            .cloned()
            .collect();
        return Err(Error::Cycle(missing));
    }
    Ok(order)
}

fn looks_like_semver(s: &str) -> bool {
    let mut parts = s.splitn(3, '.');
    let major = parts.next().and_then(|p| p.parse::<u32>().ok());
    let minor = parts.next().and_then(|p| p.parse::<u32>().ok());
    let patch_field = parts.next().map(|p| {
        let cut = p.find(['-', '+']).unwrap_or(p.len());
        &p[..cut]
    });
    let patch = patch_field.and_then(|p| p.parse::<u32>().ok());
    matches!((major, minor, patch), (Some(_), Some(_), Some(_)))
}

fn already_on_crates_io(name: &str, version: &str) -> bool {
    let url = format!("https://crates.io/api/v1/crates/{name}/{version}");
    match ureq::get(&url)
        .set("User-Agent", "winterbaume-release-batch/0.1")
        .timeout(Duration::from_secs(15))
        .call()
    {
        Ok(resp) => resp.status() == 200,
        Err(ureq::Error::Status(404, _)) => false,
        Err(e) => {
            eprintln!("warn: crates.io check for {name} v{version}: {e}; will attempt publish");
            false
        }
    }
}

fn run() -> Result<ExitCode, Error> {
    let args = Args::parse();

    if args.execute && args.version.is_none() {
        eprintln!("error: --version is required when --execute is set");
        return Ok(ExitCode::FAILURE);
    }

    let cargo = cargo_exe(&args);
    let root = workspace_root(&cargo)?;
    let meta = cargo_metadata(&cargo, &root)?;
    let members = publishable_members(&meta);
    let order = topo_sort(&meta, &members)?;

    eprintln!("workspace root: {}", root.display());
    let cargo_source_label = match cargo.source {
        CargoSource::Cli => "from --cargo flag",
        CargoSource::Env => concat!("from ", "WB_CARGO"),
        CargoSource::Default => "default on PATH",
    };
    eprintln!(
        "cargo executable: {} ({cargo_source_label})",
        cargo.path.to_string_lossy()
    );
    eprintln!("publishable crates: {}", order.len());

    // Resumability is only meaningful when --version is a concrete semver.
    let resumable_version: Option<&str> = args.version.as_deref().filter(|v| looks_like_semver(v));

    let plan: Vec<String> = match resumable_version {
        Some(v) if !args.skip_version_check => {
            let mut keep = Vec::with_capacity(order.len());
            for name in order {
                if already_on_crates_io(&name, v) {
                    eprintln!("skip {name} v{v}: already on crates.io");
                } else {
                    keep.push(name);
                }
            }
            keep
        }
        _ => {
            if !args.skip_version_check && args.version.is_some() {
                eprintln!(
                    "note: --version is a level, not a concrete semver; skipping crates.io resumability check"
                );
            }
            order
        }
    };

    let bump_label = args.version.as_deref().unwrap_or("<version|level>");
    eprintln!("to publish ({bump_label}): {}", plan.len());
    if plan.is_empty() {
        eprintln!("nothing to do");
        return Ok(ExitCode::SUCCESS);
    }

    let chunks: Vec<&[String]> = plan.chunks(args.chunk_size).collect();
    eprintln!(
        "chunks: {} of up to {} crates each",
        chunks.len(),
        args.chunk_size
    );

    for (i, chunk) in chunks.iter().enumerate() {
        let i1 = i + 1;
        eprintln!(
            "\n=== chunk {i1}/{n} ({k} crate(s)) ===",
            n = chunks.len(),
            k = chunk.len()
        );
        eprintln!("$ cargo release {bump_label} {}", chunk.join(" "));

        if !args.execute {
            continue;
        }

        let mut cmd = Command::new(&cargo.path);
        cmd.arg("release").arg(args.version.as_ref().unwrap());
        for c in *chunk {
            cmd.arg("-p").arg(c);
        }
        if args.sign {
            cmd.arg("--sign");
        }
        if args.no_confirm {
            cmd.arg("--no-confirm");
        }
        cmd.arg("--execute");
        cmd.current_dir(&root);

        let status = cmd.status()?;
        if !status.success() {
            eprintln!(
                "chunk {i1} failed (rc={:?}); fix the cause and re-run",
                status.code()
            );
            return Ok(ExitCode::from(status.code().unwrap_or(1) as u8));
        }

        if i1 < chunks.len() {
            eprintln!("sleeping {}s before next chunk", args.sleep);
            thread::sleep(Duration::from_secs(args.sleep));
        }
    }

    eprintln!("\nall chunks complete");
    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
