//! Workspace discovery via `cargo metadata` and `cargo locate-project`.
//!
//! Returns both the publishable-member list (for the harness's plan stage) and
//! the resolved dependency graph (for the batch stage's topology sort).

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

const CARGO_ENV: &str = "WB_CARGO";

#[derive(thiserror::Error, Debug)]
pub enum Error {
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

#[derive(Clone, Debug)]
pub struct CargoExe {
    path: OsString,
}

impl CargoExe {
    pub fn resolve(cli: Option<&OsString>) -> Self {
        if let Some(c) = cli {
            return Self { path: c.clone() };
        }
        if let Some(c) = env::var_os(CARGO_ENV) {
            return Self { path: c };
        }
        Self {
            path: OsString::from("cargo"),
        }
    }

    pub fn path(&self) -> &OsString {
        &self.path
    }
}

#[derive(Deserialize)]
pub struct Metadata {
    pub workspace_members: Vec<String>,
    pub packages: Vec<Package>,
    pub resolve: Resolve,
}

#[derive(Deserialize)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
    /// `null` (publishable to all registries), `[]` (publish=false), or a list of registries.
    pub publish: Option<Vec<String>>,
    /// Direct dependencies declared in this package's Cargo.toml. Unlike the
    /// resolved graph, this includes *optional* deps that aren't activated by
    /// the current feature set — important for crates like the umbrella that
    /// declare every service crate as an optional dep.
    #[serde(default)]
    pub dependencies: Vec<Dep>,
}

#[derive(Deserialize)]
pub struct Dep {
    /// The dependency's *package* name. For renamed deps, `rename` is the
    /// local alias; this field always points at the upstream package.
    pub name: String,
}

impl Package {
    pub fn is_publishable(&self) -> bool {
        !matches!(self.publish.as_ref(), Some(v) if v.is_empty())
    }

    /// Directory containing this package's Cargo.toml.
    pub fn dir(&self) -> &Path {
        self.manifest_path
            .parent()
            .expect("manifest_path always has a parent")
    }
}

#[derive(Deserialize)]
pub struct Resolve {
    pub nodes: Vec<Node>,
}

#[derive(Deserialize)]
pub struct Node {
    pub id: String,
    pub deps: Vec<NodeDep>,
}

#[derive(Deserialize)]
pub struct NodeDep {
    pub pkg: String,
}

pub fn workspace_root(cargo: &CargoExe) -> Result<PathBuf, Error> {
    let out = Command::new(cargo.path())
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

pub fn cargo_metadata(cargo: &CargoExe, root: &Path) -> Result<Metadata, Error> {
    let out = Command::new(cargo.path())
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

/// Members in the workspace that are publishable to crates.io.
pub fn publishable_members(meta: &Metadata) -> Vec<&Package> {
    let members: BTreeSet<&str> = meta.workspace_members.iter().map(|s| s.as_str()).collect();
    meta.packages
        .iter()
        .filter(|p| members.contains(p.id.as_str()) && p.is_publishable())
        .collect()
}

/// Kahn-style topological sort over the supplied member subset, so dependants
/// are published after their workspace dependencies. Edges outside the subset
/// are dropped — the caller is responsible for ordering any prerequisite
/// publish that lives outside the subset.
pub fn topo_sort(meta: &Metadata, members: &[&Package]) -> Result<Vec<String>, Error> {
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
