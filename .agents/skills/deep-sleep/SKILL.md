---
name: deep-sleep
description: "Consolidate related long-term memory documents under .agents/docs/LTM/ into broader synthesis documents, refresh the LTM index, and promote service-specific durable knowledge into .agents/docs/services/*.md. Use when LTM has grown into overlapping topic files or when service-specific LTM findings should be folded into the AWS service research documents without deleting the source documents."
user-invocable: true
allowed-tools: Bash, Read, Write, Edit, Grep, Glob
---

# Deep Sleep: Consolidate Long-Term Memory Documents

This skill reads `.agents/docs/LTM/INDEX.md`, relevant documents under `.agents/docs/LTM/`, and the service research index under `.agents/docs/services/INDEX.md`. It groups overlapping LTM topics into broader synthesis documents and also extracts durable service-specific knowledge into the matching `.agents/docs/services/<service>.md` files so future agents can find service context in one place.

Use this after `good-sleep` has already distilled chronological journal entries into topic-oriented LTM files and those files now need a second-stage consolidation.

## Goal

Turn a set of narrow LTM notes into a smaller set of durable synthesis documents while keeping the source documents intact for traceability. When the LTM contains durable knowledge about a specific AWS service, also fold that service-specific knowledge into the corresponding service research document.

## Step 1: Inspect the Existing LTM Set

Read `.agents/docs/LTM/INDEX.md` first, then inspect only the LTM documents needed to understand the candidate clusters. Also read `.agents/docs/services/INDEX.md` so you can map service names, model slugs, and service document paths before touching service docs.

Look for:

- Multiple documents about the same subsystem or migration
- Repeated pitfalls, file references, or test guidance across documents
- One overview document plus several implementation-detail documents that should be summarised together
- Service-specific facts that belong with a particular AWS service document, such as API behaviour, resource relationships, error semantics, pagination, idempotency, protocol quirks, AWS documentation implications, or Winterbaume parity guidance

Do not bulk-load every file unless the LTM set is still small enough that doing so is cheaper than selective reading.

## Step 2: Propose Consolidation Clusters

Before writing, present a plan to the user with:

- The synthesis documents you propose to create
- The source LTM documents that feed each synthesis document
- The service documents you propose to update, with the source LTM documents or synthesis documents that justify each update and whether each update will be **reference summary** or **full distillation**
- Any source documents that should remain standalone because they are already cohesive

Cluster by durable topic, not by date and not by arbitrary file-count balancing.

## Step 3: Write Synthesis Documents

Create new synthesis documents under `.agents/docs/LTM/` using descriptive kebab-case filenames such as:

- `kenall-data-python-rust-binding-synthesis.md`
- `address-parser-edge-cases-synthesis.md`

Keep the original source LTM documents. Do not delete or overwrite them unless the user explicitly asks for replacement.

Use this structure:

```markdown
# <Synthesis Title>

## Summary

<2-4 sentence overview of the merged topic and why it matters>

## Included Documents

| Document | Focus |
|----------|-------|
| [source-a.md](./source-a.md) | <short note> |
| ... | ... |

## Stable Knowledge

<Bulleted list of the durable facts, constraints, and design decisions>

## Operational Guidance

<How an agent should approach work in this area>

## Files

<Important file paths and why they matter>

## Tests

<Regression coverage and command examples, if applicable>

## Pitfalls

<Failure modes, tricky assumptions, and gotchas>
```

## Step 4: Synthesize, Do Not Merely Concatenate

When merging documents:

- Deduplicate repeated explanations
- Convert chronological narratives into timeless guidance
- Preserve exact file paths, commands, query fragments, and test names when they are useful
- Keep contradictions visible; if two source docs disagree, call that out explicitly instead of silently choosing one

Prefer compact synthesis over exhaustive restatement. The new document should help future agents orient quickly and then drill into the source docs only when needed.

## Step 5: Extract Service-Specific Knowledge

For each LTM source or synthesis document you read, identify facts that are tied to a single AWS service and should live in `.agents/docs/services/<service>.md` as service-local context.

Promote service-specific knowledge when it is durable and useful for future service work:

- AWS-observable behaviour discovered during implementation or testing
- Resource lifecycle, dependency, or relationship modelling guidance
- Error names, status codes, retryability, validation rules, pagination, ordering, idempotency, or eventual-consistency semantics
- Protocol or serialisation quirks that affect that service
- Test, moto-parity, Terraform-parity, or AWS-doc implications that are specific to that service

Do not promote transient local status:

- "Implemented", "passes quality gate", "crate was renamed", "TODO fixed", or other progress bookkeeping
- Build timings, worktree names, PR details, or agent session summaries
- Broad cross-service patterns that belong in LTM syntheses rather than one service document
- Duplicates of content already present in the service document unless the new wording clarifies a real gap

