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
- Common required input members in this group: `Key`

### Describe

- Operations: `DescribeReportCreation`

### List

- Operations: `ListRequiredTags`
- Traits: `paginated` (1)

### Start

- Operations: `StartReportCreation`
- Common required input members in this group: `S3Bucket`

### Tag

- Operations: `TagResources`
- Common required input members in this group: `ResourceARNList`, `Tags`

### Untag

- Operations: `UntagResources`
- Common required input members in this group: `ResourceARNList`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DescribeReportCreation` | `POST /DescribeReportCreation` | - | - | - | `DescribeReportCreationOutput` | `ConstraintViolationException`, `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Describes the status of the `StartReportCreation` operation. You can call this operation only from the organization's management account and from the us-east-1 Region. |
| `GetComplianceSummary` | `POST /GetComplianceSummary` | `paginated` | - | - | `GetComplianceSummaryOutput` | `ConstraintViolationException`, `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Returns a table that shows counts of resources that are noncompliant with their tag policies. For more information on tag policies, see Tag Policies in the Organizations User Guide. |
| `GetResources` | `POST /GetResources` | `paginated` | - | - | `GetResourcesOutput` | `InternalServiceException`, `InvalidParameterException`, `PaginationTokenExpiredException`, `ThrottledException` | Returns all the tagged or previously tagged resources that are located in the specified Amazon Web Services Region for the account. Depending on what information you want returned, you can also specify the following: Filters that specify what tags and... |
| `GetTagKeys` | `POST /GetTagKeys` | `paginated` | - | - | `GetTagKeysOutput` | `InternalServiceException`, `InvalidParameterException`, `PaginationTokenExpiredException`, `ThrottledException` | Returns all tag keys currently in use in the specified Amazon Web Services Region for the calling account. This operation supports pagination, where the response can be sent in multiple pages. |
| `GetTagValues` | `POST /GetTagValues` | `paginated` | `Key` | - | `GetTagValuesOutput` | `InternalServiceException`, `InvalidParameterException`, `PaginationTokenExpiredException`, `ThrottledException` | Returns all tag values for the specified key that are used in the specified Amazon Web Services Region for the calling account. This operation supports pagination, where the response can be sent in multiple pages. |
| `ListRequiredTags` | `POST /ListRequiredTags` | `paginated` | - | - | `ListRequiredTagsOutput` | `InternalServiceException`, `InvalidParameterException`, `PaginationTokenExpiredException`, `ThrottledException` | Lists the required tags for supported resource types in an Amazon Web Services account. |
| `StartReportCreation` | `POST /StartReportCreation` | - | `S3Bucket` | - | `StartReportCreationOutput` | `ConcurrentModificationException`, `ConstraintViolationException`, `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Generates a report that lists all tagged resources in the accounts across your organization and tells whether each resource is compliant with the effective tag policy. Compliance data is refreshed daily. |
| `TagResources` | `POST /TagResources` | - | `ResourceARNList`, `Tags` | - | `TagResourcesOutput` | `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Applies one or more tags to the specified resources. Note the following: Not all resources can have tags. |
| `UntagResources` | `POST /UntagResources` | - | `ResourceARNList`, `TagKeys` | - | `UntagResourcesOutput` | `InternalServiceException`, `InvalidParameterException`, `ThrottledException` | Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceException` | `structure` | `Message` | The request processing failed because of an unknown error, exception, or failure. |
| `InvalidParameterException` | `structure` | `Message` | The request failed because of one of the following reasons: A required parameter is missing. |
| `ThrottledException` | `structure` | `Message` | The request failed because it exceeded the allowed frequency of submitted requests. |
| `PaginationTokenExpiredException` | `structure` | `Message` | The request failed because the specified `PaginationToken` has expired. |
| `ConstraintViolationException` | `structure` | `Message` | The request failed because performing the operation would violate a constraint. |
| `DescribeReportCreationInput` | `structure` | - | - |
| `DescribeReportCreationOutput` | `structure` | `ErrorMessage`, `S3Location`, `StartDate`, `Status` | - |
| `GetComplianceSummaryInput` | `structure` | `GroupBy`, `MaxResults`, `PaginationToken`, `RegionFilters`, `ResourceTypeFilters`, `TagKeyFilters`, `TargetIdFilters` | - |
| `GetComplianceSummaryOutput` | `structure` | `PaginationToken`, `SummaryList` | - |
| `GetResourcesInput` | `structure` | `ExcludeCompliantResources`, `IncludeComplianceDetails`, `PaginationToken`, `ResourceARNList`, `ResourceTypeFilters`, `ResourcesPerPage`, `TagFilters`, `TagsPerPage` | - |
| `GetResourcesOutput` | `structure` | `PaginationToken`, `ResourceTagMappingList` | - |
| `GetTagKeysInput` | `structure` | `PaginationToken` | - |
| `GetTagKeysOutput` | `structure` | `PaginationToken`, `TagKeys` | - |
| `GetTagValuesInput` | `structure` | `Key`, `PaginationToken` | - |
| `GetTagValuesOutput` | `structure` | `PaginationToken`, `TagValues` | - |
| `ListRequiredTagsInput` | `structure` | `MaxResults`, `NextToken` | - |
| `ListRequiredTagsOutput` | `structure` | `NextToken`, `RequiredTags` | - |
| `StartReportCreationInput` | `structure` | `S3Bucket` | - |
| `StartReportCreationOutput` | `structure` | - | - |
| `ConcurrentModificationException` | `structure` | `Message` | The request failed because the target of the operation is currently being modified by a different request. |
| `TagResourcesInput` | `structure` | `ResourceARNList`, `Tags` | - |
| `TagResourcesOutput` | `structure` | `FailedResourcesMap` | - |
| `UntagResourcesInput` | `structure` | `ResourceARNList`, `TagKeys` | - |
| `UntagResourcesOutput` | `structure` | `FailedResourcesMap` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
