//! Stage 2 — invoke the `generate-changelog` skill's helper scripts to draft
//! per-crate CHANGELOG.md entries and the root umbrella CHANGELOG.md from the
//! plan.
//!
//! The mechanical bullet-extraction lives in the skill at
//! `.agents/skills/generate-changelog/scripts/`. For every non-skip plan
//! entry, this module:
//!
//! 1. shells out to `draft_section.py` to get a mechanical draft;
//! 2. feeds that draft + the skill's editorial rules to an AI coding CLI for
//!    the editorial polish — see `polisher` module for the supported
//!    backends (claude, codex, copilot, cursor);
//! 3. prepends the polished section to the crate's `CHANGELOG.md`.
//!
//! At least one supported AI CLI on PATH is a hard requirement — the polished
//! output is what the release expects to ship. Operators who want the raw
//! mechanical draft can invoke `draft_section.py` directly. The harness owns
//! all file IO so the model only ever generates text.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use chrono::Utc;

use crate::ChangelogArgs;
use crate::metadata::{CargoExe, cargo_metadata, publishable_members, workspace_root};
use crate::plan::{BumpDecision, Plan};
use crate::polisher;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Metadata(#[from] crate::metadata::Error),
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

fn draft_section(
    root: &Path,
    crate_name: &str,
    next_version: Option<&str>,
    date: &str,
    crate_dir: Option<&Path>,
) -> Result<String, Error> {
    let script = skill_script(root, "draft_section.py");
    let mut args: Vec<String> = vec![
        "--crate".into(),
        crate_name.into(),
        "--repo-root".into(),
        root.display().to_string(),
        "--date".into(),
        date.into(),
    ];
    if let Some(v) = next_version {
        args.push("--next-version".into());
        args.push(v.into());
    }
    if let Some(d) = crate_dir {
        // Relative-to-root pathspec to match the script's expectation.
        let rel = d.strip_prefix(root).unwrap_or(d);
        args.push("--crate-dir".into());
        args.push(rel.display().to_string());
    }
    run_python(&script, args)
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

fn polish_prompt(
    root: &Path,
    crate_name: &str,
    next_version: Option<&str>,
    mechanical_draft: &str,
) -> Result<String, Error> {
    let skill_md = fs::read_to_string(
        root.join(".agents")
            .join("skills")
            .join("generate-changelog")
            .join("SKILL.md"),
    )?;
    let heading_form = match next_version {
        Some(v) => format!("`## v{v} - <YYYY-MM-DD>`"),
        None => "`## Unreleased`".to_string(),
    };
    Ok(format!(
        "You are polishing a CHANGELOG.md section for the crate `{crate_name}`.\n\
         \n\
         A mechanical draft is provided below. Your job is to rewrite it per the \
         editorial rules in the attached skill. Critically:\n\
         - Summarise behaviour, not implementation trivia.\n\
         - Collapse repeated or related bullets into one clear entry.\n\
         - Drop noise that obviously doesn't matter to release consumers \
           ( bulk README regenerations, generated wire churn that doesn't change \
           user-visible behaviour, etc. ).\n\
         - Keep the same top-level heading ({heading_form}) and the same \
           Added / Changed / Fixed / Terraform / Documentation / Tests / Internal \
           category set; omit any category that has no entries after polishing.\n\
         - Use British English in prose ( see the skill ).\n\
         \n\
         Output ONLY the polished markdown section, starting with the `## ` \
         heading and ending with a single trailing blank line. No commentary, \
         no fenced code block around it, no explanation.\n\
         \n\
         --- BEGIN SKILL (`.agents/skills/generate-changelog/SKILL.md`) ---\n\
         {skill_md}\n\
         --- END SKILL ---\n\
         \n\
         --- BEGIN MECHANICAL DRAFT ---\n\
         {mechanical_draft}\
         --- END MECHANICAL DRAFT ---\n",
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
    let backend = polisher::resolve(Some(args.polisher.as_str()), env_override.as_deref())?;
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

        let mechanical = draft_section(&root, &entry.name, Some(next), &date, Some(pkg.dir()))?;
        let prompt = polish_prompt(&root, &entry.name, Some(next), &mechanical)?;
        let section = ensure_trailing_newline(polisher::polish(backend, &root, &prompt)?);

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

fn ensure_trailing_newline(mut s: String) -> String {
    if !s.ends_with('\n') {
        s.push('\n');
    }
    s
}
