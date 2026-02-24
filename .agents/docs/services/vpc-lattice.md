# Amazon VPC Lattice

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon VPC Lattice is a fully managed application networking service that you use to connect, secure, and monitor all of your services across multiple accounts and virtual private clouds (VPCs). Amazon VPC Lattice interconnects your microservices and legacy services within a logical boundary, so that you can discover and manage them more efficiently. For more information, see the Amazon VPC Lattice User Guide

## Possible Usage Scenarios
- Backported from `crates/winterbaume-vpclattice/tests/scenario_test.rs`: build an end-to-end service mesh with service network, service, listener, target group, targets, rules, associations, and cleanup.
- Backported from `scenario_test.rs`: switch a listener default action to a fixed-response maintenance mode.
- Scenario insight from EC2: include mutable binding failover for Amazon VPC Lattice where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon VPC Lattice by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent application-networking service meshes, service networks, listeners, rules, target groups, auth policies, resource associations, VPC associations, tags, and traffic routing updates.

## Service Identity and Protocol

- AWS model slug: `vpc-lattice`
- AWS SDK for Rust slug: `vpclattice`
- Model version: `2022-11-30`
- Model file: `vendor/api-models-aws/models/vpc-lattice/service/2022-11-30/vpc-lattice-2022-11-30.json`
- SDK ID: `VPC Lattice`
- Endpoint prefix: `-`
- ARN namespace: `vpc-lattice`
- CloudFormation name: `VpcLattice`
- CloudTrail event source: `vpc-lattice.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (16), `Delete` (15), `Get` (14), `Create` (11), `Update` (9), `Put` (2), `Batch` (1), `Deregister` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchUpdateRule`, `CreateAccessLogSubscription`, `CreateListener`, `CreateResourceConfiguration`, `CreateResourceGateway`, `CreateRule`, `CreateService`, `CreateServiceNetwork`, `CreateServiceNetworkResourceAssociation`, `CreateServiceNetworkServiceAssociation`, `CreateServiceNetworkVpcAssociation`, `CreateTargetGroup`, `DeleteAccessLogSubscription`, `DeleteAuthPolicy`, `DeleteDomainVerification`, `DeleteListener`, `DeleteResourceConfiguration`, `DeleteResourceEndpointAssociation`, `DeleteResourceGateway`, `DeleteResourcePolicy`, ... (+23).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccessLogSubscription`, `GetAuthPolicy`, `GetDomainVerification`, `GetListener`, `GetResourceConfiguration`, `GetResourceGateway`, `GetResourcePolicy`, `GetRule`, `GetService`, `GetServiceNetwork`, `GetServiceNetworkResourceAssociation`, `GetServiceNetworkServiceAssociation`, `GetServiceNetworkVpcAssociation`, `GetTargetGroup`, `ListAccessLogSubscriptions`, `ListDomainVerifications`, `ListListeners`, `ListResourceConfigurations`, `ListResourceEndpointAssociations`, `ListResourceGateways`, ... (+10).
- Pagination is modelled for 15 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 37 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDomainVerification`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 73 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `Lambda`, `EC2/VPC`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccessLogSubscription` | `accessLogSubscriptionIdentifier` | create: `CreateAccessLogSubscription`; read: `GetAccessLogSubscription`; update: `UpdateAccessLogSubscription`; delete: `DeleteAccessLogSubscription`; list: `ListAccessLogSubscriptions` | - | - |
| `DomainVerification` | `domainVerificationIdentifier` | create: `StartDomainVerification`; read: `GetDomainVerification`; delete: `DeleteDomainVerification`; list: `ListDomainVerifications` | - | - |
| `Listener` | `listenerIdentifier`, `serviceIdentifier` | create: `CreateListener`; read: `GetListener`; update: `UpdateListener`; delete: `DeleteListener`; list: `ListListeners` | - | - |
| `ResourceConfiguration` | `resourceConfigurationIdentifier` | create: `CreateResourceConfiguration`; read: `GetResourceConfiguration`; update: `UpdateResourceConfiguration`; delete: `DeleteResourceConfiguration`; list: `ListResourceConfigurations` | - | - |
| `ResourceEndpointAssociation` | `resourceEndpointAssociationIdentifier` | delete: `DeleteResourceEndpointAssociation`; list: `ListResourceEndpointAssociations` | - | - |
| `ResourceGateway` | `resourceGatewayIdentifier` | create: `CreateResourceGateway`; read: `GetResourceGateway`; update: `UpdateResourceGateway`; delete: `DeleteResourceGateway`; list: `ListResourceGateways` | - | - |
| `Rule` | `listenerIdentifier`, `ruleIdentifier`, `serviceIdentifier` | create: `CreateRule`; read: `GetRule`; update: `UpdateRule`; delete: `DeleteRule`; list: `ListRules` | - | - |
| `Service` | `serviceIdentifier` | create: `CreateService`; read: `GetService`; update: `UpdateService`; delete: `DeleteService`; list: `ListServices` | - | - |
| `ServiceLoadBalancerAssociation` | `serviceLoadBalancerAssociationIdentifier` | - | - | - |
| `ServiceNetwork` | `serviceNetworkIdentifier` | create: `CreateServiceNetwork`; read: `GetServiceNetwork`; update: `UpdateServiceNetwork`; delete: `DeleteServiceNetwork`; list: `ListServiceNetworks` | - | - |
| `ServiceNetworkResourceAssociation` | `serviceNetworkResourceAssociationIdentifier` | create: `CreateServiceNetworkResourceAssociation`; read: `GetServiceNetworkResourceAssociation`; delete: `DeleteServiceNetworkResourceAssociation`; list: `ListServiceNetworkResourceAssociations` | - | - |
| `ServiceNetworkServiceAssociation` | `serviceNetworkServiceAssociationIdentifier` | create: `CreateServiceNetworkServiceAssociation`; read: `GetServiceNetworkServiceAssociation`; delete: `DeleteServiceNetworkServiceAssociation`; list: `ListServiceNetworkServiceAssociations` | - | - |
| `ServiceNetworkVpcAssociation` | `serviceNetworkVpcAssociationIdentifier` | create: `CreateServiceNetworkVpcAssociation`; read: `GetServiceNetworkVpcAssociation`; update: `UpdateServiceNetworkVpcAssociation`; delete: `DeleteServiceNetworkVpcAssociation`; list: `ListServiceNetworkVpcAssociations` | - | - |
| `TargetGroup` | `targetGroupIdentifier` | create: `CreateTargetGroup`; read: `GetTargetGroup`; update: `UpdateTargetGroup`; delete: `DeleteTargetGroup`; list: `ListTargetGroups` | `DeregisterTargets`, `ListTargets`, `RegisterTargets` | - |

## Current Network Resource Stub Semantics

VPC Lattice currently stores service-network VPC associations and resource gateway networking inside VPC Lattice state.

- Service network VPC associations store service network ID, VPC ID, and security group IDs, and update calls replace the stored security group list.
- Resource gateways store VPC ID, subnet IDs, and security group IDs as supplied by the request.
- Resource endpoint association and service network VPC endpoint association handlers currently return placeholder or empty responses rather than backing full endpoint state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListAccessLogSubscriptions`, `ListDomainVerifications`, `ListListeners`, `ListResourceConfigurations`, `ListResourceEndpointAssociations`, `ListResourceGateways`, `ListRules`, `ListServiceNetworkResourceAssociations`, `ListServiceNetworkServiceAssociations`, `ListServiceNetworkVpcAssociations`, `ListServiceNetworkVpcEndpointAssociations`, `ListServiceNetworks`, `ListServices`, `ListTagsForResource`, `ListTargetGroups`, `ListTargets`
- Traits: `paginated` (15), `readonly` (16)
- Common required input members in this group: `listenerIdentifier`, `resourceArn`, `resourceConfigurationIdentifier`, `resourceIdentifier`, `serviceIdentifier`, `serviceNetworkIdentifier`, `targetGroupIdentifier`

