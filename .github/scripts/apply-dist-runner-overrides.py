#!/usr/bin/env python3
"""Copy `[dist.github-custom-runners]` from one dist config into another.

Used by the manual ( `workflow_dispatch` ) path of `.github/workflows/release.yml`.
A manual run releases an existing tag, whose `dist-workspace.toml` may predate
a runner fix ( e.g. a retired GitHub runner image ). The tagged source is built
as is; only the runner table is taken from the workflow's own revision.

    apply-dist-runner-overrides.py <source dist-workspace.toml> <target dist-workspace.toml>

Rules:
- no table in the source: nothing to do;
- the target already has an identical table: nothing to do;
- the target has a different table: fail, rather than guess how to merge;
- otherwise the source table is appended to the target.
"""

from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path
from typing import Any

TABLE = "github-custom-runners"


def runners(config: dict[str, Any]) -> dict[str, Any] | None:
    return config.get("dist", {}).get(TABLE)


def toml_value(value: Any) -> str:
    # json.dumps yields a valid TOML basic string for str values.
    if isinstance(value, str):
        return json.dumps(value)
    if isinstance(value, dict):
        items = ", ".join(f"{k} = {toml_value(v)}" for k, v in value.items())
        return "{ " + items + " }"
    raise SystemExit(f"unsupported value in [dist.{TABLE}]: {value!r}")


def render(table: dict[str, Any]) -> str:
    lines = ["", f"[dist.{TABLE}]"]
    lines += [f"{key} = {toml_value(value)}" for key, value in table.items()]
    return "\n".join(lines) + "\n"


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print(__doc__, file=sys.stderr)
        return 2
    source_path, target_path = Path(argv[1]), Path(argv[2])
    source = runners(tomllib.loads(source_path.read_text()))
    target_text = target_path.read_text()
    target = runners(tomllib.loads(target_text))

    if not source:
        print(f"{source_path}: no [dist.{TABLE}]; nothing to apply")
        return 0
    if target == source:
        print(f"{target_path}: [dist.{TABLE}] already matches")
        return 0
    if target is not None:
        print(
            f"::error::{target_path} has a different [dist.{TABLE}] "
            f"( {target!r} ) from the workflow revision ( {source!r} )",
        )
        return 1

    if not target_text.endswith("\n"):
        target_text += "\n"
    target_path.write_text(target_text + render(source))
    # Re-parse to make sure the result is valid and carries the table.
    if runners(tomllib.loads(target_path.read_text())) != source:
        print(f"::error::failed to apply [dist.{TABLE}] to {target_path}")
        return 1
    print(f"{target_path}: applied [dist.{TABLE}] = {source!r}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
