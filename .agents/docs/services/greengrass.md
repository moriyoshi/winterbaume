# AWS Greengrass

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS IoT Greengrass seamlessly extends AWS onto physical devices so they can act locally on the data they generate, while still using the cloud for management, analytics, and durable storage. AWS IoT Greengrass ensures your devices can respond quickly to local events and operate with intermittent connectivity. AWS IoT Greengrass minimizes the cost of transmitting data to the cloud by allowing you to author AWS Lambda functions that execute locally.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Greengrass where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Greengrass by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Greengrass resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Greengrass workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Update`, `Delete` operation families, including `GetAssociatedRole`, `GetBulkDeploymentStatus`, `GetConnectivityInfo`, `GetConnectorDefinition`, `ListBulkDeploymentDetailedReports`, `ListBulkDeployments`.

## Service Identity and Protocol

- AWS model slug: `greengrass`
- AWS SDK for Rust slug: `greengrass`
- Model version: `2017-06-07`
- Model file: `vendor/api-models-aws/models/greengrass/service/2017-06-07/greengrass-2017-06-07.json`
- SDK ID: `Greengrass`
- Endpoint prefix: `greengrass`
- ARN namespace: `greengrass`
- CloudFormation name: `Greengrass`
- CloudTrail event source: `greengrass.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (24), `List` (21), `Create` (19), `Update` (11), `Delete` (8), `Associate` (2), `Disassociate` (2), `Reset` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateRoleToGroup`, `AssociateServiceRoleToAccount`, `CreateConnectorDefinition`, `CreateConnectorDefinitionVersion`, `CreateCoreDefinition`, `CreateCoreDefinitionVersion`, `CreateDeployment`, `CreateDeviceDefinition`, `CreateDeviceDefinitionVersion`, `CreateFunctionDefinition`, `CreateFunctionDefinitionVersion`, `CreateGroup`, `CreateGroupCertificateAuthority`, `CreateGroupVersion`, `CreateLoggerDefinition`, `CreateLoggerDefinitionVersion`, `CreateResourceDefinition`, `CreateResourceDefinitionVersion`, `CreateSoftwareUpdateJob`, `CreateSubscriptionDefinition`, ... (+26).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAssociatedRole`, `GetBulkDeploymentStatus`, `GetConnectivityInfo`, `GetConnectorDefinition`, `GetConnectorDefinitionVersion`, `GetCoreDefinition`, `GetCoreDefinitionVersion`, `GetDeploymentStatus`, `GetDeviceDefinition`, `GetDeviceDefinitionVersion`, `GetFunctionDefinition`, `GetFunctionDefinitionVersion`, `GetGroup`, `GetGroupCertificateAuthority`, `GetGroupCertificateConfiguration`, `GetGroupVersion`, `GetLoggerDefinition`, `GetLoggerDefinitionVersion`, `GetResourceDefinition`, `GetResourceDefinitionVersion`, ... (+25).
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateSoftwareUpdateJob`, `ListBulkDeploymentDetailedReports`, `StartBulkDeployment`, `StopBulkDeployment`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 84 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Lambda`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetAssociatedRole`, `GetBulkDeploymentStatus`, `GetConnectivityInfo`, `GetConnectorDefinition`, `GetConnectorDefinitionVersion`, `GetCoreDefinition`, `GetCoreDefinitionVersion`, `GetDeploymentStatus`, `GetDeviceDefinition`, `GetDeviceDefinitionVersion`, `GetFunctionDefinition`, `GetFunctionDefinitionVersion`, `GetGroup`, `GetGroupCertificateAuthority`, `GetGroupCertificateConfiguration`, `GetGroupVersion`, `GetLoggerDefinition`, `GetLoggerDefinitionVersion`, `GetResourceDefinition`, `GetResourceDefinitionVersion`, `GetServiceRoleForAccount`, `GetSubscriptionDefinition`, `GetSubscriptionDefinitionVersion`, `GetThingRuntimeConfiguration`
- Common required input members in this group: `BulkDeploymentId`, `CertificateAuthorityId`, `ConnectorDefinitionId`, `ConnectorDefinitionVersionId`, `CoreDefinitionId`, `CoreDefinitionVersionId`, `DeploymentId`, `DeviceDefinitionId`, `DeviceDefinitionVersionId`, `FunctionDefinitionId`, `FunctionDefinitionVersionId`, `GroupId`, `GroupVersionId`, `LoggerDefinitionId`, `LoggerDefinitionVersionId`, `ResourceDefinitionId`, `ResourceDefinitionVersionId`, `SubscriptionDefinitionId`, `SubscriptionDefinitionVersionId`, `ThingName`

### List

- Operations: `ListBulkDeploymentDetailedReports`, `ListBulkDeployments`, `ListConnectorDefinitionVersions`, `ListConnectorDefinitions`, `ListCoreDefinitionVersions`, `ListCoreDefinitions`, `ListDeployments`, `ListDeviceDefinitionVersions`, `ListDeviceDefinitions`, `ListFunctionDefinitionVersions`, `ListFunctionDefinitions`, `ListGroupCertificateAuthorities`, `ListGroupVersions`, `ListGroups`, `ListLoggerDefinitionVersions`, `ListLoggerDefinitions`, `ListResourceDefinitionVersions`, `ListResourceDefinitions`, `ListSubscriptionDefinitionVersions`, `ListSubscriptionDefinitions`, `ListTagsForResource`
- Common required input members in this group: `BulkDeploymentId`, `ConnectorDefinitionId`, `CoreDefinitionId`, `DeviceDefinitionId`, `FunctionDefinitionId`, `GroupId`, `LoggerDefinitionId`, `ResourceArn`, `ResourceDefinitionId`, `SubscriptionDefinitionId`

### Create

- Operations: `CreateConnectorDefinition`, `CreateConnectorDefinitionVersion`, `CreateCoreDefinition`, `CreateCoreDefinitionVersion`, `CreateDeployment`, `CreateDeviceDefinition`, `CreateDeviceDefinitionVersion`, `CreateFunctionDefinition`, `CreateFunctionDefinitionVersion`, `CreateGroup`, `CreateGroupCertificateAuthority`, `CreateGroupVersion`, `CreateLoggerDefinition`, `CreateLoggerDefinitionVersion`, `CreateResourceDefinition`, `CreateResourceDefinitionVersion`, `CreateSoftwareUpdateJob`, `CreateSubscriptionDefinition`, `CreateSubscriptionDefinitionVersion`
- Common required input members in this group: `ConnectorDefinitionId`, `CoreDefinitionId`, `DeploymentType`, `DeviceDefinitionId`, `FunctionDefinitionId`, `GroupId`, `LoggerDefinitionId`, `Name`, `ResourceDefinitionId`, `S3UrlSignerRole`, `SoftwareToUpdate`, `SubscriptionDefinitionId`, `UpdateTargets`, `UpdateTargetsArchitecture`, `UpdateTargetsOperatingSystem`

### Update

- Operations: `UpdateConnectivityInfo`, `UpdateConnectorDefinition`, `UpdateCoreDefinition`, `UpdateDeviceDefinition`, `UpdateFunctionDefinition`, `UpdateGroup`, `UpdateGroupCertificateConfiguration`, `UpdateLoggerDefinition`, `UpdateResourceDefinition`, `UpdateSubscriptionDefinition`, `UpdateThingRuntimeConfiguration`
- Common required input members in this group: `ConnectorDefinitionId`, `CoreDefinitionId`, `DeviceDefinitionId`, `FunctionDefinitionId`, `GroupId`, `LoggerDefinitionId`, `ResourceDefinitionId`, `SubscriptionDefinitionId`, `ThingName`

### Delete

- Operations: `DeleteConnectorDefinition`, `DeleteCoreDefinition`, `DeleteDeviceDefinition`, `DeleteFunctionDefinition`, `DeleteGroup`, `DeleteLoggerDefinition`, `DeleteResourceDefinition`, `DeleteSubscriptionDefinition`
- Common required input members in this group: `ConnectorDefinitionId`, `CoreDefinitionId`, `DeviceDefinitionId`, `FunctionDefinitionId`, `GroupId`, `LoggerDefinitionId`, `ResourceDefinitionId`, `SubscriptionDefinitionId`

### Associate

- Operations: `AssociateRoleToGroup`, `AssociateServiceRoleToAccount`
- Common required input members in this group: `GroupId`, `RoleArn`

### Disassociate

- Operations: `DisassociateRoleFromGroup`, `DisassociateServiceRoleFromAccount`
- Common required input members in this group: `GroupId`

### Reset

- Operations: `ResetDeployments`
- Common required input members in this group: `GroupId`

### Start

- Operations: `StartBulkDeployment`
- Common required input members in this group: `ExecutionRoleArn`, `InputFileUri`

### Stop

- Operations: `StopBulkDeployment`
- Common required input members in this group: `BulkDeploymentId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateRoleToGroup` | `PUT /greengrass/groups/{GroupId}/role` | - | `GroupId`, `RoleArn` | - | `AssociateRoleToGroupResponse` | `BadRequestException`, `InternalServerErrorException` | Associates a role with a group. Your Greengrass core will use the role to access AWS cloud services. |
| `AssociateServiceRoleToAccount` | `PUT /greengrass/servicerole` | - | `RoleArn` | - | `AssociateServiceRoleToAccountResponse` | `BadRequestException`, `InternalServerErrorException` | Associates a role with your account. AWS IoT Greengrass will use the role to access your Lambda functions and AWS IoT resources. |
| `CreateConnectorDefinition` | `POST /greengrass/definition/connectors` | - | - | - | `CreateConnectorDefinitionResponse` | `BadRequestException` | Creates a connector definition. You may provide the initial version of the connector definition now or use ''CreateConnectorDefinitionVersion'' at a later time. |
| `CreateConnectorDefinitionVersion` | `POST /greengrass/definition/connectors/{ConnectorDefinitionId}/versions` | - | `ConnectorDefinitionId` | - | `CreateConnectorDefinitionVersionResponse` | `BadRequestException` | Creates a version of a connector definition which has already been defined. |
| `CreateCoreDefinition` | `POST /greengrass/definition/cores` | - | - | - | `CreateCoreDefinitionResponse` | `BadRequestException` | Creates a core definition. You may provide the initial version of the core definition now or use ''CreateCoreDefinitionVersion'' at a later time. |
| `CreateCoreDefinitionVersion` | `POST /greengrass/definition/cores/{CoreDefinitionId}/versions` | - | `CoreDefinitionId` | - | `CreateCoreDefinitionVersionResponse` | `BadRequestException` | Creates a version of a core definition that has already been defined. Greengrass groups must each contain exactly one Greengrass core. |
| `CreateDeployment` | `POST /greengrass/groups/{GroupId}/deployments` | - | `DeploymentType`, `GroupId` | - | `CreateDeploymentResponse` | `BadRequestException` | Creates a deployment. ''CreateDeployment'' requests are idempotent with respect to the ''X-Amzn-Client-Token'' token and the request parameters. |
| `CreateDeviceDefinition` | `POST /greengrass/definition/devices` | - | - | - | `CreateDeviceDefinitionResponse` | `BadRequestException` | Creates a device definition. You may provide the initial version of the device definition now or use ''CreateDeviceDefinitionVersion'' at a later time. |
| `CreateDeviceDefinitionVersion` | `POST /greengrass/definition/devices/{DeviceDefinitionId}/versions` | - | `DeviceDefinitionId` | - | `CreateDeviceDefinitionVersionResponse` | `BadRequestException` | Creates a version of a device definition that has already been defined. |
| `CreateFunctionDefinition` | `POST /greengrass/definition/functions` | - | - | - | `CreateFunctionDefinitionResponse` | `BadRequestException` | Creates a Lambda function definition which contains a list of Lambda functions and their configurations to be used in a group. You can create an initial version of the definition by providing a list of Lambda functions and their configurations now, or use... |
| `CreateFunctionDefinitionVersion` | `POST /greengrass/definition/functions/{FunctionDefinitionId}/versions` | - | `FunctionDefinitionId` | - | `CreateFunctionDefinitionVersionResponse` | `BadRequestException` | Creates a version of a Lambda function definition that has already been defined. |
| `CreateGroup` | `POST /greengrass/groups` | - | `Name` | - | `CreateGroupResponse` | `BadRequestException` | Creates a group. You may provide the initial version of the group or use ''CreateGroupVersion'' at a later time. |
| `CreateGroupCertificateAuthority` | `POST /greengrass/groups/{GroupId}/certificateauthorities` | - | `GroupId` | - | `CreateGroupCertificateAuthorityResponse` | `BadRequestException`, `InternalServerErrorException` | Creates a CA for the group. If a CA already exists, it will rotate the existing CA. |
| `CreateGroupVersion` | `POST /greengrass/groups/{GroupId}/versions` | - | `GroupId` | - | `CreateGroupVersionResponse` | `BadRequestException` | Creates a version of a group which has already been defined. |
| `CreateLoggerDefinition` | `POST /greengrass/definition/loggers` | - | - | - | `CreateLoggerDefinitionResponse` | `BadRequestException` | Creates a logger definition. You may provide the initial version of the logger definition now or use ''CreateLoggerDefinitionVersion'' at a later time. |
| `CreateLoggerDefinitionVersion` | `POST /greengrass/definition/loggers/{LoggerDefinitionId}/versions` | - | `LoggerDefinitionId` | - | `CreateLoggerDefinitionVersionResponse` | `BadRequestException` | Creates a version of a logger definition that has already been defined. |
| `CreateResourceDefinition` | `POST /greengrass/definition/resources` | - | - | - | `CreateResourceDefinitionResponse` | `BadRequestException` | Creates a resource definition which contains a list of resources to be used in a group. You can create an initial version of the definition by providing a list of resources now, or use ''CreateResourceDefinitionVersion'' later. |
| `CreateResourceDefinitionVersion` | `POST /greengrass/definition/resources/{ResourceDefinitionId}/versions` | - | `ResourceDefinitionId` | - | `CreateResourceDefinitionVersionResponse` | `BadRequestException` | Creates a version of a resource definition that has already been defined. |
| `CreateSoftwareUpdateJob` | `POST /greengrass/updates` | - | `S3UrlSignerRole`, `SoftwareToUpdate`, `UpdateTargets`, `UpdateTargetsArchitecture`, `UpdateTargetsOperatingSystem` | - | `CreateSoftwareUpdateJobResponse` | `BadRequestException`, `InternalServerErrorException` | Creates a software update for a core or group of cores (specified as an IoT thing group.) Use this to update the OTA Agent as well as the Greengrass core software. It makes use of the IoT Jobs feature which provides additional commands to manage a Greengrass... |
| `CreateSubscriptionDefinition` | `POST /greengrass/definition/subscriptions` | - | - | - | `CreateSubscriptionDefinitionResponse` | `BadRequestException` | Creates a subscription definition. You may provide the initial version of the subscription definition now or use ''CreateSubscriptionDefinitionVersion'' at a later time. |
| `CreateSubscriptionDefinitionVersion` | `POST /greengrass/definition/subscriptions/{SubscriptionDefinitionId}/versions` | - | `SubscriptionDefinitionId` | - | `CreateSubscriptionDefinitionVersionResponse` | `BadRequestException` | Creates a version of a subscription definition which has already been defined. |
| `DeleteConnectorDefinition` | `DELETE /greengrass/definition/connectors/{ConnectorDefinitionId}` | - | `ConnectorDefinitionId` | - | `DeleteConnectorDefinitionResponse` | `BadRequestException` | Deletes a connector definition. |
| `DeleteCoreDefinition` | `DELETE /greengrass/definition/cores/{CoreDefinitionId}` | - | `CoreDefinitionId` | - | `DeleteCoreDefinitionResponse` | `BadRequestException` | Deletes a core definition. |
| `DeleteDeviceDefinition` | `DELETE /greengrass/definition/devices/{DeviceDefinitionId}` | - | `DeviceDefinitionId` | - | `DeleteDeviceDefinitionResponse` | `BadRequestException` | Deletes a device definition. |
| `DeleteFunctionDefinition` | `DELETE /greengrass/definition/functions/{FunctionDefinitionId}` | - | `FunctionDefinitionId` | - | `DeleteFunctionDefinitionResponse` | `BadRequestException` | Deletes a Lambda function definition. |
| `DeleteGroup` | `DELETE /greengrass/groups/{GroupId}` | - | `GroupId` | - | `DeleteGroupResponse` | `BadRequestException` | Deletes a group. |
| `DeleteLoggerDefinition` | `DELETE /greengrass/definition/loggers/{LoggerDefinitionId}` | - | `LoggerDefinitionId` | - | `DeleteLoggerDefinitionResponse` | `BadRequestException` | Deletes a logger definition. |
| `DeleteResourceDefinition` | `DELETE /greengrass/definition/resources/{ResourceDefinitionId}` | - | `ResourceDefinitionId` | - | `DeleteResourceDefinitionResponse` | `BadRequestException` | Deletes a resource definition. |
| `DeleteSubscriptionDefinition` | `DELETE /greengrass/definition/subscriptions/{SubscriptionDefinitionId}` | - | `SubscriptionDefinitionId` | - | `DeleteSubscriptionDefinitionResponse` | `BadRequestException` | Deletes a subscription definition. |
| `DisassociateRoleFromGroup` | `DELETE /greengrass/groups/{GroupId}/role` | - | `GroupId` | - | `DisassociateRoleFromGroupResponse` | `BadRequestException`, `InternalServerErrorException` | Disassociates the role from a group. |
| `DisassociateServiceRoleFromAccount` | `DELETE /greengrass/servicerole` | - | - | - | `DisassociateServiceRoleFromAccountResponse` | `InternalServerErrorException` | Disassociates the service role from your account. Without a service role, deployments will not work. |
| `GetAssociatedRole` | `GET /greengrass/groups/{GroupId}/role` | - | `GroupId` | - | `GetAssociatedRoleResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves the role associated with a particular group. |
| `GetBulkDeploymentStatus` | `GET /greengrass/bulk/deployments/{BulkDeploymentId}/status` | - | `BulkDeploymentId` | - | `GetBulkDeploymentStatusResponse` | `BadRequestException` | Returns the status of a bulk deployment. |
| `GetConnectivityInfo` | `GET /greengrass/things/{ThingName}/connectivityInfo` | - | `ThingName` | - | `GetConnectivityInfoResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves the connectivity information for a core. |
| `GetConnectorDefinition` | `GET /greengrass/definition/connectors/{ConnectorDefinitionId}` | - | `ConnectorDefinitionId` | - | `GetConnectorDefinitionResponse` | `BadRequestException` | Retrieves information about a connector definition. |
| `GetConnectorDefinitionVersion` | `GET /greengrass/definition/connectors/{ConnectorDefinitionId}/versions/{ConnectorDefinitionVersionId}` | - | `ConnectorDefinitionId`, `ConnectorDefinitionVersionId` | - | `GetConnectorDefinitionVersionResponse` | `BadRequestException` | Retrieves information about a connector definition version, including the connectors that the version contains. Connectors are prebuilt modules that interact with local infrastructure, device protocols, AWS, and other cloud services. |
| `GetCoreDefinition` | `GET /greengrass/definition/cores/{CoreDefinitionId}` | - | `CoreDefinitionId` | - | `GetCoreDefinitionResponse` | `BadRequestException` | Retrieves information about a core definition version. |
| `GetCoreDefinitionVersion` | `GET /greengrass/definition/cores/{CoreDefinitionId}/versions/{CoreDefinitionVersionId}` | - | `CoreDefinitionId`, `CoreDefinitionVersionId` | - | `GetCoreDefinitionVersionResponse` | `BadRequestException` | Retrieves information about a core definition version. |
| `GetDeploymentStatus` | `GET /greengrass/groups/{GroupId}/deployments/{DeploymentId}/status` | - | `DeploymentId`, `GroupId` | - | `GetDeploymentStatusResponse` | `BadRequestException` | Returns the status of a deployment. |
| `GetDeviceDefinition` | `GET /greengrass/definition/devices/{DeviceDefinitionId}` | - | `DeviceDefinitionId` | - | `GetDeviceDefinitionResponse` | `BadRequestException` | Retrieves information about a device definition. |
| `GetDeviceDefinitionVersion` | `GET /greengrass/definition/devices/{DeviceDefinitionId}/versions/{DeviceDefinitionVersionId}` | - | `DeviceDefinitionId`, `DeviceDefinitionVersionId` | - | `GetDeviceDefinitionVersionResponse` | `BadRequestException` | Retrieves information about a device definition version. |
| `GetFunctionDefinition` | `GET /greengrass/definition/functions/{FunctionDefinitionId}` | - | `FunctionDefinitionId` | - | `GetFunctionDefinitionResponse` | `BadRequestException` | Retrieves information about a Lambda function definition, including its creation time and latest version. |
| `GetFunctionDefinitionVersion` | `GET /greengrass/definition/functions/{FunctionDefinitionId}/versions/{FunctionDefinitionVersionId}` | - | `FunctionDefinitionId`, `FunctionDefinitionVersionId` | - | `GetFunctionDefinitionVersionResponse` | `BadRequestException` | Retrieves information about a Lambda function definition version, including which Lambda functions are included in the version and their configurations. |
| `GetGroup` | `GET /greengrass/groups/{GroupId}` | - | `GroupId` | - | `GetGroupResponse` | `BadRequestException` | Retrieves information about a group. |
| `GetGroupCertificateAuthority` | `GET /greengrass/groups/{GroupId}/certificateauthorities/{CertificateAuthorityId}` | - | `CertificateAuthorityId`, `GroupId` | - | `GetGroupCertificateAuthorityResponse` | `BadRequestException`, `InternalServerErrorException` | Retreives the CA associated with a group. Returns the public key of the CA. |
| `GetGroupCertificateConfiguration` | `GET /greengrass/groups/{GroupId}/certificateauthorities/configuration/expiry` | - | `GroupId` | - | `GetGroupCertificateConfigurationResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves the current configuration for the CA used by the group. |
| `GetGroupVersion` | `GET /greengrass/groups/{GroupId}/versions/{GroupVersionId}` | - | `GroupId`, `GroupVersionId` | - | `GetGroupVersionResponse` | `BadRequestException` | Retrieves information about a group version. |
| `GetLoggerDefinition` | `GET /greengrass/definition/loggers/{LoggerDefinitionId}` | - | `LoggerDefinitionId` | - | `GetLoggerDefinitionResponse` | `BadRequestException` | Retrieves information about a logger definition. |
| `GetLoggerDefinitionVersion` | `GET /greengrass/definition/loggers/{LoggerDefinitionId}/versions/{LoggerDefinitionVersionId}` | - | `LoggerDefinitionId`, `LoggerDefinitionVersionId` | - | `GetLoggerDefinitionVersionResponse` | `BadRequestException` | Retrieves information about a logger definition version. |
| `GetResourceDefinition` | `GET /greengrass/definition/resources/{ResourceDefinitionId}` | - | `ResourceDefinitionId` | - | `GetResourceDefinitionResponse` | `BadRequestException` | Retrieves information about a resource definition, including its creation time and latest version. |
| `GetResourceDefinitionVersion` | `GET /greengrass/definition/resources/{ResourceDefinitionId}/versions/{ResourceDefinitionVersionId}` | - | `ResourceDefinitionId`, `ResourceDefinitionVersionId` | - | `GetResourceDefinitionVersionResponse` | `BadRequestException` | Retrieves information about a resource definition version, including which resources are included in the version. |
| `GetServiceRoleForAccount` | `GET /greengrass/servicerole` | - | - | - | `GetServiceRoleForAccountResponse` | `InternalServerErrorException` | Retrieves the service role that is attached to your account. |
| `GetSubscriptionDefinition` | `GET /greengrass/definition/subscriptions/{SubscriptionDefinitionId}` | - | `SubscriptionDefinitionId` | - | `GetSubscriptionDefinitionResponse` | `BadRequestException` | Retrieves information about a subscription definition. |
| `GetSubscriptionDefinitionVersion` | `GET /greengrass/definition/subscriptions/{SubscriptionDefinitionId}/versions/{SubscriptionDefinitionVersionId}` | - | `SubscriptionDefinitionId`, `SubscriptionDefinitionVersionId` | - | `GetSubscriptionDefinitionVersionResponse` | `BadRequestException` | Retrieves information about a subscription definition version. |
| `GetThingRuntimeConfiguration` | `GET /greengrass/things/{ThingName}/runtimeconfig` | - | `ThingName` | - | `GetThingRuntimeConfigurationResponse` | `BadRequestException`, `InternalServerErrorException` | Get the runtime configuration of a thing. |
| `ListBulkDeploymentDetailedReports` | `GET /greengrass/bulk/deployments/{BulkDeploymentId}/detailed-reports` | - | `BulkDeploymentId` | - | `ListBulkDeploymentDetailedReportsResponse` | `BadRequestException` | Gets a paginated list of the deployments that have been started in a bulk deployment operation, and their current deployment status. |
| `ListBulkDeployments` | `GET /greengrass/bulk/deployments` | - | - | - | `ListBulkDeploymentsResponse` | `BadRequestException` | Returns a list of bulk deployments. |
| `ListConnectorDefinitionVersions` | `GET /greengrass/definition/connectors/{ConnectorDefinitionId}/versions` | - | `ConnectorDefinitionId` | - | `ListConnectorDefinitionVersionsResponse` | `BadRequestException` | Lists the versions of a connector definition, which are containers for connectors. Connectors run on the Greengrass core and contain built-in integration with local infrastructure, device protocols, AWS, and other cloud services. |
| `ListConnectorDefinitions` | `GET /greengrass/definition/connectors` | - | - | - | `ListConnectorDefinitionsResponse` | - | Retrieves a list of connector definitions. |
| `ListCoreDefinitionVersions` | `GET /greengrass/definition/cores/{CoreDefinitionId}/versions` | - | `CoreDefinitionId` | - | `ListCoreDefinitionVersionsResponse` | `BadRequestException` | Lists the versions of a core definition. |
| `ListCoreDefinitions` | `GET /greengrass/definition/cores` | - | - | - | `ListCoreDefinitionsResponse` | - | Retrieves a list of core definitions. |
| `ListDeployments` | `GET /greengrass/groups/{GroupId}/deployments` | - | `GroupId` | - | `ListDeploymentsResponse` | `BadRequestException` | Returns a history of deployments for the group. |
| `ListDeviceDefinitionVersions` | `GET /greengrass/definition/devices/{DeviceDefinitionId}/versions` | - | `DeviceDefinitionId` | - | `ListDeviceDefinitionVersionsResponse` | `BadRequestException` | Lists the versions of a device definition. |
| `ListDeviceDefinitions` | `GET /greengrass/definition/devices` | - | - | - | `ListDeviceDefinitionsResponse` | - | Retrieves a list of device definitions. |
| `ListFunctionDefinitionVersions` | `GET /greengrass/definition/functions/{FunctionDefinitionId}/versions` | - | `FunctionDefinitionId` | - | `ListFunctionDefinitionVersionsResponse` | `BadRequestException` | Lists the versions of a Lambda function definition. |
| `ListFunctionDefinitions` | `GET /greengrass/definition/functions` | - | - | - | `ListFunctionDefinitionsResponse` | - | Retrieves a list of Lambda function definitions. |
| `ListGroupCertificateAuthorities` | `GET /greengrass/groups/{GroupId}/certificateauthorities` | - | `GroupId` | - | `ListGroupCertificateAuthoritiesResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves the current CAs for a group. |
| `ListGroupVersions` | `GET /greengrass/groups/{GroupId}/versions` | - | `GroupId` | - | `ListGroupVersionsResponse` | `BadRequestException` | Lists the versions of a group. |
| `ListGroups` | `GET /greengrass/groups` | - | - | - | `ListGroupsResponse` | - | Retrieves a list of groups. |
| `ListLoggerDefinitionVersions` | `GET /greengrass/definition/loggers/{LoggerDefinitionId}/versions` | - | `LoggerDefinitionId` | - | `ListLoggerDefinitionVersionsResponse` | `BadRequestException` | Lists the versions of a logger definition. |
| `ListLoggerDefinitions` | `GET /greengrass/definition/loggers` | - | - | - | `ListLoggerDefinitionsResponse` | - | Retrieves a list of logger definitions. |
| `ListResourceDefinitionVersions` | `GET /greengrass/definition/resources/{ResourceDefinitionId}/versions` | - | `ResourceDefinitionId` | - | `ListResourceDefinitionVersionsResponse` | `BadRequestException` | Lists the versions of a resource definition. |
| `ListResourceDefinitions` | `GET /greengrass/definition/resources` | - | - | - | `ListResourceDefinitionsResponse` | - | Retrieves a list of resource definitions. |
| `ListSubscriptionDefinitionVersions` | `GET /greengrass/definition/subscriptions/{SubscriptionDefinitionId}/versions` | - | `SubscriptionDefinitionId` | - | `ListSubscriptionDefinitionVersionsResponse` | `BadRequestException` | Lists the versions of a subscription definition. |
| `ListSubscriptionDefinitions` | `GET /greengrass/definition/subscriptions` | - | - | - | `ListSubscriptionDefinitionsResponse` | - | Retrieves a list of subscription definitions. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException` | Retrieves a list of resource tags for a resource arn. |
| `ResetDeployments` | `POST /greengrass/groups/{GroupId}/deployments/$reset` | - | `GroupId` | - | `ResetDeploymentsResponse` | `BadRequestException` | Resets a group's deployments. |
| `StartBulkDeployment` | `POST /greengrass/bulk/deployments` | - | `ExecutionRoleArn`, `InputFileUri` | - | `StartBulkDeploymentResponse` | `BadRequestException` | Deploys multiple groups in one operation. This action starts the bulk deployment of a specified set of group versions. |
| `StopBulkDeployment` | `PUT /greengrass/bulk/deployments/{BulkDeploymentId}/$stop` | - | `BulkDeploymentId` | - | `StopBulkDeploymentResponse` | `BadRequestException` | Stops the execution of a bulk deployment. This action returns a status of ''Stopping'' until the deployment is stopped. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn` | - | `Unit` | `BadRequestException` | Adds tags to a Greengrass resource. Valid resources are 'Group', 'ConnectorDefinition', 'CoreDefinition', 'DeviceDefinition', 'FunctionDefinition', 'LoggerDefinition', 'SubscriptionDefinition', 'ResourceDefinition', and 'BulkDeployment'. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException` | Remove resource tags from a Greengrass Resource. |
| `UpdateConnectivityInfo` | `PUT /greengrass/things/{ThingName}/connectivityInfo` | - | `ThingName` | - | `UpdateConnectivityInfoResponse` | `BadRequestException`, `InternalServerErrorException` | Updates the connectivity information for the core. Any devices that belong to the group which has this core will receive this information in order to find the location of the core and connect to it. |
| `UpdateConnectorDefinition` | `PUT /greengrass/definition/connectors/{ConnectorDefinitionId}` | - | `ConnectorDefinitionId` | - | `UpdateConnectorDefinitionResponse` | `BadRequestException` | Updates a connector definition. |
| `UpdateCoreDefinition` | `PUT /greengrass/definition/cores/{CoreDefinitionId}` | - | `CoreDefinitionId` | - | `UpdateCoreDefinitionResponse` | `BadRequestException` | Updates a core definition. |
| `UpdateDeviceDefinition` | `PUT /greengrass/definition/devices/{DeviceDefinitionId}` | - | `DeviceDefinitionId` | - | `UpdateDeviceDefinitionResponse` | `BadRequestException` | Updates a device definition. |
| `UpdateFunctionDefinition` | `PUT /greengrass/definition/functions/{FunctionDefinitionId}` | - | `FunctionDefinitionId` | - | `UpdateFunctionDefinitionResponse` | `BadRequestException` | Updates a Lambda function definition. |
| `UpdateGroup` | `PUT /greengrass/groups/{GroupId}` | - | `GroupId` | - | `UpdateGroupResponse` | `BadRequestException` | Updates a group. |
| `UpdateGroupCertificateConfiguration` | `PUT /greengrass/groups/{GroupId}/certificateauthorities/configuration/expiry` | - | `GroupId` | - | `UpdateGroupCertificateConfigurationResponse` | `BadRequestException`, `InternalServerErrorException` | Updates the Certificate expiry time for a group. |
| `UpdateLoggerDefinition` | `PUT /greengrass/definition/loggers/{LoggerDefinitionId}` | - | `LoggerDefinitionId` | - | `UpdateLoggerDefinitionResponse` | `BadRequestException` | Updates a logger definition. |
| `UpdateResourceDefinition` | `PUT /greengrass/definition/resources/{ResourceDefinitionId}` | - | `ResourceDefinitionId` | - | `UpdateResourceDefinitionResponse` | `BadRequestException` | Updates a resource definition. |
| `UpdateSubscriptionDefinition` | `PUT /greengrass/definition/subscriptions/{SubscriptionDefinitionId}` | - | `SubscriptionDefinitionId` | - | `UpdateSubscriptionDefinitionResponse` | `BadRequestException` | Updates a subscription definition. |
| `UpdateThingRuntimeConfiguration` | `PUT /greengrass/things/{ThingName}/runtimeconfig` | - | `ThingName` | - | `UpdateThingRuntimeConfigurationResponse` | `BadRequestException`, `InternalServerErrorException` | Updates the runtime configuration of a thing. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CreateConnectorDefinition` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateConnectorDefinitionVersion` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateCoreDefinition` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateCoreDefinitionVersion` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateDeployment` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateDeviceDefinition` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateDeviceDefinitionVersion` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateFunctionDefinition` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateFunctionDefinitionVersion` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateGroup` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateGroupCertificateAuthority` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateGroupVersion` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateLoggerDefinition` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateLoggerDefinitionVersion` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateResourceDefinition` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateResourceDefinitionVersion` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateSoftwareUpdateJob` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateSubscriptionDefinition` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `CreateSubscriptionDefinitionVersion` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `GetConnectorDefinitionVersion` | - | `NextToken -> NextToken` | - | - |
| `GetDeviceDefinitionVersion` | - | `NextToken -> NextToken` | - | - |
| `GetFunctionDefinitionVersion` | - | `NextToken -> NextToken` | - | - |
| `GetLoggerDefinitionVersion` | - | `NextToken -> NextToken` | - | - |
| `GetSubscriptionDefinitionVersion` | - | `NextToken -> NextToken` | - | - |
| `ListBulkDeploymentDetailedReports` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListBulkDeployments` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListConnectorDefinitions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListConnectorDefinitionVersions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListCoreDefinitions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListCoreDefinitionVersions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListDeployments` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListDeviceDefinitions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListDeviceDefinitionVersions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListFunctionDefinitions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListFunctionDefinitionVersions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListGroups` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListGroupVersions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListLoggerDefinitions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListLoggerDefinitionVersions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListResourceDefinitions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListResourceDefinitionVersions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListSubscriptionDefinitions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListSubscriptionDefinitionVersions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ResetDeployments` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `StartBulkDeployment` | `AmznClientToken -> X-Amzn-Client-Token` | - | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `ErrorDetails`, `Message` | General error information. |
| `InternalServerErrorException` | `structure` | `ErrorDetails`, `Message` | General error information. |
| `AssociateRoleToGroupRequest` | `structure` | `GroupId`, `RoleArn` | - |
| `AssociateRoleToGroupResponse` | `structure` | `AssociatedAt` | - |
| `AssociateServiceRoleToAccountRequest` | `structure` | `RoleArn` | - |
| `AssociateServiceRoleToAccountResponse` | `structure` | `AssociatedAt` | - |
| `CreateConnectorDefinitionRequest` | `structure` | `AmznClientToken`, `InitialVersion`, `Name`, `tags` | - |
| `CreateConnectorDefinitionResponse` | `structure` | `Arn`, `CreationTimestamp`, `Id`, `LastUpdatedTimestamp`, `LatestVersion`, `LatestVersionArn`, `Name` | - |
| `CreateConnectorDefinitionVersionRequest` | `structure` | `AmznClientToken`, `ConnectorDefinitionId`, `Connectors` | - |
| `CreateConnectorDefinitionVersionResponse` | `structure` | `Arn`, `CreationTimestamp`, `Id`, `Version` | - |
| `CreateCoreDefinitionRequest` | `structure` | `AmznClientToken`, `InitialVersion`, `Name`, `tags` | Information needed to create a core definition. |
| `CreateCoreDefinitionResponse` | `structure` | `Arn`, `CreationTimestamp`, `Id`, `LastUpdatedTimestamp`, `LatestVersion`, `LatestVersionArn`, `Name` | - |
| `CreateCoreDefinitionVersionRequest` | `structure` | `AmznClientToken`, `CoreDefinitionId`, `Cores` | - |
| `CreateCoreDefinitionVersionResponse` | `structure` | `Arn`, `CreationTimestamp`, `Id`, `Version` | - |
| `CreateDeploymentRequest` | `structure` | `AmznClientToken`, `DeploymentId`, `DeploymentType`, `GroupId`, `GroupVersionId` | - |
| `CreateDeploymentResponse` | `structure` | `DeploymentArn`, `DeploymentId` | - |
| `CreateDeviceDefinitionRequest` | `structure` | `AmznClientToken`, `InitialVersion`, `Name`, `tags` | - |
| `CreateDeviceDefinitionResponse` | `structure` | `Arn`, `CreationTimestamp`, `Id`, `LastUpdatedTimestamp`, `LatestVersion`, `LatestVersionArn`, `Name` | - |
| `CreateDeviceDefinitionVersionRequest` | `structure` | `AmznClientToken`, `DeviceDefinitionId`, `Devices` | - |
| `CreateDeviceDefinitionVersionResponse` | `structure` | `Arn`, `CreationTimestamp`, `Id`, `Version` | - |
| `CreateFunctionDefinitionRequest` | `structure` | `AmznClientToken`, `InitialVersion`, `Name`, `tags` | - |
| `CreateFunctionDefinitionResponse` | `structure` | `Arn`, `CreationTimestamp`, `Id`, `LastUpdatedTimestamp`, `LatestVersion`, `LatestVersionArn`, `Name` | - |
| `CreateFunctionDefinitionVersionRequest` | `structure` | `AmznClientToken`, `DefaultConfig`, `FunctionDefinitionId`, `Functions` | Information needed to create a function definition version. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
