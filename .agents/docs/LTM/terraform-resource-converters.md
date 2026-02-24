# Terraform Resource Converters

## Summary

The `winterbaume-terraform` crate translates Terraform resource state into Winterbaume service state and back again through `TerraformResourceConverter` implementations. The durable pattern is that converter work is not just per-resource glue code: it depends on `StatefulService` views, schema-aware coverage tooling, disciplined registration, and round-trip tests that prove Terraform blocks survive inject, snapshot, and extract without drift. By 2026-04-20 the converter layer had expanded into a large workspace subsystem with 267 converter structs, coverage tooling tied to the official Terraform AWS provider schema, and a new `/terraform-converter` skill to standardise future work.

## Key Facts

- The converter layer injects through service `merge()` and extracts through service `snapshot()`, so `views.rs` is the contract boundary for Terraform support.
- Coverage moved from ad-hoc converter counting to a schema-driven audit using `.agents/skills/api-coverage/scripts/generate_terraform_converter_coverage.py`.
- The 2026-04-20 expansion grew the crate from 127 converter structs across 58 files to 267 converter structs across 137 files.
- Final coverage after the enhancement passes reached 67.4% inject and 59.7% extract against the Terraform AWS provider schema, with all 265 coverable converters rated `excellent` and 2 resource types effectively `N/A`.
- State-view completeness is the main gating factor for converter fidelity. Poor or fair converter ratings usually mean the service view does not yet model the resource family or nested block.
- Nested-block work should be treated as a full vertical slice: state types, service handlers when needed, views, converter inject or extract logic, and Terraform E2E no-drift checks.
- Registration is a separate completion step. New converter files and structs are not active until they are wired into the injector build path.
- Extraction is now multi-scope aware for opted-in regional services. `ConversionContext` carries default account and region fallbacks, `ExtractedResource` carries the actual account and region, and `TerraformInjector::extract_all()` handles post-collection name collisions.
- DynamoDB, SQS, and SNS now expose scope discovery through their backend traits, allowing their converters to participate in multi-scope extraction even though they store `Arc<dyn Backend>`.
- Parallel worktree batches can create merge gaps. Journal claims about converter completion must be checked against the main working tree before being treated as durable truth.

## Details

### Converter Contract

Each converter follows the same broad shape:

1. wrap an `Arc<Service>`
2. implement `TerraformResourceConverter`
3. parse Terraform attributes in `do_inject()`
4. build or extend the service `StateView`
5. call `service.merge()` to inject the modelled state
6. call `service.snapshot()` in `do_extract()` and turn view data back into `ExtractedResource` values

This keeps the converter layer aligned with the same durable state model used by snapshots, restore paths, and notifier consumers.

### Coverage Tooling and Rating Rules

The original audit work established that converter coverage has to be measured at converter-implementation granularity, not at module granularity. That rule became much more important once the crate moved to multi-converter files and large service batches.

The later schema-driven audit added a more durable measurement loop:

- fetch the Terraform AWS provider schema via `terraform providers schema -json`
- cache it under `.agents-workspace/tmp/tf-schema/aws_provider_schema.json`
- statically inspect each converter file for inject-field reads and extract-field emissions
- compare those field sets against the provider schema
- rate each resource as `excellent`, `good`, `fair`, or `poor`

The rating thresholds that mattered during the large 2026-04-20 push were:

- `excellent`: inject >= 60% and extract >= 50%
- `good`: inject >= 40% or extract >= 30%
- `fair`: inject >= 20% or extract >= 15%
- `poor`: below fair

Two durable parser lessons came out of the coverage-tool work:

- multi-converter files must be matched by precise `resource_type()` regexes rather than loose substring search, because `depends_on_types()` can create false matches
- extract coverage needs to account for generic `*.insert()` patterns, not only `attrs.insert()`

### Round-Trip and Inject-Only Converters

Round-trip extraction is the preferred steady state, but not every Terraform resource wants a symmetric extract path.

The reference example is `aws_route`:

- inject path: append or merge route entries into an existing route table
- extract path: intentionally empty because routes are materialised through the owning `aws_route_table` converter

This is acceptable when:

- the resource is modelled as a subcomponent of another durable resource
- extraction through the parent resource is less lossy and less duplicative

### State-View Boundaries

The converter layer is only as complete as the underlying `StateView`. The early EC2 work made that explicit, and the 2026-04-20 converter push generalised it across the whole workspace.

The durable rule is:

- if the service view cannot faithfully represent the Terraform resource, extend `views.rs` first
- if the view is missing nested blocks or sibling resource families, converter coverage will stall no matter how much parsing logic is added

This pattern was reinforced during the poor or fair converter cleanup. Several converters were blocked until their services added or expanded durable view types, including:

- DAX parameter groups and subnet groups
- EBS volumes
- EC2 Instance Connect endpoints
- MemoryDB ACLs
- Pinpoint email channels
- Service Catalog products
- FSx file-system metadata
- SageMaker domain VPC and auth fields
- Elastic Beanstalk environment metadata

### 2026-04-20 Mass Expansion

The 2026-04-20 converter wave had five durable components:

1. coverage analysis tooling
2. field enhancement for existing converters
3. new converter files for previously uncovered services
4. `StateView` expansion for services whose converters were blocked by missing durable state
5. large-scale integration testing

The expansion added converters for many new service families, including API Gateway, AppConfig, GuardDuty, SecurityHub, Service Discovery, Bedrock, SageMaker, QuickSight, Macie2, DMS, Service Quotas, Signer, Budgets, OSIS, MediaPackage, Resilience Hub, and Timestream Query.

The important durable lesson is not the exact service list but the workflow:

- use the schema report to find poor, fair, or just-below-threshold converters
- prefer real state or view modelling over one-off converter hacks
- use nested Terraform blocks in integration tests once the state layer can round-trip them
- regenerate the report after each pass rather than relying on intuition

### Nested-Block Work

The nested-block follow-up changed the shape of converter work. Once top-level attribute coverage reached `excellent`, the next ceiling became nested Terraform structures such as:

- `logging_configuration`
- `encryption_configuration`
- `vpc_config`
- `snapshot_copy`
- `lifecycle_policy`
- `options`

The durable implementation rule is that nested-block support should be treated as one vertical:

1. add or restructure service state types if the service owns the block
2. update handler create or update flows when those blocks enter through service APIs
3. expose the shape in `views.rs`
4. inject and extract the Terraform nested block in the converter
5. add Terraform E2E coverage that proves `terraform plan` sees no drift after apply

This is why some services are low-complexity converter-only fixes, while others require state, handler, and view work as well.

Representative completed verticals from the 2026-04-21 batch were:

- ECR: `encryption_configuration` and `image_scanning_configuration`
- EFS: `lifecycle_policy` and `protection`
- SFN: `logging_configuration`, `tracing_configuration`, and `encryption_configuration`
- Neptune: `serverless_v2_scaling_configuration` and `parameter`
- ACM: nested `options` with converter round-tripping

### 2026-04-22 Nested-Block Mass Expansion

A mass expansion on 2026-04-22 resolved all 64 remaining open TODO items for "Terraform converter: StateView missing nested structures", touching ~55 service crates in three parallel waves.

Stats: **126 files changed**, +2843/−158 lines, 64 TODO items resolved.

Services already complete beforehand (no changes needed): sfn, neptune, efs, ecr, applicationautoscaling.

Two implementation strategies were used:

- **Typed nested structs** — used when the service crate already had matching domain types. `From`/`Into` conversions link domain and view types. Examples: cloudtrail ( `AdvancedEventSelector` / `AdvancedFieldSelector` ), identitystore ( `Name` / `Email` / `Address` / `PhoneNumber` ), networkmanager ( `Location` / `AwsLocation` ), amplify ( `AutoBranchCreationConfig` / `CacheConfig` / `CustomRule` ), synthetics ( `CanaryArtifactConfig` / `CanaryRunConfig` / `CanaryVpcConfig` ).
- **Opaque JSON blobs** ( `Option<serde_json::Value>` / `Vec<serde_json::Value>` ) — used for deeply nested blocks where full typing would duplicate auto-generated `wire.rs` structures. Examples: cognito and opensearch extra-config fields, most firehose destination configs, eventbridge targets.

Shared helpers were added in `networkfirewall` ( `parse`/`emit encryption_config` ), `autoscaling` ( `extract_json_array` ), and `rds` ( `extract_json_array` ).

### Converter Registration

New converter files and structs are not active until registered. After the 2026-04-20 and 2026-04-21 expansion waves, a dedicated registration pass was required:

- `crates/winterbaume-server/src/main.rs`: 69 new `InjectableServices` fields and `injector.register()` calls.
- `crates/winterbaume-terraform/src/converters/mod.rs`: 72 `pub mod` entries.
- `crates/winterbaume-terraform/Cargo.toml`: 79 workspace dependencies.

Final count after full registration: **267 converter structs across 131 modules**.

