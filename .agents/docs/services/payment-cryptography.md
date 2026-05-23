# Payment Cryptography Control Plane

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Payment Cryptography Control Plane APIs manage encryption keys for use during payment-related cryptographic operations. You can create, import, export, share, manage, and delete keys. You can also manage Identity and Access Management (IAM) policies for keys. For more information, see Identity and access management in the Amazon Web Services Payment Cryptography User Guide. To use encryption keys for payment-related transaction processing and associated cryptographic operations, you use the Amazon Web Services Payment Cryptography Data Plane.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Payment Cryptography Control Plane by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Payment Cryptography Control Plane resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Payment Cryptography Control Plane workflows in the local mock. Key resources include `AliasResource`, `KeyResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Add` operation families, including `GetAlias`, `GetCertificateSigningRequest`, `GetDefaultKeyReplicationRegions`, `GetKey`, `ListAliases`, `ListKeys`.

## Service Identity and Protocol

- AWS model slug: `payment-cryptography`
- AWS SDK for Rust slug: `paymentcryptography`
- Model version: `2021-09-14`
- Model file: `vendor/api-models-aws/models/payment-cryptography/service/2021-09-14/payment-cryptography-2021-09-14.json`
- SDK ID: `Payment Cryptography`
- Endpoint prefix: `controlplane.payment-cryptography`
- ARN namespace: `payment-cryptography`
- CloudFormation name: `PaymentCryptography`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (7), `List` (3), `Create` (2), `Delete` (2), `Add` (1), `Disable` (1), `Enable` (1), `Export` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddKeyReplicationRegions`, `CreateAlias`, `CreateKey`, `DeleteAlias`, `DeleteKey`, `DisableDefaultKeyReplicationRegions`, `EnableDefaultKeyReplicationRegions`, `ImportKey`, `RemoveKeyReplicationRegions`, `RestoreKey`, `StartKeyUsage`, `StopKeyUsage`, `TagResource`, `UntagResource`, `UpdateAlias`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAlias`, `GetCertificateSigningRequest`, `GetDefaultKeyReplicationRegions`, `GetKey`, `GetParametersForExport`, `GetParametersForImport`, `GetPublicKeyCertificate`, `ListAliases`, `ListKeys`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ExportKey`, `GetParametersForExport`, `GetParametersForImport`, `ImportKey`, `StartKeyUsage`, `StopKeyUsage`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 26 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AliasResource` | `AliasName` | put: `CreateAlias`; read: `GetAlias`; update: `UpdateAlias`; delete: `DeleteAlias`; list: `ListAliases` | - | - |
| `KeyResource` | `KeyIdentifier` | create: `CreateKey`; read: `GetKey`; delete: `DeleteKey`; list: `ListKeys` | `AddKeyReplicationRegions`, `RemoveKeyReplicationRegions`, `RestoreKey`, `StartKeyUsage`, `StopKeyUsage` | - |
## Operation Groups

### Get

- Operations: `GetAlias`, `GetCertificateSigningRequest`, `GetDefaultKeyReplicationRegions`, `GetKey`, `GetParametersForExport`, `GetParametersForImport`, `GetPublicKeyCertificate`
- Traits: `readonly` (3)
- Common required input members in this group: `AliasName`, `CertificateSubject`, `KeyIdentifier`, `KeyMaterialType`, `SigningAlgorithm`, `SigningKeyAlgorithm`, `WrappingKeyAlgorithm`

### List

- Operations: `ListAliases`, `ListKeys`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (3)
- Common required input members in this group: `ResourceArn`

### Create

- Operations: `CreateAlias`, `CreateKey`
- Traits: `idempotent` (1)
- Common required input members in this group: `AliasName`, `Exportable`, `KeyAttributes`

### Delete

- Operations: `DeleteAlias`, `DeleteKey`
- Traits: `idempotent` (2)
- Common required input members in this group: `AliasName`, `KeyIdentifier`

### Add

- Operations: `AddKeyReplicationRegions`
- Common required input members in this group: `KeyIdentifier`, `ReplicationRegions`

### Disable

- Operations: `DisableDefaultKeyReplicationRegions`
- Common required input members in this group: `ReplicationRegions`

### Enable

- Operations: `EnableDefaultKeyReplicationRegions`
- Common required input members in this group: `ReplicationRegions`

### Export

- Operations: `ExportKey`
- Common required input members in this group: `ExportKeyIdentifier`, `KeyMaterial`

### Import

- Operations: `ImportKey`
- Common required input members in this group: `KeyMaterial`

### Remove

- Operations: `RemoveKeyReplicationRegions`
- Common required input members in this group: `KeyIdentifier`, `ReplicationRegions`

### Restore

- Operations: `RestoreKey`
- Common required input members in this group: `KeyIdentifier`

### Start

