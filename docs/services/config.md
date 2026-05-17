# winterbaume-config

AWS Config service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Config |
| AWS model | `config-service` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 46/97 operations (47.4%) |
| stubs (routed, returns empty/default) | 3/97 operations (3.1%) |
| moto coverage | 38/97 operations (39.2%) |
| floci coverage | 0/97 operations (0.0%) |
| kumo coverage | 9/97 operations (9.3%) |
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
aws configservice describe-config-rules
```

## Example

```rust
use aws_sdk_config::config::BehaviorVersion;
use winterbaume_config::ConfigService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConfigService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_config::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_config::Client::new(&config);

    let resp = client
        .describe_configuration_recorders()
        .send()
        .await
        .expect("describe_configuration_recorders should succeed");
    println!(
        "Configuration recorders: {}",
        resp.configuration_recorders().len()
    );
}
```

## Implemented APIs (46)

- `BatchGetAggregateResourceConfig`
- `BatchGetResourceConfig`
- `DeleteAggregationAuthorization`
- `DeleteConfigRule`
- `DeleteConfigurationAggregator`
- `DeleteConfigurationRecorder`
- `DeleteDeliveryChannel`
- `DeleteOrganizationConfigRule`
- `DeleteOrganizationConformancePack`
- `DeleteRemediationConfiguration`
- `DeleteResourceConfig`
- `DeleteRetentionConfiguration`
- `DescribeAggregationAuthorizations`
- `DescribeComplianceByConfigRule`
- `DescribeConfigRuleEvaluationStatus`
- `DescribeConfigRules`
- `DescribeConfigurationAggregators`
- `DescribeConfigurationRecorderStatus`
- `DescribeConfigurationRecorders`
- `DescribeDeliveryChannels`
- `DescribeOrganizationConfigRules`
- `DescribeOrganizationConformancePackStatuses`
- `DescribeOrganizationConformancePacks`
- `DescribeRemediationConfigurations`
- `DescribeRetentionConfigurations`
- `GetComplianceDetailsByConfigRule`
- `GetOrganizationConformancePackDetailedStatus`
- `GetResourceConfigHistory`
- `ListAggregateDiscoveredResources`
- `ListDiscoveredResources`
- `ListTagsForResource`
- `PutAggregationAuthorization`
- `PutConfigRule`
- `PutConfigurationAggregator`
- `PutConfigurationRecorder`
- `PutDeliveryChannel`
- `PutEvaluations`
- `PutOrganizationConfigRule`
- `PutOrganizationConformancePack`
- `PutRemediationConfigurations`
- `PutResourceConfig`
- `PutRetentionConfiguration`
- `StartConfigurationRecorder`
- `StopConfigurationRecorder`
- `TagResource`
- `UntagResource`

<details><summary>Stubbed APIs (3) &mdash; routed but return an empty/default response</summary>

- `SelectResourceConfig`
- `StartConfigRulesEvaluation`
- `StartRemediationExecution`

</details>

<details><summary>Not yet implemented APIs (48)</summary>

- `AssociateResourceTypes`
- `DeleteConformancePack`
- `DeleteEvaluationResults`
- `DeletePendingAggregationRequest`
- `DeleteRemediationExceptions`
- `DeleteServiceLinkedConfigurationRecorder`
- `DeleteStoredQuery`
- `DeliverConfigSnapshot`
- `DescribeAggregateComplianceByConfigRules`
- `DescribeAggregateComplianceByConformancePacks`
- `DescribeComplianceByResource`
- `DescribeConfigurationAggregatorSourcesStatus`
- `DescribeConformancePackCompliance`
- `DescribeConformancePackStatus`
- `DescribeConformancePacks`
- `DescribeDeliveryChannelStatus`
- `DescribeOrganizationConfigRuleStatuses`
- `DescribePendingAggregationRequests`
- `DescribeRemediationExceptions`
- `DescribeRemediationExecutionStatus`
- `DisassociateResourceTypes`
- `GetAggregateComplianceDetailsByConfigRule`
- `GetAggregateConfigRuleComplianceSummary`
- `GetAggregateConformancePackComplianceSummary`
- `GetAggregateDiscoveredResourceCounts`
- `GetAggregateResourceConfig`
- `GetComplianceDetailsByResource`
- `GetComplianceSummaryByConfigRule`
- `GetComplianceSummaryByResourceType`
- `GetConformancePackComplianceDetails`
- `GetConformancePackComplianceSummary`
- `GetCustomRulePolicy`
- `GetDiscoveredResourceCounts`
- `GetOrganizationConfigRuleDetailedStatus`
- `GetOrganizationCustomRulePolicy`
- `GetResourceEvaluationSummary`
- `GetStoredQuery`
- `ListConfigurationRecorders`
- `ListConformancePackComplianceScores`
- `ListResourceEvaluations`
- `ListStoredQueries`
- `PutConformancePack`
- `PutExternalEvaluation`
- `PutRemediationExceptions`
- `PutServiceLinkedConfigurationRecorder`
- `PutStoredQuery`
- `SelectAggregateResourceConfig`
- `StartResourceEvaluation`

</details>
