# Journal and LTM Maintenance Workflows

## Summary

Winterbaume uses three distinct maintenance workflows for project memory. `good-sleep` distills substantive `JOURNAL.md` entries into topic-oriented LTM notes, `deep-sleep` synthesizes overlapping LTM notes into broader reference documents, and `reconcile-journal-ltm` audits whether the journal is actually covered before normalizing the journal's consolidation metadata into one canonical record.

## Key Facts

- `JOURNAL.md` is append-only for substantive entries during normal work, but `reconcile-journal-ltm` may delete entries that have already been consolidated into `.agents/docs/LTM/` ( as recorded in the canonical `## LTM Consolidation Record` table ). The journal carries only entries that have not yet been consolidated, plus the canonical record itself; LTM is the durable form.
- `.agents/docs/TODO.md` holds unresolved follow-up work extracted from journal entries and should not be treated as an LTM document.
- `good-sleep` is the workflow for moving durable knowledge from the journal into `.agents/docs/LTM/`.
- `deep-sleep` is the workflow for consolidating overlapping LTM documents without deleting the source notes.
- `deep-sleep` should prefer refreshing an existing synthesis document when the overlap is incremental; creating another synthesis tier is only justified when it removes real duplication.
- `reconcile-journal-ltm` verifies real coverage in `.agents/docs/LTM/` and `.agents/docs/TODO.md`; old journal record blocks alone are not proof of coverage.
- A `deep-sleep` refresh does not always need a new synthesis document. If the four existing synthesis documents still cover the project shape, update them in place and leave focused drill-down notes standalone.
- `reconcile-journal-ltm` is the explicit exception that may delete both ( a ) superseded consolidation-record sections from `JOURNAL.md`, and ( b ) substantive journal entries whose durable knowledge is already represented in `.agents/docs/LTM/` per the canonical `## LTM Consolidation Record` table. It must not rewrite or remove an entry whose durable knowledge has not yet been promoted to LTM.
- `.agents/docs/LTM/INDEX.md` is the current source of truth for the LTM set; `JOURNAL.md` should keep only one canonical `## LTM Consolidation Record` and the substantive entries that have not yet been consolidated.
- Durable docs do not validate themselves. Claims about external tools, providers, or specifications should be re-checked before they are promoted from `JOURNAL.md` into LTM or `QUALITY_GATE.md`.
- Documentation rewrite records in `JOURNAL.md` usually belong in existing workflow or synthesis documents, not standalone LTM files. Preserve what changed and why, then let the next reconciliation fold the record into the canonical table.

## Details

### Workflow Split

Use the maintenance workflows in this order:

1. Run `good-sleep` when substantive journal entries contain durable knowledge that is not yet represented in `.agents/docs/LTM/`.
2. Run `deep-sleep` when the LTM tree has grown overlapping topic notes that should be summarized into broader synthesis documents.
3. Run `reconcile-journal-ltm` when the repo needs an audit that compares substantive journal entries against actual LTM coverage and then rewrites scattered consolidation metadata into one canonical record.

These workflows are complementary rather than interchangeable. `good-sleep` operates on `JOURNAL.md`, `deep-sleep` operates on `.agents/docs/LTM/`, and `reconcile-journal-ltm` supervises the boundary between them.

When `deep-sleep` is extending an already sensible synthesis layer, prefer revising the existing synthesis documents and explicitly leaving other synthesis notes standalone where another top-level synthesis layer would only add indirection. This keeps the LTM tree searchable and avoids turning the synthesis layer into a stack of summaries of summaries.

### Coverage Audit Rules

When auditing whether a journal entry is covered, preserve the durable parts of the entry:

- design decisions and architectural invariants
- implementation patterns, file paths, and commands that future agents may need
- bug classes, pitfalls, and behavioural constraints
- regression guidance and validation commands
- unresolved work that belongs in `.agents/docs/TODO.md`

Do not require LTM to preserve timestamps, conversational framing, or low-value command output if the durable guidance already exists elsewhere.

When a journal entry records a correction to previously promoted guidance, treat that as first-class maintenance work:

- fix the affected LTM or canonical docs directly
- record the correction and its scope in the relevant LTM note
- avoid treating prior LTM text as evidence that the underlying fact was ever verified

### Canonical Consolidation Record

`JOURNAL.md` should keep one canonical `## LTM Consolidation Record` section that contains:

- a short note that the journal has been audited against `.agents/docs/LTM/` and `.agents/docs/TODO.md`
- a deduplicated table from substantive journal sections to the LTM document that carries the durable knowledge
- synthesis-document relationships when deeper LTM consolidation has occurred
- intentionally standalone LTM notes when they should remain unsynthesized
- a pointer to `.agents/docs/LTM/INDEX.md`

Superseded record sections such as `## LTM Consolidation Refresh (...)` or `## Deep Sleep Consolidation Record (...)` should be folded into the canonical record and then removed.

