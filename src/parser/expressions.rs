use crate::{
    ast::ExpressionID,
    parser::{Parser, ParserError, ParserResult},
    token::TokenKind,
};

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    #[default]
    None,

    // Assignment,     // =, +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=
    LogicalOr,      // ||, or
    LogicalAnd,     // &&, and
    BitwiseOr,      // |
    BitwiseXor,     // ^
    BitwiseAnd,     // &
    Equality,       // ==, !=
    Relational,     // <=, <, >, >=,
    BitwiseShift,   // <<, >>
    Additive,       // +, -
    Multiplicative, // *, /, %
    Unary,          // !, -, ~
    Primary,        // literals, identifiers
}

impl Precedence {
    pub(super) fn lowest() -> Self {
        Self::LogicalOr
    }

    pub(super) fn highest() -> Self {
        Self::Primary
    }

    pub(super) fn next(&self) -> Self {
        let value = *self as u8;
        let max = Self::highest() as u8;
        if value < max {
            unsafe { std::mem::transmute(value + 1) }
        } else {
            *self
        }
    }
}

pub(super) trait INudHandler {
    fn nud(&self, parser: &mut Parser) -> ParserResult<ExpressionID>;
}

pub(super) trait ILedHandler {
    fn led(&self, parser: &mut Parser, left: ExpressionID) -> ParserResult<ExpressionID>;

    fn get_precedence(&self) -> Precedence;
}

impl Parser {
    pub(super) fn parse_expression(&mut self, precedence: Precedence) -> ParserResult<ExpressionID> {
        let mut left = self.get_current_kind().unwrap().nud(self)?;

        loop {
            let kind = self.get_current_kind().unwrap();

            if kind.get_precedence() < precedence {
                break;
            }

            left = kind.led(self, left)?;
        }

        Ok(left)
    }

    fn parse_expression_grouping(&mut self) -> ParserResult<ExpressionID> {
        let _left_paren = self.advance().unwrap(); // for '('

        let expression = self.parse_expression(Precedence::lowest())?;

        let _right_paren = self.consume(
            TokenKind::RightParen,
            ParserError::expected_right_paren(self.get_current().unwrap().clone()),
        )?; // for ')'

        Ok(expression)
    }

    fn parse_expression_binary(
        &mut self,
        left: ExpressionID,
        precedence: Precedence,
    ) -> ParserResult<ExpressionID> {
        let operator = self.advance().unwrap().clone();

        let right = self.parse_expression(precedence.next())?;

        Ok(self.ast.add_expression_binary(left, operator, right))
    }

    fn parse_expression_unary(&mut self) -> ParserResult<ExpressionID> {
        let operator = self.advance().unwrap().clone();

        let right = self.parse_expression(Precedence::Unary)?;

        Ok(self.ast.add_expression_unary(operator, right))
    }

    fn parse_expression_primary(&mut self) -> ParserResult<ExpressionID> {
        let token = self.advance().unwrap().clone();

        match token.kind {
            TokenKind::IntegerLiteral
            | TokenKind::FloatLiteral
            | TokenKind::CharLiteral
            | TokenKind::StringLiteral
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Null => Ok(self.ast.add_expression_literal(token)),

            TokenKind::Identifier => Ok(self.ast.add_expression_identifier(token)),

            _ => Err(ParserError::unknown_primary(token)),
        }
    }
}

impl INudHandler for TokenKind {
    fn nud(&self, parser: &mut Parser) -> ParserResult<ExpressionID> {
        match self {
            TokenKind::LeftParen => parser.parse_expression_grouping(),

            TokenKind::Bang | TokenKind::Tilde | TokenKind::Plus | TokenKind::Minus => {
                parser.parse_expression_unary()
            }

            // Literals
            TokenKind::IntegerLiteral
            | TokenKind::FloatLiteral
            | TokenKind::CharLiteral
            | TokenKind::StringLiteral
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Null

            // Identifiers
            | TokenKind::Identifier => parser.parse_expression_primary(),

            _ => Err(ParserError::expected_nud_handler(
                parser.get_current().unwrap().clone(),
            )),
        }
    }
}

impl ILedHandler for TokenKind {
    fn led(&self, parser: &mut Parser, left: ExpressionID) -> ParserResult<ExpressionID> {
        match self {
            // Logical
            TokenKind::Or
            | TokenKind::And
            | TokenKind::PipePipe
            | TokenKind::AmpersandAmpersand

            // Bitwise
            | TokenKind::Pipe
            | TokenKind::Caret
            | TokenKind::Ampersand

            // Additive
            | TokenKind::Plus
            | TokenKind::Minus

            // Equality
            | TokenKind::EqualEqual
            | TokenKind::BangEqual

            // Relational
            | TokenKind::LessEqual
            | TokenKind::Less
            | TokenKind::Greater
            | TokenKind::GreaterEqual

            // BitwiseShift
            | TokenKind::ShiftLeft | TokenKind::ShiftRight

            // Multiplicative
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent => parser.parse_expression_binary(left, self.get_precedence()),

            _ => Err(ParserError::expected_led_handler(
                parser.get_current().unwrap().clone(),
            )),
        }
    }

    fn get_precedence(&self) -> Precedence {
        match self {
            TokenKind::Or | TokenKind::PipePipe => Precedence::LogicalOr,
            TokenKind::And | TokenKind::AmpersandAmpersand => Precedence::LogicalAnd,
            TokenKind::Pipe => Precedence::BitwiseOr,
            TokenKind::Caret => Precedence::BitwiseXor,
            TokenKind::Ampersand => Precedence::BitwiseAnd,
            TokenKind::EqualEqual | TokenKind::BangEqual => Precedence::Equality,
            TokenKind::LessEqual
            | TokenKind::Less
            | TokenKind::Greater
            | TokenKind::GreaterEqual => Precedence::Relational,
            TokenKind::ShiftLeft | TokenKind::ShiftRight => Precedence::BitwiseShift,
            TokenKind::Plus | TokenKind::Minus => Precedence::Additive,
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Precedence::Multiplicative,

            _ => Precedence::None,
        }
    }
}
