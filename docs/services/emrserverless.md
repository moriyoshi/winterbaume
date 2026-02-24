# winterbaume-emrserverless

EMR Serverless service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | EMR Serverless |
| AWS model | `emr-serverless` |
| Protocol | restJson1 |
| winterbaume coverage | 16/22 operations (72.7%) |
| stubs (routed, returns empty/default) | 0/22 operations (0.0%) |
| moto coverage | 11/22 operations (50.0%) |
| floci coverage | 0/22 operations (0.0%) |
| kumo coverage | 11/22 operations (50.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws emr-serverless list-applications
```

## Current Network Resource Stub Semantics

EMR Serverless currently stores application network configuration locally.

- Application records can include a network configuration with subnet IDs and security group IDs.
- Start, stop, and job-run state transitions do not allocate ENIs or check subnet capacity.
- Updating or reading an application only observes the stored EMR Serverless metadata.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_emrserverless::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_emrserverless::EmrServerlessService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(EmrServerlessService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_emrserverless::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_emrserverless::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!("EMR Serverless applications: {}", resp.applications().len());
}
```

## Implemented APIs (16)

- `CancelJobRun`
- `CreateApplication`
- `DeleteApplication`
- `GetApplication`
- `GetDashboardForJobRun`
- `GetJobRun`
- `ListApplications`
- `ListJobRunAttempts`
- `ListJobRuns`
- `ListTagsForResource`
- `StartApplication`
- `StartJobRun`
- `StopApplication`
- `TagResource`
- `UntagResource`
- `UpdateApplication`

<details><summary>Not yet implemented APIs (6)</summary>

- `GetResourceDashboard`
- `GetSession`
- `GetSessionEndpoint`
- `ListSessions`
- `StartSession`
- `TerminateSession`

</details>
