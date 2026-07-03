# winterbaume-s3vectors

S3 Vectors service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | S3 Vectors |
| AWS model | `s3vectors` |
| Protocol | restJson1 |
| winterbaume coverage | 19/19 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/19 operations (0.0%) |
| moto coverage | 15/19 operations (78.9%) |
| floci coverage | 12/19 operations (63.2%) |
| kumo coverage | 0/19 operations (0.0%) |
| fakecloud coverage | 0/19 operations (0.0%) |
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
aws s3vectors list-vector-buckets
```

## Example

```rust
use aws_sdk_s3vectors::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3vectors::S3VectorsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3VectorsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3vectors::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3vectors::Client::new(&config);

    // Create a vector bucket
    let resp = client
        .create_vector_bucket()
        .vector_bucket_name("my-vector-bucket")
        .send()
        .await
        .expect("create_vector_bucket should succeed");

    println!(
        "Created vector bucket ARN: {}",
        resp.vector_bucket_arn().unwrap_or_default()
    );

    // Get the vector bucket
    let get_resp = client
        .get_vector_bucket()
        .vector_bucket_name("my-vector-bucket")
        .send()
        .await
        .expect("get_vector_bucket should succeed");

    let bucket = get_resp.vector_bucket().expect("should have vector_bucket");
    println!("Vector bucket name: {}", bucket.vector_bucket_name());
    println!("Vector bucket ARN: {}", bucket.vector_bucket_arn());

    // List vector buckets
    let list_resp = client
        .list_vector_buckets()
        .send()
        .await
        .expect("list_vector_buckets should succeed");

    println!("Total vector buckets: {}", list_resp.vector_buckets().len());
}
```

## Implemented APIs (19)

- `CreateIndex`
- `CreateVectorBucket`
- `DeleteIndex`
- `DeleteVectorBucket`
- `DeleteVectorBucketPolicy`
- `DeleteVectors`
- `GetIndex`
- `GetVectorBucket`
- `GetVectorBucketPolicy`
- `GetVectors`
- `ListIndexes`
- `ListTagsForResource`
- `ListVectorBuckets`
- `ListVectors`
- `PutVectorBucketPolicy`
- `PutVectors`
- `QueryVectors`
- `TagResource`
- `UntagResource`
