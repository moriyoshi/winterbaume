# AWS App Runner

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

App Runner App Runner is an application service that provides a fast, simple, and cost-effective way to go directly from an existing container image or source code to a running service in the Amazon Web Services Cloud in seconds. You don't need to learn new technologies, decide which compute service to use, or understand how to provision and configure Amazon Web Services resources. App Runner connects directly to your container registry or source code repository. It provides an automatic delivery pipeline with fully managed operations, high performance, scalability, and security. For more information about App Runner, see the App Runner Developer Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS App Runner where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS App Runner by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS App Runner by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: create and operate container or source-code services with auto deployments, custom domains, VPC connectors, and observability settings.
- From the operation surface: model service deployment, pause/resume, connection management, auto-scaling configuration, and tag-based App Runner administration.

## Service Identity and Protocol

- AWS model slug: `apprunner`
- AWS SDK for Rust slug: `apprunner`
- Model version: `2020-05-15`
- Model file: `vendor/api-models-aws/models/apprunner/service/2020-05-15/apprunner-2020-05-15.json`
- SDK ID: `AppRunner`
- Endpoint prefix: `apprunner`
- ARN namespace: `apprunner`
- CloudFormation name: `AppRunner`
- CloudTrail event source: `apprunner.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (9), `Create` (6), `Delete` (6), `Describe` (6), `Update` (3), `Associate` (1), `Disassociate` (1), `Pause` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateCustomDomain`, `CreateAutoScalingConfiguration`, `CreateConnection`, `CreateObservabilityConfiguration`, `CreateService`, `CreateVpcConnector`, `CreateVpcIngressConnection`, `DeleteAutoScalingConfiguration`, `DeleteConnection`, `DeleteObservabilityConfiguration`, `DeleteService`, `DeleteVpcConnector`, `DeleteVpcIngressConnection`, `DisassociateCustomDomain`, `StartDeployment`, `TagResource`, `UntagResource`, `UpdateDefaultAutoScalingConfiguration`, `UpdateService`, `UpdateVpcIngressConnection`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAutoScalingConfiguration`, `DescribeCustomDomains`, `DescribeObservabilityConfiguration`, `DescribeService`, `DescribeVpcConnector`, `DescribeVpcIngressConnection`, `ListAutoScalingConfigurations`, `ListConnections`, `ListObservabilityConfigurations`, `ListOperations`, `ListServices`, `ListServicesForAutoScalingConfiguration`, `ListTagsForResource`, `ListVpcConnectors`, `ListVpcIngressConnections`.
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDeployment`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 37 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`, `EC2/VPC`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/apprunner/latest/dg/what-is-apprunner.html
- https://docs.aws.amazon.com/apprunner/latest/dg/service-source-code.html
- https://docs.aws.amazon.com/apprunner/latest/dg/getting-started.html

Research outcomes:
- App Runner creates managed web services from source code repositories or container images and handles build, deploy, load balancing, HTTPS, and scaling.
- Source-code services connect to repository providers and use managed runtimes plus build and start commands.
- Source directory selection limits which repository subtree App Runner uses for build and deployment.
- App Runner can deploy automatically from source changes when automatic deployments are enabled, or only when manually triggered.
- Runtime configuration can come from service settings or an `apprunner.yaml` configuration file.
- Service updates such as source code changes or configuration changes create new deployments.
- App Runner integrates with CloudWatch Logs and CloudTrail for logs and API activity.
- Managed runtime versions can reach end of support and require migration.

Parity implications:
- Model services, source configuration, image repositories, connection resources, deployments, build configuration, runtime configuration, health checks, auto scaling configuration, observability, and custom domains separately.
- Service creation and update should create deployments with asynchronous status transitions.
- Automatic deployment state should determine whether source changes trigger deployments.

## Current Network Resource Stub Semantics

App Runner exposes VPC-aware operations in the model but currently does not persist any networking resource state.

- `CreateVpcConnector`, `DescribeVpcConnector`, `ListVpcConnectors`, and `DeleteVpcConnector` return `501 NotImplemented`.
- `CreateVpcIngressConnection`, `DescribeVpcIngressConnection`, `ListVpcIngressConnections`, `UpdateVpcIngressConnection`, and `DeleteVpcIngressConnection` also return `501 NotImplemented`.
- No VPC connector, subnet, security group, ingress connection, or PrivateLink-style state is created in Winterbaume.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListAutoScalingConfigurations`, `ListConnections`, `ListObservabilityConfigurations`, `ListOperations`, `ListServices`, `ListServicesForAutoScalingConfiguration`, `ListTagsForResource`, `ListVpcConnectors`, `ListVpcIngressConnections`
- Traits: `paginated` (8)
- Common required input members in this group: `AutoScalingConfigurationArn`, `ResourceArn`, `ServiceArn`

