//! DynamoDB expression parser and evaluator.
//!
//! Supports ConditionExpression (PutItem, UpdateItem, DeleteItem) and
//! FilterExpression (Query, Scan).  All `#name` and `:value` placeholders
//! are resolved at parse time so the resulting AST contains only concrete
//! attribute names and DynamoDB-typed JSON values.

use std::borrow::Cow;
use std::collections::HashMap;

use crate::types::{AttributeValue, Item};

// ---------------------------------------------------------------------------
// AST
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum Expr {
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Comparison(Operand, CompOp, Operand),
    Between(Operand, Operand, Operand), // val BETWEEN lo AND hi
    In(Operand, Vec<Operand>),          // val IN (a, b, ...)
    AttributeExists(String),
    AttributeNotExists(String),
    AttributeType(String, Operand),
    Contains(String, Operand),
    BeginsWith(String, Operand),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug)]
pub enum Operand {
    Path(String),
    Value(AttributeValue),
    Size(String),
}

// ---------------------------------------------------------------------------
// Tokeniser
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),     // bare word, uppercased keyword or attribute name
    AttrName(String),  // #foo  (kept as-is for lookup)
    AttrValue(String), // :foo  (kept as-is for lookup)
    LParen,
    RParen,
    Comma,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' | '\r' => {
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            '=' => {
                tokens.push(Token::Eq);
                i += 1;
            }
            '<' => {
                if i + 1 < chars.len() && chars[i + 1] == '>' {
                    tokens.push(Token::Ne);
                    i += 2;
                } else if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::Le);
                    i += 2;
                } else {
                    tokens.push(Token::Lt);
                    i += 1;
                }
            }
            '>' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::Ge);
                    i += 2;
                } else {
                    tokens.push(Token::Gt);
                    i += 1;
                }
            }
            '#' => {
                let start = i;
                i += 1;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::AttrName(s));
            }
            ':' => {
                let start = i;
                i += 1;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::AttrValue(s));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                tokens.push(Token::Ident(word));
            }
            other => {
                return Err(format!("Unexpected character in expression: '{other}'"));
            }
        }
    }
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    expr_names: &'a HashMap<String, String>,
    expr_values: &'a HashMap<String, AttributeValue>,
}