The `cargo clippy -p winterbaume-terraform` (lib-only) check can pass even when `--all-targets` fails, because `tests/integration_test.rs` imports modules that may be commented out in `mod.rs`. Always verify with `cargo clippy --all-targets -p winterbaume-terraform`.

The backlog that remained after the batch was intentionally moved into `.agents/docs/TODO.md` as a long list of services whose views still lack nested-block support.

### Skill and Workflow Standardisation

The `/terraform-converter` skill was created to codify the now-stable converter workflow. Its durable value is that it makes the expected sequence explicit:

- detect whether the task is a new converter or an enhancement
- inspect service state and views before writing converter logic
- register the converter in the module tree and dependency graph
- use the coverage report to identify missing fields
- add integration tests for inject and extract
- regenerate coverage and verify the converter reaches `excellent`

The skill also documents recurring implementation pitfalls, including:

- V1 versus V2 Terraform resource-type mismatches
- schema false positives in multi-converter files
- `tags_all` counting rules
- large `json!` extract blocks needing higher recursion limits
- merge versus restore semantics in converter inject paths

### Registration and Maintenance

Adding a converter is not complete until all of these surfaces are updated:

- `crates/winterbaume-terraform/src/converters/mod.rs`
- workspace dependencies for the owning service crate
- integration or E2E tests
- any coverage-report logic that counts supported Terraform resource types
- injector registration where Terraform converters are instantiated

The last point matters because the 2026-04-20 mass expansion deliberately separated converter implementation from injector wiring. A converter file that exists on disk is still inert until the injector build path registers it.

### Verification Lessons and the Redshift Merge Gap

The 2026-04-21 peer review added an important operator lesson: journal summaries of parallel worktree batches can overstate what actually landed in the main tree.

The concrete example was Redshift nested-block work:

- the journal summary claimed the tuple-based `snapshot_copy` state had been replaced by a dedicated struct and view type
- the main working tree still retained the tuple shape
- the Redshift converter still discarded `logging` and hard-coded `snapshot_copy: None`
- the E2E test only proved apply success, not nested-block round-tripping

The durable rule is:

- do not treat a batch summary as authoritative without checking the main tree
- when nested-block work is claimed complete, verify both state shape and converter round-tripping
- treat no-drift Terraform checks as the real end state, not merely "builds cleanly" or "apply succeeds"

### Account and Region Scope Contract

The 2026-04-24 Lambda audit clarified the converter and state-view scope contract:

- `StatefulService` method arguments select the backend bucket: `(account_id, region)`
- a top-level `*StateView` represents exactly that scoped bucket
- top-level views must not persist their own durable `account_id`, `region`, or compensating scope maps
- `snapshot(account_id, region)`, `restore(account_id, region, view)`, and `merge(account_id, region, view)` must use the supplied scope rather than hard-coded defaults, except for documented global-service exceptions
- Terraform converters must preserve inject/extract scope symmetry when resources specify a non-default account or region

Lambda was the reference fix. `LambdaStateView` no longer stores top-level scope fields, restore and merge seed runtime `LambdaState.account_id` / `region` from the `StatefulService` arguments, and the merge path was expanded to preserve newer durable Lambda state families.

### Multi-Scope Extraction

The Terraform converter layer no longer assumes one fixed `(account_id, region)` during extraction.

Durable implementation shape:

- `InjectionContext` was renamed to `ConversionContext`
- `ConversionContext` fields are `default_account_id` and `default_region`, making defaults explicit
- `extract_account_id()` was added alongside `extract_region()`
- `ExtractedResource` now carries `account_id` and `region`
- `BackendState::scopes_with_state()` returns sorted `(account_id, region)` pairs without creating empty buckets
- `TerraformInjector` stores a `ConverterEntry` with the converter and an optional `ScopeProvider`
- `register_with_scopes()` opts regional services into multi-scope extraction at registration time
- `extract_all()` calls each converter once per discovered scope and then performs post-collection collision detection

Collision handling happens after all resources are collected. When multiple resources of the same Terraform type share the same extracted name, every member of the colliding group is qualified with account and region. This avoids the false-positive case where only non-default regions are suffixed and a default-region resource naturally has a region-looking suffix.

The design deliberately avoided changes to `TerraformResourceConverter` and `StatefulService`. Scope awareness flows through concrete `BackendState` inspection, public per-service `scopes_with_state()` methods, and registration-time closures in the injector build path.

Opted-in services from the first implementation batch:

| Service | Notes |
|---------|-------|
| S3 | `register_with_scopes()` for bucket converter |
| Lambda | all registered Lambda converters use scoped extraction |
| ECS | service, task definition, and cluster converters |
| RDS | registered RDS converters |
| EBS | service method added; registration depends on converter coverage |
| EKS | cluster and node group converters |

DynamoDB, SQS, and SNS originally needed a separate design because their converters use `Arc<dyn Backend>` rather than a concrete `BackendState`.

That follow-up is now resolved:

- `DynamoDbBackend`, `SqsBackend`, and `SnsBackend` expose `scopes_with_state()`
- in-memory backends delegate to their underlying `BackendState::scopes_with_state()`
- Redis backends return an empty list, which makes extraction fall back to the default scope
- the service structs expose scope discovery through their backend trait objects
- server registration uses `register_with_scopes()` for DynamoDB, SQS, and SNS converters

This preserves the backend abstraction while still giving Terraform extraction the concrete scope list when the active backend can provide it.

`extract_account_id(attrs, &ctx.default_account_id)` was also adopted where Terraform resources can genuinely override the management account scope: Organizations accounts, OUs, and policies; RAM resource shares; and GuardDuty detectors and filters. GuardDuty member resources deliberately do not use it because their `account_id` attribute is the member account, not the management account.

Scope-aware ordering for `inject_all()` remains intentionally rejected. Terraform state files are single-scoped per provider configuration, and injection is a write-side operation that should respect the explicit provider scope rather than auto-discovering other scopes.

The implementation also fixed hard-coded region strings in extraction output:

- Lambda `invoke_arn`
- Kafka broker endpoints

CloudFront's `us-east-1` remained intentional because it is modelled as a global service.

### 2026-04-26 Converter Additions

Three new converters landed alongside the AppFabric / AppFlow / Application Cost Profiler service crates:

- `AwsAppFabricAppBundleConverter`. AppBundle ARN injected from `arn` attribute or synthesised as `arn:aws:appfabric:<region>:<account>:appbundle/<uuid>`. Uses the trailing ARN path component as the state map key. Pulls `customer_managed_key_arn` and `tags`.
- `AwsAppFlowFlowConverter`. Flow ARN built from `flow/<name>` template. `trigger_config`, `source_flow_config`, `destination_flow_config_list`, and `tasks` stored as opaque `serde_json::Value`. Maps Terraform's `destination_flow_config` ( singular ) to the wire's `destination_flow_config_list`, and `task` ( singular block-list ) to `tasks`.
- `AwsApplicationCostProfilerReportDefinitionConverter`. Reads the `destination_s3_location` block ( single-element list in Terraform ) into `S3Location { bucket, prefix }`. Defaults `report_frequency` to `DAILY` and `format` to `CSV` if absent.

A separate session pass added `AwsPlacementGroupConverter` ( in `src/converters/ec2.rs` ) covering `aws_placement_group`. Inject reads `name`, `strategy`, `partition_count`, `spread_level`, `tags`, `tags_all`, `name_prefix`, `timeouts`, `id`/`placement_group_id`, `arn`. Extract emits all of these plus `placement_group_id`. Uses `merge` ( not `restore` ) to play well with batch terraform.

Two material round-trip bugs surfaced from staged-change review on these converters and were fixed during the round-5 merge-back:

- **AppFlow JSON-shape mismatch**. Terraform state uses provider-schema shapes ( singleton block arrays, snake_case keys ) while AppFlow handlers decode AWS REST JSON shapes ( camelCase keys and discriminated connector-property objects ). The converter now uses `tf_to_aws_*` helpers that lift length-1 object arrays, camel-case keys, and map connector-property discriminators such as `s3` -> `S3` and `sapo_data` -> `SAPOData`. `test_inject_appflow_flow_describe_via_sdk` injects realistic Terraform state and verifies `describe_flow` through `aws-sdk-appflow` preserves trigger, source, destination, and tasks.
- **EC2 placement-group id/name swap**. Extraction writes `"id": pg.group_name` and `"placement_group_id": pg.group_id`, matching Terraform's convention that `id` is the placement group name. Injection now prefers `placement_group_id` for the `pg-...` GroupId, treats `id` as the name, and synthesises a `pg-...` id only when no placement-group id is present. `test_inject_placement_group_preserves_pg_id_for_describe_by_id` verifies `DescribePlacementGroups` by `group_ids` works after tfstate injection.

### Converter-Wiring Touchpoints

For each new converter, three workspace touch points must be updated:

