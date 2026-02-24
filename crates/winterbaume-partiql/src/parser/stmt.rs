//! Statement-level parsing — SELECT / INSERT / UPDATE / DELETE.
//!
//! Each `parse_<stmt>` is entered after the leading keyword has been consumed
//! by [`Parser::parse_top_level`].

use super::lexer::{Keyword, Token};
use super::{Parser, PartiqlError};
use crate::operation::{
    Condition, DdbOperation, DeleteOp, InsertOp, Item, ReturningClause, SelectOp, SetValue,
    UpdateOp,
};

impl<'a> Parser<'a> {
    // ── SELECT ────────────────────────────────────────────────────────────────

    pub(crate) fn parse_select(&mut self) -> Result<DdbOperation, PartiqlError> {
        let projection = self.parse_projection()?;
        self.expect_kw(Keyword::From)?;
        let (table_name, index_name) = self.parse_table_ref()?;
        let where_clause = self.parse_optional_where()?;
        let (order_ascending, order_by_attr) = self.parse_optional_order_by()?;
        Ok(DdbOperation::Select(SelectOp {
            table_name,
            index_name,
            projection,
            where_clause,
            order_ascending,
            order_by_attr,
        }))
    }

    // ── EXISTS ─────────────────────────────────────────────────────────────────

    /// `EXISTS(SELECT … FROM … WHERE …)` — a conditional function (per AWS
    /// docs, https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ql-functions.exists.html)
    /// usable as a transaction condition check. The caller has already
    /// confirmed the lookahead is `Ident("EXISTS")` followed by `(` and
    /// consumed both — so we begin at the `SELECT` keyword.
    pub(crate) fn parse_exists(&mut self) -> Result<DdbOperation, PartiqlError> {
        self.expect_kw(Keyword::Select)?;
        let inner = self.parse_select()?;
        let rp = self.bump()?;
        if !matches!(rp, Token::RParen) {
            return Err(PartiqlError::Parse(format!(
                "expected ')' to close EXISTS, got {rp:?}"
            )));
        }
        match inner {
            DdbOperation::Select(sel) => Ok(DdbOperation::Exists(Box::new(sel))),
            _ => unreachable!("parse_select always returns DdbOperation::Select"),
        }
    }

    /// `*`, or `col, col, …`. `VALUE expr` projections are rejected as
    /// unsupported (matches today's behaviour and the message text).
    fn parse_projection(&mut self) -> Result<Option<Vec<String>>, PartiqlError> {
        if matches!(self.peek()?, Token::Star) {
            self.bump()?;
            return Ok(None);
        }
        if matches!(self.peek()?, Token::Kw(Keyword::Value)) {
            return Err(PartiqlError::Unsupported("PIVOT/VALUE projections".into()));
        }
        let mut cols = Vec::new();
        loop {
            cols.push(self.parse_path()?);
            if matches!(self.peek()?, Token::Comma) {
                self.bump()?;
                continue;
            }
            break;
        }
        Ok(Some(cols))
    }

    fn parse_optional_where(&mut self) -> Result<Option<Vec<Condition>>, PartiqlError> {
        if self.eat_kw(Keyword::Where)? {
            Ok(Some(self.parse_where_expr()?))
        } else {
            Ok(None)
        }
    }

    fn parse_optional_order_by(&mut self) -> Result<(Option<bool>, Option<String>), PartiqlError> {
        if !self.eat_kw(Keyword::Order)? {
            return Ok((None, None));
        }
        self.expect_kw(Keyword::By)?;
        let attr = self.parse_path()?;
        let ascending = if self.eat_kw(Keyword::Desc)? {
            Some(false)
        } else {
            // Either ASC explicitly or no direction — both default to ascending.
            self.eat_kw(Keyword::Asc)?;
            Some(true)
        };
        Ok((ascending, Some(attr)))
    }

    // ── INSERT ────────────────────────────────────────────────────────────────

    pub(crate) fn parse_insert(&mut self) -> Result<DdbOperation, PartiqlError> {
        self.expect_kw(Keyword::Into)?;
        let (table_name, index_name) = self.parse_table_ref()?;
        if index_name.is_some() {
            return Err(PartiqlError::Unsupported(
                "INSERT INTO with index name is not supported".into(),
            ));
        }
        self.expect_kw(Keyword::Value)?;
        let item = self.parse_struct_item()?;
        Ok(DdbOperation::Insert(InsertOp { table_name, item }))
    }

