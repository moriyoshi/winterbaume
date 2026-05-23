# Elastic Load Balancing

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Elastic Load Balancing A load balancer can distribute incoming traffic across your EC2 instances. This enables you to increase the availability of your application. The load balancer also monitors the health of its registered instances and ensures that it routes traffic only to healthy instances. You configure your load balancer to accept incoming traffic by specifying one or more listeners, which are configured with a protocol and port number for connections from clients to the load balancer and a protocol and port number for connections from the load balancer to the instances. Elastic Load Balancing supports three types of load balancers: Application Load Balancers, Network Load Balancers, and Classic Load Balancers.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Elastic Load Balancing where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Elastic Load Balancing by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Elastic Load Balancing workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Create`, `Delete`, `Set`, `Add` operation families, including `DescribeAccountLimits`, `DescribeInstanceHealth`, `DescribeLoadBalancerAttributes`, `DescribeLoadBalancerPolicies`, `CreateAppCookieStickinessPolicy`, `CreateLBCookieStickinessPolicy`.

## Service Identity and Protocol

- AWS model slug: `elastic-load-balancing`
- AWS SDK for Rust slug: `elasticloadbalancing`
- Model version: `2012-06-01`
- Model file: `vendor/api-models-aws/models/elastic-load-balancing/service/2012-06-01/elastic-load-balancing-2012-06-01.json`
- SDK ID: `Elastic Load Balancing`
- Endpoint prefix: `elasticloadbalancing`
- ARN namespace: `elasticloadbalancing`
- CloudFormation name: `ElasticLoadBalancing`
- CloudTrail event source: `elasticloadbalancing.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (7), `Create` (5), `Delete` (3), `Set` (3), `Add` (1), `Apply` (1), `Attach` (1), `Configure` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTags`, `AttachLoadBalancerToSubnets`, `CreateAppCookieStickinessPolicy`, `CreateLBCookieStickinessPolicy`, `CreateLoadBalancer`, `CreateLoadBalancerListeners`, `CreateLoadBalancerPolicy`, `DeleteLoadBalancer`, `DeleteLoadBalancerListeners`, `DeleteLoadBalancerPolicy`, `DeregisterInstancesFromLoadBalancer`, `DetachLoadBalancerFromSubnets`, `DisableAvailabilityZonesForLoadBalancer`, `EnableAvailabilityZonesForLoadBalancer`, `ModifyLoadBalancerAttributes`, `RegisterInstancesWithLoadBalancer`, `RemoveTags`, `SetLoadBalancerListenerSSLCertificate`, `SetLoadBalancerPoliciesForBackendServer`, `SetLoadBalancerPoliciesOfListener`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountLimits`, `DescribeInstanceHealth`, `DescribeLoadBalancerAttributes`, `DescribeLoadBalancerPolicies`, `DescribeLoadBalancerPolicyTypes`, `DescribeLoadBalancers`, `DescribeTags`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 27 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-healthchecks.html
- https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/ssl-server-cert.html
- https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/x-forwarded-headers.html

Research outcomes:
- Classic Load Balancers distribute traffic across registered EC2 instances and support listeners, health checks, policies, security groups, and zones.
- Health checks determine instance health and affect routing.
- SSL/TLS certificates for HTTPS/SSL listeners can come from ACM or IAM server certificates.
- Classic Load Balancers add X-Forwarded-For, X-Forwarded-Proto, and X-Forwarded-Port headers for HTTP/HTTPS requests.
- Listener and policy configuration controls protocol, port, backend protocol, SSL negotiation, stickiness, and proxy protocol behaviour.
- Classic Load Balancer semantics differ from Application, Network, and Gateway Load Balancers in ELBv2.

Parity implications:
- Model Classic Load Balancers separately from ELBv2 load balancers, listeners, target groups, and rules.
- Routing should depend on registered instance health and enabled Availability Zones.
- Listener policies, SSL certificates, and generated HTTP headers should be represented as Classic ELB-specific behaviour.

## v1/v2 State Coherence

- **Paired with `elasticloadbalancingv2` ( different SDK slug, different load balancer types ).** Classic Load Balancers ( this crate ) and Application/Network/Gateway Load Balancers ( ELBv2 ) are **separate resource types** in real AWS. A Classic ELB is not returned by ELBv2 `DescribeLoadBalancers`, and an ALB/NLB/GLB is not returned by Classic `DescribeLoadBalancers`. Listeners, target groups, and rules are ELBv2-only concepts; backend instance registration and listener policies are Classic-only.
- **Shared resources to be aware of:**
  - **Load balancer name namespace.** Real AWS reserves load balancer names in a single namespace within an account+region: a Classic ELB and an ALB cannot share the same name. Creating an ALB named `web` after creating a Classic ELB named `web` returns `DuplicateLoadBalancerName` in real AWS.
  - **Account-level limits.** `DescribeAccountLimits` reports limits that span both Classic and ELBv2 quota families ( e.g. total load balancers per region ).
  - **Tags.** Tags themselves are scoped per resource ARN, so they are correctly independent, but ARN format differs ( `arn:aws:elasticloadbalancing:…:loadbalancer/<name>` for Classic vs `arn:…:loadbalancer/<type>/<name>/<id>` for ELBv2 ).
