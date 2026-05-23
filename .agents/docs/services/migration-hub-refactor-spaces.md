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

- Operations: `ListApplications`, `ListEnvironments`, `ListEnvironmentVpcs`, `ListRoutes`, `ListServices`, `ListTagsForResource`
- Traits: `readonly` (6), `paginated` (5)
- Common required input members in this group: `EnvironmentIdentifier`, `ApplicationIdentifier`

### Delete

- Operations: `DeleteApplication`, `DeleteEnvironment`, `DeleteResourcePolicy`, `DeleteRoute`, `DeleteService`
- Traits: `idempotent` (5)
- Common required input members in this group: `EnvironmentIdentifier`, `ApplicationIdentifier`

### Get

- Operations: `GetApplication`, `GetEnvironment`, `GetResourcePolicy`, `GetRoute`, `GetService`
- Traits: `readonly` (5)
- Common required input members in this group: `EnvironmentIdentifier`, `ApplicationIdentifier`

### Create

- Operations: `CreateApplication`, `CreateEnvironment`, `CreateRoute`, `CreateService`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `Name`, `EnvironmentIdentifier`, `ApplicationIdentifier`

### Put

- Operations: `PutResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Update

- Operations: `UpdateRoute`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateApplication` | `POST /environments/{EnvironmentIdentifier}/applications` | `idempotency-token` | `Name`, `EnvironmentIdentifier`, `VpcId`, `ProxyType` | `ClientToken` | `CreateApplicationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Migration Hub Refactor Spaces application. The account that owns the environment also owns the applications created inside the environment, regardless of the account that creates the ap ... |
| `CreateEnvironment` | `POST /environments` | `idempotency-token` | `Name`, `NetworkFabricType` | `ClientToken` | `CreateEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Migration Hub Refactor Spaces environment. The caller owns the environment resource, and all Refactor Spaces applications, services, and routes created within the environment. They are ... |
| `CreateRoute` | `POST /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes` | `idempotency-token` | `EnvironmentIdentifier`, `ApplicationIdentifier`, `ServiceIdentifier`, `RouteType` | `ClientToken` | `CreateRouteResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Migration Hub Refactor Spaces route. The account owner of the service resource is always the environment owner, regardless of which account creates the route. Routes target a service in ... |
| `CreateService` | `POST /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/services` | `idempotency-token` | `Name`, `EnvironmentIdentifier`, `ApplicationIdentifier`, `EndpointType` | `ClientToken` | `CreateServiceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Migration Hub Refactor Spaces service. The account owner of the service is always the environment owner, regardless of which account in the environment creates the service. Services hav ... |
| `DeleteApplication` | `DELETE /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}` | `idempotent` | `EnvironmentIdentifier`, `ApplicationIdentifier` | - | `DeleteApplicationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Web Services Migration Hub Refactor Spaces application. Before you can delete an application, you must first delete any services or routes within the application. |
| `DeleteEnvironment` | `DELETE /environments/{EnvironmentIdentifier}` | `idempotent` | `EnvironmentIdentifier` | - | `DeleteEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Web Services Migration Hub Refactor Spaces environment. Before you can delete an environment, you must first delete any applications and services within the environment. |
| `DeleteResourcePolicy` | `DELETE /resourcepolicy/{Identifier}` | `idempotent` | `Identifier` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource policy set for the environment. |
| `DeleteRoute` | `DELETE /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes/{RouteIdentifier}` | `idempotent` | `EnvironmentIdentifier`, `ApplicationIdentifier`, `RouteIdentifier` | - | `DeleteRouteResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Web Services Migration Hub Refactor Spaces route. |
| `DeleteService` | `DELETE /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/services/{ServiceIdentifier}` | `idempotent` | `EnvironmentIdentifier`, `ApplicationIdentifier`, `ServiceIdentifier` | - | `DeleteServiceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Web Services Migration Hub Refactor Spaces service. |
| `GetApplication` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}` | `readonly` | `EnvironmentIdentifier`, `ApplicationIdentifier` | - | `GetApplicationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Web Services Migration Hub Refactor Spaces application. |
| `GetEnvironment` | `GET /environments/{EnvironmentIdentifier}` | `readonly` | `EnvironmentIdentifier` | - | `GetEnvironmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Web Services Migration Hub Refactor Spaces environment. |
| `GetResourcePolicy` | `GET /resourcepolicy/{Identifier}` | `readonly` | `Identifier` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the resource-based permission policy that is set for the given environment. |
| `GetRoute` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes/{RouteIdentifier}` | `readonly` | `EnvironmentIdentifier`, `ApplicationIdentifier`, `RouteIdentifier` | - | `GetRouteResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Web Services Migration Hub Refactor Spaces route. |
| `GetService` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/services/{ServiceIdentifier}` | `readonly` | `EnvironmentIdentifier`, `ApplicationIdentifier`, `ServiceIdentifier` | - | `GetServiceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Web Services Migration Hub Refactor Spaces service. |
| `ListApplications` | `GET /environments/{EnvironmentIdentifier}/applications` | `readonly`, `paginated` | `EnvironmentIdentifier` | - | `ListApplicationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the Amazon Web Services Migration Hub Refactor Spaces applications within an environment. |
| `ListEnvironments` | `GET /environments` | `readonly`, `paginated` | - | - | `ListEnvironmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists Amazon Web Services Migration Hub Refactor Spaces environments owned by a caller account or shared with the caller account. |
| `ListEnvironmentVpcs` | `GET /environments/{EnvironmentIdentifier}/vpcs` | `readonly`, `paginated` | `EnvironmentIdentifier` | - | `ListEnvironmentVpcsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all Amazon Web Services Migration Hub Refactor Spaces service virtual private clouds (VPCs) that are part of the environment. |
| `ListRoutes` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes` | `readonly`, `paginated` | `EnvironmentIdentifier`, `ApplicationIdentifier` | - | `ListRoutesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the Amazon Web Services Migration Hub Refactor Spaces routes within an application. |
| `ListServices` | `GET /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/services` | `readonly`, `paginated` | `EnvironmentIdentifier`, `ApplicationIdentifier` | - | `ListServicesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the Amazon Web Services Migration Hub Refactor Spaces services within an application. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags of a resource. The caller account must be the same as the resource’s OwnerAccountId . Listing tags in other accounts is not supported. |
| `PutResourcePolicy` | `PUT /resourcepolicy` | `idempotent` | `ResourceArn`, `Policy` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidResourcePolicyException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches a resource-based permission policy to the Amazon Web Services Migration Hub Refactor Spaces environment. The policy must contain the same actions and condition statements as the arn:aws:ram::aws:permission/A ... |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the tags of a given resource. Tags are metadata which can be used to manage a resource. To tag a resource, the caller account must be the same as the resource’s OwnerAccountId . Tagging resources in other acc ... |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds to or modifies the tags of the given resource. Tags are metadata which can be used to manage a resource. To untag a resource, the caller account must be the same as the resource’s OwnerAccountId . Untagging reso ... |
| `UpdateRoute` | `PATCH /environments/{EnvironmentIdentifier}/applications/{ApplicationIdentifier}/routes/{RouteIdentifier}` | - | `EnvironmentIdentifier`, `ApplicationIdentifier`, `RouteIdentifier`, `ActivationState` | - | `UpdateRouteResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an Amazon Web Services Migration Hub Refactor Spaces route. |

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
| `AccessDeniedException` | `structure` | Message | The user does not have sufficient access to perform this action. |
| `ConflictException` | `structure` | Message, ResourceId, ResourceType | Updating or deleting a resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | Message | An unexpected error occurred while processing the request. |
| `InvalidResourcePolicyException` | `structure` | Message | The resource policy is not valid. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The request references a resource that does not exist. |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceId, ResourceType, QuotaCode, ServiceCode | The request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | Message, QuotaCode, ServiceCode, RetryAfterSeconds | Request was denied because the request was throttled. |
| `ValidationException` | `structure` | Message | The input does not satisfy the constraints specified by an Amazon Web Service. |
| `CreateApplicationRequest` | `structure` | Name, EnvironmentIdentifier, VpcId, ProxyType, ApiGatewayProxy, Tags, ClientToken | - |
| `CreateApplicationResponse` | `structure` | Name, Arn, OwnerAccountId, CreatedByAccountId, ApplicationId, EnvironmentId, VpcId, ProxyType, ApiGatewayProxy, State, Tags, LastUpdatedTime, ... (+1) | - |
| `CreateEnvironmentRequest` | `structure` | Name, Description, NetworkFabricType, Tags, ClientToken | - |
| `CreateEnvironmentResponse` | `structure` | Name, Arn, Description, EnvironmentId, NetworkFabricType, OwnerAccountId, State, Tags, LastUpdatedTime, CreatedTime | - |
| `CreateRouteRequest` | `structure` | EnvironmentIdentifier, ApplicationIdentifier, ServiceIdentifier, RouteType, DefaultRoute, UriPathRoute, Tags, ClientToken | - |
| `CreateRouteResponse` | `structure` | RouteId, Arn, OwnerAccountId, CreatedByAccountId, RouteType, ServiceId, ApplicationId, UriPathRoute, State, Tags, LastUpdatedTime, CreatedTime | - |
| `CreateServiceRequest` | `structure` | Name, Description, EnvironmentIdentifier, ApplicationIdentifier, VpcId, EndpointType, UrlEndpoint, LambdaEndpoint, Tags, ClientToken | - |
| `CreateServiceResponse` | `structure` | ServiceId, Name, Arn, OwnerAccountId, CreatedByAccountId, Description, EnvironmentId, ApplicationId, VpcId, EndpointType, UrlEndpoint, LambdaEndpoint, ... (+4) | - |
| `DeleteApplicationRequest` | `structure` | EnvironmentIdentifier, ApplicationIdentifier | - |
| `DeleteApplicationResponse` | `structure` | Name, Arn, ApplicationId, EnvironmentId, State, LastUpdatedTime | - |
| `DeleteEnvironmentRequest` | `structure` | EnvironmentIdentifier | - |
| `DeleteEnvironmentResponse` | `structure` | Name, Arn, EnvironmentId, State, LastUpdatedTime | - |
| `DeleteResourcePolicyRequest` | `structure` | Identifier | - |
| `DeleteResourcePolicyResponse` | `structure` | **empty (no members)** | - |
| `DeleteRouteRequest` | `structure` | EnvironmentIdentifier, ApplicationIdentifier, RouteIdentifier | - |
| `DeleteRouteResponse` | `structure` | RouteId, Arn, ServiceId, ApplicationId, State, LastUpdatedTime | - |
| `DeleteServiceRequest` | `structure` | EnvironmentIdentifier, ApplicationIdentifier, ServiceIdentifier | - |
| `DeleteServiceResponse` | `structure` | ServiceId, Name, Arn, EnvironmentId, ApplicationId, State, LastUpdatedTime | - |
| `GetApplicationRequest` | `structure` | EnvironmentIdentifier, ApplicationIdentifier | - |
| `GetApplicationResponse` | `structure` | Name, Arn, OwnerAccountId, CreatedByAccountId, ApplicationId, EnvironmentId, VpcId, ProxyType, ApiGatewayProxy, State, Tags, Error, ... (+2) | - |
| `GetEnvironmentRequest` | `structure` | EnvironmentIdentifier | - |
| `GetEnvironmentResponse` | `structure` | Name, Arn, Description, EnvironmentId, NetworkFabricType, OwnerAccountId, TransitGatewayId, State, Tags, Error, LastUpdatedTime, CreatedTime | - |
| `GetResourcePolicyRequest` | `structure` | Identifier | - |
| `GetResourcePolicyResponse` | `structure` | Policy | - |
| `GetRouteRequest` | `structure` | EnvironmentIdentifier, ApplicationIdentifier, RouteIdentifier | - |
| `GetRouteResponse` | `structure` | RouteId, Arn, OwnerAccountId, CreatedByAccountId, RouteType, ServiceId, ApplicationId, EnvironmentId, SourcePath, Methods, IncludeChildPaths, PathResourceToId, ... (+6) | - |
| `GetServiceRequest` | `structure` | EnvironmentIdentifier, ApplicationIdentifier, ServiceIdentifier | - |
| `GetServiceResponse` | `structure` | ServiceId, Name, Arn, OwnerAccountId, CreatedByAccountId, Description, EnvironmentId, ApplicationId, VpcId, EndpointType, UrlEndpoint, LambdaEndpoint, ... (+5) | - |
| `ListApplicationsRequest` | `structure` | EnvironmentIdentifier, NextToken, MaxResults | - |
| `ListApplicationsResponse` | `structure` | ApplicationSummaryList, NextToken | - |
| `ListEnvironmentsRequest` | `structure` | NextToken, MaxResults | - |
| `ListEnvironmentsResponse` | `structure` | EnvironmentSummaryList, NextToken | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
