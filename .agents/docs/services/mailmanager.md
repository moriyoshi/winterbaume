# MailManager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon SES Mail Manager API The Amazon SES Mail Manager API contains operations and data types that comprise the Mail Manager feature of Amazon Simple Email Service (SES). Mail Manager is a set of Amazon SES email gateway features designed to help you strengthen your organization's email infrastructure, simplify email workflow management, and streamline email compliance control. To learn more, see the Mail Manager chapter in the Amazon SES Developer Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for MailManager where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: add full state-machine walks for MailManager resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented MailManager workflows in the local mock. Key resources include `AddonInstanceResource`, `AddonSubscriptionResource`, `AddressListResource`, `ArchiveResource`, `IngressPointResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetAddonInstance`, `GetAddonSubscription`, `GetAddressList`, `GetAddressListImportJob`, `ListAddonInstances`, `ListAddonSubscriptions`.

## Service Identity and Protocol

- AWS model slug: `mailmanager`
- AWS SDK for Rust slug: `ses`
- Model version: `2023-10-17`
- Model file: `vendor/api-models-aws/models/mailmanager/service/2023-10-17/mailmanager-2023-10-17.json`
- SDK ID: `MailManager`
- Endpoint prefix: `mail-manager`
- ARN namespace: `ses`
- CloudFormation name: `SES`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (15), `List` (13), `Create` (9), `Delete` (8), `Update` (5), `Start` (3), `Stop` (3), `Deregister` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAddonInstance`, `CreateAddonSubscription`, `CreateAddressList`, `CreateAddressListImportJob`, `CreateArchive`, `CreateIngressPoint`, `CreateRelay`, `CreateRuleSet`, `CreateTrafficPolicy`, `DeleteAddonInstance`, `DeleteAddonSubscription`, `DeleteAddressList`, `DeleteArchive`, `DeleteIngressPoint`, `DeleteRelay`, `DeleteRuleSet`, `DeleteTrafficPolicy`, `DeregisterMemberFromAddressList`, `RegisterMemberToAddressList`, `StartAddressListImportJob`, ... (+12).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAddonInstance`, `GetAddonSubscription`, `GetAddressList`, `GetAddressListImportJob`, `GetArchive`, `GetArchiveExport`, `GetArchiveMessage`, `GetArchiveMessageContent`, `GetArchiveSearch`, `GetArchiveSearchResults`, `GetIngressPoint`, `GetMemberOfAddressList`, `GetRelay`, `GetRuleSet`, `GetTrafficPolicy`, `ListAddonInstances`, `ListAddonSubscriptions`, `ListAddressListImportJobs`, `ListAddressLists`, `ListArchiveExports`, ... (+8).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 29 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateAddressListImportJob`, `GetAddressListImportJob`, `GetArchiveExport`, `ListAddressListImportJobs`, `ListArchiveExports`, `StartAddressListImportJob`, `StartArchiveExport`, `StartArchiveSearch`, `StopAddressListImportJob`, `StopArchiveExport`, `StopArchiveSearch`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 60 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `EC2/VPC`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AddonInstanceResource` | `AddonInstanceId` | create: `CreateAddonInstance`; read: `GetAddonInstance`; delete: `DeleteAddonInstance`; list: `ListAddonInstances` | - | - |
| `AddonSubscriptionResource` | `AddonSubscriptionId` | create: `CreateAddonSubscription`; read: `GetAddonSubscription`; delete: `DeleteAddonSubscription`; list: `ListAddonSubscriptions` | - | - |
| `AddressListResource` | `AddressListId` | create: `CreateAddressList`; read: `GetAddressList`; delete: `DeleteAddressList`; list: `ListAddressLists` | - | - |
| `ArchiveResource` | `ArchiveId` | create: `CreateArchive`; read: `GetArchive`; update: `UpdateArchive`; delete: `DeleteArchive`; list: `ListArchives` | - | - |
| `IngressPointResource` | `IngressPointId` | create: `CreateIngressPoint`; read: `GetIngressPoint`; update: `UpdateIngressPoint`; delete: `DeleteIngressPoint`; list: `ListIngressPoints` | - | - |
| `RelayResource` | `RelayId` | create: `CreateRelay`; read: `GetRelay`; update: `UpdateRelay`; delete: `DeleteRelay`; list: `ListRelays` | - | - |
| `RuleSetResource` | `RuleSetId` | create: `CreateRuleSet`; read: `GetRuleSet`; update: `UpdateRuleSet`; delete: `DeleteRuleSet`; list: `ListRuleSets` | - | - |
| `TrafficPolicyResource` | `TrafficPolicyId` | create: `CreateTrafficPolicy`; read: `GetTrafficPolicy`; update: `UpdateTrafficPolicy`; delete: `DeleteTrafficPolicy`; list: `ListTrafficPolicies` | - | - |
## Operation Groups

