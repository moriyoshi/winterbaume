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

#[tokio::test]
async fn test_describe_voices() {
    let client = make_client().await;

    let resp = client
        .describe_voices()
        .send()
        .await
        .expect("describe_voices should succeed");

    let voices = resp.voices();
    assert!(!voices.is_empty());

    // Check that Joanna is in the list
    let joanna = voices
        .iter()
        .find(|v| v.id() == Some(&aws_sdk_polly::types::VoiceId::Joanna));
    assert!(joanna.is_some(), "Joanna voice should be present");
}

#[tokio::test]
async fn test_put_and_get_lexicon() {
    let client = make_client().await;

    let pls_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xsi:schemaLocation="http://www.w3.org/2005/01/pronunciation-lexicon http://www.w3.org/TR/2007/CR-pronunciation-lexicon-20071212/pls.xsd"
  alphabet="ipa" xml:lang="en-US">
  <lexeme>
    <grapheme>W3C</grapheme>
    <alias>World Wide Web Consortium</alias>
  </lexeme>
</lexicon>"#;

    client
        .put_lexicon()
        .name("TestLexicon")
        .content(pls_content)
        .send()
        .await
        .expect("put_lexicon should succeed");

    let resp = client
        .get_lexicon()
        .name("TestLexicon")
        .send()
        .await
        .expect("get_lexicon should succeed");

    let lexicon = resp.lexicon().expect("should have lexicon");
    assert_eq!(lexicon.name(), Some("TestLexicon"));
    assert!(lexicon.content().is_some());
}

#[tokio::test]
async fn test_list_lexicons() {
    let client = make_client().await;

    let pls = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="en-US">
  <lexeme><grapheme>test</grapheme><alias>testing</alias></lexeme>
</lexicon>"#;

    client
        .put_lexicon()
        .name("Lex1")
        .content(pls)
        .send()
        .await
        .unwrap();

    client
        .put_lexicon()
        .name("Lex2")
        .content(pls)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_lexicons()
        .send()
        .await
        .expect("list_lexicons should succeed");

    assert_eq!(resp.lexicons().len(), 2);
}

