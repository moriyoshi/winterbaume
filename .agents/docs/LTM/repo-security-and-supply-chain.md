# Repo Security and Supply-Chain Hardening

## Summary

The 2026-04-27 audit pass tightened Winterbaume's GitHub Actions surface and the cargo-dist release path. The durable knowledge is which security categories were addressed (action SHA pinning, cargo-dist installer SHA-256 verification, `GITHUB_TOKEN` permission scoping, `workflow_run` chain documentation), the bump procedures that keep the pinning useful, and the residual accepted-risk items that auditors will likely revisit.

## Key Facts

- All `uses:` references in `.github/workflows/` are pinned to commit SHAs with the version tag in an inline comment, so tag-hijack of a third-party action does not propagate into a release.
- The cargo-dist installer is downloaded to a temp file, verified against a hardcoded SHA-256 digest in `.github/workflows/release.yml`, and only then executed. A pipe-to-`sh` was the auditor's only consolidated MEDIUM finding and is no longer present.
- `GITHUB_TOKEN` cannot be scoped below repository level. `record-outcome.yml` and `record-triage.yml` carry an accepted-risk comment for `contents: write`; the mitigation is branch protection on `main` so the bot can only push to `memory/triage`.
- Third-party actions that need a token (e.g. `cloudflare/wrangler-action`) get the smallest possible permission scope at the workflow's top-level `permissions:` block, with an inline comment justifying the scope.
- `workflow_run` chains are documented inline at the consumer side: dependency edge, blast radius, and why artefact contents are not executed.
- `Install Rust non-interactively` in `release.yml` still curl-pipes `https://sh.rustup.rs` because that is rust-lang.org's canonical install path; left untouched as a deliberate exception (only runs in container matrix entries).

## Details

### Action SHA Pinning

Every `uses:` reference across the nine workflow files now carries a commit SHA with the version tag as an inline comment. Renewals happen by hand through the bump procedure below. The pinned set as of 2026-04-27:

| Action | Tag | Commit SHA |
|--------|-----|------------|
| `actions/checkout` | v6 | `de0fac2e4500dabe0009e67214ff5f5447ce83dd` |
| `actions/upload-artifact` | v7 | `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a` |
| `actions/download-artifact` | v8 | `3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c` |
| `actions/setup-node` | v4 | `49933ea5288caeca8642d1e84afbd3f7d6820020` |
| `actions/github-script` | v9 | `3a2844b7e9c422d3c10d287c895573f7108da1b3` |
| `actions/stale` | v9 | `5bef64f19d7facfb25b37b414482c7164d639639` |
| `actions/ai-inference` | v2 | `e09e65981758de8b2fdab13c2bfb7c7d5493b0b6` |
| `cloudflare/wrangler-action` | v3 | `9acf94ace14e7dc412b076f2c5c20b8ce93c79cd` |
| `actions-rust-lang/setup-rust-toolchain` | (already pinned) | `150fca883cd4034361b621bd4e6a9d34e5143606` |
| `mozilla-actions/sccache-action` | (already pinned) | `7d986dd989559c6ecdb630a3fd2557667be217ad` |
| `hashicorp/setup-terraform` | (already pinned) | `5e8dbf3c6d9deaf4193ca7a8fb23f2ac83bb6c85` |

Bump procedure for any of these:

1. Identify the upstream commit for the new tag via `gh api repos/<owner>/<repo>/git/refs/tags/<tag>`.
2. Update the `uses:` line and the inline tag comment together so the comment reflects what the SHA actually points at.
3. Read the upstream changelog before bumping a major version; some bumps drop runner support (Node 16/Node 20 deprecation cycles).

### cargo-dist Installer SHA-256 Pinning

`release.yml`'s `Install dist` step is shaped:

