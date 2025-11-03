use crate::{
    ast::AST,
    token::{Token, TokenKind},
};

mod parser_error;

mod declarations;
mod expressions;
mod statements;
mod types;

pub use parser_error::*;

use crate::token::TokenStringInterner;
#[allow(unused_imports)]
pub use declarations::*;
pub use expressions::*;
#[allow(unused_imports)]
pub use statements::*;
#[allow(unused_imports)]
pub use types::*;

type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'a, 'b> {
    interner: &'a TokenStringInterner,
    ast: AST<'a>,
    tokens: &'b [Token],
    current: usize,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse(interner: &'a TokenStringInterner, tokens: &'b [Token]) -> ParserResult<AST<'a>> {
        let mut parser = Self::new(interner, tokens);

        while !parser.is_at_end() {
            parser.parse_declaration()?;
        }

        Ok(parser.ast)
    }

    fn new(interner: &'a TokenStringInterner, tokens: &'b [Token]) -> Self {
        Self {
            interner,
            ast: AST::new(interner),
            tokens,
            current: 0usize,
        }
    }

    fn consume(&mut self, kind: TokenKind, error: ParserError) -> ParserResult<&Token> {
        if Some(kind) == self.get_current_kind() {
            return Ok(self.advance().unwrap());
        }

        Err(error)
    }

    fn advance(&mut self) -> Option<&Token> {
        let current = self.current;
        self.current += 1;
        self.tokens.get(current)
    }

    fn get_current_kind(&self) -> Option<TokenKind> {
        self.get_current().map(|token| token.kind)
    }

    fn get_current(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn is_at_end(&self) -> bool {
        if let Some(kind) = self.get_current_kind() {
            return kind == TokenKind::Eof;
        }

        true
    }
}
