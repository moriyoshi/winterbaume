# tf-converter-codegen

Code-generator for the serde-driven projection layer of winterbaume's
Terraform converters. Each per-service spec under
`crates/winterbaume-terraform/specs/<service>.toml` produces a sibling
`crates/winterbaume-terraform/src/generated/<service>.rs` containing:

- A `*TfModel` struct with `#[derive(Serialize, Deserialize)]` and per-field
  `rename` / `default` annotations driven by the spec.
- `into_state_view(self, ctx, region) -> {View}` for the inject direction
  (applies ARN/URL templates from the spec).
- `From<&{View}> for {Model}` for the extract direction (covers the
  `computed_extract` entries).

Decode and encode at call sites use plain `serde_json::from_value` and
`serde_json::to_value`. Hand-written converters under
`crates/winterbaume-terraform/src/converters/<service>.rs` keep ownership of
the trait scaffolding, the `do_inject` / `do_extract` orchestration, ARN
templates that cannot be expressed as `{region}/{account}/{name}`
substitution, multi-resource composition, and any nested-block reshaping
(the AppFlow-class pitfall).

## Subcommands

```
cargo run -p tf-converter-codegen -- gen sqs           # one service
cargo run -p tf-converter-codegen -- gen-all           # every spec
cargo run -p tf-converter-codegen -- check             # CI staleness gate
cargo run -p tf-converter-codegen -- list              # list known specs
```

Default paths assume the workspace root as the working directory.

## Spec format

See the comments in `tools/tf-converter-codegen/src/spec.rs` and the worked
example at `crates/winterbaume-terraform/specs/sqs.toml`. The recognised
field types are `string`, `string?`, `u32`, `i64`, `bool`, and `tags`
(`HashMap<String, String>`).
