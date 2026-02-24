# winterbaume-ec2-generated

Auto-generated Smithy types and wire serialisers for the EC2 service.

This crate is part of the [winterbaume](https://github.com/moriyoshi/winterbaume) workspace — a suite of in-process AWS service mocks for Rust. It contains the model and wire-format code generated from the EC2 Smithy model and is consumed by [`winterbaume-ec2`](https://crates.io/crates/winterbaume-ec2).

You almost certainly want [`winterbaume-ec2`](https://crates.io/crates/winterbaume-ec2) instead. This crate is published only because cargo requires every dependency of a published crate to resolve from crates.io; it has no stable public API of its own and its contents are regenerated wholesale from the Smithy model.

## Regeneration

`src/model.rs` and `src/wire.rs` are produced by `tools/smithy-codegen`. To regenerate:

```sh
cargo run -p smithy-codegen -- gen-serializers ec2 \
    --output crates/winterbaume-ec2-generated/src/wire.rs \
    --model-output crates/winterbaume-ec2-generated/src/model.rs
```

## License

Licensed under Apache-2.0. See [LICENSE](https://github.com/moriyoshi/winterbaume/blob/main/LICENSE) for details.
