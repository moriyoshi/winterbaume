# Amazon Route 53

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Route 53 is a highly available and scalable Domain Name System (DNS) web service. You can use Route 53 to: Register domain names. For more information, see How domain registration works. Route internet traffic to the resources for your domain For more information, see How internet traffic is routed to your website or web application. Check the health of your resources.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-route53/tests/scenario_test.rs`: provision a DNS hosted zone, change record sets, inspect change status, and clean up.
- Backported from `scenario_test.rs`: reuse a delegation set across multiple zones.
- Backported from `scenario_test.rs`: manage CIDR collections and CIDR blocks.
- Scenario insight from EC2: include mutable binding failover for Amazon Route 53 where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Route 53 by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Route 53 by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: support hosted zone and record management, health checks, reusable delegation sets, traffic policies, CIDR routing collections, DNSSEC, change batches, and paginated inventory.

## Service Identity and Protocol

- AWS model slug: `route-53`
- AWS SDK for Rust slug: `route53`
- Model version: `2013-04-01`
- Model file: `vendor/api-models-aws/models/route-53/service/2013-04-01/route-53-2013-04-01.json`
- SDK ID: `Route 53`
- Endpoint prefix: `route53`
- ARN namespace: `route53`
- CloudFormation name: `Route53`
- CloudTrail event source: `route53.amazonaws.com`
- Protocols: `restXml`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (19), `Get` (18), `Create` (10), `Delete` (9), `Update` (5), `Change` (3), `Activate` (1), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateVPCWithHostedZone`, `CreateCidrCollection`, `CreateHealthCheck`, `CreateHostedZone`, `CreateKeySigningKey`, `CreateQueryLoggingConfig`, `CreateReusableDelegationSet`, `CreateTrafficPolicy`, `CreateTrafficPolicyInstance`, `CreateTrafficPolicyVersion`, `CreateVPCAssociationAuthorization`, `DeleteCidrCollection`, `DeleteHealthCheck`, `DeleteHostedZone`, `DeleteKeySigningKey`, `DeleteQueryLoggingConfig`, `DeleteReusableDelegationSet`, `DeleteTrafficPolicy`, `DeleteTrafficPolicyInstance`, `DeleteVPCAssociationAuthorization`, ... (+8).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountLimit`, `GetChange`, `GetCheckerIpRanges`, `GetDNSSEC`, `GetGeoLocation`, `GetHealthCheck`, `GetHealthCheckCount`, `GetHealthCheckLastFailureReason`, `GetHealthCheckStatus`, `GetHostedZone`, `GetHostedZoneCount`, `GetHostedZoneLimit`, `GetQueryLoggingConfig`, `GetReusableDelegationSet`, `GetReusableDelegationSetLimit`, `GetTrafficPolicy`, `GetTrafficPolicyInstance`, `GetTrafficPolicyInstanceCount`, `ListCidrBlocks`, `ListCidrCollections`, ... (+17).
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 68 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/Route53/latest/APIReference/API_ChangeResourceRecordSets.html
- https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html
- https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/hosted-zones-private.html

Research outcomes:
- ChangeResourceRecordSets applies a change batch transactionally. Route 53 validates the batch and then applies all changes or none.
- DELETE requires the record set values to match the existing record set. Deleting the same record set more than once in one batch returns InvalidChangeBatch.
- CREATE creates a record set, DELETE removes one, and UPSERT creates or updates depending on current existence.
- Submitted changes propagate asynchronously. GetChange reports PENDING until authoritative DNS propagation is complete, then INSYNC.
- Routing policies include simple, failover, geolocation, geoproximity, latency, IP-based, multivalue answer, and weighted routing.
- Multivalue answer routing can return up to eight healthy records chosen at random.
- Private hosted zones answer only within associated VPCs or connected hybrid networks. Queries from outside are resolved on the internet instead.
- Private hosted zones have reserved NS records and are resolved by VPC Resolver through private connectivity rather than public name server lookup.

Parity implications:
- Hosted zone record mutations need transactional batch validation, exact DELETE matching, duplicate-change detection, and asynchronous change records.
- Record resolution should understand routing policy fields, health checks, alias records, private/public zone visibility, and VPC associations.
- Public and private hosted zones should share record-set APIs but differ in DNS visibility and resolver-side behaviour.

## Current Network Resource Stub Semantics

Route 53 currently stores private hosted zone VPC associations as Route 53-local state.

- Hosted zone records include a list of associated VPC IDs and VPC regions.
- VPC association authorisations are keyed by hosted zone and VPC metadata inside Route 53 state.
- Associate and disassociate operations update the hosted zone's local VPC list and do not verify EC2 VPC existence or ownership.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListCidrBlocks`, `ListCidrCollections`, `ListCidrLocations`, `ListGeoLocations`, `ListHealthChecks`, `ListHostedZones`, `ListHostedZonesByName`, `ListHostedZonesByVPC`, `ListQueryLoggingConfigs`, `ListResourceRecordSets`, `ListReusableDelegationSets`, `ListTagsForResource`, `ListTagsForResources`, `ListTrafficPolicies`, `ListTrafficPolicyInstances`, `ListTrafficPolicyInstancesByHostedZone`, `ListTrafficPolicyInstancesByPolicy`, `ListTrafficPolicyVersions`, `ListVPCAssociationAuthorizations`
- Traits: `paginated` (6)
- Common required input members in this group: `CollectionId`, `HostedZoneId`, `ResourceType`

### Get

- Operations: `GetAccountLimit`, `GetChange`, `GetCheckerIpRanges`, `GetDNSSEC`, `GetGeoLocation`, `GetHealthCheck`, `GetHealthCheckCount`, `GetHealthCheckLastFailureReason`, `GetHealthCheckStatus`, `GetHostedZone`, `GetHostedZoneCount`, `GetHostedZoneLimit`, `GetQueryLoggingConfig`, `GetReusableDelegationSet`, `GetReusableDelegationSetLimit`, `GetTrafficPolicy`, `GetTrafficPolicyInstance`, `GetTrafficPolicyInstanceCount`
- Common required input members in this group: `Type`, `Id`, `HostedZoneId`, `HealthCheckId`

### Create

- Operations: `CreateCidrCollection`, `CreateHealthCheck`, `CreateHostedZone`, `CreateKeySigningKey`, `CreateQueryLoggingConfig`, `CreateReusableDelegationSet`, `CreateTrafficPolicy`, `CreateTrafficPolicyInstance`, `CreateTrafficPolicyVersion`, `CreateVPCAssociationAuthorization`
- Common required input members in this group: `Name`, `CallerReference`, `HostedZoneId`, `Document`

