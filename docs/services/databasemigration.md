# winterbaume-databasemigration

Database Migration Service (DMS) implementation for winterbaume.

**Note:** the example below calls the service handler directly via `MockRequest`. To use the AWS SDK Rust client, depend on `aws-sdk-databasemigration` ( the SDK crate name omits the trailing `service` ).

## Coverage

| Metric | Value |
|---|---|
| Service | Database Migration Service (DMS) |
| AWS model | `database-migration-service` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 42/119 operations (35.3%) |
| stubs (routed, returns empty/default) | 0/119 operations (0.0%) |
| moto coverage | 17/119 operations (14.3%) |
| floci coverage | 0/119 operations (0.0%) |
| kumo coverage | 0/119 operations (0.0%) |
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
aws dms describe-replication-instances
```

## Current Network Resource Stub Semantics

Database Migration Service currently keeps replication subnet groups as DMS-local records.

- `CreateReplicationSubnetGroup` stores the supplied subnet IDs and optional VPC ID on a `ReplicationSubnetGroup`.
- The subnet group status is returned as complete from local state; there is no asynchronous subnet validation or EC2 subnet-to-VPC derivation.
- Replication instances and tasks can refer to DMS resources independently of EC2 networking availability.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use bytes::Bytes;
use serde_json::{Value, json};
use winterbaume_core::{MockRequest, MockService};
use winterbaume_databasemigration::DatabaseMigrationService;

fn dms_request(action: &str, body: Value) -> MockRequest {
    let mut headers = http::HeaderMap::new();
    headers.insert(
        "x-amz-target",
        format!("AmazonDMSv20160101.{action}").parse().unwrap(),
    );
    headers.insert(
        http::header::CONTENT_TYPE,
        "application/x-amz-json-1.1".parse().unwrap(),
    );
    MockRequest {
        method: "POST".to_string(),
        uri: "https://dms.us-east-1.amazonaws.com".to_string(),
        headers,
        body: Bytes::from(body.to_string()),
    }
}

#[tokio::main]
async fn main() {
    let service = DatabaseMigrationService::new();

    // Create a replication instance
    let resp = service
        .handle(dms_request(
            "CreateReplicationInstance",
            json!({
                "ReplicationInstanceIdentifier": "example-instance",
                "ReplicationInstanceClass": "dms.t3.medium",
                "AllocatedStorage": 50
            }),
        ))
        .await;
    let body: Value = serde_json::from_slice(&resp.body).unwrap();
    println!("Created replication instance:");
    println!(
        "  ARN: {}",
        body["ReplicationInstance"]["ReplicationInstanceArn"]
            .as_str()
            .unwrap_or("N/A")
    );
    println!(
        "  Status: {}",
        body["ReplicationInstance"]["ReplicationInstanceStatus"]
            .as_str()
            .unwrap_or("N/A")
    );

    // Describe replication instances
    let resp = service
        .handle(dms_request("DescribeReplicationInstances", json!({})))
        .await;
    let body: Value = serde_json::from_slice(&resp.body).unwrap();
    let instances = body["ReplicationInstances"].as_array().unwrap();
    println!(
        "\nDescribeReplicationInstances: {} instance(s)",
        instances.len()
    );
    for inst in instances {
        println!(
            "  - {} ({})",
            inst["ReplicationInstanceIdentifier"].as_str().unwrap_or(""),
            inst["ReplicationInstanceClass"].as_str().unwrap_or("")
        );
    }
}
```

## Implemented APIs (42)

