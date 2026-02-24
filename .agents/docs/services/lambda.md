# AWS Lambda

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Lambda Overview Lambda is a compute service that lets you run code without provisioning or managing servers. Lambda runs your code on a high-availability compute infrastructure and performs all of the administration of the compute resources, including server and operating system maintenance, capacity provisioning and automatic scaling, code monitoring and logging. With Lambda, you can run code for virtually any type of application or backend service. For more information about the Lambda service, see What is Lambda in the Lambda Developer Guide . The Lambda API Reference provides information about each of the API methods, including details about the parameters in each API request and response.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Lambda where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Lambda by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS Lambda resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Lambda workflows in the local mock. Key resources include `CapacityProviderResource`, `CodeSigningConfigResource`, `EventSourceMapping`, `Function`, `FunctionAlias`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Delete`, `Update`, `Put` operation families, including `GetAccountSettings`, `GetAlias`, `GetCapacityProvider`, `GetCodeSigningConfig`, `ListAliases`, `ListCapacityProviders`.

## Service Identity and Protocol

- AWS model slug: `lambda`
- AWS SDK for Rust slug: `lambda`
- Model version: `2015-03-31`
- Model file: `vendor/api-models-aws/models/lambda/service/2015-03-31/lambda-2015-03-31.json`
- SDK ID: `Lambda`
- Endpoint prefix: `lambda`
- ARN namespace: `lambda`
- CloudFormation name: `Lambda`
- CloudTrail event source: `lambda.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (22), `List` (15), `Delete` (11), `Update` (8), `Put` (7), `Create` (6), `Invoke` (3), `Send` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddLayerVersionPermission`, `AddPermission`, `CreateAlias`, `CreateCapacityProvider`, `CreateCodeSigningConfig`, `CreateEventSourceMapping`, `CreateFunction`, `CreateFunctionUrlConfig`, `DeleteAlias`, `DeleteCapacityProvider`, `DeleteCodeSigningConfig`, `DeleteEventSourceMapping`, `DeleteFunction`, `DeleteFunctionCodeSigningConfig`, `DeleteFunctionConcurrency`, `DeleteFunctionEventInvokeConfig`, `DeleteFunctionUrlConfig`, `DeleteLayerVersion`, `DeleteProvisionedConcurrencyConfig`, `PutFunctionCodeSigningConfig`, ... (+19).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountSettings`, `GetAlias`, `GetCapacityProvider`, `GetCodeSigningConfig`, `GetDurableExecution`, `GetDurableExecutionHistory`, `GetDurableExecutionState`, `GetEventSourceMapping`, `GetFunction`, `GetFunctionCodeSigningConfig`, `GetFunctionConcurrency`, `GetFunctionConfiguration`, `GetFunctionEventInvokeConfig`, `GetFunctionRecursionConfig`, `GetFunctionScalingConfig`, `GetFunctionUrlConfig`, `GetLayerVersion`, `GetLayerVersionByArn`, `GetLayerVersionPolicy`, `GetPolicy`, ... (+17).
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 12 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CheckpointDurableExecution`, `GetDurableExecution`, `GetDurableExecutionHistory`, `GetDurableExecutionState`, `ListDurableExecutionsByFunction`, `SendDurableExecutionCallbackFailure`, `SendDurableExecutionCallbackHeartbeat`, `SendDurableExecutionCallbackSuccess`, `StopDurableExecution`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 85 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `SNS`, `SQS`, `Lambda`, `Glue`, `EC2/VPC`, `ECR`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CapacityProviderResource` | `CapacityProviderName` | put: `CreateCapacityProvider`; read: `GetCapacityProvider`; update: `UpdateCapacityProvider`; delete: `DeleteCapacityProvider`; list: `ListCapacityProviders` | `ListFunctionVersionsByCapacityProvider` | Represents an AWS Lambda capacity provider |
| `CodeSigningConfigResource` | `CodeSigningConfigId` | create: `CreateCodeSigningConfig`; list: `ListCodeSigningConfigs` | `DeleteCodeSigningConfig`, `GetCodeSigningConfig`, `ListFunctionsByCodeSigningConfig`, `UpdateCodeSigningConfig` | Represents an AWS Lambda code signing config |
| `EventSourceMapping` | `UUID` | create: `CreateEventSourceMapping`; read: `GetEventSourceMapping`; update: `UpdateEventSourceMapping`; delete: `DeleteEventSourceMapping`; list: `ListEventSourceMappings` | - | Represents an AWS Lambda event source mapping |
| `Function` | `FunctionName` | put: `CreateFunction`; list: `ListFunctions` | `DeleteFunctionCodeSigningConfig`, `GetFunction`, `GetFunctionCodeSigningConfig`, `GetFunctionConfiguration`, `GetFunctionRecursionConfig`, `GetFunctionScalingConfig`, `GetPolicy`, `GetRuntimeManagementConfig`, `Invoke`, `InvokeAsync`, ... (+16) | Represents an AWS Lambda function |
| `FunctionAlias` | `Alias`, `FunctionName` | put: `CreateAlias`; read: `GetAlias`; update: `UpdateAlias`; delete: `DeleteAlias`; list: `ListAliases` | - | A lambda function version or alias |
| `FunctionVersionResource` | `FunctionName`, `Version` | create: `PublishVersion` | `ListVersionsByFunction` | A lambda function version or alias |
| `LayerResource` | `LayerName` | list: `ListLayers` | - | Represents an AWS Lambda function layer |
| `LayerVersion` | `LayerName`, `LayerVersion` | list: `ListLayerVersions` | `AddLayerVersionPermission`, `DeleteLayerVersion`, `GetLayerVersion`, `GetLayerVersionByArn`, `GetLayerVersionPolicy`, `PublishLayerVersion`, `RemoveLayerVersionPermission` | Represents a version of an AWS Lambda function layer |
| `Permission` | `FunctionName`, `StatementId` | - | `AddPermission`, `RemovePermission` | - |
| `ProvisionedConcurrencyConfig` | `FunctionName`, `Qualifier` | put: `PutProvisionedConcurrencyConfig`; read: `GetProvisionedConcurrencyConfig`; delete: `DeleteProvisionedConcurrencyConfig` | - | - |
| `ResourcePolicy` | `TargetArn` | - | - | - |
## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html
- https://docs.aws.amazon.com/lambda/latest/api/API_UpdateEventSourceMapping.html
- https://docs.aws.amazon.com/lambda/latest/dg/using-aliases.html

