//! Stage 3 — group plan entries by bump level and run a chunked publish per
//! group, in-process via the `batch` module.

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
    #[error("crate {name} pinned but next version missing from plan entry")]
    PinnedWithoutVersion { name: String },
}

fn key_for(entry: &CrateEntry) -> Option<String> {
    match entry.bump {
        BumpDecision::Patch => Some("patch".to_string()),
        BumpDecision::Minor => Some("minor".to_string()),
        BumpDecision::Major => Some("major".to_string()),
        BumpDecision::Pinned => entry.next.clone(),
        // Initial = no bump; cargo-release wouldn't change the version but
        // would still publish + tag. We dispatch with the literal current
        // version so the resumability check works.
        BumpDecision::Initial => Some(entry.current.clone()),
        BumpDecision::Skip | BumpDecision::Unchanged => None,
    }
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

    // Group by version-or-level key. Each unique key becomes one
    // chunked-publish invocation.
    let mut groups: BTreeMap<String, Vec<&CrateEntry>> = BTreeMap::new();
    for entry in &plan.crates {
        if matches!(entry.bump, BumpDecision::Pinned) && entry.next.is_none() {
            return Err(Error::PinnedWithoutVersion {
                name: entry.name.clone(),
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

    for (version_or_level, entries) in &groups {
        let mut names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
        names.sort_by_key(|n| order_index.get(n.as_str()).copied().unwrap_or(usize::MAX));
        println!(
            "--- group `{version_or_level}` ({} crate(s)) ---",
            names.len()
        );
        println!(
            "$ release-harness batch --version {version_or_level} --crates {}{}{}",
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
            version_or_level,
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
            eprintln!(
                "chunked publish failed for group `{version_or_level}` (status {status}); aborting"
            );
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
