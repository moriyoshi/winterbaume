# AWS Compute Optimizer

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Compute Optimizer is a service that analyzes the configuration and utilization metrics of your Amazon Web Services compute resources, such as Amazon EC2 instances, Amazon EC2 Auto Scaling groups, Lambda functions, Amazon EBS volumes, and Amazon ECS services on Fargate. It reports whether your resources are optimal, and generates optimization recommendations to reduce the cost and improve the performance of your workloads. Compute Optimizer also provides recent utilization metric data, in addition to projected utilization metric data for the recommendations, which you can use to evaluate which recommendation provides the best price-performance trade-off. The analysis of your usage patterns can help you decide when to move or resize your running resources, and still meet your performance and capacity requirements. For more information about Compute Optimizer, including the required permissions to use the service, see the Compute Optimizer User Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Compute Optimizer by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented AWS Compute Optimizer workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Export`, `Delete`, `Describe`, `Put` operation families, including `GetAutoScalingGroupRecommendations`, `GetEBSVolumeRecommendations`, `GetEC2InstanceRecommendations`, `GetEC2RecommendationProjectedMetrics`, `ExportAutoScalingGroupRecommendations`, `ExportEBSVolumeRecommendations`.

## Service Identity and Protocol

- AWS model slug: `compute-optimizer`
- AWS SDK for Rust slug: `computeoptimizer`
- Model version: `2019-11-01`
- Model file: `vendor/api-models-aws/models/compute-optimizer/service/2019-11-01/compute-optimizer-2019-11-01.json`
- SDK ID: `Compute Optimizer`
- Endpoint prefix: `compute-optimizer`
- ARN namespace: `compute-optimizer`
- CloudFormation name: `ComputeOptimizer`
- CloudTrail event source: `computeoptimizer.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (16), `Export` (8), `Delete` (1), `Describe` (1), `Put` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteRecommendationPreferences`, `PutRecommendationPreferences`, `UpdateEnrollmentStatus`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeRecommendationExportJobs`, `GetAutoScalingGroupRecommendations`, `GetEBSVolumeRecommendations`, `GetEC2InstanceRecommendations`, `GetEC2RecommendationProjectedMetrics`, `GetECSServiceRecommendationProjectedMetrics`, `GetECSServiceRecommendations`, `GetEffectiveRecommendationPreferences`, `GetEnrollmentStatus`, `GetEnrollmentStatusesForOrganization`, `GetIdleRecommendations`, `GetLambdaFunctionRecommendations`, `GetLicenseRecommendations`, `GetRDSDatabaseRecommendationProjectedMetrics`, `GetRDSDatabaseRecommendations`, `GetRecommendationPreferences`, `GetRecommendationSummaries`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeRecommendationExportJobs`, `ExportAutoScalingGroupRecommendations`, `ExportEBSVolumeRecommendations`, `ExportEC2InstanceRecommendations`, `ExportECSServiceRecommendations`, `ExportIdleRecommendations`, `ExportLambdaFunctionRecommendations`, `ExportLicenseRecommendations`, `ExportRDSDatabaseRecommendations`.
- 28 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `Lambda`, `EC2/VPC`, `ECS`, `RDS`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/compute-optimizer/latest/ug/what-is-compute-optimizer.html
- https://docs.aws.amazon.com/compute-optimizer/latest/ug/view-ec2-recommendations.html
- https://docs.aws.amazon.com/compute-optimizer/latest/ug/rightsizing-preferences.html

Research outcomes:
- Compute Optimizer analyses AWS resource metrics to identify idle resources and produce rightsizing recommendations.
- Supported resources include EC2 instances and Auto Scaling groups, EBS volumes, Lambda functions, ECS services, RDS resources, and commercial software licenses where supported.
- Accounts must opt in before recommendations are generated. Organisation-wide use depends on management or delegated administration.
- Findings classify resources, such as over-provisioned, under-provisioned, optimised, or not optimised depending on resource type.
- EC2 recommendations include finding reasons, migration effort, platform differences, Graviton options, inferred workload types, performance risk, and savings estimates.
- Recommendation preferences can customise rightsizing behaviour by scope, preferred instance families, lookback period, CPU thresholds, and memory headroom.
- Savings estimation behaviour can differ depending on Cost Optimization Hub enrolment and preference settings.

Parity implications:
- Model enrolment, account scope, recommendation preferences, resource recommendations, finding reasons, savings estimates, inferred workload types, and export jobs separately.
- Recommendation generation should depend on opt-in state and observed metrics, with stale or absent metrics producing unavailable recommendations.
- Preference changes should affect newly computed recommendations rather than retroactively rewriting historical outputs without recomputation.

## Operation Groups

### Get

- Operations: `GetAutoScalingGroupRecommendations`, `GetEBSVolumeRecommendations`, `GetEC2InstanceRecommendations`, `GetEC2RecommendationProjectedMetrics`, `GetECSServiceRecommendationProjectedMetrics`, `GetECSServiceRecommendations`, `GetEffectiveRecommendationPreferences`, `GetEnrollmentStatus`, `GetEnrollmentStatusesForOrganization`, `GetIdleRecommendations`, `GetLambdaFunctionRecommendations`, `GetLicenseRecommendations`, `GetRDSDatabaseRecommendationProjectedMetrics`, `GetRDSDatabaseRecommendations`, `GetRecommendationPreferences`, `GetRecommendationSummaries`
- Traits: `paginated` (4)
- Common required input members in this group: `stat`, `period`, `startTime`, `endTime`, `resourceArn`

### Export

- Operations: `ExportAutoScalingGroupRecommendations`, `ExportEBSVolumeRecommendations`, `ExportEC2InstanceRecommendations`, `ExportECSServiceRecommendations`, `ExportIdleRecommendations`, `ExportLambdaFunctionRecommendations`, `ExportLicenseRecommendations`, `ExportRDSDatabaseRecommendations`
- Common required input members in this group: `s3DestinationConfig`

### Delete

- Operations: `DeleteRecommendationPreferences`
- Common required input members in this group: -

### Describe

- Operations: `DescribeRecommendationExportJobs`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Put

- Operations: `PutRecommendationPreferences`
- Common required input members in this group: -

### Update

- Operations: `UpdateEnrollmentStatus`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteRecommendationPreferences` | `-` | - | `resourceType`, `recommendationPreferenceNames` | - | `DeleteRecommendationPreferencesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Deletes a recommendation preference, such as enhanced infrastructure metrics. For more information, see Activating enhanced infrastructure metrics in the Compute Optimizer User Guide . |
| `DescribeRecommendationExportJobs` | `-` | `paginated` | - | - | `DescribeRecommendationExportJobsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Describes recommendation export jobs created in the last seven days. Use the ExportAutoScalingGroupRecommendations or ExportEC2InstanceRecommendations actions to request an export of your recommendations. Then use th ... |
| `ExportAutoScalingGroupRecommendations` | `-` | - | `s3DestinationConfig` | - | `ExportAutoScalingGroupRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Exports optimization recommendations for Auto Scaling groups. Recommendations are exported in a comma-separated values (.csv) file, and its metadata in a JavaScript Object Notation (JSON) (.json) file, to an existing ... |
| `ExportEBSVolumeRecommendations` | `-` | - | `s3DestinationConfig` | - | `ExportEBSVolumeRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Exports optimization recommendations for Amazon EBS volumes. Recommendations are exported in a comma-separated values (.csv) file, and its metadata in a JavaScript Object Notation (JSON) (.json) file, to an existing ... |
| `ExportEC2InstanceRecommendations` | `-` | - | `s3DestinationConfig` | - | `ExportEC2InstanceRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Exports optimization recommendations for Amazon EC2 instances. Recommendations are exported in a comma-separated values (.csv) file, and its metadata in a JavaScript Object Notation (JSON) (.json) file, to an existin ... |
| `ExportECSServiceRecommendations` | `-` | - | `s3DestinationConfig` | - | `ExportECSServiceRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Exports optimization recommendations for Amazon ECS services on Fargate. Recommendations are exported in a CSV file, and its metadata in a JSON file, to an existing Amazon Simple Storage Service (Amazon S3) bucket th ... |
| `ExportIdleRecommendations` | `-` | - | `s3DestinationConfig` | - | `ExportIdleRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Export optimization recommendations for your idle resources. Recommendations are exported in a comma-separated values (CSV) file, and its metadata in a JavaScript Object Notation (JSON) file, to an existing Amazon Si ... |
| `ExportLambdaFunctionRecommendations` | `-` | - | `s3DestinationConfig` | - | `ExportLambdaFunctionRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Exports optimization recommendations for Lambda functions. Recommendations are exported in a comma-separated values (.csv) file, and its metadata in a JavaScript Object Notation (JSON) (.json) file, to an existing Am ... |
| `ExportLicenseRecommendations` | `-` | - | `s3DestinationConfig` | - | `ExportLicenseRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Export optimization recommendations for your licenses. Recommendations are exported in a comma-separated values (CSV) file, and its metadata in a JavaScript Object Notation (JSON) file, to an existing Amazon Simple S ... |
| `ExportRDSDatabaseRecommendations` | `-` | - | `s3DestinationConfig` | - | `ExportRDSDatabaseRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Export optimization recommendations for your Amazon Aurora and Amazon Relational Database Service (Amazon RDS) databases. Recommendations are exported in a comma-separated values (CSV) file, and its metadata in a Jav ... |
| `GetAutoScalingGroupRecommendations` | `-` | - | - | - | `GetAutoScalingGroupRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns Auto Scaling group recommendations. Compute Optimizer generates recommendations for Amazon EC2 Auto Scaling groups that meet a specific set of requirements. For more information, see the Supported resources a ... |
| `GetEBSVolumeRecommendations` | `-` | - | - | - | `GetEBSVolumeRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns Amazon Elastic Block Store (Amazon EBS) volume recommendations. Compute Optimizer generates recommendations for Amazon EBS volumes that meet a specific set of requirements. For more information, see the Suppo ... |
| `GetEC2InstanceRecommendations` | `-` | - | - | - | `GetEC2InstanceRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns Amazon EC2 instance recommendations. Compute Optimizer generates recommendations for Amazon Elastic Compute Cloud (Amazon EC2) instances that meet a specific set of requirements. For more information, see the ... |
| `GetEC2RecommendationProjectedMetrics` | `-` | - | `instanceArn`, `stat`, `period`, `startTime`, `endTime` | - | `GetEC2RecommendationProjectedMetricsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns the projected utilization metrics of Amazon EC2 instance recommendations. The Cpu and Memory metrics are the only projected utilization metrics returned when you run this action. Additionally, the Memory metr ... |
| `GetECSServiceRecommendationProjectedMetrics` | `-` | - | `serviceArn`, `stat`, `period`, `startTime`, `endTime` | - | `GetECSServiceRecommendationProjectedMetricsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns the projected metrics of Amazon ECS service recommendations. |
| `GetECSServiceRecommendations` | `-` | - | - | - | `GetECSServiceRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns Amazon ECS service recommendations. Compute Optimizer generates recommendations for Amazon ECS services on Fargate that meet a specific set of requirements. For more information, see the Supported resources a ... |
| `GetEffectiveRecommendationPreferences` | `-` | - | `resourceArn` | - | `GetEffectiveRecommendationPreferencesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns the recommendation preferences that are in effect for a given resource, such as enhanced infrastructure metrics. Considers all applicable preferences that you might have set at the resource, account, and orga ... |
| `GetEnrollmentStatus` | `-` | - | - | - | `GetEnrollmentStatusResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `ServiceUnavailableException`, `ThrottlingException` | Returns the enrollment (opt in) status of an account to the Compute Optimizer service. If the account is the management account of an organization, this action also confirms the enrollment status of member accounts o ... |
| `GetEnrollmentStatusesForOrganization` | `-` | `paginated` | - | - | `GetEnrollmentStatusesForOrganizationResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `ServiceUnavailableException`, `ThrottlingException` | Returns the Compute Optimizer enrollment (opt-in) status of organization member accounts, if your account is an organization management account. To get the enrollment status of standalone accounts, use the GetEnrollm ... |
| `GetIdleRecommendations` | `-` | - | - | - | `GetIdleRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns idle resource recommendations. Compute Optimizer generates recommendations for idle resources that meet a specific set of requirements. For more information, see Resource requirements in the Compute Optimizer ... |
| `GetLambdaFunctionRecommendations` | `-` | `paginated` | - | - | `GetLambdaFunctionRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Returns Lambda function recommendations. Compute Optimizer generates recommendations for functions that meet a specific set of requirements. For more information, see the Supported resources and requirements in the C ... |
| `GetLicenseRecommendations` | `-` | - | - | - | `GetLicenseRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns license recommendations for Amazon EC2 instances that run on a specific license. Compute Optimizer generates recommendations for licenses that meet a specific set of requirements. For more information, see th ... |
| `GetRDSDatabaseRecommendationProjectedMetrics` | `-` | - | `resourceArn`, `stat`, `period`, `startTime`, `endTime` | - | `GetRDSDatabaseRecommendationProjectedMetricsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns the projected metrics of Aurora and RDS database recommendations. |
| `GetRDSDatabaseRecommendations` | `-` | - | - | - | `GetRDSDatabaseRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns Amazon Aurora and RDS database recommendations. Compute Optimizer generates recommendations for Amazon Aurora and RDS databases that meet a specific set of requirements. For more information, see the Supporte ... |
| `GetRecommendationPreferences` | `-` | `paginated` | `resourceType` | - | `GetRecommendationPreferencesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Returns existing recommendation preferences, such as enhanced infrastructure metrics. Use the scope parameter to specify which preferences to return. You can specify to return preferences for an organization, a speci ... |
| `GetRecommendationSummaries` | `-` | `paginated` | - | - | `GetRecommendationSummariesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Returns the optimization findings for an account. It returns the number of: Amazon EC2 instances in an account that are Underprovisioned , Overprovisioned , or Optimized . EC2Auto Scaling groups in an account that ar ... |
| `PutRecommendationPreferences` | `-` | - | `resourceType` | - | `PutRecommendationPreferencesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Creates a new recommendation preference or updates an existing recommendation preference, such as enhanced infrastructure metrics. For more information, see Activating enhanced infrastructure metrics in the Compute O ... |
| `UpdateEnrollmentStatus` | `-` | - | `status` | - | `UpdateEnrollmentStatusResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterValueException`, `MissingAuthenticationToken`, `ServiceUnavailableException`, `ThrottlingException` | Updates the enrollment (opt in and opt out) status of an account to the Compute Optimizer service. If the account is a management account of an organization, this action can also be used to enroll member accounts of ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | message | An internal error has occurred. Try your call again. |
| `InvalidParameterValueException` | `structure` | message | The value supplied for the input parameter is out of range or not valid. |
| `LimitExceededException` | `structure` | message | The request exceeds a limit of the service. |
| `MissingAuthenticationToken` | `structure` | message | The request must contain either a valid (registered) Amazon Web Services access key ID or X.509 certificate. |
| `OptInRequiredException` | `structure` | message | The account is not opted in to Compute Optimizer. |
| `ResourceNotFoundException` | `structure` | message | A resource that is required for the action doesn't exist. |
| `ServiceUnavailableException` | `structure` | message | The request has failed due to a temporary failure of the server. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `DeleteRecommendationPreferencesRequest` | `structure` | resourceType, scope, recommendationPreferenceNames | - |
| `DeleteRecommendationPreferencesResponse` | `structure` | **empty (no members)** | - |
| `DescribeRecommendationExportJobsRequest` | `structure` | jobIds, filters, nextToken, maxResults | - |
| `DescribeRecommendationExportJobsResponse` | `structure` | recommendationExportJobs, nextToken | - |
| `ExportAutoScalingGroupRecommendationsRequest` | `structure` | accountIds, filters, fieldsToExport, s3DestinationConfig, fileFormat, includeMemberAccounts, recommendationPreferences | - |
| `ExportAutoScalingGroupRecommendationsResponse` | `structure` | jobId, s3Destination | - |
| `ExportEBSVolumeRecommendationsRequest` | `structure` | accountIds, filters, fieldsToExport, s3DestinationConfig, fileFormat, includeMemberAccounts | - |
| `ExportEBSVolumeRecommendationsResponse` | `structure` | jobId, s3Destination | - |
| `ExportEC2InstanceRecommendationsRequest` | `structure` | accountIds, filters, fieldsToExport, s3DestinationConfig, fileFormat, includeMemberAccounts, recommendationPreferences | - |
| `ExportEC2InstanceRecommendationsResponse` | `structure` | jobId, s3Destination | - |
| `ExportECSServiceRecommendationsRequest` | `structure` | accountIds, filters, fieldsToExport, s3DestinationConfig, fileFormat, includeMemberAccounts | - |
| `ExportECSServiceRecommendationsResponse` | `structure` | jobId, s3Destination | - |
| `ExportIdleRecommendationsRequest` | `structure` | accountIds, filters, fieldsToExport, s3DestinationConfig, fileFormat, includeMemberAccounts | - |
| `ExportIdleRecommendationsResponse` | `structure` | jobId, s3Destination | - |
| `ExportLambdaFunctionRecommendationsRequest` | `structure` | accountIds, filters, fieldsToExport, s3DestinationConfig, fileFormat, includeMemberAccounts | - |
| `ExportLambdaFunctionRecommendationsResponse` | `structure` | jobId, s3Destination | - |
| `ExportLicenseRecommendationsRequest` | `structure` | accountIds, filters, fieldsToExport, s3DestinationConfig, fileFormat, includeMemberAccounts | - |
| `ExportLicenseRecommendationsResponse` | `structure` | jobId, s3Destination | - |
| `ExportRDSDatabaseRecommendationsRequest` | `structure` | accountIds, filters, fieldsToExport, s3DestinationConfig, fileFormat, includeMemberAccounts, recommendationPreferences | - |
| `ExportRDSDatabaseRecommendationsResponse` | `structure` | jobId, s3Destination | - |
| `GetAutoScalingGroupRecommendationsRequest` | `structure` | accountIds, autoScalingGroupArns, nextToken, maxResults, filters, recommendationPreferences | - |
| `GetAutoScalingGroupRecommendationsResponse` | `structure` | nextToken, autoScalingGroupRecommendations, errors | - |
| `GetEBSVolumeRecommendationsRequest` | `structure` | volumeArns, nextToken, maxResults, filters, accountIds | - |
| `GetEBSVolumeRecommendationsResponse` | `structure` | nextToken, volumeRecommendations, errors | - |
| `GetEC2InstanceRecommendationsRequest` | `structure` | instanceArns, nextToken, maxResults, filters, accountIds, recommendationPreferences | - |
| `GetEC2InstanceRecommendationsResponse` | `structure` | nextToken, instanceRecommendations, errors | - |
| `GetEC2RecommendationProjectedMetricsRequest` | `structure` | instanceArn, stat, period, startTime, endTime, recommendationPreferences | - |
| `GetEC2RecommendationProjectedMetricsResponse` | `structure` | recommendedOptionProjectedMetrics | - |
| `GetECSServiceRecommendationProjectedMetricsRequest` | `structure` | serviceArn, stat, period, startTime, endTime | - |
| `GetECSServiceRecommendationProjectedMetricsResponse` | `structure` | recommendedOptionProjectedMetrics | - |
| `GetECSServiceRecommendationsRequest` | `structure` | serviceArns, nextToken, maxResults, filters, accountIds | - |
| `AllocationStrategy` | `enum` | PRIORITIZED, LOWEST_PRICE | - |
| `AsgType` | `enum` | SINGLE_INSTANCE_TYPE, MIXED_INSTANCE_TYPE | - |
| `AutoScalingConfiguration` | `enum` | TARGET_TRACKING_SCALING_CPU, TARGET_TRACKING_SCALING_MEMORY | - |
| `CpuVendorArchitecture` | `enum` | AWS_ARM64, CURRENT | - |
| `Currency` | `enum` | USD, CNY | - |
| `CurrentPerformanceRisk` | `enum` | VERY_LOW, LOW, MEDIUM, HIGH | - |
| `CustomizableMetricHeadroom` | `enum` | PERCENT_30, PERCENT_20, PERCENT_10, PERCENT_0 | - |
| `CustomizableMetricName` | `enum` | CPU_UTILIZATION, MEMORY_UTILIZATION | - |
| `CustomizableMetricThreshold` | `enum` | P90, P95, P99_5 | - |
| `Dimension` | `enum` | SAVINGS_VALUE, SAVINGS_VALUE_AFTER_DISCOUNT | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
