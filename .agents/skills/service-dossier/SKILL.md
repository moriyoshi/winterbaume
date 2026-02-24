---
name: service-dossier
description: Generate a new AWS service dossier under .agents/docs/services or enhance an existing service dossier with Smithy-derived operation/resource facts, official AWS documentation research, concrete usage scenarios, and durable Winterbaume LTM lessons.
argument-hint: <service-name-or-model-slug>
user_invocable: true
---

# Service Dossier

Create or enhance `.agents/docs/services/<service>.md` so future Winterbaume work starts from accurate service context.

## Inputs

- `$0` - service name, Winterbaume crate suffix, AWS SDK slug, or Smithy model slug.

## Core Rules

- Treat `vendor/api-models-aws` as the source of truth for service identity, protocol, operations, required members, idempotency, pagination, error shapes, and Smithy resources.
- Treat official AWS documentation as the source of truth for semantics not present in Smithy: lifecycle transitions, cross-resource constraints, defaults, quotas, deletion prerequisites, policy behaviour, synchronisation, and workflow ordering.
- Read `.agents/docs/LTM/aws-doc-test-plan-catalog.md` and `.agents/docs/LTM/workspace-readmes-and-service-examples.md` before changing scenarios. Scenario bullets are planning prompts, not implementation guarantees.
- For repo-authored docs, use British English and half-width punctuation only.
- Preserve any `Winterbaume LTM Notes`, `Cross-call invariants`, service-specific TODOs, or hand-written parity notes already present in an existing dossier unless the user explicitly asks to rewrite them.

## Workflow

### 1. Resolve the Service

1. Read `.agents/docs/services/INDEX.md`.
2. If the service is listed, use that row's model slug and read `.agents/docs/services/<model-slug>.md`.
3. If the service is missing, find the Smithy model under `vendor/api-models-aws/models/<model-slug>/service/<version>/*.json`.
4. If multiple matches exist, disambiguate by title, SDK ID, endpoint prefix, and AWS SDK slug. Do not guess between similarly named control/data plane services.

Useful local commands:

```bash
grep -R -n -E "\\[<service>\\]|`<service>`|<service>" .agents/docs/services/INDEX.md vendor/api-models-aws/gradle.properties vendor/api-models-aws/models
find vendor/api-models-aws/models/<model-slug>/service -name '*.json'
```

### 2. Extract Smithy Facts

Run the bundled extractor to produce a draft or comparison aid:

```bash
python3 .agents/skills/service-dossier/scripts/extract_model_dossier.py <model-slug>
```

The extractor is read-only. When `.agents/docs/services/<model-slug>.md` already exists, it detects that file and prints an `Existing Dossier Notes` section listing merge-sensitive top-level sections. It does not merge or rewrite the dossier for you.

Use its output to populate or verify:

- Service identity and protocol.
- Operation family counts.
- Operation groups.
- Operation detail matrix.
- Important error, input, output, enum, and resource-related shapes.
- Resource model, when Smithy declares resources. If the model has no Smithy resources, derive conceptual resources from AWS docs and operation names, and say that the Smithy model lacks `resource` shapes.

### 3. Research Official AWS Documentation

Use the AWS Documentation MCP Server. Prefer official service user-guide and API-reference pages.

Search for:

- `<service title> overview concepts`
- `<service title> API Reference <core operation>`
- `<service title> create delete update list <main resource>`
- `<service title> quotas limits lifecycle access point policy tags pagination`

Read only pages that answer concrete parity questions. Record URLs in `Official AWS Documentation Research`.

Look specifically for:

- Main resources and their identifiers.
- Required creation order and deletion prerequisites.
- Lifecycle states and asynchronous job behaviour.
- Idempotency and duplicate-create behaviour beyond Smithy traits.
- Cross-resource constraints and reverse references.
- Account/service defaults that affect later creates.
- Policy, IAM, KMS, S3, EC2/VPC, Glue, Lambda, EventBridge, or other cross-service identifiers.
- Tagging semantics, quota limits, pagination ordering, and default values.

### 4. Generate or Enhance the Dossier

For a new dossier, create `.agents/docs/services/<model-slug>.md` with these sections, in this order:

1. Title.
2. Source.
3. `Service Overview`.
4. `Possible Usage Scenarios`.
5. `Service Identity and Protocol`.
6. `Behavioural Model Notes`.
7. `Official AWS Documentation Research` when official docs were read.
8. Existing durable service-specific sections, if applicable.
9. `Resource Model`.
10. `Operation Groups`.
11. `Operation Detail Matrix`.
12. `Important Shapes`.
13. `Research Checklist for Parity Work`.

For an existing dossier:

- Update stale Smithy-derived counts and tables from the current vendored model.
- Add or refresh `Official AWS Documentation Research` when useful for implementation parity.
- Improve `Possible Usage Scenarios` only where operation/resource names and docs support concrete workflows.
- Keep scenario text actionable: "create X, attach Y, verify Z appears through list/get" is better than "test CRUD".
- Treat the extractor output as a patch aid, not a replacement file. Preserve arbitrary manual edits, unknown sections, and hand-written notes unless the current Smithy model or official AWS documentation clearly contradicts them.
- Be especially careful with `Research Checklist for Parity Work`, `Possible Usage Scenarios`, `Behavioural Model Notes`, and `Official AWS Documentation Research`; these often contain human judgement and LTM-derived implementation guidance even though the extractor can emit sections with the same names.
- Preserve manual notes unless contradicted by newer official docs or the current model. If contradicted, state the correction precisely.

### 5. Scenario Quality Gate

Before finalising scenarios, apply the LTM lessons:

- Prefer concrete documented associations, attachments, memberships, targets, account defaults, jobs, executions, tasks, deployments, streams, workspaces, clusters, channels, meetings, simulations, migration tasks, lifecycle transitions, and parent-child propagation.
- Include full-input/read-back and per-call uniqueness prompts where the service mints IDs or stores optional fields.
- Include cross-call invariant prompts when one operation writes state another operation must later observe.
- Do not imply Winterbaume already implements the scenario.
- Do not invent scenarios for stateless services; explicitly keep them API/reference focused.

### 6. Update the Index

If adding a dossier:

- Add a row to `.agents/docs/services/INDEX.md` in model-slug order:

```markdown
| [<model-slug>](<model-slug>.md) | `<sdk-slug>` | <Title> | <protocols> | <operation-count> | <resource-count> |
```

- If the dossier includes `Official AWS Documentation Research`, add the slug to the sentence at the top of the index.

For SDK slug, prefer the Smithy `aws.api#service.sdkId` normalised like AWS SDK for Rust crate names. Cross-check existing index conventions for similar services.

### 7. Validate

Run focused documentation checks:

```bash
grep -n -E "behavior|synchroniz|optimization|organize|modeling|modeled|color|center|analyze" .agents/docs/services/<model-slug>.md
git diff -- .agents/docs/services/<model-slug>.md .agents/docs/services/INDEX.md
```

The grep command is a spelling smoke test, not a hard failure: API names and URLs may legitimately contain American spellings.

## Final Report

State:

- The dossier path added or updated.
- Whether AWS Documentation MCP research was included.
- Any ambiguity in service naming or missing official docs.
- That no Rust tests were run when the change is docs-only.
