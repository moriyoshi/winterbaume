# winterbaume-lambda

Lambda service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Lambda |
| AWS model | `lambda` |
| Protocol | restJson1 |
| winterbaume coverage | 85/85 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/85 operations (0.0%) |
| moto coverage | 46/85 operations (54.1%) |
| floci coverage | 0/85 operations (0.0%) |
| kumo coverage | 17/85 operations (20.0%) |
| Coverage report date | 2026-05-17 |

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
aws lambda list-functions
```

## Current Network Resource Stub Semantics

Lambda currently reserves a VPC configuration slot on function state but does not provision Lambda networking.

- Function view/state has an optional `vpc_config` JSON field, and current function creation initialises it to `None` in the implemented state path.
- Update paths do not create ENIs, allocate subnet attachments, or enforce security group membership.
- Function invocation state is independent of VPC reachability.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_lambda::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_lambda::LambdaService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(LambdaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lambda::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_lambda::Client::new(&config);

    let resp = client
        .list_functions()
        .send()
        .await
        .expect("list_functions should succeed");
    println!("Lambda functions: {}", resp.functions().len());
}
```

## Implemented APIs (85)

- `AddLayerVersionPermission`
- `AddPermission`
- `CheckpointDurableExecution`
- `CreateAlias`
- `CreateCapacityProvider`
- `CreateCodeSigningConfig`
- `CreateEventSourceMapping`
- `CreateFunction`
- `CreateFunctionUrlConfig`
- `DeleteAlias`
- `DeleteCapacityProvider`
- `DeleteCodeSigningConfig`
- `DeleteEventSourceMapping`
- `DeleteFunction`
- `DeleteFunctionCodeSigningConfig`
- `DeleteFunctionConcurrency`
- `DeleteFunctionEventInvokeConfig`
- `DeleteFunctionUrlConfig`
- `DeleteLayerVersion`
- `DeleteProvisionedConcurrencyConfig`
- `GetAccountSettings`
- `GetAlias`
- `GetCapacityProvider`
- `GetCodeSigningConfig`
- `GetDurableExecution`
- `GetDurableExecutionHistory`
- `GetDurableExecutionState`
- `GetEventSourceMapping`
- `GetFunction`
- `GetFunctionCodeSigningConfig`
- `GetFunctionConcurrency`
- `GetFunctionConfiguration`
- `GetFunctionEventInvokeConfig`
- `GetFunctionRecursionConfig`
- `GetFunctionScalingConfig`
- `GetFunctionUrlConfig`
- `GetLayerVersion`
- `GetLayerVersionByArn`
- `GetLayerVersionPolicy`
- `GetPolicy`
- `GetProvisionedConcurrencyConfig`
- `GetRuntimeManagementConfig`
- `Invoke`
- `InvokeAsync`
- `InvokeWithResponseStream`
- `ListAliases`
- `ListCapacityProviders`
- `ListCodeSigningConfigs`
- `ListDurableExecutionsByFunction`
- `ListEventSourceMappings`
- `ListFunctionEventInvokeConfigs`
- `ListFunctionUrlConfigs`
- `ListFunctionVersionsByCapacityProvider`
- `ListFunctions`
- `ListFunctionsByCodeSigningConfig`
- `ListLayerVersions`
- `ListLayers`
- `ListProvisionedConcurrencyConfigs`
- `ListTags`
- `ListVersionsByFunction`
- `PublishLayerVersion`
- `PublishVersion`
- `PutFunctionCodeSigningConfig`
- `PutFunctionConcurrency`
- `PutFunctionEventInvokeConfig`
- `PutFunctionRecursionConfig`
- `PutFunctionScalingConfig`
- `PutProvisionedConcurrencyConfig`
- `PutRuntimeManagementConfig`
- `RemoveLayerVersionPermission`
- `RemovePermission`
- `SendDurableExecutionCallbackFailure`
- `SendDurableExecutionCallbackHeartbeat`
- `SendDurableExecutionCallbackSuccess`
- `StopDurableExecution`
- `TagResource`
- `UntagResource`
- `UpdateAlias`
- `UpdateCapacityProvider`
- `UpdateCodeSigningConfig`
- `UpdateEventSourceMapping`
- `UpdateFunctionCode`
- `UpdateFunctionConfiguration`
- `UpdateFunctionEventInvokeConfig`
- `UpdateFunctionUrlConfig`
