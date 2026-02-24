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

### Publish Rate Limit and the `release-batch` Driver

crates.io enforces a `publish_new` rate limit (default: 5 new crates per 10 minutes). cargo (not cargo-release) checks this upfront before a multi-crate batch run and rejects the entire run when the planned new-crate count exceeds the quota, so a single `cargo release --workspace --execute` cannot publish 240 new crates in one go.

The `release-batch` tool at `tools/release-batch/` drives cargo-release in chunks of at most `--chunk-size` crates per invocation, sleeping `--sleep` seconds between invocations so the rate window slides. Each invocation passes cargo's upfront check because its batch size stays under the limit.

```sh
# Build once.
./.agents/bin/cargo.sh build -p release-batch --release

# Plan-only — prints the topologically ordered chunks without invoking cargo-release.
./target/release/release-batch --version 0.1.0

# Real run, unattended.
./target/release/release-batch --version 0.1.0 --execute --no-confirm

# Real run with GPG-signed commits and tags.
./target/release/release-batch --version 0.1.0 --execute --sign --no-confirm
```

Behaviour:

- Reads workspace members via `cargo metadata`, drops `publish = false` crates, topologically sorts the rest so dependencies publish before dependants.
- Skips crates that are already at the target version on crates.io (queried over the public crates.io API), making mid-launch retries idempotent. Pass `--skip-version-check` to opt out.
- Per chunk, runs `cargo release <version> -p crate1 -p crate2 ...` with optional `--sign`, `--no-confirm`, `--execute`. Sleeps `--sleep` seconds between chunks (default: 660 = 600s rate window + 60s buffer).
- A failing chunk aborts the run with a non-zero exit code. Re-running the same command resumes from the first crate that has not yet been published at the target version.
- The cargo executable used for `metadata`, `locate-project`, and `release` is configurable via `--cargo <path>` or the `WB_CARGO` environment variable, so the driver can be pointed at the project's wrapper script (e.g. `WB_CARGO=./.agents/bin/cargo.sh ./target/release/release-batch ...`).

Operational notes:

- **First launch wall time.** 240 new crates ÷ 5 per chunk × 660 s/chunk ≈ 8.5 hours unattended. Plan accordingly, or request a higher `publish_new` rate from crates.io support before launch and raise `--chunk-size` to match.
- **Subsequent releases.** After the first launch every crate already exists on crates.io, so the much higher `publish_existing` rate applies. You can switch back to a single `cargo release patch --workspace --execute` for steady-state releases, or keep using `release-batch` (it remains correct — just slower than needed).

### Excluded From Publishing

- `winterbaume-e2e-tests`: `publish = false`
- `smithy-codegen`: `publish = false`
- `sccache-wrapper`: `publish = false`

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
- [ ] Dry-run the planned crate release with cargo-release, for example `./.agents/bin/cargo.sh release patch -p winterbaume-s3`.
- [ ] For the first public launch (or any batch above the `publish_new` rate limit), build and use the `release-batch` driver: `./.agents/bin/cargo.sh build -p release-batch --release` then plan-only run `./target/release/release-batch --version <ver>`. Budget the wall time accordingly (~8.5 hours for 240 new crates at the default rate, less if a higher rate is granted).
- [ ] Publish crates with cargo-release: for steady-state releases, repeat the planned command with `--execute`. For the first public launch (240 new crates), drive cargo-release through `./target/release/release-batch --version <ver> --execute --no-confirm` (add `--sign` if signing).
- [ ] Push the `winterbaume-server-vX.Y.Z` tag after crate publish succeeds.
- [ ] Verify GitHub Release artefacts, checksums, bundled `LICENSE`, bundled `README.md`, and platform archive contents.
- [ ] Verify docs deploy after release.
- [ ] Publish the public announcement with exact version, supported surfaces, known limitations, contribution policy, and security contact.
