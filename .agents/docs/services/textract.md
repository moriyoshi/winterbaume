# Amazon Textract

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Textract detects and analyzes text in documents and converts it into machine-readable text. This is the API reference documentation for Amazon Textract.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Textract workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Start`, `Analyze`, `List`, `Create` operation families, including `GetAdapter`, `GetAdapterVersion`, `GetDocumentAnalysis`, `GetDocumentTextDetection`, `StartDocumentAnalysis`, `StartDocumentTextDetection`.

## Service Identity and Protocol

- AWS model slug: `textract`
- AWS SDK for Rust slug: `textract`
- Model version: `2018-06-27`
- Model file: `vendor/api-models-aws/models/textract/service/2018-06-27/textract-2018-06-27.json`
- SDK ID: `Textract`
- Endpoint prefix: `textract`
- ARN namespace: `textract`
- CloudFormation name: `Textract`
- CloudTrail event source: `textract.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (7), `Start` (4), `Analyze` (3), `List` (3), `Create` (2), `Delete` (2), `Detect` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAdapter`, `CreateAdapterVersion`, `DeleteAdapter`, `DeleteAdapterVersion`, `StartDocumentAnalysis`, `StartDocumentTextDetection`, `StartExpenseAnalysis`, `StartLendingAnalysis`, `TagResource`, `UntagResource`, `UpdateAdapter`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAdapter`, `GetAdapterVersion`, `GetDocumentAnalysis`, `GetDocumentTextDetection`, `GetExpenseAnalysis`, `GetLendingAnalysis`, `GetLendingAnalysisSummary`, `ListAdapterVersions`, `ListAdapters`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetDocumentAnalysis`, `GetExpenseAnalysis`, `GetLendingAnalysis`, `GetLendingAnalysisSummary`, `StartDocumentAnalysis`, `StartDocumentTextDetection`, `StartExpenseAnalysis`, `StartLendingAnalysis`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 25 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetAdapter`, `GetAdapterVersion`, `GetDocumentAnalysis`, `GetDocumentTextDetection`, `GetExpenseAnalysis`, `GetLendingAnalysis`, `GetLendingAnalysisSummary`
- Common required input members in this group: `AdapterId`, `JobId`

### Start

- Operations: `StartDocumentAnalysis`, `StartDocumentTextDetection`, `StartExpenseAnalysis`, `StartLendingAnalysis`
- Common required input members in this group: `DocumentLocation`

### Analyze

- Operations: `AnalyzeDocument`, `AnalyzeExpense`, `AnalyzeID`
- Common required input members in this group: `Document`

### List

- Operations: `ListAdapters`, `ListAdapterVersions`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Create

- Operations: `CreateAdapter`, `CreateAdapterVersion`
- Traits: `idempotent` (2), `idempotency-token` (2)
- Common required input members in this group: -

### Delete

- Operations: `DeleteAdapter`, `DeleteAdapterVersion`
- Traits: `idempotent` (2)
- Common required input members in this group: `AdapterId`

### Detect

