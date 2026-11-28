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
//! else aborts the run rather than being moved. Before the first tag is
//! written, `verify_identity` confirms git has a committer identity and, for
//! signed tags, a signing key to make them with.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
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
    #[error(
        "git cannot create {} in {}:\n{}\n\
         A fresh clone does not inherit the original checkout's local `git config`. \
         Copy user.name, user.email and, for signed tags, gpg.format and \
         user.signingkey across ( `git config --local --get <key>` there, \
         `git config <key> <value>` here ){}.",
        if *.sign { "signed tags" } else { "tags" },
        .root.display(),
        .problems.iter().map(|p| format!("  - {p}")).collect::<Vec<_>>().join("\n"),
        if *.sign { ", or pass `tag --no-sign` for unsigned tags" } else { "" },
    )]
    Identity {
        root: PathBuf,
        sign: bool,
        problems: Vec<String>,
    },
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
    if !to_create.is_empty() {
        verify_identity(&root, sign)?;
    }
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
        // One invocation per tag, as the real run does — a reader who sees a
        // single bulk push here will write one.
        if !args.no_push {
            for t in &to_push {
                println!("$ git push {} {}", args.remote, t.tag);
            }
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

    // One push per tag. A single push carrying more than three tags makes
    // GitHub discard every push event, so the `winterbaume-server-vX.Y.Z` tag
    // would land on the remote without starting the binary release — see
    // `batch::push_batches`.
    let refs: Vec<String> = to_push.iter().map(|t| t.tag.clone()).collect();
    batch::push_refs(&root, None, &refs)?;
    println!();
    println!(
        "{} tag(s) pushed to {}, one push each.",
        refs.len(),
        args.remote
    );
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

/// The signing settings `git tag -s` consults, as resolved for the repository.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct SigningConfig {
    /// `gpg.format`; unset means `openpgp`.
    format: Option<String>,
    /// `user.signingkey`.
    key: Option<String>,
    /// `gpg.ssh.defaultKeyCommand`.
    default_key_command: Option<String>,
}

/// `user.signingkey` holds the key itself rather than a path to it — git's
/// own test ( `is_literal_ssh_key` ) is these two prefixes.
fn is_literal_ssh_key(key: &str) -> bool {
    key.starts_with("key::") || key.starts_with("ssh-")
}

/// Where a path-valued `user.signingkey` points, with `~/` expanded the way
/// git's `--type=path` does and a relative path taken from the repository.
fn resolve_key_path(key: &str, root: &Path, home: Option<&Path>) -> PathBuf {
    match (key.strip_prefix("~/"), home) {
        (Some(rest), Some(h)) => h.join(rest),
        _ => root.join(key),
    }
}

/// Everything that would make `git tag -a` / `git tag -s` fail before it
/// writes a tag, as one human-readable line per problem.
///
/// `ident_failure` is the error `git var GIT_COMMITTER_IDENT` gave, if any.
/// `signing` is `None` for unsigned tags. Only the ssh format is checked in
/// depth: openpgp and x509 fall back to the committer identity to choose a
/// key, so the identity check already covers their configuration.
fn identity_problems(
    ident_failure: Option<&str>,
    signing: Option<&SigningConfig>,
    root: &Path,
    home: Option<&Path>,
    exists: impl Fn(&Path) -> bool,
) -> Vec<String> {
    let mut problems = Vec::new();
    if let Some(why) = ident_failure {
        problems.push(format!("no committer identity: {why}"));
    }
    let Some(cfg) = signing else {
        return problems;
    };
    if cfg.format.as_deref() != Some("ssh") {
        return problems;
    }
    match cfg.key.as_deref() {
        None if cfg.default_key_command.is_none() => problems.push(
            "gpg.format is ssh, but neither user.signingkey nor gpg.ssh.defaultKeyCommand is set"
                .to_string(),
        ),
        None => {}
        Some(k) if is_literal_ssh_key(k) => {}
        Some(k) => {
            let path = resolve_key_path(k, root, home);
            if !exists(&path) {
                problems.push(format!(
                    "user.signingkey names {}, which does not exist",
                    path.display()
                ));
            }
        }
    }
    problems
}

/// The last `fatal:` line of a git failure, which is the one that says what
/// was actually wrong; the advice block above it is the same every time.
fn fatal_line(stderr: &str) -> String {
    stderr
        .lines()
        .rev()
        .find(|l| l.starts_with("fatal:"))
        .or_else(|| stderr.lines().rev().find(|l| !l.trim().is_empty()))
        .unwrap_or("git var GIT_COMMITTER_IDENT failed")
        .trim()
        .to_string()
}