### Delete

- Operations: `DeleteAccessLogSubscription`, `DeleteAuthPolicy`, `DeleteDomainVerification`, `DeleteListener`, `DeleteResourceConfiguration`, `DeleteResourceEndpointAssociation`, `DeleteResourceGateway`, `DeleteResourcePolicy`, `DeleteRule`, `DeleteService`, `DeleteServiceNetwork`, `DeleteServiceNetworkResourceAssociation`, `DeleteServiceNetworkServiceAssociation`, `DeleteServiceNetworkVpcAssociation`, `DeleteTargetGroup`
- Traits: `idempotent` (15)
- Common required input members in this group: `accessLogSubscriptionIdentifier`, `domainVerificationIdentifier`, `listenerIdentifier`, `resourceArn`, `resourceConfigurationIdentifier`, `resourceEndpointAssociationIdentifier`, `resourceGatewayIdentifier`, `resourceIdentifier`, `ruleIdentifier`, `serviceIdentifier`, `serviceNetworkIdentifier`, `serviceNetworkResourceAssociationIdentifier`, `serviceNetworkServiceAssociationIdentifier`, `serviceNetworkVpcAssociationIdentifier`, `targetGroupIdentifier`

### Get

- Operations: `GetAccessLogSubscription`, `GetAuthPolicy`, `GetDomainVerification`, `GetListener`, `GetResourceConfiguration`, `GetResourceGateway`, `GetResourcePolicy`, `GetRule`, `GetService`, `GetServiceNetwork`, `GetServiceNetworkResourceAssociation`, `GetServiceNetworkServiceAssociation`, `GetServiceNetworkVpcAssociation`, `GetTargetGroup`
- Traits: `readonly` (14)
- Common required input members in this group: `accessLogSubscriptionIdentifier`, `domainVerificationIdentifier`, `listenerIdentifier`, `resourceArn`, `resourceConfigurationIdentifier`, `resourceGatewayIdentifier`, `resourceIdentifier`, `ruleIdentifier`, `serviceIdentifier`, `serviceNetworkIdentifier`, `serviceNetworkResourceAssociationIdentifier`, `serviceNetworkServiceAssociationIdentifier`, `serviceNetworkVpcAssociationIdentifier`, `targetGroupIdentifier`

