use std::fmt;

use crate::token::{KEYWORDS, Token, TokenKind, TokenStringInterner};

pub type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug)]
pub struct Lexer<'a, 'b> {
    interner: &'a mut TokenStringInterner,

    source: &'b str,

    start: usize,
    current: usize,

    line: usize,
    column: usize,

    current_column: usize,

    tokens: Vec<Token>,
}

impl<'a, 'b> Lexer<'a, 'b> {
    pub fn lex(interner: &'a mut TokenStringInterner, source: &'b str) -> LexerResult<Vec<Token>> {
        let mut lexer = Self::new(interner, source);

        while !lexer.is_at_end() {
            lexer.start = lexer.current;
            lexer.column = lexer.current_column;
            lexer.lex_token()?;
        }
        lexer.start = lexer.current;
        lexer.column = lexer.current_column;
        lexer.add_token(TokenKind::Eof);
        Ok(lexer.tokens)
    }

    fn new(interner: &'a mut TokenStringInterner, source: &'b str) -> Self {
        Self {
            interner,

            source,

            start: 0usize,
            current: 0usize,

            line: 1usize,
            column: 1usize,

            current_column: 1usize,

            tokens: Vec::new(),
        }
    }

    fn lex_token(&mut self) -> LexerResult<()> {
        let c = match self.advance() {
            Some(c) => c,
            None => return Ok(()),
        };

        match c {
            // Whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1usize;
                self.current_column = 1usize;
            }

            // Single-character tokens
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            '[' => self.add_token(TokenKind::LeftBracket),
            ']' => self.add_token(TokenKind::RightBracket),
            ',' => self.add_token(TokenKind::Comma),
            ';' => self.add_token(TokenKind::Semicolon),
            ':' => self.add_token(TokenKind::Colon),
            '~' => self.add_token(TokenKind::Tilde),
            '@' => self.add_token(TokenKind::At),
            '#' => self.add_token(TokenKind::Hash),

            // Operators that might be compound
            '.' => {
                if self.match_char_then_advance('.') {
                    if self.match_char_then_advance('=') {
                        self.add_token(TokenKind::DotDotEqual)
                    } else {
                        self.add_token(TokenKind::DotDot)
                    }
                } else {
                    self.add_token(TokenKind::Dot)
                }
            }
            '+' => {
                if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::PlusEqual);
                } else {
                    self.add_token(TokenKind::Plus);
                }
            }
            '-' => {
                if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::MinusEqual);
                } else {
                    self.add_token(TokenKind::Minus);
                }
            }
            '*' => {
                if self.match_char_then_advance('/') {
                    return Err(LexerError::unexpected_block_comment(self.line, self.column));
                } else if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::StarEqual);
                } else {
                    self.add_token(TokenKind::Star);
                }
            }
            '/' => {
                if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::SlashEqual);
                } else if self.match_char_then_advance('/') {
                    // Line comment - consume until end of line
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char_then_advance('*') {
                    // Block comment - consume until */
                    self.skip_block_comment()?;
                } else {
                    self.add_token(TokenKind::Slash);
                }
            }
            '%' => {
                if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::PercentEqual);
                } else {
                    self.add_token(TokenKind::Percent);
                }
            }
            '^' => {
                if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::CaretEqual);
                } else {
                    self.add_token(TokenKind::Caret);
                }
            }
            '&' => {
                if self.match_char_then_advance('&') {
                    self.add_token(TokenKind::AmpersandAmpersand);
                } else if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::AmpersandEqual);
                } else {
                    self.add_token(TokenKind::Ampersand);
                }
            }
            '|' => {
                if self.match_char_then_advance('|') {
                    self.add_token(TokenKind::PipePipe);
                } else if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::PipeEqual);
                } else {
                    self.add_token(TokenKind::Pipe);
                }
            }
            '=' => {
                if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::EqualEqual);
                } else {
                    self.add_token(TokenKind::Equal);
                }
            }
            '!' => {
                if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::BangEqual);
                } else {
                    self.add_token(TokenKind::Bang);
                }
            }
            '<' => {
                if self.match_char_then_advance('<') {
                    if self.match_char_then_advance('=') {
                        self.add_token(TokenKind::ShiftLeftEqual);
                    } else {
                        self.add_token(TokenKind::ShiftLeft);
                    }
                } else if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::LessEqual);
                } else {
                    self.add_token(TokenKind::Less);
                }
            }
            '>' => {
                if self.match_char_then_advance('>') {
                    if self.match_char_then_advance('=') {
                        self.add_token(TokenKind::ShiftRightEqual);
                    } else {
                        self.add_token(TokenKind::ShiftRight);
                    }
                } else if self.match_char_then_advance('=') {
                    self.add_token(TokenKind::GreaterEqual);
                } else {
                    self.add_token(TokenKind::Greater);
                }
            }

            // String literals
            '"' => self.add_string()?,

            // Character literals
            '\'' => self.add_char_literal()?,

            // Numbers
            c if c.is_ascii_digit() => self.add_number()?,

            // Identifiers and keywords
            c if Self::is_alpha(c) => self.add_keyword_or_identifier()?,

            // Unexpected character
            _ => {
                return Err(LexerError::unexpected_character(
                    self.line,
                    self.current_column,
                ));
            }
        }

        Ok(())
    }

    fn match_char_then_advance(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        let line = self.line as u32;
        let column = self.column as u32;
        let lexeme = &self.source[self.start..self.current];
        let lexeme = self.interner.intern_str(lexeme);

        self.tokens.push(Token {
            kind,
            lexeme,
            line,
            column,
        });
    }

    fn skip_block_comment(&mut self) -> LexerResult<()> {
        let mut depth = 1;

        while depth > 0 && !self.is_at_end() {
            if self.peek() == Some('/') && self.peek_next() == Some('*') {
                if let Some('\n') = self.advance() {
                    self.line += 1usize;
                    self.current_column = 1usize;
                } // consume '/'
                if let Some('\n') = self.advance() {
                    self.line += 1usize;
                    self.current_column = 1usize;
                } // consume '*'
                depth += 1;
            } else if self.peek() == Some('*') && self.peek_next() == Some('/') {
                if let Some('\n') = self.advance() {
                    self.line += 1usize;
                    self.current_column = 1usize;
                } // consume '*'
                if let Some('\n') = self.advance() {
                    self.line += 1usize;
                    self.current_column = 1usize;
                } // consume '/'
                depth -= 1;
            } else {
                if let Some('\n') = self.advance() {
                    self.line += 1usize;
                    self.current_column = 1usize;
                }
            }
        }

        if depth > 0 {
            return Err(LexerError::unexpected_eof(self.line, self.current_column));
        }

        Ok(())
    }

    fn add_string(&mut self) -> LexerResult<()> {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
                self.current_column = 1;
            }
            if self.peek() == Some('\\') {
                self.advance(); // consume the backslash
                if !self.is_at_end() {
                    self.advance(); // consume the escaped character
                }
            } else {
                self.advance();
            }
        }

        if self.is_at_end() {
            return Err(LexerError::unterminated_string(
                self.line,
                self.current_column,
            ));
        }

        // Consume the closing quote
        self.advance();
        self.add_token(TokenKind::StringLiteral);
        Ok(())
    }

    fn add_char_literal(&mut self) -> LexerResult<()> {
        if self.is_at_end() {
            return Err(LexerError::unterminated_string(
                self.line,
                self.current_column,
            ));
        }

        // Handle escaped characters
        if self.peek() == Some('\\') {
            self.advance(); // consume backslash
            if self.is_at_end() {
                return Err(LexerError::unterminated_string(
                    self.line,
                    self.current_column,
                ));
            }
            self.advance(); // consume escaped character
        } else {
            self.advance(); // consume the character
        }

        if self.peek() != Some('\'') {
            return Err(LexerError::unterminated_string(
                self.line,
                self.current_column,
            ));
        }

        // Consume the closing quote
        self.advance();
        self.add_token(TokenKind::CharLiteral);
        Ok(())
    }

    fn add_number(&mut self) -> LexerResult<()> {
        // Handle hexadecimal numbers
        if self.source[self.start..].starts_with("0x")
            || self.source[self.start..].starts_with("0X")
        {
            self.advance(); // consume 'x' or 'X'
            while self.peek().map_or(false, |c| c.is_ascii_hexdigit()) {
                self.advance();
            }
            // Handle type suffix (e.g., u8, i32)
            self.consume_number_suffix();
            self.add_token(TokenKind::IntegerLiteral);
            return Ok(());
        }

        // Handle binary numbers
        if self.source[self.start..].starts_with("0b")
            || self.source[self.start..].starts_with("0B")
        {
            self.advance(); // consume 'b' or 'B'
            while self.peek() == Some('0') || self.peek() == Some('1') {
                self.advance();
            }
            // Handle type suffix
            self.consume_number_suffix();
            self.add_token(TokenKind::IntegerLiteral);
            return Ok(());
        }

        // Handle decimal numbers
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }

        // Check for float (decimal point followed by digits)
        let mut is_float = false;
        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            is_float = true;
            self.advance(); // consume '.'
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        // Handle scientific notation
        if self.peek() == Some('e') || self.peek() == Some('E') {
            is_float = true;
            self.advance(); // consume 'e' or 'E'
            if self.peek() == Some('+') || self.peek() == Some('-') {
                self.advance(); // consume sign
            }
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        // Handle type suffix
        self.consume_number_suffix();

        if is_float {
            self.add_token(TokenKind::FloatLiteral);
        } else {
            self.add_token(TokenKind::IntegerLiteral);
        }
        Ok(())
    }

    fn consume_number_suffix(&mut self) {
        // Consume type suffixes like u8, i32, f64, etc.
        if Self::is_alpha(self.peek().unwrap_or('\0')) {
            while Self::is_alpha_numeric(self.peek().unwrap_or('\0')) {
                self.advance();
            }
        }
    }

    fn add_keyword_or_identifier(&mut self) -> LexerResult<()> {
        while Self::is_alpha_numeric(self.peek().unwrap_or('\0')) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let kind = KEYWORDS.get(text).copied().unwrap_or(TokenKind::Identifier);
        self.add_token(kind);
        Ok(())
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.current += c.len_utf8();
            self.current_column += 1usize;
            Some(c)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            Some('\0')
        } else {
            self.source[self.current..].chars().next()
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            Some('\0')
        } else {
            self.source[self.current..].chars().nth(1)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || c.is_ascii_digit()
    }
}

