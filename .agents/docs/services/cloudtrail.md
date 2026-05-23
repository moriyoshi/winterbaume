# AWS CloudTrail

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

CloudTrail This is the CloudTrail API Reference. It provides descriptions of actions, data types, common parameters, and common errors for CloudTrail. CloudTrail is a web service that records Amazon Web Services API calls for your Amazon Web Services account and delivers log files to an Amazon S3 bucket. The recorded information includes the identity of the user, the start time of the Amazon Web Services API call, the source IP address, the request parameters, and the response elements returned by the service. As an alternative to the API, you can use one of the Amazon Web Services SDKs, which consist of libraries and sample code for various programming languages and platforms (Java, Ruby, .NET, iOS, Android, etc.).

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS CloudTrail by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS CloudTrail resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS CloudTrail workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Delete`, `Start`, `Create` operation families, including `GetChannel`, `GetDashboard`, `GetEventConfiguration`, `GetEventDataStore`, `ListChannels`, `ListDashboards`.

## Service Identity and Protocol

- AWS model slug: `cloudtrail`
- AWS SDK for Rust slug: `cloudtrail`
- Model version: `2013-11-01`
- Model file: `vendor/api-models-aws/models/cloudtrail/service/2013-11-01/cloudtrail-2013-11-01.json`
- SDK ID: `CloudTrail`
- Endpoint prefix: `cloudtrail`
- ARN namespace: `cloudtrail`
- CloudFormation name: `CloudTrail`
- CloudTrail event source: `cloudtrail.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (11), `List` (11), `Delete` (5), `Start` (5), `Create` (4), `Put` (4), `Update` (4), `Stop` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTags`, `CancelQuery`, `CreateChannel`, `CreateDashboard`, `CreateEventDataStore`, `CreateTrail`, `DeleteChannel`, `DeleteDashboard`, `DeleteEventDataStore`, `DeleteResourcePolicy`, `DeleteTrail`, `DeregisterOrganizationDelegatedAdmin`, `DisableFederation`, `EnableFederation`, `PutEventConfiguration`, `PutEventSelectors`, `PutInsightSelectors`, `PutResourcePolicy`, `RegisterOrganizationDelegatedAdmin`, `RemoveTags`, ... (+13).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeQuery`, `DescribeTrails`, `GetChannel`, `GetDashboard`, `GetEventConfiguration`, `GetEventDataStore`, `GetEventSelectors`, `GetImport`, `GetInsightSelectors`, `GetQueryResults`, `GetResourcePolicy`, `GetTrail`, `GetTrailStatus`, `ListChannels`, `ListDashboards`, `ListEventDataStores`, `ListImportFailures`, `ListImports`, `ListInsightsData`, `ListInsightsMetricData`, ... (+5).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 47 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelQuery`, `GetImport`, `ListImportFailures`, `ListImports`, `StartDashboardRefresh`, `StartEventDataStoreIngestion`, `StartImport`, `StartLogging`, `StartQuery`, `StopEventDataStoreIngestion`, `StopImport`, `StopLogging`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 60 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `Lambda`, `Glue`, `ECS`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/awscloudtrail/latest/userguide/view-cloudtrail-events.html
- https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-events-with-cloudtrail.html
- https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-multi-region-trails.html

Research outcomes:
- Event history is active by default, records management events only, is limited to the last 90 days, and is scoped to one account and one Region.
- Event history does not include data events, Insights events, network activity events, or organisation-level aggregation.
- Event history searches can use only one attribute filter plus a time range.
- Management events are control-plane operations, including API calls and some non-API events such as ConsoleLogin.
- Trails and event data stores log management events by default and can be configured for read events, write events, or both.
- Multi-Region trails deliver activity from all enabled Regions to one S3 bucket and optional CloudWatch Logs group.
- A multi-Region trail is visible in all enabled Regions but can only be modified in its home Region.
- Enabling an opt-in Region creates an identical copy of each multi-Region trail there, with eventual consistency that can take minutes to hours.

Parity implications:
- Model event history, trails, event selectors, event data stores, S3 delivery, CloudWatch Logs delivery, SNS notifications, home Region, and multi-Region copies.
- LookupEvents should enforce 90-day, management-event-only, single-Region/account, and single-attribute filter constraints.
- Trail delivery should distinguish management/data/Insights events, read/write selectors, home-Region mutability, and eventual multi-Region propagation.

## Control-Plane / Data-Plane Coherence

- **Paired with `cloudtraildata`.** CloudTrail Data ( `winterbaume-cloudtraildata` ) ingests partner-sourced events into a CloudTrail **channel** that is created by this control plane via `CreateChannel`. `PutAuditEvents` requires a valid channel ARN; in real AWS, the call fails with `ChannelARNInvalidException` if the channel does not exist.
- **Current Winterbaume status: divergent.** `winterbaume-cloudtraildata` does not depend on `winterbaume-cloudtrail`; it records events with whatever `channel_arn` it is given without checking that the channel exists. Conversely, this crate's channel state is not observable to the data plane.
- **What needs to change:** `winterbaume-cloudtraildata` should observe this crate's `channels` state and reject `PutAuditEvents` for unknown channels. The reverse direction ( reading data-plane-ingested events from this crate ) is not part of real AWS — events go to the channel's destinations ( S3, CloudWatch Logs ), not back to the control plane.

## Operation Groups

### Get

- Operations: `GetChannel`, `GetDashboard`, `GetEventConfiguration`, `GetEventDataStore`, `GetEventSelectors`, `GetImport`, `GetInsightSelectors`, `GetQueryResults`, `GetResourcePolicy`, `GetTrail`, `GetTrailStatus`
- Traits: `idempotent` (9), `paginated` (1)
- Common required input members in this group: `Name`

### List

- Operations: `ListChannels`, `ListDashboards`, `ListEventDataStores`, `ListImportFailures`, `ListImports`, `ListInsightsData`, `ListInsightsMetricData`, `ListPublicKeys`, `ListQueries`, `ListTags`, `ListTrails`
- Traits: `idempotent` (11), `paginated` (10)
- Common required input members in this group: -

### Delete

- Operations: `DeleteChannel`, `DeleteDashboard`, `DeleteEventDataStore`, `DeleteResourcePolicy`, `DeleteTrail`
- Traits: `idempotent` (3)
- Common required input members in this group: -

### Start

- Operations: `StartDashboardRefresh`, `StartEventDataStoreIngestion`, `StartImport`, `StartLogging`, `StartQuery`
- Traits: `idempotent` (3)
- Common required input members in this group: -

### Create

- Operations: `CreateChannel`, `CreateDashboard`, `CreateEventDataStore`, `CreateTrail`
- Traits: `idempotent` (2)
- Common required input members in this group: `Name`

### Put

- Operations: `PutEventConfiguration`, `PutEventSelectors`, `PutInsightSelectors`, `PutResourcePolicy`
- Traits: `idempotent` (4)
- Common required input members in this group: -

### Update

- Operations: `UpdateChannel`, `UpdateDashboard`, `UpdateEventDataStore`, `UpdateTrail`
- Traits: `idempotent` (4)
- Common required input members in this group: -

### Stop

- Operations: `StopEventDataStoreIngestion`, `StopImport`, `StopLogging`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Describe

- Operations: `DescribeQuery`, `DescribeTrails`
- Traits: `idempotent` (2)
- Common required input members in this group: -

### Add

- Operations: `AddTags`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Cancel

- Operations: `CancelQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterOrganizationDelegatedAdmin`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Disable

- Operations: `DisableFederation`
- Common required input members in this group: -

### Enable

- Operations: `EnableFederation`
- Common required input members in this group: -

### Generate

- Operations: `GenerateQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Lookup

