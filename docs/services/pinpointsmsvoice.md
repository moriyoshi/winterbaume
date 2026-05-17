# winterbaume-pinpointsmsvoice

AWS Pinpoint SMS Voice service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Pinpoint SMS Voice |
| AWS model | `pinpoint-sms-voice` |
| Protocol | restJson1 |
| winterbaume coverage | 4/8 operations (50.0%) |
| stubs (routed, returns empty/default) | 0/8 operations (0.0%) |
| moto coverage | 0/8 operations (0.0%) |
| floci coverage | 0/8 operations (0.0%) |
| kumo coverage | 0/8 operations (0.0%) |
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
aws pinpointsmsvoice help
```

## Example

```rust
use aws_sdk_pinpointsmsvoice::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pinpointsmsvoice::PinpointSmsVoiceService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PinpointSmsVoiceService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pinpointsmsvoice::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pinpointsmsvoice::Client::new(&config);
    client
        .create_configuration_set()
        .configuration_set_name("demo-set")
        .send()
        .await
        .expect("create_configuration_set should succeed");
    let resp = client
        .list_configuration_sets()
        .send()
        .await
        .expect("list_configuration_sets should succeed");
    println!("Configuration sets: {:?}", resp.configuration_sets());
}
```

## Implemented APIs (4)

- `CreateConfigurationSet`
- `DeleteConfigurationSet`
- `ListConfigurationSets`
- `SendVoiceMessage`

<details><summary>Not yet implemented APIs (4)</summary>

- `CreateConfigurationSetEventDestination`
- `DeleteConfigurationSetEventDestination`
- `GetConfigurationSetEventDestinations`
- `UpdateConfigurationSetEventDestination`

</details>
