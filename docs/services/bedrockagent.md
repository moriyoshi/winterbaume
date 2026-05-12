# winterbaume-bedrockagent

Bedrock Agent service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Bedrock Agent |
| AWS model | `bedrock-agent` |
| Protocol | restJson1 |
| winterbaume coverage | 72/72 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/72 operations (0.0%) |
| moto coverage | 11/72 operations (15.3%) |
| floci coverage | 0/72 operations (0.0%) |
| kumo coverage | 0/72 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws bedrock-agent list-agents
```

## Example

```rust
use aws_sdk_bedrockagent::config::BehaviorVersion;
use winterbaume_bedrockagent::BedrockAgentService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BedrockAgentService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bedrockagent::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_bedrockagent::Client::new(&config);

    let resp = client
        .list_agents()
        .send()
        .await
        .expect("list_agents should succeed");
    println!("Agents: {}", resp.agent_summaries().len());
}
```

## Implemented APIs (72)

- `AssociateAgentCollaborator`
- `AssociateAgentKnowledgeBase`
- `CreateAgent`
- `CreateAgentActionGroup`
- `CreateAgentAlias`
- `CreateDataSource`
- `CreateFlow`
- `CreateFlowAlias`
- `CreateFlowVersion`
- `CreateKnowledgeBase`
- `CreatePrompt`
- `CreatePromptVersion`
- `DeleteAgent`
- `DeleteAgentActionGroup`
- `DeleteAgentAlias`
- `DeleteAgentVersion`
- `DeleteDataSource`
- `DeleteFlow`
- `DeleteFlowAlias`
- `DeleteFlowVersion`
- `DeleteKnowledgeBase`
- `DeleteKnowledgeBaseDocuments`
- `DeletePrompt`
- `DisassociateAgentCollaborator`
- `DisassociateAgentKnowledgeBase`
- `GetAgent`
- `GetAgentActionGroup`
- `GetAgentAlias`
- `GetAgentCollaborator`
- `GetAgentKnowledgeBase`
- `GetAgentVersion`
- `GetDataSource`
- `GetFlow`
- `GetFlowAlias`
- `GetFlowVersion`
- `GetIngestionJob`
- `GetKnowledgeBase`
- `GetKnowledgeBaseDocuments`
- `GetPrompt`
- `IngestKnowledgeBaseDocuments`
- `ListAgentActionGroups`
- `ListAgentAliases`
- `ListAgentCollaborators`
- `ListAgentKnowledgeBases`
- `ListAgentVersions`
- `ListAgents`
- `ListDataSources`
- `ListFlowAliases`
- `ListFlowVersions`
- `ListFlows`
- `ListIngestionJobs`
- `ListKnowledgeBaseDocuments`
- `ListKnowledgeBases`
- `ListPrompts`
- `ListTagsForResource`
- `PrepareAgent`
- `PrepareFlow`
- `StartIngestionJob`
- `StopIngestionJob`
- `TagResource`
- `UntagResource`
- `UpdateAgent`
- `UpdateAgentActionGroup`
- `UpdateAgentAlias`
- `UpdateAgentCollaborator`
- `UpdateAgentKnowledgeBase`
- `UpdateDataSource`
- `UpdateFlow`
- `UpdateFlowAlias`
- `UpdateKnowledgeBase`
- `UpdatePrompt`
- `ValidateFlowDefinition`
