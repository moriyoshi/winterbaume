---
name: generate-changelog
description: Generate or update the root umbrella CHANGELOG.md and per-crate CHANGELOG.md files from cargo-dist style git release tags, GitHub release context, and commit history. Use when Codex is asked to create changelogs, compare changes between independent crate releases, prepare release notes, fill an Unreleased section, regenerate crate changelogs, or refresh the root changelog for winterbaume.
---

# Generate Changelog

## Overview

Create concise, human-edited changelogs for winterbaume's independently released crates. Per-crate changelogs at `crates/<crate>/CHANGELOG.md` are the authoritative detailed record; the root `CHANGELOG.md` is an umbrella summary and index across crate releases. Each release is bounded by cargo-dist style tags named `<crate-name>-v<version>` such as `winterbaume-s3-v0.1.1`.

## Workflow

1. **Establish the release model.**
   - Read `RELEASE.md` and the root `Cargo.toml` `[workspace.metadata.release]` section.
   - Treat tags matching `<crate-name>-v<version>` as release tags, following cargo-dist conventions.
   - Scope detailed changelog generation to one crate unless the user asks for all crates, a specific list of crates, or the root umbrella changelog.
   - Derive the crate name from the package name, not the directory name. For example, package `winterbaume-s3` uses tags like `winterbaume-s3-v0.1.1` and changelog path `crates/winterbaume-s3/CHANGELOG.md`.
   - Remember that crates are independently versioned, so do not compare one crate's tag against another crate's tag.
   - Treat root `CHANGELOG.md` as a cross-crate overview that summarises release batches by date and links to per-crate changelogs.

2. **Inspect existing state.**
   - Check `git status --short` before editing and do not overwrite unrelated user changes.
   - For each target crate, work on `crates/<crate-dir>/CHANGELOG.md`.
   - If the crate changelog exists, preserve manually authored sections and the existing style unless it is clearly broken.
   - If the crate changelog does not exist, create it with `# Changelog` and an `## Unreleased` section when there are unreleased changes.
   - When generating the umbrella changelog, work on root `CHANGELOG.md`.
   - If root `CHANGELOG.md` does not exist, create it as an overview with links to crate changelogs instead of copying all crate-level release notes.
   - If local tags may be stale and current release accuracy matters, fetch tags only after following the session's network and approval rules.

3. **Discover release boundaries.**
   - List tags with enough metadata to group releases:

     ```bash
     git for-each-ref --sort=creatordate --format='%(refname:short)%09%(creatordate:short)%09%(objectname:short)%09%(subject)' refs/tags
     ```

   - Filter to release tags, normally `winterbaume-*-v*`.
   - For each target crate, keep only tags whose prefix exactly matches that package name plus `-v`.
   - Sort that crate's tags by version or creation date, then use only adjacent tags from that same crate for historical ranges.
   - If the target crate has no release tags, create or update only its `Unreleased` section and state that no tagged releases were found if the user expected historical release sections.
   - For root `CHANGELOG.md`, collect all release tags, then group releases by tag date. Keep the individual crate tag identity inside each dated section.

4. **Collect changes for each range.**
   - For each crate release tag, compare the previous release tag for the same crate to the current tag:

     ```bash
     git log --first-parent --date=short --format='%H%x09%ad%x09%s' <previous-crate-tag>..<current-crate-tag>
     ```

   - For the first tagged release of a crate, use the repository root or the earliest relevant commit as the lower boundary.
   - For `Unreleased`, compare the newest release tag for that crate to `HEAD`.
   - Filter each range to changes relevant to the target crate and shared files that affect it. Inspect:
     - The crate directory, usually `crates/<crate-dir>/`.
     - Shared workspace files that affect package contents or behaviour, such as `Cargo.toml`, `Cargo.lock`, `tools/`, `.github/workflows/`, `README.md`, `docs/`, and `.agents/docs/`, when the commit meaningfully changes that crate's release.
   - Inspect touched paths for ambiguous commits with `git show --stat --oneline <commit>` or `git show --name-only --format=fuller <commit>`.
   - If GitHub PR or release context is needed and available, use it to clarify intent, but do not rely on remote metadata when local commits are sufficient.

5. **Write the changelog.**
   - Write each target crate's changelog to `crates/<crate-dir>/CHANGELOG.md`.
   - Use reverse chronological order: `Unreleased` first, then newest release sections for that crate.
   - Prefer this section shape:

     ```markdown
     # Changelog

     ## Unreleased

     ### Added
     - ...

     ## v0.1.1 - 2026-04-29

     ### Fixed
     - ...
     ```

   - Use the crate tag date as the release date. Section headings should use the crate version (`## v0.1.1 - YYYY-MM-DD`), because the file path already identifies the crate.
   - Use categories only when they have entries. Good categories for this repository are `Added`, `Changed`, `Fixed`, `Terraform`, `Documentation`, `Tests`, and `Internal`.
   - Include commit hashes only when they add useful traceability; avoid turning the changelog into a commit dump.

6. **Write the umbrella changelog when requested.**
   - Write the root changelog to `CHANGELOG.md`.
   - Keep it shorter than the per-crate files. It should help readers find what changed across the project without duplicating every detail.
   - Prefer this section shape:

     ```markdown
     # Changelog

     This file summarises notable winterbaume releases. Detailed crate-level notes live in each crate's `CHANGELOG.md`.

     ## Unreleased

     - See crate changelogs for unreleased crate-specific work.

     ## 2026-04-29

     - `winterbaume-s3` v0.1.1: Implemented bucket tagging APIs. See [`crates/winterbaume-s3/CHANGELOG.md`](crates/winterbaume-s3/CHANGELOG.md).
     - `winterbaume-sqs` v0.1.1: Fixed queue attribute persistence. See [`crates/winterbaume-sqs/CHANGELOG.md`](crates/winterbaume-sqs/CHANGELOG.md).
     ```

   - Group entries by date when several crate tags were created on the same day.
   - Link to crate changelogs when they exist. If generating root `CHANGELOG.md` before per-crate files exist, either create the relevant crate changelogs too or avoid links to missing files.
   - Use one bullet per crate release unless a release needs two bullets to separate major user-facing themes.
   - Include workspace-wide release tooling, documentation, or infrastructure changes in root `CHANGELOG.md` only when they affect release consumers or multiple crates.

## Editorial Rules

- Summarise behaviour, not implementation trivia. "Implemented S3 bucket tagging APIs" is better than listing every helper function touched.
- Mention generated source, model, or coverage report churn only when it changes user-visible support or release contents.
- Collapse repeated crate/service updates into one clear bullet when they share the same intent.
- Omit changes that only affect other crates, even if they happened between the same dates.
- Do not let root `CHANGELOG.md` become the authoritative source for crate details; keep detailed notes in per-crate files and summarise at the root.
- Preserve issue and PR references that appear in commit messages.
- Exclude pure release metadata commits such as `chore: release ...` unless the release itself is the only notable change.
- Use British English in repo-authored prose, except for API names, Rust terms, or quoted commit titles.
- Do not invent entries. If a range is unclear, inspect the diff or say in the final response what could not be determined.

## Validation

- Re-read every edited `crates/<crate-dir>/CHANGELOG.md` after editing to check ordering, dates, duplicate entries, and Markdown structure.
- Re-read root `CHANGELOG.md` after editing to ensure every linked crate changelog path exists.
- Run `git diff -- CHANGELOG.md crates/*/CHANGELOG.md` or a narrower pathspec and verify the diff only changes intended changelog content.
- If the task only updates changelog files, no Rust build is required.
