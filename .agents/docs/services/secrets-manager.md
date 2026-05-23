# AWS Secrets Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Secrets Manager Amazon Web Services Secrets Manager provides a service to enable you to store, manage, and retrieve, secrets. This guide provides descriptions of the Secrets Manager API. For more information about using this service, see the Amazon Web Services Secrets Manager User Guide. API Version This version of the Secrets Manager API Reference documents the Secrets Manager API version 2017-10-17. For a list of endpoints, see Amazon Web Services Secrets Manager endpoints.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-secretsmanager/tests/scenario_test.rs`: rotate a secret with manual stage promotion, then revert to a previous stage.
- Backported from `scenario_test.rs`: put a new secret value without rotation and promote staging labels manually.
- From the AWS documentation and model: model secret creation, versioned values, staging labels, rotation configuration, replication, resource policies, deletion recovery windows, and tag-based secret management.

## Service Identity and Protocol

- AWS model slug: `secrets-manager`
- AWS SDK for Rust slug: `secretsmanager`
- Model version: `2017-10-17`
- Model file: `vendor/api-models-aws/models/secrets-manager/service/2017-10-17/secrets-manager-2017-10-17.json`
- SDK ID: `Secrets Manager`
- Endpoint prefix: `secretsmanager`
- ARN namespace: `secretsmanager`
- CloudFormation name: `SecretsManager`
- CloudTrail event source: `secretsmanager.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (3), `Delete` (2), `List` (2), `Put` (2), `Update` (2), `Batch` (1), `Cancel` (1), `Create` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetSecretValue`, `CancelRotateSecret`, `CreateSecret`, `DeleteResourcePolicy`, `DeleteSecret`, `PutResourcePolicy`, `PutSecretValue`, `RemoveRegionsFromReplication`, `RestoreSecret`, `StopReplicationToReplica`, `TagResource`, `UntagResource`, `UpdateSecret`, `UpdateSecretVersionStage`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeSecret`, `GetRandomPassword`, `GetResourcePolicy`, `GetSecretValue`, `ListSecretVersionIds`, `ListSecrets`, `ValidateResourcePolicy`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelRotateSecret`, `StopReplicationToReplica`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `Lambda`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html
- https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_lambda-functions.html
- https://docs.aws.amazon.com/secretsmanager/latest/userguide/security-encryption.html

Research outcomes:
- Secret values are versioned. GetSecretValue can select by VersionId or VersionStage; if neither is supplied, AWSCURRENT is returned.
- If both VersionId and VersionStage are supplied, they must refer to the same secret version.
- Staging labels track versions during rotation. AWSCURRENT, AWSPREVIOUS, and AWSPENDING have defined rotation roles.
- Rotation functions normally run createSecret, setSecret, testSecret, and finishSecret. createSecret stores the new value with AWSPENDING for idempotency.
- finishSecret moves AWSCURRENT to the new version and Secrets Manager automatically adds AWSPREVIOUS to the prior version.
- An AWSPENDING label attached to a different version than AWSCURRENT can cause later rotation attempts to treat the previous rotation as still in progress.
- Secrets Manager encrypts secret values, but not secret names, descriptions, rotation settings, KMS key ARN metadata, or tags.
- Secrets Manager uses a KMS encryption context containing SecretARN and SecretVersionId for secret value encryption and decryption.

Parity implications:
- Model secrets, versions, staging labels, KMS key metadata, rotation configuration, replication, deletion windows, and CloudTrail-visible access separately.
- Version-stage movement must be atomic and enforce AWSCURRENT/AWSPREVIOUS/AWSPENDING rotation invariants.
- GetSecretValue should enforce VersionId and VersionStage consistency, default AWSCURRENT selection, binary/string value distinction, and KMS decrypt permission where applicable.

## Operation Groups

### Get

- Operations: `GetRandomPassword`, `GetResourcePolicy`, `GetSecretValue`
- Common required input members in this group: `SecretId`

### Delete

- Operations: `DeleteResourcePolicy`, `DeleteSecret`
- Common required input members in this group: `SecretId`

### List

