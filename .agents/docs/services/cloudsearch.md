# Amazon CloudSearch

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon CloudSearch Configuration Service You use the Amazon CloudSearch configuration service to create, configure, and manage search domains. Configuration service requests are submitted using the AWS Query protocol. AWS Query requests are HTTP or HTTPS requests submitted via HTTP GET or POST with a query parameter named Action. The endpoint for configuration service requests is region-specific: cloudsearch. region .amazonaws.com.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Amazon CloudSearch by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Amazon CloudSearch workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Delete`, `Define`, `Update`, `Build` operation families, including `DescribeAnalysisSchemes`, `DescribeAvailabilityOptions`, `DescribeDomainEndpointOptions`, `DescribeDomains`, `DeleteAnalysisScheme`, `DeleteDomain`.

## Service Identity and Protocol

- AWS model slug: `cloudsearch`
- AWS SDK for Rust slug: `cloudsearch`
- Model version: `2013-01-01`
- Model file: `vendor/api-models-aws/models/cloudsearch/service/2013-01-01/cloudsearch-2013-01-01.json`
- SDK ID: `CloudSearch`
- Endpoint prefix: `cloudsearch`
- ARN namespace: `cloudsearch`
- CloudFormation name: `CloudSearch`
- CloudTrail event source: `cloudsearch.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (9), `Delete` (5), `Define` (4), `Update` (4), `Build` (1), `Create` (1), `Index` (1), `List` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDomain`, `DeleteAnalysisScheme`, `DeleteDomain`, `DeleteExpression`, `DeleteIndexField`, `DeleteSuggester`, `UpdateAvailabilityOptions`, `UpdateDomainEndpointOptions`, `UpdateScalingParameters`, `UpdateServiceAccessPolicies`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAnalysisSchemes`, `DescribeAvailabilityOptions`, `DescribeDomainEndpointOptions`, `DescribeDomains`, `DescribeExpressions`, `DescribeIndexFields`, `DescribeScalingParameters`, `DescribeServiceAccessPolicies`, `DescribeSuggesters`, `ListDomainNames`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DefineAnalysisScheme`, `DeleteAnalysisScheme`, `DescribeAnalysisSchemes`.
- 26 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-started.html
- https://docs.aws.amazon.com/cloudsearch/latest/developerguide/indexing.html
- https://docs.aws.amazon.com/cloudsearch/latest/developerguide/getting-suggestions.html

Research outcomes:
- CloudSearch domains are search domains with configuration, search endpoints, document endpoints, indexes, fields, and scaling options.
- Document uploads add, update, or delete searchable documents through the document service endpoint.
- Some configuration changes require rebuilding the index with `IndexDocuments`.
- Search requests use the domain search endpoint and return matching indexed documents.
- Suggesters provide autocomplete suggestions and require configured text fields and indexing.
- Access policies can restrict search/document endpoints by IAM and IP conditions.

Parity implications:
- Model domains, domain status, search/document endpoints, index fields, documents, suggesters, access policies, and indexing state separately.
- Document upload should affect document service state, while search visibility should depend on indexing.
- Search and document API surfaces should remain distinct from domain configuration APIs.

## Operation Groups

### Describe

- Operations: `DescribeAnalysisSchemes`, `DescribeAvailabilityOptions`, `DescribeDomainEndpointOptions`, `DescribeDomains`, `DescribeExpressions`, `DescribeIndexFields`, `DescribeScalingParameters`, `DescribeServiceAccessPolicies`, `DescribeSuggesters`
- Common required input members in this group: `DomainName`

### Delete

- Operations: `DeleteAnalysisScheme`, `DeleteDomain`, `DeleteExpression`, `DeleteIndexField`, `DeleteSuggester`
- Common required input members in this group: `AnalysisSchemeName`, `DomainName`, `ExpressionName`, `IndexFieldName`, `SuggesterName`

### Define

- Operations: `DefineAnalysisScheme`, `DefineExpression`, `DefineIndexField`, `DefineSuggester`
- Common required input members in this group: `AnalysisScheme`, `DomainName`, `Expression`, `IndexField`, `Suggester`

### Update

- Operations: `UpdateAvailabilityOptions`, `UpdateDomainEndpointOptions`, `UpdateScalingParameters`, `UpdateServiceAccessPolicies`
- Common required input members in this group: `AccessPolicies`, `DomainEndpointOptions`, `DomainName`, `MultiAZ`, `ScalingParameters`

### Build

- Operations: `BuildSuggesters`
- Common required input members in this group: `DomainName`

### Create

- Operations: `CreateDomain`
- Common required input members in this group: `DomainName`

### Index

- Operations: `IndexDocuments`
- Common required input members in this group: `DomainName`

### List

- Operations: `ListDomainNames`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BuildSuggesters` | - | - | `DomainName` | - | `BuildSuggestersResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Indexes the search suggestions. For more information, see Configuring Suggesters in the Amazon CloudSearch Developer Guide . |
| `CreateDomain` | - | - | `DomainName` | - | `CreateDomainResponse` | `BaseException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ValidationException` | Creates a new search domain. For more information, see Creating a Search Domain in the Amazon CloudSearch Developer Guide . |
| `DefineAnalysisScheme` | - | - | `AnalysisScheme`, `DomainName` | - | `DefineAnalysisSchemeResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures an analysis scheme that can be applied to a `text` or `text-array` field to define language-specific text processing options. For more information, see Configuring Analysis Schemes in the Amazon CloudSearch Developer Guide . |
| `DefineExpression` | - | - | `DomainName`, `Expression` | - | `DefineExpressionResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures an `Expression` for the search domain. Used to create new expressions and modify existing ones. |
| `DefineIndexField` | - | - | `DomainName`, `IndexField` | - | `DefineIndexFieldResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures an `IndexField` for the search domain. Used to create new fields and modify existing ones. |
| `DefineSuggester` | - | - | `DomainName`, `Suggester` | - | `DefineSuggesterResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures a suggester for a domain. A suggester enables you to display possible matches before users finish typing their queries. |
| `DeleteAnalysisScheme` | - | - | `AnalysisSchemeName`, `DomainName` | - | `DeleteAnalysisSchemeResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `ResourceNotFoundException`, `ValidationException` | Deletes an analysis scheme. For more information, see Configuring Analysis Schemes in the Amazon CloudSearch Developer Guide . |
| `DeleteDomain` | - | - | `DomainName` | - | `DeleteDomainResponse` | `BaseException`, `InternalException` | Permanently deletes a search domain and all of its data. Once a domain has been deleted, it cannot be recovered. |
| `DeleteExpression` | - | - | `DomainName`, `ExpressionName` | - | `DeleteExpressionResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `ResourceNotFoundException`, `ValidationException` | Removes an `Expression` from the search domain. For more information, see Configuring Expressions in the Amazon CloudSearch Developer Guide . |
| `DeleteIndexField` | - | - | `DomainName`, `IndexFieldName` | - | `DeleteIndexFieldResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `ResourceNotFoundException`, `ValidationException` | Removes an `IndexField` from the search domain. For more information, see Configuring Index Fields in the Amazon CloudSearch Developer Guide . |
| `DeleteSuggester` | - | - | `DomainName`, `SuggesterName` | - | `DeleteSuggesterResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `ResourceNotFoundException`, `ValidationException` | Deletes a suggester. For more information, see Getting Search Suggestions in the Amazon CloudSearch Developer Guide . |
| `DescribeAnalysisSchemes` | - | - | `DomainName` | - | `DescribeAnalysisSchemesResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets the analysis schemes configured for a domain. An analysis scheme defines language-specific text processing options for a `text` field. |
| `DescribeAvailabilityOptions` | - | - | `DomainName` | - | `DescribeAvailabilityOptionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException` | Gets the availability options configured for a domain. By default, shows the configuration with any pending changes. |
| `DescribeDomainEndpointOptions` | - | - | `DomainName` | - | `DescribeDomainEndpointOptionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException` | Returns the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see Configuring Domain Endpoint Options in the Amazon CloudSearch Developer Guide . |
| `DescribeDomains` | - | - | - | - | `DescribeDomainsResponse` | `BaseException`, `InternalException` | Gets information about the search domains owned by this account. Can be limited to specific domains. |
| `DescribeExpressions` | - | - | `DomainName` | - | `DescribeExpressionsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets the expressions configured for the search domain. Can be limited to specific expressions by name. |
| `DescribeIndexFields` | - | - | `DomainName` | - | `DescribeIndexFieldsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets information about the index fields configured for the search domain. Can be limited to specific fields by name. |
| `DescribeScalingParameters` | - | - | `DomainName` | - | `DescribeScalingParametersResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets the scaling parameters configured for a domain. A domain's scaling parameters specify the desired search instance type and replication count. |
| `DescribeServiceAccessPolicies` | - | - | `DomainName` | - | `DescribeServiceAccessPoliciesResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets information about the access policies that control access to the domain's document and search endpoints. By default, shows the configuration with any pending changes. |
| `DescribeSuggesters` | - | - | `DomainName` | - | `DescribeSuggestersResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets the suggesters configured for a domain. A suggester enables you to display possible matches before users finish typing their queries. |
| `IndexDocuments` | - | - | `DomainName` | - | `IndexDocumentsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Tells the search domain to start indexing its documents using the latest indexing options. This operation must be invoked to activate options whose OptionStatus is `RequiresIndexDocuments`. |
| `ListDomainNames` | - | - | - | - | `ListDomainNamesResponse` | `BaseException` | Lists all search domains owned by an account. |
| `UpdateAvailabilityOptions` | - | - | `DomainName`, `MultiAZ` | - | `UpdateAvailabilityOptionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures the availability options for a domain. Enabling the Multi-AZ option expands an Amazon CloudSearch domain to an additional Availability Zone in the same Region to increase fault tolerance in the event of a service disruption. |
| `UpdateDomainEndpointOptions` | - | - | `DomainEndpointOptions`, `DomainName` | - | `UpdateDomainEndpointOptionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Updates the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see Configuring Domain Endpoint Options in the Amazon CloudSearch Developer Guide . |
| `UpdateScalingParameters` | - | - | `DomainName`, `ScalingParameters` | - | `UpdateScalingParametersResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures scaling parameters for a domain. A domain's scaling parameters specify the desired search instance type and replication count. |
| `UpdateServiceAccessPolicies` | - | - | `AccessPolicies`, `DomainName` | - | `UpdateServiceAccessPoliciesResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures the access rules that control access to the domain's document and search endpoints. For more information, see Configuring Access for an Amazon CloudSearch Domain. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BaseException` | `structure` | `Code`, `Message` | An error occurred while processing the request. |
| `InternalException` | `structure` | `Code`, `Message` | An internal error occurred while processing the request. |
| `ResourceNotFoundException` | `structure` | `Code`, `Message` | The request was rejected because it attempted to reference a resource that does not exist. |
| `ValidationException` | `structure` | `Code`, `Message` | The request was rejected because it has invalid parameters. |
| `InvalidTypeException` | `structure` | `Code`, `Message` | The request was rejected because it specified an invalid type definition. |
| `LimitExceededException` | `structure` | `Code`, `Message` | The request was rejected because a resource limit has already been met. |
| `DisabledOperationException` | `structure` | `Code`, `Message` | The request was rejected because it attempted an operation which is not enabled. |
| `BuildSuggestersRequest` | `structure` | `DomainName` | Container for the parameters to the `BuildSuggester` operation. |
| `BuildSuggestersResponse` | `structure` | `FieldNames` | The result of a `BuildSuggester` request. |
| `CreateDomainRequest` | `structure` | `DomainName` | Container for the parameters to the `CreateDomain` operation. |
| `CreateDomainResponse` | `structure` | `DomainStatus` | The result of a `CreateDomainRequest`. |
| `ResourceAlreadyExistsException` | `structure` | `Code`, `Message` | The request was rejected because it attempted to create a resource that already exists. |
| `DefineAnalysisSchemeRequest` | `structure` | `AnalysisScheme`, `DomainName` | Container for the parameters to the `DefineAnalysisScheme` operation. |
| `DefineAnalysisSchemeResponse` | `structure` | `AnalysisScheme` | The result of a `DefineAnalysisScheme` request. |
| `DefineExpressionRequest` | `structure` | `DomainName`, `Expression` | Container for the parameters to the `DefineExpression` operation. |
| `DefineExpressionResponse` | `structure` | `Expression` | The result of a `DefineExpression` request. |
| `DefineIndexFieldRequest` | `structure` | `DomainName`, `IndexField` | Container for the parameters to the `DefineIndexField` operation. |
| `DefineIndexFieldResponse` | `structure` | `IndexField` | The result of a `DefineIndexField` request. |
| `DefineSuggesterRequest` | `structure` | `DomainName`, `Suggester` | Container for the parameters to the `DefineSuggester` operation. |
| `DefineSuggesterResponse` | `structure` | `Suggester` | The result of a `DefineSuggester` request. |
| `DeleteAnalysisSchemeRequest` | `structure` | `AnalysisSchemeName`, `DomainName` | Container for the parameters to the `DeleteAnalysisScheme` operation. |
| `DeleteAnalysisSchemeResponse` | `structure` | `AnalysisScheme` | The result of a `DeleteAnalysisScheme` request. |
| `DeleteDomainRequest` | `structure` | `DomainName` | Container for the parameters to the `DeleteDomain` operation. |
| `DeleteDomainResponse` | `structure` | `DomainStatus` | The result of a `DeleteDomain` request. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
