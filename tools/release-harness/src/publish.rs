//! Stage 3 — group plan entries by target version and run a chunked publish
//! per group, in-process via the `batch` module.
//!
//! Grouping is by the concrete `next` version, never by bump level: see
//! `key_for` for why a level keyword is the wrong dispatch currency.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use crate::PublishArgs;
use crate::batch::{self, BatchOptions, RunOutcome};
use crate::metadata::{
    CargoExe, Package, cargo_metadata, publishable_members, topo_sort, workspace_root,
};
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
    #[error("crate {name} is {bump} but next version missing from plan entry")]
    MissingNextVersion { name: String, bump: &'static str },
}

/// The version every dispatch is keyed on — always a concrete semver, never a
/// level keyword.
///
/// Dispatching `patch` / `minor` / `major` would be wrong in two ways. The
/// version step is not idempotent under a level: `cargo release version patch`
/// re-run bumps a *second* time, so a chunk that was partially bumped before a
/// failure gets double-bumped on resume. And `batch` can only run its
/// pre-flight "already on crates.io" prune when it knows the concrete target,
/// so a level silently disables the resumability check for the whole group.
/// The plan already computes `next` for every bumping crate, so use it.
fn key_for(entry: &CrateEntry) -> Option<String> {
    match entry.bump {
        BumpDecision::Patch | BumpDecision::Minor | BumpDecision::Major | BumpDecision::Pinned => {
            entry.next.clone()
        }
        // Initial = no bump; cargo-release wouldn't change the version but
        // would still publish + tag. We dispatch with the literal current
        // version so the resumability check works.
        BumpDecision::Initial => Some(entry.current.clone()),
        BumpDecision::Skip | BumpDecision::Unchanged => None,
    }
}

/// Whether the plan entry is expected to carry a `next` version.
fn requires_next(bump: BumpDecision) -> bool {
    matches!(
        bump,
        BumpDecision::Patch | BumpDecision::Minor | BumpDecision::Major | BumpDecision::Pinned
    )
}

pub fn run(cargo: &CargoExe, args: &PublishArgs) -> Result<ExitCode, Error> {
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

    // Group by target version. Each unique version becomes one chunked-publish
    // invocation.
    let mut groups: BTreeMap<String, Vec<&CrateEntry>> = BTreeMap::new();
    for entry in &plan.crates {
        if requires_next(entry.bump) && entry.next.is_none() {
            return Err(Error::MissingNextVersion {
                name: entry.name.clone(),
                bump: entry.bump.label(),
            });
        }
        if let Some(k) = key_for(entry) {
            groups.entry(k).or_default().push(entry);
        }
    }

    if groups.is_empty() {
        println!("no crates to publish (plan is all skip/unchanged).");
        return Ok(ExitCode::SUCCESS);
    }

    // Compute a single workspace-wide topology so each group can preserve
    // dependency-before-dependant ordering on the subset it covers.
    let meta = cargo_metadata(cargo, &root)?;
    let all_publishable = publishable_members(&meta);
    let full_order = topo_sort(&meta, &all_publishable)?;
    let order_index: BTreeMap<&str, usize> = full_order
        .iter()
        .enumerate()
        .map(|(i, n)| (n.as_str(), i))
        .collect();

    println!("workspace root:  {}", root.display());
    println!("groups to drive: {}", groups.len());
    println!();

    for (version, entries) in &groups {
        let mut names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
        names.sort_by_key(|n| order_index.get(n.as_str()).copied().unwrap_or(usize::MAX));
        println!("--- group `{version}` ({} crate(s)) ---", names.len());
        println!(
            "$ release-harness batch --version {version} --crates {}{}{}",
            names.join(" "),
            if args.sign { " --sign" } else { "" },
            if args.no_confirm { " --no-confirm" } else { "" },
        );
        if !args.execute {
            continue;
        }
        let outcome = batch::run_chunked(BatchOptions {
            cargo,
            root: &root,
            version_or_level: version,
            crates: names,
            chunk_size: args.chunk_size,
            sleep_between_chunks: Duration::from_secs(args.sleep),
            max_retries: args.max_retries,
            retry_buffer: Duration::from_secs(args.retry_buffer),
            execute: true,
            sign: args.sign,
            no_confirm: args.no_confirm,
            skip_version_check: args.skip_version_check,
        })?;
        if let RunOutcome::ChunkFailed(status) = outcome {
            eprintln!("chunked publish failed for group `{version}` (status {status}); aborting");
            return Ok(ExitCode::from(status.code().unwrap_or(1) as u8));
        }
    }

    if !args.execute {
        println!();
        println!("(dry run — re-run with --execute to actually publish)");
    } else {
        println!();
        println!("all groups complete.");
    }
    Ok(ExitCode::SUCCESS)
}

