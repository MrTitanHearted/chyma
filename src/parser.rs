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

#[allow(unused_imports)]
pub use declarations::*;
pub use expressions::*;
#[allow(unused_imports)]
pub use statements::*;
#[allow(unused_imports)]
pub use types::*;

type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, Clone)]
pub struct Parser {
    ast: AST,
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            ast: AST::new(),
            tokens: Vec::new(),
            current: 0usize,
        }
    }

    pub fn parse(&mut self, tokens: &[Token]) -> ParserResult<AST> {
        self.ast = AST::new();
        self.tokens = tokens.to_vec();
        self.current = 0usize;

        while !self.is_at_end() {
            self.parse_declaration()?;
        }

        self.tokens.clear();
        Ok(self.ast.clone())
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
