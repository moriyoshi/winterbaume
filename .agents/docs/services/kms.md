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
- Common required input members in this group: `KeyId`

### Generate

- Operations: `GenerateDataKey`, `GenerateDataKeyPair`, `GenerateDataKeyPairWithoutPlaintext`, `GenerateDataKeyWithoutPlaintext`, `GenerateMac`, `GenerateRandom`
- Common required input members in this group: `KeyId`, `KeyPairSpec`

### Get

- Operations: `GetKeyLastUsage`, `GetKeyPolicy`, `GetKeyRotationStatus`, `GetParametersForImport`, `GetPublicKey`
- Common required input members in this group: `KeyId`

### Create

- Operations: `CreateAlias`, `CreateCustomKeyStore`, `CreateGrant`, `CreateKey`
- Common required input members in this group: -

### Update

- Operations: `UpdateAlias`, `UpdateCustomKeyStore`, `UpdateKeyDescription`, `UpdatePrimaryRegion`
- Common required input members in this group: `KeyId`

### Delete

- Operations: `DeleteAlias`, `DeleteCustomKeyStore`, `DeleteImportedKeyMaterial`
- Common required input members in this group: -

### Describe

- Operations: `DescribeCustomKeyStores`, `DescribeKey`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Disable

- Operations: `DisableKey`, `DisableKeyRotation`
- Common required input members in this group: `KeyId`

### Enable

- Operations: `EnableKey`, `EnableKeyRotation`
- Common required input members in this group: `KeyId`

### Verify

- Operations: `Verify`, `VerifyMac`
- Common required input members in this group: `KeyId`, `Message`

### Cancel

- Operations: `CancelKeyDeletion`
- Common required input members in this group: -

### Connect

- Operations: `ConnectCustomKeyStore`
- Common required input members in this group: -

### Decrypt

- Operations: `Decrypt`
- Common required input members in this group: -

### Derive

- Operations: `DeriveSharedSecret`
- Common required input members in this group: -

### Disconnect

- Operations: `DisconnectCustomKeyStore`
- Common required input members in this group: -

### Encrypt

- Operations: `Encrypt`
- Common required input members in this group: -

### Import

- Operations: `ImportKeyMaterial`
- Common required input members in this group: -

### Put

- Operations: `PutKeyPolicy`
- Common required input members in this group: -

### Re

- Operations: `ReEncrypt`
- Common required input members in this group: -

### Replicate

- Operations: `ReplicateKey`
- Common required input members in this group: -

### Retire

- Operations: `RetireGrant`
- Common required input members in this group: -

### Revoke

- Operations: `RevokeGrant`
- Common required input members in this group: -

### Rotate

- Operations: `RotateKeyOnDemand`
- Common required input members in this group: -

### Schedule

- Operations: `ScheduleKeyDeletion`
- Common required input members in this group: -

### Sign

