//! Stage 1 — discovery + per-crate semver bump classification.
//!
//! After classifying each crate from its own file diff, the plan runs a
//! reverse-dep propagation pass: any workspace member that depends
//! (directly or transitively) on a crate that is going to bump gets upgraded
//! from `Unchanged` / auto-`Skip` (cosmetic-only) to `Patch`, so its
//! published manifest picks up the new dep version specifier. Crates with an
//! explicit `Skip` override are left alone but reported, and the umbrella
//! workspace-root crate is never auto-upgraded — the operator must pin it.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde::{Deserialize, Serialize};

use crate::PlanArgs;
use crate::git;
use crate::metadata::{
    CargoExe, Metadata, Package, cargo_metadata, publishable_members, workspace_root,
};
use crate::semver_checks;
use crate::version::{Level, Version};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Metadata(#[from] crate::metadata::Error),
    #[error(transparent)]
    Git(#[from] git::Error),
    #[error(transparent)]
    Semver(#[from] semver_checks::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("cannot parse current version {version:?} of {package}: {reason}")]
    BadVersion {
        package: String,
        version: String,
        reason: String,
    },
    #[error("cannot parse overrides file {path}: {source}")]
    BadOverrides {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("cannot serialize plan to TOML: {0}")]
    Serialize(#[from] toml::ser::Error),
    #[error("invalid override for {package}: {reason}")]
    InvalidOverride { package: String, reason: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan {
    pub meta: Meta,
    #[serde(default, rename = "crates")]
    pub crates: Vec<CrateEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub generated_at: String,
    pub head: String,
    pub crates_changed: usize,
    pub crates_skipped_doc_only: usize,
    pub crates_unchanged: usize,
    pub crates_initial: usize,
    /// Crates upgraded to `Patch` by the reverse-dep propagation pass because
    /// they depend (directly or transitively) on a bumping crate. Always a
    /// subset of `crates_changed`.
    #[serde(default)]
    pub crates_transitively_bumped: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrateEntry {
    pub name: String,
    pub current: String,
    /// The version that will land on crates.io. Absent for `skip`/`unchanged`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    pub bump: BumpDecision,
    /// The git tag the diff was taken against. Absent on first release.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_tag: Option<String>,
    pub reason: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files_changed: Vec<String>,
}

/// Outcome the harness records for a single crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BumpDecision {
    /// Default state for crates that haven't changed since their last tag.
    Unchanged,
    /// Only README / CHANGELOG / cosmetic files changed.
    Skip,
    Patch,
    Minor,
    Major,
    /// Explicit literal version override from release-plan-overrides.toml.
    /// Stored as a string in the plan; the harness pins this for the crate.
    Pinned,
    /// No prior tag — publish at the current Cargo.toml version, no bump.
    Initial,
}

impl BumpDecision {
    pub fn level(self) -> Option<Level> {
        match self {
            BumpDecision::Patch => Some(Level::Patch),
            BumpDecision::Minor => Some(Level::Minor),
            BumpDecision::Major => Some(Level::Major),
            _ => None,
        }
    }
}

/// `release-plan-overrides.toml` shape:
///
/// ```toml
/// [bumps]
/// winterbaume-ec2 = "minor"
/// winterbaume-foo = "skip"
/// winterbaume-bar = "0.3.1"
/// ```
#[derive(Deserialize, Default)]
struct Overrides {
    #[serde(default)]
    bumps: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
enum Override {
    Skip,
    Level(Level),
    Pinned(Version),
}

fn parse_overrides(path: &Path) -> Result<BTreeMap<String, Override>, Error> {
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let raw = fs::read_to_string(path)?;
    let parsed: Overrides = toml::from_str(&raw).map_err(|e| Error::BadOverrides {
        path: path.to_path_buf(),
        source: e,
    })?;
    let mut out = BTreeMap::new();
    for (name, value) in parsed.bumps {
        let decision = match value.as_str() {
            "skip" => Override::Skip,
            "patch" => Override::Level(Level::Patch),
            "minor" => Override::Level(Level::Minor),
            "major" => Override::Level(Level::Major),
            other => match other.parse::<Version>() {
                Ok(v) => Override::Pinned(v),
                Err(e) => {
                    return Err(Error::InvalidOverride {
                        package: name,
                        reason: format!("{other:?} is neither a level nor a semver: {e}"),
                    });
                }
            },
        };
        out.insert(name, decision);
    }
    Ok(out)
}

/// File-path globs that are treated as "cosmetic" — a diff confined to these
/// files alone is enough to skip the crate.
fn is_cosmetic_path(rel: &str) -> bool {
    // rel is repo-root relative, e.g. "crates/winterbaume-ec2/README.md".
    let basename = rel.rsplit('/').next().unwrap_or(rel);
    matches!(
        basename,
        "README.md" | "CHANGELOG.md" | ".gitignore" | "NOTICE" | "LICENSE"
    ) || basename.ends_with(".png")
        || basename.ends_with(".svg")
        || basename.ends_with(".jpg")
        || basename.ends_with(".jpeg")
        // crate-local docs subdirectories aren't user-facing API surface.
        || rel.contains("/docs/")
}

fn apply_override(
    pkg: &Package,
    current: Version,
    tag: Option<&(String, Version)>,
    o: &Override,
) -> CrateEntry {
    let (decision, next, reason) = match o {
        Override::Skip => (BumpDecision::Skip, None, "override: skip".to_string()),
        Override::Level(l) => {
            let next = current.bumped(*l);
            (
                match l {
                    Level::Patch => BumpDecision::Patch,
                    Level::Minor => BumpDecision::Minor,
                    Level::Major => BumpDecision::Major,
                },
                Some(next.to_string()),
                format!("override: {l}"),
            )
        }
        Override::Pinned(v) => (
            BumpDecision::Pinned,
            Some(v.to_string()),
            format!("override: pin to {v}"),
        ),
    };
    CrateEntry {
        name: pkg.name.clone(),
        current: pkg.version.clone(),
        next,
        bump: decision,
        last_tag: tag.map(|(t, _)| t.clone()),
        reason,
        files_changed: Vec::new(),
    }
}

fn classify_one(
    cargo: &CargoExe,
    root: &Path,
    pkg: &Package,
    head: &str,
    overrides: &BTreeMap<String, Override>,
    semver_tool_available: bool,
) -> Result<CrateEntry, Error> {
    // The umbrella crate (`winterbaume`) lives at the workspace root and has
    // no single directory we can diff against — its packaged surface is
    // defined by the `include` list in its Cargo.toml and changes whenever
    // any workspace dependency version moves. Don't try to auto-classify it;
    // the operator must pin it via release-plan-overrides.toml when they
    // want to ship a new umbrella release.
    let current_v: Version = pkg.version.parse().map_err(|e: String| Error::BadVersion {
        package: pkg.name.clone(),
        version: pkg.version.clone(),
        reason: e,
    })?;

    let tag = git::latest_release_tag(root, &pkg.name)?;

    // Explicit override always wins, even for the umbrella crate.
    if let Some(o) = overrides.get(&pkg.name) {
        return Ok(apply_override(pkg, current_v, tag.as_ref(), o));
    }

    if pkg.dir() == root {
        // Umbrella crate: no single directory to diff. Skip by default;
        // operators must pin via release-plan-overrides.toml to publish.
        return Ok(CrateEntry {
            name: pkg.name.clone(),
            current: pkg.version.clone(),
            next: None,
            bump: BumpDecision::Skip,
            last_tag: tag.map(|(t, _)| t),
            reason: "umbrella crate at workspace root; pin a version in \
                     release-plan-overrides.toml to publish"
                .to_string(),
            files_changed: Vec::new(),
        });
    }

    let pathspec = format!(
        "{}/",
        pkg.dir()
            .strip_prefix(root)
            .unwrap_or_else(|_| pkg.dir())
            .display()
    );

    let Some((tag_name, _tag_v)) = tag else {
        // No prior tag → ship at the current version, treat as initial release.
        return Ok(CrateEntry {
            name: pkg.name.clone(),
            current: pkg.version.clone(),
            next: Some(pkg.version.clone()),
            bump: BumpDecision::Initial,
            last_tag: None,
            reason: "no prior <crate>-v<ver> tag; treat as initial release at current version"
                .to_string(),
            files_changed: Vec::new(),
        });
    };

    let files = git::diff_paths(root, &tag_name, head, &pathspec)?;
    if files.is_empty() {
        let reason = format!("no files changed since {tag_name}");
        return Ok(CrateEntry {
            name: pkg.name.clone(),
            current: pkg.version.clone(),
            next: None,
            bump: BumpDecision::Unchanged,
            last_tag: Some(tag_name),
            reason,
            files_changed: Vec::new(),
        });
    }

    let substantive: Vec<&String> = files.iter().filter(|p| !is_cosmetic_path(p)).collect();
    if substantive.is_empty() {
        let reason =
            format!("only cosmetic files changed since {tag_name} (README/CHANGELOG/docs)");
        return Ok(CrateEntry {
            name: pkg.name.clone(),
            current: pkg.version.clone(),
            next: None,
            bump: BumpDecision::Skip,
            last_tag: Some(tag_name),
            reason,
            files_changed: files,
        });
    }

    // Heuristic: any added public-symbol line in src/** raises candidate to minor.
    let src_pathspec = format!("{pathspec}src/");
    let new_pub_items = git::added_pub_symbols(root, &tag_name, head, &src_pathspec)?;
    let mut candidate = if new_pub_items.is_empty() {
        Level::Patch
    } else {
        Level::Minor
    };

    // Authoritative escalation via cargo-semver-checks if available.
    let mut reason_parts = vec![format!(
        "{} substantive file(s) changed since {}",
        substantive.len(),
        tag_name
    )];
    if !new_pub_items.is_empty() {
        reason_parts.push(format!(
            "+{} new public item(s) → minor candidate",
            new_pub_items.len()
        ));
    } else {
        reason_parts.push("no new public items → patch candidate".to_string());
    }

    if semver_tool_available {
        match semver_checks::check(cargo, root, &pkg.name, candidate)? {
            semver_checks::Outcome::Breaking => {
                candidate = Level::Major;
                reason_parts.push("semver-checks: breaking change → major".to_string());
            }
            semver_checks::Outcome::NoBreaking => {
                reason_parts.push("semver-checks: clean".to_string());
            }
            semver_checks::Outcome::NotApplicable => {
                reason_parts.push("semver-checks: skipped (patch candidate)".to_string());
            }
            semver_checks::Outcome::Unavailable => {
                reason_parts.push("semver-checks: no verdict; heuristic only".to_string());
            }
        }
    } else {
        reason_parts.push("semver-checks not installed; heuristic only".to_string());
    }

    let next = current_v.bumped(candidate);
    let decision = match candidate {
        Level::Patch => BumpDecision::Patch,
        Level::Minor => BumpDecision::Minor,
        Level::Major => BumpDecision::Major,
    };

    Ok(CrateEntry {
        name: pkg.name.clone(),
        current: pkg.version.clone(),
        next: Some(next.to_string()),
        bump: decision,
        last_tag: Some(tag_name),
        reason: reason_parts.join("; "),
        files_changed: files,
    })
}

/// Map from each publishable member's name to the set of publishable members
/// that depend on it (one hop). Transitive reach is achieved by iterating the
/// propagation pass to a fixed point.
fn build_reverse_deps(meta: &Metadata, members: &[&Package]) -> BTreeMap<String, BTreeSet<String>> {
    let member_ids: BTreeSet<&str> = members.iter().map(|p| p.id.as_str()).collect();
    let id_to_name: BTreeMap<&str, &str> = meta
        .packages
        .iter()
        .map(|p| (p.id.as_str(), p.name.as_str()))
        .collect();

    let mut rdeps: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for node in &meta.resolve.nodes {
        if !member_ids.contains(node.id.as_str()) {
            continue;
        }
        let Some(src) = id_to_name.get(node.id.as_str()) else {
            continue;
        };
        for d in &node.deps {
            if !member_ids.contains(d.pkg.as_str()) {
                continue;
            }
            let Some(dep) = id_to_name.get(d.pkg.as_str()) else {
                continue;
            };
            if dep == src {
                continue;
            }
            rdeps
                .entry((*dep).to_string())
                .or_default()
                .insert((*src).to_string());
        }
    }
    rdeps
}

fn is_bumping(b: BumpDecision) -> bool {
    matches!(
        b,
        BumpDecision::Patch
            | BumpDecision::Minor
            | BumpDecision::Major
            | BumpDecision::Pinned
            | BumpDecision::Initial
    )
}

#[derive(Debug, Default)]
struct PropagationResult {
    /// Names of crates whose `BumpDecision` was upgraded by this pass.
    transitively_bumped: BTreeSet<String>,
    /// Dependant → the bumping deps that would have triggered an upgrade,
    /// for crates left at `Skip` by an explicit override.
    blocked_by_override: BTreeMap<String, BTreeSet<String>>,
    /// Umbrella crate name → the bumping deps it depends on. The umbrella
    /// is never auto-upgraded; the operator must pin it explicitly.
    umbrellas_with_moving_deps: BTreeMap<String, BTreeSet<String>>,
}

/// Iterate the propagation pass to a fixed point. Returns the names that were
/// upgraded plus the cases worth surfacing to the operator (overridden skips
/// and umbrella crates with moving deps).
fn propagate_bumps(
    entries: &mut [CrateEntry],
    rdeps: &BTreeMap<String, BTreeSet<String>>,
    umbrella_names: &BTreeSet<String>,
    overrides: &BTreeMap<String, Override>,
) -> PropagationResult {
    let mut result = PropagationResult::default();
    let index: BTreeMap<String, usize> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| (e.name.clone(), i))
        .collect();

    loop {
        let bumping: BTreeSet<String> = entries
            .iter()
            .filter(|e| is_bumping(e.bump))
            .map(|e| e.name.clone())
            .collect();

        let mut to_upgrade: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for bumper in &bumping {
            if let Some(dependants) = rdeps.get(bumper) {
                for d in dependants {
                    to_upgrade
                        .entry(d.clone())
                        .or_default()
                        .insert(bumper.clone());
                }
            }
        }

        let mut changed = false;
        for (dependant, causes) in &to_upgrade {
            if umbrella_names.contains(dependant) {
                result
                    .umbrellas_with_moving_deps
                    .entry(dependant.clone())
                    .or_default()
                    .extend(causes.iter().cloned());
                continue;
            }
            let Some(&idx) = index.get(dependant) else {
                continue;
            };
            let entry = &mut entries[idx];
            if is_bumping(entry.bump) {
                continue;
            }
            // Explicit override (Skip or otherwise) reflects the operator's
            // deliberate choice; never silently override it.
            if overrides.contains_key(dependant) {
                if matches!(entry.bump, BumpDecision::Skip) {
                    result
                        .blocked_by_override
                        .entry(dependant.clone())
                        .or_default()
                        .extend(causes.iter().cloned());
                }
                continue;
            }
            // Eligible to upgrade: Unchanged, or auto-Skip (cosmetic-only).
            match entry.bump {
                BumpDecision::Unchanged | BumpDecision::Skip => {
                    let Ok(current_v) = entry.current.parse::<Version>() else {
                        continue;
                    };
                    let next = current_v.bumped(Level::Patch);
                    let prev_reason = entry.reason.clone();
                    entry.bump = BumpDecision::Patch;
                    entry.next = Some(next.to_string());
                    let mut sorted: Vec<&str> = causes.iter().map(|s| s.as_str()).collect();
                    sorted.sort_unstable();
                    entry.reason = format!(
                        "transitive patch: depends on bumping crate(s) [{}] (previously: {})",
                        sorted.join(", "),
                        prev_reason,
                    );
                    result.transitively_bumped.insert(dependant.clone());
                    changed = true;
                }
                _ => continue,
            }
        }

        if !changed {
            break;
        }
    }

    result
}

pub fn run(cargo: &CargoExe, args: &PlanArgs) -> Result<ExitCode, Error> {
    let root = workspace_root(cargo)?;
    let meta = cargo_metadata(cargo, &root)?;
    let mut members = publishable_members(&meta);
    members.sort_by(|a, b| a.name.cmp(&b.name));

    let overrides = parse_overrides(&root.join(&args.overrides))?;
    let semver_tool_available = !args.skip_semver_checks && semver_checks::available(cargo);
    if args.skip_semver_checks {
        eprintln!("note: --skip-semver-checks set; using heuristics only");
    } else if !semver_tool_available {
        eprintln!(
            "warn: cargo-semver-checks not installed on PATH; falling back to heuristics. \
             Install with `cargo install cargo-semver-checks` to detect breaking changes."
        );
    }

    eprintln!("workspace root: {}", root.display());
    eprintln!("publishable crates: {}", members.len());

    let head_short = git::rev_short(&root, &args.head).unwrap_or_else(|e| {
        eprintln!("warn: cannot resolve {} short hash: {e}", args.head);
        args.head.clone()
    });

    let mut entries = Vec::with_capacity(members.len());
    for pkg in &members {
        let entry = classify_one(
            cargo,
            &root,
            pkg,
            &args.head,
            &overrides,
            semver_tool_available,
        )?;
        entries.push(entry);
    }

    // Reverse-dep propagation: any crate that depends on a bumping crate gets
    // patched so its published manifest references the new dep version.
    let rdeps = build_reverse_deps(&meta, &members);
    let umbrella_names: BTreeSet<String> = members
        .iter()
        .filter(|p| p.dir() == root.as_path())
        .map(|p| p.name.clone())
        .collect();
    let prop = propagate_bumps(&mut entries, &rdeps, &umbrella_names, &overrides);

    if !prop.transitively_bumped.is_empty() {
        eprintln!(
            "propagated patch bump to {} dependant crate(s): {}",
            prop.transitively_bumped.len(),
            prop.transitively_bumped
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
    for (name, causes) in &prop.blocked_by_override {
        eprintln!(
            "warn: {name} has an explicit skip override but depends on bumping crate(s) \
             [{}] — its published manifest will continue to reference the previous \
             dep version(s). Remove the override if you want to propagate the bump.",
            causes.iter().cloned().collect::<Vec<_>>().join(", "),
        );
    }
    for (name, causes) in &prop.umbrellas_with_moving_deps {
        eprintln!(
            "note: umbrella crate {name} is not auto-upgraded; pin it in \
             release-plan-overrides.toml to publish a new umbrella version that \
             references the bumped dep(s) [{}].",
            causes.iter().cloned().collect::<Vec<_>>().join(", "),
        );
    }

    // Summary counters.
    let mut crates_changed = 0;
    let mut crates_skipped_doc_only = 0;
    let mut crates_unchanged = 0;
    let mut crates_initial = 0;
    for e in &entries {
        match e.bump {
            BumpDecision::Unchanged => crates_unchanged += 1,
            BumpDecision::Skip => crates_skipped_doc_only += 1,
            BumpDecision::Initial => crates_initial += 1,
            BumpDecision::Patch
            | BumpDecision::Minor
            | BumpDecision::Major
            | BumpDecision::Pinned => crates_changed += 1,
        }
    }
    let crates_transitively_bumped = prop.transitively_bumped.len();

    let plan = Plan {
        meta: Meta {
            generated_at: chrono::Utc::now().to_rfc3339(),
            head: head_short,
            crates_changed,
            crates_skipped_doc_only,
            crates_unchanged,
            crates_initial,
            crates_transitively_bumped,
        },
        crates: entries,
    };

    let out_path = if args.out.is_absolute() {
        args.out.clone()
    } else {
        root.join(&args.out)
    };
    let serialized = toml::to_string_pretty(&plan)?;
    fs::write(&out_path, serialized)?;

    // Human summary on stdout.
    println!();
    println!("=== release plan summary ===");
    println!("written to: {}", out_path.display());
    println!("crates_changed:           {crates_changed}");
    println!("  of which transitive:    {crates_transitively_bumped}");
    println!("crates_skipped_doc_only:  {crates_skipped_doc_only}");
    println!("crates_unchanged:         {crates_unchanged}");
    println!("crates_initial:           {crates_initial}");
    if crates_changed > 0 {
        println!();
        println!("by bump level:");
        for level in [
            BumpDecision::Major,
            BumpDecision::Minor,
            BumpDecision::Patch,
            BumpDecision::Pinned,
            BumpDecision::Initial,
        ] {
            let names: Vec<&str> = plan
                .crates
                .iter()
                .filter(|e| e.bump == level)
                .map(|e| e.name.as_str())
                .collect();
            if !names.is_empty() {
                let label = match level {
                    BumpDecision::Major => "major",
                    BumpDecision::Minor => "minor",
                    BumpDecision::Patch => "patch",
                    BumpDecision::Pinned => "pinned",
                    BumpDecision::Initial => "initial",
                    _ => continue,
                };
                println!("  {label:<8} ({}): {}", names.len(), names.join(" "));
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(name: &str, current: &str, bump: BumpDecision, reason: &str) -> CrateEntry {
        let next = match bump {
            BumpDecision::Patch | BumpDecision::Minor | BumpDecision::Major => {
                let v: Version = current.parse().unwrap();
                let lvl = match bump {
                    BumpDecision::Patch => Level::Patch,
                    BumpDecision::Minor => Level::Minor,
                    BumpDecision::Major => Level::Major,
                    _ => unreachable!(),
                };
                Some(v.bumped(lvl).to_string())
            }
            BumpDecision::Initial => Some(current.to_string()),
            _ => None,
        };
        CrateEntry {
            name: name.to_string(),
            current: current.to_string(),
            next,
            bump,
            last_tag: None,
            reason: reason.to_string(),
            files_changed: Vec::new(),
        }
    }

    fn rdeps_from(pairs: &[(&str, &[&str])]) -> BTreeMap<String, BTreeSet<String>> {
        pairs
            .iter()
            .map(|(dep, dependants)| {
                (
                    dep.to_string(),
                    dependants.iter().map(|s| s.to_string()).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn propagates_direct_dependant_unchanged_to_patch() {
        let mut entries = vec![
            entry("foo", "0.2.0", BumpDecision::Patch, "files changed"),
            entry("bar", "0.5.0", BumpDecision::Unchanged, "no files changed"),
        ];
        let rdeps = rdeps_from(&[("foo", &["bar"])]);
        let res = propagate_bumps(&mut entries, &rdeps, &BTreeSet::new(), &BTreeMap::new());
        assert!(res.transitively_bumped.contains("bar"));
        let bar = entries.iter().find(|e| e.name == "bar").unwrap();
        assert_eq!(bar.bump, BumpDecision::Patch);
        assert_eq!(bar.next.as_deref(), Some("0.5.1"));
        assert!(bar.reason.starts_with("transitive patch"));
    }

    #[test]
    fn propagates_transitively_through_chain() {
        // foo (patch) <- bar <- baz; both bar and baz should become Patch.
        let mut entries = vec![
            entry("foo", "0.2.0", BumpDecision::Patch, "files changed"),
            entry("bar", "0.3.0", BumpDecision::Unchanged, "no files"),
            entry("baz", "0.4.0", BumpDecision::Unchanged, "no files"),
        ];
        let rdeps = rdeps_from(&[("foo", &["bar"]), ("bar", &["baz"])]);
        let res = propagate_bumps(&mut entries, &rdeps, &BTreeSet::new(), &BTreeMap::new());
        assert_eq!(res.transitively_bumped.len(), 2);
        assert!(res.transitively_bumped.contains("bar"));
        assert!(res.transitively_bumped.contains("baz"));
    }

    #[test]
    fn does_not_clobber_already_bumping_dependant() {
        let mut entries = vec![
            entry("foo", "0.2.0", BumpDecision::Patch, "files changed"),
            entry("bar", "0.5.0", BumpDecision::Minor, "files changed"),
        ];
        let rdeps = rdeps_from(&[("foo", &["bar"])]);
        propagate_bumps(&mut entries, &rdeps, &BTreeSet::new(), &BTreeMap::new());
        let bar = entries.iter().find(|e| e.name == "bar").unwrap();
        assert_eq!(bar.bump, BumpDecision::Minor);
        assert_eq!(bar.next.as_deref(), Some("0.6.0"));
    }

    #[test]
    fn override_skip_blocks_propagation_and_is_reported() {
        let mut entries = vec![
            entry("foo", "0.2.0", BumpDecision::Patch, "files changed"),
            entry("bar", "0.5.0", BumpDecision::Skip, "override: skip"),
        ];
        let rdeps = rdeps_from(&[("foo", &["bar"])]);
        let mut overrides = BTreeMap::new();
        overrides.insert("bar".to_string(), Override::Skip);
        let res = propagate_bumps(&mut entries, &rdeps, &BTreeSet::new(), &overrides);
        assert!(res.transitively_bumped.is_empty());
        assert!(res.blocked_by_override.contains_key("bar"));
        let bar = entries.iter().find(|e| e.name == "bar").unwrap();
        assert_eq!(bar.bump, BumpDecision::Skip);
    }

    #[test]
    fn auto_skip_cosmetic_gets_upgraded() {
        // Cosmetic-only changes left `bar` as Skip without an override entry;
        // a bumping dep should still pull it in.
        let mut entries = vec![
            entry("foo", "0.2.0", BumpDecision::Patch, "files changed"),
            entry(
                "bar",
                "0.5.0",
                BumpDecision::Skip,
                "only cosmetic files changed",
            ),
        ];
        let rdeps = rdeps_from(&[("foo", &["bar"])]);
        let res = propagate_bumps(&mut entries, &rdeps, &BTreeSet::new(), &BTreeMap::new());
        assert!(res.transitively_bumped.contains("bar"));
        let bar = entries.iter().find(|e| e.name == "bar").unwrap();
        assert_eq!(bar.bump, BumpDecision::Patch);
    }

    #[test]
    fn umbrella_is_never_auto_upgraded_and_is_reported() {
        let mut entries = vec![
            entry("foo", "0.2.0", BumpDecision::Patch, "files changed"),
            entry("umbrella", "1.0.0", BumpDecision::Skip, "umbrella crate"),
        ];
        let rdeps = rdeps_from(&[("foo", &["umbrella"])]);
        let umbrellas: BTreeSet<String> = ["umbrella".to_string()].into_iter().collect();
        let res = propagate_bumps(&mut entries, &rdeps, &umbrellas, &BTreeMap::new());
        assert!(res.transitively_bumped.is_empty());
        assert!(res.umbrellas_with_moving_deps.contains_key("umbrella"));
        let u = entries.iter().find(|e| e.name == "umbrella").unwrap();
        assert_eq!(u.bump, BumpDecision::Skip);
    }

    #[test]
    fn pinned_and_initial_count_as_bumping() {
        // Pinned propagates to dependants.
        let mut entries = vec![
            entry("foo", "0.2.0", BumpDecision::Pinned, "pinned"),
            entry("bar", "0.5.0", BumpDecision::Unchanged, "no files"),
        ];
        let rdeps = rdeps_from(&[("foo", &["bar"])]);
        let res = propagate_bumps(&mut entries, &rdeps, &BTreeSet::new(), &BTreeMap::new());
        assert!(res.transitively_bumped.contains("bar"));

        // Initial propagates too.
        let mut entries = vec![
            entry("foo", "0.1.0", BumpDecision::Initial, "first release"),
            entry("bar", "0.5.0", BumpDecision::Unchanged, "no files"),
        ];
        let rdeps = rdeps_from(&[("foo", &["bar"])]);
        let res = propagate_bumps(&mut entries, &rdeps, &BTreeSet::new(), &BTreeMap::new());
        assert!(res.transitively_bumped.contains("bar"));
    }

    #[test]
    fn no_propagation_when_no_one_is_bumping() {
        let mut entries = vec![
            entry("foo", "0.2.0", BumpDecision::Unchanged, "no files"),
            entry("bar", "0.5.0", BumpDecision::Unchanged, "no files"),
        ];
        let rdeps = rdeps_from(&[("foo", &["bar"])]);
        let res = propagate_bumps(&mut entries, &rdeps, &BTreeSet::new(), &BTreeMap::new());
        assert!(res.transitively_bumped.is_empty());
        for e in &entries {
            assert_eq!(e.bump, BumpDecision::Unchanged);
        }
    }

    #[test]
    fn reason_preserves_previous_context() {
        let mut entries = vec![
            entry("foo", "0.2.0", BumpDecision::Patch, "files changed"),
            entry(
                "bar",
                "0.5.0",
                BumpDecision::Skip,
                "only cosmetic files changed since bar-v0.4.0",
            ),
        ];
        let rdeps = rdeps_from(&[("foo", &["bar"])]);
        propagate_bumps(&mut entries, &rdeps, &BTreeSet::new(), &BTreeMap::new());
        let bar = entries.iter().find(|e| e.name == "bar").unwrap();
        assert!(bar.reason.contains("transitive patch"));
        assert!(
            bar.reason
                .contains("only cosmetic files changed since bar-v0.4.0")
        );
    }
}
