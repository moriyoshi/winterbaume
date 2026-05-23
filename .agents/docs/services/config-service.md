# AWS Config

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Config Config provides a way to keep track of the configurations of all the Amazon Web Services resources associated with your Amazon Web Services account. You can use Config to get the current and historical configurations of each Amazon Web Services resource and also to get information about the relationship between the resources. An Amazon Web Services resource can be an Amazon Compute Cloud (Amazon EC2) instance, an Elastic Block Store (EBS) volume, an elastic network Interface (ENI), or a security group. For a complete list of resources currently supported by Config, see Supported Amazon Web Services resources. You can access and manage Config through the Amazon Web Services Management Console, the Amazon Web Services Command Line Interface (Amazon Web Services CLI), the Config API, or the Amazon Web Services SDKs for Config.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Config where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Config by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Config resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Config workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Get`, `Delete`, `Put`, `List` operation families, including `DescribeAggregateComplianceByConfigRules`, `DescribeAggregateComplianceByConformancePacks`, `DescribeAggregationAuthorizations`, `DescribeComplianceByConfigRule`, `GetAggregateComplianceDetailsByConfigRule`, `GetAggregateConfigRuleComplianceSummary`.

## Service Identity and Protocol

- AWS model slug: `config-service`
- AWS SDK for Rust slug: `config`
- Model version: `2014-11-12`
- Model file: `vendor/api-models-aws/models/config-service/service/2014-11-12/config-service-2014-11-12.json`
- SDK ID: `Config Service`
- Endpoint prefix: `config`
- ARN namespace: `config`
- CloudFormation name: `Config`
- CloudTrail event source: `configservice.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (25), `Get` (19), `Delete` (16), `Put` (16), `List` (7), `Start` (4), `Batch` (2), `Select` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateResourceTypes`, `BatchGetAggregateResourceConfig`, `BatchGetResourceConfig`, `DeleteAggregationAuthorization`, `DeleteConfigRule`, `DeleteConfigurationAggregator`, `DeleteConfigurationRecorder`, `DeleteConformancePack`, `DeleteDeliveryChannel`, `DeleteEvaluationResults`, `DeleteOrganizationConfigRule`, `DeleteOrganizationConformancePack`, `DeletePendingAggregationRequest`, `DeleteRemediationConfiguration`, `DeleteRemediationExceptions`, `DeleteResourceConfig`, `DeleteRetentionConfiguration`, `DeleteServiceLinkedConfigurationRecorder`, `DeleteStoredQuery`, `DisassociateResourceTypes`, ... (+23).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAggregateComplianceByConfigRules`, `DescribeAggregateComplianceByConformancePacks`, `DescribeAggregationAuthorizations`, `DescribeComplianceByConfigRule`, `DescribeComplianceByResource`, `DescribeConfigRuleEvaluationStatus`, `DescribeConfigRules`, `DescribeConfigurationAggregatorSourcesStatus`, `DescribeConfigurationAggregators`, `DescribeConfigurationRecorderStatus`, `DescribeConfigurationRecorders`, `DescribeConformancePackCompliance`, `DescribeConformancePackStatus`, `DescribeConformancePacks`, `DescribeDeliveryChannelStatus`, `DescribeDeliveryChannels`, `DescribeOrganizationConfigRuleStatuses`, `DescribeOrganizationConfigRules`, `DescribeOrganizationConformancePackStatuses`, `DescribeOrganizationConformancePacks`, ... (+31).
- Pagination is modelled for 41 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeRemediationExecutionStatus`, `StartConfigRulesEvaluation`, `StartConfigurationRecorder`, `StartRemediationExecution`, `StartResourceEvaluation`, `StopConfigurationRecorder`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 95 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `SNS`, `Lambda`, `EC2/VPC`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/config/latest/developerguide/how-does-config-work.html
- https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_develop-rules.html
- https://docs.aws.amazon.com/config/latest/developerguide/s3-bucket-policy.html

Research outcomes:
- When AWS Config starts, it discovers supported resources and generates configuration items for them.
- AWS Config generates new configuration items when resource configurations change, and it keeps historical records from the time the recorder starts.
- Resource tracking uses Describe and List calls for resources and related resources. Config can also periodically detect changes not initiated through APIs.
- Config rules evaluate resources either in response to configuration changes or periodically. Lambda custom rules return compliance results.
- Configuration history files are delivered to S3 every six hours per recorded resource type when changes occur.
- DeliverConfigSnapshot creates a JSON snapshot containing configuration details for all recorded resources in the account.
- SNS notifications include message types for configuration item changes, oversized item changes, compliance changes, rule evaluation start, and snapshot/history delivery status.
- Config recording is Region-sensitive and can be scoped to selected resource types instead of every supported resource.

Parity implications:
- Model configuration recorders, delivery channels, resource type scopes, configuration items, snapshots, history delivery, rules, evaluations, remediations, and aggregators.
- Recording should create initial items, change-driven items, periodic detections, related-resource updates, and S3/SNS delivery artefacts.
- Rule evaluation needs trigger type, resource scope, Lambda/custom policy result ingestion, compliance transitions, and notification side effects.

## Operation Groups

### Describe

- Operations: `DescribeAggregateComplianceByConfigRules`, `DescribeAggregateComplianceByConformancePacks`, `DescribeAggregationAuthorizations`, `DescribeComplianceByConfigRule`, `DescribeComplianceByResource`, `DescribeConfigRuleEvaluationStatus`, `DescribeConfigRules`, `DescribeConfigurationAggregators`, `DescribeConfigurationAggregatorSourcesStatus`, `DescribeConfigurationRecorders`, `DescribeConfigurationRecorderStatus`, `DescribeConformancePackCompliance`, `DescribeConformancePacks`, `DescribeConformancePackStatus`, `DescribeDeliveryChannels`, `DescribeDeliveryChannelStatus`, `DescribeOrganizationConfigRules`, `DescribeOrganizationConfigRuleStatuses`, `DescribeOrganizationConformancePacks`, `DescribeOrganizationConformancePackStatuses`, `DescribePendingAggregationRequests`, `DescribeRemediationConfigurations`, `DescribeRemediationExceptions`, `DescribeRemediationExecutionStatus`, `DescribeRetentionConfigurations`
- Traits: `paginated` (20)
- Common required input members in this group: `ConfigurationAggregatorName`, `ConfigRuleName`

### Get

- Operations: `GetAggregateComplianceDetailsByConfigRule`, `GetAggregateConfigRuleComplianceSummary`, `GetAggregateConformancePackComplianceSummary`, `GetAggregateDiscoveredResourceCounts`, `GetAggregateResourceConfig`, `GetComplianceDetailsByConfigRule`, `GetComplianceDetailsByResource`, `GetComplianceSummaryByConfigRule`, `GetComplianceSummaryByResourceType`, `GetConformancePackComplianceDetails`, `GetConformancePackComplianceSummary`, `GetCustomRulePolicy`, `GetDiscoveredResourceCounts`, `GetOrganizationConfigRuleDetailedStatus`, `GetOrganizationConformancePackDetailedStatus`, `GetOrganizationCustomRulePolicy`, `GetResourceConfigHistory`, `GetResourceEvaluationSummary`, `GetStoredQuery`
- Traits: `paginated` (12)
- Common required input members in this group: `ConfigurationAggregatorName`, `ConfigRuleName`, `OrganizationConfigRuleName`

### Delete

- Operations: `DeleteAggregationAuthorization`, `DeleteConfigRule`, `DeleteConfigurationAggregator`, `DeleteConfigurationRecorder`, `DeleteConformancePack`, `DeleteDeliveryChannel`, `DeleteEvaluationResults`, `DeleteOrganizationConfigRule`, `DeleteOrganizationConformancePack`, `DeletePendingAggregationRequest`, `DeleteRemediationConfiguration`, `DeleteRemediationExceptions`, `DeleteResourceConfig`, `DeleteRetentionConfiguration`, `DeleteServiceLinkedConfigurationRecorder`, `DeleteStoredQuery`
- Common required input members in this group: `ConfigRuleName`

### Put

- Operations: `PutAggregationAuthorization`, `PutConfigRule`, `PutConfigurationAggregator`, `PutConfigurationRecorder`, `PutConformancePack`, `PutDeliveryChannel`, `PutEvaluations`, `PutExternalEvaluation`, `PutOrganizationConfigRule`, `PutOrganizationConformancePack`, `PutRemediationConfigurations`, `PutRemediationExceptions`, `PutResourceConfig`, `PutRetentionConfiguration`, `PutServiceLinkedConfigurationRecorder`, `PutStoredQuery`
- Common required input members in this group: `ConfigRuleName`

### List

- Operations: `ListAggregateDiscoveredResources`, `ListConfigurationRecorders`, `ListConformancePackComplianceScores`, `ListDiscoveredResources`, `ListResourceEvaluations`, `ListStoredQueries`, `ListTagsForResource`
- Traits: `paginated` (7)
- Common required input members in this group: -

### Start

- Operations: `StartConfigRulesEvaluation`, `StartConfigurationRecorder`, `StartRemediationExecution`, `StartResourceEvaluation`
- Common required input members in this group: -

### Batch

- Operations: `BatchGetAggregateResourceConfig`, `BatchGetResourceConfig`
- Common required input members in this group: -

### Select

- Operations: `SelectAggregateResourceConfig`, `SelectResourceConfig`
- Traits: `paginated` (2)
- Common required input members in this group: `Expression`

### Associate

- Operations: `AssociateResourceTypes`
- Common required input members in this group: -

### Deliver

- Operations: `DeliverConfigSnapshot`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateResourceTypes`
- Common required input members in this group: -

