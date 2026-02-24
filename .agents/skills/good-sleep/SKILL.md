---
name: good-sleep
description: "Read JOURNAL.md and reorganize its contents into semantically separate long-term memory documents under .agents/docs/LTM/. Use when you want to consolidate accumulated journal entries into reusable, topic-organised reference files."
user-invocable: true
allowed-tools: Bash, Read, Write, Edit, Grep, Glob
---

# Good Sleep: Reorganize JOURNAL.md into Long-Term Memory

This skill reads `.agents/docs/JOURNAL.md`, identifies semantically related sections, and reorganizes them into topic-based documents under `.agents/docs/LTM/`. This mirrors how the brain consolidates episodic memories into structured long-term knowledge during sleep.

**Use this skill when:** JOURNAL.md has accumulated multiple entries and you want to distill them into organised, reusable reference documents.

## Overview

JOURNAL.md is an append-only chronological log. Over time it grows unwieldy. This skill extracts knowledge from it and organises it by **topic** (not by date) into separate files under `.agents/docs/LTM/`.

## Step 1: Read and Analyze JOURNAL.md

Read the full contents of `.agents/docs/JOURNAL.md`.

**First, identify and extract any to-do items** (tasks, action items, follow-ups, open issues, "TODO", "FIXME", "next steps", etc.) from all journal entries. These must NOT go into LTM — they go into `.agents/docs/TODO.md` (see Step 3a below).

Identify semantically distinct topics. A "topic" is a cohesive area of knowledge that an agent would want to look up as a unit. Examples of good topic boundaries:

- A specific subsystem or component (e.g., "address parser city resolver", "bank crawler extractors")
- A specific addressing pattern or domain concept (e.g., "Tonosho-cho kou/otsu addressing")
- A category of fixes sharing a common mechanism (e.g., "town name fuzzy matching")

Do NOT split by date. Multiple journal entries about the same topic should be **merged** into a single LTM document.

## Step 2: Plan the LTM Documents

Before writing, list out the planned documents with:
- Filename (kebab-case, descriptive, e.g., `tonosho-kou-otsu-addressing.md`)
- One-line summary of what knowledge it captures
- Which JOURNAL.md sections (by heading) feed into it

Present this plan to the user for confirmation before proceeding.

## Step 3: Extract To-Dos to TODO.md

Before writing LTM documents, collect all to-do items found in JOURNAL.md entries:

- Items explicitly marked as TODO, FIXME, or "next steps"
- Action items or follow-ups described as pending work
- Open issues or unresolved investigations flagged for future attention

Append them to `.agents/docs/TODO.md`. If the file does not exist, create it with this structure:

```markdown
# Project To-Dos

Items extracted from JOURNAL.md during good-sleep consolidation. Each item should be resolved or removed once addressed.

## Open Items

- [ ] <task description> — *source: <JOURNAL.md section heading>*
```

If the file already exists, append new items under the existing `## Open Items` section (or add the section if missing). Do not duplicate items that are already listed.

## Step 4: Create LTM Documents

Create the directory `.agents/docs/LTM/` if it doesn't exist.

For each planned document, create `.agents/docs/LTM/<filename>.md` with:

### Document Structure

```markdown
# <Topic Title>

## Summary

<2-3 sentence overview of the topic — what it is, why it matters>

## Key Facts

<Bulleted list of the most important facts an agent needs to know>

## Details

<Reorganized, deduplicated content from JOURNAL.md entries>
<Merge overlapping sections, remove redundant context>
<Keep tables, code snippets, and file path references intact>

## Files

<List of relevant source files with brief descriptions>

## Test Coverage

<Summary of test cases and how to run them, if applicable>

## Pitfalls

<Known gotchas, edge cases, and warnings>
```

### Writing Guidelines

- **Merge, don't copy-paste.** If multiple journal entries cover the same topic, synthesize them into a coherent narrative. Remove chronological framing ("On 2026-02-22 we discovered...") and write in a timeless reference style.
- **Preserve precision.** Keep exact file paths, SQL queries, regex patterns, table data, and code snippets. These are high-value reference material.
- **Keep tables.** Tables from JOURNAL.md (test cases, data patterns, etc.) should be preserved or consolidated.
- **Use half-width parentheses** with spaces before/after when adjacent to non-whitespace (per CLAUDE.md rules).
- **Use half-width colons** followed by a space (per CLAUDE.md rules).
- **No full-width colons or parentheses** in the LTM documents.

## Step 5: Create an Index

Create `.agents/docs/LTM/INDEX.md` that lists all LTM documents with one-line descriptions:

```markdown
# Long-Term Memory Index

| Document | Summary |
|----------|---------|
| [tonosho-kou-otsu-addressing.md](tonosho-kou-otsu-addressing.md) | Tonosho-cho's unique 甲/乙 multi-level addressing system |
| ... | ... |
```

## Step 6: Trim JOURNAL.md

After creating LTM documents, **do NOT delete** existing JOURNAL.md content (per CLAUDE.md: never edit existing sections). Instead, append a note at the end of JOURNAL.md:

```markdown
---

## LTM Consolidation Record

The following sections have been consolidated into long-term memory documents under `.agents/docs/LTM/`:

| Section | LTM Document |
|---------|-------------|
| <original heading> | <LTM filename> |
| ... | ... |

See `.agents/docs/LTM/INDEX.md` for the full index.
```

## Step 7: Update CLAUDE.md Reference (if needed)

If `.agents/docs/LTM/` is not already referenced in any project documentation, suggest adding a reference to the project's CLAUDE.md or OVERVIEW.md so that agents know to consult LTM documents.

## Notes

- This skill is idempotent: running it again should detect already-consolidated sections and only process new JOURNAL.md entries added since the last consolidation.
- LTM documents are meant to be **edited and refined** over time, unlike JOURNAL.md which is append-only.
- If a JOURNAL.md section doesn't fit neatly into any topic, create a `miscellaneous.md` catch-all or ask the user.
