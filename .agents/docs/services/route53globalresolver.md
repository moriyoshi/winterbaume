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

### List

- Operations: `ListAccessSources`, `ListAccessTokens`, `ListDNSViews`, `ListFirewallDomainLists`, `ListFirewallDomains`, `ListFirewallRules`, `ListGlobalResolvers`, `ListHostedZoneAssociations`, `ListManagedFirewallDomainLists`, `ListTagsForResource`
- Traits: `paginated` (9), `readonly` (9)
- Common required input members in this group: `dnsViewId`, `firewallDomainListId`, `globalResolverId`, `managedFirewallDomainListType`, `resourceArn`

### Get

- Operations: `GetAccessSource`, `GetAccessToken`, `GetDNSView`, `GetFirewallDomainList`, `GetFirewallRule`, `GetGlobalResolver`, `GetHostedZoneAssociation`, `GetManagedFirewallDomainList`
- Traits: `readonly` (8)
- Common required input members in this group: `accessSourceId`, `accessTokenId`, `dnsViewId`, `firewallDomainListId`, `firewallRuleId`, `globalResolverId`, `hostedZoneAssociationId`, `managedFirewallDomainListId`

### Update

- Operations: `UpdateAccessSource`, `UpdateAccessToken`, `UpdateDNSView`, `UpdateFirewallDomains`, `UpdateFirewallRule`, `UpdateGlobalResolver`, `UpdateHostedZoneAssociation`
- Traits: `idempotency-token` (1), `idempotent` (6)
- Common required input members in this group: `accessSourceId`, `accessTokenId`, `clientToken`, `dnsViewId`, `domains`, `firewallDomainListId`, `firewallRuleId`, `globalResolverId`, `hostedZoneAssociationId`, `name`, `operation`

### Create

- Operations: `CreateAccessSource`, `CreateAccessToken`, `CreateDNSView`, `CreateFirewallDomainList`, `CreateFirewallRule`, `CreateGlobalResolver`
- Traits: `idempotency-token` (6), `idempotent` (6)
- Common required input members in this group: `action`, `cidr`, `dnsViewId`, `globalResolverId`, `name`, `protocol`, `regions`

### Delete

- Operations: `DeleteAccessSource`, `DeleteAccessToken`, `DeleteDNSView`, `DeleteFirewallDomainList`, `DeleteFirewallRule`, `DeleteGlobalResolver`
- Traits: `idempotent` (6)
- Common required input members in this group: `accessSourceId`, `accessTokenId`, `dnsViewId`, `firewallDomainListId`, `firewallRuleId`, `globalResolverId`

### Batch

- Operations: `BatchCreateFirewallRule`, `BatchDeleteFirewallRule`, `BatchUpdateFirewallRule`
- Traits: `idempotent` (3)
- Common required input members in this group: `firewallRules`

### Associate

- Operations: `AssociateHostedZone`
- Traits: `idempotent` (1)
- Common required input members in this group: `hostedZoneId`, `name`, `resourceArn`

### Disable

- Operations: `DisableDNSView`
- Traits: `idempotent` (1)
- Common required input members in this group: `dnsViewId`

### Disassociate

- Operations: `DisassociateHostedZone`
- Traits: `idempotent` (1)
- Common required input members in this group: `hostedZoneId`, `resourceArn`

### Enable

- Operations: `EnableDNSView`
- Traits: `idempotent` (1)
- Common required input members in this group: `dnsViewId`

### Import

