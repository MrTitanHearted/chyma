use std::fmt;

use crate::{
    ast::{Expr, ExprId},
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    next_expr_id: ExprId,
}

impl<'a> Parser<'a> {
    pub fn parse(tokens: &'a [Token]) -> ParserResult<Box<Expr>> {
        let mut parser = Self::new(tokens);

        parser.parse_expression(Precedence::lowest())
    }

    fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            current: 0usize,
            next_expr_id: 0usize,
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ParserResult<Box<Expr>> {
        let mut left = self.get_current_kind()?.nud(self)?;

        loop {
            let kind = self.get_current_kind()?;

            if kind.get_precedence() < precedence {
                break;
            }

            left = kind.led(self, &left)?;
        }

        Ok(left)
    }

    fn parse_expression_primary(&mut self) -> ParserResult<Box<Expr>> {
        let token = self
            .advance()
            .map_err(|err| ParserError::expected_literal(err.token))?;

        match token.kind {
            TokenKind::IntegerLiteral
            | TokenKind::FloatLiteral
            | TokenKind::StringLiteral
            | TokenKind::CharLiteral
            | TokenKind::Null => Ok(Expr::literal(self.generate_next_expr_id(), token)),
            _ => Err(ParserError::wrong_primary(token)),
        }
    }

    fn parse_expression_binary(
        &mut self,
        left: &Box<Expr>,
        precedence: Precedence,
    ) -> ParserResult<Box<Expr>> {
        let operator = self.advance()?;

        let right = self.parse_expression(precedence.next())?;

        Ok(Expr::binary(
            self.generate_next_expr_id(),
            left.clone(),
            operator,
            right,
        ))
    }

    fn parse_expression_grouping(&mut self) -> ParserResult<Box<Expr>> {
        self.advance()?; // for '('
        let expression = self.parse_expression(Precedence::lowest())?;
        self.consume(
            TokenKind::RightParen,
            ParserError::expected_right_paren_after_expression(self.previous()?),
        )?;
        Ok(expression)
    }

    fn generate_next_expr_id(&mut self) -> usize {
        let id = self.next_expr_id;
        self.next_expr_id += 1;
        id
    }

    fn consume(&mut self, kind: TokenKind, err: ParserError) -> ParserResult<Token> {
        if self.get_current_kind()? == kind {
            self.advance()
        } else {
            Err(err)
        }
    }

    fn advance(&mut self) -> ParserResult<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn previous(&self) -> ParserResult<Token> {
        if self.current == 0usize {
            Err(ParserError::no_previous_token())
        } else {
            Ok(self.tokens.get(self.current - 1usize).cloned().unwrap())
        }
    }

    fn peek(&self) -> ParserResult<Token> {
        self.tokens
            .get(self.current)
            .cloned()
            .ok_or_else(|| ParserError::expected_token(self.previous().unwrap()))
    }

    fn get_current_kind(&self) -> ParserResult<TokenKind> {
        self.tokens
            .get(self.current)
            .map(|token| token.kind)
            .ok_or_else(|| ParserError::expected_token(self.previous().unwrap()))
    }

    fn is_at_end(&self) -> bool {
        if let Ok(token) = self.peek() {
            return token.kind == TokenKind::Eof;
        }
        true
    }
}

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    #[default]
    None,
    Additive,
    Multiplicative,
    Primary,
}

impl Precedence {
    fn lowest() -> Self {
        Self::Additive
    }

    fn next(&self) -> Self {
        let val = *self as u8;
        let max = Precedence::Primary as u8;
        if val < max {
            unsafe { std::mem::transmute(val + 1) }
        } else {
            *self
        }
    }
}

trait INudHandler {
    fn nud(&self, parser: &mut Parser) -> ParserResult<Box<Expr>>;
}

trait ILedHandler {
    fn led(&self, parser: &mut Parser, left: &Box<Expr>) -> ParserResult<Box<Expr>>;
    fn get_precedence(&self) -> Precedence;
}

impl INudHandler for TokenKind {
    fn nud(&self, parser: &mut Parser) -> ParserResult<Box<Expr>> {
        match self {
            TokenKind::IntegerLiteral
            | TokenKind::FloatLiteral
            | TokenKind::StringLiteral
            | TokenKind::CharLiteral
            | TokenKind::Null => parser.parse_expression_primary(),

            TokenKind::LeftParen => parser.parse_expression_grouping(),

            _ => Err(ParserError::expected_expression(parser.previous()?)),
        }
    }
}

impl ILedHandler for TokenKind {
    fn led(&self, parser: &mut Parser, left: &Box<Expr>) -> ParserResult<Box<Expr>> {
        match self {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent => parser.parse_expression_binary(left, self.get_precedence()),
            _ => Err(ParserError::expected_led_handler(parser.previous()?)),
        }
    }

    fn get_precedence(&self) -> Precedence {
        match self {
            TokenKind::Plus | TokenKind::Minus => Precedence::Additive,

            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Precedence::Multiplicative,

            _ => Precedence::None,
        }
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, Clone)]
pub struct ParserError {
    kind: ParserErrorKind,
    token: Token,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserErrorKind {
    NoPreviousToken,
    WrongPrimary,
    ExpectedToken,
    ExpectedLiteral,
    ExpectedLedHandler,
    ExpectedExpression,
    ExpectedRightParenAfterExpression,
}

impl ParserError {
    fn no_previous_token() -> Self {
        Self {
            kind: ParserErrorKind::NoPreviousToken,
            token: Token {
                kind: TokenKind::Eof,
                lexeme: String::new(),
                line: 0usize,
                column: 0usize,
            },
        }
    }

    fn wrong_primary(token: Token) -> Self {
        Self {
            kind: ParserErrorKind::WrongPrimary,
            token,
        }
    }

    fn expected_token(token: Token) -> Self {
        Self {
            kind: ParserErrorKind::ExpectedToken,
            token,
        }
    }

    fn expected_literal(token: Token) -> Self {
        Self {
            kind: ParserErrorKind::ExpectedLiteral,
            token,
        }
    }

    fn expected_led_handler(token: Token) -> Self {
        Self {
            kind: ParserErrorKind::ExpectedLedHandler,
            token,
        }
    }

    fn expected_expression(token: Token) -> Self {
        Self {
            kind: ParserErrorKind::ExpectedExpression,
            token,
        }
    }

    fn expected_right_paren_after_expression(token: Token) -> Self {
        Self {
            kind: ParserErrorKind::ExpectedRightParenAfterExpression,
            token,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {} column {}]::[Parser Error]: {}",
            self.token.line,
            self.token.column,
            match self.kind {
                ParserErrorKind::NoPreviousToken =>
                    &format!("[Internal Error]: no previous token was found"),
                ParserErrorKind::WrongPrimary => &format!(
                    "cannot create a primary expression from {}",
                    &self.token.lexeme
                ),
                ParserErrorKind::ExpectedToken => "expected a token.",
                ParserErrorKind::ExpectedLiteral => "expected a literal value after this token.",
                ParserErrorKind::ExpectedLedHandler => &format!(
                    "[Internal Error]: expected a led handler for token: {}",
                    self.token
                ),
                ParserErrorKind::ExpectedExpression => &format!("Expected expression after '{}'", &self.token.lexeme),
                ParserErrorKind::ExpectedRightParenAfterExpression =>
                    &format!("expected a ')' after expression: {}", &self.token.lexeme),
            }
        )
    }
}

impl std::error::Error for ParserError {}
