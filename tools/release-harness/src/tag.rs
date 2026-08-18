//! Stage 5 — tag the commit the release landed on, one annotated ( signed by
//! default ) tag per published crate, then push the tags.
//!
//! Split out of `publish` because the release reaches `main` through a pull
//! request: `plan` → `changelog` + `version-bump` → release PR → merge →
//! `publish` → `tag` → push. The commit that must carry `<crate>-v<version>`
//! is the PR's merge commit, which only exists after the PR lands, and the
//! tags must not appear until crates.io has actually accepted the crates — a
//! tag for a version nobody can download is worse than no tag at all, because
//! the next `plan` diffs against it and reports the crate as unchanged.
//!
//! Everything here is a state probe first and an action second, so the command
//! is safe to re-run: tags already at the requested commit are left alone,
//! tags already on `origin` are not re-pushed, and a tag pointing anywhere
//! else aborts the run rather than being moved.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::TagArgs;
use crate::batch;
use crate::git;
use crate::metadata::{CargoExe, cargo_metadata, publishable_members, workspace_root};
use crate::plan::Plan;
use crate::publish::key_for;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Metadata(#[from] crate::metadata::Error),
    #[error(transparent)]
    Batch(#[from] batch::Error),
    #[error(transparent)]
    Git(#[from] git::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("cannot read plan {path}: {source}")]
    ReadPlan {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("plan lists {name}, which is not a publishable workspace member")]
    UnknownCrate { name: String },
    #[error(
        "{} crate(s) do not carry their planned version at {rev}: {}. \
         That commit is not the one the release PR landed — check out the merge commit \
         ( or pass --at <rev> ) before tagging.",
        .crates.len(),
        .crates.join(", "),
    )]
    ManifestMismatch { crates: Vec<String>, rev: String },
    #[error(
        "{rev} is not contained in {base}: tagging it would push a commit that never \
         went through the release PR. Merge the PR first, `git fetch {remote}`, or pass \
         --allow-unmerged if you know what you are doing."
    )]
    NotMerged {
        rev: String,
        base: String,
        remote: String,
    },
    #[error(
        "{} crate(s) are not on crates.io at their planned version: {}. \
         Publish first ( `release-harness publish --execute` ) — a tag for a version that \
         was never published makes the next `plan` report the crate as unchanged. \
         Pass --allow-unpublished to tag anyway.",
        .crates.len(),
        .crates.join(", "),
    )]
    Unpublished { crates: Vec<String> },
    #[error(
        "{} tag(s) already exist at a different commit: {}. Refusing to move a published \
         tag; delete it deliberately if it is genuinely wrong.",
        .conflicts.len(),
        .conflicts.join(", "),
    )]
    TagConflict { conflicts: Vec<String> },
}