Map the target service document by checking `.agents/docs/services/INDEX.md`, the service document title, and the `AWS model slug` line in candidate service docs. If the target is ambiguous, list the ambiguity in the plan or final note and do not guess.

When updating a service document:

- Preserve the existing generated research sections and table structure.
- Prefer adding or refreshing a concise `## Winterbaume LTM Notes` section near the end of the file, before `## Research Checklist for Parity Work` if that section exists, otherwise at the end.
- Start the section with a short source line such as `Sources: .agents/docs/LTM/<name>.md`.
- Choose one of two service-note modes for every source you promote:
  - **Reference summary**: use this when the source document contains weak, broad, already-covered, or only lightly service-specific implications. Keep the source path in `Sources:` and add a short accompanying summary bullet that explains what the source contributes and when future agents should open it.
  - **Full distillation**: use this when the source document contains durable service-specific guidance. Distil the relevant original material into the `## Winterbaume LTM Notes` section with enough detail that future service work can proceed without opening the original LTM document first. Keep the source path in `Sources:` for traceability.
- Organise the notes as compact bullets grouped by behaviour area when there is enough material.
- Use exact operation names, shape names, error names, and file paths when they are important.
- Keep the service document focused on service behaviour and parity guidance, not local implementation status.
- Do not leave a service note as a bare source link. Even reference-only updates need a short summary, and any source with substantial service-specific content needs full distillation.
- If a note applies to multiple services, either put the shared pattern in an LTM synthesis and add only the service-specific implication to each service doc, or leave it only in LTM if the service-specific implication is weak.

### Step 5b: Promote Cross-Call Invariant Inventories

The `write-tests` skill ( Step 1d ) produces a **Cross-Call Invariant Inventory** table for each service it touches and writes it into `JOURNAL.md`. After `good-sleep` reorganises journal content into LTM, those inventories surface as topic clusters in LTM. Promote them as follows:

- Recognise the artefact: a 6-row table with columns `Category | This service's invariant | Doc URL` and the canonical category set ( Toggle propagation; Modify rewrites sibling state; Per-call uniqueness; Default inheritance from parent; Lifecycle state transitions; Cross-resource references on read ).
- Promote the table into the matching `services/<service>.md` under a dedicated **`## Cross-call invariants`** section, **not** under `## Winterbaume LTM Notes`. The dedicated section makes the artefact keyword-searchable for future agents and matches the publication gate's expectations ( see `quality-gate` §2 ).
- Place the section after `## Winterbaume LTM Notes` ( if present ) or at the end of the dossier. Begin it with a one-line `Sources:` link back to the originating LTM file or files.
- Preserve the canonical six-row schema. Rows that the source marks as `N/A — <reason>` stay as `N/A` in the dossier — do not silently drop them; their absence should always be intentional and visible.
- Re-running `deep-sleep` refreshes the section in place. Do not append a duplicate `## Cross-call invariants` section. If newer invariant rows have appeared in LTM, merge them; if older rows have been superseded by AWS API changes, replace rather than accumulate.
- If the LTM does not contain an invariant inventory for a service ( e.g. the service was implemented before this artefact existed ), do not invent one — flag it as a `TODO.md` row instead so the next `write-tests` run will produce the inventory.

## Step 6: Refresh the Index

Update `.agents/docs/LTM/INDEX.md` so it clearly distinguishes:

- Source topic documents
- Synthesis documents

A simple approach is to keep one table for synthesis docs and one table for source docs.

Each synthesis entry should mention which source documents it consolidates.

Do not edit `.agents/docs/services/INDEX.md` unless you add or rename service documents. Ordinary service-note extraction updates existing service documents only.

## Step 7: Record the Consolidation

Append a short note to `.agents/docs/JOURNAL.md` describing:

- Which synthesis documents were created
- Which source LTM documents they consolidate
- Which service documents received service-specific LTM notes
- Any documents intentionally left standalone

Do not edit or delete existing journal sections. Only append.

## Guardrails

- Do not delete source LTM documents without explicit user approval
- Do not delete or regenerate service research documents while extracting LTM notes
- Do not collapse unrelated topics just to reduce file count
- Prefer a few high-value synthesis documents over a full rewrite of the entire LTM tree
- Preserve the documentation style rules used in repo-authored docs: half-width parentheses and half-width colons
- Preserve the service research documents' separation between AWS model research and Winterbaume-derived parity notes

## Notes

- This skill complements `good-sleep`; it does not replace it
- Re-running this skill should extend or refresh synthesis documents when new source LTM files appear
- Re-running this skill should also refresh existing `## Winterbaume LTM Notes` sections rather than appending duplicate service notes
- If the current LTM set is already small and non-overlapping, say so and avoid forced consolidation
