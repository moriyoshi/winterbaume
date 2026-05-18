//! Chunked `cargo release` driver for the crates.io `publish_new` rate limit.
//!
//! Originally a standalone binary (`tools/release-batch/`); folded into the
//! harness so `publish` can call it in-process and there's one entry point for
//! the release workflow.
//!
//! Cargo rejects a multi-crate batch upfront when the planned new-crate count
//! exceeds the account's `publish_new` quota (default: 5 per 10 minutes). This
//! module splits a workspace release into successive `cargo release`
//! invocations of at most `chunk_size` crates each, sleeping `sleep` seconds
//! between invocations so the rate window slides.

use std::collections::BTreeSet;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use crate::metadata::CargoExe;

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

/// Parameters to a single chunked release run.
///
/// `version_or_level` is passed verbatim as `cargo release`'s first positional
/// argument — either a level (`patch`, `minor`, `major`, `release`, `alpha`,
/// `beta`, `rc`) or a concrete semver. The crates.io resumability check only
/// runs when this is a concrete semver.
#[derive(Clone, Debug)]
pub struct BatchOptions<'a> {
    pub cargo: &'a CargoExe,
    pub root: &'a Path,
    pub version_or_level: &'a str,
    /// Topologically sorted crate names to publish, dependencies first.
    pub crates: Vec<String>,
    pub chunk_size: usize,
    pub sleep_between_chunks: Duration,
    pub max_retries: u32,
    pub retry_buffer: Duration,
    pub execute: bool,
    pub sign: bool,
    pub no_confirm: bool,
    pub skip_version_check: bool,
}

