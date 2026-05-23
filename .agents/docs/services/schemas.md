# Schemas

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon EventBridge Schema Registry

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Schemas workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `List`, `Describe`, `Create`, `Get` operation families, including `DeleteDiscoverer`, `DeleteRegistry`, `DeleteResourcePolicy`, `DeleteSchema`, `ListDiscoverers`, `ListRegistries`.

## Service Identity and Protocol

- AWS model slug: `schemas`
- AWS SDK for Rust slug: `schemas`
- Model version: `2019-12-02`
- Model file: `vendor/api-models-aws/models/schemas/service/2019-12-02/schemas-2019-12-02.json`
- SDK ID: `schemas`
- Endpoint prefix: `schemas`
- ARN namespace: `schemas`
- CloudFormation name: `EventSchemas`
- CloudTrail event source: `schemas.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (5), `List` (5), `Describe` (4), `Create` (3), `Get` (3), `Update` (3), `Put` (2), `Export` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDiscoverer`, `CreateRegistry`, `CreateSchema`, `DeleteDiscoverer`, `DeleteRegistry`, `DeleteResourcePolicy`, `DeleteSchema`, `DeleteSchemaVersion`, `PutCodeBinding`, `PutResourcePolicy`, `StartDiscoverer`, `StopDiscoverer`, `TagResource`, `UntagResource`, `UpdateDiscoverer`, `UpdateRegistry`, `UpdateSchema`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCodeBinding`, `DescribeDiscoverer`, `DescribeRegistry`, `DescribeSchema`, `GetCodeBindingSource`, `GetDiscoveredSchema`, `GetResourcePolicy`, `ListDiscoverers`, `ListRegistries`, `ListSchemaVersions`, `ListSchemas`, `ListTagsForResource`, `SearchSchemas`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ExportSchema`, `StartDiscoverer`, `StopDiscoverer`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EventBridge`.

## Operation Groups

### Delete

- Operations: `DeleteDiscoverer`, `DeleteRegistry`, `DeleteResourcePolicy`, `DeleteSchema`, `DeleteSchemaVersion`
- Common required input members in this group: `RegistryName`, `SchemaName`

### List

- Operations: `ListDiscoverers`, `ListRegistries`, `ListSchemas`, `ListSchemaVersions`, `ListTagsForResource`
- Traits: `paginated` (4)
- Common required input members in this group: `RegistryName`

### Describe

- Operations: `DescribeCodeBinding`, `DescribeDiscoverer`, `DescribeRegistry`, `DescribeSchema`
- Common required input members in this group: `RegistryName`, `SchemaName`

### Create

- Operations: `CreateDiscoverer`, `CreateRegistry`, `CreateSchema`
- Common required input members in this group: `RegistryName`

### Get

- Operations: `GetCodeBindingSource`, `GetDiscoveredSchema`, `GetResourcePolicy`
- Common required input members in this group: -

### Update

- Operations: `UpdateDiscoverer`, `UpdateRegistry`, `UpdateSchema`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `RegistryName`

### Put

- Operations: `PutCodeBinding`, `PutResourcePolicy`
- Common required input members in this group: -

### Export

- Operations: `ExportSchema`
- Common required input members in this group: -

### Search

- Operations: `SearchSchemas`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartDiscoverer`
- Common required input members in this group: -

### Stop