### Get

- Operations: `GetAddonInstance`, `GetAddonSubscription`, `GetAddressList`, `GetAddressListImportJob`, `GetArchive`, `GetArchiveExport`, `GetArchiveMessage`, `GetArchiveMessageContent`, `GetArchiveSearch`, `GetArchiveSearchResults`, `GetIngressPoint`, `GetMemberOfAddressList`, `GetRelay`, `GetRuleSet`, `GetTrafficPolicy`
- Traits: `readonly` (13)
- Common required input members in this group: `AddonInstanceId`, `AddonSubscriptionId`, `Address`, `AddressListId`, `ArchiveId`, `ArchivedMessageId`, `ExportId`, `IngressPointId`, `JobId`, `RelayId`, `RuleSetId`, `SearchId`, `TrafficPolicyId`

### List

- Operations: `ListAddonInstances`, `ListAddonSubscriptions`, `ListAddressListImportJobs`, `ListAddressLists`, `ListArchiveExports`, `ListArchiveSearches`, `ListArchives`, `ListIngressPoints`, `ListMembersOfAddressList`, `ListRelays`, `ListRuleSets`, `ListTagsForResource`, `ListTrafficPolicies`
- Traits: `paginated` (12), `readonly` (13)
- Common required input members in this group: `AddressListId`, `ArchiveId`, `ResourceArn`

### Create

- Operations: `CreateAddonInstance`, `CreateAddonSubscription`, `CreateAddressList`, `CreateAddressListImportJob`, `CreateArchive`, `CreateIngressPoint`, `CreateRelay`, `CreateRuleSet`, `CreateTrafficPolicy`
- Traits: `idempotency-token` (9), `idempotent` (9)
- Common required input members in this group: `AddonName`, `AddonSubscriptionId`, `AddressListId`, `AddressListName`, `ArchiveName`, `Authentication`, `DefaultAction`, `ImportDataFormat`, `IngressPointName`, `Name`, `PolicyStatements`, `RelayName`, `RuleSetId`, `RuleSetName`, `Rules`, `ServerName`, `ServerPort`, `TrafficPolicyId`, `TrafficPolicyName`, `Type`

### Delete

- Operations: `DeleteAddonInstance`, `DeleteAddonSubscription`, `DeleteAddressList`, `DeleteArchive`, `DeleteIngressPoint`, `DeleteRelay`, `DeleteRuleSet`, `DeleteTrafficPolicy`
- Traits: `idempotent` (8)
- Common required input members in this group: `AddonInstanceId`, `AddonSubscriptionId`, `AddressListId`, `ArchiveId`, `IngressPointId`, `RelayId`, `RuleSetId`, `TrafficPolicyId`

### Update

- Operations: `UpdateArchive`, `UpdateIngressPoint`, `UpdateRelay`, `UpdateRuleSet`, `UpdateTrafficPolicy`
- Traits: `idempotent` (5)
- Common required input members in this group: `ArchiveId`, `IngressPointId`, `RelayId`, `RuleSetId`, `TrafficPolicyId`

### Start

- Operations: `StartAddressListImportJob`, `StartArchiveExport`, `StartArchiveSearch`
- Traits: `idempotent` (1)
- Common required input members in this group: `ArchiveId`, `ExportDestinationConfiguration`, `FromTimestamp`, `JobId`, `MaxResults`, `ToTimestamp`

### Stop

- Operations: `StopAddressListImportJob`, `StopArchiveExport`, `StopArchiveSearch`
- Traits: `idempotent` (2)
- Common required input members in this group: `ExportId`, `JobId`, `SearchId`

### Deregister

- Operations: `DeregisterMemberFromAddressList`
- Traits: `idempotent` (1)
- Common required input members in this group: `Address`, `AddressListId`

### Register

