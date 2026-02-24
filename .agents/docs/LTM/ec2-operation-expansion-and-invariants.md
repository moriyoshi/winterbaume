# EC2 Operation Expansion and Invariants

## Summary

EC2 is the largest Winterbaume service and now has near-complete SDK operation coverage plus a scenario layer that exercises cross-call invariants. The durable lesson is that large shared-state service work scales through sequential family batches, but quality depends on invariant inventory and scenario tests that go beyond single-call handler round trips.

## Key Facts

- A 2026-05-01 push took `winterbaume-ec2` from 484 / 756 SDK operations to 752 / 756.
- The work landed in 12 sequential family batches because every batch touched shared central files such as `state.rs`, `views.rs`, and `handlers.rs`.
- Final crate verification after the mass-addition pass was fmt, clippy with `-D warnings`, and `cargo test -p winterbaume-ec2 --no-fail-fast`.
- The follow-up scenario pass found four real bugs that the prior quality gate missed.
- The invariant audit follow-up found a fifth EC2 invariant bug in ENI attachment id generation, confirming the value of counter-reuse auditing.
- EC2-specific generated-code layout and feature gating still live in `ec2-crate-split-and-feature-gating.md`; this note covers behavioural expansion and invariant testing.

## Details

### Operation Expansion Shape

The 12 family batches covered CoIP and ClassicLink remnants, address and BYOIP flows, customer gateway and VPN surfaces, volume and snapshot extensions, spot/reserved/fleet/image operations, network insights, traffic mirroring, Client VPN, local gateway, route server, Verified Access, capacity-reservation extensions, transit-gateway extensions, and IPAM.

Net result:

- `crates/winterbaume-ec2/src/handlers.rs` gained state-backed dispatch arms and handlers for roughly 268 operations.
- `crates/winterbaume-ec2/src/state.rs` gained the backing collections, counters, and domain errors.
- `crates/winterbaume-ec2/src/types.rs` gained the local domain resource types.
- `crates/winterbaume-ec2/src/views.rs` gained matching state-view round trips and merge support.
- `crates/winterbaume-ec2/tests/integration_test.rs` grew to hundreds of operation-focused tests.

Reduced-semantics choices were intentional where the mock cannot reasonably emulate AWS internals: network-insights analysis completes immediately, snapshot import materialises synchronously, BYOIP authorisation signatures are not verified, BGP telemetry stays empty, cross-account pending-acceptance states may need seeded views, and catalogue-like purchase operations synthesise deterministic placeholder responses.

### Terraform Follow-Up

The same session added 18 Terraform converters for high-impact EC2 families, including IPAM, Verified Access, Traffic Mirror, transit-gateway extensions, and Network Insights. Five ignored Terraform E2E tests were added for those families, while Client VPN, Local Gateway, Route Server, and some Capacity Reservation extension converters remained deferred.

### Terraform E2E Rescue Patterns

A later Terraform E2E rescue fixed 7 of 8 EC2 failures from a provider run against `terraform-provider-aws` v6.43.0. The fixed failures exposed two repeatable backend patterns that matter for future EC2 Describe handlers:

- `Describe<Resource>` handlers must honour both explicit `<Id>.N` lists and standard `Filter.N` entries. Terraform's `tfresource.AssertSinglePtrResult` returns NotFound when the provider receives anything other than exactly one row, so a handler that returns all state can pass single-resource tests and then fail as soon as a scenario creates two matching resource families.
- Generic parent Describe handlers must iterate every subtype that shares the same AWS id namespace. `DescribeTransitGatewayAttachments` had to include VPC, peering, and connect attachments because all share `tgw-attach-...` ids.

Concrete fixes from that rescue:

