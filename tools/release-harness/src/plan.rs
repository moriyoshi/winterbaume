//! Stage 1 — discovery + per-crate semver bump classification.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde::{Deserialize, Serialize};

use crate::PlanArgs;
use crate::git;
use crate::metadata::{CargoExe, Package, cargo_metadata, publishable_members, workspace_root};
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

    let plan = Plan {
        meta: Meta {
            generated_at: chrono::Utc::now().to_rfc3339(),
            head: head_short,
            crates_changed,
            crates_skipped_doc_only,
            crates_unchanged,
            crates_initial,
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
