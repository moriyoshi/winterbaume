# Rule Evaluator and Validator Crates

## Summary

Winterbaume uses standalone Rust crates for rule evaluation and structural validation instead of embedding that logic inside HTTP handlers. This document captures the durable architecture for IAM policy simulation, Step Functions ASL validation, WAFv2 rule parsing and WCU calculation, and the later Bedrock flow parser or validator split, together with the review findings that still define the open edges.

## Key Facts

- `winterbaume-iam-rule-eval` holds IAM policy evaluation logic outside the HTTP layer.
- `winterbaume-sfn-asl-eval` validates ASL definitions and now backs `ValidateStateMachineDefinition` in `winterbaume-sfn`.
- WAFv2 logic is intentionally split into `winterbaume-wafv2-webacl-rule-parser` and `winterbaume-wafv2-wcu-calculator`.
- The WAFv2 crates are implemented, but full handler-side `CheckCapacity` integration and some surcharge fidelity are still open.
- Bedrock flow validation is no longer just a plan. It now lives in two crates: `winterbaume-bedrock-flow-parser` and `winterbaume-bedrock-flow-validator`.
- Review findings matter as much as the first implementation. The durable open issues are now mostly parity and correctness gaps, not missing crate skeletons.

## Details

### Why These Crates Stay Separate

The durable reason to keep evaluator logic out of handlers is that these crates are:

- pure logic
- reusable across services
- easier to test in isolation
- easier to review against AWS documentation without protocol noise

Handlers should only:

- parse the request
- build evaluator input
- call the pure library
- map evaluator output into wire types

### IAM Policy Evaluation

`winterbaume-iam-rule-eval` started as the engine for `SimulateCustomPolicy` and later became the basis for the repo's IAM simulation work more broadly.

Durable design points:

- wildcard matching is implemented directly rather than through regexes
- action matching is case-insensitive
- resource matching is case-sensitive
- evaluation order follows AWS semantics: explicit deny, then allow, then implicit deny
- handlers carry policy-source information through evaluation so matched statements can be reported against their origin

The journal records a phased rollout, but the later quality-gate pass is the important durability signal: old TODO entries claiming both IAM simulation APIs were empty stubs had become stale. Future work should therefore treat IAM simulation as implemented-but-incomplete rather than unimplemented.

The remaining follow-up area is still conditions and broader parity:

- richer condition operators
- permissions boundaries
- any remaining simulation edge cases not covered by the current evaluator

### Step Functions ASL Validation

`winterbaume-sfn-asl-eval` validates ASL definitions before Winterbaume accepts or updates a state machine.

The durable structure is:

- a parser layer that turns JSON into internal model types
- structural rules that emit diagnostics
- handler integration that maps diagnostics into `ValidateStateMachineDefinition` output shapes

The key review-driven lessons are:

- modern `Map` states may use `ItemProcessor`, not only `Iterator`
- a missing `Type` field must be diagnosed explicitly
- states that set both `End` and `Next` are structurally invalid

Those three issues are still important because the journal records them as real false-negative or false-positive gaps after the first implementation wave. The durable rule is not "the validator is finished"; it is "the validator exists, and review-backed parity gaps should be tracked as evaluator work, not handler work".

### WAFv2 Rule Parsing and WCU Calculation

WAFv2 is intentionally split in two:

- `winterbaume-wafv2-webacl-rule-parser` owns JSON-to-AST conversion
- `winterbaume-wafv2-wcu-calculator` owns the cost model

That split matters because:

- the parser can be reused for validation work beyond capacity maths
- the calculator should not know about raw wire JSON
- managed-rule-group cost tables and referenced-rule-group resolution belong at the calculator boundary, not in parsing

The implemented AST and cost model cover the main rule families, including:

- ByteMatch
- SQLi and XSS
- regex and regex-pattern-set references
- IP set, geo, ASN, and label matching
- managed rule groups and custom rule-group references
- rate-based rules
- `And`, `Or`, and `Not`

The important review-backed caveat is that the calculator still has open fidelity gaps:

- text-transformation costs must be modelled exactly
- `FieldToMatch` and related component surcharges are still incomplete
- custom-key and forwarded-IP surcharges need the same careful treatment

The later ByteMatch fix corrected one concrete bug, but it did not close the whole fidelity gap. Treat WAFv2 as implemented infrastructure with known cost-model follow-up, not as solved parity.

### Bedrock Flow Parsing and Validation

Bedrock flow validation moved from plan to implementation on 2026-04-13.

The durable split mirrors the WAFv2 pattern:

- `winterbaume-bedrock-flow-parser` turns flow-definition JSON into a typed graph model
- `winterbaume-bedrock-flow-validator` runs structural and type-oriented checks against that model

The parser owns:

- node-type discrimination
- node configuration enums
- connection modelling
- nested loop-definition parsing

The validator owns:

- graph reachability
- cycle detection
- connection source or target validation
- missing-node-input or missing-node-output checks
- condition wiring rules
- loop-boundary and loop-role checks

The key durable lesson is the same as in IAM and WAFv2: once the engine exists, future parity work belongs in the engine crate, not as ad hoc checks in `winterbaume-bedrockagent` handlers.

### Review Findings as First-Class Maintenance Inputs

The 2026-04-12 review is durable because it changed how these crates should be maintained:

- trust the evaluator architecture
- distrust the first parity claim
- push semantic fixes into the dedicated crate

The important examples are:

- ASL `ItemProcessor` support and common-state validation rules
- WAFv2 transformation and component-cost fidelity
- any Bedrock flow rule that needs graph- or type-level understanding rather than handler heuristics

## Files

- `crates/winterbaume-iam-rule-eval/src/` - IAM evaluator logic
- `crates/winterbaume-iam/src/handlers.rs` - IAM simulation handler integration
- `crates/winterbaume-sfn-asl-eval/src/` - ASL parsing and validation logic
- `crates/winterbaume-sfn/src/handlers.rs` - ASL validation handler integration
- `crates/winterbaume-wafv2-webacl-rule-parser/src/` - WAFv2 rule parsing
- `crates/winterbaume-wafv2-wcu-calculator/src/` - WAFv2 capacity calculation
- `crates/winterbaume-wafv2/src/handlers.rs` - `CheckCapacity` integration boundary
- `crates/winterbaume-bedrock-flow-parser/src/` - Bedrock flow parser
- `crates/winterbaume-bedrock-flow-validator/src/` - Bedrock flow validator
- `crates/winterbaume-bedrockagent/src/handlers.rs` - Bedrock flow validation handler integration

## Test Coverage

- `winterbaume-iam-rule-eval` has focused unit coverage for matching and evaluation order
- IAM integration tests cover allow, implicit deny, explicit deny, and inherited-policy paths
- `winterbaume-sfn-asl-eval` has unit coverage for structural validation, but the review-established parity gaps remain important follow-up cases
- WAFv2 parser and calculator crates have focused unit coverage across leaf, compound, and managed-group paths
- Bedrock flow parser and validator crates have dedicated unit coverage for node families, graph validation, connection validation, and loop rules

## Pitfalls

- Do not move semantic rule logic back into handlers once a dedicated evaluator crate exists.
- Do not assume a first implementation means AWS parity is complete.
- Do not collapse parser and calculator concerns into one crate when the parsed model has independent value.
- Do not rely on stale TODO wording for IAM or Bedrock flow validation. The relevant engines now exist.
- Do not treat the WAFv2 calculator as final. Handler integration and surcharge fidelity still need careful review.
