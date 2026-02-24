//! Expression parsing — paths, values, arithmetic, WHERE conditions.
//!
//! Two parser entry points produce different shapes:
//!
//! - [`Parser::parse_value`] returns `serde_json::Value` (DDB wire format)
//!   and is used for INSERT VALUE field values, struct/list/bag literal
//!   bodies, and the literal arguments of `list_append` / `set_add` /
//!   `set_delete`. It rejects path references and arithmetic — only
//!   literals (incl. nested struct/list/bag) and `?` placeholders.
//!
//! - [`Parser::parse_expression`] returns an [`Expression`] and is used
//!   anywhere an arithmetic-capable value is allowed: WHERE comparison
//!   operands, BETWEEN bounds, IN list elements, the RHS of SET, and the
//!   RHS of `size(path) op …` and `begins_with`/`contains` second args.
//!
//! ### WHERE precedence (low → high)
//!
//! 1. `OR`  (left-associative)
//! 2. `AND` (left-associative)
//! 3. `NOT` (right-associative prefix)
//! 4. comparison / BETWEEN / IN / IS [NOT] (NULL|MISSING)  (non-associative)
//! 5. binary `+` / `-` (left-associative, **inside** `parse_expression`)
//! 6. unary `-` (right-associative prefix)
//! 7. primary: path, literal, function call, `?`, `( expr )`

use serde_json::{Map, Value, json};

use super::lexer::{Keyword, Token};
use super::{Parser, PartiqlError};
use crate::operation::{ArithOp, CmpOp, Condition, Expression};

impl<'a> Parser<'a> {
    // ── WHERE expression entry ───────────────────────────────────────────────

    /// Parse a top-level WHERE expression and flatten the outermost AND chain
    /// into a `Vec<Condition>` (matching the existing `flatten_and` behaviour).
    pub(crate) fn parse_where_expr(&mut self) -> Result<Vec<Condition>, PartiqlError> {
        let cond = self.parse_or()?;
        Ok(flatten_and(cond))
    }

