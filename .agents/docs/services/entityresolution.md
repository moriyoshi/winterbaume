# AWS EntityResolution

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Entity Resolution API Reference . Entity Resolution is an Amazon Web Services service that provides pre-configured entity resolution capabilities that enable developers and analysts at advertising and marketing companies to build an accurate and complete view of their consumers. With Entity Resolution, you can match source records containing consumer identifiers, such as name, email address, and phone number. This is true even when these records have incomplete or conflicting identifiers. For example, Entity Resolution can effectively match a source record from a customer relationship management (CRM) system with a source record from a marketing system containing campaign information.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS EntityResolution resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS EntityResolution workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Delete`, `Create`, `Update` operation families, including `GetIdMappingJob`, `GetIdMappingWorkflow`, `GetIdNamespace`, `GetMatchId`, `ListIdMappingJobs`, `ListIdMappingWorkflows`.

## Service Identity and Protocol

- AWS model slug: `entityresolution`
- AWS SDK for Rust slug: `entityresolution`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/entityresolution/service/2018-05-10/entityresolution-2018-05-10.json`
- SDK ID: `EntityResolution`
- Endpoint prefix: `-`
- ARN namespace: `entityresolution`
- CloudFormation name: `-`
- CloudTrail event source: `entityresolution.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (9), `List` (8), `Delete` (5), `Create` (4), `Update` (4), `Start` (2), `Add` (1), `Batch` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddPolicyStatement`, `BatchDeleteUniqueId`, `CreateIdMappingWorkflow`, `CreateIdNamespace`, `CreateMatchingWorkflow`, `CreateSchemaMapping`, `DeleteIdMappingWorkflow`, `DeleteIdNamespace`, `DeleteMatchingWorkflow`, `DeletePolicyStatement`, `DeleteSchemaMapping`, `PutPolicy`, `StartIdMappingJob`, `StartMatchingJob`, `TagResource`, `UntagResource`, `UpdateIdMappingWorkflow`, `UpdateIdNamespace`, `UpdateMatchingWorkflow`, `UpdateSchemaMapping`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetIdMappingJob`, `GetIdMappingWorkflow`, `GetIdNamespace`, `GetMatchId`, `GetMatchingJob`, `GetMatchingWorkflow`, `GetPolicy`, `GetProviderService`, `GetSchemaMapping`, `ListIdMappingJobs`, `ListIdMappingWorkflows`, `ListIdNamespaces`, `ListMatchingJobs`, `ListMatchingWorkflows`, `ListProviderServices`, `ListSchemaMappings`, `ListTagsForResource`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 12 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetIdMappingJob`, `GetMatchingJob`, `ListIdMappingJobs`, `ListMatchingJobs`, `StartIdMappingJob`, `StartMatchingJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 38 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetIdMappingJob`, `GetIdMappingWorkflow`, `GetIdNamespace`, `GetMatchId`, `GetMatchingJob`, `GetMatchingWorkflow`, `GetPolicy`, `GetProviderService`, `GetSchemaMapping`
- Traits: `readonly` (9)
- Common required input members in this group: `workflowName`, `jobId`

### List

- Operations: `ListIdMappingJobs`, `ListIdMappingWorkflows`, `ListIdNamespaces`, `ListMatchingJobs`, `ListMatchingWorkflows`, `ListProviderServices`, `ListSchemaMappings`, `ListTagsForResource`
- Traits: `readonly` (8), `paginated` (7)
- Common required input members in this group: `workflowName`

### Delete

- Operations: `DeleteIdMappingWorkflow`, `DeleteIdNamespace`, `DeleteMatchingWorkflow`, `DeletePolicyStatement`, `DeleteSchemaMapping`
- Traits: `idempotent` (5)
- Common required input members in this group: `workflowName`

### Create

- Operations: `CreateIdMappingWorkflow`, `CreateIdNamespace`, `CreateMatchingWorkflow`, `CreateSchemaMapping`
- Common required input members in this group: `workflowName`, `inputSourceConfig`

### Update

