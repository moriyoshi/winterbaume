---
name: update-readme
description: Update README.md with the latest API coverage data from the api-coverage skill.
user_invocable: true
---

# Update README

Update the "Supported Services" section in the workspace `README.md`, regenerate per-service crate `README.md` files, and sync the user-facing `docs/` site with the latest API coverage data.

## Instructions

1. **Regenerate the coverage report** by running:

   ```bash
   python3 .agents/skills/api-coverage/scripts/generate_coverage.py
   ```

2. **Update README.md files and docs/** by running:

   ```bash
   python3 .agents/skills/update-readme/scripts/update_readme.py
   ```

   This parses `.agents/docs/API_COVERAGE.md`, rewrites the Supported Services table in the workspace `README.md` (merging fully-implemented services and stub-only services into one sorted table), regenerates `crates/winterbaume-*/README.md` files, regenerates the two Terraform coverage reports under `.agents/docs/TERRAFORM_*_COVERAGE.md` and the user-facing `docs/reference/terraform.md`, and updates the rest of the `docs/` directory.

### What the script does

- Parses the overview table from `.agents/docs/API_COVERAGE.md`
- Reads the workspace `README.md` and replaces the entire `## Supported Services` section (including the old `### Stub Services` subsection if present) up to the next `##` heading
- Generates a single merged table with columns: Service, Crate, Protocol, Operations (winterbaume implemented / total), moto coverage, floci coverage, kumo coverage, and fakecloud coverage
  - Fully-implemented services link their crate to its generated crate-local `README.md`
  - All rows are sorted alphabetically by service display name
- Rewrites each service crate `README.md` with a generated coverage summary that includes the service name, AWS model, protocol, winterbaume coverage, moto coverage, floci coverage, kumo coverage, fakecloud coverage, coverage report date, an explicit list of implemented APIs, and a collapsible list of APIs not yet implemented (with notes where moto, floci, kumo, or fakecloud implement the missing operation)
- Transcribes selected service dossier sections into the matching service crate `README.md`; currently this includes `## Current Network Resource Stub Semantics` when present in `.agents/docs/services/<model-slug>.md`
- Adds an overall coverage summary line
- Preserves the rest of the README unchanged
- **docs/ updates** (unless `--no-docs` is passed):
  - Invokes `generate_terraform_resource_coverage.py` and `generate_terraform_converter_coverage.py` from the `api-coverage` skill ( unless `--no-terraform` is passed ) so the two `.agents/docs/TERRAFORM_*_COVERAGE.md` reports are fresh, then rewrites `docs/reference/terraform.md` from those reports — including per-service resource-type coverage, per-resource attribute coverage, and the overall rating distribution
  - Rewrites `docs/reference/services.md` with the same merged services table (no crate links, plain display name) and a footer linking to the Terraform coverage page with the live distinct-resource-type count
  - Updates the coverage percentage, operation count, and moto comparison in the `docs/index.md` hero feature block
  - Copies each `crates/winterbaume-*/README.md` (excluding infrastructure crates) into `docs/services/<slug>.md`, stripping crate-only guidance such as the package/workspace boilerplate, legal notice, and generated-source notes so the service docs stay focused on the service surface (no Node.js required)

### Service display names and protocols

The script maintains a mapping from crate names to human-readable service names and protocol identifiers. When adding a new service, update `CRATE_DISPLAY_INFO` in `.agents/skills/update-readme/scripts/update_readme.py`.

### CLI flags

| Flag | Default | Description |
|---|---|---|
| `--coverage PATH` | `.agents/docs/API_COVERAGE.md` | Path to the coverage report |
| `--readme PATH` | `README.md` | Path to the workspace README |
| `--crates-dir PATH` | `crates` | Path to the crates directory |
| `--docs-dir PATH` | `docs` | Path to the docs directory |
| `--no-docs` | off | Skip all docs/ updates |
| `--no-terraform` | off | Skip regenerating Terraform coverage reports and `docs/reference/terraform.md` (still runs when `--no-docs` is not set, by default) |
