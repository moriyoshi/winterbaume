//! Per-crate semver bump planner and selective-publish driver for the
//! winterbaume workspace.
//!
//! Three-stage workflow:
//!
//! 1. `plan` — discover which crates have changed since their last
//!    `<crate>-v<ver>` tag, classify the semver bump (skip / patch / minor /
//!    major / literal), and write `release-plan.toml`.
//! 2. `changelog` — draft per-crate CHANGELOG.md entries and refresh the root
//!    umbrella CHANGELOG.md based on the plan.
//! 3. `publish` — group plan entries by bump level and drive `release-batch`
//!    once per group.

mod batch;
mod changelog;
mod git;
mod metadata;
mod plan;
mod publish;
mod semver_checks;
mod version;

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::metadata::CargoExe;

#[derive(Parser, Debug)]
#[command(
    name = "release-harness",
    about = "Per-crate semver bump planner and selective-publish driver",
    long_about = "Computes per-crate semver bumps from git diffs since each crate's last \
                  release tag, refreshes per-crate and umbrella changelogs, and drives \
                  `release-batch` to publish only the crates that actually changed."
)]
struct Args {
    /// Cargo executable to invoke for `metadata` and `locate-project`.
    /// Defaults to the WB_CARGO environment variable, then `cargo` on PATH.
    #[arg(long, global = true)]
    cargo: Option<OsString>,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Discover changed crates, classify per-crate semver bump, write release-plan.toml.
    Plan(PlanArgs),
    /// Draft per-crate CHANGELOG.md entries for each non-skip crate in the plan.
    Changelog(ChangelogArgs),
    /// Run a chunked `cargo release` per bump-level group from a plan file.
    Publish(PublishArgs),
    /// Run a single chunked `cargo release` directly, without a plan file.
    /// Equivalent to the former standalone `release-batch` binary; useful for
    /// first-launch (no prior tags) or targeted retries.
    Batch(BatchArgs),
}

#[derive(Parser, Debug)]
struct PlanArgs {
    /// Output path for the plan file.
    #[arg(long, default_value = "release-plan.toml")]
    out: PathBuf,

    /// Optional overrides file (TOML). Per-crate `bumps.<name> = "skip" | "patch"
    /// | "minor" | "major" | "<literal-semver>"` entries always win over the
    /// heuristic + semver-checks classification.
    #[arg(long, default_value = "release-plan-overrides.toml")]
    overrides: PathBuf,

    /// Skip running cargo-semver-checks even if it is installed. Use when you
    /// want a fast heuristic-only plan and accept that breaking changes may be
    /// undercounted.
    #[arg(long)]
    skip_semver_checks: bool,

    /// Override the implicit `HEAD` reference used for the right-hand side of
    /// per-crate diffs. Useful for replaying a plan against a historical tip.
    #[arg(long, default_value = "HEAD")]
    head: String,
}

#[derive(Parser, Debug)]
struct ChangelogArgs {
    /// Path to the plan file written by `plan`.
    #[arg(long, default_value = "release-plan.toml")]
    plan: PathBuf,

    /// Override the release date in `## v<version> - <YYYY-MM-DD>` headings.
    /// Defaults to today's date in UTC.
    #[arg(long)]
    date: Option<String>,

    /// Polish each mechanical draft through the `claude` CLI before writing.
    /// Requires `claude` on PATH; falls back to the mechanical draft per
    /// crate (with a warning) when it isn't available.
    #[arg(long)]
    polish: bool,
}

#[derive(Parser, Debug)]
struct PublishArgs {
    /// Path to the plan file written by `plan`.
    #[arg(long, default_value = "release-plan.toml")]
    plan: PathBuf,

    /// Without this, print the planned chunked-publish invocations and exit
    /// without running anything.
    #[arg(long)]
    execute: bool,

    /// Pass --sign to `cargo release` (GPG-signed commits and tags).
    #[arg(long)]
    sign: bool,

    /// Pass --no-confirm to `cargo release` (unattended runs).
    #[arg(long)]
    no_confirm: bool,

    /// Maximum new crates per `cargo release` invocation.
    #[arg(long, default_value_t = 5)]
    chunk_size: usize,

    /// Seconds to sleep between chunks (default: 600s rate window + 60s buffer).
    #[arg(long, default_value_t = 660)]
    sleep: u64,

    /// Maximum consecutive 429 retries per chunk without forward progress.
    #[arg(long, default_value_t = 3)]
    max_retries: u32,

    /// Extra seconds added to the "try again after" deadline before retrying.
    #[arg(long, default_value_t = 30)]
    retry_buffer: u64,

    /// Skip the crates.io existence check for the target version (resumability).
    #[arg(long)]
    skip_version_check: bool,
}

#[derive(Parser, Debug)]
pub struct BatchArgs {
    /// Version (e.g. 0.1.0) or level (patch/minor/major/release/alpha/beta/rc)
    /// passed verbatim as `cargo release`'s first positional argument.
    #[arg(long)]
    pub version: String,

    /// Restrict the release to the listed crate names. Without this flag,
    /// every publishable workspace member is in scope. Mutually exclusive
    /// with `--crates-file`.
    #[arg(long, num_args = 1.., conflicts_with = "crates_file")]
    pub crates: Vec<String>,

    /// Path to a newline-delimited file listing crate names to release.
    /// Empty lines and lines beginning with `#` are ignored. Mutually
    /// exclusive with `--crates`.
    #[arg(long, conflicts_with = "crates")]
    pub crates_file: Option<PathBuf>,

    /// Actually run `cargo release --execute`. Without this flag, prints the plan only.
    #[arg(long)]
    pub execute: bool,

    /// Pass --sign to `cargo release` (GPG-signed commits and tags).
    #[arg(long)]
    pub sign: bool,

    /// Pass --no-confirm to `cargo release` (unattended runs).
    #[arg(long)]
    pub no_confirm: bool,

    /// Maximum new crates per `cargo release` invocation.
    #[arg(long, default_value_t = 5)]
    pub chunk_size: usize,

    /// Seconds to sleep between chunks (default: 600s rate window + 60s buffer).
    #[arg(long, default_value_t = 660)]
    pub sleep: u64,

    /// Maximum consecutive 429 retries per chunk without forward progress.
    #[arg(long, default_value_t = 3)]
    pub max_retries: u32,

    /// Extra seconds added to the "try again after" deadline before retrying.
    #[arg(long, default_value_t = 30)]
    pub retry_buffer: u64,

    /// Do not query crates.io to skip already-published versions.
    #[arg(long)]
    pub skip_version_check: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let cargo = CargoExe::resolve(args.cargo.as_ref());
    let result: Result<ExitCode, Box<dyn std::error::Error>> = match args.cmd {
        Cmd::Plan(p) => plan::run(&cargo, &p).map_err(Into::into),
        Cmd::Changelog(c) => changelog::run(&cargo, &c).map_err(Into::into),
        Cmd::Publish(p) => publish::run(&cargo, &p).map_err(Into::into),
        Cmd::Batch(b) => publish::run_batch(&cargo, &b).map_err(Into::into),
    };
    match result {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
