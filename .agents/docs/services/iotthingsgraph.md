# AWS IoT Things Graph

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS IoT Things Graph AWS IoT Things Graph provides an integrated set of tools that enable developers to connect devices and services that use different standards, such as units of measure and communication protocols. AWS IoT Things Graph makes it possible to build IoT applications with little to no code by connecting devices and services and defining how they interact at an abstract level. For more information about how AWS IoT Things Graph works, see the User Guide. The AWS IoT Things Graph service is discontinued.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS IoT Things Graph where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS IoT Things Graph by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS IoT Things Graph workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Search`, `Delete`, `Create`, `Deprecate` operation families, including `GetEntities`, `GetFlowTemplate`, `GetFlowTemplateRevisions`, `GetNamespaceDeletionStatus`, `SearchEntities`, `SearchFlowExecutions`.

## Service Identity and Protocol

- AWS model slug: `iotthingsgraph`
- AWS SDK for Rust slug: `iotthingsgraph`
- Model version: `2018-09-06`
- Model file: `vendor/api-models-aws/models/iotthingsgraph/service/2018-09-06/iotthingsgraph-2018-09-06.json`
- SDK ID: `IoTThingsGraph`
- Endpoint prefix: `iotthingsgraph`
- ARN namespace: `iotthingsgraph`
- CloudFormation name: `IoTThingsGraph`
- CloudTrail event source: `iotthingsgraph.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (8), `Search` (6), `Delete` (4), `Create` (3), `Deprecate` (2), `List` (2), `Update` (2), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateEntityToThing`, `CreateFlowTemplate`, `CreateSystemInstance`, `CreateSystemTemplate`, `DeleteFlowTemplate`, `DeleteNamespace`, `DeleteSystemInstance`, `DeleteSystemTemplate`, `TagResource`, `UntagResource`, `UpdateFlowTemplate`, `UpdateSystemTemplate`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeNamespace`, `GetEntities`, `GetFlowTemplate`, `GetFlowTemplateRevisions`, `GetNamespaceDeletionStatus`, `GetSystemInstance`, `GetSystemTemplate`, `GetSystemTemplateRevisions`, `GetUploadStatus`, `ListFlowExecutionMessages`, `ListTagsForResource`, `SearchEntities`, `SearchFlowExecutions`, `SearchFlowTemplates`, `SearchSystemInstances`, `SearchSystemTemplates`, `SearchThings`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ListFlowExecutionMessages`, `SearchFlowExecutions`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 35 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetEntities`, `GetFlowTemplate`, `GetFlowTemplateRevisions`, `GetNamespaceDeletionStatus`, `GetSystemInstance`, `GetSystemTemplate`, `GetSystemTemplateRevisions`, `GetUploadStatus`
- Traits: `paginated` (2)
- Common required input members in this group: `id`

### Search

- Operations: `SearchEntities`, `SearchFlowExecutions`, `SearchFlowTemplates`, `SearchSystemInstances`, `SearchSystemTemplates`, `SearchThings`
- Traits: `paginated` (6)
- Common required input members in this group: -

### Delete

- Operations: `DeleteFlowTemplate`, `DeleteNamespace`, `DeleteSystemInstance`, `DeleteSystemTemplate`
- Common required input members in this group: `id`

### Create

- Operations: `CreateFlowTemplate`, `CreateSystemInstance`, `CreateSystemTemplate`
- Common required input members in this group: `definition`

### Deprecate

- Operations: `DeprecateFlowTemplate`, `DeprecateSystemTemplate`
- Common required input members in this group: `id`

### List

- Operations: `ListFlowExecutionMessages`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Update

- Operations: `UpdateFlowTemplate`, `UpdateSystemTemplate`
- Common required input members in this group: `id`, `definition`

### Associate

- Operations: `AssociateEntityToThing`
- Common required input members in this group: -

### Deploy

- Operations: `DeploySystemInstance`
- Common required input members in this group: -

### Describe

- Operations: `DescribeNamespace`
- Common required input members in this group: -

### Dissociate

- Operations: `DissociateEntityFromThing`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Undeploy

- Operations: `UndeploySystemInstance`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Upload

