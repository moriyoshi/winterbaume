# Connect Health

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Health Agent for healthcare providers and patient engagement

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Connect Health where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: add full state-machine walks for Connect Health resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Connect Health workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Get`, `List`, `Create`, `Start`, `Activate` operation families, including `GetDomain`, `GetMedicalScribeListeningSession`, `GetPatientInsightsJob`, `GetSubscription`, `ListDomains`, `ListSubscriptions`.

## Service Identity and Protocol

- AWS model slug: `connecthealth`
- AWS SDK for Rust slug: `connecthealth`
- Model version: `2025-01-29`
- Model file: `vendor/api-models-aws/models/connecthealth/service/2025-01-29/connecthealth-2025-01-29.json`
- SDK ID: `ConnectHealth`
- Endpoint prefix: `health-agent`
- ARN namespace: `health-agent`
- CloudFormation name: `-`
- CloudTrail event source: `health-agent.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `List` (3), `Create` (2), `Start` (2), `Activate` (1), `Deactivate` (1), `Delete` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDomain`, `CreateSubscription`, `DeleteDomain`, `StartMedicalScribeListeningSession`, `StartPatientInsightsJob`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDomain`, `GetMedicalScribeListeningSession`, `GetPatientInsightsJob`, `GetSubscription`, `ListDomains`, `ListSubscriptions`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetPatientInsightsJob`, `StartMedicalScribeListeningSession`, `StartPatientInsightsJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 12 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetDomain`, `GetMedicalScribeListeningSession`, `GetPatientInsightsJob`, `GetSubscription`
- Traits: `endpoint-bound` (2), `readonly` (4)
- Common required input members in this group: `domainId`, `jobId`, `sessionId`, `subscriptionId`

### List

- Operations: `ListDomains`, `ListSubscriptions`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `domainId`, `resourceArn`

### Create

- Operations: `CreateDomain`, `CreateSubscription`
- Traits: `idempotent` (1)
- Common required input members in this group: `domainId`, `name`

### Start

- Operations: `StartMedicalScribeListeningSession`, `StartPatientInsightsJob`
- Traits: `endpoint-bound` (2), `idempotency-token` (1)
- Common required input members in this group: `domainId`, `encounterContext`, `inputDataConfig`, `insightsContext`, `languageCode`, `mediaEncoding`, `mediaSampleRateHertz`, `outputDataConfig`, `patientContext`, `sessionId`, `subscriptionId`, `userContext`

### Activate

- Operations: `ActivateSubscription`
- Common required input members in this group: `domainId`, `subscriptionId`

### Deactivate

- Operations: `DeactivateSubscription`
- Traits: `idempotent` (1)
- Common required input members in this group: `domainId`, `subscriptionId`

### Delete

- Operations: `DeleteDomain`
- Traits: `idempotent` (1)
- Common required input members in this group: `domainId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ActivateSubscription` | `POST /domains/{domainId}/subscriptions/{subscriptionId}/activate` | - | `domainId`, `subscriptionId` | - | `ActivateSubscriptionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Activates a Subscription to enable billing for a user. |
| `CreateDomain` | `POST /domain` | `idempotent` | `name` | - | `CreateDomainOutput` | `ServiceQuotaExceededException` | Creates a new Domain for managing HealthAgent resources. |
| `CreateSubscription` | `POST /domains/{domainId}/subscriptions` | - | `domainId` | - | `CreateSubscriptionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new Subscription within a Domain for billing and user management. |
| `DeactivateSubscription` | `POST /domains/{domainId}/subscriptions/{subscriptionId}/deactivate` | `idempotent` | `domainId`, `subscriptionId` | - | `DeactivateSubscriptionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deactivates a Subscription to stop billing for a user. |
| `DeleteDomain` | `DELETE /domain/{domainId}` | `idempotent` | `domainId` | - | `DeleteDomainOutput` | `ResourceNotFoundException` | Deletes a Domain and all associated resources. |
| `GetDomain` | `GET /domain/{domainId}` | `readonly` | `domainId` | - | `GetDomainOutput` | `ResourceNotFoundException` | Retrieves information about a Domain. |
| `GetMedicalScribeListeningSession` | `GET /medical-scribe-stream/domain/{domainId}/subscription/{subscriptionId}/session/{sessionId}` | `readonly`, `endpoint-bound` | `domainId`, `sessionId`, `subscriptionId` | - | `GetMedicalScribeListeningSessionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Retrieves details about an existing Medical Scribe listening session |
| `GetPatientInsightsJob` | `GET /domain/{domainId}/patient-insights-job/{jobId}` | `readonly`, `endpoint-bound` | `domainId`, `jobId` | - | `GetPatientInsightsJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get details of a started patient insights job. |
| `GetSubscription` | `GET /domains/{domainId}/subscriptions/{subscriptionId}` | `readonly` | `domainId`, `subscriptionId` | - | `GetSubscriptionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves information about a Subscription. |
| `ListDomains` | `GET /domain` | `readonly`, `paginated` | - | - | `ListDomainsOutput` | - | Lists Domains for a given account. |
| `ListSubscriptions` | `GET /domains/{domainId}/subscriptions` | `readonly`, `paginated` | `domainId` | - | `ListSubscriptionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all Subscriptions within a Domain. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | - | Lists the tags associated with the specified resource |
| `StartMedicalScribeListeningSession` | `POST /medical-scribe-stream/` | `endpoint-bound` | `domainId`, `languageCode`, `mediaEncoding`, `mediaSampleRateHertz`, `sessionId`, `subscriptionId` | - | `StartMedicalScribeListeningSessionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a new Medical Scribe listening session for real-time audio transcription |
| `StartPatientInsightsJob` | `POST /domain/{domainId}/patient-insights-job` | `endpoint-bound`, `idempotency-token` | `domainId`, `encounterContext`, `inputDataConfig`, `insightsContext`, `outputDataConfig`, `patientContext`, `userContext` | `clientToken` | `StartPatientInsightsJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a new patient insights job. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `Unit` | - | Associates the specified tags with the specified resource |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | - | Removes the specified tags from the specified resource |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message` | This error is thrown when the requested resource is not found. |
| `AccessDeniedException` | `structure` | `message` | This error is thrown when the client does not supply proper credentials to the API. |
| `InternalServerException` | `structure` | `message` | This error is thrown when a transient error causes our API to fail. |
| `ValidationException` | `structure` | `message` | This error is thrown when the client supplies invalid input to the API. |
| `ServiceQuotaExceededException` | `structure` | `message` | The request exceeds a service quota. |
| `ThrottlingException` | `structure` | `message` | This error is thrown when the client exceeds the allowed request rate. |
| `ActivateSubscriptionInput` | `structure` | `domainId`, `subscriptionId` | - |
| `ActivateSubscriptionOutput` | `structure` | `subscription` | - |
| `CreateDomainInput` | `structure` | `kmsKeyArn`, `name`, `tags`, `webAppSetupConfiguration` | - |
| `CreateDomainOutput` | `structure` | `arn`, `createdAt`, `domainId`, `encryptionContext`, `kmsKeyArn`, `name`, `status`, `webAppConfiguration`, `webAppUrl` | - |
| `CreateSubscriptionInput` | `structure` | `domainId` | - |
| `CreateSubscriptionOutput` | `structure` | `activatedAt`, `arn`, `createdAt`, `deactivatedAt`, `domainId`, `lastUpdatedAt`, `status`, `subscriptionId` | - |
| `DeactivateSubscriptionInput` | `structure` | `domainId`, `subscriptionId` | - |
| `DeactivateSubscriptionOutput` | `structure` | `subscription` | - |
| `DeleteDomainInput` | `structure` | `domainId` | - |
| `DeleteDomainOutput` | `structure` | `arn`, `domainId`, `status` | - |
| `GetDomainInput` | `structure` | `domainId` | - |
| `GetDomainOutput` | `structure` | `arn`, `createdAt`, `domainId`, `encryptionContext`, `kmsKeyArn`, `name`, `status`, `tags`, `webAppConfiguration`, `webAppUrl` | - |
| `GetMedicalScribeListeningSessionInput` | `structure` | `domainId`, `sessionId`, `subscriptionId` | - |
| `GetMedicalScribeListeningSessionOutput` | `structure` | `medicalScribeListeningSessionDetails` | - |
| `GetPatientInsightsJobRequest` | `structure` | `domainId`, `jobId` | - |
| `GetPatientInsightsJobResponse` | `structure` | `creationTime`, `encounterContext`, `inputDataConfig`, `insightsContext`, `insightsOutput`, `jobArn`, `jobId`, `jobStatus`, `outputDataConfig`, `patientContext`, `statusDetails`, `updatedTime`, ... (+1) | - |
| `GetSubscriptionInput` | `structure` | `domainId`, `subscriptionId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
