# winterbaume-mediaconnect

MediaConnect service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | MediaConnect |
| AWS model | `mediaconnect` |
| Protocol | restJson1 |
| winterbaume coverage | 21/82 operations (25.6%) |
| stubs (routed, returns empty/default) | 0/82 operations (0.0%) |
| moto coverage | 18/82 operations (22.0%) |
| floci coverage | 0/82 operations (0.0%) |
| kumo coverage | 0/82 operations (0.0%) |
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
aws mediaconnect list-flows
```

## Current Network Resource Stub Semantics

MediaConnect currently stores flow VPC interfaces inside MediaConnect flow state.

- VPC interface records keep name, role ARN, subnet ID, security group IDs, network interface type, and locally generated network interface IDs.
- `AddFlowVpcInterfaces` appends these records to the flow; `RemoveFlowVpcInterface` removes by name and returns an empty non-deleted network interface list.
- The generated network interface IDs are not inserted into EC2 state, and the role ARN is not checked against IAM.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_mediaconnect::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediaconnect::MediaConnectService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediaconnect::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediaconnect::Client::new(&config);

    let resp = client
        .list_flows()
        .send()
        .await
        .expect("list_flows should succeed");
    println!("MediaConnect flows: {}", resp.flows().len());
}
```

## Implemented APIs (21)

- `AddFlowOutputs`
- `AddFlowSources`
- `AddFlowVpcInterfaces`
- `CreateFlow`
- `DeleteFlow`
- `DescribeFlow`
- `GrantFlowEntitlements`
- `ListFlows`
- `ListTagsForResource`
- `RemoveFlowOutput`
- `RemoveFlowSource`
- `RemoveFlowVpcInterface`
- `RevokeFlowEntitlement`
- `StartFlow`
- `StopFlow`
- `TagResource`
- `UntagResource`
- `UpdateFlow`
- `UpdateFlowEntitlement`
- `UpdateFlowOutput`
- `UpdateFlowSource`

<details><summary>Not yet implemented APIs (61)</summary>

- `AddBridgeOutputs`
- `AddBridgeSources`
- `AddFlowMediaStreams`
- `BatchGetRouterInput`
- `BatchGetRouterNetworkInterface`
- `BatchGetRouterOutput`
- `CreateBridge`
- `CreateGateway`
- `CreateRouterInput`
- `CreateRouterNetworkInterface`
- `CreateRouterOutput`
- `DeleteBridge`
- `DeleteGateway`
- `DeleteRouterInput`
- `DeleteRouterNetworkInterface`
- `DeleteRouterOutput`
- `DeregisterGatewayInstance`
- `DescribeBridge`
- `DescribeFlowSourceMetadata`
- `DescribeFlowSourceThumbnail`
- `DescribeGateway`
- `DescribeGatewayInstance`
- `DescribeOffering`
- `DescribeReservation`
- `GetRouterInput`
- `GetRouterInputSourceMetadata`
- `GetRouterInputThumbnail`
- `GetRouterNetworkInterface`
- `GetRouterOutput`
- `ListBridges`
- `ListEntitlements`
- `ListGatewayInstances`
- `ListGateways`
- `ListOfferings`
- `ListReservations`
- `ListRouterInputs`
- `ListRouterNetworkInterfaces`
- `ListRouterOutputs`
- `ListTagsForGlobalResource`
- `PurchaseOffering`
- `RemoveBridgeOutput`
- `RemoveBridgeSource`
- `RemoveFlowMediaStream`
- `RestartRouterInput`
- `RestartRouterOutput`
- `StartRouterInput`
- `StartRouterOutput`
- `StopRouterInput`
- `StopRouterOutput`
- `TagGlobalResource`
- `TakeRouterInput`
- `UntagGlobalResource`
- `UpdateBridge`
- `UpdateBridgeOutput`
- `UpdateBridgeSource`
- `UpdateBridgeState`
- `UpdateFlowMediaStream`
- `UpdateGatewayInstance`
- `UpdateRouterInput`
- `UpdateRouterNetworkInterface`
- `UpdateRouterOutput`

</details>
