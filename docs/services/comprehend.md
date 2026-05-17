# winterbaume-comprehend

Comprehend service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Comprehend |
| AWS model | `comprehend` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 60/85 operations (70.6%) |
| stubs (routed, returns empty/default) | 5/85 operations (5.9%) |
| moto coverage | 63/85 operations (74.1%) |
| floci coverage | 0/85 operations (0.0%) |
| kumo coverage | 12/85 operations (14.1%) |
| Coverage report date | 2026-05-16 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws comprehend list-endpoints
```

## Example

```rust
use aws_sdk_comprehend::config::BehaviorVersion;
use winterbaume_comprehend::ComprehendService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ComprehendService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_comprehend::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_comprehend::Client::new(&config);

    let resp = client
        .list_document_classifiers()
        .send()
        .await
        .expect("list_document_classifiers should succeed");
    println!(
        "Document classifiers: {}",
        resp.document_classifier_properties_list().len()
    );
}
```

## Implemented APIs (60)

- `CreateDocumentClassifier`
- `CreateEndpoint`
- `CreateEntityRecognizer`
- `CreateFlywheel`
- `DeleteDocumentClassifier`
- `DeleteEndpoint`
- `DeleteEntityRecognizer`
- `DeleteFlywheel`
- `DeleteResourcePolicy`
- `DescribeDocumentClassificationJob`
- `DescribeDocumentClassifier`
- `DescribeDominantLanguageDetectionJob`
- `DescribeEndpoint`
- `DescribeEntitiesDetectionJob`
- `DescribeEntityRecognizer`
- `DescribeEventsDetectionJob`
- `DescribeFlywheel`
- `DescribeKeyPhrasesDetectionJob`
- `DescribePiiEntitiesDetectionJob`
- `DescribeResourcePolicy`
- `DescribeSentimentDetectionJob`
- `DescribeTargetedSentimentDetectionJob`
- `DescribeTopicsDetectionJob`
- `ListDocumentClassificationJobs`
- `ListDocumentClassifiers`
- `ListDominantLanguageDetectionJobs`
- `ListEndpoints`
- `ListEntitiesDetectionJobs`
- `ListEntityRecognizers`
- `ListEventsDetectionJobs`
- `ListFlywheels`
- `ListKeyPhrasesDetectionJobs`
- `ListPiiEntitiesDetectionJobs`
- `ListSentimentDetectionJobs`
- `ListTagsForResource`
- `ListTargetedSentimentDetectionJobs`
- `ListTopicsDetectionJobs`
- `PutResourcePolicy`
- `StartDocumentClassificationJob`
- `StartDominantLanguageDetectionJob`
- `StartEntitiesDetectionJob`
- `StartEventsDetectionJob`
- `StartFlywheelIteration`
- `StartKeyPhrasesDetectionJob`
- `StartPiiEntitiesDetectionJob`
- `StartSentimentDetectionJob`
- `StartTargetedSentimentDetectionJob`
- `StartTopicsDetectionJob`
- `StopDominantLanguageDetectionJob`
- `StopEntitiesDetectionJob`
- `StopEventsDetectionJob`
- `StopKeyPhrasesDetectionJob`
- `StopPiiEntitiesDetectionJob`
- `StopSentimentDetectionJob`
- `StopTargetedSentimentDetectionJob`
- `StopTrainingDocumentClassifier`
- `StopTrainingEntityRecognizer`
- `TagResource`
- `UntagResource`
- `UpdateEndpoint`

<details><summary>Stubbed APIs (5) &mdash; routed but return an empty/default response</summary>

- `DetectDominantLanguage`
- `DetectEntities`
- `DetectKeyPhrases`
- `DetectPiiEntities`
- `DetectSentiment`

</details>

<details><summary>Not yet implemented APIs (20)</summary>

- `BatchDetectDominantLanguage` (implemented by kumo)
- `BatchDetectEntities` (implemented by kumo)
- `BatchDetectKeyPhrases` (implemented by kumo)
- `BatchDetectSentiment` (implemented by kumo)
- `BatchDetectSyntax` (implemented by kumo)
- `BatchDetectTargetedSentiment`
- `ClassifyDocument`
- `ContainsPiiEntities` (implemented by kumo)
- `CreateDataset`
- `DescribeDataset`
- `DescribeFlywheelIteration`
- `DetectSyntax` (implemented by kumo)
- `DetectTargetedSentiment`
- `DetectToxicContent`
- `ImportModel`
- `ListDatasets`
- `ListDocumentClassifierSummaries`
- `ListEntityRecognizerSummaries`
- `ListFlywheelIterationHistory`
- `UpdateFlywheel`

</details>
