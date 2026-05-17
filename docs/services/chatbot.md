# winterbaume-chatbot

Chatbot service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Chatbot |
| AWS model | `chatbot` |
| Protocol | restJson1 |
| winterbaume coverage | 15/34 operations (44.1%) |
| stubs (routed, returns empty/default) | 0/34 operations (0.0%) |
| moto coverage | 0/34 operations (0.0%) |
| floci coverage | 0/34 operations (0.0%) |
| kumo coverage | 0/34 operations (0.0%) |
| Coverage report date | 2026-05-16 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws chatbot help
```

## Example

```rust
use aws_sdk_chatbot::config::BehaviorVersion;
use winterbaume_chatbot::ChatbotService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ChatbotService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_chatbot::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_chatbot::Client::new(&config);

    let resp = client
        .describe_slack_channel_configurations()
        .send()
        .await
        .expect("describe_slack_channel_configurations should succeed");
    println!(
        "Chatbot Slack configurations: {:?}",
        resp.slack_channel_configurations()
    );
}
```

## Implemented APIs (15)

- `CreateChimeWebhookConfiguration`
- `CreateMicrosoftTeamsChannelConfiguration`
- `CreateSlackChannelConfiguration`
- `DeleteChimeWebhookConfiguration`
- `DeleteMicrosoftTeamsChannelConfiguration`
- `DeleteSlackChannelConfiguration`
- `DescribeChimeWebhookConfigurations`
- `DescribeSlackChannelConfigurations`
- `GetMicrosoftTeamsChannelConfiguration`
- `ListMicrosoftTeamsChannelConfigurations`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateMicrosoftTeamsChannelConfiguration`
- `UpdateSlackChannelConfiguration`

<details><summary>Not yet implemented APIs (19)</summary>

- `AssociateToConfiguration`
- `CreateCustomAction`
- `DeleteCustomAction`
- `DeleteMicrosoftTeamsConfiguredTeam`
- `DeleteMicrosoftTeamsUserIdentity`
- `DeleteSlackUserIdentity`
- `DeleteSlackWorkspaceAuthorization`
- `DescribeSlackUserIdentities`
- `DescribeSlackWorkspaces`
- `DisassociateFromConfiguration`
- `GetAccountPreferences`
- `GetCustomAction`
- `ListAssociations`
- `ListCustomActions`
- `ListMicrosoftTeamsConfiguredTeams`
- `ListMicrosoftTeamsUserIdentities`
- `UpdateAccountPreferences`
- `UpdateChimeWebhookConfiguration`
- `UpdateCustomAction`

</details>
