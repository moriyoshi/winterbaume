# winterbaume-rekognition

Rekognition service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Rekognition |
| AWS model | `rekognition` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 8/75 operations (10.7%) |
| stubs (routed, returns empty/default) | 4/75 operations (5.3%) |
| moto coverage | 8/75 operations (10.7%) |
| floci coverage | 0/75 operations (0.0%) |
| kumo coverage | 13/75 operations (17.3%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws rekognition list-collections
```

## Example

```rust
use aws_sdk_rekognition::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_rekognition::RekognitionService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RekognitionService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rekognition::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_rekognition::Client::new(&config);

    let resp = client
        .list_collections()
        .send()
        .await
        .expect("list_collections should succeed");
    println!("Rekognition collections: {}", resp.collection_ids().len());
}
```

## Implemented APIs (8)

- `CreateCollection`
- `DeleteCollection`
- `DescribeCollection`
- `GetFaceSearch`
- `GetTextDetection`
- `ListCollections`
- `StartFaceSearch`
- `StartTextDetection`

<details><summary>Stubbed APIs (4) &mdash; routed but return an empty/default response</summary>

- `CompareFaces`
- `DetectCustomLabels`
- `DetectLabels`
- `DetectText`

</details>

<details><summary>Not yet implemented APIs (63)</summary>

- `AssociateFaces`
- `CopyProjectVersion`
- `CreateDataset`
- `CreateFaceLivenessSession`
- `CreateProject`
- `CreateProjectVersion`
- `CreateStreamProcessor`
- `CreateUser`
- `DeleteDataset`
- `DeleteFaces` (implemented by kumo)
- `DeleteProject`
- `DeleteProjectPolicy`
- `DeleteProjectVersion`
- `DeleteStreamProcessor`
- `DeleteUser`
- `DescribeDataset`
- `DescribeProjectVersions`
- `DescribeProjects`
- `DescribeStreamProcessor`
- `DetectFaces` (implemented by kumo)
- `DetectModerationLabels` (implemented by kumo)
- `DetectProtectiveEquipment`
- `DisassociateFaces`
- `DistributeDatasetEntries`
- `GetCelebrityInfo`
- `GetCelebrityRecognition`
- `GetContentModeration`
- `GetFaceDetection`
- `GetFaceLivenessSessionResults`
- `GetLabelDetection`
- `GetMediaAnalysisJob`
- `GetPersonTracking`
- `GetSegmentDetection`
- `IndexFaces` (implemented by kumo)
- `ListDatasetEntries`
- `ListDatasetLabels`
- `ListFaces` (implemented by kumo)
- `ListMediaAnalysisJobs`
- `ListProjectPolicies`
- `ListStreamProcessors`
- `ListTagsForResource`
- `ListUsers`
- `PutProjectPolicy`
- `RecognizeCelebrities` (implemented by kumo)
- `SearchFaces` (implemented by kumo)
- `SearchFacesByImage`
- `SearchUsers`
- `SearchUsersByImage`
- `StartCelebrityRecognition`
- `StartContentModeration`
- `StartFaceDetection`
- `StartLabelDetection`
- `StartMediaAnalysisJob`
- `StartPersonTracking`
- `StartProjectVersion`
- `StartSegmentDetection`
- `StartStreamProcessor`
- `StopProjectVersion`
- `StopStreamProcessor`
- `TagResource`
- `UntagResource`
- `UpdateDatasetEntries`
- `UpdateStreamProcessor`

</details>
