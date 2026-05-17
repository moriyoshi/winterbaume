# winterbaume-medialive

MediaLive service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | MediaLive |
| AWS model | `medialive` |
| Protocol | restJson1 |
| winterbaume coverage | 16/123 operations (13.0%) |
| stubs (routed, returns empty/default) | 0/123 operations (0.0%) |
| moto coverage | 12/123 operations (9.8%) |
| floci coverage | 0/123 operations (0.0%) |
| kumo coverage | 0/123 operations (0.0%) |
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
aws medialive list-channels
```

## Current Network Resource Stub Semantics

MediaLive currently has shallow networking placeholders.

- `CreateInputSecurityGroup` is a no-state stub that returns a fixed input-security-group-looking ARN and ID with empty or request-shaped rule data.
- Input records can carry security group IDs and VPC-style nested values in service-local state or snapshot JSON slots.
- Channels and inputs do not allocate ENIs, attach to subnets, or validate whitelist rules against EC2 networking.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_medialive::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_medialive::MediaLiveService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaLiveService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_medialive::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_medialive::Client::new(&config);

    let resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    println!("MediaLive channels: {}", resp.channels().len());
}
```

## Implemented APIs (16)

- `CreateChannel`
- `CreateInput`
- `CreateInputSecurityGroup`
- `CreateTags`
- `DeleteChannel`
- `DeleteInput`
- `DeleteTags`
- `DescribeChannel`
- `DescribeInput`
- `ListChannels`
- `ListInputs`
- `ListTagsForResource`
- `StartChannel`
- `StopChannel`
- `UpdateChannel`
- `UpdateInput`

<details><summary>Not yet implemented APIs (107)</summary>

- `AcceptInputDeviceTransfer`
- `BatchDelete`
- `BatchStart`
- `BatchStop`
- `BatchUpdateSchedule`
- `CancelInputDeviceTransfer`
- `ClaimDevice`
- `CreateChannelPlacementGroup`
- `CreateCloudWatchAlarmTemplate`
- `CreateCloudWatchAlarmTemplateGroup`
- `CreateCluster`
- `CreateEventBridgeRuleTemplate`
- `CreateEventBridgeRuleTemplateGroup`
- `CreateMultiplex`
- `CreateMultiplexProgram`
- `CreateNetwork`
- `CreateNode`
- `CreateNodeRegistrationScript`
- `CreatePartnerInput`
- `CreateSdiSource`
- `CreateSignalMap`
- `DeleteChannelPlacementGroup`
- `DeleteCloudWatchAlarmTemplate`
- `DeleteCloudWatchAlarmTemplateGroup`
- `DeleteCluster`
- `DeleteEventBridgeRuleTemplate`
- `DeleteEventBridgeRuleTemplateGroup`
- `DeleteInputSecurityGroup`
- `DeleteMultiplex`
- `DeleteMultiplexProgram`
- `DeleteNetwork`
- `DeleteNode`
- `DeleteReservation`
- `DeleteSchedule`
- `DeleteSdiSource`
- `DeleteSignalMap`
- `DescribeAccountConfiguration`
- `DescribeChannelPlacementGroup`
- `DescribeCluster`
- `DescribeInputDevice`
- `DescribeInputDeviceThumbnail`
- `DescribeInputSecurityGroup`
- `DescribeMultiplex`
- `DescribeMultiplexProgram`
- `DescribeNetwork`
- `DescribeNode`
- `DescribeOffering`
- `DescribeReservation`
- `DescribeSchedule`
- `DescribeSdiSource`
- `DescribeThumbnails`
- `GetCloudWatchAlarmTemplate`
- `GetCloudWatchAlarmTemplateGroup`
- `GetEventBridgeRuleTemplate`
- `GetEventBridgeRuleTemplateGroup`
- `GetSignalMap`
- `ListAlerts`
- `ListChannelPlacementGroups`
- `ListCloudWatchAlarmTemplateGroups`
- `ListCloudWatchAlarmTemplates`
- `ListClusterAlerts`
- `ListClusters`
- `ListEventBridgeRuleTemplateGroups`
- `ListEventBridgeRuleTemplates`
- `ListInputDeviceTransfers`
- `ListInputDevices`
- `ListInputSecurityGroups`
- `ListMultiplexAlerts`
- `ListMultiplexPrograms`
- `ListMultiplexes`
- `ListNetworks`
- `ListNodes`
- `ListOfferings`
- `ListReservations`
- `ListSdiSources`
- `ListSignalMaps`
- `ListVersions`
- `PurchaseOffering`
- `RebootInputDevice`
- `RejectInputDeviceTransfer`
- `RestartChannelPipelines`
- `StartDeleteMonitorDeployment`
- `StartInputDevice`
- `StartInputDeviceMaintenanceWindow`
- `StartMonitorDeployment`
- `StartMultiplex`
- `StartUpdateSignalMap`
- `StopInputDevice`
- `StopMultiplex`
- `TransferInputDevice`
- `UpdateAccountConfiguration`
- `UpdateChannelClass`
- `UpdateChannelPlacementGroup`
- `UpdateCloudWatchAlarmTemplate`
- `UpdateCloudWatchAlarmTemplateGroup`
- `UpdateCluster`
- `UpdateEventBridgeRuleTemplate`
- `UpdateEventBridgeRuleTemplateGroup`
- `UpdateInputDevice`
- `UpdateInputSecurityGroup`
- `UpdateMultiplex`
- `UpdateMultiplexProgram`
- `UpdateNetwork`
- `UpdateNode`
- `UpdateNodeState`
- `UpdateReservation`
- `UpdateSdiSource`

</details>
