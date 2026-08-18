//! Stage 2.5 — apply the plan's `next` versions to the workspace manifests.
//!
//! A thin driver over `cargo release version`, which already rewrites a
//! crate's `[package] version`, follows the workspace-inheritance chain into
//! the root `[workspace.dependencies]` table, preserves manifest formatting
//! and comments, leaves a path-only dep entry alone, and refreshes
//! `Cargo.lock`. This module only decides *which* crates move to *which*
//! version, and it gets that from the plan.
//!
//! Sits between `changelog` and `publish`, and is optional: `publish` drives
//! the same `version` step itself ( see `batch::run_chunk` ). Run this when the
//! bump should land as its own reviewable commit ahead of the publish.
//!
//! Two properties the plan buys us over calling cargo-release by hand:
//!
//! - **Idempotency.** We dispatch each crate's literal `next`, never a level.
//!   `cargo release version patch` re-run bumps a second time; the same
//!   literal re-run is a no-op. Crates already sitting at `next` are dropped
//!   from the dispatch entirely.
//! - **Pre-flight drift detection.** A manifest matching neither `current` nor
//!   `next` means the plan no longer describes this tree. We report every such
//!   crate and invoke nothing. cargo-release would catch only the downgrade
//!   direction, and only once it had already written the earlier groups.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::VersionBumpArgs;
use crate::batch;
use crate::metadata::{CargoExe, cargo_metadata, publishable_members, workspace_root};
use crate::plan::{BumpDecision, CrateEntry, Plan};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Metadata(#[from] crate::metadata::Error),
    #[error(transparent)]
    Batch(#[from] batch::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("cannot read plan {path}: {source}")]
    ReadPlan {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("crate {name} pinned but next version missing from plan entry")]
    PinnedWithoutVersion { name: String },
    #[error("plan lists {name}, which is not a publishable workspace member")]
    UnknownCrate { name: String },
    #[error(
        "{count} crate(s) have drifted from the plan; re-run `release-harness plan` before bumping"
    )]
    Drifted { count: usize },
    #[error("cargo-release is not installed; install it with `cargo install cargo-release`")]
    ReleaseUnavailable,
}

/// What the plan asks us to do with one crate, given what its manifest
/// currently says.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Disposition {
    /// `skip` / `unchanged` — the plan has no target version for this crate.
    NoTarget,
    /// The manifest already sits at the planned version.
    AlreadyApplied,
    /// The manifest matches neither `current` nor `next`: the plan is stale.
    Drift,
    /// Dispatch this crate to `cargo release version <target>`.
    Bump(String),
}

fn disposition(entry: &CrateEntry, manifest_version: &str) -> Result<Disposition, Error> {
    let target = match entry.bump {
        BumpDecision::Skip | BumpDecision::Unchanged => return Ok(Disposition::NoTarget),
        BumpDecision::Pinned => entry.next.as_deref().ok_or_else(|| {
            // Mirrors `publish`: a pinned entry without a version is a
            // malformed plan, not a no-op.
            Error::PinnedWithoutVersion {
                name: entry.name.clone(),
            }
        })?,
        // `initial` carries next == current, so it lands in AlreadyApplied.
        _ => match entry.next.as_deref() {
            Some(v) => v,
            None => return Ok(Disposition::NoTarget),
        },
    };
    if manifest_version == target {
        return Ok(Disposition::AlreadyApplied);
    }
    if manifest_version != entry.current {
        return Ok(Disposition::Drift);
    }
    Ok(Disposition::Bump(target.to_string()))
}

/// One crate's planned move.
struct Change {
    name: String,
    from: String,
    to: String,
    bump: BumpDecision,
}

/// `cargo release version` takes one version for every package it is given,
/// so each distinct target version becomes one invocation.
fn group_by_version(changes: &[Change]) -> BTreeMap<&str, Vec<&str>> {
    let mut groups: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for c in changes {
        groups.entry(&c.to).or_default().push(&c.name);
    }
    for names in groups.values_mut() {
        names.sort_unstable();
    }
    groups
}

