# winterbaume-connectcampaigns

Amazon Connect Campaigns service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Connect Campaigns |
| AWS model | `connectcampaigns` |
| Protocol | restJson1 |
| winterbaume coverage | 14/22 operations (63.6%) |
| stubs (routed, returns empty/default) | 0/22 operations (0.0%) |
| moto coverage | 14/22 operations (63.6%) |
| floci coverage | 0/22 operations (0.0%) |
| kumo coverage | 0/22 operations (0.0%) |
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
aws connectcampaigns list-campaigns --instance-id 00000000-0000-0000-0000-000000000000
```

## Example

```rust
use aws_sdk_connectcampaigns::config::BehaviorVersion;
use winterbaume_connectcampaigns::ConnectCampaignsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConnectCampaignsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connectcampaigns::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_connectcampaigns::Client::new(&config);

    // Connect Campaigns requires a Connect instance; create one via winterbaume-connect first.
    // This example demonstrates client setup for the ConnectCampaigns service.
    println!("ConnectCampaigns client ready.");
    let _client = client;
}
```

## Implemented APIs (14)

- `CreateCampaign`
- `DeleteCampaign`
- `DescribeCampaign`
- `GetCampaignState`
- `GetConnectInstanceConfig`
- `ListCampaigns`
- `ListTagsForResource`
- `PauseCampaign`
- `ResumeCampaign`
- `StartCampaign`
- `StartInstanceOnboardingJob`
- `StopCampaign`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (8)</summary>

- `DeleteConnectInstanceConfig`
- `DeleteInstanceOnboardingJob`
- `GetCampaignStateBatch`
- `GetInstanceOnboardingJobStatus`
- `PutDialRequestBatch`
- `UpdateCampaignDialerConfig`
- `UpdateCampaignName`
- `UpdateCampaignOutboundCallConfig`

</details>
