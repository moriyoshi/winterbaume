# Changelog

## v0.3.1 - 2026-11-25

### Internal
- Dependency-only release for `winterbaume-core` v0.3.1; no change to this crate's own code.

## v0.3.0 - 2026-11-21

### Added
- `S3ControlService::with_s3_service(Arc<S3Service>)`. With it, `ListTagsForResource`, `TagResource` and `UntagResource` on a general purpose bucket ARN ( `arn:aws:s3:::<bucket>` ) read and write that bucket's tags in S3. The Terraform AWS provider manages `aws_s3_bucket` tags this way, so tagged buckets no longer show a `tags_all` diff on every plan. Other ARNs, and an unlinked service, behave as before (#53, #59).

### Fixed
- Requests to a custom endpoint reach this service. They are signed as `s3` and carry no `s3-control` host label, so they went to a stub in `winterbaume-s3` that answered every operation with an empty 200; the service now also claims the `/v20180820/` path prefix (#59).
- `UntagResource` removes every `tagKeys` value given; it kept only the last one, undecoded (#59).
- Errors use the wrapped `<ErrorResponse><Error>` envelope, so SDKs report the error code (#59).
- Percent-encoded UTF-8 in ARNs and query values is decoded as UTF-8; the crate's private decoder turned each byte into its own character. It now uses the shared decoders in `winterbaume-core` (#58, #60).

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. S3 Control service implementation for winterbaume.
