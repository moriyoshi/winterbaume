# Winterbaume Project Overview

Winterbaume is a Rust library that provides stateful AWS service mocking for `aws-sdk-rust`. It intercepts SDK HTTP calls and routes them to in-memory service backends that simulate real AWS behaviour with persistent state. Coverage is uneven: many operations are real, state-backed implementations, while a meaningful fraction are stubs that route the request and return an empty / default response without consulting state. The headline operation count in `README.md` and `.agents/docs/API_COVERAGE.md` separates these two categories ( `Operations` versus `Stubs` ) so that callers can tell at a glance whether a given operation is genuinely implemented or merely routed.

## Scope

Winterbaume is both a Rust test library and a standalone local AWS endpoint. In library mode it supplies an `aws-smithy-runtime-api` HTTP client that can be passed directly into `aws-sdk-rust` configs. In server mode, `winterbaume-server` exposes the same service backends over HTTP for Terraform and non-Rust clients.

The project aims to be useful for realistic local workflows, not to be a bit-for-bit clone of AWS. High-value behaviour is stateful and tested against real SDK clients, moto-derived parity cases, AWS documentation plans, and Terraform provider expectations. Behaviour that depends on infrastructure Winterbaume does not emulate, such as live telemetry, organisation-wide control planes, or real execution engines, is either modelled by a dedicated engine crate or left as an explicit stub with a reason.

## Current Shape

- The workspace contains one crate per AWS service, shared infrastructure crates, optional backend crates, a standalone server, Smithy code generation tools, and Terraform E2E harnesses.
- Protocol coverage spans awsQuery, ec2Query, awsJson1.0, awsJson1.1, restJson1, restXml, and rpc-v2-cbor.
- Many service crates are fully or substantially implemented, but coverage is intentionally reported as two numbers: state-backed operations and routed stubs. `README.md` and `.agents/docs/API_COVERAGE.md` are the source of truth for current counts.
- Every service crate exposes a typed `StatefulService` view for snapshot, restore, and merge. Terraform converters, state injection, and generated documentation all rely on that stable view contract.
- Terraform support is now partly spec-driven: Terraform state resource models are generated from TOML specs into `winterbaume-tfstate-resource-models`, while hand-written converters keep ownership of state projection, nested blocks, and registration.
- Large payloads use the shared `Vfs` / `BlobStore` layer instead of living inside service state. Optional Redis and DuckDB integrations stay in separate crates so the default path remains lightweight.
- Generated Smithy `model.rs` and `wire.rs` modules are standard across mapped services. Generator fixes belong in `tools/smithy-codegen`, not in hand-edited generated files.
- Release and CI tooling are part of the project surface: cargo-dist builds the server artefacts, CI skips docs-only or source-identical work where safe, and `tools/release-batch/` preserves dependency order and chunks first-launch crates.io publishing around new-crate quota limits.

## Boundaries

Winterbaume treats AWS service boundaries deliberately:

- Some related API families should share backend state where AWS does, such as SES and SESv2.
- Some families share only narrow namespaces, such as API Gateway v1/v2 custom domains and ELB v1/v2 load balancer names.
- Some similarly named services are intentionally separate, such as MediaPackage v2 and WAFv2.
- Runtime and data-plane services should validate against their control-plane siblings unless AWS explicitly supports bring-your-own-result behaviour.
- Network-looking identifiers do not automatically imply EC2-backed validation. Service dossiers document whether a crate validates VPC, subnet, security group, VPC endpoint, VPC link, ENI, or load-balancer identifiers against another service.

Cross-service behaviour follows documented AWS integration seams rather than Winterbaume-only shortcuts. The main target seams are Lambda event sources, EventBridge rules and Pipes, AppSync data sources, Step Functions optimised integrations, and Athena's Glue catalogue relationship.

## Validation Posture

Coverage and validation are layered rather than count-driven:

- SDK integration tests prove that real `aws-sdk-rust` clients can serialize requests and deserialize Winterbaume responses.
- Scenario tests cover multi-operation workflows and cross-call invariants for stateful services.
- Moto parity ports provide behavioural examples where moto has meaningful coverage.
- AWS documentation plans keep service-specific scenario selection grounded in public contracts.
- Terraform E2E tests catch provider read-after-write, waiter, and nested-block behaviour that SDK tests often miss.

Operation-count reports are prioritisation input, not the acceptance criterion. A handler that satisfies routing but does not model state is not treated as implemented service behaviour.

## Documentation Map

- `README.md`: human-facing setup, supported-service tables, examples, and release-facing status.
- `.agents/docs/ARCHITECTURE.md`: implementation structure, runtime contracts, subsystem boundaries, and data flow.
- `.agents/docs/QUALITY_GATE.md`: implementation conventions and publication checks for service crates.
- `.agents/docs/API_COVERAGE.md`: generated operation, stub, emulator-comparison, integration-test, and Terraform E2E coverage.
- `.agents/docs/services/INDEX.md` and `.agents/docs/services/<service>.md`: service dossiers with Smithy metadata, operation inventories, AWS documentation research, local parity notes, and service-specific pitfalls.
- `.agents/docs/TODO.md`: active backlog extracted from journal and LTM maintenance.
- `.agents/docs/LTM/INDEX.md`: durable long-term memory index for detailed historical findings and synthesis documents.

## Operating Model

New service work starts from the service dossier, Smithy model, and current coverage report. Small services should usually reach full operation coverage immediately. Larger services should land a coherent root-resource lifecycle first, return explicit 501 responses for deferred families, and record the remaining work in `TODO.md`.

Documentation generated from coverage or service dossiers should be regenerated through the relevant skills and scripts rather than hand-edited inside generated blocks. Manually-authored guidance belongs in the dossier or outside generated README blocks so it survives regeneration.
