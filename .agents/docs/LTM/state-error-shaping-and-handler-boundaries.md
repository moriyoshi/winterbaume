# State Error Shaping and Handler Boundaries

## Summary

Winterbaume's durable error-handling rule is that `state.rs` should return domain-specific errors, while `handlers.rs` owns the protocol-facing translation into AWS error types, messages, and HTTP status codes. The large 2026-04-04 `refactor-state-errors` batch turned that from an architectural preference into an explicit repo-wide pattern across many service crates.

## Key Facts

- Service state layers should not construct `{Service}Error { error_type, message, status }` structs directly.
- `state.rs` should expose a domain-specific `thiserror` enum that describes business failures in service terms rather than HTTP terms.
- `handlers.rs` is the boundary that maps domain errors to AWS-facing `error_type`, `message`, and `status`.
- Error enums should not grow `error_type()` or `status()` helper methods. Those reintroduce HTTP coupling into the state layer.
- Handler match arms should normally use `err.to_string()` so the `#[error("...")]` string in the enum remains the single message source of truth.
- Partial batch work was recoverable because the stable refactor order is `state.rs` first, then `handlers.rs`.

## Details

### Architectural rule

The anti-pattern that the batch removed was:

- state helpers returning service-specific wire error structs
- state code deciding AWS error names and HTTP status codes directly
- duplicated message strings split between state and handler layers

The durable replacement is:

1. define a domain error enum in `state.rs`
2. return that enum from state helpers and validation paths
3. match exhaustively in `handlers.rs`
4. build the AWS-facing error payload only at that handler boundary

This keeps the state layer reusable and testable without smuggling protocol concerns through every mutation helper.

### What belongs in the enum

Good enum variants describe service-domain failures such as:

- missing resources
- duplicate resources
- invalid parameters or invalid state transitions
- unsupported combinations of options

What should stay out of the enum:

- HTTP status codes
- AWS error-type strings as helper methods
- duplicated user-facing message templates in both the enum and the handler

### Handler-side shaping

The handler layer owns:

- mapping enum variants to AWS error-type strings
- deciding status codes
- attaching request IDs or protocol-specific envelopes
- converting `err.to_string()` into the final response payload

The durable message rule is to keep the explanatory string in the enum's `#[error("...")]` attribute and reuse it from the handler. Inline `format!(...)` strings in the handler create a second copy of the same message and become maintenance debt.

### Batch-refactor operational lesson

The large multi-agent batch on 2026-04-04 exposed a useful execution pattern:

- writing the enum to `state.rs` first makes interrupted work recoverable
- the follow-up handler patch is then mostly a local exhaustive match and response construction update
- pre-existing service bugs can still fail tests after the refactor, but those failures should be separated from the error-shaping change itself

The reference example from that session was `winterbaume-iotdataplane`: shadow tests were already failing before the refactor, so the architectural conversion was correct even though the service still had unrelated red tests.

## Files

- `crates/winterbaume-*/src/state.rs` - domain-specific error enums and business-rule validation
- `crates/winterbaume-*/src/handlers.rs` - AWS-facing error shaping and response construction
- `.agents/skills/refactor-state-errors/SKILL.md` - batch workflow for converting services away from state-layer wire errors
- `.agents/docs/ARCHITECTURE.md` - architectural statement that handlers, not state modules, own user-facing error shaping

## Test Coverage

- The 2026-04-04 batch converted more than 30 services and used per-crate build or test runs to confirm that the error-shaping logic remained equivalent.
- When validating this pattern, prefer the smallest affected service suite plus compilation of the crate rather than only inspecting the enum by eye.

## Pitfalls

- Do not add `error_type()` or `status()` helper methods on the state-layer enum.
- Do not duplicate the same message string in both `#[error("...")]` and a handler `format!(...)` arm.
- Do not treat a pre-existing unrelated service test failure as evidence that the refactor itself is wrong.
- Do not move AWS protocol concerns back into `state.rs` just because the first handler match feels repetitive.
