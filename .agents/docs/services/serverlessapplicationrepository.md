# AWSServerlessApplicationRepository

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The AWS Serverless Application Repository makes it easy for developers and enterprises to quickly find and deploy serverless applications in the AWS Cloud. For more information about serverless applications, see Serverless Computing and Applications on the AWS website. The AWS Serverless Application Repository is deeply integrated with the AWS Lambda console, so that developers of all levels can get started with serverless computing without needing to learn anything new. You can use category keywords to browse for applications such as web and mobile backends, data processing applications, or chatbots. You can also search for applications by name, publisher, or event source.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWSServerlessApplicationRepository workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Get`, `List`, `Delete`, `Put` operation families, including `CreateApplication`, `CreateApplicationVersion`, `CreateCloudFormationChangeSet`, `CreateCloudFormationTemplate`, `GetApplication`, `GetApplicationPolicy`.

## Service Identity and Protocol

- AWS model slug: `serverlessapplicationrepository`
- AWS SDK for Rust slug: `serverlessapplicationrepository`
- Model version: `2017-09-08`
- Model file: `vendor/api-models-aws/models/serverlessapplicationrepository/service/2017-09-08/serverlessapplicationrepository-2017-09-08.json`
- SDK ID: `ServerlessApplicationRepository`
- Endpoint prefix: `serverlessrepo`
- ARN namespace: `serverlessrepo`
- CloudFormation name: `ServerlessApplicationRepository`
- CloudTrail event source: `serverlessapplicationrepository.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (4), `Get` (3), `List` (3), `Delete` (1), `Put` (1), `Unshare` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateApplication`, `CreateApplicationVersion`, `CreateCloudFormationChangeSet`, `CreateCloudFormationTemplate`, `DeleteApplication`, `PutApplicationPolicy`, `UpdateApplication`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApplication`, `GetApplicationPolicy`, `GetCloudFormationTemplate`, `ListApplicationDependencies`, `ListApplicationVersions`, `ListApplications`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- 14 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `Lambda`, `ECR`.

## Operation Groups

### Create

- Operations: `CreateApplication`, `CreateApplicationVersion`, `CreateCloudFormationChangeSet`, `CreateCloudFormationTemplate`
- Common required input members in this group: `ApplicationId`

### Get

- Operations: `GetApplication`, `GetApplicationPolicy`, `GetCloudFormationTemplate`
- Common required input members in this group: `ApplicationId`

### List

- Operations: `ListApplicationDependencies`, `ListApplications`, `ListApplicationVersions`
- Traits: `paginated` (3)
- Common required input members in this group: `ApplicationId`

### Delete

- Operations: `DeleteApplication`
- Common required input members in this group: -

### Put

- Operations: `PutApplicationPolicy`
- Common required input members in this group: -

### Unshare

- Operations: `UnshareApplication`
- Common required input members in this group: -

### Update

