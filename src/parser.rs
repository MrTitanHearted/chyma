use std::fmt;

use crate::{
    ast::{DisplayOption, Expr, ExprId, Type},
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
        if let Some(TokenKind::Eof) = tokens.iter().next().map(|token| token.kind) {
            return Err(ParserError::empty_file());
        }

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

    fn parse_type(&mut self) -> ParserResult<Box<Type>> {
        let current = self.advance()?;
        if current.is_primitive_type() {
            return Ok(Type::primitive(current));
        } else if current.kind == TokenKind::Star {
            return self.parse_type_reference();
        } else if current.kind == TokenKind::LeftBracket {
            return self.parse_type_array();
        } else if current.kind == TokenKind::LeftParen {
            return self.parse_type_tuple();
        } else if current.is_type_start() {
            return self.parse_type_optional();
        } else if current.is_function_type() {
            return self.parse_type_function();
        }

        todo!()
    }

    fn parse_type_reference(&mut self) -> ParserResult<Box<Type>> {
        let reference = self.peek()?;
        let const_keyword = DisplayOption(if let Ok(keyword) = self.peek() {
            if keyword.kind == TokenKind::Const {
                self.advance()?;
                Some(keyword)
            } else {
                None
            }
        } else {
            None
        });
        let r#type = self.parse_type()?;

        Ok(Type::reference(reference, const_keyword, r#type))
    }

    fn parse_type_array(&mut self) -> ParserResult<Box<Type>> {
        todo!()
    }

    fn parse_type_tuple(&mut self) -> ParserResult<Box<Type>> {
        todo!()
    }

    fn parse_type_optional(&mut self) -> ParserResult<Box<Type>> {
        self.parse_type_result()?;
        todo!()
    }

    fn parse_type_result(&mut self) -> ParserResult<Box<Type>> {
        todo!()
    }

    fn parse_type_function(&mut self) -> ParserResult<Box<Type>> {
        todo!()
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

    fn parse_expression_grouping(&mut self) -> ParserResult<Box<Expr>> {
        self.advance()?; // for '('
        let expression = self.parse_expression(Precedence::lowest())?;
        self.consume(
            TokenKind::RightParen,
            ParserError::expected_right_paren_after_expression(self.previous()?),
        )?;
        Ok(expression)
    }

    fn parse_expression_assignment(&mut self, left: &Box<Expr>) -> ParserResult<Box<Expr>> {
        let operator = self.advance()?;

        let right = self.parse_expression(Precedence::lowest())?;

        Ok(Expr::assignment(
            self.generate_next_expr_id(),
            left.clone(),
            operator,
            right,
        ))
    }

    fn parse_expression_binary(
        &mut self,
        left: &Box<Expr>,
        precedence: Precedence,
    ) -> ParserResult<Box<Expr>> {
        let id = self.generate_next_expr_id();

        let left = left.clone();

        let operator = self.advance()?;

        let right = self.parse_expression(precedence.next())?;

        Ok(match operator.kind {
            TokenKind::Or
            | TokenKind::PipePipe
            | TokenKind::And
            | TokenKind::AmpersandAmpersand => Expr::logical(id, left, operator, right),

            TokenKind::EqualEqual | TokenKind::BangEqual => {
                Expr::equality(id, left, operator, right)
            }

            TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual => Expr::relational(id, left, operator, right),

            TokenKind::DotDot | TokenKind::DotDotEqual => Expr::range(id, left, operator, right),

            TokenKind::Pipe | TokenKind::Caret | TokenKind::Ampersand => {
                Expr::bitwise(id, left, operator, right)
            }

            TokenKind::ShiftLeft | TokenKind::ShiftRight => Expr::shift(id, left, operator, right),

            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent => Expr::arithmetic(id, left, operator, right),

            _ => Err(ParserError::unexpected_binary_operator(operator))?,
        })
    }

    fn parse_expression_cast(&mut self) -> ParserResult<Box<Expr>> {
        let as_token = self.advance()?;
        let r#type = self.parse_type()?;

        Ok(Expr::cast(self.generate_next_expr_id(), as_token, r#type))
    }

    fn parse_expression_unary(&mut self) -> ParserResult<Box<Expr>> {
        let operator = self
            .advance()
            .map_err(|err| ParserError::expected_unary_operator(err.token))?;

        let right = self.parse_expression(Precedence::Unary)?;

        Ok(Expr::unary(self.generate_next_expr_id(), operator, right))
    }

    fn parse_expression_primary(&mut self) -> ParserResult<Box<Expr>> {
        let token = self
            .advance()
            .map_err(|err| ParserError::expected_literal(err.token))?;

        match token.kind {
            TokenKind::IntegerLiteral
            | TokenKind::FloatLiteral
            | TokenKind::CharLiteral
            | TokenKind::StringLiteral
            | TokenKind::True
            | TokenKind::False
            | TokenKind::SelfValue
            | TokenKind::Null => Ok(Expr::literal(self.generate_next_expr_id(), token)),

            TokenKind::Identifier => Ok(Expr::variable(self.generate_next_expr_id(), token)),

            _ => Err(ParserError::wrong_primary(token)),
        }
    }

    fn generate_next_expr_id(&mut self) -> usize {
        let id = self.next_expr_id;
        self.next_expr_id += 1;
        id
    }

    // fn match_and_advance(&mut self, kinds: &[TokenKind]) -> bool {
    //     if let Ok(current_kind) = self.get_current_kind() {
    //         for kind in kinds {
    //             if kind == &current_kind {
    //                 return true;
    //             }
    //         }
    //     }
    //     false
    // }

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
    Assignment,     // (=, +=, -=, *=, /=, %=,&=, |=, ^=, <<=, >>=)
    LogicalOr,      // (or, ||)
    LogicalAnd,     // (and, &&)
    Equality,       // (==, !=)
    Relational,     // (<, >, <=, >=)
    Range,          // (.., ..=)
    BitwiseOr,      // (|)
    BitwiseXor,     // (^)
    BitwiseAnd,     // (&)
    Shift,          // (<<, >>)
    Additive,       // (+, -)
    Multiplicative, // (*, /, %)
    Unary,          // (!, -, ~, *, &)
    Primary,        //
}

impl Precedence {
    fn lowest() -> Self {
        Self::Assignment
    }

    fn highest() -> Self {
        Self::Primary
    }

    fn next(&self) -> Self {
        let val = *self as u8;
        let max = Precedence::highest() as u8;
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
            TokenKind::As => parser.parse_expression_cast(),

            TokenKind::Bang
            | TokenKind::Minus
            | TokenKind::Tilde
            | TokenKind::Star
            | TokenKind::Ampersand => parser.parse_expression_unary(),

            TokenKind::IntegerLiteral
            | TokenKind::FloatLiteral
            | TokenKind::True
            | TokenKind::False
            | TokenKind::CharLiteral
            | TokenKind::StringLiteral
            | TokenKind::SelfValue
            | TokenKind::Identifier
            | TokenKind::Null => parser.parse_expression_primary(),

            TokenKind::LeftParen => parser.parse_expression_grouping(),

            _ => Err(ParserError::expected_expression(parser.previous()?)),
        }
    }
}

impl ILedHandler for TokenKind {
    fn led(&self, parser: &mut Parser, left: &Box<Expr>) -> ParserResult<Box<Expr>> {
        match self {
            // Assignment
            TokenKind::Equal
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::StarEqual
            | TokenKind::SlashEqual
            | TokenKind::PercentEqual
            | TokenKind::CaretEqual
            | TokenKind::ShiftLeftEqual
            | TokenKind::ShiftRightEqual => parser.parse_expression_assignment(left),
            // Logical
            TokenKind::Or
            | TokenKind::PipePipe
            | TokenKind::And
            | TokenKind::AmpersandAmpersand
            // Equality
            | TokenKind::EqualEqual
            | TokenKind::BangEqual
            // Relational
            | TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual
            //  Range
            | TokenKind::DotDot
            | TokenKind::DotDotEqual
            //  Bitwise
            | TokenKind::Pipe
            | TokenKind::Caret
            | TokenKind::Ampersand
            //  Shift
            | TokenKind::ShiftLeft
            | TokenKind::ShiftRight
            // Arithmatic
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent => parser.parse_expression_binary(left, self.get_precedence()),
            _ => Err(ParserError::expected_led_handler(parser.previous()?)),
        }
    }

    fn get_precedence(&self) -> Precedence {
        match self {
            TokenKind::Equal
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::StarEqual
            | TokenKind::SlashEqual
            | TokenKind::PercentEqual
            | TokenKind::CaretEqual
            | TokenKind::ShiftLeftEqual
            | TokenKind::ShiftRightEqual => Precedence::Assignment,

            TokenKind::Or | TokenKind::PipePipe => Precedence::LogicalOr,

            TokenKind::And | TokenKind::AmpersandAmpersand => Precedence::LogicalAnd,

            TokenKind::EqualEqual | TokenKind::BangEqual => Precedence::Equality,

            TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual => Precedence::Relational,

            TokenKind::DotDot | TokenKind::DotDotEqual => Precedence::Range,

            TokenKind::Pipe => Precedence::BitwiseOr,
            TokenKind::Caret => Precedence::BitwiseXor,
            TokenKind::Ampersand => Precedence::BitwiseAnd,

            TokenKind::ShiftLeft | TokenKind::ShiftRight => Precedence::Shift,

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
    EmptyFile,
    NoPreviousToken,
    WrongPrimary,
    ExpectedToken,
    ExpectedUnaryOperator,
    ExpectedLiteral,
    ExpectedLedHandler,
    ExpectedExpression,
    ExpectedRightParenAfterExpression,
    UnexpectedBinaryOperator,
}

impl ParserError {
    fn empty_file() -> Self {
        Self {
            kind: ParserErrorKind::EmptyFile,
            token: Token {
                kind: TokenKind::Eof,
                lexeme: String::new(),
                line: 0usize,
                column: 0usize,
            },
        }
    }

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

    fn expected_unary_operator(token: Token) -> Self {
        Self {
            kind: ParserErrorKind::ExpectedUnaryOperator,
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

    fn unexpected_binary_operator(token: Token) -> Self {
        Self {
            kind: ParserErrorKind::UnexpectedBinaryOperator,
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
                ParserErrorKind::EmptyFile => "empty file.",
                ParserErrorKind::NoPreviousToken =>
                    &format!("[Internal Error]: no previous token was found"),
                ParserErrorKind::WrongPrimary => &format!(
                    "cannot create a primary expression from {}",
                    &self.token.lexeme
                ),
                ParserErrorKind::ExpectedToken => "expected a token.",
                ParserErrorKind::ExpectedUnaryOperator =>
                    &format!("expected a unary operator after {}", &self.token.lexeme),
                ParserErrorKind::ExpectedLiteral => "expected a literal value after this token.",
                ParserErrorKind::ExpectedLedHandler => &format!(
                    "[Internal Error]: expected a led handler for token: {}",
                    self.token
                ),
                ParserErrorKind::ExpectedExpression =>
                    &format!("Expected expression after '{}'", &self.token.lexeme),
                ParserErrorKind::ExpectedRightParenAfterExpression =>
                    &format!("expected a ')' after expression: {}", &self.token.lexeme),

                ParserErrorKind::UnexpectedBinaryOperator =>
                    &format!("'{}' is not a valid binary operator.", &self.token.lexeme),
            }
        )
    }
}

impl std::error::Error for ParserError {}
