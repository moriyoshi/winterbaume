# Crate Publishing and Release

## Summary

Winterbaume's crate-publishing readiness was audited and prepared across April and May 2026. This document captures the publish-order constraints, cargo-dist binary release setup, per-crate README generation, release metadata corrections, and first-public-launch chunking strategy needed for crates.io publication.

## Key Facts

- The April audit covered the original 180 publishable workspace crates; the May first-launch audit covered 240 publishable crates after service expansion.
- All publishable crates have `description`, `license`, `repository`, and `readme` metadata by the May first-launch audit.
- The licence is `Apache-2.0` (not dual MIT/Apache - corrected during the audit).
- `winterbaume-e2e-tests` and `smithy-codegen` are `publish = false` (internal-only).
- `winterbaume-core` has no internal dependencies and must be published first.
- The first public release cannot be published as one workspace-sized `cargo release --execute` batch unless crates.io raises the `publish_new` quota.
- `tools/release-batch/` is the first-launch driver: it topologically chunks publishable crates into batches small enough for the crates.io upfront quota check.
- `release-batch` recovery must treat crates.io publication and git tag creation as separate side effects. A crate can upload successfully, then miss its tag if the surrounding cargo-release invocation aborts before the tag step.
- The crates.io HTTP API is useful for pre-flight and best-effort pruning, but cargo's own `is already published to crates.io` pre-flight output is the stronger recovery signal because it reads the registry index cargo will actually use.
- The DMS example uses raw `MockRequest`/`MockService` calls. The published AWS SDK crate for DMS is `aws-sdk-databasemigration` ( without the trailing `service` ); the v0.0.0 `aws-sdk-databasemigrationservice` is a placeholder. The winterbaume crate is `winterbaume-databasemigration` to match.

## Details

### Publishing Order

Crates must be published in dependency-graph order (leaf crates first):

1. **Layer 0**: `winterbaume-core` (no internal deps)
2. **Layer 1**: Standalone library crates that depend only on `winterbaume-core` (`winterbaume-iam-rule-eval`, `winterbaume-partiql`, etc.)
3. **Layer 2**: Service crates (depend on `winterbaume-core` and possibly library crates)
4. **Layer 3**: Umbrella `winterbaume` crate and `winterbaume-server` (depend on many service crates)

Within each layer, crates are sorted by reverse-dependent count (most depended-on first, then alphabetical). For first-launch publication, prefer `tools/release-batch/` over older all-at-once scripts because it preserves topological order while respecting crates.io's new-crate quota.

### First Public Release Chunking

`tools/release-batch/` is a Rust binary workspace member with `publish = false`. It exists because crates.io checks the user's `publish_new` quota upfront before cargo-release reaches any per-crate hook. A `pre-release-hook` that sleeps between publishes cannot help: cargo rejects an oversized new-crate batch before the hook runs.

The driver reads `cargo metadata`, drops `publish = false` packages, sorts publishable crates with Kahn's algorithm, then emits or runs per-chunk `cargo release <version|level> -p ...` commands. The validated first-launch shape is 240 publishable crates, chunk size 5, and 48 chunks. Chunk 1 contains leaf utility crates such as `winterbaume-core`, `winterbaume-bedrock-flow-parser`, `winterbaume-iam-rule-eval`, `winterbaume-partiql`, and `winterbaume-sfn-asl-eval`; chunk 48 contains the umbrella, server, and Terraform meta-crates.

Operational details:

- Default sleep is 660 seconds (600 second crates.io window plus 60 second buffer).
- The driver tees cargo's stdout/stderr while capturing them. On a chunk failure, it scans the captured output for the `Please try again after <date> GMT` phrase that crates.io embeds in 429 response bodies, parses the RFC 1123 deadline with `httpdate`, sleeps until then plus `--retry-buffer` seconds (default: 30), and retries the same chunk. Capped at `--max-retries` retries (default: 3). This keeps the run alive when an account's `publish_new` quota is partially consumed before the run starts (default burst is 5, so even chunk-size 5 can hit the quota cold). Non-rate-limit failures still abort.
- On retry, the driver works from a mutable `working_chunk`, not the original immutable chunk. After a 429 sleep, it prunes crates that have already landed on crates.io and retries only the survivors. If the whole chunk has already landed, the chunk is treated as successful.
- Cargo may report already-published crates before the HTTP probe sees them because `/api/v1/crates/<name>/<version>` is Fastly-cached and can lag the registry index. `parse_already_published()` scans cargo output for `error: <crate> <version> is already published to crates.io`, prunes those crates regardless of whether a 429 is also present, and retries the smaller chunk.
- Retry counters count only rate-limit sleeps. Prune-driven retries are naturally bounded by chunk size because every retry must remove at least one crate.
- The cargo executable is resolved as `--cargo <path>` > `WB_CARGO` > `cargo` on `PATH`, so agents and operators can point the driver at `.agents/bin/cargo.sh`.
- Plan-only runs do not require `--version`; they print `<version|level>` in the command preview.
- `--execute` requires a version argument, but the argument is passed through verbatim to cargo-release and may be a concrete semver or a release level such as `patch`, `minor`, `major`, `release`, `alpha`, `beta`, or `rc`.
- Resumability checks against `/api/v1/crates/<name>/<version>` only run for concrete semver versions. If the version is a level, the driver skips this check and prints a note because per-crate target versions are cargo-release's responsibility.
- Printed per-chunk commands intentionally keep the display form `$ cargo ...` even when `--cargo` or `WB_CARGO` resolves to another executable. The resolved path appears once in the startup banner.
- If a chunk is interrupted after some crates upload but before cargo-release reaches its tag step, those crates are published but untagged. The recovery path now calls `backfill_tag()` for pruned crates: if `<crate>-v<version>` is absent, create a signed annotated tag when `--sign` is enabled ( otherwise an annotated tag ) at the current `HEAD`, then push it. Tag-backfill warnings do not abort the chunk because missing tags are bookkeeping failures, not publication blockers.
- The May 2026 first-launch run exposed this tag race at scale: 167 already-published crates lacked their `<crate>-v0.1.0` tags after repeated partial 429s. A one-off backfill created signed annotated tags for those published crates, all pointing at the same release commit as the existing tags.

`tools/release-throttle.sh` and the Python `tools/release-batch.py` prototype were removed after the Rust driver landed.

### Umbrella Dependency Limit

The first public release also exposed a crates.io dependency-count limit on the umbrella `winterbaume` crate. The root package had 214 optional service dependencies and 455 dev-dependencies for example files. Because `autoexamples = false` and no `[[example]]` targets were declared, those dev-dependencies were unused, but cargo still kept versioned dev-dependencies in the published manifest. crates.io applies a maximum of 500 total dependencies, so the 669-entry manifest was rejected.

The durable fix is to keep the umbrella crate's published manifest minimal:

- delete unused root-package dev-dependencies when examples are not part of the package
- keep `autoexamples = false` while examples are excluded from the root package include list
- verify `cargo package -p winterbaume --no-verify` and inspect the generated manifest when changing the root package

After removing the root dev-dependency block, the packaged umbrella manifest contained 214 dependencies and 0 dev-dependencies, and the compressed crate size dropped materially.

### Umbrella Package Include Rules

The umbrella `winterbaume` crate is unusual because its package manifest is the workspace root `Cargo.toml`. Without an explicit package whitelist, `cargo package` walks the entire repository and includes agent memory, vendored Smithy models, VitePress sources, examples, GitHub workflows, and other non-crate material.

The root package must keep an anchored `include` whitelist. Bare include patterns such as `README.md` use cargo's gitignore-style matching and behave like `**/README.md`, which pulls nested documentation from `vendor/` and `docs/node_modules`. Use leading `/` or otherwise slash-qualified entries such as `/src/**/*.rs`, `/Cargo.toml`, and `/README.md`.

When `examples/` is intentionally excluded from the umbrella package whitelist, keep `autoexamples = false` so cargo does not auto-discover all root `examples/<service>.rs` targets and then warn about hundreds of excluded example files during release packaging.

