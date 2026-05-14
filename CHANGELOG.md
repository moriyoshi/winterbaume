# Changelog

This file summarises notable winterbaume releases across the workspace and serves
as the changelog for the umbrella `winterbaume` crate. Detailed per-crate notes
live in each crate's own `CHANGELOG.md` under `crates/<crate>/`.

Each crate is versioned independently via `cargo-release`. Release tags use the
`<crate-name>-v<version>` convention (e.g. `winterbaume-s3-v0.2.0`); the umbrella
`winterbaume` crate uses `winterbaume-v<version>`.

## Unreleased

- No changes since the v0.2.0 batch.

## 2026-05-13 — 2026-05-14: workspace v0.2.0 batch

The full workspace was rolled forward to v0.2.0 in a single coordinated batch
driven by `tools/release-batch`. 241 crates were tagged (125 on 2026-05-13 and
116 on 2026-05-14, paced by the crates.io `publish_new` rate limit). The
`winterbaume` umbrella crate itself was tagged on 2026-05-13.

Substantive crate changes:

- `winterbaume-terraform` v0.2.0: Adopted spec-driven serde codegen for all 145
  Terraform service serialisers, extracted generated Terraform-state projection
  types into the new `winterbaume-tfstate-resource-models` crate, and expanded
  Terraform resource converter coverage by roughly 700 new converters across
  EC2, S3, Route 53, IAM, API Gateway, Glue, RDS, Redshift, SageMaker,
  MediaLive, Network Firewall, S3 Tables and many other services. IAM Terraform
  coverage reached 32/34 (94%). See
  [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).
- `winterbaume-server` v0.2.0: Picked up the new Terraform resource converters
  wired through the server. See
  [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-tfstate-resource-models` v0.2.0: Initial public release of the
  new crate that hosts the generated Terraform-state projection models extracted
  from `winterbaume-terraform`. Versioned at v0.2.0 to align with the rest of
  the workspace batch; no prior v0.1.0 was published. See
  [`crates/winterbaume-tfstate-resource-models/CHANGELOG.md`](crates/winterbaume-tfstate-resource-models/CHANGELOG.md).

The remaining 237 service crates (e.g. `winterbaume-s3`, `winterbaume-sqs`,
`winterbaume-ec2`, `winterbaume-iam`, `winterbaume-dynamodb`, ...) were
republished without user-facing behaviour changes to pick up refreshed
crates.io keyword metadata and updated API coverage data. Each crate's own
`CHANGELOG.md` records this under an `Internal` section.

## 2026-05-09 — 2026-05-11: workspace v0.1.0 initial public launch

The first public release of winterbaume to crates.io: 240 crates tagged across
three days (53 on 2026-05-09, 186 on 2026-05-10, and 1 on 2026-05-11), paced
by the crates.io `publish_new` rate limit and driven by `tools/release-batch`
with automatic 429 retry handling. The `winterbaume` umbrella crate itself was
tagged on 2026-05-10.

Highlights:

- Published the umbrella `winterbaume` crate and 239 service / utility crates
  to crates.io, including all supported AWS service implementations
  (`winterbaume-s3`, `winterbaume-sqs`, `winterbaume-ec2`, `winterbaume-iam`,
  `winterbaume-dynamodb`, `winterbaume-sns`, `winterbaume-sts`, ...), the
  standalone `winterbaume-server` binary, the `winterbaume-terraform` state
  injection / extraction crate, and supporting utilities such as
  `winterbaume-sqs-redis`, `winterbaume-dynamodb-redis`,
  `winterbaume-sqlengine-duckdb` and `winterbaume-wafv2-wcu-calculator`.
- Established the full public release workflow: `cargo-release` for
  per-crate publishing, `cargo-dist` for binary releases of
  `winterbaume-server`, generated per-crate changelogs, public security
  reporting via `SECURITY.md`, and contribution policy via `CONTRIBUTING.md`.
- Shipped the `tools/release-batch` driver that chunks `cargo-release`
  invocations to fit the crates.io `publish_new` rate limit and retries
  automatically on HTTP 429 responses.

Per-crate first-release notes live in each crate's `CHANGELOG.md`.
