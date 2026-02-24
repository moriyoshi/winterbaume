# Elastic Load Balancing

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Elastic Load Balancing A load balancer distributes incoming traffic across targets, such as your EC2 instances. This enables you to increase the availability of your application. The load balancer also monitors the health of its registered targets and ensures that it routes traffic only to healthy targets. You configure your load balancer to accept incoming traffic by specifying one or more listeners, which are configured with a protocol and port number for connections from clients to the load balancer. You configure a target group with a protocol and port number for connections from the load balancer to the targets, and with health check settings to be used when checking the health status of the targets.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Elastic Load Balancing where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Elastic Load Balancing by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Elastic Load Balancing workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Modify`, `Delete`, `Create`, `Set` operation families, including `DescribeAccountLimits`, `DescribeCapacityReservation`, `DescribeListenerAttributes`, `DescribeListenerCertificates`, `ModifyCapacityReservation`, `ModifyIpPools`.

## Service Identity and Protocol

- AWS model slug: `elastic-load-balancing-v2`
- AWS SDK for Rust slug: `elasticloadbalancingv2`
- Model version: `2015-12-01`
- Model file: `vendor/api-models-aws/models/elastic-load-balancing-v2/service/2015-12-01/elastic-load-balancing-v2-2015-12-01.json`
- SDK ID: `Elastic Load Balancing v2`
- Endpoint prefix: `elasticloadbalancing`
- ARN namespace: `elasticloadbalancing`
- CloudFormation name: `ElasticLoadBalancingV2`
- CloudTrail event source: `elasticloadbalancing.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (16), `Modify` (9), `Delete` (6), `Create` (5), `Set` (4), `Add` (3), `Get` (3), `Remove` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddListenerCertificates`, `AddTags`, `AddTrustStoreRevocations`, `CreateListener`, `CreateLoadBalancer`, `CreateRule`, `CreateTargetGroup`, `CreateTrustStore`, `DeleteListener`, `DeleteLoadBalancer`, `DeleteRule`, `DeleteSharedTrustStoreAssociation`, `DeleteTargetGroup`, `DeleteTrustStore`, `DeregisterTargets`, `ModifyCapacityReservation`, `ModifyIpPools`, `ModifyListener`, `ModifyListenerAttributes`, `ModifyLoadBalancerAttributes`, ... (+12).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountLimits`, `DescribeCapacityReservation`, `DescribeListenerAttributes`, `DescribeListenerCertificates`, `DescribeListeners`, `DescribeLoadBalancerAttributes`, `DescribeLoadBalancers`, `DescribeRules`, `DescribeSSLPolicies`, `DescribeTags`, `DescribeTargetGroupAttributes`, `DescribeTargetGroups`, `DescribeTargetHealth`, `DescribeTrustStoreAssociations`, `DescribeTrustStoreRevocations`, `DescribeTrustStores`, `GetResourcePolicy`, `GetTrustStoreCaCertificatesBundle`, `GetTrustStoreRevocationContent`.
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 50 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-groups.html
- https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/how-elastic-load-balancing-works.html
- https://docs.aws.amazon.com/elasticloadbalancing/latest/network/edit-target-group-attributes.html

Research outcomes:
- Target groups route to targets using the protocol and port configured on the target group unless the registered target overrides the port.
- ALB target types are instance, ip, and lambda. Target type is immutable after target group creation.
- IP targets cannot be publicly routable addresses; allowed ranges include the VPC subnets and private/shared address ranges documented for target groups.
- Newly registered targets receive traffic after registration completes and the target passes its first initial health check, regardless of the configured healthy threshold.
- Deregistering a target stops new routing to it and places it in draining until in-flight requests complete. The default deregistration delay is 300 seconds.
- Target group attributes include deregistration delay, routing algorithm, slow start, stickiness, cross-zone setting, Lambda multi-value headers, and health threshold settings.
- Target group health settings can trigger DNS failover or routing failover when healthy targets fall below count or percentage thresholds.
- If all load balancer zones are unhealthy for DNS failover, the load balancer sends traffic to all zones, including unhealthy zones.

Parity implications:
- Model load balancers, listeners, listener rules, target groups, targets, health checks, target health, attributes, certificates, and DNS names separately.
- Registration, initial health checks, draining, deregistration delay, slow start, stickiness, cross-zone behaviour, and routing algorithms should affect traffic selection.
- Target health thresholds need DNS failover and routing failover semantics, including fail-open behaviour when all zones are unhealthy.