| Failure family | Backend rule |
|----------------|--------------|
| Local Gateway routes and VPC associations | Real AWS pre-provisions Local Gateway route tables for Outposts. The mock auto-seeds a route table, and paired Local Gateway, when a hard-coded route-table id is first referenced. |
| ENI-heavy traffic mirror and network insights tests | `DescribeNetworkInterfaces` filters by `NetworkInterfaceId.N` instead of returning every ENI in state. |
| Transit Gateway VPC attachments | Responses include `options` and `subnetIds`; generic attachment reads include connect attachments and filter across all subtype lists. |
| IPAM scopes | `DescribeIpamScopes` filters by `IpamScopeId.N`, `Filter.N.Name=ipam-scope-id`, `ipam-id`, and `is-default`, because IPAM creation auto-seeds default-private and default-public scopes alongside user scopes. |

The remaining failure, `test_ec2_capacity_block_reservation_basic`, is upstream provider behaviour rather than a mock-layer fix. The framework-SDK resource `aws_ec2_capacity_block_reservation` calls `fwflex.Flatten(ctx, cr, &data)` without `WithFieldNamePrefix` or explicit field aliases. AutoFlex therefore cannot bridge AWS SDK fields `CapacityReservationArn`, `CreateDate`, and `TotalInstanceCount` to framework model fields `ARN`, `CreatedDate`, and `InstanceCount`. The legacy SDK-v2 `aws_ec2_capacity_reservation` resource reads the same Winterbaume response correctly. The Terraform provider acceptance test is gated by `RUN_EC2_CAPACITY_BLOCK_RESERVATION_TESTS=true` and real GPU capacity, so provider CI apparently does not exercise it.

The mock should not try to add extra XML names for this case. The Smithy model defines the SDK wire names, and the SDK deserialiser ignores non-model XML fields. Track the blocker in TODO until provider code adds a prefix mapping or explicit assignments.

### Generated Feature-Map Regeneration

When regenerating `winterbaume-ec2-generated`, always pass `--features-map tools/smithy-codegen/ec2-features.toml`. Omitting the flag drops thousands of generated `#[cfg(feature = "...")]` attributes and balloons the build matrix. A refreshed vendor model also added required fields that had to be initialised as `None` in hand-written conversion code: `ClientVpnRoute.transit_gateway_attachment_id`, `TargetNetwork.availability_zone_ids`, `TargetNetwork.availability_zones`, and `VolumeModification.operator`.

### Scenario Layer and Bugs

The `/write-tests ec2` follow-up added scenario tests and exposed four bugs:

| Class | Bug | Durable fix |
|-------|-----|-------------|
| Modify rewrites sibling state | `ModifyLaunchTemplate.SetDefaultVersion` updated the parent default version but left per-version `default_version` booleans stale. | Recompute the booleans across the version vector. |
| Default inheritance from parent | `CreateVolume` used `Size=8` when `SnapshotId` was provided and `Size` omitted. | Inherit the snapshot's `volume_size`. |
| Per-call uniqueness | `AssociateAddress` minted association ids from the EIP creation counter, so re-association reused ids. | Add a dedicated `eip_assoc` counter. |
| Toggle propagation | `EnableEbsEncryptionByDefault` did not affect later volumes that omitted `Encrypted`. | Read `state.ebs_encryption_by_default` in `CreateVolume`. |

Each fix gained a focused integration regression in addition to the scenario that found it.

### Audit Rollout and ENI Attachment IDs

The follow-up run of `.agents/bin/audit-state-fields.sh --all` produced flags only for `winterbaume-ec2`. The non-EC2 fleet mostly reported no `self.counters.*` references because the heuristic is shaped around EC2's `Ec2Counters` substruct; other crates mint IDs through UUIDs, direct counter fields, hash map insertion order, or time-derived strings.

EC2 produced 12 counter-reuse flags. Eleven were false positives that fell into two legitimate shapes:

- helper plus caller, where `next_<thing>_id` increments the counter and the create/allocate/import/provision/seed method calls the helper.
- same-namespace ID minters, where multiple operations correctly share one counter for the same AWS id family, such as `run_instances` and `request_spot_instances` both producing `i-...` ids.

