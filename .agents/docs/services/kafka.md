# Managed Streaming for Kafka

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The operations for managing an Amazon MSK cluster.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Managed Streaming for Kafka resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Managed Streaming for Kafka workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Update`, `Describe`, `Create`, `Delete` operation families, including `ListClientVpcConnections`, `ListClusterOperations`, `ListClusterOperationsV2`, `ListClusters`, `UpdateBrokerCount`, `UpdateBrokerStorage`.

## Service Identity and Protocol

- AWS model slug: `kafka`
- AWS SDK for Rust slug: `kafka`
- Model version: `2018-11-14`
- Model file: `vendor/api-models-aws/models/kafka/service/2018-11-14/kafka-2018-11-14.json`
- SDK ID: `Kafka`
- Endpoint prefix: `kafka`
- ARN namespace: `kafka`
- CloudFormation name: `MSK`
- CloudTrail event source: `kafka.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (14), `Update` (13), `Describe` (10), `Create` (6), `Delete` (6), `Get` (3), `Batch` (2), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchAssociateScramSecret`, `BatchDisassociateScramSecret`, `CreateCluster`, `CreateClusterV2`, `CreateConfiguration`, `CreateReplicator`, `CreateTopic`, `CreateVpcConnection`, `DeleteCluster`, `DeleteClusterPolicy`, `DeleteConfiguration`, `DeleteReplicator`, `DeleteTopic`, `DeleteVpcConnection`, `PutClusterPolicy`, `RejectClientVpcConnection`, `TagResource`, `UntagResource`, `UpdateBrokerCount`, `UpdateBrokerStorage`, ... (+11).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCluster`, `DescribeClusterOperation`, `DescribeClusterOperationV2`, `DescribeClusterV2`, `DescribeConfiguration`, `DescribeConfigurationRevision`, `DescribeReplicator`, `DescribeTopic`, `DescribeTopicPartitions`, `DescribeVpcConnection`, `GetBootstrapBrokers`, `GetClusterPolicy`, `GetCompatibleKafkaVersions`, `ListClientVpcConnections`, `ListClusterOperations`, `ListClusterOperationsV2`, `ListClusters`, `ListClustersV2`, `ListConfigurationRevisions`, `ListConfigurations`, ... (+7).
- Pagination is modelled for 14 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 59 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `SNS`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/msk/latest/developerguide/what-is-msk.html
- https://docs.aws.amazon.com/msk/latest/developerguide/msk-get-bootstrap-brokers.html
- https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration-properties.html

Research outcomes:
- Amazon MSK is a managed Apache Kafka service that provisions and operates Kafka clusters, brokers, storage, networking, scaling, and broker recovery.
- MSK clusters expose bootstrap broker strings that Kafka clients use to connect.
- MSK supports provisioned clusters and MSK Serverless, where serverless automatically manages capacity for streaming workloads.
- Custom MSK configurations define broker-level Kafka properties and can include dynamic properties.
- Some configuration changes apply dynamically while others can require rolling broker restarts.
- Topic-level configuration is distinct from cluster configuration and is managed through Kafka APIs or tools.
- MSK Connect manages Kafka Connect connectors with autoscaling, private connectivity, and connector health monitoring.

Parity implications:
- Model clusters, brokers, bootstrap brokers, configurations, configuration revisions, serverless clusters, VPC connectivity, storage, and operations separately.
- Configuration updates should distinguish dynamic changes from rolling-restart changes.
- Control-plane cluster state should not attempt to replace Kafka topic/partition data-plane semantics unless those APIs are explicitly modelled.

## Current Network Resource Stub Semantics

Managed Streaming for Kafka currently keeps cluster networking in Kafka-local state.

