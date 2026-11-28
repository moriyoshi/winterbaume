//! What the executor and the VM helper say to each other.
//!
//! The executor writes one JSON [`VmSpec`] to the helper's stdin and closes
//! it.  The helper answers with exactly one status line on stderr ( see
//! [`HelperStatus`] ) before it boots the VM; everything the helper's stdout
//! and stderr carry after that is the guest's console, which is the
//! function's log.  The helper's exit status is the guest's.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Prefix of the helper's status line.  The record separator keeps it from
/// being mistaken for guest output.
pub const STATUS_PREFIX: &str = "\u{1e}winterbaume-lambda-vm ";

/// Path of the launch script inside the guest.  It is not a file of the root
/// file system but an in-memory overlay added by the helper.
pub const LAUNCH_SCRIPT: &str = "/.winterbaume-launch.sh";
/// Path of the guest agent inside the guest, an in-memory overlay too.
pub const AGENT_PATH: &str = "/.winterbaume-agent";

/// Guest port of the Lambda Runtime API ( `AWS_LAMBDA_RUNTIME_API` ).
pub const RUNTIME_API_GUEST_PORT: u16 = 9001;
/// Vsock port the guest agent relays the Runtime API to.
pub const RUNTIME_API_VSOCK_PORT: u32 = 1000;
/// Guest port of the AWS endpoint ( `AWS_ENDPOINT_URL` ).
pub const AWS_ENDPOINT_GUEST_PORT: u16 = 4566;
/// Vsock port the guest agent relays the AWS endpoint to.
pub const AWS_ENDPOINT_VSOCK_PORT: u32 = 1001;

/// One guest TCP port that the guest agent relays to the host: the agent
/// listens on guest `127.0.0.1:guest_port`, connects to vsock `vsock_port`,
/// and libkrun connects that to the Unix socket `host_socket`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Forward {
    pub guest_port: u16,
    pub vsock_port: u32,
    pub host_socket: PathBuf,
}

/// One microVM to boot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmSpec {
    /// libkrun to load instead of searching the default locations.
    #[serde(default)]
    pub libkrun: Option<PathBuf>,
    /// Host directory that becomes the guest's root file system, read-only.
    pub root: PathBuf,
    pub vcpus: u8,
    pub ram_mib: u32,
    /// The complete environment of the runtime process.
    pub env: BTreeMap<String, String>,
    /// Programs tried in order as the runtime's bootstrap.
    pub bootstrap: Vec<String>,
    /// Working directory of the runtime process.
    pub workdir: String,
    /// libkrun log level ( 0 off .. 5 trace ).
    #[serde(default)]
    pub log_level: u32,
    /// Only check that a VM could be started ( libkrun loads, the
    /// hypervisor is usable ), report and exit.
    #[serde(default)]
    pub probe: bool,
    /// The guest agent binary, built for the guest's architecture.
    #[serde(default)]
    pub agent: Option<PathBuf>,
    /// Guest ports relayed to the host by the agent.
    #[serde(default)]
    pub forwards: Vec<Forward>,
    /// Give the guest the host's network ( libkrun's transparent socket
    /// impersonation ).  Without it the guest can reach nothing but the
    /// forwarded ports.
    #[serde(default)]
    pub network: bool,
}

impl VmSpec {
    /// A spec that only probes.
    pub fn probe(libkrun: Option<PathBuf>) -> Self {
        Self {
            libkrun,
            root: PathBuf::new(),
            vcpus: 1,
            ram_mib: 128,
            env: BTreeMap::new(),
            bootstrap: Vec::new(),
            workdir: "/".to_string(),
            log_level: 0,
            probe: true,
            agent: None,
            forwards: Vec::new(),
            network: false,
        }
    }
}

/// The helper's status line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HelperStatus {
    /// The VM is configured and about to boot.
    Booting,
    /// The VM could not be started; nothing ran.  `reason` is a stub reason
    /// value ( `libkrun-unavailable`, `vm-unavailable`, ... ).
    Failed { reason: String, message: String },
}

impl HelperStatus {
    pub fn to_line(&self) -> String {
        match self {
            HelperStatus::Booting => format!("{STATUS_PREFIX}booting\n"),
            HelperStatus::Failed { reason, message } => {
                let message = message.replace(['\n', '\r'], " ");
                format!("{STATUS_PREFIX}failed {reason} {message}\n")
            }
        }
    }

    /// Parse a status line ( with or without its newline ).
    pub fn parse(line: &str) -> Option<Self> {
        let rest = line
            .trim_end_matches(['\n', '\r'])
            .strip_prefix(STATUS_PREFIX)?;
        if rest == "booting" {
            return Some(HelperStatus::Booting);
        }
        let failed = rest.strip_prefix("failed ")?;
        let (reason, message) = failed.split_once(' ').unwrap_or((failed, ""));
        Some(HelperStatus::Failed {
            reason: reason.to_string(),
            message: message.to_string(),
        })
    }
}

/// The exit status the launch script uses when no bootstrap exists.
pub const NO_BOOTSTRAP_EXIT: i32 = 127;
/// The exit status the launch script uses when the guest agent fails.
pub const AGENT_FAILED_EXIT: i32 = 125;

