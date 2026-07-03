# winterbaume-sns

SNS service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SNS |
| AWS model | `sns` |
| Protocol | awsQuery |
| winterbaume coverage | 41/42 operations (97.6%) |
| stubs (routed, returns empty/default) | 1/42 operations (2.4%) |
| moto coverage | 33/42 operations (78.6%) |
| floci coverage | 0/42 operations (0.0%) |
| kumo coverage | 15/42 operations (35.7%) |
| fakecloud coverage | 42/42 operations (100.0%) |
| Coverage report date | 2026-07-03 |

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
aws sns list-topics
```

## Example

```rust
use aws_sdk_sns::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sns::SnsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SnsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sns::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sns::Client::new(&config);

    client
        .create_topic()
        .name("example-topic")
        .send()
        .await
        .expect("create_topic should succeed");
    let resp = client
        .list_topics()
        .send()
        .await
        .expect("list_topics should succeed");
    println!("SNS topics: {}", resp.topics().len());
}
```

## Implemented APIs (41)

- `AddPermission`
- `CheckIfPhoneNumberIsOptedOut`
- `ConfirmSubscription`
- `CreatePlatformApplication`
- `CreatePlatformEndpoint`
- `CreateSMSSandboxPhoneNumber`
- `CreateTopic`
- `DeleteEndpoint`
- `DeletePlatformApplication`
- `DeleteSMSSandboxPhoneNumber`
- `DeleteTopic`
- `GetDataProtectionPolicy`
- `GetEndpointAttributes`
- `GetPlatformApplicationAttributes`
- `GetSMSAttributes`
- `GetSMSSandboxAccountStatus`
- `GetSubscriptionAttributes`
- `GetTopicAttributes`
- `ListEndpointsByPlatformApplication`
- `ListPhoneNumbersOptedOut`
- `ListPlatformApplications`
- `ListSMSSandboxPhoneNumbers`
- `ListSubscriptions`
- `ListSubscriptionsByTopic`
- `ListTagsForResource`
- `ListTopics`
- `OptInPhoneNumber`
- `Publish`
- `PublishBatch`
- `PutDataProtectionPolicy`
- `RemovePermission`
- `SetEndpointAttributes`
- `SetPlatformApplicationAttributes`
- `SetSMSAttributes`
- `SetSubscriptionAttributes`
- `SetTopicAttributes`
- `Subscribe`
- `TagResource`
- `Unsubscribe`
- `UntagResource`
- `VerifySMSSandboxPhoneNumber`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `ListOriginationNumbers`

</details>
