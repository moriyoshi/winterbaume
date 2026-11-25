# Changelog

## v0.3.0 - 2026-11-25

### Fixed
- `PutRule` applies `Tags` when it creates a rule and ignores them on update, as AWS documents, and stores and returns `RoleArn` (#67, #75).
- Rules are scoped by event bus. They are keyed by ( bus, name ), a rule on a custom bus has the ARN `rule/<bus>/<name>`, and every rule and target operation honours `EventBusName`, as a name or an ARN (#75).
- Unknown event buses, and schedule expressions on a custom bus, are rejected. Tag operations on a missing rule answer `ResourceNotFoundException`, and `DeleteRule` removes the rule's tags (#75).
- `CreateEventBus` stores `Tags` and `Description` (#75).
- `ListRules`, `ListRuleNamesByTarget` and `ListTargetsByRule` paginate (#75).

### Changed
- Rule tags live in one store, so tags injected from Terraform state are visible to `ListTagsForResource` (#75).
- Breaking: the public state API and types changed. Snapshots from earlier versions still restore, with custom-bus rule ARNs rewritten (#75).

## v0.2.1 - 2026-11-21

### Fixed
- `PutTargets` keeps the whole target, so `ListTargetsByRule` returns `RetryPolicy`, `DeadLetterConfig`, `RoleArn`, `InputTransformer` and every service-specific parameter block as sent. Only `Id`, `Arn`, `Input` and `InputPath` were kept (#47, #55).

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. EventBridge service implementation for winterbaume.
