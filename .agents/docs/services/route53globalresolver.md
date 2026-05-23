# Amazon Route 53 Global Resolver

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Route 53 Global Resolver is a global, internet-accessible DNS resolver that enables customers to resolve and forward traffic for both public and private domains while ensuring security and authenticity of queries over the internet. Route 53 Global Resolver supports DNS-over-port 53 (Do53), DNS-over-TLS (DoT), and DNS-over-HTTPS (DoH) protocols through global anycast IP addresses. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53 Global Resolver resources. That is, for example, specify `--region us-east-2` on Amazon Web Services CLI commands.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Route 53 Global Resolver where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Route 53 Global Resolver by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Route 53 Global Resolver by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Route 53 Global Resolver workflows in the local mock. Key resources include `AccessSource`, `AccessToken`, `DNSView`, `FirewallDomainList`, `FirewallRule`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Update`, `Create`, `Delete` operation families, including `ListAccessSources`, `ListAccessTokens`, `ListDNSViews`, `ListFirewallDomainLists`, `GetAccessSource`, `GetAccessToken`.

## Service Identity and Protocol

- AWS model slug: `route53globalresolver`
- AWS SDK for Rust slug: `route53globalresolver`
- Model version: `2022-09-27`
- Model file: `vendor/api-models-aws/models/route53globalresolver/service/2022-09-27/route53globalresolver-2022-09-27.json`
- SDK ID: `Route53GlobalResolver`
- Endpoint prefix: `-`
- ARN namespace: `route53globalresolver`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Get` (8), `Update` (7), `Create` (6), `Delete` (6), `Batch` (3), `Associate` (1), `Disable` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateHostedZone`, `BatchCreateFirewallRule`, `BatchDeleteFirewallRule`, `BatchUpdateFirewallRule`, `CreateAccessSource`, `CreateAccessToken`, `CreateDNSView`, `CreateFirewallDomainList`, `CreateFirewallRule`, `CreateGlobalResolver`, `DeleteAccessSource`, `DeleteAccessToken`, `DeleteDNSView`, `DeleteFirewallDomainList`, `DeleteFirewallRule`, `DeleteGlobalResolver`, `DisableDNSView`, `DisassociateHostedZone`, `EnableDNSView`, `ImportFirewallDomains`, ... (+9).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccessSource`, `GetAccessToken`, `GetDNSView`, `GetFirewallDomainList`, `GetFirewallRule`, `GetGlobalResolver`, `GetHostedZoneAssociation`, `GetManagedFirewallDomainList`, `ListAccessSources`, `ListAccessTokens`, `ListDNSViews`, `ListFirewallDomainLists`, `ListFirewallDomains`, `ListFirewallRules`, `ListGlobalResolvers`, `ListHostedZoneAssociations`, `ListManagedFirewallDomainLists`, `ListTagsForResource`.
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 25 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ImportFirewallDomains`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 47 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccessSource` | `id` | create: `CreateAccessSource`; read: `GetAccessSource`; update: `UpdateAccessSource`; delete: `DeleteAccessSource`; list: `ListAccessSources` | - | Represents a Route 53 Global Resolver access source that permits access to a global resolver by specifying source IP CIDR ranges and DNS protocols |
| `AccessToken` | `id` | create: `CreateAccessToken`; read: `GetAccessToken`; update: `UpdateAccessToken`; delete: `DeleteAccessToken`; list: `ListAccessTokens` | - | Represents a Route 53 Global Resolver access token that permits secure authentication to global resolver by an encrypted and time-limited token |
| `DNSView` | `id` | create: `CreateDNSView`; read: `GetDNSView`; update: `UpdateDNSView`; delete: `DeleteDNSView`; list: `ListDNSViews` | `DisableDNSView`, `EnableDNSView` | Represents a Route 53 Global Resolver DNS view that defines security policies for a group of entities that share common access controls and DNS filtering rules |
| `FirewallDomainList` | `id` | create: `CreateFirewallDomainList`; read: `GetFirewallDomainList`; delete: `DeleteFirewallDomainList`; list: `ListFirewallDomainLists` | `ImportFirewallDomains`, `ListFirewallDomains`, `UpdateFirewallDomains` | Represents a Route 53 Global Resolver firewall domain list that is a reusable set of domain specifications used in firewall rules |
| `FirewallRule` | `id` | create: `CreateFirewallRule`; read: `GetFirewallRule`; update: `UpdateFirewallRule`; delete: `DeleteFirewallRule`; list: `ListFirewallRules` | `BatchCreateFirewallRule`, `BatchDeleteFirewallRule`, `BatchUpdateFirewallRule` | Represents a Route 53 Global Resolver firewall rule that defines how DNS queries are filtered based on domain lists, with configurable actions such as allow, block, or alert for... |
| `GlobalResolver` | `id` | create: `CreateGlobalResolver`; read: `GetGlobalResolver`; update: `UpdateGlobalResolver`; delete: `DeleteGlobalResolver`; list: `ListGlobalResolvers` | - | Represents a Route 53 Global Resolver that provides DNS resolution and filtering for all clients regardless of their location |
| `HostedZoneAssociation` | `id` | create: `AssociateHostedZone`; read: `GetHostedZoneAssociation`; update: `UpdateHostedZoneAssociation`; list: `ListHostedZoneAssociations` | - | Represents a Route 53 Global Resolver hosted zone association that links a Route 53 private hosted zone to a DNS view, enabling the global resolver to resolve private queries for... |
| `ManagedFirewallDomainList` | `id` | read: `GetManagedFirewallDomainList`; list: `ListManagedFirewallDomainLists` | - | Represents a Route 53 Global Resolver managed firewall domain list that provides AWS-managed collections of domains for common security use cases, such as blocking malware... |
## Operation Groups

