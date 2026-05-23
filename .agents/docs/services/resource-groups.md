# AWS Resource Groups

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Resource Groups lets you organize Amazon Web Services resources such as Amazon Elastic Compute Cloud instances, Amazon Relational Database Service databases, and Amazon Simple Storage Service buckets into groups using criteria that you define as tags. A resource group is a collection of resources that match the resource types specified in a query, and share one or more tags or portions of tags. You can create a group of resources based on their roles in your cloud infrastructure, lifecycle stages, regions, application layers, or virtually any criteria. Resource Groups enable you to automate management tasks, such as those in Amazon Web Services Systems Manager Automation documents, on tag-related resources in Amazon Web Services Systems Manager. Groups of tagged resources also let you quickly view a custom console in Amazon Web Services Systems Manager that shows Config compliance and other monitoring data about member resources.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-resourcegroups/tests/scenario_test.rs`: create an application environment group, manage its tags, list/describe it, and delete it.
- Backported from `scenario_test.rs`: manage a configuration-backed capacity reservation group and its resources.
- Backported from `scenario_test.rs`: create, inspect, and clean up a tag-sync task.
- Scenario insight from EC2: exercise account or service defaults for AWS Resource Groups by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS Resource Groups resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support grouping AWS resources by tag query or configuration, resource membership listing, configuration updates, tag-sync automation, and lifecycle/tag management.

## Service Identity and Protocol

- AWS model slug: `resource-groups`
- AWS SDK for Rust slug: `resourcegroups`
- Model version: `2017-11-27`
- Model file: `vendor/api-models-aws/models/resource-groups/service/2017-11-27/resource-groups-2017-11-27.json`
- SDK ID: `Resource Groups`
- Endpoint prefix: `resource-groups`
- ARN namespace: `resource-groups`
- CloudFormation name: `ResourceGroups`
- CloudTrail event source: `resourcegroups.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (6), `List` (4), `Update` (3), `Cancel` (1), `Create` (1), `Delete` (1), `Group` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelTagSyncTask`, `CreateGroup`, `DeleteGroup`, `PutGroupConfiguration`, `StartTagSyncTask`, `Tag`, `Untag`, `UpdateAccountSettings`, `UpdateGroup`, `UpdateGroupQuery`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountSettings`, `GetGroup`, `GetGroupConfiguration`, `GetGroupQuery`, `GetTagSyncTask`, `GetTags`, `ListGroupResources`, `ListGroupingStatuses`, `ListGroups`, `ListTagSyncTasks`, `SearchResources`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelTagSyncTask`, `GetTagSyncTask`, `ListTagSyncTasks`, `StartTagSyncTask`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `ECS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/ARG/latest/userguide/resource-groups.html
- https://docs.aws.amazon.com/ARG/latest/userguide/gettingstarted-query.html
- https://docs.aws.amazon.com/ARG/latest/userguide/updating-tag-based-resource-groups.html

Research outcomes:
- AWS Resource Groups organises supported AWS resources into groups for viewing and bulk operations.
- Groups can be tag-based query groups or CloudFormation stack-based groups.
- Tag-based groups use resource type filters and tag filters to determine membership.
- CloudFormation stack-based groups use stack identity and resource filters.
- Group membership is query-derived and can change when resource tags or stack resources change.
- Resource groups have their own names, descriptions, tags, and resource query definitions.
- IAM permissions control who can create, update, view, and act on resource groups.

Parity implications:
- Model groups, resource queries, query type, tag filters, resource type filters, stack identifiers, tags, and derived membership separately.
- Membership should be evaluated from current resource/tag state rather than stored permanently.
- Updating group query definitions should change future membership without mutating underlying resources.

## Operation Groups

### Get

- Operations: `GetAccountSettings`, `GetGroup`, `GetGroupConfiguration`, `GetGroupQuery`, `GetTags`, `GetTagSyncTask`
- Common required input members in this group: -

### List

- Operations: `ListGroupingStatuses`, `ListGroupResources`, `ListGroups`, `ListTagSyncTasks`
- Traits: `paginated` (4)
- Common required input members in this group: -

