---
name: winterbaume-bug
description: File a winterbaume bug report on GitHub Issues using the project's official bug-report issue form layout, so that the auto-label-service workflow can apply the correct `service:<slug>` label. Use this whenever Claude (or the user) discovers a defect in a `winterbaume-<service>` crate during exploration, testing, or development — for example a wrong status code, a missing field, a serialisation bug, an incorrect default, or any divergence from real AWS behaviour — and the bug should be tracked in the issue tracker. Use proactively whenever the user says things like "file a bug", "open an issue", "report this", "track this regression", or otherwise indicates the finding should not be lost. Do NOT use for feature requests, design discussions, or work that is being fixed in the same session without an issue.
---

# winterbaume-bug

Open a GitHub issue against `moriyoshi/winterbaume` that conforms to the project's bug-report issue form. Conformance matters because an `Auto-label service` GitHub Actions workflow parses the rendered issue body for an `### Affected AWS service` header and applies a `service:<slug>` label from the value beneath it. If the body does not match, triage labelling silently fails.

This skill is self-contained: the body template, the auto-labeller regex, and the canonical list of valid service slugs are all embedded below. Do not read repository files outside this skill directory to look any of these up — if the project's authoritative template ever drifts from what is documented here, treat that as a maintenance task on this skill itself, not as a runtime lookup.

## When to use

Trigger on any of:

- A failing assertion, panic, or wrong response observed against a `winterbaume-<service>` crate that the user is not fixing right now.
- The user says "file a bug", "open an issue", "report this", or similar.
- A peer-review finding that needs to be tracked separately from the current change.

Do not use for: feature requests, "wouldn't it be nice", documentation polish, or anything actively being fixed in the same conversation (just fix it).

## Workflow

Follow these steps in order. Two checkpoints require user input: step 3 (after the duplicate search returns candidates) and step 5 (before the final `gh issue create` call). Opening an issue or commenting on one is externally visible and should never happen without explicit confirmation.

### 1. Identify the affected service slug

The slug is the part of the crate name after `winterbaume-`. For example:

- A bug in the `winterbaume-ec2` crate → slug `ec2`
- A bug in the `winterbaume-cognitoidentityprovider` crate → slug `cognitoidentityprovider`