- Operations: `ImportFirewallDomains`
- Common required input members in this group: `domainFileUrl`, `firewallDomainListId`, `operation`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateHostedZone` | `POST /hosted-zone-associations/{hostedZoneId}` | `idempotent` | `hostedZoneId`, `name`, `resourceArn` | - | `AssociateHostedZoneOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a Route 53 private hosted zone with a Route 53 Global Resolver resource. This allows the resolver to resolve DNS queries for the private hosted zone from anywhere globally. |
| `BatchCreateFirewallRule` | `POST /firewall-rules/batch-create` | `idempotent` | `firewallRules` | - | `BatchCreateFirewallRuleOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates multiple DNS firewall rules in a single operation. This is more efficient than creating rules individually when you need to set up multiple rules at once. |
| `BatchDeleteFirewallRule` | `POST /firewall-rules/batch-delete` | `idempotent` | `firewallRules` | - | `BatchDeleteFirewallRuleOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes multiple DNS firewall rules in a single operation. This is more efficient than deleting rules individually. |
| `BatchUpdateFirewallRule` | `POST /firewall-rules/batch-update` | `idempotent` | `firewallRules` | - | `BatchUpdateFirewallRuleOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates multiple DNS firewall rules in a single operation. This is more efficient than updating rules individually. |
| `CreateAccessSource` | `POST /access-sources` | `idempotent`, `idempotency-token` | `cidr`, `dnsViewId`, `protocol` | `clientToken` | `CreateAccessSourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an access source for a DNS view. Access sources define IP addresses or CIDR ranges that are allowed to send DNS queries to the Route 53 Global Resolver, along with the permitted DNS protocols. |
| `CreateAccessToken` | `POST /tokens/{dnsViewId}` | `idempotent`, `idempotency-token` | `dnsViewId` | `clientToken` | `CreateAccessTokenOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an access token for a DNS view. Access tokens provide token-based authentication for DNS-over-HTTPS (DoH) and DNS-over-TLS (DoT) connections to the Route 53 Global Resolver. |
| `CreateDNSView` | `POST /dns-views/{globalResolverId}` | `idempotent`, `idempotency-token` | `globalResolverId`, `name` | `clientToken` | `CreateDNSViewOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a DNS view within a Route 53 Global Resolver. A DNS view models end users, user groups, networks, and devices, and serves as a parent resource that holds configurations controlling access, authorization, DNS firewall rules, and forwarding rules. |
| `CreateFirewallDomainList` | `POST /firewall-domain-lists/{globalResolverId}` | `idempotent`, `idempotency-token` | `globalResolverId`, `name` | `clientToken` | `CreateFirewallDomainListOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a firewall domain list. Domain lists are reusable sets of domain specifications that you use in DNS firewall rules to allow, block, or alert on DNS queries to specific domains. |
| `CreateFirewallRule` | `POST /firewall-rules` | `idempotent`, `idempotency-token` | `action`, `dnsViewId`, `name` | `clientToken` | `CreateFirewallRuleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a DNS firewall rule. Firewall rules define actions (ALLOW, BLOCK, or ALERT) to take on DNS queries that match specified domain lists, managed domain lists, or advanced threat protections. |
| `CreateGlobalResolver` | `POST /global-resolver` | `idempotent`, `idempotency-token` | `name`, `regions` | `clientToken` | `CreateGlobalResolverOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Route 53 Global Resolver instance. A Route 53 Global Resolver is a global, internet-accessible DNS resolver that provides secure DNS resolution for both public and private domains through global anycast IP addresses. |
| `DeleteAccessSource` | `DELETE /access-sources/{accessSourceId}` | `idempotent` | `accessSourceId` | - | `DeleteAccessSourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an access source. This operation cannot be undone. |
| `DeleteAccessToken` | `DELETE /tokens/{accessTokenId}` | `idempotent` | `accessTokenId` | - | `DeleteAccessTokenOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an access token. This operation cannot be undone. |
| `DeleteDNSView` | `DELETE /dns-views/{dnsViewId}` | `idempotent` | `dnsViewId` | - | `DeleteDNSViewOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a DNS view. This operation cannot be undone. |
| `DeleteFirewallDomainList` | `DELETE /firewall-domain-lists/{firewallDomainListId}` | `idempotent` | `firewallDomainListId` | - | `DeleteFirewallDomainListOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a firewall domain list. This operation cannot be undone. |
| `DeleteFirewallRule` | `DELETE /firewall-rules/{firewallRuleId}` | `idempotent` | `firewallRuleId` | - | `DeleteFirewallRuleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a DNS firewall rule. This operation cannot be undone. |
| `DeleteGlobalResolver` | `DELETE /global-resolver/{globalResolverId}` | `idempotent` | `globalResolverId` | - | `DeleteGlobalResolverOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a Route 53 Global Resolver instance. This operation cannot be undone. |
| `DisableDNSView` | `PATCH /dns-views/{dnsViewId}/disable` | `idempotent` | `dnsViewId` | - | `DisableDNSViewOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disables a DNS view, preventing it from serving DNS queries. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with... |
| `DisassociateHostedZone` | `DELETE /hosted-zone-associations/hosted-zone/{hostedZoneId}/resource-arn/{resourceArn+}` | `idempotent` | `hostedZoneId`, `resourceArn` | - | `DisassociateHostedZoneOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a Route 53 private hosted zone from a Route 53 Global Resolver resource. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create... |
| `EnableDNSView` | `PATCH /dns-views/{dnsViewId}/enable` | `idempotent` | `dnsViewId` | - | `EnableDNSViewOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables a disabled DNS view, allowing it to serve DNS queries again. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise... |
| `GetAccessSource` | `GET /access-sources/{accessSourceId}` | `readonly` | `accessSourceId` | - | `GetAccessSourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an access source. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53 Global... |
| `GetAccessToken` | `GET /tokens/{accessTokenId}` | `readonly` | `accessTokenId` | - | `GetAccessTokenOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an access token. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53 Global... |
| `GetDNSView` | `GET /dns-views/{dnsViewId}` | `readonly` | `dnsViewId` | - | `GetDNSViewOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a DNS view. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53 Global... |
| `GetFirewallDomainList` | `GET /firewall-domain-lists/{firewallDomainListId}` | `readonly` | `firewallDomainListId` | - | `GetFirewallDomainListOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a firewall domain list. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53... |
| `GetFirewallRule` | `GET /firewall-rules/{firewallRuleId}` | `readonly` | `firewallRuleId` | - | `GetFirewallRuleOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a DNS firewall rule. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53... |
| `GetGlobalResolver` | `GET /global-resolver/{globalResolverId}` | `readonly` | `globalResolverId` | - | `GetGlobalResolverOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a Route 53 Global Resolver instance. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work... |
| `GetHostedZoneAssociation` | `GET /hosted-zone-associations/{hostedZoneAssociationId}` | `readonly` | `hostedZoneAssociationId` | - | `GetHostedZoneAssociationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a hosted zone association. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route... |
| `GetManagedFirewallDomainList` | `GET /managed-firewall-domain-lists/{managedFirewallDomainListId}` | `readonly` | `managedFirewallDomainListId` | - | `GetManagedFirewallDomainListOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an Amazon Web Services-managed firewall domain list. Managed domain lists contain domains associated with malicious activity, content categories, or specific threats. |
| `ImportFirewallDomains` | `PATCH /firewall-domain-lists/{firewallDomainListId}/domains/s3_file_url` | - | `domainFileUrl`, `firewallDomainListId`, `operation` | - | `ImportFirewallDomainsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Imports a list of domains from an Amazon S3 file into a firewall domain list. The file should contain one domain per line. |
| `ListAccessSources` | `GET /access-sources` | `readonly`, `paginated` | - | - | `ListAccessSourcesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all access sources with pagination support. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53... |
| `ListAccessTokens` | `GET /tokens/dns-view/{dnsViewId}` | `readonly`, `paginated` | `dnsViewId` | - | `ListAccessTokensOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all access tokens for a DNS view with pagination support. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work... |
| `ListDNSViews` | `GET /dns-views/resolver/{globalResolverId}` | `readonly`, `paginated` | `globalResolverId` | - | `ListDNSViewsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all DNS views for a Route 53 Global Resolver with pagination support. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or... |
| `ListFirewallDomainLists` | `GET /firewall-domain-lists` | `readonly`, `paginated` | - | - | `ListFirewallDomainListsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all firewall domain lists for a Route 53 Global Resolver with pagination support. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create... |
| `ListFirewallDomains` | `GET /firewall-domain-lists/{firewallDomainListId}/domains` | `readonly`, `paginated` | `firewallDomainListId` | - | `ListFirewallDomainsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the domains in DNS Firewall domain list you have created. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise... |
| `ListFirewallRules` | `GET /firewall-rules` | `readonly`, `paginated` | `dnsViewId` | - | `ListFirewallRulesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all DNS firewall rules for a DNS view with pagination support. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise... |
| `ListGlobalResolvers` | `GET /global-resolver` | `readonly`, `paginated` | - | - | `ListGlobalResolversOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all Route 53 Global Resolver instances in your account with pagination support. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create... |
| `ListHostedZoneAssociations` | `GET /hosted-zone-associations/resource-arn/{resourceArn+}` | `readonly`, `paginated` | `resourceArn` | - | `ListHostedZoneAssociationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all hosted zone associations for a Route 53 Global Resolver resource with pagination support. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to... |
| `ListManagedFirewallDomainLists` | `GET /list-managed-firewall-domain-lists/{managedFirewallDomainListType}` | `readonly`, `paginated` | `managedFirewallDomainListType` | - | `ListManagedFirewallDomainListsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a paginated list of the Amazon Web Services Managed DNS Lists and the categories for DNS Firewall. The categories are either `THREAT` or `CONTENT`. |
| `ListTagsForResource` | `POST /get-all-tags` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags associated with a Route 53 Global Resolver resource. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise... |
| `TagResource` | `POST /tag-resource` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Adds or updates tags for a Route 53 Global Resolver resource. Tags are key-value pairs that help you organize and identify your resources. |
| `UntagResource` | `POST /untag-resource` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Removes tags from a Route 53 Global Resolver resource. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route... |
| `UpdateAccessSource` | `PATCH /access-sources/{accessSourceId}` | `idempotent` | `accessSourceId` | - | `UpdateAccessSourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an access source. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53... |
| `UpdateAccessToken` | `PATCH /tokens/{accessTokenId}` | `idempotent` | `accessTokenId`, `name` | - | `UpdateAccessTokenOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an access token. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53 Global... |
| `UpdateDNSView` | `PATCH /dns-views/{dnsViewId}` | `idempotent` | `dnsViewId` | - | `UpdateDNSViewOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of a DNS view. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53 Global... |
| `UpdateFirewallDomains` | `PATCH /firewall-domain-lists/{firewallDomainListId}/domains` | - | `domains`, `firewallDomainListId`, `operation` | - | `UpdateFirewallDomainsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a DNS Firewall domain list from an array of specified domains. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise... |
| `UpdateFirewallRule` | `PATCH /firewall-rules/{firewallRuleId}` | `idempotent`, `idempotency-token` | `clientToken`, `firewallRuleId` | `clientToken` | `UpdateFirewallRuleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of a DNS firewall rule. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route 53... |
| `UpdateGlobalResolver` | `PATCH /global-resolver/{globalResolverId}` | `idempotent` | `globalResolverId` | - | `UpdateGlobalResolverOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of a Route 53 Global Resolver instance. You can modify the name, description, and observability Region. |
| `UpdateHostedZoneAssociation` | `PATCH /hosted-zone-associations/{hostedZoneAssociationId}` | `idempotent` | `hostedZoneAssociationId` | - | `UpdateHostedZoneAssociationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of a hosted zone association. Route 53 Global Resolver is a global service that supports resolvers in multiple Amazon Web Services Regions but you must specify the US East (Ohio) Region to create, update, or otherwise work with Route... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input parameters are invalid. |
| `AccessDeniedException` | `structure` | `message` | You don't have permission to perform this operation. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | An internal server error occurred. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was throttled due to too many requests. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The specified resource was not found. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The request conflicts with the current state of the resource. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The request would exceed one or more service quotas. |
| `AssociateHostedZoneInput` | `structure` | `hostedZoneId`, `name`, `resourceArn` | - |
| `AssociateHostedZoneOutput` | `structure` | `createdAt`, `hostedZoneId`, `hostedZoneName`, `id`, `name`, `resourceArn`, `status`, `updatedAt` | - |
| `BatchCreateFirewallRuleInput` | `structure` | `firewallRules` | - |
| `BatchCreateFirewallRuleOutput` | `structure` | `failures`, `successes` | - |
| `BatchDeleteFirewallRuleInput` | `structure` | `firewallRules` | - |
| `BatchDeleteFirewallRuleOutput` | `structure` | `failures`, `successes` | - |
| `BatchUpdateFirewallRuleInput` | `structure` | `firewallRules` | - |
| `BatchUpdateFirewallRuleOutput` | `structure` | `failures`, `successes` | - |
| `CreateAccessSourceInput` | `structure` | `cidr`, `clientToken`, `dnsViewId`, `ipAddressType`, `name`, `protocol`, `tags` | - |
| `CreateAccessSourceOutput` | `structure` | `arn`, `cidr`, `createdAt`, `dnsViewId`, `id`, `ipAddressType`, `name`, `protocol`, `status`, `updatedAt` | - |
| `CreateAccessTokenInput` | `structure` | `clientToken`, `dnsViewId`, `expiresAt`, `name`, `tags` | - |
| `CreateAccessTokenOutput` | `structure` | `arn`, `clientToken`, `createdAt`, `dnsViewId`, `expiresAt`, `id`, `name`, `status`, `value` | - |
| `CreateDNSViewInput` | `structure` | `clientToken`, `description`, `dnssecValidation`, `ednsClientSubnet`, `firewallRulesFailOpen`, `globalResolverId`, `name`, `tags` | - |
| `CreateDNSViewOutput` | `structure` | `arn`, `clientToken`, `createdAt`, `description`, `dnssecValidation`, `ednsClientSubnet`, `firewallRulesFailOpen`, `globalResolverId`, `id`, `name`, `status`, `updatedAt` | - |
| `CreateFirewallDomainListInput` | `structure` | `clientToken`, `description`, `globalResolverId`, `name`, `tags` | - |
| `CreateFirewallDomainListOutput` | `structure` | `arn`, `createdAt`, `description`, `domainCount`, `globalResolverId`, `id`, `name`, `status`, `updatedAt` | - |
| `CreateFirewallRuleInput` | `structure` | `action`, `blockOverrideDnsType`, `blockOverrideDomain`, `blockOverrideTtl`, `blockResponse`, `clientToken`, `confidenceThreshold`, `description`, `dnsAdvancedProtection`, `dnsViewId`, `firewallDomainListId`, `name`, ... (+2) | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
