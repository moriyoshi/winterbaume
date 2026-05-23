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

- Operations: `GetAddressListImportJob`, `GetArchiveExport`, `GetArchiveMessage`, `GetArchiveMessageContent`, `GetArchiveSearch`, `GetArchiveSearchResults`, `GetMemberOfAddressList`
- Traits: `readonly` (5)
- Common required input members in this group: `ArchivedMessageId`, `SearchId`

### List

- Operations: `ListAddressListImportJobs`, `ListArchiveExports`, `ListArchiveSearches`, `ListMembersOfAddressList`, `ListTagsForResource`
- Traits: `readonly` (5), `paginated` (4)
- Common required input members in this group: `AddressListId`, `ArchiveId`

### Start

- Operations: `StartAddressListImportJob`, `StartArchiveExport`, `StartArchiveSearch`
- Traits: `idempotent` (1)
- Common required input members in this group: `ArchiveId`, `FromTimestamp`, `ToTimestamp`

### Stop

- Operations: `StopAddressListImportJob`, `StopArchiveExport`, `StopArchiveSearch`
- Traits: `idempotent` (2)
- Common required input members in this group: -

### Create

- Operations: `CreateAddressListImportJob`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterMemberFromAddressList`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Register

- Operations: `RegisterMemberToAddressList`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAddressListImportJob` | `-` | `idempotent`, `idempotency-token` | `AddressListId`, `Name`, `ImportDataFormat` | `ClientToken` | `CreateAddressListImportJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an import job for an address list. |
| `DeregisterMemberFromAddressList` | `-` | `idempotent` | `AddressListId`, `Address` | - | `DeregisterMemberFromAddressListResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a member from an address list. |
| `GetAddressListImportJob` | `-` | `readonly` | `JobId` | - | `GetAddressListImportJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Fetch attributes of an import job. |
| `GetArchiveExport` | `-` | - | `ExportId` | - | `GetArchiveExportResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Retrieves the details and current status of a specific email archive export job. |
| `GetArchiveMessage` | `-` | `readonly` | `ArchivedMessageId` | - | `GetArchiveMessageResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns a pre-signed URL that provides temporary download access to the specific email message stored in the archive. |
| `GetArchiveMessageContent` | `-` | - | `ArchivedMessageId` | - | `GetArchiveMessageContentResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns the textual content of a specific email message stored in the archive. Attachments are not included. |
| `GetArchiveSearch` | `-` | `readonly` | `SearchId` | - | `GetArchiveSearchResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Retrieves the details and current status of a specific email archive search job. |
| `GetArchiveSearchResults` | `-` | `readonly` | `SearchId` | - | `GetArchiveSearchResultsResponse` | `AccessDeniedException`, `ConflictException`, `ThrottlingException`, `ValidationException` | Returns the results of a completed email archive search job. |
| `GetMemberOfAddressList` | `-` | `readonly` | `AddressListId`, `Address` | - | `GetMemberOfAddressListResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Fetch attributes of a member in an address list. |
| `ListAddressListImportJobs` | `-` | `readonly`, `paginated` | `AddressListId` | - | `ListAddressListImportJobsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists jobs for an address list. |
| `ListArchiveExports` | `-` | `readonly`, `paginated` | `ArchiveId` | - | `ListArchiveExportsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of email archive export jobs. |
| `ListArchiveSearches` | `-` | `readonly`, `paginated` | `ArchiveId` | - | `ListArchiveSearchesResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of email archive search jobs. |
| `ListMembersOfAddressList` | `-` | `readonly`, `paginated` | `AddressListId` | - | `ListMembersOfAddressListResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists members of an address list. |
| `ListTagsForResource` | `-` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves the list of tags (keys and values) assigned to the resource. |
| `RegisterMemberToAddressList` | `-` | `idempotent` | `AddressListId`, `Address` | - | `RegisterMemberToAddressListResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a member to an address list. |
| `StartAddressListImportJob` | `-` | `idempotent` | `JobId` | - | `StartAddressListImportJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts an import job for an address list. |
| `StartArchiveExport` | `-` | - | `ArchiveId`, `FromTimestamp`, `ToTimestamp`, `ExportDestinationConfiguration` | - | `StartArchiveExportResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Initiates an export of emails from the specified archive. |
| `StartArchiveSearch` | `-` | - | `ArchiveId`, `FromTimestamp`, `ToTimestamp`, `MaxResults` | - | `StartArchiveSearchResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Initiates a search across emails in the specified archive. |
| `StopAddressListImportJob` | `-` | `idempotent` | `JobId` | - | `StopAddressListImportJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops an ongoing import job for an address list. |
| `StopArchiveExport` | `-` | - | `ExportId` | - | `StopArchiveExportResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Stops an in-progress export of emails from an archive. |
| `StopArchiveSearch` | `-` | `idempotent` | `SearchId` | - | `StopArchiveSearchResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Stops an in-progress archive search job. |
| `TagResource` | `-` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Adds one or more tags (keys and values) to a specified resource. |
| `UntagResource` | `-` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Remove one or more tags (keys and values) from a specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | Occurs when a user is denied access to a specific resource or action. |
| `ConflictException` | `structure` | Message | The request configuration has conflicts. For details, see the accompanying error message. |
| `ResourceNotFoundException` | `structure` | Message | Occurs when a requested resource is not found. |
| `ServiceQuotaExceededException` | `structure` | Message | Occurs when an operation exceeds a predefined service quota or limit. |
| `ThrottlingException` | `structure` | Message | Occurs when a service's request rate limit is exceeded, resulting in throttling of further requests. |
| `ValidationException` | `structure` | Message | The request validation has failed. For details, see the accompanying error message. |
| `CreateAddressListImportJobRequest` | `structure` | ClientToken, AddressListId, Name, ImportDataFormat | - |
| `CreateAddressListImportJobResponse` | `structure` | JobId, PreSignedUrl | - |
| `DeregisterMemberFromAddressListRequest` | `structure` | AddressListId, Address | - |
| `DeregisterMemberFromAddressListResponse` | `structure` | **empty (no members)** | - |
| `GetAddressListImportJobRequest` | `structure` | JobId | - |
| `GetAddressListImportJobResponse` | `structure` | JobId, Name, Status, PreSignedUrl, ImportedItemsCount, FailedItemsCount, ImportDataFormat, AddressListId, CreatedTimestamp, StartTimestamp, CompletedTimestamp, Error | - |
| `GetArchiveExportRequest` | `structure` | ExportId | The request to retrieve details of a specific archive export job. |
| `GetArchiveExportResponse` | `structure` | ArchiveId, Filters, FromTimestamp, ToTimestamp, MaxResults, ExportDestinationConfiguration, Status | The response containing details of the specified archive export job. |
| `GetArchiveMessageRequest` | `structure` | ArchivedMessageId | The request to get details of a specific email message stored in an archive. |
| `GetArchiveMessageResponse` | `structure` | MessageDownloadLink, Metadata, Envelope | The response containing details about the requested archived email message. |
| `GetArchiveMessageContentRequest` | `structure` | ArchivedMessageId | The request to get the textual content of a specific email message stored in an archive. |
| `GetArchiveMessageContentResponse` | `structure` | Body | The response containing the textual content of the requested archived email message. |
| `GetArchiveSearchRequest` | `structure` | SearchId | The request to retrieve details of a specific archive search job. |
| `GetArchiveSearchResponse` | `structure` | ArchiveId, Filters, FromTimestamp, ToTimestamp, MaxResults, Status | The response containing details of the specified archive search job. |
| `GetArchiveSearchResultsRequest` | `structure` | SearchId | The request to retrieve results from a completed archive search job. |
| `GetArchiveSearchResultsResponse` | `structure` | Rows | The response containing search results from a completed archive search. |
| `GetMemberOfAddressListRequest` | `structure` | AddressListId, Address | - |
| `GetMemberOfAddressListResponse` | `structure` | Address, CreatedTimestamp | - |
| `ListAddressListImportJobsRequest` | `structure` | AddressListId, NextToken, PageSize | - |
| `ListAddressListImportJobsResponse` | `structure` | ImportJobs, NextToken | - |
| `ListArchiveExportsRequest` | `structure` | ArchiveId, NextToken, PageSize | The request to list archive export jobs in your account. |
| `ListArchiveExportsResponse` | `structure` | Exports, NextToken | The response containing a list of archive export jobs and their statuses. |
| `ListArchiveSearchesRequest` | `structure` | ArchiveId, NextToken, PageSize | The request to list archive search jobs in your account. |
| `ListArchiveSearchesResponse` | `structure` | Searches, NextToken | The response containing a list of archive search jobs and their statuses. |
| `ListMembersOfAddressListRequest` | `structure` | AddressListId, Filter, NextToken, PageSize | - |
| `ListMembersOfAddressListResponse` | `structure` | Addresses, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `RegisterMemberToAddressListRequest` | `structure` | AddressListId, Address | - |
| `RegisterMemberToAddressListResponse` | `structure` | **empty (no members)** | - |
| `StartAddressListImportJobRequest` | `structure` | JobId | - |
| `StartAddressListImportJobResponse` | `structure` | **empty (no members)** | - |
| `StartArchiveExportRequest` | `structure` | ArchiveId, Filters, FromTimestamp, ToTimestamp, MaxResults, ExportDestinationConfiguration, IncludeMetadata | The request to initiate an export of emails from an archive. |
| `StartArchiveExportResponse` | `structure` | ExportId | The response from initiating an archive export. |
| `AcceptAction` | `enum` | ALLOW, DENY | - |
| `ActionFailurePolicy` | `enum` | CONTINUE, DROP | - |
| `ArchiveBooleanEmailAttribute` | `enum` | HAS_ATTACHMENTS | - |
| `ArchiveBooleanOperator` | `enum` | IS_TRUE, IS_FALSE | - |
| `ArchiveState` | `enum` | ACTIVE, PENDING_DELETION | - |
| `ArchiveStringEmailAttribute` | `enum` | TO, FROM, CC, SUBJECT, ENVELOPE_TO, ENVELOPE_FROM | - |
| `ArchiveStringOperator` | `enum` | CONTAINS | - |
| `ExportState` | `enum` | QUEUED, PREPROCESSING, PROCESSING, COMPLETED, FAILED, CANCELLED | - |
| `ImportDataType` | `enum` | CSV, JSON | - |
| `ImportJobStatus` | `enum` | CREATED, PROCESSING, COMPLETED, FAILED, STOPPED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
