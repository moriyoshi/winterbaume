# AWS Comprehend Medical

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Comprehend Medical extracts structured information from unstructured clinical text. Use these actions to gain insight in your documents. Amazon Comprehend Medical only detects entities in English language texts. Amazon Comprehend Medical places limits on the sizes of files allowed for different API operations. To learn more, see Guidelines and quotas in the Amazon Comprehend Medical Developer Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Comprehend Medical resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Comprehend Medical workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Start`, `Stop`, `Detect` operation families, including `DescribeEntitiesDetectionV2Job`, `DescribeICD10CMInferenceJob`, `DescribePHIDetectionJob`, `DescribeRxNormInferenceJob`, `ListEntitiesDetectionV2Jobs`, `ListICD10CMInferenceJobs`.

## Service Identity and Protocol

- AWS model slug: `comprehendmedical`
- AWS SDK for Rust slug: `comprehendmedical`
- Model version: `2018-10-30`
- Model file: `vendor/api-models-aws/models/comprehendmedical/service/2018-10-30/comprehendmedical-2018-10-30.json`
- SDK ID: `ComprehendMedical`
- Endpoint prefix: `comprehendmedical`
- ARN namespace: `comprehendmedical`
- CloudFormation name: `ComprehendMedical`
- CloudTrail event source: `comprehendmedical.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (5), `List` (5), `Start` (5), `Stop` (5), `Detect` (3), `Infer` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `StartEntitiesDetectionV2Job`, `StartICD10CMInferenceJob`, `StartPHIDetectionJob`, `StartRxNormInferenceJob`, `StartSNOMEDCTInferenceJob`, `StopEntitiesDetectionV2Job`, `StopICD10CMInferenceJob`, `StopPHIDetectionJob`, `StopRxNormInferenceJob`, `StopSNOMEDCTInferenceJob`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeEntitiesDetectionV2Job`, `DescribeICD10CMInferenceJob`, `DescribePHIDetectionJob`, `DescribeRxNormInferenceJob`, `DescribeSNOMEDCTInferenceJob`, `ListEntitiesDetectionV2Jobs`, `ListICD10CMInferenceJobs`, `ListPHIDetectionJobs`, `ListRxNormInferenceJobs`, `ListSNOMEDCTInferenceJobs`.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeEntitiesDetectionV2Job`, `DescribeICD10CMInferenceJob`, `DescribePHIDetectionJob`, `DescribeRxNormInferenceJob`, `DescribeSNOMEDCTInferenceJob`, `ListEntitiesDetectionV2Jobs`, `ListICD10CMInferenceJobs`, `ListPHIDetectionJobs`, `ListRxNormInferenceJobs`, `ListSNOMEDCTInferenceJobs`, `StartEntitiesDetectionV2Job`, `StartICD10CMInferenceJob`, `StartPHIDetectionJob`, `StartRxNormInferenceJob`, `StartSNOMEDCTInferenceJob`, `StopEntitiesDetectionV2Job`, `StopICD10CMInferenceJob`, `StopPHIDetectionJob`, `StopRxNormInferenceJob`, `StopSNOMEDCTInferenceJob`.
- 26 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/comprehend-medical/latest/dev/comprehendmedical-welcome.html
- https://docs.aws.amazon.com/comprehend-medical/latest/dev/ontology-RxNorm.html
- https://docs.aws.amazon.com/comprehend-medical/latest/api/API_StartRxNormInferenceJob.html

Research outcomes:
- Comprehend Medical detects medical entities and protected health information in unstructured clinical text.
- It can link detected medical concepts to ontologies such as ICD-10-CM, RxNorm, and SNOMED CT.
- RxNorm linking detects medication entities and maps them to RxCUI identifiers with attributes and traits.
- Some operations are synchronous for direct text analysis, while inference jobs process S3 input asynchronously.
- Asynchronous jobs are tracked with describe operations and write output to configured S3 locations.
- The service is HIPAA eligible but requires customers to manage compliance responsibilities.

