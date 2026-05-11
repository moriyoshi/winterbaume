# Winterbaume Skill Maintenance

## Summary

Winterbaume-specific skills should be portable, self-contained workflow units rather than thin wrappers around the current repository tree. The `winterbaume-bug` skill is the reference case: it embeds the issue body contract, service-slug snapshot, and labeller regex in its own directory, then uses CI to detect drift against repository issue templates.

## Key Facts

- A skill advertised as self-contained should not read `.github/ISSUE_TEMPLATE/*`, workflow files, or crate source paths at runtime to discover its own template contract.
- Embedded snapshots are acceptable when paired with drift detection owned by the repository, not by the skill runtime.
- The `winterbaume-bug` slug source is `skills/winterbaume-bug/references/service-slugs.txt`, copied from the bug-report form dropdown.
- The bug auto-labeller contract is the regex `/###\s+Affected AWS service\s*\n+\s*([a-z0-9]+)\s*\n/i`; the affected service slug must be lowercase, on its own line, and match `[a-z0-9]+`.
- Markdown templates that contain inner fenced code blocks need an outer fence with more backticks than the inner fences.
- Examples in skills should distinguish context the agent already has from files the skill is allowed to read while running.

## Details

### Self-Contained Skill Contract

The `winterbaume-bug` skill originally depended on three out-of-skill sources while drafting issues:

- `.github/ISSUE_TEMPLATE/bug_report.yml` for field order and service dropdown options
- `.github/workflows/auto-label-service.yml` for the labeller regex
- source-tree paths such as `crates/winterbaume-ec2/...` in examples, which encouraged runtime source inspection

That made the skill non-portable: exporting the skill without the full authoring repository changed or broke its behaviour. The durable fix is to treat the skill directory as the runtime contract. If the form template, labeller regex, or slug list changes, update the skill's embedded references as a maintenance task rather than making every invocation rediscover them.

### Slug Snapshot and Drift Detection

`skills/winterbaume-bug/references/service-slugs.txt` is a one-slug-per-line snapshot of the `bug_report.yml` dropdown options. The snapshot preserves the form order and is the skill's source of truth for slug validation.

Because snapshots can rot, `.github/workflows/skill-slug-drift.yml` checks drift when the bug report form, slug snapshot, or workflow changes. The workflow extracts dropdown entries with:

```bash
grep -oE '^        - [a-z0-9]+$' .github/ISSUE_TEMPLATE/bug_report.yml | sed 's/^        - //'
```

It then diffs the result against `skills/winterbaume-bug/references/service-slugs.txt` and emits a GitHub Actions error if they diverge. The extraction assumes the form has only one options block at the eight-space indent; keep that assumption visible if the issue form grows another dropdown.

### Body Template Rendering

The bug issue body template contains shell code blocks, so the outer Markdown fence in `SKILL.md` must use four backticks. A three-backtick outer fence closes at the first inner shell fence and splits the rendered template into multiple sibling code blocks.

Keep a short note beside such templates explaining that the issue body itself contains three-backtick fences. This prevents future edits from "simplifying" the outer fence back to three backticks.

### Runtime Lookup Boundaries

Skill examples may cite file and line information that the agent already saw during a code review. That is different from instructing the skill to search the repository at runtime.

For self-contained skills:

- examples should use crate-name phrasing unless a path is essential to the issue content
- the workflow should not ask the agent to re-grep source files to fill template fields
- repository-specific drift should be enforced by CI or by an explicit maintenance workflow, not hidden inside every skill invocation

## Files

- `skills/winterbaume-bug/SKILL.md`: self-contained issue-drafting workflow, embedded body template, and labeller contract
- `skills/winterbaume-bug/references/service-slugs.txt`: service-slug snapshot used by the skill
- `.github/workflows/skill-slug-drift.yml`: CI drift check between the issue form dropdown and the skill snapshot
- `.github/ISSUE_TEMPLATE/bug_report.yml`: upstream form whose dropdown is mirrored into the skill snapshot

## Test Coverage

- Local drift verification can run the same extraction pipeline used in CI and compare it with `references/service-slugs.txt`.
- Render-sensitive Markdown changes should be reviewed in the generated page or by checking that the outer fence is longer than any inner fence.
- Self-containment checks should search `SKILL.md` for unintended runtime references to `.github`, `bug_report.yml`, `auto-label-service.yml`, or `crates/winterbaume-*`.

## Pitfalls

- Do not make a self-contained skill read repository issue templates or workflow files during normal execution.
- Do not trust embedded snapshots without a drift check.
- Do not use a three-backtick outer fence around a template that itself contains three-backtick fences.
- Do not tell the skill to re-grep source paths that were only meant as context for a human-readable issue.