1. `crates/winterbaume-terraform/Cargo.toml`: add `winterbaume-<name> = { workspace = true }` dep.
2. `crates/winterbaume-terraform/src/converters/mod.rs`: add `pub mod`.
3. `crates/winterbaume-server/src/main.rs`: add the service to `InjectableServices`, construct the `Arc<...>` once at the top of `register_all_services()` ( replacing the inline `Arc::new(...)` in the services Vec ), populate the corresponding field in the `InjectableServices` initializer, and call `injector.register(...)` inside `load_tfstate()`.

### Provider Compatibility Fixes in Converter Paths

Terraform E2E triage found converter-specific provider assumptions that SDK tests did not expose:

- `AwsBatchComputeEnvironmentConverter` must use Terraform's `name` attribute, not `compute_environment_name`
- extracted and injected fixture data must mirror Terraform provider schema names exactly
- listener-rule and CloudWatch protocol fixes sit in service handlers and codegen, but the failure mode was visible only through Terraform provider refresh paths

The durable rule is to check provider schema names before treating an AWS API field name as a Terraform state field name.

## Files

- `crates/winterbaume-terraform/src/converters/mod.rs` - module registry
- `crates/winterbaume-terraform/src/converters/*.rs` - individual converter implementations
- `crates/winterbaume-terraform/src/converter.rs` - `ConversionContext`, `TerraformResourceConverter`, and `ExtractedResource`
- `crates/winterbaume-terraform/src/injector.rs` - converter registration, `register_with_scopes()`, and `extract_all()`
- `crates/winterbaume-terraform/src/util.rs` - shared Terraform attribute helpers such as `extract_region()` and `extract_account_id()`
- `crates/winterbaume-terraform/tests/integration_test.rs` - inject and extract tests
- `crates/winterbaume-e2e-tests/tests/terraform/*.rs` - Terraform E2E no-drift coverage for nested-block resources
- `crates/winterbaume-*/src/views.rs` - service state views that gate converter fidelity
- `crates/winterbaume-*/src/types.rs` - durable service-owned nested structures needed by converters
- `.agents/skills/api-coverage/scripts/generate_terraform_converter_coverage.py` - Terraform resource coverage accounting
- `.agents/skills/terraform-converter/SKILL.md` - standardised workflow for new or enhanced converters
- `.agents/docs/TERRAFORM_CONVERTER_COVERAGE.md` - generated per-resource coverage report

## Test Coverage

- 2026-04-06 audit result: all original 99 converter implementations were covered by integration tests
- 2026-04-20 mass expansion increased integration coverage from 99 tests to 260 tests in `winterbaume-terraform`
- the state-view expansion pass added 10 integration tests for newly modelled resource families
- the final enhancement pass reached 100% resource-type test coverage across 267 converter structs
- the multi-scope extraction implementation kept 261 Terraform converter integration tests passing and added coverage for non-default region extraction limitations and collision-safe extraction
- DynamoDB, SQS, and SNS multi-scope backend trait support kept their crate-local integration suites passing and preserved the 261 Terraform converter test pass
- nested-block work should add Terraform E2E tests, not only integration tests, because no-drift plan behaviour is the real contract
- claimed nested-block completions should be verified against the main tree, especially after parallel worktree batches

## Pitfalls

- Do not write converters directly against private state when a `StateView` extension would preserve a clearer contract.
- Do not assume module-level test coverage implies converter-level test coverage.
- Do not force a symmetric extract path when the resource is naturally owned by a parent converter.
- Do not forget `depends_on_types()` for resources with ordering requirements.
- Do not treat top-level `excellent` coverage as proof that nested Terraform blocks round-trip correctly.
- Do not mark converter work complete until injector registration is updated where required.
- Do not copy Terraform nested-block JSON directly into a view when handlers expect AWS REST JSON shapes. Lift singleton blocks, convert snake_case to camelCase, and map discriminators before storing opaque `serde_json::Value`.
- Do not trust parallel-agent summaries without checking the main working tree for the claimed state-shape and converter changes.
- Do not rely on `terraform apply` success alone when the real requirement is apply followed by a no-drift plan.
- Do not store account or region selectors inside top-level `*StateView` structs. Scope comes from the `StatefulService` method arguments.
- Do not assume `extract_region()` alone gives multi-scope symmetry. Multi-account cases also need `extract_account_id()` adoption where the resource can override account.
- Do not opt global services into multi-scope extraction unless the service has an explicit global-state contract.
- Do not make `inject_all()` auto-discover scopes unless Terraform injection semantics change; extraction and injection have different scope contracts.