Parity implications:
- Model synchronous medical NLP operations, ontology inference jobs, job status, input/output S3 locations, entity scores, ontology codes, and PHI detection separately.
- Async inference should require polling and should not return full results from start operations.
- Medical ontology linking should preserve concept ids, scores, attributes, and traits distinctly from generic entity detection.

## Operation Groups

### Describe

- Operations: `DescribeEntitiesDetectionV2Job`, `DescribeICD10CMInferenceJob`, `DescribePHIDetectionJob`, `DescribeRxNormInferenceJob`, `DescribeSNOMEDCTInferenceJob`
- Common required input members in this group: `JobId`

### List

- Operations: `ListEntitiesDetectionV2Jobs`, `ListICD10CMInferenceJobs`, `ListPHIDetectionJobs`, `ListRxNormInferenceJobs`, `ListSNOMEDCTInferenceJobs`

### Start

- Operations: `StartEntitiesDetectionV2Job`, `StartICD10CMInferenceJob`, `StartPHIDetectionJob`, `StartRxNormInferenceJob`, `StartSNOMEDCTInferenceJob`
- Traits: `idempotency-token` (5)
- Common required input members in this group: `DataAccessRoleArn`, `InputDataConfig`, `LanguageCode`, `OutputDataConfig`

### Stop

- Operations: `StopEntitiesDetectionV2Job`, `StopICD10CMInferenceJob`, `StopPHIDetectionJob`, `StopRxNormInferenceJob`, `StopSNOMEDCTInferenceJob`
- Common required input members in this group: `JobId`

### Detect

- Operations: `DetectEntities`, `DetectEntitiesV2`, `DetectPHI`
- Common required input members in this group: `Text`

### Infer

