# Amazon Kinesis Analytics

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Kinesis Analytics Overview This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see Amazon Kinesis Data Analytics API V2 Documentation. This is the Amazon Kinesis Analytics v1 API Reference . The Amazon Kinesis Analytics Developer Guide provides additional information.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Kinesis Analytics resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Kinesis Analytics workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Add`, `Delete`, `List`, `Create`, `Describe` operation families, including `AddApplicationCloudWatchLoggingOption`, `AddApplicationInput`, `AddApplicationInputProcessingConfiguration`, `AddApplicationOutput`, `DeleteApplication`, `DeleteApplicationCloudWatchLoggingOption`.

## Service Identity and Protocol

- AWS model slug: `kinesis-analytics`
- AWS SDK for Rust slug: `kinesisanalytics`
- Model version: `2015-08-14`
- Model file: `vendor/api-models-aws/models/kinesis-analytics/service/2015-08-14/kinesis-analytics-2015-08-14.json`
- SDK ID: `Kinesis Analytics`
- Endpoint prefix: `kinesisanalytics`
- ARN namespace: `kinesisanalytics`
- CloudFormation name: `KinesisAnalytics`
- CloudTrail event source: `kinesisanalytics.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Add` (5), `Delete` (5), `List` (2), `Create` (1), `Describe` (1), `Discover` (1), `Start` (1), `Stop` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddApplicationCloudWatchLoggingOption`, `AddApplicationInput`, `AddApplicationInputProcessingConfiguration`, `AddApplicationOutput`, `AddApplicationReferenceDataSource`, `CreateApplication`, `DeleteApplication`, `DeleteApplicationCloudWatchLoggingOption`, `DeleteApplicationInputProcessingConfiguration`, `DeleteApplicationOutput`, `DeleteApplicationReferenceDataSource`, `StartApplication`, `StopApplication`, `TagResource`, `UntagResource`, `UpdateApplication`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApplication`, `ListApplications`, `ListTagsForResource`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartApplication`, `StopApplication`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `Lambda`, `Glue`.

## Operation Groups

### Add

- Operations: `AddApplicationCloudWatchLoggingOption`, `AddApplicationInput`, `AddApplicationInputProcessingConfiguration`, `AddApplicationOutput`, `AddApplicationReferenceDataSource`
- Common required input members in this group: `ApplicationName`, `CloudWatchLoggingOption`, `CurrentApplicationVersionId`, `Input`, `InputId`, `InputProcessingConfiguration`, `Output`, `ReferenceDataSource`

### Delete

- Operations: `DeleteApplication`, `DeleteApplicationCloudWatchLoggingOption`, `DeleteApplicationInputProcessingConfiguration`, `DeleteApplicationOutput`, `DeleteApplicationReferenceDataSource`
- Common required input members in this group: `ApplicationName`, `CloudWatchLoggingOptionId`, `CreateTimestamp`, `CurrentApplicationVersionId`, `InputId`, `OutputId`, `ReferenceId`

### List

- Operations: `ListApplications`, `ListTagsForResource`
- Common required input members in this group: `ResourceARN`

### Create

- Operations: `CreateApplication`
- Common required input members in this group: `ApplicationName`

### Describe

- Operations: `DescribeApplication`
- Common required input members in this group: `ApplicationName`

### Discover

- Operations: `DiscoverInputSchema`

### Start

- Operations: `StartApplication`
- Common required input members in this group: `ApplicationName`, `InputConfigurations`

### Stop

- Operations: `StopApplication`
- Common required input members in this group: `ApplicationName`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

### Update

