# Amazon Augmented AI Runtime

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Augmented AI (Amazon A2I) adds the benefit of human judgment to any machine learning application. When an AI application can't evaluate data with a high degree of confidence, human reviewers can take over. This human review is called a human review workflow. To create and start a human review workflow, you need three resources: a worker task template , a flow definition , and a human loop . For information about these resources and prerequisites for using Amazon A2I, see Get Started with Amazon Augmented AI in the Amazon SageMaker Developer Guide.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Augmented AI Runtime workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Describe`, `List`, `Start`, `Stop` operation families, including `DeleteHumanLoop`, `DescribeHumanLoop`, `ListHumanLoops`, `StartHumanLoop`, `StopHumanLoop`.

## Service Identity and Protocol

- AWS model slug: `sagemaker-a2i-runtime`
- AWS SDK for Rust slug: `sagemaker`
- Model version: `2019-11-07`
- Model file: `vendor/api-models-aws/models/sagemaker-a2i-runtime/service/2019-11-07/sagemaker-a2i-runtime-2019-11-07.json`
- SDK ID: `SageMaker A2I Runtime`
- Endpoint prefix: `a2i-runtime.sagemaker`
- ARN namespace: `sagemaker`
- CloudFormation name: `SageMakerA2IRuntime`
- CloudTrail event source: `sagemakera2iruntime.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (1), `Describe` (1), `List` (1), `Start` (1), `Stop` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteHumanLoop`, `StartHumanLoop`, `StopHumanLoop`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeHumanLoop`, `ListHumanLoops`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartHumanLoop`, `StopHumanLoop`.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `ECS`.

## Operation Groups

### Delete

- Operations: `DeleteHumanLoop`
- Common required input members in this group: `HumanLoopName`

### Describe

- Operations: `DescribeHumanLoop`
- Common required input members in this group: `HumanLoopName`

### List

- Operations: `ListHumanLoops`
- Traits: `paginated` (1)
- Common required input members in this group: `FlowDefinitionArn`

### Start

- Operations: `StartHumanLoop`
- Common required input members in this group: `FlowDefinitionArn`, `HumanLoopInput`, `HumanLoopName`

### Stop

- Operations: `StopHumanLoop`
- Common required input members in this group: `HumanLoopName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteHumanLoop` | `DELETE /human-loops/{HumanLoopName}` | - | `HumanLoopName` | - | `DeleteHumanLoopResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified human loop for a flow definition. If the human loop was deleted, this operation will return a `ResourceNotFoundException`. |
| `DescribeHumanLoop` | `GET /human-loops/{HumanLoopName}` | - | `HumanLoopName` | - | `DescribeHumanLoopResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the specified human loop. If the human loop was deleted, this operation will return a `ResourceNotFoundException` error. |
| `ListHumanLoops` | `GET /human-loops` | `paginated` | `FlowDefinitionArn` | - | `ListHumanLoopsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about human loops, given the specified parameters. If a human loop was deleted, it will not be included. |
| `StartHumanLoop` | `POST /human-loops` | - | `FlowDefinitionArn`, `HumanLoopInput`, `HumanLoopName` | - | `StartHumanLoopResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a human loop, provided that at least one activation condition is met. |
| `StopHumanLoop` | `POST /human-loops/stop` | - | `HumanLoopName` | - | `StopHumanLoopResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops the specified human loop. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | We couldn't process your request because of an issue with the server. |
| `ThrottlingException` | `structure` | `Message` | You exceeded the maximum number of requests. |
| `ValidationException` | `structure` | `Message` | The request isn't valid. |
| `ResourceNotFoundException` | `structure` | `Message` | We couldn't find the requested resource. |
| `DeleteHumanLoopRequest` | `structure` | `HumanLoopName` | - |
| `DeleteHumanLoopResponse` | `structure` | - | - |
| `DescribeHumanLoopRequest` | `structure` | `HumanLoopName` | - |
| `DescribeHumanLoopResponse` | `structure` | `CreationTime`, `FailureCode`, `FailureReason`, `FlowDefinitionArn`, `HumanLoopArn`, `HumanLoopName`, `HumanLoopOutput`, `HumanLoopStatus` | - |
| `ListHumanLoopsRequest` | `structure` | `CreationTimeAfter`, `CreationTimeBefore`, `FlowDefinitionArn`, `MaxResults`, `NextToken`, `SortOrder` | - |
| `ListHumanLoopsResponse` | `structure` | `HumanLoopSummaries`, `NextToken` | - |
| `StartHumanLoopRequest` | `structure` | `DataAttributes`, `FlowDefinitionArn`, `HumanLoopInput`, `HumanLoopName` | - |
| `StartHumanLoopResponse` | `structure` | `HumanLoopArn` | - |
| `ConflictException` | `structure` | `Message` | Your request has the same name as another active human loop but has different input data. |
| `ServiceQuotaExceededException` | `structure` | `Message` | You exceeded your service quota. |
| `StopHumanLoopRequest` | `structure` | `HumanLoopName` | - |
| `StopHumanLoopResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
