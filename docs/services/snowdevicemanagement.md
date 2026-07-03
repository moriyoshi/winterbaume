# winterbaume-snowdevicemanagement

AWS Snow Device Management implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Snow Device Management |
| AWS model | `snow-device-management` |
| Protocol | restJson1 |
| winterbaume coverage | 11/13 operations (84.6%) |
| stubs (routed, returns empty/default) | 0/13 operations (0.0%) |
| moto coverage | 0/13 operations (0.0%) |
| floci coverage | 0/13 operations (0.0%) |
| kumo coverage | 0/13 operations (0.0%) |
| fakecloud coverage | 0/13 operations (0.0%) |
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
aws snowdevicemanagement help
```

## Current Network Resource Stub Semantics

Snow Device Management currently returns physical network interface data as device-local response metadata.

- Device descriptions can include `physical_network_interfaces`, but current handler responses use an empty list.
- The service has no VPC, subnet, security group, or ENI state map.
- Snow device network data is not reconciled with EC2 or on-premises network resources.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_snowdevicemanagement::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_snowdevicemanagement::SnowDeviceManagementService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SnowDeviceManagementService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_snowdevicemanagement::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_snowdevicemanagement::Client::new(&config);
    let resp = client
        .list_devices()
        .send()
        .await
        .expect("list_devices should succeed");
    for d in resp.devices() {
        println!("Device: {:?}", d.managed_device_id());
    }
}
```

## Implemented APIs (11)

- `CancelTask`
- `CreateTask`
- `DescribeDevice`
- `DescribeExecution`
- `DescribeTask`
- `ListDeviceResources`
- `ListDevices`
- `ListExecutions`
- `ListTasks`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (2)</summary>

- `DescribeDeviceEc2Instances`
- `ListTagsForResource`

</details>
