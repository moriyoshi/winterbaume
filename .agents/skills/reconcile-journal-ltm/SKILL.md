---
name: reconcile-journal-ltm
description: "Audit whether the durable information in `.agents/docs/JOURNAL.md` is already represented in `.agents/docs/LTM/` and `.agents/docs/TODO.md`; if not, follow the `good-sleep` workflow to fill the gaps, then collapse all JOURNAL consolidation records into one canonical entry and remove the older record sections."
user-invocable: true
allowed-tools: Bash, Read, Write, Edit, Grep, Glob
---

# Reconcile Journal and LTM

Use this skill when `.agents/docs/JOURNAL.md` has continued to grow after one or more consolidation passes and you need to verify that the durable knowledge is actually present in long-term memory, not just claimed by old record blocks.

This skill is an orchestration layer around `good-sleep`. It should verify coverage first, run the `good-sleep` workflow only for missing material, and then normalize the consolidation history in `JOURNAL.md` into one canonical record.

## Goal

Ensure that each substantive journal entry has its durable information represented in `.agents/docs/LTM/` or `.agents/docs/TODO.md`, and keep only one consolidation-record section in `.agents/docs/JOURNAL.md`.

## Step 1: Read the Current Memory State

Read these files first:

1. `.agents/docs/JOURNAL.md`
2. `.agents/docs/LTM/INDEX.md`
3. `.agents/docs/TODO.md` if it exists
4. `.agents/skills/good-sleep/SKILL.md`

Then inspect only the LTM documents needed to verify whether the journal entries are already covered. Do not trust old consolidation records by themselves; they are hints, not proof.

While reading `JOURNAL.md`, separate:

- substantive journal entries
- consolidation-record sections such as `## LTM Consolidation Record`, `## LTM Consolidation Refresh (...)`, `## Deep Sleep Consolidation Record`, or similar headings

## Step 2: Audit Coverage Entry by Entry

For each substantive journal entry, extract the durable information that should survive consolidation:

- design decisions and invariants
- implementation patterns and file-path references
- bug classes, pitfalls, and behavioural constraints
- test strategy, regression commands, or validation guidance
- open follow-up work that belongs in `.agents/docs/TODO.md`

Do not require LTM to preserve transient noise such as timestamps, purely conversational framing, or command output that adds no durable guidance.

Treat an entry as covered only if its essential information is actually present in one or more LTM documents or in `.agents/docs/TODO.md`. Existing record tables are not enough.

Before making changes, produce an audit summary for yourself with:

- covered entries and the LTM docs that already capture them
- uncovered or partially covered entries
- stale consolidation-record sections that will need to be merged

## Step 3: Fill Gaps Through `good-sleep`

If any journal entry is uncovered or only partially covered, open `.agents/skills/good-sleep/SKILL.md` and follow its workflow for the missing material.

Use these rules while doing so:

- Reuse and extend existing LTM documents when the topic already exists.
- Create new LTM documents only when the missing knowledge does not fit an existing topic cleanly.
- Extract unresolved follow-up work into `.agents/docs/TODO.md`.
- Refresh `.agents/docs/LTM/INDEX.md` if LTM documents change.

If the missing coverage would introduce new topic documents or a meaningful re-clustering of topics, present that plan before writing, consistent with the `good-sleep` workflow.

After the LTM update, re-run the audit until every substantive journal entry is covered.

## Step 4: Rewrite Consolidation History as One Canonical Record

After the audit passes, rewrite the consolidation metadata in `.agents/docs/JOURNAL.md` so that only one canonical record section remains.

Use the heading:

```markdown
## LTM Consolidation Record
```

That canonical section should replace all older consolidation-record sections and should contain:

1. A short note saying the journal has been audited against LTM.
2. A deduplicated table mapping substantive journal sections to the LTM document that captures them.
3. If present, a deduplicated table mapping synthesis LTM documents to the source LTM documents they consolidate.
4. If present, a short list or table of LTM documents intentionally left standalone.
5. A pointer to `.agents/docs/LTM/INDEX.md`.

Sort and deduplicate the rows so the record is easy to scan. Prefer stable ordering over preserving the order of old record fragments.

## Step 5: Remove the Older Record Sections and Consolidated Entries

Delete the older consolidation-record sections after their information has been migrated into the canonical record.

After the canonical record is in place, also delete any substantive journal entry whose durable knowledge is already represented in `.agents/docs/LTM/` per the canonical record's section -> LTM mapping table. LTM is the durable form; the journal carries only entries that have not yet been consolidated, plus the canonical record itself. An entry whose durable knowledge has not been promoted to LTM ( for any reason -- audit gap, partial coverage, or a deliberate decision that the entry is too transient for LTM ) must NOT be removed.

This skill is the explicit exception to the usual append-only preference for `JOURNAL.md`: when the user invokes this skill, removing both ( a ) superseded consolidation-record sections and ( b ) consolidated substantive entries is part of the requested cleanup.

## Validation

Before finishing, verify all of the following:

1. Every substantive journal entry that remains in the file is either ( a ) not yet consolidated to LTM ( awaiting a future `good-sleep` pass ) or ( b ) the canonical `## LTM Consolidation Record` itself.
2. Every LTM document referenced by the canonical record actually exists.
3. Only one consolidation-record section remains in `.agents/docs/JOURNAL.md`.
4. No journal entry was removed unless its durable knowledge is already represented in `.agents/docs/LTM/` per the canonical record.
5. Repo-authored documentation still uses half-width parentheses and half-width colons.

## Notes

- This skill complements `good-sleep` and `deep-sleep`; it does not replace either one.
- Prefer updating existing LTM documents over creating near-duplicate notes.
- If the audit shows that LTM is already complete, skip the `good-sleep` rewrite step and only normalize the consolidation records.
