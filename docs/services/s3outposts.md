# winterbaume-s3outposts

AWS S3 on Outposts service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | S3 on Outposts |
| AWS model | `s3outposts` |
| Protocol | restJson1 |
| winterbaume coverage | 3/5 operations (60.0%) |
| stubs (routed, returns empty/default) | 1/5 operations (20.0%) |
| moto coverage | 0/5 operations (0.0%) |
| floci coverage | 0/5 operations (0.0%) |
| kumo coverage | 0/5 operations (0.0%) |
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
aws s3outposts help
```

## Current Network Resource Stub Semantics

S3 Outposts currently stores endpoint networking as endpoint-local metadata.

- Endpoint creation requires subnet ID and security group ID, then mints a synthetic VPC ID and network interface ID from the endpoint ID.
- Describe/list responses return the stored subnet, security group, VPC, and network interface fields.
- Endpoint lifecycle does not create EC2 network interfaces or validate Outpost subnet placement.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_s3outposts::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3outposts::S3OutpostsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3OutpostsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3outposts::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3outposts::Client::new(&config);
    let resp = client
        .list_outposts_with_s3()
        .send()
        .await
        .expect("list_outposts_with_s3 should succeed");
    for o in resp.outposts() {
        println!(
            "Outpost: {:?} ({} bytes)",
            o.outpost_id(),
            o.capacity_in_bytes()
        );
    }
}
```

## Implemented APIs (3)

- `CreateEndpoint`
- `DeleteEndpoint`
- `ListEndpoints`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `ListSharedEndpoints`

</details>

<details><summary>Not yet implemented APIs (1)</summary>

- `ListOutpostsWithS3`

</details>
