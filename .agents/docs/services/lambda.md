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

### Opaque binary payloads (`@httpPayload` blobs)

Several Lambda operations bind a **blob** shape to `@httpPayload`. The body is then raw, opaque bytes that Lambda passes through verbatim — it must never be UTF-8 decoded or parsed. In the `HTTP Bindings` matrix a blob payload is indistinguishable from a structure payload by binding name alone, so the target-shape kind is tracked separately.

Blob `@httpPayload` members in Lambda:

| Operation | Member | Direction | Contract |
|---|---|---|---|
| `Invoke` | `Payload` | request/response | opaque octets — the invocation event, may be arbitrary binary |
| `InvokeWithResponseStream` | `Payload` | request | opaque octets |
| `InvokeAsync` | `InvokeArgs` | request | opaque octets ( legacy API ) |
| `SendDurableExecutionCallbackSuccess` | `Result` | request | opaque octets |

Parity rule for winterbaume: a blob `@httpPayload` member must be modelled as raw bytes (`bytes::Bytes`) and **must never be run through `std::str::from_utf8`**. UTF-8-validating the body rejects any non-UTF-8 invocation payload (protobuf, msgpack, gzip, an image, …) with a spurious `400` before the handler runs — the same defect class as S3 `UploadPart`. The fix is in the shared generator (`tools/smithy-codegen/src/gen_serializers.rs`), guarded by the `payload_semantics_tests::http_payload_blobs_are_not_utf8_validated` backstop. Winterbaume's `Invoke` handler currently ignores the payload, so this was latent rather than reported, but the contract still holds. See [GitHub issue #12](https://github.com/moriyoshi/winterbaume/issues/12) for the S3 report that surfaced the shared root cause.


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

- Operations: `GetAccountSettings`, `GetDurableExecution`, `GetDurableExecutionHistory`, `GetDurableExecutionState`, `GetFunctionEventInvokeConfig`
- Traits: `readonly` (5), `paginated` (2)
- Common required input members in this group: `DurableExecutionArn`

### List

- Operations: `ListDurableExecutionsByFunction`, `ListFunctionEventInvokeConfigs`, `ListTags`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: `FunctionName`

### Send

- Operations: `SendDurableExecutionCallbackFailure`, `SendDurableExecutionCallbackHeartbeat`, `SendDurableExecutionCallbackSuccess`
- Common required input members in this group: `CallbackId`

### Delete

- Operations: `DeleteFunction`, `DeleteFunctionEventInvokeConfig`
- Traits: `idempotent` (1)
- Common required input members in this group: `FunctionName`

### Checkpoint

- Operations: `CheckpointDurableExecution`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Put

- Operations: `PutFunctionEventInvokeConfig`
- Common required input members in this group: -

### Stop

