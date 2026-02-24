/// Scenario tests for winterbaume-polly.
///
/// Each test covers an end-to-end use-case that chains 3+ operations.
use aws_sdk_polly::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_polly::PollyService;

async fn make_client() -> aws_sdk_polly::Client {
    let mock = MockAws::builder().with_service(PollyService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_polly::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_polly::Client::new(&config)
}

/// Scenario: Custom-lexicon speech synthesis pipeline.
///
/// 1. Upload a PLS lexicon.
/// 2. Synthesise speech using that lexicon — verify the call succeeds and
///    returns audio bytes.
/// 3. Delete the lexicon.
/// 4. Synthesise speech again referencing the deleted lexicon — verify it
///    now fails with a `LexiconNotFoundException`.
#[tokio::test]
async fn test_custom_lexicon_speech_pipeline() {
    let client = make_client().await;

    let pls = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon"
  alphabet="ipa" xml:lang="en-US">
  <lexeme>
    <grapheme>AWS</grapheme>
    <alias>Amazon Web Services</alias>
  </lexeme>
</lexicon>"#;

    // Step 1: Upload lexicon.
    client
        .put_lexicon()
        .name("AWSLex")
        .content(pls)
        .send()
        .await
        .expect("put_lexicon should succeed");

    // Step 2: Synthesise with the lexicon applied.
    let resp = client
        .synthesize_speech()
        .text("AWS is great")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .lexicon_names("AWSLex")
        .send()
        .await
        .expect("synthesize_speech with valid lexicon should succeed");

    let bytes = resp
        .audio_stream
        .collect()
        .await
        .expect("should read audio stream")
        .into_bytes();
    assert!(!bytes.is_empty(), "audio bytes should be non-empty");

    // Step 3: Delete the lexicon.
    client
        .delete_lexicon()
        .name("AWSLex")
        .send()
        .await
        .expect("delete_lexicon should succeed");

    // Step 4: Synthesising with the deleted lexicon must fail.
    let err = client
        .synthesize_speech()
        .text("AWS is great")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .lexicon_names("AWSLex")
        .send()
        .await;

    assert!(
        err.is_err(),
        "synthesize_speech referencing deleted lexicon should fail"
    );
    let err_str = format!("{:?}", err.unwrap_err());
    assert!(
        err_str.contains("LexiconNotFoundException") || err_str.contains("not found"),
        "expected LexiconNotFoundException, got: {err_str}"
    );
}

/// Scenario: Asynchronous synthesis task lifecycle.
///
/// 1. Start a speech synthesis task.
/// 2. Retrieve the task by ID — verify it is in the `Completed` state with
///    the correct output URI format.
/// 3. List all tasks — verify the task appears.
/// 4. Start a second task with a different voice; list and verify both tasks
///    are present.
#[tokio::test]
async fn test_synthesis_task_lifecycle() {
    let client = make_client().await;

    // Step 1: Start first synthesis task.
    let start1 = client
        .start_speech_synthesis_task()
        .text("Hello from task one")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .output_s3_bucket_name("pipeline-bucket")
        .output_s3_key_prefix("audio/")
        .send()
        .await
        .expect("start_speech_synthesis_task (1) should succeed");

    let task1 = start1.synthesis_task().expect("should have synthesis_task");
    let task_id1 = task1
        .task_id()
        .expect("task_id should be present")
        .to_string();

    // Step 2: Retrieve the task by ID.
    let get_resp = client
        .get_speech_synthesis_task()
        .task_id(&task_id1)
        .send()
        .await
        .expect("get_speech_synthesis_task should succeed");

    let fetched = get_resp
        .synthesis_task()
        .expect("should have synthesis_task");
    assert_eq!(fetched.task_id(), Some(task_id1.as_str()));
    assert_eq!(
        fetched.task_status(),
        Some(&aws_sdk_polly::types::TaskStatus::Completed),
        "task should be Completed"
    );
    let uri = fetched.output_uri().expect("output_uri should be present");
    assert!(
        uri.starts_with("s3://pipeline-bucket/audio/"),
        "output_uri should use bucket and prefix, got: {uri}"
    );

    // Step 3: List — task1 should appear.
    let list1 = client
        .list_speech_synthesis_tasks()
        .send()
        .await
        .expect("list_speech_synthesis_tasks should succeed");
    assert_eq!(
        list1.synthesis_tasks().len(),
        1,
        "one task should be listed"
    );
    assert_eq!(
        list1.synthesis_tasks()[0].task_id(),
        Some(task_id1.as_str())
    );

    // Step 4: Start a second task with a different voice.
    client
        .start_speech_synthesis_task()
        .text("Hello from task two")
        .voice_id(aws_sdk_polly::types::VoiceId::Matthew)
        .output_format(aws_sdk_polly::types::OutputFormat::OggVorbis)
        .output_s3_bucket_name("pipeline-bucket")
        .send()
        .await
        .expect("start_speech_synthesis_task (2) should succeed");

    let list2 = client
        .list_speech_synthesis_tasks()
        .send()
        .await
        .expect("list_speech_synthesis_tasks (2) should succeed");
    assert_eq!(
        list2.synthesis_tasks().len(),
        2,
        "two tasks should be listed after starting a second one"
    );
}
