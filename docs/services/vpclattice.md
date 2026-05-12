# winterbaume-vpclattice

VPC Lattice service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | VPC Lattice |
| AWS model | `vpc-lattice` |
| Protocol | restJson1 |
| winterbaume coverage | 66/73 operations (90.4%) |
| stubs (routed, returns empty/default) | 2/73 operations (2.7%) |
| moto coverage | 35/73 operations (47.9%) |
| floci coverage | 0/73 operations (0.0%) |
| kumo coverage | 0/73 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws vpc-lattice list-services
```

## Current Network Resource Stub Semantics

VPC Lattice currently stores service-network VPC associations and resource gateway networking inside VPC Lattice state.

- Service network VPC associations store service network ID, VPC ID, and security group IDs, and update calls replace the stored security group list.
- Resource gateways store VPC ID, subnet IDs, and security group IDs as supplied by the request.
- Resource endpoint association and service network VPC endpoint association handlers currently return placeholder or empty responses rather than backing full endpoint state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_vpclattice::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_vpclattice::VpcLatticeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(VpcLatticeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_vpclattice::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_vpclattice::Client::new(&config);

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    println!("VPC Lattice services: {}", resp.items().len());
}
```

## Implemented APIs (66)

- `BatchUpdateRule`
- `CreateAccessLogSubscription`
- `CreateListener`
- `CreateResourceConfiguration`
- `CreateResourceGateway`
- `CreateRule`
- `CreateService`
- `CreateServiceNetwork`
- `CreateServiceNetworkResourceAssociation`
- `CreateServiceNetworkServiceAssociation`
- `CreateServiceNetworkVpcAssociation`
- `CreateTargetGroup`
- `DeleteAccessLogSubscription`
- `DeleteAuthPolicy`
- `DeleteDomainVerification`
- `DeleteListener`
- `DeleteResourceConfiguration`
- `DeleteResourceGateway`
- `DeleteResourcePolicy`
- `DeleteRule`
- `DeleteService`
- `DeleteServiceNetwork`
- `DeleteServiceNetworkResourceAssociation`
- `DeleteServiceNetworkServiceAssociation`
- `DeleteTargetGroup`
- `DeregisterTargets`
- `GetAccessLogSubscription`
- `GetAuthPolicy`
- `GetDomainVerification`
- `GetListener`
- `GetResourceConfiguration`
- `GetResourceGateway`
- `GetResourcePolicy`
- `GetRule`
- `GetService`
- `GetServiceNetwork`
- `GetServiceNetworkResourceAssociation`
- `GetServiceNetworkServiceAssociation`
- `GetTargetGroup`
- `ListAccessLogSubscriptions`
- `ListDomainVerifications`
- `ListListeners`
- `ListResourceConfigurations`
- `ListResourceGateways`
- `ListRules`
- `ListServiceNetworkResourceAssociations`
- `ListServiceNetworkServiceAssociations`
- `ListServiceNetworks`
- `ListServices`
- `ListTagsForResource`
- `ListTargetGroups`
- `ListTargets`
- `PutAuthPolicy`
- `PutResourcePolicy`
- `RegisterTargets`
- `StartDomainVerification`
- `TagResource`
- `UntagResource`
- `UpdateAccessLogSubscription`
- `UpdateListener`
- `UpdateResourceConfiguration`
- `UpdateResourceGateway`
- `UpdateRule`
- `UpdateService`
- `UpdateServiceNetwork`
- `UpdateTargetGroup`

<details><summary>Stubbed APIs (2) &mdash; routed but return an empty/default response</summary>

- `DeleteResourceEndpointAssociation`
- `ListResourceEndpointAssociations`

</details>

<details><summary>Not yet implemented APIs (5)</summary>

- `DeleteServiceNetworkVpcAssociation`
- `GetServiceNetworkVpcAssociation`
- `ListServiceNetworkVpcAssociations` (implemented by moto)
- `ListServiceNetworkVpcEndpointAssociations`
- `UpdateServiceNetworkVpcAssociation`

</details>
