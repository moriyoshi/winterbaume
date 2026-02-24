# Developer Guide

This section covers the internals of winterbaume for contributors who want to implement new services, extend the code generator, write tests, or add pluggable backends.

## Contents

- [Architecture](./architecture) — component model, request flow, core traits
- [Adding a Service](./adding-a-service) — step-by-step guide for new service crates
- [Smithy Codegen](./smithy-codegen) — generating `model.rs` and `wire.rs` from Smithy models
- [Testing](./testing) — integration tests, moto parity ports, Terraform E2E suites
- [Backends](./backends) — implementing pluggable storage and query-execution backends

## Quick orientation

```
winterbaume/
  crates/
    winterbaume-core/     # shared traits, BackendState, MockAws builder, Vfs/BlobStore
    winterbaume-{svc}/    # one crate per AWS service
    winterbaume-server/   # standalone HTTP server (hyper)
    winterbaume-stubs/    # 501 fallback for unmapped services
    winterbaume-*-redis/  # optional Redis persistence crates
    winterbaume-sqlengine-duckdb/  # optional DuckDB query backend
    winterbaume-partiql/  # DynamoDB PartiQL parser
    winterbaume-terraform/ # Terraform provider compatibility
  tools/
    smithy-codegen/       # generates model.rs + wire.rs per service
  vendor/
    moto/                 # reference Python moto (git submodule)
  tests/
    e2e/terraform/        # Terraform E2E test suites
```

## Where to start

| Goal | Start here |
|---|---|
| Understand the overall design | [Architecture](./architecture) |
| Add a new AWS service | [Adding a Service](./adding-a-service) |
| Fix a generated serialiser | [Smithy Codegen](./smithy-codegen) |
| Add tests for a service | [Testing](./testing) |
| Add a Redis or DuckDB backend | [Backends](./backends) |