The one real bug was `next_eni_attach_id`: it formatted `eni-attach-...` from `self.counters.eni` without incrementing any attachment-specific counter. Multiple `AttachNetworkInterface` calls between ENI creations could therefore return duplicate `eni-attach-...` ids. The fix added `Ec2Counters.eni_attach`, mirrored it through `CountersView` with `#[serde(default)]`, and covered it with `tests/scenario_test.rs::test_eni_attach_ids_are_unique_per_call`.

### Invariant Inventory Pattern

The scenario pass established six inventory categories for large stateful services:

- toggle propagation
- modify rewrites sibling state
- per-call uniqueness
- default inheritance from parent
- lifecycle state transitions
- cross-resource references on read

For EC2, the 12 scenarios map across all six categories. That is the model for future high-surface services: write an explicit inventory, justify `N/A` row by row, then ensure `tests/scenario_test.rs` contains at least one scenario per non-`N/A` row.

### Sequential Batch Workflow

Sequential subagent batches are the right shape when many resource families share central state and dispatch files. Parallel work would collide on the `Ec2State` struct, view conversions, and action match arms. Sequential batches let each worker build on the previous batch, reuse emerging conventions, and keep the crate compiling between batches.

Use parallel agents instead when write sets are disjoint, such as one converter module per independent service crate.

## Files

- `crates/winterbaume-ec2/src/handlers.rs` - central dispatch and handlers.
- `crates/winterbaume-ec2/src/state.rs` - resource state, counters, and invariant logic.
- `crates/winterbaume-ec2/src/types.rs` - EC2 local domain types.
- `crates/winterbaume-ec2/src/views.rs` - snapshot, restore, and merge representation.
- `crates/winterbaume-ec2/tests/integration_test.rs` - per-operation and regression tests.
- `crates/winterbaume-ec2/tests/scenario_test.rs` - chained scenario coverage.
- `crates/winterbaume-ec2-generated/src/model.rs` and `wire.rs` - regenerated EC2 Smithy types and wire helpers; regenerate with the feature map.
- `crates/winterbaume-terraform/src/converters/ec2.rs` - Terraform converter surface.
- `.agents/bin/audit-state-fields.sh` - heuristic worklist generator for invariant audits.
- `.agents/docs/LTM/ec2-crate-split-and-feature-gating.md` - EC2 generated-crate and feature-gating mechanics.

## Test Coverage

After the operation expansion, `cargo test -p winterbaume-ec2 --no-fail-fast` reported 472 passing tests. After the scenario and invariant pass, the crate reported 591 integration tests plus 12 scenario tests passing, with clippy and rustfmt clean. After the ENI attachment id fix, the crate reported 591 integration tests plus 13 scenario tests passing, with clippy and rustfmt still clean.

After the Terraform E2E rescue, the seven fixed EC2 Terraform tests passed together with neighbouring capacity-reservation and network-interface-permission tests. Per-crate clippy and `fmt --check` were clean for `winterbaume-ec2` and `winterbaume-ec2-generated`.

Per the repo rules, use:

```bash
./.agents/bin/cargo.sh clippy -p winterbaume-ec2 --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh fmt -p winterbaume-ec2 -- --check
./.agents/bin/cargo.sh test -p winterbaume-ec2 --no-fail-fast
```

Partial-feature checks remain useful for feature-gated generated-code work, but full tests run with the default `full` feature set.

## Pitfalls

- Do not treat create/list/delete lifecycle tests as a substitute for scenario tests. They usually miss cross-call invariants.
- Do not assume a successful modifier response proves read-side sibling state was updated.
- Do not share counters across distinct id families unless AWS would reuse that namespace.
- Do not assume a helper name like `next_*_id` is harmless merely because it returns a formatted id. Confirm it increments the correct id-family counter.
- Do not forget to refresh API coverage and generated README data after large operation-count changes.
- Do not edit `winterbaume-ec2-generated/src/model.rs` or `wire.rs` directly; use the generator workflow in `ec2-crate-split-and-feature-gating.md`.
- Do not regenerate EC2 generated files without `--features-map tools/smithy-codegen/ec2-features.toml`.
- Do not mask an upstream Terraform provider framework mapping bug by emitting non-Smithy XML names from the mock.