pub fn run(cargo: &CargoExe, args: &VersionBumpArgs) -> Result<ExitCode, Error> {
    let root = workspace_root(cargo)?;
    let plan_path = if args.plan.is_absolute() {
        args.plan.clone()
    } else {
        root.join(&args.plan)
    };
    let raw = fs::read_to_string(&plan_path)?;
    let plan: Plan = toml::from_str(&raw).map_err(|e| Error::ReadPlan {
        path: plan_path.clone(),
        source: e,
    })?;

    let meta = cargo_metadata(cargo, &root)?;
    let members = publishable_members(&meta);
    let by_name: BTreeMap<&str, &crate::metadata::Package> =
        members.iter().map(|p| (p.name.as_str(), *p)).collect();

    let mut changes: Vec<Change> = Vec::new();
    let mut already: Vec<&str> = Vec::new();
    let mut drifted: Vec<(&str, &str, &str)> = Vec::new();

    for entry in &plan.crates {
        let pkg = by_name
            .get(entry.name.as_str())
            .ok_or_else(|| Error::UnknownCrate {
                name: entry.name.clone(),
            })?;
        match disposition(entry, &pkg.version)? {
            Disposition::NoTarget => {}
            Disposition::AlreadyApplied => already.push(&entry.name),
            Disposition::Drift => drifted.push((&entry.name, &pkg.version, &entry.current)),
            Disposition::Bump(to) => changes.push(Change {
                name: entry.name.clone(),
                from: pkg.version.clone(),
                to,
                bump: entry.bump,
            }),
        }
    }

    if !drifted.is_empty() {
        eprintln!(
            "error: {} crate(s) no longer match the plan's `current` version:",
            drifted.len()
        );
        for (name, manifest, planned) in &drifted {
            eprintln!("  {name}: manifest says {manifest}, plan says {planned}");
        }
        return Err(Error::Drifted {
            count: drifted.len(),
        });
    }

    println!("workspace root: {}", root.display());
    println!("plan:           {}", plan_path.display());
    println!();

    if changes.is_empty() {
        if already.is_empty() {
            println!("nothing to bump (plan is all skip/unchanged).");
        } else {
            println!(
                "nothing to bump — all {} planned crate(s) already sit at their target version.",
                already.len()
            );
        }
        return Ok(ExitCode::SUCCESS);
    }

    let width = changes.iter().map(|c| c.name.len()).max().unwrap_or(0);
    for c in &changes {
        println!(
            "{:width$}  {} -> {}  ({})",
            c.name,
            c.from,
            c.to,
            c.bump.label(),
            width = width,
        );
    }

    let groups = group_by_version(&changes);
    println!();
    println!(
        "{} crate(s) across {} target version(s)",
        changes.len(),
        groups.len()
    );
    if !already.is_empty() {
        println!(
            "{} crate(s) already at their planned version: {}",
            already.len(),
            already.join(" ")
        );
    }
    println!();

    if args.execute && !cargo.has_subcommand("release") {
        return Err(Error::ReleaseUnavailable);
    }

    for (version, names) in &groups {
        let owned: Vec<String> = names.iter().map(|s| s.to_string()).collect();
        if !args.execute {
            let p_args: String = owned.iter().map(|c| format!(" -p {c}")).collect();
            println!("$ cargo release version {version}{p_args} --allow-branch '*' --execute");
            continue;
        }
        // `--allow-branch '*'`: cargo-release refuses to run outside the
        // configured release branch, but bumping on a topic branch for review
        // is this subcommand's whole purpose, and no publish happens here.
        // `--no-confirm`: the table above is the preview.
        let (status, _) = batch::run_release_step(
            cargo,
            &root,
            &["version", version, "--allow-branch", "*"],
            &owned,
            false,
            true,
        )?;
        if !status.success() {
            eprintln!("cargo release version {version} failed (status {status}); aborting");
            return Ok(ExitCode::from(status.code().unwrap_or(1) as u8));
        }
    }

    println!();
    if args.execute {
        println!("manifests bumped. Review the diff and commit it yourself.");
    } else {
        println!("(dry run — re-run with --execute to write)");
    }
    Ok(ExitCode::SUCCESS)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(name: &str, current: &str, next: Option<&str>, bump: BumpDecision) -> CrateEntry {
        CrateEntry {
            name: name.to_string(),
            current: current.to_string(),
            next: next.map(str::to_string),
            bump,
            last_tag: None,
            reason: String::new(),
            files_changed: Vec::new(),
        }
    }

    fn change(name: &str, to: &str) -> Change {
        Change {
            name: name.to_string(),
            from: "0.0.0".to_string(),
            to: to.to_string(),
            bump: BumpDecision::Patch,
        }
    }

    #[test]
    fn skip_and_unchanged_have_no_target() {
        let e = entry("a", "0.2.0", None, BumpDecision::Skip);
        assert_eq!(disposition(&e, "0.2.0").unwrap(), Disposition::NoTarget);
        let e = entry("a", "0.2.0", None, BumpDecision::Unchanged);
        assert_eq!(disposition(&e, "0.2.0").unwrap(), Disposition::NoTarget);
    }

    #[test]
    fn bump_targets_the_planned_next() {
        let e = entry("a", "0.2.0", Some("0.2.1"), BumpDecision::Patch);
        assert_eq!(
            disposition(&e, "0.2.0").unwrap(),
            Disposition::Bump("0.2.1".to_string())
        );
    }

    #[test]
    fn rerun_after_a_partial_apply_is_a_no_op() {
        let e = entry("a", "0.2.0", Some("0.2.1"), BumpDecision::Patch);
        assert_eq!(
            disposition(&e, "0.2.1").unwrap(),
            Disposition::AlreadyApplied
        );
    }

    #[test]
    fn initial_needs_no_edit() {
        let e = entry("a", "0.1.0", Some("0.1.0"), BumpDecision::Initial);
        assert_eq!(
            disposition(&e, "0.1.0").unwrap(),
            Disposition::AlreadyApplied
        );
    }

    #[test]
    fn manifest_matching_neither_version_is_drift() {
        let e = entry("a", "0.2.0", Some("0.2.1"), BumpDecision::Patch);
        assert_eq!(disposition(&e, "0.3.0").unwrap(), Disposition::Drift);
    }

    #[test]
    fn pinned_without_next_is_an_error() {
        let e = entry("a", "0.2.0", None, BumpDecision::Pinned);
        assert!(matches!(
            disposition(&e, "0.2.0"),
            Err(Error::PinnedWithoutVersion { .. })
        ));
    }

    #[test]
    fn crates_sharing_a_target_version_ride_one_invocation() {
        let changes = vec![change("wb-b", "0.2.1"), change("wb-a", "0.2.1")];
        let groups = group_by_version(&changes);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups["0.2.1"], vec!["wb-a", "wb-b"]);
    }

    #[test]
    fn distinct_target_versions_become_distinct_invocations() {
        let changes = vec![
            change("wb-a", "0.2.1"),
            change("wb-b", "1.1.0"),
            change("wb-c", "0.2.1"),
        ];
        let groups = group_by_version(&changes);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups["0.2.1"], vec!["wb-a", "wb-c"]);
        assert_eq!(groups["1.1.0"], vec!["wb-b"]);
    }
}
