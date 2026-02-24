# Managed Streaming for Kafka Connect

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

No high-level service documentation is embedded in the AWS API model.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Managed Streaming for Kafka Connect workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Tag` operation families, including `ListConnectorOperations`, `ListConnectors`, `ListCustomPlugins`, `ListTagsForResource`, `DescribeConnector`, `DescribeConnectorOperation`.

## Service Identity and Protocol

- AWS model slug: `kafkaconnect`
- AWS SDK for Rust slug: `kafkaconnect`
- Model version: `2021-09-14`
- Model file: `vendor/api-models-aws/models/kafkaconnect/service/2021-09-14/kafkaconnect-2021-09-14.json`
- SDK ID: `KafkaConnect`
- Endpoint prefix: `kafkaconnect`
- ARN namespace: `kafkaconnect`
- CloudFormation name: `KafkaConnect`
- CloudTrail event source: `kafkaconnect.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Describe` (4), `Create` (3), `Delete` (3), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateConnector`, `CreateCustomPlugin`, `CreateWorkerConfiguration`, `DeleteConnector`, `DeleteCustomPlugin`, `DeleteWorkerConfiguration`, `TagResource`, `UntagResource`, `UpdateConnector`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeConnector`, `DescribeConnectorOperation`, `DescribeCustomPlugin`, `DescribeWorkerConfiguration`, `ListConnectorOperations`, `ListConnectors`, `ListCustomPlugins`, `ListTagsForResource`, `ListWorkerConfigurations`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 18 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`, `ECS`.

## Operation Groups

### List

- Operations: `ListConnectorOperations`, `ListConnectors`, `ListCustomPlugins`, `ListTagsForResource`, `ListWorkerConfigurations`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `connectorArn`, `resourceArn`

### Describe

- Operations: `DescribeConnector`, `DescribeConnectorOperation`, `DescribeCustomPlugin`, `DescribeWorkerConfiguration`
- Traits: `readonly` (4)
- Common required input members in this group: `connectorArn`, `connectorOperationArn`, `customPluginArn`, `workerConfigurationArn`

### Create

- Operations: `CreateConnector`, `CreateCustomPlugin`, `CreateWorkerConfiguration`
- Common required input members in this group: `capacity`, `connectorConfiguration`, `connectorName`, `contentType`, `kafkaCluster`, `kafkaClusterClientAuthentication`, `kafkaClusterEncryptionInTransit`, `kafkaConnectVersion`, `location`, `name`, `plugins`, `propertiesFileContent`, `serviceExecutionRoleArn`

### Delete

