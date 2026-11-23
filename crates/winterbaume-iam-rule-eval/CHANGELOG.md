# Changelog

## v0.3.0 - 2026-11-21

### Added
- `evaluate_with_options` with `EvalOptions`, for resource-policy callers. It evaluates conditions ( the String, Numeric, Date, Bool, Binary, IpAddress, Arn and Null operator families, with `IfExists`, `ForAllValues:` and `ForAnyValue:` ) and, with `PrincipalMode::WildcardOnly`, applies a statement only when its `Principal` is a wildcard. A missing condition key follows IAM's rules (#56).
- `evaluate` keeps its behaviour for the IAM simulation APIs: conditions and principals stay inert (#56).

### Changed
- Policy parsing accepts JSON booleans and numbers as condition values and a single-object `Statement`. This also applies to `evaluate` (#56).

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-09

### Added

- Initial public release. IAM policy evaluation engine for winterbaume.
