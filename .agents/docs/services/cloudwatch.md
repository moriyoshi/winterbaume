# Amazon CloudWatch

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon CloudWatch monitors your Amazon Web Services (Amazon Web Services) resources and the applications you run on Amazon Web Services in real time. You can use CloudWatch to collect and track metrics, which are the variables you want to measure for your resources and applications. CloudWatch alarms send notifications or automatically change the resources you are monitoring based on rules that you define. For example, you can monitor the CPU usage and disk reads and writes of your Amazon EC2 instances. Then, use this data to determine whether you should launch additional instances to handle increased load.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-cloudwatch/tests/scenario_test.rs`: create an alarm, publish breaching metric data, observe alarm state, acknowledge/remediate, and delete the alarm.
- Backported from `scenario_test.rs`: publish a dashboard, inspect its body and metadata, then tear it down.
- Backported from `scenario_test.rs`: create, start, stop, and delete a metric stream.
- Scenario insight from EC2: exercise account or service defaults for Amazon CloudWatch by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Amazon CloudWatch resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support metric ingestion, alarm evaluation, dashboards, anomaly/insight state, metric streams, tagging, and integration points with EventBridge, SNS, Kinesis Data Firehose, and IAM roles.

## Service Identity and Protocol

- AWS model slug: `cloudwatch`
- AWS SDK for Rust slug: `cloudwatch`
- Model version: `2010-08-01`
- Model file: `vendor/api-models-aws/models/cloudwatch/service/2010-08-01/cloudwatch-2010-08-01.json`
- SDK ID: `CloudWatch`
- Endpoint prefix: `monitoring`
- ARN namespace: `monitoring`
- CloudFormation name: `CloudWatch`
- CloudTrail event source: `monitoring.amazonaws.com`
- Protocols: `awsJson1_0`, `awsQuery`, `awsQueryCompatible`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Put` (9), `Get` (7), `Delete` (6), `Describe` (6), `List` (6), `Disable` (2), `Enable` (2), `Set` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteAlarmMuteRule`, `DeleteAlarms`, `DeleteAnomalyDetector`, `DeleteDashboards`, `DeleteInsightRules`, `DeleteMetricStream`, `DisableAlarmActions`, `DisableInsightRules`, `EnableAlarmActions`, `EnableInsightRules`, `PutAlarmMuteRule`, `PutAnomalyDetector`, `PutCompositeAlarm`, `PutDashboard`, `PutInsightRule`, `PutManagedInsightRules`, `PutMetricAlarm`, `PutMetricData`, `PutMetricStream`, `SetAlarmState`, ... (+4).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAlarmContributors`, `DescribeAlarmHistory`, `DescribeAlarms`, `DescribeAlarmsForMetric`, `DescribeAnomalyDetectors`, `DescribeInsightRules`, `GetAlarmMuteRule`, `GetDashboard`, `GetInsightRuleReport`, `GetMetricData`, `GetMetricStatistics`, `GetMetricStream`, `GetMetricWidgetImage`, `ListAlarmMuteRules`, `ListDashboards`, `ListManagedInsightRules`, `ListMetricStreams`, `ListMetrics`, `ListTagsForResource`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetInsightRuleReport`, `StartMetricStreams`, `StopMetricStreams`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 38 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `EC2/VPC`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html

Research outcomes:
- Metrics are time-ordered datapoints scoped to a Region. CloudWatch cannot delete metrics directly, and stale metrics expire after 15 months without new data.
- Metric identity is the combination of namespace, metric name, and zero or more dimensions. Each distinct dimension combination is a separate metric.
- Custom metrics are not automatically aggregated across dimensions. Statistics are retrieved for the exact dimension combination that was published.
- Datapoint timestamps can be up to two weeks in the past or two hours in the future; omitted timestamps use receive time, in UTC.
- High-resolution metrics support one-second storage resolution. Valid statistic periods include 1, 5, 10, 30, or multiples of 60 seconds where appropriate.
- Retention is tiered: sub-minute datapoints are retained briefly, then data is aggregated into coarser periods over time, up to 455 days for hourly datapoints.
- Percentile statistics require suitable raw datapoints and are unavailable for negative metric values.
- Alarms act on state changes after sustained evaluation, not merely because the alarm is already in a state.

Parity implications:
- Metric storage should be keyed by namespace, name, unit, and exact dimensions, with timestamp validation and retention-aware aggregation.
- Statistic retrieval needs period alignment, UTC timestamps, unit filtering, and percentile limitations.
- Alarm evaluation should maintain state, evaluation windows, missing-data handling, and action-on-transition semantics.

## Operation Groups

### Put

- Operations: `PutAlarmMuteRule`, `PutAnomalyDetector`, `PutCompositeAlarm`, `PutDashboard`, `PutInsightRule`, `PutManagedInsightRules`, `PutMetricAlarm`, `PutMetricData`, `PutMetricStream`
- Common required input members in this group: `AlarmName`, `AlarmRule`, `ComparisonOperator`, `DashboardBody`, `DashboardName`, `EvaluationPeriods`, `FirehoseArn`, `ManagedRules`, `Name`, `Namespace`, `OutputFormat`, `RoleArn`, `Rule`, `RuleDefinition`, `RuleName`

### Get

- Operations: `GetAlarmMuteRule`, `GetDashboard`, `GetInsightRuleReport`, `GetMetricData`, `GetMetricStatistics`, `GetMetricStream`, `GetMetricWidgetImage`
- Traits: `paginated` (1)
- Common required input members in this group: `AlarmMuteRuleName`, `DashboardName`, `EndTime`, `MetricDataQueries`, `MetricName`, `MetricWidget`, `Name`, `Namespace`, `Period`, `RuleName`, `StartTime`

### Delete

- Operations: `DeleteAlarmMuteRule`, `DeleteAlarms`, `DeleteAnomalyDetector`, `DeleteDashboards`, `DeleteInsightRules`, `DeleteMetricStream`
- Common required input members in this group: `AlarmMuteRuleName`, `AlarmNames`, `DashboardNames`, `Name`, `RuleNames`

### Describe

- Operations: `DescribeAlarmContributors`, `DescribeAlarmHistory`, `DescribeAlarms`, `DescribeAlarmsForMetric`, `DescribeAnomalyDetectors`, `DescribeInsightRules`
- Traits: `paginated` (4)
- Common required input members in this group: `AlarmName`, `MetricName`, `Namespace`

### List

- Operations: `ListAlarmMuteRules`, `ListDashboards`, `ListManagedInsightRules`, `ListMetricStreams`, `ListMetrics`, `ListTagsForResource`
- Traits: `paginated` (5)
- Common required input members in this group: `ResourceARN`

### Disable

- Operations: `DisableAlarmActions`, `DisableInsightRules`
- Common required input members in this group: `AlarmNames`, `RuleNames`

### Enable

- Operations: `EnableAlarmActions`, `EnableInsightRules`
- Common required input members in this group: `AlarmNames`, `RuleNames`

### Set

- Operations: `SetAlarmState`
- Common required input members in this group: `AlarmName`, `StateReason`, `StateValue`

### Start

- Operations: `StartMetricStreams`
- Common required input members in this group: `Names`

### Stop

- Operations: `StopMetricStreams`
- Common required input members in this group: `Names`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteAlarmMuteRule` | - | - | `AlarmMuteRuleName` | - | `Unit` | - | Deletes a specific alarm mute rule. When you delete a mute rule, any alarms that are currently being muted by that rule are immediately unmuted. |
| `DeleteAlarms` | - | - | `AlarmNames` | - | `Unit` | `ResourceNotFound` | Deletes the specified alarms. You can delete up to 100 alarms in one operation. |
| `DeleteAnomalyDetector` | - | - | - | - | `DeleteAnomalyDetectorOutput` | `InternalServiceFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Deletes the specified anomaly detection model from your account. For more information about how to delete an anomaly detection model, see Deleting an anomaly detection model in the CloudWatch User Guide . |
| `DeleteDashboards` | - | - | `DashboardNames` | - | `DeleteDashboardsOutput` | `ConflictException`, `DashboardNotFoundError`, `InternalServiceFault`, `InvalidParameterValueException` | Deletes all dashboards that you specify. You can specify up to 100 dashboards to delete. |
| `DeleteInsightRules` | - | - | `RuleNames` | - | `DeleteInsightRulesOutput` | `InvalidParameterValueException`, `MissingRequiredParameterException` | Permanently deletes the specified Contributor Insights rules. If you create a rule, delete it, and then re-create it with the same name, historical data from the first time the rule was created might not be available. |
| `DeleteMetricStream` | - | - | `Name` | - | `DeleteMetricStreamOutput` | `InternalServiceFault`, `InvalidParameterValueException`, `MissingRequiredParameterException` | Permanently deletes the metric stream that you specify. |
| `DescribeAlarmContributors` | - | - | `AlarmName` | - | `DescribeAlarmContributorsOutput` | `InvalidNextToken`, `ResourceNotFoundException` | Returns the information of the current alarm contributors that are in `ALARM` state. This operation returns details about the individual time series that contribute to the alarm's state. |
| `DescribeAlarmHistory` | - | `paginated` | - | - | `DescribeAlarmHistoryOutput` | `InvalidNextToken` | Retrieves the history for the specified alarm. You can filter the results by date range or item type. |
| `DescribeAlarms` | - | `paginated` | - | - | `DescribeAlarmsOutput` | `InvalidNextToken` | Retrieves the specified alarms. You can filter the results by specifying a prefix for the alarm name, the alarm state, or a prefix for any action. |
| `DescribeAlarmsForMetric` | - | - | `MetricName`, `Namespace` | - | `DescribeAlarmsForMetricOutput` | - | Retrieves the alarms for the specified metric. To filter the results, specify a statistic, period, or unit. |
| `DescribeAnomalyDetectors` | - | `paginated` | - | - | `DescribeAnomalyDetectorsOutput` | `InternalServiceFault`, `InvalidNextToken`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Lists the anomaly detection models that you have created in your account. For single metric anomaly detectors, you can list all of the models in your account or filter the results to only the models that are related to a certain namespace, metric name, or... |
| `DescribeInsightRules` | - | `paginated` | - | - | `DescribeInsightRulesOutput` | `InvalidNextToken` | Returns a list of all the Contributor Insights rules in your account. For more information about Contributor Insights, see Using Contributor Insights to Analyze High-Cardinality Data. |
| `DisableAlarmActions` | - | - | `AlarmNames` | - | `Unit` | - | Disables the actions for the specified alarms. When an alarm's actions are disabled, the alarm actions do not execute when the alarm state changes. |
| `DisableInsightRules` | - | - | `RuleNames` | - | `DisableInsightRulesOutput` | `InvalidParameterValueException`, `MissingRequiredParameterException` | Disables the specified Contributor Insights rules. When rules are disabled, they do not analyze log groups and do not incur costs. |
| `EnableAlarmActions` | - | - | `AlarmNames` | - | `Unit` | - | Enables the actions for the specified alarms. |
| `EnableInsightRules` | - | - | `RuleNames` | - | `EnableInsightRulesOutput` | `InvalidParameterValueException`, `LimitExceededException`, `MissingRequiredParameterException` | Enables the specified Contributor Insights rules. When rules are enabled, they immediately begin analyzing log data. |
| `GetAlarmMuteRule` | - | - | `AlarmMuteRuleName` | - | `GetAlarmMuteRuleOutput` | `ResourceNotFoundException` | Retrieves details for a specific alarm mute rule. This operation returns complete information about the mute rule, including its configuration, status, targeted alarms, and metadata. |
| `GetDashboard` | - | - | `DashboardName` | - | `GetDashboardOutput` | `DashboardNotFoundError`, `InternalServiceFault`, `InvalidParameterValueException` | Displays the details of the dashboard that you specify. To copy an existing dashboard, use `GetDashboard`, and then use the data returned within `DashboardBody` as the template for the new dashboard when you call `PutDashboard` to create the copy. |
| `GetInsightRuleReport` | - | - | `EndTime`, `Period`, `RuleName`, `StartTime` | - | `GetInsightRuleReportOutput` | `InvalidParameterValueException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | This operation returns the time series data collected by a Contributor Insights rule. The data includes the identity and number of contributors to the log group. |
| `GetMetricData` | - | `paginated` | `EndTime`, `MetricDataQueries`, `StartTime` | - | `GetMetricDataOutput` | `InvalidNextToken` | You can use the `GetMetricData` API to retrieve CloudWatch metric values. The operation can also include a CloudWatch Metrics Insights query, and one or more metric math functions. |
| `GetMetricStatistics` | - | - | `EndTime`, `MetricName`, `Namespace`, `Period`, `StartTime` | - | `GetMetricStatisticsOutput` | `InternalServiceFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MissingRequiredParameterException` | Gets statistics for the specified metric. The maximum number of data points returned from a single call is 1,440. |
| `GetMetricStream` | - | - | `Name` | - | `GetMetricStreamOutput` | `InternalServiceFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Returns information about the metric stream that you specify. |
| `GetMetricWidgetImage` | - | - | `MetricWidget` | - | `GetMetricWidgetImageOutput` | - | You can use the `GetMetricWidgetImage` API to retrieve a snapshot graph of one or more Amazon CloudWatch metrics as a bitmap image. You can then embed this image into your services and products, such as wiki pages, reports, and documents. |
| `ListAlarmMuteRules` | - | `paginated` | - | - | `ListAlarmMuteRulesOutput` | `InvalidNextToken`, `ResourceNotFoundException` | Lists alarm mute rules in your Amazon Web Services account and region. You can filter the results by alarm name to find all mute rules targeting a specific alarm, or by status to find rules that are scheduled, active, or expired. |
| `ListDashboards` | - | `paginated` | - | - | `ListDashboardsOutput` | `InternalServiceFault`, `InvalidParameterValueException` | Returns a list of the dashboards for your account. If you include `DashboardNamePrefix`, only those dashboards with names starting with the prefix are listed. |
| `ListManagedInsightRules` | - | `paginated` | `ResourceARN` | - | `ListManagedInsightRulesOutput` | `InvalidNextToken`, `InvalidParameterValueException`, `MissingRequiredParameterException` | Returns a list that contains the number of managed Contributor Insights rules in your account. |
| `ListMetricStreams` | - | `paginated` | - | - | `ListMetricStreamsOutput` | `InternalServiceFault`, `InvalidNextToken`, `InvalidParameterValueException`, `MissingRequiredParameterException` | Returns a list of metric streams in this account. |
| `ListMetrics` | - | `paginated` | - | - | `ListMetricsOutput` | `InternalServiceFault`, `InvalidParameterValueException` | List the specified metrics. You can use the returned metrics with GetMetricData or GetMetricStatistics to get statistical data. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceOutput` | `InternalServiceFault`, `InvalidParameterValueException`, `ResourceNotFoundException` | Displays the tags associated with a CloudWatch resource. Currently, alarms and Contributor Insights rules support tagging. |
| `PutAlarmMuteRule` | - | - | `Name`, `Rule` | - | `Unit` | `LimitExceededFault` | Creates or updates an alarm mute rule. Alarm mute rules automatically mute alarm actions during predefined time windows. |
| `PutAnomalyDetector` | - | - | - | - | `PutAnomalyDetectorOutput` | `InternalServiceFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingRequiredParameterException` | Creates an anomaly detection model for a CloudWatch metric. You can use the model to display a band of expected normal values when the metric is graphed. |
| `PutCompositeAlarm` | - | - | `AlarmName`, `AlarmRule` | - | `Unit` | `LimitExceededFault` | Creates or updates a composite alarm . When you create a composite alarm, you specify a rule expression for the alarm that takes into account the alarm states of other alarms that you have created. |
| `PutDashboard` | - | - | `DashboardBody`, `DashboardName` | - | `PutDashboardOutput` | `ConflictException`, `DashboardInvalidInputError`, `InternalServiceFault` | Creates a dashboard if it does not already exist, or updates an existing dashboard. If you update a dashboard, the entire contents are replaced with what you specify here. |
| `PutInsightRule` | - | - | `RuleDefinition`, `RuleName` | - | `PutInsightRuleOutput` | `InvalidParameterValueException`, `LimitExceededException`, `MissingRequiredParameterException` | Creates a Contributor Insights rule. Rules evaluate log events in a CloudWatch Logs log group, enabling you to find contributor data for the log events in that log group. |
| `PutManagedInsightRules` | - | - | `ManagedRules` | - | `PutManagedInsightRulesOutput` | `InvalidParameterValueException`, `MissingRequiredParameterException` | Creates a managed Contributor Insights rule for a specified Amazon Web Services resource. When you enable a managed rule, you create a Contributor Insights rule that collects data from Amazon Web Services services. |
| `PutMetricAlarm` | - | - | `AlarmName`, `ComparisonOperator`, `EvaluationPeriods` | - | `Unit` | `LimitExceededFault` | Creates or updates an alarm and associates it with the specified metric, metric math expression, anomaly detection model, or Metrics Insights query. For more information about using a Metrics Insights query for an alarm, see Create alarms on Metrics Insights... |
| `PutMetricData` | - | - | `Namespace` | - | `Unit` | `InternalServiceFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MissingRequiredParameterException` | Publishes metric data to Amazon CloudWatch. CloudWatch associates the data with the specified metric. |
| `PutMetricStream` | - | - | `FirehoseArn`, `Name`, `OutputFormat`, `RoleArn` | - | `PutMetricStreamOutput` | `ConcurrentModificationException`, `InternalServiceFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MissingRequiredParameterException` | Creates or updates a metric stream. Metric streams can automatically stream CloudWatch metrics to Amazon Web Services destinations, including Amazon S3, and to many third-party solutions. |
| `SetAlarmState` | - | - | `AlarmName`, `StateReason`, `StateValue` | - | `Unit` | `InvalidFormatFault`, `ResourceNotFound` | Temporarily sets the state of an alarm for testing purposes. When the updated state differs from the previous value, the action configured for the appropriate state is invoked. |
| `StartMetricStreams` | - | - | `Names` | - | `StartMetricStreamsOutput` | `InternalServiceFault`, `InvalidParameterValueException`, `MissingRequiredParameterException` | Starts the streaming of metrics for one or more of your metric streams. |
| `StopMetricStreams` | - | - | `Names` | - | `StopMetricStreamsOutput` | `InternalServiceFault`, `InvalidParameterValueException`, `MissingRequiredParameterException` | Stops the streaming of metrics for one or more of your metric streams. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceOutput` | `ConcurrentModificationException`, `ConflictException`, `InternalServiceFault`, `InvalidParameterValueException`, `ResourceNotFoundException` | Assigns one or more tags (key-value pairs) to the specified CloudWatch resource. Currently, the only CloudWatch resources that can be tagged are alarms and Contributor Insights rules. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceOutput` | `ConcurrentModificationException`, `ConflictException`, `InternalServiceFault`, `InvalidParameterValueException`, `ResourceNotFoundException` | Removes one or more tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidParameterValueException` | `structure` | `message` | The value of an input parameter is bad or out-of-range. |
| `InternalServiceFault` | `structure` | `Message` | Request processing has failed due to some unknown error, exception, or failure. |
| `MissingRequiredParameterException` | `structure` | `message` | An input parameter that is required is missing. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The named resource does not exist. |
| `InvalidNextToken` | `structure` | `message` | The next token specified is invalid. |
| `InvalidParameterCombinationException` | `structure` | `message` | Parameters were used together that cannot be used together. |
| `ConflictException` | `structure` | `Message` | This operation attempted to create a resource that already exists. |
| `LimitExceededException` | `structure` | `Message` | The operation exceeded one or more limits. |
| `LimitExceededFault` | `structure` | `message` | The quota for alarms for this customer has already been reached. |
| `ConcurrentModificationException` | `structure` | `Message` | More than one process tried to modify a resource at the same time. |
| `ResourceNotFound` | `structure` | `message` | The named resource does not exist. |
| `DashboardNotFoundError` | `structure` | `message` | The specified dashboard does not exist. |
| `DeleteAlarmMuteRuleInput` | `structure` | `AlarmMuteRuleName` | - |
| `DeleteAlarmsInput` | `structure` | `AlarmNames` | - |
| `DeleteAnomalyDetectorInput` | `structure` | `Dimensions`, `MetricMathAnomalyDetector`, `MetricName`, `Namespace`, `SingleMetricAnomalyDetector`, `Stat` | - |
| `DeleteAnomalyDetectorOutput` | `structure` | - | - |
| `DeleteDashboardsInput` | `structure` | `DashboardNames` | - |
| `DeleteDashboardsOutput` | `structure` | - | - |
| `DeleteInsightRulesInput` | `structure` | `RuleNames` | - |
| `DeleteInsightRulesOutput` | `structure` | `Failures` | - |
| `DeleteMetricStreamInput` | `structure` | `Name` | - |
| `DeleteMetricStreamOutput` | `structure` | - | - |
| `DescribeAlarmContributorsInput` | `structure` | `AlarmName`, `NextToken` | - |

## Winterbaume LTM Notes

Sources: .agents/docs/LTM/smithy-codegen-and-wire-serialization.md, .agents/docs/LTM/service-implementation-and-validation-synthesis.md, .agents/docs/LTM/terraform-e2e-harness-and-fix-coverage.md.

Mode: full distillation.

- CloudWatch is the reference multi-protocol service. The vendored model declares `awsJson1_0`, `awsQuery`, `awsQueryCompatible`, and `rpcv2Cbor`; Terraform provider traffic has used awsQuery even when Smithy primary-protocol selection and generated response code centred on rpc-v2-cbor.
- Generated rpc-v2-cbor output now handles scalar and list timestamp fields through model-level serde helpers that emit the private CBOR Tag 1 sentinel consumed by `wire::json_to_cbor()`. `GetMetricData` and `ListDashboards` are reference generated-serializer paths.
- The awsQuery compatibility adapter must preserve protocol-specific timestamp semantics. Converting CBOR Tag 1 responses to awsQuery XML must render RFC 3339 timestamps with millisecond precision; plain epoch numbers are not SDK-compatible on the XML path.
- Additional-protocol awsQuery generation emits secondary request/response helpers for CloudWatch-class services, including nested struct deserialisers and non-XML list/timestamp handling.
- CloudWatch remains the main request-deserialiser migration follow-up after the 2026-05 adoption sweep. Its residual body parsing is not a standard restJson1 problem; dispatch chooses between awsQuery and rpc-v2-cbor request paths from URL/body shape, so it needs a CloudWatch-specific multi-protocol migration recipe.
- Keep `awsQueryCompatible` distinct from real awsQuery support. For services such as SQS the trait can mean query-shaped errors only, but CloudWatch has actual awsQuery request traffic to support.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
