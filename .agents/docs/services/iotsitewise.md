# AWS IoT SiteWise

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the IoT SiteWise API Reference. IoT SiteWise is an Amazon Web Services service that connects Industrial Internet of Things (IIoT) devices to the power of the Amazon Web Services Cloud. For more information, see the IoT SiteWise User Guide. For information about IoT SiteWise quotas, see Quotas in the IoT SiteWise User Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS IoT SiteWise where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS IoT SiteWise by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS IoT SiteWise by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS IoT SiteWise workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Delete`, `Update`, `Create` operation families, including `ListAccessPolicies`, `ListActions`, `ListAssetModelCompositeModels`, `ListAssetModelProperties`, `DescribeAccessPolicy`, `DescribeAction`.

## Service Identity and Protocol

- AWS model slug: `iotsitewise`
- AWS SDK for Rust slug: `iotsitewise`
- Model version: `2019-12-02`
- Model file: `vendor/api-models-aws/models/iotsitewise/service/2019-12-02/iotsitewise-2019-12-02.json`
- SDK ID: `IoTSiteWise`
- Endpoint prefix: `iotsitewise`
- ARN namespace: `iotsitewise`
- CloudFormation name: `IoTSiteWise`
- CloudTrail event source: `iotsitewise.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (24), `Describe` (22), `Delete` (12), `Update` (12), `Create` (11), `Batch` (6), `Get` (4), `Put` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAssets`, `AssociateTimeSeriesToAssetProperty`, `BatchAssociateProjectAssets`, `BatchDisassociateProjectAssets`, `BatchGetAssetPropertyAggregates`, `BatchGetAssetPropertyValue`, `BatchGetAssetPropertyValueHistory`, `BatchPutAssetPropertyValue`, `CreateAccessPolicy`, `CreateAsset`, `CreateAssetModel`, `CreateAssetModelCompositeModel`, `CreateBulkImportJob`, `CreateComputationModel`, `CreateDashboard`, `CreateDataset`, `CreateGateway`, `CreatePortal`, `CreateProject`, `DeleteAccessPolicy`, ... (+31).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccessPolicy`, `DescribeAction`, `DescribeAsset`, `DescribeAssetCompositeModel`, `DescribeAssetModel`, `DescribeAssetModelCompositeModel`, `DescribeAssetModelInterfaceRelationship`, `DescribeAssetProperty`, `DescribeBulkImportJob`, `DescribeComputationModel`, `DescribeComputationModelExecutionSummary`, `DescribeDashboard`, `DescribeDataset`, `DescribeDefaultEncryptionConfiguration`, `DescribeExecution`, `DescribeGateway`, `DescribeGatewayCapabilityConfiguration`, `DescribeLoggingOptions`, `DescribePortal`, `DescribeProject`, ... (+30).
- Pagination is modelled for 29 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 38 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateBulkImportJob`, `DescribeBulkImportJob`, `DescribeComputationModelExecutionSummary`, `DescribeExecution`, `ListBulkImportJobs`, `ListExecutions`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 104 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `SNS`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAccessPolicies`, `ListActions`, `ListAssetModelCompositeModels`, `ListAssetModelProperties`, `ListAssetModels`, `ListAssetProperties`, `ListAssetRelationships`, `ListAssets`, `ListAssociatedAssets`, `ListBulkImportJobs`, `ListCompositionRelationships`, `ListComputationModelDataBindingUsages`, `ListComputationModelResolveToResources`, `ListComputationModels`, `ListDashboards`, `ListDatasets`, `ListExecutions`, `ListGateways`, `ListInterfaceRelationships`, `ListPortals`, `ListProjectAssets`, `ListProjects`, `ListTagsForResource`, `ListTimeSeries`
- Traits: `paginated` (22)
- Common required input members in this group: `targetResourceType`, `targetResourceId`, `assetModelId`, `assetId`, `projectId`

### Describe

- Operations: `DescribeAccessPolicy`, `DescribeAction`, `DescribeAsset`, `DescribeAssetCompositeModel`, `DescribeAssetModel`, `DescribeAssetModelCompositeModel`, `DescribeAssetModelInterfaceRelationship`, `DescribeAssetProperty`, `DescribeBulkImportJob`, `DescribeComputationModel`, `DescribeComputationModelExecutionSummary`, `DescribeDashboard`, `DescribeDataset`, `DescribeDefaultEncryptionConfiguration`, `DescribeExecution`, `DescribeGateway`, `DescribeGatewayCapabilityConfiguration`, `DescribeLoggingOptions`, `DescribePortal`, `DescribeProject`, `DescribeStorageConfiguration`, `DescribeTimeSeries`
- Common required input members in this group: `assetId`, `assetModelId`, `computationModelId`, `gatewayId`

### Delete

- Operations: `DeleteAccessPolicy`, `DeleteAsset`, `DeleteAssetModel`, `DeleteAssetModelCompositeModel`, `DeleteAssetModelInterfaceRelationship`, `DeleteComputationModel`, `DeleteDashboard`, `DeleteDataset`, `DeleteGateway`, `DeletePortal`, `DeleteProject`, `DeleteTimeSeries`
- Traits: `idempotency-token` (11)
- Common required input members in this group: `assetModelId`

### Update

- Operations: `UpdateAccessPolicy`, `UpdateAsset`, `UpdateAssetModel`, `UpdateAssetModelCompositeModel`, `UpdateAssetProperty`, `UpdateComputationModel`, `UpdateDashboard`, `UpdateDataset`, `UpdateGateway`, `UpdateGatewayCapabilityConfiguration`, `UpdatePortal`, `UpdateProject`
- Traits: `idempotency-token` (10)
- Common required input members in this group: `assetId`, `assetModelId`, `gatewayId`

### Create

- Operations: `CreateAccessPolicy`, `CreateAsset`, `CreateAssetModel`, `CreateAssetModelCompositeModel`, `CreateBulkImportJob`, `CreateComputationModel`, `CreateDashboard`, `CreateDataset`, `CreateGateway`, `CreatePortal`, `CreateProject`
- Traits: `idempotency-token` (9)
- Common required input members in this group: `assetModelId`

