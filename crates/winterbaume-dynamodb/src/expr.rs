//! DynamoDB expression parser and evaluator.
//!
//! Supports ConditionExpression (PutItem, UpdateItem, DeleteItem),
//! FilterExpression (Query, Scan), UpdateExpression (UpdateItem,
//! TransactWriteItems), and ProjectionExpression.  All `#name` and `:value`
//! placeholders are resolved at parse time so the resulting AST contains only
//! concrete attribute names and DynamoDB-typed JSON values.
//!
//! Condition/filter expressions and update expressions have separate
//! tokenisers and parsers: they share placeholder syntax but not their
//! grammars (`+`/`-` and dotted paths on one side, comparison operators and
//! boolean connectives on the other).

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

use crate::types::{SetOperand, UpdateAction};

// ---------------------------------------------------------------------------
// Update-expression tokeniser
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum UpdToken {
    Ident(String),     // bare word: clause keyword, function name, or attribute
    AttrName(String),  // #foo  (kept as-is for lookup)
    AttrValue(String), // :foo  (kept as-is for lookup)
    LParen,
    RParen,
    Comma,
    Dot,
    Eq,
    Plus,
    Minus,
}

impl UpdToken {
    /// Render the token the way it appeared in the source, for error messages.
    fn text(&self) -> String {
        match self {
            UpdToken::Ident(s) | UpdToken::AttrName(s) | UpdToken::AttrValue(s) => s.clone(),
            UpdToken::LParen => "(".to_string(),
            UpdToken::RParen => ")".to_string(),
            UpdToken::Comma => ",".to_string(),
            UpdToken::Dot => ".".to_string(),
            UpdToken::Eq => "=".to_string(),
            UpdToken::Plus => "+".to_string(),
            UpdToken::Minus => "-".to_string(),
        }
    }
}

fn tokenize_update(expr: &str) -> Result<Vec<UpdToken>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' | '\r' => i += 1,
            '(' => {
                tokens.push(UpdToken::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(UpdToken::RParen);
                i += 1;
            }
            ',' => {
                tokens.push(UpdToken::Comma);
                i += 1;
            }
            '.' => {
                tokens.push(UpdToken::Dot);
                i += 1;
            }
            '=' => {
                tokens.push(UpdToken::Eq);
                i += 1;
            }
            '+' => {
                tokens.push(UpdToken::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(UpdToken::Minus);
                i += 1;
            }
            '[' => {
                // List-element dereferences would need an indexed path
                // representation; say so rather than mis-parsing the path.
                return Err(invalid_update_expr(
                    "list element dereferences ('[n]') in update expressions are not supported by winterbaume",
                ));
            }
            '#' | ':' => {
                let sigil = chars[i];
                let start = i;
                i += 1;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                if s.len() == 1 {
                    return Err(invalid_update_expr(&format!(
                        "Syntax error; token: \"{sigil}\""
                    )));
                }
                if sigil == '#' {
                    tokens.push(UpdToken::AttrName(s));
                } else {
                    tokens.push(UpdToken::AttrValue(s));
                }
            }
            c if c.is_alphanumeric() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                tokens.push(UpdToken::Ident(chars[start..i].iter().collect()));
            }
            other => {
                return Err(invalid_update_expr(&format!(
                    "Syntax error; token: \"{other}\""
                )));
            }
        }
    }
    Ok(tokens)
}

fn invalid_update_expr(detail: &str) -> String {
    format!("Invalid UpdateExpression: {detail}")
}

fn syntax_error(tok: Option<&UpdToken>) -> String {
    match tok {
        Some(t) => invalid_update_expr(&format!("Syntax error; token: \"{}\"", t.text())),
        None => invalid_update_expr("Syntax error; unexpected end of expression"),
    }
}

/// Map a bare word to the clause keyword it introduces, if any.
fn clause_keyword(word: &str) -> Option<&'static str> {
    ["SET", "REMOVE", "ADD", "DELETE"]
        .into_iter()
        .find(|kw| word.eq_ignore_ascii_case(kw))
}

// ---------------------------------------------------------------------------
// Update-expression parser
// ---------------------------------------------------------------------------

struct UpdateParser<'a> {
    tokens: Vec<UpdToken>,
    pos: usize,
    expr_names: &'a HashMap<String, String>,
    expr_values: &'a HashMap<String, AttributeValue>,
}