### Create

- Operations: `CreateAccessLogSubscription`, `CreateListener`, `CreateResourceConfiguration`, `CreateResourceGateway`, `CreateRule`, `CreateService`, `CreateServiceNetwork`, `CreateServiceNetworkResourceAssociation`, `CreateServiceNetworkServiceAssociation`, `CreateServiceNetworkVpcAssociation`, `CreateTargetGroup`
- Traits: `idempotency-token` (11), `idempotent` (11)
- Common required input members in this group: `action`, `defaultAction`, `destinationArn`, `listenerIdentifier`, `match`, `name`, `priority`, `protocol`, `resourceConfigurationIdentifier`, `resourceIdentifier`, `serviceIdentifier`, `serviceNetworkIdentifier`, `type`, `vpcIdentifier`

### Update

- Operations: `UpdateAccessLogSubscription`, `UpdateListener`, `UpdateResourceConfiguration`, `UpdateResourceGateway`, `UpdateRule`, `UpdateService`, `UpdateServiceNetwork`, `UpdateServiceNetworkVpcAssociation`, `UpdateTargetGroup`
- Traits: `idempotent` (6)
- Common required input members in this group: `accessLogSubscriptionIdentifier`, `authType`, `defaultAction`, `destinationArn`, `healthCheck`, `listenerIdentifier`, `resourceConfigurationIdentifier`, `resourceGatewayIdentifier`, `ruleIdentifier`, `securityGroupIds`, `serviceIdentifier`, `serviceNetworkIdentifier`, `serviceNetworkVpcAssociationIdentifier`, `targetGroupIdentifier`

### Put