- Operations: `UpdateIdMappingWorkflow`, `UpdateIdNamespace`, `UpdateMatchingWorkflow`, `UpdateSchemaMapping`
- Traits: `idempotent` (4)
- Common required input members in this group: `workflowName`, `inputSourceConfig`

### Start

- Operations: `StartIdMappingJob`, `StartMatchingJob`
- Common required input members in this group: `workflowName`

### Add

- Operations: `AddPolicyStatement`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Batch

- Operations: `BatchDeleteUniqueId`
- Common required input members in this group: -

### Generate

- Operations: `GenerateMatchId`
- Common required input members in this group: -

### Put

- Operations: `PutPolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddPolicyStatement` | `POST /policies/{arn}/{statementId}` | `idempotent` | `arn`, `statementId`, `effect`, `action`, `principal` | - | `AddPolicyStatementOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a policy statement object. To retrieve a list of existing policy statements, use the GetPolicy API. |
| `BatchDeleteUniqueId` | `DELETE /matchingworkflows/{workflowName}/uniqueids` | - | `workflowName`, `uniqueIds` | - | `BatchDeleteUniqueIdOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes multiple unique IDs in a matching workflow. |
| `CreateIdMappingWorkflow` | `POST /idmappingworkflows` | - | `workflowName`, `inputSourceConfig`, `idMappingTechniques` | - | `CreateIdMappingWorkflowOutput` | `AccessDeniedException`, `ConflictException`, `ExceedsLimitException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates an IdMappingWorkflow object which stores the configuration of the data processing job to be run. Each IdMappingWorkflow must have a unique workflow name. To modify an existing workflow, use the UpdateIdMappin ... |
| `CreateIdNamespace` | `POST /idnamespaces` | - | `idNamespaceName`, `type` | - | `CreateIdNamespaceOutput` | `AccessDeniedException`, `ConflictException`, `ExceedsLimitException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates an ID namespace object which will help customers provide metadata explaining their dataset and how to use it. Each ID namespace must have a unique name. To modify an existing ID namespace, use the UpdateIdNam ... |
| `CreateMatchingWorkflow` | `POST /matchingworkflows` | - | `workflowName`, `inputSourceConfig`, `outputSourceConfig`, `resolutionTechniques`, `roleArn` | - | `CreateMatchingWorkflowOutput` | `AccessDeniedException`, `ConflictException`, `ExceedsLimitException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a matching workflow that defines the configuration for a data processing job. The workflow name must be unique. To modify an existing workflow, use UpdateMatchingWorkflow . For workflows where resolutionType ... |
| `CreateSchemaMapping` | `POST /schemas` | - | `schemaName`, `mappedInputFields` | - | `CreateSchemaMappingOutput` | `AccessDeniedException`, `ConflictException`, `ExceedsLimitException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a schema mapping, which defines the schema of the input customer records table. The SchemaMapping also provides Entity Resolution with some metadata about the table, such as the attribute types of the columns ... |
| `DeleteIdMappingWorkflow` | `DELETE /idmappingworkflows/{workflowName}` | `idempotent` | `workflowName` | - | `DeleteIdMappingWorkflowOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the IdMappingWorkflow with a given name. This operation will succeed even if a workflow with the given name does not exist. |
| `DeleteIdNamespace` | `DELETE /idnamespaces/{idNamespaceName}` | `idempotent` | `idNamespaceName` | - | `DeleteIdNamespaceOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the IdNamespace with a given name. |
| `DeleteMatchingWorkflow` | `DELETE /matchingworkflows/{workflowName}` | `idempotent` | `workflowName` | - | `DeleteMatchingWorkflowOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the MatchingWorkflow with a given name. This operation will succeed even if a workflow with the given name does not exist. |
| `DeletePolicyStatement` | `DELETE /policies/{arn}/{statementId}` | `idempotent` | `arn`, `statementId` | - | `DeletePolicyStatementOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the policy statement. |
| `DeleteSchemaMapping` | `DELETE /schemas/{schemaName}` | `idempotent` | `schemaName` | - | `DeleteSchemaMappingOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the SchemaMapping with a given name. This operation will succeed even if a schema with the given name does not exist. This operation will fail if there is a MatchingWorkflow object that references the SchemaM ... |
| `GenerateMatchId` | `POST /matchingworkflows/{workflowName}/generateMatches` | - | `workflowName`, `records` | - | `GenerateMatchIdOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates or retrieves Match IDs for records using a rule-based matching workflow. When you call this operation, it processes your records against the workflow's matching rules to identify potential matches. For exis ... |
| `GetIdMappingJob` | `GET /idmappingworkflows/{workflowName}/jobs/{jobId}` | `readonly` | `workflowName`, `jobId` | - | `GetIdMappingJobOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the status, metrics, and errors (if there are any) that are associated with a job. |
| `GetIdMappingWorkflow` | `GET /idmappingworkflows/{workflowName}` | `readonly` | `workflowName` | - | `GetIdMappingWorkflowOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the IdMappingWorkflow with a given name, if it exists. |
| `GetIdNamespace` | `GET /idnamespaces/{idNamespaceName}` | `readonly` | `idNamespaceName` | - | `GetIdNamespaceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the IdNamespace with a given name, if it exists. |
| `GetMatchId` | `POST /matchingworkflows/{workflowName}/matches` | `readonly` | `workflowName`, `record` | - | `GetMatchIdOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the corresponding Match ID of a customer record if the record has been processed in a rule-based matching workflow. You can call this API as a dry run of an incremental load on the rule-based matching workflow. |
| `GetMatchingJob` | `GET /matchingworkflows/{workflowName}/jobs/{jobId}` | `readonly` | `workflowName`, `jobId` | - | `GetMatchingJobOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the status, metrics, and errors (if there are any) that are associated with a job. |
| `GetMatchingWorkflow` | `GET /matchingworkflows/{workflowName}` | `readonly` | `workflowName` | - | `GetMatchingWorkflowOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the MatchingWorkflow with a given name, if it exists. |
| `GetPolicy` | `GET /policies/{arn}` | `readonly` | `arn` | - | `GetPolicyOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the resource-based policy. |
| `GetProviderService` | `GET /providerservices/{providerName}/{providerServiceName}` | `readonly` | `providerName`, `providerServiceName` | - | `GetProviderServiceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the ProviderService of a given name. |
| `GetSchemaMapping` | `GET /schemas/{schemaName}` | `readonly` | `schemaName` | - | `GetSchemaMappingOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the SchemaMapping of a given name. |
| `ListIdMappingJobs` | `GET /idmappingworkflows/{workflowName}/jobs` | `readonly`, `paginated` | `workflowName` | - | `ListIdMappingJobsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all ID mapping jobs for a given workflow. |
| `ListIdMappingWorkflows` | `GET /idmappingworkflows` | `readonly`, `paginated` | - | - | `ListIdMappingWorkflowsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all the IdMappingWorkflows that have been created for an Amazon Web Services account. |
| `ListIdNamespaces` | `GET /idnamespaces` | `readonly`, `paginated` | - | - | `ListIdNamespacesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all ID namespaces. |
| `ListMatchingJobs` | `GET /matchingworkflows/{workflowName}/jobs` | `readonly`, `paginated` | `workflowName` | - | `ListMatchingJobsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all jobs for a given workflow. |
| `ListMatchingWorkflows` | `GET /matchingworkflows` | `readonly`, `paginated` | - | - | `ListMatchingWorkflowsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all the MatchingWorkflows that have been created for an Amazon Web Services account. |
| `ListProviderServices` | `GET /providerservices` | `readonly`, `paginated` | - | - | `ListProviderServicesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all the ProviderServices that are available in this Amazon Web Services Region. |
| `ListSchemaMappings` | `GET /schemas` | `readonly`, `paginated` | - | - | `ListSchemaMappingsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all the SchemaMappings that have been created for an Amazon Web Services account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays the tags associated with an Entity Resolution resource. In Entity Resolution, SchemaMapping , and MatchingWorkflow can be tagged. |
| `PutPolicy` | `PUT /policies/{arn}` | `idempotent` | `arn`, `policy` | - | `PutPolicyOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the resource-based policy. |
| `StartIdMappingJob` | `POST /idmappingworkflows/{workflowName}/jobs` | - | `workflowName` | - | `StartIdMappingJobOutput` | `AccessDeniedException`, `ConflictException`, `ExceedsLimitException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts the IdMappingJob of a workflow. The workflow must have previously been created using the CreateIdMappingWorkflow endpoint. |
| `StartMatchingJob` | `POST /matchingworkflows/{workflowName}/jobs` | - | `workflowName` | - | `StartMatchingJobOutput` | `AccessDeniedException`, `ConflictException`, `ExceedsLimitException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts the MatchingJob of a workflow. The workflow must have previously been created using the CreateMatchingWorkflow endpoint. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified Entity Resolution resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user per ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InternalServerException`, `ResourceNotFoundException` | Removes one or more tags from the specified Entity Resolution resource. In Entity Resolution, SchemaMapping , and MatchingWorkflow can be tagged. |
| `UpdateIdMappingWorkflow` | `PUT /idmappingworkflows/{workflowName}` | `idempotent` | `workflowName`, `inputSourceConfig`, `idMappingTechniques` | - | `UpdateIdMappingWorkflowOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing IdMappingWorkflow . This method is identical to CreateIdMappingWorkflow, except it uses an HTTP PUT request instead of a POST request, and the IdMappingWorkflow must already exist for the method t ... |
| `UpdateIdNamespace` | `PUT /idnamespaces/{idNamespaceName}` | `idempotent` | `idNamespaceName` | - | `UpdateIdNamespaceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing ID namespace. |
| `UpdateMatchingWorkflow` | `PUT /matchingworkflows/{workflowName}` | `idempotent` | `workflowName`, `inputSourceConfig`, `outputSourceConfig`, `resolutionTechniques`, `roleArn` | - | `UpdateMatchingWorkflowOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing matching workflow. The workflow must already exist for this operation to succeed. For workflows where resolutionType is ML_MATCHING or PROVIDER , incremental processing is not supported. |
| `UpdateSchemaMapping` | `PUT /schemas/{schemaName}` | `idempotent` | `schemaName`, `mappedInputFields` | - | `UpdateSchemaMappingOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a schema mapping. A schema is immutable if it is being used by a workflow. Therefore, you can't update a schema mapping if it's associated with a workflow. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `BatchDeleteUniqueId` | `inputSource -> inputSource`, `uniqueIds -> uniqueIds` | - | - | - |
| `ListIdMappingJobs` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListIdMappingWorkflows` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListIdNamespaces` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListMatchingJobs` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListMatchingWorkflows` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListProviderServices` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `providerName -> providerName` | - | - |
| `ListSchemaMappings` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | The request couldn't be processed because of conflict in the current state of the resource. Example: Workflow already exists, Schema already exists, Workflo ... |
| `ExceedsLimitException` | `structure` | message, quotaName, quotaValue | The request was rejected because it attempted to create resources beyond the current Entity Resolution account limits. The error message describes the limit ... |
| `InternalServerException` | `structure` | message | This exception occurs when there is an internal failure in the Entity Resolution service. |
| `ResourceNotFoundException` | `structure` | message | The resource couldn't be found. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by Entity Resolution. |
| `AddPolicyStatementInput` | `structure` | arn, statementId, effect, action, principal, condition | - |
| `AddPolicyStatementOutput` | `structure` | arn, token, policy | - |
| `BatchDeleteUniqueIdInput` | `structure` | workflowName, inputSource, uniqueIds | - |
| `BatchDeleteUniqueIdOutput` | `structure` | status, errors, deleted, disconnectedUniqueIds | - |
| `CreateIdMappingWorkflowInput` | `structure` | workflowName, description, inputSourceConfig, outputSourceConfig, idMappingTechniques, incrementalRunConfig, roleArn, tags | - |
| `CreateIdMappingWorkflowOutput` | `structure` | workflowName, workflowArn, description, inputSourceConfig, outputSourceConfig, idMappingTechniques, incrementalRunConfig, roleArn | - |
| `CreateIdNamespaceInput` | `structure` | idNamespaceName, description, inputSourceConfig, idMappingWorkflowProperties, type, roleArn, tags | - |
| `CreateIdNamespaceOutput` | `structure` | idNamespaceName, idNamespaceArn, description, inputSourceConfig, idMappingWorkflowProperties, type, roleArn, createdAt, updatedAt, tags | - |
| `CreateMatchingWorkflowInput` | `structure` | workflowName, description, inputSourceConfig, outputSourceConfig, resolutionTechniques, incrementalRunConfig, roleArn, tags | - |
| `CreateMatchingWorkflowOutput` | `structure` | workflowName, workflowArn, description, inputSourceConfig, outputSourceConfig, resolutionTechniques, incrementalRunConfig, roleArn | - |
| `CreateSchemaMappingInput` | `structure` | schemaName, description, mappedInputFields, tags | - |
| `CreateSchemaMappingOutput` | `structure` | schemaName, schemaArn, description, mappedInputFields | - |
| `DeleteIdMappingWorkflowInput` | `structure` | workflowName | - |
| `DeleteIdMappingWorkflowOutput` | `structure` | message | - |
| `DeleteIdNamespaceInput` | `structure` | idNamespaceName | - |
| `DeleteIdNamespaceOutput` | `structure` | message | - |
| `DeleteMatchingWorkflowInput` | `structure` | workflowName | - |
| `DeleteMatchingWorkflowOutput` | `structure` | message | - |
| `DeletePolicyStatementInput` | `structure` | arn, statementId | - |
| `DeletePolicyStatementOutput` | `structure` | arn, token, policy | - |
| `DeleteSchemaMappingInput` | `structure` | schemaName | - |
| `DeleteSchemaMappingOutput` | `structure` | message | - |
| `GenerateMatchIdInput` | `structure` | workflowName, records, processingType | - |
| `GenerateMatchIdOutput` | `structure` | matchGroups, failedRecords | - |
| `GetIdMappingJobInput` | `structure` | workflowName, jobId | - |
| `GetIdMappingJobOutput` | `structure` | jobId, status, startTime, endTime, metrics, errorDetails, outputSourceConfig, jobType | - |
| `GetIdMappingWorkflowInput` | `structure` | workflowName | - |
| `GetIdMappingWorkflowOutput` | `structure` | workflowName, workflowArn, description, inputSourceConfig, outputSourceConfig, idMappingTechniques, createdAt, updatedAt, incrementalRunConfig, roleArn, tags | - |
| `GetIdNamespaceInput` | `structure` | idNamespaceName | - |
| `GetIdNamespaceOutput` | `structure` | idNamespaceName, idNamespaceArn, description, inputSourceConfig, idMappingWorkflowProperties, type, roleArn, createdAt, updatedAt, tags | - |
| `GetMatchIdInput` | `structure` | workflowName, record, applyNormalization | - |
| `GetMatchIdOutput` | `structure` | matchId, matchRule | - |
| `GetMatchingJobInput` | `structure` | workflowName, jobId | - |
| `AttributeMatchingModel` | `enum` | ONE_TO_ONE, MANY_TO_MANY | - |
| `DeleteUniqueIdErrorType` | `enum` | SERVICE_ERROR, VALIDATION_ERROR | - |
| `DeleteUniqueIdStatus` | `enum` | COMPLETED, ACCEPTED | - |
| `IdMappingIncrementalRunType` | `enum` | ON_DEMAND | - |
| `IdMappingType` | `enum` | PROVIDER, RULE_BASED | - |
| `IdMappingWorkflowRuleDefinitionType` | `enum` | SOURCE, TARGET | - |
| `IdNamespaceType` | `enum` | SOURCE, TARGET | - |
| `IncrementalRunType` | `enum` | IMMEDIATE | - |
| `JobStatus` | `enum` | RUNNING, SUCCEEDED, FAILED, QUEUED | - |
| `JobType` | `enum` | BATCH, INCREMENTAL, DELETE_ONLY | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