- Operations: `DetectDocumentText`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateAdapter`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AnalyzeDocument` | `-` | - | `Document`, `FeatureTypes` | - | `AnalyzeDocumentResponse` | `AccessDeniedException`, `BadDocumentException`, `DocumentTooLargeException`, `HumanLoopQuotaExceededException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `UnsupportedDocumentException` | Analyzes an input document for relationships between detected items. The types of information returned are as follows: Form data (key-value pairs). The related information is returned in two Block objects, each of ty ... |
| `AnalyzeExpense` | `-` | - | `Document` | - | `AnalyzeExpenseResponse` | `AccessDeniedException`, `BadDocumentException`, `DocumentTooLargeException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `UnsupportedDocumentException` | AnalyzeExpense synchronously analyzes an input document for financially related relationships between text. Information is returned as ExpenseDocuments and seperated as follows: LineItemGroups - A data set containing ... |
| `AnalyzeID` | `-` | - | `DocumentPages` | - | `AnalyzeIDResponse` | `AccessDeniedException`, `BadDocumentException`, `DocumentTooLargeException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `UnsupportedDocumentException` | Analyzes identity documents for relevant information. This information is extracted and returned as IdentityDocumentFields , which records both the normalized field and value of the extracted text. Unlike other Amazo ... |
| `CreateAdapter` | `-` | `idempotent`, `idempotency-token` | `AdapterName`, `FeatureTypes` | `ClientRequestToken` | `CreateAdapterResponse` | `AccessDeniedException`, `ConflictException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an adapter, which can be fine-tuned for enhanced performance on user provided documents. Takes an AdapterName and FeatureType. Currently the only supported feature type is QUERIES . You can also provide a Des ... |
| `CreateAdapterVersion` | `-` | `idempotent`, `idempotency-token` | `AdapterId`, `DatasetConfig`, `OutputConfig` | `ClientRequestToken` | `CreateAdapterVersionResponse` | `AccessDeniedException`, `ConflictException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new version of an adapter. Operates on a provided AdapterId and a specified dataset provided via the DatasetConfig argument. Requires that you specify an Amazon S3 bucket with the OutputConfig argument. You ... |
| `DeleteAdapter` | `-` | `idempotent` | `AdapterId` | - | `DeleteAdapterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Textract adapter. Takes an AdapterId and deletes the adapter specified by the ID. |
| `DeleteAdapterVersion` | `-` | `idempotent` | `AdapterId`, `AdapterVersion` | - | `DeleteAdapterVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Textract adapter version. Requires that you specify both an AdapterId and a AdapterVersion. Deletes the adapter version specified by the AdapterId and the AdapterVersion. |
| `DetectDocumentText` | `-` | - | `Document` | - | `DetectDocumentTextResponse` | `AccessDeniedException`, `BadDocumentException`, `DocumentTooLargeException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `UnsupportedDocumentException` | Detects text in the input document. Amazon Textract can detect lines of text and the words that make up a line of text. The input document must be in one of the following image formats: JPEG, PNG, PDF, or TIFF. Detec ... |
| `GetAdapter` | `-` | - | `AdapterId` | - | `GetAdapterResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets configuration information for an adapter specified by an AdapterId, returning information on AdapterName, Description, CreationTime, AutoUpdate status, and FeatureTypes. |
| `GetAdapterVersion` | `-` | - | `AdapterId`, `AdapterVersion` | - | `GetAdapterVersionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets configuration information for the specified adapter version, including: AdapterId, AdapterVersion, FeatureTypes, Status, StatusMessage, DatasetConfig, KMSKeyId, OutputConfig, Tags and EvaluationMetrics. |
| `GetDocumentAnalysis` | `-` | - | `JobId` | - | `GetDocumentAnalysisResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidJobIdException`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Gets the results for an Amazon Textract asynchronous operation that analyzes text in a document. You start asynchronous text analysis by calling StartDocumentAnalysis , which returns a job identifier ( JobId ). When ... |
| `GetDocumentTextDetection` | `-` | - | `JobId` | - | `GetDocumentTextDetectionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidJobIdException`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Gets the results for an Amazon Textract asynchronous operation that detects text in a document. Amazon Textract can detect lines of text and the words that make up a line of text. You start asynchronous text detectio ... |
| `GetExpenseAnalysis` | `-` | - | `JobId` | - | `GetExpenseAnalysisResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidJobIdException`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Gets the results for an Amazon Textract asynchronous operation that analyzes invoices and receipts. Amazon Textract finds contact information, items purchased, and vendor name, from input invoices and receipts. You s ... |
| `GetLendingAnalysis` | `-` | - | `JobId` | - | `GetLendingAnalysisResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidJobIdException`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Gets the results for an Amazon Textract asynchronous operation that analyzes text in a lending document. You start asynchronous text analysis by calling StartLendingAnalysis , which returns a job identifier ( JobId ) ... |
| `GetLendingAnalysisSummary` | `-` | - | `JobId` | - | `GetLendingAnalysisSummaryResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidJobIdException`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Gets summarized results for the StartLendingAnalysis operation, which analyzes text in a lending document. The returned summary consists of information about documents grouped together by a common document type. Info ... |
| `ListAdapters` | `-` | `paginated` | - | - | `ListAdaptersResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `ValidationException` | Lists all adapters that match the specified filtration criteria. |
| `ListAdapterVersions` | `-` | `paginated` | - | - | `ListAdapterVersionsResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all version of an adapter that meet the specified filtration criteria. |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags for an Amazon Textract resource. |
| `StartDocumentAnalysis` | `-` | - | `DocumentLocation`, `FeatureTypes` | - | `StartDocumentAnalysisResponse` | `AccessDeniedException`, `BadDocumentException`, `DocumentTooLargeException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `UnsupportedDocumentException` | Starts the asynchronous analysis of an input document for relationships between detected items such as key-value pairs, tables, and selection elements. StartDocumentAnalysis can analyze text in documents that are in ... |
| `StartDocumentTextDetection` | `-` | - | `DocumentLocation` | - | `StartDocumentTextDetectionResponse` | `AccessDeniedException`, `BadDocumentException`, `DocumentTooLargeException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `UnsupportedDocumentException` | Starts the asynchronous detection of text in a document. Amazon Textract can detect lines of text and the words that make up a line of text. StartDocumentTextDetection can analyze text in documents that are in JPEG, ... |
| `StartExpenseAnalysis` | `-` | - | `DocumentLocation` | - | `StartExpenseAnalysisResponse` | `AccessDeniedException`, `BadDocumentException`, `DocumentTooLargeException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `UnsupportedDocumentException` | Starts the asynchronous analysis of invoices or receipts for data like contact information, items purchased, and vendor names. StartExpenseAnalysis can analyze text in documents that are in JPEG, PNG, and PDF format. ... |
| `StartLendingAnalysis` | `-` | - | `DocumentLocation` | - | `StartLendingAnalysisResponse` | `AccessDeniedException`, `BadDocumentException`, `DocumentTooLargeException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidKMSKeyException`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `UnsupportedDocumentException` | Starts the classification and analysis of an input document. StartLendingAnalysis initiates the classification and analysis of a packet of lending documents. StartLendingAnalysis operates on a document file located i ... |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds one or more tags to the specified resource. |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes any tags with the specified keys from the specified resource. |
| `UpdateAdapter` | `-` | - | `AdapterId` | - | `UpdateAdapterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the configuration for an adapter. FeatureTypes configurations cannot be updated. At least one new parameter must be specified as an argument. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message, Code | You aren't authorized to perform the action. Use the Amazon Resource Name (ARN) of an authorized user or IAM role to perform the operation. |
| `BadDocumentException` | `structure` | Message, Code | Amazon Textract isn't able to read the document. For more information on the document limits in Amazon Textract, see limits . |
| `ConflictException` | `structure` | Message, Code | Updating or deleting a resource can cause an inconsistent state. |
| `DocumentTooLargeException` | `structure` | Message, Code | The document can't be processed because it's too large. The maximum document size for synchronous operations 10 MB. The maximum document size for asynchrono ... |
| `HumanLoopQuotaExceededException` | `structure` | ResourceType, QuotaCode, ServiceCode, Message, Code | Indicates you have exceeded the maximum number of active human in the loop workflows available |
| `IdempotentParameterMismatchException` | `structure` | Message, Code | A ClientRequestToken input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the ... |
| `InternalServerError` | `structure` | Message, Code | Amazon Textract experienced a service issue. Try your call again. |
| `InvalidJobIdException` | `structure` | Message, Code | An invalid job identifier was passed to an asynchronous analysis operation. |
| `InvalidKMSKeyException` | `structure` | Message, Code | Indicates you do not have decrypt permissions with the KMS key entered, or the KMS key was entered incorrectly. |
| `InvalidParameterException` | `structure` | Message, Code | An input parameter violated a constraint. For example, in synchronous operations, an InvalidParameterException exception occurs when neither of the S3Object ... |
| `InvalidS3ObjectException` | `structure` | Message, Code | Amazon Textract is unable to access the S3 object that's specified in the request. for more information, Configure Access to Amazon S3 For troubleshooting i ... |
| `LimitExceededException` | `structure` | Message, Code | An Amazon Textract service limit was exceeded. For example, if you start too many asynchronous jobs concurrently, calls to start operations ( StartDocumentT ... |
| `ProvisionedThroughputExceededException` | `structure` | Message, Code | The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Textract. |
| `ResourceNotFoundException` | `structure` | Message, Code | Returned when an operation tried to access a nonexistent resource. |
| `ServiceQuotaExceededException` | `structure` | Message, Code | Returned when a request cannot be completed as it would exceed a maximum service quota. |
| `ThrottlingException` | `structure` | Message, Code | Amazon Textract is temporarily unable to process the request. Try your call again. |
| `UnsupportedDocumentException` | `structure` | Message, Code | The format of the input document isn't supported. Documents for operations can be in PNG, JPEG, PDF, or TIFF format. |
| `ValidationException` | `structure` | Message, Code | Indicates that a request was not valid. Check request for proper formatting. |
| `AnalyzeDocumentRequest` | `structure` | Document, FeatureTypes, HumanLoopConfig, QueriesConfig, AdaptersConfig | - |
| `AnalyzeDocumentResponse` | `structure` | DocumentMetadata, Blocks, HumanLoopActivationOutput, AnalyzeDocumentModelVersion | - |
| `AnalyzeExpenseRequest` | `structure` | Document | - |
| `AnalyzeExpenseResponse` | `structure` | DocumentMetadata, ExpenseDocuments | - |
| `AnalyzeIDRequest` | `structure` | DocumentPages | - |
| `AnalyzeIDResponse` | `structure` | IdentityDocuments, DocumentMetadata, AnalyzeIDModelVersion | - |
| `CreateAdapterRequest` | `structure` | AdapterName, ClientRequestToken, Description, FeatureTypes, AutoUpdate, Tags | - |
| `CreateAdapterResponse` | `structure` | AdapterId | - |
| `CreateAdapterVersionRequest` | `structure` | AdapterId, ClientRequestToken, DatasetConfig, KMSKeyId, OutputConfig, Tags | - |
| `CreateAdapterVersionResponse` | `structure` | AdapterId, AdapterVersion | - |
| `DeleteAdapterRequest` | `structure` | AdapterId | - |
| `DeleteAdapterResponse` | `structure` | **empty (no members)** | - |
| `DeleteAdapterVersionRequest` | `structure` | AdapterId, AdapterVersion | - |
| `DeleteAdapterVersionResponse` | `structure` | **empty (no members)** | - |
| `DetectDocumentTextRequest` | `structure` | Document | - |
| `DetectDocumentTextResponse` | `structure` | DocumentMetadata, Blocks, DetectDocumentTextModelVersion | - |
| `GetAdapterRequest` | `structure` | AdapterId | - |
| `GetAdapterResponse` | `structure` | AdapterId, AdapterName, CreationTime, Description, FeatureTypes, AutoUpdate, Tags | - |
| `GetAdapterVersionRequest` | `structure` | AdapterId, AdapterVersion | - |
| `GetAdapterVersionResponse` | `structure` | AdapterId, AdapterVersion, CreationTime, FeatureTypes, Status, StatusMessage, DatasetConfig, KMSKeyId, OutputConfig, EvaluationMetrics, Tags | - |
| `GetDocumentAnalysisRequest` | `structure` | JobId, MaxResults, NextToken | - |
| `GetDocumentAnalysisResponse` | `structure` | DocumentMetadata, JobStatus, NextToken, Blocks, Warnings, StatusMessage, AnalyzeDocumentModelVersion | - |
| `AdapterVersionStatus` | `enum` | ACTIVE, AT_RISK, DEPRECATED, CREATION_ERROR, CREATION_IN_PROGRESS | - |
| `AutoUpdate` | `enum` | ENABLED, DISABLED | - |
| `BlockType` | `enum` | KEY_VALUE_SET, PAGE, LINE, WORD, TABLE, CELL, SELECTION_ELEMENT, MERGED_CELL, TITLE, QUERY, QUERY_RESULT, SIGNATURE, ... (+12) | - |
| `ContentClassifier` | `enum` | FREE_OF_PERSONALLY_IDENTIFIABLE_INFORMATION, FREE_OF_ADULT_CONTENT | - |
| `EntityType` | `enum` | KEY, VALUE, COLUMN_HEADER, TABLE_TITLE, TABLE_FOOTER, TABLE_SECTION_TITLE, TABLE_SUMMARY, STRUCTURED_TABLE, SEMI_STRUCTURED_TABLE | - |
| `FeatureType` | `enum` | TABLES, FORMS, QUERIES, SIGNATURES, LAYOUT | - |
| `JobStatus` | `enum` | IN_PROGRESS, SUCCEEDED, FAILED, PARTIAL_SUCCESS | - |
| `RelationshipType` | `enum` | VALUE, CHILD, COMPLEX_FEATURES, MERGED_CELL, TITLE, ANSWER, TABLE, TABLE_TITLE, TABLE_FOOTER | - |
| `SelectionStatus` | `enum` | SELECTED, NOT_SELECTED | - |
| `TextType` | `enum` | HANDWRITING, PRINTED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
