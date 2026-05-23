# AWS Elastic Beanstalk

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Elastic Beanstalk AWS Elastic Beanstalk makes it easy for you to create, deploy, and manage scalable, fault-tolerant applications running on the Amazon Web Services cloud. For more information about this product, go to the AWS Elastic Beanstalk details page. The location of the latest AWS Elastic Beanstalk WSDL is https://elasticbeanstalk.s3.amazonaws.com/doc/2010-12-01/AWSElasticBeanstalk.wsdl. To install the Software Development Kits (SDKs), Integrated Development Environment (IDE) Toolkits, and command line tools that enable you to access the API, go to Tools for Amazon Web Services. Endpoints For a list of region-specific endpoints that AWS Elastic Beanstalk supports, go to Regions and Endpoints in the Amazon Web Services Glossary .

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Elastic Beanstalk where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Elastic Beanstalk by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS Elastic Beanstalk by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Elastic Beanstalk workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Create`, `Update`, `Delete`, `List` operation families, including `DescribeAccountAttributes`, `DescribeApplicationVersions`, `DescribeApplications`, `DescribeConfigurationOptions`, `CreateApplication`, `CreateApplicationVersion`.

## Service Identity and Protocol

- AWS model slug: `elastic-beanstalk`
- AWS SDK for Rust slug: `elasticbeanstalk`
- Model version: `2010-12-01`
- Model file: `vendor/api-models-aws/models/elastic-beanstalk/service/2010-12-01/elastic-beanstalk-2010-12-01.json`
- SDK ID: `Elastic Beanstalk`
- Endpoint prefix: `elasticbeanstalk`
- ARN namespace: `elasticbeanstalk`
- CloudFormation name: `ElasticBeanstalk`
- CloudTrail event source: `elasticbeanstalk.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (13), `Create` (6), `Update` (6), `Delete` (5), `List` (4), `Abort` (1), `Apply` (1), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateEnvironmentOperationsRole`, `CreateApplication`, `CreateApplicationVersion`, `CreateConfigurationTemplate`, `CreateEnvironment`, `CreatePlatformVersion`, `CreateStorageLocation`, `DeleteApplication`, `DeleteApplicationVersion`, `DeleteConfigurationTemplate`, `DeleteEnvironmentConfiguration`, `DeletePlatformVersion`, `DisassociateEnvironmentOperationsRole`, `TerminateEnvironment`, `UpdateApplication`, `UpdateApplicationResourceLifecycle`, `UpdateApplicationVersion`, `UpdateConfigurationTemplate`, `UpdateEnvironment`, `UpdateTagsForResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `CheckDNSAvailability`, `DescribeAccountAttributes`, `DescribeApplicationVersions`, `DescribeApplications`, `DescribeConfigurationOptions`, `DescribeConfigurationSettings`, `DescribeEnvironmentHealth`, `DescribeEnvironmentManagedActionHistory`, `DescribeEnvironmentManagedActions`, `DescribeEnvironmentResources`, `DescribeEnvironments`, `DescribeEvents`, `DescribeInstancesHealth`, `DescribePlatformVersion`, `ListAvailableSolutionStacks`, `ListPlatformBranches`, `ListPlatformVersions`, `ListTagsForResource`, `ValidateConfigurationSettings`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 33 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SQS`, `EC2/VPC`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/Welcome.html
- https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/vpc.html
- https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/create_deploy_docker.container.console.html

Research outcomes:
- Elastic Beanstalk deploys applications by provisioning and managing resources such as EC2 instances, load balancers, Auto Scaling groups, security groups, and environments.
- Core resources include applications, application versions, environments, platforms, configurations, saved configurations, and environment resources.
- Application deployment creates or updates an environment to run a selected application version.
- Platforms define runtime stacks such as Docker, Java, Python, Node.js, Ruby, Go, and Windows.
- Environment configuration includes VPC/subnet settings, environment variables, managed updates, logging, health reporting, and scaling.
- Enhanced health and logs depend on environment configuration and platform support.

Parity implications:
- Model applications, versions, environments, platform branches, configuration templates, events, and underlying resource references separately.
- Environment create/update/deploy should be asynchronous and health-state driven.
- Application versions should be immutable deployment artefacts referenced by environment updates.

## Operation Groups

### Describe

- Operations: `DescribeAccountAttributes`, `DescribeApplications`, `DescribeApplicationVersions`, `DescribeConfigurationOptions`, `DescribeConfigurationSettings`, `DescribeEnvironmentHealth`, `DescribeEnvironmentManagedActionHistory`, `DescribeEnvironmentManagedActions`, `DescribeEnvironmentResources`, `DescribeEnvironments`, `DescribeEvents`, `DescribeInstancesHealth`, `DescribePlatformVersion`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Create

- Operations: `CreateApplication`, `CreateApplicationVersion`, `CreateConfigurationTemplate`, `CreateEnvironment`, `CreatePlatformVersion`, `CreateStorageLocation`
- Common required input members in this group: `ApplicationName`

### Update

- Operations: `UpdateApplication`, `UpdateApplicationResourceLifecycle`, `UpdateApplicationVersion`, `UpdateConfigurationTemplate`, `UpdateEnvironment`, `UpdateTagsForResource`
- Common required input members in this group: `ApplicationName`

### Delete

- Operations: `DeleteApplication`, `DeleteApplicationVersion`, `DeleteConfigurationTemplate`, `DeleteEnvironmentConfiguration`, `DeletePlatformVersion`
- Common required input members in this group: `ApplicationName`

### List

- Operations: `ListAvailableSolutionStacks`, `ListPlatformBranches`, `ListPlatformVersions`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Abort

- Operations: `AbortEnvironmentUpdate`
- Common required input members in this group: -

### Apply

- Operations: `ApplyEnvironmentManagedAction`
- Common required input members in this group: -

### Associate

- Operations: `AssociateEnvironmentOperationsRole`
- Common required input members in this group: -

### Check

- Operations: `CheckDNSAvailability`
- Common required input members in this group: -

### Compose

- Operations: `ComposeEnvironments`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateEnvironmentOperationsRole`
- Common required input members in this group: -

