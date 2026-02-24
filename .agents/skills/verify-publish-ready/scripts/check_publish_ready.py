#!/usr/bin/env python3
"""Static release-readiness checks for the Winterbaume repository."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path


MANUAL_FIRST_PUBLIC_GATES = [
    "GitHub repository metadata, labels, branch protection, Pages, and required secrets are configured.",
    "Git history has been scanned for secrets and non-public material.",
    "Bundled agent material has been reviewed for private notes and non-public data.",
    "Vendor, generated, and bundled third-party licence obligations have been reviewed.",
    "README public contribution policy and security contact wording have been reviewed.",
    "Full hosted CI has passed for the exact first-public-release commit.",
    "cargo-release dry-runs have passed for the planned publishable crate release set.",
    "The cargo-release crate flow and cargo-dist binary release flow have been checked against current workspace membership.",
    "Crate descriptions have been reviewed for public-facing wording.",
]


@dataclass
class Finding:
    level: str
    message: str


def run_git(args: list[str]) -> list[str]:
    proc = subprocess.run(
        ["git", *args],
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if proc.returncode != 0:
        return [f"<git {' '.join(args)} failed: {proc.stderr.strip()}>"]
    return [line for line in proc.stdout.splitlines() if line.strip()]


def read_text(path: Path) -> str:
    if not path.exists():
        return ""
    return path.read_text(encoding="utf-8")


def check_required_files(root: Path, findings: list[Finding]) -> None:
    required = [
        "README.md",
        "CHANGELOG.md",
        "RELEASE.md",
        "LICENSE",
        "NOTICE",
        "Cargo.toml",
        "Cargo.lock",
        "dist-workspace.toml",
        ".github/workflows/ci.yml",
        ".github/workflows/release.yml",
        "docs/package.json",
        "docs/index.md",
    ]
    for rel in required:
        if not (root / rel).exists():
            findings.append(Finding("FAIL", f"Missing required release file: {rel}"))


def check_readme_assets(root: Path, findings: list[Finding]) -> None:
    readme = read_text(root / "README.md")
    for match in re.finditer(r"!\[[^\]]*\]\(([^)]+)\)", readme):
        target = match.group(1).strip().strip('"').strip("'")
        target = target.split()[0]
        if not target or re.match(r"^[a-z][a-z0-9+.-]*:", target) or target.startswith("#"):
            continue
        if not (root / target).exists():
            findings.append(Finding("FAIL", f"README image asset is missing: {target}"))


def check_release_runbook(root: Path, findings: list[Finding]) -> None:
    path = root / "RELEASE.md"
    if not path.exists():
        return
    text = read_text(path)
    required_sections = [
        "## Prerequisites",
        "## Crate Release",
        "## Binary Release",
        "## Release Execution Checklist",
    ]
    for section in required_sections:
        if section not in text:
            findings.append(Finding("FAIL", f"RELEASE.md missing section `{section}`"))

    first_time_sections = [
        "## Public Release Readiness",
        "### Remaining Blockers",
        "## Verification Log",
        "## Checks Not Completed In This Audit",
    ]
    for section in first_time_sections:
        if section in text:
            findings.append(
                Finding(
                    "WARN",
                    f"RELEASE.md still contains first-time/audit-specific section `{section}`",
                )
            )

    stale_todo_source_phrases = [
        "First-time public launch tasks and recurring release-gate TODOs live in `.agents/docs/TODO.md`.",
        "Confirm there are no open first-time launch blockers in `.agents/docs/TODO.md`",
    ]
    for phrase in stale_todo_source_phrases:
        if phrase in text:
            findings.append(
                Finding(
                    "FAIL",
                    "RELEASE.md still treats `.agents/docs/TODO.md` as release criteria source of truth",
                )
            )


def inherited_workspace_value(root_data: dict, key: str) -> object | None:
    return root_data.get("workspace", {}).get("package", {}).get(key)


def check_cargo_metadata(root: Path, findings: list[Finding]) -> None:
    root_data = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    workspace_license = inherited_workspace_value(root_data, "license")
    workspace_repository = inherited_workspace_value(root_data, "repository")

    package_count = 0
    publish_false = 0
    publishable = 0

    for manifest in sorted(root.glob("**/Cargo.toml")):
        if any(part in {".agents", "target", "vendor"} for part in manifest.parts):
            continue
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
        package = data.get("package")
        if not package:
            continue

        package_count += 1
        name = str(package.get("name", manifest.parent.name))
        publish = package.get("publish", True)
        if publish is False:
            publish_false += 1
            continue
        publishable += 1

        if not package.get("description"):
            findings.append(Finding("FAIL", f"{name}: missing package description"))
        if not (package.get("license") or package.get("license-file") or workspace_license):
            findings.append(Finding("FAIL", f"{name}: missing package licence"))
        if not (package.get("repository") or workspace_repository):
            findings.append(Finding("FAIL", f"{name}: missing package repository"))

        readme = package.get("readme")
        if readme:
            if not (manifest.parent / str(readme)).exists():
                findings.append(
                    Finding("FAIL", f"{name}: readme path does not exist: {readme}")
                )
        elif manifest.parent.name.startswith("winterbaume"):
            findings.append(Finding("FAIL", f"{name}: missing package readme"))

    expected_publish_false = {
        "tools/smithy-codegen/Cargo.toml",
        "tools/sccache-wrapper/Cargo.toml",
        "crates/winterbaume-e2e-tests/Cargo.toml",
    }
    for rel in expected_publish_false:
        manifest = root / rel
        if not manifest.exists():
            findings.append(Finding("FAIL", f"Missing expected non-publishable crate: {rel}"))
            continue
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
        if data.get("package", {}).get("publish") is not False:
            findings.append(Finding("FAIL", f"{rel} should set publish = false"))

    findings.append(
        Finding(
            "INFO",
            f"Cargo packages: {package_count}; publishable: {publishable}; publish=false: {publish_false}",
        )
    )


def check_lockfiles(root: Path, findings: list[Finding]) -> None:
    actual_lockfiles = sorted(
        path.relative_to(root).as_posix()
        for path in root.glob("**/Cargo.lock")
        if not any(part in {".agents", "target", "vendor"} for part in path.parts)
    )
    unexpected = [path for path in actual_lockfiles if path != "Cargo.lock"]
    if unexpected:
        findings.append(
            Finding("FAIL", f"Unexpected crate-local Cargo.lock files: {', '.join(unexpected)}")
        )


def check_dist_config(root: Path, findings: list[Finding]) -> None:
    cargo_toml = read_text(root / "Cargo.toml")
    dist_workspace = read_text(root / "dist-workspace.toml")
    release_workflow = read_text(root / ".github/workflows/release.yml")

    if "[profile.dist]" not in cargo_toml:
        findings.append(Finding("FAIL", "Cargo.toml missing `[profile.dist]`"))
    if "[dist]" not in dist_workspace:
        findings.append(Finding("FAIL", "dist-workspace.toml missing `[dist]` configuration"))
    if "dist " not in release_workflow:
        findings.append(Finding("FAIL", ".github/workflows/release.yml does not run cargo-dist"))


def check_ci_surface(root: Path, findings: list[Finding]) -> None:
    workflow_files = sorted((root / ".github/workflows").glob("*.yml"))
    workflows = "\n".join(read_text(path) for path in workflow_files)
    required_patterns = [
        ("rustfmt", r"cargo fmt --all --check"),
        ("clippy", r"cargo clippy"),
        ("workspace tests", r"cargo test --workspace"),
        ("examples", r"cargo build --examples"),
        ("Terraform E2E", r"winterbaume-e2e-tests --test terraform"),
        ("docs build", r"npm run build"),
        ("docs generation", r"npm run generate"),
        ("cargo-dist plan", r"\bdist .*\bplan\b"),
    ]
    for name, pattern in required_patterns:
        if not re.search(pattern, workflows):
            findings.append(Finding("FAIL", f"GitHub workflows missing {name} gate"))


def check_docs_generated_not_tracked(findings: list[Finding]) -> None:
    tracked_generated = run_git(["ls-files", "docs/node_modules", "docs/.vitepress/dist"])
    if tracked_generated:
        findings.append(
            Finding(
                "FAIL",
                "Generated docs dependencies/output are tracked: "
                + ", ".join(tracked_generated[:10]),
            )
        )


def check_public_intake(root: Path, findings: list[Finding]) -> None:
    required = [
        ".github/ISSUE_TEMPLATE/bug_report.yml",
        ".github/ISSUE_TEMPLATE/feature_request.yml",
        ".github/workflows/auto-label-service.yml",
        ".github/workflows/triage-bug.yml",
        ".github/workflows/deploy-docs.yml",
    ]
    for rel in required:
        if not (root / rel).exists():
            findings.append(
                Finding("FAIL", f"Missing public intake or docs workflow file: {rel}")
            )

    if not (root / "SECURITY.md").exists():
        findings.append(Finding("FAIL", "Missing public security policy: SECURITY.md"))

    contribution_docs = ["CONTRIBUTING.md", "PULL_REQUESTS.md", "SUPPORT.md"]
    if not any((root / rel).exists() for rel in contribution_docs):
        findings.append(
            Finding(
                "FAIL",
                "Missing public contribution or support policy: CONTRIBUTING.md, PULL_REQUESTS.md, or SUPPORT.md",
            )
        )

    if not (root / "CODE_OF_CONDUCT.md").exists():
        findings.append(
            Finding(
                "WARN",
                "No CODE_OF_CONDUCT.md found; confirm this is an explicit public launch decision",
            )
        )


def check_manual_first_public_gates(findings: list[Finding]) -> None:
    for gate in MANUAL_FIRST_PUBLIC_GATES:
        findings.append(
            Finding("WARN", f"Manual/live first-public-release gate not verified: {gate}")
        )


def check_git_branch(findings: list[Finding]) -> None:
    branch = run_git(["branch", "--show-current"])
    if branch and not branch[0].startswith("<git ") and branch[0] != "main":
        findings.append(Finding("WARN", f"Current branch is `{branch[0]}`, not `main`"))


def check_git_status(require_clean: bool, findings: list[Finding]) -> None:
    status = run_git(["status", "--short"])
    if status:
        level = "FAIL" if require_clean else "WARN"
        findings.append(Finding(level, f"Working tree is dirty ({len(status)} entries)"))
        for line in status[:20]:
            findings.append(Finding("INFO", f"git status: {line}"))
    else:
        findings.append(Finding("INFO", "Working tree is clean"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--first-public-release",
        action="store_true",
        help="Run first-public-release static checks and report manual launch gates.",
    )
    parser.add_argument(
        "--require-clean",
        action="store_true",
        help="Treat a dirty worktree as a failure instead of a warning.",
    )
    args = parser.parse_args()

    root = Path.cwd()
    findings: list[Finding] = []

    check_required_files(root, findings)
    check_readme_assets(root, findings)
    check_release_runbook(root, findings)
    check_cargo_metadata(root, findings)
    check_lockfiles(root, findings)
    check_dist_config(root, findings)
    check_ci_surface(root, findings)
    check_docs_generated_not_tracked(findings)
    if args.first_public_release:
        check_public_intake(root, findings)
        check_manual_first_public_gates(findings)
    check_git_branch(findings)
    check_git_status(args.require_clean, findings)

    fail_count = sum(1 for finding in findings if finding.level == "FAIL")
    warn_count = sum(1 for finding in findings if finding.level == "WARN")

    verdict = "FAIL" if fail_count else "PASS"
    print(f"Verdict: {verdict}")
    print(f"Failures: {fail_count}")
    print(f"Warnings: {warn_count}")
    print()
    for finding in findings:
        print(f"{finding.level}: {finding.message}")

    return 1 if fail_count else 0


if __name__ == "__main__":
    sys.exit(main())
