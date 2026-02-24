//! Regression tests carried over from the previous `translate.rs`,
//! plus AWS-spec coverage tests, plus precedence/arithmetic pinning tests.
//!
//! After the IR refactor that introduced [`Expression`], every value-position
//! assertion goes through one of:
//!  - `Expression::as_path()`/`as_literal()` accessors, or
//!  - direct equality against `Expression::Literal(json!({...}))` /
//!    `Expression::Path("...")` constructors.
//!
//! The variant rename `Condition::{Eq,Ne,…}` → `Condition::Compare(_, CmpOp, _)`
//! is the other big shape change to be aware of.

use serde_json::{Value, json};

use super::{parse_statement, parse_statement_no_params};
use crate::operation::{
    ArithOp, CmpOp, Condition, DdbOperation, Expression, ReturningClause, SetValue,
};

// ─────────────────────────────────────────────────────────────────────────────
//  Tiny helpers — keep test bodies focused on intent, not destructuring.
// ─────────────────────────────────────────────────────────────────────────────

/// `Compare(Path(attr), Eq, Literal(val))` shorthand check.
fn assert_eq_path_lit(c: &Condition, expected_attr: &str, expected: Value) {
    match c {
        Condition::Compare(lhs, CmpOp::Eq, rhs) => {
            assert_eq!(
                lhs.as_path(),
                Some(expected_attr),
                "LHS path mismatch in {c:?}"
            );
            assert_eq!(
                rhs.as_literal(),
                Some(&expected),
                "RHS lit mismatch in {c:?}"
            );
        }
        other => panic!("expected Compare(_, Eq, _), got {other:?}"),
    }
}

/// `c` is `Compare(Path(attr), op, Literal(val))`.
fn matches_path_op_lit(c: &Condition, attr: &str, op: CmpOp, val: &Value) -> bool {
    matches!(c, Condition::Compare(lhs, o, rhs)
        if *o == op
            && lhs.as_path() == Some(attr)
            && rhs.as_literal() == Some(val))
}