### Stop

- Operations: `StopConfigurationRecorder`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateResourceTypes` | `-` | - | `ConfigurationRecorderArn`, `ResourceTypes` | - | `AssociateResourceTypesResponse` | `ConflictException`, `NoSuchConfigurationRecorderException`, `ValidationException` | Adds all resource types specified in the ResourceTypes list to the RecordingGroup of specified configuration recorder and includes those resource types when recording. For this operation, the specified configuration ... |
| `BatchGetAggregateResourceConfig` | `-` | - | `ConfigurationAggregatorName`, `ResourceIdentifiers` | - | `BatchGetAggregateResourceConfigResponse` | `NoSuchConfigurationAggregatorException`, `ValidationException` | Returns the current configuration items for resources that are present in your Config aggregator. The operation also returns a list of resources that are not processed in the current request. If there are no unproces ... |
| `BatchGetResourceConfig` | `-` | - | `resourceKeys` | - | `BatchGetResourceConfigResponse` | `NoAvailableConfigurationRecorderException`, `ValidationException` | Returns the BaseConfigurationItem for one or more requested resources. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operatio ... |
| `DeleteAggregationAuthorization` | `-` | - | `AuthorizedAccountId`, `AuthorizedAwsRegion` | - | `Unit` | `InvalidParameterValueException` | Deletes the authorization granted to the specified configuration aggregator account in a specified region. |
| `DeleteConfigRule` | `-` | - | `ConfigRuleName` | - | `Unit` | `NoSuchConfigRuleException`, `ResourceInUseException` | Deletes the specified Config rule and all of its evaluation results. Config sets the state of a rule to DELETING until the deletion is complete. You cannot update a rule while it is in this state. If you make a PutCo ... |
| `DeleteConfigurationAggregator` | `-` | - | `ConfigurationAggregatorName` | - | `Unit` | `NoSuchConfigurationAggregatorException` | Deletes the specified configuration aggregator and the aggregated data associated with the aggregator. |
| `DeleteConfigurationRecorder` | `-` | - | `ConfigurationRecorderName` | - | `Unit` | `NoSuchConfigurationRecorderException`, `UnmodifiableEntityException` | Deletes the customer managed configuration recorder. This operation does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using ... |
| `DeleteConformancePack` | `-` | - | `ConformancePackName` | - | `Unit` | `NoSuchConformancePackException`, `ResourceInUseException` | Deletes the specified conformance pack and all the Config rules, remediation actions, and all evaluation results within that conformance pack. Config sets the conformance pack to DELETE_IN_PROGRESS until the deletion ... |
| `DeleteDeliveryChannel` | `-` | - | `DeliveryChannelName` | - | `Unit` | `LastDeliveryChannelDeleteFailedException`, `NoSuchDeliveryChannelException` | Deletes the delivery channel. Before you can delete the delivery channel, you must stop the customer managed configuration recorder. You can use the StopConfigurationRecorder operation to stop the customer managed co ... |
| `DeleteEvaluationResults` | `-` | - | `ConfigRuleName` | - | `DeleteEvaluationResultsResponse` | `NoSuchConfigRuleException`, `ResourceInUseException` | Deletes the evaluation results for the specified Config rule. You can specify one Config rule per request. After you delete the evaluation results, you can call the StartConfigRulesEvaluation API to start evaluating ... |
| `DeleteOrganizationConfigRule` | `-` | - | `OrganizationConfigRuleName` | - | `Unit` | `NoSuchOrganizationConfigRuleException`, `OrganizationAccessDeniedException`, `ResourceInUseException` | Deletes the specified organization Config rule and all of its evaluation results from all member accounts in that organization. Only a management account and a delegated administrator account can delete an organizati ... |
| `DeleteOrganizationConformancePack` | `-` | - | `OrganizationConformancePackName` | - | `Unit` | `NoSuchOrganizationConformancePackException`, `OrganizationAccessDeniedException`, `ResourceInUseException` | Deletes the specified organization conformance pack and all of the Config rules and remediation actions from all member accounts in that organization. Only a management account or a delegated administrator account ca ... |
| `DeletePendingAggregationRequest` | `-` | - | `RequesterAccountId`, `RequesterAwsRegion` | - | `Unit` | `InvalidParameterValueException` | Deletes pending authorization requests for a specified aggregator account in a specified region. |
| `DeleteRemediationConfiguration` | `-` | - | `ConfigRuleName` | - | `DeleteRemediationConfigurationResponse` | `InsufficientPermissionsException`, `InvalidParameterValueException`, `NoSuchRemediationConfigurationException`, `RemediationInProgressException` | Deletes the remediation configuration. |
| `DeleteRemediationExceptions` | `-` | - | `ConfigRuleName`, `ResourceKeys` | - | `DeleteRemediationExceptionsResponse` | `NoSuchRemediationExceptionException` | Deletes one or more remediation exceptions mentioned in the resource keys. Config generates a remediation exception when a problem occurs executing a remediation action to a specific resource. Remediation exceptions ... |
| `DeleteResourceConfig` | `-` | - | `ResourceType`, `ResourceId` | - | `Unit` | `NoRunningConfigurationRecorderException`, `ValidationException` | Records the configuration state for a custom resource that has been deleted. This API records a new ConfigurationItem with a ResourceDeleted status. You can retrieve the ConfigurationItems recorded for this resource ... |
| `DeleteRetentionConfiguration` | `-` | - | `RetentionConfigurationName` | - | `Unit` | `InvalidParameterValueException`, `NoSuchRetentionConfigurationException` | Deletes the retention configuration. |
| `DeleteServiceLinkedConfigurationRecorder` | `-` | - | `ServicePrincipal` | - | `DeleteServiceLinkedConfigurationRecorderResponse` | `ConflictException`, `NoSuchConfigurationRecorderException`, `ValidationException` | Deletes an existing service-linked configuration recorder. This operation does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by ... |
| `DeleteStoredQuery` | `-` | - | `QueryName` | - | `DeleteStoredQueryResponse` | `ResourceNotFoundException`, `ValidationException` | Deletes the stored query for a single Amazon Web Services account and a single Amazon Web Services Region. |
| `DeliverConfigSnapshot` | `-` | - | `deliveryChannelName` | - | `DeliverConfigSnapshotResponse` | `NoAvailableConfigurationRecorderException`, `NoRunningConfigurationRecorderException`, `NoSuchDeliveryChannelException` | Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, Config sends the following notifications using an Amazon SNS topic that you ha ... |
| `DescribeAggregateComplianceByConfigRules` | `-` | `paginated` | `ConfigurationAggregatorName` | - | `DescribeAggregateComplianceByConfigRulesResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchConfigurationAggregatorException`, `ValidationException` | Returns a list of compliant and noncompliant rules with the number of resources for compliant and noncompliant rules. Does not display rules that do not have compliance results. The results can return an empty result ... |
| `DescribeAggregateComplianceByConformancePacks` | `-` | `paginated` | `ConfigurationAggregatorName` | - | `DescribeAggregateComplianceByConformancePacksResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchConfigurationAggregatorException`, `ValidationException` | Returns a list of the existing and deleted conformance packs and their associated compliance status with the count of compliant and noncompliant Config rules within each conformance pack. Also returns the total rule ... |
| `DescribeAggregationAuthorizations` | `-` | `paginated` | - | - | `DescribeAggregationAuthorizationsResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidParameterValueException` | Returns a list of authorizations granted to various aggregator accounts and regions. |
| `DescribeComplianceByConfigRule` | `-` | `paginated` | - | - | `DescribeComplianceByConfigRuleResponse` | `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchConfigRuleException` | Indicates whether the specified Config rules are compliant. If a rule is noncompliant, this operation returns the number of Amazon Web Services resources that do not comply with the rule. A rule is compliant if all o ... |
| `DescribeComplianceByResource` | `-` | `paginated` | - | - | `DescribeComplianceByResourceResponse` | `InvalidNextTokenException`, `InvalidParameterValueException` | Indicates whether the specified Amazon Web Services resources are compliant. If a resource is noncompliant, this operation returns the number of Config rules that the resource does not comply with. A resource is comp ... |
| `DescribeConfigRuleEvaluationStatus` | `-` | `paginated` | - | - | `DescribeConfigRuleEvaluationStatusResponse` | `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchConfigRuleException` | Returns status information for each of your Config managed rules. The status includes information such as the last time Config invoked the rule, the last time Config failed to invoke the rule, and the related error f ... |
| `DescribeConfigRules` | `-` | `paginated` | - | - | `DescribeConfigRulesResponse` | `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchConfigRuleException` | Returns details about your Config rules. |
| `DescribeConfigurationAggregators` | `-` | `paginated` | - | - | `DescribeConfigurationAggregatorsResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchConfigurationAggregatorException` | Returns the details of one or more configuration aggregators. If the configuration aggregator is not specified, this operation returns the details for all the configuration aggregators associated with the account. |
| `DescribeConfigurationAggregatorSourcesStatus` | `-` | `paginated` | `ConfigurationAggregatorName` | - | `DescribeConfigurationAggregatorSourcesStatusResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchConfigurationAggregatorException` | Returns status information for sources within an aggregator. The status includes information about the last time Config verified authorization between the source account and an aggregator account. In case of a failur ... |
| `DescribeConfigurationRecorders` | `-` | - | - | - | `DescribeConfigurationRecordersResponse` | `NoSuchConfigurationRecorderException`, `ValidationException` | Returns details for the configuration recorder you specify. If a configuration recorder is not specified, this operation returns details for the customer managed configuration recorder configured for the account, if ... |
| `DescribeConfigurationRecorderStatus` | `-` | - | - | - | `DescribeConfigurationRecorderStatusResponse` | `NoSuchConfigurationRecorderException`, `ValidationException` | Returns the current status of the configuration recorder you specify as well as the status of the last recording event for the configuration recorders. For a detailed status of recording events over time, add your Co ... |
| `DescribeConformancePackCompliance` | `-` | `paginated` | `ConformancePackName` | - | `DescribeConformancePackComplianceResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchConfigRuleInConformancePackException`, `NoSuchConformancePackException` | Returns compliance details for each rule in that conformance pack. You must provide exact rule names. |
| `DescribeConformancePacks` | `-` | `paginated` | - | - | `DescribeConformancePacksResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchConformancePackException` | Returns a list of one or more conformance packs. |
| `DescribeConformancePackStatus` | `-` | `paginated` | - | - | `DescribeConformancePackStatusResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidParameterValueException` | Provides one or more conformance packs deployment status. If there are no conformance packs then you will see an empty result. |
| `DescribeDeliveryChannels` | `-` | - | - | - | `DescribeDeliveryChannelsResponse` | `NoSuchDeliveryChannelException` | Returns details about the specified delivery channel. If a delivery channel is not specified, this operation returns the details of all delivery channels associated with the account. Currently, you can specify only o ... |
| `DescribeDeliveryChannelStatus` | `-` | - | - | - | `DescribeDeliveryChannelStatusResponse` | `NoSuchDeliveryChannelException` | Returns the current status of the specified delivery channel. If a delivery channel is not specified, this operation returns the current status of all delivery channels associated with the account. Currently, you can ... |
| `DescribeOrganizationConfigRules` | `-` | `paginated` | - | - | `DescribeOrganizationConfigRulesResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchOrganizationConfigRuleException`, `OrganizationAccessDeniedException` | Returns a list of organization Config rules. When you specify the limit and the next token, you receive a paginated response. Limit and next token are not applicable if you specify organization Config rule names. It ... |
| `DescribeOrganizationConfigRuleStatuses` | `-` | `paginated` | - | - | `DescribeOrganizationConfigRuleStatusesResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchOrganizationConfigRuleException`, `OrganizationAccessDeniedException` | Provides organization Config rule deployment status for an organization. The status is not considered successful until organization Config rule is successfully deployed in all the member accounts with an exception of ... |
| `DescribeOrganizationConformancePacks` | `-` | `paginated` | - | - | `DescribeOrganizationConformancePacksResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchOrganizationConformancePackException`, `OrganizationAccessDeniedException` | Returns a list of organization conformance packs. When you specify the limit and the next token, you receive a paginated response. Limit and next token are not applicable if you specify organization conformance packs ... |
| `DescribeOrganizationConformancePackStatuses` | `-` | `paginated` | - | - | `DescribeOrganizationConformancePackStatusesResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchOrganizationConformancePackException`, `OrganizationAccessDeniedException` | Provides organization conformance pack deployment status for an organization. The status is not considered successful until organization conformance pack is successfully deployed in all the member accounts with an ex ... |
| `DescribePendingAggregationRequests` | `-` | `paginated` | - | - | `DescribePendingAggregationRequestsResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidParameterValueException` | Returns a list of all pending aggregation requests. |
| `DescribeRemediationConfigurations` | `-` | - | `ConfigRuleNames` | - | `DescribeRemediationConfigurationsResponse` | - | Returns the details of one or more remediation configurations. |
| `DescribeRemediationExceptions` | `-` | `paginated` | `ConfigRuleName` | - | `DescribeRemediationExceptionsResponse` | `InvalidNextTokenException`, `InvalidParameterValueException` | Returns the details of one or more remediation exceptions. A detailed view of a remediation exception for a set of resources that includes an explanation of an exception and the time when the exception will be delete ... |
| `DescribeRemediationExecutionStatus` | `-` | `paginated` | `ConfigRuleName` | - | `DescribeRemediationExecutionStatusResponse` | `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchRemediationConfigurationException` | Provides a detailed view of a Remediation Execution for a set of resources including state, timestamps for when steps for the remediation execution occur, and any error messages for steps that have failed. When you s ... |
| `DescribeRetentionConfigurations` | `-` | `paginated` | - | - | `DescribeRetentionConfigurationsResponse` | `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchRetentionConfigurationException` | Returns the details of one or more retention configurations. If the retention configuration name is not specified, this operation returns the details for all the retention configurations for that account. Currently, ... |
| `DisassociateResourceTypes` | `-` | - | `ConfigurationRecorderArn`, `ResourceTypes` | - | `DisassociateResourceTypesResponse` | `ConflictException`, `NoSuchConfigurationRecorderException`, `ValidationException` | Removes all resource types specified in the ResourceTypes list from the RecordingGroup of configuration recorder and excludes these resource types when recording. For this operation, the configuration recorder must u ... |
| `GetAggregateComplianceDetailsByConfigRule` | `-` | `paginated` | `ConfigurationAggregatorName`, `ConfigRuleName`, `AccountId`, `AwsRegion` | - | `GetAggregateComplianceDetailsByConfigRuleResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchConfigurationAggregatorException`, `ValidationException` | Returns the evaluation results for the specified Config rule for a specific resource in a rule. The results indicate which Amazon Web Services resources were evaluated by the rule, when each resource was last evaluat ... |
| `GetAggregateConfigRuleComplianceSummary` | `-` | `paginated` | `ConfigurationAggregatorName` | - | `GetAggregateConfigRuleComplianceSummaryResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchConfigurationAggregatorException`, `ValidationException` | Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator. The results can return an empty result page, but if you have a nextToken, the results are displayed on the ... |
| `GetAggregateConformancePackComplianceSummary` | `-` | `paginated` | `ConfigurationAggregatorName` | - | `GetAggregateConformancePackComplianceSummaryResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchConfigurationAggregatorException`, `ValidationException` | Returns the count of compliant and noncompliant conformance packs across all Amazon Web Services accounts and Amazon Web Services Regions in an aggregator. You can filter based on Amazon Web Services account ID or Am ... |
| `GetAggregateDiscoveredResourceCounts` | `-` | `paginated` | `ConfigurationAggregatorName` | - | `GetAggregateDiscoveredResourceCountsResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchConfigurationAggregatorException`, `ValidationException` | Returns the resource counts across accounts and regions that are present in your Config aggregator. You can request the resource counts by providing filters and GroupByKey. For example, if the input contains accountI ... |
| `GetAggregateResourceConfig` | `-` | - | `ConfigurationAggregatorName`, `ResourceIdentifier` | - | `GetAggregateResourceConfigResponse` | `NoSuchConfigurationAggregatorException`, `OversizedConfigurationItemException`, `ResourceNotDiscoveredException`, `ValidationException` | Returns configuration item that is aggregated for your specific resource in a specific source account and region. The API does not return results for deleted resources. |
| `GetComplianceDetailsByConfigRule` | `-` | `paginated` | `ConfigRuleName` | - | `GetComplianceDetailsByConfigRuleResponse` | `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchConfigRuleException` | Returns the evaluation results for the specified Config rule. The results indicate which Amazon Web Services resources were evaluated by the rule, when each resource was last evaluated, and whether each resource comp ... |
| `GetComplianceDetailsByResource` | `-` | `paginated` | - | - | `GetComplianceDetailsByResourceResponse` | `InvalidParameterValueException` | Returns the evaluation results for the specified Amazon Web Services resource. The results indicate which Config rules were used to evaluate the resource, when each rule was last invoked, and whether the resource com ... |
| `GetComplianceSummaryByConfigRule` | `-` | - | - | - | `GetComplianceSummaryByConfigRuleResponse` | - | Returns the number of Config rules that are compliant and noncompliant, up to a maximum of 25 for each. |
| `GetComplianceSummaryByResourceType` | `-` | - | - | - | `GetComplianceSummaryByResourceTypeResponse` | `InvalidParameterValueException` | Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100. |
| `GetConformancePackComplianceDetails` | `-` | `paginated` | `ConformancePackName` | - | `GetConformancePackComplianceDetailsResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchConfigRuleInConformancePackException`, `NoSuchConformancePackException` | Returns compliance details of a conformance pack for all Amazon Web Services resources that are monitered by conformance pack. |
| `GetConformancePackComplianceSummary` | `-` | `paginated` | `ConformancePackNames` | - | `GetConformancePackComplianceSummaryResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchConformancePackException` | Returns compliance details for the conformance pack based on the cumulative compliance results of all the rules in that conformance pack. |
| `GetCustomRulePolicy` | `-` | - | - | - | `GetCustomRulePolicyResponse` | `NoSuchConfigRuleException` | Returns the policy definition containing the logic for your Config Custom Policy rule. |
| `GetDiscoveredResourceCounts` | `-` | `paginated` | - | - | `GetDiscoveredResourceCountsResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `ValidationException` | Returns the resource types, the number of each resource type, and the total number of resources that Config is recording in this region for your Amazon Web Services account. Example Config is recording three resource ... |
| `GetOrganizationConfigRuleDetailedStatus` | `-` | `paginated` | `OrganizationConfigRuleName` | - | `GetOrganizationConfigRuleDetailedStatusResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchOrganizationConfigRuleException`, `OrganizationAccessDeniedException` | Returns detailed status for each member account within an organization for a given organization Config rule. |
| `GetOrganizationConformancePackDetailedStatus` | `-` | `paginated` | `OrganizationConformancePackName` | - | `GetOrganizationConformancePackDetailedStatusResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchOrganizationConformancePackException`, `OrganizationAccessDeniedException` | Returns detailed status for each member account within an organization for a given organization conformance pack. |
| `GetOrganizationCustomRulePolicy` | `-` | - | `OrganizationConfigRuleName` | - | `GetOrganizationCustomRulePolicyResponse` | `NoSuchOrganizationConfigRuleException`, `OrganizationAccessDeniedException` | Returns the policy definition containing the logic for your organization Config Custom Policy rule. |
| `GetResourceConfigHistory` | `-` | `paginated` | `resourceType`, `resourceId` | - | `GetResourceConfigHistoryResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidTimeRangeException`, `NoAvailableConfigurationRecorderException`, `ResourceNotDiscoveredException`, `ValidationException` | For accurate reporting on the compliance status, you must record the AWS::Config::ResourceCompliance resource type. For more information, see Recording Amazon Web Services Resources in the Config Resources Developer ... |
| `GetResourceEvaluationSummary` | `-` | - | `ResourceEvaluationId` | - | `GetResourceEvaluationSummaryResponse` | `ResourceNotFoundException` | Returns a summary of resource evaluation for the specified resource evaluation ID from the proactive rules that were run. The results indicate which evaluation context was used to evaluate the rules, which resource d ... |
| `GetStoredQuery` | `-` | - | `QueryName` | - | `GetStoredQueryResponse` | `ResourceNotFoundException`, `ValidationException` | Returns the details of a specific stored query. |
| `ListAggregateDiscoveredResources` | `-` | `paginated` | `ConfigurationAggregatorName`, `ResourceType` | - | `ListAggregateDiscoveredResourcesResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchConfigurationAggregatorException`, `ValidationException` | Accepts a resource type and returns a list of resource identifiers that are aggregated for a specific resource type across accounts and regions. A resource identifier includes the resource type, ID, (if available) th ... |
| `ListConfigurationRecorders` | `-` | `paginated` | - | - | `ListConfigurationRecordersResponse` | `ValidationException` | Returns a list of configuration recorders depending on the filters you specify. |
| `ListConformancePackComplianceScores` | `-` | `paginated` | - | - | `ListConformancePackComplianceScoresResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `InvalidParameterValueException` | Returns a list of conformance pack compliance scores. A compliance score is the percentage of the number of compliant rule-resource combinations in a conformance pack compared to the number of total possible rule-res ... |
| `ListDiscoveredResources` | `-` | `paginated` | `resourceType` | - | `ListDiscoveredResourcesResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `NoAvailableConfigurationRecorderException`, `ValidationException` | Returns a list of resource resource identifiers for the specified resource types for the resources of that type. A resource identifier includes the resource type, ID, and (if available) the custom resource name. The ... |
| `ListResourceEvaluations` | `-` | `paginated` | - | - | `ListResourceEvaluationsResponse` | `InvalidNextTokenException`, `InvalidParameterValueException`, `InvalidTimeRangeException` | Returns a list of proactive resource evaluations. |
| `ListStoredQueries` | `-` | `paginated` | - | - | `ListStoredQueriesResponse` | `InvalidNextTokenException`, `ValidationException` | Lists the stored queries for a single Amazon Web Services account and a single Amazon Web Services Region. The default is 100. |
| `ListTagsForResource` | `-` | `paginated` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InvalidLimitException`, `InvalidNextTokenException`, `ResourceNotFoundException`, `ValidationException` | List the tags for Config resource. |
| `PutAggregationAuthorization` | `-` | - | `AuthorizedAccountId`, `AuthorizedAwsRegion` | - | `PutAggregationAuthorizationResponse` | `InvalidParameterValueException` | Authorizes the aggregator account and region to collect data from the source account and region. Tags are added at creation and cannot be updated with this operation PutAggregationAuthorization is an idempotent API. ... |
| `PutConfigRule` | `-` | - | `ConfigRule` | - | `Unit` | `InsufficientPermissionsException`, `InvalidParameterValueException`, `MaxNumberOfConfigRulesExceededException`, `NoAvailableConfigurationRecorderException`, `ResourceInUseException` | Adds or updates an Config rule to evaluate if your Amazon Web Services resources comply with your desired configurations. For information on how many Config rules you can have per account, see Service Limits in the C ... |
| `PutConfigurationAggregator` | `-` | - | `ConfigurationAggregatorName` | - | `PutConfigurationAggregatorResponse` | `InvalidParameterValueException`, `InvalidRoleException`, `LimitExceededException`, `NoAvailableOrganizationException`, `OrganizationAccessDeniedException`, `OrganizationAllFeaturesNotEnabledException` | Creates and updates the configuration aggregator with the selected source accounts and regions. The source account can be individual account(s) or an organization. accountIds that are passed will be replaced with exi ... |
| `PutConfigurationRecorder` | `-` | - | `ConfigurationRecorder` | - | `Unit` | `InvalidConfigurationRecorderNameException`, `InvalidRecordingGroupException`, `InvalidRoleException`, `MaxNumberOfConfigurationRecordersExceededException`, `UnmodifiableEntityException`, `ValidationException` | Creates or updates the customer managed configuration recorder. You can use this operation to create a new customer managed configuration recorder or to update the roleARN and the recordingGroup for an existing custo ... |
| `PutConformancePack` | `-` | - | `ConformancePackName` | - | `PutConformancePackResponse` | `ConformancePackTemplateValidationException`, `InsufficientPermissionsException`, `InvalidParameterValueException`, `MaxNumberOfConformancePacksExceededException`, `ResourceInUseException` | Creates or updates a conformance pack. A conformance pack is a collection of Config rules that can be easily deployed in an account and a region and across an organization. For information on how many conformance pac ... |
| `PutDeliveryChannel` | `-` | - | `DeliveryChannel` | - | `Unit` | `InsufficientDeliveryPolicyException`, `InvalidDeliveryChannelNameException`, `InvalidS3KeyPrefixException`, `InvalidS3KmsKeyArnException`, `InvalidSNSTopicARNException`, `MaxNumberOfDeliveryChannelsExceededException`, `NoAvailableConfigurationRecorderException`, `NoSuchBucketException` | Creates or updates a delivery channel to deliver configuration information and other compliance information. You can use this operation to create a new delivery channel or to update the Amazon S3 bucket and the Amazo ... |
| `PutEvaluations` | `-` | - | `ResultToken` | - | `PutEvaluationsResponse` | `InvalidParameterValueException`, `InvalidResultTokenException`, `NoSuchConfigRuleException` | Used by an Lambda function to deliver evaluation results to Config. This operation is required in every Lambda function that is invoked by an Config rule. |
| `PutExternalEvaluation` | `-` | - | `ConfigRuleName`, `ExternalEvaluation` | - | `PutExternalEvaluationResponse` | `InvalidParameterValueException`, `NoSuchConfigRuleException` | Add or updates the evaluations for process checks. This API checks if the rule is a process check when the name of the Config rule is provided. |
| `PutOrganizationConfigRule` | `-` | - | `OrganizationConfigRuleName` | - | `PutOrganizationConfigRuleResponse` | `InsufficientPermissionsException`, `InvalidParameterValueException`, `MaxNumberOfOrganizationConfigRulesExceededException`, `NoAvailableOrganizationException`, `OrganizationAccessDeniedException`, `OrganizationAllFeaturesNotEnabledException`, `ResourceInUseException`, `ValidationException` | Adds or updates an Config rule for your entire organization to evaluate if your Amazon Web Services resources comply with your desired configurations. For information on how many organization Config rules you can hav ... |
| `PutOrganizationConformancePack` | `-` | - | `OrganizationConformancePackName` | - | `PutOrganizationConformancePackResponse` | `InsufficientPermissionsException`, `MaxNumberOfOrganizationConformancePacksExceededException`, `NoAvailableOrganizationException`, `OrganizationAccessDeniedException`, `OrganizationAllFeaturesNotEnabledException`, `OrganizationConformancePackTemplateValidationException`, `ResourceInUseException`, `ValidationException` | Deploys conformance packs across member accounts in an Amazon Web Services Organization. For information on how many organization conformance packs and how many Config rules you can have per account, see Service Limi ... |
| `PutRemediationConfigurations` | `-` | - | `RemediationConfigurations` | - | `PutRemediationConfigurationsResponse` | `InsufficientPermissionsException`, `InvalidParameterValueException` | Adds or updates the remediation configuration with a specific Config rule with the selected target or action. The API creates the RemediationConfiguration object for the Config rule. The Config rule must already exis ... |
| `PutRemediationExceptions` | `-` | - | `ConfigRuleName`, `ResourceKeys` | - | `PutRemediationExceptionsResponse` | `InsufficientPermissionsException`, `InvalidParameterValueException` | A remediation exception is when a specified resource is no longer considered for auto-remediation. This API adds a new exception or updates an existing exception for a specified resource with a specified Config rule. ... |
| `PutResourceConfig` | `-` | - | `ResourceType`, `SchemaVersionId`, `ResourceId`, `Configuration` | - | `Unit` | `InsufficientPermissionsException`, `MaxActiveResourcesExceededException`, `NoRunningConfigurationRecorderException`, `ValidationException` | Records the configuration state for the resource provided in the request. The configuration state of a resource is represented in Config as Configuration Items. Once this API records the configuration item, you can r ... |
| `PutRetentionConfiguration` | `-` | - | `RetentionPeriodInDays` | - | `PutRetentionConfigurationResponse` | `InvalidParameterValueException`, `MaxNumberOfRetentionConfigurationsExceededException` | Creates and updates the retention configuration with details about retention period (number of days) that Config stores your historical information. The API creates the RetentionConfiguration object and names the obj ... |
| `PutServiceLinkedConfigurationRecorder` | `-` | - | `ServicePrincipal` | - | `PutServiceLinkedConfigurationRecorderResponse` | `ConflictException`, `InsufficientPermissionsException`, `LimitExceededException`, `ValidationException` | Creates a service-linked configuration recorder that is linked to a specific Amazon Web Services service based on the ServicePrincipal you specify. The configuration recorder's name , recordingGroup , recordingMode , ... |
| `PutStoredQuery` | `-` | - | `StoredQuery` | - | `PutStoredQueryResponse` | `ResourceConcurrentModificationException`, `TooManyTagsException`, `ValidationException` | Saves a new query or updates an existing saved query. The QueryName must be unique for a single Amazon Web Services account and a single Amazon Web Services Region. You can create upto 300 queries in a single Amazon ... |
| `SelectAggregateResourceConfig` | `-` | `paginated` | `Expression`, `ConfigurationAggregatorName` | - | `SelectAggregateResourceConfigResponse` | `InvalidExpressionException`, `InvalidLimitException`, `InvalidNextTokenException`, `NoSuchConfigurationAggregatorException` | Accepts a structured query language (SQL) SELECT command and an aggregator to query configuration state of Amazon Web Services resources across multiple accounts and regions, performs the corresponding search, and re ... |
| `SelectResourceConfig` | `-` | `paginated` | `Expression` | - | `SelectResourceConfigResponse` | `InvalidExpressionException`, `InvalidLimitException`, `InvalidNextTokenException` | Accepts a structured query language (SQL) SELECT command, performs the corresponding search, and returns resource configurations matching the properties. For more information about query components, see the Query Com ... |
| `StartConfigRulesEvaluation` | `-` | - | - | - | `StartConfigRulesEvaluationResponse` | `InvalidParameterValueException`, `LimitExceededException`, `NoSuchConfigRuleException`, `ResourceInUseException` | Runs an on-demand evaluation for the specified Config rules against the last known configuration state of the resources. Use StartConfigRulesEvaluation when you want to test that a rule you updated is working as expe ... |
| `StartConfigurationRecorder` | `-` | - | `ConfigurationRecorderName` | - | `Unit` | `NoAvailableDeliveryChannelException`, `NoSuchConfigurationRecorderException`, `UnmodifiableEntityException` | Starts the customer managed configuration recorder. The customer managed configuration recorder will begin recording configuration changes for the resource types you specify. You must have created a delivery channel ... |
| `StartRemediationExecution` | `-` | - | `ConfigRuleName`, `ResourceKeys` | - | `StartRemediationExecutionResponse` | `InsufficientPermissionsException`, `InvalidParameterValueException`, `NoSuchRemediationConfigurationException` | Runs an on-demand remediation for the specified Config rules against the last known remediation configuration. It runs an execution against the current state of your resources. Remediation execution is asynchronous. ... |
| `StartResourceEvaluation` | `-` | - | `ResourceDetails`, `EvaluationMode` | - | `StartResourceEvaluationResponse` | `IdempotentParameterMismatch`, `InvalidParameterValueException` | Runs an on-demand evaluation for the specified resource to determine whether the resource details will comply with configured Config rules. You can also use it for evaluation purposes. Config recommends using an eval ... |
| `StopConfigurationRecorder` | `-` | - | `ConfigurationRecorderName` | - | `Unit` | `NoSuchConfigurationRecorderException`, `UnmodifiableEntityException` | Stops the customer managed configuration recorder. The customer managed configuration recorder will stop recording configuration changes for the resource types you have specified. |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `Unit` | `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Associates the specified tags to a resource with the specified ResourceArn . If existing tags on a resource are not specified in the request parameters, they are not changed. If existing tags are specified, however, ... |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `Unit` | `ResourceNotFoundException`, `ValidationException` | Deletes specified tags from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConflictException` | `structure` | message | For PutServiceLinkedConfigurationRecorder , you cannot create a service-linked recorder because a service-linked recorder already exists for the specified s ... |
| `ConformancePackTemplateValidationException` | `structure` | message | You have specified a template that is not valid or supported. |
| `IdempotentParameterMismatch` | `structure` | message | Using the same client token with one or more different parameters. Specify a new client token with the parameter changes and try again. |
| `InsufficientDeliveryPolicyException` | `structure` | message | Your Amazon S3 bucket policy does not allow Config to write to it. |
| `InsufficientPermissionsException` | `structure` | message | Indicates one of the following errors: For PutConfigRule , the rule cannot be created because the IAM role assigned to Config lacks permissions to perform t ... |
| `InvalidConfigurationRecorderNameException` | `structure` | message | The configuration recorder name is not valid. The prefix " AWSConfigurationRecorderFor " is reserved for service-linked configuration recorders. |
| `InvalidDeliveryChannelNameException` | `structure` | message | The specified delivery channel name is not valid. |
| `InvalidExpressionException` | `structure` | message | The syntax of the query is incorrect. |
| `InvalidLimitException` | `structure` | message | The specified limit is outside the allowable range. |
| `InvalidNextTokenException` | `structure` | message | The specified next token is not valid. Specify the nextToken string that was returned in the previous response to get the next page of results. |
| `InvalidParameterValueException` | `structure` | message | One or more of the specified parameters are not valid. Verify that your parameters are valid and try again. |
| `InvalidRecordingGroupException` | `structure` | message | One of the following errors: You have provided a combination of parameter values that is not valid. For example: Setting the allSupported field of Recording ... |
| `InvalidResultTokenException` | `structure` | message | The specified ResultToken is not valid. |
| `InvalidRoleException` | `structure` | message | You have provided a null or empty Amazon Resource Name (ARN) for the IAM role assumed by Config and used by the customer managed configuration recorder. |
| `InvalidS3KeyPrefixException` | `structure` | message | The specified Amazon S3 key prefix is not valid. |
| `InvalidS3KmsKeyArnException` | `structure` | message | The specified Amazon KMS Key ARN is not valid. |
| `InvalidSNSTopicARNException` | `structure` | message | The specified Amazon SNS topic does not exist. |
| `InvalidTimeRangeException` | `structure` | message | The specified time range is not valid. The earlier time is not chronologically before the later time. |
| `LastDeliveryChannelDeleteFailedException` | `structure` | message | You cannot delete the delivery channel you specified because the customer managed configuration recorder is running. |
| `LimitExceededException` | `structure` | message | For PutServiceLinkedConfigurationRecorder API, this exception is thrown if the number of service-linked roles in the account exceeds the limit. For StartCon ... |
| `MaxActiveResourcesExceededException` | `structure` | message | You have reached the limit of active custom resource types in your account. There is a limit of 100,000. Delete unused resources using DeleteResourceConfig . |
| `MaxNumberOfConfigRulesExceededException` | `structure` | message | Failed to add the Config rule because the account already contains the maximum number of 1000 rules. Consider deleting any deactivated rules before you add ... |
| `MaxNumberOfConfigurationRecordersExceededException` | `structure` | message | You have reached the limit of the number of configuration recorders you can create. |
| `MaxNumberOfConformancePacksExceededException` | `structure` | message | You have reached the limit of the number of conformance packs you can create in an account. For more information, see Service Limits in the Config Developer ... |
| `MaxNumberOfDeliveryChannelsExceededException` | `structure` | message | You have reached the limit of the number of delivery channels you can create. |
| `MaxNumberOfOrganizationConfigRulesExceededException` | `structure` | message | You have reached the limit of the number of organization Config rules you can create. For more information, see see Service Limits in the Config Developer G ... |
| `MaxNumberOfOrganizationConformancePacksExceededException` | `structure` | message | You have reached the limit of the number of organization conformance packs you can create in an account. For more information, see Service Limits in the Con ... |
| `MaxNumberOfRetentionConfigurationsExceededException` | `structure` | message | Failed to add the retention configuration because a retention configuration with that name already exists. |
| `NoAvailableConfigurationRecorderException` | `structure` | message | There are no customer managed configuration recorders available to record your resources. Use the PutConfigurationRecorder operation to create the customer ... |
| `NoAvailableDeliveryChannelException` | `structure` | message | There is no delivery channel available to record configurations. |
| `NoAvailableOrganizationException` | `structure` | message | Organization is no longer available. |
| `NoRunningConfigurationRecorderException` | `structure` | message | There is no configuration recorder running. |
| `NoSuchBucketException` | `structure` | message | The specified Amazon S3 bucket does not exist. |
| `NoSuchConfigRuleException` | `structure` | message | The Config rule in the request is not valid. Verify that the rule is an Config Process Check rule, that the rule name is correct, and that valid Amazon Reso ... |
| `NoSuchConfigRuleInConformancePackException` | `structure` | message | Config rule that you passed in the filter does not exist. |
| `NoSuchConfigurationAggregatorException` | `structure` | message | You have specified a configuration aggregator that does not exist. |
| `NoSuchConfigurationRecorderException` | `structure` | message | You have specified a configuration recorder that does not exist. |
| `NoSuchConformancePackException` | `structure` | message | You specified one or more conformance packs that do not exist. |
| `NoSuchDeliveryChannelException` | `structure` | message | You have specified a delivery channel that does not exist. |
| `NoSuchOrganizationConfigRuleException` | `structure` | message | The Config rule in the request is not valid. Verify that the rule is an organization Config Process Check rule, that the rule name is correct, and that vali ... |
| `NoSuchOrganizationConformancePackException` | `structure` | message | Config organization conformance pack that you passed in the filter does not exist. For DeleteOrganizationConformancePack, you tried to delete an organizatio ... |
| `NoSuchRemediationConfigurationException` | `structure` | message | You specified an Config rule without a remediation configuration. |
| `NoSuchRemediationExceptionException` | `structure` | message | You tried to delete a remediation exception that does not exist. |
| `NoSuchRetentionConfigurationException` | `structure` | message | You have specified a retention configuration that does not exist. |
| `OrganizationAccessDeniedException` | `structure` | message | For PutConfigurationAggregator API, you can see this exception for the following reasons: No permission to call EnableAWSServiceAccess API The configuration ... |
| `OrganizationAllFeaturesNotEnabledException` | `structure` | message | Config resource cannot be created because your organization does not have all features enabled. |
| `OrganizationConformancePackTemplateValidationException` | `structure` | message | You have specified a template that is not valid or supported. |
| `OversizedConfigurationItemException` | `structure` | message | The configuration item size is outside the allowable range. |
| `RemediationInProgressException` | `structure` | message | Remediation action is in progress. You can either cancel execution in Amazon Web Services Systems Manager or wait and try again later. |
| `ResourceConcurrentModificationException` | `structure` | message | Two users are trying to modify the same query at the same time. Wait for a moment and try again. |
| `ResourceInUseException` | `structure` | message | You see this exception in the following cases: For DeleteConfigRule, Config is deleting this rule. Try your request again later. For DeleteConfigRule, the r ... |
| `ResourceNotDiscoveredException` | `structure` | message | You have specified a resource that is either unknown or has not been discovered. |
| `ResourceNotFoundException` | `structure` | message | You have specified a resource that does not exist. |
| `TooManyTagsException` | `structure` | message | You have reached the limit of the number of tags you can use. For more information, see Service Limits in the Config Developer Guide . |
| `UnmodifiableEntityException` | `structure` | message | The requested operation is not valid. For PutConfigurationRecorder , you will see this exception because you cannot use this operation to create a service-l ... |
| `ValidationException` | `structure` | message | The requested operation is not valid. You will see this exception if there are missing required fields or if the input value fails the validation. For PutSt ... |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
