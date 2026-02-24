# AppFabric

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services AppFabric quickly connects software as a service (SaaS) applications across your organization. This allows IT and security teams to easily manage and secure applications using a standard schema, and employees can complete everyday tasks faster using generative artificial intelligence (AI). You can use these APIs to complete AppFabric tasks, such as setting up audit log ingestions or viewing user access. For more information about AppFabric, including the required permissions to use the service, see the Amazon Web Services AppFabric Administration Guide. For more information about using the Command Line Interface (CLI) to manage your AppFabric resources, see the AppFabric section of the CLI Reference.

## Possible Usage Scenarios
- From the AWS documentation and model: configure app bundles, app authorisations, ingestions, and ingestion destinations for SaaS application data.
- From the operation surface: model cross-application audit ingestion, authorisation connection state, destination delivery, and tag-based bundle administration.

## Service Identity and Protocol

- AWS model slug: `appfabric`
- AWS SDK for Rust slug: `appfabric`
- Model version: `2023-05-19`
- Model file: `vendor/api-models-aws/models/appfabric/service/2023-05-19/appfabric-2023-05-19.json`
- SDK ID: `AppFabric`
- Endpoint prefix: `-`
- ARN namespace: `appfabric`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Create` (4), `Delete` (4), `Get` (4), `Start` (2), `Update` (2), `Batch` (1), `Connect` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetUserAccessTasks`, `CreateAppAuthorization`, `CreateAppBundle`, `CreateIngestion`, `CreateIngestionDestination`, `DeleteAppAuthorization`, `DeleteAppBundle`, `DeleteIngestion`, `DeleteIngestionDestination`, `StartIngestion`, `StartUserAccessTasks`, `StopIngestion`, `TagResource`, `UntagResource`, `UpdateAppAuthorization`, `UpdateIngestionDestination`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetUserAccessTasks`, `GetAppAuthorization`, `GetAppBundle`, `GetIngestion`, `GetIngestionDestination`, `ListAppAuthorizations`, `ListAppBundles`, `ListIngestionDestinations`, `ListIngestions`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchGetUserAccessTasks`, `CreateIngestion`, `CreateIngestionDestination`, `DeleteIngestion`, `DeleteIngestionDestination`, `GetIngestion`, `GetIngestionDestination`, `ListIngestionDestinations`, `ListIngestions`, `StartIngestion`, `StartUserAccessTasks`, `StopIngestion`, `UpdateIngestionDestination`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 26 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `ECS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/appfabric/latest/adminguide/getting-started-security.html
- https://docs.aws.amazon.com/appfabric/latest/adminguide/productivity-data-processing.html
- https://docs.aws.amazon.com/cli/latest/reference/appfabric/index.html

Research outcomes:
- AppFabric security setup starts with an app bundle, then app authorisations, ingestions, and ingestion destinations.
- Security ingestions normalise SaaS audit logs into OCSF format and deliver them to configured destinations.
- App authorisations represent the connection and consent state for supported SaaS applications.
- Productivity data processing handles user content with encryption, retention limits, and deletion controls.
- If a connected app is disconnected, AppFabric stops generating insights from that app.
- App bundle, app authorisation, ingestion, and destination resources have independent identifiers and lifecycles.

Parity implications:
- Model app bundles, app authorisations, ingestion jobs, ingestion destinations, destination configuration, processing configuration, and connection state separately.
- Ingestion should depend on app authorisation state and should expose asynchronous status.
- Productivity data and security-audit ingestion paths should not be conflated.

## Operation Groups

### List

- Operations: `ListAppAuthorizations`, `ListAppBundles`, `ListIngestionDestinations`, `ListIngestions`, `ListTagsForResource`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `appBundleIdentifier`, `ingestionIdentifier`, `resourceArn`

### Create

- Operations: `CreateAppAuthorization`, `CreateAppBundle`, `CreateIngestion`, `CreateIngestionDestination`
- Traits: `idempotency-token` (4), `idempotent` (4)
- Common required input members in this group: `app`, `appBundleIdentifier`, `authType`, `credential`, `destinationConfiguration`, `ingestionIdentifier`, `ingestionType`, `processingConfiguration`, `tenant`, `tenantId`

### Delete

- Operations: `DeleteAppAuthorization`, `DeleteAppBundle`, `DeleteIngestion`, `DeleteIngestionDestination`
- Traits: `idempotent` (4)
- Common required input members in this group: `appAuthorizationIdentifier`, `appBundleIdentifier`, `ingestionDestinationIdentifier`, `ingestionIdentifier`

### Get

- Operations: `GetAppAuthorization`, `GetAppBundle`, `GetIngestion`, `GetIngestionDestination`
- Traits: `readonly` (4)
- Common required input members in this group: `appAuthorizationIdentifier`, `appBundleIdentifier`, `ingestionDestinationIdentifier`, `ingestionIdentifier`

### Start

- Operations: `StartIngestion`, `StartUserAccessTasks`
- Common required input members in this group: `appBundleIdentifier`, `email`, `ingestionIdentifier`

### Update

- Operations: `UpdateAppAuthorization`, `UpdateIngestionDestination`
- Common required input members in this group: `appAuthorizationIdentifier`, `appBundleIdentifier`, `destinationConfiguration`, `ingestionDestinationIdentifier`, `ingestionIdentifier`

### Batch

- Operations: `BatchGetUserAccessTasks`
- Traits: `readonly` (1)
- Common required input members in this group: `appBundleIdentifier`, `taskIdList`

### Connect

- Operations: `ConnectAppAuthorization`
- Common required input members in this group: `appAuthorizationIdentifier`, `appBundleIdentifier`

### Stop

- Operations: `StopIngestion`
- Common required input members in this group: `appBundleIdentifier`, `ingestionIdentifier`

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
| `BatchGetUserAccessTasks` | `POST /useraccess/batchget` | `readonly` | `appBundleIdentifier`, `taskIdList` | - | `BatchGetUserAccessTasksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets user access details in a batch request. This action polls data from the tasks that are kicked off by the `StartUserAccessTasks` action. |
| `ConnectAppAuthorization` | `POST /appbundles/{appBundleIdentifier}/appauthorizations/{appAuthorizationIdentifier}/connect` | - | `appAuthorizationIdentifier`, `appBundleIdentifier` | - | `ConnectAppAuthorizationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Establishes a connection between Amazon Web Services AppFabric and an application, which allows AppFabric to call the APIs of the application. |
| `CreateAppAuthorization` | `POST /appbundles/{appBundleIdentifier}/appauthorizations` | `idempotent`, `idempotency-token` | `app`, `appBundleIdentifier`, `authType`, `credential`, `tenant` | `clientToken` | `CreateAppAuthorizationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an app authorization within an app bundle, which allows AppFabric to connect to an application. |
| `CreateAppBundle` | `POST /appbundles` | `idempotent`, `idempotency-token` | - | `clientToken` | `CreateAppBundleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an app bundle to collect data from an application using AppFabric. |
| `CreateIngestion` | `POST /appbundles/{appBundleIdentifier}/ingestions` | `idempotent`, `idempotency-token` | `app`, `appBundleIdentifier`, `ingestionType`, `tenantId` | `clientToken` | `CreateIngestionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a data ingestion for an application. |
| `CreateIngestionDestination` | `POST /appbundles/{appBundleIdentifier}/ingestions/{ingestionIdentifier}/ingestiondestinations` | `idempotent`, `idempotency-token` | `appBundleIdentifier`, `destinationConfiguration`, `ingestionIdentifier`, `processingConfiguration` | `clientToken` | `CreateIngestionDestinationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an ingestion destination, which specifies how an application's ingested data is processed by Amazon Web Services AppFabric and where it's delivered. |
| `DeleteAppAuthorization` | `DELETE /appbundles/{appBundleIdentifier}/appauthorizations/{appAuthorizationIdentifier}` | `idempotent` | `appAuthorizationIdentifier`, `appBundleIdentifier` | - | `DeleteAppAuthorizationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an app authorization. You must delete the associated ingestion before you can delete an app authorization. |
| `DeleteAppBundle` | `DELETE /appbundles/{appBundleIdentifier}` | `idempotent` | `appBundleIdentifier` | - | `DeleteAppBundleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an app bundle. You must delete all associated app authorizations before you can delete an app bundle. |
| `DeleteIngestion` | `DELETE /appbundles/{appBundleIdentifier}/ingestions/{ingestionIdentifier}` | `idempotent` | `appBundleIdentifier`, `ingestionIdentifier` | - | `DeleteIngestionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an ingestion. You must stop (disable) the ingestion and you must delete all associated ingestion destinations before you can delete an app ingestion. |
| `DeleteIngestionDestination` | `DELETE /appbundles/{appBundleIdentifier}/ingestions/{ingestionIdentifier}/ingestiondestinations/{ingestionDestinationIdentifier}` | `idempotent` | `appBundleIdentifier`, `ingestionDestinationIdentifier`, `ingestionIdentifier` | - | `DeleteIngestionDestinationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an ingestion destination. This deletes the association between an ingestion and it's destination. |
| `GetAppAuthorization` | `GET /appbundles/{appBundleIdentifier}/appauthorizations/{appAuthorizationIdentifier}` | `readonly` | `appAuthorizationIdentifier`, `appBundleIdentifier` | - | `GetAppAuthorizationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about an app authorization. |
| `GetAppBundle` | `GET /appbundles/{appBundleIdentifier}` | `readonly` | `appBundleIdentifier` | - | `GetAppBundleResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about an app bundle. |
| `GetIngestion` | `GET /appbundles/{appBundleIdentifier}/ingestions/{ingestionIdentifier}` | `readonly` | `appBundleIdentifier`, `ingestionIdentifier` | - | `GetIngestionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about an ingestion. |
| `GetIngestionDestination` | `GET /appbundles/{appBundleIdentifier}/ingestions/{ingestionIdentifier}/ingestiondestinations/{ingestionDestinationIdentifier}` | `readonly` | `appBundleIdentifier`, `ingestionDestinationIdentifier`, `ingestionIdentifier` | - | `GetIngestionDestinationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about an ingestion destination. |
| `ListAppAuthorizations` | `GET /appbundles/{appBundleIdentifier}/appauthorizations` | `readonly`, `paginated` | `appBundleIdentifier` | - | `ListAppAuthorizationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all app authorizations configured for an app bundle. |
| `ListAppBundles` | `GET /appbundles` | `readonly`, `paginated` | - | - | `ListAppBundlesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of app bundles. |
| `ListIngestionDestinations` | `GET /appbundles/{appBundleIdentifier}/ingestions/{ingestionIdentifier}/ingestiondestinations` | `readonly`, `paginated` | `appBundleIdentifier`, `ingestionIdentifier` | - | `ListIngestionDestinationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all ingestion destinations configured for an ingestion. |
| `ListIngestions` | `GET /appbundles/{appBundleIdentifier}/ingestions` | `readonly`, `paginated` | `appBundleIdentifier` | - | `ListIngestionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all ingestions configured for an app bundle. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of tags for a resource. |
| `StartIngestion` | `POST /appbundles/{appBundleIdentifier}/ingestions/{ingestionIdentifier}/start` | - | `appBundleIdentifier`, `ingestionIdentifier` | - | `StartIngestionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts (enables) an ingestion, which collects data from an application. |
| `StartUserAccessTasks` | `POST /useraccess/start` | - | `appBundleIdentifier`, `email` | - | `StartUserAccessTasksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts the tasks to search user access status for a specific email address. The tasks are stopped when the user access status data is found. |
| `StopIngestion` | `POST /appbundles/{appBundleIdentifier}/ingestions/{ingestionIdentifier}/stop` | - | `appBundleIdentifier`, `ingestionIdentifier` | - | `StopIngestionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops (disables) an ingestion. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or tags from a resource. |
| `UpdateAppAuthorization` | `PATCH /appbundles/{appBundleIdentifier}/appauthorizations/{appAuthorizationIdentifier}` | - | `appAuthorizationIdentifier`, `appBundleIdentifier` | - | `UpdateAppAuthorizationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an app authorization within an app bundle, which allows AppFabric to connect to an application. If the app authorization was in a `connected` state, updating the app authorization will set it back to a `PendingConnect` state. |
| `UpdateIngestionDestination` | `PATCH /appbundles/{appBundleIdentifier}/ingestions/{ingestionIdentifier}/ingestiondestinations/{ingestionDestinationIdentifier}` | - | `appBundleIdentifier`, `destinationConfiguration`, `ingestionDestinationIdentifier`, `ingestionIdentifier` | - | `UpdateIngestionDestinationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an ingestion destination, which specifies how an application's ingested data is processed by Amazon Web Services AppFabric and where it's delivered. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You are not authorized to perform this operation. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | The request processing has failed because of an unknown error, exception, or failure with an internal server. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request rate exceeds the limit. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The request has invalid or missing parameters. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The specified resource does not exist. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The request has created a conflict. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The request exceeds a service quota. |
| `BatchGetUserAccessTasksRequest` | `structure` | `appBundleIdentifier`, `taskIdList` | - |
| `BatchGetUserAccessTasksResponse` | `structure` | `userAccessResultsList` | - |
| `ConnectAppAuthorizationRequest` | `structure` | `appAuthorizationIdentifier`, `appBundleIdentifier`, `authRequest` | - |
| `ConnectAppAuthorizationResponse` | `structure` | `appAuthorizationSummary` | - |
| `CreateAppAuthorizationRequest` | `structure` | `app`, `appBundleIdentifier`, `authType`, `clientToken`, `credential`, `tags`, `tenant` | - |
| `CreateAppAuthorizationResponse` | `structure` | `appAuthorization` | - |
| `CreateAppBundleRequest` | `structure` | `clientToken`, `customerManagedKeyIdentifier`, `tags` | - |
| `CreateAppBundleResponse` | `structure` | `appBundle` | - |
| `CreateIngestionRequest` | `structure` | `app`, `appBundleIdentifier`, `clientToken`, `ingestionType`, `tags`, `tenantId` | - |
| `CreateIngestionResponse` | `structure` | `ingestion` | - |
| `CreateIngestionDestinationRequest` | `structure` | `appBundleIdentifier`, `clientToken`, `destinationConfiguration`, `ingestionIdentifier`, `processingConfiguration`, `tags` | - |
| `CreateIngestionDestinationResponse` | `structure` | `ingestionDestination` | - |
| `DeleteAppAuthorizationRequest` | `structure` | `appAuthorizationIdentifier`, `appBundleIdentifier` | - |
| `DeleteAppAuthorizationResponse` | `structure` | - | - |
| `DeleteAppBundleRequest` | `structure` | `appBundleIdentifier` | - |
| `DeleteAppBundleResponse` | `structure` | - | - |
| `DeleteIngestionRequest` | `structure` | `appBundleIdentifier`, `ingestionIdentifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