- Operations: `StopDurableExecution`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateFunctionEventInvokeConfig`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CheckpointDurableExecution` | `POST /2025-12-01/durable-executions/{DurableExecutionArn}/checkpoint` | `idempotent`, `idempotency-token` | `DurableExecutionArn`, `CheckpointToken` | `ClientToken` | `CheckpointDurableExecutionResponse` | `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Saves the progress of a durable function execution during runtime. This API is used by the Lambda durable functions SDK to checkpoint completed steps and schedule asynchronous operations. You typically don't need to ... |
| `DeleteFunction` | `DELETE /2015-03-31/functions/{FunctionName}` | `idempotent` | `FunctionName` | - | `DeleteFunctionResponse` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes a Lambda function. To delete a specific function version, use the Qualifier parameter. Otherwise, all versions and aliases are deleted. This doesn't require the user to have explicit permissions for DeleteAli ... |
| `DeleteFunctionEventInvokeConfig` | `DELETE /2019-09-25/functions/{FunctionName}/event-invoke-config` | - | `FunctionName` | - | `Unit` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes the configuration for asynchronous invocation for a function, version, or alias. To configure options for asynchronous invocation, use PutFunctionEventInvokeConfig . |
| `GetAccountSettings` | `GET /2016-08-19/account-settings` | `readonly` | - | - | `GetAccountSettingsResponse` | `ServiceException`, `TooManyRequestsException` | Retrieves details about your account's limits and usage in an Amazon Web Services Region. |
| `GetDurableExecution` | `GET /2025-12-01/durable-executions/{DurableExecutionArn}` | `readonly` | `DurableExecutionArn` | - | `GetDurableExecutionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves detailed information about a specific durable execution , including its current status, input payload, result or error information, and execution metadata such as start time and usage statistics. |
| `GetDurableExecutionHistory` | `GET /2025-12-01/durable-executions/{DurableExecutionArn}/history` | `readonly`, `paginated` | `DurableExecutionArn` | - | `GetDurableExecutionHistoryResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves the execution history for a durable execution , showing all the steps, callbacks, and events that occurred during the execution. This provides a detailed audit trail of the execution's progress over time. T ... |
| `GetDurableExecutionState` | `GET /2025-12-01/durable-executions/{DurableExecutionArn}/state` | `readonly`, `paginated` | `DurableExecutionArn`, `CheckpointToken` | - | `GetDurableExecutionStateResponse` | `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Retrieves the current execution state required for the replay process during durable function execution. This API is used by the Lambda durable functions SDK to get state information needed for replay. You typically ... |
| `GetFunctionEventInvokeConfig` | `GET /2019-09-25/functions/{FunctionName}/event-invoke-config` | `readonly` | `FunctionName` | - | `FunctionEventInvokeConfig` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves the configuration for asynchronous invocation for a function, version, or alias. To configure options for asynchronous invocation, use PutFunctionEventInvokeConfig . |
| `ListDurableExecutionsByFunction` | `GET /2025-12-01/functions/{FunctionName}/durable-executions` | `readonly`, `paginated` | `FunctionName` | - | `ListDurableExecutionsByFunctionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns a list of durable executions for a specified Lambda function. You can filter the results by execution name, status, and start time range. This API supports pagination for large result sets. |
| `ListFunctionEventInvokeConfigs` | `GET /2019-09-25/functions/{FunctionName}/event-invoke-config/list` | `readonly`, `paginated` | `FunctionName` | - | `ListFunctionEventInvokeConfigsResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves a list of configurations for asynchronous invocation for a function. To configure options for asynchronous invocation, use PutFunctionEventInvokeConfig . |
| `ListTags` | `GET /2017-03-31/tags/{Resource}` | `readonly` | `Resource` | - | `ListTagsResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Returns a function, event source mapping, or code signing configuration's tags . You can also view function tags with GetFunction . |
| `PutFunctionEventInvokeConfig` | `PUT /2019-09-25/functions/{FunctionName}/event-invoke-config` | - | `FunctionName` | - | `FunctionEventInvokeConfig` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Configures options for asynchronous invocation on a function, version, or alias. If a configuration already exists for a function, version, or alias, this operation overwrites it. If you exclude any settings, they ar ... |
| `SendDurableExecutionCallbackFailure` | `POST /2025-12-01/durable-execution-callbacks/{CallbackId}/fail` | - | `CallbackId` | - | `SendDurableExecutionCallbackFailureResponse` | `CallbackTimeoutException`, `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Sends a failure response for a callback operation in a durable execution. Use this API when an external system cannot complete a callback operation successfully. |
| `SendDurableExecutionCallbackHeartbeat` | `POST /2025-12-01/durable-execution-callbacks/{CallbackId}/heartbeat` | - | `CallbackId` | - | `SendDurableExecutionCallbackHeartbeatResponse` | `CallbackTimeoutException`, `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Sends a heartbeat signal for a long-running callback operation to prevent timeout. Use this API to extend the callback timeout period while the external operation is still in progress. |
| `SendDurableExecutionCallbackSuccess` | `POST /2025-12-01/durable-execution-callbacks/{CallbackId}/succeed` | - | `CallbackId` | - | `SendDurableExecutionCallbackSuccessResponse` | `CallbackTimeoutException`, `InvalidParameterValueException`, `ServiceException`, `TooManyRequestsException` | Sends a successful completion response for a callback operation in a durable execution. Use this API when an external system has successfully completed a callback operation. |
| `StopDurableExecution` | `POST /2025-12-01/durable-executions/{DurableExecutionArn}/stop` | - | `DurableExecutionArn` | - | `StopDurableExecutionResponse` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Stops a running durable execution . The execution transitions to STOPPED status and cannot be resumed. Any in-progress operations are terminated. |
| `TagResource` | `POST /2017-03-31/tags/{Resource}` | - | `Resource`, `Tags` | - | `Unit` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Adds tags to a function, event source mapping, or code signing configuration. |
| `UntagResource` | `DELETE /2017-03-31/tags/{Resource}` | - | `Resource`, `TagKeys` | - | `Unit` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Removes tags from a function, event source mapping, or code signing configuration. |
| `UpdateFunctionEventInvokeConfig` | `POST /2019-09-25/functions/{FunctionName}/event-invoke-config` | - | `FunctionName` | - | `FunctionEventInvokeConfig` | `InvalidParameterValueException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceException`, `TooManyRequestsException` | Updates the configuration for asynchronous invocation for a function, version, or alias. To configure options for asynchronous invocation, use PutFunctionEventInvokeConfig . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteFunction` | - | `Qualifier -> Qualifier` | - | - |
| `DeleteFunctionEventInvokeConfig` | - | `Qualifier -> Qualifier` | - | - |
| `GetDurableExecutionHistory` | - | `IncludeExecutionData -> IncludeExecutionData`, `MaxItems -> MaxItems`, `Marker -> Marker`, `ReverseOrder -> ReverseOrder` | - | - |
| `GetDurableExecutionState` | - | `CheckpointToken -> CheckpointToken`, `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `GetFunctionEventInvokeConfig` | - | `Qualifier -> Qualifier` | - | - |
| `ListDurableExecutionsByFunction` | - | `Qualifier -> Qualifier`, `DurableExecutionName -> DurableExecutionName`, `Statuses -> Statuses`, `StartedAfter -> StartedAfter`, `StartedBefore -> StartedBefore`, `ReverseOrder -> ReverseOrder`, `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListFunctionEventInvokeConfigs` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `PutFunctionEventInvokeConfig` | - | `Qualifier -> Qualifier` | - | - |
| `SendDurableExecutionCallbackFailure` | - | - | - | `Error` |
| `SendDurableExecutionCallbackSuccess` | - | - | - | `Result` |
| `StopDurableExecution` | - | - | - | `Error` |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |
| `UpdateFunctionEventInvokeConfig` | - | `Qualifier -> Qualifier` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CallbackTimeoutException` | `structure` | Type, Message | The callback ID token has either expired or the callback associated with the token has already been closed. |
| `CapacityProviderLimitExceededException` | `structure` | Type, message | The maximum number of capacity providers for your account has been exceeded. For more information, see Lambda quotas |
| `CodeSigningConfigNotFoundException` | `structure` | Type, Message | The specified code signing configuration does not exist. |
| `CodeStorageExceededException` | `structure` | Type, message | Your Amazon Web Services account has exceeded its maximum total code size. For more information, see Lambda quotas . |
| `CodeVerificationFailedException` | `structure` | Type, Message | The code signature failed one or more of the validation checks for signature mismatch or expiry, and the code signing policy is set to ENFORCE. Lambda block ... |
| `DurableExecutionAlreadyStartedException` | `structure` | Type, Message | The durable execution with the specified name has already been started. Each durable execution name must be unique within the function. Use a different name ... |
| `EC2AccessDeniedException` | `structure` | Type, Message | Need additional permissions to configure VPC settings. |
| `EC2ThrottledException` | `structure` | Type, Message | Amazon EC2 throttled Lambda during Lambda function initialization using the execution role provided for the function. |
| `EC2UnexpectedException` | `structure` | Type, Message, EC2ErrorCode | Lambda received an unexpected Amazon EC2 client exception while setting up for the Lambda function. |
| `EFSIOException` | `structure` | Type, Message | An error occurred when reading from or writing to a connected file system. |
| `EFSMountConnectivityException` | `structure` | Type, Message | The Lambda function couldn't make a network connection to the configured file system. |
| `EFSMountFailureException` | `structure` | Type, Message | The Lambda function couldn't mount the configured file system due to a permission or configuration issue. |
| `EFSMountTimeoutException` | `structure` | Type, Message | The Lambda function made a network connection to the configured file system, but the mount operation timed out. |
| `ENILimitReachedException` | `structure` | Type, Message | Lambda couldn't create an elastic network interface in the VPC, specified as part of Lambda function configuration, because the limit for network interfaces ... |
| `FunctionVersionsPerCapacityProviderLimitExceededException` | `structure` | Type, message | The maximum number of function versions that can be associated with a single capacity provider has been exceeded. For more information, see Lambda quotas . |
| `InvalidCodeSignatureException` | `structure` | Type, Message | The code signature failed the integrity check. If the integrity check fails, then Lambda blocks deployment, even if the code signing policy is set to WARN. |
| `InvalidParameterValueException` | `structure` | Type, message | One of the parameters in the request is not valid. |
| `InvalidRequestContentException` | `structure` | Type, message | The request body could not be parsed as JSON, or a request header is invalid. For example, the 'x-amzn-RequestId' header is not a valid UUID string. |
| `InvalidRuntimeException` | `structure` | Type, Message | The runtime or runtime version specified is not supported. |
| `InvalidSecurityGroupIDException` | `structure` | Type, Message | The security group ID provided in the Lambda function VPC configuration is not valid. |
| `InvalidSubnetIDException` | `structure` | Type, Message | The subnet ID provided in the Lambda function VPC configuration is not valid. |
| `InvalidZipFileException` | `structure` | Type, Message | Lambda could not unzip the deployment package. |
| `KMSAccessDeniedException` | `structure` | Type, Message | Lambda couldn't decrypt the environment variables because KMS access was denied. Check the Lambda function's KMS permissions. |
| `KMSDisabledException` | `structure` | Type, Message | Lambda couldn't decrypt the environment variables because the KMS key used is disabled. Check the Lambda function's KMS key settings. |
| `KMSInvalidStateException` | `structure` | Type, Message | Lambda couldn't decrypt the environment variables because the state of the KMS key used is not valid for Decrypt. Check the function's KMS key settings. |
| `KMSNotFoundException` | `structure` | Type, Message | Lambda couldn't decrypt the environment variables because the KMS key was not found. Check the function's KMS key settings. |
| `NoPublishedVersionException` | `structure` | Type, Message | The function has no published versions available. |
| `PolicyLengthExceededException` | `structure` | Type, message | The permissions policy for the resource is too large. For more information, see Lambda quotas . |
| `PreconditionFailedException` | `structure` | Type, message | The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. For AddPermission and RemovePermission API operations: Call G ... |
| `ProvisionedConcurrencyConfigNotFoundException` | `structure` | Type, message | The specified configuration does not exist. |
| `RecursiveInvocationException` | `structure` | Type, Message | Lambda has detected your function being invoked in a recursive loop with other Amazon Web Services resources and stopped your function's invocation. |
| `RequestTooLargeException` | `structure` | Type, message | The request payload exceeded the Invoke request body JSON input quota. For more information, see Lambda quotas . |
| `ResourceConflictException` | `structure` | Type, message | The resource already exists, or another operation is in progress. |
| `ResourceInUseException` | `structure` | Type, Message | The operation conflicts with the resource's availability. For example, you tried to update an event source mapping in the CREATING state, or you tried to de ... |
| `ResourceNotFoundException` | `structure` | Type, Message | The resource specified in the request does not exist. |
| `ResourceNotReadyException` | `structure` | Type, message | The function is inactive and its VPC connection is no longer available. Wait for the VPC connection to reestablish and try again. |
| `S3FilesMountConnectivityException` | `structure` | Type, Message | The Lambda function couldn't make a network connection to the configured S3 Files access point. |
| `S3FilesMountFailureException` | `structure` | Type, Message | The Lambda function couldn't mount the configured S3 Files access point due to a permission or configuration issue. |
| `S3FilesMountTimeoutException` | `structure` | Type, Message | The Lambda function made a network connection to the configured S3 Files access point, but the mount operation timed out. |
| `SerializedRequestEntityTooLargeException` | `structure` | Type, message | The request payload exceeded the maximum allowed size for serialized request entities. |
| `ServiceException` | `structure` | Type, Message | The Lambda service encountered an internal error. |
| `SnapStartException` | `structure` | Type, Message | The afterRestore() runtime hook encountered an error. For more information, check the Amazon CloudWatch logs. |
| `SnapStartNotReadyException` | `structure` | Type, Message | Lambda is initializing your function. You can invoke the function when the function state becomes Active . |
| `SnapStartTimeoutException` | `structure` | Type, Message | Lambda couldn't restore the snapshot within the timeout limit. |
| `SubnetIPAddressLimitReachedException` | `structure` | Type, Message | Lambda couldn't set up VPC access for the Lambda function because one or more configured subnets has no available IP addresses. |
| `TooManyRequestsException` | `structure` | retryAfterSeconds, Type, message, Reason | The request throughput limit was exceeded. For more information, see Lambda quotas . |
| `UnsupportedMediaTypeException` | `structure` | Type, message | The content type of the Invoke request body is not JSON. |
| `ApplicationLogLevel` | `enum` | Trace, Debug, Info, Warn, Error, Fatal | - |
| `Architecture` | `enum` | x86_64, arm64 | - |
| `CapacityProviderPredefinedMetricType` | `enum` | LambdaCapacityProviderAverageCPUUtilization | - |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises Lambda's highest-value integration paths. Open it when implementing event-source mappings or cross-service invocation because DynamoDB Streams, SQS, SNS, and Kinesis all have documented event-source or trigger integrations into Lambda.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises execution-boundary rules. Open it before treating upstream events as plain `Invoke` calls, because source-specific payloads, retries, and partial failures matter.
- Service implication: Step Functions has an optimised Lambda integration, and API Gateway has Lambda proxy and custom integrations. These are better integration-test targets than generic service-to-service shortcuts.
- Service implication: event-source work should preserve source-specific event payload contracts and error or retry semantics.

