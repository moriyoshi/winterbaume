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

- Operations: `GetCertificateSigningRequest`, `GetDefaultKeyReplicationRegions`, `GetMpaTeamAssociation`, `GetParametersForExport`, `GetParametersForImport`, `GetPublicKeyCertificate`, `GetResourcePolicy`
- Traits: `readonly` (3)
- Common required input members in this group: `KeyIdentifier`, `KeyMaterialType`

### Associate

- Operations: `AssociateMpaTeam`
- Common required input members in this group: -

### Delete

- Operations: `DeleteResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Disable

- Operations: `DisableDefaultKeyReplicationRegions`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateMpaTeam`
- Common required input members in this group: -

### Enable

- Operations: `EnableDefaultKeyReplicationRegions`
- Common required input members in this group: -

### Export

- Operations: `ExportKey`
- Common required input members in this group: -

### Import

- Operations: `ImportKey`
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateMpaTeam` | `-` | - | `Action`, `MpaTeamArn` | - | `AssociateMpaTeamOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Associates a Multi-Party Approval (MPA) team with a protected operation. For more information, see Multi-Party Approval in the Amazon Web Services Payment Cryptography User Guide. Cross-account use: This operation ca ... |
| `DeleteResourcePolicy` | `-` | `idempotent` | `ResourceArn` | - | `DeleteResourcePolicyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Removes the resource-based policy attached to an Amazon Web Services Payment Cryptography key. Cross-account use: This operation can't be used across different Amazon Web Services accounts. Related operations: PutRes ... |
| `DisableDefaultKeyReplicationRegions` | `-` | - | `ReplicationRegions` | - | `DisableDefaultKeyReplicationRegionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disables Multi-Region key replication settings for the specified Amazon Web Services Regions in your Amazon Web Services account, preventing new keys from being automatically replicated to those regions. After disabl ... |
| `DisassociateMpaTeam` | `-` | - | `Action` | - | `DisassociateMpaTeamOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Removes the association between a Multi-Party Approval (MPA) team and a protected operation. Cross-account use: This operation can't be used across different Amazon Web Services accounts. Related operations: Associat ... |
| `EnableDefaultKeyReplicationRegions` | `-` | - | `ReplicationRegions` | - | `EnableDefaultKeyReplicationRegionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables Multi-Region key replication settings for your Amazon Web Services account, causing new keys to be automatically replicated to the specified Amazon Web Services Regions when created. When Multi-Region key rep ... |
| `ExportKey` | `-` | - | `KeyMaterial`, `ExportKeyIdentifier` | - | `ExportKeyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Exports a key from Amazon Web Services Payment Cryptography. Amazon Web Services Payment Cryptography simplifies key exchange by replacing the existing paper-based approach with a modern electronic approach. With Exp ... |
| `GetCertificateSigningRequest` | `-` | - | `KeyIdentifier`, `SigningAlgorithm`, `CertificateSubject` | - | `GetCertificateSigningRequestOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Creates a certificate signing request (CSR) from a key pair. |
| `GetDefaultKeyReplicationRegions` | `-` | - | - | - | `GetDefaultKeyReplicationRegionsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the list of Amazon Web Services Regions where Multi-Region key replication is currently enabled for your Amazon Web Services account. This operation returns the current Multi-Region key replication configur ... |
| `GetMpaTeamAssociation` | `-` | `readonly` | `Action` | - | `GetMpaTeamAssociationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Returns the Multi-Party Approval (MPA) team association for a protected operation. Cross-account use: This operation can't be used across different Amazon Web Services accounts. Related operations: AssociateMpaTeam D ... |
| `GetParametersForExport` | `-` | - | `KeyMaterialType`, `SigningKeyAlgorithm` | - | `GetParametersForExportOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets the export token and the signing key certificate to initiate a TR-34 key export from Amazon Web Services Payment Cryptography. The signing key certificate signs the wrapped key under export within the TR-34 key ... |
| `GetParametersForImport` | `-` | - | `KeyMaterialType`, `WrappingKeyAlgorithm` | - | `GetParametersForImportOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets the import token and the wrapping key certificate in PEM format (base64 encoded) to initiate a TR-34 WrappedKeyBlock or a RSA WrappedKeyCryptogram import into Amazon Web Services Payment Cryptography. The wrappi ... |
| `GetPublicKeyCertificate` | `-` | `readonly` | `KeyIdentifier` | - | `GetPublicKeyCertificateOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets the public key certificate of the asymmetric key pair that exists within Amazon Web Services Payment Cryptography. Unlike the private key of an asymmetric key, which never leaves Amazon Web Services Payment Cryp ... |
| `GetResourcePolicy` | `-` | `readonly` | `ResourceArn` | - | `GetResourcePolicyOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Returns the resource-based policy attached to an Amazon Web Services Payment Cryptography key. Cross-account use: This operation can't be used across different Amazon Web Services accounts. Related operations: PutRes ... |
| `ImportKey` | `-` | - | `KeyMaterial` | - | `ImportKeyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Imports symmetric keys and public key certificates in PEM format (base64 encoded) into Amazon Web Services Payment Cryptography. Amazon Web Services Payment Cryptography simplifies key exchange by replacing the exist ... |
| `ListTagsForResource` | `-` | `readonly`, `paginated` | `ResourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Lists the tags for an Amazon Web Services resource. This is a paginated operation, which means that each response might contain only a subset of all the tags. When the response contains only a subset of tags, it incl ... |
| `PutResourcePolicy` | `-` | `idempotent` | `ResourceArn`, `Policy` | - | `PutResourcePolicyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `PublicPolicyException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Attaches or replaces a resource-based policy on an Amazon Web Services Payment Cryptography key. A resource-based policy can grant cross-account access to your key. If the policy would grant public access, the reques ... |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Adds or edits tags on an Amazon Web Services Payment Cryptography key. Tagging or untagging an Amazon Web Services Payment Cryptography key can allow or deny permission to the key. Each tag consists of a tag key and ... |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Deletes a tag from an Amazon Web Services Payment Cryptography key. Tagging or untagging an Amazon Web Services Payment Cryptography key can allow or deny permission to the key. Cross-account use: This operation supp ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. This exception is thrown when the caller lacks the necessary IAM permissions to perform the reques ... |
| `ConflictException` | `structure` | Message | This request can cause an inconsistent state for the resource. The requested operation conflicts with the current state of the resource. For example, attemp ... |
| `InternalServerException` | `structure` | Message | The request processing has failed because of an unknown error, exception, or failure. This indicates a server-side error within the Amazon Web Services Paym ... |
| `PublicPolicyException` | `structure` | Message | The resource-based policy would grant public access to the key. Modify the policy to restrict access to specific principals and resubmit the request. |
| `ResourceNotFoundException` | `structure` | ResourceId | The request was denied due to resource not found. The specified key, alias, or other resource does not exist in your account or region. Verify that the reso ... |
| `ServiceQuotaExceededException` | `structure` | Message | This request would cause a service quota to be exceeded. You have reached the maximum number of keys, aliases, or other resources allowed in your account. R ... |
| `ServiceUnavailableException` | `structure` | Message | The service cannot complete the request. The Amazon Web Services Payment Cryptography service is temporarily unavailable. This is typically a temporary cond ... |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. You have exceeded the rate limits for Amazon Web Services Payment Cryptography API calls. Implement expone ... |
| `ValidationException` | `structure` | Message | The request was denied due to an invalid request error. One or more parameters in your request are invalid. Check the parameter values, formats, and constra ... |
| `AssociateMpaTeamInput` | `structure` | Action, MpaTeamArn, RequesterComment | - |
| `AssociateMpaTeamOutput` | `structure` | MpaTeamAssociation | - |
| `DeleteResourcePolicyInput` | `structure` | ResourceArn | - |
| `DeleteResourcePolicyOutput` | `structure` | **empty (no members)** | - |
| `DisableDefaultKeyReplicationRegionsInput` | `structure` | ReplicationRegions | Input parameters for disabling default key replication regions for the account. |
| `DisableDefaultKeyReplicationRegionsOutput` | `structure` | EnabledReplicationRegions | Output from disabling default key replication regions for the account. |
| `DisassociateMpaTeamInput` | `structure` | Action, RequesterComment | - |
| `DisassociateMpaTeamOutput` | `structure` | MpaTeamAssociation | - |
| `EnableDefaultKeyReplicationRegionsInput` | `structure` | ReplicationRegions | Input parameters for enabling default key replication regions for the account. |
| `EnableDefaultKeyReplicationRegionsOutput` | `structure` | EnabledReplicationRegions | Output from enabling default key replication regions for the account. |
| `ExportKeyInput` | `structure` | KeyMaterial, ExportKeyIdentifier, ExportAttributes | - |
| `ExportKeyOutput` | `structure` | WrappedKey | - |
| `GetCertificateSigningRequestInput` | `structure` | KeyIdentifier, SigningAlgorithm, CertificateSubject | - |
| `GetCertificateSigningRequestOutput` | `structure` | CertificateSigningRequest | - |
| `GetDefaultKeyReplicationRegionsInput` | `structure` | **empty (no members)** | Input parameters for retrieving the account's default key replication regions. This operation requires no input parameters. |
| `GetDefaultKeyReplicationRegionsOutput` | `structure` | EnabledReplicationRegions | Output containing the account's current default key replication configuration. |
| `GetMpaTeamAssociationInput` | `structure` | Action | - |
| `GetMpaTeamAssociationOutput` | `structure` | MpaTeamAssociation | - |
| `GetParametersForExportInput` | `structure` | KeyMaterialType, SigningKeyAlgorithm, ReuseLastGeneratedToken | - |
| `GetParametersForExportOutput` | `structure` | SigningKeyCertificate, SigningKeyCertificateChain, SigningKeyAlgorithm, ExportToken, ParametersValidUntilTimestamp | - |
| `GetParametersForImportInput` | `structure` | KeyMaterialType, WrappingKeyAlgorithm, ReuseLastGeneratedToken | - |
| `GetParametersForImportOutput` | `structure` | WrappingKeyCertificate, WrappingKeyCertificateChain, WrappingKeyAlgorithm, ImportToken, ParametersValidUntilTimestamp | - |
| `GetPublicKeyCertificateInput` | `structure` | KeyIdentifier | - |
| `GetPublicKeyCertificateOutput` | `structure` | KeyCertificate, KeyCertificateChain | - |
| `GetResourcePolicyInput` | `structure` | ResourceArn | - |
| `GetResourcePolicyOutput` | `structure` | ResourceArn, Policy | - |
| `ImportKeyInput` | `structure` | KeyMaterial, KeyCheckValueAlgorithm, Enabled, Tags, ReplicationRegions, RequesterComment | - |
| `ImportKeyOutput` | `structure` | Key | - |
| `ListTagsForResourceInput` | `structure` | ResourceArn, NextToken, MaxResults | - |
| `ListTagsForResourceOutput` | `structure` | Tags, NextToken | - |
| `PutResourcePolicyInput` | `structure` | ResourceArn, Policy | - |
| `As2805KeyVariant` | `enum` | TERMINAL_MAJOR_KEY_VARIANT_00, PIN_ENCRYPTION_KEY_VARIANT_28, MESSAGE_AUTHENTICATION_KEY_VARIANT_24, DATA_ENCRYPTION_KEY_VARIANT_22 | - |
| `KeyDerivationFunction` | `enum` | NIST_SP800, ANSI_X963 | - |
| `KeyDerivationHashAlgorithm` | `enum` | SHA_256, SHA_384, SHA_512 | - |
| `SymmetricKeyAlgorithm` | `enum` | TDES_2KEY, TDES_3KEY, AES_128, AES_192, AES_256, HMAC_SHA256, HMAC_SHA384, HMAC_SHA512, HMAC_SHA224 | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
