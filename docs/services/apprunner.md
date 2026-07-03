# winterbaume-apprunner

AWS App Runner service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | App Runner |
| AWS model | `apprunner` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 23/37 operations (62.2%) |
| stubs (routed, returns empty/default) | 0/37 operations (0.0%) |
| moto coverage | 0/37 operations (0.0%) |
| floci coverage | 0/37 operations (0.0%) |
| kumo coverage | 0/37 operations (0.0%) |
| fakecloud coverage | 0/37 operations (0.0%) |
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
aws apprunner help
```

## Current Network Resource Stub Semantics

App Runner exposes VPC-aware operations in the model but currently does not persist any networking resource state.

- `CreateVpcConnector`, `DescribeVpcConnector`, `ListVpcConnectors`, and `DeleteVpcConnector` return `501 NotImplemented`.
- `CreateVpcIngressConnection`, `DescribeVpcIngressConnection`, `ListVpcIngressConnections`, `UpdateVpcIngressConnection`, and `DeleteVpcIngressConnection` also return `501 NotImplemented`.
- No VPC connector, subnet, security group, ingress connection, or PrivateLink-style state is created in Winterbaume.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_apprunner::config::BehaviorVersion;
use winterbaume_apprunner::AppRunnerService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppRunnerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apprunner::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_apprunner::Client::new(&config);

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    println!("App Runner services: {}", resp.service_summary_list().len());
}
```

## Implemented APIs (23)

- `CreateAutoScalingConfiguration`
- `CreateConnection`
- `CreateService`
- `CreateVpcConnector`
- `DeleteAutoScalingConfiguration`
- `DeleteConnection`
- `DeleteService`
- `DeleteVpcConnector`
- `DescribeAutoScalingConfiguration`
- `DescribeService`
- `ListAutoScalingConfigurations`
- `ListConnections`
- `ListOperations`
- `ListServices`
- `ListTagsForResource`
- `ListVpcConnectors`
- `PauseService`
- `ResumeService`
- `StartDeployment`
- `TagResource`
- `UntagResource`
- `UpdateDefaultAutoScalingConfiguration`
- `UpdateService`

<details><summary>Not yet implemented APIs (14)</summary>

- `AssociateCustomDomain`
- `CreateObservabilityConfiguration`
- `CreateVpcIngressConnection`
- `DeleteObservabilityConfiguration`
- `DeleteVpcIngressConnection`
- `DescribeCustomDomains`
- `DescribeObservabilityConfiguration`
- `DescribeVpcConnector`
- `DescribeVpcIngressConnection`
- `DisassociateCustomDomain`
- `ListObservabilityConfigurations`
- `ListServicesForAutoScalingConfiguration`
- `ListVpcIngressConnections`
- `UpdateVpcIngressConnection`

</details>
