# Changelog

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
