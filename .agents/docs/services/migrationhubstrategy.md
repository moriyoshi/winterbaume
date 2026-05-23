# Migration Hub Strategy Recommendations

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Migration Hub Strategy Recommendations This API reference provides descriptions, syntax, and other details about each of the actions and data types for Migration Hub Strategy Recommendations (Strategy Recommendations). The topic for each action shows the API request parameters and the response. Alternatively, you can use one of the AWS SDKs to access an API that is tailored to the programming language or platform that you're using. For more information, see AWS SDKs.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Migration Hub Strategy Recommendations by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Migration Hub Strategy Recommendations resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Migration Hub Strategy Recommendations workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Start`, `Update`, `Put` operation families, including `GetApplicationComponentDetails`, `GetApplicationComponentStrategies`, `GetAssessment`, `GetImportFileTask`, `ListAnalyzableServers`, `ListApplicationComponents`.

## Service Identity and Protocol

- AWS model slug: `migrationhubstrategy`
- AWS SDK for Rust slug: `migrationhubstrategy`
- Model version: `2020-02-19`
- Model file: `vendor/api-models-aws/models/migrationhubstrategy/service/2020-02-19/migrationhubstrategy-2020-02-19.json`
- SDK ID: `MigrationHubStrategy`
- Endpoint prefix: `migrationhub-strategy`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `migrationhub-strategy.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (10), `List` (5), `Start` (3), `Update` (2), `Put` (1), `Stop` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `PutPortfolioPreferences`, `StartAssessment`, `StartImportFileTask`, `StartRecommendationReportGeneration`, `StopAssessment`, `UpdateApplicationComponentConfig`, `UpdateServerConfig`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApplicationComponentDetails`, `GetApplicationComponentStrategies`, `GetAssessment`, `GetImportFileTask`, `GetLatestAssessmentId`, `GetPortfolioPreferences`, `GetPortfolioSummary`, `GetRecommendationReportDetails`, `GetServerDetails`, `GetServerStrategies`, `ListAnalyzableServers`, `ListApplicationComponents`, `ListCollectors`, `ListImportFileTask`, `ListServers`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetImportFileTask`, `GetRecommendationReportDetails`, `ListImportFileTask`, `StartAssessment`, `StartImportFileTask`, `StartRecommendationReportGeneration`, `StopAssessment`.
- 22 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetApplicationComponentDetails`, `GetApplicationComponentStrategies`, `GetAssessment`, `GetImportFileTask`, `GetLatestAssessmentId`, `GetPortfolioPreferences`, `GetPortfolioSummary`, `GetRecommendationReportDetails`, `GetServerDetails`, `GetServerStrategies`
- Traits: `readonly` (10), `paginated` (1)
- Common required input members in this group: `applicationComponentId`, `id`, `serverId`

### List

- Operations: `ListAnalyzableServers`, `ListApplicationComponents`, `ListCollectors`, `ListImportFileTask`, `ListServers`
- Traits: `readonly` (3), `paginated` (5)
- Common required input members in this group: -

### Start

- Operations: `StartAssessment`, `StartImportFileTask`, `StartRecommendationReportGeneration`
- Common required input members in this group: -

### Update

- Operations: `UpdateApplicationComponentConfig`, `UpdateServerConfig`
- Common required input members in this group: -

### Put

- Operations: `PutPortfolioPreferences`
- Common required input members in this group: -

### Stop

