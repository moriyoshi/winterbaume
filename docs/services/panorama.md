# winterbaume-panorama

AWS Panorama service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Panorama |
| AWS model | `panorama` |
| Protocol | restJson1 |
| winterbaume coverage | 10/34 operations (29.4%) |
| stubs (routed, returns empty/default) | 1/34 operations (2.9%) |
| moto coverage | 0/34 operations (0.0%) |
| floci coverage | 0/34 operations (0.0%) |
| kumo coverage | 0/34 operations (0.0%) |
| fakecloud coverage | 0/34 operations (0.0%) |
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
aws panorama help
```

## Example

```rust
use aws_sdk_panorama::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_panorama::PanoramaService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PanoramaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_panorama::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_panorama::Client::new(&config);

    // Provision a device
    let prov = client
        .provision_device()
        .name("my-panorama-device")
        .description("A mock Panorama device")
        .send()
        .await
        .expect("provision_device should succeed");

    let device_id = prov.device_id().expect("should have device_id");
    println!("Provisioned device: {device_id}");

    // List devices
    let list = client
        .list_devices()
        .send()
        .await
        .expect("list_devices should succeed");
    println!("Devices: {}", list.devices().len());

    // Describe the device
    let desc = client
        .describe_device()
        .device_id(device_id)
        .send()
        .await
        .expect("describe_device should succeed");
    println!("Device name: {:?}", desc.name());
    println!("Device status: {:?}", desc.provisioning_status());
}
```

## Implemented APIs (10)

- `CreateApplicationInstance`
- `CreateNodeFromTemplateJob`
- `DeleteDevice`
- `DescribeApplicationInstance`
- `DescribeDevice`
- `DescribeNodeFromTemplateJob`
- `ListApplicationInstances`
- `ListDevices`
- `ProvisionDevice`
- `UpdateDeviceMetadata`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `ListNodes`

</details>

<details><summary>Not yet implemented APIs (23)</summary>

- `CreateJobForDevices`
- `CreatePackage`
- `CreatePackageImportJob`
- `DeletePackage`
- `DeregisterPackageVersion`
- `DescribeApplicationInstanceDetails`
- `DescribeDeviceJob`
- `DescribeNode`
- `DescribePackage`
- `DescribePackageImportJob`
- `DescribePackageVersion`
- `ListApplicationInstanceDependencies`
- `ListApplicationInstanceNodeInstances`
- `ListDevicesJobs`
- `ListNodeFromTemplateJobs`
- `ListPackageImportJobs`
- `ListPackages`
- `ListTagsForResource`
- `RegisterPackageVersion`
- `RemoveApplicationInstance`
- `SignalApplicationInstanceNodeInstances`
- `TagResource`
- `UntagResource`

</details>
