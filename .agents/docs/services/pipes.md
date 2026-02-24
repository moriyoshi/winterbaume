# Amazon EventBridge Pipes

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon EventBridge Pipes connects event sources to targets. Pipes reduces the need for specialized knowledge and integration code when developing event driven architectures. This helps ensures consistency across your company’s applications. With Pipes, the target can be any available EventBridge target. To set up a pipe, you select the event source, add optional event filtering, define optional enrichment, and select the target for the event data.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon EventBridge Pipes workflows in the local mock. Key resources include `PipeResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Describe`, `Start` operation families, including `ListPipes`, `ListTagsForResource`, `CreatePipe`, `DeletePipe`, `DescribePipe`, `StartPipe`.

## Service Identity and Protocol

- AWS model slug: `pipes`
- AWS SDK for Rust slug: `pipes`
- Model version: `2015-10-07`
- Model file: `vendor/api-models-aws/models/pipes/service/2015-10-07/pipes-2015-10-07.json`
- SDK ID: `Pipes`
- Endpoint prefix: `pipes`
- ARN namespace: `pipes`
- CloudFormation name: `Pipes`
- CloudTrail event source: `pipes.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (2), `Create` (1), `Delete` (1), `Describe` (1), `Start` (1), `Stop` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreatePipe`, `DeletePipe`, `StartPipe`, `StopPipe`, `TagResource`, `UntagResource`, `UpdatePipe`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribePipe`, `ListPipes`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartPipe`, `StopPipe`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 10 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `SQS`, `Lambda`, `EC2/VPC`, `ECS`, `Redshift`, `Secrets Manager`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `PipeResource` | `Name` | put: `CreatePipe`; read: `DescribePipe`; update: `UpdatePipe`; delete: `DeletePipe`; list: `ListPipes` | `StartPipe`, `StopPipe` | Represents a pipe that will connect a source to a target through an optional filtering and enrichment step |

## Cross-Service Integration Gaps

- **`eventbridge-pipes`** ( primary ): Pipes store source and target ARNs but perform no actual polling, transformation, or target invocation. Sources include DynamoDB Streams, Kinesis, and SQS; targets include Lambda, SQS, SNS, Step Functions, EventBridge, and others. Tracked in `.agents/docs/TODO.md` ( "Cross-Service Integration Gaps" → `eventbridge-pipes` ).

## Operation Groups

### List

- Operations: `ListPipes`, `ListTagsForResource`
- Traits: `paginated` (1), `readonly` (2)
- Common required input members in this group: `resourceArn`

### Create

- Operations: `CreatePipe`
- Traits: `idempotent` (1)
- Common required input members in this group: `Name`, `RoleArn`, `Source`, `Target`

### Delete

- Operations: `DeletePipe`
- Traits: `idempotent` (1)
- Common required input members in this group: `Name`

### Describe

- Operations: `DescribePipe`
- Traits: `readonly` (1)
- Common required input members in this group: `Name`

### Start

- Operations: `StartPipe`
- Common required input members in this group: `Name`

### Stop