impl UpdateParser<'_> {
    fn peek(&self) -> Option<&UpdToken> {
        self.tokens.get(self.pos)
    }

    fn peek2(&self) -> Option<&UpdToken> {
        self.tokens.get(self.pos + 1)
    }

    fn consume(&mut self) -> Option<UpdToken> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, want: &UpdToken) -> Result<(), String> {
        if self.peek() == Some(want) {
            self.pos += 1;
            Ok(())
        } else {
            Err(syntax_error(self.peek()))
        }
    }

    fn resolve_name(&self, placeholder: &str) -> Result<String, String> {
        self.expr_names.get(placeholder).cloned().ok_or_else(|| {
            invalid_update_expr(&format!(
                "An expression attribute name used in the document path is not defined; attribute name: {placeholder}"
            ))
        })
    }

    fn resolve_value(&self, placeholder: &str) -> Result<AttributeValue, String> {
        self.expr_values.get(placeholder).cloned().ok_or_else(|| {
            invalid_update_expr(&format!(
                "An expression attribute value used in expression is not defined; attribute value: {placeholder}"
            ))
        })
    }

    fn parse(&mut self) -> Result<Vec<UpdateAction>, String> {
        if self.tokens.is_empty() {
            return Err(invalid_update_expr("The expression can not be empty"));
        }
        let mut actions: Vec<UpdateAction> = Vec::new();
        let mut seen: Vec<&'static str> = Vec::new();
        while self.pos < self.tokens.len() {
            let keyword = match self.peek() {
                Some(UpdToken::Ident(word)) => clause_keyword(word),
                _ => None,
            };
            let Some(keyword) = keyword else {
                return Err(syntax_error(self.peek()));
            };
            self.pos += 1;
            if seen.contains(&keyword) {
                return Err(invalid_update_expr(&format!(
                    "The \"{keyword}\" section can only be used once in an update expression"
                )));
            }
            seen.push(keyword);
            let parse_action: fn(&mut Self) -> Result<UpdateAction, String> = match keyword {
                "SET" => Self::parse_set_action,
                "REMOVE" => Self::parse_remove_action,
                "ADD" => Self::parse_add_action,
                _ => Self::parse_delete_action,
            };
            loop {
                actions.push(parse_action(self)?);
                if self.peek() == Some(&UpdToken::Comma) {
                    self.pos += 1;
                    continue;
                }
                break;
            }
        }
        Ok(actions)
    }

    /// `<path> = <operand> [(+|-) <operand>]`
    fn parse_set_action(&mut self) -> Result<UpdateAction, String> {
        let path = self.parse_path()?;
        self.expect(&UpdToken::Eq)?;
        let value = self.parse_set_value()?;
        Ok(UpdateAction::Set { path, value })
    }

    fn parse_remove_action(&mut self) -> Result<UpdateAction, String> {
        Ok(UpdateAction::Remove(self.parse_path()?))
    }

    fn parse_add_action(&mut self) -> Result<UpdateAction, String> {
        let (attr, value) = self.parse_attr_value_pair("ADD")?;
        Ok(UpdateAction::Add(attr, value))
    }

    fn parse_delete_action(&mut self) -> Result<UpdateAction, String> {
        let (attr, value) = self.parse_attr_value_pair("DELETE")?;
        Ok(UpdateAction::Delete(attr, value))
    }

    /// `ADD` / `DELETE` take a top-level attribute followed by a value
    /// placeholder — nested paths are rejected by the real service too.
    fn parse_attr_value_pair(&mut self, clause: &str) -> Result<(String, AttributeValue), String> {
        let path = self.parse_path()?;
        if path.len() != 1 {
            return Err(invalid_update_expr(&format!(
                "The document path provided in the update expression is invalid for update; clause: {clause}"
            )));
        }
        match self.consume() {
            Some(UpdToken::AttrValue(placeholder)) => {
                let value = self.resolve_value(&placeholder)?;
                Ok((path.into_iter().next().unwrap(), value))
            }
            other => Err(syntax_error(other.as_ref())),
        }
    }

    /// The right-hand side of a `SET`: a single operand, optionally one
    /// `+` or `-` applied to a second operand. DynamoDB permits at most one
    /// arithmetic operator, so a chained `a + b + c` is a syntax error.
    fn parse_set_value(&mut self) -> Result<SetOperand, String> {
        let left = self.parse_operand()?;
        match self.peek() {
            Some(UpdToken::Plus) => {
                self.pos += 1;
                let right = self.parse_operand()?;
                Ok(SetOperand::Plus(Box::new(left), Box::new(right)))
            }
            Some(UpdToken::Minus) => {
                self.pos += 1;
                let right = self.parse_operand()?;
                Ok(SetOperand::Minus(Box::new(left), Box::new(right)))
            }
            _ => Ok(left),
        }
    }

    /// `:value`, a document path, `if_not_exists(<path>, <operand>)`, or
    /// `list_append(<operand>, <operand>)`.
    fn parse_operand(&mut self) -> Result<SetOperand, String> {
        match self.peek().cloned() {
            Some(UpdToken::AttrValue(placeholder)) => {
                self.pos += 1;
                Ok(SetOperand::Value(self.resolve_value(&placeholder)?))
            }
            Some(UpdToken::Ident(name)) if self.peek2() == Some(&UpdToken::LParen) => {
                self.pos += 2; // name, (
                let operand = match name.as_str() {
                    "if_not_exists" => {
                        let path = self.parse_path()?;
                        self.expect(&UpdToken::Comma)?;
                        let fallback = self.parse_operand()?;
                        SetOperand::IfNotExists(path, Box::new(fallback))
                    }
                    "list_append" => {
                        let head = self.parse_operand()?;
                        self.expect(&UpdToken::Comma)?;
                        let tail = self.parse_operand()?;
                        SetOperand::ListAppend(Box::new(head), Box::new(tail))
                    }
                    _ => {
                        return Err(invalid_update_expr(&format!(
                            "Invalid function name; function: {name}"
                        )));
                    }
                };
                self.expect(&UpdToken::RParen)?;
                Ok(operand)
            }
            Some(UpdToken::Ident(_)) | Some(UpdToken::AttrName(_)) => {
                Ok(SetOperand::Path(self.parse_path()?))
            }
            other => Err(syntax_error(other.as_ref())),
        }
    }

    /// A dotted document path like `info.city` or `#i.#c.foo`, with `#`
    /// segments resolved through `ExpressionAttributeNames`.
    fn parse_path(&mut self) -> Result<Vec<String>, String> {
        let mut segments = vec![self.parse_path_segment()?];
        while self.peek() == Some(&UpdToken::Dot) {
            self.pos += 1;
            segments.push(self.parse_path_segment()?);
        }
        Ok(segments)
    }

    fn parse_path_segment(&mut self) -> Result<String, String> {
        match self.consume() {
            Some(UpdToken::AttrName(placeholder)) => self.resolve_name(&placeholder),
            Some(UpdToken::Ident(name)) => Ok(name),
            other => Err(syntax_error(other.as_ref())),
        }
    }
}