/// Direct `batch` subcommand entry point. Used by callers who want the
/// former `release-batch` UX without going through a plan file — typically
/// for first-launch when every publishable crate is in scope, or for
/// targeted retries.
pub fn run_batch(cargo: &CargoExe, args: &crate::BatchArgs) -> Result<ExitCode, Error> {
    let root = workspace_root(cargo)?;
    let meta = cargo_metadata(cargo, &root)?;
    let all_members: Vec<&Package> = publishable_members(&meta);

    let restriction: Option<std::collections::BTreeSet<String>> = if !args.crates.is_empty() {
        Some(args.crates.iter().cloned().collect())
    } else if let Some(p) = &args.crates_file {
        let path = if p.is_absolute() {
            p.clone()
        } else {
            root.join(p)
        };
        let raw = fs::read_to_string(&path)?;
        let mut out = std::collections::BTreeSet::new();
        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            out.insert(trimmed.to_string());
        }
        Some(out)
    } else {
        None
    };

    let members: Vec<&Package> = match &restriction {
        Some(subset) => {
            let unknown: Vec<&str> = subset
                .iter()
                .filter(|name| !all_members.iter().any(|p| p.name == **name))
                .map(|s| s.as_str())
                .collect();
            if !unknown.is_empty() {
                eprintln!(
                    "error: --crates/--crates-file lists crate(s) that are not publishable workspace members: {}",
                    unknown.join(", ")
                );
                return Ok(ExitCode::FAILURE);
            }
            all_members
                .into_iter()
                .filter(|p| subset.contains(&p.name))
                .collect()
        }
        None => all_members,
    };

    if members.is_empty() {
        eprintln!("no crates to release");
        return Ok(ExitCode::SUCCESS);
    }

    let order = topo_sort(&meta, &members)?;
    eprintln!("workspace root: {}", root.display());
    eprintln!("publishable crates: {}", order.len());

    let outcome = batch::run_chunked(BatchOptions {
        cargo,
        root: &root,
        version_or_level: &args.version,
        crates: order,
        chunk_size: args.chunk_size,
        sleep_between_chunks: Duration::from_secs(args.sleep),
        max_retries: args.max_retries,
        retry_buffer: Duration::from_secs(args.retry_buffer),
        execute: args.execute,
        sign: args.sign,
        no_confirm: args.no_confirm,
        skip_version_check: args.skip_version_check,
    })?;
    match outcome {
        RunOutcome::Success => Ok(ExitCode::SUCCESS),
        RunOutcome::ChunkFailed(status) => Ok(ExitCode::from(status.code().unwrap_or(1) as u8)),
    }
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

    #[test]
    fn level_bumps_dispatch_their_literal_target_not_the_level() {
        for (bump, next) in [
            (BumpDecision::Patch, "0.2.1"),
            (BumpDecision::Minor, "0.3.0"),
            (BumpDecision::Major, "1.0.0"),
        ] {
            let e = entry("a", "0.2.0", Some(next), bump);
            assert_eq!(key_for(&e).as_deref(), Some(next));
        }
    }

    #[test]
    fn pinned_and_initial_keep_their_literal_keys() {
        let e = entry("a", "0.2.0", Some("0.9.9"), BumpDecision::Pinned);
        assert_eq!(key_for(&e).as_deref(), Some("0.9.9"));
        let e = entry("a", "0.1.0", Some("0.1.0"), BumpDecision::Initial);
        assert_eq!(key_for(&e).as_deref(), Some("0.1.0"));
    }

    #[test]
    fn skip_and_unchanged_are_not_dispatched() {
        let e = entry("a", "0.2.0", None, BumpDecision::Skip);
        assert_eq!(key_for(&e), None);
        let e = entry("a", "0.2.0", None, BumpDecision::Unchanged);
        assert_eq!(key_for(&e), None);
    }

    #[test]
    fn crates_sharing_a_target_version_land_in_one_group() {
        // Two crates bumping patch from different currents no longer share a
        // group just because they share a level.
        let entries = [
            entry("a", "0.2.0", Some("0.2.1"), BumpDecision::Patch),
            entry("b", "0.3.0", Some("0.3.1"), BumpDecision::Patch),
            entry("c", "0.2.0", Some("0.2.1"), BumpDecision::Patch),
        ];
        let mut groups: BTreeMap<String, Vec<&str>> = BTreeMap::new();
        for e in &entries {
            if let Some(k) = key_for(e) {
                groups.entry(k).or_default().push(&e.name);
            }
        }
        assert_eq!(groups.len(), 2);
        assert_eq!(groups["0.2.1"], vec!["a", "c"]);
        assert_eq!(groups["0.3.1"], vec!["b"]);
    }

    #[test]
    fn every_bumping_decision_requires_a_next_version() {
        for bump in [
            BumpDecision::Patch,
            BumpDecision::Minor,
            BumpDecision::Major,
            BumpDecision::Pinned,
        ] {
            assert!(requires_next(bump), "{bump:?} must carry next");
        }
        for bump in [
            BumpDecision::Initial,
            BumpDecision::Skip,
            BumpDecision::Unchanged,
        ] {
            assert!(!requires_next(bump), "{bump:?} must not require next");
        }
    }
}
