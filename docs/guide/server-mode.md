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

Behaviour options (all off by default):
      --s3-enforce-bucket-policies
                             Answer 403 AccessDenied for S3 requests matched by an
                             explicit Deny in the bucket policy
      --s3-enforce-session-policies
                             Answer 403 AccessDenied for S3 requests signed with
                             STS credentials whose session policy does not allow them
      --lambda-execute       Run deployed function code on Invoke, in microVMs
                             (requires: lambda-vm feature, libkrun)
      --lambda-libkrun PATH  libkrun library, when the loader does not find it
      --lambda-vm-helper PATH
                             VM helper program; default: this binary
      --lambda-agent PATH    Guest agent; default: next to this binary
      --lambda-network       Give functions this server's network access
      --lambda-image-cache-dir DIR
                             Cache of pulled Lambda base images
      --lambda-image-dir DIR Base images to use instead of pulling them
      --lambda-offline       Never pull Lambda base images
      --lambda-registry URL  Registry to pull base images from
                             default: https://public.ecr.aws
      --lambda-warm-ttl SECS Keep idle execution environments; default: 300
      --lambda-max-vms N     Most microVMs at once; default: number of CPUs
      --lambda-max-vms-per-function N
                             Most microVMs at once for one function

Relaxations (all off by default):
      --relax RULE           Accept a request AWS refuses, by rule name; repeat
                             the flag or separate names with commas
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
| `WB_S3_ENFORCE_BUCKET_POLICIES` | `--s3-enforce-bucket-policies` |
| `WB_S3_ENFORCE_SESSION_POLICIES` | `--s3-enforce-session-policies` |
| `WB_RELAX` | `--relax` (comma-separated) |
| `WB_LAMBDA_EXECUTE` | `--lambda-execute` |
| `WB_LAMBDA_LIBKRUN` | `--lambda-libkrun` |
| `WB_LAMBDA_VM_HELPER` | `--lambda-vm-helper` |
| `WB_LAMBDA_VM_AGENT` | `--lambda-agent` |
| `WB_LAMBDA_NETWORK` | `--lambda-network` |
| `WB_LAMBDA_IMAGE_CACHE_DIR` | `--lambda-image-cache-dir` |
| `WB_LAMBDA_IMAGE_DIR` | `--lambda-image-dir` |
| `WB_LAMBDA_OFFLINE` | `--lambda-offline` |
| `WB_LAMBDA_REGISTRY` | `--lambda-registry` |
| `WB_LAMBDA_WARM_TTL` | `--lambda-warm-ttl` |
| `WB_LAMBDA_MAX_VMS` | `--lambda-max-vms` |
| `WB_LAMBDA_MAX_VMS_PER_FUNCTION` | `--lambda-max-vms-per-function` |

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

# Optional: accept requests AWS refuses ( see "Relaxations" below )
relax = ["sts-assume-role-without-role-arn"]

[backends]
# Requires: --features backend-sqs-redis
sqs = "redis://127.0.0.1/"

# Requires: --features backend-dynamodb-redis
dynamodb = "redis://127.0.0.1/"

# Requires: --features backend-vfs-fs
vfs_dir = "/var/lib/winterbaume/blobs"

[s3]
# Optional: enforce explicit Deny statements of bucket policies
enforce_bucket_policies = true
# Optional: limit STS credentials to their session policy
enforce_session_policies = true

[lambda]
# Optional: run deployed function code on Invoke, in microVMs
execute         = true
libkrun         = "/usr/local/lib64/libkrun.so.1"
image_cache_dir = "/var/cache/winterbaume/lambda"
# agent                = "/usr/local/libexec/winterbaume-lambda-vm-agent"
# network              = false
# image_dir            = "/srv/lambda-images"
# offline              = false
# registry             = "https://public.ecr.aws"
# warm_ttl_secs        = 300
# max_vms              = 8
# max_vms_per_function = 4
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

## Opt-in behaviours

By default the server authorises nothing and executes nothing: every request is allowed, and `Invoke` answers with a placeholder. Two options make it behave more like AWS when a test needs that.