/// The shell script that starts the runtime inside the guest: it sets the
/// environment, changes to the working directory and runs the first
/// bootstrap that exists.
///
/// The environment travels in the script rather than through libkrun, which
/// passes it on the kernel command line, where it does not fit.  Keys that
/// are not shell identifiers are dropped; Lambda rejects them too.
pub fn launch_script(spec: &VmSpec) -> String {
    let mut script = String::from("#!/bin/sh\n");
    for (key, value) in &spec.env {
        if !is_identifier(key) {
            continue;
        }
        script.push_str(&format!("export {key}={}\n", shell_quote(value)));
    }
    if !spec.forwards.is_empty() {
        // The agent prints `ready` and closes its stdout once it listens on
        // every port, which ends the command substitution; it keeps running
        // in the background.
        let ports: Vec<String> = spec
            .forwards
            .iter()
            .map(|f| format!("{}:{}", f.guest_port, f.vsock_port))
            .collect();
        script.push_str(&format!(
            "ready=$({AGENT_PATH} {} &)\nif [ \"$ready\" != ready ]; then\n  echo 'winterbaume: the guest agent did not start' >&2\n  exit {AGENT_FAILED_EXIT}\nfi\n",
            ports.join(" ")
        ));
    }
    script.push_str(&format!("cd {} 2>/dev/null\n", shell_quote(&spec.workdir)));
    let candidates: Vec<String> = spec.bootstrap.iter().map(|b| shell_quote(b)).collect();
    script.push_str(&format!(
        "for bootstrap in {}; do\n  if [ -x \"$bootstrap\" ]; then exec \"$bootstrap\"; fi\ndone\n",
        candidates.join(" ")
    ));
    script.push_str(&format!(
        "echo {} >&2\nexit {NO_BOOTSTRAP_EXIT}\n",
        shell_quote(&format!(
            "Couldn't find valid bootstrap(s): [{}]",
            spec.bootstrap.join(" ")
        ))
    ));
    script
}

fn is_identifier(key: &str) -> bool {
    let mut chars = key.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Quote `value` for a POSIX shell.
pub fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', r"'\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_lines_round_trip() {
        for status in [
            HelperStatus::Booting,
            HelperStatus::Failed {
                reason: "vm-unavailable".to_string(),
                message: "cannot open /dev/kvm".to_string(),
            },
        ] {
            assert_eq!(HelperStatus::parse(&status.to_line()), Some(status));
        }
        assert_eq!(HelperStatus::parse("booting\n"), None);
        assert_eq!(
            HelperStatus::parse(
                &HelperStatus::Failed {
                    reason: "x".into(),
                    message: "two\nlines".into()
                }
                .to_line()
            ),
            Some(HelperStatus::Failed {
                reason: "x".into(),
                message: "two lines".into()
            })
        );
    }

    #[test]
    fn launch_script_quotes_the_environment_and_tries_each_bootstrap() {
        let spec = VmSpec {
            libkrun: None,
            root: PathBuf::from("/r"),
            vcpus: 1,
            ram_mib: 128,
            env: BTreeMap::from([
                ("A".to_string(), "it's $HOME `x`".to_string()),
                ("BAD-KEY".to_string(), "dropped".to_string()),
                ("_H".to_string(), "index.handler".to_string()),
            ]),
            bootstrap: vec!["/var/task/bootstrap".into(), "/opt/bootstrap".into()],
            workdir: "/var/task".to_string(),
            log_level: 0,
            probe: false,
            agent: None,
            forwards: Vec::new(),
            network: false,
        };
        let script = launch_script(&spec);
        assert!(script.contains("export A='it'\\''s $HOME `x`'\n"));
        assert!(!script.contains("BAD-KEY"));
        assert!(script.contains("export _H='index.handler'\n"));
        assert!(script.contains("for bootstrap in '/var/task/bootstrap' '/opt/bootstrap'; do"));
        assert!(script.ends_with("exit 127\n"));

        // The script really runs under sh and exports what it says.
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("bootstrap");
        std::fs::write(&fake, "#!/bin/sh\nprintf '%s|%s' \"$A\" \"$_H\"\n").unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&fake, std::fs::Permissions::from_mode(0o755)).unwrap();
        let spec = VmSpec {
            bootstrap: vec!["/nonexistent/bootstrap".into(), fake.display().to_string()],
            workdir: dir.path().display().to_string(),
            ..spec
        };
        let out = std::process::Command::new("/bin/sh")
            .arg("-c")
            .arg(launch_script(&spec))
            .output()
            .unwrap();
        assert_eq!(
            String::from_utf8_lossy(&out.stdout),
            "it's $HOME `x`|index.handler"
        );

        let spec = VmSpec {
            bootstrap: vec!["/nonexistent/bootstrap".into()],
            ..spec
        };
        let out = std::process::Command::new("/bin/sh")
            .arg("-c")
            .arg(launch_script(&spec))
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(NO_BOOTSTRAP_EXIT));
        assert!(String::from_utf8_lossy(&out.stderr).contains("Couldn't find valid bootstrap(s)"));
    }
}
