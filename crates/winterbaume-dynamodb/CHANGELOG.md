# Changelog

## v0.3.0 - 2026-08-17

### Fixed

- `UpdateExpression` attribute updates were silently discarded instead of being applied, causing items to appear unchanged after a successful `UpdateItem` call (#20).
- Corrected pre-update state handling in condition and update expression evaluation: expressions now consistently observe the item's state before the update is applied, resolving several edge-case failures that followed the original fix for issue #19 (#22).

### Documentation

- Added a FakeCloud comparison column to the README API coverage table (#17).

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. DynamoDB service implementation for winterbaume.
