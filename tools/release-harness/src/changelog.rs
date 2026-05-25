//! Stage 2 — generate per-crate CHANGELOG.md entries and the root umbrella
//! CHANGELOG.md from the plan, via an AI coding CLI for editorial polish.
//!
//! For every non-skip plan entry, this module:
//!
//! 1. assembles a prompt that contains the plan entry verbatim (crate name,
//!    current/next version, bump kind, reason, files changed) plus the raw
//!    `git log` between the crate's last tag and HEAD, restricted to the
//!    crate's directory;
//! 2. sends the prompt to an AI coding CLI for editorial polish — see
//!    `polisher` module for the supported backends (claude, codex, copilot,
//!    cursor);
//! 3. prepends the polished section to the crate's `CHANGELOG.md`.
//!
//! The skill at `.agents/skills/generate-changelog/` still ships
//! `scripts/draft_section.py` for the case where a human or an agent
//! wants the mechanical bucketed bullets directly without the polish pass.
//! The harness deliberately skips that step — feeding the polisher
//! pre-bucketed bullets baked a noisy categorisation in that the polisher
//! then had to undo. The plan entry's `files_changed` list is the
//! authoritative scope; the polisher uses it to drop cross-crate noise.
//!
//! At least one supported AI CLI on PATH is a hard requirement. The harness
//! owns all file IO so the model only ever generates text.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use chrono::Utc;

