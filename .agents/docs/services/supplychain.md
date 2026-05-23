# AWS Supply Chain

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Supply Chain is a cloud-based application that works with your enterprise resource planning (ERP) and supply chain management systems. Using AWS Supply Chain, you can connect and extract your inventory, supply, and demand related data from existing ERP or supply chain systems into a single data model. The AWS Supply Chain API supports configuration data import for Supply Planning. All AWS Supply chain API operations are Amazon-authenticated and certificate-signed. They not only require the use of the AWS SDK, but also allow for the exclusive use of AWS Identity and Access Management users and roles to help facilitate access, trust, and permission policies.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Supply Chain workflows in the local mock. Key resources include `BillOfMaterialsImportJobResource`, `DataIntegrationFlowResource`, `DataLakeDatasetResource`, `DataLakeNamespaceResource`, `InstanceResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetBillOfMaterialsImportJob`, `GetDataIntegrationEvent`, `GetDataIntegrationFlow`, `GetDataIntegrationFlowExecution`, `ListDataIntegrationEvents`, `ListDataIntegrationFlowExecutions`.

## Service Identity and Protocol

- AWS model slug: `supplychain`
- AWS SDK for Rust slug: `supplychain`
- Model version: `2024-01-01`
- Model file: `vendor/api-models-aws/models/supplychain/service/2024-01-01/supplychain-2024-01-01.json`
- SDK ID: `SupplyChain`
- Endpoint prefix: `scn`
- ARN namespace: `scn`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (7), `List` (7), `Create` (5), `Delete` (4), `Update` (4), `Send` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateBillOfMaterialsImportJob`, `CreateDataIntegrationFlow`, `CreateDataLakeDataset`, `CreateDataLakeNamespace`, `CreateInstance`, `DeleteDataIntegrationFlow`, `DeleteDataLakeDataset`, `DeleteDataLakeNamespace`, `DeleteInstance`, `TagResource`, `UntagResource`, `UpdateDataIntegrationFlow`, `UpdateDataLakeDataset`, `UpdateDataLakeNamespace`, `UpdateInstance`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBillOfMaterialsImportJob`, `GetDataIntegrationEvent`, `GetDataIntegrationFlow`, `GetDataIntegrationFlowExecution`, `GetDataLakeDataset`, `GetDataLakeNamespace`, `GetInstance`, `ListDataIntegrationEvents`, `ListDataIntegrationFlowExecutions`, `ListDataIntegrationFlows`, `ListDataLakeDatasets`, `ListDataLakeNamespaces`, `ListInstances`, `ListTagsForResource`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateBillOfMaterialsImportJob`, `GetBillOfMaterialsImportJob`, `GetDataIntegrationFlowExecution`, `ListDataIntegrationFlowExecutions`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 30 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`, `EC2/VPC`, `STS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `BillOfMaterialsImportJobResource` | `instanceId`, `jobId` | create: `CreateBillOfMaterialsImportJob`; read: `GetBillOfMaterialsImportJob` | - | - |
| `DataIntegrationFlowResource` | `instanceId`, `name` | put: `CreateDataIntegrationFlow`; read: `GetDataIntegrationFlow`; update: `UpdateDataIntegrationFlow`; delete: `DeleteDataIntegrationFlow`; list: `ListDataIntegrationFlows` | - | - |
| `DataLakeDatasetResource` | `instanceId`, `name`, `namespace` | put: `CreateDataLakeDataset`; read: `GetDataLakeDataset`; update: `UpdateDataLakeDataset`; delete: `DeleteDataLakeDataset`; list: `ListDataLakeDatasets` | - | - |
| `DataLakeNamespaceResource` | `instanceId`, `name` | put: `CreateDataLakeNamespace`; read: `GetDataLakeNamespace`; update: `UpdateDataLakeNamespace`; delete: `DeleteDataLakeNamespace`; list: `ListDataLakeNamespaces` | - | - |
| `InstanceResource` | `instanceId` | create: `CreateInstance`; read: `GetInstance`; update: `UpdateInstance`; delete: `DeleteInstance`; list: `ListInstances` | - | - |
## Operation Groups

### List

- Operations: `ListDataIntegrationEvents`, `ListDataIntegrationFlowExecutions`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: `instanceId`

### Get

- Operations: `GetDataIntegrationEvent`, `GetDataIntegrationFlowExecution`
- Traits: `readonly` (2)
- Common required input members in this group: `instanceId`

### Send

- Operations: `SendDataIntegrationEvent`
- Traits: `idempotent` (1), `idempotency-token` (1)
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
| `GetDataIntegrationEvent` | `GET /api-data/data-integration/instance/{instanceId}/data-integration-events/{eventId}` | `readonly` | `instanceId`, `eventId` | - | `GetDataIntegrationEventResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically view an Amazon Web Services Supply Chain Data Integration Event. Developers can view the eventType, eventGroupId, eventTimestamp, datasetTarget, datasetLoadExecution. |
| `GetDataIntegrationFlowExecution` | `GET /api-data/data-integration/instance/{instanceId}/data-integration-flows/{flowName}/executions/{executionId}` | `readonly` | `instanceId`, `flowName`, `executionId` | - | `GetDataIntegrationFlowExecutionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the flow execution. |
| `ListDataIntegrationEvents` | `GET /api-data/data-integration/instance/{instanceId}/data-integration-events` | `readonly`, `paginated` | `instanceId` | - | `ListDataIntegrationEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically list all data integration events for the provided Amazon Web Services Supply Chain instance. |
| `ListDataIntegrationFlowExecutions` | `GET /api-data/data-integration/instance/{instanceId}/data-integration-flows/{flowName}/executions` | `readonly`, `paginated` | `instanceId`, `flowName` | - | `ListDataIntegrationFlowExecutionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List flow executions. |
| `ListTagsForResource` | `GET /api/tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all the tags for an Amazon Web ServicesSupply Chain resource. You can list all the tags added to a resource. By listing the tags, developers can view the tag level information on a resource and perform actions s ... |
| `SendDataIntegrationEvent` | `POST /api-data/data-integration/instance/{instanceId}/data-integration-events` | `idempotent`, `idempotency-token` | `instanceId`, `eventType`, `data`, `eventGroupId` | `clientToken` | `SendDataIntegrationEventResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Send the data payload for the event with real-time data for analysis or monitoring. The real-time data events are stored in an Amazon Web Services service before being processed and stored in data lake. |
| `TagResource` | `POST /api/tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | You can create tags during or after creating a resource such as instance, data flow, or dataset in AWS Supply chain. During the data ingestion process, you can add tags such as dev, test, or prod to data flows create ... |
| `UntagResource` | `DELETE /api/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | You can delete tags for an Amazon Web Services Supply chain resource such as instance, data flow, or dataset in AWS Supply Chain. During the data ingestion process, you can delete tags such as dev, test, or prod to d ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListDataIntegrationEvents` | - | `eventType -> eventType`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDataIntegrationFlowExecutions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have the required privileges to perform this action. |
| `ConflictException` | `structure` | message | Updating or deleting a resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | message | Unexpected error during processing of request. |
| `ResourceNotFoundException` | `structure` | message | Request references a resource which does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | Request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message | Request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input does not satisfy the constraints specified by an AWS service. |
| `GetDataIntegrationEventRequest` | `structure` | instanceId, eventId | The request parameters for GetDataIntegrationEvent. |
| `GetDataIntegrationEventResponse` | `structure` | event | The response parameters for GetDataIntegrationEvent. |
| `GetDataIntegrationFlowExecutionRequest` | `structure` | instanceId, flowName, executionId | The request parameters of GetFlowExecution. |
| `GetDataIntegrationFlowExecutionResponse` | `structure` | flowExecution | The response parameters of GetFlowExecution. |
| `ListDataIntegrationEventsRequest` | `structure` | instanceId, eventType, nextToken, maxResults | The request parameters for ListDataIntegrationEvents. |
| `ListDataIntegrationEventsResponse` | `structure` | events, nextToken | The response parameters for ListDataIntegrationEvents. |
| `ListDataIntegrationFlowExecutionsRequest` | `structure` | instanceId, flowName, nextToken, maxResults | The request parameters of ListFlowExecutions. |
| `ListDataIntegrationFlowExecutionsResponse` | `structure` | flowExecutions, nextToken | The response parameters of ListFlowExecutions. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | The request parameters of ListTagsForResource. |
| `ListTagsForResourceResponse` | `structure` | tags | The response parameters of ListTagsForResource. |
| `SendDataIntegrationEventRequest` | `structure` | instanceId, eventType, data, eventGroupId, eventTimestamp, clientToken, datasetTarget | The request parameters for SendDataIntegrationEvent. |
| `SendDataIntegrationEventResponse` | `structure` | eventId | The response parameters for SendDataIntegrationEvent. |
| `TagResourceRequest` | `structure` | resourceArn, tags | The request parameters of TagResource. |
| `TagResourceResponse` | `structure` | **empty (no members)** | The response parameters for TagResource. |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | The request parameters of UntagResource. |
| `UntagResourceResponse` | `structure` | **empty (no members)** | The response parameters of UntagResource. |
| `ConfigurationJobStatus` | `enum` | NEW, FAILED, IN_PROGRESS, QUEUED, SUCCESS | The status of the job. |
| `DataIntegrationEventDatasetLoadStatus` | `enum` | SUCCEEDED, IN_PROGRESS, FAILED | - |
| `DataIntegrationEventDatasetOperationType` | `enum` | APPEND, UPSERT, DELETE | - |
| `DataIntegrationEventType` | `enum` | FORECAST, INVENTORY_LEVEL, INBOUND_ORDER, INBOUND_ORDER_LINE, INBOUND_ORDER_LINE_SCHEDULE, OUTBOUND_ORDER_LINE, OUTBOUND_SHIPMENT, PROCESS_HEADER, PROCESS_OPERATION, PROCESS_PRODUCT, RESERVATION, SHIPMENT, ... (+4) | - |
| `DataIntegrationFlowDedupeStrategyType` | `enum` | FIELD_PRIORITY | - |
| `DataIntegrationFlowExecutionStatus` | `enum` | SUCCEEDED, IN_PROGRESS, FAILED | - |
| `DataIntegrationFlowFieldPriorityDedupeSortOrder` | `enum` | ASC, DESC | - |
| `DataIntegrationFlowFileType` | `enum` | CSV, PARQUET, JSON | - |
| `DataIntegrationFlowLoadType` | `enum` | INCREMENTAL, REPLACE | - |
| `DataIntegrationFlowSourceType` | `enum` | S3, DATASET | - |
| `DataIntegrationFlowTargetType` | `enum` | S3, DATASET | - |
| `DataIntegrationFlowTransformationType` | `enum` | SQL, NONE | - |
| `DataLakeDatasetPartitionTransformType` | `enum` | YEAR, MONTH, DAY, HOUR, IDENTITY | - |
| `DataLakeDatasetSchemaFieldType` | `enum` | INT, DOUBLE, STRING, TIMESTAMP, LONG | - |
| `InstanceState` | `enum` | INITIALIZING, ACTIVE, CREATE_FAILED, DELETE_FAILED, DELETING, DELETED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
