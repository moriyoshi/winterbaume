# Release Workflow

This file is the canonical release runbook. The `verify-publish-ready` skill embeds the first-time public launch and recurring release criteria used for automated audits. `.agents/docs/TODO.md` may track derived work, but it is not authoritative for release criteria.

## Prerequisites

- Authenticated with crates.io: `cargo login`.
- All intended changes committed. `cargo-release` rejects dirty trees.
- On the `main` branch.
- CI passing.
- `cargo-release` installed: `cargo install cargo-release`.
- Agents must invoke Cargo through `./.agents/bin/cargo.sh` except when deliberately troubleshooting the wrapper with an explicit `CARGO_TARGET_DIR` under `.agents-workspace/tmp/`. Humans can use their normal Cargo environment.

## Crate Release

The workspace uses [cargo-release](https://github.com/crate-ci/cargo-release) to handle the full release lifecycle: version bump, crates.io publish, git tag, and push. Each crate is versioned independently.

```sh
# Release a single crate (dry run)
cargo release patch -p winterbaume-s3

# Actually perform the release
cargo release patch -p winterbaume-s3 --execute

# Release multiple crates at once
cargo release patch -p winterbaume-s3 -p winterbaume-sqs --execute
```

Agents should run the same subcommands through the wrapper:

```sh
./.agents/bin/cargo.sh release patch -p winterbaume-s3
```

Replace `patch` with `minor` or `major` as appropriate.

### What cargo-release Does

1. Bumps the version in the target crate's `Cargo.toml`.
2. Commits the version bump: `chore: release winterbaume-s3 v0.1.1`.
3. Publishes the crate to crates.io, skipping `publish = false` crates.
4. Tags the commit: `winterbaume-s3-v0.1.1`.
5. Pushes the commit and tag to `origin`.

The pushed `winterbaume-server-vX.Y.Z` tag triggers the binary release workflow.

### Configuration

Configured in `[workspace.metadata.release]` in the root `Cargo.toml`:

| Setting | Value | Purpose |
|---------|-------|---------|
| `shared-version` | `false` | Each crate is versioned independently |
| `allow-branch` | `["main"]` | Only release from `main` |
| `tag-name` | `{{crate_name}}-v{{version}}` | Per-crate tags compatible with cargo-dist |
| `verify` | `false` | Skip build verification during publish because cross-crate dependencies may not be on crates.io yet |

### Excluded From Publishing

- `winterbaume-e2e-tests`: `publish = false`
- `smithy-codegen`: `publish = false`
- `sccache-wrapper`: `publish = false`
- `release-harness`: `publish = false`

### Release Harness (`tools/release-harness/`)

A single Rust binary owns the release lifecycle: it both classifies per-crate semver bumps from `git diff` since each crate's last `<crate>-v<ver>` tag and drives the chunked `cargo release` invocations that respect the crates.io `publish_new` rate limit (default: 5 new crates per 10 minutes, which would otherwise reject a workspace-wide release upfront).

Four subcommands. The first three are the steady-state per-crate flow; `batch` is the bypass mode for first-launch or targeted retries.

```sh
# Steady-state: per-crate semver bumps + selective publish
./.agents/bin/cargo.sh run -p release-harness -- plan
./.agents/bin/cargo.sh run -p release-harness -- changelog
./.agents/bin/cargo.sh run -p release-harness -- publish                              # dry-run
./.agents/bin/cargo.sh run -p release-harness -- publish --execute --sign --no-confirm

# Bypass mode: chunked publish at one version across every (or a listed) crate
./.agents/bin/cargo.sh run -p release-harness -- batch --version 0.1.0                # dry-run
./.agents/bin/cargo.sh run -p release-harness -- batch --version 0.1.0 --execute --sign --no-confirm
./.agents/bin/cargo.sh run -p release-harness -- batch --version 0.1.0 --crates winterbaume-s3 winterbaume-sqs --execute
```

#### `plan` — discovery + per-crate semver classification

For each publishable crate, resolve the latest `<crate>-v<X.Y.Z>` tag, `git diff` since, classify:

| Outcome | Trigger |
|---------|---------|
| `unchanged` | No files changed since the tag. |
| `skip` | Only cosmetic files changed: `README.md`, `CHANGELOG.md`, `NOTICE`, `LICENSE`, images, or anything under a `docs/` subdirectory. Not published. |
| `patch` | At least one substantive file changed (`src/**`, `tests/**`, `Cargo.toml`, ...) and no new public symbols added. |
| `minor` | A new `pub fn` / `pub struct` / `pub enum` / `pub trait` / `pub mod` / `pub const` / `pub static` / `pub type` / `pub use` line was added under `src/**` since the last tag. |
| `major` | `cargo-semver-checks check-release` reports at least one breaking lint at the candidate level. Under 0.y.z this resolves to `0.y → 0.(y+1)`. |
| `pinned` | Operator forced a literal version in `release-plan-overrides.toml`. |
| `initial` | No prior `<crate>-v<ver>` tag — publish at the current `Cargo.toml` version with no bump. |

For the `major` escalation to fire, install the tool: `cargo install cargo-semver-checks`. Without it, the harness falls back to the heuristic alone and warns. It never silently downgrades.

Per-crate overrides go in `release-plan-overrides.toml` at the repo root (checked in when used). Override entries always win over the heuristic + semver-checks classification:

```toml
[bumps]
winterbaume-ec2 = "minor"     # force a level
winterbaume-foo = "skip"      # exclude
winterbaume-bar = "0.3.1"     # pin a literal version
```

Output: `release-plan.toml` at the repo root (gitignored) plus a stdout summary grouped by bump level.

#### `changelog` — per-crate CHANGELOG drafts

Reads `release-plan.toml`. For each non-skip crate, runs `git log <last-tag>..HEAD -- <crate-dir>/`, buckets commits by conventional-commit-style prefix (`feat:` → Added, `fix:` → Fixed, etc.), and prepends a fresh `## v<next> - <date>` section to that crate's `CHANGELOG.md`. The root umbrella `CHANGELOG.md` gets a matching dated rollup. Drafts are mechanical — polish the wording (or invoke the `generate-changelog` skill) before committing.

#### `publish` — chunked publish grouped by bump level

Groups plan entries by bump level (`patch`, `minor`, `major`, plus each pinned literal version as its own one-crate group) and runs one chunked `cargo release` per group, in-process. Without `--execute`, prints the planned invocations and exits. With `--execute`, drives the actual publish.

#### `batch` — direct chunked publish, no plan file

The bypass mode. Takes `--version <level-or-semver>` and an optional `--crates <names>...` / `--crates-file <path>` subset, runs the same chunked-publish logic without going through a plan. Used for:

- **First launch.** No prior tags exist, every crate ships at the same fresh version. Drive the entire workspace in one `batch` run.
- **Targeted retries.** A specific crate fails mid-cycle; re-run just that one without re-classifying the whole workspace.

Chunked-publish behaviour (shared by `publish` and `batch`):

- Reads workspace members via `cargo metadata`, drops `publish = false` crates, topologically sorts the in-scope subset so dependencies publish before dependants.
- Skips crates that are already at the target version on crates.io, making mid-launch retries idempotent. Pass `--skip-version-check` to opt out.
- Per chunk, runs `cargo release <version> -p crate1 -p crate2 ...` with optional `--sign`, `--no-confirm`. Sleeps `--sleep` seconds between chunks (default: 660 = 600s rate window + 60s buffer).
- A chunk that fails with crates.io HTTP 429 ("publish_new" rate limit) is retried automatically: tees cargo's output, parses the embedded `Please try again after <date> GMT` deadline, sleeps until then plus `--retry-buffer` seconds (default: 30), and re-runs the same chunk. Up to `--max-retries` retries per chunk (default: 3); other failure modes still abort immediately.
- A failing chunk that is not a recoverable rate-limit hit aborts with a non-zero exit code. Re-running resumes from the first crate not yet at the target version on crates.io.
- The cargo executable is configurable via `--cargo <path>` or the `WB_CARGO` environment variable, so the harness can be pointed at the project's wrapper script.

Operational notes:

- **First-launch wall time.** 240 new crates ÷ 5 per chunk × 660 s/chunk ≈ 8.5 hours unattended. Plan accordingly, or request a higher `publish_new` rate from crates.io support before launch and raise `--chunk-size` to match.
- **Subsequent releases.** After the first launch every crate already exists on crates.io, so the much higher `publish_existing` rate applies and `publish` (or a `batch --chunk-size <large>`) finishes in minutes.

`release-plan.toml` is gitignored. `release-plan-overrides.toml` is not — when overrides are used, they should be visible in the same commit that lands the changelog draft.

## Binary Release

Binary releases are managed by [cargo-dist](https://github.com/axodotdev/cargo-dist) and automated via GitHub Actions.

### Trigger

The tag pushed by `cargo release` for `winterbaume-server`, for example `winterbaume-server-v0.1.1`, automatically triggers the release workflow. You can also push a tag manually:

```sh
git tag winterbaume-server-v0.1.1
git push origin winterbaume-server-v0.1.1
```

### What Happens

1. The CI workflow runs first: fmt, clippy, test, examples, and E2E.
2. If CI passes, cargo-dist builds `winterbaume-server` for all target platforms.
3. A GitHub Release is created with:
   - Platform archives (`.tar.xz` for Unix, `.zip` for Windows)
   - SHA-256 checksums
   - `LICENSE` and `README.md` bundled in each archive

### Target Platforms

| Target | OS |
|--------|----|
| `aarch64-apple-darwin` | macOS (Apple Silicon) |
| `x86_64-apple-darwin` | macOS (Intel) |
| `aarch64-unknown-linux-gnu` | Linux (ARM64) |
| `x86_64-unknown-linux-gnu` | Linux (x86_64, glibc) |
| `x86_64-unknown-linux-musl` | Linux (x86_64, musl/static) |
| `aarch64-pc-windows-msvc` | Windows (ARM64) |
| `x86_64-pc-windows-msvc` | Windows (x86_64) |

### Configuration Files

- `dist-workspace.toml`: cargo-dist workspace configuration
- `.github/workflows/release.yml`: generated by `cargo dist generate --mode=ci`
- `Cargo.toml` `[profile.dist]`: release profile with `lto = "thin"`

## Release Execution Checklist

- [ ] Start from a clean worktree on `main`.
- [ ] Confirm the `verify-publish-ready` skill reports no first-time launch blockers if this is the first public release.
- [ ] Run local verification or confirm the same checks pass in CI: fmt, clippy, workspace tests, examples, Terraform E2E, docs build, and cargo-dist release plan.
- [ ] Regenerate API coverage and README if any service, Terraform converter, or generated service page changed before release.
- [ ] Generate or refresh `CHANGELOG.md` with the `generate-changelog` skill, including relevant per-crate changelogs for the release set.
- [ ] For steady-state releases, run the release harness: `./.agents/bin/cargo.sh run -p release-harness -- plan`, inspect `release-plan.toml`, then `./.agents/bin/cargo.sh run -p release-harness -- changelog` and review the per-crate CHANGELOG diffs.
- [ ] Dry-run the publish: `./.agents/bin/cargo.sh run -p release-harness -- publish` for the plan-driven case, or `./.agents/bin/cargo.sh run -p release-harness -- batch --version <ver>` for first-launch / single-version mode. Budget the wall time accordingly for first launch (~8.5 hours for 240 new crates at the default rate, less if a higher rate is granted).
- [ ] Publish: re-run the same harness command with `--execute --sign --no-confirm` for unattended GPG-signed runs.
- [ ] Push the `winterbaume-server-vX.Y.Z` tag after crate publish succeeds.
- [ ] Verify GitHub Release artefacts, checksums, bundled `LICENSE`, bundled `README.md`, and platform archive contents.
- [ ] Verify docs deploy after release.
- [ ] Publish the public announcement with exact version, supported surfaces, known limitations, contribution policy, and security contact.
