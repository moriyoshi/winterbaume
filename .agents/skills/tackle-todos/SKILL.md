---
name: tackle-todos
description: "Read TODO.md and scan source code for TODO/FIXME comments, build a consolidated list, then dispatch parallel agents to address as many items as possible."
user_invocable: true
allowed-tools: Bash, Read, Write, Edit, Grep, Glob, Agent
---

# Tackle TODOs: Consolidate and Resolve Open Items

This skill scans the project for all outstanding work items — both from `.agents/docs/TODO.md` and from `// TODO` / `// FIXME` comments in source code — builds a consolidated, deduplicated, prioritized list, and then dispatches parallel agents to resolve as many items as possible.

**Use this skill when:** you want to make a focused sweep of outstanding TODOs and fix them in bulk.

## Arguments

- `[filter]` (optional): A service name or keyword to restrict which TODOs to tackle (e.g., `dynamodb`, `iam`). If omitted, all TODOs are considered.

## Step 0: Collect TODOs from TODO.md

Read `.agents/docs/TODO.md` and extract every unchecked `- [ ]` item. Record its service, description, and source.

## Step 1: Scan Source Code for TODO/FIXME Comments

Use Grep to search `crates/` for `// TODO` and `// FIXME` patterns in `*.rs` files.

Group and deduplicate the results:
- Identical or near-identical comments (e.g., `// TODO: extract from httpResponseCode field` repeated 200+ times across wire.rs files) should be collapsed into a single work item with a note about which files/crates are affected.
- Comments that are informational-only (e.g., noting a known limitation that cannot be fixed without large design changes) should be flagged but deprioritized.

## Step 2: Build a Consolidated TODO List

Merge the two sources into a single list. Deduplicate items that appear in both TODO.md and as code comments.

For each item, assign a category:

| Category | Description | Priority |
|----------|-------------|----------|
| **systematic** | Same pattern repeated across many files (e.g., httpResponseCode extraction) | High — one fix propagates widely |
| **behavioural** | Missing AWS behaviour (e.g., ConditionExpression ignored, pagination missing) | High — affects correctness |
| **validation** | Missing input validation or error responses | Medium |
| **serialization** | Wire format or serialization bugs (e.g., XML list serialization) | Medium |
| **test-only** | TODO in test code noting a test gap, not a code defect | Low |
| **design** | Requires significant design work or new abstractions | Deferred — flag for user |

Write the consolidated list to `.agents-workspace/tmp/consolidated-todos.md` for reference.

## Step 2b: Verify stale items before dispatch

Before dispatching agents on any TODO entry that is more than 24 hours old, verify the entry is still applicable: grep the relevant code for the symptom ( the `json!` site, the lowercase enum literal, the missing handler, etc. ). Round 5 ( 2026-04-29 ) found multiple already-resolved entries ( `partiql-cleanup-orphans`, URL-decode audit sites ) that wasted agent time. Skip or close stale entries instead of dispatching.

## Step 3: Filter (if argument provided)

If the user passed a `[filter]` argument, restrict the work list to items matching that filter (service name, crate name, or keyword).

## Step 4: Plan Parallel Work Items

Group the consolidated TODOs into independent, parallelizable work units. Each work unit should:
- Be self-contained (touching one crate or one cross-cutting concern)
- Not conflict with other parallel work units (no two agents editing the same file)

Present the plan to the user and get confirmation before dispatching.

## Step 5: Dispatch Parallel Agents

For each approved work unit, launch an Agent (subagent_type: general-purpose) with a clear prompt that includes:

1. The specific TODO(s) to address
2. The files to modify
3. The expected behaviour (reference AWS documentation if needed)
4. Instructions to run the relevant integration tests after making changes (`cargo test -p winterbaume-{service} -- --no-fail-fast`)

Launch as many agents in parallel as there are independent work units. Each agent works in an isolated worktree to avoid conflicts.

Never use `isolation: worktree` for parallel agents, which have repeatedly caused troubles. Instead, launch tasks independent from one other in a batch.

## Step 6: Collect Results and Update TODO.md

After all agents complete:

1. Review each agent's results — check if the fix compiled and tests passed.
2. For successfully resolved items, mark them as `- [x]` in `.agents/docs/TODO.md` and remove the corresponding `// TODO` comments from source code.
3. For items that could not be resolved, add notes about why and what was attempted.
4. Append a summary to `.agents/docs/JOURNAL.md` documenting what was tackled and the outcomes.

## Notes

- **Do not attempt design-category items** without user approval. These require architectural decisions.
- **Systematic items** (like httpResponseCode extraction) are high-value because one code-generation or macro fix can eliminate hundreds of TODOs at once.
- Respect CLAUDE.md rules: no `git checkout`, no `git restore`, no discretionary commits. Agents should only edit files, not commit.
- When running tests, always use focused filters (`cargo test <pattern>`), `--test-threads=N`, or `--no-fail-fast`, and specific crate targeting (`-p winterbaume-{service}`). Note: `--maxfail` is a pytest flag, not a Rust libtest flag — do not pass it to cargo test.