/// Refuse to start when git could not create the tags here: no committer
/// identity, or ( for signed tags ) no usable ssh signing key. A fresh clone
/// has neither when the original checkout keeps them in its local config,
/// which is how the 2026-11-27 tag run died on its first `git tag -s`.
///
/// Probes only: `git var` and `git config --get` write nothing, so the dry
/// run makes the same check and fails the same way `--execute` would.
pub(crate) fn verify_identity(root: &Path, sign: bool) -> Result<(), Error> {
    let ident = match git::committer_ident(root) {
        Ok(ident) => Ok(ident),
        Err(git::Error::GitFailed { stderr, .. }) => Err(fatal_line(&stderr)),
        Err(e) => return Err(e.into()),
    };
    let signing = if sign {
        Some(SigningConfig {
            format: git::config_get(root, "gpg.format")?,
            key: git::config_get(root, "user.signingkey")?,
            default_key_command: git::config_get(root, "gpg.ssh.defaultKeyCommand")?,
        })
    } else {
        None
    };
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let problems = identity_problems(
        ident.as_ref().err().map(String::as_str),
        signing.as_ref(),
        root,
        home.as_deref(),
        Path::exists,
    );
    if !problems.is_empty() {
        return Err(Error::Identity {
            root: root.to_path_buf(),
            sign,
            problems,
        });
    }
    let ident = ident.unwrap_or_default();
    // Name only: the address is the operator's business, not the log's.
    let name = ident.split(" <").next().unwrap_or(&ident);
    println!(
        "git identity: {name}{}",
        match signing.as_ref().and_then(|c| c.format.as_deref()) {
            _ if !sign => ", unsigned".to_string(),
            Some(f) => format!(", signing with {f}"),
            None => ", signing with openpgp".to_string(),
        }
    );
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

    fn ssh(key: Option<&str>, command: Option<&str>) -> SigningConfig {
        SigningConfig {
            format: Some("ssh".to_string()),
            key: key.map(str::to_string),
            default_key_command: command.map(str::to_string),
        }
    }

    fn problems(ident: Option<&str>, signing: Option<&SigningConfig>) -> Vec<String> {
        identity_problems(
            ident,
            signing,
            Path::new("/repo"),
            Some(Path::new("/home/op")),
            |p| p == Path::new("/home/op/.ssh/id_ed25519.pub"),
        )
    }

    #[test]
    fn a_configured_checkout_has_no_problems() {
        let cfg = ssh(Some("~/.ssh/id_ed25519.pub"), None);
        assert!(problems(None, Some(&cfg)).is_empty());
    }

    #[test]
    fn a_missing_identity_is_reported_even_unsigned() {
        // The 2026-11-27 failure: a fresh clone with no user.email.
        let why = "fatal: unable to auto-detect email address (got 'op@host.(none)')";
        let found = problems(Some(why), None);
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("unable to auto-detect email address"));
    }

    #[test]
    fn ssh_signing_without_any_key_is_reported() {
        let found = problems(None, Some(&ssh(None, None)));
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("user.signingkey"));
    }

    #[test]
    fn a_default_key_command_stands_in_for_the_key() {
        let cfg = ssh(None, Some("ssh-add -L"));
        assert!(problems(None, Some(&cfg)).is_empty());
    }

    #[test]
    fn a_literal_ssh_key_is_not_treated_as_a_path() {
        for key in [
            "ssh-ed25519 AAAAC3Nza op@host",
            "key::ssh-ed25519 AAAAC3Nza",
        ] {
            assert!(
                problems(None, Some(&ssh(Some(key), None))).is_empty(),
                "{key}"
            );
        }
    }

    #[test]
    fn a_key_path_that_does_not_exist_is_reported() {
        let found = problems(None, Some(&ssh(Some("~/.ssh/missing.pub"), None)));
        assert_eq!(found.len(), 1);
        assert!(
            found[0].contains("/home/op/.ssh/missing.pub"),
            "{}",
            found[0]
        );
    }

    #[test]
    fn a_relative_key_path_resolves_against_the_repository() {
        assert_eq!(
            resolve_key_path(
                "keys/k.pub",
                Path::new("/repo"),
                Some(Path::new("/home/op"))
            ),
            PathBuf::from("/repo/keys/k.pub")
        );
    }

    #[test]
    fn openpgp_signing_relies_on_the_identity_check() {
        let cfg = SigningConfig::default();
        assert!(problems(None, Some(&cfg)).is_empty());
    }

    #[test]
    fn both_problems_are_reported_together() {
        let found = problems(Some("fatal: empty ident name"), Some(&ssh(None, None)));
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn fatal_line_drops_the_advice_block() {
        let stderr = "Committer identity unknown\n\n*** Please tell me who you are.\n\n\
                      Run\n\n  git config --global user.email \"you@example.com\"\n\n\
                      fatal: unable to auto-detect email address (got 'op@host.(none)')\n";
        assert_eq!(
            fatal_line(stderr),
            "fatal: unable to auto-detect email address (got 'op@host.(none)')"
        );
    }

    #[test]
    fn package_version_declines_unparsable_toml() {
        assert_eq!(package_version("this is not toml = = ="), None);
    }
}
