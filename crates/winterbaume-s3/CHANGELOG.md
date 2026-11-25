# Changelog

## v1.1.1 - 2026-11-25

### Internal
- Dependency-only release for `winterbaume-core` v0.3.1; no change to this crate's own code.

## v1.1.0 - 2026-11-21

### Added
- Opt-in enforcement of explicit `Deny` statements in bucket policies, through `S3Service::with_bucket_policy_enforcement(true)`. It is off by default, and the default path is unchanged (#49, #56).
  - An object or listing request that a `Deny` matches on action, resource and conditions gets HTTP 403 `AccessDenied`. `DeleteObjects` reports denied keys as per-key errors and deletes the rest; `CopyObject` checks the source bucket's policy for the read and the destination's for the write.
  - `Allow` statements and identity policies are not evaluated. Bucket configuration requests, the policy itself included, are never denied.
  - The emulator does not authenticate callers, so only statements with a wildcard `Principal` apply. Statements naming a principal, using `NotPrincipal`, or conditioned on caller-identity keys such as `aws:userid` are skipped.
  - `aws:SecureTransport` is read from `X-Forwarded-Proto` when present, then from the request scheme.
- `S3Service::bucket_tags`, `tag_bucket` and `untag_bucket`, for services that address a bucket by ARN; they look the bucket up across Regions (#59).

### Fixed
- `PutBucketLifecycleConfiguration` and `GetBucketLifecycleConfiguration` return `x-amz-transition-default-minimum-object-size`, reading back as `all_storage_classes_128K` when the request omits it. The Terraform provider waits for this value, so `aws_s3_bucket_lifecycle_configuration` never converged (#41, #55).
- `GetBucketOwnershipControls` no longer reports a rule after `DeleteBucketOwnershipControls`; it answers 404 `OwnershipControlsNotFoundError`. New buckets are created with `BucketOwnerEnforced` stored, as S3 does (#50, #59).
- `CreateBucket` applies `CreateBucketConfiguration.Tags`, `LocationConstraint`, `x-amz-object-ownership` and `x-amz-bucket-object-lock-enabled`. It ignored its request entirely before. ACL and grant headers are still not applied (#53, #59).
- `GetBucketLocation` for a bucket outside `us-east-1` returns a body the SDKs can parse; `LocationConstraint` was nested twice (#59).
- Object keys containing non-ASCII characters are stored and listed as written, in UTF-8 byte order. They were listed as mojibake, and continuation tokens built from them did not match (#58, #60).

### Changed
- A bucket with no stored ownership controls, for example one restored from an older snapshot, now answers `GetBucketOwnershipControls` with 404 instead of a synthesised default (#59).
- A `%` not followed by two hexadecimal digits in a key or query value is kept as written (#60).

## v1.0.1 - 2026-10-25

### Fixed
- `GetObject` now correctly honours byte-range requests: responses respect the `Range` header and return the appropriate `206 Partial Content` status with the requested byte slice.

### Documentation
- Added a fakecloud comparison column to the API coverage table in the README.

### Tests
- Added integration tests covering byte-range `GetObject` requests.

## v1.0.0 - 2026-07-03

### Fixed

- `UploadPart` request bodies are now treated as opaque bytes rather than UTF-8 strings, fixing uploads of binary objects that contain invalid UTF-8 sequences. (PR #13)

## v0.2.1 - 2026-05-25

### Fixed

- Corrected conditional-request handling and ensured HEAD responses return no body.

### Tests

- Added integration tests covering conditional-request and HEAD-body behaviour.

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. S3 service implementation for winterbaume.