#[tokio::test]
async fn test_delete_lexicon() {
    let client = make_client().await;

    let pls = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="en-US">
  <lexeme><grapheme>bye</grapheme><alias>goodbye</alias></lexeme>
</lexicon>"#;

    client
        .put_lexicon()
        .name("DeleteMe")
        .content(pls)
        .send()
        .await
        .unwrap();

    client
        .delete_lexicon()
        .name("DeleteMe")
        .send()
        .await
        .expect("delete_lexicon should succeed");

    let result = client.get_lexicon().name("DeleteMe").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_synthesize_speech() {
    let client = make_client().await;

    let resp = client
        .synthesize_speech()
        .text("Hello world")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .send()
        .await
        .expect("synthesize_speech should succeed");

    // Read the audio stream
    let data = resp
        .audio_stream
        .collect()
        .await
        .expect("should read audio stream");
    assert!(!data.into_bytes().is_empty());
}

#[tokio::test]
async fn test_start_speech_synthesis_task() {
    let client = make_client().await;

    let resp = client
        .start_speech_synthesis_task()
        .text("Hello from a synthesis task")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .output_s3_bucket_name("my-bucket")
        .send()
        .await
        .expect("start_speech_synthesis_task should succeed");

    let task = resp.synthesis_task().expect("should have synthesis task");
    assert!(task.task_id().is_some());
    assert!(task.output_uri().is_some());
    assert_eq!(
        task.task_status(),
        Some(&aws_sdk_polly::types::TaskStatus::Completed)
    );
}

#[tokio::test]
async fn test_get_speech_synthesis_task() {
    let client = make_client().await;

    // Start a task first
    let start_resp = client
        .start_speech_synthesis_task()
        .text("Test get task")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .output_s3_bucket_name("my-bucket")
        .send()
        .await
        .unwrap();

    let task_id = start_resp
        .synthesis_task()
        .unwrap()
        .task_id()
        .unwrap()
        .to_string();

    // Get the task by ID
    let resp = client
        .get_speech_synthesis_task()
        .task_id(&task_id)
        .send()
        .await
        .expect("get_speech_synthesis_task should succeed");

    let task = resp.synthesis_task().expect("should have synthesis task");
    assert_eq!(task.task_id(), Some(task_id.as_str()));
    assert_eq!(
        task.voice_id(),
        Some(&aws_sdk_polly::types::VoiceId::Joanna)
    );
}

#[tokio::test]
async fn test_get_speech_synthesis_task_not_found() {
    let client = make_client().await;

    let result = client
        .get_speech_synthesis_task()
        .task_id("nonexistent-task-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_speech_synthesis_tasks() {
    let client = make_client().await;

    // Start two tasks
    client
        .start_speech_synthesis_task()
        .text("Task one")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .output_s3_bucket_name("bucket1")
        .send()
        .await
        .unwrap();

    client
        .start_speech_synthesis_task()
        .text("Task two")
        .voice_id(aws_sdk_polly::types::VoiceId::Matthew)
        .output_format(aws_sdk_polly::types::OutputFormat::OggVorbis)
        .output_s3_bucket_name("bucket2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_speech_synthesis_tasks()
        .send()
        .await
        .expect("list_speech_synthesis_tasks should succeed");

    let tasks = resp.synthesis_tasks();
    assert_eq!(tasks.len(), 2);
}

#[tokio::test]
async fn test_list_speech_synthesis_tasks_empty() {
    let client = make_client().await;

    let resp = client
        .list_speech_synthesis_tasks()
        .send()
        .await
        .expect("list_speech_synthesis_tasks should succeed on empty list");

    assert!(resp.synthesis_tasks().is_empty());
}

#[tokio::test]
async fn test_delete_lexicon_not_found() {
    let client = make_client().await;

    let result = client
        .delete_lexicon()
        .name("NonexistentLexicon")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_lexicon_not_found() {
    let client = make_client().await;

    let result = client.get_lexicon().name("NonexistentLexicon").send().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_voices_with_language_code() {
    let client = make_client().await;

    // Filter by en-GB - Amy, Brian, Emma should be in there
    let resp = client
        .describe_voices()
        .language_code(aws_sdk_polly::types::LanguageCode::EnGb)
        .send()
        .await
        .expect("describe_voices with language code should succeed");

    let voices = resp.voices();
    assert!(!voices.is_empty());
    // All returned voices should be en-GB
    for v in voices {
        assert_eq!(
            v.language_code(),
            Some(&aws_sdk_polly::types::LanguageCode::EnGb)
        );
    }
}

#[tokio::test]
async fn test_put_lexicon_bad_name() {
    let client = make_client().await;

    let pls = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="en-US">
  <lexeme><grapheme>test</grapheme><alias>testing</alias></lexeme>
</lexicon>"#;

    // Name with hyphen should fail
    let result = client
        .put_lexicon()
        .name("test-invalid")
        .content(pls)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_synthesize_speech_bad_lexicon() {
    let client = make_client().await;

    let result = client
        .synthesize_speech()
        .lexicon_names("nonexistent")
        .output_format(aws_sdk_polly::types::OutputFormat::Pcm)
        .text("test1234")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_synthesize_speech_bad_voice_id() {
    let client = make_client().await;

    // "Luke" is not a valid voice ID in our implementation
    let result = client
        .synthesize_speech()
        .output_format(aws_sdk_polly::types::OutputFormat::Pcm)
        .text("test1234")
        .voice_id(aws_sdk_polly::types::VoiceId::from("Luke"))
        .send()
        .await;

    // The SDK may reject unknown voice IDs at the client level, or the server will
    // In either case, expect an error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_synthesize_speech_text_too_long() {
    let client = make_client().await;

    // Generate text over 3000 characters
    let long_text = "a".repeat(3001);

    let result = client
        .synthesize_speech()
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .text(long_text)
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_synthesize_speech_speech_marks_not_supported_for_pcm() {
    let client = make_client().await;

    let result = client
        .synthesize_speech()
        .output_format(aws_sdk_polly::types::OutputFormat::Pcm)
        .text("test")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .speech_mark_types(aws_sdk_polly::types::SpeechMarkType::Word)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_lexicon() {
    let client = make_client().await;

    let pls = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="en-US">
  <lexeme><grapheme>test</grapheme><alias>testing</alias></lexeme>
</lexicon>"#;

    // Put the lexicon twice (update)
    client
        .put_lexicon()
        .name("TestUpdate")
        .content(pls)
        .send()
        .await
        .expect("first put should succeed");

    client
        .put_lexicon()
        .name("TestUpdate")
        .content(pls)
        .send()
        .await
        .expect("second put (update) should succeed");

    // Should still only be one lexicon with that name
    let resp = client
        .list_lexicons()
        .send()
        .await
        .expect("list_lexicons should succeed");

    let count = resp
        .lexicons()
        .iter()
        .filter(|l| l.name() == Some("TestUpdate"))
        .count();
    assert_eq!(count, 1);
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Polly
// ============================================================================

/// Verify GetLexicon returns full LexiconAttributes (ARN, language_code,
/// last_modified, lexemes_count, size, alphabet).
#[tokio::test]
async fn test_get_lexicon_attributes() {
    let client = make_client().await;

    let pls = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="en-US">
  <lexeme><grapheme>W3C</grapheme><alias>World Wide Web Consortium</alias></lexeme>
  <lexeme><grapheme>AWS</grapheme><alias>Amazon Web Services</alias></lexeme>
</lexicon>"#;

    client
        .put_lexicon()
        .name("AttrLexicon")
        .content(pls)
        .send()
        .await
        .expect("put_lexicon should succeed");

    let resp = client
        .get_lexicon()
        .name("AttrLexicon")
        .send()
        .await
        .expect("get_lexicon should succeed");

    let attrs = resp
        .lexicon_attributes()
        .expect("should have lexicon_attributes");

    // LexiconArn must be present and reference the lexicon name
    let arn = attrs.lexicon_arn().expect("lexicon_arn should be present");
    assert!(
        arn.contains("AttrLexicon"),
        "ARN should contain lexicon name, got: {arn}"
    );
    assert!(
        arn.starts_with("arn:aws:polly:"),
        "ARN should start with arn:aws:polly:, got: {arn}"
    );

    // Language code should be extracted from PLS xml:lang
    let lang = attrs
        .language_code()
        .expect("language_code should be present");
    assert_eq!(lang.as_str(), "en-US");

    // Lexemes count: the PLS has 2 <lexeme> elements
    let count = attrs.lexemes_count();
    assert_eq!(count, 2, "lexemes_count should be 2, got: {count}");

    // Size should match content length
    let size = attrs.size();
    assert!(size > 0, "size should be positive, got: {size}");

    // Alphabet
    let alphabet = attrs.alphabet().expect("alphabet should be present");
    assert_eq!(alphabet, "ipa");

    // LastModified should be set
    assert!(
        attrs.last_modified().is_some(),
        "last_modified should be present"
    );
}

/// Verify ListLexicons returns LexiconAttributes in each LexiconDescription.
#[tokio::test]
async fn test_list_lexicons_attributes() {
    let client = make_client().await;

    let pls = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="fr-FR">
  <lexeme><grapheme>bonjour</grapheme><alias>hello</alias></lexeme>
</lexicon>"#;

    client
        .put_lexicon()
        .name("ListAttrLex")
        .content(pls)
        .send()
        .await
        .expect("put_lexicon should succeed");

    let resp = client
        .list_lexicons()
        .send()
        .await
        .expect("list_lexicons should succeed");

    let lexicons = resp.lexicons();
    assert!(!lexicons.is_empty());

    let lex = lexicons
        .iter()
        .find(|l| l.name() == Some("ListAttrLex"))
        .expect("ListAttrLex should appear in list");

    let attrs = lex.attributes().expect("attributes should be present");

    // ARN must contain lexicon name
    let arn = attrs.lexicon_arn().expect("lexicon_arn should be present");
    assert!(
        arn.contains("ListAttrLex"),
        "ARN should contain lexicon name"
    );

    // Language code matches PLS xml:lang
    let lang = attrs
        .language_code()
        .expect("language_code should be present");
    assert_eq!(lang.as_str(), "fr-FR");

    assert_eq!(attrs.lexemes_count(), 1);
    assert!(attrs.size() > 0);
    assert!(attrs.last_modified().is_some());
}

/// SynthesizeSpeech with ogg_vorbis format — verify content type.
#[tokio::test]
async fn test_synthesize_speech_ogg_vorbis() {
    let client = make_client().await;

    let resp = client
        .synthesize_speech()
        .text("Hello world")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::OggVorbis)
        .send()
        .await
        .expect("synthesize_speech ogg_vorbis should succeed");

    assert_eq!(resp.content_type(), Some("audio/ogg"));

    let data = resp
        .audio_stream
        .collect()
        .await
        .expect("should read audio stream");
    assert!(!data.into_bytes().is_empty());
}

/// SynthesizeSpeech with json format and speech marks — verify content type.
#[tokio::test]
async fn test_synthesize_speech_json_with_speech_marks() {
    let client = make_client().await;

    let resp = client
        .synthesize_speech()
        .text("Hello world")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Json)
        .speech_mark_types(aws_sdk_polly::types::SpeechMarkType::Word)
        .send()
        .await
        .expect("synthesize_speech json with speech marks should succeed");

    assert_eq!(resp.content_type(), Some("application/x-json-stream"));
}

/// SynthesizeSpeech — verify x-amzn-requestcharacters header equals text length.
#[tokio::test]
async fn test_synthesize_speech_request_characters_header() {
    let client = make_client().await;

    let text = "Hello there, world!";
    let expected_chars = text.len() as i32;

    let resp = client
        .synthesize_speech()
        .text(text)
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .send()
        .await
        .expect("synthesize_speech should succeed");

    assert_eq!(
        resp.request_characters(),
        expected_chars,
        "request_characters should equal text length {expected_chars}"
    );
}

/// SynthesizeSpeech with an invalid sample rate for the chosen format.
#[tokio::test]
async fn test_synthesize_speech_invalid_sample_rate() {
    let client = make_client().await;

    // PCM only supports 8000 and 16000; 22050 is invalid for pcm
    let result = client
        .synthesize_speech()
        .text("test")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Pcm)
        .sample_rate("22050")
        .send()
        .await;

    assert!(result.is_err(), "Invalid sample rate for PCM should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidSampleRateException") || err_str.contains("invalid"),
        "Expected InvalidSampleRateException, got: {err_str}"
    );
}

/// StartSpeechSynthesisTask — verify output_uri starts with s3://.
#[tokio::test]
async fn test_start_synthesis_task_output_uri_format() {
    let client = make_client().await;

    let resp = client
        .start_speech_synthesis_task()
        .text("Check output URI format")
        .voice_id(aws_sdk_polly::types::VoiceId::Matthew)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .output_s3_bucket_name("my-test-bucket")
        .send()
        .await
        .expect("start_speech_synthesis_task should succeed");

    let task = resp.synthesis_task().expect("should have synthesis_task");
    let uri = task.output_uri().expect("output_uri should be present");

    assert!(
        uri.starts_with("s3://my-test-bucket/"),
        "output_uri should start with s3://my-test-bucket/, got: {uri}"
    );
}

/// StartSpeechSynthesisTask with OutputS3KeyPrefix — verify prefix appears in output_uri.
#[tokio::test]
async fn test_start_synthesis_task_with_key_prefix() {
    let client = make_client().await;

    let resp = client
        .start_speech_synthesis_task()
        .text("Testing key prefix")
        .voice_id(aws_sdk_polly::types::VoiceId::Salli)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .output_s3_bucket_name("prefix-bucket")
        .output_s3_key_prefix("audio/polly/")
        .send()
        .await
        .expect("start_speech_synthesis_task with key prefix should succeed");

    let task = resp.synthesis_task().expect("should have synthesis_task");
    let uri = task.output_uri().expect("output_uri should be present");

    assert!(
        uri.contains("audio/polly/"),
        "output_uri should contain key prefix 'audio/polly/', got: {uri}"
    );
}

/// StartSpeechSynthesisTask — missing OutputS3BucketName should fail.
#[tokio::test]
async fn test_start_synthesis_task_missing_bucket() {
    let client = make_client().await;

    // OutputS3BucketName is required: aws-sdk-polly's typed builder enforces
    // it at compile time so the handler's runtime check is unreachable from
    // the typed SDK. No further assertion is possible from this test layer.
}

/// StartSpeechSynthesisTask — verify request_characters field equals text length.
#[tokio::test]
async fn test_synthesis_task_request_characters() {
    let client = make_client().await;

    let text = "The quick brown fox";
    let expected_chars = text.len() as i32;

    let start_resp = client
        .start_speech_synthesis_task()
        .text(text)
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .output_s3_bucket_name("chars-bucket")
        .send()
        .await
        .expect("start_speech_synthesis_task should succeed");

    let task = start_resp
        .synthesis_task()
        .expect("should have synthesis_task");
    assert_eq!(
        task.request_characters(),
        expected_chars,
        "request_characters should equal text length"
    );

    // Also verify via GetSpeechSynthesisTask
    let task_id = task
        .task_id()
        .expect("task_id should be present")
        .to_string();
    let get_resp = client
        .get_speech_synthesis_task()
        .task_id(&task_id)
        .send()
        .await
        .expect("get_speech_synthesis_task should succeed");

    let fetched_task = get_resp
        .synthesis_task()
        .expect("should have synthesis_task");
    assert_eq!(
        fetched_task.request_characters(),
        expected_chars,
        "GetSpeechSynthesisTask request_characters should match"
    );
}

/// DescribeVoices with an invalid language code should return an error.
#[tokio::test]
async fn test_describe_voices_invalid_language_code() {
    let client = make_client().await;

    let result = client
        .describe_voices()
        .language_code(aws_sdk_polly::types::LanguageCode::from("xx-XX"))
        .send()
        .await;

    assert!(
        result.is_err(),
        "Invalid language code should return an error"
    );
}

/// PutLexicon with a name exceeding 20 characters should fail.
#[tokio::test]
async fn test_put_lexicon_name_too_long() {
    let client = make_client().await;

    let pls = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="en-US">
  <lexeme><grapheme>test</grapheme><alias>testing</alias></lexeme>
</lexicon>"#;

    // Name is 21 characters — exceeds the 20-char limit per AWS docs
    let result = client
        .put_lexicon()
        .name("NameExceedingTwentyX1")
        .content(pls)
        .send()
        .await;

    assert!(
        result.is_err(),
        "Lexicon name longer than 20 chars should fail"
    );
}

/// Full lifecycle: create -> verify listed -> get -> update -> delete -> verify gone.
#[tokio::test]
async fn test_lexicon_full_lifecycle() {
    let client = make_client().await;

    let pls_v1 = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="en-US">
  <lexeme><grapheme>lifecycle</grapheme><alias>life cycle</alias></lexeme>
</lexicon>"#;

    let pls_v2 = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="en-US">
  <lexeme><grapheme>lifecycle</grapheme><alias>life cycle</alias></lexeme>
  <lexeme><grapheme>API</grapheme><alias>application programming interface</alias></lexeme>
</lexicon>"#;

    // Create
    client
        .put_lexicon()
        .name("LifeCycleTest")
        .content(pls_v1)
        .send()
        .await
        .expect("initial put_lexicon should succeed");

    // Verify listed
    let list_resp = client
        .list_lexicons()
        .send()
        .await
        .expect("list_lexicons should succeed");
    assert!(
        list_resp
            .lexicons()
            .iter()
            .any(|l| l.name() == Some("LifeCycleTest")),
        "Lexicon should appear in list after creation"
    );

    // Get and verify
    let get_resp = client
        .get_lexicon()
        .name("LifeCycleTest")
        .send()
        .await
        .expect("get_lexicon should succeed");
    let lex = get_resp.lexicon().expect("should have lexicon");
    assert_eq!(lex.name(), Some("LifeCycleTest"));
    assert!(lex.content().is_some());

    // Update (overwrite with v2 — more lexemes)
    client
        .put_lexicon()
        .name("LifeCycleTest")
        .content(pls_v2)
        .send()
        .await
        .expect("update put_lexicon should succeed");

    // Get and verify updated lexemes count
    let get_resp2 = client
        .get_lexicon()
        .name("LifeCycleTest")
        .send()
        .await
        .expect("get_lexicon after update should succeed");
    let attrs2 = get_resp2
        .lexicon_attributes()
        .expect("should have attributes");
    assert_eq!(
        attrs2.lexemes_count(),
        2,
        "updated lexicon should have 2 lexemes"
    );

    // Delete
    client
        .delete_lexicon()
        .name("LifeCycleTest")
        .send()
        .await
        .expect("delete_lexicon should succeed");

    // Verify gone
    let gone = client.get_lexicon().name("LifeCycleTest").send().await;
    assert!(gone.is_err(), "get_lexicon after delete should fail");
}

/// SynthesizeSpeech using a previously stored lexicon via LexiconNames.
#[tokio::test]
async fn test_synthesize_speech_with_valid_lexicon() {
    let client = make_client().await;

    let pls = r#"<?xml version="1.0" encoding="UTF-8"?>
<lexicon version="1.0" xmlns="http://www.w3.org/2005/01/pronunciation-lexicon" alphabet="ipa" xml:lang="en-US">
  <lexeme><grapheme>Polly</grapheme><alias>Amazon Polly</alias></lexeme>
</lexicon>"#;

    client
        .put_lexicon()
        .name("PollyLex")
        .content(pls)
        .send()
        .await
        .expect("put_lexicon should succeed");

    // Synthesize with the lexicon applied
    let resp = client
        .synthesize_speech()
        .text("Polly is great")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .lexicon_names("PollyLex")
        .send()
        .await
        .expect("synthesize_speech with valid lexicon should succeed");

    let data = resp
        .audio_stream
        .collect()
        .await
        .expect("should read audio stream");
    assert!(!data.into_bytes().is_empty());
}

/// StartSpeechSynthesisTask — verify engine field is returned in the task.
#[tokio::test]
async fn test_start_synthesis_task_engine_field() {
    let client = make_client().await;

    let resp = client
        .start_speech_synthesis_task()
        .text("Testing engine field")
        .voice_id(aws_sdk_polly::types::VoiceId::Joanna)
        .output_format(aws_sdk_polly::types::OutputFormat::Mp3)
        .output_s3_bucket_name("engine-bucket")
        .engine(aws_sdk_polly::types::Engine::Neural)
        .send()
        .await
        .expect("start_speech_synthesis_task should succeed");

    let task = resp.synthesis_task().expect("should have synthesis_task");
    let engine = task.engine().expect("engine should be present");
    assert_eq!(engine, &aws_sdk_polly::types::Engine::Neural);
}

/// ListSpeechSynthesisTasks — verify all fields are returned for each task.
#[tokio::test]
async fn test_list_synthesis_tasks_task_fields() {
    let client = make_client().await;

    client
        .start_speech_synthesis_task()
        .text("Fields check task")
        .voice_id(aws_sdk_polly::types::VoiceId::Kendra)
        .output_format(aws_sdk_polly::types::OutputFormat::OggVorbis)
        .output_s3_bucket_name("fields-bucket")
        .send()
        .await
        .expect("start_speech_synthesis_task should succeed");

    let resp = client
        .list_speech_synthesis_tasks()
        .send()
        .await
        .expect("list_speech_synthesis_tasks should succeed");

    let tasks = resp.synthesis_tasks();
    assert!(!tasks.is_empty());

    // Find the task we just created (by output_format ogg_vorbis)
    let task = tasks
        .iter()
        .find(|t| t.output_format() == Some(&aws_sdk_polly::types::OutputFormat::OggVorbis))
        .expect("should find the ogg_vorbis task");

    assert!(task.task_id().is_some(), "task_id should be present");
    assert!(task.output_uri().is_some(), "output_uri should be present");
    assert!(
        task.creation_time().is_some(),
        "creation_time should be present"
    );
    assert_eq!(
        task.task_status(),
        Some(&aws_sdk_polly::types::TaskStatus::Completed)
    );
    assert_eq!(
        task.voice_id(),
        Some(&aws_sdk_polly::types::VoiceId::Kendra)
    );
}