- Operations: `StopAssessment`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetApplicationComponentDetails` | `GET /get-applicationcomponent-details/{applicationComponentId}` | `readonly` | `applicationComponentId` | - | `GetApplicationComponentDetailsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves details about an application component. |
| `GetApplicationComponentStrategies` | `GET /get-applicationcomponent-strategies/{applicationComponentId}` | `readonly` | `applicationComponentId` | - | `GetApplicationComponentStrategiesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a list of all the recommended strategies and tools for an application component running on a server. |
| `GetAssessment` | `GET /get-assessment/{id}` | `readonly` | `id` | - | `GetAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the status of an on-going assessment. |
| `GetImportFileTask` | `GET /get-import-file-task/{id}` | `readonly` | `id` | - | `GetImportFileTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details about a specific import task. |
| `GetLatestAssessmentId` | `GET /get-latest-assessment-id` | `readonly` | - | - | `GetLatestAssessmentIdResponse` | `AccessDeniedException`, `DependencyException`, `InternalServerException`, `ValidationException` | Retrieve the latest ID of a specific assessment task. |
| `GetPortfolioPreferences` | `GET /get-portfolio-preferences` | `readonly` | - | - | `GetPortfolioPreferencesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves your migration and modernization preferences. |
| `GetPortfolioSummary` | `GET /get-portfolio-summary` | `readonly` | - | - | `GetPortfolioSummaryResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException` | Retrieves overall summary including the number of servers to rehost and the overall number of anti-patterns. |
| `GetRecommendationReportDetails` | `GET /get-recommendation-report-details/{id}` | `readonly` | `id` | - | `GetRecommendationReportDetailsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about the specified recommendation report. |
| `GetServerDetails` | `GET /get-server-details/{serverId}` | `readonly`, `paginated` | `serverId` | - | `GetServerDetailsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specified server. |
| `GetServerStrategies` | `GET /get-server-strategies/{serverId}` | `readonly` | `serverId` | - | `GetServerStrategiesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves recommended strategies and tools for the specified server. |
| `ListAnalyzableServers` | `POST /list-analyzable-servers` | `readonly`, `paginated` | - | - | `ListAnalyzableServersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all the servers fetched from customer vCenter using Strategy Recommendation Collector. |
| `ListApplicationComponents` | `POST /list-applicationcomponents` | `paginated` | - | - | `ListApplicationComponentsResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceLinkedRoleLockClientException`, `ValidationException` | Retrieves a list of all the application components (processes). |
| `ListCollectors` | `GET /list-collectors` | `readonly`, `paginated` | - | - | `ListCollectorsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all the installed collectors. |
| `ListImportFileTask` | `GET /list-import-file-task` | `readonly`, `paginated` | - | - | `ListImportFileTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all the imports performed. |
| `ListServers` | `POST /list-servers` | `paginated` | - | - | `ListServersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all the servers. |
| `PutPortfolioPreferences` | `POST /put-portfolio-preferences` | - | - | - | `PutPortfolioPreferencesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Saves the specified migration and modernization preferences. |
| `StartAssessment` | `POST /start-assessment` | - | - | - | `StartAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException` | Starts the assessment of an on-premises environment. |
| `StartImportFileTask` | `POST /start-import-file-task` | - | `name`, `S3Bucket`, `s3key` | - | `StartImportFileTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a file import. |
| `StartRecommendationReportGeneration` | `POST /start-recommendation-report-generation` | - | - | - | `StartRecommendationReportGenerationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Starts generating a recommendation report. |
| `StopAssessment` | `POST /stop-assessment` | - | `assessmentId` | - | `StopAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Stops the assessment of an on-premises environment. |
| `UpdateApplicationComponentConfig` | `POST /update-applicationcomponent-config/` | - | `applicationComponentId` | - | `UpdateApplicationComponentConfigResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an application component. |
| `UpdateServerConfig` | `POST /update-server-config/` | - | `serverId` | - | `UpdateServerConfigResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of the specified server. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetServerDetails` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListCollectors` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListImportFileTask` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The user does not have permission to perform the action. Check the AWS Identity and Access Management (IAM) policy associated with this user. |
| `ConflictException` | `structure` | message | Exception to indicate that there is an ongoing task when a new task is created. Return when once the existing tasks are complete. |
| `DependencyException` | `structure` | message | Dependency encountered an error. |
| `InternalServerException` | `structure` | message | The server experienced an internal error. Try again. |
| `ResourceNotFoundException` | `structure` | message | The specified ID in the request is not found. |
| `ServiceLinkedRoleLockClientException` | `structure` | message | Exception to indicate that the service-linked role (SLR) is locked. |
| `ServiceQuotaExceededException` | `structure` | message | The AWS account has reached its quota of imports. Contact AWS Support to increase the quota for this account. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The request body isn't valid. |
| `GetApplicationComponentDetailsRequest` | `structure` | applicationComponentId | - |
| `GetApplicationComponentDetailsResponse` | `structure` | applicationComponentDetail, associatedApplications, moreApplicationResource, associatedServerIds | - |
| `GetApplicationComponentStrategiesRequest` | `structure` | applicationComponentId | - |
| `GetApplicationComponentStrategiesResponse` | `structure` | applicationComponentStrategies | - |
| `GetAssessmentRequest` | `structure` | id | - |
| `GetAssessmentResponse` | `structure` | id, dataCollectionDetails, assessmentTargets | - |
| `GetImportFileTaskRequest` | `structure` | id | - |
| `GetImportFileTaskResponse` | `structure` | id, status, startTime, inputS3Bucket, inputS3Key, statusReportS3Bucket, statusReportS3Key, completionTime, numberOfRecordsSuccess, numberOfRecordsFailed, importName | - |
| `GetLatestAssessmentIdRequest` | `structure` | **empty (no members)** | - |
| `GetLatestAssessmentIdResponse` | `structure` | id | - |
| `GetPortfolioPreferencesRequest` | `structure` | **empty (no members)** | - |
| `GetPortfolioPreferencesResponse` | `structure` | prioritizeBusinessGoals, applicationPreferences, databasePreferences, applicationMode | - |
| `GetPortfolioSummaryRequest` | `structure` | **empty (no members)** | - |
| `GetPortfolioSummaryResponse` | `structure` | assessmentSummary | - |
| `GetRecommendationReportDetailsRequest` | `structure` | id | - |
| `GetRecommendationReportDetailsResponse` | `structure` | id, recommendationReportDetails | - |
| `GetServerDetailsRequest` | `structure` | serverId, nextToken, maxResults | - |
| `GetServerDetailsResponse` | `structure` | nextToken, serverDetail, associatedApplications | - |
| `GetServerStrategiesRequest` | `structure` | serverId | - |
| `GetServerStrategiesResponse` | `structure` | serverStrategies | - |
| `ListAnalyzableServersRequest` | `structure` | sort, nextToken, maxResults | Represents input for ListAnalyzableServers operation. |
| `ListAnalyzableServersResponse` | `structure` | analyzableServers, nextToken | Represents output for ListAnalyzableServers operation. |
| `ListApplicationComponentsRequest` | `structure` | applicationComponentCriteria, filterValue, sort, groupIdFilter, nextToken, maxResults | - |
| `ListApplicationComponentsResponse` | `structure` | applicationComponentInfos, nextToken | - |
| `ListCollectorsRequest` | `structure` | nextToken, maxResults | - |
| `ListCollectorsResponse` | `structure` | Collectors, nextToken | - |
| `ListImportFileTaskRequest` | `structure` | nextToken, maxResults | - |
| `ListImportFileTaskResponse` | `structure` | taskInfos, nextToken | - |
| `ListServersRequest` | `structure` | serverCriteria, filterValue, sort, groupIdFilter, nextToken, maxResults | - |
| `ListServersResponse` | `structure` | serverInfos, nextToken | - |
| `PutPortfolioPreferencesRequest` | `structure` | prioritizeBusinessGoals, applicationPreferences, databasePreferences, applicationMode | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
