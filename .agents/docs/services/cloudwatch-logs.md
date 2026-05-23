# Amazon CloudWatch Logs

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use Amazon CloudWatch Logs to monitor, store, and access your log files from EC2 instances, CloudTrail, and other sources. You can then retrieve the associated log data from CloudWatch Logs using the CloudWatch console. Alternatively, you can use CloudWatch Logs commands in the Amazon Web Services CLI, CloudWatch Logs API, or CloudWatch Logs SDK. You can use CloudWatch Logs to: Monitor logs from EC2 instances in real time : You can use CloudWatch Logs to monitor applications and systems using log data. For example, CloudWatch Logs can track the number of errors that occur in your application logs.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon CloudWatch Logs where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon CloudWatch Logs by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon CloudWatch Logs resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon CloudWatch Logs workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Describe`, `Put`, `Get`, `List` operation families, including `DeleteAccountPolicy`, `DeleteDataProtectionPolicy`, `DeleteDelivery`, `DeleteDeliveryDestination`, `DescribeAccountPolicies`, `DescribeConfigurationTemplates`.

## Service Identity and Protocol

- AWS model slug: `cloudwatch-logs`
- AWS SDK for Rust slug: `cloudwatchlogs`
- Model version: `2014-03-28`
- Model file: `vendor/api-models-aws/models/cloudwatch-logs/service/2014-03-28/cloudwatch-logs-2014-03-28.json`
- SDK ID: `CloudWatch Logs`
- Endpoint prefix: `logs`
- ARN namespace: `logs`
- CloudFormation name: `Logs`
- CloudTrail event source: `logs.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (19), `Describe` (18), `Put` (18), `Get` (16), `List` (10), `Create` (7), `Update` (4), `Associate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateKmsKey`, `AssociateSourceToS3TableIntegration`, `CancelExportTask`, `CancelImportTask`, `CreateDelivery`, `CreateExportTask`, `CreateImportTask`, `CreateLogAnomalyDetector`, `CreateLogGroup`, `CreateLogStream`, `CreateScheduledQuery`, `DeleteAccountPolicy`, `DeleteDataProtectionPolicy`, `DeleteDelivery`, `DeleteDeliveryDestination`, `DeleteDeliveryDestinationPolicy`, `DeleteDeliverySource`, `DeleteDestination`, `DeleteIndexPolicy`, `DeleteIntegration`, ... (+41).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountPolicies`, `DescribeConfigurationTemplates`, `DescribeDeliveries`, `DescribeDeliveryDestinations`, `DescribeDeliverySources`, `DescribeDestinations`, `DescribeExportTasks`, `DescribeFieldIndexes`, `DescribeImportTaskBatches`, `DescribeImportTasks`, `DescribeIndexPolicies`, `DescribeLogGroups`, `DescribeLogStreams`, `DescribeMetricFilters`, `DescribeQueries`, `DescribeQueryDefinitions`, `DescribeResourcePolicies`, `DescribeSubscriptionFilters`, `GetDataProtectionPolicy`, `GetDelivery`, ... (+24).
- Pagination is modelled for 18 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelExportTask`, `CancelImportTask`, `CreateExportTask`, `CreateImportTask`, `DescribeExportTasks`, `DescribeImportTaskBatches`, `DescribeImportTasks`, `StartLiveTail`, `StartQuery`, `StopQuery`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 108 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`, `ECS`, `RDS`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Working-with-log-groups-and-streams.html
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/SearchDataFilterPattern.html
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/subscription-concepts.html

Research outcomes:
- CloudWatch Logs stores events in log groups and log streams. PutLogEvents uploads batches of log events to a log group/stream.
- Log group retention is indefinite by default. Retention is configurable per log group.
- Events that reach retention age are marked for deletion and no longer count toward stored bytes, but physical deletion usually happens later and can take up to 72 hours or more.
- Log groups support tags; tag keys are unique, case-sensitive, limited to 50 tags per log group, and keys beginning with `aws:` are reserved.
- Access to log streams is controlled at the log group level because streams are hierarchical children of groups.
- Subscription filters connect log groups to delivery destinations such as Kinesis, Lambda, and Firehose and require IAM permissions.

Parity implications:
- Model log group, log stream, event batch, retention policy, tags, and subscription filters separately.
- Retention expiry should affect query/list visibility and stored byte accounting even if physical deletion is simulated lazily.
- PutLogEvents ordering, batch constraints, and sequence-token compatibility need explicit API-level handling.

## Operation Groups

### Delete

- Operations: `DeleteAccountPolicy`, `DeleteDataProtectionPolicy`, `DeleteDelivery`, `DeleteDeliveryDestination`, `DeleteDeliveryDestinationPolicy`, `DeleteDeliverySource`, `DeleteDestination`, `DeleteIndexPolicy`, `DeleteIntegration`, `DeleteLogAnomalyDetector`, `DeleteLogGroup`, `DeleteLogStream`, `DeleteMetricFilter`, `DeleteQueryDefinition`, `DeleteResourcePolicy`, `DeleteRetentionPolicy`, `DeleteScheduledQuery`, `DeleteSubscriptionFilter`, `DeleteTransformer`
- Common required input members in this group: `anomalyDetectorArn`, `deliveryDestinationName`, `destinationName`, `filterName`, `id`, `identifier`, `integrationName`, `logGroupIdentifier`, `logGroupName`, `logStreamName`, `name`, `policyName`, `policyType`, `queryDefinitionId`

### Describe

- Operations: `DescribeAccountPolicies`, `DescribeConfigurationTemplates`, `DescribeDeliveries`, `DescribeDeliveryDestinations`, `DescribeDeliverySources`, `DescribeDestinations`, `DescribeExportTasks`, `DescribeFieldIndexes`, `DescribeImportTaskBatches`, `DescribeImportTasks`, `DescribeIndexPolicies`, `DescribeLogGroups`, `DescribeLogStreams`, `DescribeMetricFilters`, `DescribeQueries`, `DescribeQueryDefinitions`, `DescribeResourcePolicies`, `DescribeSubscriptionFilters`
- Traits: `paginated` (9)
- Common required input members in this group: `importId`, `logGroupIdentifiers`, `logGroupName`, `policyType`

### Put

- Operations: `PutAccountPolicy`, `PutBearerTokenAuthentication`, `PutDataProtectionPolicy`, `PutDeliveryDestination`, `PutDeliveryDestinationPolicy`, `PutDeliverySource`, `PutDestination`, `PutDestinationPolicy`, `PutIndexPolicy`, `PutIntegration`, `PutLogEvents`, `PutLogGroupDeletionProtection`, `PutMetricFilter`, `PutQueryDefinition`, `PutResourcePolicy`, `PutRetentionPolicy`, `PutSubscriptionFilter`, `PutTransformer`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `accessPolicy`, `bearerTokenAuthenticationEnabled`, `deletionProtectionEnabled`, `deliveryDestinationName`, `deliveryDestinationPolicy`, `destinationArn`, `destinationName`, `filterName`, `filterPattern`, `integrationName`, `integrationType`, `logEvents`, `logGroupIdentifier`, `logGroupName`, `logStreamName`, `logType`, `metricTransformations`, `name`, `policyDocument`, `policyName`, `policyType`, `queryString`, `resourceArn`, `resourceConfig`, ... (+4)

### Get

- Operations: `GetDataProtectionPolicy`, `GetDelivery`, `GetDeliveryDestination`, `GetDeliveryDestinationPolicy`, `GetDeliverySource`, `GetIntegration`, `GetLogAnomalyDetector`, `GetLogEvents`, `GetLogFields`, `GetLogGroupFields`, `GetLogObject`, `GetLogRecord`, `GetQueryResults`, `GetScheduledQuery`, `GetScheduledQueryHistory`, `GetTransformer`
- Traits: `endpoint-bound` (1), `paginated` (2)
- Common required input members in this group: `anomalyDetectorArn`, `dataSourceName`, `dataSourceType`, `deliveryDestinationName`, `endTime`, `id`, `identifier`, `integrationName`, `logGroupIdentifier`, `logObjectPointer`, `logRecordPointer`, `logStreamName`, `name`, `queryId`, `startTime`

### List

- Operations: `ListAggregateLogGroupSummaries`, `ListAnomalies`, `ListIntegrations`, `ListLogAnomalyDetectors`, `ListLogGroups`, `ListLogGroupsForQuery`, `ListScheduledQueries`, `ListSourcesForS3TableIntegration`, `ListTagsForResource`, `ListTagsLogGroup`
- Traits: `paginated` (6)
- Common required input members in this group: `groupBy`, `integrationArn`, `logGroupName`, `queryId`, `resourceArn`

### Create

- Operations: `CreateDelivery`, `CreateExportTask`, `CreateImportTask`, `CreateLogAnomalyDetector`, `CreateLogGroup`, `CreateLogStream`, `CreateScheduledQuery`
- Common required input members in this group: `deliveryDestinationArn`, `deliverySourceName`, `destination`, `executionRoleArn`, `from`, `importRoleArn`, `importSourceArn`, `logGroupArnList`, `logGroupName`, `logStreamName`, `name`, `queryLanguage`, `queryString`, `scheduleExpression`, `to`

### Update

- Operations: `UpdateAnomaly`, `UpdateDeliveryConfiguration`, `UpdateLogAnomalyDetector`, `UpdateScheduledQuery`
- Common required input members in this group: `anomalyDetectorArn`, `enabled`, `executionRoleArn`, `id`, `identifier`, `queryLanguage`, `queryString`, `scheduleExpression`

### Associate

- Operations: `AssociateKmsKey`, `AssociateSourceToS3TableIntegration`
- Common required input members in this group: `dataSource`, `integrationArn`, `kmsKeyId`

### Cancel

- Operations: `CancelExportTask`, `CancelImportTask`
- Common required input members in this group: `importId`, `taskId`

### Disassociate

- Operations: `DisassociateKmsKey`, `DisassociateSourceFromS3TableIntegration`
- Common required input members in this group: `identifier`

### Start

- Operations: `StartLiveTail`, `StartQuery`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `endTime`, `logGroupIdentifiers`, `queryString`, `startTime`

### Tag

- Operations: `TagLogGroup`, `TagResource`
- Common required input members in this group: `logGroupName`, `resourceArn`, `tags`

### Test

- Operations: `TestMetricFilter`, `TestTransformer`
- Common required input members in this group: `filterPattern`, `logEventMessages`, `transformerConfig`

### Untag

- Operations: `UntagLogGroup`, `UntagResource`
- Common required input members in this group: `logGroupName`, `resourceArn`, `tagKeys`, `tags`

### Filter

- Operations: `FilterLogEvents`
- Traits: `paginated` (1)

### Stop

- Operations: `StopQuery`
- Common required input members in this group: `queryId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateKmsKey` | - | - | `kmsKeyId` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Associates the specified KMS key with either one log group in the account, or with all stored CloudWatch Logs query insights results in the account. When you use `AssociateKmsKey`, you specify either the `logGroupName` parameter or the `resourceIdentifier`... |
| `AssociateSourceToS3TableIntegration` | - | - | `dataSource`, `integrationArn` | - | `AssociateSourceToS3TableIntegrationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a data source with an S3 Table Integration for query access in the 'logs' namespace. This enables querying log data using analytics engines that support Iceberg such as Amazon Athena, Amazon Redshift, and Apache Spark. |
| `CancelExportTask` | - | - | `taskId` | - | `Unit` | `InvalidOperationException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Cancels the specified export task. The task must be in the `PENDING` or `RUNNING` state. |
| `CancelImportTask` | - | - | `importId` | - | `CancelImportTaskResponse` | `AccessDeniedException`, `InvalidOperationException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException` | Cancels an active import task and stops importing data from the CloudTrail Lake Event Data Store. |
| `CreateDelivery` | - | - | `deliveryDestinationArn`, `deliverySourceName` | - | `CreateDeliveryResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Creates a delivery . A delivery is a connection between a logical delivery source and a logical delivery destination that you have already created. |
| `CreateExportTask` | - | - | `destination`, `from`, `logGroupName`, `to` | - | `CreateExportTaskResponse` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates an export task so that you can efficiently export data from a log group to an Amazon S3 bucket. When you perform a `CreateExportTask` operation, you must use credentials that have permission to write to the S3 bucket that you specify as the... |
| `CreateImportTask` | - | - | `importRoleArn`, `importSourceArn` | - | `CreateImportTaskResponse` | `AccessDeniedException`, `ConflictException`, `InvalidOperationException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts an import from a data source to CloudWatch Log and creates a managed log group as the destination for the imported data. Currently, CloudTrail Event Data Store is the only supported data source. |
| `CreateLogAnomalyDetector` | - | - | `logGroupArnList` | - | `CreateLogAnomalyDetectorResponse` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates an anomaly detector that regularly scans one or more log groups and look for patterns and anomalies in the logs. An anomaly detector can help surface issues by automatically discovering anomalies in your log event traffic. |
| `CreateLogGroup` | - | - | `logGroupName` | - | `Unit` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceAlreadyExistsException`, `ServiceUnavailableException` | Creates a log group with the specified name. You can create up to 1,000,000 log groups per Region per account. |
| `CreateLogStream` | - | - | `logGroupName`, `logStreamName` | - | `Unit` | `InvalidParameterException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates a log stream for the specified log group. A log stream is a sequence of log events that originate from a single source, such as an application instance or a resource that is being monitored. |
| `CreateScheduledQuery` | - | - | `executionRoleArn`, `name`, `queryLanguage`, `queryString`, `scheduleExpression` | - | `CreateScheduledQueryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a scheduled query that runs CloudWatch Logs Insights queries at regular intervals. Scheduled queries enable proactive monitoring by automatically executing queries to detect patterns and anomalies in your log data. |
| `DeleteAccountPolicy` | - | - | `policyName`, `policyType` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes a CloudWatch Logs account policy. This stops the account-wide policy from applying to log groups or data sources in the account. |
| `DeleteDataProtectionPolicy` | - | - | `logGroupIdentifier` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the data protection policy from the specified log group. For more information about data protection policies, see PutDataProtectionPolicy. |
| `DeleteDelivery` | - | - | `id` | - | `Unit` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Deletes a delivery . A delivery is a connection between a logical delivery source and a logical delivery destination . |
| `DeleteDeliveryDestination` | - | - | `name` | - | `Unit` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Deletes a delivery destination . A delivery is a connection between a logical delivery source and a logical delivery destination . |
| `DeleteDeliveryDestinationPolicy` | - | - | `deliveryDestinationName` | - | `Unit` | `ConflictException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ValidationException` | Deletes a delivery destination policy. For more information about these policies, see PutDeliveryDestinationPolicy. |
| `DeleteDeliverySource` | - | - | `name` | - | `Unit` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Deletes a delivery source . A delivery is a connection between a logical delivery source and a logical delivery destination . |
| `DeleteDestination` | - | - | `destinationName` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the specified destination, and eventually disables all the subscription filters that publish to it. This operation does not delete the physical resource encapsulated by the destination. |
| `DeleteIndexPolicy` | - | - | `logGroupIdentifier` | - | `DeleteIndexPolicyResponse` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes a log-group level field index policy that was applied to a single log group. The indexing of the log events that happened before you delete the policy will still be used for as many as 30 days to improve CloudWatch Logs Insights queries. |
| `DeleteIntegration` | - | - | `integrationName` | - | `DeleteIntegrationResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ValidationException` | Deletes the integration between CloudWatch Logs and OpenSearch Service. If your integration has active vended logs dashboards, you must specify `true` for the `force` parameter, otherwise the operation will fail. |
| `DeleteLogAnomalyDetector` | - | - | `anomalyDetectorArn` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the specified CloudWatch Logs anomaly detector. |
| `DeleteLogGroup` | - | - | `logGroupName` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ValidationException` | Deletes the specified log group and permanently deletes all the archived log events associated with the log group. |
| `DeleteLogStream` | - | - | `logGroupName`, `logStreamName` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ValidationException` | Deletes the specified log stream and permanently deletes all the archived log events associated with the log stream. |
| `DeleteMetricFilter` | - | - | `filterName`, `logGroupName` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the specified metric filter. |
| `DeleteQueryDefinition` | - | - | `queryDefinitionId` | - | `DeleteQueryDefinitionResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes a saved CloudWatch Logs Insights query definition. A query definition contains details about a saved CloudWatch Logs Insights query. |
| `DeleteResourcePolicy` | - | - | - | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes a resource policy from this account. This revokes the access of the identities in that policy to put log events to this account. |
| `DeleteRetentionPolicy` | - | - | `logGroupName` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the specified retention policy. Log events do not expire if they belong to log groups without a retention policy. |
| `DeleteScheduledQuery` | - | - | `identifier` | - | `DeleteScheduledQueryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a scheduled query and stops all future executions. This operation also removes any configured actions and associated resources. |
| `DeleteSubscriptionFilter` | - | - | `filterName`, `logGroupName` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the specified subscription filter. |
| `DeleteTransformer` | - | - | `logGroupIdentifier` | - | `Unit` | `InvalidOperationException`, `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the log transformer for the specified log group. As soon as you do this, the transformation of incoming log events according to that transformer stops. |
| `DescribeAccountPolicies` | - | - | `policyType` | - | `DescribeAccountPoliciesResponse` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of all CloudWatch Logs account policies in the account. To use this operation, you must be signed on with the correct permissions depending on the type of policy that you are retrieving information for. |
| `DescribeConfigurationTemplates` | - | `paginated` | - | - | `DescribeConfigurationTemplatesResponse` | `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Use this operation to return the valid and default values that are used when creating delivery sources, delivery destinations, and deliveries. For more information about deliveries, see CreateDelivery. |
| `DescribeDeliveries` | - | `paginated` | - | - | `DescribeDeliveriesResponse` | `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Retrieves a list of the deliveries that have been created in the account. A delivery is a connection between a delivery source and a delivery destination . |
| `DescribeDeliveryDestinations` | - | `paginated` | - | - | `DescribeDeliveryDestinationsResponse` | `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Retrieves a list of the delivery destinations that have been created in the account. |
| `DescribeDeliverySources` | - | `paginated` | - | - | `DescribeDeliverySourcesResponse` | `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Retrieves a list of the delivery sources that have been created in the account. |
| `DescribeDestinations` | - | `paginated` | - | - | `DescribeDestinationsResponse` | `InvalidParameterException`, `ServiceUnavailableException` | Lists all your destinations. The results are ASCII-sorted by destination name. |
| `DescribeExportTasks` | - | - | - | - | `DescribeExportTasksResponse` | `InvalidParameterException`, `ServiceUnavailableException` | Lists the specified export tasks. You can list all your export tasks or filter the results based on task ID or task status. |
| `DescribeFieldIndexes` | - | - | `logGroupIdentifiers` | - | `DescribeFieldIndexesResponse` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of custom and default field indexes which are discovered in log data. For more information about field index policies, see PutIndexPolicy. |
| `DescribeImportTaskBatches` | - | - | `importId` | - | `DescribeImportTaskBatchesResponse` | `AccessDeniedException`, `InvalidOperationException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException` | Gets detailed information about the individual batches within an import task, including their status and any error messages. For CloudTrail Event Data Store sources, a batch refers to a subset of stored events grouped by their eventTime. |
| `DescribeImportTasks` | - | - | - | - | `DescribeImportTasksResponse` | `AccessDeniedException`, `InvalidOperationException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException` | Lists and describes import tasks, with optional filtering by import status and source ARN. |
| `DescribeIndexPolicies` | - | - | `logGroupIdentifiers` | - | `DescribeIndexPoliciesResponse` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the field index policies of the specified log group. For more information about field index policies, see PutIndexPolicy. |
| `DescribeLogGroups` | - | `paginated` | - | - | `DescribeLogGroupsResponse` | `InvalidParameterException`, `ServiceUnavailableException` | Returns information about log groups, including data sources that ingest into each log group. You can return all your log groups or filter the results by prefix. |
| `DescribeLogStreams` | - | `paginated` | - | - | `DescribeLogStreamsResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists the log streams for the specified log group. You can list all the log streams or filter the results by prefix. |
| `DescribeMetricFilters` | - | `paginated` | - | - | `DescribeMetricFiltersResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists the specified metric filters. You can list all of the metric filters or filter the results by log name, prefix, metric name, or metric namespace. |
| `DescribeQueries` | - | - | - | - | `DescribeQueriesResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of CloudWatch Logs Insights queries that are scheduled, running, or have been run recently in this account. You can request all queries or limit it to queries of a specific log group or queries with a certain status. |
| `DescribeQueryDefinitions` | - | - | - | - | `DescribeQueryDefinitionsResponse` | `InvalidParameterException`, `ServiceUnavailableException` | This operation returns a paginated list of your saved CloudWatch Logs Insights query definitions. You can retrieve query definitions from the current account or from a source account that is linked to the current account. |
| `DescribeResourcePolicies` | - | - | - | - | `DescribeResourcePoliciesResponse` | `InvalidParameterException`, `ServiceUnavailableException` | Lists the resource policies in this account. |
| `DescribeSubscriptionFilters` | - | `paginated` | `logGroupName` | - | `DescribeSubscriptionFiltersResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists the subscription filters for the specified log group. You can list all the subscription filters or filter the results by prefix. |
| `DisassociateKmsKey` | - | - | - | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Disassociates the specified KMS key from the specified log group or from all CloudWatch Logs Insights query results in the account. When you use `DisassociateKmsKey`, you specify either the `logGroupName` parameter or the `resourceIdentifier` parameter. |
| `DisassociateSourceFromS3TableIntegration` | - | - | `identifier` | - | `DisassociateSourceFromS3TableIntegrationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a data source from an S3 Table Integration, removing query access and deleting all associated data from the integration. |
| `FilterLogEvents` | - | `paginated` | - | - | `FilterLogEventsResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists log events from the specified log group. You can list all the log events or filter the results using one or more of the following: A filter pattern A time range The log stream name, or a log stream name prefix that matches multiple log streams You must... |
| `GetDataProtectionPolicy` | - | - | `logGroupIdentifier` | - | `GetDataProtectionPolicyResponse` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns information about a log group data protection policy. |
| `GetDelivery` | - | - | `id` | - | `GetDeliveryResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Returns complete information about one logical delivery . A delivery is a connection between a delivery source and a delivery destination . |
| `GetDeliveryDestination` | - | - | `name` | - | `GetDeliveryDestinationResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Retrieves complete information about one delivery destination. |
| `GetDeliveryDestinationPolicy` | - | - | `deliveryDestinationName` | - | `GetDeliveryDestinationPolicyResponse` | `ResourceNotFoundException`, `ServiceUnavailableException`, `ValidationException` | Retrieves the delivery destination policy assigned to the delivery destination that you specify. For more information about delivery destinations and their policies, see PutDeliveryDestinationPolicy. |
| `GetDeliverySource` | - | - | `name` | - | `GetDeliverySourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Retrieves complete information about one delivery source. |
| `GetIntegration` | - | - | `integrationName` | - | `GetIntegrationResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns information about one integration between CloudWatch Logs and OpenSearch Service. |
| `GetLogAnomalyDetector` | - | - | `anomalyDetectorArn` | - | `GetLogAnomalyDetectorResponse` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Retrieves information about the log anomaly detector that you specify. The KMS key ARN detected is valid. |
| `GetLogEvents` | - | `paginated` | `logStreamName` | - | `GetLogEventsResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists log events from the specified log stream. You can list all of the log events or filter using a time range. |
| `GetLogFields` | - | - | `dataSourceName`, `dataSourceType` | - | `GetLogFieldsResponse` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Discovers available fields for a specific data source and type. The response includes any field modifications introduced through pipelines, such as new fields or changed field types. |
| `GetLogGroupFields` | - | - | - | - | `GetLogGroupFieldsResponse` | `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of the fields that are included in log events in the specified log group. Includes the percentage of log events that contain each field. |
| `GetLogObject` | - | `endpoint-bound` | `logObjectPointer` | - | `GetLogObjectResponse` | `AccessDeniedException`, `InvalidOperationException`, `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException` | Retrieves a large logging object (LLO) and streams it back. This API is used to fetch the content of large portions of log events that have been ingested through the PutOpenTelemetryLogs API. |
| `GetLogRecord` | - | - | `logRecordPointer` | - | `GetLogRecordResponse` | `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Retrieves all of the fields and values of a single log event. All fields are retrieved, even if the original query that produced the `logRecordPointer` retrieved only a subset of fields. |
| `GetQueryResults` | - | - | `queryId` | - | `GetQueryResultsResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the results from the specified query. Only the fields requested in the query are returned, along with a `@ptr` field, which is the identifier for the log record. |
| `GetScheduledQuery` | - | - | `identifier` | - | `GetScheduledQueryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about a specific scheduled query, including its configuration, execution status, and metadata. |
| `GetScheduledQueryHistory` | - | `paginated` | `endTime`, `identifier`, `startTime` | - | `GetScheduledQueryHistoryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the execution history of a scheduled query within a specified time range, including query results and destination processing status. |
| `GetTransformer` | - | - | `logGroupIdentifier` | - | `GetTransformerResponse` | `InvalidOperationException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the information about the log transformer associated with this log group. This operation returns data only for transformers created at the log group level. |
| `ListAggregateLogGroupSummaries` | - | `paginated` | `groupBy` | - | `ListAggregateLogGroupSummariesResponse` | `InvalidParameterException`, `ServiceUnavailableException`, `ValidationException` | Returns an aggregate summary of all log groups in the Region grouped by specified data source characteristics. Supports optional filtering by log group class, name patterns, and data sources. |
| `ListAnomalies` | - | `paginated` | - | - | `ListAnomaliesResponse` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of anomalies that log anomaly detectors have found. For details about the structure format of each anomaly object that is returned, see the example in this section. |
| `ListIntegrations` | - | - | - | - | `ListIntegrationsResponse` | `InvalidParameterException`, `ServiceUnavailableException` | Returns a list of integrations between CloudWatch Logs and other services in this account. Currently, only one integration can be created in an account, and this integration must be with OpenSearch Service. |
| `ListLogAnomalyDetectors` | - | `paginated` | - | - | `ListLogAnomalyDetectorsResponse` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Retrieves a list of the log anomaly detectors in the account. |
| `ListLogGroups` | - | - | - | - | `ListLogGroupsResponse` | `InvalidParameterException`, `ServiceUnavailableException` | Returns a list of log groups in the Region in your account. If you are performing this action in a monitoring account, you can choose to also return log groups from source accounts that are linked to the monitoring account. |
| `ListLogGroupsForQuery` | - | `paginated` | `queryId` | - | `ListLogGroupsForQueryResponse` | `AccessDeniedException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of the log groups that were analyzed during a single CloudWatch Logs Insights query. This can be useful for queries that use log group name prefixes or the `filterIndex` command, because the log groups are dynamically selected in these cases. |
| `ListScheduledQueries` | - | `paginated` | - | - | `ListScheduledQueriesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all scheduled queries in your account and region. You can filter results by state to show only enabled or disabled queries. |
| `ListSourcesForS3TableIntegration` | - | `paginated` | `integrationArn` | - | `ListSourcesForS3TableIntegrationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of data source associations for a specified S3 Table Integration, showing which data sources are currently associated for query access. |
| `ListTagsForResource` | - | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Displays the tags associated with a CloudWatch Logs resource. Currently, log groups and destinations support tagging. |
| `ListTagsLogGroup` | - | - | `logGroupName` | - | `ListTagsLogGroupResponse` | `ResourceNotFoundException`, `ServiceUnavailableException` | The ListTagsLogGroup operation is on the path to deprecation. We recommend that you use ListTagsForResource instead. |
| `PutAccountPolicy` | - | - | `policyDocument`, `policyName`, `policyType` | - | `PutAccountPolicyResponse` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ServiceUnavailableException` | Creates an account-level data protection policy, subscription filter policy, field index policy, transformer policy, or metric extraction policy that applies to all log groups, a subset of log groups, or a data source name and type combination in the account... |
| `PutBearerTokenAuthentication` | - | - | `bearerTokenAuthenticationEnabled`, `logGroupIdentifier` | - | `Unit` | `AccessDeniedException`, `InvalidOperationException`, `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Enables or disables bearer token authentication for the specified log group. When enabled on a log group, bearer token authentication is enabled on operations until it is explicitly disabled. |
| `PutDataProtectionPolicy` | - | - | `logGroupIdentifier`, `policyDocument` | - | `PutDataProtectionPolicyResponse` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates a data protection policy for the specified log group. A data protection policy can help safeguard sensitive data that's ingested by the log group by auditing and masking the sensitive log data. |
| `PutDeliveryDestination` | - | - | `name` | - | `PutDeliveryDestinationResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Creates or updates a logical delivery destination . A delivery destination is an Amazon Web Services resource that represents an Amazon Web Services service that logs can be sent to. |
| `PutDeliveryDestinationPolicy` | - | - | `deliveryDestinationName`, `deliveryDestinationPolicy` | - | `PutDeliveryDestinationPolicyResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ValidationException` | Creates and assigns an IAM policy that grants permissions to CloudWatch Logs to deliver logs cross-account to a specified destination in this account. To configure the delivery of logs from an Amazon Web Services service in another account to a logs delivery... |
| `PutDeliverySource` | - | - | `logType`, `name`, `resourceArn` | - | `PutDeliverySourceResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Creates or updates a logical delivery source . A delivery source represents an Amazon Web Services resource that sends logs to an logs delivery destination. |
| `PutDestination` | - | - | `destinationName`, `roleArn`, `targetArn` | - | `PutDestinationResponse` | `InvalidParameterException`, `OperationAbortedException`, `ServiceUnavailableException` | Creates or updates a destination. This operation is used only to create destinations for cross-account subscriptions. |
| `PutDestinationPolicy` | - | - | `accessPolicy`, `destinationName` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ServiceUnavailableException` | Creates or updates an access policy associated with an existing destination. An access policy is an IAM policy document that is used to authorize claims to register a subscription filter against a given destination. |
| `PutIndexPolicy` | - | - | `logGroupIdentifier`, `policyDocument` | - | `PutIndexPolicyResponse` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates or updates a field index policy for the specified log group. Only log groups in the Standard log class support field index policies. |
| `PutIntegration` | - | - | `integrationName`, `integrationType`, `resourceConfig` | - | `PutIntegrationResponse` | `InvalidParameterException`, `LimitExceededException`, `ServiceUnavailableException`, `ValidationException` | Creates an integration between CloudWatch Logs and another service in this account. Currently, only integrations with OpenSearch Service are supported, and currently you can have only one integration in your account. |
| `PutLogEvents` | - | - | `logEvents`, `logGroupName`, `logStreamName` | - | `PutLogEventsResponse` | `DataAlreadyAcceptedException`, `InvalidParameterException`, `InvalidSequenceTokenException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `UnrecognizedClientException` | Uploads a batch of log events to the specified log stream. The sequence token is now ignored in `PutLogEvents` actions. |
| `PutLogGroupDeletionProtection` | - | - | `deletionProtectionEnabled`, `logGroupIdentifier` | - | `Unit` | `AccessDeniedException`, `InvalidOperationException`, `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Enables or disables deletion protection for the specified log group. When enabled on a log group, deletion protection blocks all deletion operations until it is explicitly disabled. |
| `PutMetricFilter` | - | - | `filterName`, `filterPattern`, `logGroupName`, `metricTransformations` | - | `Unit` | `InvalidOperationException`, `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates or updates a metric filter and associates it with the specified log group. With metric filters, you can configure rules to extract metric data from log events ingested through PutLogEvents. |
| `PutQueryDefinition` | - | `idempotency-token` | `name`, `queryString` | `clientToken` | `PutQueryDefinitionResponse` | `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates or updates a query definition for CloudWatch Logs Insights. For more information, see Analyzing Log Data with CloudWatch Logs Insights. |
| `PutResourcePolicy` | - | - | - | - | `PutResourcePolicyResponse` | `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates or updates a resource policy allowing other Amazon Web Services services to put log events to this account, such as Amazon Route 53. This API has the following restrictions: Supported actions - Policy only supports `logs:PutLogEvents` and... |
| `PutRetentionPolicy` | - | - | `logGroupName`, `retentionInDays` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Sets the retention of the specified log group. With a retention policy, you can configure the number of days for which to retain log events in the specified log group. |
| `PutSubscriptionFilter` | - | - | `destinationArn`, `filterName`, `filterPattern`, `logGroupName` | - | `Unit` | `InvalidOperationException`, `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates or updates a subscription filter and associates it with the specified log group. With subscription filters, you can subscribe to a real-time stream of log events ingested through PutLogEvents and have them delivered to a specific destination. |
| `PutTransformer` | - | - | `logGroupIdentifier`, `transformerConfig` | - | `Unit` | `InvalidOperationException`, `InvalidParameterException`, `LimitExceededException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates or updates a log transformer for a single log group. You use log transformers to transform log events into a different format, making them easier for you to process and analyze. |
| `StartLiveTail` | - | `endpoint-bound` | `logGroupIdentifiers` | - | `StartLiveTailResponse` | `AccessDeniedException`, `InvalidOperationException`, `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException` | Starts a Live Tail streaming session for one or more log groups. A Live Tail session returns a stream of log events that have been recently ingested in the log groups. |
| `StartQuery` | - | - | `endTime`, `queryString`, `startTime` | - | `StartQueryResponse` | `InvalidParameterException`, `LimitExceededException`, `MalformedQueryException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Starts a query of one or more log groups or data sources using CloudWatch Logs Insights. You specify the log groups or data sources and time range to query and the query string to use. |
| `StopQuery` | - | - | `queryId` | - | `StopQueryResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Stops a CloudWatch Logs Insights query that is in progress. If the query has already ended, the operation returns an error indicating that the specified query is not running. |
| `TagLogGroup` | - | - | `logGroupName`, `tags` | - | `Unit` | `InvalidParameterException`, `ResourceNotFoundException` | The TagLogGroup operation is on the path to deprecation. We recommend that you use TagResource instead. |
| `TagResource` | - | - | `resourceArn`, `tags` | - | `Unit` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `TooManyTagsException` | Assigns one or more tags (key-value pairs) to the specified CloudWatch Logs resource. Currently, the only CloudWatch Logs resources that can be tagged are log groups and destinations. |
| `TestMetricFilter` | - | - | `filterPattern`, `logEventMessages` | - | `TestMetricFilterResponse` | `InvalidParameterException`, `ServiceUnavailableException` | Tests the filter pattern of a metric filter against a sample of log event messages. You can use this operation to validate the correctness of a metric filter pattern. |
| `TestTransformer` | - | - | `logEventMessages`, `transformerConfig` | - | `TestTransformerResponse` | `InvalidOperationException`, `InvalidParameterException`, `ServiceUnavailableException` | Use this operation to test a log transformer. You enter the transformer configuration and a set of log events to test with. |
| `UntagLogGroup` | - | - | `logGroupName`, `tags` | - | `Unit` | `ResourceNotFoundException` | The UntagLogGroup operation is on the path to deprecation. We recommend that you use UntagResource instead. |
| `UntagResource` | - | - | `resourceArn`, `tagKeys` | - | `Unit` | `InvalidParameterException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Removes one or more tags from the specified resource. |
| `UpdateAnomaly` | - | - | `anomalyDetectorArn` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Use this operation to suppress anomaly detection for a specified anomaly or pattern. If you suppress an anomaly, CloudWatch Logs won't report new occurrences of that anomaly and won't update that anomaly with new data. |
| `UpdateDeliveryConfiguration` | - | - | `id` | - | `UpdateDeliveryConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Use this operation to update the configuration of a delivery to change either the S3 path pattern or the format of the delivered logs. You can't use this operation to change the source or destination of the delivery. |
| `UpdateLogAnomalyDetector` | - | - | `anomalyDetectorArn`, `enabled` | - | `Unit` | `InvalidParameterException`, `OperationAbortedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Updates an existing log anomaly detector. |
| `UpdateScheduledQuery` | - | - | `executionRoleArn`, `identifier`, `queryLanguage`, `queryString`, `scheduleExpression` | - | `UpdateScheduledQueryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing scheduled query with new configuration. This operation uses PUT semantics, allowing modification of query parameters, schedule, and destinations. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ServiceUnavailableException` | `structure` | `message` | The service cannot complete the request. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource does not exist. |
| `InvalidParameterException` | `structure` | `message` | A parameter is specified incorrectly. |
| `OperationAbortedException` | `structure` | `message` | Multiple concurrent requests to update the same resource were in conflict. |
| `ValidationException` | `structure` | `message` | One of the parameters for the request is not valid. |
| `ThrottlingException` | `structure` | `message` | The request was throttled because of quota limits. |
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient permissions to perform this action. |
| `LimitExceededException` | `structure` | `message` | You have reached the maximum number of resources that can be created. |
| `InvalidOperationException` | `structure` | `message` | The operation is not valid on the specified resource. |
| `ServiceQuotaExceededException` | `structure` | `message` | This request exceeds a service quota. |
| `ConflictException` | `structure` | `message` | This operation attempted to create a resource that already exists. |
| `InternalServerException` | `structure` | `message` | An internal server error occurred while processing the request. |
| `ResourceAlreadyExistsException` | `structure` | `message` | The specified resource already exists. |
| `AssociateKmsKeyRequest` | `structure` | `kmsKeyId`, `logGroupName`, `resourceIdentifier` | - |
| `AssociateSourceToS3TableIntegrationRequest` | `structure` | `dataSource`, `integrationArn` | - |
| `AssociateSourceToS3TableIntegrationResponse` | `structure` | `identifier` | - |
| `CancelExportTaskRequest` | `structure` | `taskId` | - |
| `CancelImportTaskRequest` | `structure` | `importId` | - |
| `CancelImportTaskResponse` | `structure` | `creationTime`, `importId`, `importStatistics`, `importStatus`, `lastUpdatedTime` | - |
| `CreateDeliveryRequest` | `structure` | `deliveryDestinationArn`, `deliverySourceName`, `fieldDelimiter`, `recordFields`, `s3DeliveryConfiguration`, `tags` | - |
| `CreateDeliveryResponse` | `structure` | `delivery` | - |
| `CreateExportTaskRequest` | `structure` | `destination`, `destinationPrefix`, `from`, `logGroupName`, `logStreamNamePrefix`, `taskName`, `to` | - |
| `CreateExportTaskResponse` | `structure` | `taskId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