When running plain `good-sleep`, it is acceptable to append an incremental consolidation record at the end of `JOURNAL.md` rather than editing the existing canonical record in place. The next `reconcile-journal-ltm` pass should fold those incremental rows into the single canonical record, remove the superseded metadata sections, and ( per the policy above ) also remove any substantive entries whose durable knowledge is now represented in LTM.

### Service Notes and Invariant Inventories

`deep-sleep` now also promotes durable service-specific LTM findings into `.agents/docs/services/<service>.md` under `## Winterbaume LTM Notes`. The service docs should keep generated AWS model research sections intact; promoted notes must be either a full distillation or a reference summary explaining when to open the source LTM document.

Cross-call invariant inventories are authored in `JOURNAL.md` during `/write-tests`, then preserved by `good-sleep` in LTM. A later `deep-sleep` pass recognises those inventories and promotes the service-local version into a `## Cross-call invariants` section in the relevant service dossier. Do not paraphrase the inventory so aggressively during `good-sleep` that the deep-sleep recognition step can no longer identify the rows.

### Service Dossier Skill

The `service-dossier` skill creates or enhances `.agents/docs/services/<service>.md` before implementation work begins. It extracts Smithy identity, operation groups, operation detail tables, important shapes, and resource notes from `vendor/api-models-aws`, then treats AWS documentation research and Winterbaume LTM lessons as merge-sensitive human-judgement sections rather than generated replacement text.

`add-service` now resolves the model-slug-keyed dossier before selecting scope and invokes `/service-dossier {service}` when the dossier is missing. The extractor is intentionally read-only and should be used as a patch aid. It was smoke-tested against `s3files`.

### Documentation rewrite records

Short records such as "Distill Memories Record", "Core Documentation Rewrite Record", and "Quality Gate Rewrite Record" are metadata about the documentation system rather than independent product knowledge. During `good-sleep`, fold their durable content into the relevant topic docs:

- `OVERVIEW.md` owns concise project orientation, service-boundary principles, validation posture, documentation map, and operating model.
- `ARCHITECTURE.md` owns implementation detail: repository layout, request flow, core runtime components, service-crate structure, codegen, Terraform conversion, cross-service boundaries, helper crates, protocol routing, SDK/server integration, validation architecture, and architectural constraints.
- `QUALITY_GATE.md` should stay a flat checklist with single ownership sections rather than repeated rationale blocks.
- Deep-sleep records should refresh existing synthesis documents first and promote service-specific durable notes into `.agents/docs/services/<service>.md` under `## Winterbaume LTM Notes`.

Plain `good-sleep` should append an incremental consolidation record after doing this. `reconcile-journal-ltm` is still the workflow that later removes superseded metadata and normalises the journal back to one canonical record.

## Files

- `.agents/docs/JOURNAL.md`: chronological source log plus the single canonical consolidation record
- `.agents/docs/LTM/INDEX.md`: current inventory of synthesis, source-topic, and standalone LTM documents
- `.agents/docs/TODO.md`: unresolved follow-up items extracted from journal entries
- `.agents/skills/good-sleep/SKILL.md`: journal-to-LTM topic consolidation workflow
- `.agents/skills/deep-sleep/SKILL.md`: LTM-to-synthesis consolidation workflow
- `.agents/skills/reconcile-journal-ltm/SKILL.md`: audit and record-normalization workflow
- `.agents/skills/service-dossier/SKILL.md`: service research dossier creation and enhancement workflow

## Test Coverage

- No automated tests cover these documentation workflows directly.
- Validation is document-level: verify journal coverage against actual LTM content, verify `TODO.md` holds unresolved work, and verify that `JOURNAL.md` contains only one consolidation-record section after reconciliation.
- For invariant inventories, validate that the table survives the journal -> LTM -> service-dossier path.

## Pitfalls

- Do not trust old consolidation-record tables without checking the referenced LTM documents.
- Do not delete substantive journal entries whose durable knowledge has not yet been promoted to LTM. Reconciliation may only remove ( a ) superseded consolidation-record sections, or ( b ) substantive entries that are already represented in `.agents/docs/LTM/` per the canonical `## LTM Consolidation Record` table.
- Do not create near-duplicate LTM documents when extending an existing topic note would preserve the knowledge more cleanly.
- Do not create another synthesis tier merely because some overlap exists; first check whether refreshing the current synthesis documents would preserve the same durable guidance with less indirection.
- Do not leave new consolidation metadata scattered across multiple journal sections after the audit has been completed.
- Do not edit existing journal sections during plain `good-sleep` just to keep the canonical record tidy. Append the incremental record and let `reconcile-journal-ltm` normalise it.
- Do not promote provider- or documentation-specific claims into durable memory just because they already appear in `JOURNAL.md`. Re-verify them when the claim affects repo-wide guidance.
- Do not lose row-level `N/A` justifications when consolidating invariant inventories. They are part of the gate evidence.