- Operations: `LookupEvents`
- Traits: `idempotent` (1), `paginated` (1)
- Common required input members in this group: -

### Register

- Operations: `RegisterOrganizationDelegatedAdmin`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Remove

- Operations: `RemoveTags`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Restore

- Operations: `RestoreEventDataStore`
- Common required input members in this group: -

### Search

- Operations: `SearchSampleQueries`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTags` | `-` | `idempotent` | `ResourceId`, `TagsList` | - | `AddTagsResponse` | `ChannelARNInvalidException`, `ChannelNotFoundException`, `CloudTrailARNInvalidException`, `ConflictException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InvalidTagParameterException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `ResourceNotFoundException`, `ResourceTypeNotSupportedException`, `TagsLimitExceededException`, `UnsupportedOperationException` | Adds one or more tags to a trail, event data store, dashboard, or channel, up to a limit of 50. Overwrites an existing tag's value when a new value is specified for an existing tag key. Tag key names must be unique; ... |
| `CancelQuery` | `-` | `idempotent` | `QueryId` | - | `CancelQueryResponse` | `ConflictException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InactiveQueryException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `QueryIdNotFoundException`, `UnsupportedOperationException` | Cancels a query if the query is not in a terminated state, such as CANCELLED , FAILED , TIMED_OUT , or FINISHED . You must specify an ARN value for EventDataStore . The ID of the query that you want to cancel is also ... |
| `CreateChannel` | `-` | - | `Name`, `Source`, `Destinations` | - | `CreateChannelResponse` | `ChannelAlreadyExistsException`, `ChannelMaxLimitExceededException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InvalidEventDataStoreCategoryException`, `InvalidParameterException`, `InvalidSourceException`, `InvalidTagParameterException`, `OperationNotPermittedException`, `TagsLimitExceededException`, `UnsupportedOperationException` | Creates a channel for CloudTrail to ingest events from a partner or external source. After you create a channel, a CloudTrail Lake event data store can log events from the partner or source that you specify. |
| `CreateDashboard` | `-` | `idempotent` | `Name` | - | `CreateDashboardResponse` | `ConflictException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InsufficientEncryptionPolicyException`, `InvalidQueryStatementException`, `InvalidTagParameterException`, `ServiceQuotaExceededException`, `UnsupportedOperationException` | Creates a custom dashboard or the Highlights dashboard. Custom dashboards - Custom dashboards allow you to query events in any event data store type. You can add up to 10 widgets to a custom dashboard. You can manual ... |
| `CreateEventDataStore` | `-` | - | `Name` | - | `CreateEventDataStoreResponse` | `CloudTrailAccessNotEnabledException`, `ConflictException`, `EventDataStoreAlreadyExistsException`, `EventDataStoreMaxLimitExceededException`, `InsufficientDependencyServiceAccessPermissionException`, `InsufficientEncryptionPolicyException`, `InvalidEventSelectorsException`, `InvalidKmsKeyIdException`, `InvalidParameterException`, `InvalidTagParameterException`, `KmsException`, `KmsKeyNotFoundException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `OrganizationNotInAllFeaturesModeException`, `OrganizationsNotInUseException`, `ThrottlingException`, `UnsupportedOperationException` | Creates a new event data store. |
| `CreateTrail` | `-` | `idempotent` | `Name`, `S3BucketName` | - | `CreateTrailResponse` | `CloudTrailAccessNotEnabledException`, `CloudTrailInvalidClientTokenIdException`, `CloudWatchLogsDeliveryUnavailableException`, `ConflictException`, `InsufficientDependencyServiceAccessPermissionException`, `InsufficientEncryptionPolicyException`, `InsufficientS3BucketPolicyException`, `InsufficientSnsTopicPolicyException`, `InvalidCloudWatchLogsLogGroupArnException`, `InvalidCloudWatchLogsRoleArnException`, `InvalidKmsKeyIdException`, `InvalidParameterCombinationException`, `InvalidParameterException`, `InvalidS3BucketNameException`, `InvalidS3PrefixException`, `InvalidSnsTopicNameException`, `InvalidTagParameterException`, `InvalidTrailNameException`, `KmsException`, `KmsKeyDisabledException`, `KmsKeyNotFoundException`, `MaximumNumberOfTrailsExceededException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `OrganizationNotInAllFeaturesModeException`, `OrganizationsNotInUseException`, `S3BucketDoesNotExistException`, `TagsLimitExceededException`, `ThrottlingException`, `TrailAlreadyExistsException`, `TrailNotProvidedException`, `UnsupportedOperationException` | Creates a trail that specifies the settings for delivery of log data to an Amazon S3 bucket. |
| `DeleteChannel` | `-` | - | `Channel` | - | `DeleteChannelResponse` | `ChannelARNInvalidException`, `ChannelNotFoundException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Deletes a channel. |
| `DeleteDashboard` | `-` | `idempotent` | `DashboardId` | - | `DeleteDashboardResponse` | `ConflictException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Deletes the specified dashboard. You cannot delete a dashboard that has termination protection enabled. |
| `DeleteEventDataStore` | `-` | - | `EventDataStore` | - | `DeleteEventDataStoreResponse` | `ChannelExistsForEDSException`, `ConflictException`, `EventDataStoreARNInvalidException`, `EventDataStoreFederationEnabledException`, `EventDataStoreHasOngoingImportException`, `EventDataStoreNotFoundException`, `EventDataStoreTerminationProtectedException`, `InactiveEventDataStoreException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Disables the event data store specified by EventDataStore , which accepts an event data store ARN. After you run DeleteEventDataStore , the event data store enters a PENDING_DELETION state, and is automatically delet ... |
| `DeleteResourcePolicy` | `-` | `idempotent` | `ResourceArn` | - | `DeleteResourcePolicyResponse` | `ConflictException`, `OperationNotPermittedException`, `ResourceARNNotValidException`, `ResourceNotFoundException`, `ResourcePolicyNotFoundException`, `ResourceTypeNotSupportedException`, `UnsupportedOperationException` | Deletes the resource-based policy attached to the CloudTrail event data store, dashboard, or channel. |
| `DeleteTrail` | `-` | `idempotent` | `Name` | - | `DeleteTrailResponse` | `CloudTrailARNInvalidException`, `ConflictException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidHomeRegionException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `ThrottlingException`, `TrailNotFoundException`, `UnsupportedOperationException` | Deletes a trail. This operation must be called from the Region in which the trail was created. DeleteTrail cannot be called on the shadow trails (replicated trails in other Regions) of a trail that is enabled in all ... |
| `DeregisterOrganizationDelegatedAdmin` | `-` | `idempotent` | `DelegatedAdminAccountId` | - | `DeregisterOrganizationDelegatedAdminResponse` | `AccountNotFoundException`, `AccountNotRegisteredException`, `CloudTrailAccessNotEnabledException`, `ConflictException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidParameterException`, `NotOrganizationManagementAccountException`, `OperationNotPermittedException`, `OrganizationNotInAllFeaturesModeException`, `OrganizationsNotInUseException`, `UnsupportedOperationException` | Removes CloudTrail delegated administrator permissions from a member account in an organization. |
| `DescribeQuery` | `-` | `idempotent` | - | - | `DescribeQueryResponse` | `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `QueryIdNotFoundException`, `UnsupportedOperationException` | Returns metadata about a query, including query run time in milliseconds, number of events scanned and matched, and query status. If the query results were delivered to an S3 bucket, the response also provides the S3 ... |
| `DescribeTrails` | `-` | `idempotent` | - | - | `DescribeTrailsResponse` | `CloudTrailARNInvalidException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Retrieves settings for one or more trails associated with the current Region for your account. |
| `DisableFederation` | `-` | - | `EventDataStore` | - | `DisableFederationResponse` | `AccessDeniedException`, `CloudTrailAccessNotEnabledException`, `ConcurrentModificationException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `OrganizationNotInAllFeaturesModeException`, `OrganizationsNotInUseException`, `UnsupportedOperationException` | Disables Lake query federation on the specified event data store. When you disable federation, CloudTrail disables the integration with Glue, Lake Formation, and Amazon Athena. After disabling Lake query federation, ... |
| `EnableFederation` | `-` | - | `EventDataStore`, `FederationRoleArn` | - | `EnableFederationResponse` | `AccessDeniedException`, `CloudTrailAccessNotEnabledException`, `ConcurrentModificationException`, `EventDataStoreARNInvalidException`, `EventDataStoreFederationEnabledException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `OrganizationNotInAllFeaturesModeException`, `OrganizationsNotInUseException`, `UnsupportedOperationException` | Enables Lake query federation on the specified event data store. Federating an event data store lets you view the metadata associated with the event data store in the Glue Data Catalog and run SQL queries against you ... |
| `GenerateQuery` | `-` | `idempotent` | `EventDataStores`, `Prompt` | - | `GenerateQueryResponse` | `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `GenerateResponseException`, `InactiveEventDataStoreException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Generates a query from a natural language prompt. This operation uses generative artificial intelligence (generative AI) to produce a ready-to-use SQL query from the prompt. The prompt can be a question or a statemen ... |
| `GetChannel` | `-` | `idempotent` | `Channel` | - | `GetChannelResponse` | `ChannelARNInvalidException`, `ChannelNotFoundException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns information about a specific channel. |
| `GetDashboard` | `-` | `idempotent` | `DashboardId` | - | `GetDashboardResponse` | `ResourceNotFoundException`, `UnsupportedOperationException` | Returns the specified dashboard. |
| `GetEventConfiguration` | `-` | `idempotent` | - | - | `GetEventConfigurationResponse` | `CloudTrailARNInvalidException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InvalidEventDataStoreCategoryException`, `InvalidEventDataStoreStatusException`, `InvalidParameterCombinationException`, `InvalidParameterException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `TrailNotFoundException`, `UnsupportedOperationException` | Retrieves the current event configuration settings for the specified event data store or trail. The response includes maximum event size configuration, the context key selectors configured for the event data store, a ... |
| `GetEventDataStore` | `-` | `idempotent` | `EventDataStore` | - | `GetEventDataStoreResponse` | `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns information about an event data store specified as either an ARN or the ID portion of the ARN. |
| `GetEventSelectors` | `-` | `idempotent` | `TrailName` | - | `GetEventSelectorsResponse` | `CloudTrailARNInvalidException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `TrailNotFoundException`, `UnsupportedOperationException` | Describes the settings for the event selectors that you configured for your trail. The information returned for your event selectors includes the following: If your event selector includes read-only events, write-onl ... |
| `GetImport` | `-` | - | `ImportId` | - | `GetImportResponse` | `ImportNotFoundException`, `InvalidParameterException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns information about a specific import. |
| `GetInsightSelectors` | `-` | `idempotent` | - | - | `GetInsightSelectorsResponse` | `CloudTrailARNInvalidException`, `InsightNotEnabledException`, `InvalidParameterCombinationException`, `InvalidParameterException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `ThrottlingException`, `TrailNotFoundException`, `UnsupportedOperationException` | Describes the settings for the Insights event selectors that you configured for your trail or event data store. GetInsightSelectors shows if CloudTrail Insights logging is enabled and which Insights types are configu ... |
| `GetQueryResults` | `-` | `paginated` | `QueryId` | - | `GetQueryResultsResponse` | `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InsufficientEncryptionPolicyException`, `InvalidMaxResultsException`, `InvalidNextTokenException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `QueryIdNotFoundException`, `UnsupportedOperationException` | Gets event data results of a query. You must specify the QueryID value returned by the StartQuery operation. |
| `GetResourcePolicy` | `-` | `idempotent` | `ResourceArn` | - | `GetResourcePolicyResponse` | `OperationNotPermittedException`, `ResourceARNNotValidException`, `ResourceNotFoundException`, `ResourcePolicyNotFoundException`, `ResourceTypeNotSupportedException`, `UnsupportedOperationException` | Retrieves the JSON text of the resource-based policy document attached to the CloudTrail event data store, dashboard, or channel. |
| `GetTrail` | `-` | `idempotent` | `Name` | - | `GetTrailResponse` | `CloudTrailARNInvalidException`, `InvalidTrailNameException`, `OperationNotPermittedException`, `TrailNotFoundException`, `UnsupportedOperationException` | Returns settings information for a specified trail. |
| `GetTrailStatus` | `-` | `idempotent` | `Name` | - | `GetTrailStatusResponse` | `CloudTrailARNInvalidException`, `InvalidTrailNameException`, `OperationNotPermittedException`, `TrailNotFoundException`, `UnsupportedOperationException` | Returns a JSON-formatted list of information about the specified trail. Fields include information on delivery errors, Amazon SNS and Amazon S3 errors, and start and stop logging times for each trail. This operation ... |
| `ListChannels` | `-` | `idempotent`, `paginated` | - | - | `ListChannelsResponse` | `InvalidNextTokenException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Lists the channels in the current account, and their source names. |
| `ListDashboards` | `-` | `idempotent` | - | - | `ListDashboardsResponse` | `UnsupportedOperationException` | Returns information about all dashboards in the account, in the current Region. |
| `ListEventDataStores` | `-` | `idempotent`, `paginated` | - | - | `ListEventDataStoresResponse` | `InvalidMaxResultsException`, `InvalidNextTokenException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns information about all event data stores in the account, in the current Region. |
| `ListImportFailures` | `-` | `idempotent`, `paginated` | `ImportId` | - | `ListImportFailuresResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns a list of failures for the specified import. |
| `ListImports` | `-` | `idempotent`, `paginated` | - | - | `ListImportsResponse` | `EventDataStoreARNInvalidException`, `InvalidNextTokenException`, `InvalidParameterException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns information on all imports, or a select set of imports by ImportStatus or Destination . |
| `ListInsightsData` | `-` | `idempotent`, `paginated` | `InsightSource`, `DataType` | - | `ListInsightsDataResponse` | `InvalidParameterException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns Insights events generated on a trail that logs data events. You can list Insights events that occurred in a Region within the last 90 days. ListInsightsData supports the following Dimensions for Insights even ... |
| `ListInsightsMetricData` | `-` | `idempotent`, `paginated` | `EventSource`, `EventName`, `InsightType` | - | `ListInsightsMetricDataResponse` | `InvalidParameterException`, `InvalidTrailNameException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns Insights metrics data for trails that have enabled Insights. The request must include the EventSource , EventName , and InsightType parameters. If the InsightType is set to ApiErrorRateInsight , the request m ... |
| `ListPublicKeys` | `-` | `idempotent`, `paginated` | - | - | `ListPublicKeysResponse` | `InvalidTimeRangeException`, `InvalidTokenException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns all public keys whose private keys were used to sign the digest files within the specified time range. The public key is needed to validate digest files that were signed with its corresponding private key. Cl ... |
| `ListQueries` | `-` | `idempotent`, `paginated` | `EventDataStore` | - | `ListQueriesResponse` | `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InvalidDateRangeException`, `InvalidMaxResultsException`, `InvalidNextTokenException`, `InvalidParameterException`, `InvalidQueryStatusException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Returns a list of queries and query statuses for the past seven days. You must specify an ARN value for EventDataStore . Optionally, to shorten the list of results, you can specify a time range, formatted as timestam ... |
| `ListTags` | `-` | `idempotent`, `paginated` | `ResourceIdList` | - | `ListTagsResponse` | `ChannelARNInvalidException`, `CloudTrailARNInvalidException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InvalidTokenException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `ResourceNotFoundException`, `ResourceTypeNotSupportedException`, `UnsupportedOperationException` | Lists the tags for the specified trails, event data stores, dashboards, or channels in the current Region. |
| `ListTrails` | `-` | `idempotent`, `paginated` | - | - | `ListTrailsResponse` | `OperationNotPermittedException`, `UnsupportedOperationException` | Lists trails that are in the current account. |
| `LookupEvents` | `-` | `idempotent`, `paginated` | - | - | `LookupEventsResponse` | `InvalidEventCategoryException`, `InvalidLookupAttributesException`, `InvalidMaxResultsException`, `InvalidNextTokenException`, `InvalidTimeRangeException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Looks up management events or CloudTrail Insights events that are captured by CloudTrail. You can look up events that occurred in a Region within the last 90 days. LookupEvents returns recent Insights events for trai ... |
| `PutEventConfiguration` | `-` | `idempotent` | - | - | `PutEventConfigurationResponse` | `CloudTrailARNInvalidException`, `ConflictException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InsufficientDependencyServiceAccessPermissionException`, `InsufficientIAMAccessPermissionException`, `InvalidEventDataStoreCategoryException`, `InvalidEventDataStoreStatusException`, `InvalidHomeRegionException`, `InvalidParameterCombinationException`, `InvalidParameterException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `ThrottlingException`, `TrailNotFoundException`, `UnsupportedOperationException` | Updates the event configuration settings for the specified event data store or trail. This operation supports updating the maximum event size, adding or modifying context key selectors for event data store, and confi ... |
| `PutEventSelectors` | `-` | `idempotent` | `TrailName` | - | `PutEventSelectorsResponse` | `CloudTrailARNInvalidException`, `ConflictException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidEventSelectorsException`, `InvalidHomeRegionException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `ThrottlingException`, `TrailNotFoundException`, `UnsupportedOperationException` | Configures event selectors (also referred to as basic event selectors ) or advanced event selectors for your trail. You can use either AdvancedEventSelectors or EventSelectors , but not both. If you apply AdvancedEve ... |
| `PutInsightSelectors` | `-` | `idempotent` | `InsightSelectors` | - | `PutInsightSelectorsResponse` | `CloudTrailARNInvalidException`, `InsufficientEncryptionPolicyException`, `InsufficientS3BucketPolicyException`, `InvalidHomeRegionException`, `InvalidInsightSelectorsException`, `InvalidParameterCombinationException`, `InvalidParameterException`, `InvalidTrailNameException`, `KmsException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `S3BucketDoesNotExistException`, `ThrottlingException`, `TrailNotFoundException`, `UnsupportedOperationException` | Lets you enable Insights event logging on specific event categories by specifying the Insights selectors that you want to enable on an existing trail or event data store. You also use PutInsightSelectors to turn off ... |
| `PutResourcePolicy` | `-` | `idempotent` | `ResourceArn`, `ResourcePolicy` | - | `PutResourcePolicyResponse` | `ConflictException`, `OperationNotPermittedException`, `ResourceARNNotValidException`, `ResourceNotFoundException`, `ResourcePolicyNotValidException`, `ResourceTypeNotSupportedException`, `UnsupportedOperationException` | Attaches a resource-based permission policy to a CloudTrail event data store, dashboard, or channel. For more information about resource-based policies, see CloudTrail resource-based policy examples in the CloudTrail ... |
| `RegisterOrganizationDelegatedAdmin` | `-` | `idempotent` | `MemberAccountId` | - | `RegisterOrganizationDelegatedAdminResponse` | `AccountNotFoundException`, `AccountRegisteredException`, `CannotDelegateManagementAccountException`, `CloudTrailAccessNotEnabledException`, `ConflictException`, `DelegatedAdminAccountLimitExceededException`, `InsufficientDependencyServiceAccessPermissionException`, `InsufficientIAMAccessPermissionException`, `InvalidParameterException`, `NotOrganizationManagementAccountException`, `OperationNotPermittedException`, `OrganizationNotInAllFeaturesModeException`, `OrganizationsNotInUseException`, `UnsupportedOperationException` | Registers an organization’s member account as the CloudTrail delegated administrator . |
| `RemoveTags` | `-` | `idempotent` | `ResourceId`, `TagsList` | - | `RemoveTagsResponse` | `ChannelARNInvalidException`, `ChannelNotFoundException`, `CloudTrailARNInvalidException`, `ConflictException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InvalidTagParameterException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `ResourceNotFoundException`, `ResourceTypeNotSupportedException`, `UnsupportedOperationException` | Removes the specified tags from a trail, event data store, dashboard, or channel. |
| `RestoreEventDataStore` | `-` | - | `EventDataStore` | - | `RestoreEventDataStoreResponse` | `CloudTrailAccessNotEnabledException`, `EventDataStoreARNInvalidException`, `EventDataStoreMaxLimitExceededException`, `EventDataStoreNotFoundException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidEventDataStoreStatusException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `OrganizationNotInAllFeaturesModeException`, `OrganizationsNotInUseException`, `UnsupportedOperationException` | Restores a deleted event data store specified by EventDataStore , which accepts an event data store ARN. You can only restore a deleted event data store within the seven-day wait period after deletion. Restoring an e ... |
| `SearchSampleQueries` | `-` | `idempotent` | `SearchPhrase` | - | `SearchSampleQueriesResponse` | `InvalidParameterException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Searches sample queries and returns a list of sample queries that are sorted by relevance. To search for sample queries, provide a natural language SearchPhrase in English. |
| `StartDashboardRefresh` | `-` | `idempotent` | `DashboardId` | - | `StartDashboardRefreshResponse` | `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UnsupportedOperationException` | Starts a refresh of the specified dashboard. Each time a dashboard is refreshed, CloudTrail runs queries to populate the dashboard's widgets. CloudTrail must be granted permissions to run the StartQuery operation on ... |
| `StartEventDataStoreIngestion` | `-` | - | `EventDataStore` | - | `StartEventDataStoreIngestionResponse` | `ConflictException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidEventDataStoreCategoryException`, `InvalidEventDataStoreStatusException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Starts the ingestion of live events on an event data store specified as either an ARN or the ID portion of the ARN. To start ingestion, the event data store Status must be STOPPED_INGESTION and the eventCategory must ... |
| `StartImport` | `-` | - | - | - | `StartImportResponse` | `AccountHasOngoingImportException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `ImportNotFoundException`, `InactiveEventDataStoreException`, `InsufficientEncryptionPolicyException`, `InvalidEventDataStoreCategoryException`, `InvalidEventDataStoreStatusException`, `InvalidImportSourceException`, `InvalidParameterException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Starts an import of logged trail events from a source S3 bucket to a destination event data store. By default, CloudTrail only imports events contained in the S3 bucket's CloudTrail prefix and the prefixes inside the ... |
| `StartLogging` | `-` | `idempotent` | `Name` | - | `StartLoggingResponse` | `CloudTrailARNInvalidException`, `ConflictException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidHomeRegionException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `ThrottlingException`, `TrailNotFoundException`, `UnsupportedOperationException` | Starts the recording of Amazon Web Services API calls and log file delivery for a trail. For a trail that is enabled in all Regions, this operation must be called from the Region in which the trail was created. This ... |
| `StartQuery` | `-` | `idempotent` | - | - | `StartQueryResponse` | `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InsufficientEncryptionPolicyException`, `InsufficientS3BucketPolicyException`, `InvalidParameterException`, `InvalidQueryStatementException`, `InvalidS3BucketNameException`, `InvalidS3PrefixException`, `MaxConcurrentQueriesException`, `NoManagementAccountSLRExistsException`, `OperationNotPermittedException`, `S3BucketDoesNotExistException`, `UnsupportedOperationException` | Starts a CloudTrail Lake query. Use the QueryStatement parameter to provide your SQL query, enclosed in single quotation marks. Use the optional DeliveryS3Uri parameter to deliver the query results to an S3 bucket. S ... |
| `StopEventDataStoreIngestion` | `-` | - | `EventDataStore` | - | `StopEventDataStoreIngestionResponse` | `ConflictException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidEventDataStoreCategoryException`, `InvalidEventDataStoreStatusException`, `InvalidParameterException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Stops the ingestion of live events on an event data store specified as either an ARN or the ID portion of the ARN. To stop ingestion, the event data store Status must be ENABLED and the eventCategory must be Manageme ... |
| `StopImport` | `-` | - | `ImportId` | - | `StopImportResponse` | `ImportNotFoundException`, `InvalidParameterException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Stops a specified import. |
| `StopLogging` | `-` | `idempotent` | `Name` | - | `StopLoggingResponse` | `CloudTrailARNInvalidException`, `ConflictException`, `InsufficientDependencyServiceAccessPermissionException`, `InvalidHomeRegionException`, `InvalidTrailNameException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `ThrottlingException`, `TrailNotFoundException`, `UnsupportedOperationException` | Suspends the recording of Amazon Web Services API calls and log file delivery for the specified trail. Under most circumstances, there is no need to use this action. You can update a trail without stopping it first. ... |
| `UpdateChannel` | `-` | `idempotent` | `Channel` | - | `UpdateChannelResponse` | `ChannelAlreadyExistsException`, `ChannelARNInvalidException`, `ChannelNotFoundException`, `EventDataStoreARNInvalidException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InvalidEventDataStoreCategoryException`, `InvalidParameterException`, `OperationNotPermittedException`, `UnsupportedOperationException` | Updates a channel specified by a required channel ARN or UUID. |
| `UpdateDashboard` | `-` | `idempotent` | `DashboardId` | - | `UpdateDashboardResponse` | `ConflictException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InsufficientEncryptionPolicyException`, `InvalidQueryStatementException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UnsupportedOperationException` | Updates the specified dashboard. To set a refresh schedule, CloudTrail must be granted permissions to run the StartDashboardRefresh operation to refresh the dashboard on your behalf. To provide permissions, run the P ... |
| `UpdateEventDataStore` | `-` | `idempotent` | `EventDataStore` | - | `UpdateEventDataStoreResponse` | `CloudTrailAccessNotEnabledException`, `ConflictException`, `EventDataStoreAlreadyExistsException`, `EventDataStoreARNInvalidException`, `EventDataStoreHasOngoingImportException`, `EventDataStoreNotFoundException`, `InactiveEventDataStoreException`, `InsufficientDependencyServiceAccessPermissionException`, `InsufficientEncryptionPolicyException`, `InvalidEventSelectorsException`, `InvalidInsightSelectorsException`, `InvalidKmsKeyIdException`, `InvalidParameterException`, `KmsException`, `KmsKeyNotFoundException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `OrganizationNotInAllFeaturesModeException`, `OrganizationsNotInUseException`, `ThrottlingException`, `UnsupportedOperationException` | Updates an event data store. The required EventDataStore value is an ARN or the ID portion of the ARN. Other parameters are optional, but at least one optional parameter must be specified, or CloudTrail throws an err ... |
| `UpdateTrail` | `-` | `idempotent` | `Name` | - | `UpdateTrailResponse` | `CloudTrailAccessNotEnabledException`, `CloudTrailARNInvalidException`, `CloudTrailInvalidClientTokenIdException`, `CloudWatchLogsDeliveryUnavailableException`, `ConflictException`, `InsufficientDependencyServiceAccessPermissionException`, `InsufficientEncryptionPolicyException`, `InsufficientS3BucketPolicyException`, `InsufficientSnsTopicPolicyException`, `InvalidCloudWatchLogsLogGroupArnException`, `InvalidCloudWatchLogsRoleArnException`, `InvalidEventSelectorsException`, `InvalidHomeRegionException`, `InvalidKmsKeyIdException`, `InvalidParameterCombinationException`, `InvalidParameterException`, `InvalidS3BucketNameException`, `InvalidS3PrefixException`, `InvalidSnsTopicNameException`, `InvalidTrailNameException`, `KmsException`, `KmsKeyDisabledException`, `KmsKeyNotFoundException`, `NoManagementAccountSLRExistsException`, `NotOrganizationMasterAccountException`, `OperationNotPermittedException`, `OrganizationNotInAllFeaturesModeException`, `OrganizationsNotInUseException`, `S3BucketDoesNotExistException`, `ThrottlingException`, `TrailNotFoundException`, `TrailNotProvidedException`, `UnsupportedOperationException` | Updates trail settings that control what events you are logging, and how to handle log files. Changes to a trail do not require stopping the CloudTrail service. Use this action to designate an existing bucket for log ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `AccountHasOngoingImportException` | `structure` | Message | This exception is thrown when you start a new import and a previous import is still in progress. |
| `AccountNotFoundException` | `structure` | Message | This exception is thrown when the specified account is not found or not part of an organization. |
| `AccountNotRegisteredException` | `structure` | Message | This exception is thrown when the specified account is not registered as the CloudTrail delegated administrator. |
| `AccountRegisteredException` | `structure` | Message | This exception is thrown when the account is already registered as the CloudTrail delegated administrator. |
| `CannotDelegateManagementAccountException` | `structure` | Message | This exception is thrown when the management account of an organization is registered as the CloudTrail delegated administrator. |
| `ChannelARNInvalidException` | `structure` | Message | This exception is thrown when the specified value of ChannelARN is not valid. |
| `ChannelAlreadyExistsException` | `structure` | Message | This exception is thrown when the provided channel already exists. |
| `ChannelExistsForEDSException` | `structure` | Message | This exception is thrown when the specified event data store cannot yet be deleted because it is in use by a channel. |
| `ChannelMaxLimitExceededException` | `structure` | Message | This exception is thrown when the maximum number of channels limit is exceeded. |
| `ChannelNotFoundException` | `structure` | Message | This exception is thrown when CloudTrail cannot find the specified channel. |
| `CloudTrailARNInvalidException` | `structure` | Message | This exception is thrown when an operation is called with an ARN that is not valid. The following is the format of a trail ARN: arn:aws:cloudtrail:us-east-2 ... |
| `CloudTrailAccessNotEnabledException` | `structure` | Message | This exception is thrown when trusted access has not been enabled between CloudTrail and Organizations. For more information, see How to enable or disable t ... |
| `CloudTrailInvalidClientTokenIdException` | `structure` | Message | This exception is thrown when a call results in the InvalidClientTokenId error code. This can occur when you are creating or updating a trail to send notifi ... |
| `CloudWatchLogsDeliveryUnavailableException` | `structure` | Message | Cannot set a CloudWatch Logs delivery for this Region. |
| `ConcurrentModificationException` | `structure` | Message | You are trying to update a resource when another request is in progress. Allow sufficient wait time for the previous request to complete, then retry your re ... |
| `ConflictException` | `structure` | Message | This exception is thrown when the specified resource is not ready for an operation. This can occur when you try to run an operation on a resource before Clo ... |
| `DelegatedAdminAccountLimitExceededException` | `structure` | Message | This exception is thrown when the maximum number of CloudTrail delegated administrators is reached. |
| `EventDataStoreARNInvalidException` | `structure` | Message | The specified event data store ARN is not valid or does not map to an event data store in your account. |
| `EventDataStoreAlreadyExistsException` | `structure` | Message | An event data store with that name already exists. |
| `EventDataStoreFederationEnabledException` | `structure` | Message | You cannot delete the event data store because Lake query federation is enabled. To delete the event data store, run the DisableFederation operation to disa ... |
| `EventDataStoreHasOngoingImportException` | `structure` | Message | This exception is thrown when you try to update or delete an event data store that currently has an import in progress. |
| `EventDataStoreMaxLimitExceededException` | `structure` | Message | Your account has used the maximum number of event data stores. |
| `EventDataStoreNotFoundException` | `structure` | Message | The specified event data store was not found. |
| `EventDataStoreTerminationProtectedException` | `structure` | Message | The event data store cannot be deleted because termination protection is enabled for it. |
| `GenerateResponseException` | `structure` | Message | This exception is thrown when a valid query could not be generated for the provided prompt. |
| `ImportNotFoundException` | `structure` | Message | The specified import was not found. |
| `InactiveEventDataStoreException` | `structure` | Message | The event data store is inactive. |
| `InactiveQueryException` | `structure` | Message | The specified query cannot be canceled because it is in the FINISHED , FAILED , TIMED_OUT , or CANCELLED state. |
| `InsightNotEnabledException` | `structure` | Message | If you run GetInsightSelectors on a trail or event data store that does not have Insights events enabled, the operation throws the exception InsightNotEnabl ... |
| `InsufficientDependencyServiceAccessPermissionException` | `structure` | Message | This exception is thrown when the IAM identity that is used to create the organization resource lacks one or more required permissions for creating an organ ... |
| `InsufficientEncryptionPolicyException` | `structure` | Message | For the CreateTrail PutInsightSelectors , UpdateTrail , StartQuery , and StartImport operations, this exception is thrown when the policy on the S3 bucket o ... |
| `InsufficientIAMAccessPermissionException` | `structure` | Message | The task can't be completed because you are signed in with an account that lacks permissions to view or create a service-linked role. Sign in with an accoun ... |
| `InsufficientS3BucketPolicyException` | `structure` | Message | This exception is thrown when the policy on the S3 bucket is not sufficient. |
| `InsufficientSnsTopicPolicyException` | `structure` | Message | This exception is thrown when the policy on the Amazon SNS topic is not sufficient. |
| `InvalidCloudWatchLogsLogGroupArnException` | `structure` | Message | This exception is thrown when the provided CloudWatch Logs log group is not valid. |
| `InvalidCloudWatchLogsRoleArnException` | `structure` | Message | This exception is thrown when the provided role is not valid. |
| `InvalidDateRangeException` | `structure` | Message | A date range for the query was specified that is not valid. Be sure that the start time is chronologically before the end time. For more information about w ... |
| `InvalidEventCategoryException` | `structure` | Message | Occurs if an event category that is not valid is specified as a value of EventCategory . |
| `InvalidEventDataStoreCategoryException` | `structure` | Message | This exception is thrown when event categories of specified event data stores are not valid. |
| `InvalidEventDataStoreStatusException` | `structure` | Message | The event data store is not in a status that supports the operation. |
| `InvalidEventSelectorsException` | `structure` | Message | This exception is thrown when the PutEventSelectors operation is called with a number of event selectors, advanced event selectors, or data resources that i ... |
| `InvalidHomeRegionException` | `structure` | Message | This exception is thrown when an operation is called on a trail from a Region other than the Region in which the trail was created. |
| `InvalidImportSourceException` | `structure` | Message | This exception is thrown when the provided source S3 bucket is not valid for import. |
| `InvalidInsightSelectorsException` | `structure` | Message | For PutInsightSelectors , this exception is thrown when the formatting or syntax of the InsightSelectors JSON statement is not valid, or the specified Insig ... |
| `InvalidKmsKeyIdException` | `structure` | Message | This exception is thrown when the KMS key ARN is not valid. |
| `InvalidLookupAttributesException` | `structure` | Message | Occurs when a lookup attribute is specified that is not valid. |
| `InvalidMaxResultsException` | `structure` | Message | This exception is thrown if the limit specified is not valid. |
| `InvalidNextTokenException` | `structure` | Message | A token that is not valid, or a token that was previously used in a request with different parameters. This exception is thrown if the token is not valid. |
| `InvalidParameterCombinationException` | `structure` | Message | This exception is thrown when the combination of parameters provided is not valid. |
| `InvalidParameterException` | `structure` | Message | The request includes a parameter that is not valid. |
| `InvalidQueryStatementException` | `structure` | Message | The query that was submitted has validation errors, or uses incorrect syntax or unsupported keywords. For more information about writing a query, see Create ... |
| `InvalidQueryStatusException` | `structure` | Message | The query status is not valid for the operation. |
| `InvalidS3BucketNameException` | `structure` | Message | This exception is thrown when the provided S3 bucket name is not valid. |
| `InvalidS3PrefixException` | `structure` | Message | This exception is thrown when the provided S3 prefix is not valid. |
| `InvalidSnsTopicNameException` | `structure` | Message | This exception is thrown when the provided SNS topic name is not valid. |
| `InvalidSourceException` | `structure` | Message | This exception is thrown when the specified value of Source is not valid. |
| `InvalidTagParameterException` | `structure` | Message | This exception is thrown when the specified tag key or values are not valid. It can also occur if there are duplicate tags or too many tags on the resource. |
| `InvalidTimeRangeException` | `structure` | Message | Occurs if the timestamp values are not valid. Either the start time occurs after the end time, or the time range is outside the range of possible values. |
| `InvalidTokenException` | `structure` | Message | Reserved for future use. |
| `InvalidTrailNameException` | `structure` | Message | This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements: Contain only ASCII letters (a-z, A-Z), ... |
| `KmsException` | `structure` | Message | This exception is thrown when there is an issue with the specified KMS key and the trail or event data store can't be updated. |
| `KmsKeyDisabledException` | `structure` | Message | This exception is no longer in use. |
| `KmsKeyNotFoundException` | `structure` | Message | This exception is thrown when the KMS key does not exist, when the S3 bucket and the KMS key are not in the same Region, or when the KMS key associated with ... |
| `MaxConcurrentQueriesException` | `structure` | Message | You are already running the maximum number of concurrent queries. The maximum number of concurrent queries is 10. Wait a minute for some queries to finish, ... |
| `MaximumNumberOfTrailsExceededException` | `structure` | Message | This exception is thrown when the maximum number of trails is reached. |
| `NoManagementAccountSLRExistsException` | `structure` | Message | This exception is thrown when the management account does not have a service-linked role. |
| `NotOrganizationManagementAccountException` | `structure` | Message | This exception is thrown when the account making the request is not the organization's management account. |
| `NotOrganizationMasterAccountException` | `structure` | Message | This exception is thrown when the Amazon Web Services account making the request to create or update an organization trail or event data store is not the ma ... |
| `OperationNotPermittedException` | `structure` | Message | This exception is thrown when the requested operation is not permitted. |
| `OrganizationNotInAllFeaturesModeException` | `structure` | Message | This exception is thrown when Organizations is not configured to support all features. All features must be enabled in Organizations to support creating an ... |
| `OrganizationsNotInUseException` | `structure` | Message | This exception is thrown when the request is made from an Amazon Web Services account that is not a member of an organization. To make this request, sign in ... |
| `QueryIdNotFoundException` | `structure` | Message | The query ID does not exist or does not map to a query. |
| `ResourceARNNotValidException` | `structure` | Message | This exception is thrown when the provided resource does not exist, or the ARN format of the resource is not valid. The following is the format of an event ... |
| `ResourceNotFoundException` | `structure` | Message | This exception is thrown when the specified resource is not found. |
| `ResourcePolicyNotFoundException` | `structure` | Message | This exception is thrown when the specified resource policy is not found. |
| `ResourcePolicyNotValidException` | `structure` | Message | This exception is thrown when the resouce-based policy has syntax errors, or contains a principal that is not valid. |
| `ResourceTypeNotSupportedException` | `structure` | Message | This exception is thrown when the specified resource type is not supported by CloudTrail. |
| `S3BucketDoesNotExistException` | `structure` | Message | This exception is thrown when the specified S3 bucket does not exist. |
| `ServiceQuotaExceededException` | `structure` | Message | This exception is thrown when the quota is exceeded. For information about CloudTrail quotas, see Service quotas in the Amazon Web Services General Reference . |
| `TagsLimitExceededException` | `structure` | Message | The number of tags per trail, event data store, dashboard, or channel has exceeded the permitted amount. Currently, the limit is 50. |
| `ThrottlingException` | `structure` | Message | This exception is thrown when the request rate exceeds the limit. |
| `TrailAlreadyExistsException` | `structure` | Message | This exception is thrown when the specified trail already exists. |
| `TrailNotFoundException` | `structure` | Message | This exception is thrown when the trail with the given name is not found. |
| `TrailNotProvidedException` | `structure` | Message | This exception is no longer in use. |
| `UnsupportedOperationException` | `structure` | Message | This exception is thrown when the requested operation is not supported. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
