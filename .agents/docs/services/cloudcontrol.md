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

## Resource-Type Handler Contract

The `cloudcontrolapi` Smithy model types `DesiredState`, `ResourceModel`, and `ResourceDescription.Properties` as opaque JSON strings. It says nothing about what a particular resource type's read model looks like. That contract lives in a separate corpus: the per-type CloudFormation registry schema, published at `https://schema.cloudformation.us-east-1.amazonaws.com/aws-<service>-<type>.json` (for example `aws-ssm-parameter.json`). No codegen step in this repository reads that corpus, so per-type fidelity is hand-written and must be checked against the schema and, where the schema is silent, against a live-account capture.

Schema keys that govern the read model:

- `primaryIdentifier`: the property whose value becomes the Cloud Control `Identifier`. A resource must be retrievable by this value, never by a random UUID (issues #8 and #11 were that regression).
- `writeOnlyProperties`: accepted on create and update, never returned by `GetResource`, `ListResources`, or the progress event's `ResourceModel`.
- `readOnlyProperties`: generated by the service (ARNs, IDs, DNS names) and always returned.
- `createOnlyProperties`: changing one through `UpdateResource` is a replacement in CloudFormation and a rejection in Cloud Control.
- Read-handler defaults: many types report values on read that the schema does not declare as `default`. The DynamoDB, ECS, and ELBv2 read defaults all came from live-account captures in issues #7 to #11.

Winterbaume implements this contract in two ways, chosen per resource type:

1. **Shaped, Cloud Control-owned** (`crates/winterbaume-cloudcontrol/src/cfn_schema/`). A `CfnResourceShaper` strips `writeOnlyProperties`, generates `readOnlyProperties`, fills read-handler defaults, and returns the `primaryIdentifier`. The shaped model is stored in `CloudControlState` and is invisible to the owning service's own API. Registered types: `AWS::KMS::Key`, `AWS::DynamoDB::Table`, `AWS::ECS::Cluster`, `AWS::ElasticLoadBalancingV2::TargetGroup`, `AWS::ElasticLoadBalancingV2::LoadBalancer`, `AWS::ElasticLoadBalancingV2::Listener`.
2. **Service-backed** (see the next section). Create, update, and delete go to the owning service's state, and every read is built from that state. Implemented for `AWS::SSM::Parameter` (issue #33).

Any other type falls back to storing `DesiredState` verbatim, keyed by the first of `Id`, `Identifier`, `Name`, `Arn`, `BucketName`, `FunctionName`, `TableName` found in it, or by a random UUID. That fallback is a known fidelity gap, not intended behaviour; see `cloudcontrol-cfn-shapers-backfill` in `.agents/docs/TODO.md`.

Test triple for any shaped or backed type: (a) send at least one writeOnly property and assert it is absent from `GetResource`; (b) assert every readOnly property is present and well-formed; (c) assert each read-handler default appears when it was omitted from the input. `properties().is_some()` is not an assertion. moto is not a useful reference here, because its Cloud Control is also a pass-through.

## Cross-Service Resource Ownership

Real Cloud Control is a facade: it calls the resource type's handler, and the handler calls the owning service. A resource created through Cloud Control is an ordinary resource of that service, and a resource created through the service's own API is listable and readable through Cloud Control. A shaped-only implementation can get every field right and still fail that coherence: in issue #33, `ssm get-parameter` returned `ParameterNotFound` for a parameter that Cloud Control reported as created.

Service-backed types follow these rules:

- **The owning service holds the only copy of the resource.** `GetResource` and `ListResources` build the read model from the owning service's state on every call, so changes made through the service API (for example `ssm put-parameter --overwrite`, `ssm add-tags-to-resource`) appear immediately. `ListResources` therefore also returns resources that were never created through Cloud Control, as on AWS.
- **`CloudControlState` keeps only request bookkeeping** for these types: the `ResourceRequest` progress events returned by `GetResourceRequestStatus` and `ListResourceRequests`. `CloudControlState::record_operation` also writes a copy of the read model into `resources`, but nothing reads that copy for a backed type.
- **Shared state, not message passing.** `CloudControlService::with_ssm_service(Arc<SsmService>)` takes the same `SsmService` instance that is registered with `MockAws` or the server, and reaches its account- and region-scoped state through `SsmService::shared_state()`. `CloudControlService::new()` creates a private `SsmService`, so a standalone Cloud Control mock still works but is not coherent with any SSM API surface. `crates/winterbaume-server/src/main.rs` must construct Cloud Control with `with_ssm_service(Arc::clone(&ssm))`; constructing it with `new()` there silently reintroduces issue #33.
- **Lock order is Cloud Control state, then the owning service's state.** Backed mutations hold both write locks so that a failed SSM write is never recorded as a successful operation. SSM handlers only ever take the SSM lock, so the order cannot invert. Any future backed type must keep the same order.
- **Both services are notified.** After a successful mutation of a backed type, the Cloud Control dispatcher calls `notify_state_changed` on the owning service as well, so state-change subscribers of the owning service (snapshots, Terraform state sync) see writes that arrived through Cloud Control.
- **Restore both service views together.** SSM owns the live parameter contents, versions and tags. Restoring a Cloud Control resource record alone does not recreate an SSM parameter; restore the SSM view as well when preserving a complete session.
- **Errors keep Cloud Control's names.** Owning-service errors are translated into Cloud Control domain errors: `ParameterNotFound` becomes `ResourceNotFound` and `ParameterAlreadyExists` becomes `AlreadyExists`. Anything else becomes `InvalidRequest`.

### `AWS::SSM::Parameter`

Schema facts (checked against `aws-ssm-parameter.json` on 2026-10-27):

- `primaryIdentifier`: `Name`. `createOnlyProperties`: `Name`. `readOnlyProperties`: `Arn`. `writeOnlyProperties`: `Description`, `Tier`, `AllowedPattern`, `Policies`. `required`: `Type`, `Value`; `Name` is optional and generated when absent.
- `Type` enum is `String` and `StringList` only; `SecureString` cannot be created through this resource type. `DataType` enum is `text` and `aws:ec2:image`, with no schema `default`; the SSM service fills in `text`.
- `Tags` is a JSON object (`{"key": "value"}`), not the `[{"Key", "Value"}]` list most types use. Tagging is `tagOnCreate` and `tagUpdatable` through `ssm:AddTagsToResource` and `ssm:RemoveTagsFromResource`.

Winterbaume behaviour (`crates/winterbaume-cloudcontrol/src/ssm_parameter.rs`):

- The read model is exactly `Name`, `Type`, `Value`, `Arn`, `DataType`, `Tags`, built from the SSM `Parameter`. The four writeOnly properties are accepted and never returned.
- Create calls `SsmState::put_parameter` with `overwrite = false`, then sets the requested `DataType`. A parameter that already exists in SSM, however it was created, is rejected with `AlreadyExistsException`.
- An omitted `Name` is generated as `ssm-<uuid>`. Real CloudFormation and Cloud Control generate a differently formatted name; the format has not been captured from a live account, so clients must not depend on it.
- `UpdateResource` applies the patch to the current read model. It accepts only top-level paths plus `/Tags/<key>`, and only the `add`, `replace`, `remove`, and `test` operations. It rejects any change to `Name` or `Arn`, then writes a new SSM version only when `Value`, `Type`, or `DataType` changed. A tags-only update replaces the tag set without creating a new parameter version. The whole patch is validated before SSM is touched, so a rejected patch leaves the parameter unchanged.
- `DeleteResource` calls `SsmState::delete_parameter`, which also removes the version history and the tag set.

Known gaps:

- `Description`, `Tier`, `AllowedPattern`, and `Policies` are discarded rather than stored in SSM, because the SSM `Parameter` type has no fields for them. Real AWS stores them, so `ssm describe-parameters` would show a `Description` that winterbaume drops.
- `aws:ec2:image` values are not validated against EC2 images. Real SSM validates them asynchronously.

Tests: `crates/winterbaume-cloudcontrol/tests/ssm_parameter_test.rs` drives Cloud Control and SSM SDK clients against one `MockAws` instance. It covers the shared lifecycle in both directions, native SSM parameters read through Cloud Control, duplicate-create rejection, `DataType` defaulting and validation, and atomicity of a failed update.

Terraform E2E: `test_awscc_ssm_parameter_backed_by_ssm` in `crates/winterbaume-e2e-tests/tests/terraform/cloudcontrol.rs` drives `awscc_ssm_parameter` through the `awscc` provider, which is the only Terraform provider that goes through Cloud Control. It runs an isolated server where Cloud Control and the native SSM API share one `SsmService`, and checks each step against the native SSM API. The steps are: create, then confirm that SSM sees the value, `DataType`, and tags; a no-drift plan despite the write-only `Description`; drift detected after an out-of-band `PutParameter`; an in-place value and tag update that reaches SSM; and destroy, after which SSM returns `ParameterNotFound`. Run it with `cargo test -p winterbaume-e2e-tests --test terraform cloudcontrol -- --ignored`.

## Operation Groups

### Get

- Operations: `GetResource`, `GetResourceRequestStatus`
- Common required input members in this group: -

### List

- Operations: `ListResourceRequests`, `ListResources`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Cancel

- Operations: `CancelResourceRequest`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Delete

- Operations: `DeleteResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Update

- Operations: `UpdateResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelResourceRequest` | `-` | `idempotent` | `RequestToken` | - | `CancelResourceRequestOutput` | `ConcurrentModificationException`, `RequestTokenNotFoundException` | Cancels the specified resource operation request. For more information, see Canceling resource operation requests in the Amazon Web Services Cloud Control API User Guide . Only resource operations requests with a sta ... |
| `CreateResource` | `-` | `idempotency-token` | `TypeName`, `DesiredState` | `ClientToken` | `CreateResourceOutput` | `AlreadyExistsException`, `ClientTokenConflictException`, `ConcurrentOperationException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, `NetworkFailureException`, `NotStabilizedException`, `NotUpdatableException`, `PrivateTypeException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceInternalErrorException`, `ServiceLimitExceededException`, `ThrottlingException`, `TypeNotFoundException`, `UnsupportedActionException` | Creates the specified resource. For more information, see Creating a resource in the Amazon Web Services Cloud Control API User Guide . After you have initiated a resource creation request, you can monitor the progre ... |
| `DeleteResource` | `-` | `idempotency-token` | `TypeName`, `Identifier` | `ClientToken` | `DeleteResourceOutput` | `AlreadyExistsException`, `ClientTokenConflictException`, `ConcurrentOperationException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, `NetworkFailureException`, `NotStabilizedException`, `NotUpdatableException`, `PrivateTypeException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceInternalErrorException`, `ServiceLimitExceededException`, `ThrottlingException`, `TypeNotFoundException`, `UnsupportedActionException` | Deletes the specified resource. For details, see Deleting a resource in the Amazon Web Services Cloud Control API User Guide . After you have initiated a resource deletion request, you can monitor the progress of you ... |
| `GetResource` | `-` | - | `TypeName`, `Identifier` | - | `GetResourceOutput` | `AlreadyExistsException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, `NetworkFailureException`, `NotStabilizedException`, `NotUpdatableException`, `PrivateTypeException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceInternalErrorException`, `ServiceLimitExceededException`, `ThrottlingException`, `TypeNotFoundException`, `UnsupportedActionException` | Returns information about the current state of the specified resource. For details, see Reading a resource's current state . You can use this action to return information about an existing resource in your account an ... |
| `GetResourceRequestStatus` | `-` | - | `RequestToken` | - | `GetResourceRequestStatusOutput` | `RequestTokenNotFoundException` | Returns the current status of a resource operation request. For more information, see Tracking the progress of resource operation requests in the Amazon Web Services Cloud Control API User Guide . |
| `ListResourceRequests` | `-` | `paginated` | - | - | `ListResourceRequestsOutput` | - | Returns existing resource operation requests. This includes requests of all status types. For more information, see Listing active resource operation requests in the Amazon Web Services Cloud Control API User Guide . ... |
| `ListResources` | `-` | `paginated` | `TypeName` | - | `ListResourcesOutput` | `AlreadyExistsException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, `NetworkFailureException`, `NotStabilizedException`, `NotUpdatableException`, `PrivateTypeException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceInternalErrorException`, `ServiceLimitExceededException`, `ThrottlingException`, `TypeNotFoundException`, `UnsupportedActionException` | Returns information about the specified resources. For more information, see Discovering resources in the Amazon Web Services Cloud Control API User Guide . You can use this action to return information about existin ... |
| `UpdateResource` | `-` | `idempotency-token` | `TypeName`, `Identifier`, `PatchDocument` | `ClientToken` | `UpdateResourceOutput` | `AlreadyExistsException`, `ClientTokenConflictException`, `ConcurrentOperationException`, `GeneralServiceException`, `HandlerFailureException`, `HandlerInternalFailureException`, `InvalidCredentialsException`, `InvalidRequestException`, `NetworkFailureException`, `NotStabilizedException`, `NotUpdatableException`, `PrivateTypeException`, `ResourceConflictException`, `ResourceNotFoundException`, `ServiceInternalErrorException`, `ServiceLimitExceededException`, `ThrottlingException`, `TypeNotFoundException`, `UnsupportedActionException` | Updates the specified property values in the resource. You specify your resource property updates as a list of patch operations contained in a JSON patch document that adheres to the RFC 6902 - JavaScript Object Nota ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AlreadyExistsException` | `structure` | Message | The resource with the name requested already exists. |
| `ClientTokenConflictException` | `structure` | Message | The specified client token has already been used in another resource request. It's best practice for client tokens to be unique for each resource operation ... |
| `ConcurrentModificationException` | `structure` | Message | The resource is currently being modified by another operation. |
| `ConcurrentOperationException` | `structure` | Message | Another resource operation is currently being performed on this resource. |
| `GeneralServiceException` | `structure` | Message | The resource handler has returned that the downstream service generated an error that doesn't map to any other handler error code. |
| `HandlerFailureException` | `structure` | Message | The resource handler has failed without a returning a more specific error code. This can include timeouts. |
| `HandlerInternalFailureException` | `structure` | Message | The resource handler has returned that an unexpected error occurred within the resource handler. |
| `InvalidCredentialsException` | `structure` | Message | The resource handler has returned that the credentials provided by the user are invalid. |
| `InvalidRequestException` | `structure` | Message | The resource handler has returned that invalid input from the user has generated a generic exception. |
| `NetworkFailureException` | `structure` | Message | The resource handler has returned that the request couldn't be completed due to networking issues, such as a failure to receive a response from the server. |
| `NotStabilizedException` | `structure` | Message | The resource handler has returned that the downstream resource failed to complete all of its ready-state checks. |
| `NotUpdatableException` | `structure` | Message | One or more properties included in this resource operation are defined as create-only, and therefore can't be updated. |
| `PrivateTypeException` | `structure` | Message | Cloud Control API hasn't received a valid response from the resource handler, due to a configuration error. This includes issues such as the resource handle ... |
| `RequestTokenNotFoundException` | `structure` | Message | A resource operation with the specified request token can't be found. |
| `ResourceConflictException` | `structure` | Message | The resource is temporarily unavailable to be acted upon. For example, if the resource is currently undergoing an operation and can't be acted upon until th ... |
| `ResourceNotFoundException` | `structure` | Message | A resource with the specified identifier can't be found. |
| `ServiceInternalErrorException` | `structure` | Message | The resource handler has returned that the downstream service returned an internal error, typically with a 5XX HTTP status code. |
| `ServiceLimitExceededException` | `structure` | Message | The resource handler has returned that a non-transient resource limit was reached on the service side. |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. |
| `TypeNotFoundException` | `structure` | Message | The specified extension doesn't exist in the CloudFormation registry. |
| `UnsupportedActionException` | `structure` | Message | The specified resource doesn't support this resource operation. |
| `CancelResourceRequestInput` | `structure` | RequestToken | - |
| `CancelResourceRequestOutput` | `structure` | ProgressEvent | - |
| `CreateResourceInput` | `structure` | TypeName, TypeVersionId, RoleArn, ClientToken, DesiredState | - |
| `CreateResourceOutput` | `structure` | ProgressEvent | - |
| `DeleteResourceInput` | `structure` | TypeName, TypeVersionId, RoleArn, ClientToken, Identifier | - |
| `DeleteResourceOutput` | `structure` | ProgressEvent | - |
| `GetResourceInput` | `structure` | TypeName, TypeVersionId, RoleArn, Identifier | - |
| `GetResourceOutput` | `structure` | TypeName, ResourceDescription | - |
| `GetResourceRequestStatusInput` | `structure` | RequestToken | - |
| `GetResourceRequestStatusOutput` | `structure` | ProgressEvent, HooksProgressEvent | - |
| `ListResourceRequestsInput` | `structure` | MaxResults, NextToken, ResourceRequestStatusFilter | - |
| `ListResourceRequestsOutput` | `structure` | ResourceRequestStatusSummaries, NextToken | - |
| `ListResourcesInput` | `structure` | TypeName, TypeVersionId, RoleArn, NextToken, MaxResults, ResourceModel | - |
| `ListResourcesOutput` | `structure` | TypeName, ResourceDescriptions, NextToken | - |
| `UpdateResourceInput` | `structure` | TypeName, TypeVersionId, RoleArn, ClientToken, Identifier, PatchDocument | - |
| `UpdateResourceOutput` | `structure` | ProgressEvent | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
- For each resource type, fetch its CloudFormation registry schema and confirm `primaryIdentifier`, `readOnlyProperties`, `writeOnlyProperties`, `createOnlyProperties`, and the read-handler defaults before implementing it.
- For each resource type, decide whether it is shaped (Cloud Control-owned) or service-backed. If the owning service has a winterbaume crate, confirm that a resource created through Cloud Control is visible through that service's API, and the reverse.
