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

### Get

- Operations: `GetCodegenJob`, `GetComponent`, `GetForm`, `GetMetadata`, `GetTheme`
- Traits: `readonly` (5)
- Common required input members in this group: `appId`, `environmentName`, `id`

### List

- Operations: `ListCodegenJobs`, `ListComponents`, `ListForms`, `ListTagsForResource`, `ListThemes`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `appId`, `environmentName`, `resourceArn`

### Create

- Operations: `CreateComponent`, `CreateForm`, `CreateTheme`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `appId`, `componentToCreate`, `environmentName`, `formToCreate`, `themeToCreate`

### Delete

- Operations: `DeleteComponent`, `DeleteForm`, `DeleteTheme`
- Traits: `idempotent` (3)
- Common required input members in this group: `appId`, `environmentName`, `id`

### Export

- Operations: `ExportComponents`, `ExportForms`, `ExportThemes`
- Traits: `paginated` (3), `readonly` (3)
- Common required input members in this group: `appId`, `environmentName`

### Update

- Operations: `UpdateComponent`, `UpdateForm`, `UpdateTheme`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `appId`, `environmentName`, `id`, `updatedComponent`, `updatedForm`, `updatedTheme`

### Exchange

- Operations: `ExchangeCodeForToken`
- Common required input members in this group: `provider`, `request`

### Put

- Operations: `PutMetadataFlag`
- Traits: `idempotent` (1)
- Common required input members in this group: `appId`, `body`, `environmentName`, `featureName`

### Refresh

- Operations: `RefreshToken`
- Common required input members in this group: `provider`, `refreshTokenBody`

### Start

