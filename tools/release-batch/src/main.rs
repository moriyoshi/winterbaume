//! Chunked cargo-release driver for the crates.io publish_new rate limit.
//!
//! Cargo rejects a multi-crate batch upfront when the planned new-crate count
//! exceeds the account's publish_new quota (default: 5 per 10 minutes). This
//! tool splits a workspace release into successive cargo-release invocations
//! of at most --chunk-size crates each, sleeping --sleep seconds between
//! invocations so the rate window slides.

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::env;
use std::ffi::OsString;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use clap::Parser;
use serde::Deserialize;

/// Environment variable that overrides the cargo executable used by the driver.
/// Falls back to `--cargo`, then to `cargo` on PATH.
const CARGO_ENV: &str = "WB_CARGO";

#[derive(Parser, Debug)]
#[command(
    about = "Chunked cargo-release driver for the crates.io publish_new rate limit",
    long_about = "Splits a workspace release into successive cargo-release invocations of \
                  at most --chunk-size crates each, sleeping --sleep seconds between \
                  invocations so the rate window slides. Without --execute, prints the \
                  planned chunks as a dry run."
)]
struct Args {
    /// Version (e.g. 0.1.0) or level (patch/minor/major/release/alpha/beta/rc) passed
    /// to cargo-release as its first positional argument. Required when --execute is set;
    /// optional for plan-only runs (the topological chunking is independent of the value).
    /// The crates.io resumability check runs only when this is a concrete semver.
    #[arg(long)]
    version: Option<String>,

    /// Maximum new crates per cargo-release invocation.
    #[arg(long, default_value_t = 5)]
    chunk_size: usize,

    /// Seconds to sleep between chunks (default: 600s window + 60s buffer).
    #[arg(long, default_value_t = 660)]
    sleep: u64,

    /// Actually run `cargo release --execute`. Without this flag, prints the plan only.
    #[arg(long)]
    execute: bool,

    /// Pass --sign to cargo release for GPG-signed commits and tags.
    #[arg(long)]
    sign: bool,

    /// Pass --no-confirm to cargo release for unattended runs.
    #[arg(long)]
    no_confirm: bool,

    /// Do not query crates.io to skip already-published versions.
    #[arg(long)]
    skip_version_check: bool,

    /// Maximum consecutive 429 retries per chunk without forward progress.
    /// The counter resets to zero whenever an attempt manages to publish at
    /// least one new crate before being rate-limited, so a chunk that keeps
    /// landing one crate per retry is allowed to continue indefinitely.
    /// Each retry sleeps until the "Please try again after <date> GMT" timestamp
    /// embedded in the response, plus --retry-buffer seconds.
    #[arg(long, default_value_t = 3)]
    max_retries: u32,

    /// Extra seconds added to the "try again after" deadline before retrying.
    #[arg(long, default_value_t = 30)]
    retry_buffer: u64,

    /// Cargo executable to invoke for `metadata`, `locate-project`, and `release`.
    /// Defaults to the WB_CARGO environment variable, then `cargo` on PATH.
    /// Useful for forcing the project's wrapper script (e.g. `.agents/bin/cargo.sh`).
    #[arg(long)]
    cargo: Option<OsString>,
}

/// Where the resolved cargo executable came from, so the plan output can
/// document it in the form most useful to the operator.
enum CargoSource {
    /// `--cargo <path>` on the CLI.
    Cli,
    /// `WB_CARGO` environment variable.
    Env,
    /// Default `cargo` on PATH.
    Default,
}

struct CargoExe {
    path: OsString,
    source: CargoSource,
}

fn cargo_exe(args: &Args) -> CargoExe {
    if let Some(c) = &args.cargo {
        return CargoExe {
            path: c.clone(),
            source: CargoSource::Cli,
        };
    }
    if let Some(c) = env::var_os(CARGO_ENV) {
        return CargoExe {
            path: c,
            source: CargoSource::Env,
        };
    }
    CargoExe {
        path: OsString::from("cargo"),
        source: CargoSource::Default,
    }
}

#[derive(Deserialize)]
struct Metadata {
    workspace_members: Vec<String>,
    packages: Vec<Package>,
    resolve: Resolve,
}