### Create

- Operations: `CreateAutoScalingConfiguration`, `CreateConnection`, `CreateObservabilityConfiguration`, `CreateService`, `CreateVpcConnector`, `CreateVpcIngressConnection`
- Common required input members in this group: `AutoScalingConfigurationName`, `ConnectionName`, `IngressVpcConfiguration`, `ObservabilityConfigurationName`, `ProviderType`, `ServiceArn`, `ServiceName`, `SourceConfiguration`, `Subnets`, `VpcConnectorName`, `VpcIngressConnectionName`

### Delete

- Operations: `DeleteAutoScalingConfiguration`, `DeleteConnection`, `DeleteObservabilityConfiguration`, `DeleteService`, `DeleteVpcConnector`, `DeleteVpcIngressConnection`
- Common required input members in this group: `AutoScalingConfigurationArn`, `ConnectionArn`, `ObservabilityConfigurationArn`, `ServiceArn`, `VpcConnectorArn`, `VpcIngressConnectionArn`

### Describe

- Operations: `DescribeAutoScalingConfiguration`, `DescribeCustomDomains`, `DescribeObservabilityConfiguration`, `DescribeService`, `DescribeVpcConnector`, `DescribeVpcIngressConnection`
- Traits: `paginated` (1)
- Common required input members in this group: `AutoScalingConfigurationArn`, `ObservabilityConfigurationArn`, `ServiceArn`, `VpcConnectorArn`, `VpcIngressConnectionArn`

### Update

- Operations: `UpdateDefaultAutoScalingConfiguration`, `UpdateService`, `UpdateVpcIngressConnection`
- Common required input members in this group: `AutoScalingConfigurationArn`, `IngressVpcConfiguration`, `ServiceArn`, `VpcIngressConnectionArn`

### Associate

- Operations: `AssociateCustomDomain`
- Common required input members in this group: `DomainName`, `ServiceArn`

### Disassociate

- Operations: `DisassociateCustomDomain`
- Common required input members in this group: `DomainName`, `ServiceArn`

### Pause

- Operations: `PauseService`
- Common required input members in this group: `ServiceArn`

### Resume

- Operations: `ResumeService`
- Common required input members in this group: `ServiceArn`

### Start

