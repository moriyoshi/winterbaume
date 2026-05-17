# winterbaume-backupgateway

AWS Backup Gateway service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Backup Gateway |
| AWS model | `backup-gateway` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 25/25 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/25 operations (0.0%) |
| moto coverage | 0/25 operations (0.0%) |
| floci coverage | 0/25 operations (0.0%) |
| kumo coverage | 0/25 operations (0.0%) |
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
aws backupgateway help
```

## Current Network Resource Stub Semantics

Backup Gateway currently treats VPC endpoint information as gateway metadata.

- Gateway records include an optional `vpc_endpoint` string, and describe/list responses echo that value.
- There is no service-local VPC endpoint resource map and no lifecycle for endpoint creation or deletion.
- Gateway health and connection state are independent of the VPC endpoint value.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_backupgateway::config::BehaviorVersion;
use winterbaume_backupgateway::BackupGatewayService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BackupGatewayService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_backupgateway::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_backupgateway::Client::new(&config);

    let resp = client
        .list_gateways()
        .send()
        .await
        .expect("list_gateways should succeed");
    println!("Backup gateways: {}", resp.gateways().len());
}
```

## Implemented APIs (25)

- `AssociateGatewayToServer`
- `CreateGateway`
- `DeleteGateway`
- `DeleteHypervisor`
- `DisassociateGatewayFromServer`
- `GetBandwidthRateLimitSchedule`
- `GetGateway`
- `GetHypervisor`
- `GetHypervisorPropertyMappings`
- `GetVirtualMachine`
- `ImportHypervisorConfiguration`
- `ListGateways`
- `ListHypervisors`
- `ListTagsForResource`
- `ListVirtualMachines`
- `PutBandwidthRateLimitSchedule`
- `PutHypervisorPropertyMappings`
- `PutMaintenanceStartTime`
- `StartVirtualMachinesMetadataSync`
- `TagResource`
- `TestHypervisorConfiguration`
- `UntagResource`
- `UpdateGatewayInformation`
- `UpdateGatewaySoftwareNow`
- `UpdateHypervisor`