- Operations: `StopDiscoverer`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateDiscoverer` | `POST /v1/discoverers` | - | `SourceArn` | - | `CreateDiscovererResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Creates a discoverer. |
| `CreateRegistry` | `POST /v1/registries/name/{RegistryName}` | - | `RegistryName` | - | `CreateRegistryResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Creates a registry. |
| `CreateSchema` | `POST /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}` | - | `Content`, `RegistryName`, `SchemaName`, `Type` | - | `CreateSchemaResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException` | Creates a schema definition. Inactive schemas will be deleted after two years. |
| `DeleteDiscoverer` | `DELETE /v1/discoverers/id/{DiscovererId}` | - | `DiscovererId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Deletes a discoverer. |
| `DeleteRegistry` | `DELETE /v1/registries/name/{RegistryName}` | - | `RegistryName` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Deletes a Registry. |
| `DeleteResourcePolicy` | `DELETE /v1/policy` | - | - | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Delete the resource-based policy attached to the specified registry. |
| `DeleteSchema` | `DELETE /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}` | - | `RegistryName`, `SchemaName` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Delete a schema definition. |
| `DeleteSchemaVersion` | `DELETE /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}/version/{SchemaVersion}` | - | `RegistryName`, `SchemaName`, `SchemaVersion` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Delete the schema version definition |
| `DescribeCodeBinding` | `GET /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}/language/{Language}` | - | `Language`, `RegistryName`, `SchemaName` | - | `DescribeCodeBindingResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Describe the code binding URI. |
| `DescribeDiscoverer` | `GET /v1/discoverers/id/{DiscovererId}` | - | `DiscovererId` | - | `DescribeDiscovererResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Describes the discoverer. |
| `DescribeRegistry` | `GET /v1/registries/name/{RegistryName}` | - | `RegistryName` | - | `DescribeRegistryResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Describes the registry. |
| `DescribeSchema` | `GET /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}` | - | `RegistryName`, `SchemaName` | - | `DescribeSchemaResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Retrieve the schema definition. |
| `ExportSchema` | `GET /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}/export` | - | `RegistryName`, `SchemaName`, `Type` | - | `ExportSchemaResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | - |
| `GetCodeBindingSource` | `GET /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}/language/{Language}/source` | - | `Language`, `RegistryName`, `SchemaName` | - | `GetCodeBindingSourceResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Get the code binding source URI. |
| `GetDiscoveredSchema` | `POST /v1/discover` | - | `Events`, `Type` | - | `GetDiscoveredSchemaResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Get the discovered schema that was generated based on sampled events. |
| `GetResourcePolicy` | `GET /v1/policy` | - | - | - | `GetResourcePolicyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Retrieves the resource-based policy attached to a given registry. |
| `ListDiscoverers` | `GET /v1/discoverers` | `paginated` | - | - | `ListDiscoverersResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | List the discoverers. |
| `ListRegistries` | `GET /v1/registries` | `paginated` | - | - | `ListRegistriesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | List the registries. |
| `ListSchemas` | `GET /v1/registries/name/{RegistryName}/schemas` | `paginated` | `RegistryName` | - | `ListSchemasResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | List the schemas. |
| `ListSchemaVersions` | `GET /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}/versions` | `paginated` | `RegistryName`, `SchemaName` | - | `ListSchemaVersionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Provides a list of the schema versions and related information. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Get tags for resource. |
| `PutCodeBinding` | `POST /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}/language/{Language}` | - | `Language`, `RegistryName`, `SchemaName` | - | `PutCodeBindingResponse` | `BadRequestException`, `ForbiddenException`, `GoneException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Put code binding URI |
| `PutResourcePolicy` | `PUT /v1/policy` | - | `Policy` | - | `PutResourcePolicyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `PreconditionFailedException`, `ServiceUnavailableException`, `UnauthorizedException` | The name of the policy. |
| `SearchSchemas` | `GET /v1/registries/name/{RegistryName}/schemas/search` | `paginated` | `Keywords`, `RegistryName` | - | `SearchSchemasResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `UnauthorizedException` | Search the schemas |
| `StartDiscoverer` | `POST /v1/discoverers/id/{DiscovererId}/start` | - | `DiscovererId` | - | `StartDiscovererResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Starts the discoverer |
| `StopDiscoverer` | `POST /v1/discoverers/id/{DiscovererId}/stop` | - | `DiscovererId` | - | `StopDiscovererResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Stops the discoverer |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Add tags to a resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Removes tags from a resource. |
| `UpdateDiscoverer` | `PUT /v1/discoverers/id/{DiscovererId}` | - | `DiscovererId` | - | `UpdateDiscovererResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Updates the discoverer |
| `UpdateRegistry` | `PUT /v1/registries/name/{RegistryName}` | - | `RegistryName` | - | `UpdateRegistryResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `UnauthorizedException` | Updates a registry. |
| `UpdateSchema` | `PUT /v1/registries/name/{RegistryName}/schemas/name/{SchemaName}` | `idempotency-token` | `RegistryName`, `SchemaName` | `ClientTokenId` | `UpdateSchemaResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException` | Updates the schema definition Inactive schemas will be deleted after two years. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteResourcePolicy` | - | `RegistryName -> registryName` | - | - |
| `DescribeCodeBinding` | - | `SchemaVersion -> schemaVersion` | - | - |
| `DescribeSchema` | - | `SchemaVersion -> schemaVersion` | - | - |
| `ExportSchema` | - | `SchemaVersion -> schemaVersion`, `Type -> type` | - | - |
| `GetCodeBindingSource` | - | `SchemaVersion -> schemaVersion` | - | - |
| `GetResourcePolicy` | - | `RegistryName -> registryName` | - | - |
| `ListDiscoverers` | - | `DiscovererIdPrefix -> discovererIdPrefix`, `Limit -> limit`, `NextToken -> nextToken`, `SourceArnPrefix -> sourceArnPrefix` | - | - |
| `ListRegistries` | - | `Limit -> limit`, `NextToken -> nextToken`, `RegistryNamePrefix -> registryNamePrefix`, `Scope -> scope` | - | - |
| `ListSchemas` | - | `Limit -> limit`, `NextToken -> nextToken`, `SchemaNamePrefix -> schemaNamePrefix` | - | - |
| `ListSchemaVersions` | - | `Limit -> limit`, `NextToken -> nextToken` | - | - |
| `PutCodeBinding` | - | `SchemaVersion -> schemaVersion` | - | - |
| `PutResourcePolicy` | - | `RegistryName -> registryName` | - | - |
| `SearchSchemas` | - | `Keywords -> keywords`, `Limit -> limit`, `NextToken -> nextToken` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | Code, Message | - |
| `ConflictException` | `structure` | Code, Message | - |
| `ForbiddenException` | `structure` | Code, Message | - |
| `GoneException` | `structure` | Code, Message | - |
| `InternalServerErrorException` | `structure` | Code, Message | - |
| `NotFoundException` | `structure` | Code, Message | - |
| `PreconditionFailedException` | `structure` | Code, Message | - |
| `ServiceUnavailableException` | `structure` | Code, Message | - |
| `TooManyRequestsException` | `structure` | Code, Message | - |
| `UnauthorizedException` | `structure` | Code, Message | - |
| `CreateDiscovererRequest` | `structure` | Description, SourceArn, CrossAccount, Tags | - |
| `CreateDiscovererResponse` | `structure` | Description, DiscovererArn, DiscovererId, SourceArn, State, CrossAccount, Tags | - |
| `CreateRegistryRequest` | `structure` | Description, RegistryName, Tags | - |
| `CreateRegistryResponse` | `structure` | Description, RegistryArn, RegistryName, Tags | - |
| `CreateSchemaRequest` | `structure` | Content, Description, RegistryName, SchemaName, Tags, Type | - |
| `CreateSchemaResponse` | `structure` | Description, LastModified, SchemaArn, SchemaName, SchemaVersion, Tags, Type, VersionCreatedDate | - |
| `DeleteDiscovererRequest` | `structure` | DiscovererId | - |
| `DeleteRegistryRequest` | `structure` | RegistryName | - |
| `DeleteResourcePolicyRequest` | `structure` | RegistryName | - |
| `DeleteSchemaRequest` | `structure` | RegistryName, SchemaName | - |
| `DeleteSchemaVersionRequest` | `structure` | RegistryName, SchemaName, SchemaVersion | - |
| `DescribeCodeBindingRequest` | `structure` | Language, RegistryName, SchemaName, SchemaVersion | - |
| `DescribeCodeBindingResponse` | `structure` | CreationDate, LastModified, SchemaVersion, Status | - |
| `DescribeDiscovererRequest` | `structure` | DiscovererId | - |
| `DescribeDiscovererResponse` | `structure` | Description, DiscovererArn, DiscovererId, SourceArn, State, CrossAccount, Tags | - |
| `DescribeRegistryRequest` | `structure` | RegistryName | - |
| `DescribeRegistryResponse` | `structure` | Description, RegistryArn, RegistryName, Tags | - |
| `DescribeSchemaRequest` | `structure` | RegistryName, SchemaName, SchemaVersion | - |
| `DescribeSchemaResponse` | `structure` | Content, Description, LastModified, SchemaArn, SchemaName, SchemaVersion, Tags, Type, VersionCreatedDate | - |
| `ExportSchemaRequest` | `structure` | RegistryName, SchemaName, SchemaVersion, Type | - |
| `ExportSchemaResponse` | `structure` | Content, SchemaArn, SchemaName, SchemaVersion, Type | - |
| `GetCodeBindingSourceRequest` | `structure` | Language, RegistryName, SchemaName, SchemaVersion | - |
| `GetCodeBindingSourceResponse` | `structure` | Body | - |
| `GetDiscoveredSchemaRequest` | `structure` | Events, Type | - |
| `GetDiscoveredSchemaResponse` | `structure` | Content | - |
| `GetResourcePolicyRequest` | `structure` | RegistryName | - |
| `GetResourcePolicyResponse` | `structure` | Policy, RevisionId | - |
| `ListDiscoverersRequest` | `structure` | DiscovererIdPrefix, Limit, NextToken, SourceArnPrefix | - |
| `ListDiscoverersResponse` | `structure` | Discoverers, NextToken | - |
| `ListRegistriesRequest` | `structure` | Limit, NextToken, RegistryNamePrefix, Scope | - |
| `CodeGenerationStatus` | `enum` | CREATE_IN_PROGRESS, CREATE_COMPLETE, CREATE_FAILED | - |
| `DiscovererState` | `enum` | STARTED, STOPPED | - |
| `Type` | `enum` | OpenApi3, JSONSchemaDraft4 | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
