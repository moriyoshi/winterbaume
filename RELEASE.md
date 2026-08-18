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

Five subcommands. The first four are the steady-state per-crate flow ( `version-bump` is optional — see below ); `batch` is the bypass mode for first-launch or targeted retries.

```sh
# Steady-state: per-crate semver bumps + selective publish
./.agents/bin/cargo.sh run -p release-harness -- plan
./.agents/bin/cargo.sh run -p release-harness -- changelog
./.agents/bin/cargo.sh run -p release-harness -- version-bump                         # dry-run, optional
./.agents/bin/cargo.sh run -p release-harness -- version-bump --execute
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

#### `version-bump` — apply the planned versions to the manifests

A thin driver over `cargo release version`. It reads `release-plan.toml`, groups the entries by their literal `next` version, and runs one `cargo release version <next> -p <crates>... --allow-branch '*' --execute` per group. cargo-release does the actual editing: it rewrites the crate's `[package] version`, follows the workspace-inheritance chain into the root `[workspace.dependencies].<crate>.version`, preserves comments and inline-table spacing, leaves a path-only dep entry alone, and refreshes `Cargo.lock`.

```
$ release-harness version-bump
wb-a         0.2.0 -> 0.2.1  (patch)
wb-b         0.3.0 -> 0.8.0  (pinned)
wb-umbrella  0.7.0 -> 0.8.0  (pinned)

3 crate(s) across 2 target version(s)

$ cargo release version 0.2.1 -p wb-a --allow-branch '*' --execute
$ cargo release version 0.8.0 -p wb-b -p wb-umbrella --allow-branch '*' --execute

(dry run — re-run with --execute to write)
```

What the plan buys over calling cargo-release by hand:

| Behaviour | Detail |
|-----------|--------|
| Dry run by default | Prints the per-crate `current -> next` table and the invocations it would run. `--execute` runs them. |
| Idempotent | Each crate is dispatched at its literal `next`, never a level — `cargo release version patch` re-run bumps a second time, the same literal re-run is a no-op. Crates already at `next` are dropped from the dispatch entirely, so a partially-applied run is safe to repeat. `initial` entries ( `next` == `current` ) are always no-ops. |
| Pre-flight drift check | A manifest matching neither `current` nor `next` means the plan no longer describes this tree: every such crate is reported and nothing is invoked. cargo-release catches only the downgrade direction, and only after it has already written the earlier groups. |
| One invocation per target version | `cargo release version` applies one version to every package it is given, so crates sharing a `next` ride along together. Each invocation re-reads workspace metadata — budget roughly a minute per group on this workspace. |
| Aborts on failure | A failing group stops the run with cargo-release's exit status; later groups are not invoked. |

`--allow-branch '*'` is passed because cargo-release otherwise refuses to run outside the configured release branch, and bumping on a topic branch for review is the whole point of this subcommand. Nothing is published here. `--no-confirm` is passed too — the table above is the preview. The harness makes no git commit; review the diff and commit it yourself.

This step is **optional**. `publish` drives the same `cargo release version` step itself, so the shortest path is `plan` → `changelog` → `publish`. Run `version-bump` when the bump should land as its own reviewable commit ahead of the publish — for example when the version change goes through a PR, or when CI needs to build the bumped tree before anything reaches crates.io.

Running `version-bump --execute` and then `publish` against the same plan is safe: both dispatch the plan's concrete `next`, so `publish` finds the manifests already at their target, reports `manifests already at target — skipping version/replace/hook`, and goes straight to committing, publishing, and tagging. ( That composition is only sound because `publish` groups by version rather than by bump level — see below. )

#### `publish` — chunked publish grouped by target version

Groups plan entries by their concrete `next` version and runs one chunked `cargo release` per group, in-process. Without `--execute`, prints the planned invocations and exits. With `--execute`, drives the actual publish.

Grouping is by version, not by bump level, because a level keyword is the wrong dispatch currency in two ways:

- **The version step is not idempotent under a level.** `cargo release version patch` re-run bumps a *second* time. A chunk that was partially bumped before a failure — a crash or Ctrl+C midway through the step — would have its already-bumped crates moved again on resume, silently skipping a version number.
- **A level disables the resumability prune.** `batch` can only drop crates whose target is already on crates.io when it knows the concrete target; given a level it prints `skipping pre-flight crates.io resumability check` and prunes nothing.

The plan already computes `next` for every bumping crate, so `publish` dispatches that. On this workspace the 241 publishable members currently sit at 10 distinct versions, so a workspace-wide patch bump drives ~10 groups rather than 1 — a handful of extra partial chunks, which matters only under the first-launch `--sleep` settings.

Within `batch`, the version step is additionally computed per crate rather than per chunk: only the crates not yet at their target are handed to `cargo release version`. Under a level dispatch ( reachable through `batch --version patch`, the bypass mode ) a crate whose current version is *not* on crates.io is treated as already bumped and left alone. When the crates.io lookup itself fails, that check reports "not published" and the crate is skipped — the safe direction, since a skipped bump surfaces as a failed or pruned publish, whereas a spurious bump burns a version number that cannot be unpublished.

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