- Operations: `DeleteConnector`, `DeleteCustomPlugin`, `DeleteWorkerConfiguration`
- Traits: `idempotent` (3)
- Common required input members in this group: `connectorArn`, `customPluginArn`, `workerConfigurationArn`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateConnector`
- Traits: `idempotent` (1)
- Common required input members in this group: `connectorArn`, `currentVersion`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateConnector` | `POST /v1/connectors` | - | `capacity`, `connectorConfiguration`, `connectorName`, `kafkaCluster`, `kafkaClusterClientAuthentication`, `kafkaClusterEncryptionInTransit`, `kafkaConnectVersion`, `plugins`, `serviceExecutionRoleArn` | - | `CreateConnectorResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a connector using the specified properties. |
| `CreateCustomPlugin` | `POST /v1/custom-plugins` | - | `contentType`, `location`, `name` | - | `CreateCustomPluginResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a custom plugin using the specified properties. |
| `CreateWorkerConfiguration` | `POST /v1/worker-configurations` | - | `name`, `propertiesFileContent` | - | `CreateWorkerConfigurationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a worker configuration using the specified properties. |
| `DeleteConnector` | `DELETE /v1/connectors/{connectorArn}` | `idempotent` | `connectorArn` | - | `DeleteConnectorResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes the specified connector. |
| `DeleteCustomPlugin` | `DELETE /v1/custom-plugins/{customPluginArn}` | `idempotent` | `customPluginArn` | - | `DeleteCustomPluginResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a custom plugin. |
| `DeleteWorkerConfiguration` | `DELETE /v1/worker-configurations/{workerConfigurationArn}` | `idempotent` | `workerConfigurationArn` | - | `DeleteWorkerConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes the specified worker configuration. |
| `DescribeConnector` | `GET /v1/connectors/{connectorArn}` | `readonly` | `connectorArn` | - | `DescribeConnectorResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Returns summary information about the connector. |
| `DescribeConnectorOperation` | `GET /v1/connectorOperations/{connectorOperationArn}` | `readonly` | `connectorOperationArn` | - | `DescribeConnectorOperationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Returns information about the specified connector's operations. |
| `DescribeCustomPlugin` | `GET /v1/custom-plugins/{customPluginArn}` | `readonly` | `customPluginArn` | - | `DescribeCustomPluginResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | A summary description of the custom plugin. |
| `DescribeWorkerConfiguration` | `GET /v1/worker-configurations/{workerConfigurationArn}` | `readonly` | `workerConfigurationArn` | - | `DescribeWorkerConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Returns information about a worker configuration. |
| `ListConnectorOperations` | `GET /v1/connectors/{connectorArn}/operations` | `readonly`, `paginated` | `connectorArn` | - | `ListConnectorOperationsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Lists information about a connector's operation(s). |
| `ListConnectors` | `GET /v1/connectors` | `readonly`, `paginated` | - | - | `ListConnectorsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Returns a list of all the connectors in this account and Region. The list is limited to connectors whose name starts with the specified prefix. |
| `ListCustomPlugins` | `GET /v1/custom-plugins` | `readonly`, `paginated` | - | - | `ListCustomPluginsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Returns a list of all of the custom plugins in this account and Region. |
| `ListTagsForResource` | `GET /v1/tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Lists all the tags attached to the specified resource. |
| `ListWorkerConfigurations` | `GET /v1/worker-configurations` | `readonly`, `paginated` | - | - | `ListWorkerConfigurationsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Returns a list of all of the worker configurations in this account and Region. |
| `TagResource` | `POST /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Attaches tags to the specified resource. |
| `UntagResource` | `DELETE /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Removes tags from the specified resource. |
| `UpdateConnector` | `PUT /v1/connectors/{connectorArn}` | `idempotent` | `connectorArn`, `currentVersion` | - | `UpdateConnectorResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Updates the specified connector. For request body, specify only one parameter: either `capacity` or `connectorConfiguration`. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `message` | HTTP Status Code 400: Bad request due to incorrect input. |
| `ForbiddenException` | `structure` | `message` | HTTP Status Code 403: Access forbidden. |
| `InternalServerErrorException` | `structure` | `message` | HTTP Status Code 500: Unexpected internal server error. |
| `NotFoundException` | `structure` | `message` | HTTP Status Code 404: Resource not found due to incorrect input. |
| `ServiceUnavailableException` | `structure` | `message` | HTTP Status Code 503: Service Unavailable. |
| `TooManyRequestsException` | `structure` | `message` | HTTP Status Code 429: Limit exceeded. |
| `UnauthorizedException` | `structure` | `message` | HTTP Status Code 401: Unauthorized request. |
| `ConflictException` | `structure` | `message` | HTTP Status Code 409: Conflict. |
| `CreateConnectorRequest` | `structure` | `capacity`, `connectorConfiguration`, `connectorDescription`, `connectorName`, `kafkaCluster`, `kafkaClusterClientAuthentication`, `kafkaClusterEncryptionInTransit`, `kafkaConnectVersion`, `logDelivery`, `networkType`, `plugins`, `serviceExecutionRoleArn`, ... (+2) | - |
| `CreateConnectorResponse` | `structure` | `connectorArn`, `connectorName`, `connectorState` | - |
| `CreateCustomPluginRequest` | `structure` | `contentType`, `description`, `location`, `name`, `tags` | - |
| `CreateCustomPluginResponse` | `structure` | `customPluginArn`, `customPluginState`, `name`, `revision` | - |
| `CreateWorkerConfigurationRequest` | `structure` | `description`, `name`, `propertiesFileContent`, `tags` | - |
| `CreateWorkerConfigurationResponse` | `structure` | `creationTime`, `latestRevision`, `name`, `workerConfigurationArn`, `workerConfigurationState` | - |
| `DeleteConnectorRequest` | `structure` | `connectorArn`, `currentVersion` | - |
| `DeleteConnectorResponse` | `structure` | `connectorArn`, `connectorState` | - |
| `DeleteCustomPluginRequest` | `structure` | `customPluginArn` | - |
| `DeleteCustomPluginResponse` | `structure` | `customPluginArn`, `customPluginState` | - |
| `DeleteWorkerConfigurationRequest` | `structure` | `workerConfigurationArn` | - |
| `DeleteWorkerConfigurationResponse` | `structure` | `workerConfigurationArn`, `workerConfigurationState` | - |
| `DescribeConnectorRequest` | `structure` | `connectorArn` | - |
| `DescribeConnectorResponse` | `structure` | `capacity`, `connectorArn`, `connectorConfiguration`, `connectorDescription`, `connectorName`, `connectorState`, `creationTime`, `currentVersion`, `kafkaCluster`, `kafkaClusterClientAuthentication`, `kafkaClusterEncryptionInTransit`, `kafkaConnectVersion`, ... (+6) | - |
| `DescribeConnectorOperationRequest` | `structure` | `connectorOperationArn` | - |
| `DescribeConnectorOperationResponse` | `structure` | `connectorArn`, `connectorOperationArn`, `connectorOperationState`, `connectorOperationType`, `creationTime`, `endTime`, `errorInfo`, `operationSteps`, `originConnectorConfiguration`, `originWorkerSetting`, `targetConnectorConfiguration`, `targetWorkerSetting` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
