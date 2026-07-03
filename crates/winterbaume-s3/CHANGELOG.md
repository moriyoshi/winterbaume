# Changelog

## v0.3.0 - 2026-07-03

### Fixed

- `UploadPart` request bodies are now treated as opaque bytes rather than UTF-8 strings, fixing uploads of binary objects that contain invalid UTF-8 sequences. (PR #13)

## v0.2.1 - 2026-05-25

### Fixed

- Corrected conditional-request handling and ensured HEAD responses return no body.

### Tests

- Added integration tests covering conditional-request and HEAD-body behaviour.

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. S3 service implementation for winterbaume.
