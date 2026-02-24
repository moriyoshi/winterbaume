---
name: distill-memories
description: Read `.agents/docs/LTM/` documents, find durable knowledge that belongs in `.agents/docs/OVERVIEW.md` or `.agents/docs/ARCHITECTURE.md`, and update those core documents with concise synthesized findings. Use when long-term memory notes have accumulated implementation or system knowledge that should be promoted into the project's canonical overview or architecture docs.
---

# Distill Memories

## Overview

Promote durable facts from `.agents/docs/LTM/` into the project's canonical documentation. Use it to keep the following document up to date:

a. `.agents/docs/OVERVIEW.md` ... focused on high-level system understanding
b. `.agents/docs/ARCHITECTURE.md` ... focused on structure, subsystems, and technical design.
c. `.agents/docs/QUALITY_GATE.md` ... focused on various implementation notes, including project conventions and protocols.

## Read in This Order

1. Read `.agents/docs/OVERVIEW.md`, `.agents/docs/ARCHITECTURE.md`, and `.agents/docs/QUALITY_GATE.md` first.
2. Read `.agents/docs/LTM/INDEX.md`.
3. Open only the LTM documents that look relevant to the gaps, stale sections, or missing detail you identified in the target docs.

Do not bulk-load every LTM file unless the set is still small enough that selective reading costs more than reading them all.

## Classify Findings Before Editing

For each candidate fact, decide whether it belongs in:

- `.agents/docs/OVERVIEW.md`: product scope, major subsystems, repository ecosystem, deployment model, business-relevant platform capabilities, and other high-level orientation material.
- `.agents/docs/ARCHITECTURE.md`: repository layout, subsystem boundaries, runtime architecture, data flow, interface contracts, implementation patterns, storage abstractions, and testing or operational constraints that matter to engineers.
- `.agents/docs/QUALITY_GATE.md`: quality gate checklist and implementation conventions.
- Neither document: narrow bug history, one-off migrations, temporary workarounds, or details that are too fine-grained for canonical docs.

Prefer durable knowledge over incident history. Convert timelines into timeless guidance.

## Update Strategy

When updating the target docs:

- Synthesize; do not copy LTM prose verbatim.
- Merge into existing sections when possible instead of appending random new sections.
- Add a new section only when the information represents a stable topic that the current document truly lacks.
- Keep summaries compact. Core docs should stay easier to scan than the underlying LTM notes.
- Preserve exact file paths, component names, and architecture terms when they help precision.
- If multiple LTM docs disagree, call out the ambiguity or stop and ask the user before cementing one interpretation.

## Editing Heuristics

- Favor architectural patterns over patch-level history.
- Favor current subsystem boundaries over implementation anecdotes.
- Omit test names unless they explain an architectural guarantee or an important invariant.
- Omit details that are already covered well in the target document.
- Fix obvious typos or stale wording in the touched sections if doing so improves clarity.

## Validation

Before finishing:

1. Re-read the edited sections of `.agents/docs/OVERVIEW.md`,`.agents/docs/ARCHITECTURE.md`, and `.agents/docs/QUALITY_GATE.md`.
2. Check that each added fact is supported by one or more LTM documents.
3. Check that overview-level material did not leak into architecture detail sections and vice versa.
4. Keep repo-authored documentation style rules: use half-width parentheses and half-width colons.