## Member Audit

Coverage is counted per operation, `Operation Detail Matrix` lists required inputs only, and `HTTP Bindings` lists header and query bindings only. This section records what happens to the remaining members: optional body members and response members.

### Resolved

- Issue #44: `AddPermission` renders `FunctionUrlAuthType`, `InvokedViaFunctionUrl`, `SourceAccount`, `PrincipalOrgID` and `EventSourceToken` as one merged `Condition`. `Qualifier` is still ignored.
- Issue #45: event source mappings store and return `FunctionResponseTypes`, `ScalingConfig` and eleven other scalar settings. The nested configs ( `FilterCriteria`, `DestinationConfig`, the Kafka and DocumentDB configs, `MetricsConfig`, `ProvisionedPollerConfig`, `SelfManagedEventSource`, `SourceAccessConfigurations` ) are still dropped by the API.
- Issue #46: `ListEventSourceMappings` applies `FunctionName` and `EventSourceArn`. `Marker` and `MaxItems` are still ignored.
- Issue #54: every mapping response carries `EventSourceMappingArn`, derived from the function ARN and the UUID. Creation `Tags` are stored, and `ListTags` / `TagResource` / `UntagResource` accept the four documented resource types: function, event source mapping, code signing config, capacity provider.

### Open candidates ( audited 2026-11-21 )

