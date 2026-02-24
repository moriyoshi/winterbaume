# AWS Network Firewall

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the API Reference for Network Firewall. This guide is for developers who need detailed information about the Network Firewall API actions, data types, and errors. The REST API requires you to handle connection details, such as calculating signatures, handling request retries, and error handling. For general information about using the Amazon Web Services REST APIs, see Amazon Web Services APIs. To view the complete list of Amazon Web Services Regions where Network Firewall is available, see Service endpoints and quotas in the Amazon Web Services General Reference .

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Network Firewall where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Network Firewall by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Network Firewall resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Network Firewall workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Update`, `Describe`, `List`, `Delete`, `Create` operation families, including `UpdateAvailabilityZoneChangeProtection`, `UpdateFirewallAnalysisSettings`, `UpdateFirewallDeleteProtection`, `UpdateFirewallDescription`, `DescribeFirewall`, `DescribeFirewallMetadata`.

## Service Identity and Protocol

- AWS model slug: `network-firewall`
- AWS SDK for Rust slug: `networkfirewall`
- Model version: `2020-11-12`
- Model file: `vendor/api-models-aws/models/network-firewall/service/2020-11-12/network-firewall-2020-11-12.json`
- SDK ID: `Network Firewall`
- Endpoint prefix: `network-firewall`
- ARN namespace: `network-firewall`
- CloudFormation name: `NetworkFirewall`
- CloudTrail event source: `networkfirewall.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Update` (16), `Describe` (15), `List` (12), `Delete` (11), `Create` (9), `Associate` (3), `Start` (3), `Disassociate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptNetworkFirewallTransitGatewayAttachment`, `AssociateAvailabilityZones`, `AssociateFirewallPolicy`, `AssociateSubnets`, `AttachRuleGroupsToProxyConfiguration`, `CreateFirewall`, `CreateFirewallPolicy`, `CreateProxy`, `CreateProxyConfiguration`, `CreateProxyRuleGroup`, `CreateProxyRules`, `CreateRuleGroup`, `CreateTLSInspectionConfiguration`, `CreateVpcEndpointAssociation`, `DeleteFirewall`, `DeleteFirewallPolicy`, `DeleteNetworkFirewallTransitGatewayAttachment`, `DeleteProxy`, `DeleteProxyConfiguration`, `DeleteProxyRuleGroup`, ... (+31).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeFirewall`, `DescribeFirewallMetadata`, `DescribeFirewallPolicy`, `DescribeFlowOperation`, `DescribeLoggingConfiguration`, `DescribeProxy`, `DescribeProxyConfiguration`, `DescribeProxyRule`, `DescribeProxyRuleGroup`, `DescribeResourcePolicy`, `DescribeRuleGroup`, `DescribeRuleGroupMetadata`, `DescribeRuleGroupSummary`, `DescribeTLSInspectionConfiguration`, `DescribeVpcEndpointAssociation`, `GetAnalysisReportResults`, `ListAnalysisReports`, `ListFirewallPolicies`, `ListFirewalls`, `ListFlowOperationResults`, ... (+8).
- Pagination is modelled for 13 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetAnalysisReportResults`, `ListAnalysisReports`, `StartAnalysisReport`, `StartFlowCapture`, `StartFlowFlush`, `UpdateFirewallAnalysisSettings`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 79 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/network-firewall/latest/developerguide/rule-groups.html
- https://docs.aws.amazon.com/network-firewall/latest/developerguide/firewall-updating.html
- https://docs.aws.amazon.com/network-firewall/latest/developerguide/stateful-rule-groups-domain-names.html

Research outcomes:
- Network Firewall uses firewalls, firewall policies, stateless rule groups, stateful rule groups, and logging configuration to inspect VPC traffic.
- Firewall endpoints are deployed in selected subnets and route-table configuration sends traffic through them.
- Stateless rule groups inspect packets and can pass, drop, forward to stateful inspection, or apply custom actions.
- Stateful rule groups inspect traffic flows and can use Suricata-compatible rules or domain list rules.
- Firewall policies combine stateless and stateful rule groups and default actions.
- Domain list rule groups inspect domain traffic such as SNI or HTTP host information and depend on HOME_NET settings for traffic from outside the deployment VPC.
- Rule group capacity and service quotas constrain firewall policy composition.

Parity implications:
- Model firewalls, firewall status, subnet mappings, policies, stateless/stateful rule groups, rule variables, logging, TLS inspection, and capacity separately.
- Firewall policy evaluation should preserve stateless first-stage action and stateful forwarding semantics.
- Update tokens and asynchronous firewall status should guard concurrent mutations.

## Current Network Resource Stub Semantics

Network Firewall currently stores firewall subnet attachments and VPC endpoint associations inside Network Firewall state.

- Firewall records keep the supplied VPC ID, subnet mappings, and subnet-change-protection flag.
- `AssociateSubnets` appends new subnet mappings when absent, and `DisassociateSubnets` removes matching subnet IDs from the local firewall record.
- VPC endpoint association records store a VPC ID and subnet ID and are listed from Network Firewall state only.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Update

- Operations: `UpdateAvailabilityZoneChangeProtection`, `UpdateFirewallAnalysisSettings`, `UpdateFirewallDeleteProtection`, `UpdateFirewallDescription`, `UpdateFirewallEncryptionConfiguration`, `UpdateFirewallPolicy`, `UpdateFirewallPolicyChangeProtection`, `UpdateLoggingConfiguration`, `UpdateProxy`, `UpdateProxyConfiguration`, `UpdateProxyRule`, `UpdateProxyRuleGroupPriorities`, `UpdateProxyRulePriorities`, `UpdateRuleGroup`, `UpdateSubnetChangeProtection`, `UpdateTLSInspectionConfiguration`
- Common required input members in this group: `AvailabilityZoneChangeProtection`, `DefaultRulePhaseActions`, `DeleteProtection`, `FirewallPolicy`, `FirewallPolicyChangeProtection`, `NatGatewayId`, `ProxyRuleName`, `RuleGroupRequestPhase`, `RuleGroups`, `Rules`, `SubnetChangeProtection`, `TLSInspectionConfiguration`, `UpdateToken`

### Describe

- Operations: `DescribeFirewall`, `DescribeFirewallMetadata`, `DescribeFirewallPolicy`, `DescribeFlowOperation`, `DescribeLoggingConfiguration`, `DescribeProxy`, `DescribeProxyConfiguration`, `DescribeProxyRule`, `DescribeProxyRuleGroup`, `DescribeResourcePolicy`, `DescribeRuleGroup`, `DescribeRuleGroupMetadata`, `DescribeRuleGroupSummary`, `DescribeTLSInspectionConfiguration`, `DescribeVpcEndpointAssociation`
- Common required input members in this group: `FirewallArn`, `FlowOperationId`, `ProxyRuleName`, `ResourceArn`, `VpcEndpointAssociationArn`

### List

- Operations: `ListAnalysisReports`, `ListFirewallPolicies`, `ListFirewalls`, `ListFlowOperationResults`, `ListFlowOperations`, `ListProxies`, `ListProxyConfigurations`, `ListProxyRuleGroups`, `ListRuleGroups`, `ListTLSInspectionConfigurations`, `ListTagsForResource`, `ListVpcEndpointAssociations`
- Traits: `paginated` (12)
- Common required input members in this group: `FirewallArn`, `FlowOperationId`, `ResourceArn`

### Delete

- Operations: `DeleteFirewall`, `DeleteFirewallPolicy`, `DeleteNetworkFirewallTransitGatewayAttachment`, `DeleteProxy`, `DeleteProxyConfiguration`, `DeleteProxyRuleGroup`, `DeleteProxyRules`, `DeleteResourcePolicy`, `DeleteRuleGroup`, `DeleteTLSInspectionConfiguration`, `DeleteVpcEndpointAssociation`
- Common required input members in this group: `NatGatewayId`, `ResourceArn`, `Rules`, `TransitGatewayAttachmentId`, `VpcEndpointAssociationArn`

### Create

- Operations: `CreateFirewall`, `CreateFirewallPolicy`, `CreateProxy`, `CreateProxyConfiguration`, `CreateProxyRuleGroup`, `CreateProxyRules`, `CreateRuleGroup`, `CreateTLSInspectionConfiguration`, `CreateVpcEndpointAssociation`
- Common required input members in this group: `Capacity`, `DefaultRulePhaseActions`, `FirewallArn`, `FirewallName`, `FirewallPolicy`, `FirewallPolicyArn`, `FirewallPolicyName`, `NatGatewayId`, `ProxyConfigurationName`, `ProxyName`, `ProxyRuleGroupName`, `RuleGroupName`, `Rules`, `SubnetMapping`, `TLSInspectionConfiguration`, `TLSInspectionConfigurationName`, `TlsInterceptProperties`, `Type`, `VpcId`

### Associate

- Operations: `AssociateAvailabilityZones`, `AssociateFirewallPolicy`, `AssociateSubnets`
- Common required input members in this group: `AvailabilityZoneMappings`, `FirewallPolicyArn`, `SubnetMappings`

### Start

- Operations: `StartAnalysisReport`, `StartFlowCapture`, `StartFlowFlush`
- Common required input members in this group: `AnalysisType`, `FirewallArn`, `FlowFilters`

### Disassociate

- Operations: `DisassociateAvailabilityZones`, `DisassociateSubnets`
- Common required input members in this group: `AvailabilityZoneMappings`, `SubnetIds`

### Accept

- Operations: `AcceptNetworkFirewallTransitGatewayAttachment`
- Common required input members in this group: `TransitGatewayAttachmentId`

### Attach

- Operations: `AttachRuleGroupsToProxyConfiguration`
- Common required input members in this group: `RuleGroups`, `UpdateToken`

### Detach

- Operations: `DetachRuleGroupsFromProxyConfiguration`
- Common required input members in this group: `UpdateToken`

### Get

- Operations: `GetAnalysisReportResults`
- Traits: `paginated` (1)
- Common required input members in this group: `AnalysisReportId`

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: `Policy`, `ResourceArn`

### Reject

- Operations: `RejectNetworkFirewallTransitGatewayAttachment`
- Common required input members in this group: `TransitGatewayAttachmentId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptNetworkFirewallTransitGatewayAttachment` | - | - | `TransitGatewayAttachmentId` | - | `AcceptNetworkFirewallTransitGatewayAttachmentResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Accepts a transit gateway attachment request for Network Firewall. When you accept the attachment request, Network Firewall creates the necessary routing components to enable traffic flow between the transit gateway and firewall endpoints. |
| `AssociateAvailabilityZones` | - | - | `AvailabilityZoneMappings` | - | `AssociateAvailabilityZonesResponse` | `InsufficientCapacityException`, `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Associates the specified Availability Zones with a transit gateway-attached firewall. For each Availability Zone, Network Firewall creates a firewall endpoint to process traffic. |
| `AssociateFirewallPolicy` | - | - | `FirewallPolicyArn` | - | `AssociateFirewallPolicyResponse` | `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Associates a FirewallPolicy to a Firewall. A firewall policy defines how to monitor and manage your VPC network traffic, using a collection of inspection rule groups and other settings. |
| `AssociateSubnets` | - | - | `SubnetMappings` | - | `AssociateSubnetsResponse` | `InsufficientCapacityException`, `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Associates the specified subnets in the Amazon VPC to the firewall. You can specify one subnet for each of the Availability Zones that the VPC spans. |
| `AttachRuleGroupsToProxyConfiguration` | - | - | `RuleGroups`, `UpdateToken` | - | `AttachRuleGroupsToProxyConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Attaches ProxyRuleGroup resources to a ProxyConfiguration A Proxy Configuration defines the monitoring and protection behavior for a Proxy. The details of the behavior are defined in the rule groups that you add to your configuration. |
| `CreateFirewall` | - | - | `FirewallName`, `FirewallPolicyArn` | - | `CreateFirewallResponse` | `InsufficientCapacityException`, `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `LimitExceededException`, `ThrottlingException` | Creates an Network Firewall Firewall and accompanying FirewallStatus for a VPC. The firewall defines the configuration settings for an Network Firewall firewall. |
| `CreateFirewallPolicy` | - | - | `FirewallPolicy`, `FirewallPolicyName` | - | `CreateFirewallPolicyResponse` | `InsufficientCapacityException`, `InternalServerError`, `InvalidRequestException`, `LimitExceededException`, `ThrottlingException` | Creates the firewall policy for the firewall according to the specifications. An Network Firewall firewall policy defines the behavior of a firewall, in a collection of stateless and stateful rule groups and other settings. |
| `CreateProxy` | - | - | `NatGatewayId`, `ProxyName`, `TlsInterceptProperties` | - | `CreateProxyResponse` | `InternalServerError`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `UnsupportedOperationException` | Creates an Network Firewall Proxy Attaches a Proxy configuration to a NAT Gateway. To manage a proxy's tags, use the standard Amazon Web Services resource tagging operations, ListTagsForResource, TagResource, and UntagResource. |
| `CreateProxyConfiguration` | - | - | `DefaultRulePhaseActions`, `ProxyConfigurationName` | - | `CreateProxyConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Creates an Network Firewall ProxyConfiguration A Proxy Configuration defines the monitoring and protection behavior for a Proxy. The details of the behavior are defined in the rule groups that you add to your configuration. |
| `CreateProxyRuleGroup` | - | - | `ProxyRuleGroupName` | - | `CreateProxyRuleGroupResponse` | `InternalServerError`, `InvalidRequestException`, `LimitExceededException`, `ThrottlingException` | Creates an Network Firewall ProxyRuleGroup Collections of related proxy filtering rules. Rule groups help you manage and reuse sets of rules across multiple proxy configurations. |
| `CreateProxyRules` | - | - | `Rules` | - | `CreateProxyRulesResponse` | `InternalServerError`, `InvalidRequestException`, `ThrottlingException` | Creates Network Firewall ProxyRule resources. Attaches new proxy rule(s) to an existing proxy rule group. |
| `CreateRuleGroup` | - | - | `Capacity`, `RuleGroupName`, `Type` | - | `CreateRuleGroupResponse` | `InsufficientCapacityException`, `InternalServerError`, `InvalidRequestException`, `LimitExceededException`, `ThrottlingException` | Creates the specified stateless or stateful rule group, which includes the rules for network traffic inspection, a capacity setting, and tags. You provide your rule group specification in your request using either `RuleGroup` or `Rules`. |
| `CreateTLSInspectionConfiguration` | - | - | `TLSInspectionConfiguration`, `TLSInspectionConfigurationName` | - | `CreateTLSInspectionConfigurationResponse` | `InsufficientCapacityException`, `InternalServerError`, `InvalidRequestException`, `LimitExceededException`, `ThrottlingException` | Creates an Network Firewall TLS inspection configuration. Network Firewall uses TLS inspection configurations to decrypt your firewall's inbound and outbound SSL/TLS traffic. |
| `CreateVpcEndpointAssociation` | - | - | `FirewallArn`, `SubnetMapping`, `VpcId` | - | `CreateVpcEndpointAssociationResponse` | `InsufficientCapacityException`, `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a firewall endpoint for an Network Firewall firewall. This type of firewall endpoint is independent of the firewall endpoints that you specify in the `Firewall` itself, and you define it in addition to those endpoints after the firewall has been... |
| `DeleteFirewall` | - | - | - | - | `DeleteFirewallResponse` | `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnsupportedOperationException` | Deletes the specified Firewall and its FirewallStatus. This operation requires the firewall's `DeleteProtection` flag to be `FALSE`. |
| `DeleteFirewallPolicy` | - | - | - | - | `DeleteFirewallPolicyResponse` | `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnsupportedOperationException` | Deletes the specified FirewallPolicy. |
| `DeleteNetworkFirewallTransitGatewayAttachment` | - | - | `TransitGatewayAttachmentId` | - | `DeleteNetworkFirewallTransitGatewayAttachmentResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a transit gateway attachment from a Network Firewall. Either the firewall owner or the transit gateway owner can delete the attachment. |
| `DeleteProxy` | - | - | `NatGatewayId` | - | `DeleteProxyResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnsupportedOperationException` | Deletes the specified Proxy. Detaches a Proxy configuration from a NAT Gateway. |
| `DeleteProxyConfiguration` | - | - | - | - | `DeleteProxyConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified ProxyConfiguration. |
| `DeleteProxyRuleGroup` | - | - | - | - | `DeleteProxyRuleGroupResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified ProxyRuleGroup. |
| `DeleteProxyRules` | - | - | `Rules` | - | `DeleteProxyRulesResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified ProxyRule(s). currently attached to a ProxyRuleGroup |
| `DeleteResourcePolicy` | - | - | `ResourceArn` | - | `DeleteResourcePolicyResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidResourcePolicyException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a resource policy that you created in a PutResourcePolicy request. |
| `DeleteRuleGroup` | - | - | - | - | `DeleteRuleGroupResponse` | `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnsupportedOperationException` | Deletes the specified RuleGroup. |
| `DeleteTLSInspectionConfiguration` | - | - | - | - | `DeleteTLSInspectionConfigurationResponse` | `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified TLSInspectionConfiguration. |
| `DeleteVpcEndpointAssociation` | - | - | `VpcEndpointAssociationArn` | - | `DeleteVpcEndpointAssociationResponse` | `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified VpcEndpointAssociation. You can check whether an endpoint association is in use by reviewing the route tables for the Availability Zones where you have the endpoint subnet mapping. |
| `DescribeFirewall` | - | - | - | - | `DescribeFirewallResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the data objects for the specified firewall. |
| `DescribeFirewallMetadata` | - | - | - | - | `DescribeFirewallMetadataResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the high-level information about a firewall, including the Availability Zones where the Firewall is currently in use. |
| `DescribeFirewallPolicy` | - | - | - | - | `DescribeFirewallPolicyResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the data objects for the specified firewall policy. |
| `DescribeFlowOperation` | - | - | `FirewallArn`, `FlowOperationId` | - | `DescribeFlowOperationResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns key information about a specific flow operation. |
| `DescribeLoggingConfiguration` | - | - | - | - | `DescribeLoggingConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the logging configuration for the specified firewall. |
| `DescribeProxy` | - | - | - | - | `DescribeProxyResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the data objects for the specified proxy. |
| `DescribeProxyConfiguration` | - | - | - | - | `DescribeProxyConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the data objects for the specified proxy configuration. |
| `DescribeProxyRule` | - | - | `ProxyRuleName` | - | `DescribeProxyRuleResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the data objects for the specified proxy configuration for the specified proxy rule group. |
| `DescribeProxyRuleGroup` | - | - | - | - | `DescribeProxyRuleGroupResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the data objects for the specified proxy rule group. |
| `DescribeResourcePolicy` | - | - | `ResourceArn` | - | `DescribeResourcePolicyResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a resource policy that you created in a PutResourcePolicy request. |
| `DescribeRuleGroup` | - | - | - | - | `DescribeRuleGroupResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the data objects for the specified rule group. |
| `DescribeRuleGroupMetadata` | - | - | - | - | `DescribeRuleGroupMetadataResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | High-level information about a rule group, returned by operations like create and describe. You can use the information provided in the metadata to retrieve and manage a rule group. |
| `DescribeRuleGroupSummary` | - | - | - | - | `DescribeRuleGroupSummaryResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information for a stateful rule group. For active threat defense Amazon Web Services managed rule groups, this operation provides insight into the protections enabled by the rule group, based on Suricata rule metadata fields. |
| `DescribeTLSInspectionConfiguration` | - | - | - | - | `DescribeTLSInspectionConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the data objects for the specified TLS inspection configuration. |
| `DescribeVpcEndpointAssociation` | - | - | `VpcEndpointAssociationArn` | - | `DescribeVpcEndpointAssociationResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the data object for the specified VPC endpoint association. |
| `DetachRuleGroupsFromProxyConfiguration` | - | - | `UpdateToken` | - | `DetachRuleGroupsFromProxyConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Detaches ProxyRuleGroup resources from a ProxyConfiguration A Proxy Configuration defines the monitoring and protection behavior for a Proxy. The details of the behavior are defined in the rule groups that you add to your configuration. |
| `DisassociateAvailabilityZones` | - | - | `AvailabilityZoneMappings` | - | `DisassociateAvailabilityZonesResponse` | `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Removes the specified Availability Zone associations from a transit gateway-attached firewall. This removes the firewall endpoints from these Availability Zones and stops traffic filtering in those zones. |
| `DisassociateSubnets` | - | - | `SubnetIds` | - | `DisassociateSubnetsResponse` | `InternalServerError`, `InvalidOperationException`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Removes the specified subnet associations from the firewall. This removes the firewall endpoints from the subnets and removes any network filtering protections that the endpoints were providing. |
| `GetAnalysisReportResults` | - | `paginated` | `AnalysisReportId` | - | `GetAnalysisReportResultsResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | The results of a `COMPLETED` analysis report generated with StartAnalysisReport. For more information, see AnalysisTypeReportResult. |
| `ListAnalysisReports` | - | `paginated` | - | - | `ListAnalysisReportsResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of all traffic analysis reports generated within the last 30 days. |
| `ListFirewallPolicies` | - | `paginated` | - | - | `ListFirewallPoliciesResponse` | `InternalServerError`, `InvalidRequestException`, `ThrottlingException` | Retrieves the metadata for the firewall policies that you have defined. Depending on your setting for max results and the number of firewall policies, a single call might not return the full list. |
| `ListFirewalls` | - | `paginated` | - | - | `ListFirewallsResponse` | `InternalServerError`, `InvalidRequestException`, `ThrottlingException` | Retrieves the metadata for the firewalls that you have defined. If you provide VPC identifiers in your request, this returns only the firewalls for those VPCs. |
| `ListFlowOperationResults` | - | `paginated` | `FirewallArn`, `FlowOperationId` | - | `ListFlowOperationResultsResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the results of a specific flow operation. Flow operations let you manage the flows tracked in the flow table, also known as the firewall table. |
| `ListFlowOperations` | - | `paginated` | `FirewallArn` | - | `ListFlowOperationsResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of all flow operations ran in a specific firewall. You can optionally narrow the request scope by specifying the operation type or Availability Zone associated with a firewall's flow operations. |
| `ListProxies` | - | `paginated` | - | - | `ListProxiesResponse` | `InternalServerError`, `InvalidRequestException`, `ThrottlingException` | Retrieves the metadata for the proxies that you have defined. Depending on your setting for max results and the number of proxies, a single call might not return the full list. |
| `ListProxyConfigurations` | - | `paginated` | - | - | `ListProxyConfigurationsResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the metadata for the proxy configuration that you have defined. Depending on your setting for max results and the number of proxy configurations, a single call might not return the full list. |
| `ListProxyRuleGroups` | - | `paginated` | - | - | `ListProxyRuleGroupsResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the metadata for the proxy rule groups that you have defined. Depending on your setting for max results and the number of proxy rule groups, a single call might not return the full list. |
| `ListRuleGroups` | - | `paginated` | - | - | `ListRuleGroupsResponse` | `InternalServerError`, `InvalidRequestException`, `ThrottlingException` | Retrieves the metadata for the rule groups that you have defined. Depending on your setting for max results and the number of rule groups, a single call might not return the full list. |
| `ListTLSInspectionConfigurations` | - | `paginated` | - | - | `ListTLSInspectionConfigurationsResponse` | `InternalServerError`, `InvalidRequestException`, `ThrottlingException` | Retrieves the metadata for the TLS inspection configurations that you have defined. Depending on your setting for max results and the number of TLS inspection configurations, a single call might not return the full list. |
| `ListTagsForResource` | - | `paginated` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the tags associated with the specified resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. |
| `ListVpcEndpointAssociations` | - | `paginated` | - | - | `ListVpcEndpointAssociationsResponse` | `InternalServerError`, `InvalidRequestException`, `ThrottlingException` | Retrieves the metadata for the VPC endpoint associations that you have defined. If you specify a fireawll, this returns only the endpoint associations for that firewall. |
| `PutResourcePolicy` | - | - | `Policy`, `ResourceArn` | - | `PutResourcePolicyResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidResourcePolicyException`, `ResourceNotFoundException`, `ThrottlingException` | Creates or updates an IAM policy for your rule group, firewall policy, or firewall. Use this to share these resources between accounts. |
| `RejectNetworkFirewallTransitGatewayAttachment` | - | - | `TransitGatewayAttachmentId` | - | `RejectNetworkFirewallTransitGatewayAttachmentResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Rejects a transit gateway attachment request for Network Firewall. When you reject the attachment request, Network Firewall cancels the creation of routing components between the transit gateway and firewall endpoints. |
| `StartAnalysisReport` | - | - | `AnalysisType` | - | `StartAnalysisReportResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Generates a traffic analysis report for the timeframe and traffic type you specify. For information on the contents of a traffic analysis report, see AnalysisReport. |
| `StartFlowCapture` | - | - | `FirewallArn`, `FlowFilters` | - | `StartFlowCaptureResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Begins capturing the flows in a firewall, according to the filters you define. Captures are similar, but not identical to snapshots. |
| `StartFlowFlush` | - | - | `FirewallArn`, `FlowFilters` | - | `StartFlowFlushResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Begins the flushing of traffic from the firewall, according to the filters you define. When the operation starts, impacted flows are temporarily marked as timed out before the Suricata engine prunes, or flushes, the flows from the firewall table. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Adds the specified tags to the specified resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Removes the tags with the specified keys from the specified resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. |
| `UpdateAvailabilityZoneChangeProtection` | - | - | `AvailabilityZoneChangeProtection` | - | `UpdateAvailabilityZoneChangeProtectionResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ResourceOwnerCheckException`, `ThrottlingException` | Modifies the `AvailabilityZoneChangeProtection` setting for a transit gateway-attached firewall. When enabled, this setting prevents accidental changes to the firewall's Availability Zone configuration. |
| `UpdateFirewallAnalysisSettings` | - | - | - | - | `UpdateFirewallAnalysisSettingsResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Enables specific types of firewall analysis on a specific firewall you define. |
| `UpdateFirewallDeleteProtection` | - | - | `DeleteProtection` | - | `UpdateFirewallDeleteProtectionResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ResourceOwnerCheckException`, `ThrottlingException` | Modifies the flag, `DeleteProtection`, which indicates whether it is possible to delete the firewall. If the flag is set to `TRUE`, the firewall is protected against deletion. |
| `UpdateFirewallDescription` | - | - | - | - | `UpdateFirewallDescriptionResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Modifies the description for the specified firewall. Use the description to help you identify the firewall when you're working with it. |
| `UpdateFirewallEncryptionConfiguration` | - | - | - | - | `UpdateFirewallEncryptionConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ResourceOwnerCheckException`, `ThrottlingException` | A complex type that contains settings for encryption of your firewall resources. |
| `UpdateFirewallPolicy` | - | - | `FirewallPolicy`, `UpdateToken` | - | `UpdateFirewallPolicyResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the properties of the specified firewall policy. |
| `UpdateFirewallPolicyChangeProtection` | - | - | `FirewallPolicyChangeProtection` | - | `UpdateFirewallPolicyChangeProtectionResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ResourceOwnerCheckException`, `ThrottlingException` | Modifies the flag, `ChangeProtection`, which indicates whether it is possible to change the firewall. If the flag is set to `TRUE`, the firewall is protected from changes. |
| `UpdateLoggingConfiguration` | - | - | - | - | `UpdateLoggingConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `LogDestinationPermissionException`, `ResourceNotFoundException`, `ThrottlingException` | Sets the logging configuration for the specified firewall. To change the logging configuration, retrieve the LoggingConfiguration by calling DescribeLoggingConfiguration, then change it and provide the modified object to this update call. |
| `UpdateProxy` | - | - | `NatGatewayId`, `UpdateToken` | - | `UpdateProxyResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnsupportedOperationException` | Updates the properties of the specified proxy. |
| `UpdateProxyConfiguration` | - | - | `DefaultRulePhaseActions`, `UpdateToken` | - | `UpdateProxyConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the properties of the specified proxy configuration. |
| `UpdateProxyRule` | - | - | `ProxyRuleName`, `UpdateToken` | - | `UpdateProxyRuleResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the properties of the specified proxy rule. |
| `UpdateProxyRuleGroupPriorities` | - | - | `RuleGroups`, `UpdateToken` | - | `UpdateProxyRuleGroupPrioritiesResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates proxy rule group priorities within a proxy configuration. |
| `UpdateProxyRulePriorities` | - | - | `RuleGroupRequestPhase`, `Rules`, `UpdateToken` | - | `UpdateProxyRulePrioritiesResponse` | `InternalServerError`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates proxy rule priorities within a proxy rule group. |
| `UpdateRuleGroup` | - | - | `UpdateToken` | - | `UpdateRuleGroupResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the rule settings for the specified rule group. You use a rule group by reference in one or more firewall policies. |
| `UpdateSubnetChangeProtection` | - | - | `SubnetChangeProtection` | - | `UpdateSubnetChangeProtectionResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ResourceOwnerCheckException`, `ThrottlingException` | - |
| `UpdateTLSInspectionConfiguration` | - | - | `TLSInspectionConfiguration`, `UpdateToken` | - | `UpdateTLSInspectionConfigurationResponse` | `InternalServerError`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the TLS inspection configuration settings for the specified TLS inspection configuration. You use a TLS inspection configuration by referencing it in one or more firewall policies. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerError` | `structure` | `Message` | Your request is valid, but Network Firewall couldn't perform the operation because of a system problem. |
| `InvalidRequestException` | `structure` | `Message` | The operation failed because of a problem with your request. |
| `ThrottlingException` | `structure` | `Message` | Unable to process the request due to throttling limitations. |
| `ResourceNotFoundException` | `structure` | `Message` | Unable to locate a resource using the parameters that you provided. |
| `InvalidTokenException` | `structure` | `Message` | The token you provided is stale or isn't valid for the operation. |
| `InvalidOperationException` | `structure` | `Message` | The operation failed because it's not valid. |
| `LimitExceededException` | `structure` | `Message` | Unable to perform the operation because doing so would violate a limit setting. |
| `InsufficientCapacityException` | `structure` | `Message` | Amazon Web Services doesn't currently have enough available capacity to fulfill your request. |
| `UnsupportedOperationException` | `structure` | `Message` | The operation you requested isn't supported by Network Firewall. |
| `ResourceOwnerCheckException` | `structure` | `Message` | Unable to change the resource because your account doesn't own it. |
| `InvalidResourcePolicyException` | `structure` | `Message` | The policy statement failed validation. |
| `AcceptNetworkFirewallTransitGatewayAttachmentRequest` | `structure` | `TransitGatewayAttachmentId` | - |
| `AcceptNetworkFirewallTransitGatewayAttachmentResponse` | `structure` | `TransitGatewayAttachmentId`, `TransitGatewayAttachmentStatus` | - |
| `AssociateAvailabilityZonesRequest` | `structure` | `AvailabilityZoneMappings`, `FirewallArn`, `FirewallName`, `UpdateToken` | - |
| `AssociateAvailabilityZonesResponse` | `structure` | `AvailabilityZoneMappings`, `FirewallArn`, `FirewallName`, `UpdateToken` | - |
| `AssociateFirewallPolicyRequest` | `structure` | `FirewallArn`, `FirewallName`, `FirewallPolicyArn`, `UpdateToken` | - |
| `AssociateFirewallPolicyResponse` | `structure` | `FirewallArn`, `FirewallName`, `FirewallPolicyArn`, `UpdateToken` | - |
| `AssociateSubnetsRequest` | `structure` | `FirewallArn`, `FirewallName`, `SubnetMappings`, `UpdateToken` | - |
| `AssociateSubnetsResponse` | `structure` | `FirewallArn`, `FirewallName`, `SubnetMappings`, `UpdateToken` | - |
| `AttachRuleGroupsToProxyConfigurationRequest` | `structure` | `ProxyConfigurationArn`, `ProxyConfigurationName`, `RuleGroups`, `UpdateToken` | - |
| `AttachRuleGroupsToProxyConfigurationResponse` | `structure` | `ProxyConfiguration`, `UpdateToken` | - |
| `CreateFirewallRequest` | `structure` | `AvailabilityZoneChangeProtection`, `AvailabilityZoneMappings`, `DeleteProtection`, `Description`, `EnabledAnalysisTypes`, `EncryptionConfiguration`, `FirewallName`, `FirewallPolicyArn`, `FirewallPolicyChangeProtection`, `SubnetChangeProtection`, `SubnetMappings`, `Tags`, ... (+2) | - |
| `CreateFirewallResponse` | `structure` | `Firewall`, `FirewallStatus` | - |
| `CreateFirewallPolicyRequest` | `structure` | `Description`, `DryRun`, `EncryptionConfiguration`, `FirewallPolicy`, `FirewallPolicyName`, `Tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
