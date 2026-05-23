# AWSDeadlineCloud

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Web Services Deadline Cloud API provides infrastructure and centralized management for your projects. Use the Deadline Cloud API to onboard users, assign projects, and attach permissions specific to their job function. With Deadline Cloud, content production teams can deploy resources for their workforce securely in the cloud, reducing the costs of added physical infrastructure. Keep your content production operations secure, while allowing your contributors to access the tools they need, such as scalable high-speed storage, licenses, and cost management services.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWSDeadlineCloud where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWSDeadlineCloud by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWSDeadlineCloud resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWSDeadlineCloud workflows in the local mock. Key resources include `BudgetResource`, `FarmResource`, `FleetResource`, `JobResource`, `LicenseEndpointResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Update`, `Create`, `Delete` operation families, including `ListAvailableMeteredProducts`, `ListBudgets`, `ListFarmMembers`, `ListFarms`, `GetBudget`, `GetFarm`.

## Service Identity and Protocol

- AWS model slug: `deadline`
- AWS SDK for Rust slug: `deadline`
- Model version: `2023-10-12`
- Model file: `vendor/api-models-aws/models/deadline/service/2023-10-12/deadline-2023-10-12.json`
- SDK ID: `deadline`
- Endpoint prefix: `-`
- ARN namespace: `deadline`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (29), `Get` (19), `Update` (16), `Create` (13), `Delete` (13), `Assume` (5), `Associate` (4), `Disassociate` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateMemberToFarm`, `AssociateMemberToFleet`, `AssociateMemberToJob`, `AssociateMemberToQueue`, `BatchGetJobEntity`, `CreateBudget`, `CreateFarm`, `CreateFleet`, `CreateJob`, `CreateLicenseEndpoint`, `CreateLimit`, `CreateMonitor`, `CreateQueue`, `CreateQueueEnvironment`, `CreateQueueFleetAssociation`, `CreateQueueLimitAssociation`, `CreateStorageProfile`, `CreateWorker`, `DeleteBudget`, `DeleteFarm`, ... (+35).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetJobEntity`, `GetBudget`, `GetFarm`, `GetFleet`, `GetJob`, `GetLicenseEndpoint`, `GetLimit`, `GetMonitor`, `GetQueue`, `GetQueueEnvironment`, `GetQueueFleetAssociation`, `GetQueueLimitAssociation`, `GetSession`, `GetSessionAction`, `GetSessionsStatisticsAggregation`, `GetStep`, `GetStorageProfile`, `GetStorageProfileForQueue`, `GetTask`, `GetWorker`, ... (+34).
- Pagination is modelled for 29 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 52 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `AssociateMemberToJob`, `BatchGetJobEntity`, `CopyJobTemplate`, `CreateJob`, `DisassociateMemberFromJob`, `GetJob`, `GetTask`, `ListJobMembers`, `ListJobParameterDefinitions`, `ListJobs`, `ListTasks`, `SearchJobs`, `SearchTasks`, `StartSessionsStatisticsAggregation`, `UpdateJob`, `UpdateTask`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 113 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SQS`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `BudgetResource` | `budgetId`, `farmId` | create: `CreateBudget`; read: `GetBudget`; update: `UpdateBudget`; delete: `DeleteBudget`; list: `ListBudgets` | - | Represents a budget that limits the expenses allowed on a queue or farm |
| `FarmResource` | `farmId` | create: `CreateFarm`; read: `GetFarm`; update: `UpdateFarm`; delete: `DeleteFarm`; list: `ListFarms` | `AssociateMemberToFarm`, `CreateLimit`, `CreateStorageProfile`, `DeleteLimit`, `DeleteStorageProfile`, `DisassociateMemberFromFarm`, `GetLimit`, `GetStorageProfile`, `ListFarmMembers`, `ListLimits`, ... (+3) | Represents a farm that contains Deadline Cloud queues and fleets |
| `FleetResource` | `farmId`, `fleetId` | create: `CreateFleet`; read: `GetFleet`; update: `UpdateFleet`; delete: `DeleteFleet`; list: `ListFleets` | `AssociateMemberToFleet`, `AssumeFleetRoleForRead`, `DisassociateMemberFromFleet`, `ListFleetMembers` | Represents a fleet of workers that process Deadline Cloud jobs |
| `JobResource` | `farmId`, `jobId`, `queueId` | create: `CreateJob`; read: `GetJob`; update: `UpdateJob`; list: `ListJobs` | `AssociateMemberToJob`, `CopyJobTemplate`, `DisassociateMemberFromJob`, `GetSession`, `GetSessionAction`, `GetStep`, `GetTask`, `ListJobMembers`, `ListJobParameterDefinitions`, `ListSessionActions`, ... (+8) | Represents a job that is composed of steps and tasks |
| `LicenseEndpointResource` | `licenseEndpointId` | create: `CreateLicenseEndpoint`; read: `GetLicenseEndpoint`; delete: `DeleteLicenseEndpoint`; list: `ListLicenseEndpoints` | `DeleteMeteredProduct`, `ListMeteredProducts`, `PutMeteredProduct` | Represents a license endpoint that is for licensed software or a product used within Deadline Cloud |
| `MonitorResource` | `monitorId` | create: `CreateMonitor`; read: `GetMonitor`; update: `UpdateMonitor`; delete: `DeleteMonitor`; list: `ListMonitors` | - | Represents a monitor that is used to manage Deadline Cloud resources |
| `QueueResource` | `farmId`, `queueId` | create: `CreateQueue`; read: `GetQueue`; update: `UpdateQueue`; delete: `DeleteQueue`; list: `ListQueues` | `AssociateMemberToQueue`, `AssumeQueueRoleForRead`, `AssumeQueueRoleForUser`, `CreateQueueEnvironment`, `DeleteQueueEnvironment`, `DisassociateMemberFromQueue`, `GetQueueEnvironment`, `GetStorageProfileForQueue`, `ListQueueEnvironments`, `ListQueueMembers`, ... (+2) | Represents a queue of jobs that will be processed by a fleet |
| `WorkerResource` | `farmId`, `fleetId`, `workerId` | create: `CreateWorker`; read: `GetWorker`; update: `UpdateWorker`; delete: `DeleteWorker`; list: `ListWorkers` | `AssumeFleetRoleForWorker`, `AssumeQueueRoleForWorker`, `BatchGetJobEntity`, `ListSessionsForWorker`, `UpdateWorkerSchedule` | Represents a worker that is part of a fleet on a farm |
## Operation Groups

### Batch

- Operations: `BatchGetJob`, `BatchGetSession`, `BatchGetSessionAction`, `BatchGetStep`, `BatchGetTask`, `BatchGetWorker`, `BatchUpdateJob`, `BatchUpdateTask`
- Traits: `readonly` (6), `idempotent` (2), `idempotency-token` (2)
- Common required input members in this group: `identifiers`

### List

- Operations: `ListAvailableMeteredProducts`, `ListQueueFleetAssociations`, `ListQueueLimitAssociations`, `ListTagsForResource`
- Traits: `readonly` (4), `paginated` (3)
- Common required input members in this group: `farmId`

### Search

- Operations: `SearchJobs`, `SearchSteps`, `SearchTasks`, `SearchWorkers`
- Traits: `readonly` (4)
- Common required input members in this group: `farmId`, `itemOffset`, `queueIds`

### Get

- Operations: `GetQueueFleetAssociation`, `GetQueueLimitAssociation`, `GetSessionsStatisticsAggregation`
- Traits: `readonly` (3), `paginated` (1)
- Common required input members in this group: `farmId`, `queueId`

### Create

- Operations: `CreateQueueFleetAssociation`, `CreateQueueLimitAssociation`
- Traits: `idempotent` (2)
- Common required input members in this group: `farmId`, `queueId`

### Delete

- Operations: `DeleteQueueFleetAssociation`, `DeleteQueueLimitAssociation`
- Traits: `idempotent` (2)
- Common required input members in this group: `farmId`, `queueId`

### Update

- Operations: `UpdateQueueFleetAssociation`, `UpdateQueueLimitAssociation`
- Traits: `idempotent` (2)
- Common required input members in this group: `farmId`, `queueId`, `status`

### Start

- Operations: `StartSessionsStatisticsAggregation`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetJob` | `POST /2023-10-12/batch-get-job` | `readonly` | `identifiers` | - | `BatchGetJobResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ThrottlingException`, `ValidationException` | Retrieves multiple jobs in a single request. This is a batch version of the GetJob API. The result of getting each job is reported individually in the response. Because the batch request can result in a combination o ... |
| `BatchGetSession` | `POST /2023-10-12/batch-get-session` | `readonly` | `identifiers` | - | `BatchGetSessionResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ThrottlingException`, `ValidationException` | Retrieves multiple sessions in a single request. This is a batch version of the GetSession API. The result of getting each session is reported individually in the response. Because the batch request can result in a c ... |
| `BatchGetSessionAction` | `POST /2023-10-12/batch-get-session-action` | `readonly` | `identifiers` | - | `BatchGetSessionActionResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ThrottlingException`, `ValidationException` | Retrieves multiple session actions in a single request. This is a batch version of the GetSessionAction API. The result of getting each session action is reported individually in the response. Because the batch reque ... |
| `BatchGetStep` | `POST /2023-10-12/batch-get-step` | `readonly` | `identifiers` | - | `BatchGetStepResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ThrottlingException`, `ValidationException` | Retrieves multiple steps in a single request. This is a batch version of the GetStep API. The result of getting each step is reported individually in the response. Because the batch request can result in a combinatio ... |
| `BatchGetTask` | `POST /2023-10-12/batch-get-task` | `readonly` | `identifiers` | - | `BatchGetTaskResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ThrottlingException`, `ValidationException` | Retrieves multiple tasks in a single request. This is a batch version of the GetTask API. The result of getting each task is reported individually in the response. Because the batch request can result in a combinatio ... |
| `BatchGetWorker` | `POST /2023-10-12/batch-get-worker` | `readonly` | `identifiers` | - | `BatchGetWorkerResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ThrottlingException`, `ValidationException` | Retrieves multiple workers in a single request. This is a batch version of the GetWorker API. The result of getting each worker is reported individually in the response. Because the batch request can result in a comb ... |
| `BatchUpdateJob` | `PATCH /2023-10-12/batch-update-job` | `idempotent`, `idempotency-token` | `jobs` | `clientToken` | `BatchUpdateJobResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ThrottlingException`, `ValidationException` | Updates multiple jobs in a single request. This is a batch version of the UpdateJob API. The result of updating each job is reported individually in the response. Because the batch request can result in a combination ... |
| `BatchUpdateTask` | `PATCH /2023-10-12/batch-update-task` | `idempotent`, `idempotency-token` | `tasks` | `clientToken` | `BatchUpdateTaskResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ThrottlingException`, `ValidationException` | Updates multiple tasks in a single request. This is a batch version of the UpdateTask API. The result of updating each task is reported individually in the response. Because the batch request can result in a combinat ... |
| `CreateQueueFleetAssociation` | `PUT /2023-10-12/farms/{farmId}/queue-fleet-associations` | `idempotent` | `farmId`, `queueId`, `fleetId` | - | `CreateQueueFleetAssociationResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an association between a queue and a fleet. |
| `CreateQueueLimitAssociation` | `PUT /2023-10-12/farms/{farmId}/queue-limit-associations` | `idempotent` | `farmId`, `queueId`, `limitId` | - | `CreateQueueLimitAssociationResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a limit with a particular queue. After the limit is associated, all workers for jobs that specify the limit associated with the queue are subject to the limit. You can't associate two limits with the same ... |
| `DeleteQueueFleetAssociation` | `DELETE /2023-10-12/farms/{farmId}/queue-fleet-associations/{queueId}/{fleetId}` | `idempotent` | `farmId`, `queueId`, `fleetId` | - | `DeleteQueueFleetAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a queue-fleet association. |
| `DeleteQueueLimitAssociation` | `DELETE /2023-10-12/farms/{farmId}/queue-limit-associations/{queueId}/{limitId}` | `idempotent` | `farmId`, `queueId`, `limitId` | - | `DeleteQueueLimitAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association between a queue and a limit. You must use the UpdateQueueLimitAssociation operation to set the status to STOP_LIMIT_USAGE_AND_COMPLETE_TASKS or STOP_LIMIT_USAGE_AND_CANCEL_TASKS . The status d ... |
| `GetQueueFleetAssociation` | `GET /2023-10-12/farms/{farmId}/queue-fleet-associations/{queueId}/{fleetId}` | `readonly` | `farmId`, `queueId`, `fleetId` | - | `GetQueueFleetAssociationResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a queue-fleet association. |
| `GetQueueLimitAssociation` | `GET /2023-10-12/farms/{farmId}/queue-limit-associations/{queueId}/{limitId}` | `readonly` | `farmId`, `queueId`, `limitId` | - | `GetQueueLimitAssociationResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a specific association between a queue and a limit. |
| `GetSessionsStatisticsAggregation` | `GET /2023-10-12/farms/{farmId}/sessions-statistics-aggregation` | `readonly`, `paginated` | `farmId`, `aggregationId` | - | `GetSessionsStatisticsAggregationResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a set of statistics for queues or farms. Before you can call the GetSessionStatisticsAggregation operation, you must first call the StartSessionsStatisticsAggregation operation. Statistics are available for 1 ho ... |
| `ListAvailableMeteredProducts` | `GET /2023-10-12/metered-products` | `readonly`, `paginated` | - | - | `ListAvailableMeteredProductsResponse` | `InternalServerErrorException`, `ThrottlingException` | A list of the available metered products. |
| `ListQueueFleetAssociations` | `GET /2023-10-12/farms/{farmId}/queue-fleet-associations` | `readonly`, `paginated` | `farmId` | - | `ListQueueFleetAssociationsResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException` | Lists queue-fleet associations. |
| `ListQueueLimitAssociations` | `GET /2023-10-12/farms/{farmId}/queue-limit-associations` | `readonly`, `paginated` | `farmId` | - | `ListQueueLimitAssociationsResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException` | Gets a list of the associations between queues and limits defined in a farm. |
| `ListTagsForResource` | `GET /2023-10-12/tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags for a resource. |
| `SearchJobs` | `POST /2023-10-12/farms/{farmId}/search/jobs` | `readonly` | `farmId`, `itemOffset`, `queueIds` | - | `SearchJobsResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Searches for jobs. |
| `SearchSteps` | `POST /2023-10-12/farms/{farmId}/search/steps` | `readonly` | `farmId`, `itemOffset`, `queueIds` | - | `SearchStepsResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Searches for steps. |
| `SearchTasks` | `POST /2023-10-12/farms/{farmId}/search/tasks` | `readonly` | `farmId`, `itemOffset`, `queueIds` | - | `SearchTasksResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Searches for tasks. |
| `SearchWorkers` | `POST /2023-10-12/farms/{farmId}/search/workers` | `readonly` | `farmId`, `itemOffset`, `fleetIds` | - | `SearchWorkersResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Searches for workers. |
| `StartSessionsStatisticsAggregation` | `POST /2023-10-12/farms/{farmId}/sessions-statistics-aggregation` | `readonly` | `farmId`, `resourceIds`, `startTime`, `endTime`, `groupBy`, `statistics` | - | `StartSessionsStatisticsAggregationResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts an asynchronous request for getting aggregated statistics about queues and farms. Get the statistics using the GetSessionsStatisticsAggregation operation. You can only have one running aggregation for your Dea ... |
| `TagResource` | `POST /2023-10-12/tags/{resourceArn}` | - | `resourceArn` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tags a resource using the resource's ARN and desired tags. |
| `UntagResource` | `DELETE /2023-10-12/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag from a resource using the resource's ARN and tag to remove. |
| `UpdateQueueFleetAssociation` | `PATCH /2023-10-12/farms/{farmId}/queue-fleet-associations/{queueId}/{fleetId}` | `idempotent` | `farmId`, `queueId`, `fleetId`, `status` | - | `UpdateQueueFleetAssociationResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a queue-fleet association. |
| `UpdateQueueLimitAssociation` | `PATCH /2023-10-12/farms/{farmId}/queue-limit-associations/{queueId}/{limitId}` | `idempotent` | `farmId`, `queueId`, `limitId`, `status` | - | `UpdateQueueLimitAssociationResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the status of the queue. If you set the status to one of the STOP_LIMIT_USAGE* values, there will be a delay before the status transitions to the STOPPED state. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `BatchUpdateJob` | `clientToken -> X-Amz-Client-Token` | - | - | - |
| `BatchUpdateTask` | `clientToken -> X-Amz-Client-Token` | - | - | - |
| `GetSessionsStatisticsAggregation` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `aggregationId -> aggregationId` | - | - |
| `ListAvailableMeteredProducts` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListQueueFleetAssociations` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `queueId -> queueId`, `fleetId -> fleetId` | - | - |
| `ListQueueLimitAssociations` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `queueId -> queueId`, `limitId -> limitId` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, context | You don't have permission to perform the action. |
| `ConflictException` | `structure` | message, reason, resourceId, resourceType, context | Your request has conflicting operations. This can occur if you're trying to perform more than one operation on the same resource at the same time. |
| `InternalServerErrorException` | `structure` | message, retryAfterSeconds | Deadline Cloud can't process your request right now. Try again later. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType, context | The requested resource can't be found. |
| `ServiceQuotaExceededException` | `structure` | message, reason, resourceType, serviceCode, quotaCode, resourceId, context | You exceeded your service quota. Service quotas, also referred to as limits, are the maximum number of service resources or operations for your Amazon Web S ... |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds, context | Your request exceeded a request rate quota. |
| `ValidationException` | `structure` | message, reason, fieldList, context | The request isn't valid. This can occur if your request contains malformed JSON or unsupported characters. |
| `BatchGetJobRequest` | `structure` | identifiers | - |
| `BatchGetJobResponse` | `structure` | jobs, errors | - |
| `BatchGetSessionRequest` | `structure` | identifiers | - |
| `BatchGetSessionResponse` | `structure` | sessions, errors | - |
| `BatchGetSessionActionRequest` | `structure` | identifiers | - |
| `BatchGetSessionActionResponse` | `structure` | sessionActions, errors | - |
| `BatchGetStepRequest` | `structure` | identifiers | - |
| `BatchGetStepResponse` | `structure` | steps, errors | - |
| `BatchGetTaskRequest` | `structure` | identifiers | - |
| `BatchGetTaskResponse` | `structure` | tasks, errors | - |
| `BatchGetWorkerRequest` | `structure` | identifiers | - |
| `BatchGetWorkerResponse` | `structure` | workers, errors | - |
| `BatchUpdateJobRequest` | `structure` | clientToken, jobs | - |
| `BatchUpdateJobResponse` | `structure` | errors | - |
| `BatchUpdateTaskRequest` | `structure` | clientToken, tasks | - |
| `BatchUpdateTaskResponse` | `structure` | errors | - |
| `CreateQueueFleetAssociationRequest` | `structure` | farmId, queueId, fleetId | - |
| `CreateQueueFleetAssociationResponse` | `structure` | **empty (no members)** | - |
| `CreateQueueLimitAssociationRequest` | `structure` | farmId, queueId, limitId | - |
| `CreateQueueLimitAssociationResponse` | `structure` | **empty (no members)** | - |
| `DeleteQueueFleetAssociationRequest` | `structure` | farmId, queueId, fleetId | Identifier mixin for queue-fleet association operations. Composes QueueIdentifierMixin (farmId + queueId) and adds fleetId. |
| `DeleteQueueFleetAssociationResponse` | `structure` | **empty (no members)** | - |
| `DeleteQueueLimitAssociationRequest` | `structure` | farmId, queueId, limitId | - |
| `DeleteQueueLimitAssociationResponse` | `structure` | **empty (no members)** | - |
| `GetQueueFleetAssociationRequest` | `structure` | farmId, queueId, fleetId | Identifier mixin for queue-fleet association operations. Composes QueueIdentifierMixin (farmId + queueId) and adds fleetId. |
| `GetQueueFleetAssociationResponse` | `structure` | queueId, fleetId, status, createdAt, createdBy, updatedAt, updatedBy | Domain fields for QueueFleetAssociation summary/response shapes, ordered before timestamps. |
| `GetQueueLimitAssociationRequest` | `structure` | farmId, queueId, limitId | - |
| `GetQueueLimitAssociationResponse` | `structure` | queueId, limitId, status, createdAt, createdBy, updatedAt, updatedBy | Domain fields for QueueLimitAssociation summary/response shapes, ordered before timestamps. |
| `GetSessionsStatisticsAggregationRequest` | `structure` | farmId, nextToken, maxResults, aggregationId | Shared pagination fields for List operation inputs (nextToken + maxResults). |
| `GetSessionsStatisticsAggregationResponse` | `structure` | statistics, status, statusMessage, nextToken | Shared pagination field for List operation outputs (nextToken). |
| `ListAvailableMeteredProductsRequest` | `structure` | nextToken, maxResults | Shared pagination fields for List operation inputs (nextToken + maxResults). |
| `ListAvailableMeteredProductsResponse` | `structure` | meteredProducts, nextToken | Shared pagination field for List operation outputs (nextToken). |
| `ListQueueFleetAssociationsRequest` | `structure` | farmId, nextToken, maxResults, queueId, fleetId | Shared pagination fields for List operation inputs (nextToken + maxResults). |
| `AcceleratorName` | `enum` | T4, A10G, L4, L40S, RTX_PRO_SERVER_6000 | - |
| `AcceleratorType` | `enum` | GPU | - |
| `AutoScalingMode` | `enum` | NO_SCALING, EVENT_BASED_AUTO_SCALING | - |
| `AutoScalingStatus` | `enum` | GROWING, STEADY, SHRINKING | - |
| `BatchGetJobErrorCode` | `enum` | InternalServerErrorException, ResourceNotFoundException, ValidationException, AccessDeniedException, ThrottlingException | - |
| `BatchGetSessionActionErrorCode` | `enum` | InternalServerErrorException, ResourceNotFoundException, ValidationException | - |
| `BatchGetSessionErrorCode` | `enum` | InternalServerErrorException, ResourceNotFoundException, ValidationException | - |
| `BatchGetStepErrorCode` | `enum` | InternalServerErrorException, ResourceNotFoundException, ValidationException, AccessDeniedException, ThrottlingException | - |
| `BatchGetTaskErrorCode` | `enum` | InternalServerErrorException, ResourceNotFoundException, ValidationException, AccessDeniedException, ThrottlingException | - |
| `BatchGetWorkerErrorCode` | `enum` | InternalServerErrorException, ResourceNotFoundException, ValidationException | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