### Batch

- Operations: `BatchAssociateProjectAssets`, `BatchDisassociateProjectAssets`, `BatchGetAssetPropertyAggregates`, `BatchGetAssetPropertyValue`, `BatchGetAssetPropertyValueHistory`, `BatchPutAssetPropertyValue`
- Traits: `idempotency-token` (2), `paginated` (3)
- Common required input members in this group: `projectId`, `assetIds`, `entries`

### Get

- Operations: `GetAssetPropertyAggregates`, `GetAssetPropertyValue`, `GetAssetPropertyValueHistory`, `GetInterpolatedAssetPropertyValues`
- Traits: `paginated` (3)
- Common required input members in this group: -

### Put

- Operations: `PutAssetModelInterfaceRelationship`, `PutDefaultEncryptionConfiguration`, `PutLoggingOptions`, `PutStorageConfiguration`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Associate

- Operations: `AssociateAssets`, `AssociateTimeSeriesToAssetProperty`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `assetId`

### Disassociate

- Operations: `DisassociateAssets`, `DisassociateTimeSeriesFromAssetProperty`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `assetId`

### Execute

- Operations: `ExecuteAction`, `ExecuteQuery`
- Traits: `paginated` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Invoke

- Operations: `InvokeAssistant`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAssets` | `POST /assets/{assetId}/associate` | `idempotency-token` | `assetId`, `hierarchyId`, `childAssetId` | `clientToken` | `Unit` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Associates a child asset with the given parent asset through a hierarchy defined in the parent asset's model. For more information, see Associating assets in the IoT SiteWise User Guide . |
| `AssociateTimeSeriesToAssetProperty` | `POST /timeseries/associate` | `idempotency-token` | `alias`, `assetId`, `propertyId` | `clientToken` | `Unit` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Associates a time series (data stream) with an asset property. |
| `BatchAssociateProjectAssets` | `POST /projects/{projectId}/assets/associate` | `idempotency-token` | `projectId`, `assetIds` | `clientToken` | `BatchAssociateProjectAssetsResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Associates a group (batch) of assets with an IoT SiteWise Monitor project. |
| `BatchDisassociateProjectAssets` | `POST /projects/{projectId}/assets/disassociate` | `idempotency-token` | `projectId`, `assetIds` | `clientToken` | `BatchDisassociateProjectAssetsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Disassociates a group (batch) of assets from an IoT SiteWise Monitor project. |
| `BatchGetAssetPropertyAggregates` | `POST /properties/batch/aggregates` | `paginated` | `entries` | - | `BatchGetAssetPropertyAggregatesResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Gets aggregated values (for example, average, minimum, and maximum) for one or more asset properties. For more information, see Querying aggregates in the IoT SiteWise User Guide . |
| `BatchGetAssetPropertyValue` | `POST /properties/batch/latest` | `paginated` | `entries` | - | `BatchGetAssetPropertyValueResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Gets the current value for one or more asset properties. For more information, see Querying current values in the IoT SiteWise User Guide . |
| `BatchGetAssetPropertyValueHistory` | `POST /properties/batch/history` | `paginated` | `entries` | - | `BatchGetAssetPropertyValueHistoryResponse` | `InternalFailureException`, `InvalidRequestException`, `ServiceUnavailableException`, `ThrottlingException` | Gets the historical values for one or more asset properties. For more information, see Querying historical values in the IoT SiteWise User Guide . |
| `BatchPutAssetPropertyValue` | `POST /properties` | - | `entries` | - | `BatchPutAssetPropertyValueResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Sends a list of asset property values to IoT SiteWise. Each value is a timestamp-quality-value (TQV) data point. For more information, see Ingesting data using the API in the IoT SiteWise User Guide . To identify an ... |
| `CreateAccessPolicy` | `POST /access-policies` | `idempotency-token` | `accessPolicyIdentity`, `accessPolicyResource`, `accessPolicyPermission` | `clientToken` | `CreateAccessPolicyResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an access policy that grants the specified identity (IAM Identity Center user, IAM Identity Center group, or IAM user) access to the specified IoT SiteWise Monitor portal or project resource. Support for acce ... |
| `CreateAsset` | `POST /assets` | `idempotency-token` | `assetName`, `assetModelId` | `clientToken` | `CreateAssetResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an asset from an existing asset model. For more information, see Creating assets in the IoT SiteWise User Guide . |
| `CreateAssetModel` | `POST /asset-models` | `idempotency-token` | `assetModelName` | `clientToken` | `CreateAssetModelResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an asset model from specified property and hierarchy definitions. You create assets from asset models. With asset models, you can easily create assets of the same type that have standardized definitions. Each ... |
| `CreateAssetModelCompositeModel` | `POST /asset-models/{assetModelId}/composite-models` | `idempotency-token` | `assetModelId`, `assetModelCompositeModelName`, `assetModelCompositeModelType` | `clientToken` | `CreateAssetModelCompositeModelResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `PreconditionFailedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a custom composite model from specified property and hierarchy definitions. There are two types of custom composite models, inline and component-model-based . Use component-model-based custom composite models ... |
| `CreateBulkImportJob` | `POST /jobs` | - | `jobName`, `jobRoleArn`, `files`, `errorReportLocation`, `jobConfiguration` | - | `CreateBulkImportJobResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Defines a job to ingest data to IoT SiteWise from Amazon S3. For more information, see Create a bulk import job (CLI) in the Amazon Simple Storage Service User Guide . Before you create a bulk import job, you must en ... |
| `CreateComputationModel` | `POST /computation-models` | `idempotency-token` | `computationModelName`, `computationModelConfiguration`, `computationModelDataBinding` | `clientToken` | `CreateComputationModelResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Create a computation model with a configuration and data binding. |
| `CreateDashboard` | `POST /dashboards` | `idempotency-token` | `projectId`, `dashboardName`, `dashboardDefinition` | `clientToken` | `CreateDashboardResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a dashboard in an IoT SiteWise Monitor project. |
| `CreateDataset` | `POST /datasets` | `idempotency-token` | `datasetName`, `datasetSource` | `clientToken` | `CreateDatasetResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a dataset to connect an external datasource. |
| `CreateGateway` | `POST /20200301/gateways` | - | `gatewayName`, `gatewayPlatform` | - | `CreateGatewayResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ThrottlingException` | Creates a gateway, which is a virtual or edge device that delivers industrial data streams from local servers to IoT SiteWise. For more information, see Ingesting data using a gateway in the IoT SiteWise User Guide . |
| `CreatePortal` | `POST /portals` | `idempotency-token` | `portalName`, `portalContactEmail`, `roleArn` | `clientToken` | `CreatePortalResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a portal, which can contain projects and dashboards. IoT SiteWise Monitor uses IAM Identity Center or IAM to authenticate portal users and manage user permissions. Before you can sign in to a new portal, you ... |
| `CreateProject` | `POST /projects` | `idempotency-token` | `portalId`, `projectName` | `clientToken` | `CreateProjectResponse` | `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a project in the specified portal. Make sure that the project name and description don't contain confidential information. |
| `DeleteAccessPolicy` | `DELETE /access-policies/{accessPolicyId}` | `idempotency-token` | `accessPolicyId` | `clientToken` | `DeleteAccessPolicyResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an access policy that grants the specified identity access to the specified IoT SiteWise Monitor resource. You can use this operation to revoke access to an IoT SiteWise Monitor resource. |
| `DeleteAsset` | `DELETE /assets/{assetId}` | `idempotency-token` | `assetId` | `clientToken` | `DeleteAssetResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an asset. This action can't be undone. For more information, see Deleting assets and models in the IoT SiteWise User Guide . You can't delete an asset that's associated to another asset. For more information, ... |
| `DeleteAssetModel` | `DELETE /asset-models/{assetModelId}` | `idempotency-token` | `assetModelId` | `clientToken` | `DeleteAssetModelResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an asset model. This action can't be undone. You must delete all assets created from an asset model before you can delete the model. Also, you can't delete an asset model if a parent asset model exists that c ... |
| `DeleteAssetModelCompositeModel` | `DELETE /asset-models/{assetModelId}/composite-models/{assetModelCompositeModelId}` | `idempotency-token` | `assetModelId`, `assetModelCompositeModelId` | `clientToken` | `DeleteAssetModelCompositeModelResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a composite model. This action can't be undone. You must delete all assets created from a composite model before you can delete the model. Also, you can't delete a composite model if a parent asset model exis ... |
| `DeleteAssetModelInterfaceRelationship` | `DELETE /asset-models/{assetModelId}/interface/{interfaceAssetModelId}/asset-model-interface-relationship` | `idempotency-token` | `assetModelId`, `interfaceAssetModelId` | `clientToken` | `DeleteAssetModelInterfaceRelationshipResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an interface relationship between an asset model and an interface asset model. |
| `DeleteComputationModel` | `DELETE /computation-models/{computationModelId}` | `idempotency-token` | `computationModelId` | `clientToken` | `DeleteComputationModelResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a computation model. This action can't be undone. |
| `DeleteDashboard` | `DELETE /dashboards/{dashboardId}` | `idempotency-token` | `dashboardId` | `clientToken` | `DeleteDashboardResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a dashboard from IoT SiteWise Monitor. |
| `DeleteDataset` | `DELETE /datasets/{datasetId}` | `idempotency-token` | `datasetId` | `clientToken` | `DeleteDatasetResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a dataset. This cannot be undone. |
| `DeleteGateway` | `DELETE /20200301/gateways/{gatewayId}` | - | `gatewayId` | - | `Unit` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a gateway from IoT SiteWise. When you delete a gateway, some of the gateway's files remain in your gateway's file system. |
| `DeletePortal` | `DELETE /portals/{portalId}` | `idempotency-token` | `portalId` | `clientToken` | `DeletePortalResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a portal from IoT SiteWise Monitor. |
| `DeleteProject` | `DELETE /projects/{projectId}` | `idempotency-token` | `projectId` | `clientToken` | `DeleteProjectResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a project from IoT SiteWise Monitor. |
| `DeleteTimeSeries` | `POST /timeseries/delete` | `idempotency-token` | - | `clientToken` | `Unit` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a time series (data stream). If you delete a time series that's associated with an asset property, the asset property still exists, but the time series will no longer be associated with this asset property. T ... |
| `DescribeAccessPolicy` | `GET /access-policies/{accessPolicyId}` | - | `accessPolicyId` | - | `DescribeAccessPolicyResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Describes an access policy, which specifies an identity's access to an IoT SiteWise Monitor portal or project. |
| `DescribeAction` | `GET /actions/{actionId}` | - | `actionId` | - | `DescribeActionResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about an action. |
| `DescribeAsset` | `GET /assets/{assetId}` | - | `assetId` | - | `DescribeAssetResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about an asset. |
| `DescribeAssetCompositeModel` | `GET /assets/{assetId}/composite-models/{assetCompositeModelId}` | - | `assetId`, `assetCompositeModelId` | - | `DescribeAssetCompositeModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about an asset composite model (also known as an asset component). An AssetCompositeModel is an instance of an AssetModelCompositeModel . If you want to see information about the model this is b ... |
| `DescribeAssetModel` | `GET /asset-models/{assetModelId}` | - | `assetModelId` | - | `DescribeAssetModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about an asset model. This includes details about the asset model's properties, hierarchies, composite models, and any interface relationships if the asset model implements interfaces. |
| `DescribeAssetModelCompositeModel` | `GET /asset-models/{assetModelId}/composite-models/{assetModelCompositeModelId}` | - | `assetModelId`, `assetModelCompositeModelId` | - | `DescribeAssetModelCompositeModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about an asset model composite model (also known as an asset model component). For more information, see Custom composite models (Components) in the IoT SiteWise User Guide . |
| `DescribeAssetModelInterfaceRelationship` | `GET /asset-models/{assetModelId}/interface/{interfaceAssetModelId}/asset-model-interface-relationship` | - | `assetModelId`, `interfaceAssetModelId` | - | `DescribeAssetModelInterfaceRelationshipResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about an interface relationship between an asset model and an interface asset model. |
| `DescribeAssetProperty` | `GET /assets/{assetId}/properties/{propertyId}` | - | `assetId`, `propertyId` | - | `DescribeAssetPropertyResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about an asset property. When you call this operation for an attribute property, this response includes the default attribute value that you define in the asset model. If you update the default ... |
| `DescribeBulkImportJob` | `GET /jobs/{jobId}` | - | `jobId` | - | `DescribeBulkImportJobResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a bulk import job request. For more information, see Describe a bulk import job (CLI) in the Amazon Simple Storage Service User Guide . |
| `DescribeComputationModel` | `GET /computation-models/{computationModelId}` | - | `computationModelId` | - | `DescribeComputationModelResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a computation model. |
| `DescribeComputationModelExecutionSummary` | `GET /computation-models/{computationModelId}/execution-summary` | - | `computationModelId` | - | `DescribeComputationModelExecutionSummaryResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about the execution summary of a computation model. |
| `DescribeDashboard` | `GET /dashboards/{dashboardId}` | - | `dashboardId` | - | `DescribeDashboardResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a dashboard. |
| `DescribeDataset` | `GET /datasets/{datasetId}` | - | `datasetId` | - | `DescribeDatasetResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a dataset. |
| `DescribeDefaultEncryptionConfiguration` | `GET /configuration/account/encryption` | - | - | - | `DescribeDefaultEncryptionConfigurationResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves information about the default encryption configuration for the Amazon Web Services account in the default or specified Region. For more information, see Key management in the IoT SiteWise User Guide . |
| `DescribeExecution` | `GET /executions/{executionId}` | - | `executionId` | - | `DescribeExecutionResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about the execution. |
| `DescribeGateway` | `GET /20200301/gateways/{gatewayId}` | - | `gatewayId` | - | `DescribeGatewayResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a gateway. |
| `DescribeGatewayCapabilityConfiguration` | `GET /20200301/gateways/{gatewayId}/capability/{capabilityNamespace}` | - | `gatewayId`, `capabilityNamespace` | - | `DescribeGatewayCapabilityConfigurationResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Each gateway capability defines data sources for a gateway. This is the namespace of the gateway capability. . The namespace follows the format service:capability:version , where: service - The service providing the ... |
| `DescribeLoggingOptions` | `GET /logging` | - | - | - | `DescribeLoggingOptionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the current IoT SiteWise logging options. |
| `DescribePortal` | `GET /portals/{portalId}` | - | `portalId` | - | `DescribePortalResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a portal. |
| `DescribeProject` | `GET /projects/{projectId}` | - | `projectId` | - | `DescribeProjectResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a project. |
| `DescribeStorageConfiguration` | `GET /configuration/account/storage` | - | - | - | `DescribeStorageConfigurationResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about the storage configuration for IoT SiteWise. |
| `DescribeTimeSeries` | `GET /timeseries/describe` | - | - | - | `DescribeTimeSeriesResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves information about a time series (data stream). To identify a time series, do one of the following: If the time series isn't associated with an asset property, specify the alias of the time series. If the ti ... |
| `DisassociateAssets` | `POST /assets/{assetId}/disassociate` | `idempotency-token` | `assetId`, `hierarchyId`, `childAssetId` | `clientToken` | `Unit` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Disassociates a child asset from the given parent asset through a hierarchy defined in the parent asset's model. |
| `DisassociateTimeSeriesFromAssetProperty` | `POST /timeseries/disassociate` | `idempotency-token` | `alias`, `assetId`, `propertyId` | `clientToken` | `Unit` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Disassociates a time series (data stream) from an asset property. |
| `ExecuteAction` | `POST /actions` | - | `targetResource`, `actionDefinitionId`, `actionPayload` | - | `ExecuteActionResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Executes an action on a target resource. |
| `ExecuteQuery` | `POST /queries/execution` | `paginated`, `idempotency-token` | `queryStatement` | `clientToken` | `ExecuteQueryResponse` | `AccessDeniedException`, `InternalFailureException`, `InvalidRequestException`, `QueryTimeoutException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Run SQL queries to retrieve metadata and time-series data from asset models, assets, measurements, metrics, transforms, and aggregates. |
| `GetAssetPropertyAggregates` | `GET /properties/aggregates` | `paginated` | `aggregateTypes`, `resolution`, `startDate`, `endDate` | - | `GetAssetPropertyAggregatesResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Gets aggregated values for an asset property. For more information, see Querying aggregates in the IoT SiteWise User Guide . To identify an asset property, you must specify one of the following: The assetId and prope ... |
| `GetAssetPropertyValue` | `GET /properties/latest` | - | - | - | `GetAssetPropertyValueResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Gets an asset property's current value. For more information, see Querying current values in the IoT SiteWise User Guide . To identify an asset property, you must specify one of the following: The assetId and propert ... |
| `GetAssetPropertyValueHistory` | `GET /properties/history` | `paginated` | - | - | `GetAssetPropertyValueHistoryResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Gets the history of an asset property's values. For more information, see Querying historical values in the IoT SiteWise User Guide . To identify an asset property, you must specify one of the following: The assetId ... |
| `GetInterpolatedAssetPropertyValues` | `GET /properties/interpolated` | `paginated` | `startTimeInSeconds`, `endTimeInSeconds`, `quality`, `intervalInSeconds`, `type` | - | `GetInterpolatedAssetPropertyValuesResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Get interpolated values for an asset property for a specified time interval, during a period of time. If your time series is missing data points during the specified time interval, you can use interpolation to estima ... |
| `InvokeAssistant` | `POST /assistant/invocation` | - | `message` | - | `InvokeAssistantResponse` | `AccessDeniedException`, `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Invokes SiteWise Assistant to start or continue a conversation. |
| `ListAccessPolicies` | `GET /access-policies` | `paginated` | - | - | `ListAccessPoliciesResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves a paginated list of access policies for an identity (an IAM Identity Center user, an IAM Identity Center group, or an IAM user) or an IoT SiteWise Monitor resource (a portal or project). |
| `ListActions` | `GET /actions` | - | `targetResourceType`, `targetResourceId` | - | `ListActionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of actions for a specific target resource. |
| `ListAssetModelCompositeModels` | `GET /asset-models/{assetModelId}/composite-models` | `paginated` | `assetModelId` | - | `ListAssetModelCompositeModelsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of composite models associated with the asset model |
| `ListAssetModelProperties` | `GET /asset-models/{assetModelId}/properties` | `paginated` | `assetModelId` | - | `ListAssetModelPropertiesResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of properties associated with an asset model. If you update properties associated with the model before you finish listing all the properties, you need to start all over again. |
| `ListAssetModels` | `GET /asset-models` | `paginated` | - | - | `ListAssetModelsResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves a paginated list of summaries of all asset models. |
| `ListAssetProperties` | `GET /assets/{assetId}/properties` | `paginated` | `assetId` | - | `ListAssetPropertiesResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of properties associated with an asset. If you update properties associated with the model before you finish listing all the properties, you need to start all over again. |
| `ListAssetRelationships` | `GET /assets/{assetId}/assetRelationships` | `paginated` | `assetId`, `traversalType` | - | `ListAssetRelationshipsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of asset relationships for an asset. You can use this operation to identify an asset's root asset and all associated assets between that asset and its root. |
| `ListAssets` | `GET /assets` | `paginated` | - | - | `ListAssetsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of asset summaries. You can use this operation to do the following: List assets based on a specific asset model. List top-level assets. You can't use this operation to list all assets. To r ... |
| `ListAssociatedAssets` | `GET /assets/{assetId}/hierarchies` | `paginated` | `assetId` | - | `ListAssociatedAssetsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of associated assets. You can use this operation to do the following: CHILD - List all child assets associated to the asset. PARENT - List the asset's parent asset. |
| `ListBulkImportJobs` | `GET /jobs` | `paginated` | - | - | `ListBulkImportJobsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of bulk import job requests. For more information, see List bulk import jobs (CLI) in the IoT SiteWise User Guide . |
| `ListCompositionRelationships` | `GET /asset-models/{assetModelId}/composition-relationships` | `paginated` | `assetModelId` | - | `ListCompositionRelationshipsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of composition relationships for an asset model of type COMPONENT_MODEL . |
| `ListComputationModelDataBindingUsages` | `POST /computation-models/data-binding-usages` | `paginated` | `dataBindingValueFilter` | - | `ListComputationModelDataBindingUsagesResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Lists all data binding usages for computation models. This allows to identify where specific data bindings are being utilized across the computation models. This track dependencies between data sources and computatio ... |
| `ListComputationModelResolveToResources` | `GET /computation-models/{computationModelId}/resolve-to-resources` | `paginated` | `computationModelId` | - | `ListComputationModelResolveToResourcesResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all distinct resources that are resolved from the executed actions of the computation model. |
| `ListComputationModels` | `GET /computation-models` | `paginated` | - | - | `ListComputationModelsResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves a paginated list of summaries of all computation models. |
| `ListDashboards` | `GET /dashboards` | `paginated` | `projectId` | - | `ListDashboardsResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves a paginated list of dashboards for an IoT SiteWise Monitor project. |
| `ListDatasets` | `GET /datasets` | `paginated` | `sourceType` | - | `ListDatasetsResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves a paginated list of datasets for a specific target resource. |
| `ListExecutions` | `GET /executions` | `paginated` | `targetResourceType`, `targetResourceId` | - | `ListExecutionsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of summaries of all executions. |
| `ListGateways` | `GET /20200301/gateways` | `paginated` | - | - | `ListGatewaysResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves a paginated list of gateways. |
| `ListInterfaceRelationships` | `GET /interface/{interfaceAssetModelId}/asset-models` | `paginated` | `interfaceAssetModelId` | - | `ListInterfaceRelationshipsResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of asset models that have a specific interface asset model applied to them. |
| `ListPortals` | `GET /portals` | `paginated` | - | - | `ListPortalsResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves a paginated list of IoT SiteWise Monitor portals. |
| `ListProjectAssets` | `GET /projects/{projectId}/assets` | `paginated` | `projectId` | - | `ListProjectAssetsResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves a paginated list of assets associated with an IoT SiteWise Monitor project. |
| `ListProjects` | `GET /projects` | `paginated` | `portalId` | - | `ListProjectsResponse` | `InternalFailureException`, `InvalidRequestException`, `ThrottlingException` | Retrieves a paginated list of projects for an IoT SiteWise Monitor portal. |
| `ListTagsForResource` | `GET /tags` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Retrieves the list of tags for an IoT SiteWise resource. |
| `ListTimeSeries` | `GET /timeseries` | `paginated` | - | - | `ListTimeSeriesResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a paginated list of time series (data streams). |
| `PutAssetModelInterfaceRelationship` | `PUT /asset-models/{assetModelId}/interface/{interfaceAssetModelId}/asset-model-interface-relationship` | `idempotency-token` | `assetModelId`, `interfaceAssetModelId`, `propertyMappingConfiguration` | `clientToken` | `PutAssetModelInterfaceRelationshipResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Creates or updates an interface relationship between an asset model and an interface asset model. This operation applies an interface to an asset model. |
| `PutDefaultEncryptionConfiguration` | `POST /configuration/account/encryption` | - | `encryptionType` | - | `PutDefaultEncryptionConfigurationResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ThrottlingException` | Sets the default encryption configuration for the Amazon Web Services account. For more information, see Key management in the IoT SiteWise User Guide . |
| `PutLoggingOptions` | `PUT /logging` | - | `loggingOptions` | - | `PutLoggingOptionsResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Sets logging options for IoT SiteWise. |
| `PutStorageConfiguration` | `POST /configuration/account/storage` | - | `storageType` | - | `PutStorageConfigurationResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Configures storage settings for IoT SiteWise. |
| `TagResource` | `POST /tags` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException`, `UnauthorizedException` | Adds tags to an IoT SiteWise resource. If a tag already exists for the resource, this operation updates the tag's value. |
| `UntagResource` | `DELETE /tags` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Removes a tag from an IoT SiteWise resource. |
| `UpdateAccessPolicy` | `PUT /access-policies/{accessPolicyId}` | `idempotency-token` | `accessPolicyId`, `accessPolicyIdentity`, `accessPolicyResource`, `accessPolicyPermission` | `clientToken` | `UpdateAccessPolicyResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an existing access policy that specifies an identity's access to an IoT SiteWise Monitor portal or project resource. |
| `UpdateAsset` | `PUT /assets/{assetId}` | `idempotency-token` | `assetId`, `assetName` | `clientToken` | `UpdateAssetResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an asset's name. For more information, see Updating assets and models in the IoT SiteWise User Guide . |
| `UpdateAssetModel` | `PUT /asset-models/{assetModelId}` | `idempotency-token` | `assetModelId`, `assetModelName` | `clientToken` | `UpdateAssetModelResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `PreconditionFailedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an asset model and all of the assets that were created from the model. Each asset created from the model inherits the updated asset model's property and hierarchy definitions. For more information, see Updati ... |
| `UpdateAssetModelCompositeModel` | `PUT /asset-models/{assetModelId}/composite-models/{assetModelCompositeModelId}` | `idempotency-token` | `assetModelId`, `assetModelCompositeModelId`, `assetModelCompositeModelName` | `clientToken` | `UpdateAssetModelCompositeModelResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `PreconditionFailedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Updates a composite model and all of the assets that were created from the model. Each asset created from the model inherits the updated asset model's property and hierarchy definitions. For more information, see Upd ... |
| `UpdateAssetProperty` | `PUT /assets/{assetId}/properties/{propertyId}` | `idempotency-token` | `assetId`, `propertyId` | `clientToken` | `Unit` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an asset property's alias and notification state. This operation overwrites the property's existing alias and notification state. To keep your existing property's alias or notification state, you must include ... |
| `UpdateComputationModel` | `POST /computation-models/{computationModelId}` | `idempotency-token` | `computationModelId`, `computationModelName`, `computationModelConfiguration`, `computationModelDataBinding` | `clientToken` | `UpdateComputationModelResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the computation model. |
| `UpdateDashboard` | `PUT /dashboards/{dashboardId}` | `idempotency-token` | `dashboardId`, `dashboardName`, `dashboardDefinition` | `clientToken` | `UpdateDashboardResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an IoT SiteWise Monitor dashboard. |
| `UpdateDataset` | `PUT /datasets/{datasetId}` | `idempotency-token` | `datasetId`, `datasetName`, `datasetSource` | `clientToken` | `UpdateDatasetResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Updates a dataset. |
| `UpdateGateway` | `PUT /20200301/gateways/{gatewayId}` | - | `gatewayId`, `gatewayName` | - | `Unit` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates a gateway's name. |
| `UpdateGatewayCapabilityConfiguration` | `POST /20200301/gateways/{gatewayId}/capability` | - | `gatewayId`, `capabilityNamespace`, `capabilityConfiguration` | - | `UpdateGatewayCapabilityConfigurationResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Updates a gateway capability configuration or defines a new capability configuration. Each gateway capability defines data sources for a gateway. Important workflow notes: Each gateway capability defines data sources ... |
| `UpdatePortal` | `PUT /portals/{portalId}` | `idempotency-token` | `portalId`, `portalName`, `portalContactEmail`, `roleArn` | `clientToken` | `UpdatePortalResponse` | `ConflictingOperationException`, `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an IoT SiteWise Monitor portal. |
| `UpdateProject` | `PUT /projects/{projectId}` | `idempotency-token` | `projectId`, `projectName` | `clientToken` | `UpdateProjectResponse` | `InternalFailureException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an IoT SiteWise Monitor project. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `AssociateTimeSeriesToAssetProperty` | - | `alias -> alias`, `assetId -> assetId`, `propertyId -> propertyId` | - | - |
| `CreateAssetModelCompositeModel` | `ifMatch -> If-Match`, `ifNoneMatch -> If-None-Match`, `matchForVersionType -> Match-For-Version-Type` | - | - | - |
| `DeleteAccessPolicy` | - | `clientToken -> clientToken` | - | - |
| `DeleteAsset` | - | `clientToken -> clientToken` | - | - |
| `DeleteAssetModel` | `ifMatch -> If-Match`, `ifNoneMatch -> If-None-Match`, `matchForVersionType -> Match-For-Version-Type` | `clientToken -> clientToken` | - | - |
| `DeleteAssetModelCompositeModel` | `ifMatch -> If-Match`, `ifNoneMatch -> If-None-Match`, `matchForVersionType -> Match-For-Version-Type` | `clientToken -> clientToken` | - | - |
| `DeleteAssetModelInterfaceRelationship` | - | `clientToken -> clientToken` | - | - |
| `DeleteComputationModel` | - | `clientToken -> clientToken` | - | - |
| `DeleteDashboard` | - | `clientToken -> clientToken` | - | - |
| `DeleteDataset` | - | `clientToken -> clientToken` | - | - |
| `DeletePortal` | - | `clientToken -> clientToken` | - | - |
| `DeleteProject` | - | `clientToken -> clientToken` | - | - |
| `DeleteTimeSeries` | - | `alias -> alias`, `assetId -> assetId`, `propertyId -> propertyId` | - | - |
| `DescribeAsset` | - | `excludeProperties -> excludeProperties` | - | - |
| `DescribeAssetModel` | - | `excludeProperties -> excludeProperties`, `assetModelVersion -> assetModelVersion` | - | - |
| `DescribeAssetModelCompositeModel` | - | `assetModelVersion -> assetModelVersion` | - | - |
| `DescribeComputationModel` | - | `computationModelVersion -> computationModelVersion` | - | - |
| `DescribeComputationModelExecutionSummary` | - | `resolveToResourceType -> resolveToResourceType`, `resolveToResourceId -> resolveToResourceId` | - | - |
| `DescribeTimeSeries` | - | `alias -> alias`, `assetId -> assetId`, `propertyId -> propertyId` | - | - |
| `DisassociateTimeSeriesFromAssetProperty` | - | `alias -> alias`, `assetId -> assetId`, `propertyId -> propertyId` | - | - |
| `GetAssetPropertyAggregates` | - | `assetId -> assetId`, `propertyId -> propertyId`, `propertyAlias -> propertyAlias`, `aggregateTypes -> aggregateTypes`, `resolution -> resolution`, `qualities -> qualities`, `startDate -> startDate`, `endDate -> endDate`, `timeOrdering -> timeOrdering`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `GetAssetPropertyValue` | - | `assetId -> assetId`, `propertyId -> propertyId`, `propertyAlias -> propertyAlias` | - | - |
| `GetAssetPropertyValueHistory` | - | `assetId -> assetId`, `propertyId -> propertyId`, `propertyAlias -> propertyAlias`, `startDate -> startDate`, `endDate -> endDate`, `qualities -> qualities`, `timeOrdering -> timeOrdering`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `GetInterpolatedAssetPropertyValues` | - | `assetId -> assetId`, `propertyId -> propertyId`, `propertyAlias -> propertyAlias`, `startTimeInSeconds -> startTimeInSeconds`, `startTimeOffsetInNanos -> startTimeOffsetInNanos`, `endTimeInSeconds -> endTimeInSeconds`, `endTimeOffsetInNanos -> endTimeOffsetInNanos`, `quality -> quality`, `intervalInSeconds -> intervalInSeconds`, `nextToken -> nextToken`, `maxResults -> maxResults`, `type -> type`, `intervalWindowInSeconds -> intervalWindowInSeconds` | - | - |
| `ListAccessPolicies` | - | `identityType -> identityType`, `identityId -> identityId`, `resourceType -> resourceType`, `resourceId -> resourceId`, `iamArn -> iamArn`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListActions` | - | `targetResourceType -> targetResourceType`, `targetResourceId -> targetResourceId`, `nextToken -> nextToken`, `maxResults -> maxResults`, `resolveToResourceType -> resolveToResourceType`, `resolveToResourceId -> resolveToResourceId` | - | - |
| `ListAssetModelCompositeModels` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `assetModelVersion -> assetModelVersion` | - | - |
| `ListAssetModelProperties` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `filter -> filter`, `assetModelVersion -> assetModelVersion` | - | - |
| `ListAssetModels` | - | `assetModelTypes -> assetModelTypes`, `nextToken -> nextToken`, `maxResults -> maxResults`, `assetModelVersion -> assetModelVersion` | - | - |
| `ListAssetProperties` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `filter -> filter` | - | - |
| `ListAssetRelationships` | - | `traversalType -> traversalType`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAssets` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `assetModelId -> assetModelId`, `filter -> filter` | - | - |
| `ListAssociatedAssets` | - | `hierarchyId -> hierarchyId`, `traversalDirection -> traversalDirection`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListBulkImportJobs` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `filter -> filter` | - | - |
| `ListCompositionRelationships` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListComputationModelResolveToResources` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListComputationModels` | - | `computationModelType -> computationModelType`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDashboards` | - | `projectId -> projectId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDatasets` | - | `sourceType -> sourceType`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListExecutions` | - | `targetResourceType -> targetResourceType`, `targetResourceId -> targetResourceId`, `resolveToResourceType -> resolveToResourceType`, `resolveToResourceId -> resolveToResourceId`, `nextToken -> nextToken`, `maxResults -> maxResults`, `actionType -> actionType` | - | - |
| `ListGateways` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListInterfaceRelationships` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListPortals` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListProjectAssets` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListProjects` | - | `portalId -> portalId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListTagsForResource` | - | `resourceArn -> resourceArn` | - | - |
| `ListTimeSeries` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `assetId -> assetId`, `aliasPrefix -> aliasPrefix`, `timeSeriesType -> timeSeriesType` | - | - |
| `TagResource` | - | `resourceArn -> resourceArn` | - | - |
| `UntagResource` | - | `resourceArn -> resourceArn`, `tagKeys -> tagKeys` | - | - |
| `UpdateAssetModel` | `ifMatch -> If-Match`, `ifNoneMatch -> If-None-Match`, `matchForVersionType -> Match-For-Version-Type` | - | - | - |
| `UpdateAssetModelCompositeModel` | `ifMatch -> If-Match`, `ifNoneMatch -> If-None-Match`, `matchForVersionType -> Match-For-Version-Type` | - | - | - |

**Conditional-write/read coverage:** the following operations model RFC 7232 conditional headers and therefore must enforce 412 PreconditionFailed (and may emit 409 ConditionalRequestConflict on races) even though those error codes are typically not in the modelled `errors:` list: `CreateAssetModelCompositeModel`, `DeleteAssetModel`, `DeleteAssetModelCompositeModel`, `UpdateAssetModel`, `UpdateAssetModelCompositeModel`.

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | Access is denied. |
| `ConflictingOperationException` | `structure` | message, resourceId, resourceArn | Your request has conflicting operations. This can occur if you're trying to perform more than one operation on the same resource at the same time. |
| `InternalFailureException` | `structure` | message | IoT SiteWise can't process your request right now. Try again later. |
| `InvalidRequestException` | `structure` | message | The request isn't valid. This can occur if your request contains malformed JSON or unsupported characters. Check your request and try again. |
| `LimitExceededException` | `structure` | message | You've reached the quota for a resource. For example, this can occur if you're trying to associate more than the allowed number of child assets or attemptin ... |
| `PreconditionFailedException` | `structure` | message, resourceId, resourceArn | The precondition in one or more of the request-header fields evaluated to FALSE . |
| `QueryTimeoutException` | `structure` | message | The query timed out. |
| `ResourceAlreadyExistsException` | `structure` | message, resourceId, resourceArn | The resource already exists. |
| `ResourceNotFoundException` | `structure` | message | The requested resource can't be found. |
| `ServiceUnavailableException` | `structure` | message | The requested service is unavailable. |
| `ThrottlingException` | `structure` | message | Your request exceeded a rate limit. For example, you might have exceeded the number of IoT SiteWise assets that can be created per second, the allowed numbe ... |
| `TooManyTagsException` | `structure` | message, resourceName | You've reached the quota for the number of tags allowed for a resource. For more information, see Tag naming limits and requirements in the Amazon Web Servi ... |
| `UnauthorizedException` | `structure` | message | You are not authorized. |
| `ValidationException` | `structure` | message | The validation failed for this query. |
| `AssociateAssetsRequest` | `structure` | assetId, hierarchyId, childAssetId, clientToken | - |
| `AssociateTimeSeriesToAssetPropertyRequest` | `structure` | alias, assetId, propertyId, clientToken | - |
| `BatchAssociateProjectAssetsRequest` | `structure` | projectId, assetIds, clientToken | - |
| `BatchAssociateProjectAssetsResponse` | `structure` | errors | - |
| `BatchDisassociateProjectAssetsRequest` | `structure` | projectId, assetIds, clientToken | - |
| `BatchDisassociateProjectAssetsResponse` | `structure` | errors | - |
| `BatchGetAssetPropertyAggregatesRequest` | `structure` | entries, nextToken, maxResults | - |
| `BatchGetAssetPropertyAggregatesResponse` | `structure` | errorEntries, successEntries, skippedEntries, nextToken | - |
| `BatchGetAssetPropertyValueRequest` | `structure` | entries, nextToken | - |
| `BatchGetAssetPropertyValueResponse` | `structure` | errorEntries, successEntries, skippedEntries, nextToken | - |
| `BatchGetAssetPropertyValueHistoryRequest` | `structure` | entries, nextToken, maxResults | - |
| `BatchGetAssetPropertyValueHistoryResponse` | `structure` | errorEntries, successEntries, skippedEntries, nextToken | - |
| `BatchPutAssetPropertyValueRequest` | `structure` | enablePartialEntryProcessing, entries | - |
| `BatchPutAssetPropertyValueResponse` | `structure` | errorEntries | - |
| `CreateAccessPolicyRequest` | `structure` | accessPolicyIdentity, accessPolicyResource, accessPolicyPermission, clientToken, tags | - |
| `CreateAccessPolicyResponse` | `structure` | accessPolicyId, accessPolicyArn | - |
| `CreateAssetRequest` | `structure` | assetName, assetModelId, assetId, assetExternalId, clientToken, tags, assetDescription | - |
| `CreateAssetResponse` | `structure` | assetId, assetArn, assetStatus | - |
| `CreateAssetModelRequest` | `structure` | assetModelName, assetModelType, assetModelId, assetModelExternalId, assetModelDescription, assetModelProperties, assetModelHierarchies, assetModelCompositeModels, clientToken, tags | - |
| `CreateAssetModelResponse` | `structure` | assetModelId, assetModelArn, assetModelStatus | - |
| `CreateAssetModelCompositeModelRequest` | `structure` | assetModelId, assetModelCompositeModelExternalId, parentAssetModelCompositeModelId, assetModelCompositeModelId, assetModelCompositeModelDescription, assetModelCompositeModelName, assetModelCompositeModelType, clientToken, composedAssetModelId, assetModelCompositeModelProperties, ifMatch, ifNoneMatch, ... (+1) | - |
| `CreateAssetModelCompositeModelResponse` | `structure` | assetModelCompositeModelId, assetModelCompositeModelPath, assetModelStatus | - |
| `CreateBulkImportJobRequest` | `structure` | jobName, jobRoleArn, files, errorReportLocation, jobConfiguration, adaptiveIngestion, deleteFilesAfterImport | - |
| `CreateBulkImportJobResponse` | `structure` | jobId, jobName, jobStatus | - |
| `CreateComputationModelRequest` | `structure` | computationModelName, computationModelDescription, computationModelConfiguration, computationModelDataBinding, clientToken, tags | - |
| `CreateComputationModelResponse` | `structure` | computationModelId, computationModelArn, computationModelStatus | - |
| `AggregateType` | `enum` | AVERAGE, COUNT, MAXIMUM, MINIMUM, SUM, STANDARD_DEVIATION | - |
| `AssetErrorCode` | `enum` | INTERNAL_FAILURE | - |
| `AssetModelState` | `enum` | CREATING, ACTIVE, UPDATING, PROPAGATING, DELETING, FAILED | - |
| `AssetModelType` | `enum` | ASSET_MODEL, COMPONENT_MODEL, INTERFACE | - |
| `AssetModelVersionType` | `enum` | LATEST, ACTIVE | - |
| `AssetRelationshipType` | `enum` | HIERARCHY | - |
| `AssetState` | `enum` | CREATING, ACTIVE, UPDATING, DELETING, FAILED | - |
| `AuthMode` | `enum` | IAM, SSO | - |
| `BatchEntryCompletionStatus` | `enum` | SUCCESS, ERROR | - |
| `BatchGetAssetPropertyAggregatesErrorCode` | `enum` | ResourceNotFoundException, InvalidRequestException, AccessDeniedException | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
