# Contributing to Winterbäume

Thank you for your interest in Winterbäume. This document describes what kinds of contributions are accepted, where to send them, and what to expect in return.

## Pull requests are not accepted

Winterbäume does **not currently accept pull requests** from external contributors, regardless of the size or scope of the change. Any pull request opened against this repository will be closed without review.

The reasons are practical rather than ideological:

- **Provenance and licensing.** The codebase is developed predominantly by AI coding agents under direct human review. Keeping the inbound contribution surface closed lets us preserve a coherent provenance record for every line in the tree, which matters for the project's Apache-2.0 licensing posture and for downstream users who may have stricter sourcing requirements.
- **Review capacity.** The project is maintained on a best-effort basis. Inbound code review is not a workflow we can sustain alongside the agent-driven development model.

If this policy changes, this document and `README.md` will be updated. Until then, please do not open pull requests.

## What is welcome

The following inputs are genuinely useful and we encourage you to send them:

### Bug reports

Open a GitHub issue using the `Bug report` template at `.github/ISSUE_TEMPLATE/bug_report.yml`. The template's structure is required, not decorative -- the `auto-label-service` workflow parses it to apply the correct `service:<slug>` label, and reports that do not match are silently mis-triaged.

The most valuable bug reports include:

- The exact `winterbaume-<service>` crate involved.
- A minimal reproduction against `aws-sdk-rust` or the AWS CLI.
- The observed behaviour and the expected behaviour, citing the AWS documentation or SDK behaviour you compared against.
- Versions: Winterbäume crate version, `aws-sdk-rust` crate version, and Rust toolchain.

If you are a coding agent operating on this repository, follow `skills/winterbaume-bug/SKILL.md` rather than drafting freely. It encodes the issue-form contract and the duplicate-search and confirmation checkpoints.

### Feature requests

Open a GitHub issue using the `Feature request` template at `.github/ISSUE_TEMPLATE/feature_request.yml`. Note that "please add operation X to service Y" is a feature request, not a bug -- a missing operation in a partially implemented service is by design until coverage is extended.

Filing a feature request does not guarantee it will be implemented. Coverage is driven primarily by what the maintainers and the agent workflow are working on.

### Knowledge-base entries

If you have discovered behaviour in real AWS that is not obvious from the public documentation and is relevant to emulator correctness, you can record it via the `KB entry` template at `.github/ISSUE_TEMPLATE/kb-entry.yml`. These feed the project's reference material for future work.

## Security vulnerabilities

Do **not** open a public issue for security vulnerabilities. See [SECURITY.md](./SECURITY.md) for the private reporting flow via GitHub Security Advisories.

## Behaviour in issue threads

Discussion in issues should stay focused on the technical content of the report. Off-topic, abusive, harassing, or discriminatory comments will be removed and the author may be blocked from the repository at the maintainers' discretion. There is no formal code of conduct at this time; this paragraph is the policy.

## Licensing of contributions

Anything submitted to this repository through an accepted channel (issues, discussions, knowledge-base entries) is contributed under the project's Apache-2.0 licence. Do not paste proprietary code, unredacted production logs, or content under incompatible licences into issues.

## Where things live

For orientation:

- `README.md` -- project overview, supported services, usage.
- `RELEASE.md` -- release runbook (intended for maintainers).
- `LICENSE`, `NOTICE` -- licence text and third-party attribution.
- `.agents/` -- agent-driven workflow scaffolding: long-term memory, skills, tooling for autonomous sessions. This material is intentionally part of the public repository.
- `crates/winterbaume-<service>/` -- per-service emulator implementations.
- `tools/` -- in-tree codegen, build cache, and other developer tooling.
