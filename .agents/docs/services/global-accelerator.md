# AWS Global Accelerator

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Global Accelerator This is the Global Accelerator API Reference . This guide is for developers who need detailed information about Global Accelerator API actions, data types, and errors. For more information about Global Accelerator features, see the Global Accelerator Developer Guide. Global Accelerator is a service in which you create accelerators to improve the performance of your applications for local and global users. Depending on the type of accelerator you choose, you can gain additional benefits.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Global Accelerator where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Global Accelerator by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Global Accelerator workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Update`, `Create`, `Delete` operation families, including `ListAccelerators`, `ListByoipCidrs`, `ListCrossAccountAttachments`, `ListCrossAccountResourceAccounts`, `DescribeAccelerator`, `DescribeAcceleratorAttributes`.

## Service Identity and Protocol

- AWS model slug: `global-accelerator`
- AWS SDK for Rust slug: `globalaccelerator`
- Model version: `2018-08-08`
- Model file: `vendor/api-models-aws/models/global-accelerator/service/2018-08-08/global-accelerator-2018-08-08.json`
- SDK ID: `Global Accelerator`
- Endpoint prefix: `globalaccelerator`
- ARN namespace: `globalaccelerator`
- CloudFormation name: `GlobalAccelerator`
- CloudTrail event source: `globalaccelerator.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (13), `Describe` (9), `Update` (8), `Create` (7), `Delete` (7), `Add` (2), `Remove` (2), `Advertise` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddCustomRoutingEndpoints`, `AddEndpoints`, `CreateAccelerator`, `CreateCrossAccountAttachment`, `CreateCustomRoutingAccelerator`, `CreateCustomRoutingEndpointGroup`, `CreateCustomRoutingListener`, `CreateEndpointGroup`, `CreateListener`, `DeleteAccelerator`, `DeleteCrossAccountAttachment`, `DeleteCustomRoutingAccelerator`, `DeleteCustomRoutingEndpointGroup`, `DeleteCustomRoutingListener`, `DeleteEndpointGroup`, `DeleteListener`, `RemoveCustomRoutingEndpoints`, `RemoveEndpoints`, `TagResource`, `UntagResource`, ... (+8).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccelerator`, `DescribeAcceleratorAttributes`, `DescribeCrossAccountAttachment`, `DescribeCustomRoutingAccelerator`, `DescribeCustomRoutingAcceleratorAttributes`, `DescribeCustomRoutingEndpointGroup`, `DescribeCustomRoutingListener`, `DescribeEndpointGroup`, `DescribeListener`, `ListAccelerators`, `ListByoipCidrs`, `ListCrossAccountAttachments`, `ListCrossAccountResourceAccounts`, `ListCrossAccountResources`, `ListCustomRoutingAccelerators`, `ListCustomRoutingEndpointGroups`, `ListCustomRoutingListeners`, `ListCustomRoutingPortMappings`, `ListCustomRoutingPortMappingsByDestination`, `ListEndpointGroups`, ... (+2).
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 7 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 56 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/global-accelerator/latest/dg/what-is-global-accelerator.html
- https://docs.aws.amazon.com/global-accelerator/latest/dg/introduction-how-it-works.html
- https://docs.aws.amazon.com/global-accelerator/latest/dg/about-endpoints-endpoint-weights.unhealthy-endpoints.html

Research outcomes:
- Global Accelerator provides static anycast IP addresses that route user traffic over the AWS global network to healthy application endpoints.
- Standard accelerators use listeners, endpoint groups, and endpoints. Custom routing accelerators use deterministic port mappings.
- Health checks determine endpoint health and influence traffic routing.
- Traffic dials control what percentage of traffic is sent to an endpoint group, while endpoint weights distribute traffic among endpoints within the group.
- If endpoints are unhealthy, Global Accelerator fails over to healthy endpoints. Some failover paths ignore configured dial settings to restore availability.
- If no healthy endpoints exist, Global Accelerator can fail open and route traffic to endpoints anyway.
- Routing recovers after endpoint health improves, with documented recovery timing.

