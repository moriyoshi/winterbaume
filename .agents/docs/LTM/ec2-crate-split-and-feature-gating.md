# Working on `winterbaume-ec2` (split crate + feature gating)

EC2 is the largest service in the workspace. It is split into two crates and the auto-generated code is feature-gated so that partial-feature builds stay fast during development. This is the only service in the workspace organised this way; expect the layout to be unfamiliar relative to other service crates.

## Crate layout

```
crates/
├── winterbaume-ec2-generated/   # auto-generated only, ~121k lines
│   ├── Cargo.toml               # has [features]: full + 5 groups
│   └── src/
│       ├── lib.rs               # `pub mod model; pub mod wire;`
│       ├── model.rs             # ~72k lines, regenerated
│       └── wire.rs              # ~50k lines, regenerated
└── winterbaume-ec2/             # hand-written + thin re-exports
    ├── Cargo.toml               # mirrors [features], passes through to -generated
    └── src/
        ├── lib.rs               # `pub use winterbaume_ec2_generated::{model, wire};`
        ├── handlers.rs          # gated dispatch + per-handler #[cfg]
        ├── state.rs              src/types.rs   src/views.rs
        └── ...
```

`winterbaume-ec2/Cargo.toml` does NOT use `winterbaume-ec2-generated = { workspace = true }` for the dep. Workspace inheritance forbids overriding `default-features`, so we use a direct `path = "../winterbaume-ec2-generated", default-features = false` entry instead. Leave the workspace-dependencies entry alone for any future external consumers.

The root `Cargo.toml` has `[profile.dev.package.winterbaume-ec2-generated]` overrides (`codegen-units = 16`, `opt-level = 0`) so cold rebuilds of the generated crate parallelise across LLVM units.

## Feature taxonomy

Every operation maps to exactly one Cargo feature slug. See `tools/smithy-codegen/ec2-features.toml` for the authoritative mapping.

| feature | scope | approx ops |
|---------|-------|-----------|
| `network` | VPCs, subnets, security groups, ENIs, route tables, IGW/NAT, ACLs, VPC endpoints, peering, prefix lists, elastic IPs, DHCP options, flow logs, IPv4/IPv6 CIDRs | 238 |
| `compute` | Instances, AMIs, key pairs, launch templates, placement groups, dedicated hosts, capacity reservations, account attributes, monitoring | 175 |
| `storage` | EBS volumes, snapshots, FastSnapshot, FPGA images | 55 |
| `advanced-network` | Transit gateway, VPN, Client VPN, customer gateways, local gateways | 139 |
| `extras` | catch-all (spot, IPAM, traffic mirroring, verified access, outposts, etc.) | 149 |

The `default = ["full"]` and `full = ["advanced-network", "compute", "extras", "network", "storage"]` make published-crate behaviour identical to the unsplit crate.

## Build-time workflows

| command | typical wall time | when to use |
|---------|-------------------|-------------|
| `cargo check -p winterbaume-ec2` | 7+ min cold, ~20 s warm | full validation, integration tests, CI |
| `cargo check -p winterbaume-ec2 --no-default-features --features network` | ~50 s cold | iterating on networking handlers |
| `cargo check -p winterbaume-ec2 --no-default-features --features storage` | ~3 min cold | iterating on EBS handlers |
| `cargo check -p winterbaume-ec2 --no-default-features --features compute,storage` | combine slugs | working across compute and storage |

Always go through `.agents/bin/cargo.sh`, not bare `cargo`. Tests must run with default features (= `full`); partial-feature builds skip the corresponding tests because the handlers are not present.

## How to add a new operation

The mechanical steps are the same as for any other crate; the EC2-specific extras are:

1. **Decide the feature slug.** Look at the operation name and pick the matching bucket from the table above. Edit `tools/smithy-codegen/ec2-features.toml` and add the entry under `[operations]`. If the operation does not fit any of the five buckets, leave it absent — `default_feature = "extras"` will catch it.
2. **Regenerate** model.rs and wire.rs (see "Regeneration command" below).
3. **Add the handler** to `crates/winterbaume-ec2/src/handlers.rs`. Two `#[cfg]` insertions are needed:
   - Above the new `async fn handle_<op>` — `#[cfg(feature = "<slug>")]`.
   - Above the new arm inside the `match action {` dispatch block — `#[cfg(feature = "<slug>")]`.
4. **If the handler shares a name with another (e.g. `handle_classic_link_noop`)**, the cfg above the function definition must use `cfg(any(feature = "X", feature = "Y"))` covering every operation that calls it.
5. **If the handler is a `fn` outside the `impl Ec2Service` block** (e.g. a `*_to_model` helper), gate it with the same `#[cfg(feature = "<slug>")]` as its callers. The 27 existing free helpers are classified in `.agents-workspace/tmp/gate_helpers.py`.
6. **Verify** with both the full build and the relevant partial build:
   ```bash
   .agents/bin/cargo.sh check -p winterbaume-ec2
   .agents/bin/cargo.sh check -p winterbaume-ec2 --no-default-features --features <slug>
   .agents/bin/cargo.sh test -p winterbaume-ec2 -- --test-threads=4
   ```

