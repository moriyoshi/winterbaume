//! Lexer for the DynamoDB-PartiQL subset.
//!
//! Token boundaries we care about:
//! - `<<` and `>>` recognised greedily before `<` / `>` (set/bag braces).
//! - `''` inside a single-quoted string is the SQL escape for one quote.
//! - `"..."` is a delimited identifier, no escapes.
//! - `?` is a positional parameter placeholder.
//! - Keywords are matched case-insensitively against the unquoted identifier
//!   slice. Function names like `begins_with`, `list_append`, `attribute_exists`
//!   are NOT keywords; they remain `Ident` and are dispatched by the parser.

use crate::error::PartiqlError;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LBag,
    RBag,
    Comma,
    Dot,
    Colon,
    Star,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Plus,
    Minus,
    QMark,
    /// Unescaped string literal contents.
    Str(String),
    /// Numeric literal — raw textual form preserved (no f64 round-trip).
    Num(String),
    /// Unquoted identifier (case preserved as written).
    Ident(String),
    /// `"…"` delimited identifier.
    QuotedIdent(String),
    Kw(Keyword),
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Keyword {
    Select,
    From,
    Where,
    Value,
    Insert,
    Into,
    Update,
    Set,
    Remove,
    Delete,
    Order,
    By,
    Asc,
    Desc,
    And,
    Or,
    Not,
    Between,
    In_,
    Is,
    True,
    False,
    Null,
    Missing,
    Returning,
    All,
    Modified,
    New,
    Old,
}

impl Keyword {
    fn from_ident(s: &str) -> Option<Self> {
        // Case-insensitive match. Avoid allocating a lowercase copy by using
        // `eq_ignore_ascii_case` per candidate.
        macro_rules! kw {
            ($($lit:literal => $variant:ident),* $(,)?) => {
                $(if s.eq_ignore_ascii_case($lit) { return Some(Keyword::$variant); })*
            };
        }
        kw! {
            "select" => Select,
            "from" => From,
            "where" => Where,
            "value" => Value,
            "insert" => Insert,
            "into" => Into,
            "update" => Update,
            "set" => Set,
            "remove" => Remove,
            "delete" => Delete,
            "order" => Order,
            "by" => By,
            "asc" => Asc,
            "desc" => Desc,
            "and" => And,
            "or" => Or,
            "not" => Not,
            "between" => Between,
            "in" => In_,
            "is" => Is,
            "true" => True,
            "false" => False,
            "null" => Null,
            "missing" => Missing,
            "returning" => Returning,
            "all" => All,
            "modified" => Modified,
            "new" => New,
            "old" => Old,
        }
        None
    }
}

