# Amazon Connect Contact Lens

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Contact Lens actions Contact Lens data types Amazon Connect Contact Lens enables you to analyze conversations between customer and agents, by using speech transcription, natural language processing, and intelligent search capabilities. It performs sentiment analysis, detects issues, and enables you to automatically categorize contacts. Amazon Connect Contact Lens provides both real-time and post-call analytics of customer-agent conversations. For more information, see Analyze conversations using speech analytics in the Amazon Connect Administrator Guide .

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Connect Contact Lens workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List` operation families, including `ListRealtimeContactAnalysisSegments`.

## Service Identity and Protocol

- AWS model slug: `connect-contact-lens`
- AWS SDK for Rust slug: `connectcontactlens`
- Model version: `2020-08-21`
- Model file: `vendor/api-models-aws/models/connect-contact-lens/service/2020-08-21/connect-contact-lens-2020-08-21.json`
- SDK ID: `Connect Contact Lens`
- Endpoint prefix: `contact-lens`
- ARN namespace: `connect`
- CloudFormation name: `ConnectContactLens`
- CloudTrail event source: `connectcontactlens.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `ListRealtimeContactAnalysisSegments`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ListRealtimeContactAnalysisSegments`.
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/connect/latest/adminguide/analyze-conversations.html
- https://docs.aws.amazon.com/connect/latest/adminguide/contact-search.html
- https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html

Research outcomes:
- Contact Lens analyses Amazon Connect voice, chat, and email conversations for sentiment, transcripts, categories, agent compliance, and trends.
- Analytics can apply to completed contacts and, where enabled, in-progress contacts for real-time use cases.
- Contact Lens can redact personally identifiable information and generate analytics artefacts that are searchable from contact search.
- Contact search can filter completed and in-progress contacts and expose analytics-dependent fields when Contact Lens is enabled.
- Rules and categories identify conversational patterns and can drive compliance or quality monitoring.
- Service quotas apply to Contact Lens jobs and concurrent analytics operations.

Parity implications:
- Model analytics jobs, transcripts, sentiment, categories, rules, redaction configuration, contact search index state, and quota counters separately.
- Analytics should be tied to Connect contact records and channel type.
- Real-time and post-contact analytics should expose different lifecycle and availability semantics.

## Operation Groups

### List

- Operations: `ListRealtimeContactAnalysisSegments`
- Traits: `paginated` (1)
- Common required input members in this group: `ContactId`, `InstanceId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListRealtimeContactAnalysisSegments` | `POST /realtime-contact-analysis/analysis-segments` | `paginated` | `ContactId`, `InstanceId` | - | `ListRealtimeContactAnalysisSegmentsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Provides a list of analysis segments for a real-time analysis session. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ListRealtimeContactAnalysisSegmentsRequest` | `structure` | `ContactId`, `InstanceId`, `MaxResults`, `NextToken` | - |
| `ListRealtimeContactAnalysisSegmentsResponse` | `structure` | `NextToken`, `Segments` | - |
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServiceException` | `structure` | `Message` | Request processing failed due to an error or failure with the service. |
| `InvalidRequestException` | `structure` | `Message` | The request is not valid. |
| `ResourceNotFoundException` | `structure` | `Message` | The specified resource was not found. |
| `ThrottlingException` | `structure` | `Message` | The throttling limit has been exceeded. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
