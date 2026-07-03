# Changelog

## v0.3.1 - 2026-07-03

### Internal

- Bumped workspace dependency on `winterbaume-appconfig` to pick up its latest release; no user-visible behaviour changes in this crate.

## v0.3.1 - 2026-07-03

### Internal

- Patch release to maintain compatibility with the updated `winterbaume-appconfig` dependency; no changes to this crate's own code.

## v0.3.0 - 2026-05-18

### Added
- `GetLatestConfiguration` now resolves through shared AppConfig state, enabling proper configuration token tracking across requests.

### Documentation
- Updated README to reflect current API coverage.

### Tests
- Added integration tests for `GetLatestConfiguration` behaviour.

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-09

### Added

- Initial public release. AWS AppConfig Data runtime service implementation for winterbaume.
