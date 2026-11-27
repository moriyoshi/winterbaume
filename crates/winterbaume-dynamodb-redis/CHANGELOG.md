# Changelog

## v0.3.1 - 2026-11-27

### Internal
- Dependency-only release for `winterbaume-core` v0.3.2; no change to this crate's own code.

## v0.3.0 - 2026-11-25

### Fixed
- Conditional `PutItem`, `UpdateItem`, `DeleteItem` and `TransactWriteItems` are atomic: the condition is checked and the write applied in one Lua compare-and-swap, retried on conflict. The plain `UpdateItem` used to be an `HGET` followed by an `HSET` (#61, #73).

### Changed
- Built against `winterbaume-dynamodb` v0.5.0, whose `DynamoDbBackend` it implements, so this is a breaking release.

## v0.2.2 - 2026-11-21

### Fixed
- Index reads match the in-memory backend: `Scan` with `IndexName` returns only the index's items and pages with `LastEvaluatedKey`, plain `Scan` pages, `Query` on an index without a sort key resumes from `ExclusiveStartKey`, and an unknown index is a `ValidationException` instead of a 500 (#39, #40).
- Deletion protection and SSE settings are stored, survive snapshot and restore, and are enforced by `DeleteTable` (#42, #43, #55).
- `Tags` given to `CreateTable` are stored, and are removed with their table (#51, #59).

## v0.2.1 - 2026-08-17

### Fixed
- Corrected a bug where DynamoDB `UpdateItem` expressions were silently dropped rather than applied, causing update operations to return success without persisting the intended changes.
- Applied follow-up corrections to the DynamoDB state-handling logic introduced alongside the update-expression fix (issue #19).

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. Redis-backed DynamoDB storage backend for winterbaume.
