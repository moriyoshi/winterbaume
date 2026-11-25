# Changelog

## v0.3.2 - 2026-11-25

### Internal
- Dependency-only release for `winterbaume-core` v0.3.1; no change to this crate's own code.

## v0.3.1 - 2026-11-21

### Fixed
- Percent-encoded UTF-8 in request paths, query strings and form bodies is decoded as UTF-8. The private decoder this crate carried turned each byte into its own character, so non-ASCII names and values were stored mangled ( `é` as `Ã©` ). A `%` not followed by two hexadecimal digits is now kept as written; it used to be dropped with the two bytes after it. The crate now uses the shared decoders in `winterbaume-core` (#58, #60).

## v0.3.0 - 2026-05-18

### Added
- Parent-state validation for SageMaker Runtime endpoint resources.

### Tests
- Integration test coverage for parent-state validation.

### Internal
- Maintenance sweep: codegen regeneration and documentation updates.

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. SageMaker Runtime service implementation for winterbaume.
