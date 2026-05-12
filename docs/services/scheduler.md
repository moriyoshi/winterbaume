# winterbaume-scheduler

EventBridge Scheduler service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Scheduler |
| AWS model | `scheduler` |
| Protocol | restJson1 |
| winterbaume coverage | 12/12 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/12 operations (0.0%) |
| moto coverage | 12/12 operations (100.0%) |
| floci coverage | 0/12 operations (0.0%) |
| kumo coverage | 9/12 operations (75.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws scheduler list-schedules
```

## Example

```rust
use aws_sdk_scheduler::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_scheduler::SchedulerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SchedulerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_scheduler::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_scheduler::Client::new(&config);

    let resp = client
        .list_schedules()
        .send()
        .await
        .expect("list_schedules should succeed");
    println!(
        "EventBridge Scheduler schedules: {}",
        resp.schedules().len()
    );
}
```

## Implemented APIs (12)

- `CreateSchedule`
- `CreateScheduleGroup`
- `DeleteSchedule`
- `DeleteScheduleGroup`
- `GetSchedule`
- `GetScheduleGroup`
- `ListScheduleGroups`
- `ListSchedules`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateSchedule`
