---
layout: home

hero:
  name: "Winterbäume"
  text: "A stateful AWS service emulator for integration testing"
  tagline: Test AWS-style workflows locally, embedded in Rust tests or running as a standalone server.
  image: /hero-winter-tree.webp
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: Service Coverage
      link: /reference/services

features:
  - title: First-Class Rust SDK Support
    details: Embeds as an in-process HTTP client for aws-sdk-rust. Optionally layer aws_smithy_mocks rule overrides on top for fine-grained control.
  - title: Library or Server
    details: Embed directly in Rust tests via the aws-smithy-runtime-api HttpClient traits, or run as a standalone HTTP server for Terraform, CLI, and any AWS SDK.
  - title: Stateful
    details: State persists across requests, isolated per account ID and region — matching real AWS isolation semantics. Build and tear down resources across multiple SDK calls.
  - title: Smithy-Backed
    details: Per-service model.rs and wire.rs files are generated from AWS Smithy models, giving type-safe request parsing and response serialization across all implemented protocols.
  - title: 63% API Coverage
    details: 7210 of 11367 operations across 224 AWS services have real, state-backed behaviour, surpassing moto's 29.1%. A further 326 operations (2.9%) are stubs that route the request and return an empty/default response &mdash; clearly broken out per service. Implemented surfaces include S3, DynamoDB, SQS, Lambda, KMS, ECS, EKS, IAM, and many more.
  - title: Pluggable Backends
    details: Swap the default in-memory store for Redis or DuckDB. Opt-in, dependency-free from the core service crates, injected through with_backend().
