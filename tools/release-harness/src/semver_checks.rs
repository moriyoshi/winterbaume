//! Authoritative breaking-vs-additive classification via `cargo-semver-checks`.
//!
//! The harness invokes `cargo semver-checks check-release -p <name>
//! --release-type <candidate>` for each candidate ≥ patch and treats a non-zero
//! exit (with the "breaking" report token in stderr) as the signal to escalate
//! the candidate to `major`. If the tool is not installed, callers fall back
//! to the heuristic-only classification with a printed warning — we never
//! silently downgrade.

use std::path::Path;
use std::process::Command;

use crate::metadata::CargoExe;
use crate::version::Level;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// cargo-semver-checks reported no breaking changes at the candidate level.
    NoBreaking,
    /// cargo-semver-checks reported at least one breaking lint.
    Breaking,
    /// cargo-semver-checks is not installed on PATH or did not produce a
    /// recognisable verdict; the caller should fall back to heuristics and
    /// warn the operator.
    Unavailable,
    /// The candidate is `patch`; cargo-semver-checks would either no-op or
    /// emit warnings that don't map to a bump escalation.
    NotApplicable,
}

pub fn available(cargo: &CargoExe) -> bool {
    Command::new(cargo.path())
        .args(["semver-checks", "--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run `cargo semver-checks check-release` for one package and return whether
/// it reports a breaking change at `candidate`.
pub fn check(
    cargo: &CargoExe,
    root: &Path,
    package: &str,
    candidate: Level,
) -> Result<Outcome, Error> {
    if candidate == Level::Patch {
        return Ok(Outcome::NotApplicable);
    }
    let release_type = match candidate {
        Level::Patch => "patch",
        Level::Minor => "minor",
        Level::Major => "major",
    };
    let out = Command::new(cargo.path())
        .args([
            "semver-checks",
            "check-release",
            "-p",
            package,
            "--release-type",
            release_type,
            "--color",
            "never",
        ])
        .current_dir(root)
        .output()?;

    // cargo-semver-checks returns:
    //   - exit 0 when the candidate is acceptable (no breaking at candidate level)
    //   - exit non-zero when at least one breaking lint fires
    //   - exit non-zero for tool-internal errors (missing baseline, etc.) too
    //
    // Distinguish "breaking lint fired" from "tool errored" by looking for the
    // verdict marker line that cargo-semver-checks always emits when it has
    // formed an opinion. Older versions write "Summary semver requires" or
    // "FAIL"; newer ones write "FAIL  on bump". Be liberal in what we accept.
    if out.status.success() {
        return Ok(Outcome::NoBreaking);
    }
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let combined = format!("{stderr}{stdout}");
    let has_verdict = combined.contains("FAIL")
        || combined.contains("semver requires")
        || combined.contains("breaking change")
        || combined.contains("required bump");
    if has_verdict {
        Ok(Outcome::Breaking)
    } else {
        eprintln!(
            "warn: cargo-semver-checks for {package} produced no verdict (status {}); \
             treating as unavailable. stderr tail: {}",
            out.status,
            stderr.lines().rev().take(3).collect::<Vec<_>>().join(" | "),
        );
        Ok(Outcome::Unavailable)
    }
}
