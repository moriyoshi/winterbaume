# Amazon Interactive Video Service RealTime

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Interactive Video Service (IVS) real-time API is REST compatible, using a standard HTTP API and an AWS EventBridge event stream for responses. JSON is used for both requests and responses, including errors. Key Concepts Stage — A virtual space where participants can exchange video in real time. Participant token — A token that authenticates a participant when they join a stage. Participant object — Represents participants (people) in the stage and contains information about them.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Interactive Video Service RealTime resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Interactive Video Service RealTime workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Start` operation families, including `ListCompositions`, `ListEncoderConfigurations`, `ListIngestConfigurations`, `ListParticipantEvents`, `GetComposition`, `GetEncoderConfiguration`.

## Service Identity and Protocol

- AWS model slug: `ivs-realtime`
- AWS SDK for Rust slug: `ivs`
- Model version: `2020-07-14`
- Model file: `vendor/api-models-aws/models/ivs-realtime/service/2020-07-14/ivs-realtime-2020-07-14.json`
- SDK ID: `IVS RealTime`
- Endpoint prefix: `ivsrealtime`
- ARN namespace: `ivs`
- CloudFormation name: `IVS`
- CloudTrail event source: `ivs.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (11), `Get` (8), `Create` (5), `Delete` (5), `Start` (2), `Stop` (2), `Update` (2), `Disconnect` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateEncoderConfiguration`, `CreateIngestConfiguration`, `CreateParticipantToken`, `CreateStage`, `CreateStorageConfiguration`, `DeleteEncoderConfiguration`, `DeleteIngestConfiguration`, `DeletePublicKey`, `DeleteStage`, `DeleteStorageConfiguration`, `ImportPublicKey`, `StartComposition`, `StartParticipantReplication`, `StopComposition`, `StopParticipantReplication`, `TagResource`, `UntagResource`, `UpdateIngestConfiguration`, `UpdateStage`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetComposition`, `GetEncoderConfiguration`, `GetIngestConfiguration`, `GetParticipant`, `GetPublicKey`, `GetStage`, `GetStageSession`, `GetStorageConfiguration`, `ListCompositions`, `ListEncoderConfigurations`, `ListIngestConfigurations`, `ListParticipantEvents`, `ListParticipantReplicas`, `ListParticipants`, `ListPublicKeys`, `ListStageSessions`, `ListStages`, `ListStorageConfigurations`, `ListTagsForResource`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ImportPublicKey`, `StartComposition`, `StartParticipantReplication`, `StopComposition`, `StopParticipantReplication`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 39 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EventBridge`.

## Operation Groups

### List

- Operations: `ListCompositions`, `ListEncoderConfigurations`, `ListIngestConfigurations`, `ListParticipantEvents`, `ListParticipantReplicas`, `ListParticipants`, `ListPublicKeys`, `ListStageSessions`, `ListStages`, `ListStorageConfigurations`, `ListTagsForResource`
- Traits: `paginated` (10), `readonly` (11)
- Common required input members in this group: `participantId`, `resourceArn`, `sessionId`, `sourceStageArn`, `stageArn`

### Get

- Operations: `GetComposition`, `GetEncoderConfiguration`, `GetIngestConfiguration`, `GetParticipant`, `GetPublicKey`, `GetStage`, `GetStageSession`, `GetStorageConfiguration`
- Traits: `readonly` (8)
- Common required input members in this group: `arn`, `participantId`, `sessionId`, `stageArn`

### Create

- Operations: `CreateEncoderConfiguration`, `CreateIngestConfiguration`, `CreateParticipantToken`, `CreateStage`, `CreateStorageConfiguration`
- Common required input members in this group: `ingestProtocol`, `s3`, `stageArn`

### Delete

- Operations: `DeleteEncoderConfiguration`, `DeleteIngestConfiguration`, `DeletePublicKey`, `DeleteStage`, `DeleteStorageConfiguration`
- Common required input members in this group: `arn`

### Start

- Operations: `StartComposition`, `StartParticipantReplication`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `destinationStageArn`, `destinations`, `participantId`, `sourceStageArn`, `stageArn`

### Stop