use crate::ChangelogArgs;
use crate::git;
use crate::metadata::{CargoExe, cargo_metadata, publishable_members, workspace_root};
use crate::plan::{BumpDecision, CrateEntry, Plan};
use crate::polisher;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Metadata(#[from] crate::metadata::Error),
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
    #[error("crate {0} listed in plan but not present in workspace")]
    MissingPackage(String),
    #[error("{script} failed with status {status}:\n{stderr}")]
    Script {
        script: String,
        status: String,
        stderr: String,
    },
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Polisher(#[from] polisher::Error),
}

/// Resolve the absolute path to a skill script. We prefer
/// `<repo-root>/.agents/skills/generate-changelog/scripts/<name>`.
fn skill_script(root: &Path, name: &str) -> PathBuf {
    root.join(".agents")
        .join("skills")
        .join("generate-changelog")
        .join("scripts")
        .join(name)
}

fn run_python<I, S>(script: &Path, args: I) -> Result<String, Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let out = Command::new("python3").arg(script).args(args).output()?;
    if !out.status.success() {
        return Err(Error::Script {
            script: script.display().to_string(),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8(out.stdout)?)
}

/// Build a single dated umbrella section covering every published crate in
/// the plan. `note` lets us tag `initial release` / `pinned` qualifiers.
fn draft_umbrella(
    root: &Path,
    date: &str,
    entries: &[(String, String, &'static str)],
) -> Result<String, Error> {
    if entries.is_empty() {
        return Ok(String::new());
    }
    let script = skill_script(root, "draft_umbrella_entry.py");
    let mut args: Vec<String> = vec!["--date".into(), date.into()];
    for (name, version, note) in entries {
        let value = if note.is_empty() {
            format!("{name}:{version}")
        } else {
            format!("{name}:{version}:{note}")
        };
        args.push("--entry".into());
        args.push(value);
    }
    run_python(&script, args)
}

/// Assemble the polish prompt for one crate. The polisher receives the plan
/// entry verbatim plus the raw git log between `entry.last_tag` and HEAD —
/// structured "what bumped, why" facts, no pre-bucketed bullets to argue with.
fn polish_prompt(
    root: &Path,
    entry: &CrateEntry,
    crate_dir: &Path,
    next_version: &str,
    date: &str,
) -> Result<String, Error> {
    let skill_md = fs::read_to_string(
        root.join(".agents")
            .join("skills")
            .join("generate-changelog")
            .join("SKILL.md"),
    )?;

    let pathspec = {
        let rel = crate_dir.strip_prefix(root).unwrap_or(crate_dir);
        let s = rel.display().to_string();
        if s.is_empty() {
            // Umbrella crate sits at the workspace root; `strip_prefix`
            // yields an empty path. `git log -- /` is rejected as outside
            // the repo, so scope to the whole working tree via `.` (the
            // `run_git` cwd is already `root`).
            ".".to_string()
        } else {
            format!("{s}/")
        }
    };
    let from_ref = entry.last_tag.clone().unwrap_or_else(|| "HEAD".to_string());
    let commits = if entry.last_tag.is_some() {
        git::commits(root, &from_ref, "HEAD", &pathspec)?
    } else {
        Vec::new()
    };

    let bump_label = match entry.bump {
        BumpDecision::Patch => "patch",
        BumpDecision::Minor => "minor",
        BumpDecision::Major => "major",
        BumpDecision::Pinned => "pinned",
        BumpDecision::Initial => "initial",
        BumpDecision::Skip | BumpDecision::Unchanged => "skip",
    };

    let last_tag_line = entry
        .last_tag
        .as_deref()
        .map(|t| format!("last_tag: {t}"))
        .unwrap_or_else(|| "last_tag: (none — first release of this crate)".to_string());

    let files_block = if entry.files_changed.is_empty() {
        "(none recorded — see commit log below)".to_string()
    } else {
        entry
            .files_changed
            .iter()
            .map(|p| format!("- {p}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let commits_block = if commits.is_empty() {
        "(no commits in range — likely the initial release)".to_string()
    } else {
        commits
            .iter()
            .map(|c| format!("- {} {} {}", c.hash_short, c.date, c.subject))
            .collect::<Vec<_>>()
            .join("\n")
    };

    Ok(format!(
        "You are writing a CHANGELOG.md section for the crate `{name}`.\n\
         \n\
         CRITICAL OUTPUT FORMAT:\n\
         - Your entire response is ONE markdown section.\n\
         - Start with the heading `## v{next_version} - {date}`.\n\
         - End after the last bullet of the last category.\n\
         - DO NOT echo the inputs back, append a comparison, or explain.\n\
         - DO NOT wrap the response in a fenced code block.\n\
         - DO NOT include commit hashes in bullets.\n\
         \n\
         Use the structured plan entry and raw git log below to write the \
         section. The editorial rules in the attached skill apply:\n\
         - Summarise behaviour, not implementation trivia.\n\
         - Collapse repeated or related commits into one clear bullet.\n\
         - Drop noise that doesn't matter to release consumers \
           ( bulk README regenerations, generated wire churn that doesn't \
           change user-visible behaviour, commits from other crates that \
           only touched a shared file, etc. ). The `files_changed` list is \
           the authoritative scope — if a commit's subject mentions another \
           crate but only the shared file appears in `files_changed`, it \
           probably doesn't belong in this crate's changelog.\n\
         - Bucket bullets into the canonical category set: \
           Added / Changed / Fixed / Terraform / Documentation / Tests / Internal. \
           Omit any category that has no entries.\n\
         - Use British English in prose ( see the skill ).\n\
         - If nothing user-visible changed for this crate ( e.g. only \
           cross-crate noise touched its README ), emit a single `### Internal` \
           bullet acknowledging the maintenance release.\n\
         \n\
         --- BEGIN SKILL (`.agents/skills/generate-changelog/SKILL.md`) ---\n\
         {skill_md}\n\
         --- END SKILL ---\n\
         \n\
         --- BEGIN PLAN ENTRY ---\n\
         crate: {name}\n\
         current_version: {current}\n\
         next_version: {next_version}\n\
         bump: {bump_label}\n\
         {last_tag_line}\n\
         reason: {reason}\n\
         files_changed:\n\
         {files_block}\n\
         --- END PLAN ENTRY ---\n\
         \n\
         --- BEGIN GIT LOG ({from_ref}..HEAD, restricted to {pathspec}) ---\n\
         {commits_block}\n\
         --- END GIT LOG ---\n",
        name = entry.name,
        current = entry.current,
        reason = entry.reason,
    ))
}

fn upsert_crate_changelog(path: &Path, section: &str) -> Result<(), Error> {
    let existing = fs::read_to_string(path).unwrap_or_default();
    let header = "# Changelog\n\n";
    let body = if existing.is_empty() {
        format!("{header}{section}")
    } else if let Some(rest) = existing.strip_prefix(header) {
        format!("{header}{section}{rest}")
    } else {
        match existing.find("\n\n") {
            Some(idx) => format!(
                "{}{}{}",
                &existing[..idx + 2],
                section,
                &existing[idx + 2..]
            ),
            None => format!("{section}\n{existing}"),
        }
    };
    fs::write(path, body)?;
    Ok(())
}

fn upsert_umbrella_changelog(path: &Path, section: &str) -> Result<(), Error> {
    let existing = fs::read_to_string(path).unwrap_or_default();
    let header_block = "# Changelog\n\nThis file summarises notable winterbaume releases. \
                        Detailed crate-level notes live in each crate's `CHANGELOG.md`.\n\n";

    let body = if existing.is_empty() {
        format!("{header_block}{section}")
    } else if let Some(idx) = existing.find("\n## ") {
        format!("{}\n{}{}", &existing[..idx], section, &existing[idx + 1..])
    } else {
        format!("{existing}\n{section}")
    };
    fs::write(path, body)?;
    Ok(())
}

pub fn run(cargo: &CargoExe, args: &ChangelogArgs) -> Result<ExitCode, Error> {
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
    let pkg_by_name: std::collections::BTreeMap<&str, &crate::metadata::Package> =
        members.iter().map(|p| (p.name.as_str(), *p)).collect();

    let env_override = std::env::var("WB_RELEASE_POLISHER").ok();
    let backend = polisher::resolve(
        args.polisher.as_ref().map(|v| v.as_ref()),
        env_override.as_deref(),
    )?;
    eprintln!("polisher: {} ({})", backend.label(), backend.binary());

    let date = args
        .date
        .clone()
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    let mut umbrella_entries: Vec<(String, String, &'static str)> = Vec::new();
    let mut touched = 0;

    for entry in &plan.crates {
        let bump_publishes = matches!(
            entry.bump,
            BumpDecision::Patch
                | BumpDecision::Minor
                | BumpDecision::Major
                | BumpDecision::Pinned
                | BumpDecision::Initial
        );
        if !bump_publishes {
            continue;
        }
        let pkg = pkg_by_name
            .get(entry.name.as_str())
            .ok_or_else(|| Error::MissingPackage(entry.name.clone()))?;
        let next = entry.next.as_deref().unwrap_or(entry.current.as_str());

        let prompt = polish_prompt(&root, entry, pkg.dir(), next, &date)?;
        // polisher::polish already truncates at the second `## ` heading and
        // ensures a single trailing blank line, so the prepend is clean.
        let section = polisher::polish(backend, &root, &prompt)?;

        let cl_path = pkg.dir().join("CHANGELOG.md");
        upsert_crate_changelog(&cl_path, &section)?;
        touched += 1;
        let note: &'static str = match entry.bump {
            BumpDecision::Initial => "initial release",
            BumpDecision::Pinned => "pinned",
            _ => "",
        };
        umbrella_entries.push((entry.name.clone(), next.to_string(), note));
    }

    if !umbrella_entries.is_empty() {
        let umbrella_section = draft_umbrella(&root, &date, &umbrella_entries)?;
        if !umbrella_section.is_empty() {
            upsert_umbrella_changelog(&root.join("CHANGELOG.md"), &umbrella_section)?;
        }
    }

    println!(
        "wrote {touched} polished per-crate CHANGELOG section(s) for date {date} \
         via {}; root CHANGELOG.md updated.",
        backend.label(),
    );
    Ok(ExitCode::SUCCESS)
}
