# Amazon Rekognition

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the API Reference for Amazon Rekognition Image, Amazon Rekognition Custom Labels, Amazon Rekognition Stored Video, Amazon Rekognition Streaming Video. It provides descriptions of actions, data types, common parameters, and common errors. Amazon Rekognition Image AssociateFaces CompareFaces CreateCollection CreateUser DeleteCollection DeleteFaces DeleteUser DescribeCollection DetectFaces DetectLabels DetectModerationLabels DetectProtectiveEquipment DetectText DisassociateFaces GetCelebrityInfo GetMediaAnalysisJob IndexFaces ListCollections ListMediaAnalysisJob ListFaces ListUsers RecognizeCelebrities SearchFaces SearchFacesByImage SearchUsers SearchUsersByImage StartMediaAnalysisJob Amazon Rekognition Custom Labels CopyProjectVersion CreateDataset CreateProject CreateProjectVersion DeleteDataset DeleteProject DeleteProjectPolicy DeleteProjectVersion DescribeDataset DescribeProjects DescribeProjectVersions DetectCustomLabels DistributeDatasetEntries ListDatasetEntries ListDatasetLabels ListProjectPolicies PutProjectPolicy StartProjectVersion StopProjectVersion UpdateDatasetEntries Amazon Rekognition Video Stored Video GetCelebrityRecognition GetContentModeration GetFaceDetection GetFaceSearch GetLabelDetection GetPersonTracking GetSegmentDetection GetTextDetection StartCelebrityRecognition StartContentModeration StartFaceDetection StartFaceSearch StartLabelDetection...

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Rekognition where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Rekognition by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon Rekognition resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Rekognition workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Start`, `List`, `Delete`, `Create` operation families, including `GetCelebrityInfo`, `GetCelebrityRecognition`, `GetContentModeration`, `GetFaceDetection`, `StartCelebrityRecognition`, `StartContentModeration`.

## Service Identity and Protocol

- AWS model slug: `rekognition`
- AWS SDK for Rust slug: `rekognition`
- Model version: `2016-06-27`
- Model file: `vendor/api-models-aws/models/rekognition/service/2016-06-27/rekognition-2016-06-27.json`
- SDK ID: `Rekognition`
- Endpoint prefix: `rekognition`
- ARN namespace: `rekognition`
- CloudFormation name: `Rekognition`
- CloudTrail event source: `rekognition.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (11), `Start` (11), `List` (9), `Delete` (8), `Create` (7), `Detect` (6), `Describe` (5), `Search` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateFaces`, `CreateCollection`, `CreateDataset`, `CreateFaceLivenessSession`, `CreateProject`, `CreateProjectVersion`, `CreateStreamProcessor`, `CreateUser`, `DeleteCollection`, `DeleteDataset`, `DeleteFaces`, `DeleteProject`, `DeleteProjectPolicy`, `DeleteProjectVersion`, `DeleteStreamProcessor`, `DeleteUser`, `DisassociateFaces`, `PutProjectPolicy`, `StartCelebrityRecognition`, `StartContentModeration`, ... (+15).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCollection`, `DescribeDataset`, `DescribeProjectVersions`, `DescribeProjects`, `DescribeStreamProcessor`, `GetCelebrityInfo`, `GetCelebrityRecognition`, `GetContentModeration`, `GetFaceDetection`, `GetFaceLivenessSessionResults`, `GetFaceSearch`, `GetLabelDetection`, `GetMediaAnalysisJob`, `GetPersonTracking`, `GetSegmentDetection`, `GetTextDetection`, `ListCollections`, `ListDatasetEntries`, `ListDatasetLabels`, `ListFaces`, ... (+9).
- Pagination is modelled for 18 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 14 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetMediaAnalysisJob`, `ListMediaAnalysisJobs`, `StartCelebrityRecognition`, `StartContentModeration`, `StartFaceDetection`, `StartFaceSearch`, `StartLabelDetection`, `StartMediaAnalysisJob`, `StartPersonTracking`, `StartProjectVersion`, `StartSegmentDetection`, `StartStreamProcessor`, `StartTextDetection`, `StopProjectVersion`, `StopStreamProcessor`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 75 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetCelebrityInfo`, `GetCelebrityRecognition`, `GetContentModeration`, `GetFaceDetection`, `GetFaceLivenessSessionResults`, `GetFaceSearch`, `GetLabelDetection`, `GetMediaAnalysisJob`, `GetPersonTracking`, `GetSegmentDetection`, `GetTextDetection`
- Traits: `paginated` (8)
- Common required input members in this group: `JobId`

