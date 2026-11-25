# Changelog

## v0.3.0 - 2026-11-25

### Terraform
- `aws_sqs_queue`: `deduplication_scope` and `fifo_throughput_limit` (#64, #74).
- `aws_cloudwatch_metric_alarm`: `extended_statistic`, `treat_missing_data`, `datapoints_to_alarm`, `evaluate_low_sample_count_percentiles` and `threshold_metric_id` (#68, #76).
- `aws_cloudwatch_event_rule` and `aws_cloudwatch_event_target`: rules on a custom bus get bus-aware ARNs, and targets use the provider's `-`-joined state ID (#67, #75).
- `aws_lambda_function`: VPC configuration, architectures, layers and the other function settings are translated to and from the Lambda state's `FunctionSettings`, and `kms_key_arn`, `package_type` and `image_uri` are carried (#62, #73).

### Changed
- Built against `winterbaume-lambda` v3.0.0, `winterbaume-tfstate-resource-models` v0.3.0 and the new SQS, CloudWatch, EventBridge and DynamoDB releases. Its public API re-exports `winterbaume-tfstate-resource-models` as `generated` and takes those services' types, so this is a breaking release.

## v0.2.7 - 2026-11-21

### Terraform
- `aws_lambda_permission` carries `principal_org_id`, `event_source_token`, `function_url_auth_type` and `invoked_via_function_url` in both directions (#55).
- `aws_lambda_event_source_mapping` carries its settings ( `function_response_types`, `scaling_config`, batching, retry, `queues`, `topics` and the rest ) in both directions, and `arn`, `tags` and `tags_all` (#55, #59).
- `aws_lambda_code_signing_config` carries its tags (#59).
- `aws_cloudwatch_event_target` translates `retry_policy`, `dead_letter_config`, `role_arn`, `input_transformer` and the target parameter blocks to and from the API members, so injected targets are visible through `ListTargetsByRule` (#55).
- `aws_dynamodb_table` carries `deletion_protection_enabled` and `server_side_encryption` in both directions (#55).
- `aws_cloudfront_function` with `publish = true` injects the function's `LIVE` stage (#40).

## v0.2.6 - 2026-10-27

### Internal
- Bumped to track the `winterbaume-ssm` v0.3.0 release; no changes to this crate's own source.

## v0.2.5 - 2026-10-25

### Internal
- Bumped dependency versions to track the `winterbaume-keyspaces` v0.2.1 and `winterbaume-s3` v1.0.1 patch releases. No Terraform provider behaviour changed.

## v0.2.4 - 2026-08-17

### Internal
- Fixed clippy warnings in the CloudFront and Kafka converter modules to resolve toolchain drift.

## v0.2.3 - 2026-07-03

### Internal
- Bumped workspace dependencies (`winterbaume-apigateway`, `winterbaume-appconfig`, `winterbaume-lambda`, `winterbaume-s3`) to pick up their latest releases; no behaviour changes in this crate.

## v0.2.3 - 2026-07-03

### Internal
- Transitive patch release to pick up updated dependency versions for `winterbaume-apigateway`, `winterbaume-appconfig`, `winterbaume-lambda`, `winterbaume-opensearchserverless`, and `winterbaume-s3`; no changes to this crate's own code.

## v0.2.2 - 2026-05-25

### Fixed
- Lambda converter: correctly handles the `Qualifier` parameter for `Invoke` and `InvokeWithResponseStream`, and honours `RevisionId` on `AddPermission`, `RemovePermission`, `GetPolicy`, `UpdateFunctionCode`, `UpdateFunctionConfiguration`, `PublishVersion`, and layer-version policy operations.

## v0.2.1 - 2026-05-18

### Fixed

- Updated EC2 and Kinesis converters to handle newly added view struct fields, maintaining serialisation compatibility with current service implementations.

## v0.2.0 - 2026-05-14

### Changed

- Adopted spec-driven serde codegen for all 145 Terraform service serialisers, replacing per-service hand-written code.
- Extracted generated Terraform-state projection types into a separate `winterbaume-tfstate-resource-models` crate.
- Expanded Terraform resource converter coverage by ~700 new converters across the AWS service surface, including dedicated batches for medialive, networkfirewall, s3, s3tables, apigateway, glue, rds, redshift, sagemaker, route53 and ec2.
- Added IAM Terraform converter coverage: 11 top-level resources, 7 sub-resource modifiers (with two attachments brought under the spec), and 7 multi-target / exclusive converters, lifting IAM coverage to 32/34 (94%).

## v0.1.0 - 2026-05-10

### Added

- Initial public release. Terraform state injection and extraction for winterbaume.
