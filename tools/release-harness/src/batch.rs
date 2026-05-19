//! Per-step `cargo release` driver that composes a rich consolidated commit
//! message and resumes cleanly from any partial-state failure.
//!
//! cargo-release's `consolidate-commits = true` path can't substitute
//! `{{crate_name}}` / `{{version}}` (multiple crates → multiple values), so the
//! workspace's `pre-release-commit-message` template is intentionally minimal
//! and we amend the commit ourselves between `cargo release commit` and
//! `cargo release publish`. Doing the amend *before* publish keeps the SHA
//! recorded in each tarball's `.cargo_vcs_info.json` consistent with the final
//! commit and with the per-crate tags that get created afterwards.
//!
//! The chunk loop is fully resumable: every step is gated on a state probe
//! (manifest versions, HEAD subject/body, crates.io publish status, local +
//! remote git refs), so a re-run after a crash, 429 cliff, or Ctrl+C picks up
//! where the previous attempt left off without re-bumping, re-committing,
//! re-publishing, re-tagging, or re-pushing anything that already landed.
//!
//! Chunked invocation also accommodates the crates.io `publish_new` rate limit
//! (default 5 per 10 minutes): a workspace release is split into successive
//! per-step subcommand sequences of at most `chunk_size` crates each, sleeping
//! `sleep` seconds between chunks so the rate window slides. Only the
//! `publish` step is subject to that limit, so the retry-on-429 / prune
//! already-published logic lives around that one call.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use crate::metadata::{CargoExe, cargo_metadata};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("git {cmd:?} failed (status {status}): {stderr}")]
    GitFailed {
        cmd: String,
        status: String,
        stderr: String,
    },
    #[error(
        "ambiguous resume state for chunk {chunk:?}: HEAD subject = {head_subject:?}, \
         working tree is clean, but no in-progress release commit was found. Bailing out \
         to avoid clobbering unrelated work; fix manually and re-run."
    )]
    AmbiguousResume {
        chunk: Vec<String>,
        head_subject: String,
    },
    #[error(transparent)]
    Metadata(#[from] crate::metadata::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

/// Parameters to a single chunked release run.
///
/// `version_or_level` is passed verbatim as `cargo release version`'s
/// positional argument — either a level (`patch`, `minor`, `major`, `release`,
/// `alpha`, `beta`, `rc`) or a concrete semver. The crates.io resumability
/// check only runs when this is a concrete semver.
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

    let resumable_version: Option<&str> =
        if looks_like_semver(version_or_level) && !opts.skip_version_check {
            Some(version_or_level)
        } else {
            if !opts.skip_version_check {
                eprintln!(
                    "note: `{version_or_level}` is a level, not a concrete semver; \
                     skipping pre-flight crates.io resumability check"
                );
            }
            None
        };

    // Pre-flight prune: for literal-version runs, drop crates whose target is
    // already on crates.io even before we start chunking. (Within-chunk resume
    // re-checks at finer granularity.)
    let plan: Vec<String> = match resumable_version {
        Some(v) => {
            let mut keep = Vec::with_capacity(opts.crates.len());
            for name in &opts.crates {
                if already_on_crates_io(name, v) {
                    eprintln!("skip {name} v{v}: already on crates.io");
                } else {
                    keep.push(name.clone());
                }
            }
            keep
        }
        None => opts.crates.clone(),
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
        eprintln!(
            "\n=== chunk {i1}/{n} ({k} crate(s)) ===",
            n = chunks.len(),
            k = chunk.len()
        );

        if !opts.execute {
            print_dry_run(version_or_level, chunk, opts.sign, opts.no_confirm);
            continue;
        }

        if let Some(status) = run_chunk(&opts, chunk, i1)? {
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

/// Run a single chunk's worth of steps, skipping each step that the state
/// probes report is already complete. Returns `Some(status)` on a
/// non-recoverable failure (the caller turns this into
/// `RunOutcome::ChunkFailed`) and `None` on success.
fn run_chunk(
    opts: &BatchOptions<'_>,
    chunk: &[String],
    i1: usize,
) -> Result<Option<ExitStatus>, Error> {
    let cargo = opts.cargo;
    let root = opts.root;
    let version_or_level = opts.version_or_level;

    // 1-3. Version-bump / replace / hook (only when manifests are not yet at
    //      the target version).
    let mut manifest_versions = read_versions(cargo, root, chunk)?;
    if should_run_version_step(version_or_level, &manifest_versions, chunk) {
        let (status, _) = run_release_step(
            cargo,
            root,
            &["version", version_or_level],
            chunk,
            false,
            opts.no_confirm,
        )?;
        if !status.success() {
            return Ok(Some(status));
        }
        let (status, _) =
            run_release_step(cargo, root, &["replace"], chunk, false, opts.no_confirm)?;
        if !status.success() {
            return Ok(Some(status));
        }
        let (status, _) = run_release_step(cargo, root, &["hook"], chunk, false, opts.no_confirm)?;
        if !status.success() {
            return Ok(Some(status));
        }
        manifest_versions = read_versions(cargo, root, chunk)?;
    } else {
        eprintln!("manifests already at target — skipping version/replace/hook");
    }

    // From here on, manifest_versions == target_versions for this chunk.
    let target_versions = &manifest_versions;

    // 4-5. Commit + amend, guided by what HEAD currently looks like.
    let head_state = classify_head(root, chunk, target_versions)?;
    match head_state {
        HeadState::AmendedForThisChunk => {
            eprintln!("HEAD already amended for this chunk — skipping commit + amend");
        }
        HeadState::CargoReleasePlaceholder => {
            eprintln!("HEAD is a cargo-release placeholder commit — running amend only");
            let msg = build_commit_message(chunk, target_versions);
            amend_commit(root, &msg, opts.sign)?;
        }
        HeadState::Unrelated { subject } => {
            if git_status_dirty(root)? {
                let (status, _) =
                    run_release_step(cargo, root, &["commit"], chunk, opts.sign, opts.no_confirm)?;
                if !status.success() {
                    return Ok(Some(status));
                }
                let msg = build_commit_message(chunk, target_versions);
                amend_commit(root, &msg, opts.sign)?;
            } else {
                return Err(Error::AmbiguousResume {
                    chunk: chunk.to_vec(),
                    head_subject: subject,
                });
            }
        }
    }

    // 6. Publish — with prune + 429 retry. Crates already on crates.io at the
    //    target version are skipped before the first call.
    let mut working_chunk: Vec<String> = chunk
        .iter()
        .filter(|c| match target_versions.get(*c) {
            Some(v) => !already_on_crates_io(c, v),
            None => true,
        })
        .cloned()
        .collect();
    if working_chunk.len() < chunk.len() {
        eprintln!(
            "publish: {} of {} crate(s) already on crates.io",
            chunk.len() - working_chunk.len(),
            chunk.len()
        );
    }

    if !working_chunk.is_empty() {
        let mut rate_limit_attempts: u32 = 0;
        let publish_outcome = loop {
            let (status, captured) = run_release_step(
                cargo,
                root,
                &["publish"],
                &working_chunk,
                false,
                opts.no_confirm,
            )?;
            if status.success() {
                break ChunkOutcome::Success;
            }

            // Two recovery signals can show up in cargo's output:
            //   1. "<crate> <version> is already published to crates.io" —
            //      cargo's own pre-flight check. Authoritative.
            //   2. "Please try again after <date> GMT" — a 429 publish_new
            //      rate limit; sleep until the window slides.
            let already_published = parse_already_published(&captured);
            let rate_limit_deadline = parse_retry_after(&captured);

            if rate_limit_deadline.is_none() && already_published.is_empty() {
                eprintln!(
                    "chunk {i1} publish failed (rc={:?}); fix the cause and re-run",
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

            let before = working_chunk.len();
            working_chunk.retain(|name| {
                if already_published.contains(name) {
                    eprintln!("skip {name}: cargo reports already published");
                    return false;
                }
                if rate_limit_deadline.is_some() {
                    if let Some(v) = target_versions.get(name) {
                        if already_on_crates_io(name, v) {
                            eprintln!("skip {name} v{v}: published during previous attempt");
                            return false;
                        }
                    }
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

        if let ChunkOutcome::Failure(status) = publish_outcome {
            return Ok(Some(status));
        }
    } else {
        eprintln!("publish: all crates already on crates.io — skipping publish step");
    }

    // 7. Tag — one annotated (and signed when requested) tag per crate, at the
    //    amended HEAD. Direct `git tag` so we can create only the missing ones.
    let local_tags = local_tags(root)?;
    let mut tagged_now = 0;
    for c in chunk {
        let v = target_versions.get(c).cloned().unwrap_or_default();
        if v.is_empty() {
            continue;
        }
        let tag = format!("{c}-v{v}");
        if local_tags.contains(&tag) {
            continue;
        }
        create_tag(root, &tag, &tag_message(c, &v), opts.sign)?;
        eprintln!("created tag {tag}");
        tagged_now += 1;
    }
    if tagged_now == 0 {
        eprintln!("tag: all chunk tags already exist locally — skipping tag step");
    }

    // 8. Push — HEAD and any chunk tags missing on origin, in one push.
    let remote = remote_refs(root)?;
    let branch = current_branch(root)?;
    let head_sha = rev_parse(root, "HEAD")?;
    let head_pushed = remote
        .get(&format!("refs/heads/{branch}"))
        .map(|s| s == &head_sha)
        .unwrap_or(false);

    let mut refs_to_push: Vec<String> = Vec::new();
    if !head_pushed {
        refs_to_push.push(branch.clone());
    }
    for c in chunk {
        let v = target_versions.get(c).cloned().unwrap_or_default();
        if v.is_empty() {
            continue;
        }
        let tag = format!("{c}-v{v}");
        if remote.contains_key(&format!("refs/tags/{tag}")) {
            continue;
        }
        refs_to_push.push(tag);
    }
    if refs_to_push.is_empty() {
        eprintln!("push: HEAD and all chunk tags already on origin — nothing to push");
    } else {
        git_push(root, &refs_to_push)?;
    }

    Ok(None)
}

/// Result of a chunked-release run. `ChunkFailed` carries the failing step's
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

#[derive(Debug, PartialEq, Eq)]
enum HeadState {
    /// HEAD subject is `chore: release` and the body lists exactly this
    /// chunk's crates at their target versions.
    AmendedForThisChunk,
    /// HEAD subject is `chore: release` but the body is empty — i.e. cargo-
    /// release ran `commit` but our `amend` step never landed. We still need
    /// to amend, but we must not run `cargo release commit` again.
    CargoReleasePlaceholder,
    /// Anything else — could be a previous chunk's amended commit, an
    /// unrelated user commit, or a mid-rebase state.
    Unrelated { subject: String },
}

fn classify_head(
    root: &Path,
    chunk: &[String],
    target_versions: &BTreeMap<String, String>,
) -> Result<HeadState, Error> {
    let msg = read_head_message(root)?;
    Ok(classify_head_message(&msg, chunk, target_versions))
}

fn classify_head_message(
    msg: &str,
    chunk: &[String],
    target_versions: &BTreeMap<String, String>,
) -> HeadState {
    let subject = msg.lines().next().unwrap_or("").to_string();
    // Lenient: accept any subject starting with "chore: release" — covers the
    // intended clean template ("chore: release"), the placeholder-littered
    // default template ("chore: release {{crate_name}} v{{version}}"), and
    // the rendered single-crate form ("chore: release winterbaume-foo v0.3.0").
    // The body (our amend marker) is what actually pins identity.
    if !subject.trim_start().starts_with("chore: release") {
        return HeadState::Unrelated { subject };
    }
    let parsed: BTreeMap<String, String> = parse_amend_body(msg).into_iter().collect();
    if parsed.is_empty() {
        return HeadState::CargoReleasePlaceholder;
    }
    let chunk_set: BTreeMap<String, String> = chunk
        .iter()
        .filter_map(|c| target_versions.get(c).map(|v| (c.clone(), v.clone())))
        .collect();
    if parsed == chunk_set {
        HeadState::AmendedForThisChunk
    } else {
        HeadState::Unrelated { subject }
    }
}

/// Parse body lines like `- winterbaume-foo v0.3.0` into `(name, version)`.
fn parse_amend_body(msg: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in msg.lines() {
        let Some(rest) = line.strip_prefix("- ") else {
            continue;
        };
        let Some((name, ver)) = rest.rsplit_once(' ') else {
            continue;
        };
        let Some(v) = ver.strip_prefix('v') else {
            continue;
        };
        if !name.is_empty() && !v.is_empty() {
            out.push((name.to_string(), v.to_string()));
        }
    }
    out
}

fn should_run_version_step(
    version_or_level: &str,
    manifest_versions: &BTreeMap<String, String>,
    chunk: &[String],
) -> bool {
    if looks_like_semver(version_or_level) {
        chunk.iter().any(|c| {
            manifest_versions
                .get(c)
                .map(|v| v != version_or_level)
                .unwrap_or(true)
        })
    } else {
        // Level keyword: only re-bump if the current manifest versions are
        // still the ones already on crates.io. If any current version is
        // unpublished, we treat the chunk as already-bumped to avoid double-
        // bumping mid-resume.
        chunk.iter().any(|c| {
            manifest_versions
                .get(c)
                .map(|v| already_on_crates_io(c, v))
                .unwrap_or(false)
        })
    }
}

fn print_dry_run(version_or_level: &str, chunk: &[String], sign: bool, no_confirm: bool) {
    let p_args: String = chunk.iter().map(|c| format!(" -p {c}")).collect();
    let commit_sign_flag = if sign { " --sign-commit" } else { "" };
    let nc = if no_confirm { " --no-confirm" } else { "" };
    eprintln!("(steps below are gated on resume probes; some may be skipped at runtime)");
    eprintln!("$ cargo release version {version_or_level}{p_args}{nc} --execute");
    eprintln!("$ cargo release replace{p_args}{nc} --execute");
    eprintln!("$ cargo release hook{p_args}{nc} --execute");
    // `cargo release commit` rejects -p; it commits the whole working tree.
    eprintln!("$ cargo release commit{commit_sign_flag}{nc} --execute");
    eprintln!(
        "$ git commit --amend{} -m \"chore: release ... per-crate body\"",
        if sign { " -S" } else { "" }
    );
    eprintln!("$ cargo release publish{p_args}{nc} --execute");
    eprintln!(
        "$ git tag {}-m \"chore: release <crate> v<ver>\" <crate>-v<ver>   (per missing tag)",
        if sign { "-s " } else { "-a " }
    );
    eprintln!("$ git push origin <branch> <missing-tags>");
}

fn run_release_step(
    cargo: &CargoExe,
    root: &Path,
    step_args: &[&str],
    crates: &[String],
    sign: bool,
    no_confirm: bool,
) -> Result<(ExitStatus, String), Error> {
    // cargo-release's `commit` subcommand operates on the whole working tree
    // and rejects `-p` outright ("error: unexpected argument '-p' found"). All
    // other steps we drive (version / replace / hook / publish) do accept it.
    let step = step_args.first().copied().unwrap_or("");
    let pass_packages = step != "commit";
    // The legacy top-level `cargo release --sign` was split per subcommand:
    // `cargo release commit` takes `--sign-commit` and `cargo release tag`
    // takes `--sign-tag`. Map the harness's boolean to whichever is correct
    // for the active step; other steps never take a sign flag, so a `sign`
    // arg on them is silently ignored ( and is never set today anyway ).
    let sign_flag: Option<&str> = if sign {
        match step {
            "commit" => Some("--sign-commit"),
            "tag" => Some("--sign-tag"),
            _ => None,
        }
    } else {
        None
    };
    let mut cmd = Command::new(cargo.path());
    cmd.arg("release");
    for a in step_args {
        cmd.arg(a);
    }
    if pass_packages {
        for c in crates {
            cmd.arg("-p").arg(c);
        }
    }
    if let Some(flag) = sign_flag {
        cmd.arg(flag);
    }
    if no_confirm {
        cmd.arg("--no-confirm");
    }
    cmd.arg("--execute");
    cmd.current_dir(root);
    let p_args: String = if pass_packages {
        crates.iter().map(|c| format!(" -p {c}")).collect()
    } else {
        String::new()
    };
    eprintln!(
        "$ cargo release {}{}{}{}",
        step_args.join(" "),
        p_args,
        sign_flag.map(|f| format!(" {f}")).unwrap_or_default(),
        if no_confirm { " --no-confirm" } else { "" },
    );
    run_with_tee(&mut cmd)
}

fn read_versions(
    cargo: &CargoExe,
    root: &Path,
    crates: &[String],
) -> Result<BTreeMap<String, String>, Error> {
    let meta = cargo_metadata(cargo, root)?;
    let wanted: BTreeSet<&str> = crates.iter().map(|s| s.as_str()).collect();
    let mut out = BTreeMap::new();
    for pkg in &meta.packages {
        if wanted.contains(pkg.name.as_str()) {
            out.insert(pkg.name.clone(), pkg.version.clone());
        }
    }
    Ok(out)
}

fn build_commit_message(crates: &[String], versions: &BTreeMap<String, String>) -> String {
    let mut s = String::from("chore: release\n\n");
    for name in crates {
        match versions.get(name) {
            Some(v) => s.push_str(&format!("- {name} v{v}\n")),
            None => s.push_str(&format!("- {name}\n")),
        }
    }
    s
}

fn tag_message(crate_name: &str, version: &str) -> String {
    format!("chore: release {crate_name} v{version}")
}

fn amend_commit(root: &Path, message: &str, sign: bool) -> Result<(), Error> {
    let mut cmd = Command::new("git");
    cmd.arg("commit").arg("--amend");
    if sign {
        cmd.arg("-S");
    }
    cmd.arg("-m").arg(message);
    cmd.current_dir(root);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git commit --amend".to_string(),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(())
}

fn create_tag(root: &Path, tag: &str, message: &str, sign: bool) -> Result<(), Error> {
    let mut cmd = Command::new("git");
    cmd.arg("tag").arg(if sign { "-s" } else { "-a" });
    cmd.args(["-m", message]).arg(tag);
    cmd.current_dir(root);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git tag".to_string(),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(())
}

fn git_push(root: &Path, refs: &[String]) -> Result<(), Error> {
    let mut cmd = Command::new("git");
    cmd.arg("push").arg("origin");
    for r in refs {
        cmd.arg(r);
    }
    cmd.current_dir(root);
    eprintln!("$ git push origin {}", refs.join(" "));
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git push".to_string(),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    if !out.stderr.is_empty() {
        std::io::stderr().write_all(&out.stderr)?;
    }
    Ok(())
}

fn read_head_message(root: &Path) -> Result<String, Error> {
    let out = Command::new("git")
        .args(["log", "-1", "--format=%B"])
        .current_dir(root)
        .output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git log -1 --format=%B".to_string(),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8(out.stdout)?)
}

fn git_status_dirty(root: &Path) -> Result<bool, Error> {
    let out = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(root)
        .output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git status --porcelain".to_string(),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(!out.stdout.iter().all(u8::is_ascii_whitespace))
}

fn local_tags(root: &Path) -> Result<BTreeSet<String>, Error> {
    let out = Command::new("git")
        .args(["tag", "-l"])
        .current_dir(root)
        .output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git tag -l".to_string(),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8(out.stdout)?
        .lines()
        .map(|l| l.to_string())
        .collect())
}

fn remote_refs(root: &Path) -> Result<BTreeMap<String, String>, Error> {
    let out = Command::new("git")
        .args(["ls-remote", "origin"])
        .current_dir(root)
        .output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git ls-remote origin".to_string(),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    let text = String::from_utf8(out.stdout)?;
    let mut map = BTreeMap::new();
    for line in text.lines() {
        if let Some((sha, refname)) = line.split_once('\t') {
            map.insert(refname.to_string(), sha.to_string());
        }
    }
    Ok(map)
}

fn current_branch(root: &Path) -> Result<String, Error> {
    let out = Command::new("git")
        .args(["symbolic-ref", "--short", "HEAD"])
        .current_dir(root)
        .output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: "git symbolic-ref --short HEAD".to_string(),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8(out.stdout)?.trim().to_string())
}

fn rev_parse(root: &Path, refname: &str) -> Result<String, Error> {
    let out = Command::new("git")
        .args(["rev-parse", refname])
        .current_dir(root)
        .output()?;
    if !out.status.success() {
        return Err(Error::GitFailed {
            cmd: format!("git rev-parse {refname}"),
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8(out.stdout)?.trim().to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn build_commit_message_lists_each_crate() {
        let crates = vec!["winterbaume-foo".to_string(), "winterbaume-bar".to_string()];
        let mut versions = BTreeMap::new();
        versions.insert("winterbaume-foo".to_string(), "0.3.0".to_string());
        versions.insert("winterbaume-bar".to_string(), "1.0.0".to_string());
        let msg = build_commit_message(&crates, &versions);
        assert_eq!(
            msg,
            "chore: release\n\n- winterbaume-foo v0.3.0\n- winterbaume-bar v1.0.0\n"
        );
    }

    #[test]
    fn build_commit_message_preserves_topo_order() {
        let crates = vec![
            "winterbaume-z-dep".to_string(),
            "winterbaume-a-leaf".to_string(),
        ];
        let mut versions = BTreeMap::new();
        versions.insert("winterbaume-z-dep".to_string(), "0.1.0".to_string());
        versions.insert("winterbaume-a-leaf".to_string(), "0.1.0".to_string());
        let msg = build_commit_message(&crates, &versions);
        let lines: Vec<&str> = msg.lines().filter(|l| l.starts_with('-')).collect();
        assert_eq!(
            lines,
            vec!["- winterbaume-z-dep v0.1.0", "- winterbaume-a-leaf v0.1.0"]
        );
    }

    #[test]
    fn build_commit_message_handles_missing_version() {
        let crates = vec!["winterbaume-foo".to_string()];
        let versions = BTreeMap::new();
        let msg = build_commit_message(&crates, &versions);
        assert_eq!(msg, "chore: release\n\n- winterbaume-foo\n");
    }

    fn versions(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn classify_head_amended_for_this_chunk() {
        let msg = "chore: release\n\n- winterbaume-foo v0.3.0\n- winterbaume-bar v0.3.0\n";
        let chunk = vec!["winterbaume-foo".into(), "winterbaume-bar".into()];
        let targets = versions(&[("winterbaume-foo", "0.3.0"), ("winterbaume-bar", "0.3.0")]);
        assert_eq!(
            classify_head_message(msg, &chunk, &targets),
            HeadState::AmendedForThisChunk
        );
    }

    #[test]
    fn classify_head_placeholder_when_body_empty() {
        let msg = "chore: release\n";
        let chunk = vec!["winterbaume-foo".into()];
        let targets = versions(&[("winterbaume-foo", "0.3.0")]);
        assert_eq!(
            classify_head_message(msg, &chunk, &targets),
            HeadState::CargoReleasePlaceholder
        );
    }

    #[test]
    fn classify_head_placeholder_with_template_literals() {
        // cargo-release's default template leaves placeholders unrendered for
        // consolidated commits; the body is still empty, so we treat it as a
        // placeholder commit that just needs amending.
        let msg = "chore: release {{crate_name}} v{{version}}\n";
        let chunk = vec!["winterbaume-foo".into()];
        let targets = versions(&[("winterbaume-foo", "0.3.0")]);
        assert_eq!(
            classify_head_message(msg, &chunk, &targets),
            HeadState::CargoReleasePlaceholder
        );
    }

    #[test]
    fn classify_head_placeholder_with_rendered_single_crate() {
        // Non-consolidated path (or single-crate run) renders the placeholders
        // — still a release commit awaiting our amend body.
        let msg = "chore: release winterbaume-foo v0.3.0\n";
        let chunk = vec!["winterbaume-foo".into()];
        let targets = versions(&[("winterbaume-foo", "0.3.0")]);
        assert_eq!(
            classify_head_message(msg, &chunk, &targets),
            HeadState::CargoReleasePlaceholder
        );
    }

    #[test]
    fn classify_head_unrelated_when_subject_differs() {
        let msg = "fix: typo in README\n";
        let chunk = vec!["winterbaume-foo".into()];
        let targets = versions(&[("winterbaume-foo", "0.3.0")]);
        match classify_head_message(msg, &chunk, &targets) {
            HeadState::Unrelated { subject } => assert_eq!(subject, "fix: typo in README"),
            other => panic!("expected Unrelated, got {other:?}"),
        }
    }

    #[test]
    fn classify_head_unrelated_when_body_lists_different_crates() {
        // A previous chunk's amended commit: subject matches but body lists
        // crates that are not in this chunk.
        let msg = "chore: release\n\n- winterbaume-other v0.3.0\n";
        let chunk = vec!["winterbaume-foo".into()];
        let targets = versions(&[("winterbaume-foo", "0.3.0")]);
        assert!(matches!(
            classify_head_message(msg, &chunk, &targets),
            HeadState::Unrelated { .. }
        ));
    }

    #[test]
    fn classify_head_unrelated_when_versions_differ() {
        let msg = "chore: release\n\n- winterbaume-foo v0.2.0\n";
        let chunk = vec!["winterbaume-foo".into()];
        let targets = versions(&[("winterbaume-foo", "0.3.0")]);
        assert!(matches!(
            classify_head_message(msg, &chunk, &targets),
            HeadState::Unrelated { .. }
        ));
    }

    #[test]
    fn parse_amend_body_extracts_pairs() {
        let msg = "chore: release\n\n- winterbaume-foo v0.3.0\n- winterbaume-bar v1.0.0-beta.1\n";
        let pairs = parse_amend_body(msg);
        assert_eq!(
            pairs,
            vec![
                ("winterbaume-foo".to_string(), "0.3.0".to_string()),
                ("winterbaume-bar".to_string(), "1.0.0-beta.1".to_string()),
            ]
        );
    }

    #[test]
    fn parse_amend_body_ignores_non_dash_lines() {
        let msg = "chore: release\n\nsome prose\n- winterbaume-foo v0.3.0\nmore prose\n";
        let pairs = parse_amend_body(msg);
        assert_eq!(
            pairs,
            vec![("winterbaume-foo".to_string(), "0.3.0".to_string())]
        );
    }

    #[test]
    fn should_run_version_literal_skips_when_matched() {
        let chunk = vec!["winterbaume-foo".into(), "winterbaume-bar".into()];
        let mfs = versions(&[("winterbaume-foo", "0.3.0"), ("winterbaume-bar", "0.3.0")]);
        assert!(!should_run_version_step("0.3.0", &mfs, &chunk));
    }

    #[test]
    fn should_run_version_literal_runs_when_mismatched() {
        let chunk = vec!["winterbaume-foo".into(), "winterbaume-bar".into()];
        let mfs = versions(&[("winterbaume-foo", "0.2.0"), ("winterbaume-bar", "0.3.0")]);
        assert!(should_run_version_step("0.3.0", &mfs, &chunk));
    }

    #[test]
    fn tag_message_format() {
        assert_eq!(
            tag_message("winterbaume-foo", "0.3.0"),
            "chore: release winterbaume-foo v0.3.0"
        );
    }
}
