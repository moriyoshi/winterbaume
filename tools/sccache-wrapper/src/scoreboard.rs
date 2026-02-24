// Per-crate scoreboard so external observers can see which compilations are
// currently running, which sessions are contending for the same cache key,
// and how long each build has been (or was) running.
//
// Layout under `$WB_SCCACHE_WRAPPER_SCOREBOARD/`:
//   <crate>-<short_key>.json        # one file per cache key
//   <crate>-<short_key>.json.lock   # fslock sibling for serialised RMW
//
// Each session that is about to compile registers a Session, which:
//   1. Adds a `Contender` entry for this pid (with process tree, session id,
//      role) to the scoreboard file.
//   2. Spawns a heartbeat thread that bumps `last_seen` every few seconds.
//   3. On Drop, removes the contender and (if it was the builder) marks the
//      entry Ready with a final `completed_at`.
//
// Pruning is opportunistic: every read-modify-write drops contenders that
// haven't heartbeat'd within `STALE_CONTENDER_AFTER`, and removes empty
// abandoned `Building` entries / aged-out `Ready` entries.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const STALE_CONTENDER_AFTER: Duration = Duration::from_secs(30);
const KEEP_READY_FOR: Duration = Duration::from_secs(300);
const MAX_TREE_DEPTH: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Building,
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Waiting,
    Building,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub comm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contender {
    pub pid: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    pub role: Role,
    pub joined_at: u64,
    pub last_seen: u64,
    pub process_tree: Vec<ProcessInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub crate_name: String,
    pub extra_filename: String,
    pub cache_key: String,
    pub command_line: Vec<String>,
    pub state: State,
    pub started_at: u64,
    pub last_update: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<u64>,
    pub contenders: Vec<Contender>,
}

// ---------------------------------------------------------------------------
// Tracker — shared per-process configuration
// ---------------------------------------------------------------------------

pub struct Tracker {
    dir: PathBuf,
    pid: u32,
    session: Option<String>,
    process_tree: Vec<ProcessInfo>,
}

impl Tracker {
    /// Returns `None` if `WB_SCCACHE_WRAPPER_SCOREBOARD` is unset or the dir
    /// cannot be created.
    pub fn from_env() -> Option<Arc<Self>> {
        let dir = std::env::var_os("WB_SCCACHE_WRAPPER_SCOREBOARD")?;
        let dir = PathBuf::from(dir);
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!(
                "sccache-wrapper: scoreboard dir {} unusable: {e}",
                dir.display()
            );
            return None;
        }
        let pid = std::process::id();
        Some(Arc::new(Self {
            dir,
            pid,
            session: detect_session(),
            process_tree: collect_process_tree(pid),
        }))
    }
}

fn detect_session() -> Option<String> {
    let dir = std::env::var("CARGO_TARGET_DIR").ok()?;
    let path = Path::new(&dir);
    let name = path.file_name()?.to_string_lossy().into_owned();
    name.strip_prefix("target-").map(|s| s.to_owned())
}

fn collect_process_tree(start_pid: u32) -> Vec<ProcessInfo> {
    use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

    let mut sys = System::new();
    let mut tree = Vec::new();
    let mut current = Pid::from_u32(start_pid);
    let mut depth = 0;

    while depth < MAX_TREE_DEPTH {
        sys.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[current]),
            true,
            ProcessRefreshKind::new(),
        );
        let Some(proc) = sys.process(current) else {
            break;
        };
        let comm = proc.name().to_string_lossy().into_owned();
        let parent = proc.parent();
        tree.push(ProcessInfo {
            pid: current.as_u32(),
            comm,
        });
        let Some(parent) = parent else { break };
        if parent.as_u32() <= 1 {
            break;
        }
        current = parent;
        depth += 1;
    }
    tree
}

// ---------------------------------------------------------------------------
// File I/O helpers — RMW under fslock with atomic rename
// ---------------------------------------------------------------------------

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn entry_filename(crate_name: &str, cache_key: &str) -> String {
    let safe: String = crate_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let short = &cache_key[..12.min(cache_key.len())];
    format!("{safe}-{short}.json")
}

/// Outcome from the closure passed to [`with_lock`].
enum Mutation {
    Keep(Entry),
    Remove,
    Untouched,
}

