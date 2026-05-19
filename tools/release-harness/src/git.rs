//! Thin wrappers around the git CLI for the harness's needs: per-crate tag
//! lookup, file-set diffs since a tag, and commit-log harvesting.

use std::path::Path;
use std::process::Command;

use crate::version::Version;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("git {cmd:?} failed (status {status}): {stderr}")]
    GitFailed {
        cmd: String,
        status: String,
        stderr: String,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

fn run_git(root: &Path, args: &[&str]) -> Result<String, Error> {
    let out = Command::new("git").args(args).current_dir(root).output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: format!("git {}", args.join(" ")),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8(out.stdout)?)
}

/// Latest `<package>-v<X.Y.Z>` tag, picked by version-aware sort over the
/// matching ref namespace.
///
/// We sort by `*authordate` (with `version:refname` as a tiebreaker would be
/// ideal but git's sort key set doesn't include it for tag names with
/// arbitrary prefixes). For our v0.x.y reality this is fine: the latest tag
/// chronologically is the latest version.
pub fn latest_release_tag(root: &Path, package: &str) -> Result<Option<(String, Version)>, Error> {
    let pattern = format!("refs/tags/{package}-v*");
    let raw = run_git(
        root,
        &[
            "for-each-ref",
            "--sort=-creatordate",
            "--format=%(refname:short)",
            &pattern,
        ],
    )?;
    let prefix = format!("{package}-v");
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some(rest) = line.strip_prefix(&prefix) else {
            continue;
        };
        if let Ok(v) = rest.parse::<Version>() {
            return Ok(Some((line.to_string(), v)));
        }
    }
    Ok(None)
}

/// `git diff --name-only <from>..<to> -- <pathspec>` returning one path per
/// line, relative to the repo root.
pub fn diff_paths(root: &Path, from: &str, to: &str, pathspec: &str) -> Result<Vec<String>, Error> {
    let range = format!("{from}..{to}");
    let raw = run_git(root, &["diff", "--name-only", &range, "--", pathspec])?;
    Ok(raw
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect())
}

/// Lines from `git diff <from>..<to> -- <pathspec>` that start with `+pub `
/// followed by one of the public-symbol introducer keywords. Returns the
/// trimmed introducer plus the symbol name when recognisable.
///
/// Used as the cheap "additive public surface" heuristic: any new public
/// item raises the candidate bump from `patch` to `minor`.
pub fn added_pub_symbols(
    root: &Path,
    from: &str,
    to: &str,
    pathspec: &str,
) -> Result<Vec<String>, Error> {
    let range = format!("{from}..{to}");
    let raw = run_git(root, &["diff", "--unified=0", &range, "--", pathspec])?;
    let mut out = Vec::new();
    for line in raw.lines() {
        // Skip the `+++ b/...` file header lines that also start with `+`.
        if !line.starts_with('+') || line.starts_with("+++") {
            continue;
        }
        let body = line[1..].trim_start();
        let Some(rest) = body.strip_prefix("pub ") else {
            continue;
        };
        // Strip `pub(crate)` / `pub(super)` etc. — those aren't public surface.
        if body.starts_with("pub(") {
            continue;
        }
        for kw in [
            "fn ",
            "struct ",
            "enum ",
            "trait ",
            "mod ",
            "const ",
            "static ",
            "type ",
            "use ",
            "async fn ",
            "unsafe fn ",
        ] {
            if rest.starts_with(kw) {
                let snippet = body
                    .split('{')
                    .next()
                    .unwrap_or(body)
                    .trim_end_matches([';', ',', '(', ' '])
                    .to_string();
                out.push(snippet);
                break;
            }
        }
    }
    Ok(out)
}

#[derive(Clone, Debug)]
pub struct Commit {
    pub hash_short: String,
    pub date: String,
    pub subject: String,
}

/// Per-crate commit log between two refs, restricted to files under `pathspec`.
pub fn commits(root: &Path, from: &str, to: &str, pathspec: &str) -> Result<Vec<Commit>, Error> {
    let range = format!("{from}..{to}");
    let raw = run_git(
        root,
        &[
            "log",
            "--first-parent",
            "--date=short",
            "--format=%h%x09%ad%x09%s",
            &range,
            "--",
            pathspec,
        ],
    )?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let mut parts = line.splitn(3, '\t');
        let (Some(h), Some(d), Some(s)) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        out.push(Commit {
            hash_short: h.to_string(),
            date: d.to_string(),
            subject: s.to_string(),
        });
    }
    Ok(out)
}

/// Resolve `HEAD` (or any rev) to its short hash. Used in plan metadata so a
/// downstream replay can pin against the same tip.
pub fn rev_short(root: &Path, rev: &str) -> Result<String, Error> {
    let raw = run_git(root, &["rev-parse", "--short", rev])?;
    Ok(raw.trim().to_string())
}

/// `git merge-base --is-ancestor`. Returns true when `ancestor` is reachable
/// from `descendant` (i.e. `ancestor` was made first), false otherwise.
/// Distinct from the generic `GitFailed` branch: a clean status 1 is the "no"
/// answer, not an error.
pub fn is_ancestor(root: &Path, ancestor: &str, descendant: &str) -> Result<bool, Error> {
    let out = Command::new("git")
        .args(["merge-base", "--is-ancestor", ancestor, descendant])
        .current_dir(root)
        .output()?;
    match out.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(Error::GitFailed {
            cmd: format!("git merge-base --is-ancestor {ancestor} {descendant}"),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        }),
    }
}