## Regeneration command

```bash
.agents/bin/cargo.sh run -p smithy-codegen -- gen-serializers ec2 \
    --output crates/winterbaume-ec2-generated/src/wire.rs \
    --model-output crates/winterbaume-ec2-generated/src/model.rs \
    --features-map tools/smithy-codegen/ec2-features.toml
```

The header comment at the top of the regenerated files records this command verbatim. After regeneration, the codegen prints the suggested `[features]` block on stderr — it should match the existing one in `winterbaume-ec2-generated/Cargo.toml`. If a feature was added or removed, update both `winterbaume-ec2-generated/Cargo.toml` and `winterbaume-ec2/Cargo.toml` so the slug exists in `[features]`.

## How to add a new feature group

If the existing five buckets become unwieldy, you can add a new slug:

1. Add the slug to `tools/smithy-codegen/ec2-features.toml` under `[operations]`, mapping the relevant operations.
2. Add the slug to `[features]` in `winterbaume-ec2-generated/Cargo.toml` (`new-slug = []` plus include it in `full`).
3. Add the pass-through entry in `winterbaume-ec2/Cargo.toml` (`new-slug = ["winterbaume-ec2-generated/new-slug"]` plus include in its `full`).
4. Regenerate.
5. Re-run the `.agents-workspace/tmp/gate_handlers.py` script after backing up `handlers.rs`. The script reads the TOML and re-derives the cfg attributes.

## Codegen internals you may need to touch

The mechanism lives in two files:

- `tools/smithy-codegen/src/features.rs` — owns `FeatureMap` (loaded from TOML) and `compute_shape_features` (transitive shape→feature reverse-mapping). The reverse mapping tags both struct shape FQNs and any list/set/map FQN that appears as a member target — this is required so that the synthesised `Xml*List` and `*Set` wrapper structs can be gated correctly.
- `tools/smithy-codegen/src/gen_serializers.rs` — emits `#[cfg(feature = "...")]` before each `serialize_*_response*` and `deserialize_*_request*` function, before each `pub struct`, and before each XML wrapper (and its `impl` blocks).

The rule applied to per-shape gates (`cfg_attr_for_shape`):

| feature set size | emitted cfg |
|------------------|-------------|
| 0 (unreachable from any operation) | none — emitted unconditionally |
| 1 | `#[cfg(feature = "X")]` |
| ≥2 (shared core) | none — emitted unconditionally |

There is no `cfg(any(...))` for shapes; if a type is needed by multiple feature groups it is just always present. This keeps the generated output readable and avoids combinatorial cfg explosion.

## Common pitfalls

- **List/set wrapper missing cfg.** Smithy list and set shapes are emitted as Rust struct wrappers (`TerminateConnectionStatusSet`, `XmlVpcList`, etc.). Their cfg gating depends on `compute_shape_features` tagging the list shape's FQN, not just the element struct's FQN. If you change `compute_shape_features` and a partial-feature build suddenly produces "cannot find type X" errors in the thousands, this is almost always the culprit.
- **Workspace inheritance and `default-features`.** Cargo will reject `winterbaume-ec2-generated = { workspace = true, default-features = false }` because the workspace dep does not declare `default-features = true`. Use a direct path dep in `winterbaume-ec2/Cargo.toml`. Do not "fix" the workspace entry — other downstream consumers may rely on it.
- **The `crate::model::*` glob in handlers.rs.** Bulk-imports every type with `#[allow(unused_imports)]`. Any partial-feature build will quietly drop types behind disabled features without breaking the import. A handful of `as Model*` aliases needed for collision-avoidance with `crate::types` are gated with `#[cfg(feature = "network")]`.
- **`cargo run -p smithy-codegen` rebuilds the tool.** First-run after a codegen-tool change can take ~50 s. Subsequent regenerations are fast.
- **`API_COVERAGE.md` reads moto coverage from `winterbaume-ec2`, not from the generated crate.** Coverage counting still works the way it does for every other crate — the split is invisible to that machinery.

## Files referenced from this doc

- `tools/smithy-codegen/ec2-features.toml` — operation→feature mapping (authoritative)
- `tools/smithy-codegen/src/features.rs` — TOML loader + transitive shape feature computation
- `tools/smithy-codegen/src/gen_serializers.rs` — emits `#[cfg]` attributes
- `crates/winterbaume-ec2-generated/Cargo.toml` — `[features]` definitions
- `crates/winterbaume-ec2/Cargo.toml` — pass-through `[features]`, direct path dep
- `crates/winterbaume-ec2/src/handlers.rs` — gated dispatch and handlers
- `.agents-workspace/tmp/classify_ec2_ops.py` — bootstrap script that produced the initial mapping
- `.agents-workspace/tmp/gate_handlers.py` — script that gated `handlers.rs` (re-runnable)
- `.agents-workspace/tmp/gate_helpers.py` — script that gated free helper fns (re-runnable)