Parity implications:
- Model accelerators, static IPs, listeners, endpoint groups, endpoints, weights, traffic dials, health state, and custom routing mappings separately.
- Traffic routing should use health state, endpoint group dial, endpoint weights, and fail-open behaviour.
- Accelerator provisioning should expose asynchronous lifecycle state and assigned static IPs.

## Operation Groups

### List

- Operations: `ListAccelerators`, `ListByoipCidrs`, `ListCrossAccountAttachments`, `ListCrossAccountResourceAccounts`, `ListCrossAccountResources`, `ListCustomRoutingAccelerators`, `ListCustomRoutingEndpointGroups`, `ListCustomRoutingListeners`, `ListCustomRoutingPortMappings`, `ListCustomRoutingPortMappingsByDestination`, `ListEndpointGroups`, `ListListeners`, `ListTagsForResource`
- Traits: `paginated` (11)
- Common required input members in this group: `ListenerArn`, `AcceleratorArn`

### Describe

- Operations: `DescribeAccelerator`, `DescribeAcceleratorAttributes`, `DescribeCrossAccountAttachment`, `DescribeCustomRoutingAccelerator`, `DescribeCustomRoutingAcceleratorAttributes`, `DescribeCustomRoutingEndpointGroup`, `DescribeCustomRoutingListener`, `DescribeEndpointGroup`, `DescribeListener`
- Common required input members in this group: `AcceleratorArn`, `EndpointGroupArn`, `ListenerArn`

### Update

- Operations: `UpdateAccelerator`, `UpdateAcceleratorAttributes`, `UpdateCrossAccountAttachment`, `UpdateCustomRoutingAccelerator`, `UpdateCustomRoutingAcceleratorAttributes`, `UpdateCustomRoutingListener`, `UpdateEndpointGroup`, `UpdateListener`
- Common required input members in this group: `AcceleratorArn`, `ListenerArn`

### Create

- Operations: `CreateAccelerator`, `CreateCrossAccountAttachment`, `CreateCustomRoutingAccelerator`, `CreateCustomRoutingEndpointGroup`, `CreateCustomRoutingListener`, `CreateEndpointGroup`, `CreateListener`
- Traits: `idempotency-token` (7)
- Common required input members in this group: `Name`, `IdempotencyToken`, `ListenerArn`, `EndpointGroupRegion`, `AcceleratorArn`, `PortRanges`

### Delete

- Operations: `DeleteAccelerator`, `DeleteCrossAccountAttachment`, `DeleteCustomRoutingAccelerator`, `DeleteCustomRoutingEndpointGroup`, `DeleteCustomRoutingListener`, `DeleteEndpointGroup`, `DeleteListener`
- Common required input members in this group: `AcceleratorArn`, `EndpointGroupArn`, `ListenerArn`

### Add

- Operations: `AddCustomRoutingEndpoints`, `AddEndpoints`
- Common required input members in this group: `EndpointConfigurations`, `EndpointGroupArn`

### Remove

- Operations: `RemoveCustomRoutingEndpoints`, `RemoveEndpoints`
- Common required input members in this group: `EndpointGroupArn`

### Advertise

- Operations: `AdvertiseByoipCidr`
- Common required input members in this group: -

### Allow

- Operations: `AllowCustomRoutingTraffic`
- Common required input members in this group: -

### Deny

- Operations: `DenyCustomRoutingTraffic`
- Common required input members in this group: -

### Deprovision

- Operations: `DeprovisionByoipCidr`
- Common required input members in this group: -

### Provision

- Operations: `ProvisionByoipCidr`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Withdraw

