# Amazon AppStream

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon WorkSpaces Applications This is the Amazon WorkSpaces Applications API Reference . This documentation provides descriptions and syntax for each of the actions and data types in WorkSpaces Applications. WorkSpaces Applications is a fully managed, secure application streaming service that lets you stream desktop applications to users without rewriting applications. WorkSpaces Applications manages the AWS resources that are required to host and run your applications, scales automatically, and provides access to your users on demand. You can call the WorkSpaces Applications API operations by using an interface VPC endpoint (interface endpoint).

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon AppStream where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon AppStream by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon AppStream by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: manage fleets, stacks, images, image builders, users, entitlements, and streaming sessions for application streaming.
- From the operation surface: support desktop/application streaming setup, fleet scaling, directory integration, user-stack association, session lifecycle, and image pipeline workflows.

## Service Identity and Protocol

- AWS model slug: `appstream`
- AWS SDK for Rust slug: `appstream`
- Model version: `2016-12-01`
- Model file: `vendor/api-models-aws/models/appstream/service/2016-12-01/appstream-2016-12-01.json`
- SDK ID: `AppStream`
- Endpoint prefix: `appstream2`
- ARN namespace: `appstream`
- CloudFormation name: `AppStream`
- CloudTrail event source: `appstream.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (19), `Create` (17), `Delete` (13), `Update` (8), `Associate` (5), `Disassociate` (5), `List` (5), `Start` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAppBlockBuilderAppBlock`, `AssociateApplicationFleet`, `AssociateApplicationToEntitlement`, `AssociateFleet`, `AssociateSoftwareToImageBuilder`, `BatchAssociateUserStack`, `BatchDisassociateUserStack`, `CreateAppBlock`, `CreateAppBlockBuilder`, `CreateAppBlockBuilderStreamingURL`, `CreateApplication`, `CreateDirectoryConfig`, `CreateEntitlement`, `CreateExportImageTask`, `CreateFleet`, `CreateImageBuilder`, `CreateImageBuilderStreamingURL`, `CreateImportedImage`, `CreateStack`, `CreateStreamingURL`, ... (+41).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAppBlockBuilderAppBlockAssociations`, `DescribeAppBlockBuilders`, `DescribeAppBlocks`, `DescribeAppLicenseUsage`, `DescribeApplicationFleetAssociations`, `DescribeApplications`, `DescribeDirectoryConfigs`, `DescribeEntitlements`, `DescribeFleets`, `DescribeImageBuilders`, `DescribeImagePermissions`, `DescribeImages`, `DescribeSessions`, `DescribeSoftwareAssociations`, `DescribeStacks`, `DescribeThemeForStack`, `DescribeUsageReportSubscriptions`, `DescribeUserStackAssociations`, `DescribeUsers`, `GetExportImageTask`, ... (+5).
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateExportImageTask`, `CreateImportedImage`, `CreateUsageReportSubscription`, `DeleteUsageReportSubscription`, `DescribeUsageReportSubscriptions`, `GetExportImageTask`, `ListExportImageTasks`, `StartAppBlockBuilder`, `StartFleet`, `StartImageBuilder`, `StartSoftwareDeploymentToImageBuilder`, `StopAppBlockBuilder`, `StopFleet`, `StopImageBuilder`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 85 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/appstream2/latest/developerguide/managing-image-builders-actions.html
- https://docs.aws.amazon.com/appstream2/latest/developerguide/app-blocks-applications.html
- https://docs.aws.amazon.com/appstream2/latest/developerguide/data-loss-prevention.html

Research outcomes:
- AppStream 2.0 streams applications from managed fleet instances to users through stacks and fleet associations.
- Images define the applications and configuration available on fleet instances; image builders are stateful instances used to create and maintain images.
- Image builder actions are constrained by state. Operations such as start, connect, and delete depend on whether the builder is running, stopped, or in an intermediate state.
- Elastic fleets can use app blocks stored in S3 as virtual hard disks to deliver application files.
- Application settings persistence uses S3-backed virtual hard disk files and can use S3 versioning for restoration.
- Session scripts can run during session lifecycle events and can log output to S3.
- Data-loss prevention controls can restrict clipboard copy/paste, file upload, and file download between the client and streaming session.
- Fleets, stacks, users, streaming URLs, image builders, app blocks, and session state are separate resources with different lifecycles.

Parity implications:
- Model fleets, stacks, stack-fleet associations, images, image builders, users, sessions, streaming URLs, app blocks, application settings, and data-transfer policies separately.
- State-dependent actions should reject invalid intermediate-state operations.
- Session creation should depend on stack/fleet state, user assignment, and fleet capacity.

## Operation Groups

### Describe

