---
name: api-coverage
description: Generate AWS API coverage reports comparing winterbaume implementations against official AWS API models, plus Terraform converter coverage at the resource-type and per-attribute levels.
user_invocable: true
---

# API Coverage Report Generator

Three complementary reports live in this skill, each answering a different
"how much do we cover?" question:

1. **AWS API operation coverage** — `generate_coverage.py` writes
   `.agents/docs/API_COVERAGE.md`. Compares winterbaume's implemented
   operations against the official Smithy models under
   `vendor/api-models-aws/`, with moto / floci / kumo numbers alongside.
2. **Terraform resource-type coverage** — `generate_terraform_resource_coverage.py`
   writes `.agents/docs/TERRAFORM_RESOURCE_COVERAGE.md`. For each
   winterbaume service, lists how many Terraform `aws_*` resource types
   the converter handles vs how many the AWS provider schema declares
   for that service, plus the missing types per service.
3. **Terraform attribute coverage** — `generate_terraform_converter_coverage.py`
   writes `.agents/docs/TERRAFORM_CONVERTER_COVERAGE.md`. For each
   handled resource type, measures inject% and extract% as the fraction
   of the resource's TF schema attributes the converter touches in each
   direction.

Pick the right report for the question:
- "Do we know about all the IAM resources Terraform supports?" → resource coverage.
- "Does our `aws_iam_role` converter handle all the role's fields?" → attribute coverage.
- "Does winterbaume implement the AWS IAM API operations moto does?" → API operation coverage.

## Instructions

Run the report you need:

```bash
# 1. AWS API operation coverage
python3 .agents/skills/api-coverage/scripts/generate_coverage.py

# 2. Per-service Terraform resource-type coverage
python3 .agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py

# 3. Per-resource Terraform attribute coverage
python3 .agents/skills/api-coverage/scripts/generate_terraform_converter_coverage.py
```

Each writes to `.agents/docs/<NAME>.md` by default; use `--output PATH` to redirect.

### Prerequisites

`vendor/api-models-aws` must be present as a submodule:

```bash
git submodule update --init vendor/api-models-aws
```

**moto, floci, and kumo do not require submodules.** When `vendor/moto`, `vendor/floci`, or `vendor/kumo` are absent the script fetches the necessary source files directly from GitHub and caches the parsed results under `.agents/skills/api-coverage/cache/` for 7 days. A `GITHUB_TOKEN` environment variable is optional but raises the unauthenticated rate limit (60 req/hr) to 5 000 req/hr.

To force a cache refresh (e.g. after a new moto/floci/kumo release):

```bash
python3 .agents/skills/api-coverage/scripts/generate_coverage.py --refresh-remote
```

If you prefer local clones (faster, works offline):

```bash
git submodule add https://github.com/getmoto/moto     vendor/moto
git submodule add https://github.com/floci-io/floci   vendor/floci
git submodule add https://github.com/sivchari/kumo     vendor/kumo
```

The script prefers a local clone over the cache or a remote fetch when the vendor directory is present.

### What the script does

1. **Parses AWS API models** from `vendor/api-models-aws/models/<model-dir>/service/<version>/<name>.json` (Smithy JSON format, extracting shapes with `"type": "operation"`)
2. **Parses winterbaume implementations** from `crates/winterbaume-*/src/handlers.rs`, detecting operations across all dispatch patterns:
   - awsQuery: `match action { "CreateBucket" => ... }`
   - awsJson: `"ServicePrefix.Operation" => ...` in X-Amz-Target matching
   - REST: comments like `// POST /path - OperationName` and handler method names like `handle_create_email_identity`
3. **Parses moto coverage** from moto's `IMPLEMENTATION_COVERAGE.md` (fetched from GitHub and cached when `vendor/moto` is absent), mapping snake_case operation names back to PascalCase Smithy names
4. **Parses floci coverage** from per-service Markdown docs (`docs/services/*.md`), reading `Category | Operations` tables where operations are comma- or middle-dot-separated PascalCase names. Source: `vendor/floci` if present, otherwise fetched from GitHub and cached for 7 days.
5. **Parses kumo coverage** from Go source files (`internal/service/<name>/*.go`): string literals in `Actions() []string` return values (Query/JSON services) and `handle<Op>` receiver method names (REST services). Source: `vendor/kumo` if present, otherwise fetched from GitHub and cached for 7 days.
6. **Parses integration-test coverage** from `crates/winterbaume-*/tests/integration_test.rs`, detecting AWS SDK client method calls and mapping them back to implemented Smithy operations
7. **Parses Terraform E2E coverage** from `crates/winterbaume-e2e-tests/tests/terraform/*.rs`, counting success-oriented terraform tests and distinct terraform resource types per service
8. **Generates the report** with a side-by-side overview table (winterbaume / moto / floci / kumo), integration-test and Terraform E2E summary tables, unimplemented services list, and detailed per-service checklists with `W[x] M[x] F[x] K[x]` markers

### Maintaining the service mappings

