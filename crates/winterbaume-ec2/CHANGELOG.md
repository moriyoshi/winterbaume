# Changelog

## v0.3.0 - 2026-05-18

### Added
- Spot instance datafeed subscription management
- Support for seven additional EC2 operations

### Changed
- **Breaking:** Image views now include kernel ID, ramdisk ID, ENA support, SR-IOV network support, TPM support, boot mode, IMDS version, location, and AMI copy metadata

### Fixed
- State persistence now correctly exposes previously missing resource attributes

### Terraform
- Improved state conversion coverage for EC2 resources

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. EC2/VPC service implementation for winterbaume.
