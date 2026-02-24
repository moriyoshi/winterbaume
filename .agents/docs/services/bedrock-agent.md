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

### List

- Operations: `ListAgentActionGroups`, `ListAgentAliases`, `ListAgentCollaborators`, `ListAgentKnowledgeBases`, `ListAgentVersions`, `ListAgents`, `ListDataSources`, `ListFlowAliases`, `ListFlowVersions`, `ListFlows`, `ListIngestionJobs`, `ListKnowledgeBaseDocuments`, `ListKnowledgeBases`, `ListPrompts`, `ListTagsForResource`
- Traits: `paginated` (14), `readonly` (15)
- Common required input members in this group: `agentId`, `agentVersion`, `dataSourceId`, `flowIdentifier`, `knowledgeBaseId`, `resourceArn`

### Get

- Operations: `GetAgent`, `GetAgentActionGroup`, `GetAgentAlias`, `GetAgentCollaborator`, `GetAgentKnowledgeBase`, `GetAgentVersion`, `GetDataSource`, `GetFlow`, `GetFlowAlias`, `GetFlowVersion`, `GetIngestionJob`, `GetKnowledgeBase`, `GetKnowledgeBaseDocuments`, `GetPrompt`
- Traits: `readonly` (14)
- Common required input members in this group: `actionGroupId`, `agentAliasId`, `agentId`, `agentVersion`, `aliasIdentifier`, `collaboratorId`, `dataSourceId`, `documentIdentifiers`, `flowIdentifier`, `flowVersion`, `ingestionJobId`, `knowledgeBaseId`, `promptIdentifier`

### Delete

- Operations: `DeleteAgent`, `DeleteAgentActionGroup`, `DeleteAgentAlias`, `DeleteAgentVersion`, `DeleteDataSource`, `DeleteFlow`, `DeleteFlowAlias`, `DeleteFlowVersion`, `DeleteKnowledgeBase`, `DeleteKnowledgeBaseDocuments`, `DeletePrompt`
- Traits: `idempotency-token` (1), `idempotent` (11)
- Common required input members in this group: `actionGroupId`, `agentAliasId`, `agentId`, `agentVersion`, `aliasIdentifier`, `dataSourceId`, `documentIdentifiers`, `flowIdentifier`, `flowVersion`, `knowledgeBaseId`, `promptIdentifier`

### Create

- Operations: `CreateAgent`, `CreateAgentActionGroup`, `CreateAgentAlias`, `CreateDataSource`, `CreateFlow`, `CreateFlowAlias`, `CreateFlowVersion`, `CreateKnowledgeBase`, `CreatePrompt`, `CreatePromptVersion`
- Traits: `idempotency-token` (10), `idempotent` (10)
- Common required input members in this group: `actionGroupName`, `agentAliasName`, `agentId`, `agentName`, `agentVersion`, `dataSourceConfiguration`, `executionRoleArn`, `flowIdentifier`, `knowledgeBaseConfiguration`, `knowledgeBaseId`, `name`, `promptIdentifier`, `roleArn`, `routingConfiguration`

### Update

- Operations: `UpdateAgent`, `UpdateAgentActionGroup`, `UpdateAgentAlias`, `UpdateAgentCollaborator`, `UpdateAgentKnowledgeBase`, `UpdateDataSource`, `UpdateFlow`, `UpdateFlowAlias`, `UpdateKnowledgeBase`, `UpdatePrompt`
- Traits: `idempotent` (10)
- Common required input members in this group: `actionGroupId`, `actionGroupName`, `agentAliasId`, `agentAliasName`, `agentDescriptor`, `agentId`, `agentName`, `agentResourceRoleArn`, `agentVersion`, `aliasIdentifier`, `collaborationInstruction`, `collaboratorId`, `collaboratorName`, `dataSourceConfiguration`, `dataSourceId`, `executionRoleArn`, `flowIdentifier`, `foundationModel`, `knowledgeBaseConfiguration`, `knowledgeBaseId`, `name`, `promptIdentifier`, `roleArn`, `routingConfiguration`

