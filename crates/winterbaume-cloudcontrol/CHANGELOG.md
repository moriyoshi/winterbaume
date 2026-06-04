# Changelog

## v0.3.0 - 2026-06-04

### Fixed

- `GetResource` now returns properly shaped `Properties` for `AWS::DynamoDB::Table`, `AWS::ECS::Cluster`, and `AWS::KMS::Key` resource types. Previously, response bodies were passed through unmodified from the underlying service, which could omit or mis-name fields required by the CloudFormation resource-type contract. Each resource type now has a dedicated CFN schema shaper that remaps and filters the service response to match the expected shape.

### Tests

- Added integration and scenario tests covering per-resource-type `GetResource` property shaping for `AWS::DynamoDB::Table`, `AWS::ECS::Cluster`, and `AWS::KMS::Key`, asserting content-level correctness of the returned `Properties` object rather than merely checking HTTP status.

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. Cloud Control API service implementation for winterbaume.