#[test]
fn test_select_star() {
    let op = parse_statement_no_params("SELECT * FROM \"Music\"").unwrap();
    match op {
        DdbOperation::Select(s) => {
            assert_eq!(s.table_name, "Music");
            assert!(s.projection.is_none());
            assert!(s.where_clause.is_none());
            assert!(s.index_name.is_none());
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_with_where() {
    let op = parse_statement_no_params(
        "SELECT * FROM \"Music\" WHERE Artist = 'Acme Band' AND SongTitle = 'PartiQL Rocks'",
    )
    .unwrap();
    match op {
        DdbOperation::Select(s) => {
            assert_eq!(s.table_name, "Music");
            let conds = s.where_clause.unwrap();
            assert_eq!(conds.len(), 2);
            assert_eq_path_lit(&conds[0], "Artist", json!({"S": "Acme Band"}));
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_with_projection() {
    let op = parse_statement_no_params("SELECT Artist, SongTitle FROM \"Music\"").unwrap();
    match op {
        DdbOperation::Select(s) => {
            assert_eq!(s.projection.unwrap(), vec!["Artist", "SongTitle"]);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_with_order_by() {
    let op = parse_statement_no_params("SELECT * FROM \"Music\" WHERE pk = 'x' ORDER BY sk DESC")
        .unwrap();
    match op {
        DdbOperation::Select(s) => {
            assert_eq!(s.order_ascending, Some(false));
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_insert() {
    let op = parse_statement_no_params(
        "INSERT INTO \"Music\" VALUE {'Artist': 'Acme Band', 'SongTitle': 'PartiQL Rocks', 'Year': 2020}",
    )
    .unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            assert_eq!(ins.table_name, "Music");
            assert_eq!(ins.item.get("Artist").unwrap(), &json!({"S": "Acme Band"}));
            assert_eq!(ins.item.get("Year").unwrap(), &json!({"N": "2020"}));
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_delete() {
    let op = parse_statement_no_params(
        "DELETE FROM \"Music\" WHERE Artist = 'Acme Band' AND SongTitle = 'PartiQL Rocks'",
    )
    .unwrap();
    match op {
        DdbOperation::Delete(del) => {
            assert_eq!(del.table_name, "Music");
            assert_eq!(del.key_conditions.len(), 2);
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn test_update_set() {
    let op = parse_statement_no_params(
        "UPDATE \"Music\" SET AwardsWon = 1 WHERE Artist = 'Acme Band' AND SongTitle = 'PartiQL Rocks'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.table_name, "Music");
            assert_eq!(upd.sets.len(), 1);
            assert_eq!(upd.sets[0].0, "AwardsWon");
            match &upd.sets[0].1 {
                SetValue::Expr(e) => assert_eq!(e.as_literal(), Some(&json!({"N": "1"}))),
                _ => panic!("expected Expr"),
            }
            assert_eq!(upd.key_conditions.len(), 2);
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_remove() {
    let op =
        parse_statement_no_params("UPDATE \"Music\" REMOVE OldAttr WHERE Artist = 'Acme Band'")
            .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.removes, vec!["OldAttr"]);
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_parameter_binding() {
    let op = parse_statement(
        "SELECT * FROM \"Music\" WHERE Artist = ?",
        &[json!({"S": "Acme Band"})],
    )
    .unwrap();
    match op {
        DdbOperation::Select(s) => {
            let conds = s.where_clause.unwrap();
            assert_eq_path_lit(&conds[0], "Artist", json!({"S": "Acme Band"}));
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_insert_with_params() {
    let op = parse_statement(
        "INSERT INTO \"Music\" VALUE {'Artist': ?, 'SongTitle': ?}",
        &[json!({"S": "Band"}), json!({"S": "Song"})],
    )
    .unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            assert_eq!(ins.item.get("Artist").unwrap(), &json!({"S": "Band"}));
            assert_eq!(ins.item.get("SongTitle").unwrap(), &json!({"S": "Song"}));
        }
        _ => panic!("expected Insert"),
    }
}

// ── INSERT edge cases ────────────────────────────────────────────────────

#[test]
fn test_insert_unquoted_table_name() {
    let op = parse_statement_no_params("INSERT INTO Music VALUE {'pk': 'x', 'sk': 'y'}").unwrap();
    match op {
        DdbOperation::Insert(ins) => assert_eq!(ins.table_name, "Music"),
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_insert_hyphenated_table_name() {
    let op = parse_statement_no_params("INSERT INTO \"My-Table\" VALUE {'pk': 'x'}").unwrap();
    match op {
        DdbOperation::Insert(ins) => assert_eq!(ins.table_name, "My-Table"),
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_insert_nested_map() {
    let op = parse_statement_no_params(
        "INSERT INTO \"T\" VALUE {'pk': 'x', 'Meta': {'colour': 'blue', 'count': 3}}",
    )
    .unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            let meta = ins.item.get("Meta").unwrap();
            assert_eq!(
                meta.get("M").unwrap().get("colour").unwrap(),
                &json!({"S": "blue"})
            );
            assert_eq!(
                meta.get("M").unwrap().get("count").unwrap(),
                &json!({"N": "3"})
            );
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_insert_list_value() {
    let op =
        parse_statement_no_params("INSERT INTO \"T\" VALUE {'pk': 'x', 'Tags': ['alpha', 'beta']}")
            .unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            let tags = ins.item.get("Tags").unwrap();
            let list = tags.get("L").expect("expected L type");
            assert_eq!(list[0], json!({"S": "alpha"}));
            assert_eq!(list[1], json!({"S": "beta"}));
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_insert_boolean_and_null() {
    let op = parse_statement_no_params(
        "INSERT INTO \"T\" VALUE {'pk': 'x', 'Active': true, 'Removed': null}",
    )
    .unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            assert_eq!(ins.item.get("Active").unwrap(), &json!({"BOOL": true}));
            assert_eq!(ins.item.get("Removed").unwrap(), &json!({"NULL": true}));
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_insert_float_value() {
    let op =
        parse_statement_no_params("INSERT INTO \"T\" VALUE {'pk': 'x', 'Score': 3.14}").unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            let score = ins.item.get("Score").unwrap();
            assert_eq!(score.get("N").unwrap().as_str().unwrap(), "3.14");
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_insert_string_with_embedded_single_quote() {
    let op = parse_statement_no_params("INSERT INTO \"T\" VALUE {'pk': 'x', 'Name': 'O''Brien'}")
        .unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            assert_eq!(ins.item.get("Name").unwrap(), &json!({"S": "O'Brien"}));
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_insert_string_set() {
    let op = parse_statement_no_params(
        "INSERT INTO \"T\" VALUE {'pk': 'x', 'Roles': <<'admin', 'user'>>}",
    )
    .unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            let roles = ins.item.get("Roles").unwrap();
            let ss = roles.get("SS").expect("expected SS type");
            let mut vals: Vec<&str> = ss
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            vals.sort_unstable();
            assert_eq!(vals, vec!["admin", "user"]);
        }
        _ => panic!("expected Insert"),
    }
}

// ── UPDATE edge cases ────────────────────────────────────────────────────

#[test]
fn test_update_multiple_set_clauses() {
    let op = parse_statement_no_params(
        "UPDATE \"Music\" SET AwardsWon = 3 SET Genre = 'Rock' WHERE Artist = 'Band'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.sets.len(), 2);
            let set_map: std::collections::HashMap<_, _> =
                upd.sets.iter().map(|(k, v)| (k.as_str(), v)).collect();
            match set_map["AwardsWon"] {
                SetValue::Expr(e) => assert_eq!(e.as_literal(), Some(&json!({"N": "3"}))),
                _ => panic!("expected Expr for AwardsWon"),
            }
            match set_map["Genre"] {
                SetValue::Expr(e) => assert_eq!(e.as_literal(), Some(&json!({"S": "Rock"}))),
                _ => panic!("expected Expr for Genre"),
            }
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_arithmetic_add() {
    let op =
        parse_statement_no_params("UPDATE \"T\" SET counter = counter + 1 WHERE pk = 'x'").unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.sets.len(), 1);
            match &upd.sets[0].1 {
                SetValue::Expr(Expression::BinaryOp(l, ArithOp::Add, r)) => {
                    assert_eq!(l.as_path(), Some("counter"));
                    assert_eq!(r.as_literal(), Some(&json!({"N": "1"})));
                }
                other => panic!("expected Expr(BinaryOp Add), got {other:?}"),
            }
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_arithmetic_sub() {
    let op =
        parse_statement_no_params("UPDATE \"T\" SET score = score - 5 WHERE pk = 'x'").unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(Expression::BinaryOp(l, ArithOp::Sub, r)) => {
                assert_eq!(l.as_path(), Some("score"));
                assert_eq!(r.as_literal(), Some(&json!({"N": "5"})));
            }
            other => panic!("expected Expr(BinaryOp Sub), got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_list_append() {
    let op = parse_statement_no_params(
        "UPDATE \"T\" SET Tags = list_append(Tags, ['new_tag']) WHERE pk = 'x'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::ListAppend(attr, val) => {
                assert_eq!(attr, "Tags");
                let list = val.get("L").expect("expected L type");
                assert_eq!(list[0], json!({"S": "new_tag"}));
            }
            _ => panic!("expected ListAppend"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_set_add() {
    let op = parse_statement_no_params(
        "UPDATE \"T\" SET Roles = set_add(Roles, <<'admin'>>) WHERE pk = 'x'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::SetAdd(attr, val) => {
                assert_eq!(attr, "Roles");
                let ss = val.get("SS").expect("expected SS type");
                assert_eq!(ss[0].as_str().unwrap(), "admin");
            }
            _ => panic!("expected SetAdd"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_set_delete() {
    let op = parse_statement_no_params(
        "UPDATE \"T\" SET Roles = set_delete(Roles, <<'guest'>>) WHERE pk = 'x'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::SetDelete(attr, val) => {
                assert_eq!(attr, "Roles");
                let ss = val.get("SS").expect("expected SS type");
                assert_eq!(ss[0].as_str().unwrap(), "guest");
            }
            _ => panic!("expected SetDelete"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_remove_nested_path() {
    let op = parse_statement_no_params("UPDATE \"T\" REMOVE Meta.version WHERE pk = 'x'").unwrap();
    match op {
        DdbOperation::Update(upd) => assert_eq!(upd.removes, vec!["Meta.version"]),
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_remove_list_element() {
    let op = parse_statement_no_params("UPDATE \"T\" REMOVE Tags[0] WHERE pk = 'x'").unwrap();
    match op {
        DdbOperation::Update(upd) => assert_eq!(upd.removes, vec!["Tags[0]"]),
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_multiple_removes() {
    let op = parse_statement_no_params("UPDATE \"T\" REMOVE OldAttr REMOVE Tags[0] WHERE pk = 'x'")
        .unwrap();
    match op {
        DdbOperation::Update(upd) => assert_eq!(upd.removes, vec!["OldAttr", "Tags[0]"]),
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_mixed_set_and_remove() {
    let op = parse_statement_no_params("UPDATE \"T\" SET count = 1 REMOVE OldField WHERE pk = 'x'")
        .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.sets.len(), 1);
            assert_eq!(upd.removes, vec!["OldField"]);
            assert_eq!(upd.key_conditions.len(), 1);
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_returning_all_new() {
    let op = parse_statement_no_params("UPDATE \"T\" SET v = 1 WHERE pk = 'x' RETURNING ALL NEW")
        .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.returning, Some(ReturningClause::AllNew));
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_returning_all_old() {
    let op = parse_statement_no_params("UPDATE \"T\" SET v = 1 WHERE pk = 'x' RETURNING ALL OLD")
        .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.returning, Some(ReturningClause::AllOld));
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_returning_modified_new() {
    let op =
        parse_statement_no_params("UPDATE \"T\" SET v = 1 WHERE pk = 'x' RETURNING MODIFIED NEW")
            .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.returning, Some(ReturningClause::ModifiedNew));
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_returning_modified_old() {
    let op =
        parse_statement_no_params("UPDATE \"T\" SET v = 1 WHERE pk = 'x' RETURNING MODIFIED OLD")
            .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.returning, Some(ReturningClause::ModifiedOld));
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_no_where_clause() {
    let op = parse_statement_no_params("UPDATE \"T\" SET v = 42").unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert!(upd.key_conditions.is_empty());
            assert_eq!(upd.sets.len(), 1);
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_with_params() {
    let op = parse_statement(
        "UPDATE \"T\" SET v = ? WHERE pk = ?",
        &[json!({"N": "99"}), json!({"S": "mykey"})],
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            match &upd.sets[0].1 {
                SetValue::Expr(e) => assert_eq!(e.as_literal(), Some(&json!({"N": "99"}))),
                _ => panic!("expected Expr"),
            }
            assert_eq_path_lit(&upd.key_conditions[0], "pk", json!({"S": "mykey"}));
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_update_set_string_with_embedded_quote() {
    let op =
        parse_statement_no_params("UPDATE \"T\" SET Name = 'O''Brien' WHERE pk = 'x'").unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(e) => assert_eq!(e.as_literal(), Some(&json!({"S": "O'Brien"}))),
            _ => panic!("expected Expr"),
        },
        _ => panic!("expected Update"),
    }
}

// ── DELETE edge cases ────────────────────────────────────────────────────

#[test]
fn test_delete_no_where() {
    let op = parse_statement_no_params("DELETE FROM \"T\"").unwrap();
    match op {
        DdbOperation::Delete(del) => {
            assert_eq!(del.table_name, "T");
            assert!(del.key_conditions.is_empty());
            assert!(del.returning.is_none());
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn test_delete_returning_all_old() {
    let op = parse_statement_no_params(
        "DELETE FROM \"T\" WHERE pk = 'x' AND sk = 'y' RETURNING ALL OLD",
    )
    .unwrap();
    match op {
        DdbOperation::Delete(del) => {
            assert_eq!(del.key_conditions.len(), 2);
            assert_eq!(del.returning, Some(ReturningClause::AllOld));
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn test_delete_with_params() {
    let op = parse_statement(
        "DELETE FROM \"T\" WHERE pk = ? AND sk = ?",
        &[json!({"S": "p1"}), json!({"S": "s1"})],
    )
    .unwrap();
    match op {
        DdbOperation::Delete(del) => {
            assert_eq!(del.key_conditions.len(), 2);
            assert_eq_path_lit(&del.key_conditions[0], "pk", json!({"S": "p1"}));
        }
        _ => panic!("expected Delete"),
    }
}

// ── SELECT edge cases ────────────────────────────────────────────────────

#[test]
fn test_select_secondary_index() {
    let op =
        parse_statement_no_params("SELECT * FROM \"Music\".\"ArtistIndex\" WHERE Artist = 'Acme'")
            .unwrap();
    match op {
        DdbOperation::Select(s) => {
            assert_eq!(s.table_name, "Music");
            assert_eq!(s.index_name.as_deref(), Some("ArtistIndex"));
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_between() {
    let op =
        parse_statement_no_params("SELECT * FROM \"T\" WHERE pk = 'x' AND sk BETWEEN 'a' AND 'z'")
            .unwrap();
    match op {
        DdbOperation::Select(s) => {
            let conds = s.where_clause.unwrap();
            let between = conds
                .iter()
                .find(|c| matches!(c, Condition::Between(..)))
                .expect("expected Between condition");
            match between {
                Condition::Between(value, lo, hi) => {
                    assert_eq!(value.as_path(), Some("sk"));
                    assert_eq!(lo.as_literal(), Some(&json!({"S": "a"})));
                    assert_eq!(hi.as_literal(), Some(&json!({"S": "z"})));
                }
                _ => unreachable!(),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_in_list() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE pk IN ['x', 'y', 'z']").unwrap();
    match op {
        DdbOperation::Select(s) => {
            let conds = s.where_clause.unwrap();
            match &conds[0] {
                Condition::In(value, vals) => {
                    assert_eq!(value.as_path(), Some("pk"));
                    assert_eq!(vals.len(), 3);
                    assert_eq!(vals[0].as_literal(), Some(&json!({"S": "x"})));
                }
                _ => panic!("expected In"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_begins_with() {
    let op = parse_statement_no_params(
        "SELECT * FROM \"T\" WHERE pk = 'x' AND begins_with(sk, 'prefix#')",
    )
    .unwrap();
    match op {
        DdbOperation::Select(s) => {
            let conds = s.where_clause.unwrap();
            let bw = conds
                .iter()
                .find(|c| matches!(c, Condition::BeginsWith(..)))
                .expect("expected BeginsWith condition");
            match bw {
                Condition::BeginsWith(attr, val) => {
                    assert_eq!(attr, "sk");
                    assert_eq!(val.as_literal(), Some(&json!({"S": "prefix#"})));
                }
                _ => unreachable!(),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_contains() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE contains(Tags, 'rust')").unwrap();
    match op {
        DdbOperation::Select(s) => {
            let conds = s.where_clause.unwrap();
            match &conds[0] {
                Condition::Contains(attr, val) => {
                    assert_eq!(attr, "Tags");
                    assert_eq!(val.as_literal(), Some(&json!({"S": "rust"})));
                }
                _ => panic!("expected Contains"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_or_condition() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE pk = 'a' OR pk = 'b'").unwrap();
    match op {
        DdbOperation::Select(s) => {
            let conds = s.where_clause.unwrap();
            assert!(matches!(conds[0], Condition::Or(..)), "got {conds:?}");
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_not_condition() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE NOT pk = 'x'").unwrap();
    match op {
        DdbOperation::Select(s) => {
            let conds = s.where_clause.unwrap();
            assert!(matches!(conds[0], Condition::Not(..)));
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_order_by_asc_explicit() {
    let op =
        parse_statement_no_params("SELECT * FROM \"T\" WHERE pk = 'x' ORDER BY sk ASC").unwrap();
    match op {
        DdbOperation::Select(s) => {
            assert_eq!(s.order_ascending, Some(true));
            assert_eq!(s.order_by_attr.as_deref(), Some("sk"));
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_order_by_desc_with_index() {
    let op = parse_statement_no_params(
        "SELECT * FROM \"T\".\"MyIndex\" WHERE pk = 'x' ORDER BY sk DESC",
    )
    .unwrap();
    match op {
        DdbOperation::Select(s) => {
            assert_eq!(s.index_name.as_deref(), Some("MyIndex"));
            assert_eq!(s.order_ascending, Some(false));
        }
        _ => panic!("expected Select"),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  AWS-spec coverage tests.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn aws_ne_with_bang_eq() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE pk != 'x'").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert!(matches_path_op_lit(
        &conds[0],
        "pk",
        CmpOp::Ne,
        &json!({"S": "x"})
    ));
}

#[test]
fn aws_ne_with_angle_brackets() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE pk <> 'x'").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert!(matches_path_op_lit(
        &conds[0],
        "pk",
        CmpOp::Ne,
        &json!({"S": "x"})
    ));
}

#[test]
fn aws_select_non_equality_full_scan() {
    let op = parse_statement_no_params("SELECT OrderID, Total FROM \"Orders\" WHERE Total > 500")
        .unwrap();
    let (proj, conds) = match op {
        DdbOperation::Select(s) => (s.projection.unwrap(), s.where_clause.unwrap()),
        _ => panic!("expected Select"),
    };
    assert_eq!(proj, vec!["OrderID", "Total"]);
    assert!(matches_path_op_lit(
        &conds[0],
        "Total",
        CmpOp::Gt,
        &json!({"N": "500"})
    ));
}

#[test]
fn aws_select_le_ge_lt() {
    for (sql_op, expected) in [
        ("<", CmpOp::Lt),
        ("<=", CmpOp::Le),
        (">", CmpOp::Gt),
        (">=", CmpOp::Ge),
    ] {
        let stmt = format!("SELECT * FROM \"T\" WHERE n {sql_op} 5");
        let op = parse_statement_no_params(&stmt)
            .unwrap_or_else(|e| panic!("parse failed for {stmt:?}: {e}"));
        let conds = match op {
            DdbOperation::Select(s) => s.where_clause.unwrap(),
            _ => panic!("expected Select"),
        };
        match &conds[0] {
            Condition::Compare(_, op, _) => assert_eq!(*op, expected, "for {sql_op}"),
            other => panic!("expected Compare, got {other:?}"),
        }
    }
}

#[test]
fn aws_where_with_quoted_attr_names() {
    let op = parse_statement_no_params(
        "DELETE FROM \"Music\" WHERE \"Artist\" = 'Acme Band' AND \"SongTitle\" = 'PartiQL Rocks'",
    )
    .unwrap();
    match op {
        DdbOperation::Delete(del) => {
            assert_eq!(del.table_name, "Music");
            assert_eq!(del.key_conditions.len(), 2);
            assert_eq_path_lit(&del.key_conditions[0], "Artist", json!({"S": "Acme Band"}));
            assert_eq_path_lit(
                &del.key_conditions[1],
                "SongTitle",
                json!({"S": "PartiQL Rocks"}),
            );
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn aws_select_nested_path_with_index_in_projection() {
    let op = parse_statement_no_params(
        "SELECT Devices.FireStick.DateWatched[0] FROM WatchList WHERE CustomerID = 'C1' AND MovieID = 'M1'",
    )
    .unwrap();
    match op {
        DdbOperation::Select(s) => {
            assert_eq!(
                s.projection.unwrap(),
                vec!["Devices.FireStick.DateWatched[0]"]
            );
            assert_eq!(s.where_clause.unwrap().len(), 2);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn aws_where_with_nested_path_and_index() {
    let op = parse_statement_no_params(
        "SELECT Devices FROM WatchList WHERE Devices.FireStick.DateWatched[0] >= '12/24/19'",
    )
    .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert!(matches_path_op_lit(
        &conds[0],
        "Devices.FireStick.DateWatched[0]",
        CmpOp::Ge,
        &json!({"S": "12/24/19"}),
    ));
}

#[test]
fn aws_select_in_with_integers() {
    let op = parse_statement_no_params(
        "SELECT OrderID, Total FROM \"Orders\" WHERE OrderID IN [100, 300, 234]",
    )
    .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::In(value, vals) => {
            assert_eq!(value.as_path(), Some("OrderID"));
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0].as_literal(), Some(&json!({"N": "100"})));
            assert_eq!(vals[2].as_literal(), Some(&json!({"N": "234"})));
        }
        other => panic!("expected In, got {other:?}"),
    }
}

#[test]
fn aws_select_between_with_integers() {
    let op = parse_statement_no_params(
        "SELECT OrderID, Total FROM \"Orders\" WHERE Total BETWEEN 500 AND 600",
    )
    .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Between(value, lo, hi) => {
            assert_eq!(value.as_path(), Some("Total"));
            assert_eq!(lo.as_literal(), Some(&json!({"N": "500"})));
            assert_eq!(hi.as_literal(), Some(&json!({"N": "600"})));
        }
        other => panic!("expected Between, got {other:?}"),
    }
}

#[test]
fn aws_update_set_struct_literal_rhs() {
    let op = parse_statement_no_params(
        "UPDATE \"Music\" SET AwardDetail = {'Grammys': [2020, 2018]} WHERE Artist = 'Acme Band'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(e) => {
                let v = e.as_literal().expect("expected literal");
                let m = v.get("M").expect("expected M");
                let grammys = m.get("Grammys").expect("expected Grammys");
                let list = grammys.get("L").expect("expected L");
                assert_eq!(list[0], json!({"N": "2020"}));
                assert_eq!(list[1], json!({"N": "2018"}));
            }
            other => panic!("expected Expr literal, got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn aws_update_set_list_literal_rhs() {
    let op = parse_statement_no_params(
        "UPDATE \"Music\" SET AwardDetail.BillBoard = [2020] WHERE Artist = 'A'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.sets[0].0, "AwardDetail.BillBoard");
            match &upd.sets[0].1 {
                SetValue::Expr(e) => {
                    let v = e.as_literal().expect("expected literal");
                    let list = v.get("L").expect("expected L");
                    assert_eq!(list[0], json!({"N": "2020"}));
                }
                other => panic!("expected Expr literal, got {other:?}"),
            }
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn aws_update_set_bag_literal_rhs() {
    let op = parse_statement_no_params(
        "UPDATE \"Music\" SET BandMembers = <<'member1', 'member2'>> WHERE Artist = 'A'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(e) => {
                let v = e.as_literal().expect("expected literal");
                let ss = v.get("SS").expect("expected SS");
                let mut vals: Vec<&str> = ss
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap())
                    .collect();
                vals.sort_unstable();
                assert_eq!(vals, vec!["member1", "member2"]);
            }
            other => panic!("expected Expr literal, got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn aws_update_list_append_with_nested_path_target() {
    let op = parse_statement_no_params(
        "UPDATE \"Music\" SET AwardDetail.Grammys = list_append(AwardDetail.Grammys, [2016]) WHERE Artist = 'A'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.sets[0].0, "AwardDetail.Grammys");
            match &upd.sets[0].1 {
                SetValue::ListAppend(target, val) => {
                    assert_eq!(target, "AwardDetail.Grammys");
                    let list = val.get("L").expect("expected L");
                    assert_eq!(list[0], json!({"N": "2016"}));
                }
                other => panic!("expected ListAppend, got {other:?}"),
            }
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn aws_update_remove_nested_path_with_index() {
    let op = parse_statement_no_params(
        "UPDATE \"Music\" REMOVE AwardDetail.Grammys[2] WHERE Artist = 'A'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.removes, vec!["AwardDetail.Grammys[2]"]);
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn aws_returning_all_old_with_star() {
    let op =
        parse_statement_no_params("DELETE FROM \"Music\" WHERE Artist = 'A' RETURNING ALL OLD *")
            .unwrap();
    match op {
        DdbOperation::Delete(del) => {
            assert_eq!(del.returning, Some(ReturningClause::AllOld));
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn aws_returning_modified_new_with_star() {
    let op =
        parse_statement_no_params("UPDATE \"T\" SET v = 1 WHERE pk = 'x' RETURNING MODIFIED NEW *")
            .unwrap();
    match op {
        DdbOperation::Update(upd) => {
            assert_eq!(upd.returning, Some(ReturningClause::ModifiedNew));
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn empty_list_literal_in_value() {
    let op = parse_statement_no_params("INSERT INTO \"T\" VALUE {'pk': 'x', 'Tags': []}").unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            let tags = ins.item.get("Tags").unwrap();
            let list = tags.get("L").expect("expected L");
            assert_eq!(list.as_array().unwrap().len(), 0);
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn empty_struct_literal_in_value() {
    let op = parse_statement_no_params("INSERT INTO \"T\" VALUE {'pk': 'x', 'Meta': {}}").unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            let meta = ins.item.get("Meta").unwrap();
            let m = meta.get("M").expect("expected M");
            assert_eq!(m.as_object().unwrap().len(), 0);
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn mixed_type_bag_rejected_with_canonical_message() {
    let err = parse_statement_no_params("INSERT INTO \"T\" VALUE {'pk': 'x', 'Bad': <<'a', 1>>}")
        .unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("set must contain all strings or all numbers"),
        "wrong error message: {msg}",
    );
}

#[test]
fn negative_number_literal_in_value_position() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE temperature = -40").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert!(matches_path_op_lit(
        &conds[0],
        "temperature",
        CmpOp::Eq,
        &json!({"N": "-40"})
    ));
}

#[test]
fn float_with_exponent_preserved_textually() {
    let op = parse_statement_no_params("INSERT INTO \"T\" VALUE {'pk': 'x', 'big': 1e10}").unwrap();
    match op {
        DdbOperation::Insert(ins) => {
            let big = ins.item.get("big").unwrap();
            assert_eq!(big.get("N").unwrap().as_str().unwrap(), "1e10");
        }
        _ => panic!("expected Insert"),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  AWS-spec parity tests — IS MISSING / IS NULL, attribute_type, size.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn aws_is_missing() {
    let op =
        parse_statement_no_params("SELECT * FROM \"Music\" WHERE \"Awards\" IS MISSING").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert!(matches!(&conds[0], Condition::IsMissing(attr) if attr == "Awards"));
}

#[test]
fn aws_is_not_missing() {
    let op =
        parse_statement_no_params("SELECT * FROM \"Music\" WHERE Awards IS NOT MISSING").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Not(inner) => {
            assert!(matches!(inner.as_ref(), Condition::IsMissing(attr) if attr == "Awards"));
        }
        other => panic!("expected Not(IsMissing), got {other:?}"),
    }
}

#[test]
fn aws_is_null() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE deleted IS NULL").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert!(matches!(&conds[0], Condition::IsNull(attr) if attr == "deleted"));
}

#[test]
fn aws_is_not_null() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE deleted IS NOT NULL").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Not(inner) => {
            assert!(matches!(inner.as_ref(), Condition::IsNull(attr) if attr == "deleted"));
        }
        other => panic!("expected Not(IsNull), got {other:?}"),
    }
}

#[test]
fn aws_is_missing_in_compound_where() {
    let op = parse_statement_no_params(
        "SELECT * FROM \"Music\" WHERE Artist = 'Acme' AND \"Awards\" IS MISSING",
    )
    .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 2);
    assert!(matches!(&conds[0], Condition::Compare(_, CmpOp::Eq, _)));
    assert!(matches!(&conds[1], Condition::IsMissing(attr) if attr == "Awards"));
}

#[test]
fn aws_attribute_type() {
    let op =
        parse_statement_no_params("SELECT * FROM \"Music\" WHERE attribute_type(\"Artist\", 'S')")
            .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::AttributeType(attr, ty) => {
            assert_eq!(attr, "Artist");
            assert_eq!(ty, "S");
        }
        other => panic!("expected AttributeType, got {other:?}"),
    }
}

#[test]
fn aws_attribute_type_with_path_arg() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE attribute_type(Meta.flags, 'L')")
        .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::AttributeType(attr, ty) => {
            assert_eq!(attr, "Meta.flags");
            assert_eq!(ty, "L");
        }
        other => panic!("expected AttributeType, got {other:?}"),
    }
}

#[test]
fn aws_size_function_with_gt() {
    let op = parse_statement_no_params(
        "SELECT * FROM \"Orders\" WHERE OrderID = 1 AND size(\"Image\") > 300",
    )
    .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 2);
    match &conds[1] {
        Condition::Size(attr, op, val) => {
            assert_eq!(attr, "Image");
            assert_eq!(*op, CmpOp::Gt);
            assert_eq!(val.as_literal(), Some(&json!({"N": "300"})));
        }
        other => panic!("expected Size, got {other:?}"),
    }
}

#[test]
fn aws_size_function_all_comparison_ops() {
    let cases = [
        ("=", CmpOp::Eq),
        ("!=", CmpOp::Ne),
        ("<", CmpOp::Lt),
        ("<=", CmpOp::Le),
        (">", CmpOp::Gt),
        (">=", CmpOp::Ge),
    ];
    for (sql_op, expected) in cases {
        let stmt = format!("SELECT * FROM \"T\" WHERE size(blob) {sql_op} 100");
        let op = parse_statement_no_params(&stmt)
            .unwrap_or_else(|e| panic!("parse failed for {stmt:?}: {e}"));
        let conds = match op {
            DdbOperation::Select(s) => s.where_clause.unwrap(),
            _ => panic!("expected Select"),
        };
        match &conds[0] {
            Condition::Size(_, op, _) => assert_eq!(*op, expected, "for {sql_op}"),
            other => panic!("expected Size for {sql_op}, got {other:?}"),
        }
    }
}

#[test]
fn aws_size_with_nested_path() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE size(Meta.tags) >= 5").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Size(attr, op, val) => {
            assert_eq!(attr, "Meta.tags");
            assert_eq!(*op, CmpOp::Ge);
            assert_eq!(val.as_literal(), Some(&json!({"N": "5"})));
        }
        other => panic!("expected Size, got {other:?}"),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Operator precedence tests.
// ─────────────────────────────────────────────────────────────────────────────

/// Helper: assert that `c` is `Compare(Path(attr), _, _)` regardless of op/RHS.
fn matches_path_compare(c: &Condition, attr: &str) -> bool {
    matches!(c, Condition::Compare(lhs, _, _) if lhs.as_path() == Some(attr))
}

#[test]
fn precedence_and_binds_tighter_than_or() {
    let op =
        parse_statement_no_params("SELECT * FROM \"T\" WHERE a = 1 OR b = 2 AND c = 3").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 1, "OR is the top-level node");
    match &conds[0] {
        Condition::Or(left, right) => {
            assert!(matches_path_compare(left.as_ref(), "a"));
            match right.as_ref() {
                Condition::And(l, r) => {
                    assert!(matches_path_compare(l.as_ref(), "b"));
                    assert!(matches_path_compare(r.as_ref(), "c"));
                }
                other => panic!("expected And on RHS of Or, got {other:?}"),
            }
        }
        other => panic!("expected Or, got {other:?}"),
    }
}

#[test]
fn precedence_and_chain_is_left_associative_and_flattens() {
    let op =
        parse_statement_no_params("SELECT * FROM \"T\" WHERE a = 1 AND b = 2 AND c = 3").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 3);
    let attrs: Vec<&str> = conds
        .iter()
        .map(|c| match c {
            Condition::Compare(lhs, _, _) => lhs.as_path().expect("path"),
            other => panic!("expected Compare, got {other:?}"),
        })
        .collect();
    assert_eq!(attrs, vec!["a", "b", "c"]);
}

#[test]
fn precedence_or_chain_is_left_associative() {
    let op =
        parse_statement_no_params("SELECT * FROM \"T\" WHERE a = 1 OR b = 2 OR c = 3").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 1);
    match &conds[0] {
        Condition::Or(outer_l, outer_r) => {
            match outer_l.as_ref() {
                Condition::Or(l, r) => {
                    assert!(matches_path_compare(l.as_ref(), "a"));
                    assert!(matches_path_compare(r.as_ref(), "b"));
                }
                other => panic!("expected Or on left, got {other:?}"),
            }
            assert!(matches_path_compare(outer_r.as_ref(), "c"));
        }
        other => panic!("expected Or, got {other:?}"),
    }
}

#[test]
fn precedence_not_lower_than_comparison() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE NOT a = 1").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Not(inner) => {
            assert!(matches_path_compare(inner.as_ref(), "a"));
        }
        other => panic!("expected Not, got {other:?}"),
    }
}

#[test]
fn precedence_not_higher_than_and() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE NOT a = 1 AND b = 2").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 2);
    match &conds[0] {
        Condition::Not(inner) => {
            assert!(matches_path_compare(inner.as_ref(), "a"));
        }
        other => panic!("expected Not, got {other:?}"),
    }
    assert!(matches_path_compare(&conds[1], "b"));
}

#[test]
fn precedence_not_higher_than_or() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE NOT a = 1 OR b = 2").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 1);
    match &conds[0] {
        Condition::Or(l, r) => {
            assert!(matches!(l.as_ref(), Condition::Not(_)));
            assert!(matches_path_compare(r.as_ref(), "b"));
        }
        other => panic!("expected Or, got {other:?}"),
    }
}

#[test]
fn precedence_double_not_chains() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE NOT NOT x = 1").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Not(outer) => match outer.as_ref() {
            Condition::Not(inner) => {
                assert!(matches_path_compare(inner.as_ref(), "x"));
            }
            other => panic!("inner not Not: {other:?}"),
        },
        other => panic!("expected Not, got {other:?}"),
    }
}

#[test]
fn precedence_between_consumes_its_own_and() {
    let op =
        parse_statement_no_params("SELECT * FROM \"T\" WHERE pk BETWEEN 'a' AND 'z' AND sk = 'x'")
            .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 2);
    assert!(matches!(&conds[0], Condition::Between(value, _, _) if value.as_path() == Some("pk")));
    assert!(matches_path_compare(&conds[1], "sk"));
}

#[test]
fn precedence_between_with_or_after() {
    let op =
        parse_statement_no_params("SELECT * FROM \"T\" WHERE pk BETWEEN 'a' AND 'z' OR sk = 'x'")
            .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 1);
    match &conds[0] {
        Condition::Or(l, r) => {
            assert!(matches!(l.as_ref(), Condition::Between(_, _, _)));
            assert!(matches_path_compare(r.as_ref(), "sk"));
        }
        other => panic!("expected Or, got {other:?}"),
    }
}

#[test]
fn precedence_is_missing_with_compound() {
    let op = parse_statement_no_params(
        "SELECT * FROM \"T\" WHERE pk = 'x' AND awards IS MISSING OR genre = 'rock'",
    )
    .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 1);
    match &conds[0] {
        Condition::Or(l, r) => {
            match l.as_ref() {
                Condition::And(la, lb) => {
                    assert!(matches_path_compare(la.as_ref(), "pk"));
                    assert!(matches!(lb.as_ref(), Condition::IsMissing(a) if a == "awards"));
                }
                other => panic!("expected And, got {other:?}"),
            }
            assert!(matches_path_compare(r.as_ref(), "genre"));
        }
        other => panic!("expected Or, got {other:?}"),
    }
}

#[test]
fn precedence_in_then_and() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE pk IN ['a', 'b'] AND sk = 'x'")
        .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 2);
    assert!(matches!(&conds[0], Condition::In(value, _) if value.as_path() == Some("pk")));
    assert!(matches_path_compare(&conds[1], "sk"));
}

#[test]
fn precedence_size_predicate_in_compound() {
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE pk = 'x' AND size(blob) > 100")
        .unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    assert_eq!(conds.len(), 2);
    assert!(matches_path_compare(&conds[0], "pk"));
    assert!(matches!(&conds[1], Condition::Size(a, op, _)
        if a == "blob" && *op == CmpOp::Gt));
}

#[test]
fn precedence_not_around_function_call_predicate() {
    let op =
        parse_statement_no_params("SELECT * FROM \"T\" WHERE NOT begins_with(sk, 'p')").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Not(inner) => {
            assert!(matches!(inner.as_ref(), Condition::BeginsWith(a, _) if a == "sk"));
        }
        other => panic!("expected Not, got {other:?}"),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Arithmetic operators (`+` / `-`) — full Expression IR.
//  After the IR refactor, arithmetic works in any value position: SET RHS,
//  WHERE comparison RHS, BETWEEN bounds, IN list elements, function args.
//  Path + path, parameters, parentheses, and chained arithmetic all parse.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn arith_path_plus_param_placeholder() {
    let op = parse_statement(
        "UPDATE \"T\" SET counter = counter + ? WHERE pk = 'x'",
        &[json!({"N": "7"})],
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(Expression::BinaryOp(l, ArithOp::Add, r)) => {
                assert_eq!(l.as_path(), Some("counter"));
                assert_eq!(r.as_literal(), Some(&json!({"N": "7"})));
            }
            other => panic!("expected Expr(BinaryOp Add), got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn arith_path_minus_param_placeholder() {
    let op = parse_statement(
        "UPDATE \"T\" SET score = score - ? WHERE pk = 'x'",
        &[json!({"N": "5"})],
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(Expression::BinaryOp(l, ArithOp::Sub, r)) => {
                assert_eq!(l.as_path(), Some("score"));
                assert_eq!(r.as_literal(), Some(&json!({"N": "5"})));
            }
            other => panic!("expected Expr(BinaryOp Sub), got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn arith_path_plus_negative_literal() {
    let op = parse_statement_no_params("UPDATE \"T\" SET counter = counter + -1 WHERE pk = 'x'")
        .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(Expression::BinaryOp(l, ArithOp::Add, r)) => {
                assert_eq!(l.as_path(), Some("counter"));
                assert_eq!(r.as_literal(), Some(&json!({"N": "-1"})));
            }
            other => panic!("expected Expr(BinaryOp Add), got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn arith_path_plus_path_now_supported() {
    // Per AWS operator reference, `+` is an arithmetic operator without
    // context restriction. After the Expression-IR refactor, `path + path`
    // is allowed.
    let op =
        parse_statement_no_params("UPDATE \"T\" SET total = price + tax WHERE pk = 'x'").unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(Expression::BinaryOp(l, ArithOp::Add, r)) => {
                assert_eq!(l.as_path(), Some("price"));
                assert_eq!(r.as_path(), Some("tax"));
            }
            other => panic!("expected Expr(BinaryOp Add), got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn arith_chained_addition_left_associative() {
    // `counter + 1 + 2` ≡ `(counter + 1) + 2`.
    let op = parse_statement_no_params("UPDATE \"T\" SET counter = counter + 1 + 2 WHERE pk = 'x'")
        .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(Expression::BinaryOp(outer_l, ArithOp::Add, outer_r)) => {
                assert_eq!(outer_r.as_literal(), Some(&json!({"N": "2"})));
                match outer_l.as_ref() {
                    Expression::BinaryOp(l, ArithOp::Add, r) => {
                        assert_eq!(l.as_path(), Some("counter"));
                        assert_eq!(r.as_literal(), Some(&json!({"N": "1"})));
                    }
                    other => panic!("expected nested BinaryOp Add, got {other:?}"),
                }
            }
            other => panic!("expected Expr(BinaryOp Add), got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn arith_in_where_clause_lhs() {
    // `counter + 1 > 5` — arithmetic on LHS of comparison.
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE counter + 1 > 5").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Compare(lhs, CmpOp::Gt, rhs) => {
            assert!(matches!(lhs, Expression::BinaryOp(_, ArithOp::Add, _)));
            assert_eq!(rhs.as_literal(), Some(&json!({"N": "5"})));
        }
        other => panic!("expected Compare(_, Gt, _), got {other:?}"),
    }
}

#[test]
fn arith_in_where_clause_rhs() {
    // `pk = base + 10` — arithmetic on RHS of comparison.
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE pk = base + 10").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Compare(lhs, CmpOp::Eq, rhs) => {
            assert_eq!(lhs.as_path(), Some("pk"));
            match rhs {
                Expression::BinaryOp(l, ArithOp::Add, r) => {
                    assert_eq!(l.as_path(), Some("base"));
                    assert_eq!(r.as_literal(), Some(&json!({"N": "10"})));
                }
                other => panic!("expected BinaryOp Add, got {other:?}"),
            }
        }
        other => panic!("expected Compare, got {other:?}"),
    }
}

#[test]
fn arith_subexpression_with_parens() {
    // `(price + tax) - discount` — parentheses force grouping.
    let op = parse_statement_no_params(
        "UPDATE \"T\" SET total = (price + tax) - discount WHERE pk = 'x'",
    )
    .unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(Expression::BinaryOp(outer_l, ArithOp::Sub, outer_r)) => {
                assert_eq!(outer_r.as_path(), Some("discount"));
                match outer_l.as_ref() {
                    Expression::BinaryOp(l, ArithOp::Add, r) => {
                        assert_eq!(l.as_path(), Some("price"));
                        assert_eq!(r.as_path(), Some("tax"));
                    }
                    other => panic!("expected nested BinaryOp Add, got {other:?}"),
                }
            }
            other => panic!("expected Expr(BinaryOp Sub), got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn arith_parens_override_precedence() {
    // Without parens, `a - b + c` is `(a - b) + c` (left-assoc).
    // With parens, `a - (b + c)` is different.
    let op = parse_statement_no_params("UPDATE \"T\" SET v = a - (b + c) WHERE pk = 'x'").unwrap();
    match op {
        DdbOperation::Update(upd) => match &upd.sets[0].1 {
            SetValue::Expr(Expression::BinaryOp(l, ArithOp::Sub, r)) => {
                assert_eq!(l.as_path(), Some("a"));
                match r.as_ref() {
                    Expression::BinaryOp(inner_l, ArithOp::Add, inner_r) => {
                        assert_eq!(inner_l.as_path(), Some("b"));
                        assert_eq!(inner_r.as_path(), Some("c"));
                    }
                    other => panic!("expected (b + c), got {other:?}"),
                }
            }
            other => panic!("expected Expr(BinaryOp Sub), got {other:?}"),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn arith_in_in_list() {
    // `pk IN [a + 1, a + 2]` — arithmetic in IN list elements.
    let op = parse_statement_no_params("SELECT * FROM \"T\" WHERE pk IN [a + 1, a + 2]").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::In(value, vals) => {
            assert_eq!(value.as_path(), Some("pk"));
            assert_eq!(vals.len(), 2);
            assert!(matches!(&vals[0], Expression::BinaryOp(_, ArithOp::Add, _)));
            assert!(matches!(&vals[1], Expression::BinaryOp(_, ArithOp::Add, _)));
        }
        other => panic!("expected In, got {other:?}"),
    }
}

#[test]
fn arith_in_between_bounds() {
    // `n BETWEEN x - 1 AND x + 1`.
    let op =
        parse_statement_no_params("SELECT * FROM \"T\" WHERE n BETWEEN x - 1 AND x + 1").unwrap();
    let conds = match op {
        DdbOperation::Select(s) => s.where_clause.unwrap(),
        _ => panic!("expected Select"),
    };
    match &conds[0] {
        Condition::Between(value, lo, hi) => {
            assert_eq!(value.as_path(), Some("n"));
            assert!(matches!(lo, Expression::BinaryOp(_, ArithOp::Sub, _)));
            assert!(matches!(hi, Expression::BinaryOp(_, ArithOp::Add, _)));
        }
        other => panic!("expected Between, got {other:?}"),
    }
}

// ── Reject tests ────────────────────────────────────────────────────────────

#[test]
fn rejects_join() {
    let err = parse_statement_no_params("SELECT * FROM \"A\", \"B\" WHERE A.pk = 'x'").unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn rejects_unknown_statement_keyword() {
    let err = parse_statement_no_params("CREATE TABLE foo (id INT)").unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("unsupported") || msg.contains("CREATE") || msg.contains("Create"),
        "got: {msg}"
    );
}

#[test]
fn rejects_trailing_garbage_after_statement() {
    let err = parse_statement_no_params("SELECT * FROM \"T\" extra garbage").unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("trailing") || msg.contains("parse"),
        "got: {msg}"
    );
}

#[test]
fn parameter_count_mismatch_too_few() {
    let err = parse_statement(
        "SELECT * FROM \"T\" WHERE pk = ? AND sk = ?",
        &[json!({"S": "p"})],
    )
    .unwrap_err();
    assert!(err.to_string().contains("parameter"), "got: {err}");
}

#[test]
fn parameter_count_mismatch_too_many() {
    let err = parse_statement(
        "SELECT * FROM \"T\" WHERE pk = ?",
        &[json!({"S": "p"}), json!({"S": "extra"})],
    )
    .unwrap_err();
    assert!(err.to_string().contains("parameter"), "got: {err}");
}

// ─────────────────────────────────────────────────────────────────────────────
//  EXISTS — transactions-only condition check.
//  AWS docs: ql-functions.exists.html. The parser accepts EXISTS at the top
//  level; the handler in winterbaume-dynamodb restricts it to transactions
//  and treats an empty inner-SELECT result as a failed condition.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn aws_exists_basic() {
    // From AWS docs.
    let op = parse_statement_no_params(
        "EXISTS(SELECT * FROM \"Music\" WHERE \"Artist\" = 'Acme Band' AND \"SongTitle\" = 'PartiQL Rocks')",
    )
    .unwrap();
    match op {
        DdbOperation::Exists(sel) => {
            assert_eq!(sel.table_name, "Music");
            assert!(sel.projection.is_none(), "EXISTS uses SELECT *");
            let conds = sel.where_clause.as_ref().expect("WHERE clause");
            assert_eq!(conds.len(), 2);
            assert_eq_path_lit(&conds[0], "Artist", json!({"S": "Acme Band"}));
            assert_eq_path_lit(&conds[1], "SongTitle", json!({"S": "PartiQL Rocks"}));
        }
        other => panic!("expected Exists, got {other:?}"),
    }
}

#[test]
fn aws_exists_with_parameters() {
    let op = parse_statement(
        "EXISTS(SELECT * FROM \"T\" WHERE pk = ?)",
        &[json!({"S": "key1"})],
    )
    .unwrap();
    match op {
        DdbOperation::Exists(sel) => {
            assert_eq!(sel.table_name, "T");
            let conds = sel.where_clause.as_ref().unwrap();
            assert_eq_path_lit(&conds[0], "pk", json!({"S": "key1"}));
        }
        other => panic!("expected Exists, got {other:?}"),
    }
}

#[test]
fn aws_exists_table_name_propagates() {
    let op = parse_statement_no_params("EXISTS(SELECT * FROM \"Music\" WHERE pk = 'x')").unwrap();
    assert_eq!(op.table_name(), "Music");
}

#[test]
fn aws_exists_case_insensitive_keyword() {
    let op = parse_statement_no_params("exists(SELECT * FROM \"T\" WHERE pk = 'x')").unwrap();
    assert!(matches!(op, DdbOperation::Exists(_)));
}

#[test]
fn aws_exists_rejects_non_select_inside() {
    let err = parse_statement_no_params("EXISTS(INSERT INTO \"T\" VALUE {'pk': 'x'})").unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn aws_exists_requires_parens() {
    let err = parse_statement_no_params("EXISTS SELECT * FROM \"T\" WHERE pk = 'x'").unwrap_err();
    assert!(!err.to_string().is_empty());
}

#[test]
fn aws_exists_requires_closing_paren() {
    let err = parse_statement_no_params("EXISTS(SELECT * FROM \"T\" WHERE pk = 'x'").unwrap_err();
    assert!(!err.to_string().is_empty());
}
