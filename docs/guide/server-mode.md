# Server Mode

`winterbaume-server` is a standalone HTTP server that exposes all winterbaume service backends over a real TCP socket. Use it when you need to point non-Rust clients, CLI tools, or Terraform configurations at a mock AWS endpoint.

## Starting the server

The quickest way is to grab a pre-built executable from the [GitHub releases page](https://github.com/moriyoshi/winterbaume/releases/latest). Tarballs and a Windows zip are published for macOS (Apple Silicon and Intel), Linux (arm64 and x64), and Windows (x64). Extract the archive and run the binary:

```sh
./winterbaume-server [OPTIONS]
```

Alternatively, install from crates.io:

```sh
cargo install winterbaume-server
winterbaume-server [OPTIONS]
```

Or, from a checkout of this repository, run it directly through cargo:

```sh
cargo run -p winterbaume-server -- [OPTIONS]
```

The server listens on `127.0.0.1:5555` by default. Pass `--help` to print all options.

## CLI reference

```
Usage: winterbaume-server [OPTIONS]

Options:
  -h, --host HOST            Bind address            default: 127.0.0.1
  -p, --port PORT            Bind port               default: 5555
      --account-id ID        Mock AWS account ID     default: 123456789012
      --region REGION        Default AWS region      default: us-east-1
      --tfstate FILE         Load a Terraform state file at startup
      --config FILE          Load a TOML configuration file

Backend options:
      --sqs-backend URL      Redis URL for SQS persistence
                             (requires: backend-sqs-redis feature)
      --dynamodb-backend URL Redis URL for DynamoDB persistence
                             (requires: backend-dynamodb-redis feature)
      --vfs-dir DIR          Filesystem directory for S3/Glacier/EBS blob storage
                             (requires: backend-vfs-fs feature)
```

## Environment variables

Every option has a corresponding environment variable. Variables are applied after the config file but before CLI flags.

| Variable | Equivalent flag |
|---|---|
| `WB_HOST` | `--host` |
| `WB_PORT` | `--port` |
| `WB_ACCOUNT_ID` | `--account-id` |
| `WB_REGION` | `--region` |
| `WB_TFSTATE` | `--tfstate` |
| `WB_CONFIG` | `--config` |
| `WB_SQS_BACKEND` | `--sqs-backend` |
| `WB_DYNAMODB_BACKEND` | `--dynamodb-backend` |
| `WB_VFS_DIR` | `--vfs-dir` |

**Configuration precedence:** CLI flags > environment variables > config file > built-in defaults.

## Config file

Pass `--config FILE` (or set `WB_CONFIG`) to load a TOML file. Any value omitted in the file falls back to environment variables or built-in defaults.

```toml
# winterbaume.toml
host       = "0.0.0.0"
port       = 5555
account_id = "123456789012"
region     = "us-east-1"

# Optional: load Terraform state at startup
tfstate = "/path/to/terraform.tfstate"

[backends]
# Requires: --features backend-sqs-redis
sqs = "redis://127.0.0.1/"

# Requires: --features backend-dynamodb-redis
dynamodb = "redis://127.0.0.1/"

# Requires: --features backend-vfs-fs
vfs_dir = "/var/lib/winterbaume/blobs"
```

```sh
winterbaume-server --config winterbaume.toml
```

## Logging

Set `RUST_LOG` to control log verbosity:

```sh
RUST_LOG=info  winterbaume-server   # startup messages + per-request method/path
RUST_LOG=debug winterbaume-server   # full request/response detail
RUST_LOG=warn  winterbaume-server   # errors and warnings only (default is silent)
```

## Pointing clients at the server

### AWS CLI

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export AWS_DEFAULT_REGION=us-east-1

aws s3 mb s3://my-bucket
aws s3 ls
aws sqs create-queue --queue-name my-queue
aws dynamodb list-tables
aws lambda list-functions
```

### Python (boto3)

```python
import boto3

session = boto3.Session(
    aws_access_key_id="test",
    aws_secret_access_key="test",
    region_name="us-east-1",
)

endpoint = "http://localhost:5555"

s3  = session.client("s3",       endpoint_url=endpoint)
sqs = session.client("sqs",      endpoint_url=endpoint)
ddb = session.client("dynamodb", endpoint_url=endpoint)

s3.create_bucket(Bucket="my-bucket")
print(s3.list_buckets()["Buckets"])

sqs.create_queue(QueueName="my-queue")
print(sqs.list_queues())
```

### Terraform

```hcl
provider "aws" {
  region                      = "us-east-1"
  access_key                  = "test"
  secret_key                  = "test"
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true

  endpoints {
    s3             = "http://localhost:5555"
    iam            = "http://localhost:5555"
    sts            = "http://localhost:5555"
    sqs            = "http://localhost:5555"
    dynamodb       = "http://localhost:5555"
    lambda         = "http://localhost:5555"
    kms            = "http://localhost:5555"
    secretsmanager = "http://localhost:5555"
    # ... add any other service you use
  }
}
```

### aws-sdk-rust (server mode)

```rust
use aws_config::BehaviorVersion;

let config = aws_config::defaults(BehaviorVersion::latest())
    .endpoint_url("http://localhost:5555")
    .credentials_provider(aws_credential_types::provider::SharedCredentialsProvider::new(
        aws_credential_types::Credentials::new("test", "test", None, None, "static"),
    ))
    .region("us-east-1")
    .load()
    .await;

let s3 = aws_sdk_s3::Client::new(&config);
```

## Terraform state injection (`--tfstate`)

Pass a Terraform state file at startup to pre-populate service backends with resources from an existing Terraform deployment:

```sh
winterbaume-server --tfstate ./terraform.tfstate
```

The server reads the state file, extracts all supported resource types, and injects them into the corresponding service backends before the first request is served. This is useful for standing up a mock environment that mirrors a real deployed infrastructure.

**Supported Terraform resource types include:** `aws_s3_bucket`, `aws_iam_role`, `aws_iam_user`, `aws_iam_policy`, `aws_iam_group`, `aws_lambda_function`, `aws_lambda_alias`, `aws_dynamodb_table`, `aws_sqs_queue`, `aws_sns_topic`, `aws_kms_key`, `aws_secretsmanager_secret`, `aws_acm_certificate`, `aws_eks_cluster`, `aws_ecs_cluster`, `aws_ecs_service`, `aws_ecs_task_definition`, `aws_ecr_repository`, `aws_kinesis_stream`, `aws_cloudwatch_log_group`, `aws_sfn_state_machine`, `aws_lb`, `aws_cognito_user_pool`, `aws_route53_zone`, `aws_elasticache_cluster`, `aws_elasticache_replication_group`, `aws_cloudformation_stack`, and many more.

Resources whose type is not registered are silently skipped. Unsupported resource types do not cause errors.

## Persistent backends (optional compile-time features)

By default, all service state is stored in memory and lost when the server stops. Three optional Cargo features enable persistence:

### `backend-vfs-fs` — filesystem blob storage

Enables `--vfs-dir DIR` / `WB_VFS_DIR`. S3 objects, EBS snapshot data, and Glacier archives are stored as files on disk instead of in memory. The server can restart and retain all large-payload data.

```sh
cargo build -p winterbaume-server --features backend-vfs-fs
./winterbaume-server --vfs-dir /var/lib/winterbaume/blobs
```

### `backend-sqs-redis` — Redis-backed SQS

Enables `--sqs-backend URL` / `WB_SQS_BACKEND`. SQS queue state (messages, visibility timeouts, dead-letter queues) is stored in Redis.

```sh
cargo build -p winterbaume-server --features backend-sqs-redis
./winterbaume-server --sqs-backend redis://127.0.0.1/
```

### `backend-dynamodb-redis` — Redis-backed DynamoDB

Enables `--dynamodb-backend URL` / `WB_DYNAMODB_BACKEND`. DynamoDB table and item state is stored in Redis.

```sh
cargo build -p winterbaume-server --features backend-dynamodb-redis
./winterbaume-server --dynamodb-backend redis://127.0.0.1/
```

### All persistence features

```sh
cargo build -p winterbaume-server \
  --features backend-vfs-fs,backend-sqs-redis,backend-dynamodb-redis
```

When a backend option is set but the corresponding feature was not compiled in, the server starts with the in-memory fallback and logs a warning. It does not exit.

## Routing

The server uses two routing strategies, tried in order:

1. **URL pattern matching** — each service registers regex patterns on the request host (e.g. `s3\..*\.amazonaws\.com`). The first match wins.
2. **SigV4 credential scope** — if no pattern matches, the service name is extracted from the `Authorization` header's credential scope (e.g. `..../s3/aws4_request`).

A request that matches no service returns `404 UnknownService`.

## State lifecycle

State is held in memory and is isolated per `(account_id, region)` pair. All state is cleared when the server process exits unless a persistent backend is configured. To reset state without restarting, stop and restart the server.

## When to use server mode vs library mode

| Scenario | Recommendation |
|---|---|
| Rust tests using `aws-sdk-rust` | Library mode — zero network overhead |
| Python / Go / JS SDK tests | Server mode |
| AWS CLI scripts | Server mode |
| Terraform provider testing | Server mode |
| CI with multiple processes sharing state | Server mode |
| State that must survive process restarts | Server mode + persistence features |
