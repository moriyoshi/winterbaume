# DynamoDB PartiQL Integration

## Summary

The `winterbaume-partiql` crate provides DynamoDB PartiQL statement parsing, IR construction, and translation as an independent library. The original integration with `partiql-lang-rust` ( `partiql-parser` 0.14 + `partiql-ast` ) was replaced on 2026-04-28 by a hand-rolled recursive-descent parser that goes directly from text to a typed `DdbOperation` IR. The crate now has full AWS DDB-PartiQL spec parity ( all four statement types, every documented operator and conditional function, `EXISTS` for transactions, RETURNING, ORDER BY, parameter substitution ) and is empirically validated against real AWS DynamoDB in `ap-northeast-1`.

## Key Facts

- The crate no longer depends on `partiql-parser`/`partiql-ast`. Direct deps are just `serde_json` and `thiserror`. `cargo tree -p winterbaume-partiql` is two lines. Removing the upstream parser eliminated the lalrpop build script that took 30+ minutes per fresh `CARGO_TARGET_DIR` and dropped ~50 transitive crates ( logos, ariadne, plus the lalrpop generation chain ).
- The hand-rolled parser is recursive descent with one-token lookahead. Source layout under `crates/winterbaume-partiql/src/parser/`: `lexer.rs` ( ~493 LOC ), `expr.rs` ( ~414 LOC ), `stmt.rs` ( ~328 LOC ), `mod.rs` ( ~141 LOC ), `tests.rs` ( ~771+ LOC ).
- The `?` parameter placeholder is now a first-class lexer token. Parameters are substituted at parse time at value positions ( not as a textual pre-pass ). Mismatched count returns `PartiqlError`.
- The IR is fully expression-based. `Expression` enum: `Literal(AttributeValue)`, `Path(String)`, `BinaryOp(Box<Expression>, ArithOp, Box<Expression>)`, `Neg(Box<Expression>)`. `ArithOp` is `Add` / `Sub` only ( the spec lists no `*` / `/` / `%` ). All Compare/Between/In/Size operands are `Expression`. `SetValue::Expr(Expression)` replaces the old separate `Literal` / `Add` / `Sub` variants.
- `Condition` carries `Compare(Expression, CmpOp, Expression)`, `Between/In/BeginsWith/Contains/Size`, `IsMissing` / `IsNull` / `AttributeType`, and the boolean composers `And` / `Or` / `Not`. `EXISTS` is its own top-level statement variant ( `DdbOperation::Exists(Box<SelectOp>)` ), not a Condition.
- DynamoDB items use the typed `AttributeValue` enum ( `S`, `N`, `B`, `BOOL`, `NULL`, `SS`, `NS`, `BS`, `L`, `M` ) defined in `winterbaume-dynamodb/src/types.rs`; serde still serialises to the standard DynamoDB JSON wire shape.
- PartiQL execution runs through `execute_partiql_via_backend(...)` so custom backends ( including `winterbaume-dynamodb-redis` ) see the same behaviour as the in-memory backend.
- DynamoDB backend query paths accept an optional `index_name`, and tables persist typed GSI/LSI definitions so secondary-index query behaviour, snapshots, Redis persistence, and Terraform converter injection share one model.
- DynamoDB write paths append real stream change records ( `INSERT` / `MODIFY` / `REMOVE` ) derived from the same typed item model. `TableState.stream_records: Vec<StreamChangeRecord>` and `stream_sequence_counter: u64` are part of `TableStateView` with `#[serde(default)]` so they round-trip through snapshot / restore.
- AWS-fidelity has been verified empirically against real DynamoDB. Some MockAws behaviour is intentionally divergent, see Pitfalls.

## Details

### Architecture