/// Acquire the per-file lock, read (if present), apply `f`, and persist the
/// result atomically.  Returns whatever `f` returns alongside its `Mutation`.
fn with_lock<F, R>(tracker: &Tracker, filename: &str, f: F) -> Option<R>
where
    F: FnOnce(Option<Entry>) -> (Mutation, R),
{
    let entry_path = tracker.dir.join(filename);
    let lock_path = tracker.dir.join(format!("{filename}.lock"));

    let mut flock = fslock::LockFile::open(&lock_path).ok()?;
    flock.lock().ok()?;

    let existing = fs::read(&entry_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Entry>(&bytes).ok());

    let (mutation, result) = f(existing);

    match mutation {
        Mutation::Keep(entry) => {
            if let Ok(serialised) = serde_json::to_vec_pretty(&entry) {
                let tmp = entry_path.with_extension(format!("json.tmp.{}", std::process::id()));
                if fs::write(&tmp, serialised).is_ok() && fs::rename(&tmp, &entry_path).is_err() {
                    // Rename failed (e.g. cross-device, permission) — clear
                    // the staged file so we don't leak `json.tmp.<pid>`.
                    let _ = fs::remove_file(&tmp);
                }
            }
        }
        Mutation::Remove => {
            let _ = fs::remove_file(&entry_path);
        }
        Mutation::Untouched => {}
    }

    let _ = flock.unlock();
    Some(result)
}

/// Drop contenders whose last heartbeat is older than [`STALE_CONTENDER_AFTER`].
fn prune_stale_contenders(entry: &mut Entry, now: u64) {
    entry
        .contenders
        .retain(|c| now.saturating_sub(c.last_seen) < STALE_CONTENDER_AFTER.as_secs());
}

/// Decide whether an entry with no live contenders should be persisted or
/// removed.  `Building` with no contenders means the build was abandoned;
/// `Ready` is kept until [`KEEP_READY_FOR`] elapses.
fn finalise(entry: Entry, now: u64) -> Mutation {
    if !entry.contenders.is_empty() {
        return Mutation::Keep(entry);
    }
    match entry.state {
        State::Building => Mutation::Remove,
        State::Ready => match entry.completed_at {
            Some(c) if now.saturating_sub(c) > KEEP_READY_FOR.as_secs() => Mutation::Remove,
            _ => Mutation::Keep(entry),
        },
    }
}

// ---------------------------------------------------------------------------
// Session — handle on a single per-key registration
// ---------------------------------------------------------------------------

pub struct Session {
    inner: Arc<SessionInner>,
    thread: Option<JoinHandle<()>>,
}

struct SessionInner {
    tracker: Arc<Tracker>,
    filename: String,
    shutdown: Shutdown,
    finished: AtomicBool,
}

struct Shutdown {
    flag: Mutex<bool>,
    cv: Condvar,
}

impl Shutdown {
    fn new() -> Self {
        Self {
            flag: Mutex::new(false),
            cv: Condvar::new(),
        }
    }

    fn signal(&self) {
        let mut g = self.flag.lock().unwrap();
        *g = true;
        self.cv.notify_all();
    }

    /// Returns `true` if the shutdown was signalled before the timeout.
    fn wait(&self, dur: Duration) -> bool {
        let g = self.flag.lock().unwrap();
        let (g, _) = self.cv.wait_timeout(g, dur).unwrap();
        *g
    }
}

impl Session {
    /// Register this process as a participant for `cache_key`.
    pub fn start(
        tracker: Arc<Tracker>,
        cache_key: &str,
        crate_name: &str,
        extra_filename: &str,
        command_line: &[String],
        initial_role: Role,
    ) -> Self {
        let filename = entry_filename(crate_name, cache_key);
        let now = now_secs();
        let pid = tracker.pid;
        let session_id = tracker.session.clone();
        let process_tree = tracker.process_tree.clone();

        with_lock(&tracker, &filename, |existing| {
            let mut entry = match existing {
                Some(mut e) => {
                    prune_stale_contenders(&mut e, now);
                    e.contenders
                        .retain(|c| !(c.pid == pid && c.session == session_id));
                    e
                }
                None => Entry {
                    crate_name: crate_name.to_owned(),
                    extra_filename: extra_filename.to_owned(),
                    cache_key: cache_key.to_owned(),
                    command_line: command_line.to_vec(),
                    state: State::Building,
                    started_at: now,
                    last_update: now,
                    completed_at: None,
                    contenders: Vec::new(),
                },
            };
            entry.contenders.push(Contender {
                pid,
                session: session_id.clone(),
                role: initial_role,
                joined_at: now,
                last_seen: now,
                process_tree: process_tree.clone(),
            });
            if initial_role == Role::Building && entry.state != State::Building {
                entry.state = State::Building;
                entry.started_at = now;
                entry.completed_at = None;
            }
            entry.last_update = now;
            (Mutation::Keep(entry), ())
        });

        let inner = Arc::new(SessionInner {
            tracker,
            filename,
            shutdown: Shutdown::new(),
            finished: AtomicBool::new(false),
        });

        let thread_inner = inner.clone();
        let handle = thread::spawn(move || {
            while !thread_inner.shutdown.wait(HEARTBEAT_INTERVAL) {
                thread_inner.heartbeat();
            }
        });

        Self {
            inner,
            thread: Some(handle),
        }
    }

