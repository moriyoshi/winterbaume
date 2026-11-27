# Changelog

## v0.2.11 - 2026-11-27

### Added
- `--s3-enforce-session-policies`, `WB_S3_ENFORCE_SESSION_POLICIES` or `enforce_session_policies = true` under `[s3]` in the config file limits temporary credentials from STS to their session policy in S3. Catalogs that vend downscoped credentials, such as Lakekeeper, rely on a write outside the granted prefix failing. Off by default (#85, #87).

### Changed
- Picks up `winterbaume-s3` v2.0.0: `aws-chunked` uploads, as pyarrow and pyiceberg send them, are stored decoded (#84, #86).

## v0.2.10 - 2026-11-25

### Added
- `--relax RULE` ( repeatable, or a comma-separated list ), `WB_RELAX` or `relax = [...]` in the config file enables named relaxations of AWS request validation; none are enabled by default. An unknown name stops the server with the list of known ones, `--help` lists every rule, and each enabled rule is logged at startup. The first rule is `sts-assume-role-without-role-arn` (#77, #78).

### Changed
- Picks up the fixes in this release's service crates: atomic DynamoDB conditional writes (#61), Lambda function settings (#62), SQS FIFO parity (#64 – #66), EventBridge rule tags and bus scoping (#67), CloudWatch alarm settings and AWS JSON 1.0 (#68, #69), and EC2 security group descriptions.

## v0.2.9 - 2026-11-21

### Added
- `--s3-enforce-bucket-policies`, `WB_S3_ENFORCE_BUCKET_POLICIES` or `[s3] enforce_bucket_policies` turns on S3 bucket policy enforcement; off by default (#56).

### Changed
- Requests are passed to services with `x-forwarded-proto: http` and `x-forwarded-for` set when absent, so policy conditions such as `aws:SecureTransport` and `aws:SourceIp` see the real transport (#56).
- Cloud Control is linked to the CloudFront service (#40), and S3 Control to the S3 service (#59).
- Picks up the fixes in this release's service crates.

## v0.2.8 - 2026-10-27

### Internal
- Maintenance release; `main.rs` updated as part of CloudControl SSM parameter backing wiring ( see `winterbaume-cloudcontrol` v1.1.0 for the user-visible change ).

## v0.2.7 - 2026-10-25

### Internal

- Bumped dependencies on `winterbaume-keyspaces` and `winterbaume-s3` to pick up their latest patch releases.

## v0.2.6 - 2026-08-17

### Internal

- Maintenance release to pick up updated dependencies (`winterbaume-autoscaling`, `winterbaume-cloudfront`, `winterbaume-codedeploy`, `winterbaume-dynamodb`, `winterbaume-dynamodb-redis`, `winterbaume-dynamodbstreams`, `winterbaume-fis`, `winterbaume-iam`, `winterbaume-managedblockchain`, `winterbaume-servicediscovery`, `winterbaume-terraform`). No changes to `winterbaume-server` itself.

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