impl<'a> Parser<'a> {
    fn new(
        tokens: Vec<Token>,
        expr_names: &'a HashMap<String, String>,
        expr_values: &'a HashMap<String, AttributeValue>,
    ) -> Self {
        Self {
            tokens,
            pos: 0,
            expr_names,
            expr_values,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek2(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn consume(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn expect_ident(&mut self, word: &str) -> Result<(), String> {
        match self.consume() {
            Some(Token::Ident(ref s)) if s.eq_ignore_ascii_case(word) => Ok(()),
            other => Err(format!("Expected '{word}', got {other:?}")),
        }
    }

    fn expect_lparen(&mut self) -> Result<(), String> {
        match self.consume() {
            Some(Token::LParen) => Ok(()),
            other => Err(format!("Expected '(', got {other:?}")),
        }
    }

    fn expect_rparen(&mut self) -> Result<(), String> {
        match self.consume() {
            Some(Token::RParen) => Ok(()),
            other => Err(format!("Expected ')', got {other:?}")),
        }
    }

    fn resolve_name(&self, placeholder: &str) -> Result<String, String> {
        self.expr_names
            .get(placeholder)
            .cloned()
            .ok_or_else(|| format!("ExpressionAttributeNames has no entry for '{placeholder}'"))
    }

    fn resolve_value(&self, placeholder: &str) -> Result<AttributeValue, String> {
        self.expr_values
            .get(placeholder)
            .cloned()
            .ok_or_else(|| format!("ExpressionAttributeValues has no entry for '{placeholder}'"))
    }

    // ------------------------------------------------------------------
    // Grammar productions
    // ------------------------------------------------------------------

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), Some(Token::Ident(s)) if s.eq_ignore_ascii_case("OR")) {
            self.consume();
            let right = self.parse_and()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_not()?;
        while matches!(self.peek(), Some(Token::Ident(s)) if s.eq_ignore_ascii_case("AND")) {
            self.consume();
            let right = self.parse_not()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), Some(Token::Ident(s)) if s.eq_ignore_ascii_case("NOT")) {
            self.consume();
            let inner = self.parse_not()?;
            Ok(Expr::Not(Box::new(inner)))
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        // Parenthesised expression
        if matches!(self.peek(), Some(Token::LParen)) {
            self.consume(); // (
            let inner = self.parse_expr()?;
            self.expect_rparen()?;
            return Ok(inner);
        }

        // Check for function calls that start with a known function name
        if let Some(Token::Ident(name)) = self.peek().cloned() {
            match name.to_lowercase().as_str() {
                "attribute_exists" => {
                    self.consume();
                    self.expect_lparen()?;
                    let attr = self.parse_attr_path()?;
                    self.expect_rparen()?;
                    return Ok(Expr::AttributeExists(attr));
                }
                "attribute_not_exists" => {
                    self.consume();
                    self.expect_lparen()?;
                    let attr = self.parse_attr_path()?;
                    self.expect_rparen()?;
                    return Ok(Expr::AttributeNotExists(attr));
                }
                "attribute_type" => {
                    self.consume();
                    self.expect_lparen()?;
                    let attr = self.parse_attr_path()?;
                    match self.consume() {
                        Some(Token::Comma) => {}
                        other => {
                            return Err(format!("Expected ',' in attribute_type, got {other:?}"));
                        }
                    }
                    let type_op = self.parse_operand()?;
                    self.expect_rparen()?;
                    return Ok(Expr::AttributeType(attr, type_op));
                }
                "contains" => {
                    self.consume();
                    self.expect_lparen()?;
                    let attr = self.parse_attr_path()?;
                    match self.consume() {
                        Some(Token::Comma) => {}
                        other => return Err(format!("Expected ',' in contains, got {other:?}")),
                    }
                    let operand = self.parse_operand()?;
                    self.expect_rparen()?;
                    return Ok(Expr::Contains(attr, operand));
                }
                "begins_with" => {
                    self.consume();
                    self.expect_lparen()?;
                    let attr = self.parse_attr_path()?;
                    match self.consume() {
                        Some(Token::Comma) => {}
                        other => return Err(format!("Expected ',' in begins_with, got {other:?}")),
                    }
                    let operand = self.parse_operand()?;
                    self.expect_rparen()?;
                    return Ok(Expr::BeginsWith(attr, operand));
                }
                _ => {} // fall through to comparison/between/in
            }
        }

        // Operand followed by comparison, BETWEEN, or IN
        let lhs = self.parse_operand()?;

        match self.peek() {
            Some(Token::Eq) => {
                self.consume();
                let rhs = self.parse_operand()?;
                Ok(Expr::Comparison(lhs, CompOp::Eq, rhs))
            }
            Some(Token::Ne) => {
                self.consume();
                let rhs = self.parse_operand()?;
                Ok(Expr::Comparison(lhs, CompOp::Ne, rhs))
            }
            Some(Token::Lt) => {
                self.consume();
                let rhs = self.parse_operand()?;
                Ok(Expr::Comparison(lhs, CompOp::Lt, rhs))
            }
            Some(Token::Le) => {
                self.consume();
                let rhs = self.parse_operand()?;
                Ok(Expr::Comparison(lhs, CompOp::Le, rhs))
            }
            Some(Token::Gt) => {
                self.consume();
                let rhs = self.parse_operand()?;
                Ok(Expr::Comparison(lhs, CompOp::Gt, rhs))
            }
            Some(Token::Ge) => {
                self.consume();
                let rhs = self.parse_operand()?;
                Ok(Expr::Comparison(lhs, CompOp::Ge, rhs))
            }
            Some(Token::Ident(s)) if s.eq_ignore_ascii_case("BETWEEN") => {
                self.consume();
                let lo = self.parse_operand()?;
                self.expect_ident("AND")?;
                let hi = self.parse_operand()?;
                Ok(Expr::Between(lhs, lo, hi))
            }
            Some(Token::Ident(s)) if s.eq_ignore_ascii_case("IN") => {
                self.consume();
                self.expect_lparen()?;
                let mut values = vec![self.parse_operand()?];
                while matches!(self.peek(), Some(Token::Comma)) {
                    self.consume();
                    values.push(self.parse_operand()?);
                }
                self.expect_rparen()?;
                Ok(Expr::In(lhs, values))
            }
            other => Err(format!(
                "Expected comparison operator, BETWEEN, or IN after operand; got {other:?}"
            )),
        }
    }

    /// Parse an attribute path (may be `#name` or bare `Ident`).
    fn parse_attr_path(&mut self) -> Result<String, String> {
        match self.consume() {
            Some(Token::AttrName(placeholder)) => self.resolve_name(&placeholder),
            Some(Token::Ident(name)) => Ok(name),
            other => Err(format!("Expected attribute name, got {other:?}")),
        }
    }

    /// Parse an operand: `#name`, `:value`, bare `Ident` (attribute name), or `size(attr)`.
    fn parse_operand(&mut self) -> Result<Operand, String> {
        match self.peek().cloned() {
            Some(Token::Ident(ref name)) if name.to_lowercase() == "size" => {
                // Check that the next token after "size" is "("
                if matches!(self.peek2(), Some(Token::LParen)) {
                    self.consume(); // size
                    self.consume(); // (
                    let attr = self.parse_attr_path()?;
                    self.expect_rparen()?;
                    return Ok(Operand::Size(attr));
                }
                // Otherwise treat as a bare attribute name
                self.consume();
                Ok(Operand::Path(name.clone()))
            }
            Some(Token::AttrName(placeholder)) => {
                self.consume();
                let name = self.resolve_name(&placeholder)?;
                Ok(Operand::Path(name))
            }
            Some(Token::AttrValue(placeholder)) => {
                self.consume();
                let val = self.resolve_value(&placeholder)?;
                Ok(Operand::Value(val))
            }
            Some(Token::Ident(name)) => {
                self.consume();
                Ok(Operand::Path(name))
            }
            other => Err(format!("Expected operand, got {other:?}")),
        }
    }
}

// ---------------------------------------------------------------------------
// Public parse entry point
// ---------------------------------------------------------------------------

pub fn parse_expression(
    expr: &str,
    expr_names: &HashMap<String, String>,
    expr_values: &HashMap<String, AttributeValue>,
) -> Result<Expr, String> {
    let tokens = tokenize(expr)?;
    let mut parser = Parser::new(tokens, expr_names, expr_values);
    let result = parser.parse_expr()?;
    if parser.pos < parser.tokens.len() {
        return Err(format!(
            "Unexpected tokens after expression: {:?}",
            &parser.tokens[parser.pos..]
        ));
    }
    Ok(result)
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

fn get_attr<'a>(item: &'a Item, name: &str) -> Option<&'a AttributeValue> {
    item.get(name)
}

fn compute_size(val: &AttributeValue) -> Option<usize> {
    match val {
        AttributeValue::S(s) => Some(s.len()),
        AttributeValue::B(b) => Some(b.len() * 3 / 4), // Approximate base64 decoded length
        AttributeValue::SS(ss) => Some(ss.len()),
        AttributeValue::NS(ns) => Some(ns.len()),
        AttributeValue::BS(bs) => Some(bs.len()),
        AttributeValue::L(l) => Some(l.len()),
        AttributeValue::M(m) => Some(m.len()),
        _ => None,
    }
}

fn resolve_operand<'a>(op: &'a Operand, item: &'a Item) -> Option<Cow<'a, AttributeValue>> {
    match op {
        Operand::Path(name) => get_attr(item, name).map(Cow::Borrowed),
        Operand::Value(v) => Some(Cow::Borrowed(v)),
        Operand::Size(name) => {
            let attr_val = get_attr(item, name)?;
            let sz = compute_size(attr_val)?;
            Some(Cow::Owned(AttributeValue::N(sz.to_string())))
        }
    }
}

/// Compare two DynamoDB-typed AttributeValues.
/// Returns None when types are incompatible (comparison yields false).
fn compare_ddb(a: &AttributeValue, b: &AttributeValue) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (AttributeValue::S(sa), AttributeValue::S(sb)) => Some(sa.cmp(sb)),
        (AttributeValue::N(na), AttributeValue::N(nb)) => {
            let fa: f64 = na.parse().ok()?;
            let fb: f64 = nb.parse().ok()?;
            fa.partial_cmp(&fb)
        }
        (AttributeValue::Bool(ba), AttributeValue::Bool(bb)) => {
            let ia = if *ba { 1u8 } else { 0 };
            let ib = if *bb { 1u8 } else { 0 };
            Some(ia.cmp(&ib))
        }
        (AttributeValue::Null(_), AttributeValue::Null(_)) => Some(std::cmp::Ordering::Equal),
        _ => None,
    }
}