### Delete

- Operations: `DeleteCidrCollection`, `DeleteHealthCheck`, `DeleteHostedZone`, `DeleteKeySigningKey`, `DeleteQueryLoggingConfig`, `DeleteReusableDelegationSet`, `DeleteTrafficPolicy`, `DeleteTrafficPolicyInstance`, `DeleteVPCAssociationAuthorization`
- Common required input members in this group: `Id`, `HostedZoneId`

### Update

- Operations: `UpdateHealthCheck`, `UpdateHostedZoneComment`, `UpdateHostedZoneFeatures`, `UpdateTrafficPolicyComment`, `UpdateTrafficPolicyInstance`
- Common required input members in this group: `Id`

### Change

- Operations: `ChangeCidrCollection`, `ChangeResourceRecordSets`, `ChangeTagsForResource`
- Common required input members in this group: -

### Activate

- Operations: `ActivateKeySigningKey`
- Common required input members in this group: -

### Associate

- Operations: `AssociateVPCWithHostedZone`
- Common required input members in this group: -

### Deactivate

- Operations: `DeactivateKeySigningKey`
- Common required input members in this group: -

### Disable

- Operations: `DisableHostedZoneDNSSEC`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateVPCFromHostedZone`
- Common required input members in this group: -

### Enable

- Operations: `EnableHostedZoneDNSSEC`
- Common required input members in this group: -

### Test

- Operations: `TestDNSAnswer`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ActivateKeySigningKey` | `POST /2013-04-01/keysigningkey/{HostedZoneId}/{Name}/activate` | - | `HostedZoneId`, `Name` | - | `ActivateKeySigningKeyResponse` | `ConcurrentModification`, `InvalidInput`, `InvalidKeySigningKeyStatus`, `InvalidKMSArn`, `InvalidSigningStatus`, `NoSuchKeySigningKey` | Activates a key-signing key (KSK) so that it can be used for signing by DNSSEC. This operation changes the KSK status to ACTIVE . |
| `AssociateVPCWithHostedZone` | `POST /2013-04-01/hostedzone/{HostedZoneId}/associatevpc` | - | `HostedZoneId`, `VPC` | - | `AssociateVPCWithHostedZoneResponse` | `ConflictingDomainExists`, `InvalidInput`, `InvalidVPCId`, `LimitsExceeded`, `NoSuchHostedZone`, `NotAuthorizedException`, `PriorRequestNotComplete`, `PublicZoneVPCAssociation` | Associates an Amazon VPC with a private hosted zone. To perform the association, the VPC and the private hosted zone must already exist. You can't convert a public hosted zone into a private hosted zone. If you want ... |
| `ChangeCidrCollection` | `POST /2013-04-01/cidrcollection/{Id}` | - | `Id`, `Changes` | - | `ChangeCidrCollectionResponse` | `CidrBlockInUseException`, `CidrCollectionVersionMismatchException`, `ConcurrentModification`, `InvalidInput`, `LimitsExceeded`, `NoSuchCidrCollectionException` | Creates, changes, or deletes CIDR blocks within a collection. Contains authoritative IP information mapping blocks to one or multiple locations. A change request can update multiple locations in a collection at a tim ... |
| `ChangeResourceRecordSets` | `POST /2013-04-01/hostedzone/{HostedZoneId}/rrset` | - | `HostedZoneId`, `ChangeBatch` | - | `ChangeResourceRecordSetsResponse` | `InvalidChangeBatch`, `InvalidInput`, `NoSuchHealthCheck`, `NoSuchHostedZone`, `PriorRequestNotComplete` | Creates, changes, or deletes a resource record set, which contains authoritative DNS information for a specified domain name or subdomain name. For example, you can use ChangeResourceRecordSets to create a resource r ... |
| `ChangeTagsForResource` | `POST /2013-04-01/tags/{ResourceType}/{ResourceId}` | - | `ResourceType`, `ResourceId` | - | `ChangeTagsForResourceResponse` | `InvalidInput`, `NoSuchHealthCheck`, `NoSuchHostedZone`, `PriorRequestNotComplete`, `ThrottlingException` | Adds, edits, or deletes tags for a health check or a hosted zone. For information about using tags for cost allocation, see Using Cost Allocation Tags in the Billing and Cost Management User Guide . |
| `CreateCidrCollection` | `POST /2013-04-01/cidrcollection` | - | `Name`, `CallerReference` | - | `CreateCidrCollectionResponse` | `CidrCollectionAlreadyExistsException`, `ConcurrentModification`, `InvalidInput`, `LimitsExceeded` | Creates a CIDR collection in the current Amazon Web Services account. |
| `CreateHealthCheck` | `POST /2013-04-01/healthcheck` | - | `CallerReference`, `HealthCheckConfig` | - | `CreateHealthCheckResponse` | `HealthCheckAlreadyExists`, `InvalidInput`, `TooManyHealthChecks` | Creates a new health check. For information about adding health checks to resource record sets, see HealthCheckId in ChangeResourceRecordSets . ELB Load Balancers If you're registering EC2 instances with an Elastic L ... |
| `CreateHostedZone` | `POST /2013-04-01/hostedzone` | - | `Name`, `CallerReference` | - | `CreateHostedZoneResponse` | `ConflictingDomainExists`, `DelegationSetNotAvailable`, `DelegationSetNotReusable`, `HostedZoneAlreadyExists`, `InvalidDomainName`, `InvalidInput`, `InvalidVPCId`, `NoSuchDelegationSet`, `TooManyHostedZones` | Creates a new public or private hosted zone. You create records in a public hosted zone to define how you want to route traffic on the internet for a domain, such as example.com, and its subdomains (apex.example.com, ... |
| `CreateKeySigningKey` | `POST /2013-04-01/keysigningkey` | - | `CallerReference`, `HostedZoneId`, `KeyManagementServiceArn`, `Name`, `Status` | - | `CreateKeySigningKeyResponse` | `ConcurrentModification`, `InvalidArgument`, `InvalidInput`, `InvalidKeySigningKeyName`, `InvalidKeySigningKeyStatus`, `InvalidKMSArn`, `InvalidSigningStatus`, `KeySigningKeyAlreadyExists`, `NoSuchHostedZone`, `TooManyKeySigningKeys` | Creates a new key-signing key (KSK) associated with a hosted zone. You can only have two KSKs per hosted zone. |
| `CreateQueryLoggingConfig` | `POST /2013-04-01/queryloggingconfig` | - | `HostedZoneId`, `CloudWatchLogsLogGroupArn` | - | `CreateQueryLoggingConfigResponse` | `ConcurrentModification`, `InsufficientCloudWatchLogsResourcePolicy`, `InvalidInput`, `NoSuchCloudWatchLogsLogGroup`, `NoSuchHostedZone`, `QueryLoggingConfigAlreadyExists` | Creates a configuration for DNS query logging. After you create a query logging configuration, Amazon Route 53 begins to publish log data to an Amazon CloudWatch Logs log group. DNS query logs contain information abo ... |
| `CreateReusableDelegationSet` | `POST /2013-04-01/delegationset` | - | `CallerReference` | - | `CreateReusableDelegationSetResponse` | `DelegationSetAlreadyCreated`, `DelegationSetAlreadyReusable`, `DelegationSetNotAvailable`, `HostedZoneNotFound`, `InvalidArgument`, `InvalidInput`, `LimitsExceeded` | Creates a delegation set (a group of four name servers) that can be reused by multiple hosted zones that were created by the same Amazon Web Services account. You can also create a reusable delegation set that uses t ... |
| `CreateTrafficPolicy` | `POST /2013-04-01/trafficpolicy` | - | `Name`, `Document` | - | `CreateTrafficPolicyResponse` | `InvalidInput`, `InvalidTrafficPolicyDocument`, `TooManyTrafficPolicies`, `TrafficPolicyAlreadyExists` | Creates a traffic policy, which you use to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com). |
| `CreateTrafficPolicyInstance` | `POST /2013-04-01/trafficpolicyinstance` | - | `HostedZoneId`, `Name`, `TTL`, `TrafficPolicyId`, `TrafficPolicyVersion` | - | `CreateTrafficPolicyInstanceResponse` | `InvalidInput`, `NoSuchHostedZone`, `NoSuchTrafficPolicy`, `TooManyTrafficPolicyInstances`, `TrafficPolicyInstanceAlreadyExists` | Creates resource record sets in a specified hosted zone based on the settings in a specified traffic policy version. In addition, CreateTrafficPolicyInstance associates the resource record sets with a specified domai ... |
| `CreateTrafficPolicyVersion` | `POST /2013-04-01/trafficpolicy/{Id}` | - | `Id`, `Document` | - | `CreateTrafficPolicyVersionResponse` | `ConcurrentModification`, `InvalidInput`, `InvalidTrafficPolicyDocument`, `NoSuchTrafficPolicy`, `TooManyTrafficPolicyVersionsForCurrentPolicy` | Creates a new version of an existing traffic policy. When you create a new version of a traffic policy, you specify the ID of the traffic policy that you want to update and a JSON-formatted document that describes th ... |
| `CreateVPCAssociationAuthorization` | `POST /2013-04-01/hostedzone/{HostedZoneId}/authorizevpcassociation` | - | `HostedZoneId`, `VPC` | - | `CreateVPCAssociationAuthorizationResponse` | `ConcurrentModification`, `InvalidInput`, `InvalidVPCId`, `NoSuchHostedZone`, `TooManyVPCAssociationAuthorizations` | Authorizes the Amazon Web Services account that created a specified VPC to submit an AssociateVPCWithHostedZone request to associate the VPC with a specified hosted zone that was created by a different account. To su ... |
| `DeactivateKeySigningKey` | `POST /2013-04-01/keysigningkey/{HostedZoneId}/{Name}/deactivate` | - | `HostedZoneId`, `Name` | - | `DeactivateKeySigningKeyResponse` | `ConcurrentModification`, `InvalidInput`, `InvalidKeySigningKeyStatus`, `InvalidSigningStatus`, `KeySigningKeyInParentDSRecord`, `KeySigningKeyInUse`, `NoSuchKeySigningKey` | Deactivates a key-signing key (KSK) so that it will not be used for signing by DNSSEC. This operation changes the KSK status to INACTIVE . |
| `DeleteCidrCollection` | `DELETE /2013-04-01/cidrcollection/{Id}` | - | `Id` | - | `DeleteCidrCollectionResponse` | `CidrCollectionInUseException`, `ConcurrentModification`, `InvalidInput`, `NoSuchCidrCollectionException` | Deletes a CIDR collection in the current Amazon Web Services account. The collection must be empty before it can be deleted. |
| `DeleteHealthCheck` | `DELETE /2013-04-01/healthcheck/{HealthCheckId}` | - | `HealthCheckId` | - | `DeleteHealthCheckResponse` | `HealthCheckInUse`, `InvalidInput`, `NoSuchHealthCheck` | Deletes a health check. Amazon Route 53 does not prevent you from deleting a health check even if the health check is associated with one or more resource record sets. If you delete a health check and you don't updat ... |
| `DeleteHostedZone` | `DELETE /2013-04-01/hostedzone/{Id}` | - | `Id` | - | `DeleteHostedZoneResponse` | `HostedZoneNotEmpty`, `InvalidDomainName`, `InvalidInput`, `NoSuchHostedZone`, `PriorRequestNotComplete` | Deletes a hosted zone. If the hosted zone was created by another service, such as Cloud Map, see Deleting Public Hosted Zones That Were Created by Another Service in the Amazon Route 53 Developer Guide for informatio ... |
| `DeleteKeySigningKey` | `DELETE /2013-04-01/keysigningkey/{HostedZoneId}/{Name}` | - | `HostedZoneId`, `Name` | - | `DeleteKeySigningKeyResponse` | `ConcurrentModification`, `InvalidInput`, `InvalidKeySigningKeyStatus`, `InvalidKMSArn`, `InvalidSigningStatus`, `NoSuchKeySigningKey` | Deletes a key-signing key (KSK). Before you can delete a KSK, you must deactivate it. The KSK must be deactivated before you can delete it regardless of whether the hosted zone is enabled for DNSSEC signing. You can ... |
| `DeleteQueryLoggingConfig` | `DELETE /2013-04-01/queryloggingconfig/{Id}` | - | `Id` | - | `DeleteQueryLoggingConfigResponse` | `ConcurrentModification`, `InvalidInput`, `NoSuchQueryLoggingConfig` | Deletes a configuration for DNS query logging. If you delete a configuration, Amazon Route 53 stops sending query logs to CloudWatch Logs. Route 53 doesn't delete any logs that are already in CloudWatch Logs. For mor ... |
| `DeleteReusableDelegationSet` | `DELETE /2013-04-01/delegationset/{Id}` | - | `Id` | - | `DeleteReusableDelegationSetResponse` | `DelegationSetInUse`, `DelegationSetNotReusable`, `InvalidInput`, `NoSuchDelegationSet` | Deletes a reusable delegation set. You can delete a reusable delegation set only if it isn't associated with any hosted zones. To verify that the reusable delegation set is not associated with any hosted zones, submi ... |
| `DeleteTrafficPolicy` | `DELETE /2013-04-01/trafficpolicy/{Id}/{Version}` | - | `Id`, `Version` | - | `DeleteTrafficPolicyResponse` | `ConcurrentModification`, `InvalidInput`, `NoSuchTrafficPolicy`, `TrafficPolicyInUse` | Deletes a traffic policy. When you delete a traffic policy, Route 53 sets a flag on the policy to indicate that it has been deleted. However, Route 53 never fully deletes the traffic policy. Note the following: Delet ... |
| `DeleteTrafficPolicyInstance` | `DELETE /2013-04-01/trafficpolicyinstance/{Id}` | - | `Id` | - | `DeleteTrafficPolicyInstanceResponse` | `InvalidInput`, `NoSuchTrafficPolicyInstance`, `PriorRequestNotComplete` | Deletes a traffic policy instance and all of the resource record sets that Amazon Route 53 created when you created the instance. In the Route 53 console, traffic policy instances are known as policy records. |
| `DeleteVPCAssociationAuthorization` | `POST /2013-04-01/hostedzone/{HostedZoneId}/deauthorizevpcassociation` | - | `HostedZoneId`, `VPC` | - | `DeleteVPCAssociationAuthorizationResponse` | `ConcurrentModification`, `InvalidInput`, `InvalidVPCId`, `NoSuchHostedZone`, `VPCAssociationAuthorizationNotFound` | Removes authorization to submit an AssociateVPCWithHostedZone request to associate a specified VPC with a hosted zone that was created by a different account. You must use the account that created the hosted zone to ... |
| `DisableHostedZoneDNSSEC` | `POST /2013-04-01/hostedzone/{HostedZoneId}/disable-dnssec` | - | `HostedZoneId` | - | `DisableHostedZoneDNSSECResponse` | `ConcurrentModification`, `DNSSECNotFound`, `InvalidArgument`, `InvalidInput`, `InvalidKeySigningKeyStatus`, `InvalidKMSArn`, `KeySigningKeyInParentDSRecord`, `NoSuchHostedZone` | Disables DNSSEC signing in a specific hosted zone. This action does not deactivate any key-signing keys (KSKs) that are active in the hosted zone. |
| `DisassociateVPCFromHostedZone` | `POST /2013-04-01/hostedzone/{HostedZoneId}/disassociatevpc` | - | `HostedZoneId`, `VPC` | - | `DisassociateVPCFromHostedZoneResponse` | `InvalidInput`, `InvalidVPCId`, `LastVPCAssociation`, `NoSuchHostedZone`, `VPCAssociationNotFound` | Disassociates an Amazon Virtual Private Cloud (Amazon VPC) from an Amazon Route 53 private hosted zone. Note the following: You can't disassociate the last Amazon VPC from a private hosted zone. You can't convert a p ... |
| `EnableHostedZoneDNSSEC` | `POST /2013-04-01/hostedzone/{HostedZoneId}/enable-dnssec` | - | `HostedZoneId` | - | `EnableHostedZoneDNSSECResponse` | `ConcurrentModification`, `DNSSECNotFound`, `HostedZonePartiallyDelegated`, `InvalidArgument`, `InvalidInput`, `InvalidKeySigningKeyStatus`, `InvalidKMSArn`, `KeySigningKeyWithActiveStatusNotFound`, `NoSuchHostedZone` | Enables DNSSEC signing in a specific hosted zone. |
| `GetAccountLimit` | `GET /2013-04-01/accountlimit/{Type}` | - | `Type` | - | `GetAccountLimitResponse` | `InvalidInput` | Gets the specified limit for the current account, for example, the maximum number of health checks that you can create using the account. For the default limit, see Limits in the Amazon Route 53 Developer Guide . To ... |
| `GetChange` | `GET /2013-04-01/change/{Id}` | - | `Id` | - | `GetChangeResponse` | `InvalidInput`, `NoSuchChange` | Returns the current status of a change batch request. The status is one of the following values: PENDING indicates that the changes in this request have not propagated to all Amazon Route 53 DNS servers managing the ... |
| `GetCheckerIpRanges` | `GET /2013-04-01/checkeripranges` | - | - | - | `GetCheckerIpRangesResponse` | - | Route 53 does not perform authorization for this API because it retrieves information that is already available to the public. GetCheckerIpRanges still works, but we recommend that you download ip-ranges.json, which ... |
| `GetDNSSEC` | `GET /2013-04-01/hostedzone/{HostedZoneId}/dnssec` | - | `HostedZoneId` | - | `GetDNSSECResponse` | `InvalidArgument`, `InvalidInput`, `NoSuchHostedZone` | Returns information about DNSSEC for a specific hosted zone, including the key-signing keys (KSKs) in the hosted zone. |
| `GetGeoLocation` | `GET /2013-04-01/geolocation` | - | - | - | `GetGeoLocationResponse` | `InvalidInput`, `NoSuchGeoLocation` | Gets information about whether a specified geographic location is supported for Amazon Route 53 geolocation resource record sets. Route 53 does not perform authorization for this API because it retrieves information ... |
| `GetHealthCheck` | `GET /2013-04-01/healthcheck/{HealthCheckId}` | - | `HealthCheckId` | - | `GetHealthCheckResponse` | `IncompatibleVersion`, `InvalidInput`, `NoSuchHealthCheck` | Gets information about a specified health check. |
| `GetHealthCheckCount` | `GET /2013-04-01/healthcheckcount` | - | - | - | `GetHealthCheckCountResponse` | - | Retrieves the number of health checks that are associated with the current Amazon Web Services account. |
| `GetHealthCheckLastFailureReason` | `GET /2013-04-01/healthcheck/{HealthCheckId}/lastfailurereason` | - | `HealthCheckId` | - | `GetHealthCheckLastFailureReasonResponse` | `InvalidInput`, `NoSuchHealthCheck` | Gets the reason that a specified health check failed most recently. |
| `GetHealthCheckStatus` | `GET /2013-04-01/healthcheck/{HealthCheckId}/status` | - | `HealthCheckId` | - | `GetHealthCheckStatusResponse` | `InvalidInput`, `NoSuchHealthCheck` | Gets status of a specified health check. This API is intended for use during development to diagnose behavior. It doesn’t support production use-cases with high query rates that require immediate and actionable respo ... |
| `GetHostedZone` | `GET /2013-04-01/hostedzone/{Id}` | - | `Id` | - | `GetHostedZoneResponse` | `InvalidInput`, `NoSuchHostedZone` | Gets information about a specified hosted zone including the four name servers assigned to the hosted zone. returns the VPCs associated with the specified hosted zone and does not reflect the VPC associations by Rout ... |
| `GetHostedZoneCount` | `GET /2013-04-01/hostedzonecount` | - | - | - | `GetHostedZoneCountResponse` | `InvalidInput` | Retrieves the number of hosted zones that are associated with the current Amazon Web Services account. |
| `GetHostedZoneLimit` | `GET /2013-04-01/hostedzonelimit/{HostedZoneId}/{Type}` | - | `Type`, `HostedZoneId` | - | `GetHostedZoneLimitResponse` | `HostedZoneNotPrivate`, `InvalidInput`, `NoSuchHostedZone` | Gets the specified limit for a specified hosted zone, for example, the maximum number of records that you can create in the hosted zone. For the default limit, see Limits in the Amazon Route 53 Developer Guide . To r ... |
| `GetQueryLoggingConfig` | `GET /2013-04-01/queryloggingconfig/{Id}` | - | `Id` | - | `GetQueryLoggingConfigResponse` | `InvalidInput`, `NoSuchQueryLoggingConfig` | Gets information about a specified configuration for DNS query logging. For more information about DNS query logs, see CreateQueryLoggingConfig and Logging DNS Queries . |
| `GetReusableDelegationSet` | `GET /2013-04-01/delegationset/{Id}` | - | `Id` | - | `GetReusableDelegationSetResponse` | `DelegationSetNotReusable`, `InvalidInput`, `NoSuchDelegationSet` | Retrieves information about a specified reusable delegation set, including the four name servers that are assigned to the delegation set. |
| `GetReusableDelegationSetLimit` | `GET /2013-04-01/reusabledelegationsetlimit/{DelegationSetId}/{Type}` | - | `Type`, `DelegationSetId` | - | `GetReusableDelegationSetLimitResponse` | `InvalidInput`, `NoSuchDelegationSet` | Gets the maximum number of hosted zones that you can associate with the specified reusable delegation set. For the default limit, see Limits in the Amazon Route 53 Developer Guide . To request a higher limit, open a ... |
| `GetTrafficPolicy` | `GET /2013-04-01/trafficpolicy/{Id}/{Version}` | - | `Id`, `Version` | - | `GetTrafficPolicyResponse` | `InvalidInput`, `NoSuchTrafficPolicy` | Gets information about a specific traffic policy version. For information about how of deleting a traffic policy affects the response from GetTrafficPolicy , see DeleteTrafficPolicy . |
| `GetTrafficPolicyInstance` | `GET /2013-04-01/trafficpolicyinstance/{Id}` | - | `Id` | - | `GetTrafficPolicyInstanceResponse` | `InvalidInput`, `NoSuchTrafficPolicyInstance` | Gets information about a specified traffic policy instance. Use GetTrafficPolicyInstance with the id of new traffic policy instance to confirm that the CreateTrafficPolicyInstance or an UpdateTrafficPolicyInstance re ... |
| `GetTrafficPolicyInstanceCount` | `GET /2013-04-01/trafficpolicyinstancecount` | - | - | - | `GetTrafficPolicyInstanceCountResponse` | - | Gets the number of traffic policy instances that are associated with the current Amazon Web Services account. |
| `ListCidrBlocks` | `GET /2013-04-01/cidrcollection/{CollectionId}/cidrblocks` | `paginated` | `CollectionId` | - | `ListCidrBlocksResponse` | `InvalidInput`, `NoSuchCidrCollectionException`, `NoSuchCidrLocationException` | Returns a paginated list of location objects and their CIDR blocks. |
| `ListCidrCollections` | `GET /2013-04-01/cidrcollection` | `paginated` | - | - | `ListCidrCollectionsResponse` | `InvalidInput` | Returns a paginated list of CIDR collections in the Amazon Web Services account (metadata only). |
| `ListCidrLocations` | `GET /2013-04-01/cidrcollection/{CollectionId}` | `paginated` | `CollectionId` | - | `ListCidrLocationsResponse` | `InvalidInput`, `NoSuchCidrCollectionException` | Returns a paginated list of CIDR locations for the given collection (metadata only, does not include CIDR blocks). |
| `ListGeoLocations` | `GET /2013-04-01/geolocations` | - | - | - | `ListGeoLocationsResponse` | `InvalidInput` | Retrieves a list of supported geographic locations. Countries are listed first, and continents are listed last. If Amazon Route 53 supports subdivisions for a country (for example, states or provinces), the subdivisi ... |
| `ListHealthChecks` | `GET /2013-04-01/healthcheck` | `paginated` | - | - | `ListHealthChecksResponse` | `IncompatibleVersion`, `InvalidInput` | Retrieve a list of the health checks that are associated with the current Amazon Web Services account. |
| `ListHostedZones` | `GET /2013-04-01/hostedzone` | `paginated` | - | - | `ListHostedZonesResponse` | `DelegationSetNotReusable`, `InvalidInput`, `NoSuchDelegationSet` | Retrieves a list of the public and private hosted zones that are associated with the current Amazon Web Services account. The response includes a HostedZones child element for each hosted zone. Amazon Route 53 return ... |
| `ListHostedZonesByName` | `GET /2013-04-01/hostedzonesbyname` | - | - | - | `ListHostedZonesByNameResponse` | `InvalidDomainName`, `InvalidInput` | Retrieves a list of your hosted zones in lexicographic order. The response includes a HostedZones child element for each hosted zone created by the current Amazon Web Services account. ListHostedZonesByName sorts hos ... |
| `ListHostedZonesByVPC` | `GET /2013-04-01/hostedzonesbyvpc` | - | `VPCId`, `VPCRegion` | - | `ListHostedZonesByVPCResponse` | `InvalidInput`, `InvalidPaginationToken` | Lists all the private hosted zones that a specified VPC is associated with, regardless of which Amazon Web Services account or Amazon Web Services service owns the hosted zones. The HostedZoneOwner structure in the r ... |
| `ListQueryLoggingConfigs` | `GET /2013-04-01/queryloggingconfig` | `paginated` | - | - | `ListQueryLoggingConfigsResponse` | `InvalidInput`, `InvalidPaginationToken`, `NoSuchHostedZone` | Lists the configurations for DNS query logging that are associated with the current Amazon Web Services account or the configuration that is associated with a specified hosted zone. For more information about DNS que ... |
| `ListResourceRecordSets` | `GET /2013-04-01/hostedzone/{HostedZoneId}/rrset` | - | `HostedZoneId` | - | `ListResourceRecordSetsResponse` | `InvalidInput`, `NoSuchHostedZone` | Lists the resource record sets in a specified hosted zone. ListResourceRecordSets returns up to 300 resource record sets at a time in ASCII order, beginning at a position specified by the name and type elements. Sort ... |
| `ListReusableDelegationSets` | `GET /2013-04-01/delegationset` | - | - | - | `ListReusableDelegationSetsResponse` | `InvalidInput` | Retrieves a list of the reusable delegation sets that are associated with the current Amazon Web Services account. |
| `ListTagsForResource` | `GET /2013-04-01/tags/{ResourceType}/{ResourceId}` | - | `ResourceType`, `ResourceId` | - | `ListTagsForResourceResponse` | `InvalidInput`, `NoSuchHealthCheck`, `NoSuchHostedZone`, `PriorRequestNotComplete`, `ThrottlingException` | Lists tags for one health check or hosted zone. For information about using tags for cost allocation, see Using Cost Allocation Tags in the Billing and Cost Management User Guide . |
| `ListTagsForResources` | `POST /2013-04-01/tags/{ResourceType}` | - | `ResourceType`, `ResourceIds` | - | `ListTagsForResourcesResponse` | `InvalidInput`, `NoSuchHealthCheck`, `NoSuchHostedZone`, `PriorRequestNotComplete`, `ThrottlingException` | Lists tags for up to 10 health checks or hosted zones. For information about using tags for cost allocation, see Using Cost Allocation Tags in the Billing and Cost Management User Guide . |
| `ListTrafficPolicies` | `GET /2013-04-01/trafficpolicies` | - | - | - | `ListTrafficPoliciesResponse` | `InvalidInput` | Gets information about the latest version for every traffic policy that is associated with the current Amazon Web Services account. Policies are listed in the order that they were created in. For information about ho ... |
| `ListTrafficPolicyInstances` | `GET /2013-04-01/trafficpolicyinstances` | - | - | - | `ListTrafficPolicyInstancesResponse` | `InvalidInput`, `NoSuchTrafficPolicyInstance` | Gets information about the traffic policy instances that you created by using the current Amazon Web Services account. After you submit an UpdateTrafficPolicyInstance request, there's a brief delay while Amazon Route ... |
| `ListTrafficPolicyInstancesByHostedZone` | `GET /2013-04-01/trafficpolicyinstances/hostedzone` | - | `HostedZoneId` | - | `ListTrafficPolicyInstancesByHostedZoneResponse` | `InvalidInput`, `NoSuchHostedZone`, `NoSuchTrafficPolicyInstance` | Gets information about the traffic policy instances that you created in a specified hosted zone. After you submit a CreateTrafficPolicyInstance or an UpdateTrafficPolicyInstance request, there's a brief delay while A ... |
| `ListTrafficPolicyInstancesByPolicy` | `GET /2013-04-01/trafficpolicyinstances/trafficpolicy` | - | `TrafficPolicyId`, `TrafficPolicyVersion` | - | `ListTrafficPolicyInstancesByPolicyResponse` | `InvalidInput`, `NoSuchTrafficPolicy`, `NoSuchTrafficPolicyInstance` | Gets information about the traffic policy instances that you created by using a specify traffic policy version. After you submit a CreateTrafficPolicyInstance or an UpdateTrafficPolicyInstance request, there's a brie ... |
| `ListTrafficPolicyVersions` | `GET /2013-04-01/trafficpolicies/{Id}/versions` | - | `Id` | - | `ListTrafficPolicyVersionsResponse` | `InvalidInput`, `NoSuchTrafficPolicy` | Gets information about all of the versions for a specified traffic policy. Traffic policy versions are listed in numerical order by VersionNumber . |
| `ListVPCAssociationAuthorizations` | `GET /2013-04-01/hostedzone/{HostedZoneId}/authorizevpcassociation` | - | `HostedZoneId` | - | `ListVPCAssociationAuthorizationsResponse` | `InvalidInput`, `InvalidPaginationToken`, `NoSuchHostedZone` | Gets a list of the VPCs that were created by other accounts and that can be associated with a specified hosted zone because you've submitted one or more CreateVPCAssociationAuthorization requests. The response includ ... |
| `TestDNSAnswer` | `GET /2013-04-01/testdnsanswer` | - | `HostedZoneId`, `RecordName`, `RecordType` | - | `TestDNSAnswerResponse` | `InvalidInput`, `NoSuchHostedZone` | Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a sub ... |
| `UpdateHealthCheck` | `POST /2013-04-01/healthcheck/{HealthCheckId}` | - | `HealthCheckId` | - | `UpdateHealthCheckResponse` | `HealthCheckVersionMismatch`, `InvalidInput`, `NoSuchHealthCheck` | Updates an existing health check. Note that some values can't be updated. For more information about updating health checks, see Creating, Updating, and Deleting Health Checks in the Amazon Route 53 Developer Guide . |
| `UpdateHostedZoneComment` | `POST /2013-04-01/hostedzone/{Id}` | - | `Id` | - | `UpdateHostedZoneCommentResponse` | `InvalidInput`, `NoSuchHostedZone`, `PriorRequestNotComplete` | Updates the comment for a specified hosted zone. |
| `UpdateHostedZoneFeatures` | `POST /2013-04-01/hostedzone/{HostedZoneId}/features` | - | `HostedZoneId` | - | `UpdateHostedZoneFeaturesResponse` | `InvalidInput`, `LimitsExceeded`, `NoSuchHostedZone`, `PriorRequestNotComplete` | Updates the features configuration for a hosted zone. This operation allows you to enable or disable specific features for your hosted zone, such as accelerated recovery. Accelerated recovery enables you to update DN ... |
| `UpdateTrafficPolicyComment` | `POST /2013-04-01/trafficpolicy/{Id}/{Version}` | - | `Id`, `Version`, `Comment` | - | `UpdateTrafficPolicyCommentResponse` | `ConcurrentModification`, `InvalidInput`, `NoSuchTrafficPolicy` | Updates the comment for a specified traffic policy version. |
| `UpdateTrafficPolicyInstance` | `POST /2013-04-01/trafficpolicyinstance/{Id}` | - | `Id`, `TTL`, `TrafficPolicyId`, `TrafficPolicyVersion` | - | `UpdateTrafficPolicyInstanceResponse` | `ConflictingTypes`, `InvalidInput`, `NoSuchTrafficPolicy`, `NoSuchTrafficPolicyInstance`, `PriorRequestNotComplete` | After you submit a UpdateTrafficPolicyInstance request, there's a brief delay while Route 53 creates the resource record sets that are specified in the traffic policy definition. Use GetTrafficPolicyInstance with the ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetGeoLocation` | - | `ContinentCode -> continentcode`, `CountryCode -> countrycode`, `SubdivisionCode -> subdivisioncode` | - | - |
| `ListCidrBlocks` | - | `LocationName -> location`, `NextToken -> nexttoken`, `MaxResults -> maxresults` | - | - |
| `ListCidrCollections` | - | `NextToken -> nexttoken`, `MaxResults -> maxresults` | - | - |
| `ListCidrLocations` | - | `NextToken -> nexttoken`, `MaxResults -> maxresults` | - | - |
| `ListGeoLocations` | - | `StartContinentCode -> startcontinentcode`, `StartCountryCode -> startcountrycode`, `StartSubdivisionCode -> startsubdivisioncode`, `MaxItems -> maxitems` | - | - |
| `ListHealthChecks` | - | `Marker -> marker`, `MaxItems -> maxitems` | - | - |
| `ListHostedZones` | - | `Marker -> marker`, `MaxItems -> maxitems`, `DelegationSetId -> delegationsetid`, `HostedZoneType -> hostedzonetype` | - | - |
| `ListHostedZonesByName` | - | `DNSName -> dnsname`, `HostedZoneId -> hostedzoneid`, `MaxItems -> maxitems` | - | - |
| `ListHostedZonesByVPC` | - | `VPCId -> vpcid`, `VPCRegion -> vpcregion`, `MaxItems -> maxitems`, `NextToken -> nexttoken` | - | - |
| `ListQueryLoggingConfigs` | - | `HostedZoneId -> hostedzoneid`, `NextToken -> nexttoken`, `MaxResults -> maxresults` | - | - |
| `ListResourceRecordSets` | - | `StartRecordName -> name`, `StartRecordType -> type`, `StartRecordIdentifier -> identifier`, `MaxItems -> maxitems` | - | - |
| `ListReusableDelegationSets` | - | `Marker -> marker`, `MaxItems -> maxitems` | - | - |
| `ListTrafficPolicies` | - | `TrafficPolicyIdMarker -> trafficpolicyid`, `MaxItems -> maxitems` | - | - |
| `ListTrafficPolicyInstances` | - | `HostedZoneIdMarker -> hostedzoneid`, `TrafficPolicyInstanceNameMarker -> trafficpolicyinstancename`, `TrafficPolicyInstanceTypeMarker -> trafficpolicyinstancetype`, `MaxItems -> maxitems` | - | - |
| `ListTrafficPolicyInstancesByHostedZone` | - | `HostedZoneId -> id`, `TrafficPolicyInstanceNameMarker -> trafficpolicyinstancename`, `TrafficPolicyInstanceTypeMarker -> trafficpolicyinstancetype`, `MaxItems -> maxitems` | - | - |
| `ListTrafficPolicyInstancesByPolicy` | - | `TrafficPolicyId -> id`, `TrafficPolicyVersion -> version`, `HostedZoneIdMarker -> hostedzoneid`, `TrafficPolicyInstanceNameMarker -> trafficpolicyinstancename`, `TrafficPolicyInstanceTypeMarker -> trafficpolicyinstancetype`, `MaxItems -> maxitems` | - | - |
| `ListTrafficPolicyVersions` | - | `TrafficPolicyVersionMarker -> trafficpolicyversion`, `MaxItems -> maxitems` | - | - |
| `ListVPCAssociationAuthorizations` | - | `NextToken -> nexttoken`, `MaxResults -> maxresults` | - | - |
| `TestDNSAnswer` | - | `HostedZoneId -> hostedzoneid`, `RecordName -> recordname`, `RecordType -> recordtype`, `ResolverIP -> resolverip`, `EDNS0ClientSubnetIP -> edns0clientsubnetip`, `EDNS0ClientSubnetMask -> edns0clientsubnetmask` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CidrBlockInUseException` | `structure` | Message | This CIDR block is already in use. |
| `CidrCollectionAlreadyExistsException` | `structure` | Message | A CIDR collection with this name and a different caller reference already exists in this account. |
| `CidrCollectionInUseException` | `structure` | Message | This CIDR collection is in use, and isn't empty. |
| `CidrCollectionVersionMismatchException` | `structure` | Message | The CIDR collection version you provided, doesn't match the one in the ListCidrCollections operation. |
| `ConcurrentModification` | `structure` | message | Another user submitted a request to create, update, or delete the object at the same time that you did. Retry the request. |
| `ConflictingDomainExists` | `structure` | message | The cause of this error depends on the operation that you're performing: Create a public hosted zone: Two hosted zones that have the same name or that have ... |
| `ConflictingTypes` | `structure` | message | You tried to update a traffic policy instance by using a traffic policy version that has a different DNS type than the current type for the instance. You sp ... |
| `DNSSECNotFound` | `structure` | message | The hosted zone doesn't have any DNSSEC resources. |
| `DelegationSetAlreadyCreated` | `structure` | message | A delegation set with the same owner and caller reference combination has already been created. |
| `DelegationSetAlreadyReusable` | `structure` | message | The specified delegation set has already been marked as reusable. |
| `DelegationSetInUse` | `structure` | message | The specified delegation contains associated hosted zones which must be deleted before the reusable delegation set can be deleted. |
| `DelegationSetNotAvailable` | `structure` | message | You can create a hosted zone that has the same name as an existing hosted zone (example.com is common), but there is a limit to the number of hosted zones t ... |
| `DelegationSetNotReusable` | `structure` | message | A reusable delegation set with the specified ID does not exist. |
| `HealthCheckAlreadyExists` | `structure` | message | The health check you're attempting to create already exists. Amazon Route 53 returns this error when you submit a request that has the following values: The ... |
| `HealthCheckInUse` | `structure` | message | This error code is not in use. |
| `HealthCheckVersionMismatch` | `structure` | message | The value of HealthCheckVersion in the request doesn't match the value of HealthCheckVersion in the health check. |
| `HostedZoneAlreadyExists` | `structure` | message | The hosted zone you're trying to create already exists. Amazon Route 53 returns this error when a hosted zone has already been created with the specified Ca ... |
| `HostedZoneNotEmpty` | `structure` | message | The hosted zone contains resource records that are not SOA or NS records. |
| `HostedZoneNotFound` | `structure` | message | The specified HostedZone can't be found. |
| `HostedZoneNotPrivate` | `structure` | message | The specified hosted zone is a public hosted zone, not a private hosted zone. |
| `HostedZonePartiallyDelegated` | `structure` | message | The hosted zone nameservers don't match the parent nameservers. The hosted zone and parent must have the same nameservers. |
| `IncompatibleVersion` | `structure` | message | The resource you're trying to access is unsupported on this Amazon Route 53 endpoint. |
| `InsufficientCloudWatchLogsResourcePolicy` | `structure` | message | Amazon Route 53 doesn't have the permissions required to create log streams and send query logs to log streams. Possible causes include the following: There ... |
| `InvalidArgument` | `structure` | message | Parameter name is not valid. |
| `InvalidChangeBatch` | `structure` | messages, message | This exception contains a list of messages that might contain one or more error messages. Each error message indicates one error in the change batch. |
| `InvalidDomainName` | `structure` | message | The specified domain name is not valid. |
| `InvalidInput` | `structure` | message | The input is not valid. |
| `InvalidKMSArn` | `structure` | message | The KeyManagementServiceArn that you specified isn't valid to use with DNSSEC signing. |
| `InvalidKeySigningKeyName` | `structure` | message | The key-signing key (KSK) name that you specified isn't a valid name. |
| `InvalidKeySigningKeyStatus` | `structure` | message | The key-signing key (KSK) status isn't valid or another KSK has the status INTERNAL_FAILURE . |
| `InvalidPaginationToken` | `structure` | message | The value that you specified to get the second or subsequent page of results is invalid. |
| `InvalidSigningStatus` | `structure` | message | Your hosted zone status isn't valid for this operation. In the hosted zone, change the status to enable DNSSEC or disable DNSSEC . |
| `InvalidTrafficPolicyDocument` | `structure` | message | The format of the traffic policy document that you specified in the Document element is not valid. |
| `InvalidVPCId` | `structure` | message | The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC. |
| `KeySigningKeyAlreadyExists` | `structure` | message | You've already created a key-signing key (KSK) with this name or with the same customer managed key ARN. |
| `KeySigningKeyInParentDSRecord` | `structure` | message | The key-signing key (KSK) is specified in a parent DS record. |
| `KeySigningKeyInUse` | `structure` | message | The key-signing key (KSK) that you specified can't be deactivated because it's the only KSK for a currently-enabled DNSSEC. Disable DNSSEC signing, or add o ... |
| `KeySigningKeyWithActiveStatusNotFound` | `structure` | message | A key-signing key (KSK) with ACTIVE status wasn't found. |
| `LastVPCAssociation` | `structure` | message | The VPC that you're trying to disassociate from the private hosted zone is the last VPC that is associated with the hosted zone. Amazon Route 53 doesn't sup ... |
| `LimitsExceeded` | `structure` | message | This operation can't be completed because the current account has reached the limit on the resource you are trying to create. To request a higher limit, cre ... |
| `NoSuchChange` | `structure` | message | A change with the specified change ID does not exist. |
| `NoSuchCidrCollectionException` | `structure` | Message | The CIDR collection you specified, doesn't exist. |
| `NoSuchCidrLocationException` | `structure` | Message | The CIDR collection location doesn't match any locations in your account. |
| `NoSuchCloudWatchLogsLogGroup` | `structure` | message | There is no CloudWatch Logs log group with the specified ARN. |
| `NoSuchDelegationSet` | `structure` | message | A reusable delegation set with the specified ID does not exist. |
| `NoSuchGeoLocation` | `structure` | message | Amazon Route 53 doesn't support the specified geographic location. For a list of supported geolocation codes, see the GeoLocation data type. |
| `NoSuchHealthCheck` | `structure` | message | No health check exists with the specified ID. |
| `NoSuchHostedZone` | `structure` | message | No hosted zone exists with the ID that you specified. |
| `NoSuchKeySigningKey` | `structure` | message | The specified key-signing key (KSK) doesn't exist. |
| `NoSuchQueryLoggingConfig` | `structure` | message | There is no DNS query logging configuration with the specified ID. |
| `NoSuchTrafficPolicy` | `structure` | message | No traffic policy exists with the specified ID. |
| `NoSuchTrafficPolicyInstance` | `structure` | message | No traffic policy instance exists with the specified ID. |
| `NotAuthorizedException` | `structure` | message | Associating the specified VPC with the specified hosted zone has not been authorized. |
| `PriorRequestNotComplete` | `structure` | message | If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an HTTP 4 ... |
| `PublicZoneVPCAssociation` | `structure` | message | You're trying to associate a VPC with a public hosted zone. Amazon Route 53 doesn't support associating a VPC with a public hosted zone. |
| `QueryLoggingConfigAlreadyExists` | `structure` | message | You can create only one query logging configuration for a hosted zone, and a query logging configuration already exists for this hosted zone. |
| `ThrottlingException` | `structure` | message | The limit on the number of requests per second was exceeded. |
| `TooManyHealthChecks` | `structure` | message | This health check can't be created because the current account has reached the limit on the number of active health checks. For information about default li ... |
| `TooManyHostedZones` | `structure` | message | This operation can't be completed either because the current account has reached the limit on the number of hosted zones or because you've reached the limit ... |
| `TooManyKeySigningKeys` | `structure` | message | You've reached the limit for the number of key-signing keys (KSKs). Remove at least one KSK, and then try again. |
| `TooManyTrafficPolicies` | `structure` | message | This traffic policy can't be created because the current account has reached the limit on the number of traffic policies. For information about default limi ... |
| `TooManyTrafficPolicyInstances` | `structure` | message | This traffic policy instance can't be created because the current account has reached the limit on the number of traffic policy instances. For information a ... |
| `TooManyTrafficPolicyVersionsForCurrentPolicy` | `structure` | message | This traffic policy version can't be created because you've reached the limit of 1000 on the number of versions that you can create for the current traffic ... |
| `TooManyVPCAssociationAuthorizations` | `structure` | message | You've created the maximum number of authorizations that can be created for the specified hosted zone. To authorize another VPC to be associated with the ho ... |
| `TrafficPolicyAlreadyExists` | `structure` | message | A traffic policy that has the same value for Name already exists. |
| `TrafficPolicyInUse` | `structure` | message | One or more traffic policy instances were created by using the specified traffic policy. |
| `TrafficPolicyInstanceAlreadyExists` | `structure` | message | There is already a traffic policy instance with the specified ID. |
| `VPCAssociationAuthorizationNotFound` | `structure` | message | The VPC that you specified is not authorized to be associated with the hosted zone. |
| `VPCAssociationNotFound` | `structure` | message | The specified VPC and hosted zone are not currently associated. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
