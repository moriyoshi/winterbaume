# Changelog

## v0.2.1 - 2026-11-21

### Fixed
- Percent-encoded ARNs, names and tag keys are fully decoded. The crate decoded only `%3A` and `%2F` ( and in places `%20` ), so any other escape, including every non-ASCII character, was stored still encoded. It now uses the shared decoders in `winterbaume-core` (#58, #60).

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-09

### Added

- Initial public release. IAM Access Analyzer service implementation for winterbaume.