    /// Promote ourselves from `Waiting` to `Building` after acquiring the
    /// singleflight lock.
    pub fn promote_to_builder(&self) {
        let now = now_secs();
        let pid = self.inner.tracker.pid;
        let session_id = self.inner.tracker.session.clone();
        with_lock(&self.inner.tracker, &self.inner.filename, |existing| {
            let Some(mut entry) = existing else {
                return (Mutation::Untouched, ());
            };
            prune_stale_contenders(&mut entry, now);
            for c in &mut entry.contenders {
                if c.pid == pid && c.session == session_id {
                    c.role = Role::Building;
                    c.last_seen = now;
                }
            }
            if entry.state != State::Building {
                entry.state = State::Building;
                entry.started_at = now;
                entry.completed_at = None;
            }
            entry.last_update = now;
            (Mutation::Keep(entry), ())
        });
    }

    /// Mark the build as finished successfully.  The entry remains in the
    /// scoreboard with `state = Ready` until it is pruned.
    pub fn mark_ready(&self) {
        let now = now_secs();
        with_lock(&self.inner.tracker, &self.inner.filename, |existing| {
            let Some(mut entry) = existing else {
                return (Mutation::Untouched, ());
            };
            entry.state = State::Ready;
            entry.completed_at = Some(now);
            entry.last_update = now;
            (Mutation::Keep(entry), ())
        });
    }
}

impl SessionInner {
    fn heartbeat(&self) {
        if self.finished.load(Ordering::SeqCst) {
            return;
        }
        let now = now_secs();
        let pid = self.tracker.pid;
        let session_id = self.tracker.session.clone();
        with_lock(&self.tracker, &self.filename, |existing| {
            let Some(mut entry) = existing else {
                return (Mutation::Untouched, ());
            };
            let mut found = false;
            for c in &mut entry.contenders {
                if c.pid == pid && c.session == session_id {
                    c.last_seen = now;
                    found = true;
                }
            }
            if !found {
                return (Mutation::Keep(entry), ());
            }
            if entry.state == State::Building {
                entry.last_update = now;
            }
            (Mutation::Keep(entry), ())
        });
    }

    fn unregister(&self) {
        if self.finished.swap(true, Ordering::SeqCst) {
            return;
        }
        let now = now_secs();
        let pid = self.tracker.pid;
        let session_id = self.tracker.session.clone();
        with_lock(&self.tracker, &self.filename, |existing| {
            let Some(mut entry) = existing else {
                return (Mutation::Untouched, ());
            };
            entry
                .contenders
                .retain(|c| !(c.pid == pid && c.session == session_id));
            prune_stale_contenders(&mut entry, now);
            entry.last_update = now;
            (finalise(entry, now), ())
        });
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.inner.shutdown.signal();
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        self.inner.unregister();
    }
}

// ---------------------------------------------------------------------------
// One-shot directory prune
// ---------------------------------------------------------------------------

/// Walk the scoreboard directory once, locking each entry file individually
/// and applying the same prune-stale-contenders + finalise logic that the
/// per-session RMW path uses on every heartbeat.  Cache-hit fast paths skip
/// the heartbeat entirely, so without this sweep `Ready` entries can outlive
/// their [`KEEP_READY_FOR`] window and `Building` entries can persist with
/// no live contenders after the originating builder dies abnormally.
///
/// Best-effort: I/O errors on individual entries (lock failure, read failure,
/// rename failure) are swallowed so a single corrupt file does not block the
/// caller — the next sweep will retry.
pub fn prune_dir(dir: &Path, now: u64) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    for e in rd.flatten() {
        let path = e.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let Some(filename) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        prune_one(dir, filename, now);
    }
}

