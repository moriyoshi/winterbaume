# Amazon Route 53 Resolver

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

When you create a VPC using Amazon VPC, you automatically get DNS resolution within the VPC from Route 53 Resolver. By default, Resolver answers DNS queries for VPC domain names such as domain names for EC2 instances or Elastic Load Balancing load balancers. Resolver performs recursive lookups against public name servers for all other domain names. You can also configure DNS resolution between your VPC and your network over a Direct Connect or VPN connection: Forward DNS queries from resolvers on your network to Route 53 Resolver DNS resolvers on your network can forward DNS queries to Resolver in a specified VPC. This allows your DNS resolvers to easily resolve domain names for Amazon Web Services resources such as EC2 instances or records in a Route 53 private hosted zone.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Route 53 Resolver where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Route 53 Resolver by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Route 53 Resolver workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Update`, `Create`, `Delete` operation families, including `ListFirewallConfigs`, `ListFirewallDomainLists`, `ListFirewallDomains`, `ListFirewallRuleGroupAssociations`, `GetFirewallConfig`, `GetFirewallDomainList`.

## Service Identity and Protocol

- AWS model slug: `route53resolver`
- AWS SDK for Rust slug: `route53resolver`
- Model version: `2018-04-01`
- Model file: `vendor/api-models-aws/models/route53resolver/service/2018-04-01/route53resolver-2018-04-01.json`
- SDK ID: `Route53Resolver`
- Endpoint prefix: `route53resolver`
- ARN namespace: `route53resolver`
- CloudFormation name: `Route53Resolver`
- CloudTrail event source: `route53resolver.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (16), `Get` (15), `Update` (9), `Create` (7), `Delete` (7), `Associate` (4), `Disassociate` (4), `Put` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateFirewallRuleGroup`, `AssociateResolverEndpointIpAddress`, `AssociateResolverQueryLogConfig`, `AssociateResolverRule`, `CreateFirewallDomainList`, `CreateFirewallRule`, `CreateFirewallRuleGroup`, `CreateOutpostResolver`, `CreateResolverEndpoint`, `CreateResolverQueryLogConfig`, `CreateResolverRule`, `DeleteFirewallDomainList`, `DeleteFirewallRule`, `DeleteFirewallRuleGroup`, `DeleteOutpostResolver`, `DeleteResolverEndpoint`, `DeleteResolverQueryLogConfig`, `DeleteResolverRule`, `DisassociateFirewallRuleGroup`, `DisassociateResolverEndpointIpAddress`, ... (+17).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetFirewallConfig`, `GetFirewallDomainList`, `GetFirewallRuleGroup`, `GetFirewallRuleGroupAssociation`, `GetFirewallRuleGroupPolicy`, `GetOutpostResolver`, `GetResolverConfig`, `GetResolverDnssecConfig`, `GetResolverEndpoint`, `GetResolverQueryLogConfig`, `GetResolverQueryLogConfigAssociation`, `GetResolverQueryLogConfigPolicy`, `GetResolverRule`, `GetResolverRuleAssociation`, `GetResolverRulePolicy`, `ListFirewallConfigs`, `ListFirewallDomainLists`, `ListFirewallDomains`, `ListFirewallRuleGroupAssociations`, `ListFirewallRuleGroups`, ... (+11).
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ImportFirewallDomains`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 68 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`.

## Current Network Resource Stub Semantics

Route 53 Resolver currently stores resolver endpoint and rule association networking locally.