- Operations: `InferICD10CM`, `InferRxNorm`, `InferSNOMEDCT`
- Common required input members in this group: `Text`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DescribeEntitiesDetectionV2Job` | - | - | `JobId` | - | `DescribeEntitiesDetectionV2JobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a medical entities detection job. Use this operation to get the status of a detection job. |
| `DescribeICD10CMInferenceJob` | - | - | `JobId` | - | `DescribeICD10CMInferenceJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the properties associated with an InferICD10CM job. Use this operation to get the status of an inference job. |
| `DescribePHIDetectionJob` | - | - | `JobId` | - | `DescribePHIDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the properties associated with a protected health information (PHI) detection job. Use this operation to get the status of a detection job. |
| `DescribeRxNormInferenceJob` | - | - | `JobId` | - | `DescribeRxNormInferenceJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the properties associated with an InferRxNorm job. Use this operation to get the status of an inference job. |
| `DescribeSNOMEDCTInferenceJob` | - | - | `JobId` | - | `DescribeSNOMEDCTInferenceJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the properties associated with an InferSNOMEDCT job. Use this operation to get the status of an inference job. |
| `DetectEntities` | - | - | `Text` | - | `DetectEntitiesResponse` | `InternalServerException`, `InvalidEncodingException`, `InvalidRequestException`, `ServiceUnavailableException`, `TextSizeLimitExceededException`, `TooManyRequestsException` | The `DetectEntities` operation is deprecated. You should use the DetectEntitiesV2 operation instead. |
| `DetectEntitiesV2` | - | - | `Text` | - | `DetectEntitiesV2Response` | `InternalServerException`, `InvalidEncodingException`, `InvalidRequestException`, `ServiceUnavailableException`, `TextSizeLimitExceededException`, `TooManyRequestsException` | Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information. Amazon Comprehend Medical only detects medical entities in English language... |
| `DetectPHI` | - | - | `Text` | - | `DetectPHIResponse` | `InternalServerException`, `InvalidEncodingException`, `InvalidRequestException`, `ServiceUnavailableException`, `TextSizeLimitExceededException`, `TooManyRequestsException` | Inspects the clinical text for protected health information (PHI) entities and returns the entity category, location, and confidence score for each entity. Amazon Comprehend Medical only detects entities in English language texts. |
| `InferICD10CM` | - | - | `Text` | - | `InferICD10CMResponse` | `InternalServerException`, `InvalidEncodingException`, `InvalidRequestException`, `ServiceUnavailableException`, `TextSizeLimitExceededException`, `TooManyRequestsException` | InferICD10CM detects medical conditions as entities listed in a patient record and links those entities to normalized concept identifiers in the ICD-10-CM knowledge base from the Centers for Disease Control. Amazon Comprehend Medical only detects medical... |
| `InferRxNorm` | - | - | `Text` | - | `InferRxNormResponse` | `InternalServerException`, `InvalidEncodingException`, `InvalidRequestException`, `ServiceUnavailableException`, `TextSizeLimitExceededException`, `TooManyRequestsException` | InferRxNorm detects medications as entities listed in a patient record and links to the normalized concept identifiers in the RxNorm database from the National Library of Medicine. Amazon Comprehend Medical only detects medical entities in English language... |
| `InferSNOMEDCT` | - | - | `Text` | - | `InferSNOMEDCTResponse` | `InternalServerException`, `InvalidEncodingException`, `InvalidRequestException`, `ServiceUnavailableException`, `TextSizeLimitExceededException`, `TooManyRequestsException` | InferSNOMEDCT detects possible medical concepts as entities and links them to codes from the Systematized Nomenclature of Medicine, Clinical Terms (SNOMED-CT) ontology |
| `ListEntitiesDetectionV2Jobs` | - | - | - | - | `ListEntitiesDetectionV2JobsResponse` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException`, `ValidationException` | Gets a list of medical entity detection jobs that you have submitted. |
| `ListICD10CMInferenceJobs` | - | - | - | - | `ListICD10CMInferenceJobsResponse` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException`, `ValidationException` | Gets a list of InferICD10CM jobs that you have submitted. |
| `ListPHIDetectionJobs` | - | - | - | - | `ListPHIDetectionJobsResponse` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException`, `ValidationException` | Gets a list of protected health information (PHI) detection jobs you have submitted. |
| `ListRxNormInferenceJobs` | - | - | - | - | `ListRxNormInferenceJobsResponse` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException`, `ValidationException` | Gets a list of InferRxNorm jobs that you have submitted. |
| `ListSNOMEDCTInferenceJobs` | - | - | - | - | `ListSNOMEDCTInferenceJobsResponse` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException`, `ValidationException` | Gets a list of InferSNOMEDCT jobs a user has submitted. |
| `StartEntitiesDetectionV2Job` | - | `idempotency-token` | `DataAccessRoleArn`, `InputDataConfig`, `LanguageCode`, `OutputDataConfig` | `ClientRequestToken` | `StartEntitiesDetectionV2JobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Starts an asynchronous medical entity detection job for a collection of documents. Use the `DescribeEntitiesDetectionV2Job` operation to track the status of a job. |
| `StartICD10CMInferenceJob` | - | `idempotency-token` | `DataAccessRoleArn`, `InputDataConfig`, `LanguageCode`, `OutputDataConfig` | `ClientRequestToken` | `StartICD10CMInferenceJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Starts an asynchronous job to detect medical conditions and link them to the ICD-10-CM ontology. Use the `DescribeICD10CMInferenceJob` operation to track the status of a job. |
| `StartPHIDetectionJob` | - | `idempotency-token` | `DataAccessRoleArn`, `InputDataConfig`, `LanguageCode`, `OutputDataConfig` | `ClientRequestToken` | `StartPHIDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Starts an asynchronous job to detect protected health information (PHI). Use the `DescribePHIDetectionJob` operation to track the status of a job. |
| `StartRxNormInferenceJob` | - | `idempotency-token` | `DataAccessRoleArn`, `InputDataConfig`, `LanguageCode`, `OutputDataConfig` | `ClientRequestToken` | `StartRxNormInferenceJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Starts an asynchronous job to detect medication entities and link them to the RxNorm ontology. Use the `DescribeRxNormInferenceJob` operation to track the status of a job. |
| `StartSNOMEDCTInferenceJob` | - | `idempotency-token` | `DataAccessRoleArn`, `InputDataConfig`, `LanguageCode`, `OutputDataConfig` | `ClientRequestToken` | `StartSNOMEDCTInferenceJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Starts an asynchronous job to detect medical concepts and link them to the SNOMED-CT ontology. Use the DescribeSNOMEDCTInferenceJob operation to track the status of a job. |
| `StopEntitiesDetectionV2Job` | - | - | `JobId` | - | `StopEntitiesDetectionV2JobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Stops a medical entities detection job in progress. |
| `StopICD10CMInferenceJob` | - | - | `JobId` | - | `StopICD10CMInferenceJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Stops an InferICD10CM inference job in progress. |
| `StopPHIDetectionJob` | - | - | `JobId` | - | `StopPHIDetectionJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Stops a protected health information (PHI) detection job in progress. |
| `StopRxNormInferenceJob` | - | - | `JobId` | - | `StopRxNormInferenceJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Stops an InferRxNorm inference job in progress. |
| `StopSNOMEDCTInferenceJob` | - | - | `JobId` | - | `StopSNOMEDCTInferenceJobResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException` | Stops an InferSNOMEDCT inference job in progress. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | An internal server error occurred. |
| `InvalidRequestException` | `structure` | `Message` | The request that you made is invalid. |
| `TooManyRequestsException` | `structure` | `Message` | You have made too many requests within a short period of time. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource identified by the specified Amazon Resource Name (ARN) was not found. |
| `InvalidEncodingException` | `structure` | `Message` | The input text was not in valid UTF-8 character encoding. |
| `ServiceUnavailableException` | `structure` | `Message` | The Amazon Comprehend Medical service is temporarily unavailable. |
| `TextSizeLimitExceededException` | `structure` | `Message` | The size of the text you submitted exceeds the size limit. |
| `ValidationException` | `structure` | `Message` | The filter that you specified for the operation is invalid. |
| `DescribeEntitiesDetectionV2JobRequest` | `structure` | `JobId` | - |
| `DescribeEntitiesDetectionV2JobResponse` | `structure` | `ComprehendMedicalAsyncJobProperties` | - |
| `DescribeICD10CMInferenceJobRequest` | `structure` | `JobId` | - |
| `DescribeICD10CMInferenceJobResponse` | `structure` | `ComprehendMedicalAsyncJobProperties` | - |
| `DescribePHIDetectionJobRequest` | `structure` | `JobId` | - |
| `DescribePHIDetectionJobResponse` | `structure` | `ComprehendMedicalAsyncJobProperties` | - |
| `DescribeRxNormInferenceJobRequest` | `structure` | `JobId` | - |
| `DescribeRxNormInferenceJobResponse` | `structure` | `ComprehendMedicalAsyncJobProperties` | - |
| `DescribeSNOMEDCTInferenceJobRequest` | `structure` | `JobId` | - |
| `DescribeSNOMEDCTInferenceJobResponse` | `structure` | `ComprehendMedicalAsyncJobProperties` | - |
| `DetectEntitiesRequest` | `structure` | `Text` | - |
| `DetectEntitiesResponse` | `structure` | `Entities`, `ModelVersion`, `PaginationToken`, `UnmappedAttributes` | - |
| `DetectEntitiesV2Request` | `structure` | `Text` | - |
| `DetectEntitiesV2Response` | `structure` | `Entities`, `ModelVersion`, `PaginationToken`, `UnmappedAttributes` | - |
| `DetectPHIRequest` | `structure` | `Text` | - |
| `DetectPHIResponse` | `structure` | `Entities`, `ModelVersion`, `PaginationToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
