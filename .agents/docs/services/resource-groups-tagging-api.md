# AWS Resource Groups Tagging API

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Resource Groups Tagging API

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Resource Groups Tagging API workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Get`, `Describe`, `List`, `Start`, `Tag` operation families, including `GetComplianceSummary`, `GetResources`, `GetTagKeys`, `GetTagValues`, `DescribeReportCreation`, `ListRequiredTags`.

## Service Identity and Protocol

- AWS model slug: `resource-groups-tagging-api`
- AWS SDK for Rust slug: `resourcegroupstagging`
- Model version: `2017-01-26`
- Model file: `vendor/api-models-aws/models/resource-groups-tagging-api/service/2017-01-26/resource-groups-tagging-api-2017-01-26.json`
- SDK ID: `Resource Groups Tagging API`
- Endpoint prefix: `tagging`
- ARN namespace: `tagging`
- CloudFormation name: `ResourceGroupsTaggingAPI`
- CloudTrail event source: `resourcegroupstaggingapi.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `Describe` (1), `List` (1), `Start` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `StartReportCreation`, `TagResources`, `UntagResources`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeReportCreation`, `GetComplianceSummary`, `GetResources`, `GetTagKeys`, `GetTagValues`, `ListRequiredTags`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeReportCreation`, `StartReportCreation`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 9 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.

## Operation Groups

### Get

- Operations: `GetComplianceSummary`, `GetResources`, `GetTagKeys`, `GetTagValues`
- Traits: `paginated` (4)
- Common required input members in this group: -

### Describe

- Operations: `DescribeReportCreation`
- Common required input members in this group: -

### List

- Operations: `ListRequiredTags`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartReportCreation`
- Common required input members in this group: -

### Tag

- Operations: `TagResources`
- Common required input members in this group: -

### Untag

- Operations: `UntagResources`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DescribeReportCreation` | `POST /DescribeReportCreation` | - | - | - | `DescribeReportCreationOutput` | `ConstraintViolationException`, `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Describes the status of the StartReportCreation operation. You can call this operation only from the organization's management account and from the us-east-1 Region. |
| `GetComplianceSummary` | `POST /GetComplianceSummary` | `paginated` | - | - | `GetComplianceSummaryOutput` | `ConstraintViolationException`, `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Returns a table that shows counts of resources that are noncompliant with their tag policies. For more information on tag policies, see Tag Policies in the Organizations User Guide. You can call this operation only f ... |
| `GetResources` | `POST /GetResources` | `paginated` | - | - | `GetResourcesOutput` | `InternalServiceException`, `InvalidParameterException`, `PaginationTokenExpiredException`, `ThrottledException` | Returns all the tagged or previously tagged resources that are located in the specified Amazon Web Services Region for the account. Depending on what information you want returned, you can also specify the following: ... |
| `GetTagKeys` | `POST /GetTagKeys` | `paginated` | - | - | `GetTagKeysOutput` | `InternalServiceException`, `InvalidParameterException`, `PaginationTokenExpiredException`, `ThrottledException` | Returns all tag keys currently in use in the specified Amazon Web Services Region for the calling account. This operation supports pagination, where the response can be sent in multiple pages. You should check the Pa ... |
| `GetTagValues` | `POST /GetTagValues` | `paginated` | `Key` | - | `GetTagValuesOutput` | `InternalServiceException`, `InvalidParameterException`, `PaginationTokenExpiredException`, `ThrottledException` | Returns all tag values for the specified key that are used in the specified Amazon Web Services Region for the calling account. This operation supports pagination, where the response can be sent in multiple pages. Yo ... |
| `ListRequiredTags` | `POST /ListRequiredTags` | `paginated` | - | - | `ListRequiredTagsOutput` | `InternalServiceException`, `InvalidParameterException`, `PaginationTokenExpiredException`, `ThrottledException` | Lists the required tags for supported resource types in an Amazon Web Services account. |
| `StartReportCreation` | `POST /StartReportCreation` | - | `S3Bucket` | - | `StartReportCreationOutput` | `ConcurrentModificationException`, `ConstraintViolationException`, `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Generates a report that lists all tagged resources in the accounts across your organization and tells whether each resource is compliant with the effective tag policy. Compliance data is refreshed daily. The report i ... |
| `TagResources` | `POST /TagResources` | - | `ResourceARNList`, `Tags` | - | `TagResourcesOutput` | `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Applies one or more tags to the specified resources. Note the following: Not all resources can have tags. For a list of services with resources that support tagging using this operation, see Services that support the ... |
| `UntagResources` | `POST /UntagResources` | - | `ResourceARNList`, `TagKeys` | - | `UntagResourcesOutput` | `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resourc ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConcurrentModificationException` | `structure` | Message | The request failed because the target of the operation is currently being modified by a different request. Try again later. |
| `ConstraintViolationException` | `structure` | Message | The request failed because performing the operation would violate a constraint. Some of the reasons in the following list might not apply to this specific o ... |
| `InternalServiceException` | `structure` | Message | The request processing failed because of an unknown error, exception, or failure. You can retry the request. |
| `InvalidParameterException` | `structure` | Message | The request failed because of one of the following reasons: A required parameter is missing. A provided string parameter is malformed. An provided parameter ... |
| `PaginationTokenExpiredException` | `structure` | Message | The request failed because the specified PaginationToken has expired. A PaginationToken is valid for a maximum of 15 minutes. |
| `ThrottledException` | `structure` | Message | The request failed because it exceeded the allowed frequency of submitted requests. |
| `DescribeReportCreationInput` | `structure` | **empty (no members)** | - |
| `DescribeReportCreationOutput` | `structure` | Status, S3Location, StartDate, ErrorMessage | - |
| `GetComplianceSummaryInput` | `structure` | TargetIdFilters, RegionFilters, ResourceTypeFilters, TagKeyFilters, GroupBy, MaxResults, PaginationToken | - |
| `GetComplianceSummaryOutput` | `structure` | SummaryList, PaginationToken | - |
| `GetResourcesInput` | `structure` | PaginationToken, TagFilters, ResourcesPerPage, TagsPerPage, ResourceTypeFilters, IncludeComplianceDetails, ExcludeCompliantResources, ResourceARNList | - |
| `GetResourcesOutput` | `structure` | PaginationToken, ResourceTagMappingList | - |
| `GetTagKeysInput` | `structure` | PaginationToken | - |
| `GetTagKeysOutput` | `structure` | PaginationToken, TagKeys | - |
| `GetTagValuesInput` | `structure` | PaginationToken, Key | - |
| `GetTagValuesOutput` | `structure` | PaginationToken, TagValues | - |
| `ListRequiredTagsInput` | `structure` | NextToken, MaxResults | - |
| `ListRequiredTagsOutput` | `structure` | RequiredTags, NextToken | - |
| `StartReportCreationInput` | `structure` | S3Bucket | - |
| `StartReportCreationOutput` | `structure` | **empty (no members)** | - |
| `TagResourcesInput` | `structure` | ResourceARNList, Tags | - |
| `TagResourcesOutput` | `structure` | FailedResourcesMap | - |
| `UntagResourcesInput` | `structure` | ResourceARNList, TagKeys | - |
| `UntagResourcesOutput` | `structure` | FailedResourcesMap | - |
| `ErrorCode` | `enum` | INTERNAL_SERVICE_EXCEPTION, INVALID_PARAMETER_EXCEPTION | - |
| `GroupByAttribute` | `enum` | TARGET_ID, REGION, RESOURCE_TYPE | - |
| `TargetIdType` | `enum` | ACCOUNT, OU, ROOT | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
