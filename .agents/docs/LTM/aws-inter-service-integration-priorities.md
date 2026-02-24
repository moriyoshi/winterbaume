# AWS Inter-service Integration Priorities

## Summary

Winterbaume now covers enough individual AWS services that the next high-value gaps are often between services rather than within one crate. This document captures the AWS-documented integration seams that most directly overlap the current workspace, with emphasis on contracts that can drive emulator behaviour, cross-service tests, and future prioritisation. The durable lesson is to prefer documented service relationships over invented mock-only coupling, and to record precise scope where AWS only supports a narrow subset of operations.

## Key Facts

- AppSync is the strongest current integration hub because AWS documents direct data-source paths to DynamoDB, Lambda, OpenSearch, RDS or Aurora, HTTP endpoints, and EventBridge.
- EventBridge and Pipes are the strongest fan-out hubs because they bridge streams, queues, and rule targets that Winterbaume already implements.
- Lambda remains the highest-leverage execution target because DynamoDB Streams, SQS, SNS, and Kinesis all have documented event-source integrations into Lambda.
- Step Functions has several documented direct service integrations, but each one has a specific scope. For DynamoDB, the optimised integration is only `GetItem`, `PutItem`, `UpdateItem`, and `DeleteItem`.
- AppSync and EventBridge are bidirectional in AWS docs: EventBridge can target an AppSync mutation, and AppSync can emit events through an EventBridge data source.
- The immediate value of this map is prioritisation, not completeness. It highlights the best cross-service seams for future emulator behaviour and integration tests.

## Details

### Highest-Value Correlations

| Correlation | AWS-documented contract | Winterbaume relevance |
|-------------|-------------------------|-----------------------|
| AppSync ↔ DynamoDB | AppSync supports DynamoDB data sources, including versioned data sources and transaction-oriented resolver flows. | Most direct GraphQL-backed persistence path between `winterbaume-appsync` and `winterbaume-dynamodb`. |
| AppSync ↔ Lambda | AppSync supports Lambda data sources and Lambda resolvers, including batching. | Important for resolver execution, error propagation, and N+1-style behaviour. |
| AppSync ↔ OpenSearch | AppSync supports OpenSearch data sources. | Natural bridge between `winterbaume-appsync` and `winterbaume-opensearch`. |
| AppSync ↔ RDS Data API | AppSync supports Aurora or RDS-backed resolver flows through the Data API. | Natural bridge between `winterbaume-appsync`, `winterbaume-rds`, and `winterbaume-rdsdata`. |
| AppSync ↔ HTTP endpoints | AppSync supports HTTP data sources, including API Gateway-backed endpoints. | Practical AppSync ↔ API Gateway ↔ Lambda path without inventing a bespoke contract. |
| AppSync ↔ EventBridge | AppSync also documents EventBridge as a data-source type. | Means `winterbaume-appsync` may eventually need EventBridge data-source support, not only EventBridge-as-target behaviour. |
| EventBridge → AppSync | EventBridge rules can target a public AppSync GraphQL API mutation. | High-value event-to-GraphQL fan-out path for `winterbaume-eventbridge` plus `winterbaume-appsync`. |
| DynamoDB Streams → Lambda | Lambda supports DynamoDB Streams event-source mappings. | Strong seam across `winterbaume-dynamodb`, `winterbaume-dynamodbstreams`, and `winterbaume-lambda`. |
| DynamoDB Streams → EventBridge Pipes | AWS documents DynamoDB Streams as a Pipes source and also documents EventBridge consumption paths for DynamoDB changes. | Useful for stream fan-out beyond direct Lambda-style consumption. |
| SQS → Lambda | Lambda supports SQS event-source mappings. | Core queue-consumer integration across `winterbaume-sqs` and `winterbaume-lambda`. |
| SNS → Lambda | SNS supports Lambda subscriptions and triggers. | Core pub-sub to compute integration across `winterbaume-sns` and `winterbaume-lambda`. |
| Kinesis → Lambda | Lambda supports Kinesis event-source mappings. | Important stream-consumer contract across `winterbaume-kinesis` and `winterbaume-lambda`. |
| DynamoDB or Kinesis or SQS → EventBridge Pipes | EventBridge Pipes supports DynamoDB Streams, Kinesis streams, and SQS queues as sources. | Suggests reusable Pipes source-adapter behaviour rather than one-off integrations. |
| EventBridge → Lambda or SQS or SNS or Step Functions or Batch or ECS or API Gateway or CloudWatch Logs or AppSync or Kinesis or Firehose | EventBridge documents these target families for rules. | `PutTargets` metadata already exists; target execution semantics are the main missing behaviour. |
| Step Functions ↔ Lambda | Step Functions has an optimised Lambda integration. | One of the highest-value orchestration seams. |
| Step Functions ↔ DynamoDB | Step Functions supports exactly `GetItem`, `PutItem`, `UpdateItem`, and `DeleteItem` as optimised integrations. | Useful for stateful workflow tests, but not a general DynamoDB control-plane or query integration. |
| Step Functions ↔ SQS or SNS or EventBridge | Step Functions documents direct integrations for sending to SQS, publishing to SNS, and putting events to EventBridge. | Gives orchestration coverage across existing messaging crates. |
| Step Functions ↔ API Gateway | Step Functions can invoke API Gateway directly, and API Gateway can invoke Step Functions APIs such as `StartExecution`. | Establishes a bidirectional orchestration or API boundary. |
| Step Functions ↔ ECS or Batch | Step Functions documents optimised ECS and Batch job integrations. | Important for long-running job orchestration. |
| API Gateway ↔ Lambda | API Gateway documents Lambda proxy and custom integrations. | Fundamental API front-door path across `winterbaume-apigateway`, `winterbaume-apigatewayv2`, and `winterbaume-lambda`. |
| API Gateway ↔ Step Functions | AWS documents API Gateway methods invoking Step Functions APIs such as `StartExecution`. | Concrete, common service integration rather than generic HTTP passthrough. |
| Athena ↔ Glue Data Catalogue | Athena's default data catalogue is the Glue Data Catalogue. Most Athena users rely on Glue-managed databases and tables for schema definitions when executing queries. | `winterbaume-athena` accepts `DataCatalogType::Glue` but treats it as metadata only; `StartQueryExecution` does not resolve schemas from `winterbaume-glue` state. The DuckDB backend executes SQL without awareness of Glue metadata. |