#[derive(Deserialize)]
struct Package {
    id: String,
    name: String,
    /// `null` (publishable to all registries), `[]` (publish=false), or a list of registries.
    publish: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Resolve {
    nodes: Vec<Node>,
}

#[derive(Deserialize)]
struct Node {
    id: String,
    deps: Vec<NodeDep>,
}

#[derive(Deserialize)]
struct NodeDep {
    pkg: String,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("cargo {cmd:?} failed with status {status}: {stderr}")]
    CargoFailed {
        cmd: &'static str,
        status: String,
        stderr: String,
    },
    #[error("cycle in workspace topological sort: {0:?}")]
    Cycle(Vec<String>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

fn workspace_root(cargo: &CargoExe) -> Result<PathBuf, Error> {
    let out = Command::new(&cargo.path)
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()?;
    if !out.status.success() {
        return Err(Error::CargoFailed {
            cmd: "locate-project",
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    let mut path = PathBuf::from(String::from_utf8(out.stdout)?.trim());
    path.pop();
    Ok(path)
}

fn cargo_metadata(cargo: &CargoExe, root: &Path) -> Result<Metadata, Error> {
    let out = Command::new(&cargo.path)
        .args(["metadata", "--format-version", "1", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .output()?;
    if !out.status.success() {
        return Err(Error::CargoFailed {
            cmd: "metadata",
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(serde_json::from_slice(&out.stdout)?)
}

fn publishable_members(meta: &Metadata) -> Vec<&Package> {
    let members: BTreeSet<&str> = meta.workspace_members.iter().map(|s| s.as_str()).collect();
    meta.packages
        .iter()
        .filter(|p| {
            members.contains(p.id.as_str())
                && !matches!(p.publish.as_ref(), Some(v) if v.is_empty())
        })
        .collect()
}

fn topo_sort(meta: &Metadata, members: &[&Package]) -> Result<Vec<String>, Error> {
    let member_ids: BTreeSet<&str> = members.iter().map(|p| p.id.as_str()).collect();
    let id_to_name: HashMap<&str, &str> = meta
        .packages
        .iter()
        .map(|p| (p.id.as_str(), p.name.as_str()))
        .collect();

    let mut deps: HashMap<String, BTreeSet<String>> = members
        .iter()
        .map(|p| (p.name.clone(), BTreeSet::new()))
        .collect();

    for node in &meta.resolve.nodes {
        if !member_ids.contains(node.id.as_str()) {
            continue;
        }
        let src = id_to_name[node.id.as_str()];
        for d in &node.deps {
            if member_ids.contains(d.pkg.as_str()) {
                let dep_name = id_to_name[d.pkg.as_str()];
                if dep_name == src {
                    continue;
                }
                deps.get_mut(src).unwrap().insert(dep_name.to_string());
            }
        }
    }

    let mut indeg: HashMap<String, usize> =
        deps.iter().map(|(n, d)| (n.clone(), d.len())).collect();
    let mut rev: HashMap<String, BTreeSet<String>> = HashMap::new();
    for (n, ds) in &deps {
        for d in ds {
            rev.entry(d.clone()).or_default().insert(n.clone());
        }
    }

    let mut queue: VecDeque<String> = {
        let mut zero: Vec<String> = indeg
            .iter()
            .filter(|&(_, &d)| d == 0)
            .map(|(n, _)| n.clone())
            .collect();
        zero.sort();
        zero.into()
    };

    let mut order = Vec::with_capacity(deps.len());
    while let Some(n) = queue.pop_front() {
        if let Some(ms) = rev.get(&n) {
            for m in ms {
                let entry = indeg.get_mut(m).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    queue.push_back(m.clone());
                }
            }
        }
        order.push(n);
    }

    if order.len() != deps.len() {
        let missing: Vec<String> = deps
            .keys()
            .filter(|n| !order.contains(n))
            .cloned()
            .collect();
        return Err(Error::Cycle(missing));
    }
    Ok(order)
}

fn looks_like_semver(s: &str) -> bool {
    let mut parts = s.splitn(3, '.');
    let major = parts.next().and_then(|p| p.parse::<u32>().ok());
    let minor = parts.next().and_then(|p| p.parse::<u32>().ok());
    let patch_field = parts.next().map(|p| {
        let cut = p.find(['-', '+']).unwrap_or(p.len());
        &p[..cut]
    });
    let patch = patch_field.and_then(|p| p.parse::<u32>().ok());
    matches!((major, minor, patch), (Some(_), Some(_), Some(_)))
}

fn already_on_crates_io(name: &str, version: &str) -> bool {
    let url = format!("https://crates.io/api/v1/crates/{name}/{version}");
    match ureq::get(&url)
        .set("User-Agent", "winterbaume-release-batch/0.1")
        .timeout(Duration::from_secs(15))
        .call()
    {
        Ok(resp) => resp.status() == 200,
        Err(ureq::Error::Status(404, _)) => false,
        Err(e) => {
            eprintln!("warn: crates.io check for {name} v{version}: {e}; will attempt publish");
            false
        }
    }
}

/// Spawn `cmd` with stdout/stderr piped, tee both streams to our own
/// stdout/stderr line-by-line, and accumulate everything into a single
/// captured string for post-mortem inspection (rate-limit detection).
fn run_with_tee(cmd: &mut Command) -> Result<(ExitStatus, String), Error> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn()?;
    let child_stdout = child.stdout.take().expect("piped stdout");
    let child_stderr = child.stderr.take().expect("piped stderr");
    let buffer = Arc::new(Mutex::new(String::new()));

    let h_out = tee_stream(child_stdout, std::io::stdout(), Arc::clone(&buffer));
    let h_err = tee_stream(child_stderr, std::io::stderr(), Arc::clone(&buffer));

    let status = child.wait()?;
    h_out.join().expect("stdout tee thread panicked")?;
    h_err.join().expect("stderr tee thread panicked")?;

    let captured = Arc::try_unwrap(buffer)
        .expect("tee threads released buffer")
        .into_inner()
        .expect("buffer mutex not poisoned");
    Ok((status, captured))
}

fn tee_stream<R, W>(
    reader: R,
    mut writer: W,
    buf: Arc<Mutex<String>>,
) -> thread::JoinHandle<std::io::Result<()>>
where
    R: std::io::Read + Send + 'static,
    W: Write + Send + 'static,
{
    thread::spawn(move || {
        let mut reader = BufReader::new(reader);
        let mut line = String::new();
        loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 {
                break;
            }
            writer.write_all(line.as_bytes())?;
            writer.flush()?;
            buf.lock().expect("buffer mutex").push_str(&line);
        }
        Ok(())
    })
}

/// Scan cargo output for `error: <crate> <version> is already published to crates.io`
/// lines emitted by cargo's pre-flight check, and return the set of crate names.
/// This is cargo's own ground-truth signal (queried against the index, not the API),
/// so it is more reliable than the `already_on_crates_io` HTTP probe — which can lag
/// behind the index due to CDN caching.
fn parse_already_published(text: &str) -> BTreeSet<String> {
    const NEEDLE: &str = " is already published to crates.io";
    let mut out = BTreeSet::new();
    for line in text.lines() {
        let trimmed = line.trim();
        let body = trimmed.strip_prefix("error: ").unwrap_or(trimmed);
        let Some(idx) = body.find(NEEDLE) else {
            continue;
        };
        let head = &body[..idx];
        if let Some((name, _ver)) = head.rsplit_once(' ') {
            if !name.is_empty() {
                out.insert(name.to_string());
            }
        }
    }
    out
}

/// Scan cargo output for the crates.io 429 "Please try again after <date> GMT"
/// marker and return the parsed deadline. The phrase is embedded in the response
/// body crates.io returns for rate-limit hits and mirrors the rate-limit reset
/// time exposed via response headers.
fn parse_retry_after(text: &str) -> Option<SystemTime> {
    const NEEDLE: &str = "try again after ";
    for (idx, _) in text.match_indices(NEEDLE) {
        let rest = &text[idx + NEEDLE.len()..];
        let Some(end) = rest.find(" GMT") else {
            continue;
        };
        let candidate = &rest[..end + 4];
        if let Ok(t) = httpdate::parse_http_date(candidate) {
            return Some(t);
        }
    }
    None
}

enum ChunkOutcome {
    Success,
    Failure(ExitStatus),
}

fn run() -> Result<ExitCode, Error> {
    let args = Args::parse();

    if args.execute && args.version.is_none() {
        eprintln!("error: --version is required when --execute is set");
        return Ok(ExitCode::FAILURE);
    }

    let cargo = cargo_exe(&args);
    let root = workspace_root(&cargo)?;
    let meta = cargo_metadata(&cargo, &root)?;
    let members = publishable_members(&meta);
    let order = topo_sort(&meta, &members)?;

    eprintln!("workspace root: {}", root.display());
    let cargo_source_label = match cargo.source {
        CargoSource::Cli => "from --cargo flag",
        CargoSource::Env => concat!("from ", "WB_CARGO"),
        CargoSource::Default => "default on PATH",
    };
    eprintln!(
        "cargo executable: {} ({cargo_source_label})",
        cargo.path.to_string_lossy()
    );
    eprintln!("publishable crates: {}", order.len());

    // Resumability is only meaningful when --version is a concrete semver.
    let resumable_version: Option<&str> = args.version.as_deref().filter(|v| looks_like_semver(v));

    let plan: Vec<String> = match resumable_version {
        Some(v) if !args.skip_version_check => {
            let mut keep = Vec::with_capacity(order.len());
            for name in order {
                if already_on_crates_io(&name, v) {
                    eprintln!("skip {name} v{v}: already on crates.io");
                } else {
                    keep.push(name);
                }
            }
            keep
        }
        _ => {
            if !args.skip_version_check && args.version.is_some() {
                eprintln!(
                    "note: --version is a level, not a concrete semver; skipping crates.io resumability check"
                );
            }
            order
        }
    };

    let bump_label = args.version.as_deref().unwrap_or("<version|level>");
    eprintln!("to publish ({bump_label}): {}", plan.len());
    if plan.is_empty() {
        eprintln!("nothing to do");
        return Ok(ExitCode::SUCCESS);
    }

    let chunks: Vec<&[String]> = plan.chunks(args.chunk_size).collect();
    eprintln!(
        "chunks: {} of up to {} crates each",
        chunks.len(),
        args.chunk_size
    );

    for (i, chunk) in chunks.iter().enumerate() {
        let i1 = i + 1;
        let mut working_chunk: Vec<String> = chunk.to_vec();
        eprintln!(
            "\n=== chunk {i1}/{n} ({k} crate(s)) ===",
            n = chunks.len(),
            k = working_chunk.len()
        );
        eprintln!("$ cargo release {bump_label} {}", working_chunk.join(" "));

        if !args.execute {
            continue;
        }

        let mut rate_limit_attempts: u32 = 0;
        let outcome = loop {
            let mut cmd = Command::new(&cargo.path);
            cmd.arg("release").arg(args.version.as_ref().unwrap());
            for c in &working_chunk {
                cmd.arg("-p").arg(c);
            }
            if args.sign {
                cmd.arg("--sign");
            }
            if args.no_confirm {
                cmd.arg("--no-confirm");
            }
            cmd.arg("--execute");
            cmd.current_dir(&root);

            let (status, captured) = run_with_tee(&mut cmd)?;
            if status.success() {
                break ChunkOutcome::Success;
            }

            // Two recovery signals can show up in cargo's output:
            //   1. "<crate> <version> is already published to crates.io" — cargo's
            //      own pre-flight check against the registry index. Authoritative.
            //   2. "Please try again after <date> GMT" — a 429 publish_new rate
            //      limit, indicating we should sleep until the window slides.
            // If neither appears, the failure is something we can't recover from.
            let already_published = parse_already_published(&captured);
            let rate_limit_deadline = parse_retry_after(&captured);

            if rate_limit_deadline.is_none() && already_published.is_empty() {
                eprintln!(
                    "chunk {i1} failed (rc={:?}); fix the cause and re-run",
                    status.code()
                );
                break ChunkOutcome::Failure(status);
            }

            if rate_limit_deadline.is_some() && rate_limit_attempts >= args.max_retries {
                eprintln!(
                    "chunk {i1} hit crates.io rate limit and exhausted {} retries; aborting",
                    args.max_retries
                );
                break ChunkOutcome::Failure(status);
            }

            if let Some(deadline) = rate_limit_deadline {
                rate_limit_attempts += 1;
                let now = SystemTime::now();
                let wait = deadline
                    .duration_since(now)
                    .unwrap_or_default()
                    .saturating_add(Duration::from_secs(args.retry_buffer));
                eprintln!(
                    "chunk {i1} hit crates.io rate limit; sleeping {}s until {} (retry {rate_limit_attempts}/{max})",
                    wait.as_secs(),
                    httpdate::fmt_http_date(deadline),
                    max = args.max_retries,
                );
                thread::sleep(wait);
            }

            // Prune crates already on crates.io. The two signals are complementary:
            //   - cargo's "is already published" error names crates the index
            //     already has.
            //   - On a 429 retry, cargo never reached its pre-flight, so we also
            //     probe the crates.io API as a best-effort secondary path.
            let version = args.version.as_ref().unwrap();
            let before = working_chunk.len();
            working_chunk.retain(|name| {
                if already_published.contains(name) {
                    eprintln!("skip {name} v{version}: cargo reports already published");
                    return false;
                }
                if rate_limit_deadline.is_some() && already_on_crates_io(name, version) {
                    eprintln!("skip {name} v{version}: published during previous attempt");
                    return false;
                }
                true
            });

            if working_chunk.is_empty() {
                eprintln!("chunk {i1} fully landed during previous attempt; nothing left to retry");
                break ChunkOutcome::Success;
            }
            if working_chunk.len() < before {
                eprintln!(
                    "retrying chunk {i1} with {} remaining crate(s): {}",
                    working_chunk.len(),
                    working_chunk.join(" ")
                );
                // Forward progress: at least one new crate landed since the
                // last attempt, so the rate-limit retry budget is renewed.
                // Without this, a chunk that keeps publishing one crate per
                // retry would still abort after --max-retries 429s, even
                // though it is steadily draining work.
                if rate_limit_attempts > 0 {
                    eprintln!(
                        "resetting 429 retry counter (was {rate_limit_attempts}/{max}) because new crates were published",
                        max = args.max_retries,
                    );
                    rate_limit_attempts = 0;
                }
            }
        };

        if let ChunkOutcome::Failure(status) = outcome {
            return Ok(ExitCode::from(status.code().unwrap_or(1) as u8));
        }

        if i1 < chunks.len() {
            eprintln!("sleeping {}s before next chunk", args.sleep);
            thread::sleep(Duration::from_secs(args.sleep));
        }
    }

    eprintln!("\nall chunks complete");
    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verbatim error from a real crates.io 429 hit during first-launch.
    const REAL_429_OUTPUT: &str = "\
note: the following crates have not been published yet:
  winterbaume-bedrock-flow-validator v0.1.0 (/Users/moriyoshi/Source/winterbaume/crates/winterbaume-bedrock-flow-validator)
  winterbaume-tfstate v0.1.0 (/Users/moriyoshi/Source/winterbaume/crates/winterbaume-tfstate)
  winterbaume-wafv2-webacl-rule-parser v0.1.0 (/Users/moriyoshi/Source/winterbaume/crates/winterbaume-wafv2-webacl-rule-parser)

Caused by:
  the remote server responded with an error (status 429 Too Many Requests): You have published too many new crates in a short period of time. Please try again after Fri, 08 May 2026 13:02:42 GMT and see https://crates.io/docs/rate-limits for more details.
";

    #[test]
    fn parses_real_429_message() {
        let deadline = parse_retry_after(REAL_429_OUTPUT).expect("recognises 429 deadline");
        let expected = httpdate::parse_http_date("Fri, 08 May 2026 13:02:42 GMT").unwrap();
        assert_eq!(deadline, expected);
    }

    #[test]
    fn returns_none_when_phrase_absent() {
        let arbitrary = "error: failed to publish: cargo: connection refused\n";
        assert!(parse_retry_after(arbitrary).is_none());
    }

    #[test]
    fn returns_none_when_phrase_present_but_date_unparsable() {
        let garbage = "Caused by: try again after sometime soon GMT and see ...\n";
        assert!(parse_retry_after(garbage).is_none());
    }

    #[test]
    fn picks_later_candidate_when_first_is_malformed() {
        let mixed = "\
junk: try again after not-a-date GMT and see more
real: Please try again after Fri, 08 May 2026 13:02:42 GMT and see ...
";
        let deadline = parse_retry_after(mixed).expect("falls through to valid candidate");
        let expected = httpdate::parse_http_date("Fri, 08 May 2026 13:02:42 GMT").unwrap();
        assert_eq!(deadline, expected);
    }

    #[test]
    fn ignores_phrase_without_gmt_terminator() {
        let no_gmt = "try again after Fri, 08 May 2026 13:02:42 UTC\n";
        assert!(parse_retry_after(no_gmt).is_none());
    }

    /// Verbatim tail of a real `cargo release` retry that hit cargo's
    /// pre-flight pre-existence check after a partial-batch 429 left two
    /// crates already published on crates.io.
    const REAL_ALREADY_PUBLISHED: &str = "\
warning: disabled by user, skipping winterbaume-xray v0.1.0 despite being unpublished
error: winterbaume-amplify 0.1.0 is already published to crates.io
error: winterbaume-amplifybackend 0.1.0 is already published to crates.io
chunk 1 failed (rc=Some(101)); fix the cause and re-run
";

    #[test]
    fn parses_already_published_list() {
        let names = parse_already_published(REAL_ALREADY_PUBLISHED);
        assert!(names.contains("winterbaume-amplify"));
        assert!(names.contains("winterbaume-amplifybackend"));
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn already_published_empty_when_absent() {
        let only_warnings =
            "warning: disabled by user, skipping winterbaume v0.1.0 despite being unpublished\n";
        assert!(parse_already_published(only_warnings).is_empty());
    }

    #[test]
    fn already_published_handles_missing_error_prefix() {
        let no_prefix = "winterbaume-foo 0.1.0 is already published to crates.io\n";
        let names = parse_already_published(no_prefix);
        assert!(names.contains("winterbaume-foo"));
    }
}
