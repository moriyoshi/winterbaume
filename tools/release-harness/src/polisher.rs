//! Pluggable "editorial polish" backends — driven by whichever AI coding CLI
//! the operator has installed.
//!
//! Each backend takes a single prompt over the wire and returns plain text on
//! stdout. The harness normalises that text by stripping anything before the
//! first `## ` heading, so preamble lines that some CLIs emit (token usage,
//! model name, "Thinking…" notes) don't bleed into the changelog.
//!
//! The set of supported backends is small and explicit. Adding a new one is
//! a matter of extending `Backend::ALL`, mapping it to a command line, and
//! noting any extraction quirks. We do NOT shell out to operator-supplied
//! arbitrary commands — that would be a generic "AI exec" tool, not a
//! release harness.

use std::path::Path;
use std::process::{Command, Stdio};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(
        "no supported AI CLI found on PATH (tried: {tried}). Install one of \
         claude / codex / copilot / cursor-agent, or pass --polisher <name> \
         to point at a specific binary."
    )]
    NoneAvailable { tried: String },
    #[error("requested polisher {0:?} is not on PATH")]
    NotInstalled(Backend),
    #[error("{backend:?} polish failed (status {status}):\n{stderr}")]
    Failed {
        backend: Backend,
        status: String,
        stderr: String,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

/// One of the AI coding CLIs the harness knows how to invoke for editorial
/// polish. Ordered by preference: `Backend::ALL` is the auto-detect search
/// order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Backend {
    /// Anthropic Claude Code CLI (`claude`).
    Claude,
    /// OpenAI Codex CLI (`codex exec ...`).
    Codex,
    /// GitHub Copilot CLI (`copilot`).
    Copilot,
    /// Cursor agent CLI (`cursor-agent`).
    Cursor,
}

impl Backend {
    pub const ALL: &'static [Backend] = &[
        Backend::Claude,
        Backend::Codex,
        Backend::Copilot,
        Backend::Cursor,
    ];

    pub fn binary(self) -> &'static str {
        match self {
            Backend::Claude => "claude",
            Backend::Codex => "codex",
            Backend::Copilot => "copilot",
            Backend::Cursor => "cursor-agent",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Backend::Claude => "claude",
            Backend::Codex => "codex",
            Backend::Copilot => "copilot",
            Backend::Cursor => "cursor",
        }
    }

    /// Parse the CLI flag / env var value into a backend.
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "claude" => Some(Backend::Claude),
            "codex" => Some(Backend::Codex),
            "copilot" | "github-copilot" | "gh-copilot" => Some(Backend::Copilot),
            "cursor" | "cursor-agent" => Some(Backend::Cursor),
            _ => None,
        }
    }

    /// Probe the backend's binary with a `--version` invocation. Cheap and
    /// universally supported across the four CLIs we target.
    pub fn is_installed(self) -> bool {
        Command::new(self.binary())
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Build the `Command` that runs this backend non-interactively with
    /// `prompt` as its input. Each CLI has its own conventions; collected
    /// here so callers don't have to.
    fn build_command(self, prompt: &str, root: &Path) -> Command {
        let mut cmd = Command::new(self.binary());
        match self {
            // `-p` is non-interactive print mode; `--output-format text` strips
            // the otherwise verbose status frame.
            Backend::Claude => {
                cmd.args([
                    "-p",
                    prompt,
                    "--model",
                    "claude-sonnet-4-6",
                    "--output-format",
                    "text",
                ]);
            }
            // Headless single-shot exec mode. Codex prints its own preamble
            // before the actual response, which `extract_markdown_section`
            // takes care of.
            Backend::Codex => {
                cmd.args(["exec", prompt]);
            }
            // GitHub Copilot CLI (the agentic `copilot` binary, not the older
            // `gh copilot` extension). `-p` is non-interactive.
            Backend::Copilot => {
                cmd.args(["-p", prompt]);
            }
            // Cursor agent CLI: `-p`/`--print` for non-interactive output.
            Backend::Cursor => {
                cmd.args(["-p", prompt]);
            }
        }
        cmd.current_dir(root);
        cmd
    }
}

/// Resolve which backend to use from an explicit choice + env override +
/// auto-detect.
pub fn resolve(explicit: Option<&str>, env_override: Option<&str>) -> Result<Backend, Error> {
    if let Some(raw) = explicit.or(env_override) {
        let trimmed = raw.trim();
        if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("auto") {
            let backend = Backend::parse(trimmed).ok_or(Error::NoneAvailable {
                tried: trimmed.to_string(),
            })?;
            if !backend.is_installed() {
                return Err(Error::NotInstalled(backend));
            }
            return Ok(backend);
        }
    }
    for &candidate in Backend::ALL {
        if candidate.is_installed() {
            return Ok(candidate);
        }
    }
    Err(Error::NoneAvailable {
        tried: Backend::ALL
            .iter()
            .map(|b| b.binary())
            .collect::<Vec<_>>()
            .join(", "),
    })
}

