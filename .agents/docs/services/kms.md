# AWS Key Management Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Key Management Service Key Management Service (KMS) is an encryption and key management web service. This guide describes the KMS operations that you can call programmatically. For general information about KMS, see the Key Management Service Developer Guide . KMS has replaced the term customer master key (CMK) with Key Management Service key and KMS key . The concept has not changed.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-kms/tests/scenario_test.rs`: exercise envelope encryption with data-key generation, encrypt/decrypt behaviour, and key usage checks.
- Backported from `scenario_test.rs`: rotate keys and inspect audit-oriented key metadata.
- Backported from `scenario_test.rs`: manage grant delegation for a different principal, and create/sign/verify with an HMAC key.
- Scenario insight from EC2: exercise account or service defaults for AWS Key Management Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS Key Management Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent key lifecycle, aliases, grants, policies, cryptographic operations, imported key material, rotation, multi-Region keys, signing/MAC workflows, and deletion scheduling.

## Service Identity and Protocol

- AWS model slug: `kms`
- AWS SDK for Rust slug: `kms`
- Model version: `2014-11-01`
- Model file: `vendor/api-models-aws/models/kms/service/2014-11-01/kms-2014-11-01.json`
- SDK ID: `KMS`
- Endpoint prefix: `kms`
- ARN namespace: `kms`
- CloudFormation name: `KMS`
- CloudTrail event source: `kms.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Generate` (6), `Create` (4), `Get` (4), `Update` (4), `Delete` (3), `Describe` (2), `Disable` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelKeyDeletion`, `CreateAlias`, `CreateCustomKeyStore`, `CreateGrant`, `CreateKey`, `DeleteAlias`, `DeleteCustomKeyStore`, `DeleteImportedKeyMaterial`, `DisableKey`, `DisableKeyRotation`, `EnableKey`, `EnableKeyRotation`, `ImportKeyMaterial`, `PutKeyPolicy`, `RevokeGrant`, `TagResource`, `UntagResource`, `UpdateAlias`, `UpdateCustomKeyStore`, `UpdateKeyDescription`, ... (+1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCustomKeyStores`, `DescribeKey`, `GetKeyPolicy`, `GetKeyRotationStatus`, `GetParametersForImport`, `GetPublicKey`, `ListAliases`, `ListGrants`, `ListKeyPolicies`, `ListKeyRotations`, `ListKeys`, `ListResourceTags`, `ListRetirableGrants`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelKeyDeletion`, `DeleteImportedKeyMaterial`, `GetParametersForImport`, `ImportKeyMaterial`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 53 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html
- https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html
- https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html

Research outcomes:
- Customer managed keys are created and controlled by the customer account, including policies, grants, tags, aliases, enable/disable state, rotation, and scheduled deletion.
- AWS managed keys are created and managed by AWS services. They can be viewed and audited, but their policies, rotation, and deletion cannot be changed by customers.
- AWS owned keys are not in the customer account and are managed entirely by AWS services.
- KMS key identifiers include key ARN, key ID, alias ARN, and alias name. Alias names always begin with `alias/`; the `alias/aws/` prefix is reserved for AWS managed keys.
- An alias can point to different KMS keys over time, so APIs that accept aliases must resolve the alias at operation time.
- Key rotation preserves older backing key material for decrypt/verify of existing ciphertext, while new cryptographic operations use active key material.
- KMS cryptographic APIs return plaintext or ciphertext over TLS and do not store returned data keys or decrypted payloads.

Parity implications:
- Separate key metadata, key state, aliases, grants, and backing-key versions in state.
- Resolve KeyId inputs according to accepted identifier forms per operation.
- Preserve the distinction between customer managed, AWS managed, and AWS owned keys for mutability checks.
- Encryption/decryption parity should include key state, encryption context, algorithm/key-spec constraints, and alias resolution time.

## Operation Groups

### List

- Operations: `ListAliases`, `ListGrants`, `ListKeyPolicies`, `ListKeyRotations`, `ListKeys`, `ListResourceTags`, `ListRetirableGrants`
- Traits: `paginated` (7)
- Common required input members in this group: `KeyId`, `RetiringPrincipal`

### Generate

- Operations: `GenerateDataKey`, `GenerateDataKeyPair`, `GenerateDataKeyPairWithoutPlaintext`, `GenerateDataKeyWithoutPlaintext`, `GenerateMac`, `GenerateRandom`
- Common required input members in this group: `KeyId`, `KeyPairSpec`, `MacAlgorithm`, `Message`

### Create

- Operations: `CreateAlias`, `CreateCustomKeyStore`, `CreateGrant`, `CreateKey`
- Common required input members in this group: `AliasName`, `CustomKeyStoreName`, `GranteePrincipal`, `KeyId`, `Operations`, `TargetKeyId`

### Get

- Operations: `GetKeyPolicy`, `GetKeyRotationStatus`, `GetParametersForImport`, `GetPublicKey`
- Common required input members in this group: `KeyId`, `WrappingAlgorithm`, `WrappingKeySpec`

### Update

- Operations: `UpdateAlias`, `UpdateCustomKeyStore`, `UpdateKeyDescription`, `UpdatePrimaryRegion`
- Common required input members in this group: `AliasName`, `CustomKeyStoreId`, `Description`, `KeyId`, `PrimaryRegion`, `TargetKeyId`

### Delete

- Operations: `DeleteAlias`, `DeleteCustomKeyStore`, `DeleteImportedKeyMaterial`
- Common required input members in this group: `AliasName`, `CustomKeyStoreId`, `KeyId`

### Describe

- Operations: `DescribeCustomKeyStores`, `DescribeKey`
- Traits: `paginated` (1)
- Common required input members in this group: `KeyId`

### Disable

- Operations: `DisableKey`, `DisableKeyRotation`
- Common required input members in this group: `KeyId`

### Enable

- Operations: `EnableKey`, `EnableKeyRotation`
- Common required input members in this group: `KeyId`

### Verify

- Operations: `Verify`, `VerifyMac`
- Common required input members in this group: `KeyId`, `Mac`, `MacAlgorithm`, `Message`, `Signature`, `SigningAlgorithm`

### Cancel

- Operations: `CancelKeyDeletion`
- Common required input members in this group: `KeyId`

### Connect

- Operations: `ConnectCustomKeyStore`
- Common required input members in this group: `CustomKeyStoreId`

### Decrypt

- Operations: `Decrypt`

### Derive

- Operations: `DeriveSharedSecret`
- Common required input members in this group: `KeyAgreementAlgorithm`, `KeyId`, `PublicKey`

### Disconnect

- Operations: `DisconnectCustomKeyStore`
- Common required input members in this group: `CustomKeyStoreId`

### Encrypt

- Operations: `Encrypt`
- Common required input members in this group: `KeyId`, `Plaintext`

### Import

- Operations: `ImportKeyMaterial`
- Common required input members in this group: `EncryptedKeyMaterial`, `ImportToken`, `KeyId`

### Put

- Operations: `PutKeyPolicy`
- Common required input members in this group: `KeyId`, `Policy`

### Re

- Operations: `ReEncrypt`
- Common required input members in this group: `DestinationKeyId`

### Replicate

- Operations: `ReplicateKey`
- Common required input members in this group: `KeyId`, `ReplicaRegion`

### Retire

- Operations: `RetireGrant`

### Revoke

- Operations: `RevokeGrant`
- Common required input members in this group: `GrantId`, `KeyId`

### Rotate

- Operations: `RotateKeyOnDemand`
- Common required input members in this group: `KeyId`

### Schedule

- Operations: `ScheduleKeyDeletion`
- Common required input members in this group: `KeyId`

### Sign

- Operations: `Sign`
- Common required input members in this group: `KeyId`, `Message`, `SigningAlgorithm`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `KeyId`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `KeyId`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelKeyDeletion` | - | - | `KeyId` | - | `CancelKeyDeletionResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Cancels the deletion of a KMS key. When this operation succeeds, the key state of the KMS key is `Disabled`. |
| `ConnectCustomKeyStore` | - | - | `CustomKeyStoreId` | - | `ConnectCustomKeyStoreResponse` | `CloudHsmClusterInvalidConfigurationException`, `CloudHsmClusterNotActiveException`, `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `KMSInternalException` | Connects or reconnects a custom key store to its backing key store. For an CloudHSM key store, `ConnectCustomKeyStore` connects the key store to its associated CloudHSM cluster. |
| `CreateAlias` | - | - | `AliasName`, `TargetKeyId` | - | `Unit` | `AlreadyExistsException`, `DependencyTimeoutException`, `InvalidAliasNameException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException` | Creates a friendly name for a KMS key. Adding, deleting, or updating an alias can allow or deny permission to the KMS key. |
| `CreateCustomKeyStore` | - | - | `CustomKeyStoreName` | - | `CreateCustomKeyStoreResponse` | `CloudHsmClusterInUseException`, `CloudHsmClusterInvalidConfigurationException`, `CloudHsmClusterNotActiveException`, `CloudHsmClusterNotFoundException`, `CustomKeyStoreNameInUseException`, `IncorrectTrustAnchorException`, `KMSInternalException`, `LimitExceededException`, ... (+9) | Creates a custom key store backed by a key store that you own and manage. When you use a KMS key in a custom key store for a cryptographic operation, the cryptographic operation is actually performed in your key store using your keys. |
| `CreateGrant` | - | - | `GranteePrincipal`, `KeyId`, `Operations` | - | `CreateGrantResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidArnException`, `InvalidGrantTokenException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, ... (+1) | Adds a grant to a KMS key. A grant is a policy instrument that allows Amazon Web Services principals to use KMS keys in cryptographic operations. |
| `CreateKey` | - | - | - | - | `CreateKeyResponse` | `CloudHsmClusterInvalidConfigurationException`, `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `LimitExceededException`, `MalformedPolicyDocumentException`, ... (+5) | Creates a unique customer managed KMS key in your Amazon Web Services account and Region. You can use a KMS key in cryptographic operations, such as encryption and signing. |
| `Decrypt` | - | - | - | - | `DecryptResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `IncorrectKeyException`, `InvalidCiphertextException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, ... (+3) | Decrypts ciphertext that was encrypted by a KMS key using any of the following operations: Encrypt GenerateDataKey GenerateDataKeyPair GenerateDataKeyWithoutPlaintext GenerateDataKeyPairWithoutPlaintext You can use this operation to decrypt ciphertext that... |
| `DeleteAlias` | - | - | `AliasName` | - | `Unit` | `DependencyTimeoutException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Deletes the specified alias. Adding, deleting, or updating an alias can allow or deny permission to the KMS key. |
| `DeleteCustomKeyStore` | - | - | `CustomKeyStoreId` | - | `DeleteCustomKeyStoreResponse` | `CustomKeyStoreHasCMKsException`, `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `KMSInternalException` | Deletes a custom key store. This operation does not affect any backing elements of the custom key store. |
| `DeleteImportedKeyMaterial` | - | - | `KeyId` | - | `DeleteImportedKeyMaterialResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Deletes key material that was previously imported. This operation makes the specified KMS key temporarily unusable. |
| `DeriveSharedSecret` | - | - | `KeyAgreementAlgorithm`, `KeyId`, `PublicKey` | - | `DeriveSharedSecretResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidStateException`, `KeyUnavailableException`, ... (+1) | Derives a shared secret using a key agreement algorithm. You must use an asymmetric NIST-standard elliptic curve (ECC) or SM2 (China Regions only) KMS key pair with a `KeyUsage` value of `KEY_AGREEMENT` to call DeriveSharedSecret. |
| `DescribeCustomKeyStores` | - | `paginated` | - | - | `DescribeCustomKeyStoresResponse` | `CustomKeyStoreNotFoundException`, `InvalidMarkerException`, `KMSInternalException` | Gets information about custom key stores in the account and Region. This operation is part of the custom key stores feature in KMS, which combines the convenience and extensive integration of KMS with the isolation and control of a key store that you own and... |
| `DescribeKey` | - | - | `KeyId` | - | `DescribeKeyResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `NotFoundException` | Provides detailed information about a KMS key. You can run `DescribeKey` on a customer managed key or an Amazon Web Services managed key. |
| `DisableKey` | - | - | `KeyId` | - | `Unit` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Sets the state of a KMS key to disabled. This change temporarily prevents use of the KMS key for cryptographic operations. |
| `DisableKeyRotation` | - | - | `KeyId` | - | `Unit` | `DependencyTimeoutException`, `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Disables automatic rotation of the key material of the specified symmetric encryption KMS key. Automatic key rotation is supported only on symmetric encryption KMS keys. |
| `DisconnectCustomKeyStore` | - | - | `CustomKeyStoreId` | - | `DisconnectCustomKeyStoreResponse` | `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `KMSInternalException` | Disconnects the custom key store from its backing key store. This operation disconnects an CloudHSM key store from its associated CloudHSM cluster or disconnects an external key store from the external key store proxy that communicates with your external key... |
| `EnableKey` | - | - | `KeyId` | - | `Unit` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException` | Sets the key state of a KMS key to enabled. This allows you to use the KMS key for cryptographic operations. |
| `EnableKeyRotation` | - | - | `KeyId` | - | `Unit` | `DependencyTimeoutException`, `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Enables automatic rotation of the key material of the specified symmetric encryption KMS key. By default, when you enable automatic rotation of a customer managed KMS key, KMS rotates the key material of the KMS key one year (approximately 365 days) from the... |
| `Encrypt` | - | - | `KeyId`, `Plaintext` | - | `EncryptResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidStateException`, `KeyUnavailableException`, ... (+1) | Encrypts plaintext of up to 4,096 bytes using a KMS key. You can use a symmetric or asymmetric KMS key with a `KeyUsage` of `ENCRYPT_DECRYPT`. |
| `GenerateDataKey` | - | - | `KeyId` | - | `GenerateDataKeyResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidStateException`, `KeyUnavailableException`, ... (+1) | Returns a unique symmetric data key for use outside of KMS. This operation returns a plaintext copy of the data key and a copy that is encrypted under a symmetric encryption KMS key that you specify. |
| `GenerateDataKeyPair` | - | - | `KeyId`, `KeyPairSpec` | - | `GenerateDataKeyPairResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidStateException`, `KeyUnavailableException`, ... (+2) | Returns a unique asymmetric data key pair for use outside of KMS. This operation returns a plaintext public key, a plaintext private key, and a copy of the private key that is encrypted under the symmetric encryption KMS key you specify. |
| `GenerateDataKeyPairWithoutPlaintext` | - | - | `KeyId`, `KeyPairSpec` | - | `GenerateDataKeyPairWithoutPlaintextResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidStateException`, `KeyUnavailableException`, ... (+2) | Returns a unique asymmetric data key pair for use outside of KMS. This operation returns a plaintext public key and a copy of the private key that is encrypted under the symmetric encryption KMS key you specify. |
| `GenerateDataKeyWithoutPlaintext` | - | - | `KeyId` | - | `GenerateDataKeyWithoutPlaintextResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidStateException`, `KeyUnavailableException`, ... (+1) | Returns a unique symmetric data key for use outside of KMS. This operation returns a data key that is encrypted under a symmetric encryption KMS key that you specify. |
| `GenerateMac` | - | - | `KeyId`, `MacAlgorithm`, `Message` | - | `GenerateMacResponse` | `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidStateException`, `KeyUnavailableException`, `NotFoundException` | Generates a hash-based message authentication code (HMAC) for a message using an HMAC KMS key and a MAC algorithm that the key supports. HMAC KMS keys and the HMAC algorithms that KMS uses conform to industry standards defined in RFC 2104. |
| `GenerateRandom` | - | - | - | - | `GenerateRandomResponse` | `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `DependencyTimeoutException`, `KMSInternalException`, `UnsupportedOperationException` | Returns a random byte string that is cryptographically secure. You must use the `NumberOfBytes` parameter to specify the length of the random byte string. |
| `GetKeyPolicy` | - | - | `KeyId` | - | `GetKeyPolicyResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Gets a key policy attached to the specified KMS key. Cross-account use : No. |
| `GetKeyRotationStatus` | - | - | `KeyId` | - | `GetKeyRotationStatusResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Provides detailed information about the rotation status for a KMS key, including whether automatic rotation of the key material is enabled for the specified KMS key, the rotation period, and the next scheduled rotation date. Automatic key rotation is... |
| `GetParametersForImport` | - | - | `KeyId`, `WrappingAlgorithm`, `WrappingKeySpec` | - | `GetParametersForImportResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Returns the public key and an import token you need to import or reimport key material for a KMS key. By default, KMS keys are created with key material that KMS generates. |
| `GetPublicKey` | - | - | `KeyId` | - | `GetPublicKeyResponse` | `DependencyTimeoutException`, `DisabledException`, `InvalidArnException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidStateException`, `KeyUnavailableException`, ... (+2) | Returns the public key of an asymmetric KMS key. Unlike the private key of a asymmetric KMS key, which never leaves KMS unencrypted, callers with `kms:GetPublicKey` permission can download the public key of an asymmetric KMS key. |
| `ImportKeyMaterial` | - | - | `EncryptedKeyMaterial`, `ImportToken`, `KeyId` | - | `ImportKeyMaterialResponse` | `DependencyTimeoutException`, `ExpiredImportTokenException`, `IncorrectKeyMaterialException`, `InvalidArnException`, `InvalidCiphertextException`, `InvalidImportTokenException`, `KMSInternalException`, `KMSInvalidStateException`, ... (+2) | Imports or reimports key material into an existing KMS key that was created without key material. You can also use this operation to set or update the expiration model and expiration date of the imported key material. |
| `ListAliases` | - | `paginated` | - | - | `ListAliasesResponse` | `DependencyTimeoutException`, `InvalidArnException`, `InvalidMarkerException`, `KMSInternalException`, `NotFoundException` | Gets a list of aliases in the caller's Amazon Web Services account and region. For more information about aliases, see CreateAlias. |
| `ListGrants` | - | `paginated` | `KeyId` | - | `ListGrantsResponse` | `DependencyTimeoutException`, `InvalidArnException`, `InvalidGrantIdException`, `InvalidMarkerException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Gets a list of all grants for the specified KMS key. You must specify the KMS key in all requests. |
| `ListKeyPolicies` | - | `paginated` | `KeyId` | - | `ListKeyPoliciesResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Gets the names of the key policies that are attached to a KMS key. This operation is designed to get policy names that you can use in a GetKeyPolicy operation. |
| `ListKeyRotations` | - | `paginated` | `KeyId` | - | `ListKeyRotationsResponse` | `InvalidArnException`, `InvalidMarkerException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Returns information about the key materials associated with the specified KMS key. You can use the optional `IncludeKeyMaterial` parameter to control which key materials are included in the response. |
| `ListKeys` | - | `paginated` | - | - | `ListKeysResponse` | `DependencyTimeoutException`, `InvalidMarkerException`, `KMSInternalException` | Gets a list of all KMS keys in the caller's Amazon Web Services account and Region. Cross-account use : No. |
| `ListResourceTags` | - | `paginated` | `KeyId` | - | `ListResourceTagsResponse` | `InvalidArnException`, `InvalidMarkerException`, `KMSInternalException`, `NotFoundException` | Returns all tags on the specified KMS key. For general information about tags, including the format and syntax, see Tagging Amazon Web Services resources in the Amazon Web Services General Reference . |
| `ListRetirableGrants` | - | `paginated` | `RetiringPrincipal` | - | `ListGrantsResponse` | `DependencyTimeoutException`, `InvalidArnException`, `InvalidMarkerException`, `KMSInternalException`, `NotFoundException` | Returns information about all grants in the Amazon Web Services account and Region that have the specified retiring principal. You can specify any principal in your Amazon Web Services account. |
| `PutKeyPolicy` | - | - | `KeyId`, `Policy` | - | `Unit` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `MalformedPolicyDocumentException`, `NotFoundException`, `UnsupportedOperationException` | Attaches a key policy to the specified KMS key. For more information about key policies, see Key Policies in the Key Management Service Developer Guide . |
| `ReEncrypt` | - | - | `DestinationKeyId` | - | `ReEncryptResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `IncorrectKeyException`, `InvalidCiphertextException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, ... (+3) | Decrypts ciphertext and then reencrypts it entirely within KMS. You can use this operation to change the KMS key under which data is encrypted, such as when you manually rotate a KMS key or change the KMS key that protects a ciphertext. |
| `ReplicateKey` | - | - | `KeyId`, `ReplicaRegion` | - | `ReplicateKeyResponse` | `AlreadyExistsException`, `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `MalformedPolicyDocumentException`, `NotFoundException`, ... (+2) | Replicates a multi-Region key into the specified Region. This operation creates a multi-Region replica key based on a multi-Region primary key in a different Region of the same Amazon Web Services partition. |
| `RetireGrant` | - | - | - | - | `Unit` | `DependencyTimeoutException`, `DryRunOperationException`, `InvalidArnException`, `InvalidGrantIdException`, `InvalidGrantTokenException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Deletes a grant. Typically, you retire a grant when you no longer need its permissions. |
| `RevokeGrant` | - | - | `GrantId`, `KeyId` | - | `Unit` | `DependencyTimeoutException`, `DryRunOperationException`, `InvalidArnException`, `InvalidGrantIdException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Deletes the specified grant. You revoke a grant to terminate the permissions that the grant allows. |
| `RotateKeyOnDemand` | - | - | `KeyId` | - | `RotateKeyOnDemandResponse` | `ConflictException`, `DependencyTimeoutException`, `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException`, ... (+1) | Immediately initiates rotation of the key material of the specified symmetric encryption KMS key. You can perform on-demand rotation of the key material in customer managed KMS keys, regardless of whether or not automatic key rotation is enabled. |
| `ScheduleKeyDeletion` | - | - | `KeyId` | - | `ScheduleKeyDeletionResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Schedules the deletion of a KMS key. By default, KMS applies a waiting period of 30 days, but you can specify a waiting period of 7-30 days. |
| `Sign` | - | - | `KeyId`, `Message`, `SigningAlgorithm` | - | `SignResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidStateException`, `KeyUnavailableException`, ... (+1) | Creates a digital signature for a message or message digest by using the private key in an asymmetric signing KMS key. To verify the signature, use the Verify operation, or use the public key in the same asymmetric KMS key outside of KMS. |
| `TagResource` | - | - | `KeyId`, `Tags` | - | `Unit` | `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException`, `TagException` | Adds or edits tags on a customer managed key. Tagging or untagging a KMS key can allow or deny permission to the KMS key. |
| `UntagResource` | - | - | `KeyId`, `TagKeys` | - | `Unit` | `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `TagException` | Deletes tags from a customer managed key. To delete a tag, specify the tag key and the KMS key. |
| `UpdateAlias` | - | - | `AliasName`, `TargetKeyId` | - | `Unit` | `DependencyTimeoutException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException` | Associates an existing KMS alias with a different KMS key. Each alias is associated with only one KMS key at a time, although a KMS key can have multiple aliases. |
| `UpdateCustomKeyStore` | - | - | `CustomKeyStoreId` | - | `UpdateCustomKeyStoreResponse` | `CloudHsmClusterInvalidConfigurationException`, `CloudHsmClusterNotActiveException`, `CloudHsmClusterNotFoundException`, `CloudHsmClusterNotRelatedException`, `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNameInUseException`, `CustomKeyStoreNotFoundException`, `KMSInternalException`, ... (+9) | Changes the properties of a custom key store. You can use this operation to change the properties of an CloudHSM key store or an external key store. |
| `UpdateKeyDescription` | - | - | `Description`, `KeyId` | - | `Unit` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Updates the description of a KMS key. To see the description of a KMS key, use DescribeKey. |
| `UpdatePrimaryRegion` | - | - | `KeyId`, `PrimaryRegion` | - | `Unit` | `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Changes the primary key of a multi-Region key. This operation changes the replica key in the specified Region to a primary key and changes the former primary key to a replica key. |
| `Verify` | - | - | `KeyId`, `Message`, `Signature`, `SigningAlgorithm` | - | `VerifyResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidSignatureException`, `KMSInvalidStateException`, ... (+2) | Verifies a digital signature that was generated by the Sign operation. Verification confirms that an authorized user signed the message with the specified KMS key and signing algorithm, and the message hasn't changed since it was signed. |
| `VerifyMac` | - | - | `KeyId`, `Mac`, `MacAlgorithm`, `Message` | - | `VerifyMacResponse` | `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KMSInternalException`, `KMSInvalidMacException`, `KMSInvalidStateException`, `KeyUnavailableException`, ... (+1) | Verifies the hash-based message authentication code (HMAC) for a specified message, HMAC KMS key, and MAC algorithm. To verify the HMAC, `VerifyMac` computes an HMAC using the message, HMAC KMS key, and MAC algorithm that you specify, and compares the... |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `KMSInternalException` | `structure` | `message` | The request was rejected because an internal exception occurred. |
| `NotFoundException` | `structure` | `message` | The request was rejected because the specified entity or resource could not be found. |
| `KMSInvalidStateException` | `structure` | `message` | The request was rejected because the state of the specified resource is not valid for this request. |
| `DependencyTimeoutException` | `structure` | `message` | The system timed out while trying to fulfill the request. |
| `InvalidArnException` | `structure` | `message` | The request was rejected because a specified ARN, or an ARN in a key policy, is not valid. |
| `DisabledException` | `structure` | `message` | The request was rejected because the specified KMS key is not enabled. |
| `UnsupportedOperationException` | `structure` | `message` | The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. |
| `DryRunOperationException` | `structure` | `message` | The request was rejected because the DryRun parameter was specified. |
| `InvalidGrantTokenException` | `structure` | `message` | The request was rejected because the specified grant token is not valid. |
| `InvalidKeyUsageException` | `structure` | `message` | The request was rejected for one of the following reasons: The `KeyUsage` value of the KMS key is incompatible with the API operation. |
| `KeyUnavailableException` | `structure` | `message` | The request was rejected because the specified KMS key was not available. |
| `LimitExceededException` | `structure` | `message` | The request was rejected because a length constraint or quota was exceeded. |
| `CustomKeyStoreNotFoundException` | `structure` | `message` | The request was rejected because KMS cannot find a custom key store with the specified key store name or ID. |
| `InvalidMarkerException` | `structure` | `message` | The request was rejected because the marker that specifies where pagination should next begin is not valid. |
| `CustomKeyStoreInvalidStateException` | `structure` | `message` | The request was rejected because of the `ConnectionState` of the custom key store. |
| `CloudHsmClusterInvalidConfigurationException` | `structure` | `message` | The request was rejected because the associated CloudHSM cluster did not meet the configuration requirements for an CloudHSM key store. |
| `TagException` | `structure` | `message` | The request was rejected because one or more tags are not valid. |
| `CloudHsmClusterNotActiveException` | `structure` | `message` | The request was rejected because the CloudHSM cluster associated with the CloudHSM key store is not active. |
| `MalformedPolicyDocumentException` | `structure` | `message` | The request was rejected because the specified policy is not syntactically or semantically correct. |
| `InvalidCiphertextException` | `structure` | `message` | From the Decrypt or ReEncrypt operation, the request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the... |
| `InvalidGrantIdException` | `structure` | `message` | The request was rejected because the specified `GrantId` is not valid. |
| `AlreadyExistsException` | `structure` | `message` | The request was rejected because it attempted to create a resource that already exists. |
| `CloudHsmClusterNotFoundException` | `structure` | `message` | The request was rejected because KMS cannot find the CloudHSM cluster with the specified cluster ID. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