The slug **must** be one of the valid options listed in `references/service-slugs.txt` (one slug per line, in the same order the issue form's dropdown presents them). If the bug spans multiple services, file separate issues — the auto-labeller picks exactly one slug. If the slug is not in that list, stop and surface this to the user; either the dropdown needs an entry or the bug is in shared infrastructure (in which case a service label may not apply, and the user should decide how to file it).

### 2. Gather the required fields

The form has five required free-text fields plus the service dropdown. Collect each before drafting:

- **summary** — one or two sentences describing what is broken. Goes in the issue title as `[bug] <summary>` and in the body's Summary section. Keep the title under ~70 characters.
- **reproduction** — minimal steps, request, code, or shell commands. Rendered as shell.
- **expected** — what real AWS does, or what the docs/tests imply.
- **actual** — what winterbaume returns, including status code, AWS error code, and response body where relevant.
- **version** — output of `git rev-parse HEAD` from the working tree where the bug was found. Run this yourself; do not ask the user for it.
- **logs** (optional) — relevant `winterbaume-server` log lines, with secrets redacted. Rendered as shell. Include if available; omit the section entirely if not.

If you discovered the bug in this session, draft these from your own context. Only ask the user for fields you genuinely cannot derive — typically `expected` if it requires AWS behaviour you have not verified.

### 3. Search for existing duplicates or related issues

Before drafting or filing, search the issue tracker for issues that may be the same bug or closely related. Filing a duplicate wastes the maintainer's triage time and fragments the discussion thread.

Run these searches in order from most-specific to least-specific, and stop early if the first one returns clear matches:

1. **By service label, both open and closed** — narrowest, catches duplicates that were already fixed and closed:
   ```bash
   gh issue list --repo moriyoshi/winterbaume \
     --state all \
     --label "service:<slug>" \
     --search "<2-4 distinctive keywords from the summary>" \
     --limit 20
   ```
2. **By keyword across the whole repo** — broader, catches issues filed before the service label existed or filed against shared infrastructure:
   ```bash
   gh issue list --repo moriyoshi/winterbaume \
     --state all \
     --search "<distinctive AWS API name or error code>" \
     --limit 20
   ```

Pick keywords that are specific to the bug, not generic. Good keywords: AWS API operation names (`CopyObject`, `DescribeInstances`), error codes (`InvalidParameterValue`), unusual field names. Bad keywords: `bug`, `error`, `fails`, `wrong` — every issue contains those.

For each candidate, read the title and (if the title is plausible) the body via `gh issue view <number> --repo moriyoshi/winterbaume`. Classify each as:

- **Likely duplicate** — same operation, same symptom, same root cause as far as you can tell.
- **Related** — same operation or same area, but a different symptom or different root cause. Worth cross-linking, not merging.
- **Not relevant** — keyword overlap only.

Then present the findings to the user and ask how to proceed. Use this shape:

> Before filing, I searched for similar issues and found:
>
> - **#123 — [bug] S3:CopyObject ignores destination bucket** (open) — looks like a likely duplicate; same operation and same symptom of silent success.
> - **#98 — [bug] S3:CopyObject does not preserve metadata** (closed, fixed in #110) — related, different symptom.
>
> How would you like to proceed? Options:
> 1. Add a comment to #123 with the new repro and version info instead of filing a new issue.
> 2. File a new issue and reference #123 / #98 in the body so triage can decide.
> 3. File a new issue without cross-references (only if you are confident none of the above are related).

Wait for the user's choice before continuing. If they pick option 1, switch to `gh issue comment <number> --repo moriyoshi/winterbaume --body "$(cat <<'EOF' ... EOF)"` with a comment that includes the new reproduction, expected/actual, and `git rev-parse HEAD`, and skip the rest of this workflow. If they pick option 2, continue to step 4 and add a `### Related issues` section at the end of the body listing the referenced issue numbers (e.g., `Possibly related to #98. Possibly duplicate of #123 — please close one as a duplicate of the other if confirmed.`). If they pick option 3, continue to step 4 unchanged.

If the searches returned nothing relevant, say so explicitly ("No similar issues found in `service:<slug>` or by keyword search for `<keywords>`.") and continue to step 4 — being explicit lets the user catch the case where your keyword choice was too narrow.

### 4. Format the body to match the issue-form output

GitHub's issue-form renderer turns each field into a body section delimited by an `###` header that matches the field's `label`. Reproduce that exact shape so the auto-labeller's regex matches and the issue reads identically to one filed via the web form.

The auto-labeller extracts the slug with this regex:

```
/###\s+Affected AWS service\s*\n+\s*([a-z0-9]+)\s*\n/i
```

That regex is strict: the slug must sit on its own line, be lowercase, contain only `[a-z0-9]`, and have no surrounding markup (no backticks, no quotes, no list bullet).

Use this template verbatim, substituting bracketed placeholders. Do not add or rename headers. The outer fence below uses four backticks so the inner three-backtick `shell` fences nest correctly; the actual issue body sent to GitHub uses three-backtick fences for the shell blocks.

````markdown
### Affected AWS service

<slug>

### Summary

<one or two sentences>

### Reproduction

```shell
<minimal repro: shell commands, request payload, or short code snippet>
```

### Expected behaviour

<what AWS does or what docs/tests imply>

### Actual behaviour

<what winterbaume returns: status code, error code, response body>

### winterbaume version / commit

<git rev-parse HEAD output>

### Server logs

```shell
<relevant log lines, secrets redacted>
```
````

If there are no logs, drop the entire `### Server logs` section (header and code block) — do not leave an empty section, since the form treats the field as omitted.

### 5. Confirm with the user, then file

Show the user the proposed title and body and ask for explicit go-ahead. After they confirm, file via `gh`:

```bash
gh issue create \
  --repo moriyoshi/winterbaume \
  --title "[bug] <summary>" \
  --body "$(cat <<'EOF'
<body from step 4>
EOF
)"
```

Notes:

- Use a heredoc to preserve newlines and backticks. Do not pass `--body` as a single-quoted string with embedded code fences — it is fragile.
- Do not pass `--label` for the service label. The `Auto-label service` workflow applies it automatically from the body. The `bug` label is added by the issue form itself, so it is also redundant.
- Do not assign the issue or set a milestone unless the user asked.
- After creation, return the issue URL printed by `gh` to the user.

### 6. Verify the auto-label fired (optional)

After ~30 seconds, the workflow run should add `service:<slug>`. If the user wants confirmation:

```bash
gh issue view <number> --repo moriyoshi/winterbaume --json labels -q '.labels[].name'
```

If the label is missing, the most likely causes are: (a) the body's `### Affected AWS service` section does not match the regex (re-check formatting — the slug must be on its own line, lowercase, no backticks); (b) the `service:<slug>` label does not exist in the repo and needs to be created.

## Examples

### Example 1 — bug found while running integration tests

Context: while running `cargo test -p winterbaume-ec2`, a test failed because `DescribeInstances` returned `200` with an empty `Reservations` array instead of including a freshly-launched instance.

The skill should:

1. Note slug = `ec2` (present in `references/service-slugs.txt`).
2. Run `git rev-parse HEAD` for the version field.
3. Search: `gh issue list --repo moriyoshi/winterbaume --state all --label service:ec2 --search "DescribeInstances Reservations" --limit 20`. Report findings to the user.
4. Draft body with summary "EC2:DescribeInstances omits instance launched in the same session", reproduction as the failing test command + minimal `aws-sdk-rust` snippet, expected behaviour citing the AWS docs, actual behaviour citing the empty `Reservations`.
5. Confirm with user, then `gh issue create` (or `gh issue comment` if the user picked an existing issue to update).

### Example 2 — bug found during a code review

Context: while reviewing the `winterbaume-s3` crate's request handler module, Claude notices that `CopyObject` always returns `200` even when the destination key is empty, which would silently corrupt user data.

The skill should:

1. Note slug = `s3`.
2. Construct a minimal `aws s3api copy-object` repro.
3. Search: `gh issue list --repo moriyoshi/winterbaume --state all --label service:s3 --search "CopyObject" --limit 20`. If a related issue exists (e.g., `#98 — CopyObject does not preserve metadata`), present it to the user and ask whether to comment on it, file a new issue and cross-reference it, or file standalone.
4. Cite expected behaviour from AWS docs (the destination object must be created and the response includes a `CopyObjectResult`).
5. Cite the file and line you already saw during the review in the issue body, so a reader of the issue can navigate there. This is content for the reader; do not re-grep the source tree as part of running this skill.
6. Include `git rev-parse HEAD`.
7. Confirm and file.

## Anti-patterns

- **Filing without confirmation.** Issues are externally visible. Always show the title and body and wait for the user to say go.
- **Skipping the duplicate search.** Even a 30-second search saves the maintainer real triage cost. Do it every time, and report the result to the user even when nothing was found, so the user can sanity-check the keywords.
- **Inventing repros.** If the bug was reasoned about but not actually triggered, say so in the issue rather than fabricating a shell session that was never run.
- **Bundling multiple services.** One issue per service slug — the auto-labeller cannot apply two service labels from one body.
- **Adding bespoke sections.** The issue form has a fixed shape; extra `###` headers are fine for the human reader but never replace the required ones, and never change the wording of a required header (the regex and downstream tooling depend on the exact label text).
- **Skipping the version field.** It is required by the form and is what makes a future reader able to `git checkout` the exact tree that was buggy.
- **Reading repository files to confirm the template.** The body template, regex, and slug list in this skill are authoritative. If they appear to drift from the live project, fix this skill rather than reaching outside it at runtime.
