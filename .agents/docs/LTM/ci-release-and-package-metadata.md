# CI Release and Package Metadata

## Summary

Winterbaume's CI and release workflows were refined in May 2026 to reduce unnecessary CI work, make repeated checks reusable for identical source trees, keep release targets shippable, and improve crates.io discoverability metadata. The durable pattern is to gate expensive CI only when the code-relevant tree changed, cache successful job fingerprints cautiously, and keep cargo-dist target support tied to toolchain reality rather than an aspirational matrix.

## Key Facts

- `.github/workflows/ci.yml` skips the full pipeline for docs-only pushes to `main`.
- The leading `changes` job uses `dorny/paths-filter@fbd0ab8f3e69293af611ebaee6363fc25e6d187d` ( v4.0.1 ) pinned by SHA.
- Downstream CI jobs can cascade-skip through `needs:` when the root `fmt` job is skipped.
- CI also has per-job pass markers keyed by a content address of the code-relevant tree and toolchains, stored as artifacts so every branch sees them.
- Job-level `if: !failure() && !cancelled()` lets skipped upstream jobs flow through while preserving failure and cancellation semantics.
- cargo-dist release builds currently target five platforms, after dropping musl Linux and Windows ARM due to recurring upstream/toolchain failures.
- `backend-sqlengine-duckdb-bundled` remains enabled for cargo-dist server artefacts, but no static musl Linux artefact is produced.
- Workspace crates.io keywords are centralised through `[workspace.package]` and inherited by publishable crates.

## Details

### Docs-Only CI Skips

The CI workflow now starts with a `changes` job that classifies a push with `dorny/paths-filter`. The filter treats these as code-relevant:

- `crates/**`
- `src/**`
- `tools/**`
- `examples/**`
- `vendor/**`
- `Cargo.toml`, `Cargo.lock`, `rustfmt.toml`, `dist-workspace.toml`, `.gitmodules`
- `.github/workflows/ci.yml`
- `.github/actions/**`

Docs-only pushes to `main` run only the lightweight `changes` job. `workflow_dispatch` and `workflow_call` continue to run the full pipeline.

Only the root `fmt` job needs the path-filter guard. Downstream jobs keep their normal `needs:` chain and are skipped when `fmt` is skipped.

### Prior-Pass CI Markers

The `fingerprint` job computes a content address and probes for markers shaped like:

```text
ci-pass-{fmt,clippy,test,examples}-Linux-<key>
ci-pass-e2e-Linux-<e2e key>
```

- `<key>` is a SHA-256 over the git objects of the code-relevant paths ( `git ls-tree -r HEAD -- <paths>`, Markdown excluded, the `vendor/` submodule as its commit ) and the current stable Rust release from `channel-rust-stable.toml`. The E2E key adds the current Terraform release. Both toolchains float: `setup-rust-toolchain` and `setup-terraform` install the latest release, which the tree does not pin. A toolchain lookup that fails yields a value unique to the run, so it can only cause a rerun.
- A job that succeeds uploads its marker as a workflow artifact ( 30-day retention ). The probe lists artifacts by name through the REST API and counts only those whose `workflow_run.head_repository_id` is this repository, because a fork's PR run executes the fork's own workflow and could upload any name.
- Until 2026-11-25 the markers were `actions/cache` entries keyed by `hashFiles()`. GitHub scopes a cache entry to the branch that saved it ( a run can restore only its own branch's and the default branch's ), so the run on `main` after a merge never saw the PR's markers and repeated the whole hour. Artifacts are listed repository-wide: a PR tested against the tip of `main` and then merged gives the merge commit the same key, and `main` skips.
- `fingerprint` needs `actions: read`. A called workflow gets at most its caller's permissions, so `release.yml` grants `contents: read` and `actions: read` on the job that calls `ci.yml`.

The accepted drawbacks are:

- runner-image drift is not in the key
- a flaky pass can be frozen for the same key
- downstream consumers need to accept `skipped` as success if branch protection later requires these checks
- a same-repository collaborator can upload a marker by hand; markers trust write access, as the cache entries did

The default job-level `if: success()` was replaced with `!failure() && !cancelled()` where needed, because `success()` treats skipped upstream jobs as non-success.

### Release Matrix Adjustments

The original cargo-dist target list included:

- `aarch64-apple-darwin`
- `aarch64-unknown-linux-gnu`
- `aarch64-pc-windows-msvc`
- `x86_64-apple-darwin`
- `x86_64-unknown-linux-gnu`
- `x86_64-unknown-linux-musl`
- `x86_64-pc-windows-msvc`

The release workflow failed on:

- `x86_64-unknown-linux-musl`: `libduckdb-sys` needed `x86_64-linux-musl-g++`; Ubuntu `musl-tools` provides `musl-gcc` and `musl-ar`, not the C++ compiler.
- `aarch64-pc-windows-msvc`: the untagged `messense/cargo-xwin` container lagged rustc stable and then, after `rustup update`, exposed a `ring` / cargo-xwin clang argument mismatch around `/imsvc`.

An attempted musl fix downloaded `https://musl.cc/x86_64-linux-musl-cross.tgz` with a pinned SHA-256, but GitHub-hosted runners timed out fetching musl.cc. At that point, mirroring the tarball was judged not worth the operational burden.

The current release target list is:

- `aarch64-apple-darwin`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `x86_64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`

The implications are explicit:

- no fully static Linux binary is produced
- `x86_64-unknown-linux-gnu` links to the GitHub runner glibc
- no Windows ARM binary is produced

### Crates.io Keywords

All publishable crates now inherit:

```toml
keywords = ["aws", "mock", "testing"]
```

The single source of truth lives in `[workspace.package]`; the umbrella crate and all service crates use `keywords.workspace = true`. Internal `tools/*` crates are intentionally excluded because they are `publish = false`.

The keyword set uses three of crates.io's five keyword slots and stays within crates.io constraints: lowercase, alphanumeric or `_`/`-`, at most 20 characters, and at most 5 keywords.

## Files

- `.github/workflows/ci.yml`: paths-filter skip, fingerprint job, and pass-marker artifacts.
- `.github/workflows/release.yml`: cargo-dist release workflow and target-specific setup.
- `dist-workspace.toml`: active cargo-dist target list.
- `Cargo.toml`: workspace-level keywords and per-crate inheritance.

## Test Coverage

Representative verification:

```bash
cargo metadata --no-deps --format-version 1
```

For workflow changes, the real verification is hosted GitHub Actions:

- docs-only push to `main`: only `changes` should run
- same-code rerun: jobs with pass markers should skip
- merge of a PR tested against the tip of `main`: the post-merge run on `main` should skip every job
- tag or release workflow path: five target artefacts should build

## Pitfalls

- Do not repeat the docs-only path-filter guard on every downstream job unless a future workflow graph requires it.
- Do not use `success()` when skipped upstream jobs should allow downstream cache-hit skips to continue; use `!failure() && !cancelled()`.
- Do not assume pass markers capture runner-image drift. Rust and Terraform releases are in the key; anything else floating is not.
- Do not move the markers back to `actions/cache`: its branch scoping hides a PR's markers from `main`.
- Do not re-add musl or Windows ARM cargo-dist targets without testing the current cargo-dist, DuckDB, cargo-xwin, clang, and AWS SDK MSRV combination.
- Do not rely on `musl.cc` availability from GitHub-hosted runners without a mirror or fallback.
- Do not leave `messense/cargo-xwin` unpinned if a container target returns; its `latest` tag moves on its own cadence.
- Do not add keywords to `tools/*` unless those crates become publishable.