/// Parse an `UpdateExpression` plus its `ExpressionAttributeNames` /
/// `ExpressionAttributeValues` into a list of [`UpdateAction`]s.
///
/// Recognises the full DynamoDB update grammar apart from list-element
/// dereferences (`a[0]`):
/// - `SET p = <operand> [+|- <operand>]`, where an operand is a value
///   placeholder, a document path, `if_not_exists(<path>, <operand>)`, or
///   `list_append(<operand>, <operand>)` — nested arbitrarily, so the
///   atomic-counter idiom `SET p = if_not_exists(p, :zero) + :v` works
/// - `SET nested.path = :v` (dotted paths, with each segment optionally
///   resolved through `ExpressionAttributeNames`)
/// - `REMOVE p, q.r`
/// - `ADD attr :v` (numeric or set, polymorphic at apply time)
/// - `DELETE attr :v` (set difference)
///
/// Anything else is an error: an expression this emulator cannot evaluate is
/// rejected the way the real service rejects it, rather than being dropped
/// and reported as a successful no-op. Callers should surface the returned
/// message as a `ValidationException`.
pub fn parse_update_expression(
    expr: &str,
    expr_names: &HashMap<String, String>,
    expr_values: &HashMap<String, AttributeValue>,
) -> Result<Vec<UpdateAction>, String> {
    let tokens = tokenize_update(expr)?;
    UpdateParser {
        tokens,
        pos: 0,
        expr_names,
        expr_values,
    }
    .parse()
}