/// Return the DynamoDB type string ("S", "N", "BOOL", etc.) for a typed value.
fn ddb_type(val: &AttributeValue) -> Option<&'static str> {
    match val {
        AttributeValue::S(_) => Some("S"),
        AttributeValue::N(_) => Some("N"),
        AttributeValue::B(_) => Some("B"),
        AttributeValue::Bool(_) => Some("BOOL"),
        AttributeValue::Null(_) => Some("NULL"),
        AttributeValue::SS(_) => Some("SS"),
        AttributeValue::NS(_) => Some("NS"),
        AttributeValue::BS(_) => Some("BS"),
        AttributeValue::L(_) => Some("L"),
        AttributeValue::M(_) => Some("M"),
    }
}

/// Evaluate a parsed expression against an item.
pub fn evaluate(expr: &Expr, item: &Item) -> bool {
    match expr {
        Expr::And(a, b) => evaluate(a, item) && evaluate(b, item),
        Expr::Or(a, b) => evaluate(a, item) || evaluate(b, item),
        Expr::Not(e) => !evaluate(e, item),

        Expr::Comparison(lhs, op, rhs) => {
            let lv = match resolve_operand(lhs, item) {
                Some(v) => v,
                None => return false,
            };
            let rv = match resolve_operand(rhs, item) {
                Some(v) => v,
                None => return false,
            };
            match compare_ddb(&lv, &rv) {
                Some(ord) => match op {
                    CompOp::Eq => ord == std::cmp::Ordering::Equal,
                    CompOp::Ne => ord != std::cmp::Ordering::Equal,
                    CompOp::Lt => ord == std::cmp::Ordering::Less,
                    CompOp::Le => ord != std::cmp::Ordering::Greater,
                    CompOp::Gt => ord == std::cmp::Ordering::Greater,
                    CompOp::Ge => ord != std::cmp::Ordering::Less,
                },
                None => false,
            }
        }

        Expr::Between(val_op, lo_op, hi_op) => {
            let v = match resolve_operand(val_op, item) {
                Some(v) => v,
                None => return false,
            };
            let lo = match resolve_operand(lo_op, item) {
                Some(v) => v,
                None => return false,
            };
            let hi = match resolve_operand(hi_op, item) {
                Some(v) => v,
                None => return false,
            };
            let ge_lo = compare_ddb(&v, &lo)
                .map(|o| o != std::cmp::Ordering::Less)
                .unwrap_or(false);
            let le_hi = compare_ddb(&v, &hi)
                .map(|o| o != std::cmp::Ordering::Greater)
                .unwrap_or(false);
            ge_lo && le_hi
        }

        Expr::In(val_op, candidates) => {
            let v = match resolve_operand(val_op, item) {
                Some(v) => v,
                None => return false,
            };
            candidates.iter().any(|c| {
                resolve_operand(c, item)
                    .and_then(|cv| compare_ddb(&v, &cv))
                    .map(|o| o == std::cmp::Ordering::Equal)
                    .unwrap_or(false)
            })
        }

        Expr::AttributeExists(name) => get_attr(item, name).is_some(),

        Expr::AttributeNotExists(name) => get_attr(item, name).is_none(),

        Expr::AttributeType(name, type_op) => {
            let attr_val = match get_attr(item, name) {
                Some(v) => v,
                None => return false,
            };
            let actual_type = match ddb_type(attr_val) {
                Some(t) => t,
                None => return false,
            };
            // The type operand is expected to be an AttributeValue::S("typename")
            if let Operand::Value(AttributeValue::S(expected)) = type_op {
                return actual_type == expected.as_str();
            }
            false
        }

        Expr::Contains(name, substr_op) => {
            let attr_val = match get_attr(item, name) {
                Some(v) => v,
                None => return false,
            };
            // String contains substring
            if let AttributeValue::S(s) = attr_val {
                if let Some(sub) = resolve_operand(substr_op, item) {
                    if let AttributeValue::S(sub_s) = sub.as_ref() {
                        return s.contains(sub_s.as_str());
                    }
                }
                return false;
            }
            // List contains element (equality check)
            if let AttributeValue::L(list) = attr_val {
                if let Some(needle) = resolve_operand(substr_op, item) {
                    return list.iter().any(|elem| {
                        compare_ddb(elem, needle.as_ref())
                            .map(|o| o == std::cmp::Ordering::Equal)
                            .unwrap_or(false)
                    });
                }
            }
            false
        }

        Expr::BeginsWith(name, prefix_op) => {
            let attr_val = match get_attr(item, name) {
                Some(v) => v,
                None => return false,
            };
            if let AttributeValue::S(s) = attr_val {
                if let Some(prefix) = resolve_operand(prefix_op, item) {
                    if let AttributeValue::S(prefix_s) = prefix.as_ref() {
                        return s.starts_with(prefix_s.as_str());
                    }
                }
            }
            false
        }
    }
}

