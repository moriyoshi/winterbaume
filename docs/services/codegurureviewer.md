# winterbaume-codegurureviewer

AWS CodeGuru Reviewer service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CodeGuru Reviewer |
| AWS model | `codeguru-reviewer` |
| Protocol | restJson1 |
| winterbaume coverage | 9/14 operations (64.3%) |
| stubs (routed, returns empty/default) | 0/14 operations (0.0%) |
| moto coverage | 0/14 operations (0.0%) |
| floci coverage | 0/14 operations (0.0%) |
| kumo coverage | 11/14 operations (78.6%) |
| fakecloud coverage | 0/14 operations (0.0%) |
| Coverage report date | 2026-07-03 |

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
aws codegurureviewer help
```

## Example

```rust
use aws_sdk_codegurureviewer::config::BehaviorVersion;
use aws_sdk_codegurureviewer::types::{CodeCommitRepository, Repository};
use winterbaume_codegurureviewer::CodeGuruReviewerService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeGuruReviewerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codegurureviewer::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codegurureviewer::Client::new(&config);
    let repo = Repository::builder()
        .code_commit(
            CodeCommitRepository::builder()
                .name("demo-repo")
                .build()
                .expect("cc"),
        )
        .build();
    let resp = client
        .associate_repository()
        .repository(repo)
        .send()
        .await
        .expect("associate");
    if let Some(arn) = resp
        .repository_association()
        .and_then(|a| a.association_arn())
    {
        println!("Association: {arn}");
    }
}
```

## Implemented APIs (9)

- `AssociateRepository`
- `CreateCodeReview`
- `DescribeCodeReview`
- `DescribeRepositoryAssociation`
- `DisassociateRepository`
- `ListCodeReviews`
- `ListRecommendations`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (5)</summary>

- `DescribeRecommendationFeedback` (implemented by kumo)
- `ListRecommendationFeedback` (implemented by kumo)
- `ListRepositoryAssociations` (implemented by kumo)
- `ListTagsForResource`
- `PutRecommendationFeedback` (implemented by kumo)

</details>
