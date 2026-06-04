# Changelog

## 2026-06-04

- `winterbaume` v0.5.0: see [`crates/winterbaume/CHANGELOG.md`](crates/winterbaume/CHANGELOG.md).
- `winterbaume-cloudcontrol` v0.3.0: see [`crates/winterbaume-cloudcontrol/CHANGELOG.md`](crates/winterbaume-cloudcontrol/CHANGELOG.md).
- `winterbaume-server` v0.2.3: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).

## v0.5.0 - 2026-06-04

### Internal

- Maintenance release — no user-visible changes to the root `winterbaume` crate; updates in this window belong to constituent service crates (`winterbaume-cloudcontrol`) and build tooling (`sccache-wrapper`).

## 2026-05-25

- `winterbaume` v0.4.0: see [`crates/winterbaume/CHANGELOG.md`](crates/winterbaume/CHANGELOG.md).
- `winterbaume-cloudfront` v0.3.0: see [`crates/winterbaume-cloudfront/CHANGELOG.md`](crates/winterbaume-cloudfront/CHANGELOG.md).
- `winterbaume-cloudfrontkeyvaluestore` v0.2.1: see [`crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md`](crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md).
- `winterbaume-kms` v0.2.1: see [`crates/winterbaume-kms/CHANGELOG.md`](crates/winterbaume-kms/CHANGELOG.md).
- `winterbaume-lambda` v0.3.0: see [`crates/winterbaume-lambda/CHANGELOG.md`](crates/winterbaume-lambda/CHANGELOG.md).
- `winterbaume-s3` v0.2.1: see [`crates/winterbaume-s3/CHANGELOG.md`](crates/winterbaume-s3/CHANGELOG.md).
- `winterbaume-server` v0.2.2: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-terraform` v0.2.2: see [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).

## v0.4.0 - 2026-05-25

### Fixed

- **Lambda**: `Qualifier` is now honoured on `Invoke` and `InvokeWithResponseStream`; `RevisionId` is enforced on `UpdateFunctionCode`, `UpdateFunctionConfiguration`, `PublishVersion`, layer-version policy mutations, `AddPermission`, `RemovePermission`, and `GetPolicy`.
- **KMS**: Decryption operations now enforce the specified key ID against the key that actually performed the decryption.
- **CloudFront**: `If-Match` precondition is now enforced on mutating distribution and configuration APIs.
- **S3**: Conditional-request evaluation corrected; `HEAD` responses no longer include a message body.

### Internal

- Release harness: umbrella-crate changelog generation now scopes the `git log` path filter to `.` rather than `/`, fixing spurious commit inclusion in changelog drafts.

This file summarises notable winterbaume releases across the workspace and serves
as the changelog for the umbrella `winterbaume` crate. Detailed per-crate notes
live in each crate's own `CHANGELOG.md` under `crates/<crate>/`.

Each crate is versioned independently via `cargo-release`. Release tags use the
`<crate-name>-v<version>` convention (e.g. `winterbaume-s3-v0.2.0`); the umbrella
`winterbaume` crate uses `winterbaume-v<version>`.

## 2026-05-19 — `winterbaume` v0.3.0

Umbrella dependency refresh. Bumps the optional workspace-dep version
specifiers for every service crate that has been tagged since
`winterbaume-v0.2.0` (2026-05-13), so that downstream consumers enabling
feature flags pick up the new published service crates instead of the
versions captured at the previous umbrella release.

The umbrella is bumping `minor` (0.2.0 → 0.3.0) — pre-1.0 semver — because
several of the inherited service crates released breaking 0.x → 1.0 jumps:

- `winterbaume-account` v1.0.0
- `winterbaume-appconfig` v1.0.0
- `winterbaume-applicationsignals` v1.0.0
- `winterbaume-batch` v1.0.0
- `winterbaume-ec2` v1.0.0
- `winterbaume-ivs` v1.0.0
- `winterbaume-opensearch` v1.0.0
- `winterbaume-organizations` v1.0.0

Other refreshed service crates (non-breaking moves):

- `winterbaume-appconfigdata` v0.3.0
- `winterbaume-kinesisvideo` v0.3.0
- `winterbaume-kinesisvideoarchivedmedia` v0.3.0
- `winterbaume-sagemaker` v0.3.0
- `winterbaume-sesv2` v0.3.0

No new umbrella-level features or APIs; consult each per-crate
`CHANGELOG.md` for the substantive changes.

## 2026-05-18

- `winterbaume-account` v0.3.0: see [`crates/winterbaume-account/CHANGELOG.md`](crates/winterbaume-account/CHANGELOG.md).
- `winterbaume-appconfig` v0.3.0: see [`crates/winterbaume-appconfig/CHANGELOG.md`](crates/winterbaume-appconfig/CHANGELOG.md).
- `winterbaume-appconfigdata` v0.3.0: see [`crates/winterbaume-appconfigdata/CHANGELOG.md`](crates/winterbaume-appconfigdata/CHANGELOG.md).
- `winterbaume-applicationsignals` v0.3.0: see [`crates/winterbaume-applicationsignals/CHANGELOG.md`](crates/winterbaume-applicationsignals/CHANGELOG.md).
- `winterbaume-batch` v0.3.0: see [`crates/winterbaume-batch/CHANGELOG.md`](crates/winterbaume-batch/CHANGELOG.md).
- `winterbaume-ec2` v0.3.0: see [`crates/winterbaume-ec2/CHANGELOG.md`](crates/winterbaume-ec2/CHANGELOG.md).
- `winterbaume-ivs` v0.3.0: see [`crates/winterbaume-ivs/CHANGELOG.md`](crates/winterbaume-ivs/CHANGELOG.md).
- `winterbaume-kinesis` v0.2.1: see [`crates/winterbaume-kinesis/CHANGELOG.md`](crates/winterbaume-kinesis/CHANGELOG.md).
- `winterbaume-kinesisvideo` v0.3.0: see [`crates/winterbaume-kinesisvideo/CHANGELOG.md`](crates/winterbaume-kinesisvideo/CHANGELOG.md).
- `winterbaume-kinesisvideoarchivedmedia` v0.3.0: see [`crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md`](crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md).
- `winterbaume-opensearch` v0.3.0: see [`crates/winterbaume-opensearch/CHANGELOG.md`](crates/winterbaume-opensearch/CHANGELOG.md).
- `winterbaume-organizations` v0.3.0: see [`crates/winterbaume-organizations/CHANGELOG.md`](crates/winterbaume-organizations/CHANGELOG.md).
- `winterbaume-sagemaker` v0.3.0: see [`crates/winterbaume-sagemaker/CHANGELOG.md`](crates/winterbaume-sagemaker/CHANGELOG.md).
- `winterbaume-sagemakerruntime` v0.3.0: see [`crates/winterbaume-sagemakerruntime/CHANGELOG.md`](crates/winterbaume-sagemakerruntime/CHANGELOG.md).
- `winterbaume-ses` v0.3.0: see [`crates/winterbaume-ses/CHANGELOG.md`](crates/winterbaume-ses/CHANGELOG.md).
- `winterbaume-sesv2` v0.3.0: see [`crates/winterbaume-sesv2/CHANGELOG.md`](crates/winterbaume-sesv2/CHANGELOG.md).
- `winterbaume-sqs-redis` v0.2.1: see [`crates/winterbaume-sqs-redis/CHANGELOG.md`](crates/winterbaume-sqs-redis/CHANGELOG.md).
- `winterbaume-terraform` v0.2.1: see [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).

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