### `--s3-enforce-bucket-policies` — bucket policy enforcement

Enables `--s3-enforce-bucket-policies` / `WB_S3_ENFORCE_BUCKET_POLICIES` / `[s3] enforce_bucket_policies`. An S3 object or listing request that an explicit `Deny` statement of the bucket's policy matches is answered with HTTP 403 `AccessDenied`.

```sh
./winterbaume-server --s3-enforce-bucket-policies
```

- Only explicit denies are enforced. `Allow` statements and identity policies are not evaluated, so a request no `Deny` matches proceeds as before.
- Bucket configuration requests, including `PutBucketPolicy` and `DeleteBucketPolicy`, are never denied, so a test can always repair a policy that locks it out.
- The server does not authenticate callers. A `Deny` statement applies when its `Principal` is a wildcard (`"*"` or `{"AWS": "*"}`); statements naming a specific principal, using `NotPrincipal`, or conditioned on caller identity keys such as `aws:PrincipalArn` or `aws:userid` are skipped.
- The server listens on plain HTTP, so `aws:SecureTransport` is `false` unless a TLS-terminating proxy in front sets `X-Forwarded-Proto: https`. A TLS-only policy therefore denies direct requests, as it would deny plain HTTP on AWS. `aws:SourceIp` is the client address, or `X-Forwarded-For` when a proxy supplies it.

### `--s3-enforce-session-policies` — STS session policy enforcement

Enables `--s3-enforce-session-policies` / `WB_S3_ENFORCE_SESSION_POLICIES` / `[s3] enforce_session_policies`. Temporary credentials issued by `AssumeRole`, `AssumeRoleWithSAML` or `AssumeRoleWithWebIdentity` with a `Policy` ( a session policy ) are limited to what that policy allows: an S3 object or listing request signed with them is answered with HTTP 403 `AccessDenied` unless an `Allow` statement of the session policy matches it and no `Deny` does. Catalogs that vend downscoped credentials, such as Lakekeeper, check that a write outside the granted prefix fails.

```sh
./winterbaume-server --s3-enforce-session-policies
```

- Only the session policy is evaluated, not the role's own policies, so whatever the session policy allows proceeds. Requests signed with credentials that have no session policy, including every request signed with long-term keys, are not checked.
- The same requests are checked as for bucket policy enforcement, with the same actions, resources and condition keys; `s3:prefix` makes a `ListBucket` grant for one prefix work. Bucket creation, deletion and configuration requests, `ListBuckets` and `GetBucketLocation` are not checked. `DeleteObjects` is checked key by key: a refused key is reported as `AccessDenied` in the result and the others are deleted.
- The access key is read from the request's SigV4 `Authorization` header, or from `X-Amz-Credential` of a presigned URL. Signatures are not verified.
- The credentials of the `sts-assume-role-without-role-arn` relaxation carry their session policy too, as on MinIO.
- An `Allow` conditioned on caller identity keys such as `aws:PrincipalArn` cannot be decided by the server and allows nothing. A session policy that does not parse allows nothing.

In library mode, give the S3 service a `winterbaume_s3::SessionPolicyLookup`: `S3Service::new().with_session_policy_enforcement(lookup)`. `StsService::session_policy` answers the lookup for credentials an `StsService` issued.

### `--lambda-execute` — Lambda function execution

