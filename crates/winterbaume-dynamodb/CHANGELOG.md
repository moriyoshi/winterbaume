# Changelog

## v0.5.0 - 2026-11-25

### Fixed
- Conditional `PutItem`, `UpdateItem`, `DeleteItem` and `TransactWriteItems` evaluate their condition inside the backend, in the same step as the write. The handler used to read, evaluate and write in separate calls, so concurrent conditional writers could all succeed (#61, #73).
- A transaction no longer leaves earlier actions applied when a later one fails (#73).

### Changed
- A conditional write to a missing table answers `ResourceNotFoundException`. It used to answer `ConditionalCheckFailedException` (#73).
- `DynamoDbBackend` gains `put_item_conditional`, `update_item_conditional`, `delete_item_conditional` and `transact_write_items_conditional`, with `ConditionalWrite`, `TransactWriteAction`, `TransactWriteOutcome` and `TransactCancellationReason`. The default bodies read and then write, which is not atomic, so a backend that can be written to concurrently should override all four (#73).

## v0.4.0 - 2026-11-21

### Added
- `DynamoDbBackend::scan_index`, `describe_table_settings` and `update_table_settings`, each with a default implementation so existing backends keep compiling (#40, #55).
- `types::aws_managed_kms_key_arn`, the ARN reported for the AWS managed key when SSE is enabled without naming one (#55).

### Fixed
- `Query` and `Scan` on a secondary index return only the projected attributes for `KEYS_ONLY` and `INCLUDE` projections; they returned whole items. `Select=ALL_ATTRIBUTES` on a GSI without an `ALL` projection is rejected, `Scan` honours `IndexName`, indexes are sparse, and `ConsistentRead=true` on a GSI is rejected (#39, #40).
- Numeric and binary sort keys are ordered by value and by byte, not by their stored text, in queries, key conditions and filters (#40).
- `DeletionProtectionEnabled` on `CreateTable` and `UpdateTable` is stored and returned, and `DeleteTable` on a protected table fails with `ValidationException` until protection is turned off (#42, #55).
- `SSESpecification` is reflected as `SSEDescription` ( `Status: ENABLED`, `SSEType: KMS` and the key ARN ) when SSE is enabled (#43, #55).
- `Tags` given to `CreateTable` are returned by `ListTagsOfResource`. Tags are also removed with their table, so a table recreated under the same name no longer inherits them (#51, #59).

## v0.3.0 - 2026-08-17

### Fixed

- `UpdateExpression` attribute updates were silently discarded instead of being applied, causing items to appear unchanged after a successful `UpdateItem` call (#20).
- Corrected pre-update state handling in condition and update expression evaluation: expressions now consistently observe the item's state before the update is applied, resolving several edge-case failures that followed the original fix for issue #19 (#22).

### Documentation

- Added a FakeCloud comparison column to the README API coverage table (#17).

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. DynamoDB service implementation for winterbaume.
