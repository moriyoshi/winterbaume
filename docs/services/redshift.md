# winterbaume-redshift

Redshift service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Redshift |
| AWS model | `redshift` |
| Protocol | awsQuery |
| winterbaume coverage | 100/141 operations (70.9%) |
| stubs (routed, returns empty/default) | 3/141 operations (2.1%) |
| moto coverage | 35/141 operations (24.8%) |
| floci coverage | 0/141 operations (0.0%) |
| kumo coverage | 7/141 operations (5.0%) |
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
aws redshift describe-clusters
```

## Current Network Resource Stub Semantics

Redshift currently keeps subnet groups, VPC security groups, and endpoint access metadata in Redshift state.

- Cluster subnet groups store supplied subnet IDs and are referenced by clusters as local resource names.
- Clusters and endpoints store VPC security group IDs and endpoint subnet group information without EC2 validation.
- Endpoint and cluster lifecycle does not allocate ENIs or modify EC2 security groups.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_redshift::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_redshift::RedshiftService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RedshiftService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_redshift::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_redshift::Client::new(&config);

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    println!("Redshift clusters: {:?}", resp.clusters());
}
```

## Implemented APIs (100)

- `AddPartner`
- `AuthorizeClusterSecurityGroupIngress`
- `AuthorizeSnapshotAccess`
- `BatchDeleteClusterSnapshots`
- `BatchModifyClusterSnapshots`
- `CancelResize`
- `CopyClusterSnapshot`
- `CreateAuthenticationProfile`
- `CreateCluster`
- `CreateClusterParameterGroup`
- `CreateClusterSecurityGroup`
- `CreateClusterSnapshot`
- `CreateClusterSubnetGroup`
- `CreateEventSubscription`
- `CreateHsmClientCertificate`
- `CreateHsmConfiguration`
- `CreateScheduledAction`
- `CreateSnapshotCopyGrant`
- `CreateSnapshotSchedule`
- `CreateTags`
- `CreateUsageLimit`
- `DeleteAuthenticationProfile`
- `DeleteCluster`
- `DeleteClusterParameterGroup`
- `DeleteClusterSecurityGroup`
- `DeleteClusterSnapshot`
- `DeleteClusterSubnetGroup`
- `DeleteEventSubscription`
- `DeleteHsmClientCertificate`
- `DeleteHsmConfiguration`
- `DeletePartner`
- `DeleteResourcePolicy`
- `DeleteScheduledAction`
- `DeleteSnapshotCopyGrant`
- `DeleteSnapshotSchedule`
- `DeleteTags`
- `DeleteUsageLimit`
- `DescribeAccountAttributes`
- `DescribeAuthenticationProfiles`
- `DescribeClusterDbRevisions`
- `DescribeClusterParameterGroups`
- `DescribeClusterParameters`
- `DescribeClusterSecurityGroups`
- `DescribeClusterSnapshots`
- `DescribeClusterSubnetGroups`
- `DescribeClusterTracks`
- `DescribeClusterVersions`
- `DescribeClusters`
- `DescribeDefaultClusterParameters`
- `DescribeEventCategories`
- `DescribeEventSubscriptions`
- `DescribeHsmClientCertificates`
- `DescribeHsmConfigurations`
- `DescribeLoggingStatus`
- `DescribeNodeConfigurationOptions`
- `DescribeOrderableClusterOptions`
- `DescribePartners`
- `DescribeReservedNodeOfferings`
- `DescribeReservedNodes`
- `DescribeResize`
- `DescribeScheduledActions`
- `DescribeSnapshotCopyGrants`
- `DescribeSnapshotSchedules`
- `DescribeTableRestoreStatus`
- `DescribeTags`
- `DescribeUsageLimits`
- `DisableLogging`
- `DisableSnapshotCopy`
- `EnableLogging`
- `EnableSnapshotCopy`
- `FailoverPrimaryCompute`
- `GetClusterCredentials`
- `GetClusterCredentialsWithIAM`
- `GetResourcePolicy`
- `ModifyAquaConfiguration`
- `ModifyAuthenticationProfile`
- `ModifyCluster`
- `ModifyClusterDbRevision`
- `ModifyClusterIamRoles`
- `ModifyClusterMaintenance`
- `ModifyClusterParameterGroup`
- `ModifyClusterSnapshot`
- `ModifyClusterSnapshotSchedule`
- `ModifyClusterSubnetGroup`
- `ModifyEventSubscription`
- `ModifyScheduledAction`
- `ModifySnapshotCopyRetentionPeriod`
- `ModifySnapshotSchedule`
- `ModifyUsageLimit`
- `PauseCluster`
- `PutResourcePolicy`
- `RebootCluster`
- `ResetClusterParameterGroup`
- `ResizeCluster`
- `RestoreFromClusterSnapshot`
- `ResumeCluster`
- `RevokeClusterSecurityGroupIngress`
- `RevokeSnapshotAccess`
- `RotateEncryptionKey`
- `UpdatePartnerStatus`

<details><summary>Stubbed APIs (3) &mdash; routed but return an empty/default response</summary>

- `DescribeEvents`
- `DescribeStorage`
- `ListRecommendations`

</details>

<details><summary>Not yet implemented APIs (38)</summary>

- `AcceptReservedNodeExchange`
- `AssociateDataShareConsumer`
- `AuthorizeDataShare`
- `AuthorizeEndpointAccess`
- `CreateCustomDomainAssociation`
- `CreateEndpointAccess`
- `CreateIntegration`
- `CreateRedshiftIdcApplication`
- `DeauthorizeDataShare`
- `DeleteCustomDomainAssociation`
- `DeleteEndpointAccess`
- `DeleteIntegration`
- `DeleteRedshiftIdcApplication`
- `DeregisterNamespace`
- `DescribeCustomDomainAssociations`
- `DescribeDataShares`
- `DescribeDataSharesForConsumer`
- `DescribeDataSharesForProducer`
- `DescribeEndpointAccess`
- `DescribeEndpointAuthorization`
- `DescribeInboundIntegrations`
- `DescribeIntegrations`
- `DescribeRedshiftIdcApplications`
- `DescribeReservedNodeExchangeStatus`
- `DisassociateDataShareConsumer`
- `GetIdentityCenterAuthToken`
- `GetReservedNodeExchangeConfigurationOptions`
- `GetReservedNodeExchangeOfferings`
- `ModifyCustomDomainAssociation`
- `ModifyEndpointAccess`
- `ModifyIntegration`
- `ModifyLakehouseConfiguration`
- `ModifyRedshiftIdcApplication`
- `PurchaseReservedNodeOffering`
- `RegisterNamespace`
- `RejectDataShare`
- `RestoreTableFromClusterSnapshot`
- `RevokeEndpointAccess`

</details>