- Operations: `DescribeAppBlockBuilderAppBlockAssociations`, `DescribeAppBlockBuilders`, `DescribeAppBlocks`, `DescribeApplicationFleetAssociations`, `DescribeApplications`, `DescribeAppLicenseUsage`, `DescribeDirectoryConfigs`, `DescribeEntitlements`, `DescribeFleets`, `DescribeImageBuilders`, `DescribeImagePermissions`, `DescribeImages`, `DescribeSessions`, `DescribeSoftwareAssociations`, `DescribeStacks`, `DescribeThemeForStack`, `DescribeUsageReportSubscriptions`, `DescribeUsers`, `DescribeUserStackAssociations`
- Traits: `paginated` (4)
- Common required input members in this group: `StackName`

### Create

- Operations: `CreateAppBlock`, `CreateAppBlockBuilder`, `CreateAppBlockBuilderStreamingURL`, `CreateApplication`, `CreateDirectoryConfig`, `CreateEntitlement`, `CreateExportImageTask`, `CreateFleet`, `CreateImageBuilder`, `CreateImageBuilderStreamingURL`, `CreateImportedImage`, `CreateStack`, `CreateStreamingURL`, `CreateThemeForStack`, `CreateUpdatedImage`, `CreateUsageReportSubscription`, `CreateUser`
- Common required input members in this group: `Name`, `InstanceType`, `StackName`, `IamRoleArn`

### Delete

- Operations: `DeleteAppBlock`, `DeleteAppBlockBuilder`, `DeleteApplication`, `DeleteDirectoryConfig`, `DeleteEntitlement`, `DeleteFleet`, `DeleteImage`, `DeleteImageBuilder`, `DeleteImagePermissions`, `DeleteStack`, `DeleteThemeForStack`, `DeleteUsageReportSubscription`, `DeleteUser`
- Common required input members in this group: `Name`, `StackName`

### Update

- Operations: `UpdateAppBlockBuilder`, `UpdateApplication`, `UpdateDirectoryConfig`, `UpdateEntitlement`, `UpdateFleet`, `UpdateImagePermissions`, `UpdateStack`, `UpdateThemeForStack`
- Common required input members in this group: `Name`, `StackName`

### Associate

- Operations: `AssociateAppBlockBuilderAppBlock`, `AssociateApplicationFleet`, `AssociateApplicationToEntitlement`, `AssociateFleet`, `AssociateSoftwareToImageBuilder`
- Common required input members in this group: `FleetName`, `StackName`

### Disassociate

- Operations: `DisassociateAppBlockBuilderAppBlock`, `DisassociateApplicationFleet`, `DisassociateApplicationFromEntitlement`, `DisassociateFleet`, `DisassociateSoftwareFromImageBuilder`
- Common required input members in this group: `FleetName`, `StackName`

### List

- Operations: `ListAssociatedFleets`, `ListAssociatedStacks`, `ListEntitledApplications`, `ListExportImageTasks`, `ListTagsForResource`
- Common required input members in this group: `StackName`

### Start

- Operations: `StartAppBlockBuilder`, `StartFleet`, `StartImageBuilder`, `StartSoftwareDeploymentToImageBuilder`
- Common required input members in this group: `Name`

### Stop

- Operations: `StopAppBlockBuilder`, `StopFleet`, `StopImageBuilder`
- Common required input members in this group: `Name`

### Batch

- Operations: `BatchAssociateUserStack`, `BatchDisassociateUserStack`
- Common required input members in this group: `UserStackAssociations`

### Copy

- Operations: `CopyImage`
- Common required input members in this group: -

### Disable

- Operations: `DisableUser`
- Common required input members in this group: -

### Drain

- Operations: `DrainSessionInstance`
- Common required input members in this group: -

### Enable

- Operations: `EnableUser`
- Common required input members in this group: -

### Expire

- Operations: `ExpireSession`
- Common required input members in this group: -

### Get