- Operations: `UpdateApplication`
- Common required input members in this group: `ApplicationName`, `ApplicationUpdate`, `CurrentApplicationVersionId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddApplicationCloudWatchLoggingOption` | - | - | `ApplicationName`, `CloudWatchLoggingOption`, `CurrentApplicationVersionId` | - | `AddApplicationCloudWatchLoggingOptionResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `AddApplicationInput` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `Input` | - | `AddApplicationInputResponse` | `CodeValidationException`, `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `AddApplicationInputProcessingConfiguration` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `InputId`, `InputProcessingConfiguration` | - | `AddApplicationInputProcessingConfigurationResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `AddApplicationOutput` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `Output` | - | `AddApplicationOutputResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `AddApplicationReferenceDataSource` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `ReferenceDataSource` | - | `AddApplicationReferenceDataSourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `CreateApplication` | - | - | `ApplicationName` | - | `CreateApplicationResponse` | `CodeValidationException`, `ConcurrentModificationException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `TooManyTagsException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `DeleteApplication` | - | - | `ApplicationName`, `CreateTimestamp` | - | `DeleteApplicationResponse` | `ConcurrentModificationException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `DeleteApplicationCloudWatchLoggingOption` | - | - | `ApplicationName`, `CloudWatchLoggingOptionId`, `CurrentApplicationVersionId` | - | `DeleteApplicationCloudWatchLoggingOptionResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `DeleteApplicationInputProcessingConfiguration` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `InputId` | - | `DeleteApplicationInputProcessingConfigurationResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `DeleteApplicationOutput` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `OutputId` | - | `DeleteApplicationOutputResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `DeleteApplicationReferenceDataSource` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `ReferenceId` | - | `DeleteApplicationReferenceDataSourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `DescribeApplication` | - | - | `ApplicationName` | - | `DescribeApplicationResponse` | `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `DiscoverInputSchema` | - | - | - | - | `DiscoverInputSchemaResponse` | `InvalidArgumentException`, `ResourceProvisionedThroughputExceededException`, `ServiceUnavailableException`, `UnableToDetectSchemaException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `ListApplications` | - | - | - | - | `ListApplicationsResponse` | - | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceNotFoundException` | Retrieves the list of key-value tags assigned to the application. For more information, see Using Tagging. |
| `StartApplication` | - | - | `ApplicationName`, `InputConfigurations` | - | `StartApplicationResponse` | `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `StopApplication` | - | - | `ApplicationName` | - | `StopApplicationResponse` | `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Adds one or more key-value tags to a Kinesis Analytics application. Note that the maximum number of application tags includes system tags. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Removes one or more tags from a Kinesis Analytics application. For more information, see Using Tagging. |
| `UpdateApplication` | - | - | `ApplicationName`, `ApplicationUpdate`, `CurrentApplicationVersionId` | - | `UpdateApplicationResponse` | `CodeValidationException`, `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message` | Specified application can't be found. |
| `InvalidArgumentException` | `structure` | `message` | Specified input parameter value is invalid. |
| `ResourceInUseException` | `structure` | `message` | Application is not available for this operation. |
| `ConcurrentModificationException` | `structure` | `message` | Exception thrown as a result of concurrent modification to an application. |
| `UnsupportedOperationException` | `structure` | `message` | The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. |
| `CodeValidationException` | `structure` | `message` | User-provided application code (query) is invalid. |
| `TooManyTagsException` | `structure` | `message` | Application created with too many tags, or too many tags added to an application. |
| `AddApplicationCloudWatchLoggingOptionRequest` | `structure` | `ApplicationName`, `CloudWatchLoggingOption`, `CurrentApplicationVersionId` | - |
| `AddApplicationCloudWatchLoggingOptionResponse` | `structure` | - | - |
| `AddApplicationInputRequest` | `structure` | `ApplicationName`, `CurrentApplicationVersionId`, `Input` | - |
| `AddApplicationInputResponse` | `structure` | - | - |
| `AddApplicationInputProcessingConfigurationRequest` | `structure` | `ApplicationName`, `CurrentApplicationVersionId`, `InputId`, `InputProcessingConfiguration` | - |
| `AddApplicationInputProcessingConfigurationResponse` | `structure` | - | - |
| `AddApplicationOutputRequest` | `structure` | `ApplicationName`, `CurrentApplicationVersionId`, `Output` | - |
| `AddApplicationOutputResponse` | `structure` | - | - |
| `AddApplicationReferenceDataSourceRequest` | `structure` | `ApplicationName`, `CurrentApplicationVersionId`, `ReferenceDataSource` | - |
| `AddApplicationReferenceDataSourceResponse` | `structure` | - | - |
| `CreateApplicationRequest` | `structure` | `ApplicationCode`, `ApplicationDescription`, `ApplicationName`, `CloudWatchLoggingOptions`, `Inputs`, `Outputs`, `Tags` | TBD |
| `CreateApplicationResponse` | `structure` | `ApplicationSummary` | TBD |
| `LimitExceededException` | `structure` | `message` | Exceeded the number of applications allowed. |
| `DeleteApplicationRequest` | `structure` | `ApplicationName`, `CreateTimestamp` | - |
| `DeleteApplicationResponse` | `structure` | - | - |
| `DeleteApplicationCloudWatchLoggingOptionRequest` | `structure` | `ApplicationName`, `CloudWatchLoggingOptionId`, `CurrentApplicationVersionId` | - |
| `DeleteApplicationCloudWatchLoggingOptionResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