- Operations: `StopComposition`, `StopParticipantReplication`
- Common required input members in this group: `arn`, `destinationStageArn`, `participantId`, `sourceStageArn`

### Update

- Operations: `UpdateIngestConfiguration`, `UpdateStage`
- Common required input members in this group: `arn`

### Disconnect

- Operations: `DisconnectParticipant`
- Common required input members in this group: `participantId`, `stageArn`

### Import

- Operations: `ImportPublicKey`
- Common required input members in this group: `publicKeyMaterial`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateEncoderConfiguration` | `POST /CreateEncoderConfiguration` | - | - | - | `CreateEncoderConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an EncoderConfiguration object. |
| `CreateIngestConfiguration` | `POST /CreateIngestConfiguration` | - | `ingestProtocol` | - | `CreateIngestConfigurationResponse` | `AccessDeniedException`, `PendingVerification`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new IngestConfiguration resource, used to specify the ingest protocol for a stage. |
| `CreateParticipantToken` | `POST /CreateParticipantToken` | - | `stageArn` | - | `CreateParticipantTokenResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an additional token for a specified stage. This can be done after stage creation or when tokens expire. |
| `CreateStage` | `POST /CreateStage` | - | - | - | `CreateStageResponse` | `AccessDeniedException`, `PendingVerification`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new stage (and optionally participant tokens). |
| `CreateStorageConfiguration` | `POST /CreateStorageConfiguration` | - | `s3` | - | `CreateStorageConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new storage configuration, used to enable recording to Amazon S3. When a StorageConfiguration is created, IVS will modify the S3 bucketPolicy of the provided bucket. |
| `DeleteEncoderConfiguration` | `POST /DeleteEncoderConfiguration` | - | `arn` | - | `DeleteEncoderConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Deletes an EncoderConfiguration resource. Ensures that no Compositions are using this template; otherwise, returns an error. |
| `DeleteIngestConfiguration` | `POST /DeleteIngestConfiguration` | - | `arn` | - | `DeleteIngestConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Deletes a specified IngestConfiguration, so it can no longer be used to broadcast. An IngestConfiguration cannot be deleted if the publisher is actively streaming to a stage, unless `force` is set to `true`. |
| `DeletePublicKey` | `POST /DeletePublicKey` | - | `arn` | - | `DeletePublicKeyResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified public key used to sign stage participant tokens. This invalidates future participant tokens generated using the key pair’s private key. |
| `DeleteStage` | `POST /DeleteStage` | - | `arn` | - | `DeleteStageResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Shuts down and deletes the specified stage (disconnecting all participants). This operation also removes the `stageArn` from the associated IngestConfiguration, if there are participants using the IngestConfiguration to publish to the stage. |
| `DeleteStorageConfiguration` | `POST /DeleteStorageConfiguration` | - | `arn` | - | `DeleteStorageConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Deletes the storage configuration for the specified ARN. If you try to delete a storage configuration that is used by a Composition, you will get an error (409 ConflictException). |
| `DisconnectParticipant` | `POST /DisconnectParticipant` | - | `participantId`, `stageArn` | - | `DisconnectParticipantResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Disconnects a specified participant from a specified stage. If the participant is publishing using an IngestConfiguration, DisconnectParticipant also updates the `stageArn` in the IngestConfiguration to be an empty string. |
| `GetComposition` | `POST /GetComposition` | `readonly` | `arn` | - | `GetCompositionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Get information about the specified Composition resource. |
| `GetEncoderConfiguration` | `POST /GetEncoderConfiguration` | `readonly` | `arn` | - | `GetEncoderConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Gets information about the specified EncoderConfiguration resource. |
| `GetIngestConfiguration` | `POST /GetIngestConfiguration` | `readonly` | `arn` | - | `GetIngestConfigurationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets information about the specified IngestConfiguration. |
| `GetParticipant` | `POST /GetParticipant` | `readonly` | `participantId`, `sessionId`, `stageArn` | - | `GetParticipantResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets information about the specified participant token. |
| `GetPublicKey` | `POST /GetPublicKey` | `readonly` | `arn` | - | `GetPublicKeyResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets information for the specified public key. |
| `GetStage` | `POST /GetStage` | `readonly` | `arn` | - | `GetStageResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets information for the specified stage. |
| `GetStageSession` | `POST /GetStageSession` | `readonly` | `sessionId`, `stageArn` | - | `GetStageSessionResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets information for the specified stage session. |
| `GetStorageConfiguration` | `POST /GetStorageConfiguration` | `readonly` | `arn` | - | `GetStorageConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Gets the storage configuration for the specified ARN. |
| `ImportPublicKey` | `POST /ImportPublicKey` | - | `publicKeyMaterial` | - | `ImportPublicKeyResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ServiceQuotaExceededException`, `ValidationException` | Import a public key to be used for signing stage participant tokens. |
| `ListCompositions` | `POST /ListCompositions` | `readonly`, `paginated` | - | - | `ListCompositionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Gets summary information about all Compositions in your account, in the AWS region where the API request is processed. |
| `ListEncoderConfigurations` | `POST /ListEncoderConfigurations` | `readonly`, `paginated` | - | - | `ListEncoderConfigurationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Gets summary information about all EncoderConfigurations in your account, in the AWS region where the API request is processed. |
| `ListIngestConfigurations` | `POST /ListIngestConfigurations` | `readonly`, `paginated` | - | - | `ListIngestConfigurationsResponse` | `AccessDeniedException`, `ValidationException` | Lists all IngestConfigurations in your account, in the AWS region where the API request is processed. |
| `ListParticipantEvents` | `POST /ListParticipantEvents` | `readonly`, `paginated` | `participantId`, `sessionId`, `stageArn` | - | `ListParticipantEventsResponse` | `AccessDeniedException`, `ValidationException` | Lists events for a specified participant that occurred during a specified stage session. |
| `ListParticipantReplicas` | `POST /ListParticipantReplicas` | `readonly`, `paginated` | `participantId`, `sourceStageArn` | - | `ListParticipantReplicasResponse` | `AccessDeniedException`, `ValidationException` | Lists all the replicas for a participant from a source stage. |
| `ListParticipants` | `POST /ListParticipants` | `readonly`, `paginated` | `sessionId`, `stageArn` | - | `ListParticipantsResponse` | `AccessDeniedException`, `ValidationException` | Lists all participants in a specified stage session. |
| `ListPublicKeys` | `POST /ListPublicKeys` | `readonly`, `paginated` | - | - | `ListPublicKeysResponse` | `AccessDeniedException`, `ValidationException` | Gets summary information about all public keys in your account, in the AWS region where the API request is processed. |
| `ListStageSessions` | `POST /ListStageSessions` | `readonly`, `paginated` | `stageArn` | - | `ListStageSessionsResponse` | `AccessDeniedException`, `ValidationException` | Gets all sessions for a specified stage. |
| `ListStages` | `POST /ListStages` | `readonly`, `paginated` | - | - | `ListStagesResponse` | `AccessDeniedException`, `ConflictException`, `ValidationException` | Gets summary information about all stages in your account, in the AWS region where the API request is processed. |
| `ListStorageConfigurations` | `POST /ListStorageConfigurations` | `readonly`, `paginated` | - | - | `ListStorageConfigurationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Gets summary information about all storage configurations in your account, in the AWS region where the API request is processed. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about AWS tags for the specified ARN. |
| `StartComposition` | `POST /StartComposition` | `idempotency-token` | `destinations`, `stageArn` | `idempotencyToken` | `StartCompositionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Starts a Composition from a stage based on the configuration provided in the request. A Composition is an ephemeral resource that exists after this operation returns successfully. |
| `StartParticipantReplication` | `POST /StartParticipantReplication` | - | `destinationStageArn`, `participantId`, `sourceStageArn` | - | `StartParticipantReplicationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Starts replicating a publishing participant from a source stage to a destination stage. |
| `StopComposition` | `POST /StopComposition` | - | `arn` | - | `StopCompositionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Stops and deletes a Composition resource. Any broadcast from the Composition resource is stopped. |
| `StopParticipantReplication` | `POST /StopParticipantReplication` | - | `destinationStageArn`, `participantId`, `sourceStageArn` | - | `StopParticipantReplicationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Stops a replicated participant session. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds or updates tags for the AWS resource with the specified ARN. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from the resource with the specified ARN. |
| `UpdateIngestConfiguration` | `POST /UpdateIngestConfiguration` | - | `arn` | - | `UpdateIngestConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Updates a specified IngestConfiguration. Only the stage ARN attached to the IngestConfiguration can be updated. |
| `UpdateStage` | `POST /UpdateStage` | - | `arn` | - | `UpdateStageResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates a stage’s configuration. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `accessControlAllowOrigin`, `accessControlExposeHeaders`, `cacheControl`, `contentSecurityPolicy`, `exceptionMessage`, `strictTransportSecurity`, `xAmznErrorType`, `xContentTypeOptions`, `xFrameOptions` | - |
| `AccessDeniedException` | `structure` | `accessControlAllowOrigin`, `accessControlExposeHeaders`, `cacheControl`, `contentSecurityPolicy`, `exceptionMessage`, `strictTransportSecurity`, `xAmznErrorType`, `xContentTypeOptions`, `xFrameOptions` | - |
| `ResourceNotFoundException` | `structure` | `accessControlAllowOrigin`, `accessControlExposeHeaders`, `cacheControl`, `contentSecurityPolicy`, `exceptionMessage`, `strictTransportSecurity`, `xAmznErrorType`, `xContentTypeOptions`, `xFrameOptions` | - |
| `ConflictException` | `structure` | `accessControlAllowOrigin`, `accessControlExposeHeaders`, `cacheControl`, `contentSecurityPolicy`, `exceptionMessage`, `strictTransportSecurity`, `xAmznErrorType`, `xContentTypeOptions`, `xFrameOptions` | - |
| `ServiceQuotaExceededException` | `structure` | `accessControlAllowOrigin`, `accessControlExposeHeaders`, `cacheControl`, `contentSecurityPolicy`, `exceptionMessage`, `strictTransportSecurity`, `xAmznErrorType`, `xContentTypeOptions`, `xFrameOptions` | - |
| `InternalServerException` | `structure` | `accessControlAllowOrigin`, `accessControlExposeHeaders`, `cacheControl`, `contentSecurityPolicy`, `exceptionMessage`, `strictTransportSecurity`, `xAmznErrorType`, `xContentTypeOptions`, `xFrameOptions` | - |
| `PendingVerification` | `structure` | `accessControlAllowOrigin`, `accessControlExposeHeaders`, `cacheControl`, `contentSecurityPolicy`, `exceptionMessage`, `strictTransportSecurity`, `xAmznErrorType`, `xContentTypeOptions`, `xFrameOptions` | - |
| `CreateEncoderConfigurationRequest` | `structure` | `name`, `tags`, `video` | - |
| `CreateEncoderConfigurationResponse` | `structure` | `encoderConfiguration` | - |
| `CreateIngestConfigurationRequest` | `structure` | `attributes`, `ingestProtocol`, `insecureIngest`, `name`, `stageArn`, `tags`, `userId` | - |
| `CreateIngestConfigurationResponse` | `structure` | `ingestConfiguration` | - |
| `CreateParticipantTokenRequest` | `structure` | `attributes`, `capabilities`, `duration`, `stageArn`, `userId` | - |
| `CreateParticipantTokenResponse` | `structure` | `participantToken` | - |
| `CreateStageRequest` | `structure` | `autoParticipantRecordingConfiguration`, `name`, `participantTokenConfigurations`, `tags` | - |
| `CreateStageResponse` | `structure` | `participantTokens`, `stage` | - |
| `CreateStorageConfigurationRequest` | `structure` | `name`, `s3`, `tags` | - |
| `CreateStorageConfigurationResponse` | `structure` | `storageConfiguration` | - |
| `DeleteEncoderConfigurationRequest` | `structure` | `arn` | - |
| `DeleteEncoderConfigurationResponse` | `structure` | - | - |
| `DeleteIngestConfigurationRequest` | `structure` | `arn`, `force` | - |
| `DeleteIngestConfigurationResponse` | `structure` | - | - |
| `DeletePublicKeyRequest` | `structure` | `arn` | - |
| `DeletePublicKeyResponse` | `structure` | - | - |
| `DeleteStageRequest` | `structure` | `arn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