Generated by `.agents/bin/audit-wire-members.py lambda --dossier`. Rows are unclassified candidates, not confirmed defects: each one is resolved as honoured, rejected with the AWS error, ignored because AWS ignores it, or filed in `TODO.md` ( QUALITY_GATE.md §5.3 and §5.4 ). Regenerate after changing handlers.

- Unread request members: 150 in 48 operations.
- Operations whose generated request deserialiser is never called: 1.
- Never-populated response members: 73 in 31 structures ( run the script for the list ).
- Response structures populated unevenly across literals: 11.

#### Unread request members

- `add_permission` ( 1 of 12 ): `qualifier`
- `checkpoint_durable_execution` ( 3 of 4 ): `checkpoint_token`, `client_token`, `updates`
- `create_capacity_provider` ( 5 of 7 ): `capacity_provider_scaling_config`, `instance_requirements`, `kms_key_arn`, `permissions_config`, `vpc_config`
- `create_event_source_mapping` ( 9 of 28 ): `amazon_managed_kafka_event_source_config`, `destination_config`, `document_d_b_event_source_config`, `filter_criteria`, `metrics_config`, `provisioned_poller_config`, `self_managed_event_source`, `self_managed_kafka_event_source_config`, `source_access_configurations`
- `create_function` ( 18 of 28 ): `architectures`, `capacity_provider_config`, `code_signing_config_arn`, `dead_letter_config`, `durable_config`, `ephemeral_storage`, `file_system_configs`, `image_config`, `k_m_s_key_arn`, `layers`, `logging_config`, `package_type`, `publish`, `publish_to`, `snap_start`, `tenancy_config`, `tracing_config`, `vpc_config`
- `create_function_url_config` ( 1 of 5 ): `qualifier`
- `delete_function` ( 1 of 2 ): `qualifier`
- `delete_function_event_invoke_config` ( 1 of 2 ): `qualifier`
- `delete_function_url_config` ( 1 of 2 ): `qualifier`
- `get_account_settings` ( 2 of 2 ) ( input discarded ): `account_limit`, `account_usage`
- `get_durable_execution_history` ( 4 of 5 ): `include_execution_data`, `marker`, `max_items`, `reverse_order`
- `get_durable_execution_state` ( 3 of 4 ): `checkpoint_token`, `marker`, `max_items`
- `get_function` ( 1 of 2 ): `qualifier`
- `get_function_configuration` ( 1 of 2 ): `qualifier`
- `get_function_event_invoke_config` ( 1 of 2 ): `qualifier`
- `get_function_scaling_config` ( 1 of 2 ): `qualifier`
- `get_function_url_config` ( 1 of 2 ): `qualifier`
- `get_policy` ( 1 of 2 ): `qualifier`
- `get_runtime_management_config` ( 1 of 2 ): `qualifier`
- `invoke` ( 6 of 8 ): `client_context`, `durable_execution_name`, `invocation_type`, `log_type`, `payload`, `tenant_id`
- `invoke_async` ( 1 of 2 ): `invoke_args`
- `invoke_with_response_stream` ( 5 of 7 ): `client_context`, `invocation_type`, `log_type`, `payload`, `tenant_id`
- `list_aliases` ( 3 of 4 ): `function_version`, `marker`, `max_items`
- `list_capacity_providers` ( 3 of 3 ) ( input discarded ): `marker`, `max_items`, `state`
- `list_code_signing_configs` ( 2 of 2 ) ( input discarded ): `marker`, `max_items`
- `list_durable_executions_by_function` ( 8 of 9 ): `durable_execution_name`, `marker`, `max_items`, `qualifier`, `reverse_order`, `started_after`, `started_before`, `statuses`
- `list_event_source_mappings` ( 2 of 4 ): `marker`, `max_items`
- `list_function_event_invoke_configs` ( 2 of 3 ): `marker`, `max_items`
- `list_function_url_configs` ( 3 of 3 ) ( input discarded ): `function_name`, `marker`, `max_items`
- `list_function_versions_by_capacity_provider` ( 3 of 3 ) ( input discarded ): `capacity_provider_name`, `marker`, `max_items`
- `list_functions_by_code_signing_config` ( 2 of 3 ): `marker`, `max_items`
- `list_layer_versions` ( 4 of 5 ): `compatible_architecture`, `compatible_runtime`, `marker`, `max_items`
- `list_layers` ( 4 of 4 ) ( input discarded ): `compatible_architecture`, `compatible_runtime`, `marker`, `max_items`
- `list_provisioned_concurrency_configs` ( 2 of 3 ): `marker`, `max_items`
- `list_versions_by_function` ( 2 of 3 ): `marker`, `max_items`
- `publish_layer_version` ( 1 of 6 ): `content`
- `publish_version` ( 2 of 5 ): `code_sha256`, `publish_to`
- `put_function_event_invoke_config` ( 1 of 5 ): `qualifier`
- `put_function_scaling_config` ( 1 of 3 ): `qualifier`
- `put_runtime_management_config` ( 1 of 4 ): `qualifier`
- `remove_permission` ( 1 of 4 ): `qualifier`
- `send_durable_execution_callback_failure` ( 1 of 2 ): `error`
- `update_capacity_provider` ( 1 of 2 ): `capacity_provider_scaling_config`
- `update_event_source_mapping` ( 8 of 22 ): `amazon_managed_kafka_event_source_config`, `destination_config`, `document_d_b_event_source_config`, `filter_criteria`, `metrics_config`, `provisioned_poller_config`, `self_managed_kafka_event_source_config`, `source_access_configurations`
- `update_function_code` ( 10 of 12 ): `architectures`, `dry_run`, `image_uri`, `publish`, `publish_to`, `s3_bucket`, `s3_key`, `s3_object_version`, `source_k_m_s_key_arn`, `zip_file`
- `update_function_configuration` ( 13 of 21 ): `capacity_provider_config`, `dead_letter_config`, `durable_config`, `ephemeral_storage`, `file_system_configs`, `image_config`, `k_m_s_key_arn`, `layers`, `logging_config`, `role`, `snap_start`, `tracing_config`, `vpc_config`
- `update_function_event_invoke_config` ( 1 of 5 ): `qualifier`
- `update_function_url_config` ( 1 of 5 ): `qualifier`