```yaml
env:
  CARGO_DIST_INSTALLER_URL: "https://github.com/axodotdev/cargo-dist/releases/download/v0.31.0/cargo-dist-installer.sh"
  CARGO_DIST_INSTALLER_SHA256: "e79d87e418b9d2cbe992d014985457c28a5a7c553add3da4ed1047e161c928f4"
run: |
  set -euo pipefail
  installer="$(mktemp)"
  curl --proto '=https' --tlsv1.2 -LsSf "$CARGO_DIST_INSTALLER_URL" -o "$installer"
  echo "$CARGO_DIST_INSTALLER_SHA256  $installer" | sha256sum -c -
  sh "$installer"
  rm -f "$installer"
```

If upstream is tampered with or rotates the asset under the same URL, `sha256sum -c` aborts the job before any fetched code executes. Two alternatives were considered and rejected: `cargo install cargo-dist --version 0.31.0 --locked` (adds 1-2 min compile time per release run) and vendoring the installer in-repo (forces every future bump through repo).

Bump procedure for the installer:

```sh
curl --proto '=https' --tlsv1.2 -LsSf <new-installer-url> | sha256sum
```

Both `CARGO_DIST_INSTALLER_URL` and `CARGO_DIST_INSTALLER_SHA256` must move together in the same commit.

### GITHUB_TOKEN Permission Scoping

`GITHUB_TOKEN` permissions are set at the workflow's top-level `permissions:` block, not at job level. Only the recorder workflows carry `contents: write`, and they have an accepted-risk comment explaining why. The structural mitigation is branch protection: enforce that only the `memory/triage` branch is writable by the bot from those workflows, so a compromise of a recorder workflow cannot land changes on `main`.

Third-party actions that need a token are passed `gitHubToken` (or equivalent) only when the surrounding job's `permissions:` block already scopes it down. `cloudflare/wrangler-action` for example is gated to `deployments: write`.

### workflow_run Chain Documentation

`workflow_run` triggers create an implicit dependency edge that is invisible at the consumer file. Each consumer ( `record-triage.yml` ) carries an inline comment describing:

- the producing workflow ( `triage-bug.yml` ) and the artefact name
- the blast radius ( append-only JSONL write to `memory/triage` )
- why artefact contents are not executed ( stored verbatim, never `eval`'d / sourced / fed to a shell )

This makes the trust boundary obvious to future maintainers without forcing them to grep across workflow files.

### Audit Workflow

The audit ran an LLM panel ( Backdoor Hunter, Supply Chain Inspector, Integrity Analyst ) over the workflow set. The panel's `binary_change` notes are an artefact of the LLM hitting its 400 KB patch budget on the initial commit and are not actionable. Only the consolidated, multiple-reviewer-confirmed findings ( cargo-dist `remote_fetch_execute` ) were treated as real defects.

## Files

- `.github/workflows/*.yml` - all nine workflow files use SHA-pinned `uses:` references
- `.github/workflows/release.yml` - cargo-dist installer SHA-256 pinning lives here
- `.github/workflows/record-outcome.yml`, `record-triage.yml` - `contents: write` accepted-risk comments
- `.github/workflows/deploy-docs.yml` - `cloudflare/wrangler-action` token scope justification
- `.github/workflows/triage-bug.yml` - prompt-injection guardrail; covered separately in `github-issue-triage-and-automation.md`

## Test Coverage

- The audit was driven by an LLM panel; verification is by re-running the audit against future workflow changes rather than by automated tests.
- `gh workflow run` smoke tests confirm the `Install dist` step still succeeds against the pinned digest.

## Pitfalls

- Do not bump an action's `uses:` SHA without also updating the inline tag comment; the comment is what keeps the pinning auditable.
- Do not bump cargo-dist's URL without recomputing the SHA-256 digest. The pinning is only useful when the two values move together.
- Do not widen `GITHUB_TOKEN` permissions casually. Branch protection is the only structural mitigation for `contents: write`; widening the scope without revisiting branch protection is a regression.
- Do not assume the LLM-audit panel's `binary_change` notes are actionable; they are an artefact of patch-size truncation.
- Do not "fix" the rustup curl-pipe in `release.yml` without coordinating with the rust-lang.org guidance; it is the documented install path and only runs in container matrix entries.
