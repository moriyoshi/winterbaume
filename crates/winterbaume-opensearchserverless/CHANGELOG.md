# Changelog

## v0.2.1 - 2026-07-03

### Fixed

- Resolved a flaky race condition in policy version handling that could cause non-deterministic behaviour when multiple policy mutations arrived in quick succession ( PR #14 ).

### Internal

- README refreshed as part of a workspace-wide documentation sweep.

## v0.2.1 - 2026-07-03

### Fixed
- Policy version generation is now collision-proof, eliminating a race-prone flake where concurrent policy creation could produce duplicate version identifiers.

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. OpenSearch Serverless service implementation for winterbaume.