#### Operations not using the generated request deserialiser

`list_functions`

#### Unevenly populated response structures

- `AliasConfiguration` ( 2 literals ): `alias_arn` 1/2, `description` 1/2, `function_version` 1/2, `name` 1/2, `revision_id` 1/2, `routing_config` 1/2
- `CapacityProvider` ( 5 literals ): `last_modified` 4/5
- `CodeSigningConfig` ( 2 literals ): `allowed_publishers` 1/2, `code_signing_config_arn` 1/2, `code_signing_config_id` 1/2, `code_signing_policies` 1/2, `description` 1/2, `last_modified` 1/2
- `Cors` ( 2 literals ): `allow_credentials` 1/2, `allow_headers` 1/2, `allow_methods` 1/2, `allow_origins` 1/2, `expose_headers` 1/2, `max_age` 1/2
- `FunctionEventInvokeConfig` ( 3 literals ): `destination_config` 1/3
- `FunctionScalingConfig` ( 2 literals ): `max_execution_environments` 1/2
- `FunctionUrlConfig` ( 3 literals ): `auth_type` 2/3, `invoke_mode` 2/3
- `GetDurableExecutionResponse` ( 2 literals ): `durable_execution_name` 1/2, `end_timestamp` 1/2, `function_arn` 1/2, `input_payload` 1/2, `result` 1/2, `start_timestamp` 1/2, `version` 1/2
- `GetLayerVersionResponse` ( 2 literals ): `compatible_architectures` 1/2, `compatible_runtimes` 1/2, `content` 1/2, `created_date` 1/2, `description` 1/2, `layer_arn` 1/2, `layer_version_arn` 1/2, `license_info` 1/2, `version` 1/2
- `LayerVersionsListItem` ( 2 literals ): `compatible_architectures` 1/2
- `PublishLayerVersionResponse` ( 2 literals ): `compatible_architectures` 1/2, `compatible_runtimes` 1/2, `content` 1/2, `created_date` 1/2, `description` 1/2, `layer_arn` 1/2, `layer_version_arn` 1/2, `license_info` 1/2, `version` 1/2

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
