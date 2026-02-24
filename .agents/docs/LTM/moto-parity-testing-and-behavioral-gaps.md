# Moto Parity Testing and Behavioral Gaps

## Summary

Moto parity work established that API surface coverage is not enough. Winterbaume reached reliable behavioural quality only after translating moto's Python tests into Rust integration tests, then fixing subtle mismatches in defaults, error codes, field presence, idempotency, and validation.

## Key Facts

- Moto tests became the practical behavioural spec for many AWS-compatible mock behaviors.
- Early smoke tests often passed while handlers still differed materially from AWS and moto behaviour.
- By late March 2026, the project had ported parity coverage across 55+ services, with 1,600+ cumulative tests and hundreds of bugs fixed across sessions.
- EC2 later became a useful parity example in the opposite direction: Winterbaume expanded to 214 implemented operations, exceeding moto's practical EC2 surface while still using parity-style lifecycle tests as a behavioural floor.
- Common bug classes were wrong default values, missing response fields, incorrect idempotency semantics, mismatched error messages or typed exceptions, and missing validation.
- New operation work should begin from existing moto tests whenever moto covers the service.

## Details

### Why parity porting mattered

The journal repeatedly found handlers that appeared functional but were behaviourally wrong. Typical examples included:

- returning the wrong error type while still returning an error
- omitting fields that the SDK expects to deserialize
- using default values that are plausible but not AWS-compatible
- treating idempotent deletes as failures
- serializing values in a shape the SDK silently ignores

Moto tests exposed these gaps because they assert exact behaviour, not just success or failure.

### Durable categories of behavioural bugs

The same categories appeared across many services:

- Wrong default values: statuses, encryption defaults, password policy defaults, engine versions, visibility timeouts.
- Missing response fields: metadata, timestamps, algorithm lists, tag fields, `RestoreSummary`, version-qualified ARNs.
- Idempotency mismatches: delete-on-missing behaviour for SES, SNS, Step Functions, EventBridge, and others.
- Error mismatches: wrong typed exception or wrong message text.
- Validation gaps: name regexes, queue-name requirements, enum restrictions, request-shape requirements.
- Data computation issues: hashes, counters, version numbering, attribute conversions, policy serialization.

### Workflow that emerged

The durable workflow is:

1. Read moto tests for the target service or operation.
2. Implement or adjust handlers to satisfy those exact assertions.
3. Port the tests to Rust integration tests using `aws-sdk-rust`.
4. Keep the tests as the permanent regression suite.

This workflow consistently found issues that operation-count work alone missed.

### Relationship to docs-driven tests and Terraform E2E

Moto parity is strongest where moto has meaningful tests. It does not replace:

- AWS-doc-driven tests for services or operations moto does not cover well
- Terraform E2E tests for provider-specific waiter and read-back behaviour

The durable rule is to use the strongest available spec source:

- moto if it exists
- AWS docs and SDK behaviour when moto is absent
- Terraform traces when the provider introduces extra requirements

### EC2 parity as a late-stage example

The early-April EC2 work showed that parity is not only about matching moto's raw operation count. Moto's EC2 surface is broad but still partial, so the durable strategy is:

1. use moto-covered flows as the baseline behavioural contract
2. extend beyond moto where Winterbaume needs more of the real API surface
3. keep the added lifecycle and edge-case tests as the permanent regression suite

In practice this meant networking, compute, storage, and launch-template lifecycles all gained Rust integration coverage even after Winterbaume had already passed moto's approximate EC2 coverage.

## Files

- [moto tests](https://github.com/getmoto/moto/tree/master/tests): upstream Python test source for behavioural reference.
- `crates/winterbaume-*/tests/integration_test.rs`: Rust parity ports and service regressions.
- `crates/winterbaume-*/src/handlers.rs`: request parsing, error shaping, response construction.
- `crates/winterbaume-*/src/state.rs`: state transitions, validation, and data semantics.

## Test Coverage

Important milestones captured in the journal:

- 816 parity tests across 32 services with 0 failures at one major checkpoint
- later expansion to 55+ services with moto tests ported
- approximately 300+ cumulative behavioural fixes across all parity sessions
- EC2 later reached 61 passing parity-style integration tests while expanding to 214 implemented operations

Representative services with substantial parity work included:

- IAM
- S3
- SQS
- DynamoDB
- Lambda
- ECS
- KMS
- Organizations
- Secrets Manager
- EventBridge

## Pitfalls

- Weak smoke tests can falsely signal success. Exact-value parity tests are necessary.
- Some moto tests encode important quirks that are not obvious from Smithy models.
- Some services have no useful moto coverage. Do not force the workflow where moto is empty or non-authoritative.
- XML serialization issues can make an implementation look logically correct while the SDK still sees empty lists or missing data.
