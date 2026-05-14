# Changelog

## v0.2.0 - 2026-05-14

### Changed

- Adopted spec-driven serde codegen for all 145 Terraform service serialisers, replacing per-service hand-written code.
- Extracted generated Terraform-state projection types into a separate `winterbaume-tfstate-resource-models` crate.
- Expanded Terraform resource converter coverage by ~700 new converters across the AWS service surface, including dedicated batches for medialive, networkfirewall, s3, s3tables, apigateway, glue, rds, redshift, sagemaker, route53 and ec2.
- Added IAM Terraform converter coverage: 11 top-level resources, 7 sub-resource modifiers (with two attachments brought under the spec), and 7 multi-target / exclusive converters, lifting IAM coverage to 32/34 (94%).

## v0.1.0 - 2026-05-10

### Added

- Initial public release. Terraform state injection and extraction for winterbaume.
