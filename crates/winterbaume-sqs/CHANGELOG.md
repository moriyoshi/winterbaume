# Changelog

## v0.3.1 - 2026-11-27

### Internal
- Dependency-only release for `winterbaume-core` v0.3.2; no change to this crate's own code.

## v0.3.0 - 2026-11-25

### Fixed
- `DeduplicationScope` and `FifoThroughputLimit` are stored and validated: FIFO queues only, known values only, and `perMessageGroupId` only with `messageGroup` scope. `messageGroup` scope deduplicates per message group (#64, #74).
- A FIFO message group with a message in flight delivers nothing more until that message is deleted, made visible again or its visibility timeout expires (#65, #74).
- Messages carry `MessageGroupId`, `MessageDeduplicationId` and a per-queue `SequenceNumber`. `ReceiveMessage` returns them, and `SendMessage` and `SendMessageBatch` return `SequenceNumber` (#66, #74).
- Content-based deduplication hashes the body with SHA-256, not MD5 (#74).
- `MaximumMessageSize` defaults to 1 MiB (#74).

### Changed
- `ReceiveMessage` returns only the system attributes requested through `MessageSystemAttributeNames` or `AttributeNames`, and none by default, as AWS does (#74).
- Breaking: public state and backend types gained fields (#74).

### Tests
- Seven moto FIFO tests are ported with every assertion, and the two assertions dropped from `test_moto_send_message_with_message_group_id` are restored (#74).

## v0.2.1 - 2026-11-21

### Internal
- Dependency-only release for `winterbaume-core` v0.3.0; no change to this crate's own code.

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. SQS service implementation for winterbaume.
