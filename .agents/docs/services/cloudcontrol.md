# AWS Cloud Control API

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

For more information about Amazon Web Services Cloud Control API, see the Amazon Web Services Cloud Control API User Guide.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Cloud Control API workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Cancel`, `Create`, `Delete` operation families, including `GetResource`, `GetResourceRequestStatus`, `ListResourceRequests`, `ListResources`, `CancelResourceRequest`, `CreateResource`.

## Service Identity and Protocol

- AWS model slug: `cloudcontrol`
- AWS SDK for Rust slug: `cloudcontrol`
- Model version: `2021-09-30`
- Model file: `vendor/api-models-aws/models/cloudcontrol/service/2021-09-30/cloudcontrol-2021-09-30.json`
- SDK ID: `CloudControl`
- Endpoint prefix: `cloudcontrolapi`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `cloudcontrolapi.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (2), `List` (2), `Cancel` (1), `Create` (1), `Delete` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelResourceRequest`, `CreateResource`, `DeleteResource`, `UpdateResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetResource`, `GetResourceRequestStatus`, `ListResourceRequests`, `ListResources`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelResourceRequest`.
- 7 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_CreateResource.html
- https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html
- https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_CancelResourceRequest.html

Research outcomes:
- Cloud Control API provides a consistent create, read, update, delete, and list interface over supported CloudFormation resource types.
- Mutating operations such as `CreateResource`, `UpdateResource`, and `DeleteResource` return a progress event rather than completing the resource operation synchronously.
- `GetResourceRequestStatus` returns the current status of a resource operation request.
- Resource operations use type names, desired state documents, identifiers, and client tokens.
- `CancelResourceRequest` attempts to cancel an in-progress resource operation request.
- Progress events can include operation status, error code, event time, resource model, and callback context depending on operation state.
- Cloud Control behaviour depends on the underlying resource type handler contract, including stabilisation and handler errors.

Parity implications:
- Model resource type names, resource identifiers, desired state, operation requests, progress events, client tokens, cancellation, and handler status separately.
- Mutating operations should be asynchronous and observable through request status polling.
- Operation failures should surface handler-style error codes rather than generic service errors where possible.

## Operation Groups

### Get

- Operations: `GetResource`, `GetResourceRequestStatus`
- Common required input members in this group: `Identifier`, `RequestToken`, `TypeName`

### List

- Operations: `ListResourceRequests`, `ListResources`
- Traits: `paginated` (2)
- Common required input members in this group: `TypeName`

### Cancel

- Operations: `CancelResourceRequest`
- Traits: `idempotent` (1)
- Common required input members in this group: `RequestToken`

### Create