- Operations: `UpdateApplication`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateApplication` | `POST /applications` | - | `Author`, `Description`, `Name` | - | `CreateApplicationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `TooManyRequestsException` | Creates an application, optionally including an AWS SAM file to create the first application version in the same call. |
| `CreateApplicationVersion` | `PUT /applications/{ApplicationId}/versions/{SemanticVersion}` | - | `ApplicationId`, `SemanticVersion` | - | `CreateApplicationVersionResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `TooManyRequestsException` | Creates an application version. |
| `CreateCloudFormationChangeSet` | `POST /applications/{ApplicationId}/changesets` | - | `ApplicationId`, `StackName` | - | `CreateCloudFormationChangeSetResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `TooManyRequestsException` | Creates an AWS CloudFormation change set for the given application. |
| `CreateCloudFormationTemplate` | `POST /applications/{ApplicationId}/templates` | - | `ApplicationId` | - | `CreateCloudFormationTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates an AWS CloudFormation template. |
| `DeleteApplication` | `DELETE /applications/{ApplicationId}` | - | `ApplicationId` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes the specified application. |
| `GetApplication` | `GET /applications/{ApplicationId}` | - | `ApplicationId` | - | `GetApplicationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets the specified application. |
| `GetApplicationPolicy` | `GET /applications/{ApplicationId}/policy` | - | `ApplicationId` | - | `GetApplicationPolicyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the policy for the application. |
| `GetCloudFormationTemplate` | `GET /applications/{ApplicationId}/templates/{TemplateId}` | - | `ApplicationId`, `TemplateId` | - | `GetCloudFormationTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets the specified AWS CloudFormation template. |
| `ListApplicationDependencies` | `GET /applications/{ApplicationId}/dependencies` | `paginated` | `ApplicationId` | - | `ListApplicationDependenciesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the list of applications nested in the containing application. |
| `ListApplications` | `GET /applications` | `paginated` | - | - | `ListApplicationsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Lists applications owned by the requester. |
| `ListApplicationVersions` | `GET /applications/{ApplicationId}/versions` | `paginated` | `ApplicationId` | - | `ListApplicationVersionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists versions for the specified application. |
| `PutApplicationPolicy` | `PUT /applications/{ApplicationId}/policy` | - | `ApplicationId`, `Statements` | - | `PutApplicationPolicyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Sets the permission policy for an application. For the list of actions supported for this operation, see Application Permissions . |
| `UnshareApplication` | `POST /applications/{ApplicationId}/unshare` | - | `ApplicationId`, `OrganizationId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Unshares an application from an AWS Organization. This operation can be called only from the organization's master account. |
| `UpdateApplication` | `PATCH /applications/{ApplicationId}` | - | `ApplicationId` | - | `UpdateApplicationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Updates the specified application. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetApplication` | - | `SemanticVersion -> semanticVersion` | - | - |
| `ListApplicationDependencies` | - | `MaxItems -> maxItems`, `NextToken -> nextToken`, `SemanticVersion -> semanticVersion` | - | - |
| `ListApplications` | - | `MaxItems -> maxItems`, `NextToken -> nextToken` | - | - |
| `ListApplicationVersions` | - | `MaxItems -> maxItems`, `NextToken -> nextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | ErrorCode, Message | One of the parameters in the request is invalid. |
| `ConflictException` | `structure` | ErrorCode, Message | The resource already exists. |
| `ForbiddenException` | `structure` | ErrorCode, Message | The client is not authenticated. |
| `InternalServerErrorException` | `structure` | ErrorCode, Message | The AWS Serverless Application Repository service encountered an internal error. |
| `NotFoundException` | `structure` | ErrorCode, Message | The resource (for example, an access policy statement) specified in the request doesn't exist. |
| `TooManyRequestsException` | `structure` | ErrorCode, Message | The client is sending more than the allowed number of requests per unit of time. |
| `CreateApplicationRequest` | `structure` | Author, Description, HomePageUrl, Labels, LicenseBody, LicenseUrl, Name, ReadmeBody, ReadmeUrl, SemanticVersion, SourceCodeArchiveUrl, SourceCodeUrl, ... (+3) | - |
| `CreateApplicationResponse` | `structure` | ApplicationId, Author, CreationTime, Description, HomePageUrl, IsVerifiedAuthor, Labels, LicenseUrl, Name, ReadmeUrl, SpdxLicenseId, VerifiedAuthorUrl, ... (+1) | - |
| `CreateApplicationVersionRequest` | `structure` | ApplicationId, SemanticVersion, SourceCodeArchiveUrl, SourceCodeUrl, TemplateBody, TemplateUrl | - |
| `CreateApplicationVersionResponse` | `structure` | ApplicationId, CreationTime, ParameterDefinitions, RequiredCapabilities, ResourcesSupported, SemanticVersion, SourceCodeArchiveUrl, SourceCodeUrl, TemplateUrl | - |
| `CreateCloudFormationChangeSetRequest` | `structure` | ApplicationId, Capabilities, ChangeSetName, ClientToken, Description, NotificationArns, ParameterOverrides, ResourceTypes, RollbackConfiguration, SemanticVersion, StackName, Tags, ... (+1) | - |
| `CreateCloudFormationChangeSetResponse` | `structure` | ApplicationId, ChangeSetId, SemanticVersion, StackId | - |
| `CreateCloudFormationTemplateRequest` | `structure` | ApplicationId, SemanticVersion | - |
| `CreateCloudFormationTemplateResponse` | `structure` | ApplicationId, CreationTime, ExpirationTime, SemanticVersion, Status, TemplateId, TemplateUrl | - |
| `DeleteApplicationRequest` | `structure` | ApplicationId | - |
| `GetApplicationRequest` | `structure` | ApplicationId, SemanticVersion | - |
| `GetApplicationResponse` | `structure` | ApplicationId, Author, CreationTime, Description, HomePageUrl, IsVerifiedAuthor, Labels, LicenseUrl, Name, ReadmeUrl, SpdxLicenseId, VerifiedAuthorUrl, ... (+1) | - |
| `GetApplicationPolicyRequest` | `structure` | ApplicationId | - |
| `GetApplicationPolicyResponse` | `structure` | Statements | - |
| `GetCloudFormationTemplateRequest` | `structure` | ApplicationId, TemplateId | - |
| `GetCloudFormationTemplateResponse` | `structure` | ApplicationId, CreationTime, ExpirationTime, SemanticVersion, Status, TemplateId, TemplateUrl | - |
| `ListApplicationDependenciesRequest` | `structure` | ApplicationId, MaxItems, NextToken, SemanticVersion | - |
| `ListApplicationDependenciesResponse` | `structure` | Dependencies, NextToken | - |
| `ListApplicationsRequest` | `structure` | MaxItems, NextToken | - |
| `ListApplicationsResponse` | `structure` | Applications, NextToken | - |
| `ListApplicationVersionsRequest` | `structure` | ApplicationId, MaxItems, NextToken | - |
| `ListApplicationVersionsResponse` | `structure` | NextToken, Versions | - |
| `PutApplicationPolicyRequest` | `structure` | ApplicationId, Statements | - |
| `PutApplicationPolicyResponse` | `structure` | Statements | - |
| `UnshareApplicationRequest` | `structure` | ApplicationId, OrganizationId | - |
| `UpdateApplicationRequest` | `structure` | ApplicationId, Author, Description, HomePageUrl, Labels, ReadmeBody, ReadmeUrl | - |
| `UpdateApplicationResponse` | `structure` | ApplicationId, Author, CreationTime, Description, HomePageUrl, IsVerifiedAuthor, Labels, LicenseUrl, Name, ReadmeUrl, SpdxLicenseId, VerifiedAuthorUrl, ... (+1) | - |
| `Capability` | `enum` | CAPABILITY_IAM, CAPABILITY_NAMED_IAM, CAPABILITY_AUTO_EXPAND, CAPABILITY_RESOURCE_POLICY | Values that must be specified in order to deploy some applications. |
| `Status` | `enum` | PREPARING, ACTIVE, EXPIRED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