- Operations: `StartDeployment`
- Common required input members in this group: `ServiceArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateCustomDomain` | - | - | `DomainName`, `ServiceArn` | - | `AssociateCustomDomainResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException` | Associate your own domain name with the App Runner subdomain URL of your App Runner service. After you call `AssociateCustomDomain` and receive a successful response, use the information in the CustomDomain record that's returned to add CNAME records to your... |
| `CreateAutoScalingConfiguration` | - | - | `AutoScalingConfigurationName` | - | `CreateAutoScalingConfigurationResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ServiceQuotaExceededException` | Create an App Runner automatic scaling configuration resource. App Runner requires this resource when you create or update App Runner services and you require non-default auto scaling settings. |
| `CreateConnection` | - | - | `ConnectionName`, `ProviderType` | - | `CreateConnectionResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ServiceQuotaExceededException` | Create an App Runner connection resource. App Runner requires a connection resource when you create App Runner services that access private repositories from certain third-party providers. |
| `CreateObservabilityConfiguration` | - | - | `ObservabilityConfigurationName` | - | `CreateObservabilityConfigurationResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ServiceQuotaExceededException` | Create an App Runner observability configuration resource. App Runner requires this resource when you create or update App Runner services and you want to enable non-default observability features. |
| `CreateService` | - | - | `ServiceName`, `SourceConfiguration` | - | `CreateServiceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ServiceQuotaExceededException` | Create an App Runner service. After the service is created, the action also automatically starts a deployment. |
| `CreateVpcConnector` | - | - | `Subnets`, `VpcConnectorName` | - | `CreateVpcConnectorResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ServiceQuotaExceededException` | Create an App Runner VPC connector resource. App Runner requires this resource when you want to associate your App Runner service to a custom Amazon Virtual Private Cloud (Amazon VPC). |
| `CreateVpcIngressConnection` | - | - | `IngressVpcConfiguration`, `ServiceArn`, `VpcIngressConnectionName` | - | `CreateVpcIngressConnectionResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ServiceQuotaExceededException` | Create an App Runner VPC Ingress Connection resource. App Runner requires this resource when you want to associate your App Runner service with an Amazon VPC endpoint. |
| `DeleteAutoScalingConfiguration` | - | - | `AutoScalingConfigurationArn` | - | `DeleteAutoScalingConfigurationResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Delete an App Runner automatic scaling configuration resource. You can delete a top level auto scaling configuration, a specific revision of one, or all revisions associated with the top level configuration. |
| `DeleteConnection` | - | - | `ConnectionArn` | - | `DeleteConnectionResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Delete an App Runner connection. You must first ensure that there are no running App Runner services that use this connection. |
| `DeleteObservabilityConfiguration` | - | - | `ObservabilityConfigurationArn` | - | `DeleteObservabilityConfigurationResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Delete an App Runner observability configuration resource. You can delete a specific revision or the latest active revision. |
| `DeleteService` | - | - | `ServiceArn` | - | `DeleteServiceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | Delete an App Runner service. This is an asynchronous operation. |
| `DeleteVpcConnector` | - | - | `VpcConnectorArn` | - | `DeleteVpcConnectorResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Delete an App Runner VPC connector resource. You can't delete a connector that's used by one or more App Runner services. |
| `DeleteVpcIngressConnection` | - | - | `VpcIngressConnectionArn` | - | `DeleteVpcIngressConnectionResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | Delete an App Runner VPC Ingress Connection resource that's associated with an App Runner service. The VPC Ingress Connection must be in one of the following states to be deleted: `AVAILABLE` `FAILED_CREATION` `FAILED_UPDATE` `FAILED_DELETION` |
| `DescribeAutoScalingConfiguration` | - | - | `AutoScalingConfigurationArn` | - | `DescribeAutoScalingConfigurationResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Return a full description of an App Runner automatic scaling configuration resource. |
| `DescribeCustomDomains` | - | `paginated` | `ServiceArn` | - | `DescribeCustomDomainsResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Return a description of custom domain names that are associated with an App Runner service. |
| `DescribeObservabilityConfiguration` | - | - | `ObservabilityConfigurationArn` | - | `DescribeObservabilityConfigurationResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Return a full description of an App Runner observability configuration resource. |
| `DescribeService` | - | - | `ServiceArn` | - | `DescribeServiceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Return a full description of an App Runner service. |
| `DescribeVpcConnector` | - | - | `VpcConnectorArn` | - | `DescribeVpcConnectorResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Return a description of an App Runner VPC connector resource. |
| `DescribeVpcIngressConnection` | - | - | `VpcIngressConnectionArn` | - | `DescribeVpcIngressConnectionResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Return a full description of an App Runner VPC Ingress Connection resource. |
| `DisassociateCustomDomain` | - | - | `DomainName`, `ServiceArn` | - | `DisassociateCustomDomainResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | Disassociate a custom domain name from an App Runner service. Certificates tracking domain validity are associated with a custom domain and are stored in AWS Certificate Manager (ACM). |
| `ListAutoScalingConfigurations` | - | `paginated` | - | - | `ListAutoScalingConfigurationsResponse` | `InternalServiceErrorException`, `InvalidRequestException` | Returns a list of active App Runner automatic scaling configurations in your Amazon Web Services account. You can query the revisions for a specific configuration name or the revisions for all active configurations in your account. |
| `ListConnections` | - | `paginated` | - | - | `ListConnectionsResponse` | `InternalServiceErrorException`, `InvalidRequestException` | Returns a list of App Runner connections that are associated with your Amazon Web Services account. |
| `ListObservabilityConfigurations` | - | `paginated` | - | - | `ListObservabilityConfigurationsResponse` | `InternalServiceErrorException`, `InvalidRequestException` | Returns a list of active App Runner observability configurations in your Amazon Web Services account. You can query the revisions for a specific configuration name or the revisions for all active configurations in your account. |
| `ListOperations` | - | `paginated` | `ServiceArn` | - | `ListOperationsResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Return a list of operations that occurred on an App Runner service. The resulting list of OperationSummary objects is sorted in reverse chronological order. |
| `ListServices` | - | `paginated` | - | - | `ListServicesResponse` | `InternalServiceErrorException`, `InvalidRequestException` | Returns a list of running App Runner services in your Amazon Web Services account. |
| `ListServicesForAutoScalingConfiguration` | - | `paginated` | `AutoScalingConfigurationArn` | - | `ListServicesForAutoScalingConfigurationResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Returns a list of the associated App Runner services using an auto scaling configuration. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | List tags that are associated with for an App Runner resource. The response contains a list of tag key-value pairs. |
| `ListVpcConnectors` | - | `paginated` | - | - | `ListVpcConnectorsResponse` | `InternalServiceErrorException`, `InvalidRequestException` | Returns a list of App Runner VPC connectors in your Amazon Web Services account. |
| `ListVpcIngressConnections` | - | `paginated` | - | - | `ListVpcIngressConnectionsResponse` | `InternalServiceErrorException`, `InvalidRequestException` | Return a list of App Runner VPC Ingress Connections in your Amazon Web Services account. |
| `PauseService` | - | - | `ServiceArn` | - | `PauseServiceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | Pause an active App Runner service. App Runner reduces compute capacity for the service to zero and loses state (for example, ephemeral storage is removed). |
| `ResumeService` | - | - | `ServiceArn` | - | `ResumeServiceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | Resume an active App Runner service. App Runner provisions compute capacity for the service. |
| `StartDeployment` | - | - | `ServiceArn` | - | `StartDeploymentResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Initiate a manual deployment of the latest commit in a source code repository or the latest image in a source image repository to an App Runner service. For a source code repository, App Runner retrieves the commit and builds a Docker image. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | Add tags to, or update the tag values of, an App Runner resource. A tag is a key-value pair. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | Remove tags from an App Runner resource. |
| `UpdateDefaultAutoScalingConfiguration` | - | - | `AutoScalingConfigurationArn` | - | `UpdateDefaultAutoScalingConfigurationResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException` | Update an auto scaling configuration to be the default. The existing default auto scaling configuration will be set to non-default automatically. |
| `UpdateService` | - | - | `ServiceArn` | - | `UpdateServiceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | Update an App Runner service. You can update the source configuration and instance configuration of the service. |
| `UpdateVpcIngressConnection` | - | - | `IngressVpcConfiguration`, `VpcIngressConnectionArn` | - | `UpdateVpcIngressConnectionResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `InvalidStateException`, `ResourceNotFoundException` | Update an existing App Runner VPC Ingress Connection resource. The VPC Ingress Connection must be in one of the following states to be updated: AVAILABLE FAILED_CREATION FAILED_UPDATE |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceErrorException` | `structure` | `Message` | An unexpected service exception occurred. |
| `InvalidRequestException` | `structure` | `Message` | One or more input parameters aren't valid. |
| `ResourceNotFoundException` | `structure` | `Message` | A resource doesn't exist for the specified Amazon Resource Name (ARN) in your Amazon Web Services account. |
| `InvalidStateException` | `structure` | `Message` | You can't perform this action when the resource is in its current state. |
| `ServiceQuotaExceededException` | `structure` | `Message` | App Runner can't create this resource. |
| `AssociateCustomDomainRequest` | `structure` | `DomainName`, `EnableWWWSubdomain`, `ServiceArn` | - |
| `AssociateCustomDomainResponse` | `structure` | `CustomDomain`, `DNSTarget`, `ServiceArn`, `VpcDNSTargets` | - |
| `CreateAutoScalingConfigurationRequest` | `structure` | `AutoScalingConfigurationName`, `MaxConcurrency`, `MaxSize`, `MinSize`, `Tags` | - |
| `CreateAutoScalingConfigurationResponse` | `structure` | `AutoScalingConfiguration` | - |
| `CreateConnectionRequest` | `structure` | `ConnectionName`, `ProviderType`, `Tags` | - |
| `CreateConnectionResponse` | `structure` | `Connection` | - |
| `CreateObservabilityConfigurationRequest` | `structure` | `ObservabilityConfigurationName`, `Tags`, `TraceConfiguration` | - |
| `CreateObservabilityConfigurationResponse` | `structure` | `ObservabilityConfiguration` | - |
| `CreateServiceRequest` | `structure` | `AutoScalingConfigurationArn`, `EncryptionConfiguration`, `HealthCheckConfiguration`, `InstanceConfiguration`, `NetworkConfiguration`, `ObservabilityConfiguration`, `ServiceName`, `SourceConfiguration`, `Tags` | - |
| `CreateServiceResponse` | `structure` | `OperationId`, `Service` | - |
| `CreateVpcConnectorRequest` | `structure` | `SecurityGroups`, `Subnets`, `Tags`, `VpcConnectorName` | - |
| `CreateVpcConnectorResponse` | `structure` | `VpcConnector` | - |
| `CreateVpcIngressConnectionRequest` | `structure` | `IngressVpcConfiguration`, `ServiceArn`, `Tags`, `VpcIngressConnectionName` | - |
| `CreateVpcIngressConnectionResponse` | `structure` | `VpcIngressConnection` | - |
| `DeleteAutoScalingConfigurationRequest` | `structure` | `AutoScalingConfigurationArn`, `DeleteAllRevisions` | - |
| `DeleteAutoScalingConfigurationResponse` | `structure` | `AutoScalingConfiguration` | - |
| `DeleteConnectionRequest` | `structure` | `ConnectionArn` | - |
| `DeleteConnectionResponse` | `structure` | `Connection` | - |
| `DeleteObservabilityConfigurationRequest` | `structure` | `ObservabilityConfigurationArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
