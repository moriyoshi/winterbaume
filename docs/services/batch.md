# winterbaume-batch

AWS Batch service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Batch |
| AWS model | `batch` |
| Protocol | restJson1 |
| winterbaume coverage | 39/45 operations (86.7%) |
| stubs (routed, returns empty/default) | 0/45 operations (0.0%) |
| moto coverage | 24/45 operations (53.3%) |
| floci coverage | 0/45 operations (0.0%) |
| kumo coverage | 10/45 operations (22.2%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws batch describe-job-queues
```

## Example

```rust
use aws_sdk_batch::config::BehaviorVersion;
use winterbaume_batch::BatchService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(BatchService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_batch::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_batch::Client::new(&config);

    let resp = client
        .describe_job_queues()
        .send()
        .await
        .expect("describe_job_queues should succeed");
    println!("Job queues: {}", resp.job_queues().len());
}
```

## Implemented APIs (39)

- `CancelJob`
- `CreateComputeEnvironment`
- `CreateConsumableResource`
- `CreateJobQueue`
- `CreateSchedulingPolicy`
- `CreateServiceEnvironment`
- `DeleteComputeEnvironment`
- `DeleteConsumableResource`
- `DeleteJobQueue`
- `DeleteSchedulingPolicy`
- `DeleteServiceEnvironment`
- `DeregisterJobDefinition`
- `DescribeComputeEnvironments`
- `DescribeConsumableResource`
- `DescribeJobDefinitions`
- `DescribeJobQueues`
- `DescribeJobs`
- `DescribeSchedulingPolicies`
- `DescribeServiceEnvironments`
- `DescribeServiceJob`
- `GetJobQueueSnapshot`
- `ListConsumableResources`
- `ListJobs`
- `ListJobsByConsumableResource`
- `ListSchedulingPolicies`
- `ListServiceJobs`
- `ListTagsForResource`
- `RegisterJobDefinition`
- `SubmitJob`
- `SubmitServiceJob`
- `TagResource`
- `TerminateJob`
- `TerminateServiceJob`
- `UntagResource`
- `UpdateComputeEnvironment`
- `UpdateConsumableResource`
- `UpdateJobQueue`
- `UpdateSchedulingPolicy`
- `UpdateServiceEnvironment`

<details><summary>Not yet implemented APIs (6)</summary>

- `CreateQuotaShare`
- `DeleteQuotaShare`
- `DescribeQuotaShare`
- `ListQuotaShares`
- `UpdateQuotaShare`
- `UpdateServiceJob`

</details>