## v1/v2 State Coherence

- **Paired with `elasticloadbalancing` ( different SDK slug, different load balancer types ).** ELBv2 ( Application/Network/Gateway Load Balancers, this crate ) and Classic Load Balancer ( ELB v1 ) are **separate resource types** in real AWS. An ALB/NLB/GLB is not returned by Classic `DescribeLoadBalancers`, and a Classic ELB is not returned by ELBv2 `DescribeLoadBalancers`. Listeners, target groups, and rules are ELBv2-only concepts; backend instance registration and listener policies are Classic-only.
- **Shared resources to be aware of:**
  - **Load balancer name namespace.** Real AWS reserves load balancer names in a single namespace within an account+region: an ALB and a Classic ELB cannot share the same name. Creating a Classic ELB named `web` after creating an ALB named `web` returns `DuplicateLoadBalancerName` in real AWS.
  - **Account-level limits.** `DescribeAccountLimits` reports limits that span both ELBv2 and Classic quota families ( e.g. total load balancers per region ).
  - **ARN format differs** ( `arn:…:loadbalancer/<type>/<name>/<id>` for ELBv2 vs `arn:aws:elasticloadbalancing:…:loadbalancer/<name>` for Classic ); tags are scoped per ARN and correctly independent.
- **Current Winterbaume status: separated, with one observable gap.** Each crate owns its own `load_balancers` map without cross-crate name-uniqueness checks. A consumer that creates an ALB named `web` and then a Classic ELB named `web` ( or vice versa ) will succeed in Winterbaume but fail in real AWS. Cross-crate name reservation is the realistic follow-up; the resource bodies themselves are correctly separate.

## Current Network Resource Stub Semantics

ELBv2 currently synthesises load balancer networking unless later mutation calls overwrite it.

