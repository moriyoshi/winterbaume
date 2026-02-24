# Amazon Neptune Graph

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Neptune Analytics is a new analytics database engine for Amazon Neptune that helps customers get to insights faster by quickly processing large amounts of graph data, invoking popular graph analytic algorithms in low-latency queries, and getting analytics results in seconds.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Neptune Graph resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Neptune Graph workflows in the local mock. Key resources include `GraphResource`, `PrivateGraphEndpointResource`, `SnapshotResource`, `TaskResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Cancel`, `Delete` operation families, including `GetExportTask`, `GetGraph`, `GetGraphSnapshot`, `GetGraphSummary`, `ListExportTasks`, `ListGraphSnapshots`.

## Service Identity and Protocol

- AWS model slug: `neptune-graph`
- AWS SDK for Rust slug: `neptunegraph`
- Model version: `2023-11-29`
- Model file: `vendor/api-models-aws/models/neptune-graph/service/2023-11-29/neptune-graph-2023-11-29.json`
- SDK ID: `Neptune Graph`
- Endpoint prefix: `-`
- ARN namespace: `neptune-graph`
- CloudFormation name: `-`
- CloudTrail event source: `neptune-graph.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `ApiType`, `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (7), `List` (7), `Create` (4), `Cancel` (3), `Delete` (3), `Start` (3), `Execute` (1), `Reset` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelExportTask`, `CancelImportTask`, `CancelQuery`, `CreateGraph`, `CreateGraphSnapshot`, `CreateGraphUsingImportTask`, `CreatePrivateGraphEndpoint`, `DeleteGraph`, `DeleteGraphSnapshot`, `DeletePrivateGraphEndpoint`, `RestoreGraphFromSnapshot`, `StartExportTask`, `StartGraph`, `StartImportTask`, `StopGraph`, `TagResource`, `UntagResource`, `UpdateGraph`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetExportTask`, `GetGraph`, `GetGraphSnapshot`, `GetGraphSummary`, `GetImportTask`, `GetPrivateGraphEndpoint`, `GetQuery`, `ListExportTasks`, `ListGraphSnapshots`, `ListGraphs`, `ListImportTasks`, `ListPrivateGraphEndpoints`, `ListQueries`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelExportTask`, `CancelImportTask`, `CancelQuery`, `CreateGraphUsingImportTask`, `GetExportTask`, `GetImportTask`, `ListExportTasks`, `ListImportTasks`, `StartExportTask`, `StartGraph`, `StartImportTask`, `StopGraph`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 34 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `GraphResource` | - | - | `CreateGraph`, `DeleteGraph`, `GetGraph`, `ListGraphs`, `ResetGraph`, `RestoreGraphFromSnapshot`, `StartGraph`, `StopGraph`, `UpdateGraph` | - |
| `PrivateGraphEndpointResource` | - | - | `CreatePrivateGraphEndpoint`, `DeletePrivateGraphEndpoint`, `GetPrivateGraphEndpoint`, `ListPrivateGraphEndpoints` | - |
| `SnapshotResource` | - | - | `CreateGraphSnapshot`, `DeleteGraphSnapshot`, `GetGraphSnapshot`, `ListGraphSnapshots` | - |
| `TaskResource` | - | - | `CancelExportTask`, `CancelImportTask`, `CreateGraphUsingImportTask`, `GetExportTask`, `GetImportTask`, `ListExportTasks`, `ListImportTasks`, `StartExportTask`, `StartImportTask` | - |
## Operation Groups

### Get

- Operations: `GetExportTask`, `GetGraph`, `GetGraphSnapshot`, `GetGraphSummary`, `GetImportTask`, `GetPrivateGraphEndpoint`, `GetQuery`
- Traits: `endpoint-bound` (2), `readonly` (7)
- Common required input members in this group: `graphIdentifier`, `queryId`, `snapshotIdentifier`, `taskIdentifier`, `vpcId`

### List

- Operations: `ListExportTasks`, `ListGraphSnapshots`, `ListGraphs`, `ListImportTasks`, `ListPrivateGraphEndpoints`, `ListQueries`, `ListTagsForResource`
- Traits: `endpoint-bound` (1), `paginated` (5), `readonly` (7)
- Common required input members in this group: `graphIdentifier`, `maxResults`, `resourceArn`

### Create

- Operations: `CreateGraph`, `CreateGraphSnapshot`, `CreateGraphUsingImportTask`, `CreatePrivateGraphEndpoint`
- Common required input members in this group: `graphIdentifier`, `graphName`, `provisionedMemory`, `roleArn`, `snapshotName`, `source`

### Cancel

- Operations: `CancelExportTask`, `CancelImportTask`, `CancelQuery`
- Traits: `endpoint-bound` (1), `idempotent` (1)
- Common required input members in this group: `graphIdentifier`, `queryId`, `taskIdentifier`

### Delete

- Operations: `DeleteGraph`, `DeleteGraphSnapshot`, `DeletePrivateGraphEndpoint`
- Common required input members in this group: `graphIdentifier`, `skipSnapshot`, `snapshotIdentifier`, `vpcId`

### Start

- Operations: `StartExportTask`, `StartGraph`, `StartImportTask`
- Common required input members in this group: `destination`, `format`, `graphIdentifier`, `kmsKeyIdentifier`, `roleArn`, `source`

### Execute

- Operations: `ExecuteQuery`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `graphIdentifier`, `language`, `queryString`

### Reset

- Operations: `ResetGraph`
- Common required input members in this group: `graphIdentifier`, `skipSnapshot`

### Restore

- Operations: `RestoreGraphFromSnapshot`
- Common required input members in this group: `graphName`, `snapshotIdentifier`

### Stop

- Operations: `StopGraph`
- Common required input members in this group: `graphIdentifier`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateGraph`
- Common required input members in this group: `graphIdentifier`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelExportTask` | `DELETE /exporttasks/{taskIdentifier}` | - | `taskIdentifier` | - | `CancelExportTaskOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancel the specified export task. |
| `CancelImportTask` | `DELETE /importtasks/{taskIdentifier}` | - | `taskIdentifier` | - | `CancelImportTaskOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified import task. |
| `CancelQuery` | `DELETE /queries/{queryId}` | `idempotent`, `endpoint-bound` | `graphIdentifier`, `queryId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels a specified query. |
| `CreateGraph` | `POST /graphs` | - | `graphName`, `provisionedMemory` | - | `CreateGraphOutput` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Neptune Analytics graph. |
| `CreateGraphSnapshot` | `POST /snapshots` | - | `graphIdentifier`, `snapshotName` | - | `CreateGraphSnapshotOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a snapshot of the specific graph. |
| `CreateGraphUsingImportTask` | `POST /importtasks` | - | `graphName`, `roleArn`, `source` | - | `CreateGraphUsingImportTaskOutput` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Neptune Analytics graph and imports data into it, either from Amazon Simple Storage Service (S3) or from a Neptune database or a Neptune database snapshot. The data can be loaded from files in S3 that in either the Gremlin CSV format or the... |
| `CreatePrivateGraphEndpoint` | `POST /graphs/{graphIdentifier}/endpoints/` | - | `graphIdentifier` | - | `CreatePrivateGraphEndpointOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a private graph endpoint to allow private access to the graph from within a VPC. You can attach security groups to the private graph endpoint. |
| `DeleteGraph` | `DELETE /graphs/{graphIdentifier}` | - | `graphIdentifier`, `skipSnapshot` | - | `DeleteGraphOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified graph. Graphs cannot be deleted if delete-protection is enabled. |
| `DeleteGraphSnapshot` | `DELETE /snapshots/{snapshotIdentifier}` | - | `snapshotIdentifier` | - | `DeleteGraphSnapshotOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified graph snapshot. |
| `DeletePrivateGraphEndpoint` | `DELETE /graphs/{graphIdentifier}/endpoints/{vpcId}` | - | `graphIdentifier`, `vpcId` | - | `DeletePrivateGraphEndpointOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a private graph endpoint. |
| `ExecuteQuery` | `POST /queries` | `endpoint-bound` | `graphIdentifier`, `language`, `queryString` | - | `ExecuteQueryOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `UnprocessableException`, `ValidationException` | Execute an openCypher query. When invoking this operation in a Neptune Analytics cluster, the IAM user or role making the request must have a policy attached that allows one of the following IAM actions in that cluster, depending on the query... |
| `GetExportTask` | `GET /exporttasks/{taskIdentifier}` | `readonly` | `taskIdentifier` | - | `GetExportTaskOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a specified export task. |
| `GetGraph` | `GET /graphs/{graphIdentifier}` | `readonly` | `graphIdentifier` | - | `GetGraphOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a specified graph. |
| `GetGraphSnapshot` | `GET /snapshots/{snapshotIdentifier}` | `readonly` | `snapshotIdentifier` | - | `GetGraphSnapshotOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a specified graph snapshot. |
| `GetGraphSummary` | `GET /summary` | `readonly`, `endpoint-bound` | `graphIdentifier` | - | `GetGraphSummaryOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a graph summary for a property graph. |
| `GetImportTask` | `GET /importtasks/{taskIdentifier}` | `readonly` | `taskIdentifier` | - | `GetImportTaskOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a specified import task. |
| `GetPrivateGraphEndpoint` | `GET /graphs/{graphIdentifier}/endpoints/{vpcId}` | `readonly` | `graphIdentifier`, `vpcId` | - | `GetPrivateGraphEndpointOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a specified private endpoint. |
| `GetQuery` | `GET /queries/{queryId}` | `readonly`, `endpoint-bound` | `graphIdentifier`, `queryId` | - | `GetQueryOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the status of a specified query. When invoking this operation in a Neptune Analytics cluster, the IAM user or role making the request must have the `neptune-graph:GetQueryStatus` IAM action attached. |
| `ListExportTasks` | `GET /exporttasks` | `readonly`, `paginated` | - | - | `ListExportTasksOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of export tasks. |
| `ListGraphSnapshots` | `GET /snapshots` | `readonly`, `paginated` | - | - | `ListGraphSnapshotsOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists available snapshots of a specified Neptune Analytics graph. |
| `ListGraphs` | `GET /graphs` | `readonly`, `paginated` | - | - | `ListGraphsOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists available Neptune Analytics graphs. |
| `ListImportTasks` | `GET /importtasks` | `readonly`, `paginated` | - | - | `ListImportTasksOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists import tasks. |
| `ListPrivateGraphEndpoints` | `GET /graphs/{graphIdentifier}/endpoints/` | `readonly`, `paginated` | `graphIdentifier` | - | `ListPrivateGraphEndpointsOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists private endpoints for a specified Neptune Analytics graph. |
| `ListQueries` | `GET /queries` | `readonly`, `endpoint-bound` | `graphIdentifier`, `maxResults` | - | `ListQueriesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists active openCypher queries. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags associated with a specified resource. |
| `ResetGraph` | `PUT /graphs/{graphIdentifier}` | - | `graphIdentifier`, `skipSnapshot` | - | `ResetGraphOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Empties the data from a specified Neptune Analytics graph. |
| `RestoreGraphFromSnapshot` | `POST /snapshots/{snapshotIdentifier}/restore` | - | `graphName`, `snapshotIdentifier` | - | `RestoreGraphFromSnapshotOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Restores a graph from a snapshot. |
| `StartExportTask` | `POST /exporttasks` | - | `destination`, `format`, `graphIdentifier`, `kmsKeyIdentifier`, `roleArn` | - | `StartExportTaskOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Export data from an existing Neptune Analytics graph to Amazon S3. The graph state should be `AVAILABLE`. |
| `StartGraph` | `POST /graphs/{graphIdentifier}/start` | - | `graphIdentifier` | - | `StartGraphOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts the specific graph. |
| `StartImportTask` | `POST /graphs/{graphIdentifier}/importtasks` | - | `graphIdentifier`, `roleArn`, `source` | - | `StartImportTaskOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Import data into existing Neptune Analytics graph from Amazon Simple Storage Service (S3). The graph needs to be empty and in the AVAILABLE state. |
| `StopGraph` | `POST /graphs/{graphIdentifier}/stop` | - | `graphIdentifier` | - | `StopGraphOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops the specific graph. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specified tags from the specified resource. |
| `UpdateGraph` | `PATCH /graphs/{graphIdentifier}` | - | `graphIdentifier` | - | `UpdateGraphOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of a specified Neptune Analytics graph |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | A failure occurred on the server. |
| `ThrottlingException` | `structure` | `message` | The exception was interrupted by throttling. |
| `ValidationException` | `structure` | `message`, `reason` | A resource could not be validated. |
| `ResourceNotFoundException` | `structure` | `message` | A specified resource could not be located. |
| `ConflictException` | `structure` | `message`, `reason` | Raised when a conflict is encountered. |
| `AccessDeniedException` | `structure` | `message` | Raised in case of an authentication or authorization failure. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | A service quota was exceeded. |
| `CancelExportTaskInput` | `structure` | `taskIdentifier` | - |
| `CancelExportTaskOutput` | `structure` | `destination`, `format`, `graphId`, `kmsKeyIdentifier`, `parquetType`, `roleArn`, `status`, `statusReason`, `taskId` | - |
| `CancelImportTaskInput` | `structure` | `taskIdentifier` | - |
| `CancelImportTaskOutput` | `structure` | `format`, `graphId`, `parquetType`, `roleArn`, `source`, `status`, `taskId` | - |
| `CancelQueryInput` | `structure` | `graphIdentifier`, `queryId` | - |
| `CreateGraphInput` | `structure` | `deletionProtection`, `graphName`, `kmsKeyIdentifier`, `provisionedMemory`, `publicConnectivity`, `replicaCount`, `tags`, `vectorSearchConfiguration` | - |
| `CreateGraphOutput` | `structure` | `arn`, `buildNumber`, `createTime`, `deletionProtection`, `endpoint`, `id`, `kmsKeyIdentifier`, `name`, `provisionedMemory`, `publicConnectivity`, `replicaCount`, `sourceSnapshotId`, ... (+3) | - |
| `CreateGraphSnapshotInput` | `structure` | `graphIdentifier`, `snapshotName`, `tags` | - |
| `CreateGraphSnapshotOutput` | `structure` | `arn`, `id`, `kmsKeyIdentifier`, `name`, `snapshotCreateTime`, `sourceGraphId`, `status` | - |
| `CreateGraphUsingImportTaskInput` | `structure` | `blankNodeHandling`, `deletionProtection`, `failOnError`, `format`, `graphName`, `importOptions`, `kmsKeyIdentifier`, `maxProvisionedMemory`, `minProvisionedMemory`, `parquetType`, `publicConnectivity`, `replicaCount`, ... (+4) | - |
| `CreateGraphUsingImportTaskOutput` | `structure` | `format`, `graphId`, `importOptions`, `parquetType`, `roleArn`, `source`, `status`, `taskId` | - |
| `CreatePrivateGraphEndpointInput` | `structure` | `graphIdentifier`, `subnetIds`, `vpcId`, `vpcSecurityGroupIds` | - |
| `CreatePrivateGraphEndpointOutput` | `structure` | `status`, `subnetIds`, `vpcEndpointId`, `vpcId` | - |
| `DeleteGraphInput` | `structure` | `graphIdentifier`, `skipSnapshot` | - |
| `DeleteGraphOutput` | `structure` | `arn`, `buildNumber`, `createTime`, `deletionProtection`, `endpoint`, `id`, `kmsKeyIdentifier`, `name`, `provisionedMemory`, `publicConnectivity`, `replicaCount`, `sourceSnapshotId`, ... (+3) | - |
| `DeleteGraphSnapshotInput` | `structure` | `snapshotIdentifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