- Operations: `StartKeyUsage`
- Common required input members in this group: `KeyIdentifier`

### Stop

- Operations: `StopKeyUsage`
- Common required input members in this group: `KeyIdentifier`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateAlias`
- Common required input members in this group: `AliasName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddKeyReplicationRegions` | - | - | `KeyIdentifier`, `ReplicationRegions` | - | `AddKeyReplicationRegionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds replication Amazon Web Services Regions to an existing Amazon Web Services Payment Cryptography key, enabling the key to be used for cryptographic operations in additional Amazon Web Services Regions. Multi-Region key replication allow you to use the... |
| `CreateAlias` | - | `idempotent` | `AliasName` | - | `CreateAliasOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Creates an alias , or a friendly name, for an Amazon Web Services Payment Cryptography key. You can use an alias to identify a key in the console and when you call cryptographic operations such as EncryptData or DecryptData. |
| `CreateKey` | - | - | `Exportable`, `KeyAttributes` | - | `CreateKeyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Payment Cryptography key, a logical representation of a cryptographic key, that is unique in your account and Amazon Web Services Region. You use keys for cryptographic functions such as encryption and decryption. |
| `DeleteAlias` | - | `idempotent` | `AliasName` | - | `DeleteAliasOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Deletes the alias, but doesn't affect the underlying key. Each key can have multiple aliases. |
| `DeleteKey` | - | `idempotent` | `KeyIdentifier` | - | `DeleteKeyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Deletes the key material and metadata associated with Amazon Web Services Payment Cryptography key. Key deletion is irreversible. |
| `DisableDefaultKeyReplicationRegions` | - | - | `ReplicationRegions` | - | `DisableDefaultKeyReplicationRegionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disables Multi-Region key replication settings for the specified Amazon Web Services Regions in your Amazon Web Services account, preventing new keys from being automatically replicated to those regions. After disabling Multi-Region key replication for... |
| `EnableDefaultKeyReplicationRegions` | - | - | `ReplicationRegions` | - | `EnableDefaultKeyReplicationRegionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables Multi-Region key replication settings for your Amazon Web Services account, causing new keys to be automatically replicated to the specified Amazon Web Services Regions when created. When Multi-Region key replication are enabled, any new keys created... |
| `ExportKey` | - | - | `ExportKeyIdentifier`, `KeyMaterial` | - | `ExportKeyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Exports a key from Amazon Web Services Payment Cryptography. Amazon Web Services Payment Cryptography simplifies key exchange by replacing the existing paper-based approach with a modern electronic approach. |
| `GetAlias` | - | `readonly` | `AliasName` | - | `GetAliasOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets the Amazon Web Services Payment Cryptography key associated with the alias. Cross-account use: This operation can't be used across different Amazon Web Services accounts. |
| `GetCertificateSigningRequest` | - | - | `CertificateSubject`, `KeyIdentifier`, `SigningAlgorithm` | - | `GetCertificateSigningRequestOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Creates a certificate signing request (CSR) from a key pair. |
| `GetDefaultKeyReplicationRegions` | - | - | - | - | `GetDefaultKeyReplicationRegionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the list of Amazon Web Services Regions where Multi-Region key replication is currently enabled for your Amazon Web Services account. This operation returns the current Multi-Region key replication configuration. |
| `GetKey` | - | `readonly` | `KeyIdentifier` | - | `GetKeyOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets the key metadata for an Amazon Web Services Payment Cryptography key, including the immutable and mutable attributes specified when the key was created. Returns key metadata including attributes, state, and timestamps, but does not return the actual... |
| `GetParametersForExport` | - | - | `KeyMaterialType`, `SigningKeyAlgorithm` | - | `GetParametersForExportOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets the export token and the signing key certificate to initiate a TR-34 key export from Amazon Web Services Payment Cryptography. The signing key certificate signs the wrapped key under export within the TR-34 key payload. |
| `GetParametersForImport` | - | - | `KeyMaterialType`, `WrappingKeyAlgorithm` | - | `GetParametersForImportOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets the import token and the wrapping key certificate in PEM format (base64 encoded) to initiate a TR-34 WrappedKeyBlock or a RSA WrappedKeyCryptogram import into Amazon Web Services Payment Cryptography. The wrapping key certificate wraps the key under... |
| `GetPublicKeyCertificate` | - | `readonly` | `KeyIdentifier` | - | `GetPublicKeyCertificateOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets the public key certificate of the asymmetric key pair that exists within Amazon Web Services Payment Cryptography. Unlike the private key of an asymmetric key, which never leaves Amazon Web Services Payment Cryptography unencrypted, callers with... |
| `ImportKey` | - | - | `KeyMaterial` | - | `ImportKeyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Imports symmetric keys and public key certificates in PEM format (base64 encoded) into Amazon Web Services Payment Cryptography. Amazon Web Services Payment Cryptography simplifies key exchange by replacing the existing paper-based approach with a modern... |
| `ListAliases` | - | `readonly`, `paginated` | - | - | `ListAliasesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Lists the aliases for all keys in the caller's Amazon Web Services account and Amazon Web Services Region. You can filter the aliases by `keyARN`. |
| `ListKeys` | - | `readonly`, `paginated` | - | - | `ListKeysOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Lists the keys in the caller's Amazon Web Services account and Amazon Web Services Region. You can filter the list of keys. |
| `ListTagsForResource` | - | `readonly`, `paginated` | `ResourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Lists the tags for an Amazon Web Services resource. This is a paginated operation, which means that each response might contain only a subset of all the tags. |
| `RemoveKeyReplicationRegions` | - | - | `KeyIdentifier`, `ReplicationRegions` | - | `RemoveKeyReplicationRegionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes Replication Regions from an existing Amazon Web Services Payment Cryptography key, disabling the key's availability for cryptographic operations in the specified Amazon Web Services Regions. When you remove Replication Regions, the key material is... |
| `RestoreKey` | - | - | `KeyIdentifier` | - | `RestoreKeyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Cancels a scheduled key deletion during the waiting period. Use this operation to restore a `Key` that is scheduled for deletion. |
| `StartKeyUsage` | - | - | `KeyIdentifier` | - | `StartKeyUsageOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Enables an Amazon Web Services Payment Cryptography key, which makes it active for cryptographic operations within Amazon Web Services Payment Cryptography Cross-account use: This operation can't be used across different Amazon Web Services accounts. Related... |
| `StopKeyUsage` | - | - | `KeyIdentifier` | - | `StopKeyUsageOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Disables an Amazon Web Services Payment Cryptography key, which makes it inactive within Amazon Web Services Payment Cryptography. You can use this operation instead of DeleteKey to deactivate a key. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Adds or edits tags on an Amazon Web Services Payment Cryptography key. Tagging or untagging an Amazon Web Services Payment Cryptography key can allow or deny permission to the key. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Deletes a tag from an Amazon Web Services Payment Cryptography key. Tagging or untagging an Amazon Web Services Payment Cryptography key can allow or deny permission to the key. |
| `UpdateAlias` | - | - | `AliasName` | - | `UpdateAliasOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Associates an existing Amazon Web Services Payment Cryptography alias with a different key. Each alias is associated with only one Amazon Web Services Payment Cryptography key at a time, although a key can have multiple aliases. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | The request processing has failed because of an unknown error, exception, or failure. |
| `ResourceNotFoundException` | `structure` | `ResourceId` | The request was denied due to resource not found. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message` | The request was denied due to an invalid request error. |
| `ServiceUnavailableException` | `structure` | `Message` | The service cannot complete the request. |
| `ConflictException` | `structure` | `Message` | This request can cause an inconsistent state for the resource. |
| `ServiceQuotaExceededException` | `structure` | `Message` | This request would cause a service quota to be exceeded. |
| `AddKeyReplicationRegionsInput` | `structure` | `KeyIdentifier`, `ReplicationRegions` | Input parameters for adding replication regions to a specific key. |
| `AddKeyReplicationRegionsOutput` | `structure` | `Key` | Output from adding replication regions to a key. |
| `CreateAliasInput` | `structure` | `AliasName`, `KeyArn` | - |
| `CreateAliasOutput` | `structure` | `Alias` | - |
| `CreateKeyInput` | `structure` | `DeriveKeyUsage`, `Enabled`, `Exportable`, `KeyAttributes`, `KeyCheckValueAlgorithm`, `ReplicationRegions`, `Tags` | - |
| `CreateKeyOutput` | `structure` | `Key` | - |
| `DeleteAliasInput` | `structure` | `AliasName` | - |
| `DeleteAliasOutput` | `structure` | - | - |
| `DeleteKeyInput` | `structure` | `DeleteKeyInDays`, `KeyIdentifier` | - |
| `DeleteKeyOutput` | `structure` | `Key` | - |
| `DisableDefaultKeyReplicationRegionsInput` | `structure` | `ReplicationRegions` | Input parameters for disabling default key replication regions for the account. |
| `DisableDefaultKeyReplicationRegionsOutput` | `structure` | `EnabledReplicationRegions` | Output from disabling default key replication regions for the account. |
| `EnableDefaultKeyReplicationRegionsInput` | `structure` | `ReplicationRegions` | Input parameters for enabling default key replication regions for the account. |
| `EnableDefaultKeyReplicationRegionsOutput` | `structure` | `EnabledReplicationRegions` | Output from enabling default key replication regions for the account. |
| `ExportKeyInput` | `structure` | `ExportAttributes`, `ExportKeyIdentifier`, `KeyMaterial` | - |
| `ExportKeyOutput` | `structure` | `WrappedKey` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
