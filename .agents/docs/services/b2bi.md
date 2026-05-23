# AWS B2B Data Interchange

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the Amazon Web Services B2B Data Interchange API Reference . It provides descriptions, API request parameters, and the XML response for each of the B2BI API actions. B2BI enables automated exchange of EDI (electronic data interchange) based business-critical transactions at cloud scale, with elasticity and pay-as-you-go pricing. Businesses use EDI documents to exchange transactional data with trading partners, such as suppliers and end customers, using standardized formats such as X12. Rather than actually running a command, you can use the `--generate-cli-skeleton` parameter with any API call to generate and display a parameter template.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS B2B Data Interchange resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: create B2B profiles, partnerships, transformers, capabilities, and trading-partner agreements for EDI exchange.
- From the operation surface: model inbound and outbound EDI processing, transformation jobs, test conversions, tagging, and partner lifecycle administration.

## Service Identity and Protocol

- AWS model slug: `b2bi`
- AWS SDK for Rust slug: `b2bi`
- Model version: `2022-06-23`
- Model file: `vendor/api-models-aws/models/b2bi/service/2022-06-23/b2bi-2022-06-23.json`
- SDK ID: `b2bi`
- Endpoint prefix: `b2bi`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (5), `Get` (5), `List` (5), `Delete` (4), `Update` (4), `Test` (3), `Generate` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCapability`, `CreatePartnership`, `CreateProfile`, `CreateStarterMappingTemplate`, `CreateTransformer`, `DeleteCapability`, `DeletePartnership`, `DeleteProfile`, `DeleteTransformer`, `StartTransformerJob`, `TagResource`, `UntagResource`, `UpdateCapability`, `UpdatePartnership`, `UpdateProfile`, `UpdateTransformer`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCapability`, `GetPartnership`, `GetProfile`, `GetTransformer`, `GetTransformerJob`, `ListCapabilities`, `ListPartnerships`, `ListProfiles`, `ListTagsForResource`, `ListTransformers`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 18 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetTransformerJob`, `StartTransformerJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 30 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Capability` | `capabilityId` | create: `CreateCapability`; read: `GetCapability`; update: `UpdateCapability`; delete: `DeleteCapability`; list: `ListCapabilities` | - | - |
| `Partnership` | `partnershipId` | create: `CreatePartnership`; read: `GetPartnership`; update: `UpdatePartnership`; delete: `DeletePartnership`; list: `ListPartnerships` | - | - |
| `Profile` | `profileId` | create: `CreateProfile`; read: `GetProfile`; update: `UpdateProfile`; delete: `DeleteProfile`; list: `ListProfiles` | - | - |
| `Transformer` | `transformerId` | create: `CreateTransformer`; read: `GetTransformer`; update: `UpdateTransformer`; delete: `DeleteTransformer`; list: `ListTransformers` | - | - |
## Operation Groups

### Test

- Operations: `TestConversion`, `TestMapping`, `TestParsing`
- Traits: `idempotent` (3)
- Common required input members in this group: `fileFormat`

### Create

- Operations: `CreateStarterMappingTemplate`
- Common required input members in this group: -

### Generate

- Operations: `GenerateMapping`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Get

- Operations: `GetTransformerJob`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Start

- Operations: `StartTransformerJob`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateStarterMappingTemplate` | `POST /createmappingstarttemplate` | - | `mappingType`, `templateDetails` | - | `CreateStarterMappingTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Amazon Web Services B2B Data Interchange uses a mapping template in JSONata or XSLT format to transform a customer input file into a JSON or XML file that can be converted to EDI. If you provide a sample EDI file wit ... |
| `GenerateMapping` | `POST /generate-mapping` | `idempotent` | `inputFileContent`, `outputFileContent`, `mappingType` | - | `GenerateMappingResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Takes sample input and output documents and uses Amazon Bedrock to generate a mapping automatically. Depending on the accuracy and other factors, you can then edit the mapping for your needs. Before you can use the A ... |
| `GetTransformerJob` | `GET /transformer-jobs/{transformerJobId}` | `readonly` | `transformerJobId`, `transformerId` | - | `GetTransformerJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the details of the transformer run, based on the Transformer job ID. If 30 days have elapsed since your transformer job was started, the system deletes it. So, if you run GetTransformerJob and supply a transf ... |
| `ListTagsForResource` | `GET /tags/{ResourceARN}` | `readonly` | `ResourceARN` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all of the tags associated with the Amazon Resource Name (ARN) that you specify. The resource can be a capability, partnership, profile, or transformer. |
| `StartTransformerJob` | `POST /transformer-jobs` | `idempotent`, `idempotency-token` | `inputFile`, `outputLocation`, `transformerId` | `clientToken` | `StartTransformerJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Runs a job, using a transformer, to parse input EDI (electronic data interchange) file into the output structures used by Amazon Web Services B2B Data Interchange. If you only want to transform EDI (electronic data i ... |
| `TagResource` | `POST /tags/{ResourceARN}` | - | `ResourceARN`, `Tags` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches a key-value pair to a resource, as identified by its Amazon Resource Name (ARN). Resources are capability, partnership, profile, transformers and other entities. There is no response returned from this call. |
| `TestConversion` | `POST /testconversion` | `idempotent` | `source`, `target` | - | `TestConversionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation mimics the latter half of a typical Outbound EDI request. It takes an input JSON/XML in the B2Bi shape as input, converts it to an X12 EDI string, and return that string. |
| `TestMapping` | `POST /testmapping` | `idempotent` | `inputFileContent`, `mappingTemplate`, `fileFormat` | - | `TestMappingResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Maps the input file according to the provided template file. The API call downloads the file contents from the Amazon S3 location, and passes the contents in as a string, to the inputFileContent parameter. |
| `TestParsing` | `POST /testparsing` | `idempotent` | `inputFile`, `fileFormat`, `ediType` | - | `TestParsingResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Parses the input EDI (electronic data interchange) file. The input file has a file size limit of 250 KB. |
| `UntagResource` | `DELETE /tags/{ResourceARN}` | `idempotent` | `ResourceARN`, `TagKeys` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Detaches a key-value pair from the specified resource, as identified by its Amazon Resource Name (ARN). Resources are capability, partnership, profile, transformers and other entities. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetTransformerJob` | - | `transformerId -> transformerId` | - | - |
| `UntagResource` | - | `TagKeys -> TagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | A conflict exception is thrown when you attempt to delete a resource (such as a profile or a capability) that is being used by other resources. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | This exception is thrown when an error occurs in the Amazon Web Services B2B Data Interchange service. |
| `ResourceNotFoundException` | `structure` | message | Occurs when the requested resource does not exist, or cannot be found. In some cases, the resource exists in a region other than the region specified in the ... |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | Occurs when the calling command attempts to exceed one of the service quotas, for example trying to create a capability when you already have the maximum nu ... |
| `ThrottlingException` | `structure` | message, retryAfterSeconds | The request was denied due to throttling: the data speed and rendering may be limited depending on various parameters and conditions. |
| `ValidationException` | `structure` | Message | Occurs when a B2BI object cannot be validated against a request from another object. This exception can be thrown during standard EDI validation or when cus ... |
| `CreateStarterMappingTemplateRequest` | `structure` | outputSampleLocation, mappingType, templateDetails | - |
| `CreateStarterMappingTemplateResponse` | `structure` | mappingTemplate | - |
| `GenerateMappingRequest` | `structure` | inputFileContent, outputFileContent, mappingType | - |
| `GenerateMappingResponse` | `structure` | mappingTemplate, mappingAccuracy | - |
| `GetTransformerJobRequest` | `structure` | transformerJobId, transformerId | - |
| `GetTransformerJobResponse` | `structure` | status, outputFiles, message | - |
| `ListTagsForResourceRequest` | `structure` | ResourceARN | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `StartTransformerJobRequest` | `structure` | inputFile, outputLocation, transformerId, clientToken | - |
| `StartTransformerJobResponse` | `structure` | transformerJobId | - |
| `TagResourceRequest` | `structure` | ResourceARN, Tags | - |
| `TestConversionRequest` | `structure` | source, target | - |
| `TestConversionResponse` | `structure` | convertedFileContent, validationMessages | - |
| `TestMappingRequest` | `structure` | inputFileContent, mappingTemplate, fileFormat | - |
| `TestMappingResponse` | `structure` | mappedFileContent | - |
| `TestParsingRequest` | `structure` | inputFile, fileFormat, ediType, advancedOptions | - |
| `TestParsingResponse` | `structure` | parsedFileContent, parsedSplitFileContents, validationMessages | - |
| `UntagResourceRequest` | `structure` | ResourceARN, TagKeys | - |
| `CapabilityDirection` | `enum` | INBOUND, OUTBOUND | - |
| `CapabilityType` | `enum` | EDI | - |
| `ConversionSourceFormat` | `enum` | JSON, XML | - |
| `ConversionTargetFormat` | `enum` | X12 | - |
| `ElementRequirement` | `enum` | OPTIONAL, MANDATORY | - |
| `FileFormat` | `enum` | XML, JSON, NOT_USED | - |
| `FromFormat` | `enum` | X12 | - |
| `LineTerminator` | `enum` | CRLF, LF, CR | - |
| `Logging` | `enum` | ENABLED, DISABLED | - |
| `MappingTemplateLanguage` | `enum` | XSLT, JSONATA | - |
| `MappingType` | `enum` | JSONATA, XSLT | - |
| `ToFormat` | `enum` | X12 | - |
| `TransformerJobStatus` | `enum` | RUNNING, SUCCEEDED, FAILED | - |
| `TransformerStatus` | `enum` | ACTIVE, INACTIVE | - |
| `WrapFormat` | `enum` | SEGMENT, ONE_LINE, LINE_LENGTH | - |
| `X12FunctionalAcknowledgment` | `enum` | DO_NOT_GENERATE, GENERATE_ALL_SEGMENTS, GENERATE_WITHOUT_TRANSACTION_SET_RESPONSE_LOOP | - |
| `X12GS05TimeFormat` | `enum` | HHMM, HHMMSS, HHMMSSDD | Specifies the time format in the GS05 element (time) of the functional group header. The following formats use 24-hour clock time: HHMM - Hours and minutes ... |
| `X12SplitBy` | `enum` | NONE, TRANSACTION | - |
| `X12TechnicalAcknowledgment` | `enum` | DO_NOT_GENERATE, GENERATE_ALL_SEGMENTS | - |
| `X12TransactionSet` | `enum` | X12_100, X12_101, X12_102, X12_103, X12_104, X12_105, X12_106, X12_107, X12_108, X12_109, X12_110, X12_111, ... (+330) | - |
| `X12Version` | `enum` | VERSION_4010, VERSION_4030, VERSION_4050, VERSION_4060, VERSION_5010, VERSION_5010_HIPAA | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
