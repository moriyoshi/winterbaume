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
- Common required input members in this group: `DomainName`

### Define

- Operations: `DefineAnalysisScheme`, `DefineExpression`, `DefineIndexField`, `DefineSuggester`
- Common required input members in this group: `DomainName`

### Update

- Operations: `UpdateAvailabilityOptions`, `UpdateDomainEndpointOptions`, `UpdateScalingParameters`, `UpdateServiceAccessPolicies`
- Common required input members in this group: `DomainName`

### Build

- Operations: `BuildSuggesters`
- Common required input members in this group: -

### Create

- Operations: `CreateDomain`
- Common required input members in this group: -

### Index

- Operations: `IndexDocuments`
- Common required input members in this group: -

### List

- Operations: `ListDomainNames`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BuildSuggesters` | `-` | - | `DomainName` | - | `BuildSuggestersResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Indexes the search suggestions. For more information, see Configuring Suggesters in the Amazon CloudSearch Developer Guide . |
| `CreateDomain` | `-` | - | `DomainName` | - | `CreateDomainResponse` | `BaseException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ValidationException` | Creates a new search domain. For more information, see Creating a Search Domain in the Amazon CloudSearch Developer Guide . |
| `DefineAnalysisScheme` | `-` | - | `DomainName`, `AnalysisScheme` | - | `DefineAnalysisSchemeResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures an analysis scheme that can be applied to a text or text-array field to define language-specific text processing options. For more information, see Configuring Analysis Schemes in the Amazon CloudSearch De ... |
| `DefineExpression` | `-` | - | `DomainName`, `Expression` | - | `DefineExpressionResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures an Expression for the search domain. Used to create new expressions and modify existing ones. If the expression exists, the new configuration replaces the old one. For more information, see Configuring Exp ... |
| `DefineIndexField` | `-` | - | `DomainName`, `IndexField` | - | `DefineIndexFieldResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures an IndexField for the search domain. Used to create new fields and modify existing ones. You must specify the name of the domain you are configuring and an index field configuration. The index field config ... |
| `DefineSuggester` | `-` | - | `DomainName`, `Suggester` | - | `DefineSuggesterResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures a suggester for a domain. A suggester enables you to display possible matches before users finish typing their queries. When you configure a suggester, you must specify the name of the text field you want ... |
| `DeleteAnalysisScheme` | `-` | - | `DomainName`, `AnalysisSchemeName` | - | `DeleteAnalysisSchemeResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `ResourceNotFoundException`, `ValidationException` | Deletes an analysis scheme. For more information, see Configuring Analysis Schemes in the Amazon CloudSearch Developer Guide . |
| `DeleteDomain` | `-` | - | `DomainName` | - | `DeleteDomainResponse` | `BaseException`, `InternalException` | Permanently deletes a search domain and all of its data. Once a domain has been deleted, it cannot be recovered. For more information, see Deleting a Search Domain in the Amazon CloudSearch Developer Guide . |
| `DeleteExpression` | `-` | - | `DomainName`, `ExpressionName` | - | `DeleteExpressionResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `ResourceNotFoundException`, `ValidationException` | Removes an Expression from the search domain. For more information, see Configuring Expressions in the Amazon CloudSearch Developer Guide . |
| `DeleteIndexField` | `-` | - | `DomainName`, `IndexFieldName` | - | `DeleteIndexFieldResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `ResourceNotFoundException`, `ValidationException` | Removes an IndexField from the search domain. For more information, see Configuring Index Fields in the Amazon CloudSearch Developer Guide . |
| `DeleteSuggester` | `-` | - | `DomainName`, `SuggesterName` | - | `DeleteSuggesterResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `ResourceNotFoundException`, `ValidationException` | Deletes a suggester. For more information, see Getting Search Suggestions in the Amazon CloudSearch Developer Guide . |
| `DescribeAnalysisSchemes` | `-` | - | `DomainName` | - | `DescribeAnalysisSchemesResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets the analysis schemes configured for a domain. An analysis scheme defines language-specific text processing options for a text field. Can be limited to specific analysis schemes by name. By default, shows all ana ... |
| `DescribeAvailabilityOptions` | `-` | - | `DomainName` | - | `DescribeAvailabilityOptionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException` | Gets the availability options configured for a domain. By default, shows the configuration with any pending changes. Set the Deployed option to true to show the active configuration and exclude pending changes. For m ... |
| `DescribeDomainEndpointOptions` | `-` | - | `DomainName` | - | `DescribeDomainEndpointOptionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException` | Returns the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see Configuring Domain Endpoint Options in the Amazon CloudSearch Developer Guide . |
| `DescribeDomains` | `-` | - | - | - | `DescribeDomainsResponse` | `BaseException`, `InternalException` | Gets information about the search domains owned by this account. Can be limited to specific domains. Shows all domains by default. To get the number of searchable documents in a domain, use the console or submit a ma ... |
| `DescribeExpressions` | `-` | - | `DomainName` | - | `DescribeExpressionsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets the expressions configured for the search domain. Can be limited to specific expressions by name. By default, shows all expressions and includes any pending changes to the configuration. Set the Deployed option ... |
| `DescribeIndexFields` | `-` | - | `DomainName` | - | `DescribeIndexFieldsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets information about the index fields configured for the search domain. Can be limited to specific fields by name. By default, shows all fields and includes any pending changes to the configuration. Set the Deploye ... |
| `DescribeScalingParameters` | `-` | - | `DomainName` | - | `DescribeScalingParametersResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets the scaling parameters configured for a domain. A domain's scaling parameters specify the desired search instance type and replication count. For more information, see Configuring Scaling Options in the Amazon C ... |
| `DescribeServiceAccessPolicies` | `-` | - | `DomainName` | - | `DescribeServiceAccessPoliciesResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets information about the access policies that control access to the domain's document and search endpoints. By default, shows the configuration with any pending changes. Set the Deployed option to true to show the ... |
| `DescribeSuggesters` | `-` | - | `DomainName` | - | `DescribeSuggestersResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException` | Gets the suggesters configured for a domain. A suggester enables you to display possible matches before users finish typing their queries. Can be limited to specific suggesters by name. By default, shows all suggeste ... |
| `IndexDocuments` | `-` | - | `DomainName` | - | `IndexDocumentsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Tells the search domain to start indexing its documents using the latest indexing options. This operation must be invoked to activate options whose OptionStatus is RequiresIndexDocuments . |
| `ListDomainNames` | `-` | - | - | - | `ListDomainNamesResponse` | `BaseException` | Lists all search domains owned by an account. |
| `UpdateAvailabilityOptions` | `-` | - | `DomainName`, `MultiAZ` | - | `UpdateAvailabilityOptionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures the availability options for a domain. Enabling the Multi-AZ option expands an Amazon CloudSearch domain to an additional Availability Zone in the same Region to increase fault tolerance in the event of a ... |
| `UpdateDomainEndpointOptions` | `-` | - | `DomainName`, `DomainEndpointOptions` | - | `UpdateDomainEndpointOptionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Updates the domain's endpoint options, specifically whether all requests to the domain must arrive over HTTPS. For more information, see Configuring Domain Endpoint Options in the Amazon CloudSearch Developer Guide . |
| `UpdateScalingParameters` | `-` | - | `DomainName`, `ScalingParameters` | - | `UpdateScalingParametersResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures scaling parameters for a domain. A domain's scaling parameters specify the desired search instance type and replication count. Amazon CloudSearch will still automatically scale your domain based on the vol ... |
| `UpdateServiceAccessPolicies` | `-` | - | `DomainName`, `AccessPolicies` | - | `UpdateServiceAccessPoliciesResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Configures the access rules that control access to the domain's document and search endpoints. For more information, see Configuring Access for an Amazon CloudSearch Domain . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BaseException` | `structure` | Code, Message | An error occurred while processing the request. |
| `DisabledOperationException` | `structure` | Code, Message | The request was rejected because it attempted an operation which is not enabled. |
| `InternalException` | `structure` | Code, Message | An internal error occurred while processing the request. If this problem persists, report an issue from the Service Health Dashboard . |
| `InvalidTypeException` | `structure` | Code, Message | The request was rejected because it specified an invalid type definition. |
| `LimitExceededException` | `structure` | Code, Message | The request was rejected because a resource limit has already been met. |
| `ResourceAlreadyExistsException` | `structure` | Code, Message | The request was rejected because it attempted to create a resource that already exists. |
| `ResourceNotFoundException` | `structure` | Code, Message | The request was rejected because it attempted to reference a resource that does not exist. |
| `ValidationException` | `structure` | Code, Message | The request was rejected because it has invalid parameters. |
| `BuildSuggestersRequest` | `structure` | DomainName | Container for the parameters to the BuildSuggester operation. Specifies the name of the domain you want to update. |
| `BuildSuggestersResponse` | `structure` | FieldNames | The result of a BuildSuggester request. Contains a list of the fields used for suggestions. |
| `CreateDomainRequest` | `structure` | DomainName | Container for the parameters to the CreateDomain operation. Specifies a name for the new search domain. |
| `CreateDomainResponse` | `structure` | DomainStatus | The result of a CreateDomainRequest . Contains the status of a newly created domain. |
| `DefineAnalysisSchemeRequest` | `structure` | DomainName, AnalysisScheme | Container for the parameters to the DefineAnalysisScheme operation. Specifies the name of the domain you want to update and the analysis scheme configuration. |
| `DefineAnalysisSchemeResponse` | `structure` | AnalysisScheme | The result of a DefineAnalysisScheme request. Contains the status of the newly-configured analysis scheme. |
| `DefineExpressionRequest` | `structure` | DomainName, Expression | Container for the parameters to the DefineExpression operation. Specifies the name of the domain you want to update and the expression you want to configure. |
| `DefineExpressionResponse` | `structure` | Expression | The result of a DefineExpression request. Contains the status of the newly-configured expression. |
| `DefineIndexFieldRequest` | `structure` | DomainName, IndexField | Container for the parameters to the DefineIndexField operation. Specifies the name of the domain you want to update and the index field configuration. |
| `DefineIndexFieldResponse` | `structure` | IndexField | The result of a DefineIndexField request. Contains the status of the newly-configured index field. |
| `DefineSuggesterRequest` | `structure` | DomainName, Suggester | Container for the parameters to the DefineSuggester operation. Specifies the name of the domain you want to update and the suggester configuration. |
| `DefineSuggesterResponse` | `structure` | Suggester | The result of a DefineSuggester request. Contains the status of the newly-configured suggester. |
| `DeleteAnalysisSchemeRequest` | `structure` | DomainName, AnalysisSchemeName | Container for the parameters to the DeleteAnalysisScheme operation. Specifies the name of the domain you want to update and the analysis scheme you want to ... |
| `DeleteAnalysisSchemeResponse` | `structure` | AnalysisScheme | The result of a DeleteAnalysisScheme request. Contains the status of the deleted analysis scheme. |
| `DeleteDomainRequest` | `structure` | DomainName | Container for the parameters to the DeleteDomain operation. Specifies the name of the domain you want to delete. |
| `DeleteDomainResponse` | `structure` | DomainStatus | The result of a DeleteDomain request. Contains the status of a newly deleted domain, or no status if the domain has already been completely deleted. |
| `DeleteExpressionRequest` | `structure` | DomainName, ExpressionName | Container for the parameters to the DeleteExpression operation. Specifies the name of the domain you want to update and the name of the expression you want ... |
| `DeleteExpressionResponse` | `structure` | Expression | The result of a DeleteExpression request. Specifies the expression being deleted. |
| `DeleteIndexFieldRequest` | `structure` | DomainName, IndexFieldName | Container for the parameters to the DeleteIndexField operation. Specifies the name of the domain you want to update and the name of the index field you want ... |
| `DeleteIndexFieldResponse` | `structure` | IndexField | The result of a DeleteIndexField request. |
| `DeleteSuggesterRequest` | `structure` | DomainName, SuggesterName | Container for the parameters to the DeleteSuggester operation. Specifies the name of the domain you want to update and name of the suggester you want to delete. |
| `DeleteSuggesterResponse` | `structure` | Suggester | The result of a DeleteSuggester request. Contains the status of the deleted suggester. |
| `DescribeAnalysisSchemesRequest` | `structure` | DomainName, AnalysisSchemeNames, Deployed | Container for the parameters to the DescribeAnalysisSchemes operation. Specifies the name of the domain you want to describe. To limit the response to parti ... |
| `DescribeAnalysisSchemesResponse` | `structure` | AnalysisSchemes | The result of a DescribeAnalysisSchemes request. Contains the analysis schemes configured for the domain specified in the request. |
| `DescribeAvailabilityOptionsRequest` | `structure` | DomainName, Deployed | Container for the parameters to the DescribeAvailabilityOptions operation. Specifies the name of the domain you want to describe. To show the active configu ... |
| `DescribeAvailabilityOptionsResponse` | `structure` | AvailabilityOptions | The result of a DescribeAvailabilityOptions request. Indicates whether or not the Multi-AZ option is enabled for the domain specified in the request. |
| `DescribeDomainEndpointOptionsRequest` | `structure` | DomainName, Deployed | Container for the parameters to the DescribeDomainEndpointOptions operation. Specify the name of the domain you want to describe. To show the active configu ... |
| `DescribeDomainEndpointOptionsResponse` | `structure` | DomainEndpointOptions | The result of a DescribeDomainEndpointOptions request. Contains the status and configuration of a search domain's endpoint options. |
| `DescribeDomainsRequest` | `structure` | DomainNames | Container for the parameters to the DescribeDomains operation. By default shows the status of all domains. To restrict the response to particular domains, s ... |
| `DescribeDomainsResponse` | `structure` | DomainStatusList | The result of a DescribeDomains request. Contains the status of the domains specified in the request or all domains owned by the account. |
| `DescribeExpressionsRequest` | `structure` | DomainName, ExpressionNames, Deployed | Container for the parameters to the DescribeDomains operation. Specifies the name of the domain you want to describe. To restrict the response to particular ... |
| `DescribeExpressionsResponse` | `structure` | Expressions | The result of a DescribeExpressions request. Contains the expressions configured for the domain specified in the request. |
| `AlgorithmicStemming` | `enum` | none, minimal, light, full | - |
| `AnalysisSchemeLanguage` | `enum` | ar, bg, ca, cs, da, de, el, en, es, eu, fa, fi, ... (+23) | An IETF RFC 4646 language code or mul for multiple languages. |
| `IndexFieldType` | `enum` | int, double, literal, text, date, latlon, int_array, double_array, literal_array, text_array, date_array | The type of field. The valid options for a field depend on the field type. For more information about the supported field types, see Configuring Index Field ... |
| `OptionState` | `enum` | RequiresIndexDocuments, Processing, Active, FailedToValidate | The state of processing a change to an option. One of: RequiresIndexDocuments: The option's latest value will not be deployed until IndexDocuments has been ... |
| `PartitionInstanceType` | `enum` | search_m1_small, search_m1_large, search_m2_xlarge, search_m2_2xlarge, search_m3_medium, search_m3_large, search_m3_xlarge, search_m3_2xlarge, search_small, search_medium, search_large, search_xlarge, ... (+5) | The instance type (such as search.m1.small ) on which an index partition is hosted. |
| `SuggesterFuzzyMatching` | `enum` | none, low, high | - |
| `TLSSecurityPolicy` | `enum` | POLICY_MIN_TLS_1_0_2019_07, POLICY_MIN_TLS_1_2_2019_07 | The minimum required TLS version. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