### Associate

- Operations: `AssociateAgentCollaborator`, `AssociateAgentKnowledgeBase`
- Traits: `idempotency-token` (1), `idempotent` (2)
- Common required input members in this group: `agentDescriptor`, `agentId`, `agentVersion`, `collaborationInstruction`, `collaboratorName`, `description`, `knowledgeBaseId`

### Disassociate

- Operations: `DisassociateAgentCollaborator`, `DisassociateAgentKnowledgeBase`
- Traits: `idempotent` (2)
- Common required input members in this group: `agentId`, `agentVersion`, `collaboratorId`, `knowledgeBaseId`

### Prepare

- Operations: `PrepareAgent`, `PrepareFlow`
- Common required input members in this group: `agentId`, `flowIdentifier`

### Ingest

- Operations: `IngestKnowledgeBaseDocuments`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `dataSourceId`, `documents`, `knowledgeBaseId`

### Start

- Operations: `StartIngestionJob`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `dataSourceId`, `knowledgeBaseId`

### Stop

- Operations: `StopIngestionJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `dataSourceId`, `ingestionJobId`, `knowledgeBaseId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Validate

- Operations: `ValidateFlowDefinition`
- Traits: `readonly` (1)
- Common required input members in this group: `definition`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAgentCollaborator` | `PUT /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/` | `idempotent`, `idempotency-token` | `agentDescriptor`, `agentId`, `agentVersion`, `collaborationInstruction`, `collaboratorName` | `clientToken` | `AssociateAgentCollaboratorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Makes an agent a collaborator for another agent. |
| `AssociateAgentKnowledgeBase` | `PUT /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/` | `idempotent` | `agentId`, `agentVersion`, `description`, `knowledgeBaseId` | - | `AssociateAgentKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a knowledge base with an agent. If a knowledge base is associated and its `indexState` is set to `Enabled`, the agent queries the knowledge base for information to augment its response to the user. |
| `CreateAgent` | `PUT /agents/` | `idempotent`, `idempotency-token` | `agentName` | `clientToken` | `CreateAgentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an agent that orchestrates interactions between foundation models, data sources, software applications, user conversations, and APIs to carry out tasks to help customers. Specify the following fields for security purposes. |
| `CreateAgentActionGroup` | `PUT /agents/{agentId}/agentversions/{agentVersion}/actiongroups/` | `idempotent`, `idempotency-token` | `actionGroupName`, `agentId`, `agentVersion` | `clientToken` | `CreateAgentActionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an action group for an agent. An action group represents the actions that an agent can carry out for the customer by defining the APIs that an agent can call and the logic for calling them. |
| `CreateAgentAlias` | `PUT /agents/{agentId}/agentaliases/` | `idempotent`, `idempotency-token` | `agentAliasName`, `agentId` | `clientToken` | `CreateAgentAliasResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an alias of an agent that can be used to deploy the agent. |
| `CreateDataSource` | `PUT /knowledgebases/{knowledgeBaseId}/datasources/` | `idempotent`, `idempotency-token` | `dataSourceConfiguration`, `knowledgeBaseId`, `name` | `clientToken` | `CreateDataSourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Connects a knowledge base to a data source. You specify the configuration for the specific data source service in the `dataSourceConfiguration` field. |
| `CreateFlow` | `POST /flows/` | `idempotent`, `idempotency-token` | `executionRoleArn`, `name` | `clientToken` | `CreateFlowResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a prompt flow that you can use to send an input through various steps to yield an output. Configure nodes, each of which corresponds to a step of the flow, and create connections between the nodes to create paths to different outputs. |
| `CreateFlowAlias` | `POST /flows/{flowIdentifier}/aliases` | `idempotent`, `idempotency-token` | `flowIdentifier`, `name`, `routingConfiguration` | `clientToken` | `CreateFlowAliasResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an alias of a flow for deployment. For more information, see Deploy a flow in Amazon Bedrock in the Amazon Bedrock User Guide. |
| `CreateFlowVersion` | `POST /flows/{flowIdentifier}/versions` | `idempotent`, `idempotency-token` | `flowIdentifier` | `clientToken` | `CreateFlowVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a version of the flow that you can deploy. For more information, see Deploy a flow in Amazon Bedrock in the Amazon Bedrock User Guide. |
| `CreateKnowledgeBase` | `PUT /knowledgebases/` | `idempotent`, `idempotency-token` | `knowledgeBaseConfiguration`, `name`, `roleArn` | `clientToken` | `CreateKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a knowledge base. A knowledge base contains your data sources so that Large Language Models (LLMs) can use your data. |
| `CreatePrompt` | `POST /prompts/` | `idempotent`, `idempotency-token` | `name` | `clientToken` | `CreatePromptResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a prompt in your prompt library that you can add to a flow. For more information, see Prompt management in Amazon Bedrock, Create a prompt using Prompt management and Prompt flows in Amazon Bedrock in the Amazon Bedrock User Guide. |
| `CreatePromptVersion` | `POST /prompts/{promptIdentifier}/versions` | `idempotent`, `idempotency-token` | `promptIdentifier` | `clientToken` | `CreatePromptVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a static snapshot of your prompt that can be deployed to production. For more information, see Deploy prompts using Prompt management by creating versions in the Amazon Bedrock User Guide. |
| `DeleteAgent` | `DELETE /agents/{agentId}/` | `idempotent` | `agentId` | - | `DeleteAgentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an agent. |
| `DeleteAgentActionGroup` | `DELETE /agents/{agentId}/agentversions/{agentVersion}/actiongroups/{actionGroupId}/` | `idempotent` | `actionGroupId`, `agentId`, `agentVersion` | - | `DeleteAgentActionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an action group in an agent. |
| `DeleteAgentAlias` | `DELETE /agents/{agentId}/agentaliases/{agentAliasId}/` | `idempotent` | `agentAliasId`, `agentId` | - | `DeleteAgentAliasResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an alias of an agent. |
| `DeleteAgentVersion` | `DELETE /agents/{agentId}/agentversions/{agentVersion}/` | `idempotent` | `agentId`, `agentVersion` | - | `DeleteAgentVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a version of an agent. |
| `DeleteDataSource` | `DELETE /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}` | `idempotent` | `dataSourceId`, `knowledgeBaseId` | - | `DeleteDataSourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a data source from a knowledge base. |
| `DeleteFlow` | `DELETE /flows/{flowIdentifier}/` | `idempotent` | `flowIdentifier` | - | `DeleteFlowResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a flow. |
| `DeleteFlowAlias` | `DELETE /flows/{flowIdentifier}/aliases/{aliasIdentifier}` | `idempotent` | `aliasIdentifier`, `flowIdentifier` | - | `DeleteFlowAliasResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an alias of a flow. |
| `DeleteFlowVersion` | `DELETE /flows/{flowIdentifier}/versions/{flowVersion}/` | `idempotent` | `flowIdentifier`, `flowVersion` | - | `DeleteFlowVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a version of a flow. |
| `DeleteKnowledgeBase` | `DELETE /knowledgebases/{knowledgeBaseId}` | `idempotent` | `knowledgeBaseId` | - | `DeleteKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a knowledge base. Before deleting a knowledge base, you should disassociate the knowledge base from any agents that it is associated with by making a DisassociateAgentKnowledgeBase request. |
| `DeleteKnowledgeBaseDocuments` | `POST /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}/documents/deleteDocuments` | `idempotent`, `idempotency-token` | `dataSourceId`, `documentIdentifiers`, `knowledgeBaseId` | `clientToken` | `DeleteKnowledgeBaseDocumentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes documents from a data source and syncs the changes to the knowledge base that is connected to it. For more information, see Ingest changes directly into a knowledge base in the Amazon Bedrock User Guide. |
| `DeletePrompt` | `DELETE /prompts/{promptIdentifier}/` | `idempotent` | `promptIdentifier` | - | `DeletePromptResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a prompt or a version of it, depending on whether you include the `promptVersion` field or not. For more information, see Delete prompts from the Prompt management tool and Delete a version of a prompt from the Prompt management tool in the Amazon... |
| `DisassociateAgentCollaborator` | `DELETE /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/{collaboratorId}/` | `idempotent` | `agentId`, `agentVersion`, `collaboratorId` | - | `DisassociateAgentCollaboratorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates an agent collaborator. |
| `DisassociateAgentKnowledgeBase` | `DELETE /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/{knowledgeBaseId}/` | `idempotent` | `agentId`, `agentVersion`, `knowledgeBaseId` | - | `DisassociateAgentKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a knowledge base from an agent. |
| `GetAgent` | `GET /agents/{agentId}/` | `readonly` | `agentId` | - | `GetAgentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about an agent. |
| `GetAgentActionGroup` | `GET /agents/{agentId}/agentversions/{agentVersion}/actiongroups/{actionGroupId}/` | `readonly` | `actionGroupId`, `agentId`, `agentVersion` | - | `GetAgentActionGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about an action group for an agent. |
| `GetAgentAlias` | `GET /agents/{agentId}/agentaliases/{agentAliasId}/` | `readonly` | `agentAliasId`, `agentId` | - | `GetAgentAliasResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about an alias of an agent. |
| `GetAgentCollaborator` | `GET /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/{collaboratorId}/` | `readonly` | `agentId`, `agentVersion`, `collaboratorId` | - | `GetAgentCollaboratorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an agent's collaborator. |
| `GetAgentKnowledgeBase` | `GET /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/{knowledgeBaseId}/` | `readonly` | `agentId`, `agentVersion`, `knowledgeBaseId` | - | `GetAgentKnowledgeBaseResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a knowledge base associated with an agent. |
| `GetAgentVersion` | `GET /agents/{agentId}/agentversions/{agentVersion}/` | `readonly` | `agentId`, `agentVersion` | - | `GetAgentVersionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets details about a version of an agent. |
| `GetDataSource` | `GET /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}` | `readonly` | `dataSourceId`, `knowledgeBaseId` | - | `GetDataSourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a data source. |
| `GetFlow` | `GET /flows/{flowIdentifier}/` | `readonly` | `flowIdentifier` | - | `GetFlowResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a flow. For more information, see Manage a flow in Amazon Bedrock in the Amazon Bedrock User Guide. |
| `GetFlowAlias` | `GET /flows/{flowIdentifier}/aliases/{aliasIdentifier}` | `readonly` | `aliasIdentifier`, `flowIdentifier` | - | `GetFlowAliasResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a flow. For more information, see Deploy a flow in Amazon Bedrock in the Amazon Bedrock User Guide. |
| `GetFlowVersion` | `GET /flows/{flowIdentifier}/versions/{flowVersion}/` | `readonly` | `flowIdentifier`, `flowVersion` | - | `GetFlowVersionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a version of a flow. For more information, see Deploy a flow in Amazon Bedrock in the Amazon Bedrock User Guide. |
| `GetIngestionJob` | `GET /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}/ingestionjobs/{ingestionJobId}` | `readonly` | `dataSourceId`, `ingestionJobId`, `knowledgeBaseId` | - | `GetIngestionJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a data ingestion job. Data sources are ingested into your knowledge base so that Large Language Models (LLMs) can use your data. |
| `GetKnowledgeBase` | `GET /knowledgebases/{knowledgeBaseId}` | `readonly` | `knowledgeBaseId` | - | `GetKnowledgeBaseResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a knowledge base. |
| `GetKnowledgeBaseDocuments` | `POST /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}/documents/getDocuments` | `readonly` | `dataSourceId`, `documentIdentifiers`, `knowledgeBaseId` | - | `GetKnowledgeBaseDocumentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves specific documents from a data source that is connected to a knowledge base. For more information, see Ingest changes directly into a knowledge base in the Amazon Bedrock User Guide. |
| `GetPrompt` | `GET /prompts/{promptIdentifier}/` | `readonly` | `promptIdentifier` | - | `GetPromptResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the working draft (`DRAFT` version) of a prompt or a version of it, depending on whether you include the `promptVersion` field or not. For more information, see View information about prompts using Prompt management and View... |
| `IngestKnowledgeBaseDocuments` | `PUT /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}/documents` | `idempotent`, `idempotency-token` | `dataSourceId`, `documents`, `knowledgeBaseId` | `clientToken` | `IngestKnowledgeBaseDocumentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Ingests documents directly into the knowledge base that is connected to the data source. The `dataSourceType` specified in the content for each document must match the type of the data source that you specify in the header. |
| `ListAgentActionGroups` | `POST /agents/{agentId}/agentversions/{agentVersion}/actiongroups/` | `readonly`, `paginated` | `agentId`, `agentVersion` | - | `ListAgentActionGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the action groups for an agent and information about each one. |
| `ListAgentAliases` | `POST /agents/{agentId}/agentaliases/` | `readonly`, `paginated` | `agentId` | - | `ListAgentAliasesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the aliases of an agent and information about each one. |
| `ListAgentCollaborators` | `POST /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/` | `readonly`, `paginated` | `agentId`, `agentVersion` | - | `ListAgentCollaboratorsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieve a list of an agent's collaborators. |
| `ListAgentKnowledgeBases` | `POST /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/` | `readonly`, `paginated` | `agentId`, `agentVersion` | - | `ListAgentKnowledgeBasesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists knowledge bases associated with an agent and information about each one. |
| `ListAgentVersions` | `POST /agents/{agentId}/agentversions/` | `readonly`, `paginated` | `agentId` | - | `ListAgentVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the versions of an agent and information about each version. |
| `ListAgents` | `POST /agents/` | `readonly`, `paginated` | - | - | `ListAgentsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the agents belonging to an account and information about each agent. |
| `ListDataSources` | `POST /knowledgebases/{knowledgeBaseId}/datasources/` | `readonly`, `paginated` | `knowledgeBaseId` | - | `ListDataSourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the data sources in a knowledge base and information about each one. |
| `ListFlowAliases` | `GET /flows/{flowIdentifier}/aliases` | `readonly`, `paginated` | `flowIdentifier` | - | `ListFlowAliasesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of aliases for a flow. |
| `ListFlowVersions` | `GET /flows/{flowIdentifier}/versions` | `readonly`, `paginated` | `flowIdentifier` | - | `ListFlowVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of information about each flow. For more information, see Deploy a flow in Amazon Bedrock in the Amazon Bedrock User Guide. |
| `ListFlows` | `GET /flows/` | `readonly`, `paginated` | - | - | `ListFlowsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of flows and information about each flow. For more information, see Manage a flow in Amazon Bedrock in the Amazon Bedrock User Guide. |
| `ListIngestionJobs` | `POST /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}/ingestionjobs/` | `readonly`, `paginated` | `dataSourceId`, `knowledgeBaseId` | - | `ListIngestionJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the data ingestion jobs for a data source. The list also includes information about each job. |
| `ListKnowledgeBaseDocuments` | `POST /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}/documents` | `readonly`, `paginated` | `dataSourceId`, `knowledgeBaseId` | - | `ListKnowledgeBaseDocumentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves all the documents contained in a data source that is connected to a knowledge base. For more information, see Ingest changes directly into a knowledge base in the Amazon Bedrock User Guide. |
| `ListKnowledgeBases` | `POST /knowledgebases/` | `readonly`, `paginated` | - | - | `ListKnowledgeBasesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the knowledge bases in an account. The list also includesinformation about each knowledge base. |
| `ListPrompts` | `GET /prompts/` | `readonly`, `paginated` | - | - | `ListPromptsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns either information about the working draft (`DRAFT` version) of each prompt in an account, or information about of all versions of a prompt, depending on whether you include the `promptIdentifier` field or not. For more information, see View... |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all the tags for the resource you specify. |
| `PrepareAgent` | `POST /agents/{agentId}/` | - | `agentId` | - | `PrepareAgentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a `DRAFT` version of the agent that can be used for internal testing. |
| `PrepareFlow` | `POST /flows/{flowIdentifier}/` | - | `flowIdentifier` | - | `PrepareFlowResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Prepares the `DRAFT` version of a flow so that it can be invoked. For more information, see Test a flow in Amazon Bedrock in the Amazon Bedrock User Guide. |
| `StartIngestionJob` | `PUT /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}/ingestionjobs/` | `idempotent`, `idempotency-token` | `dataSourceId`, `knowledgeBaseId` | `clientToken` | `StartIngestionJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Begins a data ingestion job. Data sources are ingested into your knowledge base so that Large Language Models (LLMs) can use your data. |
| `StopIngestionJob` | `POST /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}/ingestionjobs/{ingestionJobId}/stop` | `idempotent` | `dataSourceId`, `ingestionJobId`, `knowledgeBaseId` | - | `StopIngestionJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops a currently running data ingestion job. You can send a `StartIngestionJob` request again to ingest the rest of your data when you are ready. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associate tags with a resource. For more information, see Tagging resources in the Amazon Bedrock User Guide. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove tags from a resource. |
| `UpdateAgent` | `PUT /agents/{agentId}/` | `idempotent` | `agentId`, `agentName`, `agentResourceRoleArn`, `foundationModel` | - | `UpdateAgentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an agent. |
| `UpdateAgentActionGroup` | `PUT /agents/{agentId}/agentversions/{agentVersion}/actiongroups/{actionGroupId}/` | `idempotent` | `actionGroupId`, `actionGroupName`, `agentId`, `agentVersion` | - | `UpdateAgentActionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration for an action group for an agent. |
| `UpdateAgentAlias` | `PUT /agents/{agentId}/agentaliases/{agentAliasId}/` | `idempotent` | `agentAliasId`, `agentAliasName`, `agentId` | - | `UpdateAgentAliasResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates configurations for an alias of an agent. |
| `UpdateAgentCollaborator` | `PUT /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/{collaboratorId}/` | `idempotent` | `agentDescriptor`, `agentId`, `agentVersion`, `collaborationInstruction`, `collaboratorId`, `collaboratorName` | - | `UpdateAgentCollaboratorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an agent's collaborator. |
| `UpdateAgentKnowledgeBase` | `PUT /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/{knowledgeBaseId}/` | `idempotent` | `agentId`, `agentVersion`, `knowledgeBaseId` | - | `UpdateAgentKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration for a knowledge base that has been associated with an agent. |
| `UpdateDataSource` | `PUT /knowledgebases/{knowledgeBaseId}/datasources/{dataSourceId}` | `idempotent` | `dataSourceConfiguration`, `dataSourceId`, `knowledgeBaseId`, `name` | - | `UpdateDataSourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configurations for a data source connector. You can't change the `chunkingConfiguration` after you create the data source connector. |
| `UpdateFlow` | `PUT /flows/{flowIdentifier}/` | `idempotent` | `executionRoleArn`, `flowIdentifier`, `name` | - | `UpdateFlowResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Modifies a flow. Include both fields that you want to keep and fields that you want to change. |
| `UpdateFlowAlias` | `PUT /flows/{flowIdentifier}/aliases/{aliasIdentifier}` | `idempotent` | `aliasIdentifier`, `flowIdentifier`, `name`, `routingConfiguration` | - | `UpdateFlowAliasResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Modifies the alias of a flow. Include both fields that you want to keep and ones that you want to change. |
| `UpdateKnowledgeBase` | `PUT /knowledgebases/{knowledgeBaseId}` | `idempotent` | `knowledgeBaseConfiguration`, `knowledgeBaseId`, `name`, `roleArn` | - | `UpdateKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of a knowledge base with the fields that you specify. Because all fields will be overwritten, you must include the same values for fields that you want to keep the same. |
| `UpdatePrompt` | `PUT /prompts/{promptIdentifier}/` | `idempotent` | `name`, `promptIdentifier` | - | `UpdatePromptResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Modifies a prompt in your prompt library. Include both fields that you want to keep and fields that you want to replace. |
| `ValidateFlowDefinition` | `POST /flows/validate-definition` | `readonly` | `definition` | - | `ValidateFlowDefinitionResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Validates the definition of a flow. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | The request is denied because of missing access permissions. |
| `InternalServerException` | `structure` | `message` | An internal server error occurred. |
| `ThrottlingException` | `structure` | `message` | The number of requests exceeds the limit. |
| `ValidationException` | `structure` | `fieldList`, `message` | Input validation failed. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource Amazon Resource Name (ARN) was not found. |
| `ConflictException` | `structure` | `message` | There was a conflict performing an operation. |
| `ServiceQuotaExceededException` | `structure` | `message` | The number of requests exceeds the service quota. |
| `AssociateAgentCollaboratorRequest` | `structure` | `agentDescriptor`, `agentId`, `agentVersion`, `clientToken`, `collaborationInstruction`, `collaboratorName`, `relayConversationHistory` | - |
| `AssociateAgentCollaboratorResponse` | `structure` | `agentCollaborator` | - |
| `AssociateAgentKnowledgeBaseRequest` | `structure` | `agentId`, `agentVersion`, `description`, `knowledgeBaseId`, `knowledgeBaseState` | - |
| `AssociateAgentKnowledgeBaseResponse` | `structure` | `agentKnowledgeBase` | - |
| `CreateAgentRequest` | `structure` | `agentCollaboration`, `agentName`, `agentResourceRoleArn`, `clientToken`, `customOrchestration`, `customerEncryptionKeyArn`, `description`, `foundationModel`, `guardrailConfiguration`, `idleSessionTTLInSeconds`, `instruction`, `memoryConfiguration`, ... (+3) | - |
| `CreateAgentResponse` | `structure` | `agent` | - |
| `CreateAgentActionGroupRequest` | `structure` | `actionGroupExecutor`, `actionGroupName`, `actionGroupState`, `agentId`, `agentVersion`, `apiSchema`, `clientToken`, `description`, `functionSchema`, `parentActionGroupSignature`, `parentActionGroupSignatureParams` | - |
| `CreateAgentActionGroupResponse` | `structure` | `agentActionGroup` | - |
| `CreateAgentAliasRequest` | `structure` | `agentAliasName`, `agentId`, `clientToken`, `description`, `routingConfiguration`, `tags` | - |
| `CreateAgentAliasResponse` | `structure` | `agentAlias` | - |
| `CreateDataSourceRequest` | `structure` | `clientToken`, `dataDeletionPolicy`, `dataSourceConfiguration`, `description`, `knowledgeBaseId`, `name`, `serverSideEncryptionConfiguration`, `vectorIngestionConfiguration` | - |
| `CreateDataSourceResponse` | `structure` | `dataSource` | - |
| `CreateFlowRequest` | `structure` | `clientToken`, `customerEncryptionKeyArn`, `definition`, `description`, `executionRoleArn`, `name`, `tags` | - |
| `CreateFlowResponse` | `structure` | `arn`, `createdAt`, `customerEncryptionKeyArn`, `definition`, `description`, `executionRoleArn`, `id`, `name`, `status`, `updatedAt`, `version` | - |
| `CreateFlowAliasRequest` | `structure` | `clientToken`, `concurrencyConfiguration`, `description`, `flowIdentifier`, `name`, `routingConfiguration`, `tags` | - |
| `CreateFlowAliasResponse` | `structure` | `arn`, `concurrencyConfiguration`, `createdAt`, `description`, `flowId`, `id`, `name`, `routingConfiguration`, `updatedAt` | - |
| `CreateFlowVersionRequest` | `structure` | `clientToken`, `description`, `flowIdentifier` | - |

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
