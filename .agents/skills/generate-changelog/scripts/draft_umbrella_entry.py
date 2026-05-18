#!/usr/bin/env python3
"""Draft a dated section for the root umbrella `CHANGELOG.md`.

The root changelog is an index: one dated section per release batch, with one
bullet per crate that landed on that date. Each bullet links to that crate's
own `CHANGELOG.md` for detail.

Output goes to stdout. The caller decides where it lands (typically prepended
just before the first existing `## <date>` section of the root CHANGELOG.md).

Usage:
    python3 .agents/skills/generate-changelog/scripts/draft_umbrella_entry.py \\
        --entry winterbaume-s3:0.1.1 \\
        --entry winterbaume-sqs:0.1.1 \\
        --date 2026-04-29
"""

from __future__ import annotations

import argparse
import datetime as dt
import sys
from dataclasses import dataclass


@dataclass(frozen=True)
class Entry:
    name: str
    version: str
    note: str  # `initial release` / `pinned` / empty string


def parse_entry(raw: str) -> Entry:
    """Parse `name:version[:note]` from the `--entry` flag."""
    parts = raw.split(":", 2)
    if len(parts) < 2 or not parts[0] or not parts[1]:
        raise argparse.ArgumentTypeError(
            f"--entry expects `name:version[:note]`, got {raw!r}"
        )
    note = parts[2] if len(parts) == 3 else ""
    return Entry(name=parts[0], version=parts[1], note=note)


def render(entries: list[Entry], date: str) -> str:
    lines = [f"## {date}", ""]
    for e in entries:
        suffix = f" ({e.note})" if e.note else ""
        link = f"crates/{e.name}/CHANGELOG.md"
        lines.append(
            f"- `{e.name}` v{e.version}{suffix}: see [`{link}`]({link})."
        )
    lines.append("")
    return "\n".join(lines) + "\n"


def parse_args(argv: list[str]) -> argparse.Namespace:
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument(
        "--entry",
        action="append",
        type=parse_entry,
        required=True,
        help="`name:version[:note]`, repeatable. `note` is an optional "
        "qualifier like `initial release` or `pinned`.",
    )
    p.add_argument(
        "--date",
        default=None,
        help="Release date in YYYY-MM-DD. Defaults to today (UTC).",
    )
    return p.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv if argv is not None else sys.argv[1:])
    date = args.date or dt.datetime.now(dt.UTC).strftime("%Y-%m-%d")
    sys.stdout.write(render(args.entry, date))
    return 0


if __name__ == "__main__":
    sys.exit(main())