### Rebuild

- Operations: `RebuildEnvironment`
- Common required input members in this group: -

### Request

- Operations: `RequestEnvironmentInfo`
- Common required input members in this group: -

### Restart

- Operations: `RestartAppServer`
- Common required input members in this group: -

### Retrieve

- Operations: `RetrieveEnvironmentInfo`
- Common required input members in this group: -

### Swap

- Operations: `SwapEnvironmentCNAMEs`
- Common required input members in this group: -

### Terminate

- Operations: `TerminateEnvironment`
- Common required input members in this group: -

### Validate

- Operations: `ValidateConfigurationSettings`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AbortEnvironmentUpdate` | `-` | - | - | - | `Unit` | `InsufficientPrivilegesException` | Cancels in-progress environment configuration update or application version deployment. |
| `ApplyEnvironmentManagedAction` | `-` | - | `ActionId` | - | `ApplyEnvironmentManagedActionResult` | `ElasticBeanstalkServiceException`, `ManagedActionInvalidStateException` | Applies a scheduled managed action immediately. A managed action can be applied only if its status is Scheduled . Get the status and action ID of a managed action with DescribeEnvironmentManagedActions . |
| `AssociateEnvironmentOperationsRole` | `-` | - | `EnvironmentName`, `OperationsRole` | - | `Unit` | `InsufficientPrivilegesException` | Add or change the operations role used by an environment. After this call is made, Elastic Beanstalk uses the associated operations role for permissions to downstream services during subsequent calls acting on this e ... |
| `CheckDNSAvailability` | `-` | - | `CNAMEPrefix` | - | `CheckDNSAvailabilityResultMessage` | - | Checks if the specified CNAME is available. |
| `ComposeEnvironments` | `-` | - | - | - | `EnvironmentDescriptionsMessage` | `InsufficientPrivilegesException`, `TooManyEnvironmentsException` | Create or update a group of environments that each run a separate component of a single application. Takes a list of version labels that specify application source bundles for each of the environments to create or up ... |
| `CreateApplication` | `-` | - | `ApplicationName` | - | `ApplicationDescriptionMessage` | `TooManyApplicationsException` | Creates an application that has one configuration template named default and no application versions. |
| `CreateApplicationVersion` | `-` | - | `ApplicationName`, `VersionLabel` | - | `ApplicationVersionDescriptionMessage` | `CodeBuildNotInServiceRegionException`, `InsufficientPrivilegesException`, `S3LocationNotInServiceRegionException`, `TooManyApplicationsException`, `TooManyApplicationVersionsException` | Creates an application version for the specified application. You can create an application version from a source bundle in Amazon S3, a commit in AWS CodeCommit, or the output of an AWS CodeBuild build as follows: S ... |
| `CreateConfigurationTemplate` | `-` | - | `ApplicationName`, `TemplateName` | - | `ConfigurationSettingsDescription` | `InsufficientPrivilegesException`, `TooManyBucketsException`, `TooManyConfigurationTemplatesException` | Creates an AWS Elastic Beanstalk configuration template, associated with a specific Elastic Beanstalk application. You define application configuration settings in a configuration template. You can then use the confi ... |
| `CreateEnvironment` | `-` | - | `ApplicationName` | - | `EnvironmentDescription` | `InsufficientPrivilegesException`, `TooManyEnvironmentsException` | Launches an AWS Elastic Beanstalk environment for the specified application using the specified configuration. |
| `CreatePlatformVersion` | `-` | - | `PlatformName`, `PlatformVersion`, `PlatformDefinitionBundle` | - | `CreatePlatformVersionResult` | `ElasticBeanstalkServiceException`, `InsufficientPrivilegesException`, `TooManyPlatformsException` | Create a new version of your custom platform. |
| `CreateStorageLocation` | `-` | - | - | - | `CreateStorageLocationResultMessage` | `InsufficientPrivilegesException`, `S3SubscriptionRequiredException`, `TooManyBucketsException` | Creates a bucket in Amazon S3 to store application versions, logs, and other files used by Elastic Beanstalk environments. The Elastic Beanstalk console and EB CLI call this API the first time you create an environme ... |
| `DeleteApplication` | `-` | - | `ApplicationName` | - | `Unit` | `OperationInProgressException` | Deletes the specified application along with all associated versions and configurations. The application versions will not be deleted from your Amazon S3 bucket. You cannot delete an application that has a running en ... |
| `DeleteApplicationVersion` | `-` | - | `ApplicationName`, `VersionLabel` | - | `Unit` | `InsufficientPrivilegesException`, `OperationInProgressException`, `S3LocationNotInServiceRegionException`, `SourceBundleDeletionException` | Deletes the specified version from the specified application. You cannot delete an application version that is associated with a running environment. |
| `DeleteConfigurationTemplate` | `-` | - | `ApplicationName`, `TemplateName` | - | `Unit` | `OperationInProgressException` | Deletes the specified configuration template. When you launch an environment using a configuration template, the environment gets a copy of the template. You can delete or modify the environment's copy of the templat ... |
| `DeleteEnvironmentConfiguration` | `-` | - | `ApplicationName`, `EnvironmentName` | - | `Unit` | - | Deletes the draft configuration associated with the running environment. Updating a running environment with any configuration changes creates a draft configuration set. You can get the draft configuration using Desc ... |
| `DeletePlatformVersion` | `-` | - | - | - | `DeletePlatformVersionResult` | `ElasticBeanstalkServiceException`, `InsufficientPrivilegesException`, `OperationInProgressException`, `PlatformVersionStillReferencedException` | Deletes the specified version of a custom platform. |
| `DescribeAccountAttributes` | `-` | - | - | - | `DescribeAccountAttributesResult` | `InsufficientPrivilegesException` | Returns attributes related to AWS Elastic Beanstalk that are associated with the calling AWS account. The result currently has one set of attributes—resource quotas. |
| `DescribeApplications` | `-` | - | - | - | `ApplicationDescriptionsMessage` | - | Returns the descriptions of existing applications. |
| `DescribeApplicationVersions` | `-` | - | - | - | `ApplicationVersionDescriptionsMessage` | - | Retrieve a list of application versions. |
| `DescribeConfigurationOptions` | `-` | - | - | - | `ConfigurationOptionsDescription` | `TooManyBucketsException` | Describes the configuration options that are used in a particular configuration template or environment, or that a specified solution stack defines. The description includes the values the options, their default valu ... |
| `DescribeConfigurationSettings` | `-` | - | `ApplicationName` | - | `ConfigurationSettingsDescriptions` | `TooManyBucketsException` | Returns a description of the settings for the specified configuration set, that is, either a configuration template or the configuration set associated with a running environment. When describing the settings for the ... |
| `DescribeEnvironmentHealth` | `-` | - | - | - | `DescribeEnvironmentHealthResult` | `ElasticBeanstalkServiceException`, `InvalidRequestException` | Returns information about the overall health of the specified environment. The DescribeEnvironmentHealth operation is only available with AWS Elastic Beanstalk Enhanced Health. |
| `DescribeEnvironmentManagedActionHistory` | `-` | `paginated` | - | - | `DescribeEnvironmentManagedActionHistoryResult` | `ElasticBeanstalkServiceException` | Lists an environment's completed and failed managed actions. |
| `DescribeEnvironmentManagedActions` | `-` | - | - | - | `DescribeEnvironmentManagedActionsResult` | `ElasticBeanstalkServiceException` | Lists an environment's upcoming and in-progress managed actions. |
| `DescribeEnvironmentResources` | `-` | - | - | - | `EnvironmentResourceDescriptionsMessage` | `InsufficientPrivilegesException` | Returns AWS resources for this environment. |
| `DescribeEnvironments` | `-` | - | - | - | `EnvironmentDescriptionsMessage` | - | Returns descriptions for existing environments. |
| `DescribeEvents` | `-` | `paginated` | - | - | `EventDescriptionsMessage` | - | Returns list of event descriptions matching criteria up to the last 6 weeks. This action returns the most recent 1,000 events from the specified NextToken . |
| `DescribeInstancesHealth` | `-` | - | - | - | `DescribeInstancesHealthResult` | `ElasticBeanstalkServiceException`, `InvalidRequestException` | Retrieves detailed information about the health of instances in your AWS Elastic Beanstalk. This operation requires enhanced health reporting . |
| `DescribePlatformVersion` | `-` | - | - | - | `DescribePlatformVersionResult` | `ElasticBeanstalkServiceException`, `InsufficientPrivilegesException` | Describes a platform version. Provides full details. Compare to ListPlatformVersions , which provides summary information about a list of platform versions. For definitions of platform version and other platform-rela ... |
| `DisassociateEnvironmentOperationsRole` | `-` | - | `EnvironmentName` | - | `Unit` | `InsufficientPrivilegesException` | Disassociate the operations role from an environment. After this call is made, Elastic Beanstalk uses the caller's permissions for permissions to downstream services during subsequent calls acting on this environment ... |
| `ListAvailableSolutionStacks` | `-` | - | - | - | `ListAvailableSolutionStacksResultMessage` | - | Returns a list of the available solution stack names, with the public version first and then in reverse chronological order. |
| `ListPlatformBranches` | `-` | `paginated` | - | - | `ListPlatformBranchesResult` | - | Lists the platform branches available for your account in an AWS Region. Provides summary information about each platform branch. For definitions of platform branch and other platform-related terms, see AWS Elastic B ... |
| `ListPlatformVersions` | `-` | `paginated` | - | - | `ListPlatformVersionsResult` | `ElasticBeanstalkServiceException`, `InsufficientPrivilegesException` | Lists the platform versions available for your account in an AWS Region. Provides summary information about each platform version. Compare to DescribePlatformVersion , which provides full details about a single platf ... |
| `ListTagsForResource` | `-` | - | `ResourceArn` | - | `ResourceTagsDescriptionMessage` | `InsufficientPrivilegesException`, `ResourceNotFoundException`, `ResourceTypeNotSupportedException` | Return the tags applied to an AWS Elastic Beanstalk resource. The response contains a list of tag key-value pairs. Elastic Beanstalk supports tagging of all of its resources. For details about resource tagging, see T ... |
| `RebuildEnvironment` | `-` | - | - | - | `Unit` | `InsufficientPrivilegesException` | Deletes and recreates all of the AWS resources (for example: the Auto Scaling group, load balancer, etc.) for a specified environment and forces a restart. |
| `RequestEnvironmentInfo` | `-` | - | `InfoType` | - | `Unit` | - | Initiates a request to compile the specified type of information of the deployed environment. Setting the InfoType to tail compiles the last lines from the application server log files of every Amazon EC2 instance in ... |
| `RestartAppServer` | `-` | - | - | - | `Unit` | - | Causes the environment to restart the application container server running on each Amazon EC2 instance. |
| `RetrieveEnvironmentInfo` | `-` | - | `InfoType` | - | `RetrieveEnvironmentInfoResultMessage` | - | Retrieves the compiled information from a RequestEnvironmentInfo request. Related Topics RequestEnvironmentInfo |
| `SwapEnvironmentCNAMEs` | `-` | - | - | - | `Unit` | - | Swaps the CNAMEs of two environments. |
| `TerminateEnvironment` | `-` | - | - | - | `EnvironmentDescription` | `InsufficientPrivilegesException` | Terminates the specified environment. |
| `UpdateApplication` | `-` | - | `ApplicationName` | - | `ApplicationDescriptionMessage` | - | Updates the specified application to have the specified properties. If a property (for example, description ) is not provided, the value remains unchanged. To clear these properties, specify an empty string. |
| `UpdateApplicationResourceLifecycle` | `-` | - | `ApplicationName`, `ResourceLifecycleConfig` | - | `ApplicationResourceLifecycleDescriptionMessage` | `InsufficientPrivilegesException` | Modifies lifecycle settings for an application. |
| `UpdateApplicationVersion` | `-` | - | `ApplicationName`, `VersionLabel` | - | `ApplicationVersionDescriptionMessage` | - | Updates the specified application version to have the specified properties. If a property (for example, description ) is not provided, the value remains unchanged. To clear properties, specify an empty string. |
| `UpdateConfigurationTemplate` | `-` | - | `ApplicationName`, `TemplateName` | - | `ConfigurationSettingsDescription` | `InsufficientPrivilegesException`, `TooManyBucketsException` | Updates the specified configuration template to have the specified properties or configuration option values. If a property (for example, ApplicationName ) is not provided, its value remains unchanged. To clear such ... |
| `UpdateEnvironment` | `-` | - | - | - | `EnvironmentDescription` | `InsufficientPrivilegesException`, `TooManyBucketsException` | Updates the environment description, deploys a new application version, updates the configuration settings to an entirely new configuration template, or updates select configuration option values in the running envir ... |
| `UpdateTagsForResource` | `-` | - | `ResourceArn` | - | `Unit` | `InsufficientPrivilegesException`, `OperationInProgressException`, `ResourceNotFoundException`, `ResourceTypeNotSupportedException`, `TooManyTagsException` | Update the list of tags applied to an AWS Elastic Beanstalk resource. Two lists can be passed: TagsToAdd for tags to add or update, and TagsToRemove . Elastic Beanstalk supports tagging of all of its resources. For d ... |
| `ValidateConfigurationSettings` | `-` | - | `ApplicationName`, `OptionSettings` | - | `ConfigurationSettingsValidationMessages` | `InsufficientPrivilegesException`, `TooManyBucketsException` | Takes a set of configuration settings and either a configuration template or environment, and determines whether those values are valid. This action returns a list of messages indicating any errors or warnings associ ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CodeBuildNotInServiceRegionException` | `structure` | message | AWS CodeBuild is not available in the specified region. |
| `ElasticBeanstalkServiceException` | `structure` | message | A generic service exception has occurred. |
| `InsufficientPrivilegesException` | `structure` | message | The specified account does not have sufficient privileges for one or more AWS services. |
| `InvalidRequestException` | `structure` | message | One or more input parameters is not valid. Please correct the input parameters and try the operation again. |
| `ManagedActionInvalidStateException` | `structure` | message | Cannot modify the managed action in its current state. |
| `OperationInProgressException` | `structure` | message | Unable to perform the specified operation because another operation that effects an element in this activity is already in progress. |
| `PlatformVersionStillReferencedException` | `structure` | message | You cannot delete the platform version because there are still environments running on it. |
| `ResourceNotFoundException` | `structure` | message | A resource doesn't exist for the specified Amazon Resource Name (ARN). |
| `ResourceTypeNotSupportedException` | `structure` | message | The type of the specified Amazon Resource Name (ARN) isn't supported for this operation. |
| `S3LocationNotInServiceRegionException` | `structure` | message | The specified S3 bucket does not belong to the S3 region in which the service is running. The following regions are supported: IAD/us-east-1 PDX/us-west-2 D ... |
| `S3SubscriptionRequiredException` | `structure` | message | The specified account does not have a subscription to Amazon S3. |
| `SourceBundleDeletionException` | `structure` | message | Unable to delete the Amazon S3 source bundle associated with the application version. The application version was deleted successfully. |
| `TooManyApplicationVersionsException` | `structure` | message | The specified account has reached its limit of application versions. |
| `TooManyApplicationsException` | `structure` | message | The specified account has reached its limit of applications. |
| `TooManyBucketsException` | `structure` | message | The specified account has reached its limit of Amazon S3 buckets. |
| `TooManyConfigurationTemplatesException` | `structure` | message | The specified account has reached its limit of configuration templates. |
| `TooManyEnvironmentsException` | `structure` | message | The specified account has reached its limit of environments. |
| `TooManyPlatformsException` | `structure` | message | You have exceeded the maximum number of allowed platforms associated with the account. |
| `TooManyTagsException` | `structure` | message | The number of tags in the resource would exceed the number of tags that each resource can have. To calculate this, the operation considers both the number o ... |
| `AbortEnvironmentUpdateMessage` | `structure` | EnvironmentId, EnvironmentName | - |
| `ApplyEnvironmentManagedActionRequest` | `structure` | EnvironmentName, EnvironmentId, ActionId | Request to execute a scheduled managed action immediately. |
| `ApplyEnvironmentManagedActionResult` | `structure` | ActionId, ActionDescription, ActionType, Status | The result message containing information about the managed action. |
| `AssociateEnvironmentOperationsRoleMessage` | `structure` | EnvironmentName, OperationsRole | Request to add or change the operations role used by an environment. |
| `CheckDNSAvailabilityMessage` | `structure` | CNAMEPrefix | Results message indicating whether a CNAME is available. |
| `CheckDNSAvailabilityResultMessage` | `structure` | Available, FullyQualifiedCNAME | Indicates if the specified CNAME is available. |
| `ComposeEnvironmentsMessage` | `structure` | ApplicationName, GroupName, VersionLabels | Request to create or update a group of environments. |
| `EnvironmentDescriptionsMessage` | `structure` | Environments, NextToken | Result message containing a list of environment descriptions. |
| `CreateApplicationMessage` | `structure` | ApplicationName, Description, ResourceLifecycleConfig, Tags | Request to create an application. |
| `ApplicationDescriptionMessage` | `structure` | Application | Result message containing a single description of an application. |
| `CreateApplicationVersionMessage` | `structure` | ApplicationName, VersionLabel, Description, SourceBuildInformation, SourceBundle, BuildConfiguration, AutoCreateApplication, Process, Tags | - |
| `ApplicationVersionDescriptionMessage` | `structure` | ApplicationVersion | Result message wrapping a single description of an application version. |
| `CreateConfigurationTemplateMessage` | `structure` | ApplicationName, TemplateName, SolutionStackName, PlatformArn, SourceConfiguration, EnvironmentId, Description, OptionSettings, Tags | Request to create a configuration template. |
| `ConfigurationSettingsDescription` | `structure` | SolutionStackName, PlatformArn, ApplicationName, TemplateName, Description, EnvironmentName, DeploymentStatus, DateCreated, DateUpdated, OptionSettings | Describes the settings for a configuration set. |
| `CreateEnvironmentMessage` | `structure` | ApplicationName, EnvironmentName, GroupName, Description, CNAMEPrefix, Tier, Tags, VersionLabel, TemplateName, SolutionStackName, PlatformArn, OptionSettings, ... (+2) | - |
| `EnvironmentDescription` | `structure` | EnvironmentName, EnvironmentId, ApplicationName, VersionLabel, SolutionStackName, PlatformArn, TemplateName, Description, EndpointURL, CNAME, DateCreated, DateUpdated, ... (+9) | Describes the properties of an environment. |
| `CreatePlatformVersionRequest` | `structure` | PlatformName, PlatformVersion, PlatformDefinitionBundle, EnvironmentName, OptionSettings, Tags | Request to create a new platform version. |
| `CreatePlatformVersionResult` | `structure` | PlatformSummary, Builder | - |
| `CreateStorageLocationResultMessage` | `structure` | S3Bucket | Results of a CreateStorageLocationResult call. |
| `DeleteApplicationMessage` | `structure` | ApplicationName, TerminateEnvByForce | Request to delete an application. |
| `DeleteApplicationVersionMessage` | `structure` | ApplicationName, VersionLabel, DeleteSourceBundle | Request to delete an application version. |
| `ActionHistoryStatus` | `enum` | Completed, Failed, Unknown | - |
| `ActionStatus` | `enum` | Scheduled, Pending, Running, Unknown | - |
| `ActionType` | `enum` | InstanceRefresh, PlatformUpdate, Unknown | - |
| `ApplicationVersionStatus` | `enum` | Processed, Unprocessed, Failed, Processing, Building | - |
| `ComputeType` | `enum` | BUILD_GENERAL1_SMALL, BUILD_GENERAL1_MEDIUM, BUILD_GENERAL1_LARGE | - |
| `ConfigurationDeploymentStatus` | `enum` | deployed, pending, failed | - |
| `ConfigurationOptionValueType` | `enum` | Scalar, List | - |
| `EnvironmentHealth` | `enum` | Green, Yellow, Red, Grey | - |
| `EnvironmentHealthAttribute` | `enum` | Status, Color, Causes, ApplicationMetrics, InstancesHealth, All, HealthStatus, RefreshedAt | - |
| `EnvironmentHealthStatus` | `enum` | NoData, Unknown, Pending, Ok, Info, Warning, Degraded, Severe, Suspended | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