    /// Parse the `{ 'k': v, … }` body of an INSERT VALUE and return it as
    /// an [`Item`] (a `HashMap<String, AttributeValue>`), not wrapped in `M`.
    fn parse_struct_item(&mut self) -> Result<Item, PartiqlError> {
        let lb = self.bump()?;
        if !matches!(lb, Token::LBrace) {
            return Err(PartiqlError::Parse(format!(
                "expected '{{' for VALUE, got {lb:?}"
            )));
        }
        let mut item: Item = Item::new();
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
                item.insert(key, val);
                if matches!(self.peek()?, Token::Comma) {
                    self.bump()?;
                    continue;
                }
                break;
            }
        }
        let rb = self.bump()?;
        if !matches!(rb, Token::RBrace) {
            return Err(PartiqlError::Parse(format!("expected '}}', got {rb:?}")));
        }
        Ok(item)
    }

    // ── UPDATE ────────────────────────────────────────────────────────────────

    pub(crate) fn parse_update(&mut self) -> Result<DdbOperation, PartiqlError> {
        let (table_name, index_name) = self.parse_table_ref()?;
        if index_name.is_some() {
            return Err(PartiqlError::Unsupported(
                "UPDATE with index name is not supported".into(),
            ));
        }
        let mut sets: Vec<(String, SetValue)> = Vec::new();
        let mut removes: Vec<String> = Vec::new();
        let mut key_conditions: Vec<Condition> = Vec::new();
        let mut returning: Option<ReturningClause> = None;
        loop {
            match self.peek()? {
                Token::Kw(Keyword::Set) => {
                    self.bump()?;
                    let (path, val) = self.parse_set_assignment()?;
                    sets.push((path, val));
                }
                Token::Kw(Keyword::Remove) => {
                    self.bump()?;
                    removes.push(self.parse_path()?);
                }
                Token::Kw(Keyword::Where) => {
                    self.bump()?;
                    key_conditions = self.parse_where_expr()?;
                    returning = self.parse_optional_returning()?;
                    break;
                }
                Token::Kw(Keyword::Returning) => {
                    returning = self.parse_optional_returning()?;
                    break;
                }
                Token::Eof => break,
                other => {
                    return Err(PartiqlError::Parse(format!(
                        "unexpected in UPDATE: {other:?}"
                    )));
                }
            }
        }
        Ok(DdbOperation::Update(UpdateOp {
            table_name,
            key_conditions,
            sets,
            removes,
            returning,
        }))
    }

    /// `path = <set-rhs>` — the assignment after a single `SET` keyword.
    fn parse_set_assignment(&mut self) -> Result<(String, SetValue), PartiqlError> {
        let target = self.parse_path()?;
        let eq = self.bump()?;
        if !matches!(eq, Token::Eq) {
            return Err(PartiqlError::Parse(format!(
                "expected '=' after {target}, got {eq:?}"
            )));
        }
        let val = self.parse_set_rhs()?;
        Ok((target, val))
    }

    /// SET right-hand-side. Three shapes:
    /// - Function call: `list_append(path, value)`, `set_add(path, value)`,
    ///   `set_delete(path, value)` — these are kept as their own variants
    ///   because their semantics differ from raw assignment (atomic for
    ///   set_add/set_delete; deferred concat for list_append).
    /// - Arbitrary expression: `literal`, `path`, `path + path`, `path + 1`,
    ///   `(a + b) - c`, etc. All produced as `SetValue::Expr(Expression)`.
    ///
    /// Function-call dispatch happens first via the same `Ident(`...`)`
    /// lookahead trick used for predicate function calls.
    fn parse_set_rhs(&mut self) -> Result<SetValue, PartiqlError> {
        if let Some(name) = self.peek_ident_with_lparen_for_set()? {
            let lower = name.to_ascii_lowercase();
            // Only the three documented set-mutation functions are special
            // in SET RHS. Anything else falls through to the expression
            // parser, which will surface a "function in WHERE" / unsupported
            // error if the function isn't recognised by parse_expression.
            if matches!(lower.as_str(), "list_append" | "set_add" | "set_delete") {
                // Consume the `(` we left in the queue.
                let _ = self.bump()?;
                return self.finish_set_function_call(&lower);
            }
            // Re-queue the Ident + LParen so parse_expression can take over.
            // Not really reachable for now — parse_expression doesn't accept
            // call-shaped expressions in primary position — but we can
            // surface a clean error rather than a panic.
            return Err(PartiqlError::Unsupported(format!(
                "function in SET: {name}"
            )));
        }
        Ok(SetValue::Expr(self.parse_expression()?))
    }

    /// Variant of `peek_ident_with_lparen` for SET RHS dispatch.
    /// Implemented inline (not in expr.rs) because we need slightly different
    /// semantics: we leave the `(` and the Ident BOTH queued if it's not a
    /// recognised SET function, so the expression parser can take over.
    fn peek_ident_with_lparen_for_set(&mut self) -> Result<Option<String>, PartiqlError> {
        if !matches!(self.peek()?, Token::Ident(_)) {
            return Ok(None);
        }
        let Token::Ident(name) = self.bump()? else {
            unreachable!()
        };
        let next = self.bump()?;
        if matches!(&next, Token::LParen) {
            // Leave `(` queued; caller decides whether to consume it.
            self.buf.push_front(next);
            Ok(Some(name))
        } else {
            self.unbump2(next, Token::Ident(name));
            Ok(None)
        }
    }

    /// Caller has consumed `<funcname> (`. Parse `path , value )` and dispatch
    /// by lowered function name.
    fn finish_set_function_call(&mut self, name: &str) -> Result<SetValue, PartiqlError> {
        let target = self.parse_path()?;
        let comma = self.bump()?;
        if !matches!(comma, Token::Comma) {
            return Err(PartiqlError::Parse(format!(
                "expected ',' in {name} call, got {comma:?}"
            )));
        }
        let val = self.parse_value()?;
        let rp = self.bump()?;
        if !matches!(rp, Token::RParen) {
            return Err(PartiqlError::Parse(format!(
                "expected ')' to close {name} call, got {rp:?}"
            )));
        }
        let lower = name.to_ascii_lowercase();
        match lower.as_str() {
            "list_append" => Ok(SetValue::ListAppend(target, val)),
            "set_add" => Ok(SetValue::SetAdd(target, val)),
            "set_delete" => Ok(SetValue::SetDelete(target, val)),
            _ => Err(PartiqlError::Unsupported(format!(
                "function in SET: {name}"
            ))),
        }
    }

    // ── DELETE ────────────────────────────────────────────────────────────────

    pub(crate) fn parse_delete(&mut self) -> Result<DdbOperation, PartiqlError> {
        self.expect_kw(Keyword::From)?;
        let (table_name, index_name) = self.parse_table_ref()?;
        if index_name.is_some() {
            return Err(PartiqlError::Unsupported(
                "DELETE with index name is not supported".into(),
            ));
        }
        let key_conditions = self.parse_optional_where()?.unwrap_or_default();
        let returning = self.parse_optional_returning()?;
        Ok(DdbOperation::Delete(DeleteOp {
            table_name,
            key_conditions,
            returning,
        }))
    }

    // ── Shared helpers ────────────────────────────────────────────────────────

    /// `name` or `"name"."index"`.
    fn parse_table_ref(&mut self) -> Result<(String, Option<String>), PartiqlError> {
        let table_name = match self.bump()? {
            Token::Ident(s) => s,
            Token::QuotedIdent(s) => s,
            _ => return Err(PartiqlError::MissingTableName),
        };
        let index_name = if matches!(self.peek()?, Token::Dot) {
            self.bump()?;
            match self.bump()? {
                Token::Ident(s) => Some(s),
                Token::QuotedIdent(s) => Some(s),
                Token::Str(s) => Some(s),
                other => {
                    return Err(PartiqlError::Parse(format!(
                        "expected index name after '.', got {other:?}"
                    )));
                }
            }
        } else {
            None
        };
        Ok((table_name, index_name))
    }

    /// If the next token is `RETURNING`, consume and parse the kind. Otherwise
    /// return `None`. Must be called *after* any other trailing clauses (the
    /// existing grammar puts RETURNING last).
    ///
    /// AWS DynamoDB's PartiQL grammar emits `RETURNING <kind> <when> *`
    /// (with a trailing `*` that's the SQL-style "all columns" marker the
    /// service includes when round-tripping). We accept it as optional so
    /// callers that drop the `*` (e.g. the existing unit tests) keep working.
    fn parse_optional_returning(&mut self) -> Result<Option<ReturningClause>, PartiqlError> {
        if !self.eat_kw(Keyword::Returning)? {
            return Ok(None);
        }
        let kind = self.bump()?;
        let when = self.bump()?;
        let result = match (&kind, &when) {
            (Token::Kw(Keyword::All), Token::Kw(Keyword::New)) => ReturningClause::AllNew,
            (Token::Kw(Keyword::All), Token::Kw(Keyword::Old)) => ReturningClause::AllOld,
            (Token::Kw(Keyword::Modified), Token::Kw(Keyword::New)) => ReturningClause::ModifiedNew,
            (Token::Kw(Keyword::Modified), Token::Kw(Keyword::Old)) => ReturningClause::ModifiedOld,
            _ => {
                return Err(PartiqlError::Parse(format!(
                    "unknown RETURNING clause: {kind:?} {when:?}"
                )));
            }
        };
        // Optional trailing `*`.
        if matches!(self.peek()?, Token::Star) {
            self.bump()?;
        }
        Ok(Some(result))
    }
}