```text
SQL string + parameters
        |
        v
winterbaume-partiql::parse_statement()
  |- lexer::tokenise()                     -- single pass, hand-rolled
  |- parser::parse_top_level()             -- one-token lookahead
  |    |- peek `Ident("EXISTS") (`         -> parse_exists -> DdbOperation::Exists(SelectOp)
  |    |- SELECT -> stmt::parse_select     -> DdbOperation::Select(SelectOp)
  |    |- INSERT -> stmt::parse_insert     -> DdbOperation::Insert(InsertOp)
  |    |- UPDATE -> stmt::parse_update     -> DdbOperation::Update(UpdateOp)
  |    `- DELETE -> stmt::parse_delete     -> DdbOperation::Delete(DeleteOp)
  |       (WHERE / value positions go through expr::parse_expression
  |        which substitutes `?` against the parameter vector at parse time)
        |
        v
DdbOperation enum (Select / Insert / Update / Delete / Exists)
        |
        v
winterbaume-dynamodb::partiql_exec::execute_partiql_via_backend()
  |- Select -> split key/filter conditions -> backend.query() or backend.scan()
  |- Insert -> backend.put_item()
  |- Update -> extract key -> backend.update_item()
  |- Delete -> extract key -> backend.delete_item()
  `- Exists -> evaluate inner SELECT       -> bool (non-empty result == true)
```

`evaluate_expression(&Expression, Option<&Item>) -> Option<AttributeValue>` walks the IR for any expression position ( WHERE / RETURNING projection / SET RHS ). `apply_cmp_op` and `apply_arith_op` apply the operators on resolved values. `atomic_add_delta` / `atomic_sub_delta` detect the `target = target ± literal` shape and emit DynamoDB's atomic `ADD` action; anything else ( e.g. `target = price + tax`, `target = (a + b) - c` ) is computed eagerly against the pre-update item and emitted as a `SET`.

### Parameter binding

DynamoDB's `ExecuteStatement` API accepts `Parameters: Vec<AttributeValue>` where `?` placeholders are positionally bound. The hand-rolled parser carries a `Vec<AttributeValue>` and consumes one element each time it encounters a `?` token at a value position. There is no textual pre-substitution and no quote-awareness logic ( `?` inside a string literal cannot reach the value-position parser, by construction ).

### Typed AttributeValue model

`winterbaume-dynamodb` now defines:

```rust
pub enum AttributeValue {
    S(String),
    N(String),
    B(String),
    #[serde(rename = "BOOL")]
    Bool(bool),
    #[serde(rename = "NULL")]
    Null(bool),
    SS(Vec<String>),
    NS(Vec<String>),
    BS(Vec<String>),
    L(Vec<AttributeValue>),
    M(HashMap<String, AttributeValue>),
}
```

with:

```rust
pub type Item = HashMap<String, AttributeValue>;
```

This keeps state and execution logic type-aware while preserving the external DynamoDB JSON representation through serde. It also made the DynamoDB Streams bridge explicit: the streams crate converts typed items to generated wire attributes through a serde round-trip because both sides share the same JSON shape.

### Intermediate representation

The `DdbOperation` enum captures the essential semantics of each statement:

- `SelectOp`: table_name, index_name, projection, where_clause, order_ascending, order_by_attr
- `InsertOp`: table_name, item
- `UpdateOp`: table_name, key_conditions, sets, removes, returning
- `DeleteOp`: table_name, key_conditions, returning
- `Exists(Box<SelectOp>)`: only valid as the body of a transactional statement; `handle_execute_statement` and `handle_batch_execute_statement` reject it with `ValidationException`

`Condition` is a recursive enum: `Compare(Expression, CmpOp, Expression)`, `Between/In/BeginsWith/Contains/Size`, `IsMissing` / `IsNull` / `AttributeType`, `And` / `Or` / `Not`. `Compare` carries any `Expression` on either side, which is what enables `path + literal`, `literal`, or pure `path` everywhere a comparable appears.

`SetValue` supports `Expr(Expression)` ( covers literals, paths, and arithmetic uniformly ) plus `ListAppend`, `SetAdd`, `SetDelete`. The function-call SET RHS variants keep their `(String, AttributeValue)` shape because AWS docs only document literal collection args for those.

### WHERE clause handling

The `winterbaume-partiql` crate returns all WHERE conditions as-is. The split into key conditions (usable for DynamoDB Query) vs filter conditions (post-scan filtering) happens in `winterbaume-dynamodb`'s `partiql_exec.rs`, which knows the table's key schema. This keeps the parser crate table-schema-agnostic.

### Completed execution features

The early implementation left several parsed concepts only partially executed. The durable completed behaviors are:

- index-qualified SELECTs use a correct mock fallback: if `index_name` is present, treat all conditions as filters and scan the base table rather than pretending a separate GSI store exists
- arithmetic SET clauses (`attr + N`, `attr - N`) translate into DynamoDB add operations
- `list_append`, `set_add`, and `set_delete` fetch the current item first and then apply in-memory list or set semantics
- empty sets are removed instead of persisted, matching DynamoDB's prohibition on empty-set values
- `RETURNING ALL NEW`, `ALL OLD`, `MODIFIED NEW`, and `MODIFIED OLD` are parsed and routed through `PartiqlResult::Items`
- `ORDER BY` on scan paths sorts after result collection using the requested attribute
- `BatchExecuteStatement` SELECTs must specify the partition key; otherwise execution returns `ValidationException`

### Parser rewrite ( 2026-04-28 )

The 30+ minute `partiql-parser` 0.14 lalrpop build script was eliminated by replacing both `partiql-parser` and `partiql-ast` with a hand-rolled parser. Build scripts run outside `RUSTC_WRAPPER`, so the sccache-wrapper deduplication could not help, and concurrent agents in different `CARGO_TARGET_DIR`s each ran the full lalrpop generation.

The DDB-PartiQL surface is small, stable, and our usage was narrow ( only `partiql_ast::ast` was imported into the old `translate.rs`, then immediately lowered to `DdbOperation` ). A direct text → `DdbOperation` parser is roughly 1376 LOC ( excluding tests ); the old `translate.rs` and its supporting `value.rs` together were ~1901 LOC and have been orphaned ( still on disk pending a clean `git rm` ).

One latent bug was fixed by the rewrite: the old prefix match accepted `RETURNING ALL NEW` and silently ignored a trailing `*`. Real DynamoDB requires the `*` form `RETURNING ALL NEW *` ( SQL "all columns" sigil ), and integration tests had been sending it. The new parser explicitly accepts the optional trailing `Star`, so both forms work and the integration tests stay green.

Behaviour preserved exactly across the rewrite: `MISSING` literal serialises as `{"NULL": true}`, mixed-type `<<…>>` errors with `InvalidValue("set must contain all strings or all numbers")`, path indexing renders as `Tags[0]` in `removes`/`Condition` strings, numeric literals preserve their textual form ( `3.14` stays `3.14`, no f64 round-trip lossiness ).

### Expression IR + arithmetic / sub-expression parity

After the rewrite landed, the IR was refactored a second time for full arithmetic parity. Per the AWS PartiQL operators reference, `+` and `-` should work in any value position. The previous IR only modelled `path + literal` and `path - literal` in SET RHS.

Changes:

- `Expression` enum and `ArithOp` ( `Add` / `Sub` ) added to `operation.rs`. Helper accessors `as_literal()` / `as_path()` keep test patterns terse.
- `Condition::Eq/Ne/Lt/Le/Gt/Ge` collapsed into `Condition::Compare(Expression, CmpOp, Expression)`. Both operands are full expressions.
- `Condition::Between/In/BeginsWith/Contains/Size` value positions all use `Expression`.
- `SetValue::Literal/Add/Sub` collapsed into `SetValue::Expr(Expression)`.
- New `parse_expression` in `parser/expr.rs` with proper precedence: binary `+`/`-` left-assoc, unary `-` prefix, primary ( path / literal / `(expr)` / `?` ). `parse_value` retained for literal-only contexts ( INSERT VALUE struct fields, function-call literal args ).
- Three previously-rejected arithmetic shapes ( `path + path`, chained, in-WHERE ) flipped from gap tests to positive tests.
- New `evaluate_expression` and `apply_cmp_op` / `apply_arith_op` helpers in `partiql_exec.rs`. Whole-number arithmetic results are formatted as integers ( e.g. `5 + 5` → `"10"`, not `"10.0"` ).

### EXISTS conditional function ( 2026-04-29 )

Per AWS docs, `EXISTS(SELECT … WHERE …)` is a "Conditional function" ( alongside `attribute_type` / `begins_with` / `contains` ), not a statement keyword. It can only be used in transactional operations ( `ExecuteTransaction` ). The lexer treats `EXISTS` as a regular identifier; `parse_top_level` peeks for `Ident("EXISTS") (` and dispatches to `parse_exists`, which produces `DdbOperation::Exists(Box<SelectOp>)`. ( An earlier version added `EXISTS` as a Keyword in the lexer; that was wrong because it would have shadowed the identifier role inside expressions.)

Handler enforcement:

- `handle_execute_statement` rejects EXISTS with `ValidationException` and the AWS-exact message: `"EXISTS can only be used in ExecuteTransaction write requests."`
- `handle_batch_execute_statement` rejects EXISTS in `BatchStatementError` ( currently with shorter phrasing; aligning to AWS verbatim is an open follow-up ).
- `handle_execute_transaction` validates the inner SELECT's WHERE before dispatch: requires the full primary key by Eq + at least one non-key predicate. Empty Items result aborts the whole transaction with `TransactionCanceledException`. The AWS error wording is `"EXISTS() must contain a single item read with additional condition"`.

### AWS-fidelity: empirical verification

The full conditional-function and arithmetic surface was tested against real AWS DynamoDB ( `ap-northeast-1` ) before declaring spec parity. Empirical findings worth keeping:

- AWS rejects `EXISTS(...)` in any *expression* position ( neither inside SELECT projection nor inside WHERE — error message "Unexpected path component" treats `EXISTS` as a misplaced attribute path ). Our top-level-only model matches the actual AWS behaviour, even though full PartiQL spec allows EXISTS as a value-producing predicate.
- `EXISTS` inner-SELECT requires the full primary key by Eq AND at least one non-key predicate. AWS rejects key-only EXISTS with "must contain a single item read with additional condition" — bizarre but real.
- AWS rejects `+`/`-` arithmetic in WHERE-clause operands ( `WHERE counter = 5 + 5` → "Unsupported operator in Condition Expression. Operator: +" ). MockAws's parser produces `Compare(BinaryOp(...), CmpOp::_, _)` and the runtime evaluates it. This is a documented divergence, not yet fixed; see TODO.md `partiql-arithmetic-in-where`.
- AWS rejects `-path` ( unary minus on a path ). MockAws accepts it via `Expression::Neg(Path(...))`. Same kind of divergence; see TODO.md `partiql-unary-neg-path`.
- AWS supports MORE arithmetic in SET RHS than the docs imply: `path + path`, `literal + path`, `path + literal + literal` ( chained ), `(path + 1) - 2` ( parenthesised ), `path + -literal` ( negative-literal as operand ) all work in SET. Our parser already supports all of these; no divergence here.
- `IS NULL` and `IS MISSING` are distinct in AWS: `IS NULL` matches `{"NULL": true}` attributes only, `IS MISSING` matches absent attributes. MockAws preserves the distinction.
- `attribute_type(path, 'TYPE')` accepts exactly `{N, BS, L, B, NULL, M, S, SS, NS, BOOL}`. Invalid type names are rejected by AWS with an explicit "valid types" list.
- `contains(path, val)` is overloaded: String substring match, SS/NS/BS element membership, L element-equality. The runtime evaluator was string-only before this sweep and has been extended to cover all five shapes.

### 2026-04-04 DML parser edge cases ( historical, pre-rewrite )

Notes preserved for archaeology even though the underlying integration code is gone. Pre-rewrite, two `partiql-parser` 0.14-specific bugs had to be worked around:

- `IN` conditions such as `pk IN ['x', 'y', 'z']` arrived as `Expr::Lit(Lit::ListLit(...))` rather than `Expr::List`, and the generic fallback arm produced a single DynamoDB `L` value instead of a list of attribute values.
- string literals such as `'O''Brien'` preserved the SQL escape sequence; `CharStringLit` values had to be unescaped to `O'Brien` before constructing DynamoDB strings.

These no longer apply to the hand-rolled parser, which produces a typed `AttributeValue` straight from the lexer's escape-aware string token.

### Expression audit and stream capture

The 2026-04-01 audit found that `ConditionExpression` and `FilterExpression` support was already implemented in `expr.rs` and the DynamoDB handlers. The real stale artefact was the TODO comments in tests, not the code.

The same session completed DynamoDB Streams payload capture:

- `put_item`, `update_item`, and `delete_item` now append `StreamChangeRecord` entries when a table has streaming enabled
- records carry `event_name`, `sequence_number`, keys, old image, and new image
- iterator state is tracked by `next_sequence_number`, with `TRIM_HORIZON`, `LATEST`, `AT_SEQUENCE_NUMBER`, and `AFTER_SEQUENCE_NUMBER` semantics
- the derived DynamoDB Streams service still owns iterator bookkeeping, but the authoritative record payloads come from DynamoDB write paths

### Secondary Indexes, Transact Conditions, and SET Arithmetic

The 2026-04-24 DynamoDB sweep closed several behavioural gaps that sit below both SDK and Terraform compatibility:

- `GlobalSecondaryIndexDef` and `LocalSecondaryIndexDef` live in `types.rs`
- `Table` stores GSI/LSI definitions parsed at create-table time
- `DynamoDbBackend::create_table()` accepts secondary-index definitions
- `DynamoDbBackend::query()` accepts `index_name: Option<String>`
- `state::query()` validates the requested index, then scans and filters/sorts by the selected index key schema
- non-existent indexes return an error instead of silently querying the base table

The view layer was also made typed. `TableStateView` no longer stores secondary indexes as `Vec<serde_json::Value>` blobs. It uses `SecondaryIndexView` with index name, key schema, projection type, and non-key attributes. This lets JSON snapshots restore GSI/LSI definitions and lets Terraform converter injection build the same view shape directly.

`DescribeTable` now includes GSI metadata in the generated `TableDescription`, including name, active status, key schema, projection, and ARN. That matters because provider and SDK clients often rely on `DescribeTable` rather than only query success.

Transactional write conditions were extended across all sub-item kinds:

- `Put`
- `Update`
- `Delete`
- `ConditionCheck`

The handler parses each sub-item `ConditionExpression`, fetches the existing item through the backend, evaluates the condition, and returns `TransactionCanceledException` with per-item cancellation reasons when any condition fails.

Update-expression SET arithmetic now recognises `attr = attr + :val` and `attr = attr - :val`. Subtraction negates the delta and routes through the same add path, so the current item value is read at apply time. Expression-attribute-name aliases are covered by regression tests.

`winterbaume-dynamodb-redis` had to be updated with the same trait shape after this change: Redis `StoredTable` persists secondary indexes with serde defaults for backward compatibility, and Redis-backed queries accept the optional index name.

### Query Parser and Redis Parity Fixes ( 2026-04-30 / 2026-05-01 )

The Redis E2E sweep initially misclassified DynamoDB sort-key filtering as Redis-specific. The real root cause was in the shared handler parser: `extract_key_conditions` split on `AND` and recognised only equality, so `<`, `<=`, `>`, `>=`, `BETWEEN`, and `begins_with` were dropped before either backend saw the request.

The durable fix is to parse `KeyConditionExpression` through the existing expression parser, peel top-level `And` nodes, and classify each conjunct as either hash/sort-key equality in `key_conditions` or a `SortKeyCondition` range/prefix predicate. `SortKeyCondition` lives in `types.rs` and compares string, number, and binary values with DynamoDB-compatible ordering. Equality remains in the normal key-condition item path.

The same pass made `winterbaume-dynamodb-redis` mirror the in-memory query semantics:

- apply sort-key equality and optional range/prefix filters.
- sort partition results by the range key before applying `ScanIndexForward`.
- emit `LastEvaluatedKey` when `Limit` truncates a query result.
- honour `ExclusiveStartKey` on the next page.
- persist and describe LSIs, instead of silently dropping them.
- apply projection expressions on `GetItem` and `Query`.
- support nested-path `SET`, expression-attribute names, `list_append`, `if_not_exists`, and string-set `ADD` / `DELETE` update actions.

PartiQL had a separate predicate path. Its `pk = ? AND sk = ?` equality case was fixed by ensuring sort-key equality in `key_conditions` is applied by `state.rs`; the range and `begins_with` PartiQL cases already worked.

## Files

| File | Purpose |
|------|---------|
| `crates/winterbaume-partiql/Cargo.toml` | Crate manifest ( deps: `serde_json`, `thiserror` ) |
| `crates/winterbaume-partiql/src/lib.rs` | Public API: `parse_statement()`, `parse_statement_no_params()` |
| `crates/winterbaume-partiql/src/error.rs` | `PartiqlError` error type |
| `crates/winterbaume-partiql/src/operation.rs` | `DdbOperation`, `SelectOp`, `InsertOp`, `UpdateOp`, `DeleteOp`, `Expression`, `ArithOp`, `CmpOp`, `Condition`, `SetValue`, `ReturningClause` |
| `crates/winterbaume-partiql/src/param.rs` | Positional `?` parameter holder consumed at parse time |
| `crates/winterbaume-partiql/src/parser/lexer.rs` | Hand-rolled lexer ( tokens, keywords, `<<`/`>>`/`''`/`?`/string-escape handling ) |
| `crates/winterbaume-partiql/src/parser/expr.rs` | `parse_expression` ( precedence climb ), `parse_value`, primary atoms, paths, struct/list/bag literals |
| `crates/winterbaume-partiql/src/parser/stmt.rs` | `parse_select` / `parse_insert` / `parse_update` / `parse_delete` / `parse_exists`, RETURNING + ORDER BY + SET / REMOVE |
| `crates/winterbaume-partiql/src/parser/mod.rs` | `Parser` struct ( one-token lookahead queue ), `parse_top_level`, EXISTS detection |
| `crates/winterbaume-partiql/src/parser/tests.rs` | Unit tests: regression carry-over + lexer + AWS coverage + arithmetic + EXISTS |
| `crates/winterbaume-partiql/src/translate.rs` | **Orphaned**: not in module tree after parser rewrite; pending `git rm` |
| `crates/winterbaume-partiql/src/value.rs` | **Orphaned**: same |
| `crates/winterbaume-dynamodb/src/types.rs` | Typed `AttributeValue`, `Item`, and `StreamChangeRecord` |
| `crates/winterbaume-dynamodb/src/backend.rs` | Backend trait, including `get_stream_records` |
| `crates/winterbaume-dynamodb/src/partiql_exec.rs` | Backend-aware bridge from PartiQL operations to DynamoDB backend methods |
| `crates/winterbaume-dynamodb/src/views.rs` | Typed table and secondary-index view shapes |
| `crates/winterbaume-dynamodb-redis/src/lib.rs` | Redis backend persistence for tables and secondary-index definitions |
| `crates/winterbaume-dynamodbstreams/src/handlers.rs` | Shard iterator and stream-record serialization |

## Test Coverage

After all the 2026-04-28 / 04-29 work the `winterbaume-partiql` test module is **156+ tests** ( 129 parser unit + 11 lexer + 4 param + 12 integration in `winterbaume-dynamodb`, plus 11 `test_partiql_fn_*` tests added during the AWS-fidelity sweep, bringing the total touching PartiQL to 167 ). All green; clippy clean on both crates with `-D warnings`.

Coverage groups in the parser unit tests:

- 47 regression tests carried over verbatim from the old `translate.rs` ( SELECT / INSERT / UPDATE / DELETE shape coverage ).
- 11 lexer-only tests ( token boundaries, `''` escape, case-insensitive keywords, `?`, quoted-ident with hyphen ).
- 4 `param` tests ( positional substitution, count mismatch errors ).
- 24 `aws_*` and edge-case tests ( `<>` synonym, `<=`/`>=`/`<`, quoted attribute names, nested path indexing in projection and predicate, `IN [int, int]`, `BETWEEN int AND int`, all four `SET` literal RHS shapes, `list_append` / `set_add` / `set_delete` with nested-path targets, `REMOVE nested.path[index]`, `RETURNING ALL OLD *`, empty struct / list / bag, mixed-type bag rejection canonical message, negative numeric literal, float-with-exponent textual preservation ).
- 5 gap tests for explicitly-unsupported features ( `IS [NOT] MISSING`, `attribute_type`, `size(path)` were gap tests pre-rewrite — they now flip to positive coverage; `EXISTS` rejection tests, JOIN rejection, unknown-statement-keyword rejection, trailing-garbage rejection ).
- 7 `aws_*` arithmetic precedence tests ( path + path, chained, in-WHERE LHS / RHS, parenthesised, in IN list, in BETWEEN bounds ).
- 7 `test_partiql_fn_exists_*` parser tests covering basic EXISTS, parameter substitution, table-name propagation, case-insensitive `exists`, rejection of non-SELECT inside, missing-paren errors, top-level-only parse path.

Integration tests in `winterbaume-dynamodb`:

- 12 PartiQL integration tests cover arithmetic updates, list / set functions, `RETURNING`, `ORDER BY`, batch SELECT validation, GSI query routing, missing-index errors, GSI metadata in `DescribeTable`, secondary-index view serde round-trip, GSI snapshot / restore, transact-write condition expressions, `ConditionCheck`, SET arithmetic with expression-attribute-name aliases.
- 11 `test_partiql_fn_*` AWS-fidelity tests added during the 2026-04-29 sweep: `size_in_where`, `size_on_string_set`, `attribute_type_match`, `begins_with_string`, `contains_substring_and_set`, `is_missing_distinguishes_present_vs_absent`, `is_null_distinguishes_null_value_from_missing`, `exists_rejected_in_execute_statement`, `exists_in_transaction_with_condition_met`, `exists_in_transaction_with_condition_unmet_aborts`, `exists_rejects_pk_only_inner_select`.

`winterbaume-dynamodbstreams` and `winterbaume-dynamodb-redis` carry their own coverage for stream-record capture / iterator advancement and Redis-backed table persistence respectively.

The 2026-04-30 / 2026-05-01 Redis verification also exercised the fixed paths through AWS CLI calls against a live Redis-backed server: all seven sort-key operators, numeric sort-key comparison, query ordering, query pagination, PartiQL equality, LSI describe/query, projection expressions, and update-expression sub-cases were confirmed.

Run with:

```bash
cargo test -p winterbaume-partiql
cargo test -p winterbaume-dynamodb --test integration_test -- execute_statement
cargo test -p winterbaume-dynamodb --test integration_test -- partiql_fn
cargo test -p winterbaume-dynamodbstreams
cargo test -p winterbaume-dynamodb-redis
```

## Pitfalls

- `partiql-parser` v0.14.0 is marked experimental. The AST types may change in future versions. Pin the version carefully.
- First build of `partiql-parser` takes several minutes due to lalrpop grammar compilation. Subsequent builds are much faster.
- `Expr::Lit(StructLit(...))` vs `Expr::Struct(...)`: the parser produces struct literals as `Lit::StructLit`, not as `Expr::Struct`. Both paths must be handled.
- `Expr::Lit(ListLit(...))` and `Expr::Lit(BagLit(...))` are also distinct from `Expr::List` and `Expr::Bag`; `IN` handling must account for both forms.
- `ParserError` does not implement `Display`, only `Debug`. Use `{e:?}` not `{e}` in format strings.
- DynamoDB numbers are stored as strings (`{"N": "42"}`). The PartiQL literal `42` must become `AttributeValue::N("42".into())`, not an integer JSON node.
- SQL single-quote escapes may reach `lit_to_ddb_value(...)` still encoded as `''`. Do not assume the parser already decoded them.
- Index-qualified PartiQL SELECTs are only behaviourally correct through scan-plus-filter unless the backend grows real secondary-index storage.
- Backend trait changes must be propagated to custom backends such as `winterbaume-dynamodb-redis`; otherwise workspace check and clippy fail even if the in-memory backend passes.
- Do not split `KeyConditionExpression` on the string `AND`; `BETWEEN ... AND ...` makes that parser shape incorrect.
- Redis hash iteration order is arbitrary. Any DynamoDB operation that promises sort-key ordering must explicitly sort after loading candidates.
- `ProjectionExpression` is part of the read contract, not just a response cosmetic. Provider and SDK consumers rely on attributes being omitted.

### Post-rewrite ( hand-rolled parser ) pitfalls

These supersede the upstream-parser-specific bullets above for any new work touching `winterbaume-partiql`. The pre-rewrite bullets are kept as historical context for grepping old code references.

- The crate no longer depends on `partiql-parser` / `partiql-ast`. Do not re-introduce them — the lalrpop build script ( 30+ minutes per fresh `CARGO_TARGET_DIR` ) is what motivated the rewrite. Any new SQL surface should be added to `parser/lexer.rs` + `parser/expr.rs` + `parser/stmt.rs`.
- Whole-number arithmetic results are formatted as integers ( `5 + 5` → `"10"`, not `"10.0"` ). Numeric literals preserve their *textual* form throughout — the lexer does not parse them into `f64` until evaluation needs to.
- `MISSING` literal serialises identically to `NULL` ( `{"NULL": true}` ); the AWS docs treat them as distinct in WHERE predicates ( `IS NULL` vs `IS MISSING` ), but they share a wire shape.
- `EXISTS` is *not* a Condition. It is a top-level `DdbOperation` variant that only `handle_execute_transaction` accepts. Do not try to use it inside `WHERE` — AWS rejects "Unexpected path component", and the parser will not emit it from an expression position.
- `EXISTS` inner-SELECT must specify the full primary key by Eq AND at least one non-key predicate. Key-only EXISTS is rejected by AWS; `handle_execute_transaction` enforces the same constraint.
- MockAws keeps the parser permissive, but the DynamoDB handlers run a `winterbaume-partiql/src/validate.rs` walk before execution. `validate_condition` / `validate_where` reject `BinaryOp(Add|Sub)` and `Neg` operands in condition contexts with AWS-exact messages, including batch `EXISTS` returning `"EXISTS can only be used in ExecuteTransaction write requests."`.
- `RETURNING ALL NEW *` ( with the trailing `*` SQL "all columns" sigil ) is the AWS-canonical form. The parser also accepts the form without the `*` because the previous prefix-match implementation did and existing tests omit it. AWS itself rejects the no-`*` form.
- `WHERE` is REQUIRED in DELETE per the AWS grammar. The parser still accepts `DELETE FROM "T"` with no WHERE because `test_delete_no_where` codified that lenient behaviour. DynamoDB itself rejects keyless DELETE at runtime.
- `TableState.stream_records` and `stream_sequence_counter` are part of `TableStateView` with `#[serde(default)]`. Pre-2026-04-28 snapshots that pre-date the field deserialise cleanly with empty / zero defaults; new snapshots round-trip the values. Do not strip the `#[serde(default)]`.
- The lexer skips both `-- ... \n` line comments and `/* ... */` block comments before each token. `lex_skips_line_and_block_comments` verifies a SELECT with mixed comments tokenises identically to the comment-free form.
- `crates/winterbaume-partiql/src/translate.rs` and `value.rs` are no longer present in the module tree or on disk. Treat references to them in older notes as historical.