- Operations: `UploadEntityDefinitions`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateEntityToThing` | `-` | - | `thingName`, `entityId` | - | `AssociateEntityToThingResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Associates a device with a concrete thing that is in the user's registry. A thing can be associated with only one device at a time. If you associate a thing with a new device id, its previous association will be removed. |
| `CreateFlowTemplate` | `-` | - | `definition` | - | `CreateFlowTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ThrottlingException` | Creates a workflow template. Workflows can be created only in the user's namespace. (The public namespace contains only entities.) The workflow can contain only entities in the specified namespace. The workflow is va ... |
| `CreateSystemInstance` | `-` | - | `definition`, `target` | - | `CreateSystemInstanceResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ThrottlingException` | Creates a system instance. This action validates the system instance, prepares the deployment-related resources. For Greengrass deployments, it updates the Greengrass group that is specified by the greengrassGroupNam ... |
| `CreateSystemTemplate` | `-` | - | `definition` | - | `CreateSystemTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ThrottlingException` | Creates a system. The system is validated against the entities in the latest version of the user's namespace unless another namespace version is specified in the request. |
| `DeleteFlowTemplate` | `-` | - | `id` | - | `DeleteFlowTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ThrottlingException` | Deletes a workflow. Any new system or deployment that contains this workflow will fail to update or deploy. Existing deployments that contain the workflow will continue to run (since they use a snapshot of the workfl ... |
| `DeleteNamespace` | `-` | - | - | - | `DeleteNamespaceResponse` | `InternalFailureException`, `ThrottlingException` | Deletes the specified namespace. This action deletes all of the entities in the namespace. Delete the systems and flows that use entities in the namespace before performing this action. This action takes no request p ... |
| `DeleteSystemInstance` | `-` | - | - | - | `DeleteSystemInstanceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ThrottlingException` | Deletes a system instance. Only system instances that have never been deployed, or that have been undeployed can be deleted. Users can create a new system instance that has the same ID as a deleted system instance. |
| `DeleteSystemTemplate` | `-` | - | `id` | - | `DeleteSystemTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ThrottlingException` | Deletes a system. New deployments can't contain the system after its deletion. Existing deployments that contain the system will continue to work because they use a snapshot of the system that is taken when it is dep ... |
| `DeploySystemInstance` | `-` | - | - | - | `DeploySystemInstanceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Greengrass and Cloud Deployments Deploys the system instance to the target specified in CreateSystemInstance . Greengrass Deployments If the system or any workflows and entities have been updated before this action i ... |
| `DeprecateFlowTemplate` | `-` | - | `id` | - | `DeprecateFlowTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deprecates the specified workflow. This action marks the workflow for deletion. Deprecated flows can't be deployed, but existing deployments will continue to run. |
| `DeprecateSystemTemplate` | `-` | - | `id` | - | `DeprecateSystemTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deprecates the specified system. |
| `DescribeNamespace` | `-` | - | - | - | `DescribeNamespaceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the latest version of the user's namespace and the public version that it is tracking. |
| `DissociateEntityFromThing` | `-` | - | `thingName`, `entityType` | - | `DissociateEntityFromThingResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Dissociates a device entity from a concrete thing. The action takes only the type of the entity that you need to dissociate because only one entity of a particular type can be associated with a thing. |
| `GetEntities` | `-` | - | `ids` | - | `GetEntitiesResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets definitions of the specified entities. Uses the latest version of the user's namespace by default. This API returns the following TDM entities. Properties States Events Actions Capabilities Mappings Devices Devi ... |
| `GetFlowTemplate` | `-` | - | `id` | - | `GetFlowTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the latest version of the DefinitionDocument and FlowTemplateSummary for the specified workflow. |
| `GetFlowTemplateRevisions` | `-` | `paginated` | `id` | - | `GetFlowTemplateRevisionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets revisions of the specified workflow. Only the last 100 revisions are stored. If the workflow has been deprecated, this action will return revisions that occurred before the deprecation. This action won't work fo ... |
| `GetNamespaceDeletionStatus` | `-` | - | - | - | `GetNamespaceDeletionStatusResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Gets the status of a namespace deletion task. |
| `GetSystemInstance` | `-` | - | `id` | - | `GetSystemInstanceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets a system instance. |
| `GetSystemTemplate` | `-` | - | `id` | - | `GetSystemTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets a system. |
| `GetSystemTemplateRevisions` | `-` | `paginated` | `id` | - | `GetSystemTemplateRevisionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets revisions made to the specified system template. Only the previous 100 revisions are stored. If the system has been deprecated, this action will return the revisions that occurred before its deprecation. This ac ... |
| `GetUploadStatus` | `-` | - | `uploadId` | - | `GetUploadStatusResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the status of the specified upload. |
| `ListFlowExecutionMessages` | `-` | `paginated` | `flowExecutionId` | - | `ListFlowExecutionMessagesResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of objects that contain information about events in a flow execution. |
| `ListTagsForResource` | `-` | `paginated` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ThrottlingException` | Lists all tags on an AWS IoT Things Graph resource. |
| `SearchEntities` | `-` | `paginated` | `entityTypes` | - | `SearchEntitiesResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Searches for entities of the specified type. You can search for entities in your namespace and the public namespace that you're tracking. |
| `SearchFlowExecutions` | `-` | `paginated` | `systemInstanceId` | - | `SearchFlowExecutionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Searches for AWS IoT Things Graph workflow execution instances. |
| `SearchFlowTemplates` | `-` | `paginated` | - | - | `SearchFlowTemplatesResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Searches for summary information about workflows. |
| `SearchSystemInstances` | `-` | `paginated` | - | - | `SearchSystemInstancesResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Searches for system instances in the user's account. |
| `SearchSystemTemplates` | `-` | `paginated` | - | - | `SearchSystemTemplatesResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Searches for summary information about systems in the user's account. You can filter by the ID of a workflow to return only systems that use the specified workflow. |
| `SearchThings` | `-` | `paginated` | `entityId` | - | `SearchThingsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Searches for things associated with the specified entity. You can search by both device and device model. For example, if two different devices, camera1 and camera2, implement the camera device model, the user can as ... |
| `TagResource` | `-` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ThrottlingException` | Creates a tag for the specified resource. |
| `UndeploySystemInstance` | `-` | - | - | - | `UndeploySystemInstanceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Removes a system instance from its target (Cloud or Greengrass). |
| `UntagResource` | `-` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ThrottlingException` | Removes a tag from the specified resource. |
| `UpdateFlowTemplate` | `-` | - | `id`, `definition` | - | `UpdateFlowTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the specified workflow. All deployed systems and system instances that use the workflow will see the changes in the flow when it is redeployed. If you don't want this behavior, copy the workflow (creating a n ... |
| `UpdateSystemTemplate` | `-` | - | `id`, `definition` | - | `UpdateSystemTemplateResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the specified system. You don't need to run this action after updating a workflow. Any deployment that uses the system will see the changes in the system when it is redeployed. |
| `UploadEntityDefinitions` | `-` | - | - | - | `UploadEntityDefinitionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Asynchronously uploads one or more entity definitions to the user's namespace. The document parameter is required if syncWithPublicNamespace and deleteExistingEntites are false. If the syncWithPublicNamespace paramet ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalFailureException` | `structure` | message | - |
| `InvalidRequestException` | `structure` | message | - |
| `LimitExceededException` | `structure` | message | - |
| `ResourceAlreadyExistsException` | `structure` | message | - |
| `ResourceInUseException` | `structure` | message | - |
| `ResourceNotFoundException` | `structure` | message | - |
| `ThrottlingException` | `structure` | message | - |
| `AssociateEntityToThingRequest` | `structure` | thingName, entityId, namespaceVersion | - |
| `AssociateEntityToThingResponse` | `structure` | **empty (no members)** | - |
| `CreateFlowTemplateRequest` | `structure` | definition, compatibleNamespaceVersion | - |
| `CreateFlowTemplateResponse` | `structure` | summary | - |
| `CreateSystemInstanceRequest` | `structure` | tags, definition, target, greengrassGroupName, s3BucketName, metricsConfiguration, flowActionsRoleArn | - |
| `CreateSystemInstanceResponse` | `structure` | summary | - |
| `CreateSystemTemplateRequest` | `structure` | definition, compatibleNamespaceVersion | - |
| `CreateSystemTemplateResponse` | `structure` | summary | - |
| `DeleteFlowTemplateRequest` | `structure` | id | - |
| `DeleteFlowTemplateResponse` | `structure` | **empty (no members)** | - |
| `DeleteNamespaceRequest` | `structure` | **empty (no members)** | - |
| `DeleteNamespaceResponse` | `structure` | namespaceArn, namespaceName | - |
| `DeleteSystemInstanceRequest` | `structure` | id | - |
| `DeleteSystemInstanceResponse` | `structure` | **empty (no members)** | - |
| `DeleteSystemTemplateRequest` | `structure` | id | - |
| `DeleteSystemTemplateResponse` | `structure` | **empty (no members)** | - |
| `DeploySystemInstanceRequest` | `structure` | id | - |
| `DeploySystemInstanceResponse` | `structure` | summary, greengrassDeploymentId | - |
| `DeprecateFlowTemplateRequest` | `structure` | id | - |
| `DeprecateFlowTemplateResponse` | `structure` | **empty (no members)** | - |
| `DeprecateSystemTemplateRequest` | `structure` | id | - |
| `DeprecateSystemTemplateResponse` | `structure` | **empty (no members)** | - |
| `DescribeNamespaceRequest` | `structure` | namespaceName | - |
| `DescribeNamespaceResponse` | `structure` | namespaceArn, namespaceName, trackingNamespaceName, trackingNamespaceVersion, namespaceVersion | - |
| `DissociateEntityFromThingRequest` | `structure` | thingName, entityType | - |
| `DissociateEntityFromThingResponse` | `structure` | **empty (no members)** | - |
| `GetEntitiesRequest` | `structure` | ids, namespaceVersion | - |
| `GetEntitiesResponse` | `structure` | descriptions | - |
| `GetFlowTemplateRequest` | `structure` | id, revisionNumber | - |
| `GetFlowTemplateResponse` | `structure` | description | - |
| `GetFlowTemplateRevisionsRequest` | `structure` | id, nextToken, maxResults | - |
| `GetFlowTemplateRevisionsResponse` | `structure` | summaries, nextToken | - |
| `GetNamespaceDeletionStatusRequest` | `structure` | **empty (no members)** | - |
| `DefinitionLanguage` | `enum` | GRAPHQL | - |
| `DeploymentTarget` | `enum` | GREENGRASS, CLOUD | - |
| `EntityFilterName` | `enum` | NAME, NAMESPACE, SEMANTIC_TYPE_PATH, REFERENCED_ENTITY_ID | - |
| `EntityType` | `enum` | DEVICE, SERVICE, DEVICE_MODEL, CAPABILITY, STATE, ACTION, EVENT, PROPERTY, MAPPING, ENUM | - |
| `FlowExecutionEventType` | `enum` | EXECUTION_STARTED, EXECUTION_FAILED, EXECUTION_ABORTED, EXECUTION_SUCCEEDED, STEP_STARTED, STEP_FAILED, STEP_SUCCEEDED, ACTIVITY_SCHEDULED, ACTIVITY_STARTED, ACTIVITY_FAILED, ACTIVITY_SUCCEEDED, START_FLOW_EXECUTION_TASK, ... (+5) | - |
| `FlowExecutionStatus` | `enum` | RUNNING, ABORTED, SUCCEEDED, FAILED | - |
| `FlowTemplateFilterName` | `enum` | DEVICE_MODEL_ID | - |
| `NamespaceDeletionStatus` | `enum` | IN_PROGRESS, SUCCEEDED, FAILED | - |
| `NamespaceDeletionStatusErrorCodes` | `enum` | VALIDATION_FAILED | - |
| `SystemInstanceDeploymentStatus` | `enum` | NOT_DEPLOYED, BOOTSTRAP, DEPLOY_IN_PROGRESS, DEPLOYED_IN_TARGET, UNDEPLOY_IN_PROGRESS, FAILED, PENDING_DELETE, DELETED_IN_TARGET | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
