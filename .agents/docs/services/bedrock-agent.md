# Agents for Amazon Bedrock

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Describes the API operations for creating and managing Amazon Bedrock agents.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Agents for Amazon Bedrock where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Agents for Amazon Bedrock by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Agents for Amazon Bedrock resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: create agents, aliases, action groups, knowledge bases, data sources, flows, prompts, and agent versions.
- From the operation surface: support agent build/test/publish workflows, knowledge ingestion jobs, collaboration aliases, flow versions, guardrails, and tag-based AI application administration.

## Service Identity and Protocol

- AWS model slug: `bedrock-agent`
- AWS SDK for Rust slug: `bedrockagent`
- Model version: `2023-06-05`
- Model file: `vendor/api-models-aws/models/bedrock-agent/service/2023-06-05/bedrock-agent-2023-06-05.json`
- SDK ID: `Bedrock Agent`
- Endpoint prefix: `bedrock-agent`
- ARN namespace: `bedrock`
- CloudFormation name: `-`
- CloudTrail event source: `bedrock.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (15), `Get` (14), `Delete` (11), `Create` (10), `Update` (10), `Associate` (2), `Disassociate` (2), `Prepare` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAgentCollaborator`, `AssociateAgentKnowledgeBase`, `CreateAgent`, `CreateAgentActionGroup`, `CreateAgentAlias`, `CreateDataSource`, `CreateFlow`, `CreateFlowAlias`, `CreateFlowVersion`, `CreateKnowledgeBase`, `CreatePrompt`, `CreatePromptVersion`, `DeleteAgent`, `DeleteAgentActionGroup`, `DeleteAgentAlias`, `DeleteAgentVersion`, `DeleteDataSource`, `DeleteFlow`, `DeleteFlowAlias`, `DeleteFlowVersion`, ... (+19).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAgent`, `GetAgentActionGroup`, `GetAgentAlias`, `GetAgentCollaborator`, `GetAgentKnowledgeBase`, `GetAgentVersion`, `GetDataSource`, `GetFlow`, `GetFlowAlias`, `GetFlowVersion`, `GetIngestionJob`, `GetKnowledgeBase`, `GetKnowledgeBaseDocuments`, `GetPrompt`, `ListAgentActionGroups`, `ListAgentAliases`, `ListAgentCollaborators`, `ListAgentKnowledgeBases`, `ListAgentVersions`, `ListAgents`, ... (+10).
- Pagination is modelled for 14 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 39 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetIngestionJob`, `ListIngestionJobs`, `StartIngestionJob`, `StopIngestionJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 72 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Lambda`, `Glue`, `ECS`, `RDS`, `Redshift`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ActionGroupResource` | - | - | `CreateAgentActionGroup`, `DeleteAgentActionGroup`, `GetAgentActionGroup`, `ListAgentActionGroups`, `UpdateAgentActionGroup` | - |
| `AgentCollaboratorResource` | - | - | `AssociateAgentCollaborator`, `DisassociateAgentCollaborator`, `GetAgentCollaborator`, `ListAgentCollaborators`, `UpdateAgentCollaborator` | - |
| `AgentResource` | - | - | `CreateAgent`, `DeleteAgent`, `GetAgent`, `ListAgents`, `PrepareAgent`, `UpdateAgent` | - |
| `AliasResource` | - | - | `CreateAgentAlias`, `DeleteAgentAlias`, `GetAgentAlias`, `ListAgentAliases`, `UpdateAgentAlias` | - |
| `DataSourceResource` | - | - | `CreateDataSource`, `DeleteDataSource`, `GetDataSource`, `ListDataSources`, `UpdateDataSource` | - |
| `FlowAliasResource` | `aliasIdentifier`, `flowIdentifier` | create: `CreateFlowAlias`; read: `GetFlowAlias`; update: `UpdateFlowAlias`; delete: `DeleteFlowAlias`; list: `ListFlowAliases` | - | - |
| `FlowResource` | `flowIdentifier` | create: `CreateFlow`; read: `GetFlow`; update: `UpdateFlow`; delete: `DeleteFlow`; list: `ListFlows` | `PrepareFlow` | - |
| `FlowVersionResource` | `flowIdentifier`, `flowVersion` | create: `CreateFlowVersion`; read: `GetFlowVersion`; delete: `DeleteFlowVersion`; list: `ListFlowVersions` | - | - |
| `IngestionJobResource` | - | - | `GetIngestionJob`, `ListIngestionJobs`, `StartIngestionJob`, `StopIngestionJob` | - |
| `KnowledgeBaseDocumentResource` | - | - | `DeleteKnowledgeBaseDocuments`, `GetKnowledgeBaseDocuments`, `IngestKnowledgeBaseDocuments`, `ListKnowledgeBaseDocuments` | - |
| `KnowledgeBaseResource` | - | - | `AssociateAgentKnowledgeBase`, `CreateKnowledgeBase`, `DeleteKnowledgeBase`, `DisassociateAgentKnowledgeBase`, `GetAgentKnowledgeBase`, `GetKnowledgeBase`, `ListAgentKnowledgeBases`, `ListKnowledgeBases`, `UpdateAgentKnowledgeBase`, `UpdateKnowledgeBase` | - |
| `PromptResource` | `promptIdentifier` | create: `CreatePrompt`; read: `GetPrompt`; update: `UpdatePrompt`; delete: `DeletePrompt`; list: `ListPrompts` | `CreatePromptVersion` | - |
| `TaggingResource` | - | - | `ListTagsForResource`, `TagResource`, `UntagResource` | - |
| `VersionResource` | - | - | `DeleteAgentVersion`, `GetAgentVersion`, `ListAgentVersions` | - |
## Operation Groups

### Validate

- Operations: `ValidateFlowDefinition`
- Traits: `readonly` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ValidateFlowDefinition` | `POST /flows/validate-definition` | `readonly` | `definition` | - | `ValidateFlowDefinitionResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Validates the definition of a flow. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The request is denied because of missing access permissions. |
| `ConflictException` | `structure` | message | There was a conflict performing an operation. |
| `InternalServerException` | `structure` | message | An internal server error occurred. Retry your request. |
| `ResourceNotFoundException` | `structure` | message | The specified resource Amazon Resource Name (ARN) was not found. Check the Amazon Resource Name (ARN) and try your request again. |
| `ServiceQuotaExceededException` | `structure` | message | The number of requests exceeds the service quota. Resubmit your request later. |
| `ThrottlingException` | `structure` | message | The number of requests exceeds the limit. Resubmit your request later. |
| `ValidationException` | `structure` | message, fieldList | Input validation failed. Check your request parameters and retry the request. |
| `ValidateFlowDefinitionRequest` | `structure` | definition | - |
| `ValidateFlowDefinitionResponse` | `structure` | validations | - |
| `ActionGroupSignature` | `enum` | AMAZON_USERINPUT, AMAZON_CODEINTERPRETER, ANTHROPIC_COMPUTER, ANTHROPIC_BASH, ANTHROPIC_TEXTEDITOR | - |
| `ActionGroupState` | `enum` | ENABLED, DISABLED | - |
| `AgentAliasStatus` | `enum` | CREATING, PREPARED, FAILED, UPDATING, DELETING, DISSOCIATED | - |
| `AgentCollaboration` | `enum` | SUPERVISOR, SUPERVISOR_ROUTER, DISABLED | - |
| `AgentStatus` | `enum` | CREATING, PREPARING, PREPARED, NOT_PREPARED, DELETING, FAILED, VERSIONING, UPDATING | - |
| `AliasInvocationState` | `enum` | ACCEPT_INVOCATIONS, REJECT_INVOCATIONS | Enum representing the invocation state of an agent alias |
| `CachePointType` | `enum` | DEFAULT | - |
| `ChunkingStrategy` | `enum` | FIXED_SIZE, NONE, HIERARCHICAL, SEMANTIC | - |
| `ConcurrencyType` | `enum` | AUTOMATIC, MANUAL | - |
| `ConfluenceAuthType` | `enum` | BASIC, OAUTH2_CLIENT_CREDENTIALS | - |
| `ConfluenceHostType` | `enum` | SAAS | - |
| `ContentDataSourceType` | `enum` | CUSTOM, S3 | - |
| `ContextEnrichmentType` | `enum` | BEDROCK_FOUNDATION_MODEL | - |
| `ConversationRole` | `enum` | USER, ASSISTANT | - |
| `CrawlFilterConfigurationType` | `enum` | PATTERN | - |
| `CreationMode` | `enum` | DEFAULT, OVERRIDDEN | - |
| `CustomControlMethod` | `enum` | RETURN_CONTROL | - |
| `CustomSourceType` | `enum` | IN_LINE, S3_LOCATION | - |
| `DataDeletionPolicy` | `enum` | RETAIN, DELETE | - |
| `DataSourceStatus` | `enum` | AVAILABLE, DELETING, DELETE_UNSUCCESSFUL | - |
| `DataSourceType` | `enum` | S3, WEB, CONFLUENCE, SALESFORCE, SHAREPOINT, CUSTOM, REDSHIFT_METADATA | - |
| `DocumentStatus` | `enum` | INDEXED, PARTIALLY_INDEXED, PENDING, FAILED, METADATA_PARTIALLY_INDEXED, METADATA_UPDATE_FAILED, IGNORED, NOT_FOUND, STARTING, IN_PROGRESS, DELETING, DELETE_IN_PROGRESS | - |
| `EmbeddingDataType` | `enum` | FLOAT32, BINARY | Bedrock models embedding data type. Can be either float32 or binary. |
| `EnrichmentStrategyMethod` | `enum` | CHUNK_ENTITY_EXTRACTION | - |
| `FlowConnectionType` | `enum` | DATA, CONDITIONAL | - |
| `FlowNodeIODataType` | `enum` | STRING, NUMBER, BOOLEAN, OBJECT, ARRAY | - |
| `FlowNodeInputCategory` | `enum` | LOOP_CONDITION, RETURN_VALUE_TO_LOOP_START, EXIT_LOOP | - |
| `FlowNodeType` | `enum` | INPUT, OUTPUT, KNOWLEDGE_BASE, CONDITION, LEX, PROMPT, LAMBDA_FUNCTION, STORAGE, AGENT, RETRIEVAL, ITERATOR, COLLECTOR, ... (+4) | - |
| `FlowStatus` | `enum` | FAILED, PREPARED, PREPARING, NOT_PREPARED | - |
| `FlowValidationSeverity` | `enum` | WARNING, ERROR | - |
| `FlowValidationType` | `enum` | CYCLIC_CONNECTION, DUPLICATE_CONNECTIONS, DUPLICATE_CONDITION_EXPRESSION, UNREACHABLE_NODE, UNKNOWN_CONNECTION_SOURCE, UNKNOWN_CONNECTION_SOURCE_OUTPUT, UNKNOWN_CONNECTION_TARGET, UNKNOWN_CONNECTION_TARGET_INPUT, UNKNOWN_CONNECTION_CONDITION, MALFORMED_CONDITION_EXPRESSION, MALFORMED_NODE_INPUT_EXPRESSION, MISMATCHED_NODE_INPUT_TYPE, ... (+21) | - |
| `IncludeExclude` | `enum` | INCLUDE, EXCLUDE | - |
| `IncompatibleLoopNodeType` | `enum` | INPUT, CONDITION, ITERATOR, COLLECTOR | - |
| `IngestionJobFilterAttribute` | `enum` | STATUS | - |
| `IngestionJobFilterOperator` | `enum` | EQ | - |
| `IngestionJobSortByAttribute` | `enum` | STATUS, STARTED_AT | - |
| `IngestionJobStatus` | `enum` | STARTING, IN_PROGRESS, COMPLETE, FAILED, STOPPING, STOPPED | - |
| `InlineContentType` | `enum` | BYTE, TEXT | - |
| `KnowledgeBaseState` | `enum` | ENABLED, DISABLED | - |
| `KnowledgeBaseStatus` | `enum` | CREATING, ACTIVE, DELETING, UPDATING, FAILED, DELETE_UNSUCCESSFUL | - |
| `KnowledgeBaseStorageType` | `enum` | OPENSEARCH_SERVERLESS, PINECONE, REDIS_ENTERPRISE_CLOUD, RDS, MONGO_DB_ATLAS, NEPTUNE_ANALYTICS, OPENSEARCH_MANAGED_CLUSTER, S3_VECTORS | - |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/rule-evaluator-and-validator-crates.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: full distillation.

### Flow Parser and Validator

- Bedrock flow parsing and validation live in two pure crates: `winterbaume-bedrock-flow-parser` for typed graph modelling and `winterbaume-bedrock-flow-validator` for structural and type-oriented checks.
- Parser responsibilities include node-type discrimination, node configuration enums, connection modelling, and nested loop-definition parsing.
- Validator responsibilities include graph reachability, cycle detection, connection source or target validation, missing-node-input or missing-node-output checks, condition wiring rules, and loop-boundary or loop-role checks.

### Service Boundary

- Future flow parity rules should be added to the parser or validator crates, not as handler-local heuristics in `winterbaume-bedrockagent`.
- Handler work should translate Bedrock Agent request payloads into parser input, invoke validation, and shape service errors or response fields. Keep graph semantics reusable outside the HTTP layer.
- Tests for flow operations should include both accepted graph definitions and AWS-shaped validation failures for missing nodes, broken connections, invalid conditions, and loop misuse.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
