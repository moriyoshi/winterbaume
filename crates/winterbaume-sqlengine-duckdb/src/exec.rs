//! Shared DuckDB execution helper used by both Athena and Redshift backends.

use duckdb::Connection;
use duckdb::types::ValueRef;

/// Column metadata and row data extracted from a DuckDB query.
pub(crate) struct RawQueryResult {
    /// `(name, type_str)` — type strings use lowercase DuckDB names such as
    /// `"varchar"`, `"integer"`, `"bigint"`, `"double"`, `"boolean"`.
    pub columns: Vec<(String, String)>,
    /// Row data; `None` cells represent SQL NULL.
    pub rows: Vec<Vec<Option<String>>>,
}

/// Execute `sql` on the given DuckDB connection and return columnar
/// results, or an error string suitable for storing as `error_message`.
pub(crate) fn execute_duckdb_sql(conn: &Connection, sql: &str) -> Result<RawQueryResult, String> {
    let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;

    // `column_count()` and `column_name()` panic before execution, so we
    // execute first via `query()` and then read metadata from the statement
    // reference available through `Rows::as_ref()`.
    let mut rows_iter = stmt.query([]).map_err(|e| e.to_string())?;

    let stmt_ref = rows_iter
        .as_ref()
        .ok_or_else(|| "no statement after query execution".to_string())?;

    let col_count = stmt_ref.column_count();
    let col_names: Vec<String> = stmt_ref.column_names();

    let mut rows: Vec<Vec<Option<String>>> = Vec::new();
    // Track the resolved type per column; filled from the first non-NULL value.
    let mut col_type_hints: Vec<Option<String>> = vec![None; col_count];

    while let Some(row) = rows_iter.next().map_err(|e| e.to_string())? {
        let mut row_vals: Vec<Option<String>> = Vec::with_capacity(col_count);
        for (i, type_hint) in col_type_hints.iter_mut().enumerate() {
            let val_ref = row.get_ref(i).map_err(|e| e.to_string())?;
            if type_hint.is_none() && !matches!(val_ref, ValueRef::Null) {
                *type_hint = Some(valueref_type_name(&val_ref).to_string());
            }
            row_vals.push(valueref_to_string(&val_ref));
        }
        rows.push(row_vals);
    }

    let columns: Vec<(String, String)> = col_names
        .into_iter()
        .zip(col_type_hints)
        .map(|(name, hint)| (name, hint.unwrap_or_else(|| "varchar".to_string())))
        .collect();

    Ok(RawQueryResult { columns, rows })
}

fn valueref_to_string(v: &ValueRef<'_>) -> Option<String> {
    match v {
        ValueRef::Null => None,
        ValueRef::Boolean(b) => Some(b.to_string()),
        ValueRef::TinyInt(n) => Some(n.to_string()),
        ValueRef::SmallInt(n) => Some(n.to_string()),
        ValueRef::Int(n) => Some(n.to_string()),
        ValueRef::BigInt(n) => Some(n.to_string()),
        ValueRef::HugeInt(n) => Some(n.to_string()),
        ValueRef::UTinyInt(n) => Some(n.to_string()),
        ValueRef::USmallInt(n) => Some(n.to_string()),
        ValueRef::UInt(n) => Some(n.to_string()),
        ValueRef::UBigInt(n) => Some(n.to_string()),
        ValueRef::Float(f) => Some(f.to_string()),
        ValueRef::Double(d) => Some(d.to_string()),
        ValueRef::Text(bytes) => Some(String::from_utf8_lossy(bytes).into_owned()),
        ValueRef::Blob(bytes) => Some(String::from_utf8_lossy(bytes).into_owned()),
        // For complex types, fall back to a debug representation.
        other => Some(format!("{other:?}")),
    }
}

fn valueref_type_name(v: &ValueRef<'_>) -> &'static str {
    match v {
        ValueRef::Null => "varchar",
        ValueRef::Boolean(_) => "boolean",
        ValueRef::TinyInt(_) | ValueRef::SmallInt(_) | ValueRef::Int(_) => "integer",
        ValueRef::BigInt(_) | ValueRef::HugeInt(_) => "bigint",
        ValueRef::UTinyInt(_)
        | ValueRef::USmallInt(_)
        | ValueRef::UInt(_)
        | ValueRef::UBigInt(_) => "bigint",
        ValueRef::Float(_) => "float",
        ValueRef::Double(_) => "double",
        ValueRef::Text(_) => "varchar",
        ValueRef::Blob(_) => "varbinary",
        _ => "varchar",
    }
}
