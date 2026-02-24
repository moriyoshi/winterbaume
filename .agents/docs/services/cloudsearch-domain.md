# Amazon CloudSearch Domain

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You use the AmazonCloudSearch2013 API to upload documents to a search domain and search those documents. The endpoints for submitting `UploadDocuments`, `Search`, and `Suggest` requests are domain-specific. To get the endpoints for your domain, use the Amazon CloudSearch configuration service `DescribeDomains` action. The domain endpoints are also displayed on the domain dashboard in the Amazon CloudSearch console. You submit suggest requests to the search endpoint.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon CloudSearch Domain workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Search`, `Suggest`, `Upload` operation families, including `Search`, `Suggest`, `UploadDocuments`.

## Service Identity and Protocol

- AWS model slug: `cloudsearch-domain`
- AWS SDK for Rust slug: `cloudsearchdomain`
- Model version: `2013-01-01`
- Model file: `vendor/api-models-aws/models/cloudsearch-domain/service/2013-01-01/cloudsearch-domain-2013-01-01.json`
- SDK ID: `CloudSearch Domain`
- Endpoint prefix: `cloudsearchdomain`
- ARN namespace: `cloudsearch`
- CloudFormation name: `CloudSearchDomain`
- CloudTrail event source: `cloudsearchdomain.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Search` (1), `Suggest` (1), `Upload` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `Search`.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cloudsearch/latest/developerguide/searching.html
- https://docs.aws.amazon.com/cloudsearch/latest/developerguide/submitting-documents.html
- https://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html

Research outcomes:
- CloudSearch Domain is the data-plane endpoint service for searching, suggesting, and submitting documents to a specific CloudSearch domain.
- Search requests run against indexed documents and support query parsers, return fields, facets, sorting, highlighting, and pagination.
- Suggest requests use configured suggesters and return autocomplete suggestions.
- Document batch submission accepts add and delete operations for documents in the domain.
- Data-plane endpoint access is governed by the domain access policies and endpoint URLs generated for the domain.

Parity implications:
- Model CloudSearch domain data-plane state separately from CloudSearch control-plane domain metadata.
- Search should read indexed state; document submission should update pending/indexable document state.
- Suggest results should require configured suggesters and indexed suggestion data.

## Operation Groups

### Search

- Operations: `Search`
- Common required input members in this group: `query`

### Suggest

- Operations: `Suggest`
- Common required input members in this group: `query`, `suggester`

### Upload

- Operations: `UploadDocuments`
- Common required input members in this group: `contentType`, `documents`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `Search` | `GET /2013-01-01/search?format=sdk&pretty=true` | - | `query` | - | `SearchResponse` | `SearchException` | Retrieves a list of documents that match the specified search criteria. How you specify the search criteria depends on which query parser you use. |
| `Suggest` | `GET /2013-01-01/suggest?format=sdk&pretty=true` | - | `query`, `suggester` | - | `SuggestResponse` | `SearchException` | Retrieves autocomplete suggestions for a partial query string. You can use suggestions enable you to display likely matches before users finish typing. |
| `UploadDocuments` | `POST /2013-01-01/documents/batch?format=sdk` | - | `contentType`, `documents` | - | `UploadDocumentsResponse` | `DocumentServiceException` | Posts a batch of documents to a search domain for indexing. A document batch is a collection of add and delete operations that represent the documents you want to add, update, or delete from your domain. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `SearchException` | `structure` | `message` | Information about any problems encountered while processing a search request. |
| `SearchRequest` | `structure` | `cursor`, `expr`, `facet`, `filterQuery`, `highlight`, `partial`, `query`, `queryOptions`, `queryParser`, `return`, `size`, `sort`, ... (+2) | Container for the parameters to the `Search` request. |
| `SearchResponse` | `structure` | `facets`, `hits`, `stats`, `status` | The result of a `Search` request. |
| `SuggestRequest` | `structure` | `query`, `size`, `suggester` | Container for the parameters to the `Suggest` request. |
| `SuggestResponse` | `structure` | `status`, `suggest` | Contains the response to a `Suggest` request. |
| `UploadDocumentsRequest` | `structure` | `contentType`, `documents` | Container for the parameters to the `UploadDocuments` request. |
| `UploadDocumentsResponse` | `structure` | `adds`, `deletes`, `status`, `warnings` | Contains the response to an `UploadDocuments` request. |
| `DocumentServiceException` | `structure` | `message`, `status` | Information about any problems encountered while processing an upload request. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