### Start

- Operations: `StartCelebrityRecognition`, `StartContentModeration`, `StartFaceDetection`, `StartFaceSearch`, `StartLabelDetection`, `StartMediaAnalysisJob`, `StartPersonTracking`, `StartProjectVersion`, `StartSegmentDetection`, `StartStreamProcessor`, `StartTextDetection`
- Traits: `idempotent` (9), `idempotency-token` (1)
- Common required input members in this group: `Video`

### List

- Operations: `ListCollections`, `ListDatasetEntries`, `ListDatasetLabels`, `ListFaces`, `ListMediaAnalysisJobs`, `ListProjectPolicies`, `ListStreamProcessors`, `ListTagsForResource`, `ListUsers`
- Traits: `paginated` (8)
- Common required input members in this group: `DatasetArn`, `CollectionId`

### Delete

- Operations: `DeleteCollection`, `DeleteDataset`, `DeleteFaces`, `DeleteProject`, `DeleteProjectPolicy`, `DeleteProjectVersion`, `DeleteStreamProcessor`, `DeleteUser`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `CollectionId`, `ProjectArn`

### Create

- Operations: `CreateCollection`, `CreateDataset`, `CreateFaceLivenessSession`, `CreateProject`, `CreateProjectVersion`, `CreateStreamProcessor`, `CreateUser`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: `CollectionId`, `ProjectArn`

### Detect

- Operations: `DetectCustomLabels`, `DetectFaces`, `DetectLabels`, `DetectModerationLabels`, `DetectProtectiveEquipment`, `DetectText`
- Common required input members in this group: `Image`

### Describe

- Operations: `DescribeCollection`, `DescribeDataset`, `DescribeProjects`, `DescribeProjectVersions`, `DescribeStreamProcessor`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Search

- Operations: `SearchFaces`, `SearchFacesByImage`, `SearchUsers`, `SearchUsersByImage`
- Common required input members in this group: `CollectionId`, `Image`

### Stop

- Operations: `StopProjectVersion`, `StopStreamProcessor`
- Common required input members in this group: -

### Update

- Operations: `UpdateDatasetEntries`, `UpdateStreamProcessor`
- Common required input members in this group: -

### Associate

- Operations: `AssociateFaces`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Compare

- Operations: `CompareFaces`
- Common required input members in this group: -

### Copy