### Disassociate

- Operations: `DisassociateHostedZone`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
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
| `DisassociateHostedZone` | `DELETE /hosted-zone-associations/hosted-zone/{hostedZoneId}/resource-arn/{resourceArn+}` | `idempotent` | `hostedZoneId`, `resourceArn` | - | `DisassociateHostedZoneOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a Route 53 private hosted zone from a Route 53 Global Resolver resource. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify ... |
| `ListTagsForResource` | `POST /get-all-tags` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags associated with a Route 53 Global Resolver resource. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) ... |
| `TagResource` | `POST /tag-resource` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Adds or updates tags for a Route 53 Global Resolver resource. Tags are key-value pairs that help you organize and identify your resources. Route 53 Global Resolver is a global service that supports resolvers in multi ... |
| `UntagResource` | `POST /untag-resource` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Removes tags from a Route 53 Global Resolver resource. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to cr ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have permission to perform this operation. Check your IAM permissions and try again. |
| `ConflictException` | `structure` | message, resourceId, resourceType | The request conflicts with the current state of the resource. This can occur when trying to modify a resource that is not in a valid state for the requested ... |
| `InternalServerException` | `structure` | message, retryAfterSeconds | An internal server error occurred. Try again later. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The specified resource was not found. Verify the resource ID and try again. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | The request would exceed one or more service quotas. Check your current usage and quotas, then try again. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was throttled due to too many requests. Wait a moment and try again. |
| `ValidationException` | `structure` | message, reason, fieldList | The input parameters are invalid. Check the parameter values and try again. |
| `DisassociateHostedZoneInput` | `structure` | hostedZoneId, resourceArn | - |
| `DisassociateHostedZoneOutput` | `structure` | id, resourceArn, hostedZoneId, hostedZoneName, name, createdAt, updatedAt, status | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `BlockOverrideDnsQueryType` | `enum` | CNAME | - |
| `CRResourceStatus` | `enum` | CREATING, OPERATIONAL, UPDATING, DELETING | - |
| `ConfidenceThreshold` | `enum` | LOW, MEDIUM, HIGH | - |
| `DnsAdvancedProtection` | `enum` | DGA, DNS_TUNNELING, DICTIONARY_DGA | - |
| `DnsProtocol` | `enum` | DO53, DOH, DOT | - |
| `DnsSecValidationType` | `enum` | ENABLED, DISABLED | - |
| `EdnsClientSubnetType` | `enum` | ENABLED, DISABLED | - |
| `FirewallBlockResponse` | `enum` | NODATA, NXDOMAIN, OVERRIDE | - |
| `FirewallRuleAction` | `enum` | ALLOW, ALERT, BLOCK | - |
| `FirewallRulesFailOpenType` | `enum` | ENABLED, DISABLED | - |
| `GlobalResolverIpAddressType` | `enum` | IPV4, DUAL_STACK | - |
| `HostedZoneAssociationStatus` | `enum` | CREATING, OPERATIONAL, DELETING | - |
| `IpAddressType` | `enum` | IPV4, IPV6 | - |
| `ProfileResourceStatus` | `enum` | CREATING, OPERATIONAL, UPDATING, ENABLING, DISABLING, DISABLED, DELETING | - |
| `TokenStatus` | `enum` | CREATING, OPERATIONAL, DELETING | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