### Prioritisation Guidance

The most useful near-term clusters are:

1. AppSync data sources, especially DynamoDB, Lambda, OpenSearch, RDS Data API, HTTP, and EventBridge
2. EventBridge targets and Pipes, especially Lambda, SQS, SNS, Step Functions, and AppSync
3. Lambda-centred trigger chains from DynamoDB Streams, SQS, SNS, and Kinesis
4. Step Functions direct integrations, starting with Lambda and the messaging services before moving to ECS, Batch, and API Gateway
5. Athena ↔ Glue catalogue resolution, so that query execution against Glue-type catalogues can resolve database and table schemas from Glue state

### Workspace-State Implications

The journal review identified several durable implications for the current workspace:

- `winterbaume-appsync` is still mostly control-plane oriented, so AppSync data-source execution is the clearest prerequisite before richer inter-service flows can exist.
- `winterbaume-eventbridge` already stores target metadata through `PutTargets`, which means the main gap is target-side execution semantics rather than missing CRUD APIs.
- `winterbaume-dynamodbstreams` already exists, which makes DynamoDB Streams → Lambda and DynamoDB Streams → EventBridge Pipes realistic near-term targets.
- Step Functions ↔ DynamoDB work should stay scoped to the four supported optimised actions rather than being framed as general DynamoDB support.
- `winterbaume-athena` accepts `DataCatalogType::Glue` at CRUD level but does not resolve Glue metadata at query time. The `AthenaQueryBackend` trait and the DuckDB backend both operate without catalogue awareness. Bridging this requires a catalogue-resolution layer that consults `winterbaume-glue` state when the target catalogue type is `Glue`, and a schema-injection mechanism so the query backend receives table definitions.

## Files

- `crates/winterbaume-appsync/src/handlers.rs` - AppSync control-plane surface and future data-source entry points
- `crates/winterbaume-eventbridge/src/handlers.rs` - EventBridge rule and target metadata handling
- `crates/winterbaume-dynamodbstreams/src/handlers.rs` - stream-consumption surface for downstream integrations
- `crates/winterbaume-lambda/src/handlers.rs` - event-source and invocation behaviour
- `crates/winterbaume-sfn/src/handlers.rs` - direct service-integration entry points
- `crates/winterbaume-athena/src/handlers.rs` - Athena query execution and catalogue management
- `crates/winterbaume-athena/src/backend.rs` - `AthenaQueryBackend` trait and in-memory adapter
- `crates/winterbaume-glue/src/handlers.rs` - Glue catalogue, database, and table metadata
- `crates/winterbaume-sqlengine-duckdb/src/athena.rs` - DuckDB Athena backend (no Glue awareness)
- `README.md` - current per-service implementation summaries that help identify cross-service gaps

## Test Coverage

- No direct cross-service emulator suite was recorded in the journal for these paths yet.
- Future work should prefer behaviour-level integration tests that span at least two services, for example EventBridge target execution, Pipes source consumption, or Step Functions task invocation.
- AppSync and Step Functions integrations should use AWS-documented contract boundaries rather than custom Winterbaume-only shortcuts.

## Pitfalls

- Do not treat "service A can call service B" as enough detail; the AWS-supported action set may be narrow.
- Do not describe Step Functions ↔ DynamoDB as general CRUD. The optimised path is only `GetItem`, `PutItem`, `UpdateItem`, and `DeleteItem`.
- Do not forget the AppSync ↔ EventBridge relationship is bidirectional in AWS docs.
- Do not prioritise inter-service execution before the prerequisite control-plane resources exist, especially for AppSync data sources and resolvers.
- Do not invent mock-only coupling when AWS already documents a precise integration contract.
- Do not assume that Athena's `DataCatalogType::Glue` support means Glue integration is working. In real AWS, this is the most common catalogue type, but winterbaume currently treats it as inert metadata.
