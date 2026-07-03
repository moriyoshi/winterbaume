# winterbaume-emrcontainers

Amazon EMR on EKS (EMR Containers) service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | EMR Containers |
| AWS model | `emr-containers` |
| Protocol | restJson1 |
| winterbaume coverage | 23/23 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/23 operations (0.0%) |
| moto coverage | 8/23 operations (34.8%) |
| floci coverage | 0/23 operations (0.0%) |
| kumo coverage | 0/23 operations (0.0%) |
| fakecloud coverage | 0/23 operations (0.0%) |
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
aws emr-containers list-virtual-clusters
```

## Example

```rust
use aws_sdk_emrcontainers::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_emrcontainers::EmrContainersService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(EmrContainersService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_emrcontainers::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_emrcontainers::Client::new(&config);

    let resp = client
        .list_virtual_clusters()
        .send()
        .await
        .expect("list_virtual_clusters should succeed");
    println!("EMR virtual clusters: {}", resp.virtual_clusters().len());
}
```

## Implemented APIs (23)

- `CancelJobRun`
- `CreateJobTemplate`
- `CreateManagedEndpoint`
- `CreateSecurityConfiguration`
- `CreateVirtualCluster`
- `DeleteJobTemplate`
- `DeleteManagedEndpoint`
- `DeleteVirtualCluster`
- `DescribeJobRun`
- `DescribeJobTemplate`
- `DescribeManagedEndpoint`
- `DescribeSecurityConfiguration`
- `DescribeVirtualCluster`
- `GetManagedEndpointSessionCredentials`
- `ListJobRuns`
- `ListJobTemplates`
- `ListManagedEndpoints`
- `ListSecurityConfigurations`
- `ListTagsForResource`
- `ListVirtualClusters`
- `StartJobRun`
- `TagResource`
- `UntagResource`