- `CreateLoadBalancer` ignores the real EC2 topology and creates local load balancer records with default placeholder VPC, subnet, and security group values such as `vpc-12345678`, `subnet-aaaa1111`, and `sg-12345678`.
- `CreateTargetGroup` stores the supplied VPC ID when present and otherwise defaults to the same placeholder VPC ID.
- `SetSubnets` replaces the stored subnet or subnet-mapping list and synthesises availability-zone records from the supplied order; `SetSecurityGroups` replaces the stored security group list.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccountLimits`, `DescribeCapacityReservation`, `DescribeListenerAttributes`, `DescribeListenerCertificates`, `DescribeListeners`, `DescribeLoadBalancerAttributes`, `DescribeLoadBalancers`, `DescribeRules`, `DescribeSSLPolicies`, `DescribeTags`, `DescribeTargetGroupAttributes`, `DescribeTargetGroups`, `DescribeTargetHealth`, `DescribeTrustStoreAssociations`, `DescribeTrustStoreRevocations`, `DescribeTrustStores`
- Traits: `paginated` (9)
- Common required input members in this group: `ListenerArn`, `LoadBalancerArn`, `ResourceArns`, `TargetGroupArn`, `TrustStoreArn`

### Modify

- Operations: `ModifyCapacityReservation`, `ModifyIpPools`, `ModifyListener`, `ModifyListenerAttributes`, `ModifyLoadBalancerAttributes`, `ModifyRule`, `ModifyTargetGroup`, `ModifyTargetGroupAttributes`, `ModifyTrustStore`
- Common required input members in this group: `Attributes`, `CaCertificatesBundleS3Bucket`, `CaCertificatesBundleS3Key`, `ListenerArn`, `LoadBalancerArn`, `RuleArn`, `TargetGroupArn`, `TrustStoreArn`

### Delete

- Operations: `DeleteListener`, `DeleteLoadBalancer`, `DeleteRule`, `DeleteSharedTrustStoreAssociation`, `DeleteTargetGroup`, `DeleteTrustStore`
- Common required input members in this group: `ListenerArn`, `LoadBalancerArn`, `ResourceArn`, `RuleArn`, `TargetGroupArn`, `TrustStoreArn`

### Create

- Operations: `CreateListener`, `CreateLoadBalancer`, `CreateRule`, `CreateTargetGroup`, `CreateTrustStore`
- Common required input members in this group: `Actions`, `CaCertificatesBundleS3Bucket`, `CaCertificatesBundleS3Key`, `Conditions`, `DefaultActions`, `ListenerArn`, `LoadBalancerArn`, `Name`, `Priority`

### Set

- Operations: `SetIpAddressType`, `SetRulePriorities`, `SetSecurityGroups`, `SetSubnets`
- Common required input members in this group: `IpAddressType`, `LoadBalancerArn`, `RulePriorities`, `SecurityGroups`

### Add

- Operations: `AddListenerCertificates`, `AddTags`, `AddTrustStoreRevocations`
- Common required input members in this group: `Certificates`, `ListenerArn`, `ResourceArns`, `Tags`, `TrustStoreArn`

### Get

- Operations: `GetResourcePolicy`, `GetTrustStoreCaCertificatesBundle`, `GetTrustStoreRevocationContent`
- Common required input members in this group: `ResourceArn`, `RevocationId`, `TrustStoreArn`

### Remove

- Operations: `RemoveListenerCertificates`, `RemoveTags`, `RemoveTrustStoreRevocations`
- Common required input members in this group: `Certificates`, `ListenerArn`, `ResourceArns`, `RevocationIds`, `TagKeys`, `TrustStoreArn`

### Deregister

- Operations: `DeregisterTargets`
- Common required input members in this group: `TargetGroupArn`, `Targets`

### Register

- Operations: `RegisterTargets`
- Common required input members in this group: `TargetGroupArn`, `Targets`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddListenerCertificates` | - | - | `Certificates`, `ListenerArn` | - | `AddListenerCertificatesOutput` | `CertificateNotFoundException`, `ListenerNotFoundException`, `TooManyCertificatesException` | Adds the specified SSL server certificate to the certificate list for the specified HTTPS or TLS listener. If the certificate in already in the certificate list, the call is successful but the certificate is not added again. |
| `AddTags` | - | - | `ResourceArns`, `Tags` | - | `AddTagsOutput` | `DuplicateTagKeysException`, `ListenerNotFoundException`, `LoadBalancerNotFoundException`, `RuleNotFoundException`, `TargetGroupNotFoundException`, `TooManyTagsException`, `TrustStoreNotFoundException` | Adds the specified tags to the specified Elastic Load Balancing resource. You can tag your Application Load Balancers, Network Load Balancers, Gateway Load Balancers, target groups, trust stores, listeners, and rules. |
| `AddTrustStoreRevocations` | - | - | `TrustStoreArn` | - | `AddTrustStoreRevocationsOutput` | `InvalidRevocationContentException`, `RevocationContentNotFoundException`, `TooManyTrustStoreRevocationEntriesException`, `TrustStoreNotFoundException` | Adds the specified revocation file to the specified trust store. |
| `CreateListener` | - | - | `DefaultActions`, `LoadBalancerArn` | - | `CreateListenerOutput` | `ALPNPolicyNotSupportedException`, `CertificateNotFoundException`, `DuplicateListenerException`, `IncompatibleProtocolsException`, `InvalidConfigurationRequestException`, `InvalidLoadBalancerActionException`, `LoadBalancerNotFoundException`, `SSLPolicyNotFoundException`, ... (+12) | Creates a listener for the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. For more information, see the following: Listeners for your Application Load Balancers Listeners for your Network Load Balancers Listeners for... |
| `CreateLoadBalancer` | - | - | `Name` | - | `CreateLoadBalancerOutput` | `AllocationIdNotFoundException`, `AvailabilityZoneNotSupportedException`, `DuplicateLoadBalancerNameException`, `DuplicateTagKeysException`, `InvalidConfigurationRequestException`, `InvalidSchemeException`, `InvalidSecurityGroupException`, `InvalidSubnetException`, ... (+5) | Creates an Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. For more information, see the following: Application Load Balancers Network Load Balancers Gateway Load Balancers This operation is idempotent, which means that it... |
| `CreateRule` | - | - | `Actions`, `Conditions`, `ListenerArn`, `Priority` | - | `CreateRuleOutput` | `IncompatibleProtocolsException`, `InvalidConfigurationRequestException`, `InvalidLoadBalancerActionException`, `ListenerNotFoundException`, `PriorityInUseException`, `TargetGroupAssociationLimitException`, `TargetGroupNotFoundException`, `TooManyActionsException`, ... (+7) | Creates a rule for the specified listener. The listener must be associated with an Application Load Balancer. |
| `CreateTargetGroup` | - | - | `Name` | - | `CreateTargetGroupOutput` | `DuplicateTargetGroupNameException`, `InvalidConfigurationRequestException`, `TooManyTagsException`, `TooManyTargetGroupsException` | Creates a target group. For more information, see the following: Target groups for your Application Load Balancers Target groups for your Network Load Balancers Target groups for your Gateway Load Balancers This operation is idempotent, which means that it... |
| `CreateTrustStore` | - | - | `CaCertificatesBundleS3Bucket`, `CaCertificatesBundleS3Key`, `Name` | - | `CreateTrustStoreOutput` | `CaCertificatesBundleNotFoundException`, `DuplicateTagKeysException`, `DuplicateTrustStoreNameException`, `InvalidCaCertificatesBundleException`, `TooManyTagsException`, `TooManyTrustStoresException` | Creates a trust store. For more information, see Mutual TLS for Application Load Balancers. |
| `DeleteListener` | - | - | `ListenerArn` | - | `DeleteListenerOutput` | `ListenerNotFoundException`, `ResourceInUseException` | Deletes the specified listener. Alternatively, your listener is deleted when you delete the load balancer to which it is attached. |
| `DeleteLoadBalancer` | - | - | `LoadBalancerArn` | - | `DeleteLoadBalancerOutput` | `LoadBalancerNotFoundException`, `OperationNotPermittedException`, `ResourceInUseException` | Deletes the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. Deleting a load balancer also deletes its listeners. |
| `DeleteRule` | - | - | `RuleArn` | - | `DeleteRuleOutput` | `OperationNotPermittedException`, `RuleNotFoundException` | Deletes the specified rule. You can't delete the default rule. |
| `DeleteSharedTrustStoreAssociation` | - | - | `ResourceArn`, `TrustStoreArn` | - | `DeleteSharedTrustStoreAssociationOutput` | `DeleteAssociationSameAccountException`, `TrustStoreAssociationNotFoundException`, `TrustStoreNotFoundException` | Deletes a shared trust store association. |
| `DeleteTargetGroup` | - | - | `TargetGroupArn` | - | `DeleteTargetGroupOutput` | `ResourceInUseException` | Deletes the specified target group. You can delete a target group if it is not referenced by any actions. |
| `DeleteTrustStore` | - | - | `TrustStoreArn` | - | `DeleteTrustStoreOutput` | `TrustStoreInUseException`, `TrustStoreNotFoundException` | Deletes a trust store. |
| `DeregisterTargets` | - | - | `TargetGroupArn`, `Targets` | - | `DeregisterTargetsOutput` | `InvalidTargetException`, `TargetGroupNotFoundException` | Deregisters the specified targets from the specified target group. After the targets are deregistered, they no longer receive traffic from the load balancer. |
| `DescribeAccountLimits` | - | `paginated` | - | - | `DescribeAccountLimitsOutput` | - | Describes the current Elastic Load Balancing resource limits for your Amazon Web Services account. For more information, see the following: Quotas for your Application Load Balancers Quotas for your Network Load Balancers Quotas for your Gateway Load Balancers |
| `DescribeCapacityReservation` | - | - | `LoadBalancerArn` | - | `DescribeCapacityReservationOutput` | `LoadBalancerNotFoundException` | Describes the capacity reservation status for the specified load balancer. |
| `DescribeListenerAttributes` | - | - | `ListenerArn` | - | `DescribeListenerAttributesOutput` | `ListenerNotFoundException` | Describes the attributes for the specified listener. |
| `DescribeListenerCertificates` | - | `paginated` | `ListenerArn` | - | `DescribeListenerCertificatesOutput` | `ListenerNotFoundException` | Describes the default certificate and the certificate list for the specified HTTPS or TLS listener. If the default certificate is also in the certificate list, it appears twice in the results (once with `IsDefault` set to true and once with `IsDefault` set to... |
| `DescribeListeners` | - | `paginated` | - | - | `DescribeListenersOutput` | `ListenerNotFoundException`, `LoadBalancerNotFoundException`, `UnsupportedProtocolException` | Describes the specified listeners or the listeners for the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. You must specify either a load balancer or one or more listeners. |
| `DescribeLoadBalancerAttributes` | - | - | `LoadBalancerArn` | - | `DescribeLoadBalancerAttributesOutput` | `LoadBalancerNotFoundException` | Describes the attributes for the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. For more information, see the following: Load balancer attributes in the Application Load Balancers Guide Load balancer attributes in the... |
| `DescribeLoadBalancers` | - | `paginated` | - | - | `DescribeLoadBalancersOutput` | `LoadBalancerNotFoundException` | Describes the specified load balancers or all of your load balancers. |
| `DescribeRules` | - | `paginated` | - | - | `DescribeRulesOutput` | `ListenerNotFoundException`, `RuleNotFoundException`, `UnsupportedProtocolException` | Describes the specified rules or the rules for the specified listener. You must specify either a listener or rules. |
| `DescribeSSLPolicies` | - | - | - | - | `DescribeSSLPoliciesOutput` | `SSLPolicyNotFoundException` | Describes the specified policies or all policies used for SSL negotiation. For more information, see Security policies in the Application Load Balancers Guide and Security policies in the Network Load Balancers Guide . |
| `DescribeTags` | - | - | `ResourceArns` | - | `DescribeTagsOutput` | `ListenerNotFoundException`, `LoadBalancerNotFoundException`, `RuleNotFoundException`, `TargetGroupNotFoundException`, `TrustStoreNotFoundException` | Describes the tags for the specified Elastic Load Balancing resources. You can describe the tags for one or more Application Load Balancers, Network Load Balancers, Gateway Load Balancers, target groups, listeners, or rules. |
| `DescribeTargetGroupAttributes` | - | - | `TargetGroupArn` | - | `DescribeTargetGroupAttributesOutput` | `TargetGroupNotFoundException` | Describes the attributes for the specified target group. For more information, see the following: Target group attributes in the Application Load Balancers Guide Target group attributes in the Network Load Balancers Guide Target group attributes in the... |
| `DescribeTargetGroups` | - | `paginated` | - | - | `DescribeTargetGroupsOutput` | `LoadBalancerNotFoundException`, `TargetGroupNotFoundException` | Describes the specified target groups or all of your target groups. By default, all target groups are described. |
| `DescribeTargetHealth` | - | - | `TargetGroupArn` | - | `DescribeTargetHealthOutput` | `HealthUnavailableException`, `InvalidTargetException`, `TargetGroupNotFoundException` | Describes the health of the specified targets or all of your targets. |
| `DescribeTrustStoreAssociations` | - | `paginated` | `TrustStoreArn` | - | `DescribeTrustStoreAssociationsOutput` | `TrustStoreNotFoundException` | Describes all resources associated with the specified trust store. |
| `DescribeTrustStoreRevocations` | - | `paginated` | `TrustStoreArn` | - | `DescribeTrustStoreRevocationsOutput` | `RevocationIdNotFoundException`, `TrustStoreNotFoundException` | Describes the revocation files in use by the specified trust store or revocation files. |
| `DescribeTrustStores` | - | `paginated` | - | - | `DescribeTrustStoresOutput` | `TrustStoreNotFoundException` | Describes all trust stores for the specified account. |
| `GetResourcePolicy` | - | - | `ResourceArn` | - | `GetResourcePolicyOutput` | `ResourceNotFoundException` | Retrieves the resource policy for a specified resource. |
| `GetTrustStoreCaCertificatesBundle` | - | - | `TrustStoreArn` | - | `GetTrustStoreCaCertificatesBundleOutput` | `TrustStoreNotFoundException` | Retrieves the ca certificate bundle. This action returns a pre-signed S3 URI which is active for ten minutes. |
| `GetTrustStoreRevocationContent` | - | - | `RevocationId`, `TrustStoreArn` | - | `GetTrustStoreRevocationContentOutput` | `RevocationIdNotFoundException`, `TrustStoreNotFoundException` | Retrieves the specified revocation file. This action returns a pre-signed S3 URI which is active for ten minutes. |
| `ModifyCapacityReservation` | - | - | `LoadBalancerArn` | - | `ModifyCapacityReservationOutput` | `CapacityDecreaseRequestsLimitExceededException`, `CapacityReservationPendingException`, `CapacityUnitsLimitExceededException`, `InsufficientCapacityException`, `InvalidConfigurationRequestException`, `LoadBalancerNotFoundException`, `OperationNotPermittedException`, `PriorRequestNotCompleteException` | Modifies the capacity reservation of the specified load balancer. When modifying capacity reservation, you must include at least one `MinimumLoadBalancerCapacity` or `ResetCapacityReservation`. |
| `ModifyIpPools` | - | - | `LoadBalancerArn` | - | `ModifyIpPoolsOutput` | `LoadBalancerNotFoundException` | [Application Load Balancers] Modify the IP pool associated to a load balancer. |
| `ModifyListener` | - | - | `ListenerArn` | - | `ModifyListenerOutput` | `ALPNPolicyNotSupportedException`, `CertificateNotFoundException`, `DuplicateListenerException`, `IncompatibleProtocolsException`, `InvalidConfigurationRequestException`, `InvalidLoadBalancerActionException`, `ListenerNotFoundException`, `SSLPolicyNotFoundException`, ... (+11) | Replaces the specified properties of the specified listener. Any properties that you do not specify remain unchanged. |
| `ModifyListenerAttributes` | - | - | `Attributes`, `ListenerArn` | - | `ModifyListenerAttributesOutput` | `InvalidConfigurationRequestException`, `ListenerNotFoundException` | Modifies the specified attributes of the specified listener. |
| `ModifyLoadBalancerAttributes` | - | - | `Attributes`, `LoadBalancerArn` | - | `ModifyLoadBalancerAttributesOutput` | `InvalidConfigurationRequestException`, `LoadBalancerNotFoundException` | Modifies the specified attributes of the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. If any of the specified attributes can't be modified as requested, the call fails. |
| `ModifyRule` | - | - | `RuleArn` | - | `ModifyRuleOutput` | `IncompatibleProtocolsException`, `InvalidLoadBalancerActionException`, `OperationNotPermittedException`, `RuleNotFoundException`, `TargetGroupAssociationLimitException`, `TargetGroupNotFoundException`, `TooManyActionsException`, `TooManyRegistrationsForTargetIdException`, ... (+3) | Replaces the specified properties of the specified rule. Any properties that you do not specify are unchanged. |
| `ModifyTargetGroup` | - | - | `TargetGroupArn` | - | `ModifyTargetGroupOutput` | `InvalidConfigurationRequestException`, `TargetGroupNotFoundException` | Modifies the health checks used when evaluating the health state of the targets in the specified target group. |
| `ModifyTargetGroupAttributes` | - | - | `Attributes`, `TargetGroupArn` | - | `ModifyTargetGroupAttributesOutput` | `InvalidConfigurationRequestException`, `TargetGroupNotFoundException` | Modifies the specified attributes of the specified target group. |
| `ModifyTrustStore` | - | - | `CaCertificatesBundleS3Bucket`, `CaCertificatesBundleS3Key`, `TrustStoreArn` | - | `ModifyTrustStoreOutput` | `CaCertificatesBundleNotFoundException`, `InvalidCaCertificatesBundleException`, `TrustStoreNotFoundException` | Update the ca certificate bundle for the specified trust store. |
| `RegisterTargets` | - | - | `TargetGroupArn`, `Targets` | - | `RegisterTargetsOutput` | `InvalidTargetException`, `TargetGroupNotFoundException`, `TooManyRegistrationsForTargetIdException`, `TooManyTargetsException` | Registers the specified targets with the specified target group. If the target is an EC2 instance, it must be in the `running` state when you register it. |
| `RemoveListenerCertificates` | - | - | `Certificates`, `ListenerArn` | - | `RemoveListenerCertificatesOutput` | `ListenerNotFoundException`, `OperationNotPermittedException` | Removes the specified certificate from the certificate list for the specified HTTPS or TLS listener. |
| `RemoveTags` | - | - | `ResourceArns`, `TagKeys` | - | `RemoveTagsOutput` | `ListenerNotFoundException`, `LoadBalancerNotFoundException`, `RuleNotFoundException`, `TargetGroupNotFoundException`, `TooManyTagsException`, `TrustStoreNotFoundException` | Removes the specified tags from the specified Elastic Load Balancing resources. You can remove the tags for one or more Application Load Balancers, Network Load Balancers, Gateway Load Balancers, target groups, listeners, or rules. |
| `RemoveTrustStoreRevocations` | - | - | `RevocationIds`, `TrustStoreArn` | - | `RemoveTrustStoreRevocationsOutput` | `RevocationIdNotFoundException`, `TrustStoreNotFoundException` | Removes the specified revocation file from the specified trust store. |
| `SetIpAddressType` | - | - | `IpAddressType`, `LoadBalancerArn` | - | `SetIpAddressTypeOutput` | `InvalidConfigurationRequestException`, `InvalidSubnetException`, `LoadBalancerNotFoundException` | Sets the type of IP addresses used by the subnets of the specified load balancer. |
| `SetRulePriorities` | - | - | `RulePriorities` | - | `SetRulePrioritiesOutput` | `OperationNotPermittedException`, `PriorityInUseException`, `RuleNotFoundException` | Sets the priorities of the specified rules. You can reorder the rules as long as there are no priority conflicts in the new order. |
| `SetSecurityGroups` | - | - | `LoadBalancerArn`, `SecurityGroups` | - | `SetSecurityGroupsOutput` | `InvalidConfigurationRequestException`, `InvalidSecurityGroupException`, `LoadBalancerNotFoundException` | Associates the specified security groups with the specified Application Load Balancer or Network Load Balancer. The specified security groups override the previously associated security groups. |
| `SetSubnets` | - | - | `LoadBalancerArn` | - | `SetSubnetsOutput` | `AllocationIdNotFoundException`, `AvailabilityZoneNotSupportedException`, `CapacityReservationPendingException`, `InvalidConfigurationRequestException`, `InvalidSubnetException`, `LoadBalancerNotFoundException`, `SubnetNotFoundException` | Enables the Availability Zones for the specified public subnets for the specified Application Load Balancer, Network Load Balancer or Gateway Load Balancer. The specified subnets replace the previously enabled subnets. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `LoadBalancerNotFoundException` | `structure` | `Message` | The specified load balancer does not exist. |
| `TrustStoreNotFoundException` | `structure` | `Message` | The specified trust store does not exist. |
| `TargetGroupNotFoundException` | `structure` | `Message` | The specified target group does not exist. |
| `ListenerNotFoundException` | `structure` | `Message` | The specified listener does not exist. |
| `InvalidConfigurationRequestException` | `structure` | `Message` | The requested configuration is not valid. |
| `RuleNotFoundException` | `structure` | `Message` | The specified rule does not exist. |
| `TooManyTagsException` | `structure` | `Message` | You've reached the limit on the number of tags for this resource. |
| `OperationNotPermittedException` | `structure` | `Message` | This operation is not allowed. |
| `UnsupportedProtocolException` | `structure` | `Message` | The specified protocol is not supported. |
| `TooManyRegistrationsForTargetIdException` | `structure` | `Message` | You've reached the limit on the number of times a target can be registered with a load balancer. |
| `TooManyTargetsException` | `structure` | `Message` | You've reached the limit on the number of targets. |
| `IncompatibleProtocolsException` | `structure` | `Message` | The specified configuration is not valid with this protocol. |
| `InvalidLoadBalancerActionException` | `structure` | `Message` | The requested action is not valid. |
| `TargetGroupAssociationLimitException` | `structure` | `Message` | You've reached the limit on the number of load balancers per target group. |
| `TooManyActionsException` | `structure` | `Message` | You've reached the limit on the number of actions per rule. |
| `TooManyUniqueTargetGroupsPerLoadBalancerException` | `structure` | `Message` | You've reached the limit on the number of unique target groups per load balancer across all listeners. |
| `ResourceInUseException` | `structure` | `Message` | A specified resource is in use. |
| `CertificateNotFoundException` | `structure` | `Message` | The specified certificate does not exist. |
| `TooManyCertificatesException` | `structure` | `Message` | You've reached the limit on the number of certificates per load balancer. |
| `DuplicateTagKeysException` | `structure` | `Message` | A tag key was specified more than once. |
| `SSLPolicyNotFoundException` | `structure` | `Message` | The specified SSL policy does not exist. |
| `InvalidSubnetException` | `structure` | `Message` | The specified subnet is out of available addresses. |
| `InvalidTargetException` | `structure` | `Message` | The specified target does not exist, is not in the same VPC as the target group, or has an unsupported instance type. |
| `RevocationIdNotFoundException` | `structure` | `Message` | The specified revocation ID does not exist. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
