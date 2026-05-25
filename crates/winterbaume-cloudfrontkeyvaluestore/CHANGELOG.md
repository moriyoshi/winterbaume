# Changelog

## v0.2.1 - 2026-05-25

### Fixed
- `If-Match` / `IfMatch` header is now enforced: mutating operations reject requests whose `If-Match` value does not match the current entity tag, returning the appropriate error response.

### Tests
- Added integration test coverage for `If-Match` enforcement on mutating operations.

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-09

### Added

- Initial public release. AWS CloudFront KeyValueStore service implementation for winterbaume.
