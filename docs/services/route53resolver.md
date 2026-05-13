# winterbaume-route53resolver

Route 53 Resolver service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Route 53 Resolver |
| AWS model | `route53resolver` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 28/68 operations (41.2%) |
| stubs (routed, returns empty/default) | 0/68 operations (0.0%) |
| moto coverage | 28/68 operations (41.2%) |
| floci coverage | 0/68 operations (0.0%) |
| kumo coverage | 11/68 operations (16.2%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws route53resolver list-resolver-endpoints
```

## Current Network Resource Stub Semantics

Route 53 Resolver currently stores resolver endpoint and rule association networking locally.

- Resolver endpoints store security group IDs, direction, host VPC ID, and IP address records with subnet IDs.
- Endpoint creation mints a synthetic host VPC ID instead of resolving it from the requested subnets.
- Resolver rule associations store resolver rule ID plus VPC ID pairs and enforce duplicates only within Route 53 Resolver state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_route53resolver::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53resolver::Route53ResolverService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Route53ResolverService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53resolver::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_route53resolver::Client::new(&config);

    let resp = client
        .list_resolver_endpoints()
        .send()
        .await
        .expect("list_resolver_endpoints should succeed");
    println!(
        "Route 53 Resolver endpoints: {}",
        resp.resolver_endpoints().len()
    );
}
```

## Implemented APIs (28)

- `AssociateResolverEndpointIpAddress`
- `AssociateResolverQueryLogConfig`
- `AssociateResolverRule`
- `CreateResolverEndpoint`
- `CreateResolverQueryLogConfig`
- `CreateResolverRule`
- `DeleteResolverEndpoint`
- `DeleteResolverRule`
- `DisassociateResolverEndpointIpAddress`
- `DisassociateResolverRule`
- `GetResolverDnssecConfig`
- `GetResolverEndpoint`
- `GetResolverQueryLogConfig`
- `GetResolverQueryLogConfigAssociation`
- `GetResolverRule`
- `GetResolverRuleAssociation`
- `ListResolverDnssecConfigs`
- `ListResolverEndpointIpAddresses`
- `ListResolverEndpoints`
- `ListResolverQueryLogConfigAssociations`
- `ListResolverQueryLogConfigs`
- `ListResolverRuleAssociations`
- `ListResolverRules`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateResolverDnssecConfig`
- `UpdateResolverEndpoint`

<details><summary>Not yet implemented APIs (40)</summary>

- `AssociateFirewallRuleGroup`
- `CreateFirewallDomainList`
- `CreateFirewallRule`
- `CreateFirewallRuleGroup`
- `CreateOutpostResolver`
- `DeleteFirewallDomainList`
- `DeleteFirewallRule`
- `DeleteFirewallRuleGroup`
- `DeleteOutpostResolver`
- `DeleteResolverQueryLogConfig`
- `DisassociateFirewallRuleGroup`
- `DisassociateResolverQueryLogConfig`
- `GetFirewallConfig`
- `GetFirewallDomainList`
- `GetFirewallRuleGroup`
- `GetFirewallRuleGroupAssociation`
- `GetFirewallRuleGroupPolicy`
- `GetOutpostResolver`
- `GetResolverConfig`
- `GetResolverQueryLogConfigPolicy`
- `GetResolverRulePolicy`
- `ImportFirewallDomains`
- `ListFirewallConfigs`
- `ListFirewallDomainLists`
- `ListFirewallDomains`
- `ListFirewallRuleGroupAssociations`
- `ListFirewallRuleGroups`
- `ListFirewallRules`
- `ListOutpostResolvers`
- `ListResolverConfigs`
- `PutFirewallRuleGroupPolicy`
- `PutResolverQueryLogConfigPolicy`
- `PutResolverRulePolicy`
- `UpdateFirewallConfig`
- `UpdateFirewallDomains`
- `UpdateFirewallRule`
- `UpdateFirewallRuleGroupAssociation`
- `UpdateOutpostResolver`
- `UpdateResolverConfig`
- `UpdateResolverRule`

</details>
