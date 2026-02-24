//! Scenario tests for BCM Recommended Actions.
//!
//! Scenario: BCM Recommended Actions is an AWS-managed corpus service — there is no public
//! `Create*` API for recommended actions. The service exposes a single operation
//! (`ListRecommendedActions`), making a 3+-operation end-to-end chain impossible to
//! construct without cross-service dependencies. Scenario tests are therefore omitted
//! per the quality-gate opt-out for AWS-managed-corpus / metadata-only services.
//!
//! State can be seeded directly via `service.restore(...)` using `BcmRecommendedActionsStateView`
//! for tests that need pre-populated data — see `tests/integration_test.rs` for examples.
