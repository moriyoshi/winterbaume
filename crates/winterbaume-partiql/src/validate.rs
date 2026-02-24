//! Runtime validators for AWS DynamoDB PartiQL fidelity.
//!
//! The hand-rolled parser produces a maximally general IR (e.g. arithmetic
//! is allowed wherever an [`Expression`] is allowed) so that a single AST
//! shape can serve UPDATE-SET right-hand sides, WHERE operands, and BETWEEN
//! bounds alike. AWS DynamoDB's runtime, however, rejects certain operand
//! shapes in WHERE-clause contexts — most notably `+`/`-` arithmetic and
//! unary minus over a path expression. This module walks a parsed
//! [`Condition`] tree and surfaces the exact AWS error wording when those
//! disallowed shapes appear.
//!
//! See:
//! - <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ql-reference.condition.html>

use crate::operation::{ArithOp, Condition, Expression};

/// Error wording AWS uses for `+`/`-` arithmetic appearing inside a
/// condition-expression operand. The message references the offending
/// operator literally; both `+` and `-` cases share this wording with only
/// the operator character differing.
fn unsupported_arith_message(op: ArithOp) -> String {
    let ch = match op {
        ArithOp::Add => '+',
        ArithOp::Sub => '-',
    };
    format!("Unsupported operator in Condition Expression. Operator: {ch}")
}

/// Error wording AWS uses for `-path` (unary minus over a non-literal
/// expression). AWS frames this as a unary-arity error against the binary
/// `-` operator: "operator or function: -, number of operands: 1".
const UNARY_NEG_MESSAGE: &str = "Incorrect number of operands for operator or function; operator or function: -, number of operands: 1";

/// Walk a single [`Expression`] and return the AWS-shaped error string if
/// the expression contains arithmetic or a unary-minus subtree. Used as a
/// helper for the top-level [`validate_condition`] walk.
fn validate_expression(expr: &Expression) -> Result<(), String> {
    match expr {
        Expression::Literal(_) | Expression::Path(_) => Ok(()),
        Expression::BinaryOp(_, op, _) => Err(unsupported_arith_message(*op)),
        Expression::Neg(_) => Err(UNARY_NEG_MESSAGE.to_string()),
    }
}

/// Walk a single [`Condition`] and return the AWS-shaped ValidationException
/// message string if any operand contains a forbidden expression shape
/// (arithmetic or unary-minus). Returns `Ok(())` if every operand is a
/// literal, path, or boolean combinator over the same.
///
/// The walk is shallow per node — it inspects every direct expression
/// operand of the condition and recurses through `And`/`Or`/`Not`. It does
/// **not** descend into [`Expression`] subtrees beyond the top level
/// because the parser folds nested arithmetic into a single
/// [`Expression::BinaryOp`] / [`Expression::Neg`] node at the operand
/// position; any forbidden shape is therefore visible at the operand root.
pub fn validate_condition(cond: &Condition) -> Result<(), String> {
    match cond {
        Condition::Compare(lhs, _, rhs) => {
            validate_expression(lhs)?;
            validate_expression(rhs)?;
            Ok(())
        }
        Condition::Between(value, lo, hi) => {
            validate_expression(value)?;
            validate_expression(lo)?;
            validate_expression(hi)?;
            Ok(())
        }
        Condition::In(value, list) => {
            validate_expression(value)?;
            for item in list {
                validate_expression(item)?;
            }
            Ok(())
        }
        Condition::Size(_, _, rhs) => {
            validate_expression(rhs)?;
            Ok(())
        }
        Condition::BeginsWith(_, rhs) | Condition::Contains(_, rhs) => validate_expression(rhs),
        Condition::And(a, b) | Condition::Or(a, b) => {
            validate_condition(a)?;
            validate_condition(b)?;
            Ok(())
        }
        Condition::Not(inner) => validate_condition(inner),
        Condition::IsMissing(_) | Condition::IsNull(_) | Condition::AttributeType(_, _) => Ok(()),
    }
}

