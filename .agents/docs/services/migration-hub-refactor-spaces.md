# AWS Migration Hub Refactor Spaces

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Migration Hub Refactor Spaces This API reference provides descriptions, syntax, and other details about each of the actions and data types for Amazon Web Services Migration Hub Refactor Spaces (Refactor Spaces). The topic for each action shows the API request parameters and the response. Alternatively, you can use one of the Amazon Web Services SDKs to access an API that is tailored to the programming language or platform that you're using. For more information, see Amazon Web Services SDKs. To share Refactor Spaces environments with other Amazon Web Services accounts or with Organizations and their OUs, use Resource Access Manager's `CreateResourceShare` API.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Migration Hub Refactor Spaces where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Migration Hub Refactor Spaces by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Migration Hub Refactor Spaces workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Delete`, `Get`, `Create`, `Put` operation families, including `ListApplications`, `ListEnvironmentVpcs`, `ListEnvironments`, `ListRoutes`, `DeleteApplication`, `DeleteEnvironment`.

## Service Identity and Protocol

- AWS model slug: `migration-hub-refactor-spaces`
- AWS SDK for Rust slug: `migrationhubrefactorspaces`
- Model version: `2021-10-26`
- Model file: `vendor/api-models-aws/models/migration-hub-refactor-spaces/service/2021-10-26/migration-hub-refactor-spaces-2021-10-26.json`
- SDK ID: `Migration Hub Refactor Spaces`
- Endpoint prefix: `refactor-spaces`
- ARN namespace: `refactor-spaces`
- CloudFormation name: `RefactorSpaces`
- CloudTrail event source: `refactor-spaces.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Delete` (5), `Get` (5), `Create` (4), `Put` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateApplication`, `CreateEnvironment`, `CreateRoute`, `CreateService`, `DeleteApplication`, `DeleteEnvironment`, `DeleteResourcePolicy`, `DeleteRoute`, `DeleteService`, `PutResourcePolicy`, `TagResource`, `UntagResource`, `UpdateRoute`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApplication`, `GetEnvironment`, `GetResourcePolicy`, `GetRoute`, `GetService`, `ListApplications`, `ListEnvironmentVpcs`, `ListEnvironments`, `ListRoutes`, `ListServices`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 24 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `Lambda`, `EC2/VPC`.

## Operation Groups

### List

- Operations: `ListApplications`, `ListEnvironmentVpcs`, `ListEnvironments`, `ListRoutes`, `ListServices`, `ListTagsForResource`
- Traits: `paginated` (5), `readonly` (6)
- Common required input members in this group: `ApplicationIdentifier`, `EnvironmentIdentifier`, `ResourceArn`

### Delete

- Operations: `DeleteApplication`, `DeleteEnvironment`, `DeleteResourcePolicy`, `DeleteRoute`, `DeleteService`
- Traits: `idempotent` (5)
- Common required input members in this group: `ApplicationIdentifier`, `EnvironmentIdentifier`, `Identifier`, `RouteIdentifier`, `ServiceIdentifier`

### Get

- Operations: `GetApplication`, `GetEnvironment`, `GetResourcePolicy`, `GetRoute`, `GetService`
- Traits: `readonly` (5)
- Common required input members in this group: `ApplicationIdentifier`, `EnvironmentIdentifier`, `Identifier`, `RouteIdentifier`, `ServiceIdentifier`

### Create

- Operations: `CreateApplication`, `CreateEnvironment`, `CreateRoute`, `CreateService`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `ApplicationIdentifier`, `EndpointType`, `EnvironmentIdentifier`, `Name`, `NetworkFabricType`, `ProxyType`, `RouteType`, `ServiceIdentifier`, `VpcId`

### Put

- Operations: `PutResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: `Policy`, `ResourceArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateRoute`
- Common required input members in this group: `ActivationState`, `ApplicationIdentifier`, `EnvironmentIdentifier`, `RouteIdentifier`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateApplication` | `POST /environments/{EnvironmentIdentifier}/applications` | `idempotency-token` | `EnvironmentIdentifier`, `Name`, `ProxyType`, `VpcId` | `ClientToken` | `CreateApplicationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Migration Hub Refactor Spaces application. The account that owns the environment also owns the applications created inside the environment, regardless of the account that creates the application. |
| `CreateEnvironment` | `POST /environments` | `idempotency-token` | `Name`, `NetworkFabricType` | `ClientToken` | `CreateEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Migration Hub Refactor Spaces environment. The caller owns the environment resource, and all Refactor Spaces applications, services, and routes created within the environment. |
| `CreateRoute` | `POST /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes` | `idempotency-token` | `ApplicationIdentifier`, `EnvironmentIdentifier`, `RouteType`, `ServiceIdentifier` | `ClientToken` | `CreateRouteResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Migration Hub Refactor Spaces route. The account owner of the service resource is always the environment owner, regardless of which account creates the route. |
| `CreateService` | `POST /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/services` | `idempotency-token` | `ApplicationIdentifier`, `EndpointType`, `EnvironmentIdentifier`, `Name` | `ClientToken` | `CreateServiceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Migration Hub Refactor Spaces service. The account owner of the service is always the environment owner, regardless of which account in the environment creates the service. |
| `DeleteApplication` | `DELETE /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}` | `idempotent` | `ApplicationIdentifier`, `EnvironmentIdentifier` | - | `DeleteApplicationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Web Services Migration Hub Refactor Spaces application. Before you can delete an application, you must first delete any services or routes within the application. |
| `DeleteEnvironment` | `DELETE /environments/{EnvironmentIdentifier}` | `idempotent` | `EnvironmentIdentifier` | - | `DeleteEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Web Services Migration Hub Refactor Spaces environment. Before you can delete an environment, you must first delete any applications and services within the environment. |
| `DeleteResourcePolicy` | `DELETE /resourcepolicy/{Identifier}` | `idempotent` | `Identifier` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource policy set for the environment. |
| `DeleteRoute` | `DELETE /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes/{RouteIdentifier}` | `idempotent` | `ApplicationIdentifier`, `EnvironmentIdentifier`, `RouteIdentifier` | - | `DeleteRouteResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Web Services Migration Hub Refactor Spaces route. |
| `DeleteService` | `DELETE /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/services/{ServiceIdentifier}` | `idempotent` | `ApplicationIdentifier`, `EnvironmentIdentifier`, `ServiceIdentifier` | - | `DeleteServiceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Web Services Migration Hub Refactor Spaces service. |
| `GetApplication` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}` | `readonly` | `ApplicationIdentifier`, `EnvironmentIdentifier` | - | `GetApplicationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Web Services Migration Hub Refactor Spaces application. |
| `GetEnvironment` | `GET /environments/{EnvironmentIdentifier}` | `readonly` | `EnvironmentIdentifier` | - | `GetEnvironmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Web Services Migration Hub Refactor Spaces environment. |
| `GetResourcePolicy` | `GET /resourcepolicy/{Identifier}` | `readonly` | `Identifier` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the resource-based permission policy that is set for the given environment. |
| `GetRoute` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes/{RouteIdentifier}` | `readonly` | `ApplicationIdentifier`, `EnvironmentIdentifier`, `RouteIdentifier` | - | `GetRouteResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Web Services Migration Hub Refactor Spaces route. |
| `GetService` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/services/{ServiceIdentifier}` | `readonly` | `ApplicationIdentifier`, `EnvironmentIdentifier`, `ServiceIdentifier` | - | `GetServiceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Web Services Migration Hub Refactor Spaces service. |
| `ListApplications` | `GET /environments/{EnvironmentIdentifier}/applications` | `readonly`, `paginated` | `EnvironmentIdentifier` | - | `ListApplicationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the Amazon Web Services Migration Hub Refactor Spaces applications within an environment. |
| `ListEnvironmentVpcs` | `GET /environments/{EnvironmentIdentifier}/vpcs` | `readonly`, `paginated` | `EnvironmentIdentifier` | - | `ListEnvironmentVpcsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all Amazon Web Services Migration Hub Refactor Spaces service virtual private clouds (VPCs) that are part of the environment. |
| `ListEnvironments` | `GET /environments` | `readonly`, `paginated` | - | - | `ListEnvironmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists Amazon Web Services Migration Hub Refactor Spaces environments owned by a caller account or shared with the caller account. |
| `ListRoutes` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes` | `readonly`, `paginated` | `ApplicationIdentifier`, `EnvironmentIdentifier` | - | `ListRoutesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the Amazon Web Services Migration Hub Refactor Spaces routes within an application. |
| `ListServices` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/services` | `readonly`, `paginated` | `ApplicationIdentifier`, `EnvironmentIdentifier` | - | `ListServicesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the Amazon Web Services Migration Hub Refactor Spaces services within an application. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags of a resource. The caller account must be the same as the resource’s `OwnerAccountId`. |
| `PutResourcePolicy` | `PUT /resourcepolicy` | `idempotent` | `Policy`, `ResourceArn` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidResourcePolicyException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches a resource-based permission policy to the Amazon Web Services Migration Hub Refactor Spaces environment. The policy must contain the same actions and condition statements as the... |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the tags of a given resource. Tags are metadata which can be used to manage a resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds to or modifies the tags of the given resource. Tags are metadata which can be used to manage a resource. |
| `UpdateRoute` | `PATCH /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes/{RouteIdentifier}` | - | `ActivationState`, `ApplicationIdentifier`, `EnvironmentIdentifier`, `RouteIdentifier` | - | `UpdateRouteResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an Amazon Web Services Migration Hub Refactor Spaces route. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListApplications` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListEnvironments` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListEnvironmentVpcs` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListRoutes` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListServices` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | An unexpected error occurred while processing the request. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The request references a resource that does not exist. |
| `ValidationException` | `structure` | `Message` | The input does not satisfy the constraints specified by an Amazon Web Service. |
| `AccessDeniedException` | `structure` | `Message` | The user does not have sufficient access to perform this action. |
| `ThrottlingException` | `structure` | `Message`, `QuotaCode`, `RetryAfterSeconds`, `ServiceCode` | Request was denied because the request was throttled. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | Updating or deleting a resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceId`, `ResourceType`, `ServiceCode` | The request would cause a service quota to be exceeded. |
| `CreateApplicationRequest` | `structure` | `ApiGatewayProxy`, `ClientToken`, `EnvironmentIdentifier`, `Name`, `ProxyType`, `Tags`, `VpcId` | - |
| `CreateApplicationResponse` | `structure` | `ApiGatewayProxy`, `ApplicationId`, `Arn`, `CreatedByAccountId`, `CreatedTime`, `EnvironmentId`, `LastUpdatedTime`, `Name`, `OwnerAccountId`, `ProxyType`, `State`, `Tags`, ... (+1) | - |
| `CreateEnvironmentRequest` | `structure` | `ClientToken`, `Description`, `Name`, `NetworkFabricType`, `Tags` | - |
| `CreateEnvironmentResponse` | `structure` | `Arn`, `CreatedTime`, `Description`, `EnvironmentId`, `LastUpdatedTime`, `Name`, `NetworkFabricType`, `OwnerAccountId`, `State`, `Tags` | - |
| `CreateRouteRequest` | `structure` | `ApplicationIdentifier`, `ClientToken`, `DefaultRoute`, `EnvironmentIdentifier`, `RouteType`, `ServiceIdentifier`, `Tags`, `UriPathRoute` | - |
| `CreateRouteResponse` | `structure` | `ApplicationId`, `Arn`, `CreatedByAccountId`, `CreatedTime`, `LastUpdatedTime`, `OwnerAccountId`, `RouteId`, `RouteType`, `ServiceId`, `State`, `Tags`, `UriPathRoute` | - |
| `CreateServiceRequest` | `structure` | `ApplicationIdentifier`, `ClientToken`, `Description`, `EndpointType`, `EnvironmentIdentifier`, `LambdaEndpoint`, `Name`, `Tags`, `UrlEndpoint`, `VpcId` | - |
| `CreateServiceResponse` | `structure` | `ApplicationId`, `Arn`, `CreatedByAccountId`, `CreatedTime`, `Description`, `EndpointType`, `EnvironmentId`, `LambdaEndpoint`, `LastUpdatedTime`, `Name`, `OwnerAccountId`, `ServiceId`, ... (+4) | - |
| `DeleteApplicationRequest` | `structure` | `ApplicationIdentifier`, `EnvironmentIdentifier` | - |
| `DeleteApplicationResponse` | `structure` | `ApplicationId`, `Arn`, `EnvironmentId`, `LastUpdatedTime`, `Name`, `State` | - |
| `DeleteEnvironmentRequest` | `structure` | `EnvironmentIdentifier` | - |
| `DeleteEnvironmentResponse` | `structure` | `Arn`, `EnvironmentId`, `LastUpdatedTime`, `Name`, `State` | - |
| `DeleteResourcePolicyRequest` | `structure` | `Identifier` | - |
| `DeleteResourcePolicyResponse` | `structure` | - | - |
| `DeleteRouteRequest` | `structure` | `ApplicationIdentifier`, `EnvironmentIdentifier`, `RouteIdentifier` | - |
| `DeleteRouteResponse` | `structure` | `ApplicationId`, `Arn`, `LastUpdatedTime`, `RouteId`, `ServiceId`, `State` | - |
| `DeleteServiceRequest` | `structure` | `ApplicationIdentifier`, `EnvironmentIdentifier`, `ServiceIdentifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