/// Format a number the way DynamoDB does: integer-valued numbers as
/// integers, others via the default `f64` formatter.
fn format_number(n: f64) -> String {
    // DynamoDB's `N` tops out at 38 significant digits; `{:.0}` keeps
    // integral values in positional notation instead of `f64`'s `1e16`.
    if n.fract() == 0.0 && n.abs() < 1e38 {
        format!("{n:.0}")
    } else {
        format!("{n}")
    }
}

/// Add or subtract two DynamoDB `N` literals.
///
/// Integers are added exactly through `i128`, which spans DynamoDB's 38
/// significant digits; anything with a fractional part falls back to `f64`.
fn arith_number(op: char, a: &str, b: &str) -> Option<String> {
    if let (Ok(x), Ok(y)) = (a.parse::<i128>(), b.parse::<i128>()) {
        let sum = if op == '-' {
            x.checked_sub(y)
        } else {
            x.checked_add(y)
        };
        if let Some(v) = sum {
            return Some(v.to_string());
        }
    }
    let x: f64 = a.parse().ok()?;
    let y: f64 = b.parse().ok()?;
    Some(format_number(if op == '-' { x - y } else { x + y }))
}

/// Apply a list of [`UpdateAction`]s to `item` in order.
///
/// The semantics here mirror the AWS DynamoDB UpdateItem behaviour for
/// the action variants we recognise — including dotted-path SET, set ADD /
/// DELETE on `SS`/`NS`/`BS`, `list_append`, and `if_not_exists` — so
/// callers get the same result regardless of which backend they go
/// through.
///
/// Returns the `ValidationException` message the real service would produce
/// when an operand has the wrong type or refers to a missing attribute. The
/// actions are applied to a working copy, so `item` is left untouched when
/// any of them fails.
pub fn apply_update_actions(item: &mut Item, actions: &[UpdateAction]) -> Result<(), String> {
    let mut working = item.clone();
    for action in actions {
        match action {
            UpdateAction::Set { path, value } => {
                let value = eval_set_operand(&working, value)?;
                set_at_path(&mut working, path, value);
            }
            UpdateAction::Remove(path) => {
                remove_at_path(&mut working, path);
            }
            UpdateAction::Add(attr, delta) => {
                apply_add(&mut working, attr, delta)?;
            }
            UpdateAction::Delete(attr, members) => {
                apply_set_delete(&mut working, attr, members)?;
            }
        }
    }
    *item = working;
    Ok(())
}

/// The `ValidationException` message AWS returns when an operand's type does
/// not suit the operator or function applied to it.
fn operand_type_error(operator: &str, value: &AttributeValue) -> String {
    format!(
        "Invalid UpdateExpression: Incorrect operand type for operator or function; operator or function: {operator}, operand type: {}",
        ddb_type(value).unwrap_or("NULL")
    )
}

/// Evaluate the right-hand side of a `SET` against the item as updated so far.
fn eval_set_operand(item: &Item, operand: &SetOperand) -> Result<AttributeValue, String> {
    match operand {
        SetOperand::Value(value) => Ok(value.clone()),
        SetOperand::Path(path) => get_at_path(item, path).cloned().ok_or_else(|| {
            "The provided expression refers to an attribute that does not exist in the item"
                .to_string()
        }),
        SetOperand::IfNotExists(path, fallback) => match get_at_path(item, path) {
            Some(value) => Ok(value.clone()),
            None => eval_set_operand(item, fallback),
        },
        SetOperand::ListAppend(head, tail) => {
            let head = eval_set_operand(item, head)?;
            let tail = eval_set_operand(item, tail)?;
            match (head, tail) {
                (AttributeValue::L(mut first), AttributeValue::L(second)) => {
                    first.extend(second);
                    Ok(AttributeValue::L(first))
                }
                (AttributeValue::L(_), other) | (other, _) => {
                    Err(operand_type_error("list_append", &other))
                }
            }
        }
        SetOperand::Plus(left, right) => eval_arith('+', item, left, right),
        SetOperand::Minus(left, right) => eval_arith('-', item, left, right),
    }
}

