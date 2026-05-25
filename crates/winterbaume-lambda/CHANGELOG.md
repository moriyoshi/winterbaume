# Changelog

## v0.3.0 - 2026-05-25

### Fixed

- `Invoke` and `InvokeWithResponseStream` now honour the `Qualifier` request parameter, routing invocations to the correct function version or alias.
- `AddPermission`, `RemovePermission`, and `GetPolicy` now honour `RevisionId`, returning `PreconditionFailedException` when the revision does not match.
- `UpdateFunctionCode`, `UpdateFunctionConfiguration`, `PublishVersion`, and the layer-version policy operations now honour `RevisionId` with the same precondition semantics.

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. Lambda service implementation for winterbaume.