- Operations: `RegisterMemberToAddressList`
- Traits: `idempotent` (1)
- Common required input members in this group: `Address`, `AddressListId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAddonInstance` | - | `idempotent`, `idempotency-token` | `AddonSubscriptionId` | `ClientToken` | `CreateAddonInstanceResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an Add On instance for the subscription indicated in the request. The resulting Amazon Resource Name (ARN) can be used in a conditional statement for a rule set or traffic policy. |
| `CreateAddonSubscription` | - | `idempotent`, `idempotency-token` | `AddonName` | `ClientToken` | `CreateAddonSubscriptionResponse` | `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a subscription for an Add On representing the acceptance of its terms of use and additional pricing. The subscription can then be used to create an instance for use in rule sets or traffic policies. |
| `CreateAddressList` | - | `idempotent`, `idempotency-token` | `AddressListName` | `ClientToken` | `CreateAddressListResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new address list. |
| `CreateAddressListImportJob` | - | `idempotent`, `idempotency-token` | `AddressListId`, `ImportDataFormat`, `Name` | `ClientToken` | `CreateAddressListImportJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an import job for an address list. |
| `CreateArchive` | - | `idempotent`, `idempotency-token` | `ArchiveName` | `ClientToken` | `CreateArchiveResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new email archive resource for storing and retaining emails. |
| `CreateIngressPoint` | - | `idempotent`, `idempotency-token` | `IngressPointName`, `RuleSetId`, `TrafficPolicyId`, `Type` | `ClientToken` | `CreateIngressPointResponse` | `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Provision a new ingress endpoint resource. |
| `CreateRelay` | - | `idempotent`, `idempotency-token` | `Authentication`, `RelayName`, `ServerName`, `ServerPort` | `ClientToken` | `CreateRelayResponse` | `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a relay resource which can be used in rules to relay incoming emails to defined relay destinations. |
| `CreateRuleSet` | - | `idempotent`, `idempotency-token` | `RuleSetName`, `Rules` | `ClientToken` | `CreateRuleSetResponse` | `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Provision a new rule set. |
| `CreateTrafficPolicy` | - | `idempotent`, `idempotency-token` | `DefaultAction`, `PolicyStatements`, `TrafficPolicyName` | `ClientToken` | `CreateTrafficPolicyResponse` | `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Provision a new traffic policy resource. |
| `DeleteAddonInstance` | - | `idempotent` | `AddonInstanceId` | - | `DeleteAddonInstanceResponse` | `ConflictException`, `ValidationException` | Deletes an Add On instance. |
| `DeleteAddonSubscription` | - | `idempotent` | `AddonSubscriptionId` | - | `DeleteAddonSubscriptionResponse` | `ConflictException`, `ValidationException` | Deletes an Add On subscription. |
| `DeleteAddressList` | - | `idempotent` | `AddressListId` | - | `DeleteAddressListResponse` | `AccessDeniedException`, `ConflictException`, `ThrottlingException` | Deletes an address list. |
| `DeleteArchive` | - | `idempotent` | `ArchiveId` | - | `DeleteArchiveResponse` | `AccessDeniedException`, `ConflictException`, `ThrottlingException`, `ValidationException` | Initiates deletion of an email archive. This changes the archive state to pending deletion. |
| `DeleteIngressPoint` | - | `idempotent` | `IngressPointId` | - | `DeleteIngressPointResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Delete an ingress endpoint resource. |
| `DeleteRelay` | - | `idempotent` | `RelayId` | - | `DeleteRelayResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes an existing relay resource. |
| `DeleteRuleSet` | - | `idempotent` | `RuleSetId` | - | `DeleteRuleSetResponse` | `ConflictException`, `ValidationException` | Delete a rule set. |
| `DeleteTrafficPolicy` | - | `idempotent` | `TrafficPolicyId` | - | `DeleteTrafficPolicyResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Delete a traffic policy resource. |
| `DeregisterMemberFromAddressList` | - | `idempotent` | `Address`, `AddressListId` | - | `DeregisterMemberFromAddressListResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a member from an address list. |
| `GetAddonInstance` | - | `readonly` | `AddonInstanceId` | - | `GetAddonInstanceResponse` | `ResourceNotFoundException`, `ValidationException` | Gets detailed information about an Add On instance. |
| `GetAddonSubscription` | - | `readonly` | `AddonSubscriptionId` | - | `GetAddonSubscriptionResponse` | `ResourceNotFoundException`, `ValidationException` | Gets detailed information about an Add On subscription. |
| `GetAddressList` | - | `readonly` | `AddressListId` | - | `GetAddressListResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Fetch attributes of an address list. |
| `GetAddressListImportJob` | - | `readonly` | `JobId` | - | `GetAddressListImportJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Fetch attributes of an import job. |
| `GetArchive` | - | `readonly` | `ArchiveId` | - | `GetArchiveResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the full details and current state of a specified email archive. |
| `GetArchiveExport` | - | - | `ExportId` | - | `GetArchiveExportResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Retrieves the details and current status of a specific email archive export job. |
| `GetArchiveMessage` | - | `readonly` | `ArchivedMessageId` | - | `GetArchiveMessageResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a pre-signed URL that provides temporary download access to the specific email message stored in the archive. |
| `GetArchiveMessageContent` | - | - | `ArchivedMessageId` | - | `GetArchiveMessageContentResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns the textual content of a specific email message stored in the archive. Attachments are not included. |
| `GetArchiveSearch` | - | `readonly` | `SearchId` | - | `GetArchiveSearchResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Retrieves the details and current status of a specific email archive search job. |
| `GetArchiveSearchResults` | - | `readonly` | `SearchId` | - | `GetArchiveSearchResultsResponse` | `AccessDeniedException`, `ConflictException`, `ThrottlingException`, `ValidationException` | Returns the results of a completed email archive search job. |
| `GetIngressPoint` | - | `readonly` | `IngressPointId` | - | `GetIngressPointResponse` | `ResourceNotFoundException`, `ValidationException` | Fetch ingress endpoint resource attributes. |
| `GetMemberOfAddressList` | - | `readonly` | `Address`, `AddressListId` | - | `GetMemberOfAddressListResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Fetch attributes of a member in an address list. |
| `GetRelay` | - | `readonly` | `RelayId` | - | `GetRelayResponse` | `ResourceNotFoundException`, `ValidationException` | Fetch the relay resource and it's attributes. |
| `GetRuleSet` | - | `readonly` | `RuleSetId` | - | `GetRuleSetResponse` | `ResourceNotFoundException`, `ValidationException` | Fetch attributes of a rule set. |
| `GetTrafficPolicy` | - | `readonly` | `TrafficPolicyId` | - | `GetTrafficPolicyResponse` | `ResourceNotFoundException`, `ValidationException` | Fetch attributes of a traffic policy resource. |
| `ListAddonInstances` | - | `readonly`, `paginated` | - | - | `ListAddonInstancesResponse` | `ValidationException` | Lists all Add On instances in your account. |
| `ListAddonSubscriptions` | - | `readonly`, `paginated` | - | - | `ListAddonSubscriptionsResponse` | `ValidationException` | Lists all Add On subscriptions in your account. |
| `ListAddressListImportJobs` | - | `readonly`, `paginated` | `AddressListId` | - | `ListAddressListImportJobsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists jobs for an address list. |
| `ListAddressLists` | - | `readonly`, `paginated` | - | - | `ListAddressListsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Lists address lists for this account. |
| `ListArchiveExports` | - | `readonly`, `paginated` | `ArchiveId` | - | `ListArchiveExportsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of email archive export jobs. |
| `ListArchiveSearches` | - | `readonly`, `paginated` | `ArchiveId` | - | `ListArchiveSearchesResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of email archive search jobs. |
| `ListArchives` | - | `readonly`, `paginated` | - | - | `ListArchivesResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a list of all email archives in your account. |
| `ListIngressPoints` | - | `readonly`, `paginated` | - | - | `ListIngressPointsResponse` | `ValidationException` | List all ingress endpoint resources. |
| `ListMembersOfAddressList` | - | `readonly`, `paginated` | `AddressListId` | - | `ListMembersOfAddressListResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists members of an address list. |
| `ListRelays` | - | `readonly`, `paginated` | - | - | `ListRelaysResponse` | `ValidationException` | Lists all the existing relay resources. |
| `ListRuleSets` | - | `readonly`, `paginated` | - | - | `ListRuleSetsResponse` | `ValidationException` | List rule sets for this account. |
| `ListTagsForResource` | - | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves the list of tags (keys and values) assigned to the resource. |
| `ListTrafficPolicies` | - | `readonly`, `paginated` | - | - | `ListTrafficPoliciesResponse` | `ValidationException` | List traffic policy resources. |
| `RegisterMemberToAddressList` | - | `idempotent` | `Address`, `AddressListId` | - | `RegisterMemberToAddressListResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a member to an address list. |
| `StartAddressListImportJob` | - | `idempotent` | `JobId` | - | `StartAddressListImportJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts an import job for an address list. |
| `StartArchiveExport` | - | - | `ArchiveId`, `ExportDestinationConfiguration`, `FromTimestamp`, `ToTimestamp` | - | `StartArchiveExportResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Initiates an export of emails from the specified archive. |
| `StartArchiveSearch` | - | - | `ArchiveId`, `FromTimestamp`, `MaxResults`, `ToTimestamp` | - | `StartArchiveSearchResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Initiates a search across emails in the specified archive. |
| `StopAddressListImportJob` | - | `idempotent` | `JobId` | - | `StopAddressListImportJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops an ongoing import job for an address list. |
| `StopArchiveExport` | - | - | `ExportId` | - | `StopArchiveExportResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Stops an in-progress export of emails from an archive. |
| `StopArchiveSearch` | - | `idempotent` | `SearchId` | - | `StopArchiveSearchResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Stops an in-progress archive search job. |
| `TagResource` | - | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Adds one or more tags (keys and values) to a specified resource. |
| `UntagResource` | - | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Remove one or more tags (keys and values) from a specified resource. |
| `UpdateArchive` | - | `idempotent` | `ArchiveId` | - | `UpdateArchiveResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the attributes of an existing email archive. |
| `UpdateIngressPoint` | - | `idempotent` | `IngressPointId` | - | `UpdateIngressPointResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Update attributes of a provisioned ingress endpoint resource. |
| `UpdateRelay` | - | `idempotent` | `RelayId` | - | `UpdateRelayResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Updates the attributes of an existing relay resource. |
| `UpdateRuleSet` | - | `idempotent` | `RuleSetId` | - | `UpdateRuleSetResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Update attributes of an already provisioned rule set. |
| `UpdateTrafficPolicy` | - | `idempotent` | `TrafficPolicyId` | - | `UpdateTrafficPolicyResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Update attributes of an already provisioned traffic policy resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `Message` | The request validation has failed. |
| `ResourceNotFoundException` | `structure` | `Message` | Occurs when a requested resource is not found. |
| `AccessDeniedException` | `structure` | `Message` | Occurs when a user is denied access to a specific resource or action. |
| `ThrottlingException` | `structure` | `Message` | Occurs when a service's request rate limit is exceeded, resulting in throttling of further requests. |
| `ConflictException` | `structure` | `Message` | The request configuration has conflicts. |
| `ServiceQuotaExceededException` | `structure` | `Message` | Occurs when an operation exceeds a predefined service quota or limit. |
| `CreateAddonInstanceRequest` | `structure` | `AddonSubscriptionId`, `ClientToken`, `Tags` | - |
| `CreateAddonInstanceResponse` | `structure` | `AddonInstanceId` | - |
| `CreateAddonSubscriptionRequest` | `structure` | `AddonName`, `ClientToken`, `Tags` | - |
| `CreateAddonSubscriptionResponse` | `structure` | `AddonSubscriptionId` | - |
| `CreateAddressListRequest` | `structure` | `AddressListName`, `ClientToken`, `Tags` | - |
| `CreateAddressListResponse` | `structure` | `AddressListId` | - |
| `CreateAddressListImportJobRequest` | `structure` | `AddressListId`, `ClientToken`, `ImportDataFormat`, `Name` | - |
| `CreateAddressListImportJobResponse` | `structure` | `JobId`, `PreSignedUrl` | - |
| `CreateArchiveRequest` | `structure` | `ArchiveName`, `ClientToken`, `KmsKeyArn`, `Retention`, `Tags` | The request to create a new email archive. |
| `CreateArchiveResponse` | `structure` | `ArchiveId` | The response from creating a new email archive. |
| `CreateIngressPointRequest` | `structure` | `ClientToken`, `IngressPointConfiguration`, `IngressPointName`, `NetworkConfiguration`, `RuleSetId`, `Tags`, `TrafficPolicyId`, `Type` | - |
| `CreateIngressPointResponse` | `structure` | `IngressPointId` | - |
| `CreateRelayRequest` | `structure` | `Authentication`, `ClientToken`, `RelayName`, `ServerName`, `ServerPort`, `Tags` | - |
| `CreateRelayResponse` | `structure` | `RelayId` | - |
| `CreateRuleSetRequest` | `structure` | `ClientToken`, `RuleSetName`, `Rules`, `Tags` | - |
| `CreateRuleSetResponse` | `structure` | `RuleSetId` | - |
| `CreateTrafficPolicyRequest` | `structure` | `ClientToken`, `DefaultAction`, `MaxMessageSizeBytes`, `PolicyStatements`, `Tags`, `TrafficPolicyName` | - |
| `CreateTrafficPolicyResponse` | `structure` | `TrafficPolicyId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
