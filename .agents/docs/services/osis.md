# Amazon OpenSearch Ingestion

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Use the Amazon OpenSearch Ingestion API to create and manage ingestion pipelines. OpenSearch Ingestion is a fully managed data collector that delivers real-time log and trace data to OpenSearch Service domains. For more information, see Getting data into your cluster using OpenSearch Ingestion.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon OpenSearch Ingestion resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon OpenSearch Ingestion workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Delete`, `Create`, `Put` operation families, including `ListPipelineBlueprints`, `ListPipelineEndpointConnections`, `ListPipelineEndpoints`, `ListPipelines`, `GetPipeline`, `GetPipelineBlueprint`.

## Service Identity and Protocol

- AWS model slug: `osis`
- AWS SDK for Rust slug: `osis`
- Model version: `2022-01-01`
- Model file: `vendor/api-models-aws/models/osis/service/2022-01-01/osis-2022-01-01.json`
- SDK ID: `OSIS`
- Endpoint prefix: `osis`
- ARN namespace: `osis`
- CloudFormation name: `OSIS`
- CloudTrail event source: `osis.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Get` (4), `Delete` (3), `Create` (2), `Put` (1), `Revoke` (1), `Start` (1), `Stop` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreatePipeline`, `CreatePipelineEndpoint`, `DeletePipeline`, `DeletePipelineEndpoint`, `DeleteResourcePolicy`, `PutResourcePolicy`, `RevokePipelineEndpointConnections`, `StartPipeline`, `StopPipeline`, `TagResource`, `UntagResource`, `UpdatePipeline`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetPipeline`, `GetPipelineBlueprint`, `GetPipelineChangeProgress`, `GetResourcePolicy`, `ListPipelineBlueprints`, `ListPipelineEndpointConnections`, `ListPipelineEndpoints`, `ListPipelines`, `ListTagsForResource`, `ValidatePipeline`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartPipeline`, `StopPipeline`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 22 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`, `ECS`.

## Current Network Resource Stub Semantics

OSIS currently exposes VPC option shapes but does not persist detailed networking state.

- Pipeline views include an optional `vpc_options` JSON slot, and current snapshot construction sets it to `None`.
- Pipeline endpoints and service VPC endpoint shapes exist in the model, but implemented state does not create EC2 VPC endpoints or ENIs.
- VPC endpoint service names and endpoint ownership are therefore not backed by cross-service networking state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListPipelineBlueprints`, `ListPipelineEndpointConnections`, `ListPipelineEndpoints`, `ListPipelines`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: -

### Get

- Operations: `GetPipeline`, `GetPipelineBlueprint`, `GetPipelineChangeProgress`, `GetResourcePolicy`
- Common required input members in this group: `PipelineName`

### Delete

- Operations: `DeletePipeline`, `DeletePipelineEndpoint`, `DeleteResourcePolicy`
- Traits: `idempotent` (2)
- Common required input members in this group: -

### Create

- Operations: `CreatePipeline`, `CreatePipelineEndpoint`
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Revoke

- Operations: `RevokePipelineEndpointConnections`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Start

- Operations: `StartPipeline`
- Common required input members in this group: -

### Stop