#[derive(Debug, Clone)]
pub struct LexerError {
    line: usize,
    column: usize,
    kind: LexerErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LexerErrorKind {
    UnexpectedCharacter,    // a character that cannot be tokenized
    UnexpectedBlockComment, // unexpected block comment
    UnterminatedString,     // string literal not closed
    UnexpectedEof,          // EOF reached unexpectedly
}

impl LexerError {
    fn unexpected_character(line: usize, column: usize) -> Self {
        LexerError {
            line,
            column,
            kind: LexerErrorKind::UnexpectedCharacter,
        }
    }

    fn unexpected_block_comment(line: usize, column: usize) -> Self {
        LexerError {
            line,
            column,
            kind: LexerErrorKind::UnexpectedBlockComment,
        }
    }

    fn unterminated_string(line: usize, column: usize) -> Self {
        LexerError {
            line,
            column,
            kind: LexerErrorKind::UnterminatedString,
        }
    }

    fn unexpected_eof(line: usize, column: usize) -> Self {
        LexerError {
            line,
            column,
            kind: LexerErrorKind::UnexpectedEof,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}, column {}]::[Lexer Error]: {}",
            self.line,
            self.column,
            match self.kind {
                LexerErrorKind::UnexpectedCharacter => "Unexpected character",
                LexerErrorKind::UnexpectedBlockComment =>
                    "Unexpected closing of unopened block comment",
                LexerErrorKind::UnterminatedString => "Unterminated string literal",
                LexerErrorKind::UnexpectedEof => "Unexpected end of file",
            },
        )
    }
}

impl std::error::Error for LexerError {}