- Operations: `StopPipe`
- Common required input members in this group: `Name`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdatePipe`
- Traits: `idempotent` (1)
- Common required input members in this group: `Name`, `RoleArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreatePipe` | `POST /v1/pipes/{Name}` | `idempotent` | `Name`, `RoleArn`, `Source`, `Target` | - | `CreatePipeResponse` | `ConflictException`, `InternalException`, `NotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a pipe. Amazon EventBridge Pipes connect event sources to targets and reduces the need for specialized knowledge and integration code. |
| `DeletePipe` | `DELETE /v1/pipes/{Name}` | `idempotent` | `Name` | - | `DeletePipeResponse` | `ConflictException`, `InternalException`, `NotFoundException`, `ThrottlingException`, `ValidationException` | Delete an existing pipe. For more information about pipes, see Amazon EventBridge Pipes in the Amazon EventBridge User Guide. |
| `DescribePipe` | `GET /v1/pipes/{Name}` | `readonly` | `Name` | - | `DescribePipeResponse` | `InternalException`, `NotFoundException`, `ThrottlingException`, `ValidationException` | Get the information about an existing pipe. For more information about pipes, see Amazon EventBridge Pipes in the Amazon EventBridge User Guide. |
| `ListPipes` | `GET /v1/pipes` | `readonly`, `paginated` | - | - | `ListPipesResponse` | `InternalException`, `ThrottlingException`, `ValidationException` | Get the pipes associated with this account. For more information about pipes, see Amazon EventBridge Pipes in the Amazon EventBridge User Guide. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalException`, `NotFoundException`, `ValidationException` | Displays the tags associated with a pipe. |
| `StartPipe` | `POST /v1/pipes/{Name}/start` | - | `Name` | - | `StartPipeResponse` | `ConflictException`, `InternalException`, `NotFoundException`, `ThrottlingException`, `ValidationException` | Start an existing pipe. |
| `StopPipe` | `POST /v1/pipes/{Name}/stop` | - | `Name` | - | `StopPipeResponse` | `ConflictException`, `InternalException`, `NotFoundException`, `ThrottlingException`, `ValidationException` | Stop an existing pipe. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalException`, `NotFoundException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified pipe. Tags can help you organize and categorize your resources. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalException`, `NotFoundException`, `ValidationException` | Removes one or more tags from the specified pipes. |
| `UpdatePipe` | `PUT /v1/pipes/{Name}` | `idempotent` | `Name`, `RoleArn` | - | `UpdatePipeResponse` | `ConflictException`, `InternalException`, `NotFoundException`, `ThrottlingException`, `ValidationException` | Update an existing pipe. When you call `UpdatePipe`, EventBridge only the updates fields you have specified in the request; the rest remain unchanged. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalException` | `structure` | `message`, `retryAfterSeconds` | This exception occurs due to unexpected causes. |
| `ValidationException` | `structure` | `fieldList`, `message` | Indicates that an error has occurred while performing a validate operation. |
| `NotFoundException` | `structure` | `message` | An entity that you specified does not exist. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | An action was throttled. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | An action you attempted resulted in an exception. |
| `CreatePipeRequest` | `structure` | `Description`, `DesiredState`, `Enrichment`, `EnrichmentParameters`, `KmsKeyIdentifier`, `LogConfiguration`, `Name`, `RoleArn`, `Source`, `SourceParameters`, `Tags`, `Target`, ... (+1) | - |
| `CreatePipeResponse` | `structure` | `Arn`, `CreationTime`, `CurrentState`, `DesiredState`, `LastModifiedTime`, `Name` | - |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | A quota has been exceeded. |
| `DeletePipeRequest` | `structure` | `Name` | - |
| `DeletePipeResponse` | `structure` | `Arn`, `CreationTime`, `CurrentState`, `DesiredState`, `LastModifiedTime`, `Name` | - |
| `DescribePipeRequest` | `structure` | `Name` | - |
| `DescribePipeResponse` | `structure` | `Arn`, `CreationTime`, `CurrentState`, `Description`, `DesiredState`, `Enrichment`, `EnrichmentParameters`, `KmsKeyIdentifier`, `LastModifiedTime`, `LogConfiguration`, `Name`, `RoleArn`, ... (+6) | - |
| `ListPipesRequest` | `structure` | `CurrentState`, `DesiredState`, `Limit`, `NamePrefix`, `NextToken`, `SourcePrefix`, `TargetPrefix` | - |
| `ListPipesResponse` | `structure` | `NextToken`, `Pipes` | - |
| `ListTagsForResourceRequest` | `structure` | `resourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |
| `StartPipeRequest` | `structure` | `Name` | - |
| `StartPipeResponse` | `structure` | `Arn`, `CreationTime`, `CurrentState`, `DesiredState`, `LastModifiedTime`, `Name` | - |
| `StopPipeRequest` | `structure` | `Name` | - |
| `StopPipeResponse` | `structure` | `Arn`, `CreationTime`, `CurrentState`, `DesiredState`, `LastModifiedTime`, `Name` | - |
| `TagResourceRequest` | `structure` | `resourceArn`, `tags` | - |
| `TagResourceResponse` | `structure` | - | - |
| `UntagResourceRequest` | `structure` | `resourceArn`, `tagKeys` | - |
| `UntagResourceResponse` | `structure` | - | - |

## Winterbaume LTM Notes

Sources: .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises the highest-value Pipes source paths. Open it for DynamoDB Streams, Kinesis streams, and SQS queue sources.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises shared source-adapter boundaries. Open it before implementing a pipe source as a one-off bridge.
- Service implication: source-adapter behaviour should be reusable across DynamoDB Streams, Kinesis, and SQS where AWS contracts align, while preserving each source's event payload and cursor semantics.
- Service implication: Pipes work should be tested as a cross-service flow, not only as control-plane CRUD over pipe definitions.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