/// Run `backend` against `prompt`, capture stdout, and normalise the response
/// down to a CHANGELOG section (everything from the first `## ` line onwards).
pub fn polish(backend: Backend, root: &Path, prompt: &str) -> Result<String, Error> {
    let mut cmd = backend.build_command(prompt, root);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(Error::Failed {
            backend,
            status: out.status.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    let raw = String::from_utf8(out.stdout)?;
    Ok(extract_markdown_section(&raw))
}

/// Extract the first `## `-headed markdown section from `raw`, dropping any
/// preamble before it and any subsequent `## `-headed sections after it.
///
/// Both ends matter:
///
/// - Some CLIs print metadata (token counts, model labels, "Thinking…"
///   frames) *before* the response body; those land before the first `## `.
/// - Models sometimes echo the input back or append a comparison section
///   *after* the polished output. Empirically, polish backends do this even
///   when the prompt forbids it, so we stop at the second top-level heading.
///
/// Fences are not tracked: a CHANGELOG section with `## ` inside a fenced
/// code block would be incorrectly truncated, but that's not a shape this
/// project's changelogs use.
///
/// Falls back to the trimmed raw output when no `## ` heading is found, so
/// the operator still gets *something* to inspect rather than an empty
/// section.
pub(crate) fn extract_markdown_section(raw: &str) -> String {
    let mut out = String::new();
    let mut in_section = false;
    for line in raw.lines() {
        if !in_section {
            if line.starts_with("## ") {
                in_section = true;
            } else {
                continue;
            }
        } else if line.starts_with("## ") {
            // Second top-level heading: the polished output ended at the
            // previous line. Anything after is preamble of an echoed
            // mechanical draft, a comparison block, or a "before/after"
            // dump — none of which belong in CHANGELOG.md.
            break;
        }
        out.push_str(line);
        out.push('\n');
    }
    if !in_section {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            out.push_str(trimmed);
            out.push('\n');
        }
        return out;
    }
    // Normalise the trailing whitespace to exactly one blank line so the
    // caller's prepend doesn't visually collide with the next existing
    // `## v...` heading.
    while out.ends_with('\n') {
        out.pop();
    }
    out.push('\n');
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_label_synonyms() {
        assert_eq!(Backend::parse("claude"), Some(Backend::Claude));
        assert_eq!(Backend::parse("CLAUDE"), Some(Backend::Claude));
        assert_eq!(Backend::parse("gh-copilot"), Some(Backend::Copilot));
        assert_eq!(Backend::parse("cursor"), Some(Backend::Cursor));
        assert_eq!(Backend::parse("cursor-agent"), Some(Backend::Cursor));
        assert_eq!(Backend::parse("not-a-backend"), None);
    }

    #[test]
    fn extract_strips_preamble() {
        let raw = "[token usage: 1234]\nThinking...\n## v0.3.0 - 2026-05-18\n\n- ok\n";
        let extracted = extract_markdown_section(raw);
        assert_eq!(extracted, "## v0.3.0 - 2026-05-18\n\n- ok\n\n");
    }

    #[test]
    fn extract_stops_at_second_heading() {
        // Reproduces the bug where polish backends echoed the mechanical
        // draft after the polished section. We must keep only the first
        // section so the echoed `## v...` block doesn't land in CHANGELOG.md.
        let raw = "\
## v0.2.1 - 2026-05-18

### Fixed

- Corrected per-shard sequence number tracking.
## v0.2.1 - 2026-05-18

### Changed

- ec2: noise that should not appear (4760384f)
";
        let extracted = extract_markdown_section(raw);
        let expected = "\
## v0.2.1 - 2026-05-18

### Fixed

- Corrected per-shard sequence number tracking.

";
        assert_eq!(extracted, expected);
    }

    #[test]
    fn extract_keeps_body_inside_single_section() {
        let raw = "## Unreleased\n\n### Added\n- foo\n\n### Fixed\n- bar\n\nTrailing text.\n";
        let extracted = extract_markdown_section(raw);
        // Trailing whitespace is normalised to one blank line.
        let expected =
            "## Unreleased\n\n### Added\n- foo\n\n### Fixed\n- bar\n\nTrailing text.\n\n";
        assert_eq!(extracted, expected);
    }

    #[test]
    fn extract_falls_back_when_no_heading() {
        let raw = "  weird output without a heading  ";
        let extracted = extract_markdown_section(raw);
        assert_eq!(extracted, "weird output without a heading\n");
    }

    #[test]
    fn extract_empty_when_input_empty() {
        assert_eq!(extract_markdown_section(""), "");
        assert_eq!(extract_markdown_section("\n\n\n"), "");
    }
}