pub fn run_chunked(opts: BatchOptions<'_>) -> Result<RunOutcome, Error> {
    let cargo = opts.cargo;
    let root = opts.root;
    let version_or_level = opts.version_or_level;

    // Resumability is only meaningful when version_or_level is a concrete semver.
    let resumable_version: Option<&str> =
        if looks_like_semver(version_or_level) && !opts.skip_version_check {
            Some(version_or_level)
        } else {
            if !opts.skip_version_check {
                eprintln!(
                    "note: `{version_or_level}` is a level, not a concrete semver; \
                     skipping crates.io resumability check"
                );
            }
            None
        };

    let plan: Vec<String> = match resumable_version {
        Some(v) => {
            let mut keep = Vec::with_capacity(opts.crates.len());
            for name in opts.crates {
                if already_on_crates_io(&name, v) {
                    eprintln!("skip {name} v{v}: already on crates.io");
                } else {
                    keep.push(name);
                }
            }
            keep
        }
        None => opts.crates,
    };

    eprintln!("to publish ({version_or_level}): {}", plan.len());
    if plan.is_empty() {
        eprintln!("nothing to do");
        return Ok(RunOutcome::Success);
    }

    let chunks: Vec<&[String]> = plan.chunks(opts.chunk_size).collect();
    eprintln!(
        "chunks: {} of up to {} crates each",
        chunks.len(),
        opts.chunk_size
    );

    for (i, chunk) in chunks.iter().enumerate() {
        let i1 = i + 1;
        let mut working_chunk: Vec<String> = chunk.to_vec();
        eprintln!(
            "\n=== chunk {i1}/{n} ({k} crate(s)) ===",
            n = chunks.len(),
            k = working_chunk.len()
        );
        eprintln!(
            "$ cargo release {version_or_level} {}",
            working_chunk.join(" ")
        );

        if !opts.execute {
            continue;
        }

        let mut rate_limit_attempts: u32 = 0;
        let outcome = loop {
            let mut cmd = Command::new(cargo.path());
            cmd.arg("release").arg(version_or_level);
            for c in &working_chunk {
                cmd.arg("-p").arg(c);
            }
            if opts.sign {
                cmd.arg("--sign");
            }
            if opts.no_confirm {
                cmd.arg("--no-confirm");
            }
            cmd.arg("--execute");
            cmd.current_dir(root);

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
            let uploaded = parse_uploaded(&captured, version_or_level);

            // Any chunk-failure path may have published crates before the failure
            // hit. Backfill their tags before we either bail out or retry — the
            // pruning loop below would otherwise miss crates that fall outside the
            // 429 / already-published recovery branches.
            for name in &uploaded {
                let tag = format!("{name}-v{version_or_level}");
                match backfill_tag(&tag, opts.sign, root) {
                    Ok(BackfillOutcome::AlreadyExists) => {}
                    Ok(BackfillOutcome::Created) => eprintln!("backfilled tag: {tag}"),
                    Err(e) => eprintln!("warn: failed to backfill tag {tag}: {e}"),
                }
            }

            if rate_limit_deadline.is_none() && already_published.is_empty() {
                eprintln!(
                    "chunk {i1} failed (rc={:?}); fix the cause and re-run",
                    status.code()
                );
                break ChunkOutcome::Failure(status);
            }

            if rate_limit_deadline.is_some() && rate_limit_attempts >= opts.max_retries {
                eprintln!(
                    "chunk {i1} hit crates.io rate limit and exhausted {} retries; aborting",
                    opts.max_retries
                );
                break ChunkOutcome::Failure(status);
            }

            if let Some(deadline) = rate_limit_deadline {
                rate_limit_attempts += 1;
                let now = SystemTime::now();
                let wait = deadline
                    .duration_since(now)
                    .unwrap_or_default()
                    .saturating_add(opts.retry_buffer);
                eprintln!(
                    "chunk {i1} hit crates.io rate limit; sleeping {}s until {} (retry {rate_limit_attempts}/{max})",
                    wait.as_secs(),
                    httpdate::fmt_http_date(deadline),
                    max = opts.max_retries,
                );
                thread::sleep(wait);
            }

            // Prune crates already on crates.io. The two signals are complementary:
            //   - cargo's "is already published" error names crates the index
            //     already has.
            //   - On a 429 retry, cargo never reached its pre-flight, so we also
            //     probe the crates.io API as a best-effort secondary path.
            let before = working_chunk.len();
            let mut pruned: Vec<String> = Vec::new();
            working_chunk.retain(|name| {
                if already_published.contains(name) {
                    eprintln!("skip {name} v{version_or_level}: cargo reports already published");
                    pruned.push(name.clone());
                    return false;
                }
                if rate_limit_deadline.is_some() && already_on_crates_io(name, version_or_level) {
                    eprintln!("skip {name} v{version_or_level}: published during previous attempt");
                    pruned.push(name.clone());
                    return false;
                }
                true
            });

            // cargo-release pushes tags only as part of the same invocation
            // that publishes the crate. When a chunk is interrupted mid-batch
            // by a 429, the crates that landed before the failure never get
            // tagged, and the retry's `cargo release` invocation no longer
            // includes them (we just pruned them above) — so the tag would be
            // dropped on the floor unless we backfill it here.
            for name in &pruned {
                let tag = format!("{name}-v{version_or_level}");
                match backfill_tag(&tag, opts.sign, root) {
                    Ok(BackfillOutcome::AlreadyExists) => {}
                    Ok(BackfillOutcome::Created) => eprintln!("backfilled tag: {tag}"),
                    Err(e) => eprintln!("warn: failed to backfill tag {tag}: {e}"),
                }
            }

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
                if rate_limit_attempts > 0 {
                    eprintln!(
                        "resetting 429 retry counter (was {rate_limit_attempts}/{max}) \
                         because new crates were published",
                        max = opts.max_retries,
                    );
                    rate_limit_attempts = 0;
                }
            }
        };

        if let ChunkOutcome::Failure(status) = outcome {
            return Ok(RunOutcome::ChunkFailed(status));
        }

        if i1 < chunks.len() {
            eprintln!(
                "sleeping {}s before next chunk",
                opts.sleep_between_chunks.as_secs()
            );
            thread::sleep(opts.sleep_between_chunks);
        }
    }

    eprintln!("\nall chunks complete");
    Ok(RunOutcome::Success)
}

/// Result of a chunked-release run. `ChunkFailed` carries the failing chunk's
/// process exit status so the caller can propagate the same exit code.
#[derive(Debug)]
pub enum RunOutcome {
    Success,
    ChunkFailed(ExitStatus),
}

enum ChunkOutcome {
    Success,
    Failure(ExitStatus),
}

enum BackfillOutcome {
    AlreadyExists,
    Created,
}

pub(crate) fn looks_like_semver(s: &str) -> bool {
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
        .set("User-Agent", "winterbaume-release-harness/0.1")
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

/// Scan cargo output for `Uploaded <crate> v<version>` status lines emitted by
/// cargo when it has finished pushing a crate's tarball to crates.io.
///
/// Returns an empty set if `version` is not a concrete semver (i.e., a level
/// keyword) — uploaded-line matching is only meaningful when we know the
/// landing version up front.
fn parse_uploaded(text: &str, version: &str) -> BTreeSet<String> {
    if !looks_like_semver(version) {
        return BTreeSet::new();
    }
    const PREFIX: &str = "Uploaded ";
    let suffix = format!(" v{version}");
    let mut out = BTreeSet::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix(PREFIX) else {
            continue;
        };
        let Some(name_end) = rest.find(' ') else {
            continue;
        };
        let name = &rest[..name_end];
        let tail = &rest[name_end..];
        if !(tail == suffix || tail.starts_with(&format!("{suffix} "))) {
            continue;
        }
        if !name.is_empty() {
            out.insert(name.to_string());
        }
    }
    out
}

