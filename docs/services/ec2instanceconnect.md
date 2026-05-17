# winterbaume-ec2instanceconnect

EC2 Instance Connect service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | EC2 Instance Connect |
| AWS model | `ec2-instance-connect` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 2/2 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/2 operations (0.0%) |
| moto coverage | 1/2 operations (50.0%) |
| floci coverage | 0/2 operations (0.0%) |
| kumo coverage | 0/2 operations (0.0%) |
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
aws ec2-instance-connect send-ssh-public-key --instance-id i-00000000 --instance-os-user ec2-user --ssh-public-key file://key.pub
```

## Current Network Resource Stub Semantics

EC2 Instance Connect stores endpoint networking as service-local endpoint records.

- Endpoint state records include subnet ID, VPC ID, security group IDs, preserve-client-IP mode, DNS name, FIPS DNS name, and network interface IDs.
- Created endpoints mint their own endpoint and network-interface-looking identifiers, and describe/list responses echo the stored fields.
- Endpoint lifecycle is not reflected into EC2 network interface state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_ec2instanceconnect::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ec2instanceconnect::Ec2InstanceConnectService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Ec2InstanceConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2instanceconnect::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ec2instanceconnect::Client::new(&config);

    // EC2 Instance Connect sends SSH keys to running instances.
    // This example demonstrates client setup for the EC2 Instance Connect service.
    println!("EC2 Instance Connect client ready. Use send_ssh_public_key() to push an SSH key.");
    let _client = client;
}
```

## Implemented APIs (2)

- `SendSSHPublicKey`
- `SendSerialConsoleSSHPublicKey`
