# Changelog

## v0.3.0 - 2026-11-25

### Added
- `SqsQueueTfModel`: `deduplication_scope` and `fifo_throughput_limit` (#74).
- `MetricAlarmTfModel`: `extended_statistic`, `treat_missing_data`, `datapoints_to_alarm`, `evaluate_low_sample_count_percentile` and `threshold_metric_id` (#76).
- `LambdaFunctionTfModel`: `kms_key_arn`, `package_type` and `image_uri` (#73).

### Changed
- Breaking: these are new public fields on structs without `#[non_exhaustive]`, so struct literals must set them.

## v0.2.1 - 2026-11-21

### Terraform
- Regenerated models: `deletion_protection_enabled` on `aws_dynamodb_table`, `role_arn` on `aws_cloudwatch_event_target`, `event_source_token` and `function_url_auth_type` on `aws_lambda_permission`, and `arn`, `kms_key_arn` and `starting_position_timestamp` on `aws_lambda_event_source_mapping` (#55, #59).

## v0.2.0 - 2026-05-13

### Added

- Initial public release. Auto-generated Terraform resource projection models for winterbaume converters.
- Hosts the spec-driven serde codegen output for Terraform-state projection types, extracted from `winterbaume-terraform` so downstream crates can depend on the generated model code without rebuilding the converter pipeline.
