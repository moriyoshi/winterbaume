# Changelog

## v0.2.1 - 2026-05-25

### Fixed

- `Decrypt` now enforces that the key ID specified in the request matches the key that originally encrypted the ciphertext, returning an appropriate error when there is a mismatch.

### Tests

- Added integration and scenario tests covering the key ID enforcement behaviour for `Decrypt`.

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. KMS service implementation for winterbaume.