- Operations: `Sign`
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
| `CancelKeyDeletion` | `-` | - | `KeyId` | - | `CancelKeyDeletionResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Cancels the deletion of a KMS key. When this operation succeeds, the key state of the KMS key is Disabled . To enable the KMS key, use EnableKey . For more information about scheduling and canceling deletion of a KMS ... |
| `ConnectCustomKeyStore` | `-` | - | `CustomKeyStoreId` | - | `ConnectCustomKeyStoreResponse` | `CloudHsmClusterInvalidConfigurationException`, `CloudHsmClusterNotActiveException`, `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `KMSInternalException` | Connects or reconnects a custom key store to its backing key store. For an CloudHSM key store, ConnectCustomKeyStore connects the key store to its associated CloudHSM cluster. For an external key store, ConnectCustom ... |
| `CreateAlias` | `-` | - | `AliasName`, `TargetKeyId` | - | `Unit` | `AlreadyExistsException`, `DependencyTimeoutException`, `InvalidAliasNameException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException` | Creates a friendly name for a KMS key. Adding, deleting, or updating an alias can allow or deny permission to the KMS key. For details, see ABAC for KMS in the Key Management Service Developer Guide . You can use an ... |
| `CreateCustomKeyStore` | `-` | - | `CustomKeyStoreName` | - | `CreateCustomKeyStoreResponse` | `CloudHsmClusterInUseException`, `CloudHsmClusterInvalidConfigurationException`, `CloudHsmClusterNotActiveException`, `CloudHsmClusterNotFoundException`, `CustomKeyStoreNameInUseException`, `IncorrectTrustAnchorException`, `KMSInternalException`, `LimitExceededException`, `XksProxyIncorrectAuthenticationCredentialException`, `XksProxyInvalidConfigurationException`, `XksProxyInvalidResponseException`, `XksProxyUriEndpointInUseException`, `XksProxyUriInUseException`, `XksProxyUriUnreachableException`, `XksProxyVpcEndpointServiceInUseException`, `XksProxyVpcEndpointServiceInvalidConfigurationException`, `XksProxyVpcEndpointServiceNotFoundException` | Creates a custom key store backed by a key store that you own and manage. When you use a KMS key in a custom key store for a cryptographic operation, the cryptographic operation is actually performed in your key stor ... |
| `CreateGrant` | `-` | - | `KeyId`, `GranteePrincipal`, `Operations` | - | `CreateGrantResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidArnException`, `InvalidGrantTokenException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException` | Adds a grant to a KMS key. A grant is a policy instrument that allows Amazon Web Services principals to use KMS keys in cryptographic operations. It also can allow them to view a KMS key ( DescribeKey ) and create an ... |
| `CreateKey` | `-` | - | - | - | `CreateKeyResponse` | `CloudHsmClusterInvalidConfigurationException`, `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `LimitExceededException`, `MalformedPolicyDocumentException`, `TagException`, `UnsupportedOperationException`, `XksKeyAlreadyInUseException`, `XksKeyInvalidConfigurationException`, `XksKeyNotFoundException` | Creates a unique customer managed KMS key in your Amazon Web Services account and Region. You can use a KMS key in cryptographic operations, such as encryption and signing. Some Amazon Web Services services let you u ... |
| `Decrypt` | `-` | - | - | - | `DecryptResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `IncorrectKeyException`, `InvalidCiphertextException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Decrypts ciphertext that was encrypted by a KMS key using any of the following operations: Encrypt GenerateDataKey GenerateDataKeyPair GenerateDataKeyWithoutPlaintext GenerateDataKeyPairWithoutPlaintext You can use t ... |
| `DeleteAlias` | `-` | - | `AliasName` | - | `Unit` | `DependencyTimeoutException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Deletes the specified alias. Adding, deleting, or updating an alias can allow or deny permission to the KMS key. For details, see ABAC for KMS in the Key Management Service Developer Guide . Because an alias is not a ... |
| `DeleteCustomKeyStore` | `-` | - | `CustomKeyStoreId` | - | `DeleteCustomKeyStoreResponse` | `CustomKeyStoreHasCMKsException`, `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `KMSInternalException` | Deletes a custom key store . This operation does not affect any backing elements of the custom key store. It does not delete the CloudHSM cluster that is associated with an CloudHSM key store, or affect any users or ... |
| `DeleteImportedKeyMaterial` | `-` | - | `KeyId` | - | `DeleteImportedKeyMaterialResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Deletes key material that was previously imported. This operation makes the specified KMS key temporarily unusable. To restore the usability of the KMS key, reimport the same key material. For more information about ... |
| `DeriveSharedSecret` | `-` | - | `KeyId`, `KeyAgreementAlgorithm`, `PublicKey` | - | `DeriveSharedSecretResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Derives a shared secret using a key agreement algorithm. You must use an asymmetric NIST-standard elliptic curve (ECC) or SM2 (China Regions only) KMS key pair with a KeyUsage value of KEY_AGREEMENT to call DeriveSha ... |
| `DescribeCustomKeyStores` | `-` | `paginated` | - | - | `DescribeCustomKeyStoresResponse` | `CustomKeyStoreNotFoundException`, `InvalidMarkerException`, `KMSInternalException` | Gets information about custom key stores in the account and Region. This operation is part of the custom key stores feature in KMS, which combines the convenience and extensive integration of KMS with the isolation a ... |
| `DescribeKey` | `-` | - | `KeyId` | - | `DescribeKeyResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `NotFoundException` | Provides detailed information about a KMS key. You can run DescribeKey on a customer managed key or an Amazon Web Services managed key . This detailed information includes the key ARN, creation date (and deletion dat ... |
| `DisableKey` | `-` | - | `KeyId` | - | `Unit` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Sets the state of a KMS key to disabled. This change temporarily prevents use of the KMS key for cryptographic operations . The KMS key that you use for this operation must be in a compatible key state. For more info ... |
| `DisableKeyRotation` | `-` | - | `KeyId` | - | `Unit` | `DependencyTimeoutException`, `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Disables automatic rotation of the key material of the specified symmetric encryption KMS key. Automatic key rotation is supported only on symmetric encryption KMS keys. You cannot enable automatic rotation of asymme ... |
| `DisconnectCustomKeyStore` | `-` | - | `CustomKeyStoreId` | - | `DisconnectCustomKeyStoreResponse` | `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `KMSInternalException` | Disconnects the custom key store from its backing key store. This operation disconnects an CloudHSM key store from its associated CloudHSM cluster or disconnects an external key store from the external key store prox ... |
| `EnableKey` | `-` | - | `KeyId` | - | `Unit` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException` | Sets the key state of a KMS key to enabled. This allows you to use the KMS key for cryptographic operations . The KMS key that you use for this operation must be in a compatible key state. For details, see Key states ... |
| `EnableKeyRotation` | `-` | - | `KeyId` | - | `Unit` | `DependencyTimeoutException`, `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Enables automatic rotation of the key material of the specified symmetric encryption KMS key. By default, when you enable automatic rotation of a customer managed KMS key , KMS rotates the key material of the KMS key ... |
| `Encrypt` | `-` | - | `KeyId`, `Plaintext` | - | `EncryptResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Encrypts plaintext of up to 4,096 bytes using a KMS key. You can use a symmetric or asymmetric KMS key with a KeyUsage of ENCRYPT_DECRYPT . You can use this operation to encrypt small amounts of arbitrary data, such ... |
| `GenerateDataKey` | `-` | - | `KeyId` | - | `GenerateDataKeyResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Returns a unique symmetric data key for use outside of KMS. This operation returns a plaintext copy of the data key and a copy that is encrypted under a symmetric encryption KMS key that you specify. The bytes in the ... |
| `GenerateDataKeyPair` | `-` | - | `KeyId`, `KeyPairSpec` | - | `GenerateDataKeyPairResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Returns a unique asymmetric data key pair for use outside of KMS. This operation returns a plaintext public key, a plaintext private key, and a copy of the private key that is encrypted under the symmetric encryption ... |
| `GenerateDataKeyPairWithoutPlaintext` | `-` | - | `KeyId`, `KeyPairSpec` | - | `GenerateDataKeyPairWithoutPlaintextResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Returns a unique asymmetric data key pair for use outside of KMS. This operation returns a plaintext public key and a copy of the private key that is encrypted under the symmetric encryption KMS key you specify. Unli ... |
| `GenerateDataKeyWithoutPlaintext` | `-` | - | `KeyId` | - | `GenerateDataKeyWithoutPlaintextResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Returns a unique symmetric data key for use outside of KMS. This operation returns a data key that is encrypted under a symmetric encryption KMS key that you specify. The bytes in the key are random; they are not rel ... |
| `GenerateMac` | `-` | - | `Message`, `KeyId`, `MacAlgorithm` | - | `GenerateMacResponse` | `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Generates a hash-based message authentication code (HMAC) for a message using an HMAC KMS key and a MAC algorithm that the key supports. HMAC KMS keys and the HMAC algorithms that KMS uses conform to industry standar ... |
| `GenerateRandom` | `-` | - | - | - | `GenerateRandomResponse` | `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNotFoundException`, `DependencyTimeoutException`, `KMSInternalException`, `UnsupportedOperationException` | Returns a random byte string that is cryptographically secure. You must use the NumberOfBytes parameter to specify the length of the random byte string. There is no default value for string length. By default, the ra ... |
| `GetKeyLastUsage` | `-` | - | `KeyId` | - | `GetKeyLastUsageResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `NotFoundException` | Returns usage information about the last successful cryptographic operation performed with a specified KMS key, including the operation type, timestamp, and associated CloudTrail event ID. The TrackingStartDate in th ... |
| `GetKeyPolicy` | `-` | - | `KeyId` | - | `GetKeyPolicyResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Gets a key policy attached to the specified KMS key. Cross-account use : No. You cannot perform this operation on a KMS key in a different Amazon Web Services account. Required permissions : kms:GetKeyPolicy (key pol ... |
| `GetKeyRotationStatus` | `-` | - | `KeyId` | - | `GetKeyRotationStatusResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Provides detailed information about the rotation status for a KMS key, including whether automatic rotation of the key material is enabled for the specified KMS key, the rotation period , and the next scheduled rotat ... |
| `GetParametersForImport` | `-` | - | `KeyId`, `WrappingAlgorithm`, `WrappingKeySpec` | - | `GetParametersForImportResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Returns the public key and an import token you need to import or reimport key material for a KMS key. By default, KMS keys are created with key material that KMS generates. This operation supports Importing key mater ... |
| `GetPublicKey` | `-` | - | `KeyId` | - | `GetPublicKeyResponse` | `DependencyTimeoutException`, `DisabledException`, `InvalidArnException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Returns the public key of an asymmetric KMS key. Unlike the private key of a asymmetric KMS key, which never leaves KMS unencrypted, callers with kms:GetPublicKey permission can download the public key of an asymmetr ... |
| `ImportKeyMaterial` | `-` | - | `KeyId`, `ImportToken`, `EncryptedKeyMaterial` | - | `ImportKeyMaterialResponse` | `DependencyTimeoutException`, `ExpiredImportTokenException`, `IncorrectKeyMaterialException`, `InvalidArnException`, `InvalidCiphertextException`, `InvalidImportTokenException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Imports or reimports key material into an existing KMS key that was created without key material. You can also use this operation to set or update the expiration model and expiration date of the imported key material ... |
| `ListAliases` | `-` | `paginated` | - | - | `ListAliasesResponse` | `DependencyTimeoutException`, `InvalidArnException`, `InvalidMarkerException`, `KMSInternalException`, `NotFoundException` | Gets a list of aliases in the caller's Amazon Web Services account and region. For more information about aliases, see CreateAlias . By default, the ListAliases operation returns all aliases in the account and region ... |
| `ListGrants` | `-` | `paginated` | `KeyId` | - | `ListGrantsResponse` | `DependencyTimeoutException`, `InvalidArnException`, `InvalidGrantIdException`, `InvalidMarkerException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Gets a list of all grants for the specified KMS key. You must specify the KMS key in all requests. You can filter the grant list by grant ID or grantee principal. For detailed information about grants, including gran ... |
| `ListKeyPolicies` | `-` | `paginated` | `KeyId` | - | `ListKeyPoliciesResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Gets the names of the key policies that are attached to a KMS key. This operation is designed to get policy names that you can use in a GetKeyPolicy operation. However, the only valid policy name is default . Cross-a ... |
| `ListKeyRotations` | `-` | `paginated` | `KeyId` | - | `ListKeyRotationsResponse` | `InvalidArnException`, `InvalidMarkerException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Returns information about the key materials associated with the specified KMS key. You can use the optional IncludeKeyMaterial parameter to control which key materials are included in the response. You must specify t ... |
| `ListKeys` | `-` | `paginated` | - | - | `ListKeysResponse` | `DependencyTimeoutException`, `InvalidMarkerException`, `KMSInternalException` | Gets a list of all KMS keys in the caller's Amazon Web Services account and Region. Cross-account use : No. You cannot perform this operation on a KMS key in a different Amazon Web Services account. Required permissi ... |
| `ListResourceTags` | `-` | `paginated` | `KeyId` | - | `ListResourceTagsResponse` | `InvalidArnException`, `InvalidMarkerException`, `KMSInternalException`, `NotFoundException` | Returns all tags on the specified KMS key. For general information about tags, including the format and syntax, see Tagging Amazon Web Services resources in the Amazon Web Services General Reference . For information ... |
| `ListRetirableGrants` | `-` | `paginated` | `RetiringPrincipal` | - | `ListGrantsResponse` | `DependencyTimeoutException`, `InvalidArnException`, `InvalidMarkerException`, `KMSInternalException`, `NotFoundException` | Returns information about all grants in the Amazon Web Services account and Region that have the specified retiring principal. You can specify any principal in your Amazon Web Services account. The grants that are re ... |
| `PutKeyPolicy` | `-` | - | `KeyId`, `Policy` | - | `Unit` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `MalformedPolicyDocumentException`, `NotFoundException`, `UnsupportedOperationException` | Attaches a key policy to the specified KMS key. For more information about key policies, see Key Policies in the Key Management Service Developer Guide . For help writing and formatting a JSON policy document, see th ... |
| `ReEncrypt` | `-` | - | `DestinationKeyId` | - | `ReEncryptResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `IncorrectKeyException`, `InvalidCiphertextException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Decrypts ciphertext and then reencrypts it entirely within KMS. You can use this operation to change the KMS key under which data is encrypted, such as when you manually rotate a KMS key or change the KMS key that pr ... |
| `ReplicateKey` | `-` | - | `KeyId`, `ReplicaRegion` | - | `ReplicateKeyResponse` | `AlreadyExistsException`, `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `MalformedPolicyDocumentException`, `NotFoundException`, `TagException`, `UnsupportedOperationException` | Replicates a multi-Region key into the specified Region. This operation creates a multi-Region replica key based on a multi-Region primary key in a different Region of the same Amazon Web Services partition. You can ... |
| `RetireGrant` | `-` | - | - | - | `Unit` | `DependencyTimeoutException`, `DryRunOperationException`, `InvalidArnException`, `InvalidGrantIdException`, `InvalidGrantTokenException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Deletes a grant. Typically, you retire a grant when you no longer need its permissions. To identify the grant to retire, use a grant token , or both the grant ID and a key identifier (key ID or key ARN) of the KMS ke ... |
| `RevokeGrant` | `-` | - | `KeyId`, `GrantId` | - | `Unit` | `DependencyTimeoutException`, `DryRunOperationException`, `InvalidArnException`, `InvalidGrantIdException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Deletes the specified grant. You revoke a grant to terminate the permissions that the grant allows. For more information, see Retiring and revoking grants in the Key Management Service Developer Guide . When you crea ... |
| `RotateKeyOnDemand` | `-` | - | `KeyId` | - | `RotateKeyOnDemandResponse` | `ConflictException`, `DependencyTimeoutException`, `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException`, `UnsupportedOperationException` | Immediately initiates rotation of the key material of the specified symmetric encryption KMS key. You can perform on-demand rotation of the key material in customer managed KMS keys, regardless of whether or not auto ... |
| `ScheduleKeyDeletion` | `-` | - | `KeyId` | - | `ScheduleKeyDeletionResponse` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Schedules the deletion of a KMS key. By default, KMS applies a waiting period of 30 days, but you can specify a waiting period of 7-30 days. When this operation is successful, the key state of the KMS key changes to ... |
| `Sign` | `-` | - | `KeyId`, `Message`, `SigningAlgorithm` | - | `SignResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Creates a digital signature for a message or message digest by using the private key in an asymmetric signing KMS key. To verify the signature, use the Verify operation, or use the public key in the same asymmetric K ... |
| `TagResource` | `-` | - | `KeyId`, `Tags` | - | `Unit` | `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException`, `TagException` | Adds or edits tags on a customer managed key . Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see ABAC for KMS in the Key Management Service Developer Guide . Each tag consis ... |
| `UntagResource` | `-` | - | `KeyId`, `TagKeys` | - | `Unit` | `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `TagException` | Deletes tags from a customer managed key . To delete a tag, specify the tag key and the KMS key. Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see ABAC for KMS in the Key Ma ... |
| `UpdateAlias` | `-` | - | `AliasName`, `TargetKeyId` | - | `Unit` | `DependencyTimeoutException`, `KMSInternalException`, `KMSInvalidStateException`, `LimitExceededException`, `NotFoundException` | Associates an existing KMS alias with a different KMS key. Each alias is associated with only one KMS key at a time, although a KMS key can have multiple aliases. The alias and the KMS key must be in the same Amazon ... |
| `UpdateCustomKeyStore` | `-` | - | `CustomKeyStoreId` | - | `UpdateCustomKeyStoreResponse` | `CloudHsmClusterInvalidConfigurationException`, `CloudHsmClusterNotActiveException`, `CloudHsmClusterNotFoundException`, `CloudHsmClusterNotRelatedException`, `CustomKeyStoreInvalidStateException`, `CustomKeyStoreNameInUseException`, `CustomKeyStoreNotFoundException`, `KMSInternalException`, `XksProxyIncorrectAuthenticationCredentialException`, `XksProxyInvalidConfigurationException`, `XksProxyInvalidResponseException`, `XksProxyUriEndpointInUseException`, `XksProxyUriInUseException`, `XksProxyUriUnreachableException`, `XksProxyVpcEndpointServiceInUseException`, `XksProxyVpcEndpointServiceInvalidConfigurationException`, `XksProxyVpcEndpointServiceNotFoundException` | Changes the properties of a custom key store. You can use this operation to change the properties of an CloudHSM key store or an external key store. Use the required CustomKeyStoreId parameter to identify the custom ... |
| `UpdateKeyDescription` | `-` | - | `KeyId`, `Description` | - | `Unit` | `DependencyTimeoutException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException` | Updates the description of a KMS key. To see the description of a KMS key, use DescribeKey . The KMS key that you use for this operation must be in a compatible key state. For details, see Key states of KMS keys in t ... |
| `UpdatePrimaryRegion` | `-` | - | `KeyId`, `PrimaryRegion` | - | `Unit` | `DisabledException`, `InvalidArnException`, `KMSInternalException`, `KMSInvalidStateException`, `NotFoundException`, `UnsupportedOperationException` | Changes the primary key of a multi-Region key. This operation changes the replica key in the specified Region to a primary key and changes the former primary key to a replica key. For example, suppose you have a prim ... |
| `Verify` | `-` | - | `KeyId`, `Message`, `Signature`, `SigningAlgorithm` | - | `VerifyResponse` | `DependencyTimeoutException`, `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidSignatureException`, `KMSInvalidStateException`, `NotFoundException` | Verifies a digital signature that was generated by the Sign operation. Verification confirms that an authorized user signed the message with the specified KMS key and signing algorithm, and the message hasn't changed ... |
| `VerifyMac` | `-` | - | `Message`, `KeyId`, `MacAlgorithm`, `Mac` | - | `VerifyMacResponse` | `DisabledException`, `DryRunOperationException`, `InvalidGrantTokenException`, `InvalidKeyUsageException`, `KeyUnavailableException`, `KMSInternalException`, `KMSInvalidMacException`, `KMSInvalidStateException`, `NotFoundException` | Verifies the hash-based message authentication code (HMAC) for a specified message, HMAC KMS key, and MAC algorithm. To verify the HMAC, VerifyMac computes an HMAC using the message, HMAC KMS key, and MAC algorithm t ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AlreadyExistsException` | `structure` | message | The request was rejected because it attempted to create a resource that already exists. |
| `CloudHsmClusterInUseException` | `structure` | message | The request was rejected because the specified CloudHSM cluster is already associated with an CloudHSM key store in the account, or it shares a backup histo ... |
| `CloudHsmClusterInvalidConfigurationException` | `structure` | message | The request was rejected because the associated CloudHSM cluster did not meet the configuration requirements for an CloudHSM key store. The CloudHSM cluster ... |
| `CloudHsmClusterNotActiveException` | `structure` | message | The request was rejected because the CloudHSM cluster associated with the CloudHSM key store is not active. Initialize and activate the cluster and try the ... |
| `CloudHsmClusterNotFoundException` | `structure` | message | The request was rejected because KMS cannot find the CloudHSM cluster with the specified cluster ID. Retry the request with a different cluster ID. |
| `CloudHsmClusterNotRelatedException` | `structure` | message | The request was rejected because the specified CloudHSM cluster has a different cluster certificate than the original cluster. You cannot use the operation ... |
| `ConflictException` | `structure` | message | The request was rejected because an automatic rotation of this key is currently in progress or scheduled to begin within the next 20 minutes. |
| `CustomKeyStoreHasCMKsException` | `structure` | message | The request was rejected because the custom key store contains KMS keys. After verifying that you do not need to use the KMS keys, use the ScheduleKeyDeleti ... |
| `CustomKeyStoreInvalidStateException` | `structure` | message | The request was rejected because of the ConnectionState of the custom key store. To get the ConnectionState of a custom key store, use the DescribeCustomKey ... |
| `CustomKeyStoreNameInUseException` | `structure` | message | The request was rejected because the specified custom key store name is already assigned to another custom key store in the account. Try again with a custom ... |
| `CustomKeyStoreNotFoundException` | `structure` | message | The request was rejected because KMS cannot find a custom key store with the specified key store name or ID. |
| `DependencyTimeoutException` | `structure` | message | The system timed out while trying to fulfill the request. You can retry the request. |
| `DisabledException` | `structure` | message | The request was rejected because the specified KMS key is not enabled. |
| `DryRunOperationException` | `structure` | message | The request was rejected because the DryRun parameter was specified. |
| `ExpiredImportTokenException` | `structure` | message | The request was rejected because the specified import token is expired. Use GetParametersForImport to get a new import token and public key, use the new pub ... |
| `IncorrectKeyException` | `structure` | message | The request was rejected because the specified KMS key cannot decrypt the data. The KeyId in a Decrypt request and the SourceKeyId in a ReEncrypt request mu ... |
| `IncorrectKeyMaterialException` | `structure` | message | The request was rejected because the key material in the request is, expired, invalid, or does not meet expectations. For example, it is not the same key ma ... |
| `IncorrectTrustAnchorException` | `structure` | message | The request was rejected because the trust anchor certificate in the request to create an CloudHSM key store is not the trust anchor certificate for the spe ... |
| `InvalidAliasNameException` | `structure` | message | The request was rejected because the specified alias name is not valid. |
| `InvalidArnException` | `structure` | message | The request was rejected because a specified ARN, or an ARN in a key policy, is not valid. |
| `InvalidCiphertextException` | `structure` | message | From the Decrypt or ReEncrypt operation, the request was rejected because the specified ciphertext, or additional authenticated data incorporated into the c ... |
| `InvalidGrantIdException` | `structure` | message | The request was rejected because the specified GrantId is not valid. |
| `InvalidGrantTokenException` | `structure` | message | The request was rejected because the specified grant token is not valid. |
| `InvalidImportTokenException` | `structure` | message | The request was rejected because the provided import token is invalid or is associated with a different KMS key. |
| `InvalidKeyUsageException` | `structure` | message | The request was rejected for one of the following reasons: The KeyUsage value of the KMS key is incompatible with the API operation. The encryption algorith ... |
| `InvalidMarkerException` | `structure` | message | The request was rejected because the marker that specifies where pagination should next begin is not valid. |
| `KMSInternalException` | `structure` | message | The request was rejected because an internal exception occurred. The request can be retried. |
| `KMSInvalidMacException` | `structure` | message | The request was rejected because the HMAC verification failed. HMAC verification fails when the HMAC computed by using the specified message, HMAC KMS key, ... |
| `KMSInvalidSignatureException` | `structure` | message | The request was rejected because the signature verification failed. Signature verification fails when it cannot confirm that signature was produced by signi ... |
| `KMSInvalidStateException` | `structure` | message | The request was rejected because the state of the specified resource is not valid for this request. This exceptions means one of the following: The key stat ... |
| `KeyUnavailableException` | `structure` | message | The request was rejected because the specified KMS key was not available. You can retry the request. |
| `LimitExceededException` | `structure` | message | The request was rejected because a length constraint or quota was exceeded. For more information, see Quotas in the Key Management Service Developer Guide . |
| `MalformedPolicyDocumentException` | `structure` | message | The request was rejected because the specified policy is not syntactically or semantically correct. |
| `NotFoundException` | `structure` | message | The request was rejected because the specified entity or resource could not be found. |
| `TagException` | `structure` | message | The request was rejected because one or more tags are not valid. |
| `UnsupportedOperationException` | `structure` | message | The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. |
| `XksKeyAlreadyInUseException` | `structure` | message | The request was rejected because the ( XksKeyId ) is already associated with another KMS key in this external key store. Each KMS key in an external key sto ... |
| `XksKeyInvalidConfigurationException` | `structure` | message | The request was rejected because the external key specified by the XksKeyId parameter did not meet the configuration requirements for an external key store. ... |
| `XksKeyNotFoundException` | `structure` | message | The request was rejected because the external key store proxy could not find the external key. This exception is thrown when the value of the XksKeyId param ... |
| `XksProxyIncorrectAuthenticationCredentialException` | `structure` | message | The request was rejected because the proxy credentials failed to authenticate to the specified external key store proxy. The specified external key store pr ... |
| `XksProxyInvalidConfigurationException` | `structure` | message | The request was rejected because the external key store proxy is not configured correctly. To identify the cause, see the error message that accompanies the ... |
| `XksProxyInvalidResponseException` | `structure` | message | KMS cannot interpret the response it received from the external key store proxy. The problem might be a poorly constructed response, but it could also be a ... |
| `XksProxyUriEndpointInUseException` | `structure` | message | The request was rejected because the XksProxyUriEndpoint is already associated with another external key store in this Amazon Web Services Region. To identi ... |
| `XksProxyUriInUseException` | `structure` | message | The request was rejected because the concatenation of the XksProxyUriEndpoint and XksProxyUriPath is already associated with another external key store in t ... |
| `XksProxyUriUnreachableException` | `structure` | message | KMS was unable to reach the specified XksProxyUriPath . The path must be reachable before you create the external key store or update its settings. This exc ... |
| `XksProxyVpcEndpointServiceInUseException` | `structure` | message | The request was rejected because the specified Amazon VPC endpoint service is already associated with another external key store in this Amazon Web Services ... |
| `XksProxyVpcEndpointServiceInvalidConfigurationException` | `structure` | message | The request was rejected because the Amazon VPC endpoint service configuration does not fulfill the requirements for an external key store. To identify the ... |
| `XksProxyVpcEndpointServiceNotFoundException` | `structure` | message | The request was rejected because KMS could not find the specified VPC endpoint service. Use DescribeCustomKeyStores to verify the VPC endpoint service name ... |
| `AlgorithmSpec` | `enum` | RSAES_PKCS1_V1_5, RSAES_OAEP_SHA_1, RSAES_OAEP_SHA_256, RSA_AES_KEY_WRAP_SHA_1, RSA_AES_KEY_WRAP_SHA_256, SM2PKE | - |
| `ConnectionErrorCodeType` | `enum` | INVALID_CREDENTIALS, CLUSTER_NOT_FOUND, NETWORK_ERRORS, INTERNAL_ERROR, INSUFFICIENT_CLOUDHSM_HSMS, USER_LOCKED_OUT, USER_NOT_FOUND, USER_LOGGED_IN, SUBNET_NOT_FOUND, INSUFFICIENT_FREE_ADDRESSES_IN_SUBNET, XKS_PROXY_ACCESS_DENIED, XKS_PROXY_NOT_REACHABLE, ... (+6) | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