Enables `--lambda-execute` / `WB_LAMBDA_EXECUTE` / `[lambda] execute`. `Invoke` runs the deployed package in a [libkrun](https://github.com/containers/libkrun) microVM booted from the stock AWS Lambda base image of the function's runtime (`public.ecr.aws/lambda/nodejs:22`, `.../python:3.13`, `.../provided:al2023`, ...), and returns its result. The image's own runtime interface client runs the handler, so handler loading, the `context` object and the error documents (`Runtime.ImportModuleError`, `Runtime.HandlerNotFound`, ...) are those of AWS. The function's environment variables, `MemorySize` and `Timeout` apply, and errors are reported as AWS does (`X-Amz-Function-Error`, `LogType: Tail`).

```sh
./winterbaume-server --lambda-execute --lambda-libkrun /usr/local/lib64/libkrun.so.1
```

Requirements:

- A server built for Linux (x86_64 or aarch64) or macOS on arm64. Release builds for these platforms include the executor, and so does `cargo build -p winterbaume-server` there (the default `lambda-vm` feature). libkrun is loaded at run time, so the build needs no system libraries.
- libkrun 1.x and libkrunfw. Ubuntu does not package them. Install the [prebuilt libkrunfw](https://github.com/libkrun/libkrunfw/releases) and build libkrun with `make && sudo make install`. On macOS, libkrun is available from the `slp/krun` Homebrew tap. If the loader does not find the libraries, point `--lambda-libkrun` (and `LD_LIBRARY_PATH`) at them.
- Hardware virtualisation:
  - On Linux (x86_64 or aarch64), read and write access to `/dev/kvm`, for example through membership of the `kvm` group.
  - On macOS (arm64 only), the server binary must carry the `com.apple.security.hypervisor` entitlement: `codesign -s - --entitlements hv.plist ./winterbaume-server`.
- The guest agent, a static Linux binary of about 6 KB without libc, which relays the guest's ports to the host. It is built into the server binary:
  - Release builds include it.
  - A source build includes it when the Rust target for the guest is installed. On Linux that is the host's own target. On macOS it is `aarch64-unknown-linux-gnu` (`rustup target add aarch64-unknown-linux-gnu`). Without it, the build warns and the server looks for `winterbaume-lambda-vm-agent` next to itself.
  - `--lambda-agent` uses another agent binary instead.
- Network access to `public.ecr.aws` for the first invocation of each runtime, or images supplied through `--lambda-image-dir`. Pulled images are unpacked into `--lambda-image-cache-dir` (default `~/.cache/winterbaume/lambda`, or `~/Library/Caches/winterbaume/lambda` on macOS). The cache must be on a case-sensitive file system; on macOS that means a case-sensitive APFS volume.

How functions run:

- **Supported functions.** Any runtime AWS publishes a base image for, with a ZIP package uploaded inline (`ZipFile`).
- **Root file system.** The function sees the base image, read-only, with its package at `/var/task`. `/tmp` is writable but not executable, and its size is half the VM's memory rather than `EphemeralStorage`.
- **Memory and CPU.** The VM gets `MemorySize` plus 64 MB for the guest kernel, and one vCPU per 1769 MB, as Lambda allocates CPU.
- **Warm reuse.** Execution environments are kept for `--lambda-warm-ttl` seconds and reused, so module-level state survives between invocations, as on AWS. An environment that failed or timed out is discarded, and so are all environments of a function that was updated or deleted.
- **Calls back to the emulator.** Functions get `AWS_ENDPOINT_URL=http://127.0.0.1:4566` and placeholder credentials. The guest agent relays that port over vsock to a Unix socket on the host, where the server's own services answer. AWS SDK calls made inside a function therefore reach this server's state without the server's TCP port being involved. The SDKs use path-style S3 requests for an IP endpoint.
- **Isolation.** The guest cannot see the host's files. By default it has no network either: the Lambda Runtime API and the AWS endpoint, both relayed by the guest agent to Unix sockets in a private directory, are all it can reach. `--lambda-network` gives functions the server's network instead; libkrun then proxies the guest's sockets through the server's process, with the server user's access.
- **Invocation types.** `InvocationType: Event` returns 202 and runs the function once in the background, without retries or destinations. `DryRun` returns 204.
- **Architecture.** A function whose `Architectures` differs from the host's is not run: libkrun cannot emulate another instruction set. The `lambda-run-on-host-architecture` relaxation runs such functions on the host's architecture instead.
- **Not run.** Container image functions (`PackageType: Image`), functions with layers, packages referenced by `S3Bucket` / `S3Key`, and `InvokeWithResponseStream`. Function URLs and event source mappings do not invoke functions yet.

Every `Invoke` response says whether the code ran, in the `x-winterbaume-lambda-execution` header: `executed`, or `stub` for the placeholder `{}` payload. The header is present whether or not execution is enabled, so a test can tell a placeholder from a handler that returned an empty object.

A stub also carries `x-winterbaume-lambda-stub-reason`, one of:

| Reason | Meaning |
|---|---|
| `execution-disabled` | `--lambda-execute` is off |
| `no-code` | no stored package (S3 source, or state restored without its blobs) |
| `invalid-package` | the ZIP cannot be unpacked |
| `unsupported-runtime` | an unknown runtime |
| `deprecated-runtime` | a runtime without a published image |
| `image-function-unsupported` | a container image function |
| `layers-unsupported` | a function with layers |
| `architecture-unsupported` | another architecture than the host's |
| `image-unavailable` | the base image cannot be pulled or found |
| `image-cache-unusable` | the cache directory cannot be used |
| `vm-helper-not-found` | the VM helper program is missing |
| `vm-agent-not-found` | the guest agent is missing |
| `libkrun-unavailable` | libkrun cannot be loaded |
| `vm-unavailable` | no usable hypervisor |
| `response-stream-unsupported` | `InvokeWithResponseStream` |

In library mode, give the Lambda service an executor: `LambdaService::new().with_code_execution(Arc::new(KrunExecutor::new(KrunExecutorConfig::new())))`, from the `winterbaume-lambda-vm` crate. A test program then needs a VM helper: either the `winterbaume-lambda-vm-helper` binary, found next to the executable, on `PATH` or through `WB_LAMBDA_VM_HELPER`, or its own binary calling `winterbaume_lambda_vm::helper::helper_main()`. It also needs the guest agent, given with `KrunExecutorConfig::with_agent` or `WB_LAMBDA_VM_AGENT`.

For functions' AWS calls to reach the services of the same mock environment, connect an endpoint once the environment exists:

```rust
let endpoint = AwsEndpoint::new();
let executor = KrunExecutor::new(KrunExecutorConfig::new().with_aws_endpoint(endpoint.clone()));
let mock = MockAws::builder()
    .with_service(LambdaService::new().with_code_execution(Arc::new(executor)))
    .with_service(S3Service::new())
    .build();
endpoint.connect(mock.dispatcher());
```

## Relaxations

By default a service refuses what AWS refuses. A relaxation names one request that AWS rejects but that the server accepts when you enable it, usually because a client was written against another AWS-compatible server. Each relaxation is enabled on its own, by name:

```sh
./winterbaume-server --relax sts-assume-role-without-role-arn
WB_RELAX=sts-assume-role-without-role-arn ./winterbaume-server
```

- Repeat `--relax`, or separate names with commas, to enable several. `relax = [...]` in the config file does the same.
- A list from the command line or `WB_RELAX` replaces the config file's list rather than adding to it, as for every other option.
- An unknown name stops the server with an error that lists the known names. `--help` lists them too, and the server logs each enabled relaxation at startup.

| Rule | What it accepts | AWS behaviour |
|---|---|---|
| `lambda-run-on-host-architecture` | Running a function whose `Architectures` differs from the host's (`x86_64` on an arm64 host, or the reverse) on the host's architecture, with that architecture's base image. Interpreted code usually runs unchanged; native modules built for the configured architecture fail to load. The response carries `x-winterbaume-lambda-architecture-substituted: <configured>-><used>`. | (runs on the configured architecture) |
| `sts-assume-role-without-role-arn` | `AssumeRole` without `RoleArn`, as MinIO accepts it. The server issues temporary credentials that keep the caller's own identity: `GetCallerIdentity` with them answers the caller's ARN. `RoleSessionName` is still required, and the response has no `AssumedRoleUser`. Clients such as Lakekeeper send this for `"flavor": "minio"` storage profiles. | `400 MissingParameter` |

In library mode, pass the same set to the service: `StsService::new().with_relaxations(&Relaxations::new().with(winterbaume_sts::ASSUME_ROLE_WITHOUT_ROLE_ARN))`.

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
