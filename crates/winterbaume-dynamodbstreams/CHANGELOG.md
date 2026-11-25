# Changelog

## v0.3.0 - 2026-11-25

### Internal
- Dependency-only release for `winterbaume-dynamodb` v0.5.0 and `winterbaume-core` v0.3.1; no change to this crate's own code. Its public API takes an `Arc<dyn DynamoDbBackend>`, so the new `winterbaume-dynamodb` makes this a breaking release.

## v0.2.2 - 2026-11-21

### Internal
- Dependency-only release for `winterbaume-core` v0.3.0, `winterbaume-dynamodb` v0.4.0; no change to this crate's own code.

## v0.2.1 - 2026-08-17

### Fixed
- Applied follow-up handler corrections related to DynamoDB issue #19 (PR #22).

### Documentation
- Added a FakeCloud comparison column to the API coverage table in the README (PR #17).

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. Amazon DynamoDB Streams service implementation for winterbaume.
