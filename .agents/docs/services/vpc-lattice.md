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

### Delete

- Operations: `DeleteAuthPolicy`, `DeleteResourcePolicy`
- Traits: `idempotent` (2)
- Common required input members in this group: -

### Get

- Operations: `GetAuthPolicy`, `GetResourcePolicy`
- Traits: `readonly` (2)
- Common required input members in this group: -

### List

- Operations: `ListServiceNetworkVpcEndpointAssociations`, `ListTagsForResource`
- Traits: `readonly` (2), `paginated` (1)
- Common required input members in this group: -

### Put

- Operations: `PutAuthPolicy`, `PutResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: `policy`

### Batch

- Operations: `BatchUpdateRule`
- Traits: `idempotent` (1)
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
| `BatchUpdateRule` | `PATCH /services/{serviceIdentifier}/listeners/{listenerIdentifier}/rules` | `idempotent` | `serviceIdentifier`, `listenerIdentifier`, `rules` | - | `BatchUpdateRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the listener rules in a batch. You can use this operation to change the priority of listener rules. This can be useful when bulk updating or swapping rule priority. Required permissions: vpc-lattice:UpdateRul ... |
| `DeleteAuthPolicy` | `DELETE /authpolicy/{resourceIdentifier}` | `idempotent` | `resourceIdentifier` | - | `DeleteAuthPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified auth policy. If an auth is set to AWS_IAM and the auth policy is deleted, all requests are denied. If you are trying to remove the auth policy completely, you must set the auth type to NONE . If ... |
| `DeleteResourcePolicy` | `DELETE /resourcepolicy/{resourceArn}` | `idempotent` | `resourceArn` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified resource policy. |
| `GetAuthPolicy` | `GET /authpolicy/{resourceIdentifier}` | `readonly` | `resourceIdentifier` | - | `GetAuthPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the auth policy for the specified service or service network. |
| `GetResourcePolicy` | `GET /resourcepolicy/{resourceArn}` | `readonly` | `resourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified resource policy. The resource policy is an IAM policy created on behalf of the resource owner when they share a resource. |
| `ListServiceNetworkVpcEndpointAssociations` | `GET /servicenetworkvpcendpointassociations` | `readonly`, `paginated` | `serviceNetworkIdentifier` | - | `ListServiceNetworkVpcEndpointAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the associations between a service network and a VPC endpoint. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags for the specified resource. |
| `PutAuthPolicy` | `PUT /authpolicy/{resourceIdentifier}` | - | `resourceIdentifier`, `policy` | - | `PutAuthPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates the auth policy. The policy string in JSON must not contain newlines or blank lines. For more information, see Auth policies in the Amazon VPC Lattice User Guide . |
| `PutResourcePolicy` | `PUT /resourcepolicy/{resourceArn}` | `idempotent` | `resourceArn`, `policy` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches a resource-based permission policy to a service or service network. The policy must contain the same actions and condition statements as the Amazon Web Services Resource Access Manager permission for sharing ... |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds the specified tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListServiceNetworkVpcEndpointAssociations` | - | `serviceNetworkIdentifier -> serviceNetworkIdentifier`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The user does not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | The request conflicts with the current state of the resource. Updating or deleting a resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | An unexpected error occurred while processing the request. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The request references a resource that does not exist. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | The request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The limit on the number of requests per second was exceeded. |
| `ValidationException` | `structure` | message, reason, fieldList | The input does not satisfy the constraints specified by an Amazon Web Services service. |
| `BatchUpdateRuleRequest` | `structure` | serviceIdentifier, listenerIdentifier, rules | - |
| `BatchUpdateRuleResponse` | `structure` | successful, unsuccessful | - |
| `DeleteAuthPolicyRequest` | `structure` | resourceIdentifier | - |
| `DeleteAuthPolicyResponse` | `structure` | **empty (no members)** | - |
| `DeleteResourcePolicyRequest` | `structure` | resourceArn | - |
| `DeleteResourcePolicyResponse` | `structure` | **empty (no members)** | - |
| `GetAuthPolicyRequest` | `structure` | resourceIdentifier | - |
| `GetAuthPolicyResponse` | `structure` | policy, state, createdAt, lastUpdatedAt | - |
| `GetResourcePolicyRequest` | `structure` | resourceArn | - |
| `GetResourcePolicyResponse` | `structure` | policy | - |
| `ListServiceNetworkVpcEndpointAssociationsRequest` | `structure` | serviceNetworkIdentifier, maxResults, nextToken | - |
| `ListServiceNetworkVpcEndpointAssociationsResponse` | `structure` | items, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `PutAuthPolicyRequest` | `structure` | resourceIdentifier, policy | - |
| `PutAuthPolicyResponse` | `structure` | policy, state | - |
| `PutResourcePolicyRequest` | `structure` | resourceArn, policy | - |
| `PutResourcePolicyResponse` | `structure` | **empty (no members)** | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
