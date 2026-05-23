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

### Get

- Operations: `GetBillOfMaterialsImportJob`, `GetDataIntegrationEvent`, `GetDataIntegrationFlow`, `GetDataIntegrationFlowExecution`, `GetDataLakeDataset`, `GetDataLakeNamespace`, `GetInstance`
- Traits: `readonly` (7)
- Common required input members in this group: `eventId`, `executionId`, `flowName`, `instanceId`, `jobId`, `name`, `namespace`

### List

- Operations: `ListDataIntegrationEvents`, `ListDataIntegrationFlowExecutions`, `ListDataIntegrationFlows`, `ListDataLakeDatasets`, `ListDataLakeNamespaces`, `ListInstances`, `ListTagsForResource`
- Traits: `paginated` (6), `readonly` (7)
- Common required input members in this group: `flowName`, `instanceId`, `namespace`, `resourceArn`

### Create

- Operations: `CreateBillOfMaterialsImportJob`, `CreateDataIntegrationFlow`, `CreateDataLakeDataset`, `CreateDataLakeNamespace`, `CreateInstance`
- Traits: `idempotency-token` (2), `idempotent` (5)
- Common required input members in this group: `instanceId`, `name`, `namespace`, `s3uri`, `sources`, `target`, `transformation`

### Delete

- Operations: `DeleteDataIntegrationFlow`, `DeleteDataLakeDataset`, `DeleteDataLakeNamespace`, `DeleteInstance`
- Traits: `idempotent` (4)
- Common required input members in this group: `instanceId`, `name`, `namespace`

### Update

- Operations: `UpdateDataIntegrationFlow`, `UpdateDataLakeDataset`, `UpdateDataLakeNamespace`, `UpdateInstance`
- Common required input members in this group: `instanceId`, `name`, `namespace`

### Send

