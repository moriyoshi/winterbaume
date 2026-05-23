# AmplifyBackend

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Amplify Admin API

## Possible Usage Scenarios
- From the AWS documentation and model: manage Amplify backend apps, environments, auth resources, storage resources, and backend configuration exports.
- From the operation surface: support mobile or web backend bootstrapping, environment cloning, generated backend jobs, and removal of backend resources.

## Service Identity and Protocol

- AWS model slug: `amplifybackend`
- AWS SDK for Rust slug: `amplifybackend`
- Model version: `2020-08-11`
- Model file: `vendor/api-models-aws/models/amplifybackend/service/2020-08-11/amplifybackend-2020-08-11.json`
- SDK ID: `AmplifyBackend`
- Endpoint prefix: `amplifybackend`
- ARN namespace: `amplifybackend`
- CloudFormation name: `AmplifyBackend`
- CloudTrail event source: `amplifybackend.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (7), `Create` (6), `Delete` (5), `Update` (5), `Import` (2), `List` (2), `Remove` (2), `Clone` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateBackend`, `CreateBackendAPI`, `CreateBackendAuth`, `CreateBackendConfig`, `CreateBackendStorage`, `CreateToken`, `DeleteBackend`, `DeleteBackendAPI`, `DeleteBackendAuth`, `DeleteBackendStorage`, `DeleteToken`, `ImportBackendAuth`, `ImportBackendStorage`, `RemoveAllBackends`, `RemoveBackendConfig`, `UpdateBackendAPI`, `UpdateBackendAuth`, `UpdateBackendConfig`, `UpdateBackendJob`, `UpdateBackendStorage`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBackend`, `GetBackendAPI`, `GetBackendAPIModels`, `GetBackendAuth`, `GetBackendJob`, `GetBackendStorage`, `GetToken`, `ListBackendJobs`, `ListS3Buckets`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetBackendJob`, `ImportBackendAuth`, `ImportBackendStorage`, `ListBackendJobs`, `UpdateBackendJob`.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.

## Operation Groups

### Get

- Operations: `GetBackend`, `GetBackendAPI`, `GetBackendAPIModels`, `GetBackendAuth`, `GetBackendJob`, `GetBackendStorage`, `GetToken`
- Common required input members in this group: `AppId`, `BackendEnvironmentName`, `ResourceName`

### Create

- Operations: `CreateBackend`, `CreateBackendAPI`, `CreateBackendAuth`, `CreateBackendConfig`, `CreateBackendStorage`, `CreateToken`
- Common required input members in this group: `AppId`, `BackendEnvironmentName`, `ResourceConfig`, `ResourceName`

### Delete

- Operations: `DeleteBackend`, `DeleteBackendAPI`, `DeleteBackendAuth`, `DeleteBackendStorage`, `DeleteToken`
- Common required input members in this group: `AppId`, `BackendEnvironmentName`, `ResourceName`

### Update

- Operations: `UpdateBackendAPI`, `UpdateBackendAuth`, `UpdateBackendConfig`, `UpdateBackendJob`, `UpdateBackendStorage`
- Common required input members in this group: `AppId`, `BackendEnvironmentName`, `ResourceName`, `ResourceConfig`

### Import

- Operations: `ImportBackendAuth`, `ImportBackendStorage`
- Common required input members in this group: `AppId`, `BackendEnvironmentName`

### List

- Operations: `ListBackendJobs`, `ListS3Buckets`
- Common required input members in this group: -

### Remove

- Operations: `RemoveAllBackends`, `RemoveBackendConfig`
- Common required input members in this group: `AppId`

### Clone

- Operations: `CloneBackend`
- Common required input members in this group: -

### Generate

