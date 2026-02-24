# winterbaume-networkfirewall

AWS Network Firewall service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Network Firewall |
| AWS model | `network-firewall` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 79/79 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/79 operations (0.0%) |
| moto coverage | 5/79 operations (6.3%) |
| floci coverage | 0/79 operations (0.0%) |
| kumo coverage | 0/79 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws network-firewall list-firewalls
```

## Current Network Resource Stub Semantics

Network Firewall currently stores firewall subnet attachments and VPC endpoint associations inside Network Firewall state.

- Firewall records keep the supplied VPC ID, subnet mappings, and subnet-change-protection flag.
- `AssociateSubnets` appends new subnet mappings when absent, and `DisassociateSubnets` removes matching subnet IDs from the local firewall record.
- VPC endpoint association records store a VPC ID and subnet ID and are listed from Network Firewall state only.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_networkfirewall::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_networkfirewall::NetworkFirewallService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(NetworkFirewallService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_networkfirewall::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_networkfirewall::Client::new(&config);

    let resp = client
        .list_firewalls()
        .send()
        .await
        .expect("list_firewalls should succeed");
    println!("Network firewalls: {}", resp.firewalls().len());
}
```

## Implemented APIs (79)

- `AcceptNetworkFirewallTransitGatewayAttachment`
- `AssociateAvailabilityZones`
- `AssociateFirewallPolicy`
- `AssociateSubnets`
- `AttachRuleGroupsToProxyConfiguration`
- `CreateFirewall`
- `CreateFirewallPolicy`
- `CreateProxy`
- `CreateProxyConfiguration`
- `CreateProxyRuleGroup`
- `CreateProxyRules`
- `CreateRuleGroup`
- `CreateTLSInspectionConfiguration`
- `CreateVpcEndpointAssociation`
- `DeleteFirewall`
- `DeleteFirewallPolicy`
- `DeleteNetworkFirewallTransitGatewayAttachment`
- `DeleteProxy`
- `DeleteProxyConfiguration`
- `DeleteProxyRuleGroup`
- `DeleteProxyRules`
- `DeleteResourcePolicy`
- `DeleteRuleGroup`
- `DeleteTLSInspectionConfiguration`
- `DeleteVpcEndpointAssociation`
- `DescribeFirewall`
- `DescribeFirewallMetadata`
- `DescribeFirewallPolicy`
- `DescribeFlowOperation`
- `DescribeLoggingConfiguration`
- `DescribeProxy`
- `DescribeProxyConfiguration`
- `DescribeProxyRule`
- `DescribeProxyRuleGroup`
- `DescribeResourcePolicy`
- `DescribeRuleGroup`
- `DescribeRuleGroupMetadata`
- `DescribeRuleGroupSummary`
- `DescribeTLSInspectionConfiguration`
- `DescribeVpcEndpointAssociation`
- `DetachRuleGroupsFromProxyConfiguration`
- `DisassociateAvailabilityZones`
- `DisassociateSubnets`
- `GetAnalysisReportResults`
- `ListAnalysisReports`
- `ListFirewallPolicies`
- `ListFirewalls`
- `ListFlowOperationResults`
- `ListFlowOperations`
- `ListProxies`
- `ListProxyConfigurations`
- `ListProxyRuleGroups`
- `ListRuleGroups`
- `ListTLSInspectionConfigurations`
- `ListTagsForResource`
- `ListVpcEndpointAssociations`
- `PutResourcePolicy`
- `RejectNetworkFirewallTransitGatewayAttachment`
- `StartAnalysisReport`
- `StartFlowCapture`
- `StartFlowFlush`
- `TagResource`
- `UntagResource`
- `UpdateAvailabilityZoneChangeProtection`
- `UpdateFirewallAnalysisSettings`
- `UpdateFirewallDeleteProtection`
- `UpdateFirewallDescription`
- `UpdateFirewallEncryptionConfiguration`
- `UpdateFirewallPolicy`
- `UpdateFirewallPolicyChangeProtection`
- `UpdateLoggingConfiguration`
- `UpdateProxy`
- `UpdateProxyConfiguration`
- `UpdateProxyRule`
- `UpdateProxyRuleGroupPriorities`
- `UpdateProxyRulePriorities`
- `UpdateRuleGroup`
- `UpdateSubnetChangeProtection`
- `UpdateTLSInspectionConfiguration`