- Operations: `ListSecrets`, `ListSecretVersionIds`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`, `PutSecretValue`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `SecretId`

### Update

- Operations: `UpdateSecret`, `UpdateSecretVersionStage`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `SecretId`

### Batch

- Operations: `BatchGetSecretValue`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Cancel

- Operations: `CancelRotateSecret`
- Common required input members in this group: -

### Create

- Operations: `CreateSecret`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Describe

- Operations: `DescribeSecret`
- Common required input members in this group: -

### Remove

- Operations: `RemoveRegionsFromReplication`
- Common required input members in this group: -

### Replicate

- Operations: `ReplicateSecretToRegions`
- Common required input members in this group: -

### Restore

- Operations: `RestoreSecret`
- Common required input members in this group: -

### Rotate

- Operations: `RotateSecret`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Stop

- Operations: `StopReplicationToReplica`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Validate

- Operations: `ValidateResourcePolicy`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetSecretValue` | `-` | `paginated` | - | - | `BatchGetSecretValueResponse` | `DecryptionFailure`, `InternalServiceError`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Retrieves the contents of the encrypted fields SecretString or SecretBinary for up to 20 secrets. To retrieve a single secret, call GetSecretValue . To choose which secrets to retrieve, you can specify a list of secr ... |
| `CancelRotateSecret` | `-` | - | `SecretId` | - | `CancelRotateSecretResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Turns off automatic rotation, and if a rotation is currently in progress, cancels the rotation. If you cancel a rotation in progress, it can leave the VersionStage labels in an unexpected state. You might need to rem ... |
| `CreateSecret` | `-` | `idempotency-token` | `Name` | `ClientRequestToken` | `CreateSecretResponse` | `DecryptionFailure`, `EncryptionFailure`, `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `MalformedPolicyDocumentException`, `PreconditionNotMetException`, `ResourceExistsException`, `ResourceNotFoundException` | Creates a new secret. A secret can be a password, a set of credentials such as a user name and password, an OAuth token, or other secret information that you store in an encrypted form in Secrets Manager. The secret ... |
| `DeleteResourcePolicy` | `-` | - | `SecretId` | - | `DeleteResourcePolicyResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Deletes the resource-based permission policy attached to the secret. To attach a policy to a secret, use PutResourcePolicy . Secrets Manager generates a CloudTrail log entry when you call this action. Do not include ... |
| `DeleteSecret` | `-` | - | `SecretId` | - | `DeleteSecretResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Deletes a secret and all of its versions. You can specify a recovery window during which you can restore the secret. The minimum recovery window is 7 days. The default recovery window is 30 days. Secrets Manager atta ... |
| `DescribeSecret` | `-` | - | `SecretId` | - | `DescribeSecretResponse` | `InternalServiceError`, `InvalidParameterException`, `ResourceNotFoundException` | Retrieves the details of a secret. It does not include the encrypted secret value. Secrets Manager only returns fields that have a value in the response. Secrets Manager generates a CloudTrail log entry when you call ... |
| `GetRandomPassword` | `-` | - | - | - | `GetRandomPasswordResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException` | Generates a random password. We recommend that you specify the maximum length and include every character type that the system you are generating a password for can support. By default, Secrets Manager uses uppercase ... |
| `GetResourcePolicy` | `-` | - | `SecretId` | - | `GetResourcePolicyResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Retrieves the JSON text of the resource-based policy document attached to the secret. For more information about permissions policies attached to a secret, see Permissions policies attached to a secret . Secrets Mana ... |
| `GetSecretValue` | `-` | - | `SecretId` | - | `GetSecretValueResponse` | `DecryptionFailure`, `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Retrieves the contents of the encrypted fields SecretString or SecretBinary from the specified version of a secret, whichever contains content. To retrieve the values for a group of secrets, call BatchGetSecretValue ... |
| `ListSecrets` | `-` | `paginated` | - | - | `ListSecretsResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidRequestException` | Lists the secrets that are stored by Secrets Manager in the Amazon Web Services account, not including secrets that are marked for deletion. To see secrets marked for deletion, use the Secrets Manager console. All Se ... |
| `ListSecretVersionIds` | `-` | `paginated` | `SecretId` | - | `ListSecretVersionIdsResponse` | `InternalServiceError`, `InvalidNextTokenException`, `InvalidParameterException`, `ResourceNotFoundException` | Lists the versions of a secret. Secrets Manager uses staging labels to indicate the different versions of a secret. For more information, see Secrets Manager concepts: Versions . To list the secrets in the account, u ... |
| `PutResourcePolicy` | `-` | - | `SecretId`, `ResourcePolicy` | - | `PutResourcePolicyResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `MalformedPolicyDocumentException`, `PublicPolicyException`, `ResourceNotFoundException` | Attaches a resource-based permission policy to a secret. A resource-based policy is optional. For more information, see Authentication and access control for Secrets Manager For information about attaching a policy i ... |
| `PutSecretValue` | `-` | `idempotency-token` | `SecretId` | `ClientRequestToken` | `PutSecretValueResponse` | `DecryptionFailure`, `EncryptionFailure`, `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceExistsException`, `ResourceNotFoundException` | Creates a new version of your secret by creating a new encrypted value and attaching it to the secret. version can contain a new SecretString value or a new SecretBinary value. Do not call PutSecretValue at a sustain ... |
| `RemoveRegionsFromReplication` | `-` | - | `SecretId`, `RemoveReplicaRegions` | - | `RemoveRegionsFromReplicationResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | For a secret that is replicated to other Regions, deletes the secret replicas from the Regions you specify. Secrets Manager generates a CloudTrail log entry when you call this action. Do not include sensitive informa ... |
| `ReplicateSecretToRegions` | `-` | - | `SecretId`, `AddReplicaRegions` | - | `ReplicateSecretToRegionsResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Replicates the secret to a new Regions. See Multi-Region secrets . Secrets Manager generates a CloudTrail log entry when you call this action. Do not include sensitive information in request parameters because it mig ... |
| `RestoreSecret` | `-` | - | `SecretId` | - | `RestoreSecretResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Cancels the scheduled deletion of a secret by removing the DeletedDate time stamp. You can access a secret again after it has been restored. Secrets Manager generates a CloudTrail log entry when you call this action. ... |
| `RotateSecret` | `-` | `idempotency-token` | `SecretId` | `ClientRequestToken` | `RotateSecretResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Configures and starts the asynchronous process of rotating the secret. For information about rotation, see Rotate secrets in the Secrets Manager User Guide . If you include the configuration parameters, the operation ... |
| `StopReplicationToReplica` | `-` | - | `SecretId` | - | `StopReplicationToReplicaResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Removes the link between the replica secret and the primary secret and promotes the replica to a primary secret in the replica Region. You must call this operation from the Region in which you want to promote the rep ... |
| `TagResource` | `-` | - | `SecretId`, `Tags` | - | `Unit` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Attaches tags to a secret. Tags consist of a key name and a value. Tags are part of the secret's metadata. They are not associated with specific versions of the secret. This operation appends tags to the existing lis ... |
| `UntagResource` | `-` | - | `SecretId`, `TagKeys` | - | `Unit` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException` | Removes specific tags from a secret. This operation is idempotent. If a requested tag is not attached to the secret, no error is returned and the secret metadata is unchanged. If you use tags as part of your security ... |
| `UpdateSecret` | `-` | `idempotency-token` | `SecretId` | `ClientRequestToken` | `UpdateSecretResponse` | `DecryptionFailure`, `EncryptionFailure`, `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `MalformedPolicyDocumentException`, `PreconditionNotMetException`, `ResourceExistsException`, `ResourceNotFoundException` | Modifies the details of a secret, including metadata and the secret value. To change the secret value, you can also use PutSecretValue . To change the rotation configuration of a secret, use RotateSecret instead. To ... |
| `UpdateSecretVersionStage` | `-` | - | `SecretId`, `VersionStage` | - | `UpdateSecretVersionStageResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException` | Modifies the staging labels attached to a version of a secret. Secrets Manager uses staging labels to track a version as it progresses through the secret rotation process. Each staging label can be attached to only o ... |
| `ValidateResourcePolicy` | `-` | - | `ResourcePolicy` | - | `ValidateResourcePolicyResponse` | `InternalServiceError`, `InvalidParameterException`, `InvalidRequestException`, `MalformedPolicyDocumentException`, `ResourceNotFoundException` | Validates that a resource policy does not grant a wide range of principals access to your secret. A resource-based policy is optional for secrets. The API performs three checks when validating the policy: Sends a cal ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DecryptionFailure` | `structure` | Message | Secrets Manager can't decrypt the protected secret text using the provided KMS key. |
| `EncryptionFailure` | `structure` | Message | Secrets Manager can't encrypt the protected secret text using the provided KMS key. Check that the KMS key is available, enabled, and not in an invalid stat ... |
| `InternalServiceError` | `structure` | Message | An error occurred on the server side. |
| `InvalidNextTokenException` | `structure` | Message | The NextToken value is invalid. |
| `InvalidParameterException` | `structure` | Message | The parameter name or value is invalid. |
| `InvalidRequestException` | `structure` | Message | A parameter value is not valid for the current state of the resource. Possible causes: The secret is scheduled for deletion. You tried to enable rotation on ... |
| `LimitExceededException` | `structure` | Message | The request failed because it would exceed one of the Secrets Manager quotas. |
| `MalformedPolicyDocumentException` | `structure` | Message | The resource policy has syntax errors. |
| `PreconditionNotMetException` | `structure` | Message | The request failed because you did not complete all the prerequisite steps. |
| `PublicPolicyException` | `structure` | Message | The BlockPublicPolicy parameter is set to true, and the resource policy did not prevent broad access to the secret. |
| `ResourceExistsException` | `structure` | Message | A resource with the ID you requested already exists. |
| `ResourceNotFoundException` | `structure` | Message | Secrets Manager can't find the resource that you asked for. |
| `BatchGetSecretValueRequest` | `structure` | SecretIdList, Filters, MaxResults, NextToken | - |
| `BatchGetSecretValueResponse` | `structure` | SecretValues, NextToken, Errors | - |
| `CancelRotateSecretRequest` | `structure` | SecretId | - |
| `CancelRotateSecretResponse` | `structure` | ARN, Name, VersionId | - |
| `CreateSecretRequest` | `structure` | Name, ClientRequestToken, Description, KmsKeyId, SecretBinary, SecretString, Tags, AddReplicaRegions, ForceOverwriteReplicaSecret, Type | - |
| `CreateSecretResponse` | `structure` | ARN, Name, VersionId, ReplicationStatus | - |
| `DeleteResourcePolicyRequest` | `structure` | SecretId | - |
| `DeleteResourcePolicyResponse` | `structure` | ARN, Name | - |
| `DeleteSecretRequest` | `structure` | SecretId, RecoveryWindowInDays, ForceDeleteWithoutRecovery | - |
| `DeleteSecretResponse` | `structure` | ARN, Name, DeletionDate | - |
| `DescribeSecretRequest` | `structure` | SecretId | - |
| `DescribeSecretResponse` | `structure` | ARN, Name, Type, Description, KmsKeyId, RotationEnabled, RotationLambdaARN, RotationRules, ExternalSecretRotationMetadata, ExternalSecretRotationRoleArn, LastRotatedDate, LastChangedDate, ... (+9) | - |
| `GetRandomPasswordRequest` | `structure` | PasswordLength, ExcludeCharacters, ExcludeNumbers, ExcludePunctuation, ExcludeUppercase, ExcludeLowercase, IncludeSpace, RequireEachIncludedType | - |
| `GetRandomPasswordResponse` | `structure` | RandomPassword | - |
| `GetResourcePolicyRequest` | `structure` | SecretId | - |
| `GetResourcePolicyResponse` | `structure` | ARN, Name, ResourcePolicy | - |
| `GetSecretValueRequest` | `structure` | SecretId, VersionId, VersionStage | - |
| `GetSecretValueResponse` | `structure` | ARN, Name, VersionId, SecretBinary, SecretString, VersionStages, CreatedDate | - |
| `ListSecretsRequest` | `structure` | IncludePlannedDeletion, MaxResults, NextToken, Filters, SortOrder, SortBy | - |
| `ListSecretsResponse` | `structure` | SecretList, NextToken | - |
| `ListSecretVersionIdsRequest` | `structure` | SecretId, MaxResults, NextToken, IncludeDeprecated | - |
| `ListSecretVersionIdsResponse` | `structure` | Versions, NextToken, ARN, Name | - |
| `PutResourcePolicyRequest` | `structure` | SecretId, ResourcePolicy, BlockPublicPolicy | - |
| `PutResourcePolicyResponse` | `structure` | ARN, Name | - |
| `PutSecretValueRequest` | `structure` | SecretId, ClientRequestToken, SecretBinary, SecretString, VersionStages, RotationToken | - |
| `PutSecretValueResponse` | `structure` | ARN, Name, VersionId, VersionStages | - |
| `RemoveRegionsFromReplicationRequest` | `structure` | SecretId, RemoveReplicaRegions | - |
| `RemoveRegionsFromReplicationResponse` | `structure` | ARN, ReplicationStatus | - |
| `FilterNameStringType` | `enum` | description, name, tag_key, tag_value, primary_region, owning_service, all | - |
| `SortByType` | `enum` | created_date, last_accessed_date, last_changed_date, name | - |
| `SortOrderType` | `enum` | asc, desc | - |
| `StatusType` | `enum` | InSync, Failed, InProgress | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
