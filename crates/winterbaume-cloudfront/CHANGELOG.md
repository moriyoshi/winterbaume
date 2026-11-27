# Changelog

## v1.1.2 - 2026-11-27

### Internal
- Dependency-only release for `winterbaume-core` v0.3.2; no change to this crate's own code.

## v1.1.1 - 2026-11-25

### Internal
- Dependency-only release for `winterbaume-core` v0.3.1; no change to this crate's own code.

## v1.1.0 - 2026-11-21

### Added
- `CloudFrontService::shared_state` and `get_function_in_stage`, with the `CloudFrontFunctionStage` types, so that Cloud Control can serve CloudFront resources from live state (#40).

## v1.0.1 - 2026-08-17

### Documentation
- Updated README coverage table to include a FakeCloud comparison column.

### Internal
- Fixed Clippy warnings arising from toolchain drift.

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
