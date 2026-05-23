# Firewall Management Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the Firewall Manager API Reference . This guide is for developers who need detailed information about the Firewall Manager API actions, data types, and errors. For detailed information about Firewall Manager features, see the Firewall Manager Developer Guide. Some API actions require explicit resource permissions. For information, see the developer guide topic Service roles for Firewall Manager.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Firewall Management Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Firewall Management Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Firewall Management Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Put`, `Delete`, `Associate` operation families, including `ListAdminAccountsForOrganization`, `ListAdminsManagingAccount`, `ListAppsLists`, `ListComplianceStatus`, `GetAdminAccount`, `GetAdminScope`.

## Service Identity and Protocol

- AWS model slug: `fms`
- AWS SDK for Rust slug: `fms`
- Model version: `2018-01-01`
- Model file: `vendor/api-models-aws/models/fms/service/2018-01-01/fms-2018-01-01.json`
- SDK ID: `FMS`
- Endpoint prefix: `fms`
- ARN namespace: `fms`
- CloudFormation name: `FMS`
- CloudTrail event source: `fms.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Get` (11), `Put` (6), `Delete` (5), `Associate` (2), `Batch` (2), `Disassociate` (2), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAdminAccount`, `AssociateThirdPartyFirewall`, `BatchAssociateResource`, `BatchDisassociateResource`, `DeleteAppsList`, `DeleteNotificationChannel`, `DeletePolicy`, `DeleteProtocolsList`, `DeleteResourceSet`, `DisassociateAdminAccount`, `DisassociateThirdPartyFirewall`, `PutAdminAccount`, `PutAppsList`, `PutNotificationChannel`, `PutPolicy`, `PutProtocolsList`, `PutResourceSet`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAdminAccount`, `GetAdminScope`, `GetAppsList`, `GetComplianceDetail`, `GetNotificationChannel`, `GetPolicy`, `GetProtectionStatus`, `GetProtocolsList`, `GetResourceSet`, `GetThirdPartyFirewallAssociationStatus`, `GetViolationDetails`, `ListAdminAccountsForOrganization`, `ListAdminsManagingAccount`, `ListAppsLists`, `ListComplianceStatus`, `ListDiscoveredResources`, `ListMemberAccounts`, `ListPolicies`, `ListProtocolsLists`, `ListResourceSetResources`, ... (+3).
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 42 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `EC2/VPC`, `STS`.

## Operation Groups

### List

- Operations: `ListAdminAccountsForOrganization`, `ListAdminsManagingAccount`, `ListAppsLists`, `ListComplianceStatus`, `ListDiscoveredResources`, `ListMemberAccounts`, `ListPolicies`, `ListProtocolsLists`, `ListResourceSetResources`, `ListResourceSets`, `ListTagsForResource`, `ListThirdPartyFirewallFirewallPolicies`
- Traits: `paginated` (8)
- Common required input members in this group: `Identifier`, `MaxResults`, `MemberAccountIds`, `PolicyId`, `ResourceArn`, `ResourceType`, `ThirdPartyFirewall`

### Get

- Operations: `GetAdminAccount`, `GetAdminScope`, `GetAppsList`, `GetComplianceDetail`, `GetNotificationChannel`, `GetPolicy`, `GetProtectionStatus`, `GetProtocolsList`, `GetResourceSet`, `GetThirdPartyFirewallAssociationStatus`, `GetViolationDetails`
- Common required input members in this group: `AdminAccount`, `Identifier`, `ListId`, `MemberAccount`, `PolicyId`, `ResourceId`, `ResourceType`, `ThirdPartyFirewall`

### Put

- Operations: `PutAdminAccount`, `PutAppsList`, `PutNotificationChannel`, `PutPolicy`, `PutProtocolsList`, `PutResourceSet`
- Common required input members in this group: `AdminAccount`, `AppsList`, `Policy`, `ProtocolsList`, `ResourceSet`, `SnsRoleName`, `SnsTopicArn`

### Delete