    fn parse_or(&mut self) -> Result<Condition, PartiqlError> {
        let mut lhs = self.parse_and()?;
        while self.eat_kw(Keyword::Or)? {
            let rhs = self.parse_and()?;
            lhs = Condition::Or(Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_and(&mut self) -> Result<Condition, PartiqlError> {
        let mut lhs = self.parse_not()?;
        while self.eat_kw(Keyword::And)? {
            let rhs = self.parse_not()?;
            lhs = Condition::And(Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_not(&mut self) -> Result<Condition, PartiqlError> {
        if self.eat_kw(Keyword::Not)? {
            let inner = self.parse_not()?;
            Ok(Condition::Not(Box::new(inner)))
        } else {
            self.parse_predicate()
        }
    }

    /// One non-logical predicate: either a function-call condition like
    /// `begins_with(path, val)` / `contains(path, val)` / `attribute_type(...)`
    /// / `size(path) op …`, or `lhs OP rhs` with OP ∈
    /// `{=, !=, <>, <, <=, >, >=, BETWEEN, IN, IS [NOT] (NULL|MISSING)}`.
    fn parse_predicate(&mut self) -> Result<Condition, PartiqlError> {
        // Function-call predicate: Ident immediately followed by `(`.
        // Note: ambiguity with `path + …` (Ident not followed by `(`) is
        // resolved by `peek_ident_with_lparen`.
        if let Some(name) = self.peek_ident_with_lparen()? {
            return self.parse_call_condition(&name);
        }

        // Otherwise: full expression on the LHS, then a comparison /
        // BETWEEN / IN / IS.
        let lhs = self.parse_expression()?;
        let tok = self.bump()?;
        match tok {
            Token::Eq => Ok(Condition::Compare(lhs, CmpOp::Eq, self.parse_expression()?)),
            Token::Ne => Ok(Condition::Compare(lhs, CmpOp::Ne, self.parse_expression()?)),
            Token::Lt => Ok(Condition::Compare(lhs, CmpOp::Lt, self.parse_expression()?)),
            Token::Le => Ok(Condition::Compare(lhs, CmpOp::Le, self.parse_expression()?)),
            Token::Gt => Ok(Condition::Compare(lhs, CmpOp::Gt, self.parse_expression()?)),
            Token::Ge => Ok(Condition::Compare(lhs, CmpOp::Ge, self.parse_expression()?)),
            Token::Kw(Keyword::Between) => {
                let lo = self.parse_expression()?;
                self.expect_kw(Keyword::And)?;
                let hi = self.parse_expression()?;
                Ok(Condition::Between(lhs, lo, hi))
            }
            Token::Kw(Keyword::In_) => {
                let vals = self.parse_expression_collection()?;
                Ok(Condition::In(lhs, vals))
            }
            Token::Kw(Keyword::Is) => {
                // IS predicate requires a path on the LHS — `(price + 1) IS
                // NULL` is semantically ill-defined.
                let path = lhs.as_path().ok_or_else(|| {
                    PartiqlError::Parse(
                        "IS [NOT] (NULL|MISSING) requires a path on the left".into(),
                    )
                })?;
                let path = path.to_string();
                self.parse_is_predicate(path)
            }
            other => Err(PartiqlError::Unsupported(format!(
                "WHERE expression type: {other:?}"
            ))),
        }
    }

    /// Parse the tail of `path IS [NOT] (NULL | MISSING)`. The `IS` token has
    /// already been consumed.
    fn parse_is_predicate(&mut self, path: String) -> Result<Condition, PartiqlError> {
        let negated = self.eat_kw(Keyword::Not)?;
        let kind = self.bump()?;
        let inner = match kind {
            Token::Kw(Keyword::Missing) => Condition::IsMissing(path),
            Token::Kw(Keyword::Null) => Condition::IsNull(path),
            other => {
                return Err(PartiqlError::Parse(format!(
                    "expected NULL or MISSING after IS, got {other:?}"
                )));
            }
        };
        if negated {
            Ok(Condition::Not(Box::new(inner)))
        } else {
            Ok(inner)
        }
    }

    /// If the next two tokens form `Ident LParen`, return the identifier name
    /// with the `(` left in the lookahead queue. Otherwise both tokens stay
    /// queued and we return `None`.
    fn peek_ident_with_lparen(&mut self) -> Result<Option<String>, PartiqlError> {
        if !matches!(self.peek()?, Token::Ident(_)) {
            return Ok(None);
        }
        let Token::Ident(name) = self.bump()? else {
            unreachable!()
        };
        let next = self.bump()?;
        if matches!(&next, Token::LParen) {
            self.buf.push_front(next);
            Ok(Some(name))
        } else {
            self.unbump2(next, Token::Ident(name));
            Ok(None)
        }
    }

    // ── Path parsing ─────────────────────────────────────────────────────────

    /// Parse a path expression and return its rendered form ("a.b", "a[0]",
    /// "a.b[0]", etc.).
    pub(crate) fn parse_path(&mut self) -> Result<String, PartiqlError> {
        let head = self.parse_simple_ident()?;
        let mut parts = vec![head];
        loop {
            match self.peek()? {
                Token::Dot => {
                    self.bump()?;
                    parts.push(self.parse_simple_ident()?);
                }
                Token::LBracket => {
                    self.bump()?;
                    match self.bump()? {
                        Token::Str(s) => parts.push(s),
                        Token::Num(n) => {
                            if n.contains('.') || n.contains('e') || n.contains('E') {
                                return Err(PartiqlError::Unsupported("non-integer index".into()));
                            }
                            let last = parts.len() - 1;
                            parts[last] = format!("{}[{n}]", parts[last]);
                        }
                        other => {
                            return Err(PartiqlError::Unsupported(format!(
                                "complex index expression: {other:?}"
                            )));
                        }
                    }
                    let close = self.bump()?;
                    if !matches!(close, Token::RBracket) {
                        return Err(PartiqlError::Parse(format!(
                            "expected ']' after index, got {close:?}"
                        )));
                    }
                }
                _ => break,
            }
        }
        Ok(parts.join("."))
    }

    fn parse_simple_ident(&mut self) -> Result<String, PartiqlError> {
        match self.bump()? {
            Token::Ident(s) => Ok(s),
            Token::QuotedIdent(s) => Ok(s),
            other => Err(PartiqlError::Parse(format!(
                "expected identifier, got {other:?}"
            ))),
        }
    }

    // ── Expression parsing (arithmetic-capable) ──────────────────────────────

    /// Parse a value expression that may include `+`/`-` arithmetic, path
    /// references, sub-expressions in parentheses, and `?` placeholders. The
    /// result is an [`Expression`] AST node.
    pub(crate) fn parse_expression(&mut self) -> Result<Expression, PartiqlError> {
        self.parse_additive()
    }

    /// Parse `+` / `-` left-associative arithmetic.
    fn parse_additive(&mut self) -> Result<Expression, PartiqlError> {
        let mut lhs = self.parse_unary()?;
        loop {
            let op = match self.peek()? {
                Token::Plus => ArithOp::Add,
                Token::Minus => ArithOp::Sub,
                _ => break,
            };
            self.bump()?;
            let rhs = self.parse_unary()?;
            lhs = Expression::BinaryOp(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    /// Parse unary minus (right-associative). Folds `-numeric_literal` into a
    /// single negated [`Expression::Literal`] for compactness so the AST
    /// shape matches the literal-position parse path.
    fn parse_unary(&mut self) -> Result<Expression, PartiqlError> {
        if matches!(self.peek()?, Token::Minus) {
            self.bump()?;
            // Special-case `-NUM`: produce a single literal with the negative
            // sign baked in. Anything else gets wrapped in `Neg`.
            if matches!(self.peek()?, Token::Num(_)) {
                let n = match self.bump()? {
                    Token::Num(s) => s,
                    _ => unreachable!(),
                };
                return Ok(Expression::Literal(json!({"N": format!("-{n}")})));
            }
            let inner = self.parse_unary()?;
            Ok(Expression::Neg(Box::new(inner)))
        } else {
            self.parse_primary_expression()
        }
    }

    /// Parse a primary expression: parenthesised sub-expression, path, or
    /// literal value. Function calls in the value position would land here
    /// but PartiQL-for-DDB only documents value-producing functions inside
    /// predicates (size, attribute_type), so any `Ident(`...`)` in pure
    /// value position falls through to literal parsing and errors there.
    fn parse_primary_expression(&mut self) -> Result<Expression, PartiqlError> {
        match self.peek()? {
            Token::LParen => {
                self.bump()?;
                let inner = self.parse_expression()?;
                let rp = self.bump()?;
                if !matches!(rp, Token::RParen) {
                    return Err(PartiqlError::Parse(format!(
                        "expected ')' to close sub-expression, got {rp:?}"
                    )));
                }
                Ok(inner)
            }
            Token::Ident(_) | Token::QuotedIdent(_) => Ok(Expression::Path(self.parse_path()?)),
            // Everything else: a literal.
            _ => Ok(Expression::Literal(self.parse_value()?)),
        }
    }

    /// Parse a `[ list ]` or `<< bag >>` of expressions for an `IN` predicate.
    fn parse_expression_collection(&mut self) -> Result<Vec<Expression>, PartiqlError> {
        match self.bump()? {
            Token::LBracket => {
                let mut out = Vec::new();
                if !matches!(self.peek()?, Token::RBracket) {
                    loop {
                        out.push(self.parse_expression()?);
                        if matches!(self.peek()?, Token::Comma) {
                            self.bump()?;
                            continue;
                        }
                        break;
                    }
                }
                let close = self.bump()?;
                if !matches!(close, Token::RBracket) {
                    return Err(PartiqlError::Parse(format!("expected ']', got {close:?}")));
                }
                Ok(out)
            }
            Token::LBag => {
                let mut out = Vec::new();
                if !matches!(self.peek()?, Token::RBag) {
                    loop {
                        out.push(self.parse_expression()?);
                        if matches!(self.peek()?, Token::Comma) {
                            self.bump()?;
                            continue;
                        }
                        break;
                    }
                }
                let close = self.bump()?;
                if !matches!(close, Token::RBag) {
                    return Err(PartiqlError::Parse(format!("expected '>>', got {close:?}")));
                }
                Ok(out)
            }
            other => Err(PartiqlError::Parse(format!(
                "expected list or bag after IN, got {other:?}"
            ))),
        }
    }

    // ── Literal-value parsing (no arithmetic, no paths) ──────────────────────

    /// Parse a literal value: scalar, struct/list/bag, `?` placeholder, or
    /// negative-number literal. Used for INSERT VALUE struct fields, the
    /// literal arguments of `list_append`/`set_add`/`set_delete`, and as a
    /// terminal for [`parse_expression`] when the next token isn't a path or
    /// parenthesised sub-expression.
    pub(crate) fn parse_value(&mut self) -> Result<Value, PartiqlError> {
        let tok = self.bump()?;
        match tok {
            Token::Str(s) => Ok(json!({"S": s})),
            Token::Num(n) => Ok(json!({"N": n})),
            Token::Kw(Keyword::True) => Ok(json!({"BOOL": true})),
            Token::Kw(Keyword::False) => Ok(json!({"BOOL": false})),
            Token::Kw(Keyword::Null) | Token::Kw(Keyword::Missing) => Ok(json!({"NULL": true})),
            Token::QMark => self.params.next(),
            Token::LBrace => self.parse_struct_body(),
            Token::LBracket => self.parse_list_body(),
            Token::LBag => self.parse_bag_body(),
            Token::Minus => {
                // Negative number literal in a literal-only context.
                match self.bump()? {
                    Token::Num(n) => Ok(json!({"N": format!("-{n}")})),
                    other => Err(PartiqlError::InvalidValue(format!(
                        "expected number after '-', got {other:?}"
                    ))),
                }
            }
            other => Err(PartiqlError::InvalidValue(format!(
                "cannot parse value: {other:?}"
            ))),
        }
    }

    /// `{ 'k': v, … }` after the opening `{` has been consumed.
    fn parse_struct_body(&mut self) -> Result<Value, PartiqlError> {
        let mut map = Map::new();
        if !matches!(self.peek()?, Token::RBrace) {
            loop {
                let key = match self.bump()? {
                    Token::Str(s) => s,
                    Token::Ident(s) => s,
                    Token::QuotedIdent(s) => s,
                    other => {
                        return Err(PartiqlError::InvalidValue(format!(
                            "struct key must be string, got {other:?}"
                        )));
                    }
                };
                let colon = self.bump()?;
                if !matches!(colon, Token::Colon) {
                    return Err(PartiqlError::Parse(format!(
                        "expected ':' after struct key, got {colon:?}"
                    )));
                }
                let val = self.parse_value()?;
                map.insert(key, val);
                if matches!(self.peek()?, Token::Comma) {
                    self.bump()?;
                    continue;
                }
                break;
            }
        }
        let close = self.bump()?;
        if !matches!(close, Token::RBrace) {
            return Err(PartiqlError::Parse(format!("expected '}}', got {close:?}")));
        }
        Ok(json!({"M": map}))
    }

    /// `[ v1, v2, … ]` after the opening `[` has been consumed.
    fn parse_list_body(&mut self) -> Result<Value, PartiqlError> {
        let mut items = Vec::new();
        if !matches!(self.peek()?, Token::RBracket) {
            loop {
                items.push(self.parse_value()?);
                if matches!(self.peek()?, Token::Comma) {
                    self.bump()?;
                    continue;
                }
                break;
            }
        }
        let close = self.bump()?;
        if !matches!(close, Token::RBracket) {
            return Err(PartiqlError::Parse(format!("expected ']', got {close:?}")));
        }
        Ok(json!({"L": items}))
    }

    /// `<< v1, v2, … >>` after the opening `<<` has been consumed.
    fn parse_bag_body(&mut self) -> Result<Value, PartiqlError> {
        let mut items = Vec::new();
        if !matches!(self.peek()?, Token::RBag) {
            loop {
                items.push(self.parse_value()?);
                if matches!(self.peek()?, Token::Comma) {
                    self.bump()?;
                    continue;
                }
                break;
            }
        }
        let close = self.bump()?;
        if !matches!(close, Token::RBag) {
            return Err(PartiqlError::Parse(format!("expected '>>', got {close:?}")));
        }
        bag_items_to_set(items)
    }

    // ── Function-call predicates ─────────────────────────────────────────────

    /// Caller has confirmed the next token is `(`. Dispatches by lowercased
    /// function name. begins_with / contains / attribute_type are complete
    /// predicates; size produces a numeric value compared against an
    /// expression on the right.
    fn parse_call_condition(&mut self, name: &str) -> Result<Condition, PartiqlError> {
        let lower = name.to_ascii_lowercase();
        let lp = self.bump()?;
        debug_assert!(matches!(lp, Token::LParen));
        match lower.as_str() {
            "begins_with" => {
                let path = self.parse_path()?;
                self.expect_token(Token::Comma)?;
                let val = self.parse_expression()?;
                self.expect_token(Token::RParen)?;
                Ok(Condition::BeginsWith(path, val))
            }
            "contains" => {
                let path = self.parse_path()?;
                self.expect_token(Token::Comma)?;
                let val = self.parse_expression()?;
                self.expect_token(Token::RParen)?;
                Ok(Condition::Contains(path, val))
            }
            "attribute_type" => {
                let path = self.parse_path()?;
                self.expect_token(Token::Comma)?;
                let type_name = match self.bump()? {
                    Token::Str(s) => s,
                    other => {
                        return Err(PartiqlError::InvalidValue(format!(
                            "attribute_type expects a string type name, got {other:?}"
                        )));
                    }
                };
                self.expect_token(Token::RParen)?;
                Ok(Condition::AttributeType(path, type_name))
            }
            "size" => {
                let path = self.parse_path()?;
                self.expect_token(Token::RParen)?;
                let op = match self.bump()? {
                    Token::Eq => CmpOp::Eq,
                    Token::Ne => CmpOp::Ne,
                    Token::Lt => CmpOp::Lt,
                    Token::Le => CmpOp::Le,
                    Token::Gt => CmpOp::Gt,
                    Token::Ge => CmpOp::Ge,
                    other => {
                        return Err(PartiqlError::Parse(format!(
                            "expected comparison after size(...), got {other:?}"
                        )));
                    }
                };
                let val = self.parse_expression()?;
                Ok(Condition::Size(path, op, val))
            }
            other => Err(PartiqlError::Unsupported(format!(
                "function in WHERE: {other}"
            ))),
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), PartiqlError> {
        let tok = self.bump()?;
        if std::mem::discriminant(&tok) == std::mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(PartiqlError::Parse(format!(
                "expected {expected:?}, got {tok:?}"
            )))
        }
    }
}

fn flatten_and(cond: Condition) -> Vec<Condition> {
    match cond {
        Condition::And(lhs, rhs) => {
            let mut out = flatten_and(*lhs);
            out.extend(flatten_and(*rhs));
            out
        }
        other => vec![other],
    }
}

fn bag_items_to_set(items: Vec<Value>) -> Result<Value, PartiqlError> {
    if items.iter().all(|v| v.get("S").is_some()) {
        let ss: Vec<String> = items
            .iter()
            .filter_map(|v| v.get("S").and_then(|s| s.as_str()).map(String::from))
            .collect();
        Ok(json!({"SS": ss}))
    } else if items.iter().all(|v| v.get("N").is_some()) {
        let ns: Vec<String> = items
            .iter()
            .filter_map(|v| v.get("N").and_then(|s| s.as_str()).map(String::from))
            .collect();
        Ok(json!({"NS": ns}))
    } else {
        Err(PartiqlError::InvalidValue(
            "set must contain all strings or all numbers".into(),
        ))
    }
}
