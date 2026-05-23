# AmazonMQ

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon MQ is a managed message broker service for Apache ActiveMQ and RabbitMQ that makes it easy to set up and operate message brokers in the cloud. A message broker allows software applications and components to communicate using various programming languages, operating systems, and formal messaging protocols.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AmazonMQ by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented AmazonMQ workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Create`, `Delete`, `Update` operation families, including `DescribeBroker`, `DescribeBrokerEngineTypes`, `DescribeBrokerInstanceOptions`, `DescribeConfiguration`, `ListBrokers`, `ListConfigurationRevisions`.

## Service Identity and Protocol

- AWS model slug: `mq`
- AWS SDK for Rust slug: `mq`
- Model version: `2017-11-27`
- Model file: `vendor/api-models-aws/models/mq/service/2017-11-27/mq-2017-11-27.json`
- SDK ID: `mq`
- Endpoint prefix: `mq`
- ARN namespace: `mq`
- CloudFormation name: `AmazonMQ`
- CloudTrail event source: `mq.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (6), `List` (5), `Create` (4), `Delete` (4), `Update` (3), `Promote` (1), `Reboot` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateBroker`, `CreateConfiguration`, `CreateTags`, `CreateUser`, `DeleteBroker`, `DeleteConfiguration`, `DeleteTags`, `DeleteUser`, `UpdateBroker`, `UpdateConfiguration`, `UpdateUser`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBroker`, `DescribeBrokerEngineTypes`, `DescribeBrokerInstanceOptions`, `DescribeConfiguration`, `DescribeConfigurationRevision`, `DescribeUser`, `ListBrokers`, `ListConfigurationRevisions`, `ListConfigurations`, `ListTags`, `ListUsers`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 24 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/welcome.html
- https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/getting-started-activemq.html
- https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/data-protection.html

Research outcomes:
- Amazon MQ is a managed message broker service for Apache ActiveMQ and RabbitMQ.
- Brokers are created with engine type/version, deployment mode, instance type, storage, users, network settings, encryption, and maintenance settings.
- ActiveMQ and RabbitMQ brokers expose protocol-specific endpoints and management consoles.
- Deployment modes control availability characteristics, such as single-instance and highly available broker deployments.
- Broker data at rest is encrypted with AWS owned or customer managed KMS keys, and data in transit uses TLS.
- CloudWatch metrics and broker logs provide monitoring, with metrics polled periodically.
- Broker users and engine-level permissions are separate from IAM control-plane permissions.

Parity implications:
- Model brokers, engine type, deployment mode, users, configurations, revisions, endpoints, encryption, logs, maintenance windows, and broker state separately.
- Broker creation/update should be asynchronous and engine-specific.
- IAM authorisation and broker user authentication should remain separate layers.

## Current Network Resource Stub Semantics

Amazon MQ currently treats broker networking as broker metadata.