- Operations: `StopPipeline`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdatePipeline`
- Common required input members in this group: -

### Validate

- Operations: `ValidatePipeline`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreatePipeline` | `POST /2022-01-01/osis/createPipeline` | - | `PipelineName`, `MinUnits`, `MaxUnits`, `PipelineConfigurationBody` | - | `CreatePipelineResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ValidationException` | Creates an OpenSearch Ingestion pipeline. For more information, see Creating Amazon OpenSearch Ingestion pipelines . |
| `CreatePipelineEndpoint` | `POST /2022-01-01/osis/createPipelineEndpoint` | - | `PipelineArn`, `VpcOptions` | - | `CreatePipelineEndpointResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Creates a VPC endpoint for an OpenSearch Ingestion pipeline. Pipeline endpoints allow you to ingest data from your VPC into pipelines that you have access to. |
| `DeletePipeline` | `DELETE /2022-01-01/osis/deletePipeline/{PipelineName}` | - | `PipelineName` | - | `DeletePipelineResponse` | `AccessDeniedException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Deletes an OpenSearch Ingestion pipeline. For more information, see Deleting Amazon OpenSearch Ingestion pipelines . |
| `DeletePipelineEndpoint` | `DELETE /2022-01-01/osis/deletePipelineEndpoint/{EndpointId}` | `idempotent` | `EndpointId` | - | `DeletePipelineEndpointResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `ValidationException` | Deletes a VPC endpoint for an OpenSearch Ingestion pipeline. |
| `DeleteResourcePolicy` | `DELETE /2022-01-01/osis/resourcePolicy/{ResourceArn}` | `idempotent` | `ResourceArn` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Deletes a resource-based policy from an OpenSearch Ingestion resource. |
| `GetPipeline` | `GET /2022-01-01/osis/getPipeline/{PipelineName}` | - | `PipelineName` | - | `GetPipelineResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Retrieves information about an OpenSearch Ingestion pipeline. |
| `GetPipelineBlueprint` | `GET /2022-01-01/osis/getPipelineBlueprint/{BlueprintName}` | - | `BlueprintName` | - | `GetPipelineBlueprintResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Retrieves information about a specific blueprint for OpenSearch Ingestion. Blueprints are templates for the configuration needed for a CreatePipeline request. For more information, see Using blueprints to create a pi ... |
| `GetPipelineChangeProgress` | `GET /2022-01-01/osis/getPipelineChangeProgress/{PipelineName}` | - | `PipelineName` | - | `GetPipelineChangeProgressResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns progress information for the current change happening on an OpenSearch Ingestion pipeline. Currently, this operation only returns information when a pipeline is being created. For more information, see Tracki ... |
| `GetResourcePolicy` | `GET /2022-01-01/osis/resourcePolicy/{ResourceArn}` | - | `ResourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the resource-based policy attached to an OpenSearch Ingestion resource. |
| `ListPipelineBlueprints` | `POST /2022-01-01/osis/listPipelineBlueprints` | - | - | - | `ListPipelineBlueprintsResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `InvalidPaginationTokenException`, `ValidationException` | Retrieves a list of all available blueprints for Data Prepper. For more information, see Using blueprints to create a pipeline . |
| `ListPipelineEndpointConnections` | `GET /2022-01-01/osis/listPipelineEndpointConnections` | `paginated` | - | - | `ListPipelineEndpointConnectionsResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ValidationException` | Lists the pipeline endpoints connected to pipelines in your account. |
| `ListPipelineEndpoints` | `GET /2022-01-01/osis/listPipelineEndpoints` | `paginated` | - | - | `ListPipelineEndpointsResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ValidationException` | Lists all pipeline endpoints in your account. |
| `ListPipelines` | `GET /2022-01-01/osis/listPipelines` | `paginated` | - | - | `ListPipelinesResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `InvalidPaginationTokenException`, `ValidationException` | Lists all OpenSearch Ingestion pipelines in the current Amazon Web Services account and Region. For more information, see Viewing Amazon OpenSearch Ingestion pipelines . |
| `ListTagsForResource` | `GET /2022-01-01/osis/listTagsForResource` | - | `Arn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists all resource tags associated with an OpenSearch Ingestion pipeline. For more information, see Tagging Amazon OpenSearch Ingestion pipelines . |
| `PutResourcePolicy` | `PUT /2022-01-01/osis/resourcePolicy/{ResourceArn}` | `idempotent` | `ResourceArn`, `Policy` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Attaches a resource-based policy to an OpenSearch Ingestion resource. Resource-based policies grant permissions to principals to perform actions on the resource. |
| `RevokePipelineEndpointConnections` | `POST /2022-01-01/osis/revokePipelineEndpointConnections` | `idempotent` | `PipelineArn`, `EndpointIds` | - | `RevokePipelineEndpointConnectionsResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ValidationException` | Revokes pipeline endpoints from specified endpoint IDs. |
| `StartPipeline` | `PUT /2022-01-01/osis/startPipeline/{PipelineName}` | - | `PipelineName` | - | `StartPipelineResponse` | `AccessDeniedException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Starts an OpenSearch Ingestion pipeline. For more information, see Starting an OpenSearch Ingestion pipeline . |
| `StopPipeline` | `PUT /2022-01-01/osis/stopPipeline/{PipelineName}` | - | `PipelineName` | - | `StopPipelineResponse` | `AccessDeniedException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Stops an OpenSearch Ingestion pipeline. For more information, see Stopping an OpenSearch Ingestion pipeline . |
| `TagResource` | `POST /2022-01-01/osis/tagResource` | - | `Arn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Tags an OpenSearch Ingestion pipeline. For more information, see Tagging Amazon OpenSearch Ingestion pipelines . |
| `UntagResource` | `POST /2022-01-01/osis/untagResource` | - | `Arn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Removes one or more tags from an OpenSearch Ingestion pipeline. For more information, see Tagging Amazon OpenSearch Ingestion pipelines . |
| `UpdatePipeline` | `PUT /2022-01-01/osis/updatePipeline/{PipelineName}` | - | `PipelineName` | - | `UpdatePipelineResponse` | `AccessDeniedException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Updates an OpenSearch Ingestion pipeline. For more information, see Updating Amazon OpenSearch Ingestion pipelines . |
| `ValidatePipeline` | `POST /2022-01-01/osis/validatePipeline` | - | `PipelineConfigurationBody` | - | `ValidatePipelineResponse` | `AccessDeniedException`, `DisabledOperationException`, `InternalException`, `ValidationException` | Checks whether an OpenSearch Ingestion pipeline configuration is valid prior to creation. For more information, see Creating Amazon OpenSearch Ingestion pipelines . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetPipelineBlueprint` | - | `Format -> format` | - | - |
| `ListPipelineEndpointConnections` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListPipelineEndpoints` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListPipelines` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListTagsForResource` | - | `Arn -> arn` | - | - |
| `TagResource` | - | `Arn -> arn` | - | - |
| `UntagResource` | - | `Arn -> arn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have permissions to access the resource. |
| `ConflictException` | `structure` | message | The client attempted to remove a resource that is currently in use. |
| `DisabledOperationException` | `structure` | message | Exception is thrown when an operation has been disabled. |
| `InternalException` | `structure` | message | The request failed because of an unknown error, exception, or failure (the failure is internal to the service). |
| `InvalidPaginationTokenException` | `structure` | message | An invalid pagination token provided in the request. |
| `LimitExceededException` | `structure` | message | You attempted to create more than the allowed number of tags. |
| `ResourceAlreadyExistsException` | `structure` | message | You attempted to create a resource that already exists. |
| `ResourceNotFoundException` | `structure` | message | You attempted to access or delete a resource that does not exist. |
| `ValidationException` | `structure` | message | An exception for missing or invalid input fields. |
| `CreatePipelineRequest` | `structure` | PipelineName, MinUnits, MaxUnits, PipelineConfigurationBody, LogPublishingOptions, VpcOptions, BufferOptions, EncryptionAtRestOptions, Tags, PipelineRoleArn | - |
| `CreatePipelineResponse` | `structure` | Pipeline | - |
| `CreatePipelineEndpointRequest` | `structure` | PipelineArn, VpcOptions | - |
| `CreatePipelineEndpointResponse` | `structure` | PipelineArn, EndpointId, Status, VpcId | - |
| `DeletePipelineRequest` | `structure` | PipelineName | - |
| `DeletePipelineResponse` | `structure` | **empty (no members)** | - |
| `DeletePipelineEndpointRequest` | `structure` | EndpointId | - |
| `DeletePipelineEndpointResponse` | `structure` | **empty (no members)** | - |
| `DeleteResourcePolicyRequest` | `structure` | ResourceArn | - |
| `DeleteResourcePolicyResponse` | `structure` | **empty (no members)** | - |
| `GetPipelineRequest` | `structure` | PipelineName | - |
| `GetPipelineResponse` | `structure` | Pipeline | - |
| `GetPipelineBlueprintRequest` | `structure` | BlueprintName, Format | - |
| `GetPipelineBlueprintResponse` | `structure` | Blueprint, Format | - |
| `GetPipelineChangeProgressRequest` | `structure` | PipelineName | - |
| `GetPipelineChangeProgressResponse` | `structure` | ChangeProgressStatuses | - |
| `GetResourcePolicyRequest` | `structure` | ResourceArn | - |
| `GetResourcePolicyResponse` | `structure` | ResourceArn, Policy | - |
| `ListPipelineBlueprintsRequest` | `structure` | **empty (no members)** | - |
| `ListPipelineBlueprintsResponse` | `structure` | Blueprints | - |
| `ListPipelineEndpointConnectionsRequest` | `structure` | MaxResults, NextToken | - |
| `ListPipelineEndpointConnectionsResponse` | `structure` | NextToken, PipelineEndpointConnections | - |
| `ListPipelineEndpointsRequest` | `structure` | MaxResults, NextToken | - |
| `ListPipelineEndpointsResponse` | `structure` | NextToken, PipelineEndpoints | - |
| `ListPipelinesRequest` | `structure` | MaxResults, NextToken | - |
| `ListPipelinesResponse` | `structure` | NextToken, Pipelines | - |
| `ListTagsForResourceRequest` | `structure` | Arn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `PutResourcePolicyRequest` | `structure` | ResourceArn, Policy | - |
| `PutResourcePolicyResponse` | `structure` | ResourceArn, Policy | - |
| `RevokePipelineEndpointConnectionsRequest` | `structure` | PipelineArn, EndpointIds | - |
| `ChangeProgressStageStatuses` | `enum` | PENDING, IN_PROGRESS, COMPLETED, FAILED | - |
| `ChangeProgressStatuses` | `enum` | PENDING, IN_PROGRESS, COMPLETED, FAILED | - |
| `PipelineEndpointStatus` | `enum` | CREATING, ACTIVE, CREATE_FAILED, DELETING, REVOKING, REVOKED | - |
| `PipelineStatus` | `enum` | CREATING, ACTIVE, UPDATING, DELETING, CREATE_FAILED, UPDATE_FAILED, STARTING, START_FAILED, STOPPING, STOPPED | - |
| `VpcEndpointManagement` | `enum` | CUSTOMER, SERVICE | - |
| `VpcEndpointServiceName` | `enum` | OPENSEARCH_SERVERLESS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
