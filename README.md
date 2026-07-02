# winterbaume triage memory

This branch is the bug-triage bot's working memory. **It is bot-owned.** Do not edit by hand.

- `triage-log/YYYY-MM.jsonl` -- one JSON record per triage workflow run. Two schemas appear in this file:
  - `triage-log/v1` -- ordinary triage decision ( verdict ∈ `ready` / `needs-feedback` / `not-a-bug` ), populated by the AI triage step.
  - `guardrail-flag/v1` -- prompt-injection guardrail flagged the issue ( verdict `abuse` ); the AI triage step is skipped, so this is the only audit record produced for that issue.
- `outcomes/YYYY-MM.jsonl` -- one record per closed issue (final labels, closing comment excerpt).

Both files are append-only and rotated monthly. Schemas live in the `schema` field of each record.

Writers:
- `.github/workflows/record-triage.yml`
- `.github/workflows/record-outcome.yml`

Reader: the triage workflow's pinned GitHub MCP server (`get_file_contents`, `ref: memory/triage`).
