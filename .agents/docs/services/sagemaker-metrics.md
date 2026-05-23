# Amazon SageMaker Metrics Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Contains all data plane API operations and data types for Amazon SageMaker Metrics. Use these APIs to put and retrieve (get) features related to your training run. BatchPutMetrics

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon SageMaker Metrics Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model workflows exposed by the modelled operation families across the `Batch` operation families, including `BatchGetMetrics`, `BatchPutMetrics`.

## Service Identity and Protocol

- AWS model slug: `sagemaker-metrics`
- AWS SDK for Rust slug: `sagemakermetrics`
- Model version: `2022-09-30`
- Model file: `vendor/api-models-aws/models/sagemaker-metrics/service/2022-09-30/sagemaker-metrics-2022-09-30.json`
- SDK ID: `SageMaker Metrics`
- Endpoint prefix: `metrics.sagemaker`
- ARN namespace: `sagemaker`
- CloudFormation name: `SageMakerMetrics`
- CloudTrail event source: `sagemakermetrics.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Batch` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetMetrics`, `BatchPutMetrics`.
- Documentation and model terms indicate cross-service dependencies or identifiers: `CloudWatch`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Control-Plane / Data-Plane Coherence

- **Paired with `sagemaker` ( same SDK slug `sagemaker` ).** `BatchPutMetrics` writes telemetry against trial components owned by the SageMaker control plane ( `winterbaume-sagemaker` ). Unlike `sagemakerruntime`, this is **fire-and-forget telemetry** — real AWS does not validate the trial component name on `BatchPutMetrics`, and `BatchGetMetrics` returns whatever was written.
- **Current Winterbaume status: correctly separate.** This crate stores metrics keyed by trial component name without depending on `winterbaume-sagemaker`. Telemetry semantics do not require cross-crate state, so this is benign.
- **No change needed.** If a future `winterbaume-sagemaker` exposes metric data via training-job / trial-component reads, that would be the right place to fold this in — but the current AWS API does not, and neither should Winterbaume.

## Operation Groups

### Batch

- Operations: `BatchGetMetrics`, `BatchPutMetrics`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetMetrics` | `POST /BatchGetMetrics` | - | `MetricQueries` | - | `BatchGetMetricsResponse` | - | Used to retrieve training metrics from SageMaker. |
| `BatchPutMetrics` | `PUT /BatchPutMetrics` | - | `TrialComponentName`, `MetricData` | - | `BatchPutMetricsResponse` | - | Used to ingest training metrics into SageMaker. These metrics can be visualized in SageMaker Studio. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BatchGetMetricsRequest` | `structure` | MetricQueries | - |
| `BatchGetMetricsResponse` | `structure` | MetricQueryResults | - |
| `BatchPutMetricsRequest` | `structure` | TrialComponentName, MetricData | - |
| `BatchPutMetricsResponse` | `structure` | Errors | - |
| `MetricQueryResultStatus` | `enum` | COMPLETE, TRUNCATED, INTERNAL_ERROR, VALIDATION_ERROR | - |
| `MetricStatistic` | `enum` | MIN, MAX, AVG, COUNT, STD_DEV, LAST | - |
| `Period` | `enum` | ONE_MINUTE, FIVE_MINUTE, ONE_HOUR, ITERATION_NUMBER | - |
| `PutMetricsErrorCode` | `enum` | METRIC_LIMIT_EXCEEDED, INTERNAL_ERROR, VALIDATION_ERROR, CONFLICT_ERROR | - |
| `XAxisType` | `enum` | ITERATION_NUMBER, TIMESTAMP | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
