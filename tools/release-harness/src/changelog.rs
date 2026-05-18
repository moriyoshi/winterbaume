//! Stage 2 — draft per-crate CHANGELOG.md entries and refresh the root
//! umbrella CHANGELOG.md from the plan.
//!
//! The harness writes a *draft* `## v<next> - <date>` section per crate built
//! from `git log <last-tag>..HEAD -- crates/<name>/`, categorised by
//! conventional-commit-style prefixes (`feat:`, `fix:`, `chore:`, ...). The
//! goal is to give the operator a structured starting point that mirrors the
//! `generate-changelog` skill's section conventions; the operator (or the
//! skill, invoked by an agent) polishes the wording before stage 3.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use chrono::Utc;

use crate::ChangelogArgs;
use crate::git;
use crate::metadata::{CargoExe, cargo_metadata, publishable_members, workspace_root};
use crate::plan::{BumpDecision, CrateEntry, Plan};

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
}

/// Section bins, ordered the way they appear in the rendered changelog.
const SECTIONS: &[(&str, &[&str])] = &[
    ("Added", &["feat", "add"]),
    ("Changed", &["change", "refactor", "perf", "improve"]),
    ("Fixed", &["fix", "bug"]),
    ("Terraform", &["terraform", "tf"]),
    ("Documentation", &["docs", "doc"]),
    ("Tests", &["test", "tests"]),
    ("Internal", &["chore", "build", "ci", "release"]),
];

fn classify_commit(subject: &str) -> &'static str {
    // Strip leading `<scope>: ` or `<scope>(<area>): ` if present.
    let head = subject.split(':').next().unwrap_or(subject).trim();
    let scope = head.split('(').next().unwrap_or(head).trim().to_lowercase();
    for (label, prefixes) in SECTIONS {
        if prefixes.iter().any(|p| scope == *p) {
            return label;
        }
    }
    // No conventional prefix matched. Bucket by content heuristics.
    let lower = subject.to_lowercase();
    if lower.starts_with("fix") || lower.contains(" fix ") {
        "Fixed"
    } else if lower.contains("doc") {
        "Documentation"
    } else if lower.contains("test") {
        "Tests"
    } else {
        "Changed"
    }
}

fn render_crate_section(entry: &CrateEntry, commits: &[git::Commit], date: &str) -> String {
    let next = entry.next.clone().unwrap_or_else(|| entry.current.clone());
    let mut bins: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
    for c in commits {
        // Skip pure release-metadata commits per the skill convention.
        if c.subject.starts_with("chore: release ") {
            continue;
        }
        bins.entry(classify_commit(&c.subject))
            .or_default()
            .push(format!("{} ({})", c.subject, c.hash_short));
    }

    let mut out = String::new();
    out.push_str(&format!("## v{next} - {date}\n\n"));
    if bins.is_empty() {
        out.push_str("- _No commits attributed to this crate since the last release._\n\n");
        return out;
    }
    for (label, _) in SECTIONS {
        let Some(items) = bins.remove(*label) else {
            continue;
        };
        if items.is_empty() {
            continue;
        }
        out.push_str(&format!("### {label}\n\n"));
        for item in items {
            out.push_str(&format!("- {item}\n"));
        }
        out.push('\n');
    }
    // Anything not in the canonical list (shouldn't happen, but defensive).
    for (label, items) in bins {
        out.push_str(&format!("### {label}\n\n"));
        for item in items {
            out.push_str(&format!("- {item}\n"));
        }
        out.push('\n');
    }
    out
}

fn upsert_crate_changelog(path: &Path, section: &str) -> Result<(), Error> {
    let existing = fs::read_to_string(path).unwrap_or_default();
    let header = "# Changelog\n\n";
    let body = if existing.is_empty() {
        format!("{header}{section}")
    } else if let Some(rest) = existing.strip_prefix(header) {
        format!("{header}{section}{rest}")
    } else {
        // File doesn't start with the canonical header — preserve it and
        // prepend the section after the first line break we find, or at the
        // top if there isn't one.
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

fn render_umbrella_section(date: &str, entries: &[(&CrateEntry, String)]) -> String {
    let mut out = format!("## {date}\n\n");
    for (entry, link) in entries {
        let version = entry.next.clone().unwrap_or_else(|| entry.current.clone());
        let label = match entry.bump {
            BumpDecision::Initial => " (initial release)",
            BumpDecision::Pinned => " (pinned)",
            _ => "",
        };
        out.push_str(&format!(
            "- `{}` v{}{}: see [`{}`]({}).\n",
            entry.name, version, label, link, link
        ));
    }
    out.push('\n');
    out
}

fn upsert_umbrella_changelog(path: &Path, section: &str) -> Result<(), Error> {
    let existing = fs::read_to_string(path).unwrap_or_default();
    let header_block = "# Changelog\n\nThis file summarises notable winterbaume releases. \
                        Detailed crate-level notes live in each crate's `CHANGELOG.md`.\n\n";

    let body = if existing.is_empty() {
        format!("{header_block}{section}")
    } else if let Some(idx) = existing.find("\n## ") {
        // Insert immediately before the first existing dated section.
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
    let pkg_by_name: BTreeMap<&str, &crate::metadata::Package> =
        members.iter().map(|p| (p.name.as_str(), *p)).collect();

    let date = args
        .date
        .clone()
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    let mut umbrella_entries: Vec<(&CrateEntry, String)> = Vec::new();
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
        let pathspec = format!(
            "{}/",
            pkg.dir()
                .strip_prefix(&root)
                .unwrap_or_else(|_| pkg.dir())
                .display()
        );
        let from_ref = entry.last_tag.clone().unwrap_or_else(|| "HEAD".to_string());
        let commits = if entry.last_tag.is_some() {
            git::commits(&root, &from_ref, "HEAD", &pathspec)?
        } else {
            // No prior tag — emit a placeholder section without commit dump.
            Vec::new()
        };
        let section = render_crate_section(entry, &commits, &date);
        let cl_path = pkg.dir().join("CHANGELOG.md");
        upsert_crate_changelog(&cl_path, &section)?;
        touched += 1;
        let rel = cl_path
            .strip_prefix(&root)
            .unwrap_or(&cl_path)
            .display()
            .to_string();
        umbrella_entries.push((entry, rel));
    }

    if !umbrella_entries.is_empty() {
        let umbrella_section = render_umbrella_section(&date, &umbrella_entries);
        upsert_umbrella_changelog(&root.join("CHANGELOG.md"), &umbrella_section)?;
    }

    println!(
        "drafted {touched} per-crate CHANGELOG section(s) for date {date}; root CHANGELOG.md \
         updated."
    );
    Ok(ExitCode::SUCCESS)
}
