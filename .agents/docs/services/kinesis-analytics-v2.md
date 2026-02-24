# Amazon Kinesis Analytics

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Managed Service for Apache Flink was previously known as Amazon Kinesis Data Analytics for Apache Flink. Amazon Managed Service for Apache Flink is a fully managed service that you can use to process and analyze streaming data using Java, Python, SQL, or Scala. The service enables you to quickly author and run Java, SQL, or Scala code against streaming sources to perform time series analytics, feed real-time dashboards, and create real-time metrics.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Kinesis Analytics resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Kinesis Analytics workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Add`, `List`, `Describe`, `Create` operation families, including `DeleteApplication`, `DeleteApplicationCloudWatchLoggingOption`, `DeleteApplicationInputProcessingConfiguration`, `DeleteApplicationOutput`, `AddApplicationCloudWatchLoggingOption`, `AddApplicationInput`.

## Service Identity and Protocol

- AWS model slug: `kinesis-analytics-v2`
- AWS SDK for Rust slug: `kinesisanalyticsv2`
- Model version: `2018-05-23`
- Model file: `vendor/api-models-aws/models/kinesis-analytics-v2/service/2018-05-23/kinesis-analytics-v2-2018-05-23.json`
- SDK ID: `Kinesis Analytics V2`
- Endpoint prefix: `kinesisanalytics`
- ARN namespace: `kinesisanalytics`
- CloudFormation name: `KinesisAnalyticsV2`
- CloudTrail event source: `kinesisanalytics.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (7), `Add` (6), `List` (5), `Describe` (4), `Create` (3), `Update` (2), `Discover` (1), `Rollback` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddApplicationCloudWatchLoggingOption`, `AddApplicationInput`, `AddApplicationInputProcessingConfiguration`, `AddApplicationOutput`, `AddApplicationReferenceDataSource`, `AddApplicationVpcConfiguration`, `CreateApplication`, `CreateApplicationPresignedUrl`, `CreateApplicationSnapshot`, `DeleteApplication`, `DeleteApplicationCloudWatchLoggingOption`, `DeleteApplicationInputProcessingConfiguration`, `DeleteApplicationOutput`, `DeleteApplicationReferenceDataSource`, `DeleteApplicationSnapshot`, `DeleteApplicationVpcConfiguration`, `StartApplication`, `StopApplication`, `TagResource`, `UntagResource`, ... (+2).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApplication`, `DescribeApplicationOperation`, `DescribeApplicationSnapshot`, `DescribeApplicationVersion`, `ListApplicationOperations`, `ListApplicationSnapshots`, `ListApplicationVersions`, `ListApplications`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartApplication`, `StopApplication`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 33 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `Lambda`, `Glue`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/managed-flink/latest/java/how-zeppelin-checkpoint.html
- https://docs.aws.amazon.com/managed-flink/latest/java/troubleshooting-checkpoints.html
- https://docs.aws.amazon.com/managed-flink/latest/apiv2/API_CheckpointConfigurationUpdate.html

Research outcomes:
- Kinesis Analytics v2 corresponds to Amazon Managed Service for Apache Flink for Apache Flink applications.
- Applications run Apache Flink code with managed runtime, snapshots, checkpoints, scaling, and monitoring.
- Checkpoint configuration controls checkpoint interval, minimum pause between checkpoints, and default/custom configuration mode.
- Checkpoints provide fault tolerance and can become bottlenecked by operators, state backends, or storage uploads.
- Flink applications can be stopped, updated, and restarted from snapshots depending on application state and configuration.
- Legacy Kinesis Data Analytics SQL applications have a documented discontinuation path and are distinct from Flink applications.

Parity implications:
- Model applications, application versions, runtime environment, snapshots, checkpoint configuration, parallelism, input/output configuration, and run state separately.
- Update operations should increment application version and respect run-state constraints.
- Checkpoint and snapshot settings should affect restart and recovery semantics rather than being inert metadata.

## Current Network Resource Stub Semantics

Kinesis Analytics V2 currently has a minimal VPC-configuration placeholder.

- `AddApplicationVpcConfiguration` is routed and returns a VPC configuration description, but the state path does not use EC2 to derive VPC ID or validate the supplied subnets and security groups.
- `DeleteApplicationVpcConfiguration` removes or acknowledges the local placeholder by application name and VPC configuration ID.
- Application descriptions expose whatever local VPC configuration description the Kinesis Analytics state has retained.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Delete

- Operations: `DeleteApplication`, `DeleteApplicationCloudWatchLoggingOption`, `DeleteApplicationInputProcessingConfiguration`, `DeleteApplicationOutput`, `DeleteApplicationReferenceDataSource`, `DeleteApplicationSnapshot`, `DeleteApplicationVpcConfiguration`
- Common required input members in this group: `ApplicationName`, `CloudWatchLoggingOptionId`, `CreateTimestamp`, `CurrentApplicationVersionId`, `InputId`, `OutputId`, `ReferenceId`, `SnapshotCreationTimestamp`, `SnapshotName`, `VpcConfigurationId`

### Add

- Operations: `AddApplicationCloudWatchLoggingOption`, `AddApplicationInput`, `AddApplicationInputProcessingConfiguration`, `AddApplicationOutput`, `AddApplicationReferenceDataSource`, `AddApplicationVpcConfiguration`
- Common required input members in this group: `ApplicationName`, `CloudWatchLoggingOption`, `CurrentApplicationVersionId`, `Input`, `InputId`, `InputProcessingConfiguration`, `Output`, `ReferenceDataSource`, `VpcConfiguration`

### List

- Operations: `ListApplicationOperations`, `ListApplicationSnapshots`, `ListApplicationVersions`, `ListApplications`, `ListTagsForResource`
- Traits: `paginated` (4)
- Common required input members in this group: `ApplicationName`, `ResourceARN`

### Describe

- Operations: `DescribeApplication`, `DescribeApplicationOperation`, `DescribeApplicationSnapshot`, `DescribeApplicationVersion`
- Common required input members in this group: `ApplicationName`, `ApplicationVersionId`, `OperationId`, `SnapshotName`

### Create

- Operations: `CreateApplication`, `CreateApplicationPresignedUrl`, `CreateApplicationSnapshot`
- Common required input members in this group: `ApplicationName`, `RuntimeEnvironment`, `ServiceExecutionRole`, `SnapshotName`, `UrlType`

### Update

- Operations: `UpdateApplication`, `UpdateApplicationMaintenanceConfiguration`
- Common required input members in this group: `ApplicationMaintenanceConfigurationUpdate`, `ApplicationName`

### Discover

- Operations: `DiscoverInputSchema`
- Common required input members in this group: `ServiceExecutionRole`

### Rollback

- Operations: `RollbackApplication`
- Common required input members in this group: `ApplicationName`, `CurrentApplicationVersionId`

### Start

- Operations: `StartApplication`
- Common required input members in this group: `ApplicationName`

### Stop

- Operations: `StopApplication`
- Common required input members in this group: `ApplicationName`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddApplicationCloudWatchLoggingOption` | - | - | `ApplicationName`, `CloudWatchLoggingOption` | - | `AddApplicationCloudWatchLoggingOptionResponse` | `ConcurrentModificationException`, `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds an Amazon CloudWatch log stream to monitor application configuration errors. |
| `AddApplicationInput` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `Input` | - | `AddApplicationInputResponse` | `CodeValidationException`, `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds a streaming source to your SQL-based Kinesis Data Analytics application. You can add a streaming source when you create an application, or you can use this operation to add a streaming source after you create an application. |
| `AddApplicationInputProcessingConfiguration` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `InputId`, `InputProcessingConfiguration` | - | `AddApplicationInputProcessingConfigurationResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds an InputProcessingConfiguration to a SQL-based Kinesis Data Analytics application. An input processor pre-processes records on the input stream before the application's SQL code executes. |
| `AddApplicationOutput` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `Output` | - | `AddApplicationOutputResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds an external destination to your SQL-based Kinesis Data Analytics application. If you want Kinesis Data Analytics to deliver data from an in-application stream within your application to an external destination (such as an Kinesis data stream, a Kinesis... |
| `AddApplicationReferenceDataSource` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `ReferenceDataSource` | - | `AddApplicationReferenceDataSourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds a reference data source to an existing SQL-based Kinesis Data Analytics application. Kinesis Data Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. |
| `AddApplicationVpcConfiguration` | - | - | `ApplicationName`, `VpcConfiguration` | - | `AddApplicationVpcConfigurationResponse` | `ConcurrentModificationException`, `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds a Virtual Private Cloud (VPC) configuration to the application. Applications can use VPCs to store and access resources securely. |
| `CreateApplication` | - | - | `ApplicationName`, `RuntimeEnvironment`, `ServiceExecutionRole` | - | `CreateApplicationResponse` | `CodeValidationException`, `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `LimitExceededException`, `ResourceInUseException`, `TooManyTagsException`, `UnsupportedOperationException` | Creates a Managed Service for Apache Flink application. For information about creating a Managed Service for Apache Flink application, see Creating an Application. |
| `CreateApplicationPresignedUrl` | - | - | `ApplicationName`, `UrlType` | - | `CreateApplicationPresignedUrlResponse` | `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException` | Creates and returns a URL that you can use to connect to an application's extension. The IAM role or user used to call this API defines the permissions to access the extension. |
| `CreateApplicationSnapshot` | - | - | `ApplicationName`, `SnapshotName` | - | `CreateApplicationSnapshotResponse` | `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `InvalidRequestException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Creates a snapshot of the application's state data. |
| `DeleteApplication` | - | - | `ApplicationName`, `CreateTimestamp` | - | `DeleteApplicationResponse` | `ConcurrentModificationException`, `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes the specified application. Managed Service for Apache Flink halts application execution and deletes the application. |
| `DeleteApplicationCloudWatchLoggingOption` | - | - | `ApplicationName`, `CloudWatchLoggingOptionId` | - | `DeleteApplicationCloudWatchLoggingOptionResponse` | `ConcurrentModificationException`, `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes an Amazon CloudWatch log stream from an SQL-based Kinesis Data Analytics application. |
| `DeleteApplicationInputProcessingConfiguration` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `InputId` | - | `DeleteApplicationInputProcessingConfigurationResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes an InputProcessingConfiguration from an input. |
| `DeleteApplicationOutput` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `OutputId` | - | `DeleteApplicationOutputResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes the output destination configuration from your SQL-based Kinesis Data Analytics application's configuration. Kinesis Data Analytics will no longer write data from the corresponding in-application stream to the external output destination. |
| `DeleteApplicationReferenceDataSource` | - | - | `ApplicationName`, `CurrentApplicationVersionId`, `ReferenceId` | - | `DeleteApplicationReferenceDataSourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a reference data source configuration from the specified SQL-based Kinesis Data Analytics application's configuration. If the application is running, Kinesis Data Analytics immediately removes the in-application table that you created using the... |
| `DeleteApplicationSnapshot` | - | - | `ApplicationName`, `SnapshotCreationTimestamp`, `SnapshotName` | - | `DeleteApplicationSnapshotResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Deletes a snapshot of application state. |
| `DeleteApplicationVpcConfiguration` | - | - | `ApplicationName`, `VpcConfigurationId` | - | `DeleteApplicationVpcConfigurationResponse` | `ConcurrentModificationException`, `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException` | Removes a VPC configuration from a Managed Service for Apache Flink application. |
| `DescribeApplication` | - | - | `ApplicationName` | - | `DescribeApplicationResponse` | `InvalidArgumentException`, `InvalidRequestException`, `ResourceNotFoundException` | Returns information about a specific Managed Service for Apache Flink application. If you want to retrieve a list of all applications in your account, use the ListApplications operation. |
| `DescribeApplicationOperation` | - | - | `ApplicationName`, `OperationId` | - | `DescribeApplicationOperationResponse` | `InvalidArgumentException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Provides a detailed description of a specified application operation. To see a list of all the operations of an application, invoke the ListApplicationOperations operation. |
| `DescribeApplicationSnapshot` | - | - | `ApplicationName`, `SnapshotName` | - | `DescribeApplicationSnapshotResponse` | `InvalidArgumentException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Returns information about a snapshot of application state data. |
| `DescribeApplicationVersion` | - | - | `ApplicationName`, `ApplicationVersionId` | - | `DescribeApplicationVersionResponse` | `InvalidArgumentException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Provides a detailed description of a specified version of the application. To see a list of all the versions of an application, invoke the ListApplicationVersions operation. |
| `DiscoverInputSchema` | - | - | `ServiceExecutionRole` | - | `DiscoverInputSchemaResponse` | `InvalidArgumentException`, `InvalidRequestException`, `ResourceProvisionedThroughputExceededException`, `ServiceUnavailableException`, `UnableToDetectSchemaException`, `UnsupportedOperationException` | Infers a schema for a SQL-based Kinesis Data Analytics application by evaluating sample records on the specified streaming source (Kinesis data stream or Kinesis Data Firehose delivery stream) or Amazon S3 object. In the response, the operation returns the... |
| `ListApplicationOperations` | - | `paginated` | `ApplicationName` | - | `ListApplicationOperationsResponse` | `InvalidArgumentException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Lists all the operations performed for the specified application such as UpdateApplication, StartApplication etc. The response also includes a summary of the operation. |
| `ListApplicationSnapshots` | - | `paginated` | `ApplicationName` | - | `ListApplicationSnapshotsResponse` | `InvalidArgumentException`, `UnsupportedOperationException` | Lists information about the current application snapshots. |
| `ListApplicationVersions` | - | `paginated` | `ApplicationName` | - | `ListApplicationVersionsResponse` | `InvalidArgumentException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Lists all the versions for the specified application, including versions that were rolled back. The response also includes a summary of the configuration associated with each version. |
| `ListApplications` | - | `paginated` | - | - | `ListApplicationsResponse` | `InvalidRequestException` | Returns a list of Managed Service for Apache Flink applications in your account. For each application, the response includes the application name, Amazon Resource Name (ARN), and status. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceNotFoundException` | Retrieves the list of key-value tags assigned to the application. For more information, see Using Tagging. |
| `RollbackApplication` | - | - | `ApplicationName`, `CurrentApplicationVersionId` | - | `RollbackApplicationResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Reverts the application to the previous running version. You can roll back an application if you suspect it is stuck in a transient status or in the running status. |
| `StartApplication` | - | - | `ApplicationName` | - | `StartApplicationResponse` | `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Starts the specified Managed Service for Apache Flink application. After creating an application, you must exclusively call this operation to start your application. |
| `StopApplication` | - | - | `ApplicationName` | - | `StopApplicationResponse` | `ConcurrentModificationException`, `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException` | Stops the application from processing data. You can stop an application only if it is in the running status, unless you set the `Force` parameter to `true`. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Adds one or more key-value tags to a Managed Service for Apache Flink application. Note that the maximum number of application tags includes system tags. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Removes one or more tags from a Managed Service for Apache Flink application. For more information, see Using Tagging. |
| `UpdateApplication` | - | - | `ApplicationName` | - | `UpdateApplicationResponse` | `CodeValidationException`, `ConcurrentModificationException`, `InvalidApplicationConfigurationException`, `InvalidArgumentException`, `InvalidRequestException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates an existing Managed Service for Apache Flink application. Using this operation, you can update application code, input configuration, and output configuration. |
| `UpdateApplicationMaintenanceConfiguration` | - | - | `ApplicationMaintenanceConfigurationUpdate`, `ApplicationName` | - | `UpdateApplicationMaintenanceConfigurationResponse` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Updates the maintenance configuration of the Managed Service for Apache Flink application. You can invoke this operation on an application that is in one of the two following states: `READY` or `RUNNING`. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidArgumentException` | `structure` | `Message` | The specified input parameter value is not valid. |
| `ResourceNotFoundException` | `structure` | `Message` | Specified application can't be found. |
| `ResourceInUseException` | `structure` | `Message` | The application is not available for this operation. |
| `ConcurrentModificationException` | `structure` | `Message` | Exception thrown as a result of concurrent modifications to an application. |
| `InvalidRequestException` | `structure` | `Message` | The request JSON is not valid for the operation. |
| `UnsupportedOperationException` | `structure` | `Message` | The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. |
| `InvalidApplicationConfigurationException` | `structure` | `Message` | The user-provided application configuration is not valid. |
| `CodeValidationException` | `structure` | `Message` | The user-provided application code (query) is not valid. |
| `LimitExceededException` | `structure` | `Message` | The number of allowed resources has been exceeded. |
| `TooManyTagsException` | `structure` | `message` | Application created with too many tags, or too many tags added to an application. |
| `AddApplicationCloudWatchLoggingOptionRequest` | `structure` | `ApplicationName`, `CloudWatchLoggingOption`, `ConditionalToken`, `CurrentApplicationVersionId` | - |
| `AddApplicationCloudWatchLoggingOptionResponse` | `structure` | `ApplicationARN`, `ApplicationVersionId`, `CloudWatchLoggingOptionDescriptions`, `OperationId` | - |
| `AddApplicationInputRequest` | `structure` | `ApplicationName`, `CurrentApplicationVersionId`, `Input` | - |
| `AddApplicationInputResponse` | `structure` | `ApplicationARN`, `ApplicationVersionId`, `InputDescriptions` | - |
| `AddApplicationInputProcessingConfigurationRequest` | `structure` | `ApplicationName`, `CurrentApplicationVersionId`, `InputId`, `InputProcessingConfiguration` | - |
| `AddApplicationInputProcessingConfigurationResponse` | `structure` | `ApplicationARN`, `ApplicationVersionId`, `InputId`, `InputProcessingConfigurationDescription` | - |
| `AddApplicationOutputRequest` | `structure` | `ApplicationName`, `CurrentApplicationVersionId`, `Output` | - |
| `AddApplicationOutputResponse` | `structure` | `ApplicationARN`, `ApplicationVersionId`, `OutputDescriptions` | - |
| `AddApplicationReferenceDataSourceRequest` | `structure` | `ApplicationName`, `CurrentApplicationVersionId`, `ReferenceDataSource` | - |
| `AddApplicationReferenceDataSourceResponse` | `structure` | `ApplicationARN`, `ApplicationVersionId`, `ReferenceDataSourceDescriptions` | - |
| `AddApplicationVpcConfigurationRequest` | `structure` | `ApplicationName`, `ConditionalToken`, `CurrentApplicationVersionId`, `VpcConfiguration` | - |
| `AddApplicationVpcConfigurationResponse` | `structure` | `ApplicationARN`, `ApplicationVersionId`, `OperationId`, `VpcConfigurationDescription` | - |
| `CreateApplicationRequest` | `structure` | `ApplicationConfiguration`, `ApplicationDescription`, `ApplicationMode`, `ApplicationName`, `CloudWatchLoggingOptions`, `RuntimeEnvironment`, `ServiceExecutionRole`, `Tags` | - |
| `CreateApplicationResponse` | `structure` | `ApplicationDetail` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
