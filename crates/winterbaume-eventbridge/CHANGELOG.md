# Changelog

## v0.2.1 - 2026-11-21

### Fixed
- `PutTargets` keeps the whole target, so `ListTargetsByRule` returns `RetryPolicy`, `DeadLetterConfig`, `RoleArn`, `InputTransformer` and every service-specific parameter block as sent. Only `Id`, `Arn`, `Input` and `InputPath` were kept (#47, #55).

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. EventBridge service implementation for winterbaume.