- Operations: `PutAuthPolicy`, `PutResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: `policy`, `resourceArn`, `resourceIdentifier`

### Batch

- Operations: `BatchUpdateRule`
- Traits: `idempotent` (1)
- Common required input members in this group: `listenerIdentifier`, `rules`, `serviceIdentifier`

### Deregister

- Operations: `DeregisterTargets`
- Traits: `idempotent` (1)
- Common required input members in this group: `targetGroupIdentifier`, `targets`

### Register

- Operations: `RegisterTargets`
- Common required input members in this group: `targetGroupIdentifier`, `targets`

### Start

- Operations: `StartDomainVerification`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `domainName`

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
| `BatchUpdateRule` | `PATCH /services/{serviceIdentifier}/listeners/{listenerIdentifier}/rules` | `idempotent` | `listenerIdentifier`, `rules`, `serviceIdentifier` | - | `BatchUpdateRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the listener rules in a batch. You can use this operation to change the priority of listener rules. |
| `CreateAccessLogSubscription` | `POST /accesslogsubscriptions` | `idempotent`, `idempotency-token` | `destinationArn`, `resourceIdentifier` | `clientToken` | `CreateAccessLogSubscriptionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables access logs to be sent to Amazon CloudWatch, Amazon S3, and Amazon Kinesis Data Firehose. The service network owner can use the access logs to audit the services in the network. |
| `CreateListener` | `POST /services/{serviceIdentifier}/listeners` | `idempotent`, `idempotency-token` | `defaultAction`, `name`, `protocol`, `serviceIdentifier` | `clientToken` | `CreateListenerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a listener for a service. Before you start using your Amazon VPC Lattice service, you must add one or more listeners. |
| `CreateResourceConfiguration` | `POST /resourceconfigurations` | `idempotent`, `idempotency-token` | `name`, `type` | `clientToken` | `CreateResourceConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a resource configuration. A resource configuration defines a specific resource. |
| `CreateResourceGateway` | `POST /resourcegateways` | `idempotent`, `idempotency-token` | `name` | `clientToken` | `CreateResourceGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | A resource gateway is a point of ingress into the VPC where a resource resides. It spans multiple Availability Zones. |
| `CreateRule` | `POST /services/{serviceIdentifier}/listeners/{listenerIdentifier}/rules` | `idempotent`, `idempotency-token` | `action`, `listenerIdentifier`, `match`, `name`, `priority`, `serviceIdentifier` | `clientToken` | `CreateRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a listener rule. Each listener has a default rule for checking connection requests, but you can define additional rules. |
| `CreateService` | `POST /services` | `idempotent`, `idempotency-token` | `name` | `clientToken` | `CreateServiceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a service. A service is any software application that can run on instances containers, or serverless functions within an account or virtual private cloud (VPC). |
| `CreateServiceNetwork` | `POST /servicenetworks` | `idempotent`, `idempotency-token` | `name` | `clientToken` | `CreateServiceNetworkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a service network. A service network is a logical boundary for a collection of services. |
| `CreateServiceNetworkResourceAssociation` | `POST /servicenetworkresourceassociations` | `idempotent`, `idempotency-token` | `resourceConfigurationIdentifier`, `serviceNetworkIdentifier` | `clientToken` | `CreateServiceNetworkResourceAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates the specified service network with the specified resource configuration. This allows the resource configuration to receive connections through the service network, including through a service network VPC endpoint. |
| `CreateServiceNetworkServiceAssociation` | `POST /servicenetworkserviceassociations` | `idempotent`, `idempotency-token` | `serviceIdentifier`, `serviceNetworkIdentifier` | `clientToken` | `CreateServiceNetworkServiceAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates the specified service with the specified service network. For more information, see Manage service associations in the Amazon VPC Lattice User Guide . |
| `CreateServiceNetworkVpcAssociation` | `POST /servicenetworkvpcassociations` | `idempotent`, `idempotency-token` | `serviceNetworkIdentifier`, `vpcIdentifier` | `clientToken` | `CreateServiceNetworkVpcAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a VPC with a service network. When you associate a VPC with the service network, it enables all the resources within that VPC to be clients and communicate with other services in the service network. |
| `CreateTargetGroup` | `POST /targetgroups` | `idempotent`, `idempotency-token` | `name`, `type` | `clientToken` | `CreateTargetGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a target group. A target group is a collection of targets, or compute resources, that run your application or service. |
| `DeleteAccessLogSubscription` | `DELETE /accesslogsubscriptions/{accessLogSubscriptionIdentifier}` | `idempotent` | `accessLogSubscriptionIdentifier` | - | `DeleteAccessLogSubscriptionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified access log subscription. |
| `DeleteAuthPolicy` | `DELETE /authpolicy/{resourceIdentifier}` | `idempotent` | `resourceIdentifier` | - | `DeleteAuthPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified auth policy. If an auth is set to `AWS_IAM` and the auth policy is deleted, all requests are denied. |
| `DeleteDomainVerification` | `DELETE /domainverifications/{domainVerificationIdentifier}` | `idempotent` | `domainVerificationIdentifier` | - | `DeleteDomainVerificationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified domain verification. |
| `DeleteListener` | `DELETE /services/{serviceIdentifier}/listeners/{listenerIdentifier}` | `idempotent` | `listenerIdentifier`, `serviceIdentifier` | - | `DeleteListenerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified listener. |
| `DeleteResourceConfiguration` | `DELETE /resourceconfigurations/{resourceConfigurationIdentifier}` | `idempotent` | `resourceConfigurationIdentifier` | - | `DeleteResourceConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified resource configuration. |
| `DeleteResourceEndpointAssociation` | `DELETE /resourceendpointassociations/{resourceEndpointAssociationIdentifier}` | `idempotent` | `resourceEndpointAssociationIdentifier` | - | `DeleteResourceEndpointAssociationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates the resource configuration from the resource VPC endpoint. |
| `DeleteResourceGateway` | `DELETE /resourcegateways/{resourceGatewayIdentifier}` | `idempotent` | `resourceGatewayIdentifier` | - | `DeleteResourceGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified resource gateway. |
| `DeleteResourcePolicy` | `DELETE /resourcepolicy/{resourceArn}` | `idempotent` | `resourceArn` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified resource policy. |
| `DeleteRule` | `DELETE /services/{serviceIdentifier}/listeners/{listenerIdentifier}/rules/{ruleIdentifier}` | `idempotent` | `listenerIdentifier`, `ruleIdentifier`, `serviceIdentifier` | - | `DeleteRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a listener rule. Each listener has a default rule for checking connection requests, but you can define additional rules. |
| `DeleteService` | `DELETE /services/{serviceIdentifier}` | `idempotent` | `serviceIdentifier` | - | `DeleteServiceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a service. A service can't be deleted if it's associated with a service network. |
| `DeleteServiceNetwork` | `DELETE /servicenetworks/{serviceNetworkIdentifier}` | `idempotent` | `serviceNetworkIdentifier` | - | `DeleteServiceNetworkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a service network. You can only delete the service network if there is no service or VPC associated with it. |
| `DeleteServiceNetworkResourceAssociation` | `DELETE /servicenetworkresourceassociations/{serviceNetworkResourceAssociationIdentifier}` | `idempotent` | `serviceNetworkResourceAssociationIdentifier` | - | `DeleteServiceNetworkResourceAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the association between a service network and a resource configuration. |
| `DeleteServiceNetworkServiceAssociation` | `DELETE /servicenetworkserviceassociations/{serviceNetworkServiceAssociationIdentifier}` | `idempotent` | `serviceNetworkServiceAssociationIdentifier` | - | `DeleteServiceNetworkServiceAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the association between a service and a service network. This operation fails if an association is still in progress. |
| `DeleteServiceNetworkVpcAssociation` | `DELETE /servicenetworkvpcassociations/{serviceNetworkVpcAssociationIdentifier}` | `idempotent` | `serviceNetworkVpcAssociationIdentifier` | - | `DeleteServiceNetworkVpcAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates the VPC from the service network. You can't disassociate the VPC if there is a create or update association in progress. |
| `DeleteTargetGroup` | `DELETE /targetgroups/{targetGroupIdentifier}` | `idempotent` | `targetGroupIdentifier` | - | `DeleteTargetGroupResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a target group. You can't delete a target group if it is used in a listener rule or if the target group creation is in progress. |
| `DeregisterTargets` | `POST /targetgroups/{targetGroupIdentifier}/deregistertargets` | `idempotent` | `targetGroupIdentifier`, `targets` | - | `DeregisterTargetsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deregisters the specified targets from the specified target group. |
| `GetAccessLogSubscription` | `GET /accesslogsubscriptions/{accessLogSubscriptionIdentifier}` | `readonly` | `accessLogSubscriptionIdentifier` | - | `GetAccessLogSubscriptionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified access log subscription. |
| `GetAuthPolicy` | `GET /authpolicy/{resourceIdentifier}` | `readonly` | `resourceIdentifier` | - | `GetAuthPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the auth policy for the specified service or service network. |
| `GetDomainVerification` | `GET /domainverifications/{domainVerificationIdentifier}` | `readonly` | `domainVerificationIdentifier` | - | `GetDomainVerificationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a domain verification.ß |
| `GetListener` | `GET /services/{serviceIdentifier}/listeners/{listenerIdentifier}` | `readonly` | `listenerIdentifier`, `serviceIdentifier` | - | `GetListenerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified listener for the specified service. |
| `GetResourceConfiguration` | `GET /resourceconfigurations/{resourceConfigurationIdentifier}` | `readonly` | `resourceConfigurationIdentifier` | - | `GetResourceConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified resource configuration. |
| `GetResourceGateway` | `GET /resourcegateways/{resourceGatewayIdentifier}` | `readonly` | `resourceGatewayIdentifier` | - | `GetResourceGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified resource gateway. |
| `GetResourcePolicy` | `GET /resourcepolicy/{resourceArn}` | `readonly` | `resourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified resource policy. The resource policy is an IAM policy created on behalf of the resource owner when they share a resource. |
| `GetRule` | `GET /services/{serviceIdentifier}/listeners/{listenerIdentifier}/rules/{ruleIdentifier}` | `readonly` | `listenerIdentifier`, `ruleIdentifier`, `serviceIdentifier` | - | `GetRuleResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified listener rules. You can also retrieve information about the default listener rule. |
| `GetService` | `GET /services/{serviceIdentifier}` | `readonly` | `serviceIdentifier` | - | `GetServiceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified service. |
| `GetServiceNetwork` | `GET /servicenetworks/{serviceNetworkIdentifier}` | `readonly` | `serviceNetworkIdentifier` | - | `GetServiceNetworkResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified service network. |
| `GetServiceNetworkResourceAssociation` | `GET /servicenetworkresourceassociations/{serviceNetworkResourceAssociationIdentifier}` | `readonly` | `serviceNetworkResourceAssociationIdentifier` | - | `GetServiceNetworkResourceAssociationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified association between a service network and a resource configuration. |
| `GetServiceNetworkServiceAssociation` | `GET /servicenetworkserviceassociations/{serviceNetworkServiceAssociationIdentifier}` | `readonly` | `serviceNetworkServiceAssociationIdentifier` | - | `GetServiceNetworkServiceAssociationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified association between a service network and a service. |
| `GetServiceNetworkVpcAssociation` | `GET /servicenetworkvpcassociations/{serviceNetworkVpcAssociationIdentifier}` | `readonly` | `serviceNetworkVpcAssociationIdentifier` | - | `GetServiceNetworkVpcAssociationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified association between a service network and a VPC. |
| `GetTargetGroup` | `GET /targetgroups/{targetGroupIdentifier}` | `readonly` | `targetGroupIdentifier` | - | `GetTargetGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified target group. |
| `ListAccessLogSubscriptions` | `GET /accesslogsubscriptions` | `readonly`, `paginated` | `resourceIdentifier` | - | `ListAccessLogSubscriptionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the access log subscriptions for the specified service network or service. |
| `ListDomainVerifications` | `GET /domainverifications` | `readonly`, `paginated` | - | - | `ListDomainVerificationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the domain verifications. |
| `ListListeners` | `GET /services/{serviceIdentifier}/listeners` | `readonly`, `paginated` | `serviceIdentifier` | - | `ListListenersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the listeners for the specified service. |
| `ListResourceConfigurations` | `GET /resourceconfigurations` | `readonly`, `paginated` | - | - | `ListResourceConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the resource configurations owned by or shared with this account. |
| `ListResourceEndpointAssociations` | `GET /resourceendpointassociations` | `readonly`, `paginated` | `resourceConfigurationIdentifier` | - | `ListResourceEndpointAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the associations for the specified VPC endpoint. |
| `ListResourceGateways` | `GET /resourcegateways` | `readonly`, `paginated` | - | - | `ListResourceGatewaysResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the resource gateways that you own or that were shared with you. |
| `ListRules` | `GET /services/{serviceIdentifier}/listeners/{listenerIdentifier}/rules` | `readonly`, `paginated` | `listenerIdentifier`, `serviceIdentifier` | - | `ListRulesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the rules for the specified listener. |
| `ListServiceNetworkResourceAssociations` | `GET /servicenetworkresourceassociations` | `readonly`, `paginated` | - | - | `ListServiceNetworkResourceAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the associations between a service network and a resource configuration. |
| `ListServiceNetworkServiceAssociations` | `GET /servicenetworkserviceassociations` | `readonly`, `paginated` | - | - | `ListServiceNetworkServiceAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the associations between a service network and a service. You can filter the list either by service or service network. |
| `ListServiceNetworkVpcAssociations` | `GET /servicenetworkvpcassociations` | `readonly`, `paginated` | - | - | `ListServiceNetworkVpcAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the associations between a service network and a VPC. You can filter the list either by VPC or service network. |
| `ListServiceNetworkVpcEndpointAssociations` | `GET /servicenetworkvpcendpointassociations` | `readonly`, `paginated` | `serviceNetworkIdentifier` | - | `ListServiceNetworkVpcEndpointAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the associations between a service network and a VPC endpoint. |
| `ListServiceNetworks` | `GET /servicenetworks` | `readonly`, `paginated` | - | - | `ListServiceNetworksResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the service networks owned by or shared with this account. The account ID in the ARN shows which account owns the service network. |
| `ListServices` | `GET /services` | `readonly`, `paginated` | - | - | `ListServicesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the services owned by the caller account or shared with the caller account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags for the specified resource. |
| `ListTargetGroups` | `GET /targetgroups` | `readonly`, `paginated` | - | - | `ListTargetGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists your target groups. You can narrow your search by using the filters below in your request. |
| `ListTargets` | `POST /targetgroups/{targetGroupIdentifier}/listtargets` | `readonly`, `paginated` | `targetGroupIdentifier` | - | `ListTargetsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the targets for the target group. By default, all targets are included. |
| `PutAuthPolicy` | `PUT /authpolicy/{resourceIdentifier}` | - | `policy`, `resourceIdentifier` | - | `PutAuthPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates the auth policy. The policy string in JSON must not contain newlines or blank lines. |
| `PutResourcePolicy` | `PUT /resourcepolicy/{resourceArn}` | `idempotent` | `policy`, `resourceArn` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches a resource-based permission policy to a service or service network. The policy must contain the same actions and condition statements as the Amazon Web Services Resource Access Manager permission for sharing services and service networks. |
| `RegisterTargets` | `POST /targetgroups/{targetGroupIdentifier}/registertargets` | - | `targetGroupIdentifier`, `targets` | - | `RegisterTargetsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Registers the targets with the target group. If it's a Lambda target, you can only have one target in a target group. |
| `StartDomainVerification` | `POST /domainverifications` | `idempotent`, `idempotency-token` | `domainName` | `clientToken` | `StartDomainVerificationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts the domain verification process for a custom domain name. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds the specified tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified tags from the specified resource. |
| `UpdateAccessLogSubscription` | `PATCH /accesslogsubscriptions/{accessLogSubscriptionIdentifier}` | `idempotent` | `accessLogSubscriptionIdentifier`, `destinationArn` | - | `UpdateAccessLogSubscriptionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified access log subscription. |
| `UpdateListener` | `PATCH /services/{serviceIdentifier}/listeners/{listenerIdentifier}` | `idempotent` | `defaultAction`, `listenerIdentifier`, `serviceIdentifier` | - | `UpdateListenerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the specified listener for the specified service. |
| `UpdateResourceConfiguration` | `PATCH /resourceconfigurations/{resourceConfigurationIdentifier}` | - | `resourceConfigurationIdentifier` | - | `UpdateResourceConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the specified resource configuration. |
| `UpdateResourceGateway` | `PATCH /resourcegateways/{resourceGatewayIdentifier}` | - | `resourceGatewayIdentifier` | - | `UpdateResourceGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified resource gateway. |
| `UpdateRule` | `PATCH /services/{serviceIdentifier}/listeners/{listenerIdentifier}/rules/{ruleIdentifier}` | `idempotent` | `listenerIdentifier`, `ruleIdentifier`, `serviceIdentifier` | - | `UpdateRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a specified rule for the listener. You can't modify a default listener rule. |
| `UpdateService` | `PATCH /services/{serviceIdentifier}` | - | `serviceIdentifier` | - | `UpdateServiceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the specified service. |
| `UpdateServiceNetwork` | `PATCH /servicenetworks/{serviceNetworkIdentifier}` | `idempotent` | `authType`, `serviceNetworkIdentifier` | - | `UpdateServiceNetworkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified service network. |
| `UpdateServiceNetworkVpcAssociation` | `PATCH /servicenetworkvpcassociations/{serviceNetworkVpcAssociationIdentifier}` | `idempotent` | `securityGroupIds`, `serviceNetworkVpcAssociationIdentifier` | - | `UpdateServiceNetworkVpcAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the service network and VPC association. If you add a security group to the service network and VPC association, the association must continue to have at least one security group. |
| `UpdateTargetGroup` | `PATCH /targetgroups/{targetGroupIdentifier}` | `idempotent` | `healthCheck`, `targetGroupIdentifier` | - | `UpdateTargetGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the specified target group. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | An unexpected error occurred while processing the request. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input does not satisfy the constraints specified by an Amazon Web Services service. |
| `AccessDeniedException` | `structure` | `message` | The user does not have sufficient access to perform this action. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The limit on the number of requests per second was exceeded. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The request references a resource that does not exist. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The request conflicts with the current state of the resource. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The request would cause a service quota to be exceeded. |
| `BatchUpdateRuleRequest` | `structure` | `listenerIdentifier`, `rules`, `serviceIdentifier` | - |
| `BatchUpdateRuleResponse` | `structure` | `successful`, `unsuccessful` | - |
| `CreateAccessLogSubscriptionRequest` | `structure` | `clientToken`, `destinationArn`, `resourceIdentifier`, `serviceNetworkLogType`, `tags` | - |
| `CreateAccessLogSubscriptionResponse` | `structure` | `arn`, `destinationArn`, `id`, `resourceArn`, `resourceId`, `serviceNetworkLogType` | - |
| `CreateListenerRequest` | `structure` | `clientToken`, `defaultAction`, `name`, `port`, `protocol`, `serviceIdentifier`, `tags` | - |
| `CreateListenerResponse` | `structure` | `arn`, `defaultAction`, `id`, `name`, `port`, `protocol`, `serviceArn`, `serviceId` | - |
| `CreateResourceConfigurationRequest` | `structure` | `allowAssociationToShareableServiceNetwork`, `clientToken`, `customDomainName`, `domainVerificationIdentifier`, `groupDomain`, `name`, `portRanges`, `protocol`, `resourceConfigurationDefinition`, `resourceConfigurationGroupIdentifier`, `resourceGatewayIdentifier`, `tags`, ... (+1) | - |
| `CreateResourceConfigurationResponse` | `structure` | `allowAssociationToShareableServiceNetwork`, `arn`, `createdAt`, `customDomainName`, `domainVerificationArn`, `domainVerificationId`, `failureReason`, `groupDomain`, `id`, `name`, `portRanges`, `protocol`, ... (+5) | - |
| `CreateResourceGatewayRequest` | `structure` | `clientToken`, `ipAddressType`, `ipv4AddressesPerEni`, `name`, `securityGroupIds`, `subnetIds`, `tags`, `vpcIdentifier` | - |
| `CreateResourceGatewayResponse` | `structure` | `arn`, `id`, `ipAddressType`, `ipv4AddressesPerEni`, `name`, `securityGroupIds`, `status`, `subnetIds`, `vpcIdentifier` | - |
| `CreateRuleRequest` | `structure` | `action`, `clientToken`, `listenerIdentifier`, `match`, `name`, `priority`, `serviceIdentifier`, `tags` | - |
| `CreateRuleResponse` | `structure` | `action`, `arn`, `id`, `match`, `name`, `priority` | - |
| `CreateServiceRequest` | `structure` | `authType`, `certificateArn`, `clientToken`, `customDomainName`, `name`, `tags` | - |
| `CreateServiceResponse` | `structure` | `arn`, `authType`, `certificateArn`, `customDomainName`, `dnsEntry`, `id`, `name`, `status` | - |
| `CreateServiceNetworkRequest` | `structure` | `authType`, `clientToken`, `name`, `sharingConfig`, `tags` | - |
| `CreateServiceNetworkResponse` | `structure` | `arn`, `authType`, `id`, `name`, `sharingConfig` | - |
| `CreateServiceNetworkResourceAssociationRequest` | `structure` | `clientToken`, `privateDnsEnabled`, `resourceConfigurationIdentifier`, `serviceNetworkIdentifier`, `tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
