#!/usr/bin/env python3
"""Draft a per-crate CHANGELOG.md section from `git log` since the crate's
last `<crate>-v<X.Y.Z>` tag.

This is the *mechanical* half of the generate-changelog skill: it emits raw
commit subjects bucketed by conventional-commit prefix. The editorial pass —
summarising behaviour, collapsing repeated updates, dropping implementation
trivia — is the human or model step that runs *after* this script, per the
rules in SKILL.md.

Output goes to stdout. The caller decides where it lands (typically prepended
to `crates/<crate>/CHANGELOG.md`).

Usage:
    python3 .agents/skills/generate-changelog/scripts/draft_section.py \\
        --crate winterbaume-ec2 \\
        --next-version 0.3.0

    # Unreleased section (no `--next-version`):
    python3 .agents/skills/generate-changelog/scripts/draft_section.py \\
        --crate winterbaume-ec2

    # Pin a specific date instead of today UTC:
    python3 .agents/skills/generate-changelog/scripts/draft_section.py \\
        --crate winterbaume-ec2 --next-version 0.3.0 --date 2026-05-18
"""

from __future__ import annotations

import argparse
import datetime as dt
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


# Conventional-commit prefix → section heading. Order is significant: it
# determines the order the sections appear in the rendered markdown.
SECTIONS: list[tuple[str, tuple[str, ...]]] = [
    ("Added", ("feat", "add")),
    ("Changed", ("change", "refactor", "perf", "improve")),
    ("Fixed", ("fix", "bug")),
    ("Terraform", ("terraform", "tf")),
    ("Documentation", ("docs", "doc")),
    ("Tests", ("test", "tests")),
    ("Internal", ("chore", "build", "ci", "release")),
]

# `<X>.<Y>.<Z>` with optional `-pre`/`+meta` suffix.
SEMVER_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)(?:[-+].*)?$")


@dataclass(frozen=True)
class Commit:
    hash_short: str
    date: str
    subject: str


def classify(subject: str) -> str:
    """Bucket a commit subject into one of the canonical section labels.

    Looks at the leading `<prefix>:` or `<prefix>(<scope>):` token. Falls back
    to content heuristics ("fix" / "doc" / "test" / else Changed) when no
    recognised prefix is present.
    """
    head = subject.split(":", 1)[0].strip()
    scope = head.split("(", 1)[0].strip().lower()
    for label, prefixes in SECTIONS:
        if scope in prefixes:
            return label
    lower = subject.lower()
    if lower.startswith("fix") or " fix " in lower:
        return "Fixed"
    if "doc" in lower:
        return "Documentation"
    if "test" in lower:
        return "Tests"
    return "Changed"


def run_git(args: list[str], cwd: Path) -> str:
    out = subprocess.run(
        ["git", *args],
        cwd=cwd,
        check=True,
        capture_output=True,
        text=True,
    )
    return out.stdout


def latest_tag(repo_root: Path, crate: str) -> str | None:
    """Latest `<crate>-v<X.Y.Z>` tag, picked by `-creatordate`."""
    pattern = f"refs/tags/{crate}-v*"
    raw = run_git(
        [
            "for-each-ref",
            "--sort=-creatordate",
            "--format=%(refname:short)",
            pattern,
        ],
        cwd=repo_root,
    )
    prefix = f"{crate}-v"
    for line in raw.splitlines():
        line = line.strip()
        if not line.startswith(prefix):
            continue
        rest = line[len(prefix) :]
        if SEMVER_RE.match(rest):
            return line
    return None


def commits_since(repo_root: Path, tag: str, pathspec: str) -> list[Commit]:
    """Per-crate commit log between the supplied tag and HEAD, restricted to
    files under `pathspec`. First-parent traversal so feature-branch merge
    bodies don't multiply the entry count."""
    raw = run_git(
        [
            "log",
            "--first-parent",
            "--date=short",
            "--format=%h%x09%ad%x09%s",
            f"{tag}..HEAD",
            "--",
            pathspec,
        ],
        cwd=repo_root,
    )
    out: list[Commit] = []
    for line in raw.splitlines():
        parts = line.split("\t", 2)
        if len(parts) != 3:
            continue
        h, d, s = parts
        out.append(Commit(hash_short=h, date=d, subject=s))
    return out


