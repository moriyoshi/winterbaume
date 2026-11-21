# Changelog

## v0.2.1 - 2026-11-21

### Fixed
- Percent-encoded UTF-8 in request paths and query strings is decoded as UTF-8. The private decoder this crate carried turned each byte into its own character, so non-ASCII names and values were stored mangled ( `é` as `Ã©` ). The crate now uses the shared decoders in `winterbaume-core` (#58, #60).

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. AWS Glue DataBrew service implementation for winterbaume.