/// Walk an entire WHERE-clause [`Vec<Condition>`] and short-circuit at the
/// first forbidden operand, returning the AWS-shaped message. Convenience
/// wrapper around [`validate_condition`].
pub fn validate_where(conds: &[Condition]) -> Result<(), String> {
    for c in conds {
        validate_condition(c)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::operation::{ArithOp, CmpOp, Condition, Expression};

    fn lit_n(n: i64) -> Expression {
        Expression::Literal(json!({"N": n.to_string()}))
    }

    #[test]
    fn allows_plain_compare() {
        let c = Condition::Compare(Expression::path("counter"), CmpOp::Eq, lit_n(5));
        assert!(validate_condition(&c).is_ok());
    }

    #[test]
    fn rejects_add_in_compare_rhs() {
        // counter = 5 + 5
        let rhs = Expression::BinaryOp(Box::new(lit_n(5)), ArithOp::Add, Box::new(lit_n(5)));
        let c = Condition::Compare(Expression::path("counter"), CmpOp::Eq, rhs);
        let msg = validate_condition(&c).unwrap_err();
        assert_eq!(
            msg,
            "Unsupported operator in Condition Expression. Operator: +"
        );
    }

    #[test]
    fn rejects_sub_in_compare_lhs() {
        // counter - 1 = 4
        let lhs = Expression::BinaryOp(
            Box::new(Expression::path("counter")),
            ArithOp::Sub,
            Box::new(lit_n(1)),
        );
        let c = Condition::Compare(lhs, CmpOp::Eq, lit_n(4));
        let msg = validate_condition(&c).unwrap_err();
        assert_eq!(
            msg,
            "Unsupported operator in Condition Expression. Operator: -"
        );
    }

    #[test]
    fn rejects_neg_in_compare() {
        // -counter = 5
        let lhs = Expression::Neg(Box::new(Expression::path("counter")));
        let c = Condition::Compare(lhs, CmpOp::Eq, lit_n(5));
        let msg = validate_condition(&c).unwrap_err();
        assert!(msg.starts_with("Incorrect number of operands"));
        assert!(msg.contains("operator or function: -, number of operands: 1"));
    }

    #[test]
    fn rejects_arith_inside_between_bound() {
        // counter BETWEEN 1 AND (5 + 5)
        let hi = Expression::BinaryOp(Box::new(lit_n(5)), ArithOp::Add, Box::new(lit_n(5)));
        let c = Condition::Between(Expression::path("counter"), lit_n(1), hi);
        let msg = validate_condition(&c).unwrap_err();
        assert!(msg.contains("Operator: +"));
    }

    #[test]
    fn rejects_arith_in_in_list_element() {
        // counter IN (1, 2 + 3)
        let elem = Expression::BinaryOp(Box::new(lit_n(2)), ArithOp::Add, Box::new(lit_n(3)));
        let c = Condition::In(Expression::path("counter"), vec![lit_n(1), elem]);
        let msg = validate_condition(&c).unwrap_err();
        assert!(msg.contains("Operator: +"));
    }

    #[test]
    fn rejects_arith_in_size_rhs() {
        // size(name) > (1 + 1)
        let rhs = Expression::BinaryOp(Box::new(lit_n(1)), ArithOp::Add, Box::new(lit_n(1)));
        let c = Condition::Size("name".to_string(), CmpOp::Gt, rhs);
        let msg = validate_condition(&c).unwrap_err();
        assert!(msg.contains("Operator: +"));
    }

    #[test]
    fn descends_through_and_or_not() {
        // NOT (a = 1 AND counter = 5 + 5)
        let bad = Condition::Compare(
            Expression::path("counter"),
            CmpOp::Eq,
            Expression::BinaryOp(Box::new(lit_n(5)), ArithOp::Add, Box::new(lit_n(5))),
        );
        let good = Condition::Compare(Expression::path("a"), CmpOp::Eq, lit_n(1));
        let combined = Condition::Not(Box::new(Condition::And(Box::new(good), Box::new(bad))));
        let msg = validate_condition(&combined).unwrap_err();
        assert!(msg.contains("Operator: +"));
    }

    #[test]
    fn validate_where_short_circuits_on_first_offender() {
        let good = Condition::Compare(Expression::path("a"), CmpOp::Eq, lit_n(1));
        let bad = Condition::Compare(
            Expression::path("counter"),
            CmpOp::Eq,
            Expression::BinaryOp(Box::new(lit_n(5)), ArithOp::Add, Box::new(lit_n(5))),
        );
        let res = validate_where(&[good, bad]);
        assert!(res.is_err());
    }
}
