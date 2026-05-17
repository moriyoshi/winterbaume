# winterbaume-timestreaminfluxdb

Amazon Timestream for InfluxDB service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Timestream InfluxDB |
| AWS model | `timestream-influxdb` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 19/19 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/19 operations (0.0%) |
| moto coverage | 13/19 operations (68.4%) |
| floci coverage | 0/19 operations (0.0%) |
| kumo coverage | 0/19 operations (0.0%) |
| Coverage report date | 2026-05-17 |

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
aws timestream-influxdb list-db-instances
```

## Current Network Resource Stub Semantics

Timestream for InfluxDB currently stores VPC subnet and security group lists directly on instances and clusters.

- Instance creation requires VPC subnet IDs and VPC security group IDs and stores both vectors in state.
- Cluster creation and update paths similarly store subnet and security group lists as service-local fields.
- Describe and list responses echo those stored vectors without deriving VPC membership or provisioning network interfaces.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_timestreaminfluxdb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_timestreaminfluxdb::TimestreamInfluxDbService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TimestreamInfluxDbService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_timestreaminfluxdb::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_timestreaminfluxdb::Client::new(&config);

    let resp = client
        .list_db_instances()
        .send()
        .await
        .expect("list_db_instances should succeed");
    println!("Timestream InfluxDB instances: {}", resp.items().len());
}
```

## Implemented APIs (19)

- `CreateDbCluster`
- `CreateDbInstance`
- `CreateDbParameterGroup`
- `DeleteDbCluster`
- `DeleteDbInstance`
- `GetDbCluster`
- `GetDbInstance`
- `GetDbParameterGroup`
- `ListDbClusters`
- `ListDbInstances`
- `ListDbInstancesForCluster`
- `ListDbParameterGroups`
- `ListTagsForResource`
- `RebootDbCluster`
- `RebootDbInstance`
- `TagResource`
- `UntagResource`
- `UpdateDbCluster`
- `UpdateDbInstance`
