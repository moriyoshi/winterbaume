# Changelog

## v0.2.1 - 2026-08-17

### Fixed
- Corrected a bug where DynamoDB `UpdateItem` expressions were silently dropped rather than applied, causing update operations to return success without persisting the intended changes.
- Applied follow-up corrections to the DynamoDB state-handling logic introduced alongside the update-expression fix (issue #19).

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. Redis-backed DynamoDB storage backend for winterbaume.