fn prune_one(dir: &Path, filename: &str, now: u64) {
    let entry_path = dir.join(filename);
    let lock_path = dir.join(format!("{filename}.lock"));

    let Ok(mut flock) = fslock::LockFile::open(&lock_path) else {
        return;
    };
    if flock.lock().is_err() {
        return;
    }

    let outcome = (|| -> Option<Mutation> {
        let bytes = fs::read(&entry_path).ok()?;
        let mut entry = serde_json::from_slice::<Entry>(&bytes).ok()?;
        prune_stale_contenders(&mut entry, now);
        Some(finalise(entry, now))
    })();

    match outcome {
        Some(Mutation::Keep(entry)) => {
            if let Ok(serialised) = serde_json::to_vec_pretty(&entry) {
                let tmp = entry_path.with_extension(format!("json.tmp.{}", std::process::id()));
                if fs::write(&tmp, serialised).is_ok() && fs::rename(&tmp, &entry_path).is_err() {
                    let _ = fs::remove_file(&tmp);
                }
            }
        }
        Some(Mutation::Remove) => {
            let _ = fs::remove_file(&entry_path);
        }
        Some(Mutation::Untouched) | None => {}
    }

    let _ = flock.unlock();
}

// ---------------------------------------------------------------------------
// Diagnostic dump
// ---------------------------------------------------------------------------

pub fn show_scoreboard() -> io::Result<()> {
    let dir = match std::env::var_os("WB_SCCACHE_WRAPPER_SCOREBOARD") {
        Some(d) => PathBuf::from(d),
        None => {
            eprintln!("sccache-wrapper: WB_SCCACHE_WRAPPER_SCOREBOARD not set");
            return Ok(());
        }
    };
    if !dir.exists() {
        println!("(no scoreboard dir at {})", dir.display());
        return Ok(());
    }

    let now = now_secs();
    // Cache-hit fast paths skip the per-session R-M-W, so stale `Ready` and
    // abandoned `Building` entries can accumulate.  Run a one-shot prune over
    // the directory before listing so the output reflects current liveness.
    prune_dir(&dir, now);

    let mut entries: Vec<Entry> = Vec::new();
    for e in fs::read_dir(&dir)? {
        let path = e?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        if let Ok(bytes) = fs::read(&path)
            && let Ok(entry) = serde_json::from_slice::<Entry>(&bytes)
        {
            entries.push(entry);
        }
    }
    entries.sort_by_key(|b| std::cmp::Reverse(b.started_at));

    for e in &entries {
        let dur = match (e.state, e.completed_at) {
            (State::Ready, Some(c)) => format!("{}s (done)", c.saturating_sub(e.started_at)),
            (State::Ready, None) => "?s (done)".to_owned(),
            _ => format!("{}s (live)", now.saturating_sub(e.started_at)),
        };
        let state = match e.state {
            State::Building => "building",
            State::Ready => "ready",
        };
        println!(
            "--- {} ({}) {state} {dur} ---",
            e.crate_name,
            &e.cache_key[..12.min(e.cache_key.len())],
        );
        println!("  cmd:  {}", shell_join(&e.command_line));
        if e.contenders.is_empty() {
            println!("  contenders: (none)");
        } else {
            println!("  contenders:");
            for c in &e.contenders {
                let role = match c.role {
                    Role::Building => "B",
                    Role::Waiting => "W",
                };
                let session = c.session.as_deref().unwrap_or("-");
                let chain = c
                    .process_tree
                    .iter()
                    .map(|p| format!("{}({})", p.comm, p.pid))
                    .collect::<Vec<_>>()
                    .join(" → ");
                println!(
                    "    [{role}] pid={} session={session} {chain} (last_seen {}s ago)",
                    c.pid,
                    now.saturating_sub(c.last_seen),
                );
            }
        }
    }
    println!(
        "\n{} scoreboard entries in {}",
        entries.len(),
        dir.display()
    );
    Ok(())
}