- Operations: `GenerateBackendAPIModels`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CloneBackend` | `POST /backend/{AppId}/environments/{BackendEnvironmentName}/clone` | - | `AppId`, `BackendEnvironmentName`, `TargetEnvironmentName` | - | `CloneBackendResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | This operation clones an existing backend. |
| `CreateBackend` | `POST /backend` | - | `AppId`, `AppName`, `BackendEnvironmentName` | - | `CreateBackendResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | This operation creates a backend for an Amplify app. Backends are automatically created at the time of app creation. |
| `CreateBackendAPI` | `POST /backend/{AppId}/api` | - | `AppId`, `BackendEnvironmentName`, `ResourceConfig`, `ResourceName` | - | `CreateBackendAPIResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Creates a new backend API resource. |
| `CreateBackendAuth` | `POST /backend/{AppId}/auth` | - | `AppId`, `BackendEnvironmentName`, `ResourceConfig`, `ResourceName` | - | `CreateBackendAuthResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Creates a new backend authentication resource. |
| `CreateBackendConfig` | `POST /backend/{AppId}/config` | - | `AppId` | - | `CreateBackendConfigResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Creates a config object for a backend. |
| `CreateBackendStorage` | `POST /backend/{AppId}/storage` | - | `AppId`, `BackendEnvironmentName`, `ResourceConfig`, `ResourceName` | - | `CreateBackendStorageResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Creates a backend storage resource. |
| `CreateToken` | `POST /backend/{AppId}/challenge` | - | `AppId` | - | `CreateTokenResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Generates a one-time challenge code to authenticate a user into your Amplify Admin UI. |
| `DeleteBackend` | `POST /backend/{AppId}/environments/{BackendEnvironmentName}/remove` | - | `AppId`, `BackendEnvironmentName` | - | `DeleteBackendResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Removes an existing environment from your Amplify project. |
| `DeleteBackendAPI` | `POST /backend/{AppId}/api/{BackendEnvironmentName}/remove` | - | `AppId`, `BackendEnvironmentName`, `ResourceName` | - | `DeleteBackendAPIResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Deletes an existing backend API resource. |
| `DeleteBackendAuth` | `POST /backend/{AppId}/auth/{BackendEnvironmentName}/remove` | - | `AppId`, `BackendEnvironmentName`, `ResourceName` | - | `DeleteBackendAuthResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Deletes an existing backend authentication resource. |
| `DeleteBackendStorage` | `POST /backend/{AppId}/storage/{BackendEnvironmentName}/remove` | - | `AppId`, `BackendEnvironmentName`, `ResourceName`, `ServiceName` | - | `DeleteBackendStorageResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Removes the specified backend storage resource. |
| `DeleteToken` | `POST /backend/{AppId}/challenge/{SessionId}/remove` | - | `AppId`, `SessionId` | - | `DeleteTokenResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Deletes the challenge token based on the given appId and sessionId. |
| `GenerateBackendAPIModels` | `POST /backend/{AppId}/api/{BackendEnvironmentName}/generateModels` | - | `AppId`, `BackendEnvironmentName`, `ResourceName` | - | `GenerateBackendAPIModelsResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Generates a model schema for an existing backend API resource. |
| `GetBackend` | `POST /backend/{AppId}/details` | - | `AppId` | - | `GetBackendResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Provides project-level details for your Amplify UI project. |
| `GetBackendAPI` | `POST /backend/{AppId}/api/{BackendEnvironmentName}/details` | - | `AppId`, `BackendEnvironmentName`, `ResourceName` | - | `GetBackendAPIResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Gets the details for a backend API. |
| `GetBackendAPIModels` | `POST /backend/{AppId}/api/{BackendEnvironmentName}/getModels` | - | `AppId`, `BackendEnvironmentName`, `ResourceName` | - | `GetBackendAPIModelsResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Gets a model introspection schema for an existing backend API resource. |
| `GetBackendAuth` | `POST /backend/{AppId}/auth/{BackendEnvironmentName}/details` | - | `AppId`, `BackendEnvironmentName`, `ResourceName` | - | `GetBackendAuthResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Gets a backend auth details. |
| `GetBackendJob` | `GET /backend/{AppId}/job/{BackendEnvironmentName}/{JobId}` | - | `AppId`, `BackendEnvironmentName`, `JobId` | - | `GetBackendJobResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Returns information about a specific job. |
| `GetBackendStorage` | `POST /backend/{AppId}/storage/{BackendEnvironmentName}/details` | - | `AppId`, `BackendEnvironmentName`, `ResourceName` | - | `GetBackendStorageResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Gets details for a backend storage resource. |
| `GetToken` | `GET /backend/{AppId}/challenge/{SessionId}` | - | `AppId`, `SessionId` | - | `GetTokenResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Gets the challenge token based on the given appId and sessionId. |
| `ImportBackendAuth` | `POST /backend/{AppId}/auth/{BackendEnvironmentName}/import` | - | `AppId`, `BackendEnvironmentName`, `NativeClientId`, `UserPoolId`, `WebClientId` | - | `ImportBackendAuthResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Imports an existing backend authentication resource. |
| `ImportBackendStorage` | `POST /backend/{AppId}/storage/{BackendEnvironmentName}/import` | - | `AppId`, `BackendEnvironmentName`, `ServiceName` | - | `ImportBackendStorageResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Imports an existing backend storage resource. |
| `ListBackendJobs` | `POST /backend/{AppId}/job/{BackendEnvironmentName}` | - | `AppId`, `BackendEnvironmentName` | - | `ListBackendJobsResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Lists the jobs for the backend of an Amplify app. |
| `ListS3Buckets` | `POST /s3Buckets` | - | - | - | `ListS3BucketsResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | The list of S3 buckets in your account. |
| `RemoveAllBackends` | `POST /backend/{AppId}/remove` | - | `AppId` | - | `RemoveAllBackendsResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Removes all backend environments from your Amplify project. |
| `RemoveBackendConfig` | `POST /backend/{AppId}/config/remove` | - | `AppId` | - | `RemoveBackendConfigResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Removes the AWS resources required to access the Amplify Admin UI. |
| `UpdateBackendAPI` | `POST /backend/{AppId}/api/{BackendEnvironmentName}` | - | `AppId`, `BackendEnvironmentName`, `ResourceName` | - | `UpdateBackendAPIResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Updates an existing backend API resource. |
| `UpdateBackendAuth` | `POST /backend/{AppId}/auth/{BackendEnvironmentName}` | - | `AppId`, `BackendEnvironmentName`, `ResourceConfig`, `ResourceName` | - | `UpdateBackendAuthResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Updates an existing backend authentication resource. |
| `UpdateBackendConfig` | `POST /backend/{AppId}/config/update` | - | `AppId` | - | `UpdateBackendConfigResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Updates the AWS resources required to access the Amplify Admin UI. |
| `UpdateBackendJob` | `POST /backend/{AppId}/job/{BackendEnvironmentName}/{JobId}` | - | `AppId`, `BackendEnvironmentName`, `JobId` | - | `UpdateBackendJobResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Updates a specific job. |
| `UpdateBackendStorage` | `POST /backend/{AppId}/storage/{BackendEnvironmentName}` | - | `AppId`, `BackendEnvironmentName`, `ResourceConfig`, `ResourceName` | - | `UpdateBackendStorageResponse` | `BadRequestException`, `GatewayTimeoutException`, `NotFoundException`, `TooManyRequestsException` | Updates an existing backend storage resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | Message | An error returned if a request is not formed properly. |
| `GatewayTimeoutException` | `structure` | Message | An error returned if there's a temporary issue with the service. |
| `NotFoundException` | `structure` | Message, ResourceType | An error returned when a specific resource type is not found. |
| `TooManyRequestsException` | `structure` | LimitType, Message | An error that is returned when a limit of a specific type has been exceeded. |
| `CloneBackendRequest` | `structure` | AppId, BackendEnvironmentName, TargetEnvironmentName | The request body for CloneBackend. |
| `CloneBackendResponse` | `structure` | AppId, BackendEnvironmentName, Error, JobId, Operation, Status | - |
| `CreateBackendRequest` | `structure` | AppId, AppName, BackendEnvironmentName, ResourceConfig, ResourceName | The request body for CreateBackend. |
| `CreateBackendResponse` | `structure` | AppId, BackendEnvironmentName, Error, JobId, Operation, Status | - |
| `CreateBackendAPIRequest` | `structure` | AppId, BackendEnvironmentName, ResourceConfig, ResourceName | The request body for CreateBackendAPI. |
| `CreateBackendAPIResponse` | `structure` | AppId, BackendEnvironmentName, Error, JobId, Operation, Status | - |
| `CreateBackendAuthRequest` | `structure` | AppId, BackendEnvironmentName, ResourceConfig, ResourceName | The request body for CreateBackendAuth. |
| `CreateBackendAuthResponse` | `structure` | AppId, BackendEnvironmentName, Error, JobId, Operation, Status | - |
| `CreateBackendConfigRequest` | `structure` | AppId, BackendManagerAppId | The request body for CreateBackendConfig. |
| `CreateBackendConfigResponse` | `structure` | AppId, BackendEnvironmentName, JobId, Status | - |
| `CreateBackendStorageRequest` | `structure` | AppId, BackendEnvironmentName, ResourceConfig, ResourceName | The request body for CreateBackendStorage. |
| `CreateBackendStorageResponse` | `structure` | AppId, BackendEnvironmentName, JobId, Status | - |
| `CreateTokenRequest` | `structure` | AppId | - |
| `CreateTokenResponse` | `structure` | AppId, ChallengeCode, SessionId, Ttl | - |
| `DeleteBackendRequest` | `structure` | AppId, BackendEnvironmentName | - |
| `DeleteBackendResponse` | `structure` | AppId, BackendEnvironmentName, Error, JobId, Operation, Status | - |
| `DeleteBackendAPIRequest` | `structure` | AppId, BackendEnvironmentName, ResourceConfig, ResourceName | The request body for DeleteBackendAPI. |
| `DeleteBackendAPIResponse` | `structure` | AppId, BackendEnvironmentName, Error, JobId, Operation, Status | - |
| `DeleteBackendAuthRequest` | `structure` | AppId, BackendEnvironmentName, ResourceName | The request body for DeleteBackendAuth. |
| `DeleteBackendAuthResponse` | `structure` | AppId, BackendEnvironmentName, Error, JobId, Operation, Status | - |
| `DeleteBackendStorageRequest` | `structure` | AppId, BackendEnvironmentName, ResourceName, ServiceName | The request body for DeleteBackendStorage. |
| `DeleteBackendStorageResponse` | `structure` | AppId, BackendEnvironmentName, JobId, Status | - |
| `DeleteTokenRequest` | `structure` | AppId, SessionId | - |
| `DeleteTokenResponse` | `structure` | IsSuccess | - |
| `GenerateBackendAPIModelsRequest` | `structure` | AppId, BackendEnvironmentName, ResourceName | The request body for GenerateBackendAPIModels. |
| `GenerateBackendAPIModelsResponse` | `structure` | AppId, BackendEnvironmentName, Error, JobId, Operation, Status | - |
| `GetBackendRequest` | `structure` | AppId, BackendEnvironmentName | The request body for GetBackend. |
| `GetBackendResponse` | `structure` | AmplifyFeatureFlags, AmplifyMetaConfig, AppId, AppName, BackendEnvironmentList, BackendEnvironmentName, Error | - |
| `GetBackendAPIRequest` | `structure` | AppId, BackendEnvironmentName, ResourceConfig, ResourceName | The request body for GetBackendAPI. |
| `GetBackendAPIResponse` | `structure` | AppId, BackendEnvironmentName, Error, ResourceConfig, ResourceName | - |
| `GetBackendAPIModelsRequest` | `structure` | AppId, BackendEnvironmentName, ResourceName | The request body for GetBackendAPIModels. |
| `GetBackendAPIModelsResponse` | `structure` | Models, Status, ModelIntrospectionSchema | - |
| `GetBackendAuthRequest` | `structure` | AppId, BackendEnvironmentName, ResourceName | The request body for GetBackendAuth. |
| `GetBackendAuthResponse` | `structure` | AppId, BackendEnvironmentName, Error, ResourceConfig, ResourceName | - |
| `GetBackendJobRequest` | `structure` | AppId, BackendEnvironmentName, JobId | - |
| `GetBackendJobResponse` | `structure` | AppId, BackendEnvironmentName, CreateTime, Error, JobId, Operation, Status, UpdateTime | - |
| `AdditionalConstraintsElement` | `enum` | REQUIRE_DIGIT, REQUIRE_LOWERCASE, REQUIRE_SYMBOL, REQUIRE_UPPERCASE | - |
| `AuthResources` | `enum` | USER_POOL_ONLY, IDENTITY_POOL_AND_USER_POOL | - |
| `AuthenticatedElement` | `enum` | READ, CREATE_AND_UPDATE, DELETE | - |
| `DeliveryMethod` | `enum` | EMAIL, SMS | The type of verification message to send. |
| `MFAMode` | `enum` | ON, OFF, OPTIONAL | - |
| `MfaTypesElement` | `enum` | SMS, TOTP | - |
| `Mode` | `enum` | API_KEY, AWS_IAM, AMAZON_COGNITO_USER_POOLS, OPENID_CONNECT | - |
| `OAuthGrantType` | `enum` | CODE, IMPLICIT | - |
| `OAuthScopesElement` | `enum` | PHONE, EMAIL, OPENID, PROFILE, AWS_COGNITO_SIGNIN_USER_ADMIN | - |
| `RequiredSignUpAttributesElement` | `enum` | ADDRESS, BIRTHDATE, EMAIL, FAMILY_NAME, GENDER, GIVEN_NAME, LOCALE, MIDDLE_NAME, NAME, NICKNAME, PHONE_NUMBER, PICTURE, ... (+5) | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
