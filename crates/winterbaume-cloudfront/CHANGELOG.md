# Changelog

## v0.3.0 - 2026-05-25

### Fixed
- `If-Match` (ETag) precondition is now enforced on mutating CloudFront distribution operations; requests with a stale or missing `If-Match` value are rejected with the appropriate precondition-failure error.

### Tests
- Added integration tests covering `If-Match` enforcement across mutating distribution APIs.

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. CloudFront service implementation for winterbaume.