def render(
    crate: str,
    next_version: str | None,
    date: str,
    commits: list[Commit],
) -> str:
    """Render the markdown section. `next_version=None` ⇒ `## Unreleased`."""
    heading = (
        f"## v{next_version} - {date}" if next_version is not None else "## Unreleased"
    )
    bins: dict[str, list[str]] = {}
    for c in commits:
        # Pure release-metadata commits are noise per the skill convention.
        if c.subject.startswith("chore: release "):
            continue
        label = classify(c.subject)
        bins.setdefault(label, []).append(f"{c.subject} ({c.hash_short})")

    lines = [heading, ""]
    if not bins:
        lines += [
            "- _No commits attributed to this crate since the last release._",
            "",
        ]
        return "\n".join(lines) + "\n"

    rendered: set[str] = set()
    for label, _prefixes in SECTIONS:
        items = bins.get(label)
        if not items:
            continue
        rendered.add(label)
        lines += [f"### {label}", ""]
        for item in items:
            lines.append(f"- {item}")
        lines.append("")

    # Defensive: anything bucketed under a non-canonical label still gets
    # printed (shouldn't happen because classify() only returns canonical
    # labels, but futureproof against table edits).
    for label in sorted(bins):
        if label in rendered:
            continue
        lines += [f"### {label}", ""]
        for item in bins[label]:
            lines.append(f"- {item}")
        lines.append("")

    return "\n".join(lines) + "\n"


def parse_args(argv: list[str]) -> argparse.Namespace:
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument(
        "--crate",
        required=True,
        help="Cargo package name, e.g. winterbaume-ec2.",
    )
    p.add_argument(
        "--next-version",
        default=None,
        help="Target version for the heading (e.g. 0.3.0). Omit for an "
        "`## Unreleased` heading.",
    )
    p.add_argument(
        "--date",
        default=None,
        help="Release date in YYYY-MM-DD. Defaults to today (UTC). Ignored "
        "when --next-version is omitted.",
    )
    p.add_argument(
        "--repo-root",
        default=None,
        help="Repository root. Defaults to `git rev-parse --show-toplevel` "
        "from the current working directory.",
    )
    p.add_argument(
        "--crate-dir",
        default=None,
        help="Crate directory relative to repo root. Defaults to "
        "`crates/<crate>`.",
    )
    p.add_argument(
        "--from-ref",
        default=None,
        help="Override the lower-bound ref of the diff range. Defaults to "
        "the latest <crate>-v<X.Y.Z> tag. Use this for crates with no prior "
        "tag (e.g. `--from-ref <initial-commit>`).",
    )
    return p.parse_args(argv)


def resolve_repo_root(arg: str | None) -> Path:
    if arg:
        return Path(arg).resolve()
    raw = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()
    return Path(raw)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv if argv is not None else sys.argv[1:])
    repo_root = resolve_repo_root(args.repo_root)
    crate_dir = args.crate_dir or f"crates/{args.crate}"
    pathspec = f"{crate_dir.rstrip('/')}/"

    from_ref = args.from_ref or latest_tag(repo_root, args.crate)
    if from_ref is None:
        print(
            f"warn: no prior `{args.crate}-v<ver>` tag and no --from-ref; "
            "treating this as the initial release (no commit bullets)",
            file=sys.stderr,
        )
        commits: list[Commit] = []
    else:
        commits = commits_since(repo_root, from_ref, pathspec)

    date = args.date or dt.datetime.now(dt.UTC).strftime("%Y-%m-%d")
    sys.stdout.write(render(args.crate, args.next_version, date, commits))
    return 0


if __name__ == "__main__":
    sys.exit(main())
