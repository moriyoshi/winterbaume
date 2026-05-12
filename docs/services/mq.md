# winterbaume-mq

Amazon MQ service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | MQ |
| AWS model | `mq` |
| Protocol | restJson1 |
| winterbaume coverage | 23/24 operations (95.8%) |
| stubs (routed, returns empty/default) | 1/24 operations (4.2%) |
| moto coverage | 19/24 operations (79.2%) |
| floci coverage | 0/24 operations (0.0%) |
| kumo coverage | 6/24 operations (25.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws mq list-brokers
```

## Current Network Resource Stub Semantics

Amazon MQ currently treats broker networking as broker metadata.

- Broker model fields include subnet IDs and security groups, but current create/describe paths return simplified stored values and often default networking lists to empty.
- Pending security groups are local broker fields and are not the result of EC2 security group mutation.
- Broker endpoints and deployment state are independent of subnet availability.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_mq::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mq::MqService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(MqService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mq::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mq::Client::new(&config);

    let resp = client
        .list_brokers()
        .send()
        .await
        .expect("list_brokers should succeed");
    println!("MQ brokers: {}", resp.broker_summaries().len());
}
```

## Implemented APIs (23)

- `CreateBroker`
- `CreateConfiguration`
- `CreateTags`
- `CreateUser`
- `DeleteBroker`
- `DeleteConfiguration`
- `DeleteTags`
- `DeleteUser`
- `DescribeBroker`
- `DescribeBrokerEngineTypes`
- `DescribeConfiguration`
- `DescribeConfigurationRevision`
- `DescribeUser`
- `ListBrokers`
- `ListConfigurationRevisions`
- `ListConfigurations`
- `ListTags`
- `ListUsers`
- `Promote`
- `RebootBroker`
- `UpdateBroker`
- `UpdateConfiguration`
- `UpdateUser`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `DescribeBrokerInstanceOptions`

</details>
