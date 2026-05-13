# winterbaume-elasticloadbalancing

Classic Elastic Load Balancing service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | ELB |
| AWS model | `elastic-load-balancing` |
| Protocol | awsQuery |
| winterbaume coverage | 29/29 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/29 operations (0.0%) |
| moto coverage | 21/29 operations (72.4%) |
| floci coverage | 0/29 operations (0.0%) |
| kumo coverage | 0/29 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws elb describe-load-balancers
```

## Current Network Resource Stub Semantics

Classic ELB currently stores subnet and security group attachments on the load balancer record.

- `CreateLoadBalancer` records supplied subnets and security groups when present and otherwise uses local defaults.
- `AttachLoadBalancerToSubnets` appends requested subnet IDs to the stored list, `DetachLoadBalancerFromSubnets` removes them, and `ApplySecurityGroupsToLoadBalancer` replaces the stored security group list.
- Source security group and VPC fields are response metadata derived by the ELB crate, not from EC2 security group ownership.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_elasticloadbalancing::config::BehaviorVersion;
use aws_sdk_elasticloadbalancing::types::Listener;
use winterbaume_core::MockAws;
use winterbaume_elasticloadbalancing::ElasticLoadBalancingService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ElasticLoadBalancingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticloadbalancing::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_elasticloadbalancing::Client::new(&config);

    // Create a classic load balancer
    let create_resp = client
        .create_load_balancer()
        .load_balancer_name("my-classic-lb")
        .availability_zones("us-east-1a")
        .listeners(
            Listener::builder()
                .protocol("HTTP")
                .load_balancer_port(80)
                .instance_port(8080)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_load_balancer should succeed");

    println!(
        "Created load balancer with DNS: {:?}",
        create_resp.dns_name()
    );

    // Describe all load balancers
    let resp = client
        .describe_load_balancers()
        .send()
        .await
        .expect("describe_load_balancers should succeed");

    println!(
        "Classic ELB load balancers: {:?}",
        resp.load_balancer_descriptions()
    );
}
```

## Implemented APIs (29)

- `AddTags`
- `ApplySecurityGroupsToLoadBalancer`
- `AttachLoadBalancerToSubnets`
- `ConfigureHealthCheck`
- `CreateAppCookieStickinessPolicy`
- `CreateLBCookieStickinessPolicy`
- `CreateLoadBalancer`
- `CreateLoadBalancerListeners`
- `CreateLoadBalancerPolicy`
- `DeleteLoadBalancer`
- `DeleteLoadBalancerListeners`
- `DeleteLoadBalancerPolicy`
- `DeregisterInstancesFromLoadBalancer`
- `DescribeAccountLimits`
- `DescribeInstanceHealth`
- `DescribeLoadBalancerAttributes`
- `DescribeLoadBalancerPolicies`
- `DescribeLoadBalancerPolicyTypes`
- `DescribeLoadBalancers`
- `DescribeTags`
- `DetachLoadBalancerFromSubnets`
- `DisableAvailabilityZonesForLoadBalancer`
- `EnableAvailabilityZonesForLoadBalancer`
- `ModifyLoadBalancerAttributes`
- `RegisterInstancesWithLoadBalancer`
- `RemoveTags`
- `SetLoadBalancerListenerSSLCertificate`
- `SetLoadBalancerPoliciesForBackendServer`
- `SetLoadBalancerPoliciesOfListener`
