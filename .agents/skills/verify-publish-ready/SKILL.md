---
name: verify-publish-ready
description: Verify whether the current Winterbaume repository state conforms to RELEASE.md and the public publish readiness gates embedded in this skill. Use when asked to check release readiness, publish readiness, first public release readiness, crates.io publishability metadata, cargo-dist release readiness, or public launch blockers.
---

# Verify Publish Ready

Check the current checkout against the release runbook in `RELEASE.md` and the embedded publish-readiness criteria below. This skill reports findings; do not fix issues unless the user explicitly asks.

Do not treat `.agents/docs/TODO.md` as the source of truth for release criteria. It may contain derived work items, but it is supplementary and can lag behind the runbook.

## Workflow

1. Read `RELEASE.md` and this skill's embedded criteria.
2. Run the static checker:

   ```bash
   python3 .agents/skills/verify-publish-ready/scripts/check_publish_ready.py
   ```

   For the first public launch, include:

   ```bash
   python3 .agents/skills/verify-publish-ready/scripts/check_publish_ready.py --first-public-release
   ```

3. Run the low-cost Cargo gates from the runbook:

   ```bash
   ./.agents/bin/cargo.sh fmt --all --check
   ./.agents/bin/cargo.sh metadata --no-deps --format-version 1
   ```

4. If the user requested a complete verification, run the heavier release gates:

   ```bash
   ./.agents/bin/cargo.sh clippy --workspace --exclude winterbaume-sqlengine-duckdb --all-targets --all-features -- -D warnings
   ./.agents/bin/cargo.sh test --workspace --exclude winterbaume-sqlengine-duckdb --no-fail-fast
   ./.agents/bin/cargo.sh clippy -p winterbaume-sqlengine-duckdb --no-default-features --all-targets -- -D warnings
   ./.agents/bin/cargo.sh test -p winterbaume-sqlengine-duckdb --no-default-features --no-fail-fast
   ./.agents/bin/cargo.sh test -p winterbaume-e2e-tests --test terraform -- --ignored
   ```

5. Check docs and coverage when relevant:

   ```bash
   python3 .agents/skills/api-coverage/scripts/generate_coverage.py
   python3 .agents/skills/update-readme/scripts/update_readme.py
   ```

   If documentation changed or the user requested full verification, run the VitePress docs build from `docs/` using the repo's existing Node tooling.

6. Generate or refresh `CHANGELOG.md` with the `generate-changelog` skill. For a crate release, refresh the relevant `crates/<crate>/CHANGELOG.md` files as well as the root umbrella changelog.

7. For crate publishing batches, follow the cargo-release flow documented in `RELEASE.md`. Run the planned `cargo release <level> -p <crate>` command without `--execute` as the dry run, then repeat with `--execute` only when actually publishing. For binary release planning, rely on the cargo-dist workflow and generated release plan.

## Report Format

Lead with the verdict:

- `PASS`: all requested gates passed.
- `FAIL`: one or more requested gates failed.
- `PARTIAL`: static and low-cost gates passed, but heavier gates were not run.

Then list:

- Commands run and whether they passed.
- Static checker failures and warnings.
- First-public-release blockers if `--first-public-release` was used.
- Manual or live checks that the script could not verify.
- Gates not run, with the reason.

## Embedded Publish-Readiness Criteria

These criteria are the source of truth for this skill's audits. Prefer updating this section and the bundled checker together when the release policy changes.

### First-Time Public Launch Gates

- Root legal and overview files exist: `README.md`, `LICENSE`, `NOTICE`, and the README image asset referenced by the project overview.
- Every publishable Winterbaume crate has crates.io metadata: description, licence, repository, and a crate-local README where expected.
- Non-publishable crates are explicitly marked `publish = false`: `winterbaume-e2e-tests`, `smithy-codegen`, and `sccache-wrapper`.
- The workspace has only the root `Cargo.lock`; crate-local lockfiles are absent.
- cargo-dist configuration and release workflow files exist: `dist-workspace.toml`, `.github/workflows/release.yml`, and `[profile.dist]` in the root manifest.
- CI covers format, clippy, workspace tests, examples, Terraform E2E, generated docs checks, and cargo-dist release planning.
- Documentation sources exist, while generated docs output and `node_modules` are not tracked.
- Issue templates and triage workflows exist for public bug and feature intake.
- Public security contact and vulnerability process are documented in `SECURITY.md`.
- Public contribution or support expectations are documented in at least one of `CONTRIBUTING.md`, `PULL_REQUESTS.md`, or `SUPPORT.md`.
- Code of conduct posture is decided before launch; `CODE_OF_CONDUCT.md` is preferred if the project accepts community contributions.
- Repository metadata, branch protection, GitHub Pages, labels, and required secrets are configured in GitHub.
- Git history and bundled agent material have been reviewed for secrets, private notes, and non-public data.
- Vendor, generated, and bundled third-party licence obligations have been reviewed.
- README public contribution policy is reviewed for accuracy.
- Full CI has passed before the first public release.
- Root `CHANGELOG.md` has been generated with the `generate-changelog` skill.
- cargo-release dry-runs have passed for the planned publishable crate release set.
- The cargo-release crate flow and cargo-dist binary release flow have been checked against current workspace membership.
- Crate descriptions have been reviewed for public-facing wording.

### Recurring Release Gates

- Start from a clean worktree on `main`.
- Full CI passes: format, clippy, workspace tests, examples, Terraform E2E, docs build, and cargo-dist release plan.
- API coverage and README are refreshed when service, Terraform converter, or generated service page coverage changed.
- Root `CHANGELOG.md` and relevant per-crate changelogs are generated or refreshed with the `generate-changelog` skill.
- The planned `cargo release <level> -p <crate>` command passes without `--execute`.
- Crates are published through cargo-release by repeating the planned command with `--execute`.
- The `winterbaume-server-vX.Y.Z` tag is pushed after crate publish succeeds.
- GitHub Release artefacts, checksums, bundled `LICENSE`, bundled `README.md`, and platform archive contents are verified.
- Documentation deploy is verified after release.
- Public announcement includes exact version, supported surfaces, known limitations, contribution policy, and security contact.

## Notes

- Use `./.agents/bin/cargo.sh` for Cargo commands unless deliberately troubleshooting the wrapper with an explicit `CARGO_TARGET_DIR` under `.agents-workspace/tmp/`.
- Treat a dirty worktree as a release blocker for an actual release, but not as a blocker when the user is asking for an audit during active edits.
- Do not delete files or create commits as part of verification.
