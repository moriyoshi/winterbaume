//! Hand-rolled recursive-descent parser for the DynamoDB-PartiQL subset.
//!
//! Replaces the upstream `partiql-parser` + `partiql-ast` lalrpop pipeline,
//! which had a 30+ minute build script per fresh `CARGO_TARGET_DIR`. The
//! grammar covered here is exactly what `winterbaume-dynamodb` exercises:
//! SELECT / INSERT / UPDATE / DELETE with limited WHERE expressions, bag and
//! struct literals, and DynamoDB-specific function-call shapes.
//!
//! Parses straight into [`DdbOperation`]; no intermediate AST.

mod expr;
mod lexer;
mod stmt;

#[cfg(test)]
mod tests;

use std::collections::VecDeque;

use lexer::{Keyword, Lexer, Token};
use serde_json::Value;

use crate::error::PartiqlError;
use crate::operation::DdbOperation;
use crate::param::ParamBinder;

/// Parse a PartiQL statement and translate it into a [`DdbOperation`].
///
/// `parameters` are DynamoDB AttributeValue JSON values for `?` placeholders.
pub fn parse_statement(
    statement: &str,
    parameters: &[Value],
) -> Result<DdbOperation, PartiqlError> {
    let mut parser = Parser::new(statement, ParamBinder::new(parameters.to_vec()));
    let op = parser.parse_top_level()?;
    parser.params.verify_exhausted()?;
    Ok(op)
}

/// Parse a PartiQL statement with no parameters.
pub fn parse_statement_no_params(statement: &str) -> Result<DdbOperation, PartiqlError> {
    parse_statement(statement, &[])
}

/// Recursive-descent parser with a small token lookahead queue.
///
/// The queue (rather than a single `Option<Token>`) lets the predicate
/// dispatch in `expr.rs` push two tokens back when an `Ident(`...`)` peek
/// turns out not to be a function call. All other paths use one-token
/// lookahead.
pub(crate) struct Parser<'a> {
    lex: Lexer<'a>,
    /// Front of the queue is the next token to return.
    buf: VecDeque<Token>,
    pub(crate) params: ParamBinder,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(src: &'a str, params: ParamBinder) -> Self {
        Self {
            lex: Lexer::new(src),
            buf: VecDeque::with_capacity(2),
            params,
        }
    }

    /// Peek at the next token without consuming it.
    pub(crate) fn peek(&mut self) -> Result<&Token, PartiqlError> {
        if self.buf.is_empty() {
            let tok = self.lex.next_token()?;
            self.buf.push_back(tok);
        }
        Ok(self.buf.front().unwrap())
    }

    /// Consume and return the next token.
    pub(crate) fn bump(&mut self) -> Result<Token, PartiqlError> {
        if let Some(t) = self.buf.pop_front() {
            Ok(t)
        } else {
            self.lex.next_token()
        }
    }

    /// Push back two tokens so the next two `bump`/`peek` calls return them
    /// in the given order. Caller's responsibility to ensure the buffer can
    /// hold them (it always can — we never queue more than two).
    pub(crate) fn unbump2(&mut self, second_to_yield: Token, first_to_yield: Token) {
        // The queue is FIFO with `pop_front`; to make `first_to_yield` come
        // out first, push it to the front, then push `second_to_yield` after.
        // We push them in reverse order at the front to preserve the
        // requested yield order.
        self.buf.push_front(second_to_yield);
        self.buf.push_front(first_to_yield);
    }

    /// If the next token equals the keyword, consume it and return `true`.
    pub(crate) fn eat_kw(&mut self, kw: Keyword) -> Result<bool, PartiqlError> {
        if matches!(self.peek()?, Token::Kw(k) if *k == kw) {
            self.bump()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Require the next token to be the given keyword.
    pub(crate) fn expect_kw(&mut self, kw: Keyword) -> Result<(), PartiqlError> {
        let tok = self.bump()?;
        if matches!(&tok, Token::Kw(k) if *k == kw) {
            Ok(())
        } else {
            Err(PartiqlError::Parse(format!(
                "expected keyword {kw:?}, got {tok:?}"
            )))
        }
    }

    /// Dispatch based on the leading token. Statement keywords (SELECT /
    /// INSERT / UPDATE / DELETE) route to their respective parsers. The
    /// only function that can appear in statement position is `EXISTS(
    /// SELECT … )` — it's a conditional function per AWS docs, used only
    /// in transactional operations to gate the transaction. Detecting it
    /// here keeps it as a function (Ident, not a Keyword) while still
    /// reaching the dedicated parser.
    fn parse_top_level(&mut self) -> Result<DdbOperation, PartiqlError> {
        // Peek for `Ident("EXISTS") (` before consuming the leading token.
        if let Some(name) = self.peek_call_name()?
            && name.eq_ignore_ascii_case("exists")
        {
            // Consume the `(` that peek_call_name left queued.
            let _ = self.bump()?;
            return self.parse_exists();
        }

        let first = self.bump()?;
        let op = match first {
            Token::Kw(Keyword::Select) => self.parse_select()?,
            Token::Kw(Keyword::Insert) => self.parse_insert()?,
            Token::Kw(Keyword::Update) => self.parse_update()?,
            Token::Kw(Keyword::Delete) => self.parse_delete()?,
            other => {
                return Err(PartiqlError::Unsupported(format!(
                    "unsupported statement type: {other:?}"
                )));
            }
        };
        match self.peek()? {
            Token::Eof => Ok(op),
            extra => Err(PartiqlError::Parse(format!(
                "trailing tokens after statement: {extra:?}"
            ))),
        }
    }

    /// If the next two tokens form `Ident LParen`, return the identifier
    /// name and leave the `(` queued. Otherwise restore both tokens and
    /// return None. Same shape as `peek_ident_with_lparen` in expr.rs but
    /// usable from mod.rs without crossing impl boundaries.
    fn peek_call_name(&mut self) -> Result<Option<String>, PartiqlError> {
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
}
