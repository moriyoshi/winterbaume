//! Stage 4 — sort every planned crate into one workspace-topological order,
//! each at its own target version, and run one chunked publish over it,
//! in-process via the `batch` module.
//!
//! Each crate is dispatched at its concrete `next` version, never at its bump
//! level: see `key_for` for why a level keyword is the wrong dispatch
//! currency. See `publish_order` for why the order spans every version.
//!
//! By default this stage publishes and nothing else. The version bump reaches
//! `main` through the release PR ( `version-bump` → PR → merge ) and the tags
//! are created afterwards by the `tag` subcommand, at the merge commit, once
//! crates.io has accepted every crate. `--all-in-one` restores the pre-PR
//! behaviour where this command also bumps, commits, tags, and pushes.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use crate::batch::{self, BatchOptions, RunOutcome, Stages, Targets};
use crate::metadata::{
    CargoExe, Package, cargo_metadata, publishable_members, topo_sort, workspace_root,
};
use crate::plan::{BumpDecision, CrateEntry, Plan};
use crate::{PublishArgs, TagArgs};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Metadata(#[from] crate::metadata::Error),
    #[error(transparent)]
    Batch(#[from] batch::Error),
    #[error(transparent)]
    Git(#[from] crate::git::Error),
    #[error(transparent)]
    Tag(#[from] Box<crate::tag::Error>),
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
    #[error(
        "{} crate(s) are not at their planned version in this tree: {}. \
         The release PR has not landed here — run `release-harness version-bump --execute`, \
         merge the release PR into main, then re-run publish ( or pass --all-in-one to bump \
         in place, the pre-PR flow ).",
        .crates.len(),
        .crates.join(", "),
    )]
    ManifestsBehindPlan { crates: Vec<String> },
    #[error("plan lists {name}, which is not a publishable workspace member")]
    UnknownCrate { name: String },
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
///
/// Shared with the `tag` subcommand: the tag set and the publish set are the
/// same set by construction, so they must be derived the same way.
pub(crate) fn key_for(entry: &CrateEntry) -> Option<String> {
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

/// Every crate from every group in one workspace-topological order, with its
/// own target version.
///
/// Groups are keyed by target version, and a version says nothing about the
/// dependency graph: driving them one after another in key order published
/// the `0.2.1` group, which depends on `winterbaume-core` `0.3.0`, before the
/// `0.3.0` group that holds core. Crates missing from `full_order` go last, by
/// name, so the result is deterministic.
fn publish_order(
    groups: &BTreeMap<String, Vec<&CrateEntry>>,
    full_order: &[String],
) -> (Vec<String>, Targets) {
    let index: BTreeMap<&str, usize> = full_order
        .iter()
        .enumerate()
        .map(|(i, n)| (n.as_str(), i))
        .collect();
    let mut targets = BTreeMap::new();
    for (version, entries) in groups {
        for e in entries {
            targets.insert(e.name.clone(), version.clone());
        }
    }
    let mut names: Vec<String> = targets.keys().cloned().collect();
    names.sort_by(|a, b| {
        let ka = index.get(a.as_str()).copied().unwrap_or(usize::MAX);
        let kb = index.get(b.as_str()).copied().unwrap_or(usize::MAX);
        ka.cmp(&kb).then_with(|| a.cmp(b))
    });
    (names, Targets::PerCrate(targets))
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

    // One workspace-wide topology, and one publish order across every group.
    let meta = cargo_metadata(cargo, &root)?;
    let all_publishable = publishable_members(&meta);
    let full_order = topo_sort(&meta, &all_publishable)?;
    let (names, targets) = publish_order(&groups, &full_order);

    let stages = if args.all_in_one {
        Stages::all()
    } else {
        Stages::publish_only()
    };

    println!("workspace root:  {}", root.display());
    println!("target versions: {}", groups.len());
    println!(
        "mode:            {}",
        if args.all_in_one {
            "all-in-one (bump + commit + publish + tag + push)"
        } else {
            "publish only (the release PR carries the bump; `tag` follows)"
        }
    );
    if !args.all_in_one {
        // Chunk by chunk, `batch` refuses to publish a crate whose manifest is
        // behind the plan — but only once that chunk's turn comes round, by
        // which time the earlier chunks are on crates.io for good. One sweep
        // up front turns a half-published release into a clean refusal.
        assert_manifests_match_plan(&groups, &all_publishable)?;
        warn_if_not_on_release_branch(&root, args)?;
    }
    // Both modes write commits or tags once the crates are on crates.io for
    // good. Without a git identity that half fails after the irreversible
    // one, so find out now. `--tag` always signs; `--all-in-one` follows
    // `--sign`.
    if args.tag || args.all_in_one {
        crate::tag::verify_identity(&root, args.tag || args.sign).map_err(Box::new)?;
    }
    println!();

    for (version, entries) in &groups {
        println!("group `{version}`: {} crate(s)", entries.len());
    }
    println!(
        "\npublish order, dependencies first, across all groups ( {} crate(s), chunks of {} ):",
        names.len(),
        args.chunk_size
    );
    for (i, chunk) in names.chunks(args.chunk_size.max(1)).enumerate() {
        let listed: Vec<String> = chunk
            .iter()
            .map(|n| format!("{n} v{}", targets.for_crate(n).unwrap_or("?")))
            .collect();
        println!("  {:>3}: {}", i + 1, listed.join(", "));
    }
    if args.all_in_one {
        println!(
            "( all-in-one: each chunk also bumps, commits, tags and pushes; one \
             `cargo release version` per target version present in the chunk )"
        );
    }
    println!(
        "( crates already on crates.io at their target are dropped at run time, \
         which moves the chunk boundaries )"
    );

    if args.execute {
        let outcome = batch::run_chunked(BatchOptions {
            cargo,
            root: &root,
            targets,
            crates: names,
            chunk_size: args.chunk_size,
            sleep_between_chunks: Duration::from_secs(args.sleep),
            max_retries: args.max_retries,
            retry_buffer: Duration::from_secs(args.retry_buffer),
            execute: true,
            sign: args.sign,
            no_confirm: args.no_confirm,
            skip_version_check: args.skip_version_check,
            stages,
        })?;
        if let RunOutcome::ChunkFailed(status) = outcome {
            eprintln!("chunked publish failed (status {status}); aborting");
            return Ok(ExitCode::from(status.code().unwrap_or(1) as u8));
        }
    }

    println!();
    if !args.execute {
        println!("(dry run — re-run with --execute to actually publish)");
        if !args.all_in_one && !args.tag {
            println!("next after a real publish: release-harness tag --execute");
        }
        return Ok(ExitCode::SUCCESS);
    }

    println!("all crates published.");
    if args.all_in_one {
        return Ok(ExitCode::SUCCESS);
    }
    if args.tag {
        println!();
        return tag_after_publish(cargo, args)
            .map_err(Box::new)
            .map_err(Into::into);
    }
    println!();
    println!("nothing is tagged yet. Tag the merge commit and push the tags with:");
    println!(
        "  release-harness tag --execute{}",
        if args.plan.as_os_str() == "release-plan.toml" {
            String::new()
        } else {
            format!(" --plan {}", args.plan.display())
        }
    );
    Ok(ExitCode::SUCCESS)
}

/// `publish --tag`: run the `tag` stage at HEAD straight after the publish, for
/// operators who want the two halves in one command. The checks are the tag
/// subcommand's own — nothing is loosened by coming through this door.
fn tag_after_publish(cargo: &CargoExe, args: &PublishArgs) -> Result<ExitCode, crate::tag::Error> {
    crate::tag::run(
        cargo,
        &TagArgs {
            plan: args.plan.clone(),
            at: "HEAD".to_string(),
            execute: true,
            // Always signed, regardless of publish's `--sign` ( which governs
            // the release commit this mode does not create ). Unsigned tags
            // are available through the standalone `tag --no-sign`.
            no_sign: false,
            no_push: false,
            allow_unpublished: false,
            allow_unmerged: false,
            remote: "origin".to_string(),
            branch: "main".to_string(),
        },
    )
}

/// Every crate the plan dispatches must already sit at its target version in
/// the working tree. Checked across all groups before the first publish, since
/// a crates.io release cannot be taken back.
fn assert_manifests_match_plan(
    groups: &BTreeMap<String, Vec<&CrateEntry>>,
    publishable: &[&Package],
) -> Result<(), Error> {
    let by_name: BTreeMap<&str, &Package> =
        publishable.iter().map(|p| (p.name.as_str(), *p)).collect();
    let mut behind: Vec<String> = Vec::new();
    for (version, entries) in groups {
        for entry in entries {
            let pkg = by_name
                .get(entry.name.as_str())
                .ok_or_else(|| Error::UnknownCrate {
                    name: entry.name.clone(),
                })?;
            if &pkg.version != version {
                behind.push(format!(
                    "{} (manifest {}, planned {version})",
                    entry.name, pkg.version
                ));
            }
        }
    }
    if !behind.is_empty() {
        return Err(Error::ManifestsBehindPlan { crates: behind });
    }
    Ok(())
}

/// In the PR-based flow the publish runs from the merge commit on `main`. A
/// commit that `origin` has never seen is not fatal — the remote-tracking ref
/// can simply be stale — but it is worth saying out loud before 241 crates go
/// to crates.io from the wrong tree.
fn warn_if_not_on_release_branch(root: &std::path::Path, args: &PublishArgs) -> Result<(), Error> {
    let base = "origin/main";
    if !crate::git::rev_exists(root, base)? {
        return Ok(());
    }
    let head = batch::rev_parse(root, "HEAD")?;
    if crate::git::is_ancestor(root, &head, base)? {
        return Ok(());
    }
    let hint = if args.execute { "" } else { " (dry run)" };
    println!(
        "warning:         HEAD ({head}) is not contained in {base}{hint} — in the PR-based \
         flow you publish from the merged release commit. Run `git fetch origin` if the \
         remote-tracking ref is just stale."
    );
    Ok(())
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
    // `batch` commits and tags every chunk after publishing it; check git can.
    crate::tag::verify_identity(&root, args.sign).map_err(Box::new)?;

    let outcome = batch::run_chunked(BatchOptions {
        cargo,
        root: &root,
        targets: Targets::Uniform(args.version.clone()),
        crates: order,
        chunk_size: args.chunk_size,
        sleep_between_chunks: Duration::from_secs(args.sleep),
        max_retries: args.max_retries,
        retry_buffer: Duration::from_secs(args.retry_buffer),
        execute: args.execute,
        sign: args.sign,
        no_confirm: args.no_confirm,
        skip_version_check: args.skip_version_check,
        // The bypass mode owns the whole lifecycle: it exists for the first
        // launch and for targeted retries, neither of which goes through a
        // release PR.
        stages: Stages::all(),
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

    fn package(name: &str, version: &str) -> Package {
        Package {
            id: name.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            manifest_path: format!("crates/{name}/Cargo.toml").into(),
            publish: None,
            dependencies: Vec::new(),
        }
    }

    fn one_group<'a>(
        version: &str,
        entries: &'a [CrateEntry],
    ) -> BTreeMap<String, Vec<&'a CrateEntry>> {
        let mut groups = BTreeMap::new();
        groups.insert(version.to_string(), entries.iter().collect());
        groups
    }

    #[test]
    fn a_tree_already_at_the_planned_versions_passes_the_pre_flight() {
        let entries = [entry("a", "0.2.0", Some("0.2.1"), BumpDecision::Patch)];
        let pkgs = [package("a", "0.2.1")];
        let refs: Vec<&Package> = pkgs.iter().collect();
        assert!(assert_manifests_match_plan(&one_group("0.2.1", &entries), &refs).is_ok());
    }

    #[test]
    fn an_unbumped_manifest_fails_the_pre_flight_before_anything_publishes() {
        // The release PR has not landed: the plan wants 0.2.1, the tree still
        // says 0.2.0. Catching it here is the difference between refusing and
        // publishing the groups that happen to sort first.
        let entries = [entry("a", "0.2.0", Some("0.2.1"), BumpDecision::Patch)];
        let pkgs = [package("a", "0.2.0")];
        let refs: Vec<&Package> = pkgs.iter().collect();
        let err = assert_manifests_match_plan(&one_group("0.2.1", &entries), &refs).unwrap_err();
        assert!(matches!(err, Error::ManifestsBehindPlan { ref crates } if crates.len() == 1));
    }

    #[test]
    fn a_plan_naming_an_unknown_crate_fails_the_pre_flight() {
        let entries = [entry("ghost", "0.2.0", Some("0.2.1"), BumpDecision::Patch)];
        let refs: Vec<&Package> = Vec::new();
        assert!(matches!(
            assert_manifests_match_plan(&one_group("0.2.1", &entries), &refs),
            Err(Error::UnknownCrate { .. })
        ));
    }

    #[test]
    fn the_publish_order_follows_dependencies_across_version_groups() {
        // The 2026-11-21 cut: `core` goes to 0.3.0 while its dependants go to
        // 0.2.1 and 1.0.1. Key order would run `0.2.1` first; the topology
        // must win.
        let entries = [
            entry("core", "0.2.0", Some("0.3.0"), BumpDecision::Minor),
            entry("svc-a", "0.2.0", Some("0.2.1"), BumpDecision::Patch),
            entry("svc-b", "1.0.0", Some("1.0.1"), BumpDecision::Patch),
            entry("server", "0.2.8", Some("0.2.9"), BumpDecision::Patch),
        ];
        let mut groups: BTreeMap<String, Vec<&CrateEntry>> = BTreeMap::new();
        for e in &entries {
            groups.entry(key_for(e).unwrap()).or_default().push(e);
        }
        assert_eq!(
            groups.keys().cloned().collect::<Vec<_>>(),
            ["0.2.1", "0.2.9", "0.3.0", "1.0.1"],
            "the groups' own order puts dependants first"
        );
        let full_order: Vec<String> = ["core", "svc-b", "svc-a", "server"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let (names, targets) = publish_order(&groups, &full_order);
        assert_eq!(names, ["core", "svc-b", "svc-a", "server"]);
        assert_eq!(targets.for_crate("core"), Some("0.3.0"));
        assert_eq!(targets.for_crate("svc-a"), Some("0.2.1"));
        assert_eq!(targets.for_crate("svc-b"), Some("1.0.1"));
        assert_eq!(targets.for_crate("server"), Some("0.2.9"));
    }

    #[test]
    fn crates_outside_the_topology_go_last_by_name() {
        let entries = [
            entry("zeta", "0.1.0", Some("0.1.1"), BumpDecision::Patch),
            entry("alpha", "0.1.0", Some("0.1.1"), BumpDecision::Patch),
            entry("core", "0.2.0", Some("0.3.0"), BumpDecision::Minor),
        ];
        let mut groups: BTreeMap<String, Vec<&CrateEntry>> = BTreeMap::new();
        for e in &entries {
            groups.entry(key_for(e).unwrap()).or_default().push(e);
        }
        let (names, _) = publish_order(&groups, &["core".to_string()]);
        assert_eq!(names, ["core", "alpha", "zeta"]);
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