- Operations: `DeleteAppsList`, `DeleteNotificationChannel`, `DeletePolicy`, `DeleteProtocolsList`, `DeleteResourceSet`
- Common required input members in this group: `Identifier`, `ListId`, `PolicyId`

### Associate

- Operations: `AssociateAdminAccount`, `AssociateThirdPartyFirewall`
- Common required input members in this group: `AdminAccount`, `ThirdPartyFirewall`

### Batch

- Operations: `BatchAssociateResource`, `BatchDisassociateResource`
- Common required input members in this group: `Items`, `ResourceSetIdentifier`

### Disassociate

- Operations: `DisassociateAdminAccount`, `DisassociateThirdPartyFirewall`
- Common required input members in this group: `ThirdPartyFirewall`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `TagList`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAdminAccount` | - | - | `AdminAccount` | - | `Unit` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Sets a Firewall Manager default administrator account. The Firewall Manager default administrator account can manage third-party firewalls and has full administrative scope that allows administration of all policy types, accounts, organizational units, and... |
| `AssociateThirdPartyFirewall` | - | - | `ThirdPartyFirewall` | - | `AssociateThirdPartyFirewallResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Sets the Firewall Manager policy administrator as a tenant administrator of a third-party firewall service. A tenant is an instance of the third-party firewall service that's associated with your Amazon Web Services customer account. |
| `BatchAssociateResource` | - | - | `Items`, `ResourceSetIdentifier` | - | `BatchAssociateResourceResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Associate resources to a Firewall Manager resource set. |
| `BatchDisassociateResource` | - | - | `Items`, `ResourceSetIdentifier` | - | `BatchDisassociateResourceResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Disassociates resources from a Firewall Manager resource set. |
| `DeleteAppsList` | - | - | `ListId` | - | `Unit` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Permanently deletes an Firewall Manager applications list. |
| `DeleteNotificationChannel` | - | - | - | - | `Unit` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Deletes an Firewall Manager association with the IAM role and the Amazon Simple Notification Service (SNS) topic that is used to record Firewall Manager SNS logs. |
| `DeletePolicy` | - | - | `PolicyId` | - | `Unit` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Permanently deletes an Firewall Manager policy. |
| `DeleteProtocolsList` | - | - | `ListId` | - | `Unit` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Permanently deletes an Firewall Manager protocols list. |
| `DeleteResourceSet` | - | - | `Identifier` | - | `Unit` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Deletes the specified ResourceSet. |
| `DisassociateAdminAccount` | - | - | - | - | `Unit` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Disassociates an Firewall Manager administrator account. To set a different account as an Firewall Manager administrator, submit a PutAdminAccount request. |
| `DisassociateThirdPartyFirewall` | - | - | `ThirdPartyFirewall` | - | `DisassociateThirdPartyFirewallResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Disassociates a Firewall Manager policy administrator from a third-party firewall tenant. When you call `DisassociateThirdPartyFirewall`, the third-party firewall vendor deletes all of the firewalls that are associated with the account. |
| `GetAdminAccount` | - | - | - | - | `GetAdminAccountResponse` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Returns the Organizations account that is associated with Firewall Manager as the Firewall Manager default administrator. |
| `GetAdminScope` | - | - | `AdminAccount` | - | `GetAdminScopeResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Returns information about the specified account's administrative scope. The administrative scope defines the resources that an Firewall Manager administrator can manage. |
| `GetAppsList` | - | - | `ListId` | - | `GetAppsListResponse` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Returns information about the specified Firewall Manager applications list. |
| `GetComplianceDetail` | - | - | `MemberAccount`, `PolicyId` | - | `GetComplianceDetailResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Returns detailed compliance information about the specified member account. Details include resources that are in and out of compliance with the specified policy. |
| `GetNotificationChannel` | - | - | - | - | `GetNotificationChannelResponse` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Information about the Amazon Simple Notification Service (SNS) topic that is used to record Firewall Manager SNS logs. |
| `GetPolicy` | - | - | `PolicyId` | - | `GetPolicyResponse` | `InternalErrorException`, `InvalidOperationException`, `InvalidTypeException`, `ResourceNotFoundException` | Returns information about the specified Firewall Manager policy. |
| `GetProtectionStatus` | - | - | `PolicyId` | - | `GetProtectionStatusResponse` | `InternalErrorException`, `InvalidInputException`, `ResourceNotFoundException` | If you created a Shield Advanced policy, returns policy-level attack summary information in the event of a potential DDoS attack. Other policy types are currently unsupported. |
| `GetProtocolsList` | - | - | `ListId` | - | `GetProtocolsListResponse` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Returns information about the specified Firewall Manager protocols list. |
| `GetResourceSet` | - | - | `Identifier` | - | `GetResourceSetResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Gets information about a specific resource set. |
| `GetThirdPartyFirewallAssociationStatus` | - | - | `ThirdPartyFirewall` | - | `GetThirdPartyFirewallAssociationStatusResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | The onboarding status of a Firewall Manager admin account to third-party firewall vendor tenant. |
| `GetViolationDetails` | - | - | `MemberAccount`, `PolicyId`, `ResourceId`, `ResourceType` | - | `GetViolationDetailsResponse` | `InternalErrorException`, `InvalidInputException`, `ResourceNotFoundException` | Retrieves violations for a resource based on the specified Firewall Manager policy and Amazon Web Services account. |
| `ListAdminAccountsForOrganization` | - | `paginated` | - | - | `ListAdminAccountsForOrganizationResponse` | `InternalErrorException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Returns a `AdminAccounts` object that lists the Firewall Manager administrators within the organization that are onboarded to Firewall Manager by AssociateAdminAccount. This operation can be called only from the organization's management account. |
| `ListAdminsManagingAccount` | - | `paginated` | - | - | `ListAdminsManagingAccountResponse` | `InternalErrorException`, `InvalidInputException`, `ResourceNotFoundException` | Lists the accounts that are managing the specified Organizations member account. This is useful for any member account so that they can view the accounts who are managing their account. |
| `ListAppsLists` | - | `paginated` | `MaxResults` | - | `ListAppsListsResponse` | `InternalErrorException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Returns an array of `AppsListDataSummary` objects. |
| `ListComplianceStatus` | - | `paginated` | `PolicyId` | - | `ListComplianceStatusResponse` | `InternalErrorException`, `ResourceNotFoundException` | Returns an array of `PolicyComplianceStatus` objects. Use `PolicyComplianceStatus` to get a summary of which member accounts are protected by the specified policy. |
| `ListDiscoveredResources` | - | - | `MemberAccountIds`, `ResourceType` | - | `ListDiscoveredResourcesResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException` | Returns an array of resources in the organization's accounts that are available to be associated with a resource set. |
| `ListMemberAccounts` | - | `paginated` | - | - | `ListMemberAccountsResponse` | `InternalErrorException`, `ResourceNotFoundException` | Returns a `MemberAccounts` object that lists the member accounts in the administrator's Amazon Web Services organization. Either an Firewall Manager administrator or the organization's management account can make this request. |
| `ListPolicies` | - | `paginated` | - | - | `ListPoliciesResponse` | `InternalErrorException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Returns an array of `PolicySummary` objects. |
| `ListProtocolsLists` | - | `paginated` | `MaxResults` | - | `ListProtocolsListsResponse` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Returns an array of `ProtocolsListDataSummary` objects. |
| `ListResourceSetResources` | - | - | `Identifier` | - | `ListResourceSetResourcesResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Returns an array of resources that are currently associated to a resource set. |
| `ListResourceSets` | - | - | - | - | `ListResourceSetsResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException` | Returns an array of `ResourceSetSummary` objects. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Retrieves the list of tags for the specified Amazon Web Services resource. |
| `ListThirdPartyFirewallFirewallPolicies` | - | `paginated` | `MaxResults`, `ThirdPartyFirewall` | - | `ListThirdPartyFirewallFirewallPoliciesResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Retrieves a list of all of the third-party firewall policies that are associated with the third-party firewall administrator's account. |
| `PutAdminAccount` | - | - | `AdminAccount` | - | `Unit` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `LimitExceededException` | Creates or updates an Firewall Manager administrator account. The account must be a member of the organization that was onboarded to Firewall Manager by AssociateAdminAccount. |
| `PutAppsList` | - | - | `AppsList` | - | `PutAppsListResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Creates an Firewall Manager applications list. |
| `PutNotificationChannel` | - | - | `SnsRoleName`, `SnsTopicArn` | - | `Unit` | `InternalErrorException`, `InvalidOperationException`, `ResourceNotFoundException` | Designates the IAM role and Amazon Simple Notification Service (SNS) topic that Firewall Manager uses to record SNS logs. To perform this action outside of the console, you must first configure the SNS topic's access policy to allow the `SnsRoleName` to... |
| `PutPolicy` | - | - | `Policy` | - | `PutPolicyResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException` | Creates an Firewall Manager policy. A Firewall Manager policy is specific to the individual policy type. |
| `PutProtocolsList` | - | - | `ProtocolsList` | - | `PutProtocolsListResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Creates an Firewall Manager protocols list. |
| `PutResourceSet` | - | - | `ResourceSet` | - | `PutResourceSetResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `LimitExceededException` | Creates the resource set. An Firewall Manager resource set defines the resources to import into an Firewall Manager policy from another Amazon Web Services service. |
| `TagResource` | - | - | `ResourceArn`, `TagList` | - | `TagResourceResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Adds one or more tags to an Amazon Web Services resource. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalErrorException`, `InvalidInputException`, `InvalidOperationException`, `ResourceNotFoundException` | Removes one or more tags from an Amazon Web Services resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalErrorException` | `structure` | `Message` | The operation failed because of a system problem, even though the request was valid. |
| `ResourceNotFoundException` | `structure` | `Message` | The specified resource was not found. |
| `InvalidOperationException` | `structure` | `Message` | The operation failed because there was nothing to do or the operation wasn't possible. |
| `InvalidInputException` | `structure` | `Message` | The parameters of the request were invalid. |
| `LimitExceededException` | `structure` | `Message` | The operation exceeds a resource limit, for example, the maximum number of `policy` objects that you can create for an Amazon Web Services account. |
| `InvalidTypeException` | `structure` | `Message` | The value of the `Type` parameter is invalid. |
| `AssociateAdminAccountRequest` | `structure` | `AdminAccount` | - |
| `AssociateThirdPartyFirewallRequest` | `structure` | `ThirdPartyFirewall` | - |
| `AssociateThirdPartyFirewallResponse` | `structure` | `ThirdPartyFirewallStatus` | - |
| `BatchAssociateResourceRequest` | `structure` | `Items`, `ResourceSetIdentifier` | - |
| `BatchAssociateResourceResponse` | `structure` | `FailedItems`, `ResourceSetIdentifier` | - |
| `BatchDisassociateResourceRequest` | `structure` | `Items`, `ResourceSetIdentifier` | - |
| `BatchDisassociateResourceResponse` | `structure` | `FailedItems`, `ResourceSetIdentifier` | - |
| `DeleteAppsListRequest` | `structure` | `ListId` | - |
| `DeleteNotificationChannelRequest` | `structure` | - | - |
| `DeletePolicyRequest` | `structure` | `DeleteAllPolicyResources`, `PolicyId` | - |
| `DeleteProtocolsListRequest` | `structure` | `ListId` | - |
| `DeleteResourceSetRequest` | `structure` | `Identifier` | - |
| `DisassociateAdminAccountRequest` | `structure` | - | - |
| `DisassociateThirdPartyFirewallRequest` | `structure` | `ThirdPartyFirewall` | - |
| `DisassociateThirdPartyFirewallResponse` | `structure` | `ThirdPartyFirewallStatus` | - |
| `GetAdminAccountRequest` | `structure` | - | - |
| `GetAdminAccountResponse` | `structure` | `AdminAccount`, `RoleStatus` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
