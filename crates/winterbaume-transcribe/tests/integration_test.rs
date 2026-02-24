use aws_sdk_transcribe::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_transcribe::TranscribeService;

async fn make_client() -> aws_sdk_transcribe::Client {
    let mock = MockAws::builder()
        .with_service(TranscribeService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_transcribe::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_transcribe::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_vocabulary() {
    let client = make_client().await;

    let create_resp = client
        .create_vocabulary()
        .vocabulary_name("my-vocab")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .phrases("hello")
        .phrases("world")
        .send()
        .await
        .expect("create_vocabulary should succeed");

    assert_eq!(create_resp.vocabulary_name().unwrap(), "my-vocab");
    assert_eq!(
        create_resp.vocabulary_state().unwrap(),
        &aws_sdk_transcribe::types::VocabularyState::Ready
    );

    let get_resp = client
        .get_vocabulary()
        .vocabulary_name("my-vocab")
        .send()
        .await
        .expect("get_vocabulary should succeed");

    assert_eq!(get_resp.vocabulary_name().unwrap(), "my-vocab");
    assert_eq!(
        get_resp.language_code().unwrap(),
        &aws_sdk_transcribe::types::LanguageCode::EnUs
    );
    assert_eq!(
        get_resp.vocabulary_state().unwrap(),
        &aws_sdk_transcribe::types::VocabularyState::Ready
    );
}

#[tokio::test]
async fn test_delete_vocabulary() {
    let client = make_client().await;

    client
        .create_vocabulary()
        .vocabulary_name("delete-me")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .phrases("test")
        .send()
        .await
        .expect("create should succeed");

    client
        .delete_vocabulary()
        .vocabulary_name("delete-me")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_vocabulary()
        .vocabulary_name("delete-me")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_vocabularies() {
    let client = make_client().await;

    for name in ["vocab-a", "vocab-b"] {
        client
            .create_vocabulary()
            .vocabulary_name(name)
            .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
            .phrases("test")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_vocabularies()
        .send()
        .await
        .expect("list_vocabularies should succeed");

    assert_eq!(resp.vocabularies().len(), 2);
}

#[tokio::test]
async fn test_create_duplicate_vocabulary() {
    let client = make_client().await;

    client
        .create_vocabulary()
        .vocabulary_name("dup-vocab")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .phrases("test")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_vocabulary()
        .vocabulary_name("dup-vocab")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .phrases("test")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_vocabulary() {
    let client = make_client().await;

    let result = client
        .delete_vocabulary()
        .vocabulary_name("does-not-exist")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_vocabularies_with_name_contains() {
    let client = make_client().await;

    for name in ["alpha-vocab", "beta-vocab", "gamma-other"] {
        client
            .create_vocabulary()
            .vocabulary_name(name)
            .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
            .phrases("test")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_vocabularies()
        .name_contains("vocab")
        .send()
        .await
        .expect("list with name_contains should succeed");

    assert_eq!(resp.vocabularies().len(), 2);
}

// --- Transcription Jobs ---

#[tokio::test]
async fn test_start_and_get_transcription_job() {
    let client = make_client().await;

    let start_resp = client
        .start_transcription_job()
        .transcription_job_name("my-job")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .media(
            aws_sdk_transcribe::types::Media::builder()
                .media_file_uri("s3://bucket/audio.mp3")
                .build(),
        )
        .send()
        .await
        .expect("start_transcription_job should succeed");

    let job = start_resp.transcription_job().unwrap();
    assert_eq!(job.transcription_job_name().unwrap(), "my-job");
    assert_eq!(
        job.transcription_job_status().unwrap(),
        &aws_sdk_transcribe::types::TranscriptionJobStatus::Completed,
    );

    let get_resp = client
        .get_transcription_job()
        .transcription_job_name("my-job")
        .send()
        .await
        .expect("get_transcription_job should succeed");

    let job = get_resp.transcription_job().unwrap();
    assert_eq!(job.transcription_job_name().unwrap(), "my-job");
    assert_eq!(
        job.language_code().unwrap(),
        &aws_sdk_transcribe::types::LanguageCode::EnUs,
    );
}

#[tokio::test]
async fn test_delete_transcription_job() {
    let client = make_client().await;

    client
        .start_transcription_job()
        .transcription_job_name("delete-job")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .media(
            aws_sdk_transcribe::types::Media::builder()
                .media_file_uri("s3://bucket/audio.mp3")
                .build(),
        )
        .send()
        .await
        .expect("start should succeed");

    client
        .delete_transcription_job()
        .transcription_job_name("delete-job")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_transcription_job()
        .transcription_job_name("delete-job")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_transcription_jobs() {
    let client = make_client().await;

    for name in ["job-a", "job-b"] {
        client
            .start_transcription_job()
            .transcription_job_name(name)
            .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
            .media(
                aws_sdk_transcribe::types::Media::builder()
                    .media_file_uri("s3://bucket/audio.mp3")
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_transcription_jobs()
        .send()
        .await
        .expect("list_transcription_jobs should succeed");

    assert_eq!(resp.transcription_job_summaries().len(), 2);
}

// --- Medical Transcription Jobs ---

#[tokio::test]
async fn test_start_and_get_medical_transcription_job() {
    let client = make_client().await;

    let start_resp = client
        .start_medical_transcription_job()
        .medical_transcription_job_name("med-job")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .media(
            aws_sdk_transcribe::types::Media::builder()
                .media_file_uri("s3://bucket/medical-audio.mp3")
                .build(),
        )
        .output_bucket_name("my-output-bucket")
        .specialty(aws_sdk_transcribe::types::Specialty::Primarycare)
        .r#type(aws_sdk_transcribe::types::Type::Conversation)
        .send()
        .await
        .expect("start_medical_transcription_job should succeed");

    let job = start_resp.medical_transcription_job().unwrap();
    assert_eq!(job.medical_transcription_job_name().unwrap(), "med-job");
    assert_eq!(
        job.transcription_job_status().unwrap(),
        &aws_sdk_transcribe::types::TranscriptionJobStatus::Completed,
    );

    let get_resp = client
        .get_medical_transcription_job()
        .medical_transcription_job_name("med-job")
        .send()
        .await
        .expect("get_medical_transcription_job should succeed");

    let job = get_resp.medical_transcription_job().unwrap();
    assert_eq!(job.medical_transcription_job_name().unwrap(), "med-job");
    assert_eq!(
        job.specialty().unwrap(),
        &aws_sdk_transcribe::types::Specialty::Primarycare,
    );
}

#[tokio::test]
async fn test_delete_medical_transcription_job() {
    let client = make_client().await;

    client
        .start_medical_transcription_job()
        .medical_transcription_job_name("del-med-job")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .media(
            aws_sdk_transcribe::types::Media::builder()
                .media_file_uri("s3://bucket/audio.mp3")
                .build(),
        )
        .output_bucket_name("bucket")
        .specialty(aws_sdk_transcribe::types::Specialty::Primarycare)
        .r#type(aws_sdk_transcribe::types::Type::Conversation)
        .send()
        .await
        .expect("start should succeed");

    client
        .delete_medical_transcription_job()
        .medical_transcription_job_name("del-med-job")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_medical_transcription_job()
        .medical_transcription_job_name("del-med-job")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_medical_transcription_jobs() {
    let client = make_client().await;

    for name in ["med-job-a", "med-job-b"] {
        client
            .start_medical_transcription_job()
            .medical_transcription_job_name(name)
            .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
            .media(
                aws_sdk_transcribe::types::Media::builder()
                    .media_file_uri("s3://bucket/audio.mp3")
                    .build(),
            )
            .output_bucket_name("bucket")
            .specialty(aws_sdk_transcribe::types::Specialty::Primarycare)
            .r#type(aws_sdk_transcribe::types::Type::Conversation)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_medical_transcription_jobs()
        .send()
        .await
        .expect("list_medical_transcription_jobs should succeed");

    assert_eq!(resp.medical_transcription_job_summaries().len(), 2);
}

// --- Error cases ported from moto ---

#[tokio::test]
async fn test_get_nonexistent_transcription_job() {
    let client = make_client().await;

    let result = client
        .get_transcription_job()
        .transcription_job_name("NonexistentJobName")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_medical_transcription_job() {
    let client = make_client().await;

    let result = client
        .get_medical_transcription_job()
        .medical_transcription_job_name("NonexistentJobName")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_run_transcription_job_with_existing_job_name() {
    let client = make_client().await;

    client
        .start_transcription_job()
        .transcription_job_name("MyJob")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .media(
            aws_sdk_transcribe::types::Media::builder()
                .media_file_uri("s3://my-bucket/my-media-file.wav")
                .build(),
        )
        .send()
        .await
        .expect("first start should succeed");

    let result = client
        .start_transcription_job()
        .transcription_job_name("MyJob")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .media(
            aws_sdk_transcribe::types::Media::builder()
                .media_file_uri("s3://my-bucket/my-media-file.wav")
                .build(),
        )
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_run_medical_transcription_job_with_existing_job_name() {
    let client = make_client().await;

    client
        .start_medical_transcription_job()
        .medical_transcription_job_name("MyMedJob")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .media(
            aws_sdk_transcribe::types::Media::builder()
                .media_file_uri("s3://my-bucket/my-media-file.wav")
                .build(),
        )
        .output_bucket_name("my-output-bucket")
        .specialty(aws_sdk_transcribe::types::Specialty::Primarycare)
        .r#type(aws_sdk_transcribe::types::Type::Conversation)
        .send()
        .await
        .expect("first start should succeed");

    let result = client
        .start_medical_transcription_job()
        .medical_transcription_job_name("MyMedJob")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .media(
            aws_sdk_transcribe::types::Media::builder()
                .media_file_uri("s3://my-bucket/my-media-file.wav")
                .build(),
        )
        .output_bucket_name("my-output-bucket")
        .specialty(aws_sdk_transcribe::types::Specialty::Primarycare)
        .r#type(aws_sdk_transcribe::types::Type::Conversation)
        .send()
        .await;
    assert!(result.is_err());
}

// --- Filter by JobNameContains ported from moto ---

#[tokio::test]
async fn test_list_transcription_jobs_with_job_name_contains() {
    let client = make_client().await;

    for name in ["foo-bar", "foo-baz", "other-job"] {
        client
            .start_transcription_job()
            .transcription_job_name(name)
            .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
            .media(
                aws_sdk_transcribe::types::Media::builder()
                    .media_file_uri("s3://bucket/audio.mp3")
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_transcription_jobs()
        .job_name_contains("foo")
        .send()
        .await
        .expect("list with job_name_contains should succeed");

    assert_eq!(resp.transcription_job_summaries().len(), 2);
}

#[tokio::test]
async fn test_list_medical_transcription_jobs_with_job_name_contains() {
    let client = make_client().await;

    for name in ["med-foo", "med-bar", "other"] {
        client
            .start_medical_transcription_job()
            .medical_transcription_job_name(name)
            .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
            .media(
                aws_sdk_transcribe::types::Media::builder()
                    .media_file_uri("s3://bucket/audio.mp3")
                    .build(),
            )
            .output_bucket_name("bucket")
            .specialty(aws_sdk_transcribe::types::Specialty::Primarycare)
            .r#type(aws_sdk_transcribe::types::Type::Conversation)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_medical_transcription_jobs()
        .job_name_contains("med-")
        .send()
        .await
        .expect("list with job_name_contains should succeed");

    assert_eq!(resp.medical_transcription_job_summaries().len(), 2);
}

// --- Vocabulary with VocabularyFileUri ported from moto ---

#[tokio::test]
async fn test_create_vocabulary_with_file_uri() {
    let client = make_client().await;

    let create_resp = client
        .create_vocabulary()
        .vocabulary_name("uri-vocab")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .vocabulary_file_uri("https://s3.us-east-1.amazonaws.com/AWSDOC-EXAMPLE-BUCKET/vocab.txt")
        .send()
        .await
        .expect("create_vocabulary with file URI should succeed");

    assert_eq!(create_resp.vocabulary_name().unwrap(), "uri-vocab");
    assert_eq!(
        create_resp.vocabulary_state().unwrap(),
        &aws_sdk_transcribe::types::VocabularyState::Ready
    );

    let get_resp = client
        .get_vocabulary()
        .vocabulary_name("uri-vocab")
        .send()
        .await
        .expect("get_vocabulary should succeed");

    assert_eq!(get_resp.vocabulary_name().unwrap(), "uri-vocab");
    assert_eq!(
        get_resp.language_code().unwrap(),
        &aws_sdk_transcribe::types::LanguageCode::EnUs
    );
}

// --- Medical Vocabularies ---

#[tokio::test]
async fn test_create_and_get_medical_vocabulary() {
    let client = make_client().await;

    let create_resp = client
        .create_medical_vocabulary()
        .vocabulary_name("med-vocab")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .vocabulary_file_uri("s3://bucket/medical-vocab.txt")
        .send()
        .await
        .expect("create_medical_vocabulary should succeed");

    assert_eq!(create_resp.vocabulary_name().unwrap(), "med-vocab");
    assert_eq!(
        create_resp.vocabulary_state().unwrap(),
        &aws_sdk_transcribe::types::VocabularyState::Ready,
    );

    let get_resp = client
        .get_medical_vocabulary()
        .vocabulary_name("med-vocab")
        .send()
        .await
        .expect("get_medical_vocabulary should succeed");

    assert_eq!(get_resp.vocabulary_name().unwrap(), "med-vocab");
    assert_eq!(
        get_resp.language_code().unwrap(),
        &aws_sdk_transcribe::types::LanguageCode::EnUs,
    );
}

#[tokio::test]
async fn test_delete_medical_vocabulary() {
    let client = make_client().await;

    client
        .create_medical_vocabulary()
        .vocabulary_name("del-med-vocab")
        .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
        .vocabulary_file_uri("s3://bucket/vocab.txt")
        .send()
        .await
        .expect("create should succeed");

    client
        .delete_medical_vocabulary()
        .vocabulary_name("del-med-vocab")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_medical_vocabulary()
        .vocabulary_name("del-med-vocab")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_medical_vocabularies() {
    let client = make_client().await;

    for name in ["med-vocab-a", "med-vocab-b"] {
        client
            .create_medical_vocabulary()
            .vocabulary_name(name)
            .language_code(aws_sdk_transcribe::types::LanguageCode::EnUs)
            .vocabulary_file_uri("s3://bucket/vocab.txt")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_medical_vocabularies()
        .send()
        .await
        .expect("list_medical_vocabularies should succeed");

    assert_eq!(resp.vocabularies().len(), 2);
}