### Update

- Operations: `UpdateAccountSettings`, `UpdateGroup`, `UpdateGroupQuery`
- Common required input members in this group: -

### Cancel

- Operations: `CancelTagSyncTask`
- Common required input members in this group: -

### Create

- Operations: `CreateGroup`
- Common required input members in this group: -

### Delete

- Operations: `DeleteGroup`
- Common required input members in this group: -

### Group

- Operations: `GroupResources`
- Common required input members in this group: -

### Put

- Operations: `PutGroupConfiguration`
- Common required input members in this group: -

### Search

- Operations: `SearchResources`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartTagSyncTask`
- Common required input members in this group: -

### Tag

- Operations: `Tag`
- Common required input members in this group: -

### Ungroup

- Operations: `UngroupResources`
- Common required input members in this group: -

### Untag

- Operations: `Untag`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelTagSyncTask` | `POST /cancel-tag-sync-task` | - | `TaskArn` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException`, `UnauthorizedException` | Cancels the specified tag-sync task. Minimum permissions To run this command, you must have the following permissions: resource-groups:CancelTagSyncTask on the application group resource-groups:DeleteGroup |
| `CreateGroup` | `POST /groups` | - | `Name` | - | `CreateGroupOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Creates a resource group with the specified name and description. You can optionally include either a resource query or a service configuration. For more information about constructing a resource query, see Build que ... |
| `DeleteGroup` | `POST /delete-group` | - | - | - | `DeleteGroupOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Deletes the specified resource group. Deleting a resource group does not delete any resources that are members of the group; it only deletes the group structure. Minimum permissions To run this command, you must have ... |
| `GetAccountSettings` | `POST /get-account-settings` | - | - | - | `GetAccountSettingsOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Retrieves the current status of optional features in Resource Groups. |
| `GetGroup` | `POST /get-group` | - | - | - | `GetGroupOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Returns information about a specified resource group. Minimum permissions To run this command, you must have the following permissions: resource-groups:GetGroup |
| `GetGroupConfiguration` | `POST /get-group-configuration` | - | - | - | `GetGroupConfigurationOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the service configuration associated with the specified resource group. For details about the service configuration syntax, see Service configurations for Resource Groups . Minimum permissions To run this c ... |
| `GetGroupQuery` | `POST /get-group-query` | - | - | - | `GetGroupQueryOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the resource query associated with the specified resource group. For more information about resource queries, see Create a tag-based group in Resource Groups . Minimum permissions To run this command, you m ... |
| `GetTags` | `GET /resources/{Arn}/tags` | - | `Arn` | - | `GetTagsOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Returns a list of tags that are associated with a resource group, specified by an Amazon resource name (ARN). Minimum permissions To run this command, you must have the following permissions: resource-groups:GetTags |
| `GetTagSyncTask` | `POST /get-tag-sync-task` | - | `TaskArn` | - | `GetTagSyncTaskOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Returns information about a specified tag-sync task. Minimum permissions To run this command, you must have the following permissions: resource-groups:GetTagSyncTask on the application group |
| `GroupResources` | `POST /group-resources` | - | `Group`, `ResourceArns` | - | `GroupResourcesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Adds the specified resources to the specified group. You can only use this operation with the following groups: AWS::EC2::HostManagement AWS::EC2::CapacityReservationPool AWS::ResourceGroups::ApplicationGroup Other r ... |
| `ListGroupingStatuses` | `POST /list-grouping-statuses` | `paginated` | `Group` | - | `ListGroupingStatusesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Returns the status of the last grouping or ungrouping action for each resource in the specified application group. |
| `ListGroupResources` | `POST /list-group-resources` | `paginated` | - | - | `ListGroupResourcesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Returns a list of Amazon resource names (ARNs) of the resources that are members of a specified resource group. Minimum permissions To run this command, you must have the following permissions: resource-groups:ListGr ... |
| `ListGroups` | `POST /groups-list` | `paginated` | - | - | `ListGroupsOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Returns a list of existing Resource Groups in your account. Minimum permissions To run this command, you must have the following permissions: resource-groups:ListGroups |
| `ListTagSyncTasks` | `POST /list-tag-sync-tasks` | `paginated` | - | - | `ListTagSyncTasksOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException`, `UnauthorizedException` | Returns a list of tag-sync tasks. Minimum permissions To run this command, you must have the following permissions: resource-groups:ListTagSyncTasks with the group passed in the filters as the resource or * if using ... |
| `PutGroupConfiguration` | `POST /put-group-configuration` | - | - | - | `PutGroupConfigurationOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Attaches a service configuration to the specified group. This occurs asynchronously, and can take time to complete. You can use GetGroupConfiguration to check the status of the update. Minimum permissions To run this ... |
| `SearchResources` | `POST /resources/search` | `paginated` | `ResourceQuery` | - | `SearchResourcesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException`, `UnauthorizedException` | Returns a list of Amazon Web Services resource identifiers that matches the specified query. The query uses the same format as a resource query in a CreateGroup or UpdateGroupQuery operation. Minimum permissions To r ... |
| `StartTagSyncTask` | `POST /start-tag-sync-task` | - | `Group`, `RoleArn` | - | `StartTagSyncTaskOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a new tag-sync task to onboard and sync resources tagged with a specific tag key-value pair to an application. To start a tag-sync task, you need a resource tagging role . The resource tagging role grants per ... |
| `Tag` | `PUT /resources/{Arn}/tags` | - | `Arn`, `Tags` | - | `TagOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Adds tags to a resource group with the specified Amazon resource name (ARN). Existing tags on a resource group are not changed if they are not specified in the request parameters. Do not store personally identifiable ... |
| `UngroupResources` | `POST /ungroup-resources` | - | `Group`, `ResourceArns` | - | `UngroupResourcesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Removes the specified resources from the specified group. This operation works only with static groups that you populated using the GroupResources operation. It doesn't work with any resource groups that are automati ... |
| `Untag` | `PATCH /resources/{Arn}/tags` | - | `Arn`, `Keys` | - | `UntagOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Deletes tags from a specified resource group. Minimum permissions To run this command, you must have the following permissions: resource-groups:Untag |
| `UpdateAccountSettings` | `POST /update-account-settings` | - | - | - | `UpdateAccountSettingsOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Turns on or turns off optional features in Resource Groups. The preceding example shows that the request to turn on group lifecycle events is IN_PROGRESS . You can call the GetAccountSettings operation to check for c ... |
| `UpdateGroup` | `POST /update-group` | - | - | - | `UpdateGroupOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Updates the description for an existing group. You cannot update the name of a resource group. Minimum permissions To run this command, you must have the following permissions: resource-groups:UpdateGroup |
| `UpdateGroupQuery` | `POST /update-group-query` | - | `ResourceQuery` | - | `UpdateGroupQueryOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Updates the resource query of a group. For more information about resource queries, see Create a tag-based group in Resource Groups . Minimum permissions To run this command, you must have the following permissions: ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListGroups` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | Message | The request includes one or more parameters that violate validation rules. |
| `ForbiddenException` | `structure` | Message | The caller isn't authorized to make the request. Check permissions. |
| `InternalServerErrorException` | `structure` | Message | An internal error occurred while processing the request. Try again later. |
| `MethodNotAllowedException` | `structure` | Message | The request uses an HTTP method that isn't allowed for the specified resource. |
| `NotFoundException` | `structure` | Message | One or more of the specified resources don't exist. |
| `TooManyRequestsException` | `structure` | Message | You've exceeded throttling limits by making too many requests in a period of time. |
| `UnauthorizedException` | `structure` | Message | The request was rejected because it doesn't have valid credentials for the target resource. |
| `CancelTagSyncTaskInput` | `structure` | TaskArn | - |
| `CreateGroupInput` | `structure` | Name, Description, ResourceQuery, Tags, Configuration, Criticality, Owner, DisplayName | - |
| `CreateGroupOutput` | `structure` | Group, ResourceQuery, Tags, GroupConfiguration | - |
| `DeleteGroupInput` | `structure` | GroupName, Group | - |
| `DeleteGroupOutput` | `structure` | Group | - |
| `GetAccountSettingsOutput` | `structure` | AccountSettings | - |
| `GetGroupInput` | `structure` | GroupName, Group | - |
| `GetGroupOutput` | `structure` | Group | - |
| `GetGroupConfigurationInput` | `structure` | Group | - |
| `GetGroupConfigurationOutput` | `structure` | GroupConfiguration | - |
| `GetGroupQueryInput` | `structure` | GroupName, Group | - |
| `GetGroupQueryOutput` | `structure` | GroupQuery | - |
| `GetTagsInput` | `structure` | Arn | - |
| `GetTagsOutput` | `structure` | Arn, Tags | - |
| `GetTagSyncTaskInput` | `structure` | TaskArn | - |
| `GetTagSyncTaskOutput` | `structure` | GroupArn, GroupName, TaskArn, TagKey, TagValue, ResourceQuery, RoleArn, Status, ErrorMessage, CreatedAt | - |
| `GroupResourcesInput` | `structure` | Group, ResourceArns | - |
| `GroupResourcesOutput` | `structure` | Succeeded, Failed, Pending | - |
| `ListGroupingStatusesInput` | `structure` | Group, MaxResults, Filters, NextToken | - |
| `ListGroupingStatusesOutput` | `structure` | Group, GroupingStatuses, NextToken | - |
| `ListGroupResourcesInput` | `structure` | GroupName, Group, Filters, MaxResults, NextToken | - |
| `ListGroupResourcesOutput` | `structure` | Resources, ResourceIdentifiers, NextToken, QueryErrors | - |
| `ListGroupsInput` | `structure` | Filters, MaxResults, NextToken | - |
| `ListGroupsOutput` | `structure` | GroupIdentifiers, Groups, NextToken | - |
| `ListTagSyncTasksInput` | `structure` | Filters, MaxResults, NextToken | - |
| `ListTagSyncTasksOutput` | `structure` | TagSyncTasks, NextToken | - |
| `PutGroupConfigurationInput` | `structure` | Group, Configuration | - |
| `PutGroupConfigurationOutput` | `structure` | **empty (no members)** | - |
| `SearchResourcesInput` | `structure` | ResourceQuery, MaxResults, NextToken | - |
| `SearchResourcesOutput` | `structure` | ResourceIdentifiers, NextToken, QueryErrors | - |
| `StartTagSyncTaskInput` | `structure` | Group, TagKey, TagValue, ResourceQuery, RoleArn | - |
| `StartTagSyncTaskOutput` | `structure` | GroupArn, GroupName, TaskArn, TagKey, TagValue, ResourceQuery, RoleArn | - |
| `TagInput` | `structure` | Arn, Tags | - |
| `GroupConfigurationStatus` | `enum` | UPDATING, UPDATE_COMPLETE, UPDATE_FAILED | - |
| `GroupFilterName` | `enum` | ResourceType, ConfigurationType, Owner, DisplayName, Criticality | - |
| `GroupLifecycleEventsDesiredStatus` | `enum` | ACTIVE, INACTIVE | - |
| `GroupLifecycleEventsStatus` | `enum` | ACTIVE, INACTIVE, IN_PROGRESS, ERROR | - |
| `GroupingStatus` | `enum` | SUCCESS, FAILED, IN_PROGRESS, SKIPPED | - |
| `GroupingType` | `enum` | GROUP, UNGROUP | - |
| `ListGroupingStatusesFilterName` | `enum` | Status, ResourceArn | - |
| `QueryErrorCode` | `enum` | CLOUDFORMATION_STACK_INACTIVE, CLOUDFORMATION_STACK_NOT_EXISTING, CLOUDFORMATION_STACK_UNASSUMABLE_ROLE, RESOURCE_TYPE_NOT_SUPPORTED | - |
| `QueryType` | `enum` | TAG_FILTERS_1_0, CLOUDFORMATION_STACK_1_0 | - |
| `ResourceFilterName` | `enum` | ResourceType | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
