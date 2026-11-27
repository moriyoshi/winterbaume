# Changelog

## v1.0.3 - 2026-11-27

### Internal
- Dependency-only release for `winterbaume-core` v0.3.2; no change to this crate's own code.

## v1.0.2 - 2026-11-25

### Internal
- Dependency-only release for `winterbaume-core` v0.3.1; no change to this crate's own code.

## v1.0.1 - 2026-11-21

### Fixed
- Percent-encoded ARNs, names and tag keys are fully decoded. The crate decoded only `%3A` and `%2F` ( and in places `%20` ), so any other escape, including every non-ASCII character, was stored still encoded. It now uses the shared decoders in `winterbaume-core` (#58, #60).

## v0.3.0 - 2026-05-18

### Changed
- **Breaking:** Updated generated types and wire protocol; may require code changes when upgrading from v0.2.0

### Added
- Expanded OpenSearch API coverage with 410 new operations and types

### Internal
- Regenerated model and wire files as part of cross-crate codegen maintenance

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-09

### Added

- Initial public release. Amazon OpenSearch service implementation for winterbaume.