- `AddTagsToResource`
- `CreateEndpoint`
- `CreateEventSubscription`
- `CreateReplicationInstance`
- `CreateReplicationSubnetGroup`
- `CreateReplicationTask`
- `DeleteCertificate`
- `DeleteEndpoint`
- `DeleteEventSubscription`
- `DeleteReplicationInstance`
- `DeleteReplicationSubnetGroup`
- `DeleteReplicationTask`
- `DescribeAccountAttributes`
- `DescribeApplicableIndividualAssessments`
- `DescribeCertificates`
- `DescribeConnections`
- `DescribeEndpointSettings`
- `DescribeEndpointTypes`
- `DescribeEndpoints`
- `DescribeEngineVersions`
- `DescribeEventCategories`
- `DescribeEventSubscriptions`
- `DescribeEvents`
- `DescribeOrderableReplicationInstances`
- `DescribePendingMaintenanceActions`
- `DescribeReplicationInstances`
- `DescribeReplicationSubnetGroups`
- `DescribeReplicationTaskAssessmentResults`
- `DescribeReplicationTaskAssessmentRuns`
- `DescribeReplicationTaskIndividualAssessments`
- `DescribeReplicationTasks`
- `ImportCertificate`
- `ListTagsForResource`
- `ModifyEndpoint`
- `ModifyEventSubscription`
- `ModifyReplicationInstance`
- `ModifyReplicationSubnetGroup`
- `ModifyReplicationTask`
- `RemoveTagsFromResource`
- `StartReplicationTask`
- `StopReplicationTask`
- `TestConnection`

<details><summary>Not yet implemented APIs (77)</summary>

- `ApplyPendingMaintenanceAction`
- `BatchStartRecommendations`
- `CancelMetadataModelConversion`
- `CancelMetadataModelCreation`
- `CancelReplicationTaskAssessmentRun`
- `CreateDataMigration`
- `CreateDataProvider`
- `CreateFleetAdvisorCollector`
- `CreateInstanceProfile`
- `CreateMigrationProject`
- `CreateReplicationConfig`
- `DeleteConnection`
- `DeleteDataMigration`
- `DeleteDataProvider`
- `DeleteFleetAdvisorCollector`
- `DeleteFleetAdvisorDatabases`
- `DeleteInstanceProfile`
- `DeleteMigrationProject`
- `DeleteReplicationConfig`
- `DeleteReplicationTaskAssessmentRun`
- `DescribeConversionConfiguration`
- `DescribeDataMigrations`
- `DescribeDataProviders`
- `DescribeExtensionPackAssociations`
- `DescribeFleetAdvisorCollectors`
- `DescribeFleetAdvisorDatabases`
- `DescribeFleetAdvisorLsaAnalysis`
- `DescribeFleetAdvisorSchemaObjectSummary`
- `DescribeFleetAdvisorSchemas`
- `DescribeInstanceProfiles`
- `DescribeMetadataModel`
- `DescribeMetadataModelAssessments`
- `DescribeMetadataModelChildren`
- `DescribeMetadataModelConversions`
- `DescribeMetadataModelCreations`
- `DescribeMetadataModelExportsAsScript`
- `DescribeMetadataModelExportsToTarget`
- `DescribeMetadataModelImports`
- `DescribeMigrationProjects`
- `DescribeRecommendationLimitations`
- `DescribeRecommendations`
- `DescribeRefreshSchemasStatus`
- `DescribeReplicationConfigs`
- `DescribeReplicationInstanceTaskLogs`
- `DescribeReplicationTableStatistics`
- `DescribeReplications`
- `DescribeSchemas`
- `DescribeTableStatistics`
- `ExportMetadataModelAssessment`
- `GetTargetSelectionRules`
- `ModifyConversionConfiguration`
- `ModifyDataMigration`
- `ModifyDataProvider`
- `ModifyInstanceProfile`
- `ModifyMigrationProject`
- `ModifyReplicationConfig`
- `MoveReplicationTask`
- `RebootReplicationInstance`
- `RefreshSchemas`
- `ReloadReplicationTables`
- `ReloadTables`
- `RunFleetAdvisorLsaAnalysis`
- `StartDataMigration`
- `StartExtensionPackAssociation`
- `StartMetadataModelAssessment`
- `StartMetadataModelConversion`
- `StartMetadataModelCreation`
- `StartMetadataModelExportAsScript`
- `StartMetadataModelExportToTarget`
- `StartMetadataModelImport`
- `StartRecommendations`
- `StartReplication`
- `StartReplicationTaskAssessment`
- `StartReplicationTaskAssessmentRun`
- `StopDataMigration`
- `StopReplication`
- `UpdateSubscriptionsToEventBridge`

</details>