// ---------------------------------------------------------------------------
// Update-expression parsing and application
// ---------------------------------------------------------------------------

use crate::types::UpdateAction;

/// Parse an `UpdateExpression` plus its `ExpressionAttributeNames` /
/// `ExpressionAttributeValues` into a list of [`UpdateAction`]s.
///
/// Recognises:
/// - `SET p = :v`, `SET p = p + :delta`, `SET p = p - :delta`
/// - `SET p = list_append(p, :v)` and `SET p = if_not_exists(p, :v)`
///   (same-path forms only)
/// - `SET nested.path = :v` (dotted paths, with each segment optionally
///   resolved through `ExpressionAttributeNames`)
/// - `REMOVE p, q.r`
/// - `ADD attr :v` (numeric or set, polymorphic at apply time)
/// - `DELETE attr :v` (set difference)
///
/// Unrecognised fragments are silently dropped — matching the previous
/// implementation's behaviour and keeping malformed inputs from breaking
/// the request.
pub fn parse_update_expression(
    expr: &str,
    expr_names: &HashMap<String, String>,
    expr_values: &HashMap<String, AttributeValue>,
) -> Vec<UpdateAction> {
    let mut actions: Vec<UpdateAction> = Vec::new();

    let upper = expr.to_uppercase();
    let mut sections: Vec<(&str, usize)> = Vec::new();
    for keyword in &["SET ", "REMOVE ", "ADD ", "DELETE "] {
        let mut search_from = 0;
        while let Some(pos) = upper[search_from..].find(keyword) {
            let abs_pos = search_from + pos;
            if abs_pos == 0
                || expr.as_bytes()[abs_pos - 1] == b' '
                || expr.as_bytes()[abs_pos - 1] == b','
            {
                sections.push((keyword.trim(), abs_pos));
            }
            search_from = abs_pos + keyword.len();
        }
    }
    sections.sort_by_key(|&(_, pos)| pos);

    for (i, &(keyword, start)) in sections.iter().enumerate() {
        let content_start = start + keyword.len() + 1;
        let content_end = if i + 1 < sections.len() {
            sections[i + 1].1
        } else {
            expr.len()
        };
        let content = expr[content_start..content_end].trim();

        match keyword {
            "SET" => {
                for assignment in split_top_level(content) {
                    if let Some(action) = parse_set_assignment(&assignment, expr_names, expr_values)
                    {
                        actions.push(action);
                    }
                }
            }
            "REMOVE" => {
                for attr in content.split(',') {
                    let attr = attr.trim();
                    if attr.is_empty() {
                        continue;
                    }
                    let path = resolve_path(attr, expr_names);
                    if !path.is_empty() {
                        actions.push(UpdateAction::Remove(path));
                    }
                }
            }
            "ADD" => {
                for term in content.split(',') {
                    let term = term.trim();
                    if let Some((lhs, rhs)) = term.split_once(' ') {
                        let path = resolve_path(lhs.trim(), expr_names);
                        if path.len() != 1 {
                            continue; // ADD only supports top-level attributes
                        }
                        let rhs = rhs.trim();
                        if rhs.starts_with(':')
                            && let Some(val) = expr_values.get(rhs)
                        {
                            actions.push(UpdateAction::Add(
                                path.into_iter().next().unwrap(),
                                val.clone(),
                            ));
                        }
                    }
                }
            }
            "DELETE" => {
                for term in content.split(',') {
                    let term = term.trim();
                    if let Some((lhs, rhs)) = term.split_once(' ') {
                        let path = resolve_path(lhs.trim(), expr_names);
                        if path.len() != 1 {
                            continue;
                        }
                        let rhs = rhs.trim();
                        if rhs.starts_with(':')
                            && let Some(val) = expr_values.get(rhs)
                        {
                            actions.push(UpdateAction::Delete(
                                path.into_iter().next().unwrap(),
                                val.clone(),
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    actions
}

/// Split `content` on commas that are *not* nested inside parentheses, so
/// `list_append(a, :b), c = :d` splits into two assignments rather than
/// breaking the function call.
fn split_top_level(content: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut buf = String::new();
    let mut depth: i32 = 0;
    for ch in content.chars() {
        match ch {
            '(' => {
                depth += 1;
                buf.push(ch);
            }
            ')' => {
                depth -= 1;
                buf.push(ch);
            }
            ',' if depth == 0 => {
                if !buf.trim().is_empty() {
                    out.push(buf.trim().to_string());
                }
                buf.clear();
            }
            _ => buf.push(ch),
        }
    }
    if !buf.trim().is_empty() {
        out.push(buf.trim().to_string());
    }
    out
}

fn parse_set_assignment(
    assignment: &str,
    expr_names: &HashMap<String, String>,
    expr_values: &HashMap<String, AttributeValue>,
) -> Option<UpdateAction> {
    let (lhs, rhs) = assignment.split_once('=')?;
    let lhs = lhs.trim();
    let rhs = rhs.trim();
    let path = resolve_path(lhs, expr_names);
    if path.is_empty() {
        return None;
    }

    // list_append(path, :v)
    if let Some(args) = rhs
        .strip_prefix("list_append(")
        .and_then(|s| s.strip_suffix(')'))
    {
        let parts: Vec<&str> = args.splitn(2, ',').collect();
        if parts.len() == 2 {
            // Same-path form: list_append(<path>, :v).
            let src_path = resolve_path(parts[0].trim(), expr_names);
            let v_ref = parts[1].trim();
            if src_path == path
                && v_ref.starts_with(':')
                && let Some(val) = expr_values.get(v_ref)
            {
                return Some(UpdateAction::SetListAppend {
                    path,
                    value: val.clone(),
                });
            }
        }
        return None;
    }

    // if_not_exists(path, :v)
    if let Some(args) = rhs
        .strip_prefix("if_not_exists(")
        .and_then(|s| s.strip_suffix(')'))
    {
        let parts: Vec<&str> = args.splitn(2, ',').collect();
        if parts.len() == 2 {
            let src_path = resolve_path(parts[0].trim(), expr_names);
            let v_ref = parts[1].trim();
            if src_path == path
                && v_ref.starts_with(':')
                && let Some(val) = expr_values.get(v_ref)
            {
                return Some(UpdateAction::SetIfNotExists {
                    path,
                    value: val.clone(),
                });
            }
        }
        return None;
    }

    // Arithmetic: <path> + :delta or <path> - :delta. Look for the operator
    // that splits a `<path-on-left>` from a `<value-on-right>`; only `+`/`-`
    // between the same path and a placeholder constitute atomic ADD.
    if let Some(pos) = find_arith_op(rhs) {
        let op = rhs.as_bytes()[pos] as char;
        let left = rhs[..pos].trim();
        let right = rhs[pos + 1..].trim();
        let left_path = resolve_path(left, expr_names);
        if left_path == path
            && right.starts_with(':')
            && let Some(val) = expr_values.get(right)
        {
            let mut delta = val.clone();
            if op == '-'
                && let AttributeValue::N(ref s) = delta
            {
                let n: f64 = s.parse().unwrap_or(0.0);
                delta = AttributeValue::N(format_number(-n));
            }
            return Some(UpdateAction::SetArithmetic { path, delta });
        }
    }

    // Plain value: SET <path> = :v
    if rhs.starts_with(':')
        && let Some(val) = expr_values.get(rhs)
    {
        return Some(UpdateAction::SetValue {
            path,
            value: val.clone(),
        });
    }

    None
}

/// Find the byte offset of an arithmetic operator (`+` or `-`) at the top
/// level of `rhs`, returning `None` when the right-hand side is not an
/// arithmetic expression. Operators inside parentheses are skipped.
fn find_arith_op(rhs: &str) -> Option<usize> {
    let mut depth: i32 = 0;
    for (i, b) in rhs.bytes().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'+' | b'-' if depth == 0 && i > 0 => return Some(i),
            _ => {}
        }
    }
    None
}

/// Resolve a possibly dotted path like `info.city` or `#i.#c.foo`,
/// substituting `ExpressionAttributeNames` aliases for any segment that
/// starts with `#`. Empty segments (e.g. from a stray dot) are dropped.
fn resolve_path(raw: &str, expr_names: &HashMap<String, String>) -> Vec<String> {
    raw.split('.')
        .map(|seg| seg.trim())
        .filter(|seg| !seg.is_empty())
        .map(|seg| {
            if let Some(alias) = seg.strip_prefix('#')
                && let Some(target) = expr_names.get(&format!("#{alias}"))
            {
                target.clone()
            } else {
                seg.to_string()
            }
        })
        .collect()
}

/// Format a number the way DynamoDB does: integer-valued numbers as
/// integers, others via the default `f64` formatter.
fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

/// Apply a list of [`UpdateAction`]s to `item` in order.
///
/// The semantics here mirror the AWS DynamoDB UpdateItem behaviour for
/// the action variants we recognise — including dotted-path SET, set ADD /
/// DELETE on `SS`/`NS`/`BS`, `list_append`, and `if_not_exists` — so
/// callers get the same result regardless of which backend they go
/// through.
pub fn apply_update_actions(item: &mut Item, actions: &[UpdateAction]) {
    for action in actions {
        match action {
            UpdateAction::SetValue { path, value } => {
                set_at_path(item, path, value.clone());
            }
            UpdateAction::SetArithmetic { path, delta } => {
                let current = get_at_path(item, path);
                let cur_n = match current {
                    Some(AttributeValue::N(s)) => s.parse::<f64>().unwrap_or(0.0),
                    _ => 0.0,
                };
                let delta_n = match delta {
                    AttributeValue::N(s) => s.parse::<f64>().unwrap_or(0.0),
                    _ => 0.0,
                };
                set_at_path(
                    item,
                    path,
                    AttributeValue::N(format_number(cur_n + delta_n)),
                );
            }
            UpdateAction::SetListAppend { path, value } => {
                let mut list = match get_at_path(item, path).cloned() {
                    Some(AttributeValue::L(l)) => l,
                    _ => Vec::new(),
                };
                match value {
                    AttributeValue::L(extra) => list.extend(extra.iter().cloned()),
                    other => list.push(other.clone()),
                }
                set_at_path(item, path, AttributeValue::L(list));
            }
            UpdateAction::SetIfNotExists { path, value } => {
                if get_at_path(item, path).is_none() {
                    set_at_path(item, path, value.clone());
                }
            }
            UpdateAction::Remove(path) => {
                remove_at_path(item, path);
            }
            UpdateAction::Add(attr, delta) => {
                apply_add(item, attr, delta);
            }
            UpdateAction::Delete(attr, members) => {
                apply_set_delete(item, attr, members);
            }
        }
    }
}

fn get_at_path<'a>(item: &'a Item, path: &[String]) -> Option<&'a AttributeValue> {
    let mut cur = item.get(&path[0])?;
    for seg in &path[1..] {
        match cur {
            AttributeValue::M(map) => cur = map.get(seg)?,
            _ => return None,
        }
    }
    Some(cur)
}

fn set_at_path(item: &mut Item, path: &[String], value: AttributeValue) {
    if path.len() == 1 {
        item.insert(path[0].clone(), value);
        return;
    }
    let head = &path[0];
    let entry = item
        .entry(head.clone())
        .or_insert_with(|| AttributeValue::M(std::collections::HashMap::new()));
    if !matches!(entry, AttributeValue::M(_)) {
        *entry = AttributeValue::M(std::collections::HashMap::new());
    }
    if let AttributeValue::M(map) = entry {
        set_in_map(map, &path[1..], value);
    }
}

fn set_in_map(
    map: &mut std::collections::HashMap<String, AttributeValue>,
    path: &[String],
    value: AttributeValue,
) {
    if path.len() == 1 {
        map.insert(path[0].clone(), value);
        return;
    }
    let head = &path[0];
    let entry = map
        .entry(head.clone())
        .or_insert_with(|| AttributeValue::M(std::collections::HashMap::new()));
    if !matches!(entry, AttributeValue::M(_)) {
        *entry = AttributeValue::M(std::collections::HashMap::new());
    }
    if let AttributeValue::M(inner) = entry {
        set_in_map(inner, &path[1..], value);
    }
}

fn remove_at_path(item: &mut Item, path: &[String]) {
    if path.len() == 1 {
        item.remove(&path[0]);
        return;
    }
    if let Some(AttributeValue::M(map)) = item.get_mut(&path[0]) {
        remove_in_map(map, &path[1..]);
    }
}

fn remove_in_map(map: &mut std::collections::HashMap<String, AttributeValue>, path: &[String]) {
    if path.len() == 1 {
        map.remove(&path[0]);
        return;
    }
    if let Some(AttributeValue::M(inner)) = map.get_mut(&path[0]) {
        remove_in_map(inner, &path[1..]);
    }
}

fn apply_add(item: &mut Item, attr: &str, delta: &AttributeValue) {
    match (item.get(attr), delta) {
        (Some(AttributeValue::SS(cur)), AttributeValue::SS(extra)) => {
            let mut merged: Vec<String> = cur.clone();
            for v in extra {
                if !merged.contains(v) {
                    merged.push(v.clone());
                }
            }
            item.insert(attr.to_string(), AttributeValue::SS(merged));
        }
        (None, AttributeValue::SS(extra)) => {
            let mut merged: Vec<String> = Vec::new();
            for v in extra {
                if !merged.contains(v) {
                    merged.push(v.clone());
                }
            }
            item.insert(attr.to_string(), AttributeValue::SS(merged));
        }
        (Some(AttributeValue::NS(cur)), AttributeValue::NS(extra)) => {
            let mut merged: Vec<String> = cur.clone();
            for v in extra {
                if !merged.contains(v) {
                    merged.push(v.clone());
                }
            }
            item.insert(attr.to_string(), AttributeValue::NS(merged));
        }
        (None, AttributeValue::NS(extra)) => {
            let mut merged: Vec<String> = Vec::new();
            for v in extra {
                if !merged.contains(v) {
                    merged.push(v.clone());
                }
            }
            item.insert(attr.to_string(), AttributeValue::NS(merged));
        }
        (Some(AttributeValue::BS(cur)), AttributeValue::BS(extra)) => {
            let mut merged: Vec<String> = cur.clone();
            for v in extra {
                if !merged.contains(v) {
                    merged.push(v.clone());
                }
            }
            item.insert(attr.to_string(), AttributeValue::BS(merged));
        }
        (None, AttributeValue::BS(extra)) => {
            let mut merged: Vec<String> = Vec::new();
            for v in extra {
                if !merged.contains(v) {
                    merged.push(v.clone());
                }
            }
            item.insert(attr.to_string(), AttributeValue::BS(merged));
        }
        // Otherwise treat as numeric (matches the prior behaviour for `N`
        // and the `numeric ADD on missing attr` case).
        _ => {
            let cur_n = match item.get(attr) {
                Some(AttributeValue::N(s)) => s.parse::<f64>().unwrap_or(0.0),
                _ => 0.0,
            };
            let delta_n = match delta {
                AttributeValue::N(s) => s.parse::<f64>().unwrap_or(0.0),
                _ => 0.0,
            };
            item.insert(
                attr.to_string(),
                AttributeValue::N(format_number(cur_n + delta_n)),
            );
        }
    }
}

fn apply_set_delete(item: &mut Item, attr: &str, members: &AttributeValue) {
    let new_value = match (item.get(attr), members) {
        (Some(AttributeValue::SS(cur)), AttributeValue::SS(rm)) => {
            let kept: Vec<String> = cur.iter().filter(|v| !rm.contains(v)).cloned().collect();
            if kept.is_empty() {
                None
            } else {
                Some(AttributeValue::SS(kept))
            }
        }
        (Some(AttributeValue::NS(cur)), AttributeValue::NS(rm)) => {
            let kept: Vec<String> = cur.iter().filter(|v| !rm.contains(v)).cloned().collect();
            if kept.is_empty() {
                None
            } else {
                Some(AttributeValue::NS(kept))
            }
        }
        (Some(AttributeValue::BS(cur)), AttributeValue::BS(rm)) => {
            let kept: Vec<String> = cur.iter().filter(|v| !rm.contains(v)).cloned().collect();
            if kept.is_empty() {
                None
            } else {
                Some(AttributeValue::BS(kept))
            }
        }
        _ => return,
    };
    match new_value {
        Some(v) => {
            item.insert(attr.to_string(), v);
        }
        None => {
            item.remove(attr);
        }
    }
}

// ---------------------------------------------------------------------------
// ProjectionExpression
// ---------------------------------------------------------------------------

/// Parse a `ProjectionExpression` like `"tags, info.city, #a.#b"` plus its
/// `ExpressionAttributeNames` map into a list of attribute paths. Empty
/// segments are dropped. Returns `None` when no projection was supplied.
pub fn parse_projection_expression(
    raw: Option<&str>,
    expr_names: &std::collections::HashMap<String, String>,
) -> Option<Vec<Vec<String>>> {
    let raw = raw?;
    let mut paths: Vec<Vec<String>> = Vec::new();
    for piece in raw.split(',') {
        let path = resolve_path(piece.trim(), expr_names);
        if !path.is_empty() {
            paths.push(path);
        }
    }
    if paths.is_empty() { None } else { Some(paths) }
}

/// Return a new [`Item`] containing only the attributes selected by
/// `paths`. Top-level attributes are copied directly. Nested-path
/// projections build map sub-trees containing only the requested fields,
/// preserving the parent attribute name.
pub fn apply_projection(item: &Item, paths: &[Vec<String>]) -> Item {
    let mut out: Item = std::collections::HashMap::new();
    for path in paths {
        if path.is_empty() {
            continue;
        }
        if path.len() == 1 {
            if let Some(v) = item.get(&path[0]) {
                out.insert(path[0].clone(), v.clone());
            }
            continue;
        }
        // Nested path: descend into the source, build up a parallel
        // map structure in the output.
        let head = &path[0];
        let Some(src_root) = item.get(head) else {
            continue;
        };
        let Some(extracted) = extract_nested_value(src_root, &path[1..]) else {
            continue;
        };
        merge_nested_into(&mut out, head.clone(), &path[1..], extracted);
    }
    out
}

fn extract_nested_value(src: &AttributeValue, path: &[String]) -> Option<AttributeValue> {
    if path.is_empty() {
        return Some(src.clone());
    }
    if let AttributeValue::M(map) = src
        && let Some(child) = map.get(&path[0])
    {
        return extract_nested_value(child, &path[1..]);
    }
    None
}

fn merge_nested_into(out: &mut Item, head: String, rest: &[String], leaf: AttributeValue) {
    let entry = out
        .entry(head)
        .or_insert_with(|| AttributeValue::M(std::collections::HashMap::new()));
    if !matches!(entry, AttributeValue::M(_)) {
        // Top-level already projected as a leaf; leave it alone.
        return;
    }
    if let AttributeValue::M(map) = entry {
        merge_into_map(map, rest, leaf);
    }
}

fn merge_into_map(
    map: &mut std::collections::HashMap<String, AttributeValue>,
    path: &[String],
    leaf: AttributeValue,
) {
    if path.len() == 1 {
        map.insert(path[0].clone(), leaf);
        return;
    }
    let entry = map
        .entry(path[0].clone())
        .or_insert_with(|| AttributeValue::M(std::collections::HashMap::new()));
    if !matches!(entry, AttributeValue::M(_)) {
        return;
    }
    if let AttributeValue::M(inner) = entry {
        merge_into_map(inner, &path[1..], leaf);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    fn av(v: Value) -> AttributeValue {
        serde_json::from_value(v).unwrap_or(AttributeValue::Null(true))
    }

    fn item(attrs: &[(&str, Value)]) -> Item {
        attrs
            .iter()
            .map(|(k, v)| (k.to_string(), av(v.clone())))
            .collect()
    }

    fn names(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn values(pairs: &[(&str, Value)]) -> HashMap<String, AttributeValue> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), av(v.clone())))
            .collect()
    }

    #[test]
    fn test_eq_string() {
        let expr = parse_expression(
            "status = :v",
            &HashMap::new(),
            &values(&[(":v", json!({"S": "active"}))]),
        )
        .unwrap();
        assert!(evaluate(
            &expr,
            &item(&[("status", json!({"S": "active"}))])
        ));
        assert!(!evaluate(
            &expr,
            &item(&[("status", json!({"S": "inactive"}))])
        ));
    }

    #[test]
    fn test_attribute_not_exists() {
        let expr =
            parse_expression("attribute_not_exists(pk)", &HashMap::new(), &HashMap::new()).unwrap();
        assert!(evaluate(&expr, &item(&[])));
        assert!(!evaluate(&expr, &item(&[("pk", json!({"S": "x"}))])));
    }

    #[test]
    fn test_between_numeric() {
        let expr = parse_expression(
            "age BETWEEN :lo AND :hi",
            &HashMap::new(),
            &values(&[(":lo", json!({"N": "10"})), (":hi", json!({"N": "20"}))]),
        )
        .unwrap();
        assert!(evaluate(&expr, &item(&[("age", json!({"N": "15"}))])));
        assert!(evaluate(&expr, &item(&[("age", json!({"N": "10"}))])));
        assert!(evaluate(&expr, &item(&[("age", json!({"N": "20"}))])));
        assert!(!evaluate(&expr, &item(&[("age", json!({"N": "9"}))])));
        assert!(!evaluate(&expr, &item(&[("age", json!({"N": "21"}))])));
    }

    #[test]
    fn test_in_operator() {
        let expr = parse_expression(
            "status IN (:a, :b)",
            &HashMap::new(),
            &values(&[
                (":a", json!({"S": "active"})),
                (":b", json!({"S": "pending"})),
            ]),
        )
        .unwrap();
        assert!(evaluate(
            &expr,
            &item(&[("status", json!({"S": "active"}))])
        ));
        assert!(evaluate(
            &expr,
            &item(&[("status", json!({"S": "pending"}))])
        ));
        assert!(!evaluate(
            &expr,
            &item(&[("status", json!({"S": "deleted"}))])
        ));
    }

    #[test]
    fn test_and_or() {
        let expr = parse_expression(
            "a = :a AND b = :b",
            &HashMap::new(),
            &values(&[(":a", json!({"S": "1"})), (":b", json!({"S": "2"}))]),
        )
        .unwrap();
        assert!(evaluate(
            &expr,
            &item(&[("a", json!({"S": "1"})), ("b", json!({"S": "2"}))])
        ));
        assert!(!evaluate(
            &expr,
            &item(&[("a", json!({"S": "1"})), ("b", json!({"S": "x"}))])
        ));
    }

    #[test]
    fn test_expression_attribute_names() {
        let expr = parse_expression(
            "#n = :v",
            &names(&[("#n", "name")]),
            &values(&[(":v", json!({"S": "alice"}))]),
        )
        .unwrap();
        assert!(evaluate(&expr, &item(&[("name", json!({"S": "alice"}))])));
        assert!(!evaluate(&expr, &item(&[("name", json!({"S": "bob"}))])));
    }

    #[test]
    fn test_numeric_comparison() {
        let expr = parse_expression(
            "version = :expected",
            &HashMap::new(),
            &values(&[(":expected", json!({"N": "99"}))]),
        )
        .unwrap();
        assert!(!evaluate(&expr, &item(&[("version", json!({"N": "1"}))])));
    }
}