- Operations: `CopyProjectVersion`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateFaces`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Distribute

- Operations: `DistributeDatasetEntries`
- Common required input members in this group: -

### Index

- Operations: `IndexFaces`
- Common required input members in this group: -

### Put

- Operations: `PutProjectPolicy`
- Common required input members in this group: -

### Recognize

- Operations: `RecognizeCelebrities`
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
| `AssociateFaces` | `-` | `idempotency-token` | `CollectionId`, `UserId`, `FaceIds` | `ClientRequestToken` | `AssociateFacesResponse` | `AccessDeniedException`, `ConflictException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Associates one or more faces with an existing UserID. Takes an array of FaceIds . Each FaceId that are present in the FaceIds list is associated with the provided UserID. The number of FaceIds that can be used as inp ... |
| `CompareFaces` | `-` | - | `SourceImage`, `TargetImage` | - | `CompareFacesResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Compares a face in the source input image with each of the 100 largest faces detected in the target input image. If the source image contains multiple faces, the service detects the largest face and compares it with ... |
| `CopyProjectVersion` | `-` | - | `SourceProjectArn`, `SourceProjectVersionArn`, `DestinationProjectArn`, `VersionName`, `OutputConfig` | - | `CopyProjectVersionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Copies a version of an Amazon Rekognition Custom Labels model from a source project to a destination project. The source and destination projects can b ... |
| `CreateCollection` | `-` | - | `CollectionId` | - | `CreateCollectionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceAlreadyExistsException`, `ServiceQuotaExceededException`, `ThrottlingException` | Creates a collection in an AWS Region. You can add faces to the collection using the IndexFaces operation. For example, you might create collections, one for each of your application users. A user can then index face ... |
| `CreateDataset` | `-` | - | `DatasetType`, `ProjectArn` | - | `CreateDatasetResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Creates a new Amazon Rekognition Custom Labels dataset. You can create a dataset by using an Amazon Sagemaker format manifest file or by copying an exi ... |
| `CreateFaceLivenessSession` | `-` | `idempotent` | - | - | `CreateFaceLivenessSessionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | This API operation initiates a Face Liveness session. It returns a SessionId , which you can use to start streaming Face Liveness video and get the results for a Face Liveness session. You can use the OutputConfig op ... |
| `CreateProject` | `-` | - | `ProjectName` | - | `CreateProjectResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ThrottlingException` | Creates a new Amazon Rekognition project. A project is a group of resources (datasets, model versions) that you use to create and manage a Amazon Rekognition Custom Labels Model or custom adapter. You can specify a f ... |
| `CreateProjectVersion` | `-` | - | `ProjectArn`, `VersionName`, `OutputConfig` | - | `CreateProjectVersionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Creates a new version of Amazon Rekognition project (like a Custom Labels model or a custom adapter) and begins training. Models and adapters are managed as part of a Rekognition project. The response from CreateProj ... |
| `CreateStreamProcessor` | `-` | - | `Input`, `Output`, `Name`, `Settings`, `RoleArn` | - | `CreateStreamProcessorResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ServiceQuotaExceededException`, `ThrottlingException` | Creates an Amazon Rekognition stream processor that you can use to detect and recognize faces or to detect labels in a streaming video. Amazon Rekognition Video is a consumer of live video from Amazon Kinesis Video S ... |
| `CreateUser` | `-` | `idempotency-token` | `CollectionId`, `UserId` | `ClientRequestToken` | `CreateUserResponse` | `AccessDeniedException`, `ConflictException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Creates a new User within a collection specified by CollectionId . Takes UserId as a parameter, which is a user provided ID which should be unique within the collection. The provided UserId will alias the system gene ... |
| `DeleteCollection` | `-` | - | `CollectionId` | - | `DeleteCollectionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified collection. Note that this operation removes all faces in the collection. For an example, see Deleting a collection . This operation requires permissions to perform the rekognition:DeleteCollect ... |
| `DeleteDataset` | `-` | - | `DatasetArn` | - | `DeleteDatasetResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Deletes an existing Amazon Rekognition Custom Labels dataset. Deleting a dataset might take while. Use DescribeDataset to check the current status. The ... |
| `DeleteFaces` | `-` | - | `CollectionId`, `FaceIds` | - | `DeleteFacesResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes faces from a collection. You specify a collection ID and an array of face IDs to remove from the collection. This operation requires permissions to perform the rekognition:DeleteFaces action. |
| `DeleteProject` | `-` | - | `ProjectArn` | - | `DeleteProjectResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a Amazon Rekognition project. To delete a project you must first delete all models or adapters associated with the project. To delete a model or adapter, see DeleteProjectVersion . DeleteProject is an asynchr ... |
| `DeleteProjectPolicy` | `-` | - | `ProjectArn`, `PolicyName` | - | `DeleteProjectPolicyResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `InvalidPolicyRevisionIdException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Deletes an existing project policy. To get a list of project policies attached to a project, call ListProjectPolicies . To attach a project policy to a ... |
| `DeleteProjectVersion` | `-` | - | `ProjectVersionArn` | - | `DeleteProjectVersionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a Rekognition project model or project version, like a Amazon Rekognition Custom Labels model or a custom adapter. You can't delete a project version if it is running or if it is training. To check the status ... |
| `DeleteStreamProcessor` | `-` | - | `Name` | - | `DeleteStreamProcessorResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the stream processor identified by Name . You assign the value for Name when you create the stream processor with CreateStreamProcessor . You might not be able to use the same name for a stream processor for ... |
| `DeleteUser` | `-` | `idempotency-token` | `CollectionId`, `UserId` | `ClientRequestToken` | `DeleteUserResponse` | `AccessDeniedException`, `ConflictException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified UserID within the collection. Faces that are associated with the UserID are disassociated from the UserID before deleting the specified UserID. If the specified Collection or UserID is already d ... |
| `DescribeCollection` | `-` | - | `CollectionId` | - | `DescribeCollectionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Describes the specified collection. You can use DescribeCollection to get information, such as the number of faces indexed into a collection and the version of the model used by the collection for face detection. For ... |
| `DescribeDataset` | `-` | - | `DatasetArn` | - | `DescribeDatasetResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Describes an Amazon Rekognition Custom Labels dataset. You can get information such as the current status of a dataset and statistics about the images ... |
| `DescribeProjects` | `-` | `paginated` | - | - | `DescribeProjectsResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Gets information about your Rekognition projects. This operation requires permissions to perform the rekognition:DescribeProjects action. |
| `DescribeProjectVersions` | `-` | `paginated` | `ProjectArn` | - | `DescribeProjectVersionsResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Lists and describes the versions of an Amazon Rekognition project. You can specify up to 10 model or adapter versions in ProjectVersionArns . If you don't specify a value, descriptions for all model/adapter versions ... |
| `DescribeStreamProcessor` | `-` | - | `Name` | - | `DescribeStreamProcessorResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Provides information about a stream processor created by CreateStreamProcessor . You can get information about the input and output streams, the input parameters for the face recognition being performed, and the curr ... |
| `DetectCustomLabels` | `-` | - | `ProjectVersionArn`, `Image` | - | `DetectCustomLabelsResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Detects custom labels in a supplied image by using an Amazon Rekognition Custom Labels model. You specify which version of a model version to use by us ... |
| `DetectFaces` | `-` | - | `Image` | - | `DetectFacesResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Detects faces within an image that is provided as input. DetectFaces detects the 100 largest faces in the image. For each face detected, the operation returns face details. These details include a bounding box of the ... |
| `DetectLabels` | `-` | - | `Image` | - | `DetectLabelsResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Detects instances of real-world entities within an image (JPEG or PNG) provided as input. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; and concepts like lan ... |
| `DetectModerationLabels` | `-` | - | `Image` | - | `DetectModerationLabelsResponse` | `AccessDeniedException`, `HumanLoopQuotaExceededException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | Detects unsafe content in a specified JPEG or PNG format image. Use DetectModerationLabels to moderate images depending on your requirements. For example, you might want to filter images that contain nudity, but not ... |
| `DetectProtectiveEquipment` | `-` | - | `Image` | - | `DetectProtectiveEquipmentResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Detects Personal Protective Equipment (PPE) worn by people detected in an image. Amazon Rekognition can detect the following types of PPE. Face cover Hand cover Head cover You pass the input image as base64-encoded i ... |
| `DetectText` | `-` | - | `Image` | - | `DetectTextResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Detects text in the input image and converts it into machine-readable text. Pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amaz ... |
| `DisassociateFaces` | `-` | `idempotency-token` | `CollectionId`, `UserId`, `FaceIds` | `ClientRequestToken` | `DisassociateFacesResponse` | `AccessDeniedException`, `ConflictException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Removes the association between a Face supplied in an array of FaceIds and the User. If the User is not present already, then a ResourceNotFound exception is thrown. If successful, an array of faces that are disassoc ... |
| `DistributeDatasetEntries` | `-` | - | `Datasets` | - | `DistributeDatasetEntriesResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Distributes the entries (images) in a training dataset across the training dataset and the test dataset for a project. DistributeDatasetEntries moves 2 ... |
| `GetCelebrityInfo` | `-` | - | `Id` | - | `GetCelebrityInfoResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the name and additional information about a celebrity based on their Amazon Rekognition ID. The additional information is returned as an array of URLs. If there is no additional information about the celebrity, ... |
| `GetCelebrityRecognition` | `-` | `paginated` | `JobId` | - | `GetCelebrityRecognitionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the celebrity recognition results for a Amazon Rekognition Video analysis started by StartCelebrityRecognition . Celebrity recognition in a video is an asynchronous operation. Analysis is started by a call to St ... |
| `GetContentModeration` | `-` | `paginated` | `JobId` | - | `GetContentModerationResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the inappropriate, unwanted, or offensive content analysis results for a Amazon Rekognition Video analysis started by StartContentModeration . For a list of moderation labels in Amazon Rekognition, see Using the ... |
| `GetFaceDetection` | `-` | `paginated` | `JobId` | - | `GetFaceDetectionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Gets face detection results for a Amazon Rekognition Video analysis started by StartFaceDetection . Face detection with Amazon Rekognition Video is an asynchronous operation. You start face detection by calling Start ... |
| `GetFaceLivenessSessionResults` | `-` | - | `SessionId` | - | `GetFaceLivenessSessionResultsResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `SessionNotFoundException`, `ThrottlingException` | Retrieves the results of a specific Face Liveness session. It requires the sessionId as input, which was created using CreateFaceLivenessSession . Returns the corresponding Face Liveness confidence score, a reference ... |
| `GetFaceSearch` | `-` | `paginated` | `JobId` | - | `GetFaceSearchResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the face search results for Amazon Rekognition Video face search started by StartFaceSearch . The search returns faces in a collection that match the faces of persons detected in a video. It also includes the ti ... |
| `GetLabelDetection` | `-` | `paginated` | `JobId` | - | `GetLabelDetectionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the label detection results of a Amazon Rekognition Video analysis started by StartLabelDetection . The label detection operation is started by a call to StartLabelDetection which returns a job identifier ( JobI ... |
| `GetMediaAnalysisJob` | `-` | - | `JobId` | - | `GetMediaAnalysisJobResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the results for a given media analysis job. Takes a JobId returned by StartMediaAnalysisJob. |
| `GetPersonTracking` | `-` | `paginated` | `JobId` | - | `GetPersonTrackingResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | End of support notice: On October 31, 2025, AWS will discontinue support for Amazon Rekognition People Pathing. After October 31, 2025, you will no longer be able to use the Rekognition People Pathing capability. For ... |
| `GetSegmentDetection` | `-` | `paginated` | `JobId` | - | `GetSegmentDetectionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the segment detection results of a Amazon Rekognition Video analysis started by StartSegmentDetection . Segment detection with Amazon Rekognition Video is an asynchronous operation. You start segment detection b ... |
| `GetTextDetection` | `-` | `paginated` | `JobId` | - | `GetTextDetectionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the text detection results of a Amazon Rekognition Video analysis started by StartTextDetection . Text detection with Amazon Rekognition Video is an asynchronous operation. You start text detection by calling St ... |
| `IndexFaces` | `-` | - | `CollectionId`, `Image` | - | `IndexFacesResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Detects faces in the input image and adds them to the specified collection. Amazon Rekognition doesn't save the actual faces that are detected. Instead, the underlying detection algorithm first detects the faces in t ... |
| `ListCollections` | `-` | `paginated` | - | - | `ListCollectionsResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Returns list of collection IDs in your account. If the result is truncated, the response also provides a NextToken that you can use in the subsequent request to fetch the next set of collection IDs. For an example, s ... |
| `ListDatasetEntries` | `-` | `paginated` | `DatasetArn` | - | `ListDatasetEntriesResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Lists the entries (images) within a dataset. An entry is a JSON Line that contains the information for a single image, including the image location, as ... |
| `ListDatasetLabels` | `-` | `paginated` | `DatasetArn` | - | `ListDatasetLabelsResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Lists the labels in a dataset. Amazon Rekognition Custom Labels uses labels to describe images. For more information, see Labeling images . Lists the l ... |
| `ListFaces` | `-` | `paginated` | `CollectionId` | - | `ListFacesResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Returns metadata for faces in the specified collection. This metadata includes information such as the bounding box coordinates, the confidence (that the bounding box contains a face), and face ID. For an example, se ... |
| `ListMediaAnalysisJobs` | `-` | `paginated` | - | - | `ListMediaAnalysisJobsResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Returns a list of media analysis jobs. Results are sorted by CreationTimestamp in descending order. |
| `ListProjectPolicies` | `-` | `paginated` | `ProjectArn` | - | `ListProjectPoliciesResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Gets a list of the project policies attached to a project. To attach a project policy to a project, call PutProjectPolicy . To remove a project policy ... |
| `ListStreamProcessors` | `-` | `paginated` | - | - | `ListStreamProcessorsResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Gets a list of stream processors that you have created with CreateStreamProcessor . |
| `ListTagsForResource` | `-` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of tags in an Amazon Rekognition collection, stream processor, or Custom Labels model. This operation requires permissions to perform the rekognition:ListTagsForResource action. |
| `ListUsers` | `-` | `paginated` | `CollectionId` | - | `ListUsersResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidPaginationTokenException`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Returns metadata of the User such as UserID in the specified collection. Anonymous User (to reserve faces without any identity) is not returned as part of this request. The results are sorted by system generated prim ... |
| `PutProjectPolicy` | `-` | - | `ProjectArn`, `PolicyName`, `PolicyDocument` | - | `PutProjectPolicyResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `InvalidPolicyRevisionIdException`, `LimitExceededException`, `MalformedPolicyDocumentException`, `ProvisionedThroughputExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Attaches a project policy to a Amazon Rekognition Custom Labels project in a trusting AWS account. A project policy specifies that a trusted AWS accoun ... |
| `RecognizeCelebrities` | `-` | - | `Image` | - | `RecognizeCelebritiesResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ThrottlingException` | Returns an array of celebrities recognized in the input image. For more information, see Recognizing celebrities in the Amazon Rekognition Developer Guide. RecognizeCelebrities returns the 64 largest faces in the ima ... |
| `SearchFaces` | `-` | - | `CollectionId`, `FaceId` | - | `SearchFacesResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the IndexFaces operation. The operation compares the feature ... |
| `SearchFacesByImage` | `-` | - | `CollectionId`, `Image` | - | `SearchFacesByImageResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | For a given input image, first detects the largest face in the image, and then searches the specified collection for matching faces. The operation compares the features of the input face with faces in the specified c ... |
| `SearchUsers` | `-` | - | `CollectionId` | - | `SearchUsersResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Searches for UserIDs within a collection based on a FaceId or UserId . This API can be used to find the closest UserID (with a highest similarity) to associate a face. The request must be provided with either FaceId ... |
| `SearchUsersByImage` | `-` | - | `CollectionId`, `Image` | - | `SearchUsersByImageResponse` | `AccessDeniedException`, `ImageTooLargeException`, `InternalServerError`, `InvalidImageFormatException`, `InvalidParameterException`, `InvalidS3ObjectException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Searches for UserIDs using a supplied image. It first detects the largest face in the image, and then searches a specified collection for matching UserIDs. The operation returns an array of UserIDs that match the fac ... |
| `StartCelebrityRecognition` | `-` | `idempotent` | `Video` | - | `StartCelebrityRecognitionResponse` | `AccessDeniedException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `VideoTooLargeException` | Starts asynchronous recognition of celebrities in a stored video. Amazon Rekognition Video can detect celebrities in a video must be stored in an Amazon S3 bucket. Use Video to specify the bucket name and the filenam ... |
| `StartContentModeration` | `-` | `idempotent` | `Video` | - | `StartContentModerationResponse` | `AccessDeniedException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `VideoTooLargeException` | Starts asynchronous detection of inappropriate, unwanted, or offensive content in a stored video. For a list of moderation labels in Amazon Rekognition, see Using the image and video moderation APIs . Amazon Rekognit ... |
| `StartFaceDetection` | `-` | `idempotent` | `Video` | - | `StartFaceDetectionResponse` | `AccessDeniedException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `VideoTooLargeException` | Starts asynchronous detection of faces in a stored video. Amazon Rekognition Video can detect faces in a video stored in an Amazon S3 bucket. Use Video to specify the bucket name and the filename of the video. StartF ... |
| `StartFaceSearch` | `-` | `idempotent` | `Video`, `CollectionId` | - | `StartFaceSearchResponse` | `AccessDeniedException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `VideoTooLargeException` | Starts the asynchronous search for faces in a collection that match the faces of persons detected in a stored video. The video must be stored in an Amazon S3 bucket. Use Video to specify the bucket name and the filen ... |
| `StartLabelDetection` | `-` | `idempotent` | `Video` | - | `StartLabelDetectionResponse` | `AccessDeniedException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `VideoTooLargeException` | Starts asynchronous detection of labels in a stored video. Amazon Rekognition Video can detect labels in a video. Labels are instances of real-world entities. This includes objects like flower, tree, and table; event ... |
| `StartMediaAnalysisJob` | `-` | `idempotent`, `idempotency-token` | `OperationsConfig`, `Input`, `OutputConfig` | `ClientRequestToken` | `StartMediaAnalysisJobResponse` | `AccessDeniedException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidManifestException`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | Initiates a new media analysis job. Accepts a manifest file in an Amazon S3 bucket. The output is a manifest file and a summary of the manifest stored in the Amazon S3 bucket. |
| `StartPersonTracking` | `-` | `idempotent` | `Video` | - | `StartPersonTrackingResponse` | `AccessDeniedException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `VideoTooLargeException` | End of support notice: On October 31, 2025, AWS will discontinue support for Amazon Rekognition People Pathing. After October 31, 2025, you will no longer be able to use the Rekognition People Pathing capability. For ... |
| `StartProjectVersion` | `-` | - | `ProjectVersionArn`, `MinInferenceUnits` | - | `StartProjectVersionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Starts the running of the version of a model. Starting a model takes a while to complete. To check the current state of the model, use DescribeProjectV ... |
| `StartSegmentDetection` | `-` | `idempotent` | `Video`, `SegmentTypes` | - | `StartSegmentDetectionResponse` | `AccessDeniedException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `VideoTooLargeException` | Starts asynchronous detection of segment detection in a stored video. Amazon Rekognition Video can detect segments in a video stored in an Amazon S3 bucket. Use Video to specify the bucket name and the filename of th ... |
| `StartStreamProcessor` | `-` | - | `Name` | - | `StartStreamProcessorResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Starts processing a stream processor. You create a stream processor by calling CreateStreamProcessor . To tell StartStreamProcessor which stream processor to start, use the value of the Name field specified in the ca ... |
| `StartTextDetection` | `-` | `idempotent` | `Video` | - | `StartTextDetectionResponse` | `AccessDeniedException`, `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidParameterException`, `InvalidS3ObjectException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ThrottlingException`, `VideoTooLargeException` | Starts asynchronous detection of text in a stored video. Amazon Rekognition Video can detect text in a video stored in an Amazon S3 bucket. Use Video to specify the bucket name and the filename of the video. StartTex ... |
| `StopProjectVersion` | `-` | - | `ProjectVersionArn` | - | `StopProjectVersionResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Stops a running model. The operation might take a while to complete. To check the current status, call DescribeProjectVersions . Only applies to Custom ... |
| `StopStreamProcessor` | `-` | - | `Name` | - | `StopStreamProcessorResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Stops a running stream processor that was created by CreateStreamProcessor . |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Adds one or more key-value tags to an Amazon Rekognition collection, stream processor, or Custom Labels model. For more information, see Tagging AWS Resources . This operation requires permissions to perform the reko ... |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Removes one or more tags from an Amazon Rekognition collection, stream processor, or Custom Labels model. This operation requires permissions to perform the rekognition:UntagResource action. |
| `UpdateDatasetEntries` | `-` | - | `DatasetArn`, `Changes` | - | `UpdateDatasetEntriesResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `LimitExceededException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | This operation applies only to Amazon Rekognition Custom Labels. Adds or updates one or more entries (images) in a dataset. An entry is a JSON Line which contains the information for a single image, including the ima ... |
| `UpdateStreamProcessor` | `-` | - | `Name` | - | `UpdateStreamProcessorResponse` | `AccessDeniedException`, `InternalServerError`, `InvalidParameterException`, `ProvisionedThroughputExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException` | Allows you to update a stream processor. You can change some settings and regions of interest and delete certain parameters. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message, Code, Logref | You are not authorized to perform the action. |
| `ConflictException` | `structure` | Message, Code, Logref | A User with the same Id already exists within the collection, or the update or deletion of the User caused an inconsistent state. ** |
| `HumanLoopQuotaExceededException` | `structure` | ResourceType, QuotaCode, ServiceCode, Message, Code, Logref | The number of in-progress human reviews you have has exceeded the number allowed. |
| `IdempotentParameterMismatchException` | `structure` | Message, Code, Logref | A ClientRequestToken input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the ... |
| `ImageTooLargeException` | `structure` | Message, Code, Logref | The input image size exceeds the allowed limit. If you are calling DetectProtectiveEquipment, the image size or resolution exceeds the allowed limit. For mo ... |
| `InternalServerError` | `structure` | Message, Code, Logref | Amazon Rekognition experienced a service issue. Try your call again. |
| `InvalidImageFormatException` | `structure` | Message, Code, Logref | The provided image format is not supported. |
| `InvalidManifestException` | `structure` | Message, Code, Logref | Indicates that a provided manifest file is empty or larger than the allowed limit. |
| `InvalidPaginationTokenException` | `structure` | Message, Code, Logref | Pagination token in the request is not valid. |
| `InvalidParameterException` | `structure` | Message, Code, Logref | Input parameter violated a constraint. Validate your parameter before calling the API operation again. |
| `InvalidPolicyRevisionIdException` | `structure` | Message, Code, Logref | The supplied revision id for the project policy is invalid. |
| `InvalidS3ObjectException` | `structure` | Message, Code, Logref | Amazon Rekognition is unable to access the S3 object specified in the request. |
| `LimitExceededException` | `structure` | Message, Code, Logref | An Amazon Rekognition service limit was exceeded. For example, if you start too many jobs concurrently, subsequent calls to start operations (ex: StartLabel ... |
| `MalformedPolicyDocumentException` | `structure` | Message, Code, Logref | The format of the project policy document that you supplied to PutProjectPolicy is incorrect. |
| `ProvisionedThroughputExceededException` | `structure` | Message, Code, Logref | The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition. |
| `ResourceAlreadyExistsException` | `structure` | Message, Code, Logref | A resource with the specified ID already exists. |
| `ResourceInUseException` | `structure` | Message, Code, Logref | The specified resource is already being used. |
| `ResourceNotFoundException` | `structure` | Message, Code, Logref | The resource specified in the request cannot be found. |
| `ResourceNotReadyException` | `structure` | Message, Code, Logref | The requested resource isn't ready. For example, this exception occurs when you call DetectCustomLabels with a model version that isn't deployed. |
| `ServiceQuotaExceededException` | `structure` | Message, Code, Logref | The size of the collection exceeds the allowed limit. For more information, see Guidelines and quotas in Amazon Rekognition in the Amazon Rekognition Develo ... |
| `SessionNotFoundException` | `structure` | Message, Code, Logref | Occurs when a given sessionId is not found. |
| `ThrottlingException` | `structure` | Message, Code, Logref | Amazon Rekognition is temporarily unable to process the request. Try your call again. |
| `VideoTooLargeException` | `structure` | Message, Code, Logref | The file size or duration of the supplied media is too large. The maximum file size is 10GB. The maximum duration is 6 hours. |
| `AssociateFacesRequest` | `structure` | CollectionId, UserId, FaceIds, UserMatchThreshold, ClientRequestToken | - |
| `AssociateFacesResponse` | `structure` | AssociatedFaces, UnsuccessfulFaceAssociations, UserStatus | - |
| `CompareFacesRequest` | `structure` | SourceImage, TargetImage, SimilarityThreshold, QualityFilter | - |
| `CompareFacesResponse` | `structure` | SourceImageFace, FaceMatches, UnmatchedFaces, SourceImageOrientationCorrection, TargetImageOrientationCorrection | - |
| `CopyProjectVersionRequest` | `structure` | SourceProjectArn, SourceProjectVersionArn, DestinationProjectArn, VersionName, OutputConfig, Tags, KmsKeyId | - |
| `CopyProjectVersionResponse` | `structure` | ProjectVersionArn | - |
| `CreateCollectionRequest` | `structure` | CollectionId, Tags | - |
| `CreateCollectionResponse` | `structure` | StatusCode, CollectionArn, FaceModelVersion | - |
| `CreateDatasetRequest` | `structure` | DatasetSource, DatasetType, ProjectArn, Tags | - |
| `CreateDatasetResponse` | `structure` | DatasetArn | - |
| `CreateFaceLivenessSessionRequest` | `structure` | KmsKeyId, Settings, ClientRequestToken | - |
| `CreateFaceLivenessSessionResponse` | `structure` | SessionId | - |
| `CreateProjectRequest` | `structure` | ProjectName, Feature, AutoUpdate, Tags | - |
| `CreateProjectResponse` | `structure` | ProjectArn | - |
| `CreateProjectVersionRequest` | `structure` | ProjectArn, VersionName, OutputConfig, TrainingData, TestingData, Tags, KmsKeyId, VersionDescription, FeatureConfig | - |
| `CreateProjectVersionResponse` | `structure` | ProjectVersionArn | - |
| `CreateStreamProcessorRequest` | `structure` | Input, Output, Name, Settings, RoleArn, Tags, NotificationChannel, KmsKeyId, RegionsOfInterest, DataSharingPreference | - |
| `Attribute` | `enum` | DEFAULT, ALL, AGE_RANGE, BEARD, EMOTIONS, EYE_DIRECTION, EYEGLASSES, EYES_OPEN, GENDER, MOUTH_OPEN, MUSTACHE, FACE_OCCLUDED, ... (+2) | - |
| `BodyPart` | `enum` | FACE, HEAD, LEFT_HAND, RIGHT_HAND | - |
| `CelebrityRecognitionSortBy` | `enum` | ID, TIMESTAMP | - |
| `ChallengeType` | `enum` | FACE_MOVEMENT_AND_LIGHT_CHALLENGE, FACE_MOVEMENT_CHALLENGE | - |
| `ContentClassifier` | `enum` | FREE_OF_PERSONALLY_IDENTIFIABLE_INFORMATION, FREE_OF_ADULT_CONTENT | - |
| `ContentModerationAggregateBy` | `enum` | TIMESTAMPS, SEGMENTS | - |
| `ContentModerationSortBy` | `enum` | NAME, TIMESTAMP | - |
| `CustomizationFeature` | `enum` | CONTENT_MODERATION, CUSTOM_LABELS | - |
| `DatasetStatus` | `enum` | CREATE_IN_PROGRESS, CREATE_COMPLETE, CREATE_FAILED, UPDATE_IN_PROGRESS, UPDATE_COMPLETE, UPDATE_FAILED, DELETE_IN_PROGRESS | - |
| `DatasetStatusMessageCode` | `enum` | SUCCESS, SERVICE_ERROR, CLIENT_ERROR | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