### cargo-dist Binary Releases

Binary releases for `winterbaume-server` use cargo-dist (v0.31.0), configured in `dist-workspace.toml`:

- **Targets**: `aarch64-apple-darwin`, `aarch64-unknown-linux-gnu`, `aarch64-pc-windows-msvc`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `x86_64-pc-windows-msvc`
- **Trigger**: pushing a version tag (e.g. `v0.1.0` or `winterbaume-server/v0.1.0`)
- **Build profile**: `[profile.dist]` with `inherits = "release"` and `lto = "thin"`
- **CI gating**: the release workflow calls `ci.yml` as a reusable workflow; no artefacts build unless CI passes
- Only `winterbaume-server` is dist-able (the sole binary crate with `publish != false`)

### Per-Crate READMEs

Each crate has its own `README.md` (no workspace-level `readme` inheritance):

- **Service crate READMEs** (164): generated by `update_readme.py`, include umbrella crate reference paragraph, inline code example sourced from `examples/<slug>.rs`
- **Standalone library crate READMEs** (14): generated with description, umbrella crate reference, and licence section
- Example slug aliases are configured via `_EXAMPLE_SLUG_ALIASES` in `update_readme.py` for crates whose example file name differs from the crate name

### Metadata Corrections

- `[workspace.package].license` was `"MIT OR Apache-2.0"` but only an Apache-2.0 `LICENSE` file existed; corrected to `"Apache-2.0"`
- `repository` field was added to `[workspace.package]` and inherited by all 180 crates
- `winterbaume-s3files` needed explicit `readme = "README.md"` and `repository.workspace = true`; missing `repository` would have silently removed the repo link from the crates.io page even though the static checker only flagged `readme`
- The root `winterbaume` package needed anchored `include` patterns and `autoexamples = false` to avoid shipping `.agents/`, `vendor/`, generated docs, and all example sources

## Files

- `Cargo.toml`: workspace-level `[workspace.package]` metadata (license, repository)
- `dist-workspace.toml`: cargo-dist configuration
- `.github/workflows/release.yml`: cargo-dist release workflow
- `.github/workflows/ci.yml`: CI workflow (also used as reusable workflow by release)
- `tools/release-batch/`: dependency-ordered chunked cargo-release driver for first-launch publication
- `RELEASE.md`: operator checklist and first-launch release-batch guidance
- `scripts/publish-all.py`: older dependency-ordered batch publish script
- `.agents/skills/update-readme/scripts/update_readme.py`: per-crate README generation

## Pitfalls

- Do not publish crates out of dependency order; downstream crates will fail to resolve unpublished internal deps.
- Do not assume `cargo package` succeeds for crates with unpublished workspace dependencies; test leaf crates first.
- Do not rely on cargo-release hooks to work around the crates.io `publish_new` limit; the quota check runs before per-crate publish hooks.
- Do not run first-launch publishing as one workspace `--execute` batch unless crates.io has raised the new-crate quota.
- Do not use bare root-package `include` patterns such as `README.md`; anchor them to avoid packaging nested repository content.
- Do not remove `autoexamples = false` from the umbrella crate while `examples/` is excluded from the package include list.
- Do not expect resumability checks to work when `release-batch --version` is a release level rather than a concrete semver.
- Do not assume a successful upload implies a matching git tag exists. A partial cargo-release invocation can upload crates and abort before tag creation.
- Do not rely only on the crates.io HTTP API immediately after publication. Cargo's `is already published to crates.io` message may be the first reliable sign that the registry index already contains the version.
- Do not leave unused root dev-dependencies in the umbrella crate. crates.io counts dev-dependencies toward the package dependency limit.
- `aws-sdk-databasemigrationservice` ( with trailing `service` ) is a v0.0.0 placeholder — depend on `aws-sdk-databasemigration` for the real SDK client.
- The publish script was rewritten from shell to Python because bash `mapfile`, subshell variable scoping, and prezto alias interference made the shell version non-portable on macOS.