/// Scan cargo output for the crates.io 429 "Please try again after <date> GMT"
/// marker and return the parsed deadline.
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

/// Create a (signed when requested) annotated tag at HEAD if it doesn't exist
/// and push it to origin. Used to recover tags for crates that were uploaded
/// to crates.io in the same chunk but where cargo-release failed before
/// reaching its tag step (typically on a partial-batch 429).
fn backfill_tag(tag: &str, sign: bool, root: &Path) -> Result<BackfillOutcome, Error> {
    let exists = Command::new("git")
        .args(["tag", "-l", tag])
        .current_dir(root)
        .output()?;
    if !exists.status.success() {
        return Err(Error::GitFailed {
            cmd: "git tag -l".to_string(),
            status: exists.status.to_string(),
            stderr: String::from_utf8_lossy(&exists.stderr).into_owned(),
        });
    }
    if !exists.stdout.is_empty() {
        return Ok(BackfillOutcome::AlreadyExists);
    }

    let mut create = Command::new("git");
    create
        .arg("tag")
        .arg(if sign { "-s" } else { "-a" })
        .args(["-m", ""])
        .arg(tag)
        .current_dir(root);
    let create_out = create.output()?;
    if !create_out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git tag".to_string(),
            status: create_out.status.to_string(),
            stderr: String::from_utf8_lossy(&create_out.stderr).into_owned(),
        });
    }

    let push_out = Command::new("git")
        .args(["push", "origin", tag])
        .current_dir(root)
        .output()?;
    if !push_out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git push origin <tag>".to_string(),
            status: push_out.status.to_string(),
            stderr: String::from_utf8_lossy(&push_out.stderr).into_owned(),
        });
    }
    Ok(BackfillOutcome::Created)
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

    const REAL_UPLOAD_LOG: &str = "\
    Finished `release` profile [optimized] target(s) in 1.50s
   Packaging winterbaume-foo v0.2.0 (/path/to/crates/winterbaume-foo)
   Verifying winterbaume-foo v0.2.0 (/path/to/crates/winterbaume-foo)
   Uploading winterbaume-foo v0.2.0 (/path/to/crates/winterbaume-foo)
    Uploaded winterbaume-foo v0.2.0
   Packaging winterbaume-bar v0.2.0 (/path/to/crates/winterbaume-bar)
    Uploaded winterbaume-bar v0.2.0 to crates.io
   Published winterbaume-foo v0.2.0
error: failed to publish to registry at https://crates.io/
";

    #[test]
    fn parses_uploaded_crates_from_status_lines() {
        let names = parse_uploaded(REAL_UPLOAD_LOG, "0.2.0");
        assert!(names.contains("winterbaume-foo"));
        assert!(names.contains("winterbaume-bar"));
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn parse_uploaded_filters_by_version() {
        let names = parse_uploaded(REAL_UPLOAD_LOG, "0.3.0");
        assert!(names.is_empty());
    }

    #[test]
    fn parse_uploaded_skipped_for_level_keywords() {
        let names = parse_uploaded(REAL_UPLOAD_LOG, "patch");
        assert!(names.is_empty());
    }

    #[test]
    fn parse_uploaded_ignores_uploading_progress_lines() {
        let progress_only = "\
   Uploading winterbaume-foo v0.2.0
   Uploading winterbaume-bar v0.2.0
";
        let names = parse_uploaded(progress_only, "0.2.0");
        assert!(names.is_empty());
    }

    #[test]
    fn parse_uploaded_rejects_version_mismatch() {
        let mixed = "\
    Uploaded winterbaume-foo v0.1.0
    Uploaded winterbaume-bar v0.2.0
";
        let names = parse_uploaded(mixed, "0.2.0");
        assert!(!names.contains("winterbaume-foo"));
        assert!(names.contains("winterbaume-bar"));
        assert_eq!(names.len(), 1);
    }

    #[test]
    fn parse_uploaded_empty_when_no_match() {
        let unrelated = "error: build failed\nwarning: deprecated\n";
        assert!(parse_uploaded(unrelated, "0.2.0").is_empty());
    }
}
