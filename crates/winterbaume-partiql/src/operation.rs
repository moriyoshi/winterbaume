use std::collections::HashMap;

use serde_json::Value;

/// A DynamoDB-typed attribute value (e.g., {"S": "foo"}, {"N": "42"}).
pub type AttributeValue = Value;

/// A DynamoDB item -- map of attribute name to typed value.
pub type Item = HashMap<String, AttributeValue>;

/// Parsed and translated DynamoDB PartiQL operation.
#[derive(Debug, Clone)]
pub enum DdbOperation {
    Select(SelectOp),
    Insert(InsertOp),
    Update(UpdateOp),
    Delete(DeleteOp),
    /// `EXISTS(SELECT … FROM … WHERE …)` — transactions-only condition
    /// check per AWS docs. Returns TRUE iff the inner SELECT yields any
    /// items. Boxed because it embeds a full SelectOp and we want the
    /// other variants to stay compact.
    Exists(Box<SelectOp>),
}

impl DdbOperation {
    /// Returns the table name referenced by this operation.
    pub fn table_name(&self) -> &str {
        match self {
            DdbOperation::Select(op) => &op.table_name,
            DdbOperation::Insert(op) => &op.table_name,
            DdbOperation::Update(op) => &op.table_name,
            DdbOperation::Delete(op) => &op.table_name,
            DdbOperation::Exists(op) => &op.table_name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectOp {
    /// Table name (unquoted).
    pub table_name: String,
    /// Optional index name (from "table"."index" syntax).
    pub index_name: Option<String>,
    /// Projected attributes. None = SELECT *.
    pub projection: Option<Vec<String>>,
    /// WHERE clause conditions.
    pub where_clause: Option<Vec<Condition>>,
    /// ORDER BY direction. None = default (ascending).
    pub order_ascending: Option<bool>,
    /// Attribute to sort by from ORDER BY clause.
    pub order_by_attr: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InsertOp {
    /// Table name.
    pub table_name: String,
    /// The item to insert, as a map of attribute name -> DDB-typed value.
    pub item: Item,
}

#[derive(Debug, Clone)]
pub struct UpdateOp {
    /// Table name.
    pub table_name: String,
    /// WHERE clause conditions (typically key identification).
    pub key_conditions: Vec<Condition>,
    /// SET operations: attribute path -> new value.
    pub sets: Vec<(String, SetValue)>,
    /// REMOVE operations: attribute paths to remove.
    pub removes: Vec<String>,
    /// RETURNING clause, if present.
    pub returning: Option<ReturningClause>,
}

/// The right-hand side of a SET clause in an UPDATE statement.
#[derive(Debug, Clone)]
pub enum SetValue {
    /// `SET col = expr` — covers literal, path reference, and arithmetic.
    /// The previous `Literal`/`Add`/`Sub` variants are subsumed here:
    /// - `SET col = 1` → `Expr(Literal(N=1))`
    /// - `SET col = counter + 1` → `Expr(BinaryOp(Path(counter), Add, Literal(N=1)))`
    /// - `SET col = price + tax` → `Expr(BinaryOp(Path(price), Add, Path(tax)))`
    Expr(Expression),
    /// `list_append(target, value)` — second arg is a literal list/value.
    ListAppend(String, AttributeValue),
    /// `set_add(target, value)`.
    SetAdd(String, AttributeValue),
    /// `set_delete(target, value)`.
    SetDelete(String, AttributeValue),
}

#[derive(Debug, Clone)]
pub struct DeleteOp {
    /// Table name.
    pub table_name: String,
    /// WHERE clause conditions for identifying the item to delete.
    pub key_conditions: Vec<Condition>,
    /// RETURNING clause, if present.
    pub returning: Option<ReturningClause>,
}

/// RETURNING clause from UPDATE/DELETE statements.
#[derive(Debug, Clone, PartialEq)]
pub enum ReturningClause {
    AllNew,
    AllOld,
    ModifiedNew,
    ModifiedOld,
}

/// A six-way comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Arithmetic operators recognised by AWS PartiQL-for-DynamoDB.
/// Per the operators reference, only `+` and `-` are supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithOp {
    Add,
    Sub,
}

impl Expression {
    /// Convenience constructor for a literal expression.
    pub fn lit(v: AttributeValue) -> Self {
        Expression::Literal(v)
    }

    /// Convenience constructor for a path-reference expression.
    pub fn path<S: Into<String>>(s: S) -> Self {
        Expression::Path(s.into())
    }
}

/// An expression that evaluates to a DynamoDB attribute value at runtime.
///
/// Used in any value position: WHERE comparison operands, BETWEEN bounds,
/// IN list elements, function arguments, SET right-hand sides, and the RHS
/// of `size(path) op …`.
///
/// Parenthesised sub-expressions (`(expr)`) are folded directly into the
/// child expression — no wrapper variant is needed at the IR level.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A literal value: string, number, bool, null, missing, list, struct,
    /// or set, all in DynamoDB JSON wire format.
    Literal(AttributeValue),
    /// Reference to an item attribute by path (e.g. `counter`, `Meta.flags`,
    /// `Tags[0]`). Evaluates to the attribute's current value, or NULL if
    /// the attribute is missing.
    Path(String),
    /// Binary arithmetic: `lhs op rhs` where op ∈ {Add, Sub}.
    BinaryOp(Box<Expression>, ArithOp, Box<Expression>),
    /// Unary minus. The parser folds `-numeric_literal` into a single
    /// `Literal(...)` for compactness, so `Neg` only wraps non-literal
    /// expressions in practice.
    Neg(Box<Expression>),
}

impl Expression {
    /// If this expression is a literal, return a reference to its value.
    pub fn as_literal(&self) -> Option<&AttributeValue> {
        match self {
            Expression::Literal(v) => Some(v),
            _ => None,
        }
    }

    /// If this expression is a bare path reference, return its string form.
    pub fn as_path(&self) -> Option<&str> {
        match self {
            Expression::Path(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

/// A single condition from a WHERE clause.
#[derive(Debug, Clone)]
pub enum Condition {
    /// `lhs op rhs` — six-way comparison. Both sides are full expressions
    /// so paths, arithmetic, and literals can appear on either side.
    Compare(Expression, CmpOp, Expression),
    /// `value BETWEEN lo AND hi` — inclusive on both bounds.
    Between(Expression, Expression, Expression),
    /// `value IN [list]` / `value IN <<bag>>`.
    In(Expression, Vec<Expression>),
    /// `begins_with(path, value)`. First arg is constrained to a path.
    BeginsWith(String, Expression),
    /// `contains(path, value)`. First arg is constrained to a path.
    Contains(String, Expression),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
    /// `path IS MISSING`. Negation is `Not(IsMissing(path))`.
    IsMissing(String),
    /// `path IS NULL`. Negation is `Not(IsNull(path))`.
    IsNull(String),
    /// `attribute_type(path, 'TYPE')` — DynamoDB type names: S, N, B, BOOL,
    /// NULL, L, M, SS, NS, BS.
    AttributeType(String, String),
    /// `size(path) op rhs`.
    Size(String, CmpOp, Expression),
}
