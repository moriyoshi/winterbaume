# winterbaume-servicediscovery

AWS Cloud Map (servicediscovery) service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Service Discovery |
| AWS model | `servicediscovery` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 27/30 operations (90.0%) |
| stubs (routed, returns empty/default) | 0/30 operations (0.0%) |
| moto coverage | 27/30 operations (90.0%) |
| floci coverage | 0/30 operations (0.0%) |
| kumo coverage | 0/30 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws servicediscovery list-namespaces
```

## Current Network Resource Stub Semantics

Cloud Map service discovery currently stores private namespace VPC IDs inside Cloud Map state.

- Private DNS namespaces record the supplied VPC ID as raw namespace metadata.
- The state enforces service-discovery-local uniqueness for private DNS namespaces associated with the same VPC.
- Public DNS and HTTP namespaces have no VPC value, and instance registration is independent of EC2 networking.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_servicediscovery::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicediscovery::ServiceDiscoveryService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ServiceDiscoveryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicediscovery::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_servicediscovery::Client::new(&config);

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    println!("Service Discovery services: {}", resp.services().len());
}
```

## Implemented APIs (27)

- `CreateHttpNamespace`
- `CreatePrivateDnsNamespace`
- `CreatePublicDnsNamespace`
- `CreateService`
- `DeleteNamespace`
- `DeleteService`
- `DeregisterInstance`
- `DiscoverInstances`
- `DiscoverInstancesRevision`
- `GetInstance`
- `GetInstancesHealthStatus`
- `GetNamespace`
- `GetOperation`
- `GetService`
- `ListInstances`
- `ListNamespaces`
- `ListOperations`
- `ListServices`
- `ListTagsForResource`
- `RegisterInstance`
- `TagResource`
- `UntagResource`
- `UpdateHttpNamespace`
- `UpdateInstanceCustomHealthStatus`
- `UpdatePrivateDnsNamespace`
- `UpdatePublicDnsNamespace`
- `UpdateService`

<details><summary>Not yet implemented APIs (3)</summary>

- `DeleteServiceAttributes`
- `GetServiceAttributes`
- `UpdateServiceAttributes`

</details>