- Operations: `WithdrawByoipCidr`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddCustomRoutingEndpoints` | `-` | - | `EndpointConfigurations`, `EndpointGroupArn` | - | `AddCustomRoutingEndpointsResponse` | `AccessDeniedException`, `ConflictException`, `EndpointAlreadyExistsException`, `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `LimitExceededException` | Associate a virtual private cloud (VPC) subnet endpoint with your custom routing accelerator. The listener port range must be large enough to support the number of IP addresses that can be specified in your subnet. T ... |
| `AddEndpoints` | `-` | - | `EndpointConfigurations`, `EndpointGroupArn` | - | `AddEndpointsResponse` | `AccessDeniedException`, `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `LimitExceededException`, `TransactionInProgressException` | Add endpoints to an endpoint group. The AddEndpoints API operation is the recommended option for adding endpoints. The alternative options are to add endpoints when you create an endpoint group (with the CreateEndpoi ... |
| `AdvertiseByoipCidr` | `-` | - | `Cidr` | - | `AdvertiseByoipCidrResponse` | `AccessDeniedException`, `ByoipCidrNotFoundException`, `IncorrectCidrStateException`, `InternalServiceErrorException`, `InvalidArgumentException` | Advertises an IPv4 address range that is provisioned for use with your Amazon Web Services resources through bring your own IP addresses (BYOIP). It can take a few minutes before traffic to the specified addresses st ... |
| `AllowCustomRoutingTraffic` | `-` | - | `EndpointGroupArn`, `EndpointId` | - | `Unit` | `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Specify the Amazon EC2 instance (destination) IP addresses and ports for a VPC subnet endpoint that can receive traffic for a custom routing accelerator. You can allow traffic to all destinations in the subnet endpoi ... |
| `CreateAccelerator` | `-` | `idempotency-token` | `Name`, `IdempotencyToken` | `IdempotencyToken` | `CreateAcceleratorResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidArgumentException`, `LimitExceededException`, `TransactionInProgressException` | Create an accelerator. An accelerator includes one or more listeners that process inbound connections and direct traffic to one or more endpoint groups, each of which includes endpoints, such as Network Load Balancer ... |
| `CreateCrossAccountAttachment` | `-` | `idempotency-token` | `Name`, `IdempotencyToken` | `IdempotencyToken` | `CreateCrossAccountAttachmentResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidArgumentException`, `LimitExceededException`, `TransactionInProgressException` | Create a cross-account attachment in Global Accelerator. You create a cross-account attachment to specify the principals who have permission to work with resources in accelerators in their own account. You specify, i ... |
| `CreateCustomRoutingAccelerator` | `-` | `idempotency-token` | `Name`, `IdempotencyToken` | `IdempotencyToken` | `CreateCustomRoutingAcceleratorResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidArgumentException`, `LimitExceededException`, `TransactionInProgressException` | Create a custom routing accelerator. A custom routing accelerator directs traffic to one of possibly thousands of Amazon EC2 instance destinations running in a single or multiple virtual private clouds (VPC) subnet e ... |
| `CreateCustomRoutingEndpointGroup` | `-` | `idempotency-token` | `ListenerArn`, `EndpointGroupRegion`, `DestinationConfigurations`, `IdempotencyToken` | `IdempotencyToken` | `CreateCustomRoutingEndpointGroupResponse` | `AcceleratorNotFoundException`, `AccessDeniedException`, `EndpointGroupAlreadyExistsException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidPortRangeException`, `LimitExceededException`, `ListenerNotFoundException` | Create an endpoint group for the specified listener for a custom routing accelerator. An endpoint group is a collection of endpoints in one Amazon Web Services Region. |
| `CreateCustomRoutingListener` | `-` | `idempotency-token` | `AcceleratorArn`, `PortRanges`, `IdempotencyToken` | `IdempotencyToken` | `CreateCustomRoutingListenerResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidPortRangeException`, `LimitExceededException` | Create a listener to process inbound connections from clients to a custom routing accelerator. Connections arrive to assigned static IP addresses on the port range that you specify. |
| `CreateEndpointGroup` | `-` | `idempotency-token` | `ListenerArn`, `EndpointGroupRegion`, `IdempotencyToken` | `IdempotencyToken` | `CreateEndpointGroupResponse` | `AcceleratorNotFoundException`, `AccessDeniedException`, `EndpointGroupAlreadyExistsException`, `InternalServiceErrorException`, `InvalidArgumentException`, `LimitExceededException`, `ListenerNotFoundException` | Create an endpoint group for the specified listener. An endpoint group is a collection of endpoints in one Amazon Web Services Region. A resource must be valid and active when you add it as an endpoint. For more info ... |
| `CreateListener` | `-` | `idempotency-token` | `AcceleratorArn`, `PortRanges`, `Protocol`, `IdempotencyToken` | `IdempotencyToken` | `CreateListenerResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidPortRangeException`, `LimitExceededException` | Create a listener to process inbound connections from clients to an accelerator. Connections arrive to assigned static IP addresses on a port, port range, or list of port ranges that you specify. |
| `DeleteAccelerator` | `-` | - | `AcceleratorArn` | - | `Unit` | `AcceleratorNotDisabledException`, `AcceleratorNotFoundException`, `AssociatedListenerFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `TransactionInProgressException` | Delete an accelerator. Before you can delete an accelerator, you must disable it and remove all dependent resources (listeners and endpoint groups). To disable the accelerator, update the accelerator to set Enabled t ... |
| `DeleteCrossAccountAttachment` | `-` | - | `AttachmentArn` | - | `Unit` | `AccessDeniedException`, `AttachmentNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `TransactionInProgressException` | Delete a cross-account attachment. When you delete an attachment, Global Accelerator revokes the permission to use the resources in the attachment from all principals in the list of principals. Global Accelerator rev ... |
| `DeleteCustomRoutingAccelerator` | `-` | - | `AcceleratorArn` | - | `Unit` | `AcceleratorNotDisabledException`, `AcceleratorNotFoundException`, `AssociatedListenerFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `TransactionInProgressException` | Delete a custom routing accelerator. Before you can delete an accelerator, you must disable it and remove all dependent resources (listeners and endpoint groups). To disable the accelerator, update the accelerator to ... |
| `DeleteCustomRoutingEndpointGroup` | `-` | - | `EndpointGroupArn` | - | `Unit` | `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Delete an endpoint group from a listener for a custom routing accelerator. |
| `DeleteCustomRoutingListener` | `-` | - | `ListenerArn` | - | `Unit` | `AssociatedEndpointGroupFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `ListenerNotFoundException` | Delete a listener for a custom routing accelerator. |
| `DeleteEndpointGroup` | `-` | - | `EndpointGroupArn` | - | `Unit` | `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Delete an endpoint group from a listener. |
| `DeleteListener` | `-` | - | `ListenerArn` | - | `Unit` | `AssociatedEndpointGroupFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `ListenerNotFoundException` | Delete a listener from an accelerator. |
| `DenyCustomRoutingTraffic` | `-` | - | `EndpointGroupArn`, `EndpointId` | - | `Unit` | `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Specify the Amazon EC2 instance (destination) IP addresses and ports for a VPC subnet endpoint that cannot receive traffic for a custom routing accelerator. You can deny traffic to all destinations in the VPC endpoin ... |
| `DeprovisionByoipCidr` | `-` | - | `Cidr` | - | `DeprovisionByoipCidrResponse` | `AccessDeniedException`, `ByoipCidrNotFoundException`, `IncorrectCidrStateException`, `InternalServiceErrorException`, `InvalidArgumentException` | Releases the specified address range that you provisioned to use with your Amazon Web Services resources through bring your own IP addresses (BYOIP) and deletes the corresponding address pool. Before you can release ... |
| `DescribeAccelerator` | `-` | - | `AcceleratorArn` | - | `DescribeAcceleratorResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Describe an accelerator. |
| `DescribeAcceleratorAttributes` | `-` | - | `AcceleratorArn` | - | `DescribeAcceleratorAttributesResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Describe the attributes of an accelerator. |
| `DescribeCrossAccountAttachment` | `-` | - | `AttachmentArn` | - | `DescribeCrossAccountAttachmentResponse` | `AccessDeniedException`, `AttachmentNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Gets configuration information about a cross-account attachment. |
| `DescribeCustomRoutingAccelerator` | `-` | - | `AcceleratorArn` | - | `DescribeCustomRoutingAcceleratorResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Describe a custom routing accelerator. |
| `DescribeCustomRoutingAcceleratorAttributes` | `-` | - | `AcceleratorArn` | - | `DescribeCustomRoutingAcceleratorAttributesResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Describe the attributes of a custom routing accelerator. |
| `DescribeCustomRoutingEndpointGroup` | `-` | - | `EndpointGroupArn` | - | `DescribeCustomRoutingEndpointGroupResponse` | `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Describe an endpoint group for a custom routing accelerator. |
| `DescribeCustomRoutingListener` | `-` | - | `ListenerArn` | - | `DescribeCustomRoutingListenerResponse` | `InternalServiceErrorException`, `InvalidArgumentException`, `ListenerNotFoundException` | The description of a listener for a custom routing accelerator. |
| `DescribeEndpointGroup` | `-` | - | `EndpointGroupArn` | - | `DescribeEndpointGroupResponse` | `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Describe an endpoint group. |
| `DescribeListener` | `-` | - | `ListenerArn` | - | `DescribeListenerResponse` | `InternalServiceErrorException`, `InvalidArgumentException`, `ListenerNotFoundException` | Describe a listener. |
| `ListAccelerators` | `-` | `paginated` | - | - | `ListAcceleratorsResponse` | `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException` | List the accelerators for an Amazon Web Services account. |
| `ListByoipCidrs` | `-` | `paginated` | - | - | `ListByoipCidrsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException` | Lists the IP address ranges that were specified in calls to ProvisionByoipCidr , including the current state and a history of state changes. |
| `ListCrossAccountAttachments` | `-` | `paginated` | - | - | `ListCrossAccountAttachmentsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException` | List the cross-account attachments that have been created in Global Accelerator. |
| `ListCrossAccountResourceAccounts` | `-` | - | - | - | `ListCrossAccountResourceAccountsResponse` | `AccessDeniedException`, `InternalServiceErrorException` | List the accounts that have cross-account resources. For more information, see Working with cross-account attachments and resources in Global Accelerator in the Global Accelerator Developer Guide . |
| `ListCrossAccountResources` | `-` | `paginated` | `ResourceOwnerAwsAccountId` | - | `ListCrossAccountResourcesResponse` | `AcceleratorNotFoundException`, `AccessDeniedException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException` | List the cross-account resources available to work with. |
| `ListCustomRoutingAccelerators` | `-` | `paginated` | - | - | `ListCustomRoutingAcceleratorsResponse` | `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException` | List the custom routing accelerators for an Amazon Web Services account. |
| `ListCustomRoutingEndpointGroups` | `-` | `paginated` | `ListenerArn` | - | `ListCustomRoutingEndpointGroupsResponse` | `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException`, `ListenerNotFoundException` | List the endpoint groups that are associated with a listener for a custom routing accelerator. |
| `ListCustomRoutingListeners` | `-` | `paginated` | `AcceleratorArn` | - | `ListCustomRoutingListenersResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException` | List the listeners for a custom routing accelerator. |
| `ListCustomRoutingPortMappings` | `-` | `paginated` | `AcceleratorArn` | - | `ListCustomRoutingPortMappingsResponse` | `AcceleratorNotFoundException`, `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException` | Provides a complete mapping from the public accelerator IP address and port to destination EC2 instance IP addresses and ports in the virtual public cloud (VPC) subnet endpoint for a custom routing accelerator. For e ... |
| `ListCustomRoutingPortMappingsByDestination` | `-` | `paginated` | `EndpointId`, `DestinationAddress` | - | `ListCustomRoutingPortMappingsByDestinationResponse` | `EndpointNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException` | List the port mappings for a specific EC2 instance (destination) in a VPC subnet endpoint. The response is the mappings for one destination IP address. This is useful when your subnet endpoint has mappings that span ... |
| `ListEndpointGroups` | `-` | `paginated` | `ListenerArn` | - | `ListEndpointGroupsResponse` | `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException`, `ListenerNotFoundException` | List the endpoint groups that are associated with a listener. |
| `ListListeners` | `-` | `paginated` | `AcceleratorArn` | - | `ListListenersResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidNextTokenException` | List the listeners for an accelerator. |
| `ListTagsForResource` | `-` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AcceleratorNotFoundException`, `AttachmentNotFoundException`, `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `ListenerNotFoundException` | List all tags for an accelerator. For more information, see Tagging in Global Accelerator in the Global Accelerator Developer Guide . |
| `ProvisionByoipCidr` | `-` | - | `Cidr`, `CidrAuthorizationContext` | - | `ProvisionByoipCidrResponse` | `AccessDeniedException`, `IncorrectCidrStateException`, `InternalServiceErrorException`, `InvalidArgumentException`, `LimitExceededException` | Provisions an IP address range to use with your Amazon Web Services resources through bring your own IP addresses (BYOIP) and creates a corresponding address pool. After the address range is provisioned, it is ready ... |
| `RemoveCustomRoutingEndpoints` | `-` | - | `EndpointIds`, `EndpointGroupArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `EndpointGroupNotFoundException`, `EndpointNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Remove endpoints from a custom routing accelerator. |
| `RemoveEndpoints` | `-` | - | `EndpointIdentifiers`, `EndpointGroupArn` | - | `Unit` | `AccessDeniedException`, `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `TransactionInProgressException` | Remove endpoints from an endpoint group. The RemoveEndpoints API operation is the recommended option for removing endpoints. The alternative is to remove endpoints by updating an endpoint group by using the UpdateEnd ... |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Add tags to an accelerator resource. For more information, see Tagging in Global Accelerator in the Global Accelerator Developer Guide . |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AcceleratorNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException` | Remove tags from a Global Accelerator resource. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from an accelerator tha ... |
| `UpdateAccelerator` | `-` | - | `AcceleratorArn` | - | `UpdateAcceleratorResponse` | `AcceleratorNotFoundException`, `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `InvalidArgumentException`, `TransactionInProgressException` | Update an accelerator to make changes, such as the following: Change the name of the accelerator. Disable the accelerator so that it no longer accepts or routes traffic, or so that you can delete it. Enable the accel ... |
| `UpdateAcceleratorAttributes` | `-` | - | `AcceleratorArn` | - | `UpdateAcceleratorAttributesResponse` | `AcceleratorNotFoundException`, `AccessDeniedException`, `InternalServiceErrorException`, `InvalidArgumentException`, `TransactionInProgressException` | Update the attributes for an accelerator. |
| `UpdateCrossAccountAttachment` | `-` | - | `AttachmentArn` | - | `UpdateCrossAccountAttachmentResponse` | `AccessDeniedException`, `AttachmentNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `LimitExceededException`, `TransactionInProgressException` | Update a cross-account attachment to add or remove principals or resources. When you update an attachment to remove a principal (account ID or accelerator) or a resource, Global Accelerator revokes the permission for ... |
| `UpdateCustomRoutingAccelerator` | `-` | - | `AcceleratorArn` | - | `UpdateCustomRoutingAcceleratorResponse` | `AcceleratorNotFoundException`, `ConflictException`, `InternalServiceErrorException`, `InvalidArgumentException`, `TransactionInProgressException` | Update a custom routing accelerator. |
| `UpdateCustomRoutingAcceleratorAttributes` | `-` | - | `AcceleratorArn` | - | `UpdateCustomRoutingAcceleratorAttributesResponse` | `AcceleratorNotFoundException`, `AccessDeniedException`, `InternalServiceErrorException`, `InvalidArgumentException`, `TransactionInProgressException` | Update the attributes for a custom routing accelerator. |
| `UpdateCustomRoutingListener` | `-` | - | `ListenerArn`, `PortRanges` | - | `UpdateCustomRoutingListenerResponse` | `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidPortRangeException`, `LimitExceededException`, `ListenerNotFoundException` | Update a listener for a custom routing accelerator. |
| `UpdateEndpointGroup` | `-` | - | `EndpointGroupArn` | - | `UpdateEndpointGroupResponse` | `AccessDeniedException`, `EndpointGroupNotFoundException`, `InternalServiceErrorException`, `InvalidArgumentException`, `LimitExceededException` | Update an endpoint group. A resource must be valid and active when you add it as an endpoint. |
| `UpdateListener` | `-` | - | `ListenerArn` | - | `UpdateListenerResponse` | `InternalServiceErrorException`, `InvalidArgumentException`, `InvalidPortRangeException`, `LimitExceededException`, `ListenerNotFoundException` | Update a listener. |
| `WithdrawByoipCidr` | `-` | - | `Cidr` | - | `WithdrawByoipCidrResponse` | `AccessDeniedException`, `ByoipCidrNotFoundException`, `IncorrectCidrStateException`, `InternalServiceErrorException`, `InvalidArgumentException` | Stops advertising an address range that is provisioned as an address pool. You can perform this operation at most once every 10 seconds, even if you specify different address ranges each time. It can take a few minut ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AcceleratorNotDisabledException` | `structure` | Message | The accelerator that you specified could not be disabled. |
| `AcceleratorNotFoundException` | `structure` | Message | The accelerator that you specified doesn't exist. |
| `AccessDeniedException` | `structure` | Message | You don't have access permission. |
| `AssociatedEndpointGroupFoundException` | `structure` | Message | The listener that you specified has an endpoint group associated with it. You must remove all dependent resources from a listener before you can delete it. |
| `AssociatedListenerFoundException` | `structure` | Message | The accelerator that you specified has a listener associated with it. You must remove all dependent resources from an accelerator before you can delete it. |
| `AttachmentNotFoundException` | `structure` | Message | No cross-account attachment was found. |
| `ByoipCidrNotFoundException` | `structure` | Message | The CIDR that you specified was not found or is incorrect. |
| `ConflictException` | `structure` | Message | You can't use both of those options. |
| `EndpointAlreadyExistsException` | `structure` | Message | The endpoint that you specified doesn't exist. |
| `EndpointGroupAlreadyExistsException` | `structure` | Message | The endpoint group that you specified already exists. |
| `EndpointGroupNotFoundException` | `structure` | Message | The endpoint group that you specified doesn't exist. |
| `EndpointNotFoundException` | `structure` | Message | The endpoint that you specified doesn't exist. |
| `IncorrectCidrStateException` | `structure` | Message | The CIDR that you specified is not valid for this action. For example, the state of the CIDR might be incorrect for this action. |
| `InternalServiceErrorException` | `structure` | Message | There was an internal error for Global Accelerator. |
| `InvalidArgumentException` | `structure` | Message | An argument that you specified is invalid. |
| `InvalidNextTokenException` | `structure` | Message | There isn't another item to return. |
| `InvalidPortRangeException` | `structure` | Message | The port numbers that you specified are not valid numbers or are not unique for this accelerator. |
| `LimitExceededException` | `structure` | Message | Processing your request would cause you to exceed an Global Accelerator limit. |
| `ListenerNotFoundException` | `structure` | Message | The listener that you specified doesn't exist. |
| `TransactionInProgressException` | `structure` | Message | There's already a transaction in progress. Another transaction can't be processed. |
| `AddCustomRoutingEndpointsRequest` | `structure` | EndpointConfigurations, EndpointGroupArn | - |
| `AddCustomRoutingEndpointsResponse` | `structure` | EndpointDescriptions, EndpointGroupArn | - |
| `AddEndpointsRequest` | `structure` | EndpointConfigurations, EndpointGroupArn | - |
| `AddEndpointsResponse` | `structure` | EndpointDescriptions, EndpointGroupArn | - |
| `AdvertiseByoipCidrRequest` | `structure` | Cidr | - |
| `AdvertiseByoipCidrResponse` | `structure` | ByoipCidr | - |
| `AllowCustomRoutingTrafficRequest` | `structure` | EndpointGroupArn, EndpointId, DestinationAddresses, DestinationPorts, AllowAllTrafficToEndpoint | - |
| `CreateAcceleratorRequest` | `structure` | Name, IpAddressType, IpAddresses, Enabled, IdempotencyToken, Tags | - |
| `CreateAcceleratorResponse` | `structure` | Accelerator | - |
| `CreateCrossAccountAttachmentRequest` | `structure` | Name, Principals, Resources, IdempotencyToken, Tags | - |
| `CreateCrossAccountAttachmentResponse` | `structure` | CrossAccountAttachment | - |
| `CreateCustomRoutingAcceleratorRequest` | `structure` | Name, IpAddressType, IpAddresses, Enabled, IdempotencyToken, Tags | - |
| `CreateCustomRoutingAcceleratorResponse` | `structure` | Accelerator | - |
| `CreateCustomRoutingEndpointGroupRequest` | `structure` | ListenerArn, EndpointGroupRegion, DestinationConfigurations, IdempotencyToken | - |
| `CreateCustomRoutingEndpointGroupResponse` | `structure` | EndpointGroup | - |
| `CreateCustomRoutingListenerRequest` | `structure` | AcceleratorArn, PortRanges, IdempotencyToken | - |
| `CreateCustomRoutingListenerResponse` | `structure` | Listener | - |
| `CreateEndpointGroupRequest` | `structure` | ListenerArn, EndpointGroupRegion, EndpointConfigurations, TrafficDialPercentage, HealthCheckPort, HealthCheckProtocol, HealthCheckPath, HealthCheckIntervalSeconds, ThresholdCount, IdempotencyToken, PortOverrides | - |
| `CreateEndpointGroupResponse` | `structure` | EndpointGroup | - |
| `CreateListenerRequest` | `structure` | AcceleratorArn, PortRanges, Protocol, ClientAffinity, IdempotencyToken | - |
| `AcceleratorStatus` | `enum` | DEPLOYED, IN_PROGRESS | - |
| `ByoipCidrState` | `enum` | PENDING_PROVISIONING, READY, PENDING_ADVERTISING, ADVERTISING, PENDING_WITHDRAWING, PENDING_DEPROVISIONING, DEPROVISIONED, FAILED_PROVISION, FAILED_ADVERTISING, FAILED_WITHDRAW, FAILED_DEPROVISION | - |
| `ClientAffinity` | `enum` | NONE, SOURCE_IP | - |
| `CustomRoutingAcceleratorStatus` | `enum` | DEPLOYED, IN_PROGRESS | - |
| `CustomRoutingDestinationTrafficState` | `enum` | ALLOW, DENY | - |
| `CustomRoutingProtocol` | `enum` | TCP, UDP | - |
| `HealthCheckProtocol` | `enum` | TCP, HTTP, HTTPS | - |
| `HealthState` | `enum` | INITIAL, HEALTHY, UNHEALTHY | - |
| `IpAddressFamily` | `enum` | IPv4, IPv6 | - |
| `IpAddressType` | `enum` | IPV4, DUAL_STACK | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
