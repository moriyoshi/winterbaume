# AWS Signer

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Signer is a fully managed code-signing service to help you ensure the trust and integrity of your code. Signer supports the following applications: With code signing for AWS Lambda, you can sign AWS Lambda deployment packages. Integrated support is provided for Amazon S3, Amazon CloudWatch, and AWS CloudTrail. In order to sign code, you create a signing profile and then use Signer to sign Lambda zip files in S3. With code signing for IoT, you can sign code for any IoT device that is supported by AWS.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Signer resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Signer workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Get`, `Revoke`, `Add`, `Cancel` operation families, including `ListProfilePermissions`, `ListSigningJobs`, `ListSigningPlatforms`, `ListSigningProfiles`, `GetRevocationStatus`, `GetSigningPlatform`.

## Service Identity and Protocol

- AWS model slug: `signer`
- AWS SDK for Rust slug: `signer`
- Model version: `2017-08-25`
- Model file: `vendor/api-models-aws/models/signer/service/2017-08-25/signer-2017-08-25.json`
- SDK ID: `signer`
- Endpoint prefix: `signer`
- ARN namespace: `signer`
- CloudFormation name: `Signer`
- CloudTrail event source: `signer.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Get` (3), `Revoke` (2), `Add` (1), `Cancel` (1), `Describe` (1), `Put` (1), `Remove` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddProfilePermission`, `CancelSigningProfile`, `PutSigningProfile`, `RemoveProfilePermission`, `RevokeSignature`, `RevokeSigningProfile`, `StartSigningJob`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeSigningJob`, `GetRevocationStatus`, `GetSigningPlatform`, `GetSigningProfile`, `ListProfilePermissions`, `ListSigningJobs`, `ListSigningPlatforms`, `ListSigningProfiles`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelSigningProfile`, `DescribeSigningJob`, `ListSigningJobs`, `StartSigningJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `Lambda`.

## Operation Groups

### List

- Operations: `ListProfilePermissions`, `ListSigningJobs`, `ListSigningPlatforms`, `ListSigningProfiles`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: -

### Get

- Operations: `GetRevocationStatus`, `GetSigningPlatform`, `GetSigningProfile`
- Common required input members in this group: `platformId`

### Revoke

- Operations: `RevokeSignature`, `RevokeSigningProfile`
- Common required input members in this group: `reason`

### Add

- Operations: `AddProfilePermission`
- Common required input members in this group: -

### Cancel

- Operations: `CancelSigningProfile`
- Common required input members in this group: -

### Describe

- Operations: `DescribeSigningJob`
- Common required input members in this group: -

### Put

- Operations: `PutSigningProfile`
- Common required input members in this group: -

### Remove

- Operations: `RemoveProfilePermission`
- Common required input members in this group: -

### Sign

- Operations: `SignPayload`
- Common required input members in this group: -

### Start

- Operations: `StartSigningJob`
- Traits: `idempotency-token` (1)
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
| `AddProfilePermission` | `POST /signing-profiles/{profileName}/permissions` | - | `profileName`, `action`, `principal`, `statementId` | - | `AddProfilePermissionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ServiceLimitExceededException`, `TooManyRequestsException`, `ValidationException` | Adds cross-account permissions to a signing profile. |
| `CancelSigningProfile` | `DELETE /signing-profiles/{profileName}` | - | `profileName` | - | `Unit` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException` | Changes the state of an ACTIVE signing profile to CANCELED . A canceled profile is still viewable with the ListSigningProfiles operation, but it cannot perform new signing jobs. See Data Retention for more informatio ... |
| `DescribeSigningJob` | `GET /signing-jobs/{jobId}` | - | `jobId` | - | `DescribeSigningJobResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException` | Returns information about a specific code signing job. You specify the job by using the jobId value that is returned by the StartSigningJob operation. |
| `GetRevocationStatus` | `GET /revocations` | - | `signatureTimestamp`, `platformId`, `profileVersionArn`, `jobArn`, `certificateHashes` | - | `GetRevocationStatusResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `TooManyRequestsException`, `ValidationException` | Retrieves the revocation status of one or more of the signing profile, signing job, and signing certificate. |
| `GetSigningPlatform` | `GET /signing-platforms/{platformId}` | - | `platformId` | - | `GetSigningPlatformResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException` | Returns information on a specific signing platform. |
| `GetSigningProfile` | `GET /signing-profiles/{profileName}` | - | `profileName` | - | `GetSigningProfileResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException` | Returns information on a specific signing profile. |
| `ListProfilePermissions` | `GET /signing-profiles/{profileName}/permissions` | - | `profileName` | - | `ListProfilePermissionsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Lists the cross-account permissions associated with a signing profile. |
| `ListSigningJobs` | `GET /signing-jobs` | `paginated` | - | - | `ListSigningJobsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `TooManyRequestsException`, `ValidationException` | Lists all your signing jobs. You can use the maxResults parameter to limit the number of signing jobs that are returned in the response. If additional jobs remain to be listed, AWS Signer returns a nextToken value. U ... |
| `ListSigningPlatforms` | `GET /signing-platforms` | `paginated` | - | - | `ListSigningPlatformsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `TooManyRequestsException`, `ValidationException` | Lists all signing platforms available in AWS Signer that match the request parameters. If additional jobs remain to be listed, Signer returns a nextToken value. Use this value in subsequent calls to ListSigningJobs t ... |
| `ListSigningProfiles` | `GET /signing-profiles` | `paginated` | - | - | `ListSigningProfilesResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `TooManyRequestsException` | Lists all available signing profiles in your AWS account. Returns only profiles with an ACTIVE status unless the includeCanceled request field is set to true . If additional jobs remain to be listed, AWS Signer retur ... |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalServiceErrorException`, `NotFoundException`, `TooManyRequestsException` | Returns a list of the tags associated with a signing profile resource. |
| `PutSigningProfile` | `PUT /signing-profiles/{profileName}` | - | `profileName`, `platformId` | - | `PutSigningProfileResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Creates a signing profile. A signing profile is a code-signing template that can be used to carry out a pre-defined signing job. |
| `RemoveProfilePermission` | `DELETE /signing-profiles/{profileName}/permissions/{statementId}` | - | `profileName`, `revisionId`, `statementId` | - | `RemoveProfilePermissionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Removes cross-account permissions from a signing profile. |
| `RevokeSignature` | `PUT /signing-jobs/{jobId}/revoke` | - | `jobId`, `reason` | - | `Unit` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Changes the state of a signing job to REVOKED . This indicates that the signature is no longer valid. |
| `RevokeSigningProfile` | `PUT /signing-profiles/{profileName}/revoke` | - | `profileName`, `profileVersion`, `reason`, `effectiveTime` | - | `Unit` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Changes the state of a signing profile to REVOKED . This indicates that signatures generated using the signing profile after an effective start date are no longer valid. A revoked profile is still viewable with the L ... |
| `SignPayload` | `POST /signing-jobs/with-payload` | - | `profileName`, `payload`, `payloadFormat` | - | `SignPayloadResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Signs a binary payload and returns a signature envelope. |
| `StartSigningJob` | `POST /signing-jobs` | `idempotency-token` | `source`, `destination`, `profileName`, `clientRequestToken` | `clientRequestToken` | `StartSigningJobResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyRequestsException`, `ValidationException` | Initiates a signing job to be performed on the code provided. Signing jobs are viewable by the ListSigningJobs operation. Note the following requirements: You must create an Amazon S3 source bucket. For more informat ... |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `InternalServiceErrorException`, `NotFoundException`, `TooManyRequestsException` | Adds one or more tags to a signing profile. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a key and an optional value. To specify the signing profile, use its Amaz ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `InternalServiceErrorException`, `NotFoundException`, `TooManyRequestsException` | Removes one or more tags from a signing profile. To remove the tags, specify a list of tag keys. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetRevocationStatus` | - | `signatureTimestamp -> signatureTimestamp`, `platformId -> platformId`, `profileVersionArn -> profileVersionArn`, `jobArn -> jobArn`, `certificateHashes -> certificateHashes` | - | - |
| `GetSigningProfile` | - | `profileOwner -> profileOwner` | - | - |
| `ListProfilePermissions` | - | `nextToken -> nextToken` | - | - |
| `ListSigningJobs` | - | `status -> status`, `platformId -> platformId`, `requestedBy -> requestedBy`, `maxResults -> maxResults`, `nextToken -> nextToken`, `isRevoked -> isRevoked`, `signatureExpiresBefore -> signatureExpiresBefore`, `signatureExpiresAfter -> signatureExpiresAfter`, `jobInvoker -> jobInvoker` | - | - |
| `ListSigningPlatforms` | - | `category -> category`, `partner -> partner`, `target -> target`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListSigningProfiles` | - | `includeCanceled -> includeCanceled`, `maxResults -> maxResults`, `nextToken -> nextToken`, `platformId -> platformId`, `statuses -> statuses` | - | - |
| `RemoveProfilePermission` | - | `revisionId -> revisionId` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, code | You do not have sufficient access to perform this action. |
| `BadRequestException` | `structure` | message, code | The request contains invalid parameters for the ARN or tags. This exception also occurs when you call a tagging API on a cancelled signing profile. |
| `ConflictException` | `structure` | message, code | The resource encountered a conflicting state. |
| `InternalServiceErrorException` | `structure` | message, code | An internal error occurred. |
| `NotFoundException` | `structure` | message, code | The signing profile was not found. |
| `ResourceNotFoundException` | `structure` | message, code | A specified resource could not be found. |
| `ServiceLimitExceededException` | `structure` | message, code | The client is making a request that exceeds service limits. |
| `ThrottlingException` | `structure` | message, code | The request was denied due to request throttling. Instead of this error, TooManyRequestsException should be used. |
| `TooManyRequestsException` | `structure` | message, code | The allowed number of job-signing requests has been exceeded. This error supersedes the error ThrottlingException . |
| `ValidationException` | `structure` | message, code | You signing certificate could not be validated. |
| `AddProfilePermissionRequest` | `structure` | profileName, profileVersion, action, principal, revisionId, statementId | - |
| `AddProfilePermissionResponse` | `structure` | revisionId | - |
| `CancelSigningProfileRequest` | `structure` | profileName | - |
| `DescribeSigningJobRequest` | `structure` | jobId | - |
| `DescribeSigningJobResponse` | `structure` | jobId, source, signingMaterial, platformId, platformDisplayName, profileName, profileVersion, overrides, signingParameters, createdAt, completedAt, signatureExpiresAt, ... (+7) | - |
| `GetRevocationStatusRequest` | `structure` | signatureTimestamp, platformId, profileVersionArn, jobArn, certificateHashes | - |
| `GetRevocationStatusResponse` | `structure` | revokedEntities | - |
| `GetSigningPlatformRequest` | `structure` | platformId | - |
| `GetSigningPlatformResponse` | `structure` | platformId, displayName, partner, target, category, signingConfiguration, signingImageFormat, maxSizeInMB, revocationSupported | - |
| `GetSigningProfileRequest` | `structure` | profileName, profileOwner | - |
| `GetSigningProfileResponse` | `structure` | profileName, profileVersion, profileVersionArn, revocationRecord, signingMaterial, platformId, platformDisplayName, signatureValidityPeriod, overrides, signingParameters, status, statusReason, ... (+2) | - |
| `ListProfilePermissionsRequest` | `structure` | profileName, nextToken | - |
| `ListProfilePermissionsResponse` | `structure` | revisionId, policySizeBytes, permissions, nextToken | - |
| `ListSigningJobsRequest` | `structure` | status, platformId, requestedBy, maxResults, nextToken, isRevoked, signatureExpiresBefore, signatureExpiresAfter, jobInvoker | - |
| `ListSigningJobsResponse` | `structure` | jobs, nextToken | - |
| `ListSigningPlatformsRequest` | `structure` | category, partner, target, maxResults, nextToken | - |
| `ListSigningPlatformsResponse` | `structure` | platforms, nextToken | - |
| `ListSigningProfilesRequest` | `structure` | includeCanceled, maxResults, nextToken, platformId, statuses | - |
| `ListSigningProfilesResponse` | `structure` | profiles, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `PutSigningProfileRequest` | `structure` | profileName, signingMaterial, signatureValidityPeriod, platformId, overrides, signingParameters, tags | - |
| `PutSigningProfileResponse` | `structure` | arn, profileVersion, profileVersionArn | - |
| `RemoveProfilePermissionRequest` | `structure` | profileName, revisionId, statementId | - |
| `RemoveProfilePermissionResponse` | `structure` | revisionId | - |
| `RevokeSignatureRequest` | `structure` | jobId, jobOwner, reason | - |
| `RevokeSigningProfileRequest` | `structure` | profileName, profileVersion, reason, effectiveTime | - |
| `SignPayloadRequest` | `structure` | profileName, profileOwner, payload, payloadFormat | - |
| `SignPayloadResponse` | `structure` | jobId, jobOwner, metadata, signature | - |
| `StartSigningJobRequest` | `structure` | source, destination, profileName, clientRequestToken, profileOwner | - |
| `Category` | `enum` | AWSIoT | - |
| `EncryptionAlgorithm` | `enum` | RSA, ECDSA | - |
| `HashAlgorithm` | `enum` | SHA1, SHA256 | - |
| `ImageFormat` | `enum` | JSON, JSONEmbedded, JSONDetached | - |
| `SigningProfileStatus` | `enum` | Active, Canceled, Revoked | - |
| `SigningStatus` | `enum` | InProgress, Failed, Succeeded | - |
| `ValidityType` | `enum` | DAYS, MONTHS, YEARS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
