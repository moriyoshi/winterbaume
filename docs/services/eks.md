# winterbaume-eks

EKS service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | EKS |
| AWS model | `eks` |
| Protocol | restJson1 |
| winterbaume coverage | 55/64 operations (85.9%) |
| stubs (routed, returns empty/default) | 4/64 operations (6.2%) |
| moto coverage | 17/64 operations (26.6%) |
| floci coverage | 0/64 operations (0.0%) |
| kumo coverage | 8/64 operations (12.5%) |
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
aws eks list-clusters
```

## Current Network Resource Stub Semantics

EKS currently treats cluster networking values as cluster metadata.

- Cluster VPC configuration fields such as subnet IDs, security group IDs, cluster security group ID, endpoint access flags, and public access CIDRs are stored or returned with the cluster record when handled.
- Node group and Fargate profile subnet references are service-local placement fields.
- The implementation does not allocate ENIs, reconcile control-plane security groups, or enforce subnet/VPC consistency.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_eks::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_eks::EksService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EksService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_eks::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_eks::Client::new(&config);

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");
    println!("EKS clusters: {}", resp.clusters().len());
}
```

## Implemented APIs (55)

- `AssociateAccessPolicy`
- `AssociateEncryptionConfig`
- `AssociateIdentityProviderConfig`
- `CreateAccessEntry`
- `CreateAddon`
- `CreateCapability`
- `CreateCluster`
- `CreateFargateProfile`
- `CreateNodegroup`
- `CreatePodIdentityAssociation`
- `DeleteAccessEntry`
- `DeleteAddon`
- `DeleteCapability`
- `DeleteCluster`
- `DeleteFargateProfile`
- `DeleteNodegroup`
- `DeletePodIdentityAssociation`
- `DeregisterCluster`
- `DescribeAccessEntry`
- `DescribeAddon`
- `DescribeAddonConfiguration`
- `DescribeAddonVersions`
- `DescribeCapability`
- `DescribeCluster`
- `DescribeClusterVersions`
- `DescribeFargateProfile`
- `DescribeIdentityProviderConfig`
- `DescribeNodegroup`
- `DescribePodIdentityAssociation`
- `DescribeUpdate`
- `DisassociateAccessPolicy`
- `DisassociateIdentityProviderConfig`
- `ListAccessEntries`
- `ListAccessPolicies`
- `ListAddons`
- `ListAssociatedAccessPolicies`
- `ListCapabilities`
- `ListClusters`
- `ListFargateProfiles`
- `ListIdentityProviderConfigs`
- `ListNodegroups`
- `ListPodIdentityAssociations`
- `ListTagsForResource`
- `ListUpdates`
- `RegisterCluster`
- `TagResource`
- `UntagResource`
- `UpdateAccessEntry`
- `UpdateAddon`
- `UpdateCapability`
- `UpdateClusterConfig`
- `UpdateClusterVersion`
- `UpdateNodegroupConfig`
- `UpdateNodegroupVersion`
- `UpdatePodIdentityAssociation`

<details><summary>Stubbed APIs (4) &mdash; routed but return an empty/default response</summary>

- `DescribeInsight`
- `DescribeInsightsRefresh`
- `ListInsights`
- `StartInsightsRefresh`

</details>

<details><summary>Not yet implemented APIs (5)</summary>

- `CreateEksAnywhereSubscription`
- `DeleteEksAnywhereSubscription`
- `DescribeEksAnywhereSubscription`
- `ListEksAnywhereSubscriptions`
- `UpdateEksAnywhereSubscription`

</details>