fn shell_join(args: &[String]) -> String {
    args.iter()
        .map(|a| {
            if a.is_empty() || a.contains(' ') || a.contains('"') || a.contains('\'') {
                format!("'{}'", a.replace('\'', "'\\''"))
            } else {
                a.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_tmp_dir(label: &str) -> PathBuf {
        let nonce = format!("{}-{}-{}", std::process::id(), label, now_secs());
        let dir = std::env::temp_dir().join(format!("wb-scoreboard-test-{nonce}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_entry(dir: &Path, key: &str, entry: &Entry) {
        let path = dir.join(entry_filename(&entry.crate_name, key));
        fs::write(&path, serde_json::to_vec_pretty(entry).unwrap()).unwrap();
    }

    fn ready(now: u64, completed_secs_ago: u64) -> Entry {
        let completed = now.saturating_sub(completed_secs_ago);
        Entry {
            crate_name: "demo".to_owned(),
            extra_filename: "-abc".to_owned(),
            cache_key: "k".to_owned(),
            command_line: vec!["rustc".to_owned()],
            state: State::Ready,
            started_at: completed.saturating_sub(1),
            last_update: completed,
            completed_at: Some(completed),
            contenders: vec![],
        }
    }

    fn building(now: u64, contenders: Vec<Contender>) -> Entry {
        Entry {
            crate_name: "demo".to_owned(),
            extra_filename: "-abc".to_owned(),
            cache_key: "k".to_owned(),
            command_line: vec!["rustc".to_owned()],
            state: State::Building,
            started_at: now.saturating_sub(10),
            last_update: now.saturating_sub(10),
            completed_at: None,
            contenders,
        }
    }

    fn contender(pid: u32, last_seen: u64) -> Contender {
        Contender {
            pid,
            session: None,
            role: Role::Building,
            joined_at: last_seen,
            last_seen,
            process_tree: vec![],
        }
    }

    #[test]
    fn prune_removes_ready_entry_past_keep_window() {
        let dir = unique_tmp_dir("ready-stale");
        let now: u64 = 10_000;
        write_entry(&dir, "stale", &ready(now, KEEP_READY_FOR.as_secs() + 60));
        prune_dir(&dir, now);
        assert!(!dir.join(entry_filename("demo", "stale")).exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn prune_keeps_ready_entry_within_keep_window() {
        let dir = unique_tmp_dir("ready-fresh");
        let now: u64 = 10_000;
        write_entry(&dir, "fresh", &ready(now, KEEP_READY_FOR.as_secs() / 2));
        prune_dir(&dir, now);
        assert!(dir.join(entry_filename("demo", "fresh")).exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn prune_removes_building_entry_with_no_live_contenders() {
        let dir = unique_tmp_dir("abandoned");
        let now: u64 = 10_000;
        let stale = now.saturating_sub(STALE_CONTENDER_AFTER.as_secs() + 60);
        let entry = building(now, vec![contender(123, stale)]);
        write_entry(&dir, "abandoned", &entry);
        prune_dir(&dir, now);
        assert!(!dir.join(entry_filename("demo", "abandoned")).exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn prune_keeps_building_entry_with_live_contender() {
        let dir = unique_tmp_dir("live-build");
        let now: u64 = 10_000;
        let entry = building(now, vec![contender(456, now)]);
        write_entry(&dir, "live", &entry);
        prune_dir(&dir, now);
        let kept_path = dir.join(entry_filename("demo", "live"));
        assert!(kept_path.exists());
        // The live contender should still be present after prune.
        let bytes = fs::read(&kept_path).unwrap();
        let restored: Entry = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(restored.contenders.len(), 1);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn prune_drops_stale_contenders_but_keeps_entry_when_others_remain() {
        let dir = unique_tmp_dir("mixed");
        let now: u64 = 10_000;
        let stale = now.saturating_sub(STALE_CONTENDER_AFTER.as_secs() + 60);
        let entry = building(now, vec![contender(123, stale), contender(456, now)]);
        write_entry(&dir, "mixed", &entry);
        prune_dir(&dir, now);
        let kept_path = dir.join(entry_filename("demo", "mixed"));
        assert!(kept_path.exists());
        let restored: Entry = serde_json::from_slice(&fs::read(&kept_path).unwrap()).unwrap();
        assert_eq!(restored.contenders.len(), 1);
        assert_eq!(restored.contenders[0].pid, 456);
        let _ = fs::remove_dir_all(&dir);
    }
}
