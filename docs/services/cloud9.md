# winterbaume-cloud9

AWS Cloud9 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Cloud9 |
| AWS model | `cloud9` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 13/13 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/13 operations (0.0%) |
| moto coverage | 0/13 operations (0.0%) |
| floci coverage | 0/13 operations (0.0%) |
| kumo coverage | 0/13 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws cloud9 help
```

## Example

```rust
use aws_sdk_cloud9::config::BehaviorVersion;
use winterbaume_cloud9::Cloud9Service;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Cloud9Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloud9::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloud9::Client::new(&config);
    let create = client
        .create_environment_ec2()
        .name("demo")
        .image_id("amazonlinux-2023-x86_64")
        .instance_type("t3.small")
        .send()
        .await
        .expect("create_environment_ec2 should succeed");
    println!("Created environment: {:?}", create.environment_id());
}
```

## Implemented APIs (13)

- `CreateEnvironmentEC2`
- `CreateEnvironmentMembership`
- `DeleteEnvironment`
- `DeleteEnvironmentMembership`
- `DescribeEnvironmentMemberships`
- `DescribeEnvironmentStatus`
- `DescribeEnvironments`
- `ListEnvironments`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateEnvironment`
- `UpdateEnvironmentMembership`
