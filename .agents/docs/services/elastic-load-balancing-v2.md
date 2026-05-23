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
- Common required input members in this group: `LoadBalancerArn`, `ListenerArn`, `TargetGroupArn`, `TrustStoreArn`

### Modify

- Operations: `ModifyCapacityReservation`, `ModifyIpPools`, `ModifyListener`, `ModifyListenerAttributes`, `ModifyLoadBalancerAttributes`, `ModifyRule`, `ModifyTargetGroup`, `ModifyTargetGroupAttributes`, `ModifyTrustStore`
- Common required input members in this group: `LoadBalancerArn`, `ListenerArn`, `Attributes`, `TargetGroupArn`

### Delete

- Operations: `DeleteListener`, `DeleteLoadBalancer`, `DeleteRule`, `DeleteSharedTrustStoreAssociation`, `DeleteTargetGroup`, `DeleteTrustStore`
- Common required input members in this group: `TrustStoreArn`

### Create

- Operations: `CreateListener`, `CreateLoadBalancer`, `CreateRule`, `CreateTargetGroup`, `CreateTrustStore`
- Common required input members in this group: `Name`

### Set

- Operations: `SetIpAddressType`, `SetRulePriorities`, `SetSecurityGroups`, `SetSubnets`
- Common required input members in this group: `LoadBalancerArn`

### Add

- Operations: `AddListenerCertificates`, `AddTags`, `AddTrustStoreRevocations`
- Common required input members in this group: -

### Get

- Operations: `GetResourcePolicy`, `GetTrustStoreCaCertificatesBundle`, `GetTrustStoreRevocationContent`
- Common required input members in this group: `TrustStoreArn`

### Remove