fn eval_arith(
    op: char,
    item: &Item,
    left: &SetOperand,
    right: &SetOperand,
) -> Result<AttributeValue, String> {
    let left = eval_set_operand(item, left)?;
    let right = eval_set_operand(item, right)?;
    let (AttributeValue::N(a), AttributeValue::N(b)) = (&left, &right) else {
        let offender = if matches!(left, AttributeValue::N(_)) {
            &right
        } else {
            &left
        };
        return Err(operand_type_error(&op.to_string(), offender));
    };
    arith_number(op, a, b)
        .map(AttributeValue::N)
        .ok_or_else(|| operand_type_error(&op.to_string(), &left))
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

fn apply_add(item: &mut Item, attr: &str, delta: &AttributeValue) -> Result<(), String> {
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
        // Numeric ADD: on a missing attribute the delta becomes the value.
        (None, AttributeValue::N(delta)) => {
            item.insert(attr.to_string(), AttributeValue::N(delta.clone()));
        }
        (Some(AttributeValue::N(cur)), AttributeValue::N(delta)) => {
            let sum = arith_number('+', cur, delta)
                .ok_or_else(|| operand_type_error("ADD", &AttributeValue::N(cur.clone())))?;
            item.insert(attr.to_string(), AttributeValue::N(sum));
        }
        // Any other combination — a numeric delta against a string
        // attribute, mismatched set types, and so on — is the same
        // ValidationException the real service raises.
        (current, delta) => {
            return Err(format!(
                "An operand in the update expression has an incorrect data type: ADD on attribute '{attr}' of type {} with a value of type {}",
                current.and_then(ddb_type).unwrap_or("NULL"),
                ddb_type(delta).unwrap_or("NULL"),
            ));
        }
    }
    Ok(())
}