- Operations: `CreateResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `DesiredState`, `TypeName`

### Delete

- Operations: `DeleteResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Identifier`, `TypeName`

### Update

- Operations: `UpdateResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Identifier`, `PatchDocument`, `TypeName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelResourceRequest` | - | `idempotent` | `RequestToken` | - | `CancelResourceRequestOutput` | `ConcurrentModificationException`, `RequestTokenNotFoundException` | Cancels the specified resource operation request. For more information, see Canceling resource operation requests in the Amazon Web Services Cloud Control API User Guide . |
| `CreateResource` | - | `idempotency-token` | `DesiredState`, `TypeName` | `ClientToken` | `CreateResourceOutput` | `AlreadyExistsException`, `ClientTokenConflictException`, `ConcurrentOperationException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, ... (+11) | Creates the specified resource. For more information, see Creating a resource in the Amazon Web Services Cloud Control API User Guide . |
| `DeleteResource` | - | `idempotency-token` | `Identifier`, `TypeName` | `ClientToken` | `DeleteResourceOutput` | `AlreadyExistsException`, `ClientTokenConflictException`, `ConcurrentOperationException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, ... (+11) | Deletes the specified resource. For details, see Deleting a resource in the Amazon Web Services Cloud Control API User Guide . |
| `GetResource` | - | - | `Identifier`, `TypeName` | - | `GetResourceOutput` | `AlreadyExistsException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, `NetworkFailureException`, `NotStabilizedException`, ... (+9) | Returns information about the current state of the specified resource. For details, see Reading a resource's current state. |
| `GetResourceRequestStatus` | - | - | `RequestToken` | - | `GetResourceRequestStatusOutput` | `RequestTokenNotFoundException` | Returns the current status of a resource operation request. For more information, see Tracking the progress of resource operation requests in the Amazon Web Services Cloud Control API User Guide . |
| `ListResourceRequests` | - | `paginated` | - | - | `ListResourceRequestsOutput` | - | Returns existing resource operation requests. This includes requests of all status types. |
| `ListResources` | - | `paginated` | `TypeName` | - | `ListResourcesOutput` | `AlreadyExistsException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, `NetworkFailureException`, `NotStabilizedException`, ... (+9) | Returns information about the specified resources. For more information, see Discovering resources in the Amazon Web Services Cloud Control API User Guide . |
| `UpdateResource` | - | `idempotency-token` | `Identifier`, `PatchDocument`, `TypeName` | `ClientToken` | `UpdateResourceOutput` | `AlreadyExistsException`, `ClientTokenConflictException`, `ConcurrentOperationException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, ... (+11) | Updates the specified property values in the resource. You specify your resource property updates as a list of patch operations contained in a JSON patch document that adheres to the RFC 6902 - JavaScript Object Notation (JSON) Patch standard. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AlreadyExistsException` | `structure` | `Message` | The resource with the name requested already exists. |
| `GeneralServiceException` | `structure` | `Message` | The resource handler has returned that the downstream service generated an error that doesn't map to any other handler error code. |
| `HandlerFailureException` | `structure` | `Message` | The resource handler has failed without a returning a more specific error code. |
| `HandlerInternalFailureException` | `structure` | `Message` | The resource handler has returned that an unexpected error occurred within the resource handler. |
| `InvalidCredentialsException` | `structure` | `Message` | The resource handler has returned that the credentials provided by the user are invalid. |
| `InvalidRequestException` | `structure` | `Message` | The resource handler has returned that invalid input from the user has generated a generic exception. |
| `NetworkFailureException` | `structure` | `Message` | The resource handler has returned that the request couldn't be completed due to networking issues, such as a failure to receive a response from the server. |
| `NotStabilizedException` | `structure` | `Message` | The resource handler has returned that the downstream resource failed to complete all of its ready-state checks. |
| `NotUpdatableException` | `structure` | `Message` | One or more properties included in this resource operation are defined as create-only, and therefore can't be updated. |
| `PrivateTypeException` | `structure` | `Message` | Cloud Control API hasn't received a valid response from the resource handler, due to a configuration error. |
| `ResourceConflictException` | `structure` | `Message` | The resource is temporarily unavailable to be acted upon. |
| `ResourceNotFoundException` | `structure` | `Message` | A resource with the specified identifier can't be found. |
| `ServiceInternalErrorException` | `structure` | `Message` | The resource handler has returned that the downstream service returned an internal error, typically with a `5XX HTTP` status code. |
| `ServiceLimitExceededException` | `structure` | `Message` | The resource handler has returned that a non-transient resource limit was reached on the service side. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `TypeNotFoundException` | `structure` | `Message` | The specified extension doesn't exist in the CloudFormation registry. |
| `UnsupportedActionException` | `structure` | `Message` | The specified resource doesn't support this resource operation. |
| `ClientTokenConflictException` | `structure` | `Message` | The specified client token has already been used in another resource request. |
| `ConcurrentOperationException` | `structure` | `Message` | Another resource operation is currently being performed on this resource. |
| `RequestTokenNotFoundException` | `structure` | `Message` | A resource operation with the specified request token can't be found. |
| `CancelResourceRequestInput` | `structure` | `RequestToken` | - |
| `CancelResourceRequestOutput` | `structure` | `ProgressEvent` | - |
| `ConcurrentModificationException` | `structure` | `Message` | The resource is currently being modified by another operation. |
| `CreateResourceInput` | `structure` | `ClientToken`, `DesiredState`, `RoleArn`, `TypeName`, `TypeVersionId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
