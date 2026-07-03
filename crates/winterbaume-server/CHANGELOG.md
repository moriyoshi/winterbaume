# Changelog

## v0.2.5 - 2026-07-03

### Internal
- Bumped workspace dependencies (`winterbaume-apigateway`, `winterbaume-appconfig`, `winterbaume-lambda`, `winterbaume-s3`) to their latest releases.

## v0.2.5 - 2026-07-03

### Internal

- Bumped dependency versions to track patch releases of `winterbaume-apigateway`, `winterbaume-appconfig`, `winterbaume-lambda`, `winterbaume-opensearchserverless`, and `winterbaume-s3`. No behavioural changes in this crate.

## v0.2.4 - 2026-06-04

### Internal

- Patch release to pick up fixes in `winterbaume-cloudcontrol` v0.2.4; no changes to this crate's own source.

## v0.2.3 - 2026-06-04

### Internal

- Bumped dependency on `winterbaume-cloudcontrol` to pick up patch fixes in that crate; no changes to `winterbaume-server` itself.

## v0.2.2 - 2026-05-25

### Internal

- Bumped dependency versions to pull in patch releases of `winterbaume-cloudfront`, `winterbaume-cloudfrontkeyvaluestore`, `winterbaume-kms`, `winterbaume-lambda`, `winterbaume-s3`, and `winterbaume-terraform`; no changes to server logic.

## v0.2.1 - 2026-05-19

### Changed

- Dependency refresh. Picks up the service-crate releases that landed
  between `winterbaume-server-v0.2.0` and HEAD: `winterbaume-account`,
  `winterbaume-appconfig`, `winterbaume-appconfigdata`,
  `winterbaume-applicationsignals`, `winterbaume-batch`, `winterbaume-ec2`,
  `winterbaume-ivs`, `winterbaume-kinesis`, `winterbaume-kinesisvideo`,
  `winterbaume-kinesisvideoarchivedmedia`, `winterbaume-opensearch`,
  `winterbaume-organizations`, `winterbaume-sagemaker`,
  `winterbaume-sagemakerruntime`, `winterbaume-ses`, `winterbaume-sesv2`,
  `winterbaume-sqs-redis`, `winterbaume-terraform`. No server-side code or
  behaviour changes; this release exists so that consumers of
  `winterbaume-server` from crates.io see a published manifest pinning the
  new dep versions.

## v0.2.0 - 2026-05-14

### Changed

- Picked up ~700 new Terraform resource converters wired through the server, covering EC2, S3, Route 53, IAM, API Gateway, Glue, RDS, Redshift, SageMaker, MediaLive, Network Firewall, S3 Tables and many other services.

## v0.1.0 - 2026-05-11

### Added

- Initial public release. Standalone HTTP server for winterbaume AWS mock services.