- **Current Winterbaume status: separated, with one observable gap.** Each crate owns its own `load_balancers` map without cross-crate name-uniqueness checks. A consumer that creates a Classic ELB named `web` and then an ALB named `web` ( or vice versa ) will succeed in Winterbaume but fail in real AWS. Cross-crate name reservation is the realistic follow-up; the resource bodies themselves are correctly separate.

## Current Network Resource Stub Semantics

Classic ELB currently stores subnet and security group attachments on the load balancer record.

- `CreateLoadBalancer` records supplied subnets and security groups when present and otherwise uses local defaults.
- `AttachLoadBalancerToSubnets` appends requested subnet IDs to the stored list, `DetachLoadBalancerFromSubnets` removes them, and `ApplySecurityGroupsToLoadBalancer` replaces the stored security group list.
- Source security group and VPC fields are response metadata derived by the ELB crate, not from EC2 security group ownership.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccountLimits`, `DescribeInstanceHealth`, `DescribeLoadBalancerAttributes`, `DescribeLoadBalancerPolicies`, `DescribeLoadBalancerPolicyTypes`, `DescribeLoadBalancers`, `DescribeTags`
- Traits: `paginated` (1)
- Common required input members in this group: `LoadBalancerName`, `LoadBalancerNames`

### Create

- Operations: `CreateAppCookieStickinessPolicy`, `CreateLBCookieStickinessPolicy`, `CreateLoadBalancer`, `CreateLoadBalancerListeners`, `CreateLoadBalancerPolicy`
- Common required input members in this group: `CookieName`, `Listeners`, `LoadBalancerName`, `PolicyName`, `PolicyTypeName`

### Delete

- Operations: `DeleteLoadBalancer`, `DeleteLoadBalancerListeners`, `DeleteLoadBalancerPolicy`
- Common required input members in this group: `LoadBalancerName`, `LoadBalancerPorts`, `PolicyName`

### Set

- Operations: `SetLoadBalancerListenerSSLCertificate`, `SetLoadBalancerPoliciesForBackendServer`, `SetLoadBalancerPoliciesOfListener`
- Common required input members in this group: `InstancePort`, `LoadBalancerName`, `LoadBalancerPort`, `PolicyNames`, `SSLCertificateId`

### Add

- Operations: `AddTags`
- Common required input members in this group: `LoadBalancerNames`, `Tags`

### Apply

- Operations: `ApplySecurityGroupsToLoadBalancer`
- Common required input members in this group: `LoadBalancerName`, `SecurityGroups`

### Attach

- Operations: `AttachLoadBalancerToSubnets`
- Common required input members in this group: `LoadBalancerName`, `Subnets`

### Configure

- Operations: `ConfigureHealthCheck`
- Common required input members in this group: `HealthCheck`, `LoadBalancerName`

### Deregister

- Operations: `DeregisterInstancesFromLoadBalancer`
- Common required input members in this group: `Instances`, `LoadBalancerName`

### Detach

- Operations: `DetachLoadBalancerFromSubnets`
- Common required input members in this group: `LoadBalancerName`, `Subnets`

### Disable

- Operations: `DisableAvailabilityZonesForLoadBalancer`
- Common required input members in this group: `AvailabilityZones`, `LoadBalancerName`

### Enable

- Operations: `EnableAvailabilityZonesForLoadBalancer`
- Common required input members in this group: `AvailabilityZones`, `LoadBalancerName`

### Modify

- Operations: `ModifyLoadBalancerAttributes`
- Common required input members in this group: `LoadBalancerAttributes`, `LoadBalancerName`

### Register

- Operations: `RegisterInstancesWithLoadBalancer`
- Common required input members in this group: `Instances`, `LoadBalancerName`

### Remove

- Operations: `RemoveTags`
- Common required input members in this group: `LoadBalancerNames`, `Tags`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTags` | - | - | `LoadBalancerNames`, `Tags` | - | `AddTagsOutput` | `AccessPointNotFoundException`, `DuplicateTagKeysException`, `TooManyTagsException` | Adds the specified tags to the specified load balancer. Each load balancer can have a maximum of 10 tags. |
| `ApplySecurityGroupsToLoadBalancer` | - | - | `LoadBalancerName`, `SecurityGroups` | - | `ApplySecurityGroupsToLoadBalancerOutput` | `AccessPointNotFoundException`, `InvalidConfigurationRequestException`, `InvalidSecurityGroupException` | Associates one or more security groups with your load balancer in a virtual private cloud (VPC). The specified security groups override the previously associated security groups. |
| `AttachLoadBalancerToSubnets` | - | - | `LoadBalancerName`, `Subnets` | - | `AttachLoadBalancerToSubnetsOutput` | `AccessPointNotFoundException`, `InvalidConfigurationRequestException`, `InvalidSubnetException`, `SubnetNotFoundException` | Adds one or more subnets to the set of configured subnets for the specified load balancer. The load balancer evenly distributes requests across all registered subnets. |
| `ConfigureHealthCheck` | - | - | `HealthCheck`, `LoadBalancerName` | - | `ConfigureHealthCheckOutput` | `AccessPointNotFoundException` | Specifies the health check settings to use when evaluating the health state of your EC2 instances. For more information, see Configure Health Checks for Your Load Balancer in the Classic Load Balancers Guide . |
| `CreateAppCookieStickinessPolicy` | - | - | `CookieName`, `LoadBalancerName`, `PolicyName` | - | `CreateAppCookieStickinessPolicyOutput` | `AccessPointNotFoundException`, `DuplicatePolicyNameException`, `InvalidConfigurationRequestException`, `TooManyPoliciesException` | Generates a stickiness policy with sticky session lifetimes that follow that of an application-generated cookie. This policy can be associated only with HTTP/HTTPS listeners. |
| `CreateLBCookieStickinessPolicy` | - | - | `LoadBalancerName`, `PolicyName` | - | `CreateLBCookieStickinessPolicyOutput` | `AccessPointNotFoundException`, `DuplicatePolicyNameException`, `InvalidConfigurationRequestException`, `TooManyPoliciesException` | Generates a stickiness policy with sticky session lifetimes controlled by the lifetime of the browser (user-agent) or a specified expiration period. This policy can be associated only with HTTP/HTTPS listeners. |
| `CreateLoadBalancer` | - | - | `Listeners`, `LoadBalancerName` | - | `CreateAccessPointOutput` | `CertificateNotFoundException`, `DuplicateAccessPointNameException`, `DuplicateTagKeysException`, `InvalidConfigurationRequestException`, `InvalidSchemeException`, `InvalidSecurityGroupException`, `InvalidSubnetException`, `OperationNotPermittedException`, ... (+4) | Creates a Classic Load Balancer. You can add listeners, security groups, subnets, and tags when you create your load balancer, or you can add them later using CreateLoadBalancerListeners, ApplySecurityGroupsToLoadBalancer, AttachLoadBalancerToSubnets, and... |
| `CreateLoadBalancerListeners` | - | - | `Listeners`, `LoadBalancerName` | - | `CreateLoadBalancerListenerOutput` | `AccessPointNotFoundException`, `CertificateNotFoundException`, `DuplicateListenerException`, `InvalidConfigurationRequestException`, `UnsupportedProtocolException` | Creates one or more listeners for the specified load balancer. If a listener with the specified port does not already exist, it is created; otherwise, the properties of the new listener must match the properties of the existing listener. |
| `CreateLoadBalancerPolicy` | - | - | `LoadBalancerName`, `PolicyName`, `PolicyTypeName` | - | `CreateLoadBalancerPolicyOutput` | `AccessPointNotFoundException`, `DuplicatePolicyNameException`, `InvalidConfigurationRequestException`, `PolicyTypeNotFoundException`, `TooManyPoliciesException` | Creates a policy with the specified attributes for the specified load balancer. Policies are settings that are saved for your load balancer and that can be applied to the listener or the application server, depending on the policy type. |
| `DeleteLoadBalancer` | - | - | `LoadBalancerName` | - | `DeleteAccessPointOutput` | - | Deletes the specified load balancer. If you are attempting to recreate a load balancer, you must reconfigure all settings. |
| `DeleteLoadBalancerListeners` | - | - | `LoadBalancerName`, `LoadBalancerPorts` | - | `DeleteLoadBalancerListenerOutput` | `AccessPointNotFoundException` | Deletes the specified listeners from the specified load balancer. |
| `DeleteLoadBalancerPolicy` | - | - | `LoadBalancerName`, `PolicyName` | - | `DeleteLoadBalancerPolicyOutput` | `AccessPointNotFoundException`, `InvalidConfigurationRequestException` | Deletes the specified policy from the specified load balancer. This policy must not be enabled for any listeners. |
| `DeregisterInstancesFromLoadBalancer` | - | - | `Instances`, `LoadBalancerName` | - | `DeregisterEndPointsOutput` | `AccessPointNotFoundException`, `InvalidEndPointException` | Deregisters the specified instances from the specified load balancer. After the instance is deregistered, it no longer receives traffic from the load balancer. |
| `DescribeAccountLimits` | - | - | - | - | `DescribeAccountLimitsOutput` | - | Describes the current Elastic Load Balancing resource limits for your AWS account. For more information, see Limits for Your Classic Load Balancer in the Classic Load Balancers Guide . |
| `DescribeInstanceHealth` | - | - | `LoadBalancerName` | - | `DescribeEndPointStateOutput` | `AccessPointNotFoundException`, `InvalidEndPointException` | Describes the state of the specified instances with respect to the specified load balancer. If no instances are specified, the call describes the state of all instances that are currently registered with the load balancer. |
| `DescribeLoadBalancerAttributes` | - | - | `LoadBalancerName` | - | `DescribeLoadBalancerAttributesOutput` | `AccessPointNotFoundException`, `LoadBalancerAttributeNotFoundException` | Describes the attributes for the specified load balancer. |
| `DescribeLoadBalancerPolicies` | - | - | - | - | `DescribeLoadBalancerPoliciesOutput` | `AccessPointNotFoundException`, `PolicyNotFoundException` | Describes the specified policies. If you specify a load balancer name, the action returns the descriptions of all policies created for the load balancer. |
| `DescribeLoadBalancerPolicyTypes` | - | - | - | - | `DescribeLoadBalancerPolicyTypesOutput` | `PolicyTypeNotFoundException` | Describes the specified load balancer policy types or all load balancer policy types. The description of each type indicates how it can be used. |
| `DescribeLoadBalancers` | - | `paginated` | - | - | `DescribeAccessPointsOutput` | `AccessPointNotFoundException`, `DependencyThrottleException` | Describes the specified the load balancers. If no load balancers are specified, the call describes all of your load balancers. |
| `DescribeTags` | - | - | `LoadBalancerNames` | - | `DescribeTagsOutput` | `AccessPointNotFoundException` | Describes the tags associated with the specified load balancers. |
| `DetachLoadBalancerFromSubnets` | - | - | `LoadBalancerName`, `Subnets` | - | `DetachLoadBalancerFromSubnetsOutput` | `AccessPointNotFoundException`, `InvalidConfigurationRequestException` | Removes the specified subnets from the set of configured subnets for the load balancer. After a subnet is removed, all EC2 instances registered with the load balancer in the removed subnet go into the `OutOfService` state. |
| `DisableAvailabilityZonesForLoadBalancer` | - | - | `AvailabilityZones`, `LoadBalancerName` | - | `RemoveAvailabilityZonesOutput` | `AccessPointNotFoundException`, `InvalidConfigurationRequestException` | Removes the specified Availability Zones from the set of Availability Zones for the specified load balancer in EC2-Classic or a default VPC. For load balancers in a non-default VPC, use DetachLoadBalancerFromSubnets. |
| `EnableAvailabilityZonesForLoadBalancer` | - | - | `AvailabilityZones`, `LoadBalancerName` | - | `AddAvailabilityZonesOutput` | `AccessPointNotFoundException` | Adds the specified Availability Zones to the set of Availability Zones for the specified load balancer in EC2-Classic or a default VPC. For load balancers in a non-default VPC, use AttachLoadBalancerToSubnets. |
| `ModifyLoadBalancerAttributes` | - | - | `LoadBalancerAttributes`, `LoadBalancerName` | - | `ModifyLoadBalancerAttributesOutput` | `AccessPointNotFoundException`, `InvalidConfigurationRequestException`, `LoadBalancerAttributeNotFoundException` | Modifies the attributes of the specified load balancer. You can modify the load balancer attributes, such as `AccessLogs`, `ConnectionDraining`, and `CrossZoneLoadBalancing` by either enabling or disabling them. |
| `RegisterInstancesWithLoadBalancer` | - | - | `Instances`, `LoadBalancerName` | - | `RegisterEndPointsOutput` | `AccessPointNotFoundException`, `InvalidEndPointException` | Adds the specified instances to the specified load balancer. The instance must be a running instance in the same network as the load balancer (EC2-Classic or the same VPC). |
| `RemoveTags` | - | - | `LoadBalancerNames`, `Tags` | - | `RemoveTagsOutput` | `AccessPointNotFoundException` | Removes one or more tags from the specified load balancer. |
| `SetLoadBalancerListenerSSLCertificate` | - | - | `LoadBalancerName`, `LoadBalancerPort`, `SSLCertificateId` | - | `SetLoadBalancerListenerSSLCertificateOutput` | `AccessPointNotFoundException`, `CertificateNotFoundException`, `InvalidConfigurationRequestException`, `ListenerNotFoundException`, `UnsupportedProtocolException` | Sets the certificate that terminates the specified listener's SSL connections. The specified certificate replaces any prior certificate that was used on the same load balancer and port. |
| `SetLoadBalancerPoliciesForBackendServer` | - | - | `InstancePort`, `LoadBalancerName`, `PolicyNames` | - | `SetLoadBalancerPoliciesForBackendServerOutput` | `AccessPointNotFoundException`, `InvalidConfigurationRequestException`, `PolicyNotFoundException` | Replaces the set of policies associated with the specified port on which the EC2 instance is listening with a new set of policies. At this time, only the back-end server authentication policy type can be applied to the instance ports; this policy type is... |
| `SetLoadBalancerPoliciesOfListener` | - | - | `LoadBalancerName`, `LoadBalancerPort`, `PolicyNames` | - | `SetLoadBalancerPoliciesOfListenerOutput` | `AccessPointNotFoundException`, `InvalidConfigurationRequestException`, `ListenerNotFoundException`, `PolicyNotFoundException` | Replaces the current set of policies for the specified load balancer port with the specified set of policies. To enable back-end server authentication, use SetLoadBalancerPoliciesForBackendServer. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessPointNotFoundException` | `structure` | `Message` | The specified load balancer does not exist. |
| `InvalidConfigurationRequestException` | `structure` | `Message` | The requested configuration change is not valid. |
| `DuplicatePolicyNameException` | `structure` | `Message` | A policy with the specified name already exists for this load balancer. |
| `TooManyPoliciesException` | `structure` | `Message` | The quota for the number of policies for this load balancer has been reached. |
| `CertificateNotFoundException` | `structure` | `Message` | The specified ARN does not refer to a valid SSL certificate in AWS Identity and Access Management (IAM) or AWS Certificate Manager (ACM). |
| `UnsupportedProtocolException` | `structure` | `Message` | The specified protocol or signature version is not supported. |
| `InvalidEndPointException` | `structure` | `Message` | The specified endpoint is not valid. |
| `PolicyNotFoundException` | `structure` | `Message` | One or more of the specified policies do not exist. |
| `DuplicateTagKeysException` | `structure` | `Message` | A tag key was specified more than once. |
| `TooManyTagsException` | `structure` | `Message` | The quota for the number of tags that can be assigned to a load balancer has been reached. |
| `InvalidSecurityGroupException` | `structure` | `Message` | One or more of the specified security groups do not exist. |
| `InvalidSubnetException` | `structure` | `Message` | The specified VPC has no associated Internet gateway. |
| `SubnetNotFoundException` | `structure` | `Message` | One or more of the specified subnets do not exist. |
| `PolicyTypeNotFoundException` | `structure` | `Message` | One or more of the specified policy types do not exist. |
| `LoadBalancerAttributeNotFoundException` | `structure` | `Message` | The specified load balancer attribute does not exist. |
| `ListenerNotFoundException` | `structure` | `Message` | The load balancer does not have a listener configured at the specified port. |
| `AddTagsInput` | `structure` | `LoadBalancerNames`, `Tags` | Contains the parameters for AddTags. |
| `AddTagsOutput` | `structure` | - | Contains the output of AddTags. |
| `ApplySecurityGroupsToLoadBalancerInput` | `structure` | `LoadBalancerName`, `SecurityGroups` | Contains the parameters for ApplySecurityGroupsToLoadBalancer. |
| `ApplySecurityGroupsToLoadBalancerOutput` | `structure` | `SecurityGroups` | Contains the output of ApplySecurityGroupsToLoadBalancer. |
| `AttachLoadBalancerToSubnetsInput` | `structure` | `LoadBalancerName`, `Subnets` | Contains the parameters for AttachLoaBalancerToSubnets. |
| `AttachLoadBalancerToSubnetsOutput` | `structure` | `Subnets` | Contains the output of AttachLoadBalancerToSubnets. |
| `ConfigureHealthCheckInput` | `structure` | `HealthCheck`, `LoadBalancerName` | Contains the parameters for ConfigureHealthCheck. |
| `ConfigureHealthCheckOutput` | `structure` | `HealthCheck` | Contains the output of ConfigureHealthCheck. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
