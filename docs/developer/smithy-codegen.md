# Smithy Codegen

`tools/smithy-codegen` is the code generator that produces `model.rs` and `wire.rs` for each AWS service crate. Both files carry the header:

```rust
//! Do not edit manually. Regenerate with: smithy-codegen gen-serializers <service>
```

Never hand-edit them. If generated output is wrong, fix the generator and regenerate.

## Finding the service slug

`gen-serializers` takes a service slug — usually the crate suffix (e.g. `lambda` for `winterbaume-lambda`). Smithy model directories under `vendor/api-models-aws/models/` are not always identical to that slug, but `list-services` shows both columns:

```sh
cargo run -p smithy-codegen -- list-services
```

Example output (abbreviated, columns are `<slug>  <model-dir>`):
```
lambda                    lambda
route53                   route-53
s3                        s3
elasticloadbalancingv2    elastic-load-balancing-v2
```

The default `--models-dir` is `vendor/api-models-aws/models`; override only if you are pointing at a non-vendored checkout.

## Regenerating a single service

```sh
cargo run -p smithy-codegen -- gen-serializers <slug> \
    --output        crates/winterbaume-<crate>/src/wire.rs \
    --model-output  crates/winterbaume-<crate>/src/model.rs
```

Example for Lambda:

```sh
cargo run -p smithy-codegen -- gen-serializers lambda \
    --output        crates/winterbaume-lambda/src/wire.rs \
    --model-output  crates/winterbaume-lambda/src/model.rs
```

## Regenerating all services

There is no built-in `--all` switch; loop in shell over `list-services` instead:

```sh
# Build the generator first
cargo build -p smithy-codegen

# Then run gen-serializers for every slug
cargo run -p smithy-codegen -- list-services | awk '{print $1}' | while read slug; do
    cargo run -p smithy-codegen -- gen-serializers "$slug" \
        --output       "crates/winterbaume-${slug}/src/wire.rs" \
        --model-output "crates/winterbaume-${slug}/src/model.rs"
done
```

## What is generated

### `model.rs`

Typed Rust structs and enums derived from the Smithy operation shapes:

```rust
// model.rs (generated)
#[derive(Debug, Default)]
pub struct CreateFunctionInput {
    pub function_name: Option<String>,
    pub runtime: Option<String>,
    pub role: Option<String>,
    pub handler: Option<String>,
    // ...
}

#[derive(Debug, Default)]
pub struct CreateFunctionOutput {
    pub function_name: Option<String>,
    pub function_arn: Option<String>,
    // ...
}
```

### `wire.rs`

Serializer/deserializer functions that convert between raw `MockRequest`/`MockResponse` bytes and the typed model structs, plus a re-export of `model`:

```rust
// wire.rs (generated)
pub use super::model::*;

pub fn parse_create_function_request(req: &MockRequest) -> Result<CreateFunctionInput, ...> { ... }
pub fn serialize_create_function_response(out: &CreateFunctionOutput) -> MockResponse { ... }
pub fn error_resource_not_found(msg: &str) -> MockResponse { ... }
```

## Fixing a generated output

1. Identify the wrong behaviour (e.g. a field is missing, an XML wrapper is wrong, a timestamp format is incorrect).
2. Find the responsible code in `tools/smithy-codegen/src/gen_serializers.rs`.
3. Fix it.
4. Rebuild:
   ```sh
   cargo build -p smithy-codegen
   ```
5. Regenerate the affected service(s):
   ```sh
   cargo run -p smithy-codegen -- gen-serializers <slug> \
       --output crates/winterbaume-<crate>/src/wire.rs \
       --model-output crates/winterbaume-<crate>/src/model.rs
   ```
6. Run the service's integration tests to confirm:
   ```sh
   cargo test -p winterbaume-{svc}
   ```

## Protocol-specific quirks

The generator handles several protocol-specific serialization differences:

| Protocol | Quirk |
|---|---|
| `awsQuery` | Responses wrapped in `<{Action}Response>` / `<{Action}Result>` |
| `ec2Query` | Same as awsQuery but EC2 uses different list-member encoding |
| `restXml` | XML bodies, container wrapper names from Smithy `@xmlName` trait |
| `awsJson1.0` / `awsJson1.1` | JSON bodies, `X-Amz-Target` header routing |
| `restJson1` | JSON bodies, path/method routing, no Target header |

Fixes to these quirks belong in the generator so they are applied consistently to all services sharing that protocol.

## When NOT to regenerate

- If `model.rs` or `wire.rs` have no bugs, there is no need to regenerate just because you are working on that service.
- If you add a new operation to a service, add the handler manually in `handlers.rs` and only regenerate if the existing generated types do not cover the new operation's shapes.
