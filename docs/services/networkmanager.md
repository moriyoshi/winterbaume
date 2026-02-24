# winterbaume-networkmanager

AWS Network Manager service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Network Manager |
| AWS model | `networkmanager` |
| Protocol | restJson1 |
| winterbaume coverage | 53/95 operations (55.8%) |
| stubs (routed, returns empty/default) | 0/95 operations (0.0%) |
| moto coverage | 18/95 operations (18.9%) |
| floci coverage | 0/95 operations (0.0%) |
| kumo coverage | 0/95 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws networkmanager list-core-networks
```

## Current Network Resource Stub Semantics

Network Manager currently stores VPC attachments by ARN and subnet ARN metadata.

- `CreateVpcAttachment` requires a VPC ARN and records it as the attachment resource ARN.
- Supplied subnet ARNs are stored as raw strings on the attachment.
- Attachment state is not linked to EC2 VPC, subnet, or transit gateway attachment state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_networkmanager::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_networkmanager::NetworkManagerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(NetworkManagerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_networkmanager::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_networkmanager::Client::new(&config);

    let resp = client
        .describe_global_networks()
        .send()
        .await
        .expect("describe_global_networks should succeed");
    println!("Global networks: {}", resp.global_networks().len());
}
```

## Implemented APIs (53)

- `AcceptAttachment`
- `AssociateConnectPeer`
- `AssociateCustomerGateway`
- `AssociateLink`
- `AssociateTransitGatewayConnectPeer`
- `CreateConnectPeer`
- `CreateConnection`
- `CreateCoreNetwork`
- `CreateDevice`
- `CreateGlobalNetwork`
- `CreateLink`
- `CreateSite`
- `DeleteAttachment`
- `DeleteConnectPeer`
- `DeleteConnection`
- `DeleteCoreNetwork`
- `DeleteDevice`
- `DeleteGlobalNetwork`
- `DeleteLink`
- `DeleteSite`
- `DeregisterTransitGateway`
- `DescribeGlobalNetworks`
- `DisassociateConnectPeer`
- `DisassociateCustomerGateway`
- `DisassociateLink`
- `DisassociateTransitGatewayConnectPeer`
- `GetConnectPeer`
- `GetConnectPeerAssociations`
- `GetConnections`
- `GetCoreNetwork`
- `GetCustomerGatewayAssociations`
- `GetDevices`
- `GetLinkAssociations`
- `GetLinks`
- `GetRouteAnalysis`
- `GetSites`
- `GetTransitGatewayConnectPeerAssociations`
- `GetTransitGatewayRegistrations`
- `ListAttachments`
- `ListConnectPeers`
- `ListCoreNetworks`
- `ListTagsForResource`
- `RegisterTransitGateway`
- `RejectAttachment`
- `StartRouteAnalysis`
- `TagResource`
- `UntagResource`
- `UpdateConnection`
- `UpdateCoreNetwork`
- `UpdateDevice`
- `UpdateGlobalNetwork`
- `UpdateLink`
- `UpdateSite`

<details><summary>Not yet implemented APIs (42)</summary>

- `CreateConnectAttachment`
- `CreateCoreNetworkPrefixListAssociation`
- `CreateDirectConnectGatewayAttachment`
- `CreateSiteToSiteVpnAttachment`
- `CreateTransitGatewayPeering`
- `CreateTransitGatewayRouteTableAttachment`
- `CreateVpcAttachment`
- `DeleteCoreNetworkPolicyVersion`
- `DeleteCoreNetworkPrefixListAssociation`
- `DeletePeering`
- `DeleteResourcePolicy`
- `ExecuteCoreNetworkChangeSet`
- `GetConnectAttachment`
- `GetCoreNetworkChangeEvents`
- `GetCoreNetworkChangeSet`
- `GetCoreNetworkPolicy`
- `GetDirectConnectGatewayAttachment`
- `GetNetworkResourceCounts`
- `GetNetworkResourceRelationships`
- `GetNetworkResources`
- `GetNetworkRoutes`
- `GetNetworkTelemetry`
- `GetResourcePolicy`
- `GetSiteToSiteVpnAttachment`
- `GetTransitGatewayPeering`
- `GetTransitGatewayRouteTableAttachment`
- `GetVpcAttachment`
- `ListAttachmentRoutingPolicyAssociations`
- `ListCoreNetworkPolicyVersions`
- `ListCoreNetworkPrefixListAssociations`
- `ListCoreNetworkRoutingInformation`
- `ListOrganizationServiceAccessStatus`
- `ListPeerings`
- `PutAttachmentRoutingPolicyLabel`
- `PutCoreNetworkPolicy`
- `PutResourcePolicy`
- `RemoveAttachmentRoutingPolicyLabel`
- `RestoreCoreNetworkPolicyVersion`
- `StartOrganizationServiceAccessUpdate`
- `UpdateDirectConnectGatewayAttachment`
- `UpdateNetworkResourceMetadata`
- `UpdateVpcAttachment`

</details>