- Operations: `RemoveListenerCertificates`, `RemoveTags`, `RemoveTrustStoreRevocations`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterTargets`
- Common required input members in this group: -

### Register

- Operations: `RegisterTargets`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddListenerCertificates` | `-` | - | `ListenerArn`, `Certificates` | - | `AddListenerCertificatesOutput` | `CertificateNotFoundException`, `ListenerNotFoundException`, `TooManyCertificatesException` | Adds the specified SSL server certificate to the certificate list for the specified HTTPS or TLS listener. If the certificate in already in the certificate list, the call is successful but the certificate is not adde ... |
| `AddTags` | `-` | - | `ResourceArns`, `Tags` | - | `AddTagsOutput` | `DuplicateTagKeysException`, `ListenerNotFoundException`, `LoadBalancerNotFoundException`, `RuleNotFoundException`, `TargetGroupNotFoundException`, `TooManyTagsException`, `TrustStoreNotFoundException` | Adds the specified tags to the specified Elastic Load Balancing resource. You can tag your Application Load Balancers, Network Load Balancers, Gateway Load Balancers, target groups, trust stores, listeners, and rules ... |
| `AddTrustStoreRevocations` | `-` | - | `TrustStoreArn` | - | `AddTrustStoreRevocationsOutput` | `InvalidRevocationContentException`, `RevocationContentNotFoundException`, `TooManyTrustStoreRevocationEntriesException`, `TrustStoreNotFoundException` | Adds the specified revocation file to the specified trust store. |
| `CreateListener` | `-` | - | `LoadBalancerArn`, `DefaultActions` | - | `CreateListenerOutput` | `ALPNPolicyNotSupportedException`, `CertificateNotFoundException`, `DuplicateListenerException`, `IncompatibleProtocolsException`, `InvalidConfigurationRequestException`, `InvalidLoadBalancerActionException`, `LoadBalancerNotFoundException`, `SSLPolicyNotFoundException`, `TargetGroupAssociationLimitException`, `TargetGroupNotFoundException`, `TooManyActionsException`, `TooManyCertificatesException`, `TooManyListenersException`, `TooManyRegistrationsForTargetIdException`, `TooManyTagsException`, `TooManyTargetsException`, `TooManyUniqueTargetGroupsPerLoadBalancerException`, `TrustStoreNotFoundException`, `TrustStoreNotReadyException`, `UnsupportedProtocolException` | Creates a listener for the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. For more information, see the following: Listeners for your Application Load Balancers Listeners for yo ... |
| `CreateLoadBalancer` | `-` | - | `Name` | - | `CreateLoadBalancerOutput` | `AllocationIdNotFoundException`, `AvailabilityZoneNotSupportedException`, `DuplicateLoadBalancerNameException`, `DuplicateTagKeysException`, `InvalidConfigurationRequestException`, `InvalidSchemeException`, `InvalidSecurityGroupException`, `InvalidSubnetException`, `OperationNotPermittedException`, `ResourceInUseException`, `SubnetNotFoundException`, `TooManyLoadBalancersException`, `TooManyTagsException` | Creates an Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. For more information, see the following: Application Load Balancers Network Load Balancers Gateway Load Balancers This operation ... |
| `CreateRule` | `-` | - | `ListenerArn`, `Conditions`, `Priority`, `Actions` | - | `CreateRuleOutput` | `IncompatibleProtocolsException`, `InvalidConfigurationRequestException`, `InvalidLoadBalancerActionException`, `ListenerNotFoundException`, `PriorityInUseException`, `TargetGroupAssociationLimitException`, `TargetGroupNotFoundException`, `TooManyActionsException`, `TooManyRegistrationsForTargetIdException`, `TooManyRulesException`, `TooManyTagsException`, `TooManyTargetGroupsException`, `TooManyTargetsException`, `TooManyUniqueTargetGroupsPerLoadBalancerException`, `UnsupportedProtocolException` | Creates a rule for the specified listener. The listener must be associated with an Application Load Balancer. Each rule consists of a priority, one or more actions, one or more conditions, and up to two optional tran ... |
| `CreateTargetGroup` | `-` | - | `Name` | - | `CreateTargetGroupOutput` | `DuplicateTargetGroupNameException`, `InvalidConfigurationRequestException`, `TooManyTagsException`, `TooManyTargetGroupsException` | Creates a target group. For more information, see the following: Target groups for your Application Load Balancers Target groups for your Network Load Balancers Target groups for your Gateway Load Balancers This oper ... |
| `CreateTrustStore` | `-` | - | `Name`, `CaCertificatesBundleS3Bucket`, `CaCertificatesBundleS3Key` | - | `CreateTrustStoreOutput` | `CaCertificatesBundleNotFoundException`, `DuplicateTagKeysException`, `DuplicateTrustStoreNameException`, `InvalidCaCertificatesBundleException`, `TooManyTagsException`, `TooManyTrustStoresException` | Creates a trust store. For more information, see Mutual TLS for Application Load Balancers . |
| `DeleteListener` | `-` | - | `ListenerArn` | - | `DeleteListenerOutput` | `ListenerNotFoundException`, `ResourceInUseException` | Deletes the specified listener. Alternatively, your listener is deleted when you delete the load balancer to which it is attached. |
| `DeleteLoadBalancer` | `-` | - | `LoadBalancerArn` | - | `DeleteLoadBalancerOutput` | `LoadBalancerNotFoundException`, `OperationNotPermittedException`, `ResourceInUseException` | Deletes the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. Deleting a load balancer also deletes its listeners. You can't delete a load balancer if deletion protection is enable ... |
| `DeleteRule` | `-` | - | `RuleArn` | - | `DeleteRuleOutput` | `OperationNotPermittedException`, `RuleNotFoundException` | Deletes the specified rule. You can't delete the default rule. |
| `DeleteSharedTrustStoreAssociation` | `-` | - | `TrustStoreArn`, `ResourceArn` | - | `DeleteSharedTrustStoreAssociationOutput` | `DeleteAssociationSameAccountException`, `TrustStoreAssociationNotFoundException`, `TrustStoreNotFoundException` | Deletes a shared trust store association. |
| `DeleteTargetGroup` | `-` | - | `TargetGroupArn` | - | `DeleteTargetGroupOutput` | `ResourceInUseException` | Deletes the specified target group. You can delete a target group if it is not referenced by any actions. Deleting a target group also deletes any associated health checks. Deleting a target group does not affect its ... |
| `DeleteTrustStore` | `-` | - | `TrustStoreArn` | - | `DeleteTrustStoreOutput` | `TrustStoreInUseException`, `TrustStoreNotFoundException` | Deletes a trust store. |
| `DeregisterTargets` | `-` | - | `TargetGroupArn`, `Targets` | - | `DeregisterTargetsOutput` | `InvalidTargetException`, `TargetGroupNotFoundException` | Deregisters the specified targets from the specified target group. After the targets are deregistered, they no longer receive traffic from the load balancer. The load balancer stops sending requests to targets that a ... |
| `DescribeAccountLimits` | `-` | `paginated` | - | - | `DescribeAccountLimitsOutput` | - | Describes the current Elastic Load Balancing resource limits for your Amazon Web Services account. For more information, see the following: Quotas for your Application Load Balancers Quotas for your Network Load Bala ... |
| `DescribeCapacityReservation` | `-` | - | `LoadBalancerArn` | - | `DescribeCapacityReservationOutput` | `LoadBalancerNotFoundException` | Describes the capacity reservation status for the specified load balancer. |
| `DescribeListenerAttributes` | `-` | - | `ListenerArn` | - | `DescribeListenerAttributesOutput` | `ListenerNotFoundException` | Describes the attributes for the specified listener. |
| `DescribeListenerCertificates` | `-` | `paginated` | `ListenerArn` | - | `DescribeListenerCertificatesOutput` | `ListenerNotFoundException` | Describes the default certificate and the certificate list for the specified HTTPS or TLS listener. If the default certificate is also in the certificate list, it appears twice in the results (once with IsDefault set ... |
| `DescribeListeners` | `-` | `paginated` | - | - | `DescribeListenersOutput` | `ListenerNotFoundException`, `LoadBalancerNotFoundException`, `UnsupportedProtocolException` | Describes the specified listeners or the listeners for the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. You must specify either a load balancer or one or more listeners. |
| `DescribeLoadBalancerAttributes` | `-` | - | `LoadBalancerArn` | - | `DescribeLoadBalancerAttributesOutput` | `LoadBalancerNotFoundException` | Describes the attributes for the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. For more information, see the following: Load balancer attributes in the Application Load Balance ... |
| `DescribeLoadBalancers` | `-` | `paginated` | - | - | `DescribeLoadBalancersOutput` | `LoadBalancerNotFoundException` | Describes the specified load balancers or all of your load balancers. |
| `DescribeRules` | `-` | `paginated` | - | - | `DescribeRulesOutput` | `ListenerNotFoundException`, `RuleNotFoundException`, `UnsupportedProtocolException` | Describes the specified rules or the rules for the specified listener. You must specify either a listener or rules. |
| `DescribeSSLPolicies` | `-` | - | - | - | `DescribeSSLPoliciesOutput` | `SSLPolicyNotFoundException` | Describes the specified policies or all policies used for SSL negotiation. For more information, see Security policies in the Application Load Balancers Guide and Security policies in the Network Load Balancers Guide . |
| `DescribeTags` | `-` | - | `ResourceArns` | - | `DescribeTagsOutput` | `ListenerNotFoundException`, `LoadBalancerNotFoundException`, `RuleNotFoundException`, `TargetGroupNotFoundException`, `TrustStoreNotFoundException` | Describes the tags for the specified Elastic Load Balancing resources. You can describe the tags for one or more Application Load Balancers, Network Load Balancers, Gateway Load Balancers, target groups, listeners, o ... |
| `DescribeTargetGroupAttributes` | `-` | - | `TargetGroupArn` | - | `DescribeTargetGroupAttributesOutput` | `TargetGroupNotFoundException` | Describes the attributes for the specified target group. For more information, see the following: Target group attributes in the Application Load Balancers Guide Target group attributes in the Network Load Balancers ... |
| `DescribeTargetGroups` | `-` | `paginated` | - | - | `DescribeTargetGroupsOutput` | `LoadBalancerNotFoundException`, `TargetGroupNotFoundException` | Describes the specified target groups or all of your target groups. By default, all target groups are described. Alternatively, you can specify one of the following to filter the results: the ARN of the load balancer ... |
| `DescribeTargetHealth` | `-` | - | `TargetGroupArn` | - | `DescribeTargetHealthOutput` | `HealthUnavailableException`, `InvalidTargetException`, `TargetGroupNotFoundException` | Describes the health of the specified targets or all of your targets. |
| `DescribeTrustStoreAssociations` | `-` | `paginated` | `TrustStoreArn` | - | `DescribeTrustStoreAssociationsOutput` | `TrustStoreNotFoundException` | Describes all resources associated with the specified trust store. |
| `DescribeTrustStoreRevocations` | `-` | `paginated` | `TrustStoreArn` | - | `DescribeTrustStoreRevocationsOutput` | `RevocationIdNotFoundException`, `TrustStoreNotFoundException` | Describes the revocation files in use by the specified trust store or revocation files. |
| `DescribeTrustStores` | `-` | `paginated` | - | - | `DescribeTrustStoresOutput` | `TrustStoreNotFoundException` | Describes all trust stores for the specified account. |
| `GetResourcePolicy` | `-` | - | `ResourceArn` | - | `GetResourcePolicyOutput` | `ResourceNotFoundException` | Retrieves the resource policy for a specified resource. |
| `GetTrustStoreCaCertificatesBundle` | `-` | - | `TrustStoreArn` | - | `GetTrustStoreCaCertificatesBundleOutput` | `TrustStoreNotFoundException` | Retrieves the ca certificate bundle. This action returns a pre-signed S3 URI which is active for ten minutes. |
| `GetTrustStoreRevocationContent` | `-` | - | `TrustStoreArn`, `RevocationId` | - | `GetTrustStoreRevocationContentOutput` | `RevocationIdNotFoundException`, `TrustStoreNotFoundException` | Retrieves the specified revocation file. This action returns a pre-signed S3 URI which is active for ten minutes. |
| `ModifyCapacityReservation` | `-` | - | `LoadBalancerArn` | - | `ModifyCapacityReservationOutput` | `CapacityDecreaseRequestsLimitExceededException`, `CapacityReservationPendingException`, `CapacityUnitsLimitExceededException`, `InsufficientCapacityException`, `InvalidConfigurationRequestException`, `LoadBalancerNotFoundException`, `OperationNotPermittedException`, `PriorRequestNotCompleteException` | Modifies the capacity reservation of the specified load balancer. When modifying capacity reservation, you must include at least one MinimumLoadBalancerCapacity or ResetCapacityReservation . |
| `ModifyIpPools` | `-` | - | `LoadBalancerArn` | - | `ModifyIpPoolsOutput` | `LoadBalancerNotFoundException` | [Application Load Balancers] Modify the IP pool associated to a load balancer. |
| `ModifyListener` | `-` | - | `ListenerArn` | - | `ModifyListenerOutput` | `ALPNPolicyNotSupportedException`, `CertificateNotFoundException`, `DuplicateListenerException`, `IncompatibleProtocolsException`, `InvalidConfigurationRequestException`, `InvalidLoadBalancerActionException`, `ListenerNotFoundException`, `SSLPolicyNotFoundException`, `TargetGroupAssociationLimitException`, `TargetGroupNotFoundException`, `TooManyActionsException`, `TooManyCertificatesException`, `TooManyListenersException`, `TooManyRegistrationsForTargetIdException`, `TooManyTargetsException`, `TooManyUniqueTargetGroupsPerLoadBalancerException`, `TrustStoreNotFoundException`, `TrustStoreNotReadyException`, `UnsupportedProtocolException` | Replaces the specified properties of the specified listener. Any properties that you do not specify remain unchanged. Changing the protocol from HTTPS to HTTP, or from TLS to TCP, removes the security policy and defa ... |
| `ModifyListenerAttributes` | `-` | - | `ListenerArn`, `Attributes` | - | `ModifyListenerAttributesOutput` | `InvalidConfigurationRequestException`, `ListenerNotFoundException` | Modifies the specified attributes of the specified listener. |
| `ModifyLoadBalancerAttributes` | `-` | - | `LoadBalancerArn`, `Attributes` | - | `ModifyLoadBalancerAttributesOutput` | `InvalidConfigurationRequestException`, `LoadBalancerNotFoundException` | Modifies the specified attributes of the specified Application Load Balancer, Network Load Balancer, or Gateway Load Balancer. If any of the specified attributes can't be modified as requested, the call fails. Any ex ... |
| `ModifyRule` | `-` | - | `RuleArn` | - | `ModifyRuleOutput` | `IncompatibleProtocolsException`, `InvalidLoadBalancerActionException`, `OperationNotPermittedException`, `RuleNotFoundException`, `TargetGroupAssociationLimitException`, `TargetGroupNotFoundException`, `TooManyActionsException`, `TooManyRegistrationsForTargetIdException`, `TooManyTargetsException`, `TooManyUniqueTargetGroupsPerLoadBalancerException`, `UnsupportedProtocolException` | Replaces the specified properties of the specified rule. Any properties that you do not specify are unchanged. To add an item to a list, remove an item from a list, or update an item in a list, you must provide the e ... |
| `ModifyTargetGroup` | `-` | - | `TargetGroupArn` | - | `ModifyTargetGroupOutput` | `InvalidConfigurationRequestException`, `TargetGroupNotFoundException` | Modifies the health checks used when evaluating the health state of the targets in the specified target group. |
| `ModifyTargetGroupAttributes` | `-` | - | `TargetGroupArn`, `Attributes` | - | `ModifyTargetGroupAttributesOutput` | `InvalidConfigurationRequestException`, `TargetGroupNotFoundException` | Modifies the specified attributes of the specified target group. |
| `ModifyTrustStore` | `-` | - | `TrustStoreArn`, `CaCertificatesBundleS3Bucket`, `CaCertificatesBundleS3Key` | - | `ModifyTrustStoreOutput` | `CaCertificatesBundleNotFoundException`, `InvalidCaCertificatesBundleException`, `TrustStoreNotFoundException` | Update the ca certificate bundle for the specified trust store. |
| `RegisterTargets` | `-` | - | `TargetGroupArn`, `Targets` | - | `RegisterTargetsOutput` | `InvalidTargetException`, `TargetGroupNotFoundException`, `TooManyRegistrationsForTargetIdException`, `TooManyTargetsException` | Registers the specified targets with the specified target group. If the target is an EC2 instance, it must be in the running state when you register it. By default, the load balancer routes requests to registered tar ... |
| `RemoveListenerCertificates` | `-` | - | `ListenerArn`, `Certificates` | - | `RemoveListenerCertificatesOutput` | `ListenerNotFoundException`, `OperationNotPermittedException` | Removes the specified certificate from the certificate list for the specified HTTPS or TLS listener. |
| `RemoveTags` | `-` | - | `ResourceArns`, `TagKeys` | - | `RemoveTagsOutput` | `ListenerNotFoundException`, `LoadBalancerNotFoundException`, `RuleNotFoundException`, `TargetGroupNotFoundException`, `TooManyTagsException`, `TrustStoreNotFoundException` | Removes the specified tags from the specified Elastic Load Balancing resources. You can remove the tags for one or more Application Load Balancers, Network Load Balancers, Gateway Load Balancers, target groups, liste ... |
| `RemoveTrustStoreRevocations` | `-` | - | `TrustStoreArn`, `RevocationIds` | - | `RemoveTrustStoreRevocationsOutput` | `RevocationIdNotFoundException`, `TrustStoreNotFoundException` | Removes the specified revocation file from the specified trust store. |
| `SetIpAddressType` | `-` | - | `LoadBalancerArn`, `IpAddressType` | - | `SetIpAddressTypeOutput` | `InvalidConfigurationRequestException`, `InvalidSubnetException`, `LoadBalancerNotFoundException` | Sets the type of IP addresses used by the subnets of the specified load balancer. |
| `SetRulePriorities` | `-` | - | `RulePriorities` | - | `SetRulePrioritiesOutput` | `OperationNotPermittedException`, `PriorityInUseException`, `RuleNotFoundException` | Sets the priorities of the specified rules. You can reorder the rules as long as there are no priority conflicts in the new order. Any existing rules that you do not specify retain their current priority. |
| `SetSecurityGroups` | `-` | - | `LoadBalancerArn`, `SecurityGroups` | - | `SetSecurityGroupsOutput` | `InvalidConfigurationRequestException`, `InvalidSecurityGroupException`, `LoadBalancerNotFoundException` | Associates the specified security groups with the specified Application Load Balancer or Network Load Balancer. The specified security groups override the previously associated security groups. You can't perform this ... |
| `SetSubnets` | `-` | - | `LoadBalancerArn` | - | `SetSubnetsOutput` | `AllocationIdNotFoundException`, `AvailabilityZoneNotSupportedException`, `CapacityReservationPendingException`, `InvalidConfigurationRequestException`, `InvalidSubnetException`, `LoadBalancerNotFoundException`, `SubnetNotFoundException` | Enables the Availability Zones for the specified public subnets for the specified Application Load Balancer, Network Load Balancer or Gateway Load Balancer. The specified subnets replace the previously enabled subnets. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ALPNPolicyNotSupportedException` | `structure` | Message | The specified ALPN policy is not supported. |
| `AllocationIdNotFoundException` | `structure` | Message | The specified allocation ID does not exist. |
| `AvailabilityZoneNotSupportedException` | `structure` | Message | The specified Availability Zone is not supported. |
| `CaCertificatesBundleNotFoundException` | `structure` | Message | The specified ca certificate bundle does not exist. |
| `CapacityDecreaseRequestsLimitExceededException` | `structure` | Message | You've exceeded the daily capacity decrease limit for this reservation. |
| `CapacityReservationPendingException` | `structure` | Message | There is a pending capacity reservation. |
| `CapacityUnitsLimitExceededException` | `structure` | Message | You've exceeded the capacity units limit. |
| `CertificateNotFoundException` | `structure` | Message | The specified certificate does not exist. |
| `DeleteAssociationSameAccountException` | `structure` | Message | The specified association can't be within the same account. |
| `DuplicateListenerException` | `structure` | Message | A listener with the specified port already exists. |
| `DuplicateLoadBalancerNameException` | `structure` | Message | A load balancer with the specified name already exists. |
| `DuplicateTagKeysException` | `structure` | Message | A tag key was specified more than once. |
| `DuplicateTargetGroupNameException` | `structure` | Message | A target group with the specified name already exists. |
| `DuplicateTrustStoreNameException` | `structure` | Message | A trust store with the specified name already exists. |
| `HealthUnavailableException` | `structure` | Message | The health of the specified targets could not be retrieved due to an internal error. |
| `IncompatibleProtocolsException` | `structure` | Message | The specified configuration is not valid with this protocol. |
| `InsufficientCapacityException` | `structure` | Message | There is insufficient capacity to reserve. |
| `InvalidCaCertificatesBundleException` | `structure` | Message | The specified ca certificate bundle is in an invalid format, or corrupt. |
| `InvalidConfigurationRequestException` | `structure` | Message | The requested configuration is not valid. |
| `InvalidLoadBalancerActionException` | `structure` | Message | The requested action is not valid. |
| `InvalidRevocationContentException` | `structure` | Message | The provided revocation file is an invalid format, or uses an incorrect algorithm. |
| `InvalidSchemeException` | `structure` | Message | The requested scheme is not valid. |
| `InvalidSecurityGroupException` | `structure` | Message | The specified security group does not exist. |
| `InvalidSubnetException` | `structure` | Message | The specified subnet is out of available addresses. |
| `InvalidTargetException` | `structure` | Message | The specified target does not exist, is not in the same VPC as the target group, or has an unsupported instance type. |
| `ListenerNotFoundException` | `structure` | Message | The specified listener does not exist. |
| `LoadBalancerNotFoundException` | `structure` | Message | The specified load balancer does not exist. |
| `OperationNotPermittedException` | `structure` | Message | This operation is not allowed. |
| `PriorRequestNotCompleteException` | `structure` | Message | This operation is not allowed while a prior request has not been completed. |
| `PriorityInUseException` | `structure` | Message | The specified priority is in use. |
| `ResourceInUseException` | `structure` | Message | A specified resource is in use. |
| `ResourceNotFoundException` | `structure` | Message | The specified resource does not exist. |
| `RevocationContentNotFoundException` | `structure` | Message | The specified revocation file does not exist. |
| `RevocationIdNotFoundException` | `structure` | Message | The specified revocation ID does not exist. |
| `RuleNotFoundException` | `structure` | Message | The specified rule does not exist. |
| `SSLPolicyNotFoundException` | `structure` | Message | The specified SSL policy does not exist. |
| `SubnetNotFoundException` | `structure` | Message | The specified subnet does not exist. |
| `TargetGroupAssociationLimitException` | `structure` | Message | You've reached the limit on the number of load balancers per target group. |
| `TargetGroupNotFoundException` | `structure` | Message | The specified target group does not exist. |
| `TooManyActionsException` | `structure` | Message | You've reached the limit on the number of actions per rule. |
| `TooManyCertificatesException` | `structure` | Message | You've reached the limit on the number of certificates per load balancer. |
| `TooManyListenersException` | `structure` | Message | You've reached the limit on the number of listeners per load balancer. |
| `TooManyLoadBalancersException` | `structure` | Message | You've reached the limit on the number of load balancers for your Amazon Web Services account. |
| `TooManyRegistrationsForTargetIdException` | `structure` | Message | You've reached the limit on the number of times a target can be registered with a load balancer. |
| `TooManyRulesException` | `structure` | Message | You've reached the limit on the number of rules per load balancer. |
| `TooManyTagsException` | `structure` | Message | You've reached the limit on the number of tags for this resource. |
| `TooManyTargetGroupsException` | `structure` | Message | You've reached the limit on the number of target groups for your Amazon Web Services account. |
| `TooManyTargetsException` | `structure` | Message | You've reached the limit on the number of targets. |
| `TooManyTrustStoreRevocationEntriesException` | `structure` | Message | The specified trust store has too many revocation entries. |
| `TooManyTrustStoresException` | `structure` | Message | You've reached the limit on the number of trust stores for your Amazon Web Services account. |
| `TooManyUniqueTargetGroupsPerLoadBalancerException` | `structure` | Message | You've reached the limit on the number of unique target groups per load balancer across all listeners. If a target group is used by multiple actions for a l ... |
| `TrustStoreAssociationNotFoundException` | `structure` | Message | The specified association does not exist. |
| `TrustStoreInUseException` | `structure` | Message | The specified trust store is currently in use. |
| `TrustStoreNotFoundException` | `structure` | Message | The specified trust store does not exist. |
| `TrustStoreNotReadyException` | `structure` | Message | The specified trust store is not active. |
| `UnsupportedProtocolException` | `structure` | Message | The specified protocol is not supported. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