- Resolver endpoints store security group IDs, direction, host VPC ID, and IP address records with subnet IDs.
- Endpoint creation mints a synthetic host VPC ID instead of resolving it from the requested subnets.
- Resolver rule associations store resolver rule ID plus VPC ID pairs and enforce duplicates only within Route 53 Resolver state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListFirewallConfigs`, `ListFirewallDomainLists`, `ListFirewallDomains`, `ListFirewallRuleGroupAssociations`, `ListFirewallRuleGroups`, `ListFirewallRules`, `ListOutpostResolvers`, `ListResolverConfigs`, `ListResolverDnssecConfigs`, `ListResolverEndpointIpAddresses`, `ListResolverEndpoints`, `ListResolverQueryLogConfigAssociations`, `ListResolverQueryLogConfigs`, `ListResolverRuleAssociations`, `ListResolverRules`, `ListTagsForResource`
- Traits: `paginated` (16)
- Common required input members in this group: `FirewallDomainListId`, `FirewallRuleGroupId`, `ResolverEndpointId`, `ResourceArn`

### Get

- Operations: `GetFirewallConfig`, `GetFirewallDomainList`, `GetFirewallRuleGroup`, `GetFirewallRuleGroupAssociation`, `GetFirewallRuleGroupPolicy`, `GetOutpostResolver`, `GetResolverConfig`, `GetResolverDnssecConfig`, `GetResolverEndpoint`, `GetResolverQueryLogConfig`, `GetResolverQueryLogConfigAssociation`, `GetResolverQueryLogConfigPolicy`, `GetResolverRule`, `GetResolverRuleAssociation`, `GetResolverRulePolicy`
- Common required input members in this group: `Arn`, `FirewallDomainListId`, `FirewallRuleGroupAssociationId`, `FirewallRuleGroupId`, `Id`, `ResolverEndpointId`, `ResolverQueryLogConfigAssociationId`, `ResolverQueryLogConfigId`, `ResolverRuleAssociationId`, `ResolverRuleId`, `ResourceId`

### Update

- Operations: `UpdateFirewallConfig`, `UpdateFirewallDomains`, `UpdateFirewallRule`, `UpdateFirewallRuleGroupAssociation`, `UpdateOutpostResolver`, `UpdateResolverConfig`, `UpdateResolverDnssecConfig`, `UpdateResolverEndpoint`, `UpdateResolverRule`
- Common required input members in this group: `AutodefinedReverseFlag`, `Config`, `Domains`, `FirewallDomainListId`, `FirewallFailOpen`, `FirewallRuleGroupAssociationId`, `FirewallRuleGroupId`, `Id`, `Operation`, `ResolverEndpointId`, `ResolverRuleId`, `ResourceId`, `Validation`

### Create

- Operations: `CreateFirewallDomainList`, `CreateFirewallRule`, `CreateFirewallRuleGroup`, `CreateOutpostResolver`, `CreateResolverEndpoint`, `CreateResolverQueryLogConfig`, `CreateResolverRule`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `Action`, `CreatorRequestId`, `DestinationArn`, `Direction`, `FirewallRuleGroupId`, `IpAddresses`, `Name`, `OutpostArn`, `PreferredInstanceType`, `Priority`, `RuleType`, `SecurityGroupIds`

### Delete

- Operations: `DeleteFirewallDomainList`, `DeleteFirewallRule`, `DeleteFirewallRuleGroup`, `DeleteOutpostResolver`, `DeleteResolverEndpoint`, `DeleteResolverQueryLogConfig`, `DeleteResolverRule`
- Common required input members in this group: `FirewallDomainListId`, `FirewallRuleGroupId`, `Id`, `ResolverEndpointId`, `ResolverQueryLogConfigId`, `ResolverRuleId`

### Associate

- Operations: `AssociateFirewallRuleGroup`, `AssociateResolverEndpointIpAddress`, `AssociateResolverQueryLogConfig`, `AssociateResolverRule`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `CreatorRequestId`, `FirewallRuleGroupId`, `IpAddress`, `Name`, `Priority`, `ResolverEndpointId`, `ResolverQueryLogConfigId`, `ResolverRuleId`, `ResourceId`, `VPCId`, `VpcId`

### Disassociate

- Operations: `DisassociateFirewallRuleGroup`, `DisassociateResolverEndpointIpAddress`, `DisassociateResolverQueryLogConfig`, `DisassociateResolverRule`
- Common required input members in this group: `FirewallRuleGroupAssociationId`, `IpAddress`, `ResolverEndpointId`, `ResolverQueryLogConfigId`, `ResolverRuleId`, `ResourceId`, `VPCId`

### Put

- Operations: `PutFirewallRuleGroupPolicy`, `PutResolverQueryLogConfigPolicy`, `PutResolverRulePolicy`
- Common required input members in this group: `Arn`, `FirewallRuleGroupPolicy`, `ResolverQueryLogConfigPolicy`, `ResolverRulePolicy`

### Import

- Operations: `ImportFirewallDomains`
- Common required input members in this group: `DomainFileUrl`, `FirewallDomainListId`, `Operation`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateFirewallRuleGroup` | - | `idempotency-token` | `CreatorRequestId`, `FirewallRuleGroupId`, `Name`, `Priority`, `VpcId` | `CreatorRequestId` | `AssociateFirewallRuleGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a FirewallRuleGroup with a VPC, to provide DNS filtering for the VPC. |
| `AssociateResolverEndpointIpAddress` | - | - | `IpAddress`, `ResolverEndpointId` | - | `AssociateResolverEndpointIpAddressResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Adds IP addresses to an inbound or an outbound Resolver endpoint. If you want to add more than one IP address, submit one `AssociateResolverEndpointIpAddress` request for each IP address. |
| `AssociateResolverQueryLogConfig` | - | - | `ResolverQueryLogConfigId`, `ResourceId` | - | `AssociateResolverQueryLogConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Associates an Amazon VPC with a specified query logging configuration. Route 53 Resolver logs DNS queries that originate in all of the Amazon VPCs that are associated with a specified query logging configuration. |
| `AssociateResolverRule` | - | - | `ResolverRuleId`, `VPCId` | - | `AssociateResolverRuleResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceExistsException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `ThrottlingException` | Associates a Resolver rule with a VPC. When you associate a rule with a VPC, Resolver forwards all DNS queries for the domain name that is specified in the rule and that originate in the VPC. |
| `CreateFirewallDomainList` | - | `idempotency-token` | `CreatorRequestId`, `Name` | `CreatorRequestId` | `CreateFirewallDomainListResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates an empty firewall domain list for use in DNS Firewall rules. You can populate the domains for the new list with a file, using ImportFirewallDomains, or with domain strings, using UpdateFirewallDomains. |
| `CreateFirewallRule` | - | `idempotency-token` | `Action`, `CreatorRequestId`, `FirewallRuleGroupId`, `Name`, `Priority` | `CreatorRequestId` | `CreateFirewallRuleResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a single DNS Firewall rule in the specified rule group, using the specified domain list. |
| `CreateFirewallRuleGroup` | - | `idempotency-token` | `CreatorRequestId`, `Name` | `CreatorRequestId` | `CreateFirewallRuleGroupResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates an empty DNS Firewall rule group for filtering DNS network traffic in a VPC. You can add rules to the new rule group by calling CreateFirewallRule. |
| `CreateOutpostResolver` | - | - | `CreatorRequestId`, `Name`, `OutpostArn`, `PreferredInstanceType` | - | `CreateOutpostResolverResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a Route 53 Resolver on an Outpost. |
| `CreateResolverEndpoint` | - | - | `CreatorRequestId`, `Direction`, `IpAddresses`, `SecurityGroupIds` | - | `CreateResolverEndpointResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a Resolver endpoint. There are two types of Resolver endpoints, inbound and outbound: An inbound Resolver endpoint forwards DNS queries to the DNS service for a VPC from your network. |
| `CreateResolverQueryLogConfig` | - | `idempotency-token` | `CreatorRequestId`, `DestinationArn`, `Name` | `CreatorRequestId` | `CreateResolverQueryLogConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a Resolver query logging configuration, which defines where you want Resolver to save DNS query logs that originate in your VPCs. Resolver can log queries only for VPCs that are in the same Region as the query logging configuration. |
| `CreateResolverRule` | - | - | `CreatorRequestId`, `RuleType` | - | `CreateResolverRuleResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceExistsException`, `ResourceNotFoundException`, `ResourceUnavailableException`, ... (+1) | For DNS queries that originate in your VPCs, specifies which Resolver endpoint the queries pass through, one domain name that you want to forward to your network, and the IP addresses of the DNS resolvers in your network. |
| `DeleteFirewallDomainList` | - | - | `FirewallDomainListId` | - | `DeleteFirewallDomainListResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified domain list. |
| `DeleteFirewallRule` | - | - | `FirewallRuleGroupId` | - | `DeleteFirewallRuleResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified firewall rule. |
| `DeleteFirewallRuleGroup` | - | - | `FirewallRuleGroupId` | - | `DeleteFirewallRuleGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified firewall rule group. |
| `DeleteOutpostResolver` | - | - | `Id` | - | `DeleteOutpostResolverResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a Resolver on the Outpost. |
| `DeleteResolverEndpoint` | - | - | `ResolverEndpointId` | - | `DeleteResolverEndpointResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a Resolver endpoint. The effect of deleting a Resolver endpoint depends on whether it's an inbound or an outbound Resolver endpoint: Inbound : DNS queries from your network are no longer routed to the DNS service for the specified VPC. |
| `DeleteResolverQueryLogConfig` | - | - | `ResolverQueryLogConfigId` | - | `DeleteResolverQueryLogConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a query logging configuration. When you delete a configuration, Resolver stops logging DNS queries for all of the Amazon VPCs that are associated with the configuration. |
| `DeleteResolverRule` | - | - | `ResolverRuleId` | - | `DeleteResolverRuleResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a Resolver rule. Before you can delete a Resolver rule, you must disassociate it from all the VPCs that you associated the Resolver rule with. |
| `DisassociateFirewallRuleGroup` | - | - | `FirewallRuleGroupAssociationId` | - | `DisassociateFirewallRuleGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a FirewallRuleGroup from a VPC, to remove DNS filtering from the VPC. |
| `DisassociateResolverEndpointIpAddress` | - | - | `IpAddress`, `ResolverEndpointId` | - | `DisassociateResolverEndpointIpAddressResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Removes IP addresses from an inbound or an outbound Resolver endpoint. If you want to remove more than one IP address, submit one `DisassociateResolverEndpointIpAddress` request for each IP address. |
| `DisassociateResolverQueryLogConfig` | - | - | `ResolverQueryLogConfigId`, `ResourceId` | - | `DisassociateResolverQueryLogConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Disassociates a VPC from a query logging configuration. Before you can delete a query logging configuration, you must first disassociate all VPCs from the configuration. |
| `DisassociateResolverRule` | - | - | `ResolverRuleId`, `VPCId` | - | `DisassociateResolverRuleResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException` | Removes the association between a specified Resolver rule and a specified VPC. If you disassociate a Resolver rule from a VPC, Resolver stops forwarding DNS queries for the domain name that you specified in the Resolver rule. |
| `GetFirewallConfig` | - | - | `ResourceId` | - | `GetFirewallConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the configuration of the firewall behavior provided by DNS Firewall for a single VPC from Amazon Virtual Private Cloud (Amazon VPC). |
| `GetFirewallDomainList` | - | - | `FirewallDomainListId` | - | `GetFirewallDomainListResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the specified firewall domain list. |
| `GetFirewallRuleGroup` | - | - | `FirewallRuleGroupId` | - | `GetFirewallRuleGroupResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the specified firewall rule group. |
| `GetFirewallRuleGroupAssociation` | - | - | `FirewallRuleGroupAssociationId` | - | `GetFirewallRuleGroupAssociationResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a firewall rule group association, which enables DNS filtering for a VPC with one rule group. A VPC can have more than one firewall rule group association, and a rule group can be associated with more than one VPC. |
| `GetFirewallRuleGroupPolicy` | - | - | `Arn` | - | `GetFirewallRuleGroupPolicyResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the Identity and Access Management (Amazon Web Services IAM) policy for sharing the specified rule group. You can use the policy to share the rule group using Resource Access Manager (RAM). |
| `GetOutpostResolver` | - | - | `Id` | - | `GetOutpostResolverResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a specified Resolver on the Outpost, such as its instance count and type, name, and the current status of the Resolver. |
| `GetResolverConfig` | - | - | `ResourceId` | - | `GetResolverConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the behavior configuration of Route 53 Resolver behavior for a single VPC from Amazon Virtual Private Cloud. |
| `GetResolverDnssecConfig` | - | - | `ResourceId` | - | `GetResolverDnssecConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets DNSSEC validation information for a specified resource. |
| `GetResolverEndpoint` | - | - | `ResolverEndpointId` | - | `GetResolverEndpointResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException` | Gets information about a specified Resolver endpoint, such as whether it's an inbound or an outbound Resolver endpoint, and the current status of the endpoint. |
| `GetResolverQueryLogConfig` | - | - | `ResolverQueryLogConfigId` | - | `GetResolverQueryLogConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets information about a specified Resolver query logging configuration, such as the number of VPCs that the configuration is logging queries for and the location that logs are sent to. |
| `GetResolverQueryLogConfigAssociation` | - | - | `ResolverQueryLogConfigAssociationId` | - | `GetResolverQueryLogConfigAssociationResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Gets information about a specified association between a Resolver query logging configuration and an Amazon VPC. When you associate a VPC with a query logging configuration, Resolver logs DNS queries that originate in that VPC. |
| `GetResolverQueryLogConfigPolicy` | - | - | `Arn` | - | `GetResolverQueryLogConfigPolicyResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `UnknownResourceException` | Gets information about a query logging policy. A query logging policy specifies the Resolver query logging operations and resources that you want to allow another Amazon Web Services account to be able to use. |
| `GetResolverRule` | - | - | `ResolverRuleId` | - | `GetResolverRuleResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException` | Gets information about a specified Resolver rule, such as the domain name that the rule forwards DNS queries for and the ID of the outbound Resolver endpoint that the rule is associated with. |
| `GetResolverRuleAssociation` | - | - | `ResolverRuleAssociationId` | - | `GetResolverRuleAssociationResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException` | Gets information about an association between a specified Resolver rule and a VPC. You associate a Resolver rule and a VPC using AssociateResolverRule. |
| `GetResolverRulePolicy` | - | - | `Arn` | - | `GetResolverRulePolicyResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `UnknownResourceException` | Gets information about the Resolver rule policy for a specified rule. A Resolver rule policy includes the rule that you want to share with another account, the account that you want to share the rule with, and the Resolver operations that you want to allow... |
| `ImportFirewallDomains` | - | - | `DomainFileUrl`, `FirewallDomainListId`, `Operation` | - | `ImportFirewallDomainsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Imports domain names from a file into a domain list, for use in a DNS firewall rule group. Each domain specification in your domain list must satisfy the following requirements: It can optionally start with `*` (asterisk). |
| `ListFirewallConfigs` | - | `paginated` | - | - | `ListFirewallConfigsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ThrottlingException`, `ValidationException` | Retrieves the firewall configurations that you have defined. DNS Firewall uses the configurations to manage firewall behavior for your VPCs. |
| `ListFirewallDomainLists` | - | `paginated` | - | - | `ListFirewallDomainListsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ThrottlingException`, `ValidationException` | Retrieves the firewall domain lists that you have defined. For each firewall domain list, you can retrieve the domains that are defined for a list by calling ListFirewallDomains. |
| `ListFirewallDomains` | - | `paginated` | `FirewallDomainListId` | - | `ListFirewallDomainsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the domains that you have defined for the specified firewall domain list. A single call might return only a partial list of the domains. |
| `ListFirewallRuleGroupAssociations` | - | `paginated` | - | - | `ListFirewallRuleGroupAssociationsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ThrottlingException`, `ValidationException` | Retrieves the firewall rule group associations that you have defined. Each association enables DNS filtering for a VPC with one rule group. |
| `ListFirewallRuleGroups` | - | `paginated` | - | - | `ListFirewallRuleGroupsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ThrottlingException`, `ValidationException` | Retrieves the minimal high-level information for the rule groups that you have defined. A single call might return only a partial list of the rule groups. |
| `ListFirewallRules` | - | `paginated` | `FirewallRuleGroupId` | - | `ListFirewallRulesResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the firewall rules that you have defined for the specified firewall rule group. DNS Firewall uses the rules in a rule group to filter DNS network traffic for a VPC. |
| `ListOutpostResolvers` | - | `paginated` | - | - | `ListOutpostResolversResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the Resolvers on Outposts that were created using the current Amazon Web Services account. |
| `ListResolverConfigs` | - | `paginated` | - | - | `ListResolverConfigsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidRequestException`, `ThrottlingException`, `ValidationException` | Retrieves the Resolver configurations that you have defined. Route 53 Resolver uses the configurations to manage DNS resolution behavior for your VPCs. |
| `ListResolverDnssecConfigs` | - | `paginated` | - | - | `ListResolverDnssecConfigsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidRequestException`, `ThrottlingException` | Lists the configurations for DNSSEC validation that are associated with the current Amazon Web Services account. |
| `ListResolverEndpointIpAddresses` | - | `paginated` | `ResolverEndpointId` | - | `ListResolverEndpointIpAddressesResponse` | `InternalServiceErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the IP addresses for a specified Resolver endpoint. |
| `ListResolverEndpoints` | - | `paginated` | - | - | `ListResolverEndpointsResponse` | `InternalServiceErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidRequestException`, `ThrottlingException` | Lists all the Resolver endpoints that were created using the current Amazon Web Services account. |
| `ListResolverQueryLogConfigAssociations` | - | `paginated` | - | - | `ListResolverQueryLogConfigAssociationsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ThrottlingException` | Lists information about associations between Amazon VPCs and query logging configurations. |
| `ListResolverQueryLogConfigs` | - | `paginated` | - | - | `ListResolverQueryLogConfigsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidRequestException`, `ThrottlingException` | Lists information about the specified query logging configurations. Each configuration defines where you want Resolver to save DNS query logs and specifies the VPCs that you want to log queries for. |
| `ListResolverRuleAssociations` | - | `paginated` | - | - | `ListResolverRuleAssociationsResponse` | `InternalServiceErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidRequestException`, `ThrottlingException` | Lists the associations that were created between Resolver rules and VPCs using the current Amazon Web Services account. |
| `ListResolverRules` | - | `paginated` | - | - | `ListResolverRulesResponse` | `InternalServiceErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidRequestException`, `ThrottlingException` | Lists the Resolver rules that were created using the current Amazon Web Services account. |
| `ListTagsForResource` | - | `paginated` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServiceErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Lists the tags that you associated with the specified resource. |
| `PutFirewallRuleGroupPolicy` | - | - | `Arn`, `FirewallRuleGroupPolicy` | - | `PutFirewallRuleGroupPolicyResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches an Identity and Access Management (Amazon Web Services IAM) policy for sharing the rule group. You can use the policy to share the rule group using Resource Access Manager (RAM). |
| `PutResolverQueryLogConfigPolicy` | - | - | `Arn`, `ResolverQueryLogConfigPolicy` | - | `PutResolverQueryLogConfigPolicyResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidPolicyDocument`, `InvalidRequestException`, `UnknownResourceException` | Specifies an Amazon Web Services account that you want to share a query logging configuration with, the query logging configuration that you want to share, and the operations that you want the account to be able to perform on the configuration. |
| `PutResolverRulePolicy` | - | - | `Arn`, `ResolverRulePolicy` | - | `PutResolverRulePolicyResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidPolicyDocument`, `UnknownResourceException` | Specifies an Amazon Web Services rule that you want to share with another account, the account that you want to share the rule with, and the operations that you want the account to be able to perform on the rule. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `InvalidTagException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Adds one or more tags to a specified resource. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Removes one or more tags from a specified resource. |
| `UpdateFirewallConfig` | - | - | `FirewallFailOpen`, `ResourceId` | - | `UpdateFirewallConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of the firewall behavior provided by DNS Firewall for a single VPC from Amazon Virtual Private Cloud (Amazon VPC). |
| `UpdateFirewallDomains` | - | - | `Domains`, `FirewallDomainListId`, `Operation` | - | `UpdateFirewallDomainsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the firewall domain list from an array of domain specifications. |
| `UpdateFirewallRule` | - | - | `FirewallRuleGroupId` | - | `UpdateFirewallRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified firewall rule. |
| `UpdateFirewallRuleGroupAssociation` | - | - | `FirewallRuleGroupAssociationId` | - | `UpdateFirewallRuleGroupAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Changes the association of a FirewallRuleGroup with a VPC. The association enables DNS filtering for the VPC. |
| `UpdateOutpostResolver` | - | - | `Id` | - | `UpdateOutpostResolverResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | You can use `UpdateOutpostResolver` to update the instance count, type, or name of a Resolver on an Outpost. |
| `UpdateResolverConfig` | - | - | `AutodefinedReverseFlag`, `ResourceId` | - | `UpdateResolverConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `ThrottlingException`, ... (+1) | Updates the behavior configuration of Route 53 Resolver behavior for a single VPC from Amazon Virtual Private Cloud. |
| `UpdateResolverDnssecConfig` | - | - | `ResourceId`, `Validation` | - | `UpdateResolverDnssecConfigResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an existing DNSSEC validation configuration. If there is no existing DNSSEC validation configuration, one is created. |
| `UpdateResolverEndpoint` | - | - | `ResolverEndpointId` | - | `UpdateResolverEndpointResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the name, or endpoint type for an inbound or an outbound Resolver endpoint. You can only update between IPV4 and DUALSTACK, IPV6 endpoint type can't be updated to other type. |
| `UpdateResolverRule` | - | - | `Config`, `ResolverRuleId` | - | `UpdateResolverRuleResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidParameterException`, `InvalidRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `ThrottlingException` | Updates settings for a specified Resolver rule. `ResolverRuleId` is required, and all other parameters are optional. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceErrorException` | `structure` | `Message` | We encountered an unknown error. |
| `ThrottlingException` | `structure` | `Message` | The request was throttled. |
| `AccessDeniedException` | `structure` | `Message` | The current account doesn't have the IAM permissions required to perform the specified Resolver operation. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceType` | The specified resource doesn't exist. |
| `InvalidParameterException` | `structure` | `FieldName`, `Message` | One or more parameters in this request are not valid. |
| `ValidationException` | `structure` | `Message` | You have provided an invalid command. |
| `InvalidRequestException` | `structure` | `Message` | The request is invalid. |
| `LimitExceededException` | `structure` | `Message`, `ResourceType` | The request caused one or more limits to be exceeded. |
| `ConflictException` | `structure` | `Message` | The requested state transition isn't valid. |
| `InvalidNextTokenException` | `structure` | `Message` | The value that you specified for `NextToken` in a `List` request isn't valid. |
| `ResourceExistsException` | `structure` | `Message`, `ResourceType` | The resource that you tried to create already exists. |
| `ResourceUnavailableException` | `structure` | `Message`, `ResourceType` | The specified resource isn't available. |
| `UnknownResourceException` | `structure` | `Message` | The specified resource doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | `Message` | Fulfilling the request would cause one or more quotas to be exceeded. |
| `InvalidPolicyDocument` | `structure` | `Message` | The specified Resolver rule policy is invalid. |
| `AssociateFirewallRuleGroupRequest` | `structure` | `CreatorRequestId`, `FirewallRuleGroupId`, `MutationProtection`, `Name`, `Priority`, `Tags`, `VpcId` | - |
| `AssociateFirewallRuleGroupResponse` | `structure` | `FirewallRuleGroupAssociation` | - |
| `AssociateResolverEndpointIpAddressRequest` | `structure` | `IpAddress`, `ResolverEndpointId` | - |
| `AssociateResolverEndpointIpAddressResponse` | `structure` | `ResolverEndpoint` | - |
| `AssociateResolverQueryLogConfigRequest` | `structure` | `ResolverQueryLogConfigId`, `ResourceId` | - |
| `AssociateResolverQueryLogConfigResponse` | `structure` | `ResolverQueryLogConfigAssociation` | - |
| `AssociateResolverRuleRequest` | `structure` | `Name`, `ResolverRuleId`, `VPCId` | - |
| `AssociateResolverRuleResponse` | `structure` | `ResolverRuleAssociation` | - |
| `CreateFirewallDomainListRequest` | `structure` | `CreatorRequestId`, `Name`, `Tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