- Operations: `SendDataIntegrationEvent`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `data`, `eventGroupId`, `eventType`, `instanceId`

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
| `CreateBillOfMaterialsImportJob` | `POST /api/configuration/instances/{instanceId}/bill-of-materials-import-jobs` | `idempotent`, `idempotency-token` | `instanceId`, `s3uri` | `clientToken` | `CreateBillOfMaterialsImportJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | CreateBillOfMaterialsImportJob creates an import job for the Product Bill Of Materials (BOM) entity. For information on the product_bom entity, see the AWS Supply Chain User Guide. |
| `CreateDataIntegrationFlow` | `PUT /api/data-integration/instance/{instanceId}/data-integration-flows/{name}` | `idempotent` | `instanceId`, `name`, `sources`, `target`, `transformation` | - | `CreateDataIntegrationFlowResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically create a data pipeline to ingest data from source systems such as Amazon S3 buckets, to a predefined Amazon Web Services Supply Chain dataset (product, inbound_order) or a temporary dataset along with the data transformation... |
| `CreateDataLakeDataset` | `PUT /api/datalake/instance/{instanceId}/namespaces/{namespace}/datasets/{name}` | `idempotent` | `instanceId`, `name`, `namespace` | - | `CreateDataLakeDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically create an Amazon Web Services Supply Chain data lake dataset. Developers can create the datasets using their pre-defined or custom schema for a given instance ID, namespace, and dataset name. |
| `CreateDataLakeNamespace` | `PUT /api/datalake/instance/{instanceId}/namespaces/{name}` | `idempotent` | `instanceId`, `name` | - | `CreateDataLakeNamespaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically create an Amazon Web Services Supply Chain data lake namespace. Developers can create the namespaces for a given instance ID. |
| `CreateInstance` | `POST /api/instance` | `idempotent`, `idempotency-token` | - | `clientToken` | `CreateInstanceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically create an Amazon Web Services Supply Chain instance by applying KMS keys and relevant information associated with the API without using the Amazon Web Services console. This is an asynchronous operation. |
| `DeleteDataIntegrationFlow` | `DELETE /api/data-integration/instance/{instanceId}/data-integration-flows/{name}` | `idempotent` | `instanceId`, `name` | - | `DeleteDataIntegrationFlowResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Enable you to programmatically delete an existing data pipeline for the provided Amazon Web Services Supply Chain instance and DataIntegrationFlow name. |
| `DeleteDataLakeDataset` | `DELETE /api/datalake/instance/{instanceId}/namespaces/{namespace}/datasets/{name}` | `idempotent` | `instanceId`, `name`, `namespace` | - | `DeleteDataLakeDatasetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically delete an Amazon Web Services Supply Chain data lake dataset. Developers can delete the existing datasets for a given instance ID, namespace, and instance name. |
| `DeleteDataLakeNamespace` | `DELETE /api/datalake/instance/{instanceId}/namespaces/{name}` | `idempotent` | `instanceId`, `name` | - | `DeleteDataLakeNamespaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically delete an Amazon Web Services Supply Chain data lake namespace and its underling datasets. Developers can delete the existing namespaces for a given instance ID and namespace name. |
| `DeleteInstance` | `DELETE /api/instance/{instanceId}` | `idempotent` | `instanceId` | - | `DeleteInstanceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically delete an Amazon Web Services Supply Chain instance by deleting the KMS keys and relevant information associated with the API without using the Amazon Web Services console. This is an asynchronous operation. |
| `GetBillOfMaterialsImportJob` | `GET /api/configuration/instances/{instanceId}/bill-of-materials-import-jobs/{jobId}` | `readonly` | `instanceId`, `jobId` | - | `GetBillOfMaterialsImportJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get status and details of a BillOfMaterialsImportJob. |
| `GetDataIntegrationEvent` | `GET /api-data/data-integration/instance/{instanceId}/data-integration-events/{eventId}` | `readonly` | `eventId`, `instanceId` | - | `GetDataIntegrationEventResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically view an Amazon Web Services Supply Chain Data Integration Event. Developers can view the eventType, eventGroupId, eventTimestamp, datasetTarget, datasetLoadExecution. |
| `GetDataIntegrationFlow` | `GET /api/data-integration/instance/{instanceId}/data-integration-flows/{name}` | `readonly` | `instanceId`, `name` | - | `GetDataIntegrationFlowResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically view a specific data pipeline for the provided Amazon Web Services Supply Chain instance and DataIntegrationFlow name. |
| `GetDataIntegrationFlowExecution` | `GET /api-data/data-integration/instance/{instanceId}/data-integration-flows/{flowName}/executions/{executionId}` | `readonly` | `executionId`, `flowName`, `instanceId` | - | `GetDataIntegrationFlowExecutionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the flow execution. |
| `GetDataLakeDataset` | `GET /api/datalake/instance/{instanceId}/namespaces/{namespace}/datasets/{name}` | `readonly` | `instanceId`, `name`, `namespace` | - | `GetDataLakeDatasetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically view an Amazon Web Services Supply Chain data lake dataset. Developers can view the data lake dataset information such as namespace, schema, and so on for a given instance ID, namespace, and dataset name. |
| `GetDataLakeNamespace` | `GET /api/datalake/instance/{instanceId}/namespaces/{name}` | `readonly` | `instanceId`, `name` | - | `GetDataLakeNamespaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically view an Amazon Web Services Supply Chain data lake namespace. Developers can view the data lake namespace information such as description for a given instance ID and namespace name. |
| `GetInstance` | `GET /api/instance/{instanceId}` | `readonly` | `instanceId` | - | `GetInstanceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically retrieve the information related to an Amazon Web Services Supply Chain instance ID. |
| `ListDataIntegrationEvents` | `GET /api-data/data-integration/instance/{instanceId}/data-integration-events` | `readonly`, `paginated` | `instanceId` | - | `ListDataIntegrationEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically list all data integration events for the provided Amazon Web Services Supply Chain instance. |
| `ListDataIntegrationFlowExecutions` | `GET /api-data/data-integration/instance/{instanceId}/data-integration-flows/{flowName}/executions` | `readonly`, `paginated` | `flowName`, `instanceId` | - | `ListDataIntegrationFlowExecutionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List flow executions. |
| `ListDataIntegrationFlows` | `GET /api/data-integration/instance/{instanceId}/data-integration-flows` | `readonly`, `paginated` | `instanceId` | - | `ListDataIntegrationFlowsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically list all data pipelines for the provided Amazon Web Services Supply Chain instance. |
| `ListDataLakeDatasets` | `GET /api/datalake/instance/{instanceId}/namespaces/{namespace}/datasets` | `readonly`, `paginated` | `instanceId`, `namespace` | - | `ListDataLakeDatasetsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically view the list of Amazon Web Services Supply Chain data lake datasets. Developers can view the datasets and the corresponding information such as namespace, schema, and so on for a given instance ID and namespace. |
| `ListDataLakeNamespaces` | `GET /api/datalake/instance/{instanceId}/namespaces` | `readonly`, `paginated` | `instanceId` | - | `ListDataLakeNamespacesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically view the list of Amazon Web Services Supply Chain data lake namespaces. Developers can view the namespaces and the corresponding information such as description for a given instance ID. |
| `ListInstances` | `GET /api/instance` | `readonly`, `paginated` | - | - | `ListInstancesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List all Amazon Web Services Supply Chain instances for a specific account. Enables you to programmatically list all Amazon Web Services Supply Chain instances based on their account ID, instance name, and state of the instance (active or delete). |
| `ListTagsForResource` | `GET /api/tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all the tags for an Amazon Web ServicesSupply Chain resource. You can list all the tags added to a resource. |
| `SendDataIntegrationEvent` | `POST /api-data/data-integration/instance/{instanceId}/data-integration-events` | `idempotent`, `idempotency-token` | `data`, `eventGroupId`, `eventType`, `instanceId` | `clientToken` | `SendDataIntegrationEventResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Send the data payload for the event with real-time data for analysis or monitoring. The real-time data events are stored in an Amazon Web Services service before being processed and stored in data lake. |
| `TagResource` | `POST /api/tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | You can create tags during or after creating a resource such as instance, data flow, or dataset in AWS Supply chain. During the data ingestion process, you can add tags such as dev, test, or prod to data flows created during the data ingestion process in the... |
| `UntagResource` | `DELETE /api/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | You can delete tags for an Amazon Web Services Supply chain resource such as instance, data flow, or dataset in AWS Supply Chain. During the data ingestion process, you can delete tags such as dev, test, or prod to data flows created during the data ingestion... |
| `UpdateDataIntegrationFlow` | `PATCH /api/data-integration/instance/{instanceId}/data-integration-flows/{name}` | - | `instanceId`, `name` | - | `UpdateDataIntegrationFlowResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically update an existing data pipeline to ingest data from the source systems such as, Amazon S3 buckets, to a predefined Amazon Web Services Supply Chain dataset (product, inbound_order) or a temporary dataset along with the data... |
| `UpdateDataLakeDataset` | `PATCH /api/datalake/instance/{instanceId}/namespaces/{namespace}/datasets/{name}` | - | `instanceId`, `name`, `namespace` | - | `UpdateDataLakeDatasetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically update an Amazon Web Services Supply Chain data lake dataset. Developers can update the description of a data lake dataset for a given instance ID, namespace, and dataset name. |
| `UpdateDataLakeNamespace` | `PATCH /api/datalake/instance/{instanceId}/namespaces/{name}` | - | `instanceId`, `name` | - | `UpdateDataLakeNamespaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically update an Amazon Web Services Supply Chain data lake namespace. Developers can update the description of a data lake namespace for a given instance ID and namespace name. |
| `UpdateInstance` | `PATCH /api/instance/{instanceId}` | - | `instanceId` | - | `UpdateInstanceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to programmatically update an Amazon Web Services Supply Chain instance description by providing all the relevant information such as account ID, instance ID and so on without using the AWS console. |

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
| `AccessDeniedException` | `structure` | `message` | You do not have the required privileges to perform this action. |
| `InternalServerException` | `structure` | `message` | Unexpected error during processing of request. |
| `ThrottlingException` | `structure` | `message` | Request was denied due to request throttling. |
| `ValidationException` | `structure` | `message` | The input does not satisfy the constraints specified by an AWS service. |
| `ResourceNotFoundException` | `structure` | `message` | Request references a resource which does not exist. |
| `ConflictException` | `structure` | `message` | Updating or deleting a resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `message` | Request would cause a service quota to be exceeded. |
| `CreateBillOfMaterialsImportJobRequest` | `structure` | `clientToken`, `instanceId`, `s3uri` | The request parameters for CreateBillOfMaterialsImportJob. |
| `CreateBillOfMaterialsImportJobResponse` | `structure` | `jobId` | The response parameters of CreateBillOfMaterialsImportJob. |
| `CreateDataIntegrationFlowRequest` | `structure` | `instanceId`, `name`, `sources`, `tags`, `target`, `transformation` | The request parameters for CreateDataIntegrationFlow. |
| `CreateDataIntegrationFlowResponse` | `structure` | `instanceId`, `name` | The response parameters for CreateDataIntegrationFlow. |
| `CreateDataLakeDatasetRequest` | `structure` | `description`, `instanceId`, `name`, `namespace`, `partitionSpec`, `schema`, `tags` | The request parameters for CreateDataLakeDataset. |
| `CreateDataLakeDatasetResponse` | `structure` | `dataset` | The response parameters of CreateDataLakeDataset. |
| `CreateDataLakeNamespaceRequest` | `structure` | `description`, `instanceId`, `name`, `tags` | The request parameters for CreateDataLakeNamespace. |
| `CreateDataLakeNamespaceResponse` | `structure` | `namespace` | The response parameters of CreateDataLakeNamespace. |
| `CreateInstanceRequest` | `structure` | `clientToken`, `instanceDescription`, `instanceName`, `kmsKeyArn`, `tags`, `webAppDnsDomain` | The request parameters for CreateInstance. |
| `CreateInstanceResponse` | `structure` | `instance` | The response parameters for CreateInstance. |
| `DeleteDataIntegrationFlowRequest` | `structure` | `instanceId`, `name` | The request parameters for DeleteDataIntegrationFlow. |
| `DeleteDataIntegrationFlowResponse` | `structure` | `instanceId`, `name` | The response parameters for DeleteDataIntegrationFlow. |
| `DeleteDataLakeDatasetRequest` | `structure` | `instanceId`, `name`, `namespace` | The request parameters of DeleteDataLakeDataset. |
| `DeleteDataLakeDatasetResponse` | `structure` | `instanceId`, `name`, `namespace` | The response parameters of DeleteDataLakeDataset. |
| `DeleteDataLakeNamespaceRequest` | `structure` | `instanceId`, `name` | The request parameters of DeleteDataLakeNamespace. |
| `DeleteDataLakeNamespaceResponse` | `structure` | `instanceId`, `name` | The response parameters of DeleteDataLakeNamespace. |
| `DeleteInstanceRequest` | `structure` | `instanceId` | The request parameters for DeleteInstance. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
