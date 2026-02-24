# winterbaume-fis

AWS Fault Injection Simulator service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | FIS |
| AWS model | `fis` |
| Protocol | restJson1 |
| winterbaume coverage | 8/26 operations (30.8%) |
| stubs (routed, returns empty/default) | 0/26 operations (0.0%) |
| moto coverage | 5/26 operations (19.2%) |
| floci coverage | 0/26 operations (0.0%) |
| kumo coverage | 0/26 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws fis help
```

## Example

```rust
use aws_sdk_fis::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_fis::FisService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(FisService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_fis::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_fis::Client::new(&config);

    let resp = client
        .list_experiment_templates()
        .send()
        .await
        .expect("list_experiment_templates should succeed");
    println!(
        "FIS experiment templates: {:?}",
        resp.experiment_templates()
    );
}
```

## Implemented APIs (8)

- `CreateExperimentTemplate`
- `DeleteExperimentTemplate`
- `GetExperimentTemplate`
- `ListExperimentTemplates`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateExperimentTemplate`

<details><summary>Not yet implemented APIs (18)</summary>

- `CreateTargetAccountConfiguration`
- `DeleteTargetAccountConfiguration`
- `GetAction`
- `GetExperiment`
- `GetExperimentTargetAccountConfiguration`
- `GetSafetyLever`
- `GetTargetAccountConfiguration`
- `GetTargetResourceType`
- `ListActions`
- `ListExperimentResolvedTargets`
- `ListExperimentTargetAccountConfigurations`
- `ListExperiments`
- `ListTargetAccountConfigurations`
- `ListTargetResourceTypes`
- `StartExperiment`
- `StopExperiment`
- `UpdateSafetyLeverState`
- `UpdateTargetAccountConfiguration`

</details>
