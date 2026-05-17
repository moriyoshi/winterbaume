# winterbaume-elasticloadbalancingv2

Elastic Load Balancing v2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | ELBv2 |
| AWS model | `elastic-load-balancing-v2` |
| Protocol | awsQuery |
| winterbaume coverage | 50/51 operations (98.0%) |
| stubs (routed, returns empty/default) | 1/51 operations (2.0%) |
| moto coverage | 33/51 operations (64.7%) |
| floci coverage | 0/51 operations (0.0%) |
| kumo coverage | 22/51 operations (43.1%) |
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
aws elbv2 describe-load-balancers
```

## Current Network Resource Stub Semantics

ELBv2 currently synthesises load balancer networking unless later mutation calls overwrite it.

- `CreateLoadBalancer` ignores the real EC2 topology and creates local load balancer records with default placeholder VPC, subnet, and security group values such as `vpc-12345678`, `subnet-aaaa1111`, and `sg-12345678`.
- `CreateTargetGroup` stores the supplied VPC ID when present and otherwise defaults to the same placeholder VPC ID.
- `SetSubnets` replaces the stored subnet or subnet-mapping list and synthesises availability-zone records from the supplied order; `SetSecurityGroups` replaces the stored security group list.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_elasticloadbalancingv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_elasticloadbalancingv2::ElasticLoadBalancingV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ElasticLoadBalancingV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticloadbalancingv2::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_elasticloadbalancingv2::Client::new(&config);

    let resp = client
        .describe_load_balancers()
        .send()
        .await
        .expect("describe_load_balancers should succeed");
    println!("Load balancers: {}", resp.load_balancers().len());
}
```

## Implemented APIs (50)

- `AddListenerCertificates`
- `AddTags`
- `AddTrustStoreRevocations`
- `CreateListener`
- `CreateLoadBalancer`
- `CreateRule`
- `CreateTargetGroup`
- `CreateTrustStore`
- `DeleteListener`
- `DeleteLoadBalancer`
- `DeleteRule`
- `DeleteSharedTrustStoreAssociation`
- `DeleteTargetGroup`
- `DeleteTrustStore`
- `DeregisterTargets`
- `DescribeCapacityReservation`
- `DescribeListenerAttributes`
- `DescribeListenerCertificates`
- `DescribeListeners`
- `DescribeLoadBalancerAttributes`
- `DescribeLoadBalancers`
- `DescribeRules`
- `DescribeSSLPolicies`
- `DescribeTags`
- `DescribeTargetGroupAttributes`
- `DescribeTargetGroups`
- `DescribeTargetHealth`
- `DescribeTrustStoreAssociations`
- `DescribeTrustStoreRevocations`
- `DescribeTrustStores`
- `GetResourcePolicy`
- `GetTrustStoreCaCertificatesBundle`
- `GetTrustStoreRevocationContent`
- `ModifyCapacityReservation`
- `ModifyIpPools`
- `ModifyListener`
- `ModifyListenerAttributes`
- `ModifyLoadBalancerAttributes`
- `ModifyRule`
- `ModifyTargetGroup`
- `ModifyTargetGroupAttributes`
- `ModifyTrustStore`
- `RegisterTargets`
- `RemoveListenerCertificates`
- `RemoveTags`
- `RemoveTrustStoreRevocations`
- `SetIpAddressType`
- `SetRulePriorities`
- `SetSecurityGroups`
- `SetSubnets`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `DescribeAccountLimits`

</details>
