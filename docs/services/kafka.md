# winterbaume-kafka

Amazon MSK service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | MSK |
| AWS model | `kafka` |
| Protocol | restJson1 |
| winterbaume coverage | 10/59 operations (16.9%) |
| stubs (routed, returns empty/default) | 0/59 operations (0.0%) |
| moto coverage | 13/59 operations (22.0%) |
| floci coverage | 0/59 operations (0.0%) |
| kumo coverage | 6/59 operations (10.2%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws kafka list-clusters
```

## Current Network Resource Stub Semantics

Managed Streaming for Kafka currently keeps cluster networking in Kafka-local state.

- Provisioned cluster records store broker node client subnet IDs and security group IDs supplied at creation.
- Serverless cluster records store VPC configs with subnet and security group IDs.
- Cluster state, bootstrap brokers, and connectivity information are not derived from EC2 subnet, security group, or VPC endpoint state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_kafka::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kafka::KafkaService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(KafkaService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kafka::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kafka::Client::new(&config);

    let resp = client
        .list_clusters_v2()
        .send()
        .await
        .expect("list_clusters_v2 should succeed");
    println!("MSK clusters: {}", resp.cluster_info_list().len());
}
```

## Implemented APIs (10)

- `CreateCluster`
- `CreateClusterV2`
- `DeleteCluster`
- `DescribeCluster`
- `DescribeClusterV2`
- `ListClusters`
- `ListClustersV2`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (49)</summary>

- `BatchAssociateScramSecret`
- `BatchDisassociateScramSecret`
- `CreateConfiguration`
- `CreateReplicator`
- `CreateTopic`
- `CreateVpcConnection`
- `DeleteClusterPolicy` (implemented by moto)
- `DeleteConfiguration`
- `DeleteReplicator`
- `DeleteTopic`
- `DeleteVpcConnection`
- `DescribeClusterOperation`
- `DescribeClusterOperationV2`
- `DescribeConfiguration`
- `DescribeConfigurationRevision`
- `DescribeReplicator`
- `DescribeTopic`
- `DescribeTopicPartitions`
- `DescribeVpcConnection`
- `GetBootstrapBrokers` (implemented by kumo)
- `GetClusterPolicy` (implemented by moto)
- `GetCompatibleKafkaVersions`
- `ListClientVpcConnections`
- `ListClusterOperations`
- `ListClusterOperationsV2`
- `ListConfigurationRevisions`
- `ListConfigurations`
- `ListKafkaVersions`
- `ListNodes`
- `ListReplicators`
- `ListScramSecrets`
- `ListTopics`
- `ListVpcConnections`
- `PutClusterPolicy` (implemented by moto)
- `RebootBroker`
- `RejectClientVpcConnection`
- `UpdateBrokerCount`
- `UpdateBrokerStorage`
- `UpdateBrokerType`
- `UpdateClusterConfiguration` (implemented by kumo)
- `UpdateClusterKafkaVersion`
- `UpdateConfiguration`
- `UpdateConnectivity`
- `UpdateMonitoring`
- `UpdateRebalancing`
- `UpdateReplicationInfo`
- `UpdateSecurity`
- `UpdateStorage`
- `UpdateTopic`

</details>