pub(crate) struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    pub fn next_token(&mut self) -> Result<Token, PartiqlError> {
        self.skip_whitespace();
        let bytes = self.src.as_bytes();
        if self.pos >= bytes.len() {
            return Ok(Token::Eof);
        }
        let c = bytes[self.pos];
        match c {
            b'(' => self.single(Token::LParen),
            b')' => self.single(Token::RParen),
            b'[' => self.single(Token::LBracket),
            b']' => self.single(Token::RBracket),
            b'{' => self.single(Token::LBrace),
            b'}' => self.single(Token::RBrace),
            b',' => self.single(Token::Comma),
            b'.' => self.single(Token::Dot),
            b':' => self.single(Token::Colon),
            b'*' => self.single(Token::Star),
            b'+' => self.single(Token::Plus),
            b'-' => self.single(Token::Minus),
            b'?' => self.single(Token::QMark),
            b'=' => self.single(Token::Eq),
            b'!' => {
                if self.peek_byte(1) == Some(b'=') {
                    self.pos += 2;
                    Ok(Token::Ne)
                } else {
                    Err(PartiqlError::Parse(format!(
                        "unexpected character '!' at offset {}",
                        self.pos
                    )))
                }
            }
            b'<' => match self.peek_byte(1) {
                Some(b'<') => {
                    self.pos += 2;
                    Ok(Token::LBag)
                }
                Some(b'=') => {
                    self.pos += 2;
                    Ok(Token::Le)
                }
                Some(b'>') => {
                    // SQL <> is alias for !=
                    self.pos += 2;
                    Ok(Token::Ne)
                }
                _ => self.single(Token::Lt),
            },
            b'>' => match self.peek_byte(1) {
                Some(b'>') => {
                    self.pos += 2;
                    Ok(Token::RBag)
                }
                Some(b'=') => {
                    self.pos += 2;
                    Ok(Token::Ge)
                }
                _ => self.single(Token::Gt),
            },
            b'\'' => self.lex_string(),
            b'"' => self.lex_quoted_ident(),
            c if c.is_ascii_digit() => self.lex_number(),
            c if c.is_ascii_alphabetic() || c == b'_' => self.lex_ident_or_kw(),
            other => Err(PartiqlError::Parse(format!(
                "unexpected character {:?} at offset {}",
                other as char, self.pos
            ))),
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    fn single(&mut self, tok: Token) -> Result<Token, PartiqlError> {
        self.pos += 1;
        Ok(tok)
    }

    fn peek_byte(&self, offset: usize) -> Option<u8> {
        self.src.as_bytes().get(self.pos + offset).copied()
    }

    fn skip_whitespace(&mut self) {
        let bytes = self.src.as_bytes();
        loop {
            while self.pos < bytes.len() && bytes[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos + 1 < bytes.len() && bytes[self.pos] == b'-' && bytes[self.pos + 1] == b'-'
            {
                self.pos += 2;
                while self.pos < bytes.len() && bytes[self.pos] != b'\n' {
                    self.pos += 1;
                }
                continue;
            }
            if self.pos + 1 < bytes.len() && bytes[self.pos] == b'/' && bytes[self.pos + 1] == b'*'
            {
                self.pos += 2;
                while self.pos + 1 < bytes.len()
                    && !(bytes[self.pos] == b'*' && bytes[self.pos + 1] == b'/')
                {
                    self.pos += 1;
                }
                if self.pos + 1 < bytes.len() {
                    self.pos += 2;
                }
                continue;
            }
            break;
        }
    }

    fn lex_string(&mut self) -> Result<Token, PartiqlError> {
        // Position is at the opening `'`.
        debug_assert_eq!(self.src.as_bytes()[self.pos], b'\'');
        let mut out = String::new();
        // Index past the opening quote.
        let mut i = self.pos + 1;
        let bytes = self.src.as_bytes();
        while i < bytes.len() {
            if bytes[i] == b'\'' {
                if bytes.get(i + 1) == Some(&b'\'') {
                    out.push('\'');
                    i += 2;
                    continue;
                }
                self.pos = i + 1;
                return Ok(Token::Str(out));
            }
            // Consume one UTF-8 scalar so we don't slice mid-codepoint.
            let ch = self.src[i..]
                .chars()
                .next()
                .expect("byte index inside string slice");
            out.push(ch);
            i += ch.len_utf8();
        }
        Err(PartiqlError::Parse("unterminated string literal".into()))
    }

    fn lex_quoted_ident(&mut self) -> Result<Token, PartiqlError> {
        debug_assert_eq!(self.src.as_bytes()[self.pos], b'"');
        let start = self.pos + 1;
        let bytes = self.src.as_bytes();
        let mut i = start;
        while i < bytes.len() {
            if bytes[i] == b'"' {
                let name = self.src[start..i].to_string();
                self.pos = i + 1;
                return Ok(Token::QuotedIdent(name));
            }
            let ch = self.src[i..]
                .chars()
                .next()
                .expect("byte index inside string slice");
            i += ch.len_utf8();
        }
        Err(PartiqlError::Parse("unterminated quoted identifier".into()))
    }

    fn lex_number(&mut self) -> Result<Token, PartiqlError> {
        let start = self.pos;
        let bytes = self.src.as_bytes();
        while self.pos < bytes.len() && bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        // Optional fractional part — only consume `.` if a digit follows.
        // (`Tags.field` would otherwise be misread.)
        if self.pos < bytes.len()
            && bytes[self.pos] == b'.'
            && bytes.get(self.pos + 1).is_some_and(|b| b.is_ascii_digit())
        {
            self.pos += 1;
            while self.pos < bytes.len() && bytes[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }
        // Optional exponent: `[eE][+-]?digits`.
        if self.pos < bytes.len() && (bytes[self.pos] == b'e' || bytes[self.pos] == b'E') {
            let saved = self.pos;
            self.pos += 1;
            if self.pos < bytes.len() && (bytes[self.pos] == b'+' || bytes[self.pos] == b'-') {
                self.pos += 1;
            }
            let digits_start = self.pos;
            while self.pos < bytes.len() && bytes[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            if self.pos == digits_start {
                // No digits after `e`/`E`; not really an exponent.
                self.pos = saved;
            }
        }
        let raw = &self.src[start..self.pos];
        if raw.parse::<f64>().is_err() {
            return Err(PartiqlError::Parse(format!(
                "invalid numeric literal {raw:?}"
            )));
        }
        Ok(Token::Num(raw.to_string()))
    }

    fn lex_ident_or_kw(&mut self) -> Result<Token, PartiqlError> {
        let start = self.pos;
        let bytes = self.src.as_bytes();
        while self.pos < bytes.len() {
            let c = bytes[self.pos];
            if c.is_ascii_alphanumeric() || c == b'_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let slice = &self.src[start..self.pos];
        if let Some(kw) = Keyword::from_ident(slice) {
            Ok(Token::Kw(kw))
        } else {
            Ok(Token::Ident(slice.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect(src: &str) -> Result<Vec<Token>, PartiqlError> {
        let mut lex = Lexer::new(src);
        let mut out = Vec::new();
        loop {
            let tok = lex.next_token()?;
            if tok == Token::Eof {
                break;
            }
            out.push(tok);
        }
        Ok(out)
    }

    #[test]
    fn lex_select_basics() {
        let toks = collect("SELECT * FROM \"Music\"").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Kw(Keyword::Select),
                Token::Star,
                Token::Kw(Keyword::From),
                Token::QuotedIdent("Music".into()),
            ]
        );
    }

    #[test]
    fn lex_string_escape() {
        // `'O''Brien'` is a single string literal with one embedded quote.
        let toks = collect("'O''Brien'").unwrap();
        assert_eq!(toks, vec![Token::Str("O'Brien".into())]);
    }

    #[test]
    fn lex_bag_brackets_before_lt_gt() {
        let toks = collect("<<'a', 'b'>>").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::LBag,
                Token::Str("a".into()),
                Token::Comma,
                Token::Str("b".into()),
                Token::RBag,
            ]
        );
    }

    #[test]
    fn lex_lt_le_gt_ge_ne() {
        let toks = collect("< <= > >= != <>").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Lt,
                Token::Le,
                Token::Gt,
                Token::Ge,
                Token::Ne,
                Token::Ne,
            ]
        );
    }

    #[test]
    fn lex_question_mark_placeholder() {
        let toks = collect("WHERE pk = ?").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Kw(Keyword::Where),
                Token::Ident("pk".into()),
                Token::Eq,
                Token::QMark,
            ]
        );
    }

    #[test]
    fn lex_keywords_case_insensitive() {
        let toks = collect("select FROM Where ORDER by AsC dEsC").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Kw(Keyword::Select),
                Token::Kw(Keyword::From),
                Token::Kw(Keyword::Where),
                Token::Kw(Keyword::Order),
                Token::Kw(Keyword::By),
                Token::Kw(Keyword::Asc),
                Token::Kw(Keyword::Desc),
            ]
        );
    }

    #[test]
    fn lex_quoted_ident_with_hyphen() {
        let toks = collect("\"My-Table\"").unwrap();
        assert_eq!(toks, vec![Token::QuotedIdent("My-Table".into())]);
    }

    #[test]
    fn lex_number_int_float_and_exponent() {
        let toks = collect("42 3.14 1e10 1.5E-3").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Num("42".into()),
                Token::Num("3.14".into()),
                Token::Num("1e10".into()),
                Token::Num("1.5E-3".into()),
            ]
        );
    }

    #[test]
    fn lex_path_dot_does_not_consume_field() {
        // `Tags.field` — the dot should not be lexed as part of a number even
        // though it follows digits would be (no digits here, but check the
        // `path.field` form too).
        let toks = collect("Tags.field").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Ident("Tags".into()),
                Token::Dot,
                Token::Ident("field".into()),
            ]
        );
    }

    #[test]
    fn lex_function_name_stays_ident() {
        let toks = collect("begins_with(sk, 'p')").unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Ident("begins_with".into()),
                Token::LParen,
                Token::Ident("sk".into()),
                Token::Comma,
                Token::Str("p".into()),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn lex_unterminated_string_errors() {
        let err = collect("'abc").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("unterminated"), "got: {msg}");
    }

    #[test]
    fn lex_skips_line_and_block_comments() {
        let toks = collect(
            "-- a leading line comment\nSELECT /* block */ a /* multi\n line */ FROM t -- trailing",
        )
        .unwrap();
        assert_eq!(
            toks,
            vec![
                Token::Kw(Keyword::Select),
                Token::Ident("a".into()),
                Token::Kw(Keyword::From),
                Token::Ident("t".into()),
            ]
        );
    }
}