/// What the repository already knows about one crate's tag.
#[derive(Clone, Debug, PartialEq, Eq)]
enum TagState {
    /// Neither local nor remote has it.
    Missing,
    /// Local tag is at the requested commit; `origin` does not have it yet.
    NeedsPush,
    /// Local and remote both have it at the requested commit.
    Done,
    /// Exists, but points somewhere other than the requested commit.
    Conflict { at: String, scope: &'static str },
}

/// One crate's tag plus the state probe's verdict.
struct Target {
    name: String,
    version: String,
    tag: String,
    state: TagState,
}

/// Classify one tag against the requested commit. `local` / `remote` are the
/// commits the tag currently peels to, when it exists at all.
fn classify(local: Option<&str>, remote: Option<&str>, want: &str) -> TagState {
    if let Some(l) = local {
        if l != want {
            return TagState::Conflict {
                at: l.to_string(),
                scope: "local",
            };
        }
    }
    if let Some(r) = remote {
        if r != want {
            return TagState::Conflict {
                at: r.to_string(),
                scope: "origin",
            };
        }
    }
    match (local.is_some(), remote.is_some()) {
        (_, true) => TagState::Done,
        (true, false) => TagState::NeedsPush,
        (false, false) => TagState::Missing,
    }
}

/// The `[package] version` a manifest declares at some revision. `None` when
/// the manifest is absent at that revision or inherits its version from the
/// workspace — neither is something this check can adjudicate, so the caller
/// reports and moves on rather than failing.
fn package_version(manifest: &str) -> Option<String> {
    let doc: toml::Value = manifest.parse().ok()?;
    doc.get("package")?
        .get("version")?
        .as_str()
        .map(str::to_string)
}

pub fn run(cargo: &CargoExe, args: &TagArgs) -> Result<ExitCode, Error> {
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

    let planned: Vec<(String, String)> = plan
        .crates
        .iter()
        .filter_map(|e| key_for(e).map(|v| (e.name.clone(), v)))
        .collect();

    println!("workspace root: {}", root.display());
    println!("plan:           {}", plan_path.display());

    if planned.is_empty() {
        println!();
        println!("nothing to tag (plan is all skip/unchanged).");
        return Ok(ExitCode::SUCCESS);
    }

    let rev = batch::rev_parse(&root, &format!("{}^{{commit}}", args.at))?;
    println!("tagging commit: {rev} ({})", args.at);
    println!();

    verify_manifests(cargo, &root, &rev, &planned)?;
    verify_merged(&root, &rev, args)?;
    if !args.allow_unpublished {
        verify_published(&planned)?;
    } else {
        println!("crates.io check skipped (--allow-unpublished).");
    }

    // Tag state. Annotated tags peel through `^{}` on the remote; take the
    // peeled entry when it is there so we compare commits with commits.
    let local = batch::local_tags(&root)?;
    let remote_refs = batch::remote_refs(&root)?;
    let mut targets: Vec<Target> = Vec::with_capacity(planned.len());
    for (name, version) in &planned {
        let tag = format!("{name}-v{version}");
        let local_sha = if local.contains(&tag) {
            Some(batch::rev_parse(&root, &format!("{tag}^{{commit}}"))?)
        } else {
            None
        };
        let remote_sha = remote_refs
            .get(&format!("refs/tags/{tag}^{{}}"))
            .or_else(|| remote_refs.get(&format!("refs/tags/{tag}")))
            .cloned();
        let state = classify(local_sha.as_deref(), remote_sha.as_deref(), &rev);
        targets.push(Target {
            name: name.clone(),
            version: version.clone(),
            tag,
            state,
        });
    }

    let conflicts: Vec<String> = targets
        .iter()
        .filter_map(|t| match &t.state {
            TagState::Conflict { at, scope } => Some(format!("{} ({scope} {at})", t.tag)),
            _ => None,
        })
        .collect();
    if !conflicts.is_empty() {
        return Err(Error::TagConflict { conflicts });
    }

    let width = targets.iter().map(|t| t.tag.len()).max().unwrap_or(0);
    for t in &targets {
        let note = match t.state {
            TagState::Missing => "create",
            TagState::NeedsPush => "exists locally, push",
            TagState::Done => "already tagged and pushed",
            TagState::Conflict { .. } => unreachable!("conflicts abort above"),
        };
        println!("{:width$}  {note}", t.tag, width = width);
    }
    println!();

    let to_create: Vec<&Target> = targets
        .iter()
        .filter(|t| t.state == TagState::Missing)
        .collect();
    let to_push: Vec<&Target> = targets
        .iter()
        .filter(|t| matches!(t.state, TagState::Missing | TagState::NeedsPush))
        .collect();

    if to_create.is_empty() && to_push.is_empty() {
        println!(
            "all {} tag(s) already exist on origin at {rev}.",
            targets.len()
        );
        return Ok(ExitCode::SUCCESS);
    }

    let sign = !args.no_sign;
    println!(
        "{} tag(s) to create, {} to push{}",
        to_create.len(),
        to_push.len(),
        if sign { ", signed" } else { ", unsigned" },
    );
    println!();

    if !args.execute {
        for t in &to_create {
            println!(
                "$ git tag {}-m \"{}\" {} {rev}",
                if sign { "-s " } else { "-a " },
                batch::tag_message(&t.name, &t.version),
                t.tag,
            );
        }
        if !args.no_push && !to_push.is_empty() {
            println!(
                "$ git push {} {}",
                args.remote,
                to_push
                    .iter()
                    .map(|t| t.tag.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        println!();
        println!("(dry run — re-run with --execute to create and push the tags)");
        return Ok(ExitCode::SUCCESS);
    }

    for t in &to_create {
        batch::create_tag_at(
            &root,
            &t.tag,
            &batch::tag_message(&t.name, &t.version),
            sign,
            &rev,
        )?;
        println!("created tag {}", t.tag);
    }

    if args.no_push {
        println!();
        println!("push skipped (--no-push). Push the tags yourself to trigger the binary release.");
        return Ok(ExitCode::SUCCESS);
    }

    let refs: Vec<String> = to_push.iter().map(|t| t.tag.clone()).collect();
    batch::git_push(&root, &refs)?;
    println!();
    println!("{} tag(s) pushed to {}.", refs.len(), args.remote);
    Ok(ExitCode::SUCCESS)
}

/// Every crate must already declare its planned version at the commit being
/// tagged. This is the check that catches "the release PR has not merged yet"
/// and "you are tagging main from before the merge".
fn verify_manifests(
    cargo: &CargoExe,
    root: &std::path::Path,
    rev: &str,
    planned: &[(String, String)],
) -> Result<(), Error> {
    let meta = cargo_metadata(cargo, root)?;
    let members = publishable_members(&meta);
    let by_name: BTreeMap<&str, &crate::metadata::Package> =
        members.iter().map(|p| (p.name.as_str(), *p)).collect();

    let mut mismatched: Vec<String> = Vec::new();
    let mut unverifiable: Vec<&str> = Vec::new();
    for (name, version) in planned {
        let pkg = by_name
            .get(name.as_str())
            .ok_or_else(|| Error::UnknownCrate { name: name.clone() })?;
        let rel = pkg
            .manifest_path
            .strip_prefix(root)
            .unwrap_or(&pkg.manifest_path)
            .to_string_lossy()
            .replace('\\', "/");
        match git::show_file(root, rev, &rel)?
            .as_deref()
            .and_then(package_version)
        {
            Some(found) if &found == version => {}
            Some(found) => mismatched.push(format!("{name} (declares {found}, planned {version})")),
            None => unverifiable.push(name),
        }
    }

    if !unverifiable.is_empty() {
        println!(
            "note: {} crate(s) declare no literal `[package] version` at {rev}; \
             their manifests were not checked: {}",
            unverifiable.len(),
            unverifiable.join(" "),
        );
    }
    if !mismatched.is_empty() {
        return Err(Error::ManifestMismatch {
            crates: mismatched,
            rev: rev.to_string(),
        });
    }
    println!(
        "manifests at {rev}: all {} at their planned version.",
        planned.len()
    );
    Ok(())
}

/// The commit must already be on the release branch as `origin` sees it —
/// otherwise the push would carry commits that never went through the PR.
fn verify_merged(root: &std::path::Path, rev: &str, args: &TagArgs) -> Result<(), Error> {
    if args.allow_unmerged {
        println!("containment check skipped (--allow-unmerged).");
        return Ok(());
    }
    let base = format!("{}/{}", args.remote, args.branch);
    if !git::rev_exists(root, &base)? {
        println!(
            "note: {base} does not exist locally; skipping the containment check. \
             Run `git fetch {}` for it to mean something.",
            args.remote,
        );
        return Ok(());
    }
    if !git::is_ancestor(root, rev, &base)? {
        return Err(Error::NotMerged {
            rev: rev.to_string(),
            base,
            remote: args.remote.clone(),
        });
    }
    println!("containment: {rev} is contained in {base}.");
    Ok(())
}

/// Tags follow the publish, never precede it.
fn verify_published(planned: &[(String, String)]) -> Result<(), Error> {
    let mut missing: Vec<String> = Vec::new();
    for (name, version) in planned {
        if !batch::already_on_crates_io(name, version) {
            missing.push(format!("{name} v{version}"));
        }
    }
    if !missing.is_empty() {
        return Err(Error::Unpublished { crates: missing });
    }
    println!("crates.io: all {} crate(s) published.", planned.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA: &str = "1111111111111111111111111111111111111111";
    const OTHER: &str = "2222222222222222222222222222222222222222";

    #[test]
    fn absent_everywhere_is_missing() {
        assert_eq!(classify(None, None, SHA), TagState::Missing);
    }

    #[test]
    fn local_only_at_the_right_commit_needs_a_push() {
        assert_eq!(classify(Some(SHA), None, SHA), TagState::NeedsPush);
    }

    #[test]
    fn present_on_origin_is_done() {
        assert_eq!(classify(Some(SHA), Some(SHA), SHA), TagState::Done);
    }

    #[test]
    fn a_tag_only_on_origin_is_still_done() {
        // Someone else pushed it; nothing left for us to do even though our
        // clone has not fetched the tag.
        assert_eq!(classify(None, Some(SHA), SHA), TagState::Done);
    }

    #[test]
    fn a_local_tag_elsewhere_is_a_conflict() {
        assert_eq!(
            classify(Some(OTHER), None, SHA),
            TagState::Conflict {
                at: OTHER.to_string(),
                scope: "local",
            }
        );
    }

    #[test]
    fn a_remote_tag_elsewhere_is_a_conflict() {
        // The dangerous case: our local tag looks right, but origin already
        // published the same name at another commit.
        assert_eq!(
            classify(Some(SHA), Some(OTHER), SHA),
            TagState::Conflict {
                at: OTHER.to_string(),
                scope: "origin",
            }
        );
    }

    #[test]
    fn package_version_reads_a_literal_version() {
        let manifest = "[package]\nname = \"winterbaume-foo\"\nversion = \"0.3.0\"\n";
        assert_eq!(package_version(manifest).as_deref(), Some("0.3.0"));
    }

    #[test]
    fn package_version_declines_workspace_inheritance() {
        let manifest = "[package]\nname = \"winterbaume-foo\"\nversion.workspace = true\n";
        assert_eq!(package_version(manifest), None);
    }

    #[test]
    fn package_version_declines_a_manifest_without_a_package_section() {
        let manifest = "[workspace]\nmembers = [\"crates/*\"]\n";
        assert_eq!(package_version(manifest), None);
    }

    #[test]
    fn package_version_declines_unparsable_toml() {
        assert_eq!(package_version("this is not toml = = ="), None);
    }
}