fn apply_set_delete(item: &mut Item, attr: &str, members: &AttributeValue) -> Result<(), String> {
    // DELETE against a missing attribute is a no-op, as on the real service.
    if !item.contains_key(attr) {
        return Ok(());
    }
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
        (current, members) => {
            return Err(format!(
                "An operand in the update expression has an incorrect data type: DELETE on attribute '{attr}' of type {} with a value of type {}",
                current.and_then(ddb_type).unwrap_or("NULL"),
                ddb_type(members).unwrap_or("NULL"),
            ));
        }
    };
    match new_value {
        Some(v) => {
            item.insert(attr.to_string(), v);
        }
        None => {
            item.remove(attr);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// ProjectionExpression
// ---------------------------------------------------------------------------

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

    // -----------------------------------------------------------------
    // UpdateExpression parsing / application
    // -----------------------------------------------------------------

    /// Parse and apply in one step, the way a backend does.
    fn update(
        start: &[(&str, Value)],
        expr: &str,
        names_pairs: &[(&str, &str)],
        values_pairs: &[(&str, Value)],
    ) -> Result<Item, String> {
        let actions = parse_update_expression(expr, &names(names_pairs), &values(values_pairs))?;
        let mut it = item(start);
        apply_update_actions(&mut it, &actions)?;
        Ok(it)
    }

    /// Regression for issue #19: `if_not_exists` nested inside arithmetic —
    /// the atomic-counter-with-default idiom — used to be dropped silently.
    #[test]
    fn test_set_if_not_exists_inside_arithmetic() {
        // First call: `p` is absent, so the default feeds the addition.
        let out = update(
            &[],
            "SET p = if_not_exists(p, :zero) + :v",
            &[],
            &[(":zero", json!({"N": "0"})), (":v", json!({"N": "1"}))],
        )
        .unwrap();
        assert_eq!(out.get("p"), Some(&av(json!({"N": "1"}))));

        // Second call: the stored value is used instead of the default.
        let out = update(
            &[("p", json!({"N": "1"}))],
            "SET p = if_not_exists(p, :zero) + :v",
            &[],
            &[(":zero", json!({"N": "0"})), (":v", json!({"N": "1"}))],
        )
        .unwrap();
        assert_eq!(out.get("p"), Some(&av(json!({"N": "2"}))));
    }

    #[test]
    fn test_set_nested_list_append_and_if_not_exists() {
        let out = update(
            &[],
            "SET #l = list_append(if_not_exists(#l, :empty), :extra)",
            &[("#l", "items")],
            &[
                (":empty", json!({"L": []})),
                (":extra", json!({"L": [{"S": "a"}]})),
            ],
        )
        .unwrap();
        assert_eq!(out.get("items"), Some(&av(json!({"L": [{"S": "a"}]}))));
    }

    #[test]
    fn test_set_arithmetic_between_two_paths() {
        let out = update(
            &[("a", json!({"N": "10"})), ("b", json!({"N": "4"}))],
            "SET c = a - b",
            &[],
            &[],
        )
        .unwrap();
        assert_eq!(out.get("c"), Some(&av(json!({"N": "6"}))));
    }

    #[test]
    fn test_set_arithmetic_is_exact_beyond_f64_integers() {
        let out = update(
            &[("n", json!({"N": "9007199254740993"}))],
            "SET n = n + :one",
            &[],
            &[(":one", json!({"N": "1"}))],
        )
        .unwrap();
        assert_eq!(out.get("n"), Some(&av(json!({"N": "9007199254740994"}))));
    }

    #[test]
    fn test_multiple_clauses_and_dotted_paths() {
        let out = update(
            &[
                ("gone", json!({"S": "x"})),
                ("tags", json!({"SS": ["a", "b"]})),
            ],
            "SET info.city = :c REMOVE gone ADD hits :one DELETE tags :rm",
            &[],
            &[
                (":c", json!({"S": "berlin"})),
                (":one", json!({"N": "1"})),
                (":rm", json!({"SS": ["a"]})),
            ],
        )
        .unwrap();
        assert_eq!(
            out.get("info"),
            Some(&av(json!({"M": {"city": {"S": "berlin"}}})))
        );
        assert!(!out.contains_key("gone"));
        assert_eq!(out.get("hits"), Some(&av(json!({"N": "1"}))));
        assert_eq!(out.get("tags"), Some(&av(json!({"SS": ["b"]}))));
    }

    /// An expression the emulator cannot evaluate must be rejected, not
    /// dropped: issue #19's central complaint.
    #[test]
    fn test_unparseable_assignments_are_rejected() {
        for expr in [
            "SET p = ",
            "SET p",
            "SET p = :a + :b + :c",
            "SET p = bogus_fn(p, :a)",
            "SET p = :a SET q = :a",
            "SET p = :a, p[0] = :a",
            "GIVE p :a",
            "",
        ] {
            let err = parse_update_expression(
                expr,
                &HashMap::new(),
                &values(&[
                    (":a", json!({"N": "1"})),
                    (":b", json!({"N": "2"})),
                    (":c", json!({"N": "3"})),
                ]),
            )
            .expect_err(&format!("expected `{expr}` to be rejected"));
            assert!(
                err.starts_with("Invalid UpdateExpression:"),
                "unexpected message for `{expr}`: {err}"
            );
        }
    }

    #[test]
    fn test_undefined_placeholders_are_rejected() {
        assert!(
            parse_update_expression("SET p = :missing", &HashMap::new(), &HashMap::new()).is_err()
        );
        assert!(
            parse_update_expression(
                "SET #missing = :v",
                &HashMap::new(),
                &values(&[(":v", json!({"N": "1"}))])
            )
            .is_err()
        );
    }

    /// Operand type errors surface at apply time, and leave the item alone.
    #[test]
    fn test_operand_type_errors() {
        let err = update(
            &[("s", json!({"S": "text"}))],
            "SET s = s + :one",
            &[],
            &[(":one", json!({"N": "1"}))],
        )
        .unwrap_err();
        assert!(err.contains("Incorrect operand type"), "{err}");

        // Arithmetic on an attribute that isn't there is a ValidationException
        // on the real service — that is what if_not_exists is for.
        let err = update(
            &[],
            "SET missing = missing + :one",
            &[],
            &[(":one", json!({"N": "1"}))],
        )
        .unwrap_err();
        assert!(err.contains("does not exist in the item"), "{err}");
    }

    #[test]
    fn test_failed_action_leaves_item_untouched() {
        let actions = parse_update_expression(
            "SET a = :one, b = missing",
            &HashMap::new(),
            &values(&[(":one", json!({"N": "1"}))]),
        )
        .unwrap();
        let mut it = item(&[("keep", json!({"S": "yes"}))]);
        assert!(apply_update_actions(&mut it, &actions).is_err());
        assert_eq!(it, item(&[("keep", json!({"S": "yes"}))]));
    }
}
