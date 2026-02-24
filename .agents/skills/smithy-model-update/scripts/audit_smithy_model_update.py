#!/usr/bin/env python3
"""Audit Winterbaume fallout after vendored AWS Smithy model updates."""

from __future__ import annotations

import argparse
import importlib.util
import json
import re
import sys
from pathlib import Path
from typing import Any


HELPER_CRATES = {
    "winterbaume-bedrock-flow-parser",
    "winterbaume-bedrock-flow-validator",
    "winterbaume-core",
    "winterbaume-dynamodb-redis",
    "winterbaume-e2e-tests",
    "winterbaume-ec2-generated",
    "winterbaume-iam-rule-eval",
    "winterbaume-partiql",
    "winterbaume-server",
    "winterbaume-sfn-asl-eval",
    "winterbaume-sqlengine-duckdb",
    "winterbaume-sqs-redis",
    "winterbaume-terraform",
    "winterbaume-tfstate",
    "winterbaume-wafv2-wcu-calculator",
    "winterbaume-wafv2-webacl-rule-parser",
}

CRATE_MODEL_EXCEPTIONS = {
    "winterbaume-apigatewaymanagement": "apigatewaymanagementapi",
    "winterbaume-config": "config-service",
    "winterbaume-costandusagereport": "cost-and-usage-report-service",
    "winterbaume-databasemigration": "database-migration-service",
    "winterbaume-directory": "directory-service",
    "winterbaume-resourcegroupstagging": "resource-groups-tagging-api",
}

# Keep richer implementation tasks out of the flat unimplemented-service TODO list.
TODO_DEDUPED_MODELS = {
    "pinpoint-sms-voice-v2",
}


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def crate_name(cargo_toml: Path) -> str | None:
    match = re.search(r'^name\s*=\s*"([^"]+)"', cargo_toml.read_text(), re.MULTILINE)
    return match.group(1) if match else None


def discover_models(root: Path) -> list[str]:
    models_root = root / "vendor" / "api-models-aws" / "models"
    return sorted(path.name for path in models_root.iterdir() if (path / "service").is_dir())


def discover_service_crates(root: Path, models: list[str]) -> dict[str, str]:
    model_set = set(models)
    compact_models = {model.replace("-", ""): model for model in models}
    crates: dict[str, str] = {}

    for cargo_toml in sorted((root / "crates").glob("winterbaume-*/Cargo.toml")):
        crate = crate_name(cargo_toml)
        if crate is None or crate in HELPER_CRATES:
            continue

        suffix = crate.removeprefix("winterbaume-")
        model = CRATE_MODEL_EXCEPTIONS.get(crate)
        if model is None and suffix in model_set:
            model = suffix
        if model is None:
            model = compact_models.get(suffix)
        if model is not None:
            crates[crate] = model

    return crates


def parse_todo_services(root: Path) -> list[str]:
    todo = root / ".agents" / "docs" / "TODO.md"
    text = todo.read_text()
    try:
        section = text.split("### Services Not Yet Implemented", 1)[1]
        section = section.split("### Backlog Hygiene", 1)[0]
    except IndexError:
        return []
    return [
        line.split("] ", 1)[1].strip()
        for line in section.splitlines()
        if line.startswith("- [ ] ")
    ]


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Audit Smithy model, service crate, coverage, README, and TODO alignment.",
    )
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--json", action="store_true", help="emit machine-readable JSON")
    parser.add_argument("--check", action="store_true", help="exit non-zero when drift is found")
    args = parser.parse_args()

    root = args.root.resolve()
    models = discover_models(root)
    crates = discover_service_crates(root, models)

    coverage = load_module(
        root / ".agents" / "skills" / "api-coverage" / "scripts" / "generate_coverage.py",
        "generate_coverage",
    )
    readme = load_module(
        root / ".agents" / "skills" / "update-readme" / "scripts" / "update_readme.py",
        "update_readme",
    )

    expected_unimplemented = sorted(
        model for model in models if model not in set(crates.values()) | TODO_DEDUPED_MODELS
    )
    todo_services = parse_todo_services(root)

    report = {
        "model_count": len(models),
        "service_crate_count": len(crates),
        "expected_unimplemented_count": len(expected_unimplemented),
        "todo_unimplemented_count": len(todo_services),
        "missing_crate_to_model": {
            crate: model
            for crate, model in sorted(crates.items())
            if coverage.CRATE_TO_MODEL.get(crate) != model
        },
        "stale_crate_to_model": {
            crate: model
            for crate, model in sorted(coverage.CRATE_TO_MODEL.items())
            if crate not in crates
        },
        "missing_readme_display": sorted(
            crate for crate in crates if crate not in readme.CRATE_DISPLAY_INFO
        ),
        "todo_missing_services": sorted(set(expected_unimplemented) - set(todo_services)),
        "todo_extra_services": sorted(set(todo_services) - set(expected_unimplemented)),
    }

    has_drift = any(
        report[key]
        for key in (
            "missing_crate_to_model",
            "stale_crate_to_model",
            "missing_readme_display",
            "todo_missing_services",
            "todo_extra_services",
        )
    )

    if args.json:
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        print(
            "Smithy model update audit: "
            f"{report['model_count']} models, "
            f"{report['service_crate_count']} service crates, "
            f"{report['expected_unimplemented_count']} expected TODO services"
        )
        for key, value in report.items():
            if key.endswith("_count") or key == "model_count":
                continue
            if value:
                print(f"\n{key}:")
                if isinstance(value, dict):
                    for item_key, item_value in value.items():
                        print(f"  {item_key}: {item_value}")
                else:
                    for item in value:
                        print(f"  {item}")
        if not has_drift:
            print("No Smithy model update drift found.")

    return 1 if args.check and has_drift else 0


if __name__ == "__main__":
    sys.exit(main())
