use std::fmt;

use crate::token::{Token, TokenStringInterner};

#[derive(Debug, Clone)]
pub struct ParserError {
    token: Token,
    kind: ParserErrorKind,
}

impl ParserError {
    pub(super) fn expected_type(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedType,
        }
    }

    pub(super) fn expected_identifier(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedIdentifier,
        }
    }

    pub(super) fn expected_semicolon(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedSemicolon,
        }
    }

    pub(super) fn expected_colon(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedColon,
        }
    }

    pub(super) fn expected_comma(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedComma,
        }
    }

    pub(super) fn expected_nud_handler(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedNudHandler,
        }
    }

    pub(super) fn expected_led_handler(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedLedHandler,
        }
    }

    pub(super) fn expected_left_paren(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedLeftParen,
        }
    }

    pub(super) fn expected_right_paren(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedRightParen,
        }
    }

    pub(super) fn expected_right_brace(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::ExpectedRightBrace,
        }
    }

    pub(super) fn unknown_primary(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::UnknownPrimary,
        }
    }

    pub(super) fn unknown_type(token: Token) -> Self {
        Self {
            token,
            kind: ParserErrorKind::UnknownType,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserErrorKind {
    ExpectedType,
    ExpectedIdentifier,
    ExpectedSemicolon,
    ExpectedColon,
    ExpectedComma,
    ExpectedNudHandler,
    ExpectedLedHandler,
    ExpectedLeftParen,
    ExpectedRightParen,
    ExpectedRightBrace,
    UnknownPrimary,
    UnknownType,
}

impl fmt::Display for ParserError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("Use ParserErrorFormatter to display errors!")
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug)]
pub struct ParserErrorFormatter<'a, 'b> {
    interner: &'a TokenStringInterner,
    error: &'b ParserError,
}

impl<'a, 'b> ParserErrorFormatter<'a, 'b> {
    pub fn new(interner: &'a TokenStringInterner, error: &'b ParserError) -> Self {
        Self { interner, error }
    }
}

impl<'a, 'b> fmt::Display for ParserErrorFormatter<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}, column {}]::[Parser Error]: {}",
            self.error.token.line,
            self.error.token.column,
            match self.error.kind {
                ParserErrorKind::ExpectedType => format!(
                    "expected '<type>' instead of '<{}>' after variable declaration.",
                    self.error.token.kind
                ),
                ParserErrorKind::ExpectedIdentifier => format!(
                    "expected '<identifier>' instead of '<{}>' after variable declaration.",
                    self.error.token.kind
                ),
                ParserErrorKind::ExpectedSemicolon =>
                    format!("expected '<;>' instead of '<{}>'.", self.error.token.kind),
                ParserErrorKind::ExpectedColon =>
                    format!("expected '<:>' instead of '<{}>'.", self.error.token.kind),
                ParserErrorKind::ExpectedComma =>
                    format!("expected '<,>' instead of '<{}>'.", self.error.token.kind),
                ParserErrorKind::ExpectedNudHandler => format!(
                    "expected nud handler for '<{}>' token kind.",
                    self.error.token.kind
                ),
                ParserErrorKind::ExpectedLedHandler => format!(
                    "expected led handler for '<{}>' token kind.",
                    self.error.token.kind
                ),
                ParserErrorKind::ExpectedLeftParen =>
                    format!("expected '<(>' instead of '<{}>'.", self.error.token.kind),
                ParserErrorKind::ExpectedRightParen =>
                    format!("expected '<)>' instead of '<{}>'.", self.error.token.kind),
                ParserErrorKind::ExpectedRightBrace =>
                    format!("expected '<}}>' instead of '<{}>'.", self.error.token.kind),
                ParserErrorKind::UnknownPrimary => format!(
                    "'{}' is unknown primary.",
                    self.interner.get_str_or_empty(self.error.token.lexeme)
                ),
                ParserErrorKind::UnknownType => format!(
                    "'{}' is unknown type.",
                    self.interner.get_str_or_empty(self.error.token.lexeme)
                ),
            }
        )
    }
}

impl std::error::Error for ParserErrorFormatter<'_, '_> {}