- Broker model fields include subnet IDs and security groups, but current create/describe paths return simplified stored values and often default networking lists to empty.
- Pending security groups are local broker fields and are not the result of EC2 security group mutation.
- Broker endpoints and deployment state are independent of subnet availability.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeBroker`, `DescribeBrokerEngineTypes`, `DescribeBrokerInstanceOptions`, `DescribeConfiguration`, `DescribeConfigurationRevision`, `DescribeUser`
- Common required input members in this group: `BrokerId`, `ConfigurationId`, `ConfigurationRevision`, `Username`

### List

- Operations: `ListBrokers`, `ListConfigurationRevisions`, `ListConfigurations`, `ListTags`, `ListUsers`
- Traits: `paginated` (1)
- Common required input members in this group: `BrokerId`, `ConfigurationId`, `ResourceArn`

### Create

- Operations: `CreateBroker`, `CreateConfiguration`, `CreateTags`, `CreateUser`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `BrokerId`, `BrokerName`, `DeploymentMode`, `EngineType`, `HostInstanceType`, `Name`, `Password`, `PubliclyAccessible`, `ResourceArn`, `Username`

### Delete

- Operations: `DeleteBroker`, `DeleteConfiguration`, `DeleteTags`, `DeleteUser`
- Common required input members in this group: `BrokerId`, `ConfigurationId`, `ResourceArn`, `TagKeys`, `Username`

### Update

- Operations: `UpdateBroker`, `UpdateConfiguration`, `UpdateUser`
- Common required input members in this group: `BrokerId`, `ConfigurationId`, `Data`, `Username`

### Promote

- Operations: `Promote`
- Common required input members in this group: `BrokerId`, `Mode`

### Reboot

- Operations: `RebootBroker`
- Common required input members in this group: `BrokerId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateBroker` | `POST /v1/brokers` | `idempotency-token` | `BrokerName`, `DeploymentMode`, `EngineType`, `HostInstanceType`, `PubliclyAccessible` | `CreatorRequestId` | `CreateBrokerResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `UnauthorizedException` | Creates a broker. Note: This API is asynchronous. |
| `CreateConfiguration` | `POST /v1/configurations` | - | `EngineType`, `Name` | - | `CreateConfigurationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException` | Creates a new configuration for the specified configuration name. Amazon MQ uses the default configuration (the engine type and version). |
| `CreateTags` | `POST /v1/tags/{ResourceArn}` | - | `ResourceArn` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Add a tag to a resource. |
| `CreateUser` | `POST /v1/brokers/{BrokerId}/users/{Username}` | - | `BrokerId`, `Password`, `Username` | - | `CreateUserResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Creates an ActiveMQ user. Do not add personally identifiable information (PII) or other confidential or sensitive information in broker usernames. |
| `DeleteBroker` | `DELETE /v1/brokers/{BrokerId}` | - | `BrokerId` | - | `DeleteBrokerResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Deletes a broker. Note: This API is asynchronous. |
| `DeleteConfiguration` | `DELETE /v1/configurations/{ConfigurationId}` | - | `ConfigurationId` | - | `DeleteConfigurationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Deletes the specified configuration. |
| `DeleteTags` | `DELETE /v1/tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Removes a tag from a resource. |
| `DeleteUser` | `DELETE /v1/brokers/{BrokerId}/users/{Username}` | - | `BrokerId`, `Username` | - | `DeleteUserResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Deletes an ActiveMQ user. |
| `DescribeBroker` | `GET /v1/brokers/{BrokerId}` | - | `BrokerId` | - | `DescribeBrokerResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Returns information about the specified broker. |
| `DescribeBrokerEngineTypes` | `GET /v1/broker-engine-types` | - | - | - | `DescribeBrokerEngineTypesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException` | Describe available engine types and versions. |
| `DescribeBrokerInstanceOptions` | `GET /v1/broker-instance-options` | - | - | - | `DescribeBrokerInstanceOptionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException` | Describe available broker instance options. |
| `DescribeConfiguration` | `GET /v1/configurations/{ConfigurationId}` | - | `ConfigurationId` | - | `DescribeConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Returns information about the specified configuration. |
| `DescribeConfigurationRevision` | `GET /v1/configurations/{ConfigurationId}/revisions/{ConfigurationRevision}` | - | `ConfigurationId`, `ConfigurationRevision` | - | `DescribeConfigurationRevisionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Returns the specified configuration revision for the specified configuration. |
| `DescribeUser` | `GET /v1/brokers/{BrokerId}/users/{Username}` | - | `BrokerId`, `Username` | - | `DescribeUserResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Returns information about an ActiveMQ user. |
| `ListBrokers` | `GET /v1/brokers` | `paginated` | - | - | `ListBrokersResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException` | Returns a list of all brokers. |
| `ListConfigurationRevisions` | `GET /v1/configurations/{ConfigurationId}/revisions` | - | `ConfigurationId` | - | `ListConfigurationRevisionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Returns a list of all revisions for the specified configuration. |
| `ListConfigurations` | `GET /v1/configurations` | - | - | - | `ListConfigurationsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException` | Returns a list of all configurations. |
| `ListTags` | `GET /v1/tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Lists tags for a resource. |
| `ListUsers` | `GET /v1/brokers/{BrokerId}/users` | - | `BrokerId` | - | `ListUsersResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Returns a list of all ActiveMQ users. |
| `Promote` | `POST /v1/brokers/{BrokerId}/promote` | - | `BrokerId`, `Mode` | - | `PromoteResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Promotes a data replication replica broker to the primary broker role. |
| `RebootBroker` | `POST /v1/brokers/{BrokerId}/reboot` | - | `BrokerId` | - | `RebootBrokerResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Reboots a broker. Note: This API is asynchronous. |
| `UpdateBroker` | `PUT /v1/brokers/{BrokerId}` | - | `BrokerId` | - | `UpdateBrokerResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Adds a pending configuration change to a broker. |
| `UpdateConfiguration` | `PUT /v1/configurations/{ConfigurationId}` | - | `ConfigurationId`, `Data` | - | `UpdateConfigurationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Updates the specified configuration. |
| `UpdateUser` | `PUT /v1/brokers/{BrokerId}/users/{Username}` | - | `BrokerId`, `Username` | - | `UpdateUserResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Updates the information for an ActiveMQ user. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteTags` | - | `TagKeys -> tagKeys` | - | - |
| `DescribeBrokerEngineTypes` | - | `EngineType -> engineType`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `DescribeBrokerInstanceOptions` | - | `EngineType -> engineType`, `HostInstanceType -> hostInstanceType`, `MaxResults -> maxResults`, `NextToken -> nextToken`, `StorageType -> storageType` | - | - |
| `ListBrokers` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListConfigurationRevisions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListConfigurations` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListUsers` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `ErrorAttribute`, `Message` | Returns information about an error. |
| `ForbiddenException` | `structure` | `ErrorAttribute`, `Message` | Returns information about an error. |
| `InternalServerErrorException` | `structure` | `ErrorAttribute`, `Message` | Returns information about an error. |
| `NotFoundException` | `structure` | `ErrorAttribute`, `Message` | Returns information about an error. |
| `ConflictException` | `structure` | `ErrorAttribute`, `Message` | Returns information about an error. |
| `CreateBrokerRequest` | `structure` | `AuthenticationStrategy`, `AutoMinorVersionUpgrade`, `BrokerName`, `Configuration`, `CreatorRequestId`, `DataReplicationMode`, `DataReplicationPrimaryBrokerArn`, `DeploymentMode`, `EncryptionOptions`, `EngineType`, `EngineVersion`, `HostInstanceType`, ... (+9) | Creates a broker using the specified properties. |
| `CreateBrokerResponse` | `structure` | `BrokerArn`, `BrokerId` | - |
| `UnauthorizedException` | `structure` | `ErrorAttribute`, `Message` | Returns information about an error. |
| `CreateConfigurationRequest` | `structure` | `AuthenticationStrategy`, `EngineType`, `EngineVersion`, `Name`, `Tags` | Creates a new configuration for the specified configuration name. |
| `CreateConfigurationResponse` | `structure` | `Arn`, `AuthenticationStrategy`, `Created`, `Id`, `LatestRevision`, `Name` | - |
| `CreateTagsRequest` | `structure` | `ResourceArn`, `Tags` | A map of the key-value pairs for the resource tag. |
| `CreateUserRequest` | `structure` | `BrokerId`, `ConsoleAccess`, `Groups`, `Password`, `ReplicationUser`, `Username` | Creates a new ActiveMQ user. |
| `CreateUserResponse` | `structure` | - | - |
| `DeleteBrokerRequest` | `structure` | `BrokerId` | - |
| `DeleteBrokerResponse` | `structure` | `BrokerId` | - |
| `DeleteConfigurationRequest` | `structure` | `ConfigurationId` | - |
| `DeleteConfigurationResponse` | `structure` | `ConfigurationId` | - |
| `DeleteTagsRequest` | `structure` | `ResourceArn`, `TagKeys` | - |
| `DeleteUserRequest` | `structure` | `BrokerId`, `Username` | - |
| `DeleteUserResponse` | `structure` | - | - |
| `DescribeBrokerRequest` | `structure` | `BrokerId` | - |
| `DescribeBrokerResponse` | `structure` | `ActionsRequired`, `AuthenticationStrategy`, `AutoMinorVersionUpgrade`, `BrokerArn`, `BrokerId`, `BrokerInstances`, `BrokerName`, `BrokerState`, `Configurations`, `Created`, `DataReplicationMetadata`, `DataReplicationMode`, ... (+21) | - |
| `DescribeBrokerEngineTypesRequest` | `structure` | `EngineType`, `MaxResults`, `NextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
