# AWS IoT TwinMaker

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

IoT TwinMaker is a service with which you can build operational digital twins of physical systems. IoT TwinMaker overlays measurements and analysis from real-world sensors, cameras, and enterprise applications so you can create data visualizations to monitor your physical factory, building, or industrial plant. You can use this real-world data to monitor operations and diagnose and repair errors.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS IoT TwinMaker resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS IoT TwinMaker workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListComponentTypes`, `ListComponents`, `ListEntities`, `ListMetadataTransferJobs`, `GetComponentType`, `GetEntity`.

## Service Identity and Protocol

- AWS model slug: `iottwinmaker`
- AWS SDK for Rust slug: `iottwinmaker`
- Model version: `2021-11-29`
- Model file: `vendor/api-models-aws/models/iottwinmaker/service/2021-11-29/iottwinmaker-2021-11-29.json`
- SDK ID: `IoTTwinMaker`
- Endpoint prefix: `iottwinmaker`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Get` (9), `Create` (6), `Delete` (5), `Update` (5), `Batch` (1), `Cancel` (1), `Execute` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchPutPropertyValues`, `CancelMetadataTransferJob`, `CreateComponentType`, `CreateEntity`, `CreateMetadataTransferJob`, `CreateScene`, `CreateSyncJob`, `CreateWorkspace`, `DeleteComponentType`, `DeleteEntity`, `DeleteScene`, `DeleteSyncJob`, `DeleteWorkspace`, `TagResource`, `UntagResource`, `UpdateComponentType`, `UpdateEntity`, `UpdatePricingPlan`, `UpdateScene`, `UpdateWorkspace`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetComponentType`, `GetEntity`, `GetMetadataTransferJob`, `GetPricingPlan`, `GetPropertyValue`, `GetPropertyValueHistory`, `GetScene`, `GetSyncJob`, `GetWorkspace`, `ListComponentTypes`, `ListComponents`, `ListEntities`, `ListMetadataTransferJobs`, `ListProperties`, `ListScenes`, `ListSyncJobs`, `ListSyncResources`, `ListTagsForResource`, `ListWorkspaces`.
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelMetadataTransferJob`, `CreateMetadataTransferJob`, `CreateSyncJob`, `DeleteSyncJob`, `GetMetadataTransferJob`, `GetSyncJob`, `ListMetadataTransferJobs`, `ListSyncJobs`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 40 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `Lambda`.

## Operation Groups

### List

- Operations: `ListComponentTypes`, `ListComponents`, `ListEntities`, `ListMetadataTransferJobs`, `ListProperties`, `ListScenes`, `ListSyncJobs`, `ListSyncResources`, `ListTagsForResource`, `ListWorkspaces`
- Traits: `endpoint-bound` (10), `paginated` (9)
- Common required input members in this group: `destinationType`, `entityId`, `resourceARN`, `sourceType`, `syncSource`, `workspaceId`

### Get

- Operations: `GetComponentType`, `GetEntity`, `GetMetadataTransferJob`, `GetPricingPlan`, `GetPropertyValue`, `GetPropertyValueHistory`, `GetScene`, `GetSyncJob`, `GetWorkspace`
- Traits: `endpoint-bound` (9), `paginated` (2), `readonly` (3)
- Common required input members in this group: `componentTypeId`, `entityId`, `metadataTransferJobId`, `sceneId`, `selectedProperties`, `syncSource`, `workspaceId`

### Create

- Operations: `CreateComponentType`, `CreateEntity`, `CreateMetadataTransferJob`, `CreateScene`, `CreateSyncJob`, `CreateWorkspace`
- Traits: `endpoint-bound` (6)
- Common required input members in this group: `componentTypeId`, `contentLocation`, `destination`, `entityName`, `sceneId`, `sources`, `syncRole`, `syncSource`, `workspaceId`

### Delete

- Operations: `DeleteComponentType`, `DeleteEntity`, `DeleteScene`, `DeleteSyncJob`, `DeleteWorkspace`
- Traits: `endpoint-bound` (5)
- Common required input members in this group: `componentTypeId`, `entityId`, `sceneId`, `syncSource`, `workspaceId`

### Update

- Operations: `UpdateComponentType`, `UpdateEntity`, `UpdatePricingPlan`, `UpdateScene`, `UpdateWorkspace`
- Traits: `endpoint-bound` (5)
- Common required input members in this group: `componentTypeId`, `entityId`, `pricingMode`, `sceneId`, `workspaceId`

### Batch

- Operations: `BatchPutPropertyValues`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `entries`, `workspaceId`

### Cancel

- Operations: `CancelMetadataTransferJob`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `metadataTransferJobId`

### Execute

- Operations: `ExecuteQuery`
- Traits: `endpoint-bound` (1), `paginated` (1)
- Common required input members in this group: `queryStatement`, `workspaceId`

### Tag

- Operations: `TagResource`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `resourceARN`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `resourceARN`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchPutPropertyValues` | `POST /workspaces/{workspaceId}/entity-properties` | `endpoint-bound` | `entries`, `workspaceId` | - | `BatchPutPropertyValuesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sets values for multiple time series properties. |
| `CancelMetadataTransferJob` | `PUT /metadata-transfer-jobs/{metadataTransferJobId}/cancel` | `endpoint-bound` | `metadataTransferJobId` | - | `CancelMetadataTransferJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels the metadata transfer job. |
| `CreateComponentType` | `POST /workspaces/{workspaceId}/component-types/{componentTypeId}` | `endpoint-bound` | `componentTypeId`, `workspaceId` | - | `CreateComponentTypeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a component type. |
| `CreateEntity` | `POST /workspaces/{workspaceId}/entities` | `endpoint-bound` | `entityName`, `workspaceId` | - | `CreateEntityResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an entity. |
| `CreateMetadataTransferJob` | `POST /metadata-transfer-jobs` | `endpoint-bound` | `destination`, `sources` | - | `CreateMetadataTransferJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new metadata transfer job. |
| `CreateScene` | `POST /workspaces/{workspaceId}/scenes` | `endpoint-bound` | `contentLocation`, `sceneId`, `workspaceId` | - | `CreateSceneResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a scene. |
| `CreateSyncJob` | `POST /workspaces/{workspaceId}/sync-jobs/{syncSource}` | `endpoint-bound` | `syncRole`, `syncSource`, `workspaceId` | - | `CreateSyncJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This action creates a SyncJob. |
| `CreateWorkspace` | `POST /workspaces/{workspaceId}` | `endpoint-bound` | `workspaceId` | - | `CreateWorkspaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a workplace. |
| `DeleteComponentType` | `DELETE /workspaces/{workspaceId}/component-types/{componentTypeId}` | `endpoint-bound` | `componentTypeId`, `workspaceId` | - | `DeleteComponentTypeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a component type. |
| `DeleteEntity` | `DELETE /workspaces/{workspaceId}/entities/{entityId}` | `endpoint-bound` | `entityId`, `workspaceId` | - | `DeleteEntityResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes an entity. |
| `DeleteScene` | `DELETE /workspaces/{workspaceId}/scenes/{sceneId}` | `endpoint-bound` | `sceneId`, `workspaceId` | - | `DeleteSceneResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a scene. |
| `DeleteSyncJob` | `DELETE /workspaces/{workspaceId}/sync-jobs/{syncSource}` | `endpoint-bound` | `syncSource`, `workspaceId` | - | `DeleteSyncJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Delete the SyncJob. |
| `DeleteWorkspace` | `DELETE /workspaces/{workspaceId}` | `endpoint-bound` | `workspaceId` | - | `DeleteWorkspaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a workspace. |
| `ExecuteQuery` | `POST /queries/execution` | `paginated`, `endpoint-bound` | `queryStatement`, `workspaceId` | - | `ExecuteQueryResponse` | `AccessDeniedException`, `InternalServerException`, `QueryTimeoutException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Run queries to access information from your knowledge graph of entities within individual workspaces. The ExecuteQuery action only works with Amazon Web Services Java SDK2. |
| `GetComponentType` | `GET /workspaces/{workspaceId}/component-types/{componentTypeId}` | `endpoint-bound` | `componentTypeId`, `workspaceId` | - | `GetComponentTypeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a component type. |
| `GetEntity` | `GET /workspaces/{workspaceId}/entities/{entityId}` | `endpoint-bound` | `entityId`, `workspaceId` | - | `GetEntityResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about an entity. |
| `GetMetadataTransferJob` | `GET /metadata-transfer-jobs/{metadataTransferJobId}` | `readonly`, `endpoint-bound` | `metadataTransferJobId` | - | `GetMetadataTransferJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a nmetadata transfer job. |
| `GetPricingPlan` | `GET /pricingplan` | `readonly`, `endpoint-bound` | - | - | `GetPricingPlanResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets the pricing plan. |
| `GetPropertyValue` | `POST /workspaces/{workspaceId}/entity-properties/value` | `paginated`, `endpoint-bound` | `selectedProperties`, `workspaceId` | - | `GetPropertyValueResponse` | `AccessDeniedException`, `ConnectorFailureException`, `ConnectorTimeoutException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the property values for a component, component type, entity, or workspace. You must specify a value for either `componentName`, `componentTypeId`, `entityId`, or `workspaceId`. |
| `GetPropertyValueHistory` | `POST /workspaces/{workspaceId}/entity-properties/history` | `paginated`, `endpoint-bound` | `selectedProperties`, `workspaceId` | - | `GetPropertyValueHistoryResponse` | `AccessDeniedException`, `ConnectorFailureException`, `ConnectorTimeoutException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the history of a time series property value for a component, component type, entity, or workspace. You must specify a value for `workspaceId`. |
| `GetScene` | `GET /workspaces/{workspaceId}/scenes/{sceneId}` | `endpoint-bound` | `sceneId`, `workspaceId` | - | `GetSceneResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a scene. |
| `GetSyncJob` | `GET /sync-jobs/{syncSource}` | `readonly`, `endpoint-bound` | `syncSource` | - | `GetSyncJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets the SyncJob. |
| `GetWorkspace` | `GET /workspaces/{workspaceId}` | `endpoint-bound` | `workspaceId` | - | `GetWorkspaceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about a workspace. |
| `ListComponentTypes` | `POST /workspaces/{workspaceId}/component-types-list` | `paginated`, `endpoint-bound` | `workspaceId` | - | `ListComponentTypesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all component types in a workspace. |
| `ListComponents` | `POST /workspaces/{workspaceId}/entities/{entityId}/components-list` | `paginated`, `endpoint-bound` | `entityId`, `workspaceId` | - | `ListComponentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This API lists the components of an entity. |
| `ListEntities` | `POST /workspaces/{workspaceId}/entities-list` | `paginated`, `endpoint-bound` | `workspaceId` | - | `ListEntitiesResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all entities in a workspace. |
| `ListMetadataTransferJobs` | `POST /metadata-transfer-jobs-list` | `paginated`, `endpoint-bound` | `destinationType`, `sourceType` | - | `ListMetadataTransferJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the metadata transfer jobs. |
| `ListProperties` | `POST /workspaces/{workspaceId}/properties-list` | `paginated`, `endpoint-bound` | `entityId`, `workspaceId` | - | `ListPropertiesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This API lists the properties of a component. |
| `ListScenes` | `POST /workspaces/{workspaceId}/scenes-list` | `paginated`, `endpoint-bound` | `workspaceId` | - | `ListScenesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all scenes in a workspace. |
| `ListSyncJobs` | `POST /workspaces/{workspaceId}/sync-jobs-list` | `paginated`, `endpoint-bound` | `workspaceId` | - | `ListSyncJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | List all SyncJobs. |
| `ListSyncResources` | `POST /workspaces/{workspaceId}/sync-jobs/{syncSource}/resources-list` | `paginated`, `endpoint-bound` | `syncSource`, `workspaceId` | - | `ListSyncResourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists the sync resources. |
| `ListTagsForResource` | `POST /tags-list` | `endpoint-bound` | `resourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Lists all tags associated with a resource. |
| `ListWorkspaces` | `POST /workspaces-list` | `paginated`, `endpoint-bound` | - | - | `ListWorkspacesResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about workspaces in the current account. |
| `TagResource` | `POST /tags` | `endpoint-bound` | `resourceARN`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `TooManyTagsException` | Adds tags to a resource. |
| `UntagResource` | `DELETE /tags` | `endpoint-bound` | `resourceARN`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Removes tags from a resource. |
| `UpdateComponentType` | `PUT /workspaces/{workspaceId}/component-types/{componentTypeId}` | `endpoint-bound` | `componentTypeId`, `workspaceId` | - | `UpdateComponentTypeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates information in a component type. |
| `UpdateEntity` | `PUT /workspaces/{workspaceId}/entities/{entityId}` | `endpoint-bound` | `entityId`, `workspaceId` | - | `UpdateEntityResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an entity. |
| `UpdatePricingPlan` | `POST /pricingplan` | `endpoint-bound` | `pricingMode` | - | `UpdatePricingPlanResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Update the pricing plan. |
| `UpdateScene` | `PUT /workspaces/{workspaceId}/scenes/{sceneId}` | `endpoint-bound` | `sceneId`, `workspaceId` | - | `UpdateSceneResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a scene. |
| `UpdateWorkspace` | `PUT /workspaces/{workspaceId}` | `endpoint-bound` | `workspaceId` | - | `UpdateWorkspaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a workspace. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | An unexpected error has occurred. |
| `ThrottlingException` | `structure` | `message` | The rate exceeds the limit. |
| `ValidationException` | `structure` | `message` | Failed |
| `AccessDeniedException` | `structure` | `message` | Access is denied. |
| `ResourceNotFoundException` | `structure` | `message` | The resource wasn't found. |
| `ServiceQuotaExceededException` | `structure` | `message` | The service quota was exceeded. |
| `ConflictException` | `structure` | `message` | A conflict occurred. |
| `ConnectorFailureException` | `structure` | `message` | The connector failed. |
| `ConnectorTimeoutException` | `structure` | `message` | The connector timed out. |
| `BatchPutPropertyValuesRequest` | `structure` | `entries`, `workspaceId` | - |
| `BatchPutPropertyValuesResponse` | `structure` | `errorEntries` | - |
| `CancelMetadataTransferJobRequest` | `structure` | `metadataTransferJobId` | - |
| `CancelMetadataTransferJobResponse` | `structure` | `arn`, `metadataTransferJobId`, `progress`, `status`, `updateDateTime` | - |
| `CreateComponentTypeRequest` | `structure` | `componentTypeId`, `componentTypeName`, `compositeComponentTypes`, `description`, `extendsFrom`, `functions`, `isSingleton`, `propertyDefinitions`, `propertyGroups`, `tags`, `workspaceId` | - |
| `CreateComponentTypeResponse` | `structure` | `arn`, `creationDateTime`, `state` | - |
| `CreateEntityRequest` | `structure` | `components`, `compositeComponents`, `description`, `entityId`, `entityName`, `parentEntityId`, `tags`, `workspaceId` | - |
| `CreateEntityResponse` | `structure` | `arn`, `creationDateTime`, `entityId`, `state` | - |
| `CreateMetadataTransferJobRequest` | `structure` | `description`, `destination`, `metadataTransferJobId`, `sources` | - |
| `CreateMetadataTransferJobResponse` | `structure` | `arn`, `creationDateTime`, `metadataTransferJobId`, `status` | - |
| `CreateSceneRequest` | `structure` | `capabilities`, `contentLocation`, `description`, `sceneId`, `sceneMetadata`, `tags`, `workspaceId` | - |
| `CreateSceneResponse` | `structure` | `arn`, `creationDateTime` | - |
| `CreateSyncJobRequest` | `structure` | `syncRole`, `syncSource`, `tags`, `workspaceId` | - |
| `CreateSyncJobResponse` | `structure` | `arn`, `creationDateTime`, `state` | - |
| `CreateWorkspaceRequest` | `structure` | `description`, `role`, `s3Location`, `tags`, `workspaceId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
