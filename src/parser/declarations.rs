use crate::{
    ast::DeclarationID,
    parser::{Parser, ParserError, ParserResult, Precedence},
    token::TokenKind,
};

impl Parser {
    pub(super) fn parse_declaration(&mut self) -> ParserResult<DeclarationID> {
        let current = self.get_current().unwrap().clone();

        match current.kind {
            TokenKind::Fun => self.parse_declaration_function(),
            TokenKind::Let => self.parse_declaration_let(),
            _ => self.parse_declaration_statement(),
        }
    }

    fn parse_declaration_function(&mut self) -> ParserResult<DeclarationID> {
        let fun_token = self.advance().unwrap().clone();

        let identifier = self
            .consume(
                TokenKind::Identifier,
                ParserError::expected_identifier(self.get_current().unwrap().clone()),
            )?
            .clone();

        let left_paren = self
            .consume(
                TokenKind::LeftParen,
                ParserError::expected_left_paren(self.get_current().unwrap().clone()),
            )?
            .clone();

        let mut parameters = Vec::new();

        if Some(TokenKind::RightParen) != self.get_current_kind() {
            loop {
                let name = self
                    .consume(
                        TokenKind::Identifier,
                        ParserError::expected_identifier(self.get_current().unwrap().clone()),
                    )?
                    .clone();

                let _colon = self
                    .consume(
                        TokenKind::Colon,
                        ParserError::expected_colon(self.get_current().unwrap().clone()),
                    )?
                    .clone();

                let ty = self.parse_type()?;

                parameters.push((name, ty));

                if Some(TokenKind::Comma) == self.get_current_kind() {
                    let _comma = self
                        .consume(
                            TokenKind::Comma,
                            ParserError::expected_comma(self.get_current().unwrap().clone()),
                        )?
                        .clone();

                    if Some(TokenKind::RightParen) == self.get_current_kind() {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        let right_paren = self
            .consume(
                TokenKind::RightParen,
                ParserError::expected_right_paren(self.get_current().unwrap().clone()),
            )?
            .clone();

        let return_type = if let Some(TokenKind::Colon) = self.get_current_kind() {
            let _colon = self.advance().unwrap().clone();
            let ty = self.parse_type()?;
            Some(ty)
        } else {
            None
        };

        let body = self.parse_statement()?;

        Ok(self.ast.add_declaration_function(
            fun_token,
            identifier,
            left_paren,
            parameters,
            right_paren,
            return_type,
            body,
        ))
    }

    fn parse_declaration_let(&mut self) -> ParserResult<DeclarationID> {
        let let_token = self.advance().unwrap().clone();

        let mut_token = if Some(TokenKind::Mut) == self.get_current_kind() {
            Some(self.advance().unwrap().clone())
        } else {
            None
        };

        let identifier = self
            .consume(
                TokenKind::Identifier,
                ParserError::expected_identifier(self.get_current().unwrap().clone()),
            )?
            .clone();

        let type_token = if Some(TokenKind::Colon) == self.get_current_kind() {
            let _colon = self.advance().unwrap().clone();
            let ty = self.parse_type()?;
            Some(ty)
        } else {
            None
        };

        let initializer = if Some(TokenKind::Equal) == self.get_current_kind() {
            let _equal = self.advance().unwrap().clone();
            let initializer = self.parse_expression(Precedence::lowest())?;
            Some(initializer)
        } else {
            None
        };

        let _semicolon = self.consume(
            TokenKind::Semicolon,
            ParserError::expected_semicolon(self.get_current().unwrap().clone()),
        )?;

        Ok(self
            .ast
            .add_declaration_let(let_token, mut_token, identifier, type_token, initializer))
    }

    fn parse_declaration_statement(&mut self) -> ParserResult<DeclarationID> {
        let statement = self.parse_statement()?;

        Ok(self.ast.add_declaration_statement(statement))
    }
}