Four mappings are defined in `.agents/skills/api-coverage/scripts/generate_coverage.py`:
- `CRATE_TO_MODEL`: Maps winterbaume crate directory names to api-models-aws model directory names. Add an entry when creating a new service crate.
- `MODEL_TO_MOTO`: Maps api-models-aws model directory names to moto service names as they appear in moto's `IMPLEMENTATION_COVERAGE.md` (only where they differ, e.g., `"config-service": "config"`).
- `FLOCI_STEM_TO_MODEL`: Maps floci service document filename stems to api-models-aws model directory names (only where they differ, e.g., `"cognito": "cognito-identity-provider"`). Add an entry when floci adds a service whose doc filename does not match the model dir name.
- `KUMO_DIR_TO_MODEL`: Maps kumo `internal/service/<name>` directory names to api-models-aws model directory names (only where they differ, e.g., `"elbv2": "elastic-load-balancing-v2"`). Add an entry when kumo adds a service whose directory name does not match the model dir name.

### Output format

The report is written as Markdown with:
- Overview table: service, model, winterbaume count, moto count, floci count, kumo count, total, wb%, moto%, floci%, kumo%
- Overall aggregate stats for winterbaume, moto, floci, and kumo
- Integration-test summary lines plus a per-service integration-test table
- Terraform E2E summary lines plus a per-service Terraform E2E table
- List of AWS services not yet implemented
- Detailed per-service sections with `W[x] M[x] F[x] K[x]` side-by-side checklists for every operation, plus integration-test and Terraform E2E notes

## Terraform resource-type coverage

`generate_terraform_resource_coverage.py` answers: "for each winterbaume
service, how many of the AWS Terraform provider's resource types do we
actually handle?"

It reads the canonical converter manifest at
`crates/winterbaume-terraform/specs/<service>.toml` (the same specs the
`tf-converter-codegen` tool consumes), extracts each `[[resource]] type`
declaration, and compares the set against the AWS provider schema obtained
via `terraform providers schema -json`. The schema is cached at
`.agents-workspace/tmp/tf-schema/aws_provider_schema.json` (auto-generated
on first run; reused thereafter).

For each service:
- The "service prefix" is the longest common prefix of the handled
  resource types, capped at two underscore-separated segments after
  `aws_` so a 1-handled service doesn't produce an over-narrow prefix.
- Services with heterogeneous resource names ( `ec2` mixes `aws_vpc`,
  `aws_subnet`, `aws_security_group`, etc.; `rds` mixes `aws_db_*` and
  `aws_rds_*`; `elbv2` mixes `aws_lb*` and `aws_alb*` ) use a manual
  override list defined in `PREFIX_OVERRIDES` at the top of the script.
  Add a service to that table if its resources don't share a clean
  common prefix.
- Coverage % = handled / candidates_in_schema, where candidates are
  every schema resource matching the service's prefix(es).

Output sections:
1. A summary header with totals ( schema resources, classified, handled,
   missing ).
2. A table sorted by absolute missing-count desc.
3. Per-service "missing resources" list, suitable for triage and
   converter expansion.
4. ( If applicable ) a list of resources declared in specs but absent
   from the schema — typically typos or resources removed in newer
   provider versions.

## Terraform attribute coverage

`generate_terraform_converter_coverage.py` answers: "for each handled
resource type, how many of its TF schema attributes does the converter
actually touch?"

It greps each migrated converter and the TfModel spec for field-level
references, then computes inject% and extract% per resource. The
`terraform-converter` skill uses these numbers to gate "excellent"
( inject ≥ 60%, extract ≥ 50% ).

## Important Notes

- winterbaume-ses implements SES v2 (sesv2 model), not SES v1
- Only operations that exist in the official model are counted; spurious handler methods are filtered out
- Handler methods preceded by a `// STUB[category]: reason` comment (see `.agents/docs/QUALITY_GATE.md` "Stub Handler Discipline") are **excluded** from the `winterbaume` count and surfaced in a separate `Stubs` column, because they return empty/default responses without real logic. They still satisfy API routing but are not functionally implemented. To restore a stub to the count, remove the `STUB[...]` comment once real logic is added.
- The script also performs **heuristic stub detection** for unannotated drift: a handler whose body is short, contains a default/empty response indicator (`::default()`, `..Default::default()`, `Vec::new()`, `success_empty`, ...), and contains no state-access indicator (`self.backend`, `with_state`, `?;`, `request.body`, ...) is also classified as a stub even if it is missing the canonical `// STUB[<category>]: ...` annotation. The annotated-vs-heuristic split is not reported separately in the table; both flavours appear in the `Stubs` column. If a handler is misclassified, the canonical fix is to either annotate it explicitly or to make its real logic obvious to the heuristic.
- Integration-test coverage is reported against implemented operations only. It is derived from SDK method calls found in `tests/integration_test.rs`, so it is a useful approximation rather than a proof that every behavioural path is covered.
- Terraform E2E coverage is reported separately from API operation coverage. One Terraform resource can exercise multiple API operations, so the E2E metrics are a complementary validation signal rather than an operation-count substitute.
