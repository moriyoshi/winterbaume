# AWS Amplify UI Builder

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amplify UI Builder API provides a programmatic interface for creating and configuring user interface (UI) component libraries and themes for use in your Amplify applications. You can then connect these UI components to an application's backend Amazon Web Services resources. You can also use the Amplify Studio visual designer to create UI components and model data for an app. For more information, see Introduction in the Amplify Docs . The Amplify Framework is a comprehensive set of SDKs, libraries, tools, and documentation for client app development.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-amplifyuibuilder/tests/scenario_test.rs`: manage feature flags through create, list, update, and delete operations.
- Backported from `scenario_test.rs`: exercise the token exchange and refresh workflow for stateless UI Builder authorisation operations.
- Backported from `scenario_test.rs`: create, update, and delete components, themes, and forms for an Amplify Studio project.
- Scenario insight from EC2: add full state-machine walks for AWS Amplify UI Builder resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: model generated front-end assets across apps and environments, including component catalogues, theme customisation, form schemas, and export/import workflows.

## Service Identity and Protocol

- AWS model slug: `amplifyuibuilder`
- AWS SDK for Rust slug: `amplifyuibuilder`
- Model version: `2021-08-11`
- Model file: `vendor/api-models-aws/models/amplifyuibuilder/service/2021-08-11/amplifyuibuilder-2021-08-11.json`
- SDK ID: `AmplifyUIBuilder`
- Endpoint prefix: `-`
- ARN namespace: `amplifyuibuilder`
- CloudFormation name: `AmplifyUIBuilder`
- CloudTrail event source: `amplifyuibuilder.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5), `List` (5), `Create` (3), `Delete` (3), `Export` (3), `Update` (3), `Exchange` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateComponent`, `CreateForm`, `CreateTheme`, `DeleteComponent`, `DeleteForm`, `DeleteTheme`, `PutMetadataFlag`, `StartCodegenJob`, `TagResource`, `UntagResource`, `UpdateComponent`, `UpdateForm`, `UpdateTheme`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `ExportComponents`, `ExportForms`, `ExportThemes`, `GetCodegenJob`, `GetComponent`, `GetForm`, `GetMetadata`, `GetTheme`, `ListCodegenJobs`, `ListComponents`, `ListForms`, `ListTagsForResource`, `ListThemes`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ExportComponents`, `ExportForms`, `ExportThemes`, `GetCodegenJob`, `ListCodegenJobs`, `StartCodegenJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 28 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EC2/VPC`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CodegenJobResource` | `appId`, `environmentName`, `id` | create: `StartCodegenJob`; read: `GetCodegenJob`; list: `ListCodegenJobs` | - | - |
| `ComponentResource` | `appId`, `environmentName`, `id` | create: `CreateComponent`; read: `GetComponent`; update: `UpdateComponent`; delete: `DeleteComponent`; list: `ListComponents` | `ExportComponents` | - |
| `FormResource` | `appId`, `environmentName`, `id` | create: `CreateForm`; read: `GetForm`; update: `UpdateForm`; delete: `DeleteForm`; list: `ListForms` | `ExportForms` | - |
| `ThemeResource` | `appId`, `environmentName`, `id` | create: `CreateTheme`; read: `GetTheme`; update: `UpdateTheme`; delete: `DeleteTheme`; list: `ListThemes` | `ExportThemes` | - |
## Operation Groups

### Exchange

- Operations: `ExchangeCodeForToken`
- Common required input members in this group: -

### Get

- Operations: `GetMetadata`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Put

- Operations: `PutMetadataFlag`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Refresh

- Operations: `RefreshToken`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ExchangeCodeForToken` | `POST /tokens/{provider}` | - | `provider`, `request` | - | `ExchangeCodeForTokenResponse` | `InvalidParameterException` | This is for internal use. Amplify uses this action to exchange an access code for a token. |
| `GetMetadata` | `GET /app/{appId}/environment/{environmentName}/metadata` | `readonly` | `appId`, `environmentName` | - | `GetMetadataResponse` | `InvalidParameterException`, `UnauthorizedException` | Returns existing metadata for an Amplify app. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Returns a list of tags for a specified Amazon Resource Name (ARN). |
| `PutMetadataFlag` | `PUT /app/{appId}/environment/{environmentName}/metadata/features/{featureName}` | `idempotent` | `appId`, `environmentName`, `featureName`, `body` | - | `Unit` | `InvalidParameterException`, `UnauthorizedException` | Stores the metadata information about a feature on a form. |
| `RefreshToken` | `POST /tokens/{provider}/refresh` | - | `provider`, `refreshTokenBody` | - | `RefreshTokenResponse` | `InvalidParameterException` | This is for internal use. Amplify uses this action to refresh a previously issued access token that might have expired. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Tags the resource with a tag key and value. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Untags a resource with a specified Amazon Resource Name (ARN). |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ExchangeCodeForToken` | - | - | - | `request` |
| `PutMetadataFlag` | - | - | - | `body` |
| `RefreshToken` | - | - | - | `refreshTokenBody` |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | message | An internal error has occurred. Please retry your request. |
| `InvalidParameterException` | `structure` | message | An invalid or out-of-range value was supplied for the input parameter. |
| `ResourceConflictException` | `structure` | message | The resource specified in the request conflicts with an existing resource. |
| `ResourceNotFoundException` | `structure` | message | The requested resource does not exist, or access was denied. |
| `ServiceQuotaExceededException` | `structure` | message | You exceeded your service quota. Service quotas, also referred to as limits, are the maximum number of service resources or operations for your Amazon Web S ... |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `UnauthorizedException` | `structure` | message | You don't have permission to perform this operation. |
| `ExchangeCodeForTokenRequest` | `structure` | provider, request | - |
| `ExchangeCodeForTokenResponse` | `structure` | accessToken, expiresIn, refreshToken | - |
| `GetMetadataRequest` | `structure` | appId, environmentName | - |
| `GetMetadataResponse` | `structure` | features | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `PutMetadataFlagRequest` | `structure` | appId, environmentName, featureName, body | - |
| `RefreshTokenRequest` | `structure` | provider, refreshTokenBody | - |
| `RefreshTokenResponse` | `structure` | accessToken, expiresIn | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `CodegenGenericDataFieldDataType` | `enum` | ID, STRING, INT, FLOAT, AWS_DATE, AWS_TIME, AWS_DATE_TIME, AWS_TIMESTAMP, AWS_EMAIL, AWS_URL, AWS_IP_ADDRESS, BOOLEAN, ... (+5) | - |
| `CodegenJobGenericDataSourceType` | `enum` | DATA_STORE | - |
| `CodegenJobStatus` | `enum` | IN_PROGRESS, FAILED, SUCCEEDED | - |
| `FixedPosition` | `enum` | FIRST | - |
| `FormActionType` | `enum` | CREATE, UPDATE | - |
| `FormButtonsPosition` | `enum` | TOP, BOTTOM, TOP_AND_BOTTOM | - |
| `GenericDataRelationshipType` | `enum` | HAS_MANY, HAS_ONE, BELONGS_TO | - |
| `JSModule` | `enum` | ES2020, ESNEXT | - |
| `JSScript` | `enum` | JSX, TSX, JS | - |
| `JSTarget` | `enum` | ES2015, ES2020 | - |
| `SortDirection` | `enum` | ASC, DESC | - |
| `StorageAccessLevel` | `enum` | PUBLIC, PROTECTED, PRIVATE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