Research outcomes:
- Event source mappings connect an event source to a Lambda function; Lambda reads from the source and invokes the function.
- Supported event source mapping families include DynamoDB Streams, Kinesis, SQS, Amazon MQ/RabbitMQ, MSK, self-managed Kafka, and DocumentDB.
- Stream sources expose failure controls such as BisectBatchOnFunctionError, MaximumRecordAgeInSeconds, MaximumRetryAttempts, and OnFailure destinations.
- DynamoDB and Kinesis mappings support ParallelizationFactor for concurrent processing per shard.
- UpdateEventSourceMapping can pause and resume invocation while preserving location.
- Lambda aliases can be used by event sources and resource policies to scope invocations to a version-like stable ARN.

Parity implications:
- Model function configuration/version/alias separately from event-source mapping state.
- Event source mappings should have source-family-specific validation and failure behaviour.
- Stream processing state needs checkpoints, retry counters, batch splitting, discarded-record destinations, and pause/resume state.

## Cross-Service Integration Gaps

- **`lambda-event-sources`** ( primary ): `create_event_source_mapping()` stores mappings but does not poll DynamoDB Streams, SQS, SNS, or Kinesis sources or invoke the Lambda function. Tracked in `.agents/docs/TODO.md` ( "Cross-Service Integration Gaps" → `lambda-event-sources` ).
- **Secondary target of** `eventbridge-targets`, `eventbridge-pipes`, `sfn-execution`, `appsync-resolvers`, `apigateway-lambda`, `dynamodbstreams-lambda` — every one of those gaps ends in "and invoke the Lambda function". The right reusable seam is a single `invoke_function(arn, payload)` entry point on this crate that the source crates can call once their dispatchers exist; without it each source crate would re-implement the Lambda invocation contract.

## Current Network Resource Stub Semantics

Lambda currently reserves a VPC configuration slot on function state but does not provision Lambda networking.

