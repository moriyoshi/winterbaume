use serde_json::Value;
use winterbaume_partiql::operation::{ArithOp, CmpOp, Expression};
use winterbaume_partiql::{
    Condition, DdbOperation, DeleteOp, InsertOp, ReturningClause, SelectOp, SetValue, UpdateOp,
};

use crate::backend::DynamoDbBackend;
use crate::state::{DynamoDbError, DynamoDbState};
use crate::types::{AttributeValue, Item};

/// Convert a serde_json::Value (DynamoDB JSON wire format) to our typed AttributeValue.
fn json_to_attr(v: &Value) -> AttributeValue {
    serde_json::from_value(v.clone()).unwrap_or(AttributeValue::Null(true))
}

/// Convert a partiql Item (HashMap<String, Value>) to our typed Item (HashMap<String, AttributeValue>).
fn partiql_item_to_item(partiql_item: &winterbaume_partiql::operation::Item) -> Item {
    partiql_item
        .iter()
        .map(|(k, v)| (k.clone(), json_to_attr(v)))
        .collect()
}

/// Convert a Vec<(String, Value)> to Vec<(String, AttributeValue)>.
fn partiql_updates_to_av(updates: &[(String, Value)]) -> Vec<(String, AttributeValue)> {
    updates
        .iter()
        .map(|(k, v)| (k.clone(), json_to_attr(v)))
        .collect()
}

/// Negate a numeric AttributeValue (for SUB → negative ADD).
fn negate_number_av(v: &AttributeValue) -> AttributeValue {
    if let AttributeValue::N(s) = v {
        let n: f64 = s.parse().unwrap_or(0.0);
        let neg = if (-n).fract() == 0.0 && (-n).abs() < 1e15 {
            format!("{}", -n as i64)
        } else {
            format!("{}", -n)
        };
        AttributeValue::N(neg)
    } else {
        v.clone()
    }
}

/// Union two sets (SS/NS/BS).  Returns `new_elements` unchanged if types don't match.
fn set_union(current: Option<&AttributeValue>, new_elements: &AttributeValue) -> AttributeValue {
    match (current, new_elements) {
        (Some(AttributeValue::SS(existing)), AttributeValue::SS(to_add)) => {
            let mut set = existing.clone();
            for el in to_add {
                if !set.contains(el) {
                    set.push(el.clone());
                }
            }
            AttributeValue::SS(set)
        }
        (Some(AttributeValue::NS(existing)), AttributeValue::NS(to_add)) => {
            let mut set = existing.clone();
            for el in to_add {
                if !set.contains(el) {
                    set.push(el.clone());
                }
            }
            AttributeValue::NS(set)
        }
        (Some(AttributeValue::BS(existing)), AttributeValue::BS(to_add)) => {
            let mut set = existing.clone();
            for el in to_add {
                if !set.contains(el) {
                    set.push(el.clone());
                }
            }
            AttributeValue::BS(set)
        }
        _ => new_elements.clone(),
    }
}

/// Remove elements from a set (SS/NS/BS).
fn set_difference(
    current: Option<&AttributeValue>,
    remove_elements: &AttributeValue,
) -> AttributeValue {
    match (current, remove_elements) {
        (Some(AttributeValue::SS(existing)), AttributeValue::SS(to_remove)) => AttributeValue::SS(
            existing
                .iter()
                .filter(|el| !to_remove.contains(*el))
                .cloned()
                .collect(),
        ),
        (Some(AttributeValue::NS(existing)), AttributeValue::NS(to_remove)) => AttributeValue::NS(
            existing
                .iter()
                .filter(|el| !to_remove.contains(*el))
                .cloned()
                .collect(),
        ),
        (Some(AttributeValue::BS(existing)), AttributeValue::BS(to_remove)) => AttributeValue::BS(
            existing
                .iter()
                .filter(|el| !to_remove.contains(*el))
                .cloned()
                .collect(),
        ),
        (Some(v), _) => v.clone(),
        (None, _) => AttributeValue::SS(vec![]),
    }
}

