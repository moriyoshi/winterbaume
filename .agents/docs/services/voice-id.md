# Amazon Voice ID

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Connect Voice ID provides real-time caller authentication and fraud risk detection, which make voice interactions in contact centers more secure and efficient.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Voice ID where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Voice ID by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon Voice ID resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Voice ID workflows in the local mock. Key resources include `DomainResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Delete`, `Create`, `Start` operation families, including `ListDomains`, `ListFraudsterRegistrationJobs`, `ListFraudsters`, `ListSpeakerEnrollmentJobs`, `DescribeDomain`, `DescribeFraudster`.

## Service Identity and Protocol

- AWS model slug: `voice-id`
- AWS SDK for Rust slug: `voiceid`
- Model version: `2021-09-27`
- Model file: `vendor/api-models-aws/models/voice-id/service/2021-09-27/voice-id-2021-09-27.json`
- SDK ID: `Voice ID`
- Endpoint prefix: `voiceid`
- ARN namespace: `voiceid`
- CloudFormation name: `VoiceID`
- CloudTrail event source: `voiceid.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Describe` (6), `Delete` (4), `Create` (2), `Start` (2), `Update` (2), `Associate` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateFraudster`, `CreateDomain`, `CreateWatchlist`, `DeleteDomain`, `DeleteFraudster`, `DeleteSpeaker`, `DeleteWatchlist`, `DisassociateFraudster`, `StartFraudsterRegistrationJob`, `StartSpeakerEnrollmentJob`, `TagResource`, `UntagResource`, `UpdateDomain`, `UpdateWatchlist`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDomain`, `DescribeFraudster`, `DescribeFraudsterRegistrationJob`, `DescribeSpeaker`, `DescribeSpeakerEnrollmentJob`, `DescribeWatchlist`, `ListDomains`, `ListFraudsterRegistrationJobs`, `ListFraudsters`, `ListSpeakerEnrollmentJobs`, `ListSpeakers`, `ListTagsForResource`, `ListWatchlists`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeFraudsterRegistrationJob`, `DescribeSpeakerEnrollmentJob`, `ListFraudsterRegistrationJobs`, `ListSpeakerEnrollmentJobs`, `StartFraudsterRegistrationJob`, `StartSpeakerEnrollmentJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 29 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `KMS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `DomainResource` | `DomainId` | create: `CreateDomain`; read: `DescribeDomain`; update: `UpdateDomain`; delete: `DeleteDomain`; list: `ListDomains` | - | - |
## Operation Groups

### List

- Operations: `ListDomains`, `ListFraudsterRegistrationJobs`, `ListFraudsters`, `ListSpeakerEnrollmentJobs`, `ListSpeakers`, `ListTagsForResource`, `ListWatchlists`
- Traits: `paginated` (6), `readonly` (1)
- Common required input members in this group: `DomainId`, `ResourceArn`

### Describe

- Operations: `DescribeDomain`, `DescribeFraudster`, `DescribeFraudsterRegistrationJob`, `DescribeSpeaker`, `DescribeSpeakerEnrollmentJob`, `DescribeWatchlist`
- Traits: `readonly` (1)
- Common required input members in this group: `DomainId`, `FraudsterId`, `JobId`, `SpeakerId`, `WatchlistId`

### Delete

- Operations: `DeleteDomain`, `DeleteFraudster`, `DeleteSpeaker`, `DeleteWatchlist`
- Traits: `idempotent` (1)
- Common required input members in this group: `DomainId`, `FraudsterId`, `SpeakerId`, `WatchlistId`

### Create

- Operations: `CreateDomain`, `CreateWatchlist`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `DomainId`, `Name`, `ServerSideEncryptionConfiguration`

### Start

- Operations: `StartFraudsterRegistrationJob`, `StartSpeakerEnrollmentJob`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `DataAccessRoleArn`, `DomainId`, `InputDataConfig`, `OutputDataConfig`

### Update

- Operations: `UpdateDomain`, `UpdateWatchlist`
- Common required input members in this group: `DomainId`, `Name`, `ServerSideEncryptionConfiguration`, `WatchlistId`

### Associate

- Operations: `AssociateFraudster`
- Common required input members in this group: `DomainId`, `FraudsterId`, `WatchlistId`

### Disassociate

- Operations: `DisassociateFraudster`
- Common required input members in this group: `DomainId`, `FraudsterId`, `WatchlistId`

### Evaluate

- Operations: `EvaluateSession`
- Common required input members in this group: `DomainId`, `SessionNameOrId`

### Opt

- Operations: `OptOutSpeaker`
- Common required input members in this group: `DomainId`, `SpeakerId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateFraudster` | - | - | `DomainId`, `FraudsterId`, `WatchlistId` | - | `AssociateFraudsterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates the fraudsters with the watchlist specified in the same domain. |
| `CreateDomain` | - | `idempotent`, `idempotency-token` | `Name`, `ServerSideEncryptionConfiguration` | `ClientToken` | `CreateDomainResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a domain that contains all Amazon Connect Voice ID data, such as speakers, fraudsters, customer audio, and voiceprints. Every domain is created with a default watchlist that fraudsters can be a part of. |
| `CreateWatchlist` | - | `idempotent`, `idempotency-token` | `DomainId`, `Name` | `ClientToken` | `CreateWatchlistResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a watchlist that fraudsters can be a part of. |
| `DeleteDomain` | - | `idempotent` | `DomainId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified domain from Voice ID. |
| `DeleteFraudster` | - | - | `DomainId`, `FraudsterId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified fraudster from Voice ID. This action disassociates the fraudster from any watchlists it is a part of. |
| `DeleteSpeaker` | - | - | `DomainId`, `SpeakerId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified speaker from Voice ID. |
| `DeleteWatchlist` | - | - | `DomainId`, `WatchlistId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified watchlist from Voice ID. This API throws an exception when there are fraudsters in the watchlist that you are trying to delete. |
| `DescribeDomain` | - | `readonly` | `DomainId` | - | `DescribeDomainResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified domain. |
| `DescribeFraudster` | - | - | `DomainId`, `FraudsterId` | - | `DescribeFraudsterResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified fraudster. |
| `DescribeFraudsterRegistrationJob` | - | - | `DomainId`, `JobId` | - | `DescribeFraudsterRegistrationJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified fraudster registration job. |
| `DescribeSpeaker` | - | - | `DomainId`, `SpeakerId` | - | `DescribeSpeakerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified speaker. |
| `DescribeSpeakerEnrollmentJob` | - | - | `DomainId`, `JobId` | - | `DescribeSpeakerEnrollmentJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified speaker enrollment job. |
| `DescribeWatchlist` | - | - | `DomainId`, `WatchlistId` | - | `DescribeWatchlistResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified watchlist. |
| `DisassociateFraudster` | - | - | `DomainId`, `FraudsterId`, `WatchlistId` | - | `DisassociateFraudsterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates the fraudsters from the watchlist specified. Voice ID always expects a fraudster to be a part of at least one watchlist. |
| `EvaluateSession` | - | - | `DomainId`, `SessionNameOrId` | - | `EvaluateSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Evaluates a specified session based on audio data accumulated during a streaming Amazon Connect Voice ID call. |
| `ListDomains` | - | `readonly`, `paginated` | - | - | `ListDomainsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all the domains in the Amazon Web Services account. |
| `ListFraudsterRegistrationJobs` | - | `paginated` | `DomainId` | - | `ListFraudsterRegistrationJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the fraudster registration jobs in the domain with the given `JobStatus`. If `JobStatus` is not provided, this lists all fraudster registration jobs in the given domain. |
| `ListFraudsters` | - | `paginated` | `DomainId` | - | `ListFraudstersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all fraudsters in a specified watchlist or domain. |
| `ListSpeakerEnrollmentJobs` | - | `paginated` | `DomainId` | - | `ListSpeakerEnrollmentJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the speaker enrollment jobs in the domain with the specified `JobStatus`. If `JobStatus` is not provided, this lists all jobs with all possible speaker enrollment job statuses. |
| `ListSpeakers` | - | `paginated` | `DomainId` | - | `ListSpeakersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all speakers in a specified domain. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags associated with a specified Voice ID resource. |
| `ListWatchlists` | - | `paginated` | `DomainId` | - | `ListWatchlistsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all watchlists in a specified domain. |
| `OptOutSpeaker` | - | - | `DomainId`, `SpeakerId` | - | `OptOutSpeakerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Opts out a speaker from Voice ID. A speaker can be opted out regardless of whether or not they already exist in Voice ID. |
| `StartFraudsterRegistrationJob` | - | `idempotent`, `idempotency-token` | `DataAccessRoleArn`, `DomainId`, `InputDataConfig`, `OutputDataConfig` | `ClientToken` | `StartFraudsterRegistrationJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a new batch fraudster registration job using provided details. |
| `StartSpeakerEnrollmentJob` | - | `idempotent`, `idempotency-token` | `DataAccessRoleArn`, `DomainId`, `InputDataConfig`, `OutputDataConfig` | `ClientToken` | `StartSpeakerEnrollmentJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a new batch speaker enrollment job using specified details. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tags a Voice ID resource with the provided list of tags. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes specified tags from a specified Amazon Connect Voice ID resource. |
| `UpdateDomain` | - | - | `DomainId`, `Name`, `ServerSideEncryptionConfiguration` | - | `UpdateDomainResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified domain. This API has clobber behavior, and clears and replaces all attributes. |
| `UpdateWatchlist` | - | - | `DomainId`, `WatchlistId` | - | `UpdateWatchlistResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified watchlist. Every domain has a default watchlist which cannot be updated. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient permissions to perform this action. |
| `InternalServerException` | `structure` | `Message` | The request failed due to an unknown error on the server side. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message` | The request failed one or more validations; check the error message for more details. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceType` | The specified resource cannot be found. |
| `ConflictException` | `structure` | `ConflictType`, `Message` | The request failed due to a conflict. |
| `ServiceQuotaExceededException` | `structure` | `Message` | The request exceeded the service quota. |
| `AssociateFraudsterRequest` | `structure` | `DomainId`, `FraudsterId`, `WatchlistId` | - |
| `AssociateFraudsterResponse` | `structure` | `Fraudster` | - |
| `CreateDomainRequest` | `structure` | `ClientToken`, `Description`, `Name`, `ServerSideEncryptionConfiguration`, `Tags` | - |
| `CreateDomainResponse` | `structure` | `Domain` | - |
| `CreateWatchlistRequest` | `structure` | `ClientToken`, `Description`, `DomainId`, `Name` | - |
| `CreateWatchlistResponse` | `structure` | `Watchlist` | - |
| `DeleteDomainRequest` | `structure` | `DomainId` | - |
| `DeleteFraudsterRequest` | `structure` | `DomainId`, `FraudsterId` | - |
| `DeleteSpeakerRequest` | `structure` | `DomainId`, `SpeakerId` | - |
| `DeleteWatchlistRequest` | `structure` | `DomainId`, `WatchlistId` | - |
| `DescribeDomainRequest` | `structure` | `DomainId` | - |
| `DescribeDomainResponse` | `structure` | `Domain` | - |
| `DescribeFraudsterRequest` | `structure` | `DomainId`, `FraudsterId` | - |
| `DescribeFraudsterResponse` | `structure` | `Fraudster` | - |
| `DescribeFraudsterRegistrationJobRequest` | `structure` | `DomainId`, `JobId` | - |
| `DescribeFraudsterRegistrationJobResponse` | `structure` | `Job` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