- Function view/state has an optional `vpc_config` JSON field, and current function creation initialises it to `None` in the implemented state path.
- Update paths do not create ENIs, allocate subnet attachments, or enforce security group membership.
- Function invocation state is independent of VPC reachability.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetAccountSettings`, `GetAlias`, `GetCapacityProvider`, `GetCodeSigningConfig`, `GetDurableExecution`, `GetDurableExecutionHistory`, `GetDurableExecutionState`, `GetEventSourceMapping`, `GetFunction`, `GetFunctionCodeSigningConfig`, `GetFunctionConcurrency`, `GetFunctionConfiguration`, `GetFunctionEventInvokeConfig`, `GetFunctionRecursionConfig`, `GetFunctionScalingConfig`, `GetFunctionUrlConfig`, `GetLayerVersion`, `GetLayerVersionByArn`, `GetLayerVersionPolicy`, `GetPolicy`, `GetProvisionedConcurrencyConfig`, `GetRuntimeManagementConfig`
- Traits: `paginated` (2), `readonly` (22)
- Common required input members in this group: `Arn`, `CapacityProviderName`, `CheckpointToken`, `CodeSigningConfigArn`, `DurableExecutionArn`, `FunctionName`, `LayerName`, `Name`, `Qualifier`, `UUID`, `VersionNumber`

### List

- Operations: `ListAliases`, `ListCapacityProviders`, `ListCodeSigningConfigs`, `ListDurableExecutionsByFunction`, `ListEventSourceMappings`, `ListFunctionEventInvokeConfigs`, `ListFunctionUrlConfigs`, `ListFunctionVersionsByCapacityProvider`, `ListFunctions`, `ListFunctionsByCodeSigningConfig`, `ListLayerVersions`, `ListLayers`, `ListProvisionedConcurrencyConfigs`, `ListTags`, `ListVersionsByFunction`
- Traits: `paginated` (14), `readonly` (15)
- Common required input members in this group: `CapacityProviderName`, `CodeSigningConfigArn`, `FunctionName`, `LayerName`, `Resource`

### Delete

- Operations: `DeleteAlias`, `DeleteCapacityProvider`, `DeleteCodeSigningConfig`, `DeleteEventSourceMapping`, `DeleteFunction`, `DeleteFunctionCodeSigningConfig`, `DeleteFunctionConcurrency`, `DeleteFunctionEventInvokeConfig`, `DeleteFunctionUrlConfig`, `DeleteLayerVersion`, `DeleteProvisionedConcurrencyConfig`
- Traits: `idempotent` (7)
- Common required input members in this group: `CapacityProviderName`, `CodeSigningConfigArn`, `FunctionName`, `LayerName`, `Name`, `Qualifier`, `UUID`, `VersionNumber`

### Update

- Operations: `UpdateAlias`, `UpdateCapacityProvider`, `UpdateCodeSigningConfig`, `UpdateEventSourceMapping`, `UpdateFunctionCode`, `UpdateFunctionConfiguration`, `UpdateFunctionEventInvokeConfig`, `UpdateFunctionUrlConfig`
- Common required input members in this group: `CapacityProviderName`, `CodeSigningConfigArn`, `FunctionName`, `Name`, `UUID`

### Put

- Operations: `PutFunctionCodeSigningConfig`, `PutFunctionConcurrency`, `PutFunctionEventInvokeConfig`, `PutFunctionRecursionConfig`, `PutFunctionScalingConfig`, `PutProvisionedConcurrencyConfig`, `PutRuntimeManagementConfig`
- Traits: `idempotent` (1)
- Common required input members in this group: `CodeSigningConfigArn`, `FunctionName`, `ProvisionedConcurrentExecutions`, `Qualifier`, `RecursiveLoop`, `ReservedConcurrentExecutions`, `UpdateRuntimeOn`

### Create

- Operations: `CreateAlias`, `CreateCapacityProvider`, `CreateCodeSigningConfig`, `CreateEventSourceMapping`, `CreateFunction`, `CreateFunctionUrlConfig`
- Traits: `idempotent` (3)
- Common required input members in this group: `AllowedPublishers`, `AuthType`, `CapacityProviderName`, `Code`, `FunctionName`, `FunctionVersion`, `Name`, `PermissionsConfig`, `Role`, `VpcConfig`

### Invoke

- Operations: `Invoke`, `InvokeAsync`, `InvokeWithResponseStream`
- Common required input members in this group: `FunctionName`, `InvokeArgs`

### Send

- Operations: `SendDurableExecutionCallbackFailure`, `SendDurableExecutionCallbackHeartbeat`, `SendDurableExecutionCallbackSuccess`
- Common required input members in this group: `CallbackId`

### Add

- Operations: `AddLayerVersionPermission`, `AddPermission`
- Common required input members in this group: `Action`, `FunctionName`, `LayerName`, `Principal`, `StatementId`, `VersionNumber`

### Publish

- Operations: `PublishLayerVersion`, `PublishVersion`
- Common required input members in this group: `Content`, `FunctionName`, `LayerName`

### Remove

- Operations: `RemoveLayerVersionPermission`, `RemovePermission`
- Common required input members in this group: `FunctionName`, `LayerName`, `StatementId`, `VersionNumber`

### Checkpoint

- Operations: `CheckpointDurableExecution`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `CheckpointToken`, `DurableExecutionArn`

### Stop

- Operations: `StopDurableExecution`
- Common required input members in this group: `DurableExecutionArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `Resource`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `Resource`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddLayerVersionPermission` | `POST /2018-10-31/layers/{LayerName}/versions/{VersionNumber}/policy` | - | `Action`, `LayerName`, `Principal`, `StatementId`, `VersionNumber` | - | `AddLayerVersionPermissionResponse` | `InvalidParameterValueException`, `PolicyLengthExceededException`, `PreconditionFailedException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Adds permissions to the resource-based policy of a version of an Lambda layer. Use this action to grant layer usage permission to other accounts. |
| `AddPermission` | `POST /2015-03-31/functions/{FunctionName}/policy` | - | `Action`, `FunctionName`, `Principal`, `StatementId` | - | `AddPermissionResponse` | `InvalidParameterValueException`, `PolicyLengthExceededException`, `PreconditionFailedException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Grants a principal permission to use a function. You can apply the policy at the function level, or specify a qualifier to restrict access to a single version or alias. |
| `CheckpointDurableExecution` | `POST /2025-12-01/durable-executions/{DurableExecutionArn}/checkpoint` | `idempotent`, `idempotency-token` | `CheckpointToken`, `DurableExecutionArn` | `ClientToken` | `CheckpointDurableExecutionResponse` | `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Saves the progress of a durable function execution during runtime. This API is used by the Lambda durable functions SDK to checkpoint completed steps and schedule asynchronous operations. |
| `CreateAlias` | `POST /2015-03-31/functions/{FunctionName}/aliases` | `idempotent` | `FunctionName`, `FunctionVersion`, `Name` | - | `AliasConfiguration` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Creates an alias for a Lambda function version. Use aliases to provide clients with a function identifier that you can update to invoke a different version. |
| `CreateCapacityProvider` | `POST /2025-11-30/capacity-providers` | `idempotent` | `CapacityProviderName`, `PermissionsConfig`, `VpcConfig` | - | `CreateCapacityProviderResponse` | `CapacityProviderLimitExceededException`, `InvalidParameterValueException`, `ResourceConflictException`, `ServiceException`, `TooManyRequestsException` | Creates a capacity provider that manages compute resources for Lambda functions |
| `CreateCodeSigningConfig` | `POST /2020-04-22/code-signing-configs` | - | `AllowedPublishers` | - | `CreateCodeSigningConfigResponse` | `InvalidParameterValueException`, `ServiceException` | Creates a code signing configuration. A code signing configuration defines a list of allowed signing profiles and defines the code-signing validation policy (action to be taken if deployment validation checks fail). |
| `CreateEventSourceMapping` | `POST /2015-03-31/event-source-mappings` | - | `FunctionName` | - | `EventSourceMappingConfiguration` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Creates a mapping between an event source and an Lambda function. Lambda reads items from the event source and invokes the function. |
| `CreateFunction` | `POST /2015-03-31/functions` | `idempotent` | `Code`, `FunctionName`, `Role` | - | `FunctionConfiguration` | `CodeSigningConfigNotFoundException`, `CodeStorageExceededException`, `CodeVerificationFailedException`, `FunctionVersionsPerCapacityProviderLimitExceededException`, `InvalidCodeSignatureException`, `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, ... (+2) | Creates a Lambda function. To create a function, you need a deployment package and an execution role. |
| `CreateFunctionUrlConfig` | `POST /2021-10-31/functions/{FunctionName}/url` | - | `AuthType`, `FunctionName` | - | `CreateFunctionUrlConfigResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Creates a Lambda function URL with the specified configuration parameters. A function URL is a dedicated HTTP(S) endpoint that you can use to invoke your function. |
| `DeleteAlias` | `DELETE /2015-03-31/functions/{FunctionName}/aliases/{Name}` | `idempotent` | `FunctionName`, `Name` | - | `Unit` | `InvalidParameterValueException`, `ResourceConflictException`, `ServiceException`, `TooManyRequestsException` | Deletes a Lambda function alias. |
| `DeleteCapacityProvider` | `DELETE /2025-11-30/capacity-providers/{CapacityProviderName}` | `idempotent` | `CapacityProviderName` | - | `DeleteCapacityProviderResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes a capacity provider. You cannot delete a capacity provider that is currently being used by Lambda functions. |
| `DeleteCodeSigningConfig` | `DELETE /2020-04-22/code-signing-configs/{CodeSigningConfigArn}` | `idempotent` | `CodeSigningConfigArn` | - | `DeleteCodeSigningConfigResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException` | Deletes the code signing configuration. You can delete the code signing configuration only if no function is using it. |
| `DeleteEventSourceMapping` | `DELETE /2015-03-31/event-source-mappings/{UUID}` | `idempotent` | `UUID` | - | `EventSourceMappingConfiguration` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes an event source mapping. You can get the identifier of a mapping from the output of ListEventSourceMappings. |
| `DeleteFunction` | `DELETE /2015-03-31/functions/{FunctionName}` | `idempotent` | `FunctionName` | - | `DeleteFunctionResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes a Lambda function. To delete a specific function version, use the `Qualifier` parameter. |
| `DeleteFunctionCodeSigningConfig` | `DELETE /2020-06-30/functions/{FunctionName}/code-signing-config` | - | `FunctionName` | - | `Unit` | `CodeSigningConfigNotFoundException`, `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Removes the code signing configuration from the function. |
| `DeleteFunctionConcurrency` | `DELETE /2017-10-31/functions/{FunctionName}/concurrency` | - | `FunctionName` | - | `Unit` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Removes a concurrent execution limit from a function. |
| `DeleteFunctionEventInvokeConfig` | `DELETE /2019-09-25/functions/{FunctionName}/event-invoke-config` | - | `FunctionName` | - | `Unit` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes the configuration for asynchronous invocation for a function, version, or alias. To configure options for asynchronous invocation, use PutFunctionEventInvokeConfig. |
| `DeleteFunctionUrlConfig` | `DELETE /2021-10-31/functions/{FunctionName}/url` | - | `FunctionName` | - | `Unit` | `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes a Lambda function URL. When you delete a function URL, you can't recover it. |
| `DeleteLayerVersion` | `DELETE /2018-10-31/layers/{LayerName}/versions/{VersionNumber}` | `idempotent` | `LayerName`, `VersionNumber` | - | `Unit` | `ServiceException`, `TooManyRequestsException` | Deletes a version of an Lambda layer. Deleted versions can no longer be viewed or added to functions. |
| `DeleteProvisionedConcurrencyConfig` | `DELETE /2019-09-30/functions/{FunctionName}/provisioned-concurrency` | `idempotent` | `FunctionName`, `Qualifier` | - | `Unit` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes the provisioned concurrency configuration for a function. |
| `GetAccountSettings` | `GET /2016-08-19/account-settings` | `readonly` | - | - | `GetAccountSettingsResponse` | `ServiceException`, `TooManyRequestsException` | Retrieves details about your account's limits and usage in an Amazon Web Services Region. |
| `GetAlias` | `GET /2015-03-31/functions/{FunctionName}/aliases/{Name}` | `readonly` | `FunctionName`, `Name` | - | `AliasConfiguration` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns details about a Lambda function alias. |
| `GetCapacityProvider` | `GET /2025-11-30/capacity-providers/{CapacityProviderName}` | `readonly` | `CapacityProviderName` | - | `GetCapacityProviderResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves information about a specific capacity provider, including its configuration, state, and associated resources. |
| `GetCodeSigningConfig` | `GET /2020-04-22/code-signing-configs/{CodeSigningConfigArn}` | `readonly` | `CodeSigningConfigArn` | - | `GetCodeSigningConfigResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException` | Returns information about the specified code signing configuration. |
| `GetDurableExecution` | `GET /2025-12-01/durable-executions/{DurableExecutionArn}` | `readonly` | `DurableExecutionArn` | - | `GetDurableExecutionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves detailed information about a specific durable execution, including its current status, input payload, result or error information, and execution metadata such as start time and usage statistics. |
| `GetDurableExecutionHistory` | `GET /2025-12-01/durable-executions/{DurableExecutionArn}/history` | `readonly`, `paginated` | `DurableExecutionArn` | - | `GetDurableExecutionHistoryResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves the execution history for a durable execution, showing all the steps, callbacks, and events that occurred during the execution. This provides a detailed audit trail of the execution's progress over time. |
| `GetDurableExecutionState` | `GET /2025-12-01/durable-executions/{DurableExecutionArn}/state` | `readonly`, `paginated` | `CheckpointToken`, `DurableExecutionArn` | - | `GetDurableExecutionStateResponse` | `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Retrieves the current execution state required for the replay process during durable function execution. This API is used by the Lambda durable functions SDK to get state information needed for replay. |
| `GetEventSourceMapping` | `GET /2015-03-31/event-source-mappings/{UUID}` | `readonly` | `UUID` | - | `EventSourceMappingConfiguration` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns details about an event source mapping. You can get the identifier of a mapping from the output of ListEventSourceMappings. |
| `GetFunction` | `GET /2015-03-31/functions/{FunctionName}` | `readonly` | `FunctionName` | - | `GetFunctionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns information about the function or function version, with a link to download the deployment package that's valid for 10 minutes. If you specify a function version, only details that are specific to that version are returned. |
| `GetFunctionCodeSigningConfig` | `GET /2020-06-30/functions/{FunctionName}/code-signing-config` | `readonly` | `FunctionName` | - | `GetFunctionCodeSigningConfigResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns the code signing configuration for the specified function. |
| `GetFunctionConcurrency` | `GET /2019-09-30/functions/{FunctionName}/concurrency` | `readonly` | `FunctionName` | - | `GetFunctionConcurrencyResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns details about the reserved concurrency configuration for a function. To set a concurrency limit for a function, use PutFunctionConcurrency. |
| `GetFunctionConfiguration` | `GET /2015-03-31/functions/{FunctionName}/configuration` | `readonly` | `FunctionName` | - | `FunctionConfiguration` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns the version-specific settings of a Lambda function or version. The output includes only options that can vary between versions of a function. |
| `GetFunctionEventInvokeConfig` | `GET /2019-09-25/functions/{FunctionName}/event-invoke-config` | `readonly` | `FunctionName` | - | `FunctionEventInvokeConfig` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves the configuration for asynchronous invocation for a function, version, or alias. To configure options for asynchronous invocation, use PutFunctionEventInvokeConfig. |
| `GetFunctionRecursionConfig` | `GET /2024-08-31/functions/{FunctionName}/recursion-config` | `readonly` | `FunctionName` | - | `GetFunctionRecursionConfigResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns your function's recursive loop detection configuration. |
| `GetFunctionScalingConfig` | `GET /2025-11-30/functions/{FunctionName}/function-scaling-config` | `readonly` | `FunctionName`, `Qualifier` | - | `GetFunctionScalingConfigResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves the scaling configuration for a Lambda Managed Instances function. |
| `GetFunctionUrlConfig` | `GET /2021-10-31/functions/{FunctionName}/url` | `readonly` | `FunctionName` | - | `GetFunctionUrlConfigResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns details about a Lambda function URL. |
| `GetLayerVersion` | `GET /2018-10-31/layers/{LayerName}/versions/{VersionNumber}` | `readonly` | `LayerName`, `VersionNumber` | - | `GetLayerVersionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns information about a version of an Lambda layer, with a link to download the layer archive that's valid for 10 minutes. |
| `GetLayerVersionByArn` | `GET /2018-10-31/layers?find=LayerVersion` | `readonly` | `Arn` | - | `GetLayerVersionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns information about a version of an Lambda layer, with a link to download the layer archive that's valid for 10 minutes. |
| `GetLayerVersionPolicy` | `GET /2018-10-31/layers/{LayerName}/versions/{VersionNumber}/policy` | `readonly` | `LayerName`, `VersionNumber` | - | `GetLayerVersionPolicyResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns the permission policy for a version of an Lambda layer. For more information, see AddLayerVersionPermission. |
| `GetPolicy` | `GET /2015-03-31/functions/{FunctionName}/policy` | `readonly` | `FunctionName` | - | `GetPolicyResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns the resource-based IAM policy for a function, version, or alias. |
| `GetProvisionedConcurrencyConfig` | `GET /2019-09-30/functions/{FunctionName}/provisioned-concurrency` | `readonly` | `FunctionName`, `Qualifier` | - | `GetProvisionedConcurrencyConfigResponse` | `InvalidParameterValueException`, `ProvisionedConcurrencyConfigNotFoundException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves the provisioned concurrency configuration for a function's alias or version. |
| `GetRuntimeManagementConfig` | `GET /2021-07-20/functions/{FunctionName}/runtime-management-config` | `readonly` | `FunctionName` | - | `GetRuntimeManagementConfigResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves the runtime management configuration for a function's version. If the runtime update mode is Manual , this includes the ARN of the runtime version and the runtime update mode. |
| `Invoke` | `POST /2015-03-31/functions/{FunctionName}/invocations` | - | `FunctionName` | - | `InvocationResponse` | `DurableExecutionAlreadyStartedException`, `EC2AccessDeniedException`, `EC2ThrottledException`, `EC2UnexpectedException`, `EFSIOException`, `EFSMountConnectivityException`, `EFSMountFailureException`, `EFSMountTimeoutException`, ... (+25) | Invokes a Lambda function. You can invoke a function synchronously (and wait for the response), or asynchronously. |
| `InvokeAsync` | `POST /2014-11-13/functions/{FunctionName}/invoke-async` | - | `FunctionName`, `InvokeArgs` | - | `InvokeAsyncResponse` | `InvalidRequestContentException`, `InvalidRuntimeException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException` | For asynchronous function invocation, use Invoke. Invokes a function asynchronously. |
| `InvokeWithResponseStream` | `POST /2021-11-15/functions/{FunctionName}/response-streaming-invocations` | - | `FunctionName` | - | `InvokeWithResponseStreamResponse` | `EC2AccessDeniedException`, `EC2ThrottledException`, `EC2UnexpectedException`, `EFSIOException`, `EFSMountConnectivityException`, `EFSMountFailureException`, `EFSMountTimeoutException`, `ENILimitReachedException`, ... (+24) | Configure your Lambda functions to stream response payloads back to clients. For more information, see Configuring a Lambda function to stream responses. |
| `ListAliases` | `GET /2015-03-31/functions/{FunctionName}/aliases` | `readonly`, `paginated` | `FunctionName` | - | `ListAliasesResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns a list of aliases for a Lambda function. |
| `ListCapacityProviders` | `GET /2025-11-30/capacity-providers` | `readonly`, `paginated` | - | - | `ListCapacityProvidersResponse` | `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Returns a list of capacity providers in your account. |
| `ListCodeSigningConfigs` | `GET /2020-04-22/code-signing-configs` | `readonly`, `paginated` | - | - | `ListCodeSigningConfigsResponse` | `InvalidParameterValueException`, `ServiceException` | Returns a list of code signing configurations. A request returns up to 10,000 configurations per call. |
| `ListDurableExecutionsByFunction` | `GET /2025-12-01/functions/{FunctionName}/durable-executions` | `readonly`, `paginated` | `FunctionName` | - | `ListDurableExecutionsByFunctionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns a list of durable executions for a specified Lambda function. You can filter the results by execution name, status, and start time range. |
| `ListEventSourceMappings` | `GET /2015-03-31/event-source-mappings` | `readonly`, `paginated` | - | - | `ListEventSourceMappingsResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Lists event source mappings. Specify an `EventSourceArn` to show only event source mappings for a single event source. |
| `ListFunctionEventInvokeConfigs` | `GET /2019-09-25/functions/{FunctionName}/event-invoke-config/list` | `readonly`, `paginated` | `FunctionName` | - | `ListFunctionEventInvokeConfigsResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves a list of configurations for asynchronous invocation for a function. To configure options for asynchronous invocation, use PutFunctionEventInvokeConfig. |
| `ListFunctionUrlConfigs` | `GET /2021-10-31/functions/{FunctionName}/urls` | `readonly`, `paginated` | `FunctionName` | - | `ListFunctionUrlConfigsResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns a list of Lambda function URLs for the specified function. |
| `ListFunctionVersionsByCapacityProvider` | `GET /2025-11-30/capacity-providers/{CapacityProviderName}/function-versions` | `readonly`, `paginated` | `CapacityProviderName` | - | `ListFunctionVersionsByCapacityProviderResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns a list of function versions that are configured to use a specific capacity provider. |
| `ListFunctions` | `GET /2015-03-31/functions` | `readonly`, `paginated` | - | - | `ListFunctionsResponse` | `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Returns a list of Lambda functions, with the version-specific configuration of each. Lambda returns up to 50 functions per call. |
| `ListFunctionsByCodeSigningConfig` | `GET /2020-04-22/code-signing-configs/{CodeSigningConfigArn}/functions` | `readonly`, `paginated` | `CodeSigningConfigArn` | - | `ListFunctionsByCodeSigningConfigResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException` | List the functions that use the specified code signing configuration. You can use this method prior to deleting a code signing configuration, to verify that no functions are using it. |
| `ListLayerVersions` | `GET /2018-10-31/layers/{LayerName}/versions` | `readonly`, `paginated` | `LayerName` | - | `ListLayerVersionsResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Lists the versions of an Lambda layer. Versions that have been deleted aren't listed. |
| `ListLayers` | `GET /2018-10-31/layers` | `readonly`, `paginated` | - | - | `ListLayersResponse` | `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Lists Lambda layers and shows information about the latest version of each. Specify a runtime identifier to list only layers that indicate that they're compatible with that runtime. |
| `ListProvisionedConcurrencyConfigs` | `GET /2019-09-30/functions/{FunctionName}/provisioned-concurrency?List=ALL` | `readonly`, `paginated` | `FunctionName` | - | `ListProvisionedConcurrencyConfigsResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves a list of provisioned concurrency configurations for a function. |
| `ListTags` | `GET /2017-03-31/tags/{Resource}` | `readonly` | `Resource` | - | `ListTagsResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns a function, event source mapping, or code signing configuration's tags. You can also view function tags with GetFunction. |
| `ListVersionsByFunction` | `GET /2015-03-31/functions/{FunctionName}/versions` | `readonly`, `paginated` | `FunctionName` | - | `ListVersionsByFunctionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns a list of versions, with the version-specific configuration of each. Lambda returns up to 50 versions per call. |
| `PublishLayerVersion` | `POST /2018-10-31/layers/{LayerName}/versions` | - | `Content`, `LayerName` | - | `PublishLayerVersionResponse` | `CodeStorageExceededException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Creates an Lambda layer from a ZIP archive. Each time you call `PublishLayerVersion` with the same layer name, a new version is created. |
| `PublishVersion` | `POST /2015-03-31/functions/{FunctionName}/versions` | - | `FunctionName` | - | `FunctionConfiguration` | `CodeStorageExceededException`, `FunctionVersionsPerCapacityProviderLimitExceededException`, `InvalidParameterValueException`, `PreconditionFailedException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Creates a version from the current code and configuration of a function. Use versions to create a snapshot of your function code and configuration that doesn't change. |
| `PutFunctionCodeSigningConfig` | `PUT /2020-06-30/functions/{FunctionName}/code-signing-config` | - | `CodeSigningConfigArn`, `FunctionName` | - | `PutFunctionCodeSigningConfigResponse` | `CodeSigningConfigNotFoundException`, `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Update the code signing configuration for the function. Changes to the code signing configuration take effect the next time a user tries to deploy a code package to the function. |
| `PutFunctionConcurrency` | `PUT /2017-10-31/functions/{FunctionName}/concurrency` | - | `FunctionName`, `ReservedConcurrentExecutions` | - | `Concurrency` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Sets the maximum number of simultaneous executions for a function, and reserves capacity for that concurrency level. Concurrency settings apply to the function as a whole, including all published versions and the unpublished version. |
| `PutFunctionEventInvokeConfig` | `PUT /2019-09-25/functions/{FunctionName}/event-invoke-config` | - | `FunctionName` | - | `FunctionEventInvokeConfig` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Configures options for asynchronous invocation on a function, version, or alias. If a configuration already exists for a function, version, or alias, this operation overwrites it. |
| `PutFunctionRecursionConfig` | `PUT /2024-08-31/functions/{FunctionName}/recursion-config` | - | `FunctionName`, `RecursiveLoop` | - | `PutFunctionRecursionConfigResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Sets your function's recursive loop detection configuration. When you configure a Lambda function to output to the same service or resource that invokes the function, it's possible to create an infinite recursive loop. |
| `PutFunctionScalingConfig` | `PUT /2025-11-30/functions/{FunctionName}/function-scaling-config` | - | `FunctionName`, `Qualifier` | - | `PutFunctionScalingConfigResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Sets the scaling configuration for a Lambda Managed Instances function. The scaling configuration defines the minimum and maximum number of execution environments that can be provisioned for the function, allowing you to control scaling behavior and resource... |
| `PutProvisionedConcurrencyConfig` | `PUT /2019-09-30/functions/{FunctionName}/provisioned-concurrency` | `idempotent` | `FunctionName`, `ProvisionedConcurrentExecutions`, `Qualifier` | - | `PutProvisionedConcurrencyConfigResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Adds a provisioned concurrency configuration to a function's alias or version. |
| `PutRuntimeManagementConfig` | `PUT /2021-07-20/functions/{FunctionName}/runtime-management-config` | - | `FunctionName`, `UpdateRuntimeOn` | - | `PutRuntimeManagementConfigResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Sets the runtime management configuration for a function's version. For more information, see Runtime updates. |
| `RemoveLayerVersionPermission` | `DELETE /2018-10-31/layers/{LayerName}/versions/{VersionNumber}/policy/{StatementId}` | - | `LayerName`, `StatementId`, `VersionNumber` | - | `Unit` | `InvalidParameterValueException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Removes a statement from the permissions policy for a version of an Lambda layer. For more information, see AddLayerVersionPermission. |
| `RemovePermission` | `DELETE /2015-03-31/functions/{FunctionName}/policy/{StatementId}` | - | `FunctionName`, `StatementId` | - | `Unit` | `InvalidParameterValueException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Revokes function-use permission from an Amazon Web Services service or another Amazon Web Services account. You can get the ID of the statement from the output of GetPolicy. |
| `SendDurableExecutionCallbackFailure` | `POST /2025-12-01/durable-execution-callbacks/{CallbackId}/fail` | - | `CallbackId` | - | `SendDurableExecutionCallbackFailureResponse` | `CallbackTimeoutException`, `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Sends a failure response for a callback operation in a durable execution. Use this API when an external system cannot complete a callback operation successfully. |
| `SendDurableExecutionCallbackHeartbeat` | `POST /2025-12-01/durable-execution-callbacks/{CallbackId}/heartbeat` | - | `CallbackId` | - | `SendDurableExecutionCallbackHeartbeatResponse` | `CallbackTimeoutException`, `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Sends a heartbeat signal for a long-running callback operation to prevent timeout. Use this API to extend the callback timeout period while the external operation is still in progress. |
| `SendDurableExecutionCallbackSuccess` | `POST /2025-12-01/durable-execution-callbacks/{CallbackId}/succeed` | - | `CallbackId` | - | `SendDurableExecutionCallbackSuccessResponse` | `CallbackTimeoutException`, `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Sends a successful completion response for a callback operation in a durable execution. Use this API when an external system has successfully completed a callback operation. |
| `StopDurableExecution` | `POST /2025-12-01/durable-executions/{DurableExecutionArn}/stop` | - | `DurableExecutionArn` | - | `StopDurableExecutionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Stops a running durable execution. The execution transitions to STOPPED status and cannot be resumed. |
| `TagResource` | `POST /2017-03-31/tags/{Resource}` | - | `Resource`, `Tags` | - | `Unit` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Adds tags to a function, event source mapping, or code signing configuration. |
| `UntagResource` | `DELETE /2017-03-31/tags/{Resource}` | - | `Resource`, `TagKeys` | - | `Unit` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Removes tags from a function, event source mapping, or code signing configuration. |
| `UpdateAlias` | `PUT /2015-03-31/functions/{FunctionName}/aliases/{Name}` | - | `FunctionName`, `Name` | - | `AliasConfiguration` | `InvalidParameterValueException`, `PreconditionFailedException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Updates the configuration of a Lambda function alias. |
| `UpdateCapacityProvider` | `PUT /2025-11-30/capacity-providers/{CapacityProviderName}` | - | `CapacityProviderName` | - | `UpdateCapacityProviderResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Updates the configuration of an existing capacity provider. |
| `UpdateCodeSigningConfig` | `PUT /2020-04-22/code-signing-configs/{CodeSigningConfigArn}` | - | `CodeSigningConfigArn` | - | `UpdateCodeSigningConfigResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException` | Update the code signing configuration. Changes to the code signing configuration take effect the next time a user tries to deploy a code package to the function. |
| `UpdateEventSourceMapping` | `PUT /2015-03-31/event-source-mappings/{UUID}` | - | `UUID` | - | `EventSourceMappingConfiguration` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Updates an event source mapping. You can change the function that Lambda invokes, or pause invocation and resume later from the same location. |
| `UpdateFunctionCode` | `PUT /2015-03-31/functions/{FunctionName}/code` | - | `FunctionName` | - | `FunctionConfiguration` | `CodeSigningConfigNotFoundException`, `CodeStorageExceededException`, `CodeVerificationFailedException`, `InvalidCodeSignatureException`, `InvalidParameterValueException`, `PreconditionFailedException`, `ResourceConflictException`, `ResourceNotFoundException`, ... (+2) | Updates a Lambda function's code. If code signing is enabled for the function, the code package must be signed by a trusted publisher. |
| `UpdateFunctionConfiguration` | `PUT /2015-03-31/functions/{FunctionName}/configuration` | - | `FunctionName` | - | `FunctionConfiguration` | `CodeSigningConfigNotFoundException`, `CodeVerificationFailedException`, `InvalidCodeSignatureException`, `InvalidParameterValueException`, `PreconditionFailedException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, ... (+1) | Modify the version-specific settings of a Lambda function. When you update a function, Lambda provisions an instance of the function and its supporting resources. |
| `UpdateFunctionEventInvokeConfig` | `POST /2019-09-25/functions/{FunctionName}/event-invoke-config` | - | `FunctionName` | - | `FunctionEventInvokeConfig` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Updates the configuration for asynchronous invocation for a function, version, or alias. To configure options for asynchronous invocation, use PutFunctionEventInvokeConfig. |
| `UpdateFunctionUrlConfig` | `PUT /2021-10-31/functions/{FunctionName}/url` | - | `FunctionName` | - | `UpdateFunctionUrlConfigResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Updates the configuration for a Lambda function URL. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ServiceException` | `structure` | `Message`, `Type` | The Lambda service encountered an internal error. |
| `InvalidParameterValueException` | `structure` | `Type`, `message` | One of the parameters in the request is not valid. |
| `TooManyRequestsException` | `structure` | `Reason`, `Type`, `message`, `retryAfterSeconds` | The request throughput limit was exceeded. |
| `ResourceNotFoundException` | `structure` | `Message`, `Type` | The resource specified in the request does not exist. |
| `ResourceConflictException` | `structure` | `Type`, `message` | The resource already exists, or another operation is in progress. |
| `PreconditionFailedException` | `structure` | `Type`, `message` | The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. |
| `FunctionConfiguration` | `structure` | `Architectures`, `CapacityProviderConfig`, `CodeSha256`, `CodeSize`, `ConfigSha256`, `DeadLetterConfig`, `Description`, `DurableConfig`, `Environment`, `EphemeralStorage`, `FileSystemConfigs`, `FunctionArn`, ... (+28) | Details about a function's configuration. |
| `CodeSigningConfigNotFoundException` | `structure` | `Message`, `Type` | The specified code signing configuration does not exist. |
| `EventSourceMappingConfiguration` | `structure` | `AmazonManagedKafkaEventSourceConfig`, `BatchSize`, `BisectBatchOnFunctionError`, `DestinationConfig`, `DocumentDBEventSourceConfig`, `EventSourceArn`, `EventSourceMappingArn`, `FilterCriteria`, `FilterCriteriaError`, `FunctionArn`, `FunctionResponseTypes`, `KMSKeyArn`, ... (+21) | A mapping between an Amazon Web Services resource and a Lambda function. |
| `CodeStorageExceededException` | `structure` | `Type`, `message` | Your Amazon Web Services account has exceeded its maximum total code size. |
| `AliasConfiguration` | `structure` | `AliasArn`, `Description`, `FunctionVersion`, `Name`, `RevisionId`, `RoutingConfig` | Provides configuration information about a Lambda function alias. |
| `CodeVerificationFailedException` | `structure` | `Message`, `Type` | The code signature failed one or more of the validation checks for signature mismatch or expiry, and the code signing policy is set to ENFORCE. |
| `InvalidCodeSignatureException` | `structure` | `Message`, `Type` | The code signature failed the integrity check. |
| `FunctionEventInvokeConfig` | `structure` | `DestinationConfig`, `FunctionArn`, `LastModified`, `MaximumEventAgeInSeconds`, `MaximumRetryAttempts` | - |
| `InvalidRequestContentException` | `structure` | `Type`, `message` | The request body could not be parsed as JSON, or a request header is invalid. |
| `InvalidRuntimeException` | `structure` | `Message`, `Type` | The runtime or runtime version specified is not supported. |
| `CallbackTimeoutException` | `structure` | `Message`, `Type` | The callback ID token has either expired or the callback associated with the token has already been closed. |
| `PolicyLengthExceededException` | `structure` | `Type`, `message` | The permissions policy for the resource is too large. |
| `FunctionVersionsPerCapacityProviderLimitExceededException` | `structure` | `Type`, `message` | The maximum number of function versions that can be associated with a single capacity provider has been exceeded. |
| `ResourceInUseException` | `structure` | `Message`, `Type` | The operation conflicts with the resource's availability. |
| `GetLayerVersionResponse` | `structure` | `CompatibleArchitectures`, `CompatibleRuntimes`, `Content`, `CreatedDate`, `Description`, `LayerArn`, `LayerVersionArn`, `LicenseInfo`, `Version` | - |
| `EC2AccessDeniedException` | `structure` | `Message`, `Type` | Need additional permissions to configure VPC settings. |
| `EC2ThrottledException` | `structure` | `Message`, `Type` | Amazon EC2 throttled Lambda during Lambda function initialization using the execution role provided for the function. |

## Winterbaume LTM Notes

Sources: .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises Lambda's highest-value integration paths. Open it when implementing event-source mappings or cross-service invocation because DynamoDB Streams, SQS, SNS, and Kinesis all have documented event-source or trigger integrations into Lambda.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises execution-boundary rules. Open it before treating upstream events as plain `Invoke` calls, because source-specific payloads, retries, and partial failures matter.
- Service implication: Step Functions has an optimised Lambda integration, and API Gateway has Lambda proxy and custom integrations. These are better integration-test targets than generic service-to-service shortcuts.
- Service implication: event-source work should preserve source-specific event payload contracts and error or retry semantics.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