/// Execute a parsed PartiQL operation against DynamoDB state.
pub fn execute_partiql(
    state: &mut DynamoDbState,
    op: &DdbOperation,
) -> Result<PartiqlResult, DynamoDbError> {
    match op {
        DdbOperation::Select(sel) => execute_select(state, sel),
        DdbOperation::Insert(ins) => execute_insert(state, ins),
        DdbOperation::Update(upd) => execute_update(state, upd),
        DdbOperation::Delete(del) => execute_delete(state, del),
        DdbOperation::Exists(sel) => execute_select(state, sel),
    }
}

pub enum PartiqlResult {
    // NOTE: This type is part of the public API via `DynamoDbBackend::execute_partiql`.
    /// Items returned from a SELECT or a write with RETURNING.
    Items(Vec<Item>),
    /// No items returned (write operation without RETURNING).
    Empty,
}

/// Execute a PartiQL operation by decomposing it into backend trait method
/// calls.  This is the default implementation that any [`DynamoDbBackend`] can
/// delegate to — it uses only `describe_table`, `query`, `scan`, `put_item`,
/// `update_item`, and `delete_item`.
///
/// Accepts `Arc<dyn DynamoDbBackend>` so the returned future is `'static` and
/// can be used inside `Pin<Box<dyn Future + Send>>` trait method bodies.
///
/// When `batch_mode` is true, SELECT statements that do not reference the
/// partition key are rejected with a ValidationException (DynamoDB restriction
/// on BatchExecuteStatement).
pub async fn execute_partiql_via_backend(
    backend: std::sync::Arc<dyn DynamoDbBackend>,
    account_id: String,
    region: String,
    op: DdbOperation,
    batch_mode: bool,
) -> Result<PartiqlResult, DynamoDbError> {
    match &op {
        DdbOperation::Select(sel) => {
            let table = backend
                .describe_table(account_id.clone(), region.clone(), sel.table_name.clone())
                .await?;

            let (key_conditions, filter_conditions) = if let Some(ref conds) = sel.where_clause {
                if sel.index_name.is_some() {
                    // Index query: skip base-table key extraction; fall through to full scan
                    // with all WHERE conditions applied as filters.
                    (Item::new(), conds.clone())
                } else {
                    split_key_conditions(
                        conds,
                        &table.hash_key_attr,
                        table.range_key_attr.as_deref(),
                    )
                }
            } else {
                (Item::new(), vec![])
            };

            // BatchExecuteStatement: SELECT must specify the partition key.
            if batch_mode && !key_conditions.contains_key(&table.hash_key_attr) {
                return Err(DynamoDbError::QueryConditionMissedKey);
            }

            let mut items = if key_conditions.contains_key(&table.hash_key_attr) {
                let scan_forward = sel.order_ascending.unwrap_or(true);
                let result = backend
                    .query(
                        account_id,
                        region,
                        sel.table_name.clone(),
                        key_conditions,
                        None,
                        None,
                        scan_forward,
                        None,
                        None,
                    )
                    .await?;
                result.items
            } else {
                let result = backend
                    .scan(account_id, region, sel.table_name.clone(), None, None)
                    .await?;
                // Apply ORDER BY if specified (scan results are unordered)
                let mut items = result.items;
                if let Some(ref order_attr) = sel.order_by_attr {
                    let ascending = sel.order_ascending.unwrap_or(true);
                    items.sort_by(|a, b| {
                        compare_av_opt(a.get(order_attr), b.get(order_attr))
                            .map(|o| if ascending { o } else { o.reverse() })
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
                items
            };

            // Apply filter conditions
            if !filter_conditions.is_empty() {
                items.retain(|item| {
                    filter_conditions
                        .iter()
                        .all(|c| evaluate_condition(c, item))
                });
            }

            // Apply projection
            let items = if let Some(ref proj) = sel.projection {
                items
                    .into_iter()
                    .map(|item| {
                        let mut projected = Item::new();
                        for attr in proj {
                            if let Some(val) = item.get(attr) {
                                projected.insert(attr.clone(), val.clone());
                            }
                        }
                        projected
                    })
                    .collect()
            } else {
                items
            };

            Ok(PartiqlResult::Items(items))
        }
        DdbOperation::Insert(ins) => {
            backend
                .put_item(
                    account_id,
                    region,
                    ins.table_name.clone(),
                    partiql_item_to_item(&ins.item),
                )
                .await?;
            Ok(PartiqlResult::Empty)
        }
        DdbOperation::Update(upd) => {
            let table = backend
                .describe_table(account_id.clone(), region.clone(), upd.table_name.clone())
                .await?;
            let key = extract_key_from_conditions(
                &upd.key_conditions,
                &table.hash_key_attr,
                table.range_key_attr.as_deref(),
            )?;

            // Determine whether we need the current item (for RETURNING ALL OLD or
            // for functional SET operations that depend on existing values).
            let needs_old_item = upd.returning == Some(ReturningClause::AllOld)
                || upd.returning == Some(ReturningClause::ModifiedOld)
                || upd.sets.iter().any(|(_, sv)| {
                    matches!(
                        sv,
                        SetValue::ListAppend(..) | SetValue::SetAdd(..) | SetValue::SetDelete(..)
                    )
                });

            let old_item: Option<Item> = if needs_old_item {
                backend
                    .get_item(
                        account_id.clone(),
                        region.clone(),
                        upd.table_name.clone(),
                        key.clone(),
                    )
                    .await?
            } else {
                None
            };

            let mut updates: Vec<(String, AttributeValue)> = Vec::new();
            let mut adds: Vec<(String, AttributeValue)> = Vec::new();
            let mut removes: Vec<String> = upd.removes.clone();

            for (attr, sv) in &upd.sets {
                match sv {
                    SetValue::Expr(expr) => {
                        // Recognise `target = target + literal` and
                        // `target = target - literal` as DynamoDB's atomic
                        // ADD action. Anything else is computed eagerly
                        // against `old_item` and emitted as a SET. If the
                        // expression doesn't produce a value (e.g. references
                        // a missing path), fall back to NULL — same shape as
                        // a regular SET against a missing source attribute.
                        if let Some(delta) = atomic_add_delta(expr, attr) {
                            adds.push((attr.clone(), delta));
                        } else if let Some(delta) = atomic_sub_delta(expr, attr) {
                            adds.push((attr.clone(), negate_number_av(&delta)));
                        } else {
                            let computed = evaluate_expression(expr, old_item.as_ref())
                                .unwrap_or(AttributeValue::Null(true));
                            updates.push((attr.clone(), computed));
                        }
                    }
                    SetValue::ListAppend(_, append_val) => {
                        // list_append(list1, list2) concatenates two lists.
                        let append_av = json_to_attr(append_val);
                        let to_append: Vec<AttributeValue> = match append_av {
                            AttributeValue::L(elems) => elems,
                            other => vec![other],
                        };
                        let new_list = if let Some(ref curr) = old_item {
                            if let Some(AttributeValue::L(mut list)) = curr.get(attr).cloned() {
                                list.extend(to_append);
                                AttributeValue::L(list)
                            } else {
                                AttributeValue::L(to_append)
                            }
                        } else {
                            AttributeValue::L(to_append)
                        };
                        updates.push((attr.clone(), new_list));
                    }
                    SetValue::SetAdd(_, add_vals) => {
                        let add_av = json_to_attr(add_vals);
                        let new_set =
                            set_union(old_item.as_ref().and_then(|i| i.get(attr)), &add_av);
                        updates.push((attr.clone(), new_set));
                    }
                    SetValue::SetDelete(_, del_vals) => {
                        let del_av = json_to_attr(del_vals);
                        let new_set =
                            set_difference(old_item.as_ref().and_then(|i| i.get(attr)), &del_av);
                        match &new_set {
                            AttributeValue::SS(s) if s.is_empty() => removes.push(attr.clone()),
                            AttributeValue::NS(s) if s.is_empty() => removes.push(attr.clone()),
                            AttributeValue::BS(s) if s.is_empty() => removes.push(attr.clone()),
                            _ => updates.push((attr.clone(), new_set)),
                        }
                    }
                }
            }

            let mut actions: Vec<crate::types::UpdateAction> = Vec::new();
            for (attr, value) in updates {
                actions.push(crate::types::UpdateAction::SetValue {
                    path: vec![attr],
                    value,
                });
            }
            for attr in removes {
                actions.push(crate::types::UpdateAction::Remove(vec![attr]));
            }
            for (attr, delta) in adds {
                actions.push(crate::types::UpdateAction::Add(attr, delta));
            }

            let new_item = backend
                .update_item(account_id, region, upd.table_name.clone(), key, actions)
                .await?;

            match &upd.returning {
                None => Ok(PartiqlResult::Empty),
                Some(ReturningClause::AllNew) | Some(ReturningClause::ModifiedNew) => {
                    Ok(PartiqlResult::Items(new_item.into_iter().collect()))
                }
                Some(ReturningClause::AllOld) | Some(ReturningClause::ModifiedOld) => {
                    Ok(PartiqlResult::Items(old_item.into_iter().collect()))
                }
            }
        }
        DdbOperation::Delete(del) => {
            let table = backend
                .describe_table(account_id.clone(), region.clone(), del.table_name.clone())
                .await?;
            let key = extract_key_from_conditions(
                &del.key_conditions,
                &table.hash_key_attr,
                table.range_key_attr.as_deref(),
            )?;

            // Fetch old item before deletion if RETURNING ALL OLD is requested.
            let old_item: Option<Item> = if matches!(
                &del.returning,
                Some(ReturningClause::AllOld) | Some(ReturningClause::ModifiedOld)
            ) {
                backend
                    .get_item(
                        account_id.clone(),
                        region.clone(),
                        del.table_name.clone(),
                        key.clone(),
                    )
                    .await?
            } else {
                None
            };

            backend
                .delete_item(account_id, region, del.table_name.clone(), key)
                .await?;

            match &del.returning {
                None => Ok(PartiqlResult::Empty),
                Some(ReturningClause::AllOld) | Some(ReturningClause::ModifiedOld) => {
                    Ok(PartiqlResult::Items(old_item.into_iter().collect()))
                }
                Some(ReturningClause::AllNew) | Some(ReturningClause::ModifiedNew) => {
                    // Item no longer exists after delete.
                    Ok(PartiqlResult::Items(vec![]))
                }
            }
        }
        DdbOperation::Exists(sel) => {
            // EXISTS runs the inner SELECT. The transaction handler
            // inspects the returned `Items` for truthiness (non-empty ==
            // TRUE). AWS-specific validation of the inner SELECT (full PK
            // + ≥1 non-key predicate) is enforced by the handler before
            // dispatch, so we just delegate to the SELECT logic here.
            let select_op = DdbOperation::Select(SelectOp {
                table_name: sel.table_name.clone(),
                index_name: sel.index_name.clone(),
                projection: sel.projection.clone(),
                where_clause: sel.where_clause.clone(),
                order_ascending: sel.order_ascending,
                order_by_attr: sel.order_by_attr.clone(),
            });
            // Recursively invoke the dispatcher so the existing SELECT
            // arm handles backend lookups + post-scan filtering. We have
            // to re-box the future because Rust can't directly recurse
            // into an async fn.
            Box::pin(execute_partiql_via_backend(
                backend, account_id, region, select_op, batch_mode,
            ))
            .await
        }
    }
}

fn execute_select(state: &DynamoDbState, op: &SelectOp) -> Result<PartiqlResult, DynamoDbError> {
    let table_name = &op.table_name;

    // Determine if we can use query (partition key specified) or must scan.
    let ts = state
        .tables
        .get(table_name)
        .ok_or_else(|| resource_not_found(table_name))?;

    let hash_key_attr = &ts.table.hash_key_attr;

    // Try to extract key conditions from the WHERE clause.
    let (key_conditions, filter_conditions) = if let Some(ref conds) = op.where_clause {
        if op.index_name.is_some() {
            (Item::new(), conds.clone())
        } else {
            split_key_conditions(conds, hash_key_attr, ts.table.range_key_attr.as_deref())
        }
    } else {
        (Item::new(), vec![])
    };

    let mut items = if key_conditions.contains_key(hash_key_attr) {
        // Use query
        let scan_forward = op.order_ascending.unwrap_or(true);
        let result = state.query(
            table_name,
            &key_conditions,
            None,
            None,
            scan_forward,
            None,
            None,
        )?;
        result.items
    } else {
        // Use scan
        let result = state.scan(table_name, None, None)?;
        let mut items = result.items;
        if let Some(ref order_attr) = op.order_by_attr {
            let ascending = op.order_ascending.unwrap_or(true);
            items.sort_by(|a, b| {
                compare_av_opt(a.get(order_attr), b.get(order_attr))
                    .map(|o| if ascending { o } else { o.reverse() })
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        items
    };

    // Apply filter conditions
    if !filter_conditions.is_empty() {
        items.retain(|item| {
            filter_conditions
                .iter()
                .all(|c| evaluate_condition(c, item))
        });
    }

    // Apply projection
    let items = if let Some(ref proj) = op.projection {
        items
            .into_iter()
            .map(|item| {
                let mut projected = Item::new();
                for attr in proj {
                    if let Some(val) = item.get(attr) {
                        projected.insert(attr.clone(), val.clone());
                    }
                }
                projected
            })
            .collect()
    } else {
        items
    };

    Ok(PartiqlResult::Items(items))
}

fn execute_insert(
    state: &mut DynamoDbState,
    op: &InsertOp,
) -> Result<PartiqlResult, DynamoDbError> {
    state.put_item(&op.table_name, partiql_item_to_item(&op.item))?;
    Ok(PartiqlResult::Empty)
}

fn execute_update(
    state: &mut DynamoDbState,
    op: &UpdateOp,
) -> Result<PartiqlResult, DynamoDbError> {
    let table_name = &op.table_name;
    let ts = state
        .tables
        .get(table_name)
        .ok_or_else(|| resource_not_found(table_name))?;

    // Extract key from conditions
    let key = extract_key_from_conditions(
        &op.key_conditions,
        &ts.table.hash_key_attr,
        ts.table.range_key_attr.as_deref(),
    )?;

    // Convert literal SetValue::Expr items to (String, AttributeValue)
    // tuples for state.update_item. Path/arithmetic expressions in this
    // direct-state path aren't pre-evaluated; the backend-trait path
    // (execute_partiql_via_backend) handles them more thoroughly.
    let mut actions: Vec<crate::types::UpdateAction> = Vec::new();
    for (attr, sv) in &op.sets {
        if let SetValue::Expr(expr) = sv
            && let Some(v) = expr.as_literal()
        {
            actions.push(crate::types::UpdateAction::SetValue {
                path: vec![attr.clone()],
                value: json_to_attr(v),
            });
        }
    }
    for attr in &op.removes {
        actions.push(crate::types::UpdateAction::Remove(vec![attr.clone()]));
    }

    state.update_item(table_name, &key, &actions)?;
    Ok(PartiqlResult::Empty)
}

fn execute_delete(
    state: &mut DynamoDbState,
    op: &DeleteOp,
) -> Result<PartiqlResult, DynamoDbError> {
    let table_name = &op.table_name;
    let ts = state
        .tables
        .get(table_name)
        .ok_or_else(|| resource_not_found(table_name))?;

    let key = extract_key_from_conditions(
        &op.key_conditions,
        &ts.table.hash_key_attr,
        ts.table.range_key_attr.as_deref(),
    )?;

    state.delete_item(table_name, &key)?;
    Ok(PartiqlResult::Empty)
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn resource_not_found(table_name: &str) -> DynamoDbError {
    DynamoDbError::ResourceNotFound(table_name.to_string())
}

/// Split conditions into key conditions (equality on hash/range key) and filter conditions.
///
/// A condition contributes to the key item only if it has the shape
/// `Compare(Path(key_attr), CmpOp::Eq, Literal(val))`. Arithmetic on either
/// side, path-on-RHS comparisons, or non-equality operators all fall through
/// to the filter list.
fn split_key_conditions(
    conditions: &[Condition],
    hash_key: &str,
    range_key: Option<&str>,
) -> (Item, Vec<Condition>) {
    let mut key_item = Item::new();
    let mut filters = Vec::new();

    for cond in conditions {
        if let Some((attr, val)) = key_eq_pair(cond)
            && (attr == hash_key || range_key == Some(attr))
        {
            key_item.insert(attr.to_string(), json_to_attr(val));
        } else {
            filters.push(cond.clone());
        }
    }

    (key_item, filters)
}

/// Extract key item from conditions.
fn extract_key_from_conditions(
    conditions: &[Condition],
    hash_key: &str,
    range_key: Option<&str>,
) -> Result<Item, DynamoDbError> {
    let mut key = Item::new();
    for cond in conditions {
        if let Some((attr, val)) = key_eq_pair(cond)
            && (attr == hash_key || range_key == Some(attr))
        {
            key.insert(attr.to_string(), json_to_attr(val));
        }
    }
    if !key.contains_key(hash_key) {
        return Err(DynamoDbError::QueryConditionMissedKey);
    }
    Ok(key)
}

/// If `cond` is `Compare(Path(p), Eq, Literal(v))`, return `(p, &v)`. This
/// is the only condition shape that can contribute to a DynamoDB primary
/// key match.
fn key_eq_pair(cond: &Condition) -> Option<(&str, &Value)> {
    match cond {
        Condition::Compare(lhs, CmpOp::Eq, rhs) => {
            let attr = lhs.as_path()?;
            let val = rhs.as_literal()?;
            Some((attr, val))
        }
        _ => None,
    }
}

/// Compare AttributeValue with a partiql Value (JSON wire format).
fn compare_av_with_json(a: &AttributeValue, b: &Value) -> Option<std::cmp::Ordering> {
    let b_av = json_to_attr(b);
    compare_av(a, &b_av)
}

/// Compare two AttributeValues.
fn compare_av(a: &AttributeValue, b: &AttributeValue) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (AttributeValue::S(sa), AttributeValue::S(sb)) => Some(sa.cmp(sb)),
        (AttributeValue::N(na), AttributeValue::N(nb)) => {
            let fa: f64 = na.parse().ok()?;
            let fb: f64 = nb.parse().ok()?;
            fa.partial_cmp(&fb)
        }
        _ => None,
    }
}

/// Compare two optional AttributeValues (None sorts before Some).
fn compare_av_opt(
    a: Option<&AttributeValue>,
    b: Option<&AttributeValue>,
) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (Some(av), Some(bv)) => compare_av(av, bv),
        (Some(_), None) => Some(std::cmp::Ordering::Greater),
        (None, Some(_)) => Some(std::cmp::Ordering::Less),
        (None, None) => Some(std::cmp::Ordering::Equal),
    }
}

/// Evaluate a condition against an item (for post-scan filtering).
fn evaluate_condition(cond: &Condition, item: &Item) -> bool {
    match cond {
        Condition::Compare(lhs, op, rhs) => {
            let l = evaluate_expression(lhs, Some(item));
            let r = evaluate_expression(rhs, Some(item));
            match (l, r) {
                (Some(la), Some(ra)) => apply_cmp_op(&la, &ra, *op),
                _ => false,
            }
        }
        Condition::And(l, r) => evaluate_condition(l, item) && evaluate_condition(r, item),
        Condition::Or(l, r) => evaluate_condition(l, item) || evaluate_condition(r, item),
        Condition::Not(inner) => !evaluate_condition(inner, item),
        Condition::BeginsWith(attr, val_expr) => {
            let val_av = match evaluate_expression(val_expr, Some(item)) {
                Some(v) => v,
                None => return false,
            };
            if let (Some(AttributeValue::S(item_s)), AttributeValue::S(prefix)) =
                (item.get(attr), &val_av)
            {
                item_s.starts_with(prefix.as_str())
            } else {
                false
            }
        }
        Condition::Contains(attr, val_expr) => {
            let val_av = match evaluate_expression(val_expr, Some(item)) {
                Some(v) => v,
                None => return false,
            };
            // AWS `contains(path, val)` semantics:
            // - If `path` is a String, `val` is a String → substring check.
            // - If `path` is SS/NS/BS, `val` is the matching scalar →
            //   element-membership check.
            // - If `path` is L → element-equality membership.
            match (item.get(attr), &val_av) {
                (Some(AttributeValue::S(s)), AttributeValue::S(substr)) => s.contains(substr),
                (Some(AttributeValue::SS(set)), AttributeValue::S(elem)) => set.contains(elem),
                (Some(AttributeValue::NS(set)), AttributeValue::N(elem)) => set.contains(elem),
                (Some(AttributeValue::BS(set)), AttributeValue::B(elem)) => set.contains(elem),
                (Some(AttributeValue::L(list)), v) => list.iter().any(|el| el == v),
                _ => false,
            }
        }
        Condition::Between(value_expr, lo_expr, hi_expr) => {
            let v = evaluate_expression(value_expr, Some(item));
            let lo = evaluate_expression(lo_expr, Some(item));
            let hi = evaluate_expression(hi_expr, Some(item));
            match (v, lo, hi) {
                (Some(av), Some(lo), Some(hi)) => {
                    let ge_lo = compare_av(&av, &lo)
                        .map(|o| o != std::cmp::Ordering::Less)
                        .unwrap_or(false);
                    let le_hi = compare_av(&av, &hi)
                        .map(|o| o != std::cmp::Ordering::Greater)
                        .unwrap_or(false);
                    ge_lo && le_hi
                }
                _ => false,
            }
        }
        Condition::In(value_expr, list_exprs) => {
            let v = match evaluate_expression(value_expr, Some(item)) {
                Some(v) => v,
                None => return false,
            };
            list_exprs.iter().any(|e| {
                evaluate_expression(e, Some(item))
                    .as_ref()
                    .is_some_and(|av| av == &v)
            })
        }
        Condition::IsMissing(attr) => !item.contains_key(attr),
        Condition::IsNull(attr) => matches!(item.get(attr), Some(AttributeValue::Null(_))),
        Condition::AttributeType(attr, type_name) => match item.get(attr) {
            Some(av) => attribute_value_type_name(av).eq_ignore_ascii_case(type_name),
            None => false,
        },
        Condition::Size(attr, op, val_expr) => {
            let actual = match item.get(attr).and_then(attribute_value_size) {
                Some(n) => n,
                None => return false,
            };
            let target_av = match evaluate_expression(val_expr, Some(item)) {
                Some(av) => av,
                None => return false,
            };
            let target: Option<i64> = match &target_av {
                AttributeValue::N(s) => s.parse().ok(),
                _ => None,
            };
            match target {
                Some(t) => match op {
                    CmpOp::Eq => actual == t,
                    CmpOp::Ne => actual != t,
                    CmpOp::Lt => actual < t,
                    CmpOp::Le => actual <= t,
                    CmpOp::Gt => actual > t,
                    CmpOp::Ge => actual >= t,
                },
                None => false,
            }
        }
    }
}

/// Evaluate an [`Expression`] against the (optional) current item. Returns
/// `None` when the result is undefined — e.g. a path reference where the
/// attribute is missing, or arithmetic on non-numeric operands.
fn evaluate_expression(expr: &Expression, item: Option<&Item>) -> Option<AttributeValue> {
    match expr {
        Expression::Literal(v) => Some(json_to_attr(v)),
        Expression::Path(p) => item.and_then(|it| it.get(p).cloned()),
        Expression::Neg(inner) => {
            let v = evaluate_expression(inner, item)?;
            Some(negate_number_av(&v))
        }
        Expression::BinaryOp(l, op, r) => {
            let lv = evaluate_expression(l, item)?;
            let rv = evaluate_expression(r, item)?;
            apply_arith_op(&lv, *op, &rv)
        }
    }
}

/// Apply a numeric arithmetic op to two AttributeValues. Returns None if
/// either side isn't a number.
fn apply_arith_op(l: &AttributeValue, op: ArithOp, r: &AttributeValue) -> Option<AttributeValue> {
    let ls = match l {
        AttributeValue::N(s) => s,
        _ => return None,
    };
    let rs = match r {
        AttributeValue::N(s) => s,
        _ => return None,
    };
    let lf: f64 = ls.parse().ok()?;
    let rf: f64 = rs.parse().ok()?;
    let out = match op {
        ArithOp::Add => lf + rf,
        ArithOp::Sub => lf - rf,
    };
    let s = if out.fract() == 0.0 && out.abs() < 1e15 {
        format!("{}", out as i64)
    } else {
        format!("{out}")
    };
    Some(AttributeValue::N(s))
}

/// Apply a comparison op to two AttributeValues.
fn apply_cmp_op(l: &AttributeValue, r: &AttributeValue, op: CmpOp) -> bool {
    match op {
        CmpOp::Eq => l == r,
        CmpOp::Ne => l != r,
        CmpOp::Lt => compare_av(l, r) == Some(std::cmp::Ordering::Less),
        CmpOp::Le => matches!(
            compare_av(l, r),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        ),
        CmpOp::Gt => compare_av(l, r) == Some(std::cmp::Ordering::Greater),
        CmpOp::Ge => matches!(
            compare_av(l, r),
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        ),
    }
}

/// Detect `target = target + literal` for atomic ADD. Returns the literal
/// AttributeValue (a number) when the pattern matches.
fn atomic_add_delta(expr: &Expression, target_attr: &str) -> Option<AttributeValue> {
    if let Expression::BinaryOp(l, ArithOp::Add, r) = expr
        && let Some(p) = l.as_path()
        && p == target_attr
        && let Some(v) = r.as_literal()
    {
        let av = json_to_attr(v);
        if matches!(av, AttributeValue::N(_)) {
            return Some(av);
        }
    }
    None
}

/// Detect `target = target - literal` for atomic ADD with negated delta.
fn atomic_sub_delta(expr: &Expression, target_attr: &str) -> Option<AttributeValue> {
    if let Expression::BinaryOp(l, ArithOp::Sub, r) = expr
        && let Some(p) = l.as_path()
        && p == target_attr
        && let Some(v) = r.as_literal()
    {
        let av = json_to_attr(v);
        if matches!(av, AttributeValue::N(_)) {
            return Some(av);
        }
    }
    None
}

/// AWS-canonical type name for an AttributeValue. Used by `attribute_type`.
fn attribute_value_type_name(av: &AttributeValue) -> &'static str {
    match av {
        AttributeValue::S(_) => "S",
        AttributeValue::N(_) => "N",
        AttributeValue::B(_) => "B",
        AttributeValue::Bool(_) => "BOOL",
        AttributeValue::Null(_) => "NULL",
        AttributeValue::L(_) => "L",
        AttributeValue::M(_) => "M",
        AttributeValue::SS(_) => "SS",
        AttributeValue::NS(_) => "NS",
        AttributeValue::BS(_) => "BS",
    }
}

/// Best-effort `size()` implementation. AWS docs define sizes per type:
///
/// - S: number of UTF-8 bytes
/// - B: number of bytes
/// - L / SS / NS / BS: number of elements
/// - M: number of name-value pairs
///
/// Other types return None (size is undefined).
fn attribute_value_size(av: &AttributeValue) -> Option<i64> {
    match av {
        AttributeValue::S(s) => Some(s.len() as i64),
        AttributeValue::B(b) => Some(b.len() as i64),
        AttributeValue::L(l) => Some(l.len() as i64),
        AttributeValue::SS(s) => Some(s.len() as i64),
        AttributeValue::NS(s) => Some(s.len() as i64),
        AttributeValue::BS(s) => Some(s.len() as i64),
        AttributeValue::M(m) => Some(m.len() as i64),
        _ => None,
    }
}