- Operations: `StartCodegenJob`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `appId`, `codegenJobToCreate`, `environmentName`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateComponent` | `POST /app/{appId}/environment/{environmentName}/components` | `idempotent`, `idempotency-token` | `appId`, `componentToCreate`, `environmentName` | `clientToken` | `CreateComponentResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceConflictException`, `ServiceQuotaExceededException` | Creates a new component for an Amplify app. |
| `CreateForm` | `POST /app/{appId}/environment/{environmentName}/forms` | `idempotent`, `idempotency-token` | `appId`, `environmentName`, `formToCreate` | `clientToken` | `CreateFormResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceConflictException`, `ServiceQuotaExceededException` | Creates a new form for an Amplify app. |
| `CreateTheme` | `POST /app/{appId}/environment/{environmentName}/themes` | `idempotent`, `idempotency-token` | `appId`, `environmentName`, `themeToCreate` | `clientToken` | `CreateThemeResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceConflictException`, `ServiceQuotaExceededException` | Creates a theme to apply to the components in an Amplify app. |
| `DeleteComponent` | `DELETE /app/{appId}/environment/{environmentName}/components/{id}` | `idempotent` | `appId`, `environmentName`, `id` | - | `Unit` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException` | Deletes a component from an Amplify app. |
| `DeleteForm` | `DELETE /app/{appId}/environment/{environmentName}/forms/{id}` | `idempotent` | `appId`, `environmentName`, `id` | - | `Unit` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException` | Deletes a form from an Amplify app. |
| `DeleteTheme` | `DELETE /app/{appId}/environment/{environmentName}/themes/{id}` | `idempotent` | `appId`, `environmentName`, `id` | - | `Unit` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException` | Deletes a theme from an Amplify app. |
| `ExchangeCodeForToken` | `POST /tokens/{provider}` | - | `provider`, `request` | - | `ExchangeCodeForTokenResponse` | `InvalidParameterException` | This is for internal use. Amplify uses this action to exchange an access code for a token. |
| `ExportComponents` | `GET /export/app/{appId}/environment/{environmentName}/components` | `readonly`, `paginated` | `appId`, `environmentName` | - | `ExportComponentsResponse` | `InternalServerException`, `InvalidParameterException` | Exports component configurations to code that is ready to integrate into an Amplify app. |
| `ExportForms` | `GET /export/app/{appId}/environment/{environmentName}/forms` | `readonly`, `paginated` | `appId`, `environmentName` | - | `ExportFormsResponse` | `InternalServerException`, `InvalidParameterException` | Exports form configurations to code that is ready to integrate into an Amplify app. |
| `ExportThemes` | `GET /export/app/{appId}/environment/{environmentName}/themes` | `readonly`, `paginated` | `appId`, `environmentName` | - | `ExportThemesResponse` | `InternalServerException`, `InvalidParameterException` | Exports theme configurations to code that is ready to integrate into an Amplify app. |
| `GetCodegenJob` | `GET /app/{appId}/environment/{environmentName}/codegen-jobs/{id}` | `readonly` | `appId`, `environmentName`, `id` | - | `GetCodegenJobResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException` | Returns an existing code generation job. |
| `GetComponent` | `GET /app/{appId}/environment/{environmentName}/components/{id}` | `readonly` | `appId`, `environmentName`, `id` | - | `GetComponentResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns an existing component for an Amplify app. |
| `GetForm` | `GET /app/{appId}/environment/{environmentName}/forms/{id}` | `readonly` | `appId`, `environmentName`, `id` | - | `GetFormResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns an existing form for an Amplify app. |
| `GetMetadata` | `GET /app/{appId}/environment/{environmentName}/metadata` | `readonly` | `appId`, `environmentName` | - | `GetMetadataResponse` | `InvalidParameterException`, `UnauthorizedException` | Returns existing metadata for an Amplify app. |
| `GetTheme` | `GET /app/{appId}/environment/{environmentName}/themes/{id}` | `readonly` | `appId`, `environmentName`, `id` | - | `GetThemeResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns an existing theme for an Amplify app. |
| `ListCodegenJobs` | `GET /app/{appId}/environment/{environmentName}/codegen-jobs` | `readonly`, `paginated` | `appId`, `environmentName` | - | `ListCodegenJobsResponse` | `InternalServerException`, `InvalidParameterException`, `ThrottlingException` | Retrieves a list of code generation jobs for a specified Amplify app and backend environment. |
| `ListComponents` | `GET /app/{appId}/environment/{environmentName}/components` | `readonly`, `paginated` | `appId`, `environmentName` | - | `ListComponentsResponse` | `InternalServerException`, `InvalidParameterException` | Retrieves a list of components for a specified Amplify app and backend environment. |
| `ListForms` | `GET /app/{appId}/environment/{environmentName}/forms` | `readonly`, `paginated` | `appId`, `environmentName` | - | `ListFormsResponse` | `InternalServerException`, `InvalidParameterException` | Retrieves a list of forms for a specified Amplify app and backend environment. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Returns a list of tags for a specified Amazon Resource Name (ARN). |
| `ListThemes` | `GET /app/{appId}/environment/{environmentName}/themes` | `readonly`, `paginated` | `appId`, `environmentName` | - | `ListThemesResponse` | `InternalServerException`, `InvalidParameterException` | Retrieves a list of themes for a specified Amplify app and backend environment. |
| `PutMetadataFlag` | `PUT /app/{appId}/environment/{environmentName}/metadata/features/{featureName}` | `idempotent` | `appId`, `body`, `environmentName`, `featureName` | - | `Unit` | `InvalidParameterException`, `UnauthorizedException` | Stores the metadata information about a feature on a form. |
| `RefreshToken` | `POST /tokens/{provider}/refresh` | - | `provider`, `refreshTokenBody` | - | `RefreshTokenResponse` | `InvalidParameterException` | This is for internal use. Amplify uses this action to refresh a previously issued access token that might have expired. |
| `StartCodegenJob` | `POST /app/{appId}/environment/{environmentName}/codegen-jobs` | `idempotency-token` | `appId`, `codegenJobToCreate`, `environmentName` | `clientToken` | `StartCodegenJobResponse` | `InternalServerException`, `InvalidParameterException`, `ThrottlingException` | Starts a code generation job for a specified Amplify app and backend environment. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Tags the resource with a tag key and value. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Untags a resource with a specified Amazon Resource Name (ARN). |
| `UpdateComponent` | `PATCH /app/{appId}/environment/{environmentName}/components/{id}` | `idempotent`, `idempotency-token` | `appId`, `environmentName`, `id`, `updatedComponent` | `clientToken` | `UpdateComponentResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceConflictException` | Updates an existing component. |
| `UpdateForm` | `PATCH /app/{appId}/environment/{environmentName}/forms/{id}` | `idempotent`, `idempotency-token` | `appId`, `environmentName`, `id`, `updatedForm` | `clientToken` | `UpdateFormResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceConflictException` | Updates an existing form. |
| `UpdateTheme` | `PATCH /app/{appId}/environment/{environmentName}/themes/{id}` | `idempotent`, `idempotency-token` | `appId`, `environmentName`, `id`, `updatedTheme` | `clientToken` | `UpdateThemeResponse` | `InternalServerException`, `InvalidParameterException`, `ResourceConflictException` | Updates an existing theme. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidParameterException` | `structure` | `message` | An invalid or out-of-range value was supplied for the input parameter. |
| `InternalServerException` | `structure` | `message` | An internal error has occurred. |
| `ResourceNotFoundException` | `structure` | `message` | The requested resource does not exist, or access was denied. |
| `ResourceConflictException` | `structure` | `message` | The resource specified in the request conflicts with an existing resource. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `UnauthorizedException` | `structure` | `message` | You don't have permission to perform this operation. |
| `ServiceQuotaExceededException` | `structure` | `message` | You exceeded your service quota. |
| `CreateComponentRequest` | `structure` | `appId`, `clientToken`, `componentToCreate`, `environmentName` | - |
| `CreateComponentResponse` | `structure` | `entity` | - |
| `CreateFormRequest` | `structure` | `appId`, `clientToken`, `environmentName`, `formToCreate` | - |
| `CreateFormResponse` | `structure` | `entity` | - |
| `CreateThemeRequest` | `structure` | `appId`, `clientToken`, `environmentName`, `themeToCreate` | - |
| `CreateThemeResponse` | `structure` | `entity` | - |
| `DeleteComponentRequest` | `structure` | `appId`, `environmentName`, `id` | - |
| `DeleteFormRequest` | `structure` | `appId`, `environmentName`, `id` | - |
| `DeleteThemeRequest` | `structure` | `appId`, `environmentName`, `id` | - |
| `ExchangeCodeForTokenRequest` | `structure` | `provider`, `request` | - |
| `ExchangeCodeForTokenResponse` | `structure` | `accessToken`, `expiresIn`, `refreshToken` | - |
| `ExportComponentsRequest` | `structure` | `appId`, `environmentName`, `nextToken` | - |
| `ExportComponentsResponse` | `structure` | `entities`, `nextToken` | - |
| `ExportFormsRequest` | `structure` | `appId`, `environmentName`, `nextToken` | - |
| `ExportFormsResponse` | `structure` | `entities`, `nextToken` | - |
| `ExportThemesRequest` | `structure` | `appId`, `environmentName`, `nextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
