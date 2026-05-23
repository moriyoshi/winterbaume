# AWS Device Farm

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the AWS Device Farm API documentation, which contains APIs for: Testing on desktop browsers Device Farm makes it possible for you to test your web applications on desktop browsers using Selenium. The APIs for desktop browser testing contain `TestGrid` in their names. For more information, see Testing Web Applications on Selenium with Device Farm. Testing on real mobile devices Device Farm makes it possible for you to test apps on physical phones, tablets, and other devices in the cloud. For more information, see the Device Farm Developer Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Device Farm by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS Device Farm resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Device Farm workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListArtifacts`, `ListDeviceInstances`, `ListDevicePools`, `ListDevices`, `GetAccountSettings`, `GetDevice`.

## Service Identity and Protocol

- AWS model slug: `device-farm`
- AWS SDK for Rust slug: `devicefarm`
- Model version: `2015-06-23`
- Model file: `vendor/api-models-aws/models/device-farm/service/2015-06-23/device-farm-2015-06-23.json`
- SDK ID: `Device Farm`
- Endpoint prefix: `devicefarm`
- ARN namespace: `devicefarm`
- CloudFormation name: `DeviceFarm`
- CloudTrail event source: `devicefarm.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (24), `Get` (18), `Create` (9), `Delete` (9), `Update` (8), `Stop` (3), `Install` (1), `Purchase` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDevicePool`, `CreateInstanceProfile`, `CreateNetworkProfile`, `CreateProject`, `CreateRemoteAccessSession`, `CreateTestGridProject`, `CreateTestGridUrl`, `CreateUpload`, `CreateVPCEConfiguration`, `DeleteDevicePool`, `DeleteInstanceProfile`, `DeleteNetworkProfile`, `DeleteProject`, `DeleteRemoteAccessSession`, `DeleteRun`, `DeleteTestGridProject`, `DeleteUpload`, `DeleteVPCEConfiguration`, `StopJob`, `StopRemoteAccessSession`, ... (+11).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountSettings`, `GetDevice`, `GetDeviceInstance`, `GetDevicePool`, `GetDevicePoolCompatibility`, `GetInstanceProfile`, `GetJob`, `GetNetworkProfile`, `GetOfferingStatus`, `GetProject`, `GetRemoteAccessSession`, `GetRun`, `GetSuite`, `GetTest`, `GetTestGridProject`, `GetTestGridSession`, `GetUpload`, `GetVPCEConfiguration`, `ListArtifacts`, `ListDeviceInstances`, ... (+22).
- Pagination is modelled for 18 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetJob`, `ListJobs`, `StopJob`, `StopRemoteAccessSession`, `StopRun`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 77 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/devicefarm/latest/developerguide/welcome.html
- https://docs.aws.amazon.com/devicefarm/latest/developerguide/getting-started.html
- https://docs.aws.amazon.com/devicefarm/latest/developerguide/how-to-create-test-run.html

Research outcomes:
- Device Farm tests Android, iOS, and web applications on real devices and desktop browsers.
- Core resources include projects, uploads, device pools, runs, jobs, suites, tests, artefacts, and VPCE configurations.
- Test runs require uploaded application/test packages, selected device pools, test type, and optional custom test specs or environments.
- Runs execute across selected devices and produce results, logs, screenshots, videos, and other artefacts.
- Device Farm supports remote access sessions and automated app testing.
- VPC endpoint configuration can provide private access to services under test.

Parity implications:
- Model projects, uploads, device pools, runs, jobs, artefacts, remote access sessions, and VPCE configurations separately.
- ScheduleRun should validate upload status and create asynchronous run/job state.
- Run results should aggregate per-device job/test outcomes and artefacts.

## Operation Groups

### List

- Operations: `ListArtifacts`, `ListDeviceInstances`, `ListDevicePools`, `ListDevices`, `ListInstanceProfiles`, `ListJobs`, `ListNetworkProfiles`, `ListOfferingPromotions`, `ListOfferings`, `ListOfferingTransactions`, `ListProjects`, `ListRemoteAccessSessions`, `ListRuns`, `ListSamples`, `ListSuites`, `ListTagsForResource`, `ListTestGridProjects`, `ListTestGridSessionActions`, `ListTestGridSessionArtifacts`, `ListTestGridSessions`, `ListTests`, `ListUniqueProblems`, `ListUploads`, `ListVPCEConfigurations`
- Traits: `paginated` (17)
- Common required input members in this group: `arn`, `sessionArn`

### Get

- Operations: `GetAccountSettings`, `GetDevice`, `GetDeviceInstance`, `GetDevicePool`, `GetDevicePoolCompatibility`, `GetInstanceProfile`, `GetJob`, `GetNetworkProfile`, `GetOfferingStatus`, `GetProject`, `GetRemoteAccessSession`, `GetRun`, `GetSuite`, `GetTest`, `GetTestGridProject`, `GetTestGridSession`, `GetUpload`, `GetVPCEConfiguration`
- Traits: `paginated` (1)
- Common required input members in this group: `arn`

### Create

- Operations: `CreateDevicePool`, `CreateInstanceProfile`, `CreateNetworkProfile`, `CreateProject`, `CreateRemoteAccessSession`, `CreateTestGridProject`, `CreateTestGridUrl`, `CreateUpload`, `CreateVPCEConfiguration`
- Common required input members in this group: `projectArn`, `name`

### Delete

- Operations: `DeleteDevicePool`, `DeleteInstanceProfile`, `DeleteNetworkProfile`, `DeleteProject`, `DeleteRemoteAccessSession`, `DeleteRun`, `DeleteTestGridProject`, `DeleteUpload`, `DeleteVPCEConfiguration`
- Common required input members in this group: `arn`

### Update

- Operations: `UpdateDeviceInstance`, `UpdateDevicePool`, `UpdateInstanceProfile`, `UpdateNetworkProfile`, `UpdateProject`, `UpdateTestGridProject`, `UpdateUpload`, `UpdateVPCEConfiguration`
- Common required input members in this group: `arn`

### Stop

- Operations: `StopJob`, `StopRemoteAccessSession`, `StopRun`
- Common required input members in this group: `arn`

### Install

- Operations: `InstallToRemoteAccessSession`
- Common required input members in this group: -

### Purchase

- Operations: `PurchaseOffering`
- Common required input members in this group: -

### Renew

- Operations: `RenewOffering`
- Common required input members in this group: -

### Schedule

- Operations: `ScheduleRun`
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
| `CreateDevicePool` | `-` | - | `projectArn`, `name`, `rules` | - | `CreateDevicePoolResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Creates a device pool. |
| `CreateInstanceProfile` | `-` | - | `name` | - | `CreateInstanceProfileResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Creates a profile that can be applied to one or more private fleet device instances. |
| `CreateNetworkProfile` | `-` | - | `projectArn`, `name` | - | `CreateNetworkProfileResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Creates a network profile. |
| `CreateProject` | `-` | - | `name` | - | `CreateProjectResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException`, `TagOperationException` | Creates a project. |
| `CreateRemoteAccessSession` | `-` | - | `projectArn`, `deviceArn` | - | `CreateRemoteAccessSessionResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Specifies and starts a remote access session. |
| `CreateTestGridProject` | `-` | - | `name` | - | `CreateTestGridProjectResult` | `ArgumentException`, `InternalServiceException`, `LimitExceededException` | Creates a Selenium testing project. Projects are used to track TestGridSession instances. |
| `CreateTestGridUrl` | `-` | - | `projectArn`, `expiresInSeconds` | - | `CreateTestGridUrlResult` | `ArgumentException`, `InternalServiceException`, `NotFoundException` | Creates a signed, short-term URL that can be passed to a Selenium RemoteWebDriver constructor. |
| `CreateUpload` | `-` | - | `projectArn`, `name`, `type` | - | `CreateUploadResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Uploads an app or test scripts. |
| `CreateVPCEConfiguration` | `-` | - | `vpceConfigurationName`, `vpceServiceName`, `serviceDnsName` | - | `CreateVPCEConfigurationResult` | `ArgumentException`, `LimitExceededException`, `ServiceAccountException` | Creates a configuration record in Device Farm for your Amazon Virtual Private Cloud (VPC) endpoint. |
| `DeleteDevicePool` | `-` | - | `arn` | - | `DeleteDevicePoolResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Deletes a device pool given the pool ARN. Does not allow deletion of curated pools owned by the system. |
| `DeleteInstanceProfile` | `-` | - | `arn` | - | `DeleteInstanceProfileResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Deletes a profile that can be applied to one or more private device instances. |
| `DeleteNetworkProfile` | `-` | - | `arn` | - | `DeleteNetworkProfileResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Deletes a network profile. |
| `DeleteProject` | `-` | - | `arn` | - | `DeleteProjectResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Deletes an AWS Device Farm project, given the project ARN. You cannot delete a project if it has an active run or session. You cannot undo this operation. |
| `DeleteRemoteAccessSession` | `-` | - | `arn` | - | `DeleteRemoteAccessSessionResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Deletes a completed remote access session and its results. You cannot delete a remote access session if it is still active. You cannot undo this operation. |
| `DeleteRun` | `-` | - | `arn` | - | `DeleteRunResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Deletes the run, given the run ARN. You cannot delete a run if it is still active. You cannot undo this operation. |
| `DeleteTestGridProject` | `-` | - | `projectArn` | - | `DeleteTestGridProjectResult` | `ArgumentException`, `CannotDeleteException`, `InternalServiceException`, `NotFoundException` | Deletes a Selenium testing project and all content generated under it. You cannot delete a project if it has active sessions. You cannot undo this operation. |
| `DeleteUpload` | `-` | - | `arn` | - | `DeleteUploadResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Deletes an upload given the upload ARN. |
| `DeleteVPCEConfiguration` | `-` | - | `arn` | - | `DeleteVPCEConfigurationResult` | `ArgumentException`, `InvalidOperationException`, `NotFoundException`, `ServiceAccountException` | Deletes a configuration for your Amazon Virtual Private Cloud (VPC) endpoint. |
| `GetAccountSettings` | `-` | - | - | - | `GetAccountSettingsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Returns the number of unmetered iOS or unmetered Android devices that have been purchased by the account. |
| `GetDevice` | `-` | - | `arn` | - | `GetDeviceResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about a unique device type. |
| `GetDeviceInstance` | `-` | - | `arn` | - | `GetDeviceInstanceResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Returns information about a device instance that belongs to a private device fleet. |
| `GetDevicePool` | `-` | - | `arn` | - | `GetDevicePoolResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about a device pool. |
| `GetDevicePoolCompatibility` | `-` | - | `devicePoolArn` | - | `GetDevicePoolCompatibilityResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about compatibility with a device pool. |
| `GetInstanceProfile` | `-` | - | `arn` | - | `GetInstanceProfileResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Returns information about the specified instance profile. |
| `GetJob` | `-` | - | `arn` | - | `GetJobResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about a job. |
| `GetNetworkProfile` | `-` | - | `arn` | - | `GetNetworkProfileResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Returns information about a network profile. |
| `GetOfferingStatus` | `-` | `paginated` | - | - | `GetOfferingStatusResult` | `ArgumentException`, `LimitExceededException`, `NotEligibleException`, `NotFoundException`, `ServiceAccountException` | Gets the current status and future status of all offerings purchased by an AWS account. The response indicates how many offerings are currently available and the offerings that will be available in the next period. T ... |
| `GetProject` | `-` | - | `arn` | - | `GetProjectResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about a project. |
| `GetRemoteAccessSession` | `-` | - | `arn` | - | `GetRemoteAccessSessionResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Returns a link to a currently running remote access session. |
| `GetRun` | `-` | - | `arn` | - | `GetRunResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about a run. |
| `GetSuite` | `-` | - | `arn` | - | `GetSuiteResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about a suite. |
| `GetTest` | `-` | - | `arn` | - | `GetTestResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about a test. |
| `GetTestGridProject` | `-` | - | `projectArn` | - | `GetTestGridProjectResult` | `ArgumentException`, `InternalServiceException`, `NotFoundException` | Retrieves information about a Selenium testing project. |
| `GetTestGridSession` | `-` | - | - | - | `GetTestGridSessionResult` | `ArgumentException`, `InternalServiceException`, `NotFoundException` | A session is an instance of a browser created through a RemoteWebDriver with the URL from CreateTestGridUrlResult$url . You can use the following to look up sessions: The session ARN ( GetTestGridSessionRequest$sessi ... |
| `GetUpload` | `-` | - | `arn` | - | `GetUploadResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about an upload. |
| `GetVPCEConfiguration` | `-` | - | `arn` | - | `GetVPCEConfigurationResult` | `ArgumentException`, `NotFoundException`, `ServiceAccountException` | Returns information about the configuration settings for your Amazon Virtual Private Cloud (VPC) endpoint. |
| `InstallToRemoteAccessSession` | `-` | - | `remoteAccessSessionArn`, `appArn` | - | `InstallToRemoteAccessSessionResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Installs an application to the device in a remote access session. For Android applications, the file must be in .apk format. For iOS applications, the file must be in .ipa format. |
| `ListArtifacts` | `-` | `paginated` | `arn`, `type` | - | `ListArtifactsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about artifacts. |
| `ListDeviceInstances` | `-` | - | - | - | `ListDeviceInstancesResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Returns information about the private device instances associated with one or more AWS accounts. |
| `ListDevicePools` | `-` | `paginated` | `arn` | - | `ListDevicePoolsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about device pools. |
| `ListDevices` | `-` | `paginated` | - | - | `ListDevicesResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about unique device types. |
| `ListInstanceProfiles` | `-` | - | - | - | `ListInstanceProfilesResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Returns information about all the instance profiles in an AWS account. |
| `ListJobs` | `-` | `paginated` | `arn` | - | `ListJobsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about jobs for a given test run. |
| `ListNetworkProfiles` | `-` | - | `arn` | - | `ListNetworkProfilesResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Returns the list of available network profiles. |
| `ListOfferingPromotions` | `-` | - | - | - | `ListOfferingPromotionsResult` | `ArgumentException`, `LimitExceededException`, `NotEligibleException`, `NotFoundException`, `ServiceAccountException` | Returns a list of offering promotions. Each offering promotion record contains the ID and description of the promotion. The API returns a NotEligible error if the caller is not permitted to invoke the operation. Cont ... |
| `ListOfferings` | `-` | `paginated` | - | - | `ListOfferingsResult` | `ArgumentException`, `LimitExceededException`, `NotEligibleException`, `NotFoundException`, `ServiceAccountException` | Returns a list of products or offerings that the user can manage through the API. Each offering record indicates the recurring price per unit and the frequency for that offering. The API returns a NotEligible error i ... |
| `ListOfferingTransactions` | `-` | `paginated` | - | - | `ListOfferingTransactionsResult` | `ArgumentException`, `LimitExceededException`, `NotEligibleException`, `NotFoundException`, `ServiceAccountException` | Returns a list of all historical purchases, renewals, and system renewal transactions for an AWS account. The list is paginated and ordered by a descending timestamp (most recent transactions are first). The API retu ... |
| `ListProjects` | `-` | `paginated` | - | - | `ListProjectsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about projects. |
| `ListRemoteAccessSessions` | `-` | - | `arn` | - | `ListRemoteAccessSessionsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Returns a list of all currently running remote access sessions. |
| `ListRuns` | `-` | `paginated` | `arn` | - | `ListRunsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about runs, given an AWS Device Farm project ARN. |
| `ListSamples` | `-` | `paginated` | `arn` | - | `ListSamplesResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about samples, given an AWS Device Farm job ARN. |
| `ListSuites` | `-` | `paginated` | `arn` | - | `ListSuitesResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about test suites for a given job. |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `ArgumentException`, `NotFoundException`, `TagOperationException` | List the tags for an AWS Device Farm resource. |
| `ListTestGridProjects` | `-` | `paginated` | - | - | `ListTestGridProjectsResult` | `ArgumentException`, `InternalServiceException` | Gets a list of all Selenium testing projects in your account. |
| `ListTestGridSessionActions` | `-` | `paginated` | `sessionArn` | - | `ListTestGridSessionActionsResult` | `ArgumentException`, `InternalServiceException`, `NotFoundException` | Returns a list of the actions taken in a TestGridSession . |
| `ListTestGridSessionArtifacts` | `-` | `paginated` | `sessionArn` | - | `ListTestGridSessionArtifactsResult` | `ArgumentException`, `InternalServiceException`, `NotFoundException` | Retrieves a list of artifacts created during the session. |
| `ListTestGridSessions` | `-` | `paginated` | `projectArn` | - | `ListTestGridSessionsResult` | `ArgumentException`, `InternalServiceException`, `NotFoundException` | Retrieves a list of sessions for a TestGridProject . |
| `ListTests` | `-` | `paginated` | `arn` | - | `ListTestsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about tests in a given test suite. |
| `ListUniqueProblems` | `-` | `paginated` | `arn` | - | `ListUniqueProblemsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about unique problems, such as exceptions or crashes. Unique problems are defined as a single instance of an error across a run, job, or suite. For example, if a call in your application consistently ... |
| `ListUploads` | `-` | `paginated` | `arn` | - | `ListUploadsResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Gets information about uploads, given an AWS Device Farm project ARN. |
| `ListVPCEConfigurations` | `-` | - | - | - | `ListVPCEConfigurationsResult` | `ArgumentException`, `ServiceAccountException` | Returns information about all Amazon Virtual Private Cloud (VPC) endpoint configurations in the AWS account. |
| `PurchaseOffering` | `-` | - | `offeringId`, `quantity` | - | `PurchaseOfferingResult` | `ArgumentException`, `LimitExceededException`, `NotEligibleException`, `NotFoundException`, `ServiceAccountException` | Immediately purchases offerings for an AWS account. Offerings renew with the latest total purchased quantity for an offering, unless the renewal was overridden. The API returns a NotEligible error if the user is not ... |
| `RenewOffering` | `-` | - | `offeringId`, `quantity` | - | `RenewOfferingResult` | `ArgumentException`, `LimitExceededException`, `NotEligibleException`, `NotFoundException`, `ServiceAccountException` | Explicitly sets the quantity of devices to renew for an offering, starting from the effectiveDate of the next period. The API returns a NotEligible error if the user is not permitted to invoke the operation. If you m ... |
| `ScheduleRun` | `-` | - | `projectArn`, `test` | - | `ScheduleRunResult` | `ArgumentException`, `IdempotencyException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Schedules a run. |
| `StopJob` | `-` | - | `arn` | - | `StopJobResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Initiates a stop request for the current job. AWS Device Farm immediately stops the job on the device where tests have not started. You are not billed for this device. On the device where tests have started, setup su ... |
| `StopRemoteAccessSession` | `-` | - | `arn` | - | `StopRemoteAccessSessionResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Ends a specified remote access session. |
| `StopRun` | `-` | - | `arn` | - | `StopRunResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Initiates a stop request for the current test run. AWS Device Farm immediately stops the run on devices where tests have not started. You are not billed for these devices. On devices where tests have started executin ... |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `ArgumentException`, `NotFoundException`, `TagOperationException`, `TagPolicyException`, `TooManyTagsException` | Associates the specified tags to a resource with the specified resourceArn . If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags asso ... |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ArgumentException`, `NotFoundException`, `TagOperationException` | Deletes the specified tags from a resource. |
| `UpdateDeviceInstance` | `-` | - | `arn` | - | `UpdateDeviceInstanceResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Updates information about a private device instance. |
| `UpdateDevicePool` | `-` | - | `arn` | - | `UpdateDevicePoolResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Modifies the name, description, and rules in a device pool given the attributes and the pool ARN. Rule updates are all-or-nothing, meaning they can only be updated as a whole (or not at all). |
| `UpdateInstanceProfile` | `-` | - | `arn` | - | `UpdateInstanceProfileResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Updates information about an existing private device instance profile. |
| `UpdateNetworkProfile` | `-` | - | `arn` | - | `UpdateNetworkProfileResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Updates the network profile. |
| `UpdateProject` | `-` | - | `arn` | - | `UpdateProjectResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Modifies the specified project name, given the project ARN and a new name. |
| `UpdateTestGridProject` | `-` | - | `projectArn` | - | `UpdateTestGridProjectResult` | `ArgumentException`, `InternalServiceException`, `LimitExceededException`, `NotFoundException` | Change details of a project. |
| `UpdateUpload` | `-` | - | `arn` | - | `UpdateUploadResult` | `ArgumentException`, `LimitExceededException`, `NotFoundException`, `ServiceAccountException` | Updates an uploaded test spec. |
| `UpdateVPCEConfiguration` | `-` | - | `arn` | - | `UpdateVPCEConfigurationResult` | `ArgumentException`, `InvalidOperationException`, `NotFoundException`, `ServiceAccountException` | Updates information about an Amazon Virtual Private Cloud (VPC) endpoint configuration. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ArgumentException` | `structure` | message | An invalid argument was specified. |
| `CannotDeleteException` | `structure` | message | The requested object could not be deleted. |
| `IdempotencyException` | `structure` | message | An entity with the same name already exists. |
| `InternalServiceException` | `structure` | message | An internal exception was raised in the service. Contact aws-devicefarm-support@amazon.com if you see this error. |
| `InvalidOperationException` | `structure` | message | There was an error with the update request, or you do not have sufficient permissions to update this VPC endpoint configuration. |
| `LimitExceededException` | `structure` | message | A limit was exceeded. |
| `NotEligibleException` | `structure` | message | Exception gets thrown when a user is not eligible to perform the specified transaction. |
| `NotFoundException` | `structure` | message | The specified entity was not found. |
| `ServiceAccountException` | `structure` | message | There was a problem with the service account. |
| `TagOperationException` | `structure` | message, resourceName | The operation was not successful. Try again. |
| `TagPolicyException` | `structure` | message, resourceName | The request doesn't comply with the AWS Identity and Access Management (IAM) tag policy. Correct your request and then retry it. |
| `TooManyTagsException` | `structure` | message, resourceName | The list of tags on the repository is over the limit. The maximum number of tags that can be applied to a repository is 50. |
| `CreateDevicePoolRequest` | `structure` | projectArn, name, description, rules, maxDevices | Represents a request to the create device pool operation. |
| `CreateDevicePoolResult` | `structure` | devicePool | Represents the result of a create device pool request. |
| `CreateInstanceProfileRequest` | `structure` | name, description, packageCleanup, excludeAppPackagesFromCleanup, rebootAfterUse | - |
| `CreateInstanceProfileResult` | `structure` | instanceProfile | - |
| `CreateNetworkProfileRequest` | `structure` | projectArn, name, description, type, uplinkBandwidthBits, downlinkBandwidthBits, uplinkDelayMs, downlinkDelayMs, uplinkJitterMs, downlinkJitterMs, uplinkLossPercent, downlinkLossPercent | - |
| `CreateNetworkProfileResult` | `structure` | networkProfile | - |
| `CreateProjectRequest` | `structure` | name, defaultJobTimeoutMinutes, vpcConfig, environmentVariables, executionRoleArn | Represents a request to the create project operation. |
| `CreateProjectResult` | `structure` | project | Represents the result of a create project request. |
| `CreateRemoteAccessSessionRequest` | `structure` | projectArn, deviceArn, appArn, instanceArn, name, configuration, interactionMode, skipAppResign | Creates and submits a request to start a remote access session. |
| `CreateRemoteAccessSessionResult` | `structure` | remoteAccessSession | Represents the server response from a request to create a remote access session. |
| `CreateTestGridProjectRequest` | `structure` | name, description, vpcConfig | - |
| `CreateTestGridProjectResult` | `structure` | testGridProject | - |
| `CreateTestGridUrlRequest` | `structure` | projectArn, expiresInSeconds | - |
| `CreateTestGridUrlResult` | `structure` | url, expires | - |
| `CreateUploadRequest` | `structure` | projectArn, name, type, contentType | Represents a request to the create upload operation. |
| `CreateUploadResult` | `structure` | upload | Represents the result of a create upload request. |
| `CreateVPCEConfigurationRequest` | `structure` | vpceConfigurationName, vpceServiceName, serviceDnsName, vpceConfigurationDescription | - |
| `CreateVPCEConfigurationResult` | `structure` | vpceConfiguration | - |
| `DeleteDevicePoolRequest` | `structure` | arn | Represents a request to the delete device pool operation. |
| `DeleteDevicePoolResult` | `structure` | **empty (no members)** | Represents the result of a delete device pool request. |
| `DeleteInstanceProfileRequest` | `structure` | arn | - |
| `DeleteInstanceProfileResult` | `structure` | **empty (no members)** | - |
| `DeleteNetworkProfileRequest` | `structure` | arn | - |
| `DeleteNetworkProfileResult` | `structure` | **empty (no members)** | - |
| `DeleteProjectRequest` | `structure` | arn | Represents a request to the delete project operation. |
| `DeleteProjectResult` | `structure` | **empty (no members)** | Represents the result of a delete project request. |
| `DeleteRemoteAccessSessionRequest` | `structure` | arn | Represents the request to delete the specified remote access session. |
| `DeleteRemoteAccessSessionResult` | `structure` | **empty (no members)** | The response from the server when a request is made to delete the remote access session. |
| `ArtifactCategory` | `enum` | SCREENSHOT, FILE, LOG | - |
| `ArtifactType` | `enum` | UNKNOWN, SCREENSHOT, DEVICE_LOG, MESSAGE_LOG, VIDEO_LOG, RESULT_LOG, SERVICE_LOG, WEBKIT_LOG, INSTRUMENTATION_OUTPUT, EXERCISER_MONKEY_OUTPUT, CALABASH_JSON_OUTPUT, CALABASH_PRETTY_OUTPUT, ... (+16) | - |
| `BillingMethod` | `enum` | METERED, UNMETERED | - |
| `CurrencyCode` | `enum` | USD | - |
| `DeviceAttribute` | `enum` | ARN, PLATFORM, FORM_FACTOR, MANUFACTURER, REMOTE_ACCESS_ENABLED, REMOTE_DEBUG_ENABLED, APPIUM_VERSION, INSTANCE_ARN, INSTANCE_LABELS, FLEET_TYPE, OS_VERSION, MODEL, ... (+1) | - |
| `DeviceAvailability` | `enum` | TEMPORARY_NOT_AVAILABLE, BUSY, AVAILABLE, HIGHLY_AVAILABLE | - |
| `DeviceFilterAttribute` | `enum` | ARN, PLATFORM, OS_VERSION, MODEL, AVAILABILITY, FORM_FACTOR, MANUFACTURER, REMOTE_ACCESS_ENABLED, REMOTE_DEBUG_ENABLED, INSTANCE_ARN, INSTANCE_LABELS, FLEET_TYPE | - |
| `DeviceFormFactor` | `enum` | PHONE, TABLET | - |
| `DevicePlatform` | `enum` | ANDROID, IOS | - |
| `DevicePoolType` | `enum` | CURATED, PRIVATE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
