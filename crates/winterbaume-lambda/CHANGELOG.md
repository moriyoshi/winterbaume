# Changelog

## v3.0.1 - 2026-11-27

### Internal
- Dependency-only release for `winterbaume-core` v0.3.2; no change to this crate's own code.

## v3.0.0 - 2026-11-25

### Fixed
- `CreateFunction` and `UpdateFunctionConfiguration` store, and the function reads return, `VpcConfig`, `Architectures`, `EphemeralStorage`, `TracingConfig`, `LoggingConfig` and the other function settings they used to drop (#62, #73).

### Changed
- Breaking: function configuration is one API-shaped `FunctionSettings` ( `settings` ) on the stored function. It replaces eight `Option<serde_json::Value>` fields ( `vpc_config`, `dead_letter_config`, `ephemeral_storage`, `file_system_config`, `image_config`, `logging_config`, `snap_start`, `tracing_config` ) that only the Terraform converter wrote, in Terraform's shape (#73).

## v2.1.0 - 2026-11-21

### Fixed
- `AddPermission` keeps `FunctionUrlAuthType`, `InvokedViaFunctionUrl`, `SourceAccount`, `PrincipalOrgID` and `EventSourceToken` as one merged `Condition`, in the response and in `GetPolicy`. A function-URL-only grant used to read back as unconditional (#44, #55).
- Event source mappings keep `FunctionResponseTypes`, `ScalingConfig` and eleven other settings across create, get, update, list and delete; an update changes only the members it supplies (#45, #55).
- `ListEventSourceMappings` applies `FunctionName` ( name, ARN, partial or qualified ) and `EventSourceArn` (#46, #55).
- Event source mappings return `EventSourceMappingArn` and keep their creation `Tags`. `ListTags`, `TagResource` and `UntagResource` accept function, event source mapping, code signing config and capacity provider ARNs (#54, #59).
- Capacity provider operations are reachable under the API version prefix the SDK sends (#59).

### Internal
- Uses the shared percent-decoders in `winterbaume-core` in place of a private copy (#60).

## v2.0.0 - 2026-07-03

### Internal

- Maintenance release with no user-visible changes to this crate.

## v0.3.0 - 2026-05-25

### Fixed

- `Invoke` and `InvokeWithResponseStream` now honour the `Qualifier` request parameter, routing invocations to the correct function version or alias.
- `AddPermission`, `RemovePermission`, and `GetPolicy` now honour `RevisionId`, returning `PreconditionFailedException` when the revision does not match.
- `UpdateFunctionCode`, `UpdateFunctionConfiguration`, `PublishVersion`, and the layer-version policy operations now honour `RevisionId` with the same precondition semantics.

## v0.2.0 - 2026-05-14

### Internal

- Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data.

## v0.1.0 - 2026-05-10

### Added

- Initial public release. Lambda service implementation for winterbaume.