- Operations: `GetExportImageTask`
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
| `AssociateAppBlockBuilderAppBlock` | `-` | - | `AppBlockArn`, `AppBlockBuilderName` | - | `AssociateAppBlockBuilderAppBlockResult` | `ConcurrentModificationException`, `InvalidParameterCombinationException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Associates the specified app block builder with the specified app block. |
| `AssociateApplicationFleet` | `-` | - | `FleetName`, `ApplicationArn` | - | `AssociateApplicationFleetResult` | `ConcurrentModificationException`, `InvalidParameterCombinationException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Associates the specified application with the specified fleet. This is only supported for Elastic fleets. |
| `AssociateApplicationToEntitlement` | `-` | - | `StackName`, `EntitlementName`, `ApplicationIdentifier` | - | `AssociateApplicationToEntitlementResult` | `EntitlementNotFoundException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Associates an application to entitle. |
| `AssociateFleet` | `-` | - | `FleetName`, `StackName` | - | `AssociateFleetResult` | `ConcurrentModificationException`, `IncompatibleImageException`, `InvalidAccountStatusException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Associates the specified fleet with the specified stack. |
| `AssociateSoftwareToImageBuilder` | `-` | - | `ImageBuilderName`, `SoftwareNames` | - | `AssociateSoftwareToImageBuilderResult` | `ConcurrentModificationException`, `IncompatibleImageException`, `InvalidParameterCombinationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Associates license included application(s) with an existing image builder instance. |
| `BatchAssociateUserStack` | `-` | - | `UserStackAssociations` | - | `BatchAssociateUserStackResult` | `InvalidParameterCombinationException`, `OperationNotPermittedException` | Associates the specified users with the specified stacks. Users in a user pool cannot be assigned to stacks with fleets that are joined to an Active Directory domain. |
| `BatchDisassociateUserStack` | `-` | - | `UserStackAssociations` | - | `BatchDisassociateUserStackResult` | `InvalidParameterCombinationException`, `OperationNotPermittedException` | Disassociates the specified users from the specified stacks. |
| `CopyImage` | `-` | - | `SourceImageName`, `DestinationImageName`, `DestinationRegion` | - | `CopyImageResponse` | `IncompatibleImageException`, `InvalidAccountStatusException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Copies the image within the same region or to a new region within the same AWS account. Note that any tags you added to the image will not be copied. |
| `CreateAppBlock` | `-` | - | `Name`, `SourceS3Location` | - | `CreateAppBlockResult` | `ConcurrentModificationException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceAlreadyExistsException` | Creates an app block. App blocks are a WorkSpaces Applications resource that stores the details about the virtual hard disk in an S3 bucket. It also stores the setup script with details about how to mount the virtual ... |
| `CreateAppBlockBuilder` | `-` | - | `Name`, `Platform`, `InstanceType`, `VpcConfig` | - | `CreateAppBlockBuilderResult` | `ConcurrentModificationException`, `InvalidAccountStatusException`, `InvalidParameterCombinationException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `RequestLimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Creates an app block builder. |
| `CreateAppBlockBuilderStreamingURL` | `-` | - | `AppBlockBuilderName` | - | `CreateAppBlockBuilderStreamingURLResult` | `OperationNotPermittedException`, `ResourceNotFoundException` | Creates a URL to start a create app block builder streaming session. |
| `CreateApplication` | `-` | - | `Name`, `IconS3Location`, `LaunchPath`, `Platforms`, `InstanceFamilies`, `AppBlockArn` | - | `CreateApplicationResult` | `ConcurrentModificationException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates an application. Applications are a WorkSpaces Applications resource that stores the details about how to launch applications on Elastic fleet streaming instances. An application consists of the launch details ... |
| `CreateDirectoryConfig` | `-` | - | `DirectoryName`, `OrganizationalUnitDistinguishedNames` | - | `CreateDirectoryConfigResult` | `InvalidAccountStatusException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates a Directory Config object in WorkSpaces Applications. This object includes the configuration information required to join fleets and image builders to Microsoft Active Directory domains. |
| `CreateEntitlement` | `-` | - | `Name`, `StackName`, `AppVisibility`, `Attributes` | - | `CreateEntitlementResult` | `EntitlementAlreadyExistsException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Creates a new entitlement. Entitlements control access to specific applications within a stack, based on user attributes. Entitlements apply to SAML 2.0 federated user identities. WorkSpaces Applications user pool an ... |
| `CreateExportImageTask` | `-` | - | `ImageName`, `AmiName`, `IamRoleArn` | - | `CreateExportImageTaskResult` | `ConcurrentModificationException`, `InvalidAccountStatusException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Creates a task to export a WorkSpaces Applications image to an EC2 AMI. This allows you to use your customized WorkSpaces Applications images with other AWS services or for backup purposes. |
| `CreateFleet` | `-` | - | `Name`, `InstanceType` | - | `CreateFleetResult` | `ConcurrentModificationException`, `IncompatibleImageException`, `InvalidAccountStatusException`, `InvalidParameterCombinationException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `RequestLimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Creates a fleet. A fleet consists of streaming instances that your users access for their applications and desktops. |
| `CreateImageBuilder` | `-` | - | `Name`, `InstanceType` | - | `CreateImageBuilderResult` | `ConcurrentModificationException`, `IncompatibleImageException`, `InvalidAccountStatusException`, `InvalidParameterCombinationException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `RequestLimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Creates an image builder. An image builder is a virtual machine that is used to create an image. The initial state of the builder is PENDING . When it is ready, the state is RUNNING . |
| `CreateImageBuilderStreamingURL` | `-` | - | `Name` | - | `CreateImageBuilderStreamingURLResult` | `OperationNotPermittedException`, `ResourceNotFoundException` | Creates a URL to start an image builder streaming session. |
| `CreateImportedImage` | `-` | - | `Name`, `SourceAmiId`, `IamRoleArn` | - | `CreateImportedImageResult` | `DryRunOperationException`, `IncompatibleImageException`, `InvalidAccountStatusException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates a custom WorkSpaces Applications image by importing an EC2 AMI. This allows you to use your own customized AMI to create WorkSpaces Applications images that support additional instance types beyond the standa ... |
| `CreateStack` | `-` | - | `Name` | - | `CreateStackResult` | `ConcurrentModificationException`, `InvalidAccountStatusException`, `InvalidParameterCombinationException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates a stack to start streaming applications to users. A stack consists of an associated fleet, user access policies, and storage configurations. |
| `CreateStreamingURL` | `-` | - | `StackName`, `FleetName`, `UserId` | - | `CreateStreamingURLResult` | `InvalidParameterCombinationException`, `OperationNotPermittedException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Creates a temporary URL to start an WorkSpaces Applications streaming session for the specified user. A streaming URL enables application streaming to be tested without user setup. |
| `CreateThemeForStack` | `-` | - | `StackName`, `TitleText`, `ThemeStyling`, `OrganizationLogoS3Location`, `FaviconS3Location` | - | `CreateThemeForStackResult` | `ConcurrentModificationException`, `InvalidAccountStatusException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates custom branding that customizes the appearance of the streaming application catalog page. |
| `CreateUpdatedImage` | `-` | - | `existingImageName`, `newImageName` | - | `CreateUpdatedImageResult` | `ConcurrentModificationException`, `IncompatibleImageException`, `InvalidAccountStatusException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates a new image with the latest Windows operating system updates, driver updates, and WorkSpaces Applications agent software. For more information, see the "Update an Image by Using Managed WorkSpaces Application ... |
| `CreateUsageReportSubscription` | `-` | - | - | - | `CreateUsageReportSubscriptionResult` | `InvalidAccountStatusException`, `InvalidRoleException`, `LimitExceededException` | Creates a usage report subscription. Usage reports are generated daily. |
| `CreateUser` | `-` | - | `UserName`, `AuthenticationType` | - | `CreateUserResult` | `InvalidAccountStatusException`, `InvalidParameterCombinationException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceAlreadyExistsException` | Creates a new user in the user pool. |
| `DeleteAppBlock` | `-` | - | `Name` | - | `DeleteAppBlockResult` | `ConcurrentModificationException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes an app block. |
| `DeleteAppBlockBuilder` | `-` | - | `Name` | - | `DeleteAppBlockBuilderResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes an app block builder. An app block builder can only be deleted when it has no association with an app block. |
| `DeleteApplication` | `-` | - | `Name` | - | `DeleteApplicationResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes an application. |
| `DeleteDirectoryConfig` | `-` | - | `DirectoryName` | - | `DeleteDirectoryConfigResult` | `ResourceInUseException`, `ResourceNotFoundException` | Deletes the specified Directory Config object from WorkSpaces Applications. This object includes the information required to join streaming instances to an Active Directory domain. |
| `DeleteEntitlement` | `-` | - | `Name`, `StackName` | - | `DeleteEntitlementResult` | `ConcurrentModificationException`, `EntitlementNotFoundException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Deletes the specified entitlement. |
| `DeleteFleet` | `-` | - | `Name` | - | `DeleteFleetResult` | `ConcurrentModificationException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes the specified fleet. |
| `DeleteImage` | `-` | - | `Name` | - | `DeleteImageResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes the specified image. You cannot delete an image when it is in use. After you delete an image, you cannot provision new capacity using the image. |
| `DeleteImageBuilder` | `-` | - | `Name` | - | `DeleteImageBuilderResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Deletes the specified image builder and releases the capacity. |
| `DeleteImagePermissions` | `-` | - | `Name`, `SharedAccountId` | - | `DeleteImagePermissionsResult` | `ResourceNotAvailableException`, `ResourceNotFoundException` | Deletes permissions for the specified private image. After you delete permissions for an image, AWS accounts to which you previously granted these permissions can no longer use the image. |
| `DeleteStack` | `-` | - | `Name` | - | `DeleteStackResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes the specified stack. After the stack is deleted, the application streaming environment provided by the stack is no longer available to users. Also, any reservations made for application streaming sessions for ... |
| `DeleteThemeForStack` | `-` | - | `StackName` | - | `DeleteThemeForStackResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Deletes custom branding that customizes the appearance of the streaming application catalog page. |
| `DeleteUsageReportSubscription` | `-` | - | - | - | `DeleteUsageReportSubscriptionResult` | `InvalidAccountStatusException`, `ResourceNotFoundException` | Disables usage report generation. |
| `DeleteUser` | `-` | - | `UserName`, `AuthenticationType` | - | `DeleteUserResult` | `ResourceNotFoundException` | Deletes a user from the user pool. |
| `DescribeAppBlockBuilderAppBlockAssociations` | `-` | `paginated` | - | - | `DescribeAppBlockBuilderAppBlockAssociationsResult` | `InvalidParameterCombinationException`, `OperationNotPermittedException` | Retrieves a list that describes one or more app block builder associations. |
| `DescribeAppBlockBuilders` | `-` | `paginated` | - | - | `DescribeAppBlockBuildersResult` | `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves a list that describes one or more app block builders. |
| `DescribeAppBlocks` | `-` | - | - | - | `DescribeAppBlocksResult` | `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves a list that describes one or more app blocks. |
| `DescribeApplicationFleetAssociations` | `-` | - | - | - | `DescribeApplicationFleetAssociationsResult` | `InvalidParameterCombinationException`, `OperationNotPermittedException` | Retrieves a list that describes one or more application fleet associations. Either ApplicationArn or FleetName must be specified. |
| `DescribeApplications` | `-` | - | - | - | `DescribeApplicationsResult` | `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves a list that describes one or more applications. |
| `DescribeAppLicenseUsage` | `-` | - | `BillingPeriod` | - | `DescribeAppLicenseUsageResult` | `InvalidParameterCombinationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves license included application usage information. |
| `DescribeDirectoryConfigs` | `-` | - | - | - | `DescribeDirectoryConfigsResult` | `ResourceNotFoundException` | Retrieves a list that describes one or more specified Directory Config objects for WorkSpaces Applications, if the names for these objects are provided. Otherwise, all Directory Config objects in the account are desc ... |
| `DescribeEntitlements` | `-` | - | `StackName` | - | `DescribeEntitlementsResult` | `EntitlementNotFoundException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves a list that describes one of more entitlements. |
| `DescribeFleets` | `-` | - | - | - | `DescribeFleetsResult` | `ResourceNotFoundException` | Retrieves a list that describes one or more specified fleets, if the fleet names are provided. Otherwise, all fleets in the account are described. |
| `DescribeImageBuilders` | `-` | - | - | - | `DescribeImageBuildersResult` | `ResourceNotFoundException` | Retrieves a list that describes one or more specified image builders, if the image builder names are provided. Otherwise, all image builders in the account are described. |
| `DescribeImagePermissions` | `-` | `paginated` | `Name` | - | `DescribeImagePermissionsResult` | `ResourceNotFoundException` | Retrieves a list that describes the permissions for shared AWS account IDs on a private image that you own. |
| `DescribeImages` | `-` | `paginated` | - | - | `DescribeImagesResult` | `InvalidParameterCombinationException`, `ResourceNotFoundException` | Retrieves a list that describes one or more specified images, if the image names or image ARNs are provided. Otherwise, all images in the account are described. |
| `DescribeSessions` | `-` | - | `StackName`, `FleetName` | - | `DescribeSessionsResult` | `InvalidParameterCombinationException` | Retrieves a list that describes the streaming sessions for a specified stack and fleet. If a UserId is provided for the stack and fleet, only streaming sessions for that user are described. If an authentication type ... |
| `DescribeSoftwareAssociations` | `-` | - | `AssociatedResource` | - | `DescribeSoftwareAssociationsResult` | `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves license included application associations for a specified resource. |
| `DescribeStacks` | `-` | - | - | - | `DescribeStacksResult` | `ResourceNotFoundException` | Retrieves a list that describes one or more specified stacks, if the stack names are provided. Otherwise, all stacks in the account are described. |
| `DescribeThemeForStack` | `-` | - | `StackName` | - | `DescribeThemeForStackResult` | `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves a list that describes the theme for a specified stack. A theme is custom branding that customizes the appearance of the streaming application catalog page. |
| `DescribeUsageReportSubscriptions` | `-` | - | - | - | `DescribeUsageReportSubscriptionsResult` | `InvalidAccountStatusException`, `ResourceNotFoundException` | Retrieves a list that describes one or more usage report subscriptions. |
| `DescribeUsers` | `-` | - | `AuthenticationType` | - | `DescribeUsersResult` | `InvalidParameterCombinationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves a list that describes one or more specified users in the user pool. |
| `DescribeUserStackAssociations` | `-` | - | - | - | `DescribeUserStackAssociationsResult` | `InvalidParameterCombinationException`, `OperationNotPermittedException` | Retrieves a list that describes the UserStackAssociation objects. You must specify either or both of the following: The stack name The user name (email address of the user associated with the stack) and the authentic ... |
| `DisableUser` | `-` | - | `UserName`, `AuthenticationType` | - | `DisableUserResult` | `ResourceNotFoundException` | Disables the specified user in the user pool. Users can't sign in to WorkSpaces Applications until they are re-enabled. This action does not delete the user. |
| `DisassociateAppBlockBuilderAppBlock` | `-` | - | `AppBlockArn`, `AppBlockBuilderName` | - | `DisassociateAppBlockBuilderAppBlockResult` | `ConcurrentModificationException`, `InvalidParameterCombinationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Disassociates a specified app block builder from a specified app block. |
| `DisassociateApplicationFleet` | `-` | - | `FleetName`, `ApplicationArn` | - | `DisassociateApplicationFleetResult` | `ConcurrentModificationException`, `InvalidParameterCombinationException`, `OperationNotPermittedException` | Disassociates the specified application from the fleet. |
| `DisassociateApplicationFromEntitlement` | `-` | - | `StackName`, `EntitlementName`, `ApplicationIdentifier` | - | `DisassociateApplicationFromEntitlementResult` | `EntitlementNotFoundException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Deletes the specified application from the specified entitlement. |
| `DisassociateFleet` | `-` | - | `FleetName`, `StackName` | - | `DisassociateFleetResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceInUseException`, `ResourceNotFoundException` | Disassociates the specified fleet from the specified stack. |
| `DisassociateSoftwareFromImageBuilder` | `-` | - | `ImageBuilderName`, `SoftwareNames` | - | `DisassociateSoftwareFromImageBuilderResult` | `ConcurrentModificationException`, `InvalidParameterCombinationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Removes license included application(s) association(s) from an image builder instance. |
| `DrainSessionInstance` | `-` | - | `SessionId` | - | `DrainSessionInstanceResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Drains the instance hosting the specified streaming session. The instance stops accepting new sessions while existing sessions continue uninterrupted. Once all sessions end, the instance is reclaimed and replaced. Th ... |
| `EnableUser` | `-` | - | `UserName`, `AuthenticationType` | - | `EnableUserResult` | `InvalidAccountStatusException`, `ResourceNotFoundException` | Enables a user in the user pool. After being enabled, users can sign in to WorkSpaces Applications and open applications from the stacks to which they are assigned. |
| `ExpireSession` | `-` | - | `SessionId` | - | `ExpireSessionResult` | - | Immediately stops the specified streaming session. |
| `GetExportImageTask` | `-` | - | - | - | `GetExportImageTaskResult` | `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves information about an export image task, including its current state, progress, and any error details. |
| `ListAssociatedFleets` | `-` | - | `StackName` | - | `ListAssociatedFleetsResult` | - | Retrieves the name of the fleet that is associated with the specified stack. |
| `ListAssociatedStacks` | `-` | - | `FleetName` | - | `ListAssociatedStacksResult` | - | Retrieves the name of the stack with which the specified fleet is associated. |
| `ListEntitledApplications` | `-` | - | `StackName`, `EntitlementName` | - | `ListEntitledApplicationsResult` | `EntitlementNotFoundException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Retrieves a list of entitled applications. |
| `ListExportImageTasks` | `-` | - | - | - | `ListExportImageTasksResult` | `OperationNotPermittedException` | Lists export image tasks, with optional filtering and pagination. Use this operation to monitor the status of multiple export operations. |
| `ListTagsForResource` | `-` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Retrieves a list of all tags for the specified WorkSpaces Applications resource. You can tag WorkSpaces Applications image builders, images, fleets, and stacks. For more information about tags, see Tagging Your Resou ... |
| `StartAppBlockBuilder` | `-` | - | `Name` | - | `StartAppBlockBuilderResult` | `ConcurrentModificationException`, `InvalidAccountStatusException`, `LimitExceededException`, `OperationNotPermittedException`, `RequestLimitExceededException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Starts an app block builder. An app block builder can only be started when it's associated with an app block. Starting an app block builder starts a new instance, which is equivalent to an elastic fleet instance with ... |
| `StartFleet` | `-` | - | `Name` | - | `StartFleetResult` | `ConcurrentModificationException`, `InvalidAccountStatusException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `RequestLimitExceededException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Starts the specified fleet. |
| `StartImageBuilder` | `-` | - | `Name` | - | `StartImageBuilderResult` | `ConcurrentModificationException`, `IncompatibleImageException`, `InvalidAccountStatusException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Starts the specified image builder. |
| `StartSoftwareDeploymentToImageBuilder` | `-` | - | `ImageBuilderName` | - | `StartSoftwareDeploymentToImageBuilderResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Initiates license included applications deployment to an image builder instance. |
| `StopAppBlockBuilder` | `-` | - | `Name` | - | `StopAppBlockBuilderResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Stops an app block builder. Stopping an app block builder terminates the instance, and the instance state is not persisted. |
| `StopFleet` | `-` | - | `Name` | - | `StopFleetResult` | `ConcurrentModificationException`, `ResourceNotFoundException` | Stops the specified fleet. |
| `StopImageBuilder` | `-` | - | `Name` | - | `StopImageBuilderResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Stops the specified image builder. |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InvalidAccountStatusException`, `LimitExceededException`, `ResourceNotFoundException` | Adds or overwrites one or more tags for the specified WorkSpaces Applications resource. You can tag WorkSpaces Applications image builders, images, fleets, and stacks. Each tag consists of a key and an optional value ... |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Disassociates one or more specified tags from the specified WorkSpaces Applications resource. To list the current tags for your resources, use ListTagsForResource . For more information about tags, see Tagging Your R ... |
| `UpdateAppBlockBuilder` | `-` | - | `Name` | - | `UpdateAppBlockBuilderResult` | `ConcurrentModificationException`, `InvalidAccountStatusException`, `InvalidParameterCombinationException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `RequestLimitExceededException`, `ResourceInUseException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Updates an app block builder. If the app block builder is in the STARTING or STOPPING state, you can't update it. If the app block builder is in the RUNNING state, you can only update the DisplayName and Description. ... |
| `UpdateApplication` | `-` | - | `Name` | - | `UpdateApplicationResult` | `ConcurrentModificationException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Updates the specified application. |
| `UpdateDirectoryConfig` | `-` | - | `DirectoryName` | - | `UpdateDirectoryConfigResult` | `ConcurrentModificationException`, `IncompatibleImageException`, `InvalidRoleException`, `OperationNotPermittedException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates the specified Directory Config object in WorkSpaces Applications. This object includes the configuration information required to join fleets and image builders to Microsoft Active Directory domains. |
| `UpdateEntitlement` | `-` | - | `Name`, `StackName` | - | `UpdateEntitlementResult` | `ConcurrentModificationException`, `EntitlementNotFoundException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Updates the specified entitlement. |
| `UpdateFleet` | `-` | - | - | - | `UpdateFleetResult` | `ConcurrentModificationException`, `IncompatibleImageException`, `InvalidAccountStatusException`, `InvalidParameterCombinationException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `RequestLimitExceededException`, `ResourceInUseException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Updates the specified fleet. If the fleet is in the STOPPED state, you can update any attribute except the fleet name. If the fleet is in the RUNNING state, you can update the following based on the fleet type: Alway ... |
| `UpdateImagePermissions` | `-` | - | `Name`, `SharedAccountId`, `ImagePermissions` | - | `UpdateImagePermissionsResult` | `LimitExceededException`, `ResourceNotAvailableException`, `ResourceNotFoundException` | Adds or updates permissions for the specified private image. |
| `UpdateStack` | `-` | - | `Name` | - | `UpdateStackResult` | `ConcurrentModificationException`, `IncompatibleImageException`, `InvalidAccountStatusException`, `InvalidParameterCombinationException`, `InvalidRoleException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates the specified fields for the specified stack. |
| `UpdateThemeForStack` | `-` | - | `StackName` | - | `UpdateThemeForStackResult` | `ConcurrentModificationException`, `InvalidAccountStatusException`, `InvalidParameterCombinationException`, `LimitExceededException`, `OperationNotPermittedException`, `ResourceNotFoundException` | Updates custom branding that customizes the appearance of the streaming application catalog page. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConcurrentModificationException` | `structure` | Message | An API error occurred. Wait a few minutes and try again. |
| `DryRunOperationException` | `structure` | Message | The exception that is thrown when a dry run operation is requested. This indicates that the validation checks have been performed successfully, but no actua ... |
| `EntitlementAlreadyExistsException` | `structure` | Message | The entitlement already exists. |
| `EntitlementNotFoundException` | `structure` | Message | The entitlement can't be found. |
| `IncompatibleImageException` | `structure` | Message | The image can't be updated because it's not compatible for updates. |
| `InvalidAccountStatusException` | `structure` | Message | The resource cannot be created because your AWS account is suspended. For assistance, contact AWS Support. |
| `InvalidParameterCombinationException` | `structure` | Message | Indicates an incorrect combination of parameters, or a missing parameter. |
| `InvalidRoleException` | `structure` | Message | The specified role is invalid. |
| `LimitExceededException` | `structure` | Message | The requested limit exceeds the permitted limit for an account. |
| `OperationNotPermittedException` | `structure` | Message | The attempted operation is not permitted. |
| `RequestLimitExceededException` | `structure` | Message | WorkSpaces Applications can’t process the request right now because the Describe calls from your AWS account are being throttled by Amazon EC2. Try again later. |
| `ResourceAlreadyExistsException` | `structure` | Message | The specified resource already exists. |
| `ResourceInUseException` | `structure` | Message | The specified resource is in use. |
| `ResourceNotAvailableException` | `structure` | Message | The specified resource exists and is not in use, but isn't available. |
| `ResourceNotFoundException` | `structure` | Message | The specified resource was not found. |
| `AssociateAppBlockBuilderAppBlockRequest` | `structure` | AppBlockArn, AppBlockBuilderName | - |
| `AssociateAppBlockBuilderAppBlockResult` | `structure` | AppBlockBuilderAppBlockAssociation | - |
| `AssociateApplicationFleetRequest` | `structure` | FleetName, ApplicationArn | - |
| `AssociateApplicationFleetResult` | `structure` | ApplicationFleetAssociation | - |
| `AssociateApplicationToEntitlementRequest` | `structure` | StackName, EntitlementName, ApplicationIdentifier | - |
| `AssociateApplicationToEntitlementResult` | `structure` | **empty (no members)** | - |
| `AssociateFleetRequest` | `structure` | FleetName, StackName | - |
| `AssociateFleetResult` | `structure` | **empty (no members)** | - |
| `AssociateSoftwareToImageBuilderRequest` | `structure` | ImageBuilderName, SoftwareNames | - |
| `AssociateSoftwareToImageBuilderResult` | `structure` | **empty (no members)** | - |
| `BatchAssociateUserStackRequest` | `structure` | UserStackAssociations | - |
| `BatchAssociateUserStackResult` | `structure` | errors | - |
| `BatchDisassociateUserStackRequest` | `structure` | UserStackAssociations | - |
| `BatchDisassociateUserStackResult` | `structure` | errors | - |
| `CopyImageRequest` | `structure` | SourceImageName, DestinationImageName, DestinationRegion, DestinationImageDescription | - |
| `CopyImageResponse` | `structure` | DestinationImageName | - |
| `CreateAppBlockRequest` | `structure` | Name, Description, DisplayName, SourceS3Location, SetupScriptDetails, Tags, PostSetupScriptDetails, PackagingType | - |
| `CreateAppBlockResult` | `structure` | AppBlock | - |
| `CreateAppBlockBuilderRequest` | `structure` | Name, Description, DisplayName, Tags, Platform, InstanceType, VpcConfig, EnableDefaultInternetAccess, IamRoleArn, AccessEndpoints, DisableIMDSV1 | - |
| `CreateAppBlockBuilderResult` | `structure` | AppBlockBuilder | - |
| `CreateAppBlockBuilderStreamingURLRequest` | `structure` | AppBlockBuilderName, Validity | - |
| `CreateAppBlockBuilderStreamingURLResult` | `structure` | StreamingURL, Expires | - |
| `CreateApplicationRequest` | `structure` | Name, DisplayName, Description, IconS3Location, LaunchPath, WorkingDirectory, LaunchParameters, Platforms, InstanceFamilies, AppBlockArn, Tags | - |
| `CreateApplicationResult` | `structure` | Application | - |
| `CreateDirectoryConfigRequest` | `structure` | DirectoryName, OrganizationalUnitDistinguishedNames, ServiceAccountCredentials, CertificateBasedAuthProperties | - |
| `AccessEndpointType` | `enum` | STREAMING | - |
| `Action` | `enum` | CLIPBOARD_COPY_FROM_LOCAL_DEVICE, CLIPBOARD_COPY_TO_LOCAL_DEVICE, FILE_UPLOAD, FILE_DOWNLOAD, PRINTING_TO_LOCAL_DEVICE, DOMAIN_PASSWORD_SIGNIN, DOMAIN_SMART_CARD_SIGNIN, AUTO_TIME_ZONE_REDIRECTION | - |
| `AgentAction` | `enum` | COMPUTER_VISION, COMPUTER_INPUT | The type of agent action. COMPUTER_VISION - Allows agents to take screenshots of the desktop. COMPUTER_INPUT - Allows agents to click, type, and scroll on t ... |
| `AgentSoftwareVersion` | `enum` | CURRENT_LATEST, ALWAYS_LATEST | The image type is the type of AppStream image resource. |
| `AppBlockBuilderAttribute` | `enum` | IAM_ROLE_ARN, ACCESS_ENDPOINTS, VPC_CONFIGURATION_SECURITY_GROUP_IDS | - |
| `AppBlockBuilderPlatformType` | `enum` | WINDOWS_SERVER_2019 | - |
| `AppBlockBuilderState` | `enum` | STARTING, RUNNING, STOPPING, STOPPED | - |
| `AppBlockBuilderStateChangeReasonCode` | `enum` | INTERNAL_ERROR | - |
| `AppBlockState` | `enum` | INACTIVE, ACTIVE | - |
| `AppVisibility` | `enum` | ALL, ASSOCIATED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