- Provisioned cluster records store broker node client subnet IDs and security group IDs supplied at creation.
- Serverless cluster records store VPC configs with subnet and security group IDs.
- Cluster state, bootstrap brokers, and connectivity information are not derived from EC2 subnet, security group, or VPC endpoint state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListClientVpcConnections`, `ListClusterOperations`, `ListClusterOperationsV2`, `ListClusters`, `ListClustersV2`, `ListConfigurationRevisions`, `ListConfigurations`, `ListKafkaVersions`, `ListNodes`, `ListReplicators`, `ListScramSecrets`, `ListTagsForResource`, `ListTopics`, `ListVpcConnections`
- Traits: `paginated` (13)
- Common required input members in this group: `ClusterArn`

### Update

- Operations: `UpdateBrokerCount`, `UpdateBrokerStorage`, `UpdateBrokerType`, `UpdateClusterConfiguration`, `UpdateClusterKafkaVersion`, `UpdateConfiguration`, `UpdateConnectivity`, `UpdateMonitoring`, `UpdateRebalancing`, `UpdateReplicationInfo`, `UpdateSecurity`, `UpdateStorage`, `UpdateTopic`
- Common required input members in this group: `ClusterArn`, `CurrentVersion`

### Describe

- Operations: `DescribeCluster`, `DescribeClusterOperation`, `DescribeClusterOperationV2`, `DescribeClusterV2`, `DescribeConfiguration`, `DescribeConfigurationRevision`, `DescribeReplicator`, `DescribeTopic`, `DescribeTopicPartitions`, `DescribeVpcConnection`
- Traits: `paginated` (1)
- Common required input members in this group: `ClusterArn`, `ClusterOperationArn`, `Arn`, `TopicName`

### Create

- Operations: `CreateCluster`, `CreateClusterV2`, `CreateConfiguration`, `CreateReplicator`, `CreateTopic`, `CreateVpcConnection`
- Common required input members in this group: `ClusterName`

### Delete

- Operations: `DeleteCluster`, `DeleteClusterPolicy`, `DeleteConfiguration`, `DeleteReplicator`, `DeleteTopic`, `DeleteVpcConnection`
- Common required input members in this group: `ClusterArn`, `Arn`

### Get

- Operations: `GetBootstrapBrokers`, `GetClusterPolicy`, `GetCompatibleKafkaVersions`
- Common required input members in this group: `ClusterArn`

### Batch

- Operations: `BatchAssociateScramSecret`, `BatchDisassociateScramSecret`
- Common required input members in this group: `ClusterArn`, `SecretArnList`

### Put

- Operations: `PutClusterPolicy`
- Common required input members in this group: -

### Reboot

- Operations: `RebootBroker`
- Common required input members in this group: -

### Reject

- Operations: `RejectClientVpcConnection`
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
| `BatchAssociateScramSecret` | `POST /v1/clusters/{ClusterArn}/scram-secrets` | - | `ClusterArn`, `SecretArnList` | - | `BatchAssociateScramSecretResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Associates one or more Scram Secrets with an Amazon MSK cluster. |
| `BatchDisassociateScramSecret` | `PATCH /v1/clusters/{ClusterArn}/scram-secrets` | - | `ClusterArn`, `SecretArnList` | - | `BatchDisassociateScramSecretResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Disassociates one or more Scram Secrets from an Amazon MSK cluster. |
| `CreateCluster` | `POST /v1/clusters` | - | `BrokerNodeGroupInfo`, `ClusterName`, `KafkaVersion`, `NumberOfBrokerNodes` | - | `CreateClusterResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a new MSK cluster. |
| `CreateClusterV2` | `POST /api/v2/clusters` | - | `ClusterName` | - | `CreateClusterV2Response` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a new MSK cluster. |
| `CreateConfiguration` | `POST /v1/configurations` | - | `Name`, `ServerProperties` | - | `CreateConfigurationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a new MSK configuration. |
| `CreateReplicator` | `POST /replication/v1/replicators` | - | `KafkaClusters`, `ReplicationInfoList`, `ReplicatorName`, `ServiceExecutionRoleArn` | - | `CreateReplicatorResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Creates the replicator. |
| `CreateTopic` | `POST /v1/clusters/{ClusterArn}/topics` | - | `ClusterArn`, `TopicName`, `PartitionCount`, `ReplicationFactor` | - | `CreateTopicResponse` | `BadRequestException`, `ClusterConnectivityException`, `ConflictException`, `ControllerMovedException`, `ForbiddenException`, `GroupSubscribedToTopicException`, `InternalServerErrorException`, `KafkaRequestException`, `KafkaTimeoutException`, `NotControllerException`, `ReassignmentInProgressException`, `ServiceUnavailableException`, `TooManyRequestsException`, `TopicExistsException`, `UnauthorizedException`, `UnknownTopicOrPartitionException` | Creates a topic in the specified MSK cluster. |
| `CreateVpcConnection` | `POST /v1/vpc-connection` | - | `TargetClusterArn`, `Authentication`, `VpcId`, `ClientSubnets`, `SecurityGroups` | - | `CreateVpcConnectionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a new MSK VPC connection. |
| `DeleteCluster` | `DELETE /v1/clusters/{ClusterArn}` | - | `ClusterArn` | - | `DeleteClusterResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Deletes the MSK cluster specified by the Amazon Resource Name (ARN) in the request. |
| `DeleteClusterPolicy` | `DELETE /v1/clusters/{ClusterArn}/policy` | - | `ClusterArn` | - | `DeleteClusterPolicyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Deletes the MSK cluster policy specified by the Amazon Resource Name (ARN) in the request. |
| `DeleteConfiguration` | `DELETE /v1/configurations/{Arn}` | - | `Arn` | - | `DeleteConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Deletes an MSK Configuration. |
| `DeleteReplicator` | `DELETE /replication/v1/replicators/{ReplicatorArn}` | - | `ReplicatorArn` | - | `DeleteReplicatorResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a replicator. |
| `DeleteTopic` | `DELETE /v1/clusters/{ClusterArn}/topics/{TopicName}` | - | `ClusterArn`, `TopicName` | - | `DeleteTopicResponse` | `BadRequestException`, `ClusterConnectivityException`, `ControllerMovedException`, `ForbiddenException`, `GroupSubscribedToTopicException`, `InternalServerErrorException`, `KafkaRequestException`, `KafkaTimeoutException`, `NotControllerException`, `NotFoundException`, `ReassignmentInProgressException`, `UnknownTopicOrPartitionException` | Deletes a topic in the specified MSK cluster. |
| `DeleteVpcConnection` | `DELETE /v1/vpc-connection/{Arn}` | - | `Arn` | - | `DeleteVpcConnectionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Deletes a MSK VPC connection. |
| `DescribeCluster` | `GET /v1/clusters/{ClusterArn}` | - | `ClusterArn` | - | `DescribeClusterResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `UnauthorizedException` | Returns a description of the MSK cluster whose Amazon Resource Name (ARN) is specified in the request. |
| `DescribeClusterOperation` | `GET /v1/operations/{ClusterOperationArn}` | - | `ClusterOperationArn` | - | `DescribeClusterOperationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `UnauthorizedException` | Returns a description of the cluster operation specified by the ARN. |
| `DescribeClusterOperationV2` | `GET /api/v2/operations/{ClusterOperationArn}` | - | `ClusterOperationArn` | - | `DescribeClusterOperationV2Response` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Returns a description of the cluster operation specified by the ARN. |
| `DescribeClusterV2` | `GET /api/v2/clusters/{ClusterArn}` | - | `ClusterArn` | - | `DescribeClusterV2Response` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `UnauthorizedException` | Returns a description of the MSK cluster whose Amazon Resource Name (ARN) is specified in the request. |
| `DescribeConfiguration` | `GET /v1/configurations/{Arn}` | - | `Arn` | - | `DescribeConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Returns a description of this MSK configuration. |
| `DescribeConfigurationRevision` | `GET /v1/configurations/{Arn}/revisions/{Revision}` | - | `Arn`, `Revision` | - | `DescribeConfigurationRevisionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Returns a description of this revision of the configuration. |
| `DescribeReplicator` | `GET /replication/v1/replicators/{ReplicatorArn}` | - | `ReplicatorArn` | - | `DescribeReplicatorResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Describes a replicator. |
| `DescribeTopic` | `GET /v1/clusters/{ClusterArn}/topics/{TopicName}` | - | `ClusterArn`, `TopicName` | - | `DescribeTopicResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `UnauthorizedException` | Returns topic details of this topic on a MSK cluster. |
| `DescribeTopicPartitions` | `GET /v1/clusters/{ClusterArn}/topics/{TopicName}/partitions` | `paginated` | `ClusterArn`, `TopicName` | - | `DescribeTopicPartitionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `UnauthorizedException` | Returns partition details of this topic on a MSK cluster. |
| `DescribeVpcConnection` | `GET /v1/vpc-connection/{Arn}` | - | `Arn` | - | `DescribeVpcConnectionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Returns a description of this MSK VPC connection. |
| `GetBootstrapBrokers` | `GET /v1/clusters/{ClusterArn}/bootstrap-brokers` | - | `ClusterArn` | - | `GetBootstrapBrokersResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `UnauthorizedException` | A list of brokers that a client application can use to bootstrap. This list doesn't necessarily include all of the brokers in the cluster. The following Python 3.6 example shows how you can use the Amazon Resource Na ... |
| `GetClusterPolicy` | `GET /v1/clusters/{ClusterArn}/policy` | - | `ClusterArn` | - | `GetClusterPolicyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Get the MSK cluster policy specified by the Amazon Resource Name (ARN) in the request. |
| `GetCompatibleKafkaVersions` | `GET /v1/compatible-kafka-versions` | - | - | - | `GetCompatibleKafkaVersionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Gets the Apache Kafka versions to which you can update the MSK cluster. |
| `ListClientVpcConnections` | `GET /v1/clusters/{ClusterArn}/client-vpc-connections` | `paginated` | `ClusterArn` | - | `ListClientVpcConnectionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Returns a list of all the VPC connections in this Region. |
| `ListClusterOperations` | `GET /v1/clusters/{ClusterArn}/operations` | `paginated` | `ClusterArn` | - | `ListClusterOperationsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `UnauthorizedException` | Returns a list of all the operations that have been performed on the specified MSK cluster. |
| `ListClusterOperationsV2` | `GET /api/v2/clusters/{ClusterArn}/operations` | `paginated` | `ClusterArn` | - | `ListClusterOperationsV2Response` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Returns a list of all the operations that have been performed on the specified MSK cluster. |
| `ListClusters` | `GET /v1/clusters` | `paginated` | - | - | `ListClustersResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `UnauthorizedException` | Returns a list of all the MSK clusters in the current Region. |
| `ListClustersV2` | `GET /api/v2/clusters` | `paginated` | - | - | `ListClustersV2Response` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `UnauthorizedException` | Returns a list of all the MSK clusters in the current Region. |
| `ListConfigurationRevisions` | `GET /v1/configurations/{Arn}/revisions` | `paginated` | `Arn` | - | `ListConfigurationRevisionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Returns a list of all the MSK configurations in this Region. |
| `ListConfigurations` | `GET /v1/configurations` | `paginated` | - | - | `ListConfigurationsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Returns a list of all the MSK configurations in this Region. |
| `ListKafkaVersions` | `GET /v1/kafka-versions` | `paginated` | - | - | `ListKafkaVersionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `UnauthorizedException` | Returns a list of Apache Kafka versions. |
| `ListNodes` | `GET /v1/clusters/{ClusterArn}/nodes` | `paginated` | `ClusterArn` | - | `ListNodesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Returns a list of the broker nodes in the cluster. |
| `ListReplicators` | `GET /replication/v1/replicators` | `paginated` | - | - | `ListReplicatorsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Lists the replicators. |
| `ListScramSecrets` | `GET /v1/clusters/{ClusterArn}/scram-secrets` | `paginated` | `ClusterArn` | - | `ListScramSecretsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Returns a list of the Scram Secrets associated with an Amazon MSK cluster. |
| `ListTagsForResource` | `GET /v1/tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Returns a list of the tags associated with the specified resource. |
| `ListTopics` | `GET /v1/clusters/{ClusterArn}/topics` | `paginated` | `ClusterArn` | - | `ListTopicsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | List topics in a MSK cluster. |
| `ListVpcConnections` | `GET /v1/vpc-connections` | `paginated` | - | - | `ListVpcConnectionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Returns a list of all the VPC connections in this Region. |
| `PutClusterPolicy` | `PUT /v1/clusters/{ClusterArn}/policy` | - | `ClusterArn`, `Policy` | - | `PutClusterPolicyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException` | Creates or updates the MSK cluster policy specified by the cluster Amazon Resource Name (ARN) in the request. |
| `RebootBroker` | `PUT /v1/clusters/{ClusterArn}/reboot-broker` | - | `BrokerIds`, `ClusterArn` | - | `RebootBrokerResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Reboots brokers. |
| `RejectClientVpcConnection` | `PUT /v1/clusters/{ClusterArn}/client-vpc-connection` | - | `ClusterArn`, `VpcConnectionArn` | - | `RejectClientVpcConnectionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Returns empty response. |
| `TagResource` | `POST /v1/tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Adds tags to the specified MSK resource. |
| `UntagResource` | `DELETE /v1/tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Removes the tags associated with the keys that are provided in the query. |
| `UpdateBrokerCount` | `PUT /v1/clusters/{ClusterArn}/nodes/count` | - | `ClusterArn`, `CurrentVersion`, `TargetNumberOfBrokerNodes` | - | `UpdateBrokerCountResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Updates the number of broker nodes in the cluster. |
| `UpdateBrokerStorage` | `PUT /v1/clusters/{ClusterArn}/nodes/storage` | - | `ClusterArn`, `CurrentVersion`, `TargetBrokerEBSVolumeInfo` | - | `UpdateBrokerStorageResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Updates the EBS storage associated with MSK brokers. |
| `UpdateBrokerType` | `PUT /v1/clusters/{ClusterArn}/nodes/type` | - | `ClusterArn`, `CurrentVersion`, `TargetInstanceType` | - | `UpdateBrokerTypeResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Updates EC2 instance type. |
| `UpdateClusterConfiguration` | `PUT /v1/clusters/{ClusterArn}/configuration` | - | `ClusterArn`, `ConfigurationInfo`, `CurrentVersion` | - | `UpdateClusterConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Updates the cluster with the configuration that is specified in the request body. |
| `UpdateClusterKafkaVersion` | `PUT /v1/clusters/{ClusterArn}/version` | - | `ClusterArn`, `CurrentVersion`, `TargetKafkaVersion` | - | `UpdateClusterKafkaVersionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Updates the Apache Kafka version for the cluster. |
| `UpdateConfiguration` | `PUT /v1/configurations/{Arn}` | - | `Arn`, `ServerProperties` | - | `UpdateConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Updates an MSK configuration. |
| `UpdateConnectivity` | `PUT /v1/clusters/{ClusterArn}/connectivity` | - | `ClusterArn`, `CurrentVersion` | - | `UpdateConnectivityResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Updates the cluster's connectivity configuration. |
| `UpdateMonitoring` | `PUT /v1/clusters/{ClusterArn}/monitoring` | - | `ClusterArn`, `CurrentVersion` | - | `UpdateMonitoringResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Updates the monitoring settings for the cluster. You can use this operation to specify which Apache Kafka metrics you want Amazon MSK to send to Amazon CloudWatch. You can also specify settings for open monitoring wi ... |
| `UpdateRebalancing` | `PUT /v1/clusters/{ClusterArn}/rebalancing` | - | `ClusterArn`, `CurrentVersion`, `Rebalancing` | - | `UpdateRebalancingResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Use this resource to update the intelligent rebalancing status of an Amazon MSK Provisioned cluster with Express brokers. |
| `UpdateReplicationInfo` | `PUT /replication/v1/replicators/{ReplicatorArn}/replication-info` | - | `CurrentVersion`, `ReplicatorArn` | - | `UpdateReplicationInfoResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Updates replication info of a replicator. |
| `UpdateSecurity` | `PATCH /v1/clusters/{ClusterArn}/security` | - | `ClusterArn`, `CurrentVersion` | - | `UpdateSecurityResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Updates the security settings for the cluster. You can use this operation to specify encryption and authentication on existing clusters. |
| `UpdateStorage` | `PUT /v1/clusters/{ClusterArn}/storage` | - | `ClusterArn`, `CurrentVersion` | - | `UpdateStorageResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Updates cluster broker volume size (or) sets cluster storage mode to TIERED. |
| `UpdateTopic` | `PUT /v1/clusters/{ClusterArn}/topics/{TopicName}` | - | `ClusterArn`, `TopicName` | - | `UpdateTopicResponse` | `BadRequestException`, `ClusterConnectivityException`, `ControllerMovedException`, `ForbiddenException`, `GroupSubscribedToTopicException`, `InternalServerErrorException`, `KafkaRequestException`, `KafkaTimeoutException`, `NotControllerException`, `NotFoundException`, `ReassignmentInProgressException`, `ServiceUnavailableException`, `UnauthorizedException`, `UnknownTopicOrPartitionException` | Updates the configuration of the specified topic. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteCluster` | - | `CurrentVersion -> currentVersion` | - | - |
| `DeleteReplicator` | - | `CurrentVersion -> currentVersion` | - | - |
| `DescribeTopicPartitions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetCompatibleKafkaVersions` | - | `ClusterArn -> clusterArn` | - | - |
| `ListClientVpcConnections` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListClusterOperations` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListClusterOperationsV2` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListClusters` | - | `ClusterNameFilter -> clusterNameFilter`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListClustersV2` | - | `ClusterNameFilter -> clusterNameFilter`, `ClusterTypeFilter -> clusterTypeFilter`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListConfigurationRevisions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListConfigurations` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListKafkaVersions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListNodes` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListReplicators` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `ReplicatorNameFilter -> replicatorNameFilter` | - | - |
| `ListScramSecrets` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListTopics` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `TopicNameFilter -> topicNameFilter` | - | - |
| `ListVpcConnections` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `ClusterConnectivityException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `ConflictException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `ControllerMovedException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `ForbiddenException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `GroupSubscribedToTopicException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `InternalServerErrorException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `KafkaRequestException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `KafkaTimeoutException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `NotControllerException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `NotFoundException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `ReassignmentInProgressException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `ServiceUnavailableException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `TooManyRequestsException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `TopicExistsException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `UnauthorizedException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `UnknownTopicOrPartitionException` | `structure` | InvalidParameter, Message | Returns information about an error. |
| `BatchAssociateScramSecretRequest` | `structure` | ClusterArn, SecretArnList | Associates sasl scram secrets to cluster. |
| `BatchAssociateScramSecretResponse` | `structure` | ClusterArn, UnprocessedScramSecrets | - |
| `BatchDisassociateScramSecretRequest` | `structure` | ClusterArn, SecretArnList | Disassociates sasl scram secrets to cluster. |
| `BatchDisassociateScramSecretResponse` | `structure` | ClusterArn, UnprocessedScramSecrets | - |
| `CreateClusterRequest` | `structure` | BrokerNodeGroupInfo, Rebalancing, ClientAuthentication, ClusterName, ConfigurationInfo, EncryptionInfo, EnhancedMonitoring, OpenMonitoring, KafkaVersion, LoggingInfo, NumberOfBrokerNodes, Tags, ... (+1) | - |
| `CreateClusterResponse` | `structure` | ClusterArn, ClusterName, State | - |
| `CreateClusterV2Request` | `structure` | ClusterName, Tags, Provisioned, Serverless | - |
| `CreateClusterV2Response` | `structure` | ClusterArn, ClusterName, State, ClusterType | - |
| `CreateConfigurationRequest` | `structure` | Description, KafkaVersions, Name, ServerProperties | - |
| `CreateConfigurationResponse` | `structure` | Arn, CreationTime, LatestRevision, Name, State | - |
| `CreateReplicatorRequest` | `structure` | Description, KafkaClusters, ReplicationInfoList, ReplicatorName, ServiceExecutionRoleArn, Tags, LogDelivery | Creates a replicator using the specified configuration. |
| `CreateReplicatorResponse` | `structure` | ReplicatorArn, ReplicatorName, ReplicatorState | - |
| `CreateTopicRequest` | `structure` | ClusterArn, TopicName, PartitionCount, ReplicationFactor, Configs | - |
| `CreateTopicResponse` | `structure` | TopicArn, TopicName, Status | - |
| `CreateVpcConnectionRequest` | `structure` | TargetClusterArn, Authentication, VpcId, ClientSubnets, SecurityGroups, Tags | - |
| `CreateVpcConnectionResponse` | `structure` | VpcConnectionArn, State, Authentication, VpcId, ClientSubnets, SecurityGroups, CreationTime, Tags | - |
| `DeleteClusterRequest` | `structure` | ClusterArn, CurrentVersion | - |
| `DeleteClusterResponse` | `structure` | ClusterArn, State | - |
| `DeleteClusterPolicyRequest` | `structure` | ClusterArn | - |
| `DeleteClusterPolicyResponse` | `structure` | **empty (no members)** | - |
| `DeleteConfigurationRequest` | `structure` | Arn | - |
| `DeleteConfigurationResponse` | `structure` | Arn, State | - |
| `DeleteReplicatorRequest` | `structure` | CurrentVersion, ReplicatorArn | - |
| `BrokerAZDistribution` | `enum` | DEFAULT | The distribution of broker nodes across Availability Zones. This is an optional parameter. If you don't specify it, Amazon MSK gives it the value DEFAULT. Y ... |
| `ClientBroker` | `enum` | TLS, TLS_PLAINTEXT, PLAINTEXT | Client-broker encryption in transit setting. |
| `ClusterState` | `enum` | ACTIVE, CREATING, DELETING, FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, UPDATING | The state of the Apache Kafka cluster. |
| `ClusterType` | `enum` | PROVISIONED, SERVERLESS | The type of cluster. |
| `ConfigurationState` | `enum` | ACTIVE, DELETING, DELETE_FAILED | The state of a configuration. |
| `ConsumerGroupOffsetSyncMode` | `enum` | LEGACY, ENHANCED | The consumer group offset synchronization mode. With LEGACY, offsets are synchronized when producers write to the source cluster. With ENHANCED, consumer of ... |
| `CustomerActionStatus` | `enum` | CRITICAL_ACTION_REQUIRED, ACTION_RECOMMENDED, NONE | A type of an action required from the customer. |
| `EnhancedMonitoring` | `enum` | DEFAULT, PER_BROKER, PER_TOPIC_PER_BROKER, PER_TOPIC_PER_PARTITION | Specifies which metrics are gathered for the MSK cluster. This property has the following possible values: DEFAULT, PER_BROKER, PER_TOPIC_PER_BROKER, and PE ... |
| `KafkaClusterEncryptionInTransitType` | `enum` | TLS | The type of encryption in transit to the Apache Kafka cluster. |
| `KafkaClusterSaslScramMechanism` | `enum` | SHA256, SHA512 | The SASL/SCRAM authentication mechanism. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
