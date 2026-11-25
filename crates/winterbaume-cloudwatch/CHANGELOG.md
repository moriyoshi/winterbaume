# Changelog

## v0.3.0 - 2026-11-25

### Added
- AWS JSON 1.0: requests with `X-Amz-Target: GraniteServiceVersion20100801.<Operation>`, as the AWS CLI v2 and boto3 send them, are dispatched and answered in JSON, beside rpcv2Cbor and awsQuery (#69, #76).

### Fixed
- `PutMetricAlarm` stores and returns `TreatMissingData`, `DatapointsToAlarm`, `ExtendedStatistic`, `Metrics`, `ThresholdMetricId` and `EvaluateLowSampleCountPercentile`. Alarms also return their state and configuration timestamps and `StateReasonData` (#68, #76).
- New metric and composite alarms start in `INSUFFICIENT_DATA`. An update keeps the alarm's state and ignores `Tags` (#76).
- awsQuery integer members were parsed as floats and then dropped, so `Period=300` fell back to 60. Query values stay strings and are read as the member's type (#76).
- JSON and CBOR error responses carry `x-amzn-query-error` (#76).

### Changed
- Breaking: `MetricAlarmView` gains the new settings, and some of its field types changed. Older snapshots still load (#76).

## v0.2.1 - 2026-11-21

### Internal
- Dependency-only release for `winterbaume-core` v0.3.0; no change to this crate's own code.

## v0.2.0 - 2026-05-13

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-09

### Added

- Initial public release. CloudWatch service implementation for winterbaume.
