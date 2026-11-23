# Changelog

## v0.2.2 - 2026-11-21

### Fixed
- `CreateRole` and `GetRole` return `Tags`, `Description` and `PermissionsBoundary`; `CreateUser` and `GetUser` return `Tags` and `PermissionsBoundary`. The `PermissionsBoundary` input of both create operations is stored. Terraform reads role tags from `GetRole`, so a tagged `aws_iam_role` showed a diff on every plan (#52, #59).
- `GetAccountAuthorizationDetails` returns tags and permissions boundaries for users and roles (#59).
- Non-ASCII values in requests, such as a role description or a tag, are decoded as UTF-8; they were stored mangled ( `café` as `cafÃ©` ). A `%` not followed by two hexadecimal digits is kept as written (#58, #60).

### Changed
- `PermissionsBoundaryType` is `PermissionsBoundaryPolicy`, the documented value, where `Policy` was emitted (#59).
- `ListInstanceProfiles` no longer returns profile tags, which the API reference documents as omitted (#59).

## v0.2.1 - 2026-08-17

### Documentation

- Added a fakecloud comparison column to the README coverage table.

### Internal

- Fixed clippy warnings introduced by toolchain drift (PR #21).

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. IAM service implementation for winterbaume.
