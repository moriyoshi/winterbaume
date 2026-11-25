# Changelog

## v0.3.0 - 2026-11-25

### Added
- The `sts-assume-role-without-role-arn` relaxation ( `ASSUME_ROLE_WITHOUT_ROLE_ARN` ), enabled through `StsService::with_relaxations` or `winterbaume-server --relax`. `AssumeRole` without `RoleArn` then issues temporary credentials that keep the caller's own identity, as MinIO does, for clients such as Lakekeeper. `RoleSessionName` is still required, and the response has no `AssumedRoleUser`. Off by default: AWS answers `MissingParameter` (#77, #78).
- `RELAXATIONS`, `StsService::with_relaxations` and `StsService::relaxations` (#78).

### Changed
- `GetCallerIdentity` resolves the caller from the access key in the SigV4 credential scope. Credentials from `AssumeRole`, `AssumeRoleWithSAML` and `AssumeRoleWithWebIdentity` now answer the assumed-role ARN; every caller used to get the default `user/moto` identity (#78).
- Breaking: `StsState` and `StsStateView` gain `caller_sessions`. The view field defaults to empty and is omitted when empty, so existing snapshots still load (#78).

## v0.2.1 - 2026-11-21

### Fixed
- Percent-encoded UTF-8 in request paths, query strings and form bodies is decoded as UTF-8. The private decoder this crate carried turned each byte into its own character, so non-ASCII names and values were stored mangled ( `é` as `Ã©` ). A `%` not followed by two hexadecimal digits is now kept as written; it used to be dropped with the two bytes after it. The crate now uses the shared decoders in `winterbaume-core` (#58, #60).

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. STS service mock implementation for winterbaume.
