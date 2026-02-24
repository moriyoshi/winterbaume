# winterbaume-transcribe

Transcribe service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Transcribe |
| AWS model | `transcribe` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 16/43 operations (37.2%) |
| stubs (routed, returns empty/default) | 0/43 operations (0.0%) |
| moto coverage | 16/43 operations (37.2%) |
| floci coverage | 0/43 operations (0.0%) |
| kumo coverage | 0/43 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws transcribe list-transcription-jobs
```

## Example

```rust
use aws_sdk_transcribe::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_transcribe::TranscribeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TranscribeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_transcribe::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_transcribe::Client::new(&config);

    let resp = client
        .list_transcription_jobs()
        .send()
        .await
        .expect("list_transcription_jobs should succeed");
    println!(
        "Transcription jobs: {}",
        resp.transcription_job_summaries().len()
    );
}
```

## Implemented APIs (16)

- `CreateMedicalVocabulary`
- `CreateVocabulary`
- `DeleteMedicalTranscriptionJob`
- `DeleteMedicalVocabulary`
- `DeleteTranscriptionJob`
- `DeleteVocabulary`
- `GetMedicalTranscriptionJob`
- `GetMedicalVocabulary`
- `GetTranscriptionJob`
- `GetVocabulary`
- `ListMedicalTranscriptionJobs`
- `ListMedicalVocabularies`
- `ListTranscriptionJobs`
- `ListVocabularies`
- `StartMedicalTranscriptionJob`
- `StartTranscriptionJob`

<details><summary>Not yet implemented APIs (27)</summary>

- `CreateCallAnalyticsCategory`
- `CreateLanguageModel`
- `CreateVocabularyFilter`
- `DeleteCallAnalyticsCategory`
- `DeleteCallAnalyticsJob`
- `DeleteLanguageModel`
- `DeleteMedicalScribeJob`
- `DeleteVocabularyFilter`
- `DescribeLanguageModel`
- `GetCallAnalyticsCategory`
- `GetCallAnalyticsJob`
- `GetMedicalScribeJob`
- `GetVocabularyFilter`
- `ListCallAnalyticsCategories`
- `ListCallAnalyticsJobs`
- `ListLanguageModels`
- `ListMedicalScribeJobs`
- `ListTagsForResource`
- `ListVocabularyFilters`
- `StartCallAnalyticsJob`
- `StartMedicalScribeJob`
- `TagResource`
- `UntagResource`
- `UpdateCallAnalyticsCategory`
- `UpdateMedicalVocabulary`
- `UpdateVocabulary`
- `UpdateVocabularyFilter`

</details>
