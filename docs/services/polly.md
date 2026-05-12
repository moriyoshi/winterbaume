# winterbaume-polly

Polly service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Polly |
| AWS model | `polly` |
| Protocol | restJson1 |
| winterbaume coverage | 9/10 operations (90.0%) |
| stubs (routed, returns empty/default) | 0/10 operations (0.0%) |
| moto coverage | 5/10 operations (50.0%) |
| floci coverage | 0/10 operations (0.0%) |
| kumo coverage | 0/10 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws polly describe-voices
```

## Example

```rust
use aws_sdk_polly::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_polly::PollyService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(PollyService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_polly::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_polly::Client::new(&config);

    let resp = client
        .describe_voices()
        .send()
        .await
        .expect("describe_voices should succeed");
    println!("Polly voices: {}", resp.voices().len());
}
```

## Implemented APIs (9)

- `DeleteLexicon`
- `DescribeVoices`
- `GetLexicon`
- `GetSpeechSynthesisTask`
- `ListLexicons`
- `ListSpeechSynthesisTasks`
- `PutLexicon`
- `StartSpeechSynthesisTask`
- `SynthesizeSpeech`

<details><summary>Not yet implemented APIs (1)</summary>

- `StartSpeechSynthesisStream`

</details>
