use crate::{
    ast::{ExpressionID, StatementID},
    parser::{Parser, ParserError, ParserResult, Precedence},
    token::TokenKind,
};

impl<'a, 'b> Parser<'a, 'b> {
    pub(super) fn parse_statement(&mut self) -> ParserResult<StatementID> {
        let current_kind = self.get_current_kind().unwrap();

        match current_kind {
            TokenKind::LeftBrace => self.parse_statement_block(),
            TokenKind::Return => self.parse_statement_return(),
            _ => self.parse_statement_expression(),
        }
    }

    fn parse_statement_block(&mut self) -> ParserResult<StatementID> {
        let _left_brace = self.advance().unwrap().clone();

        let mut declarations = Vec::new();

        while Some(TokenKind::RightBrace) != self.get_current_kind() {
            declarations.push(self.parse_declaration()?);
        }

        let _right_brace = self.consume(
            TokenKind::RightBrace,
            ParserError::expected_right_brace(self.get_current().unwrap().clone()),
        )?;

        Ok(self.ast.add_statement_block(declarations))
    }

    fn parse_statement_return(&mut self) -> ParserResult<StatementID> {
        let _return_token = self.advance().unwrap().clone();

        let return_value = if Some(TokenKind::Semicolon) != self.get_current_kind() {
            let expression = self.parse_expression(Precedence::lowest())?;
            Some(expression)
        } else {
            None
        };

        let _semicolon = self
            .consume(
                TokenKind::Semicolon,
                ParserError::expected_semicolon(self.get_current().unwrap().clone()),
            )?
            .clone();

        Ok(self.ast.add_statement_return(return_value))
    }

    fn parse_statement_expression(&mut self) -> ParserResult<StatementID> {
        let expression = self.parse_expression(Precedence::lowest())?;

        if let Some(
            TokenKind::Equal
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::StarEqual
            | TokenKind::SlashEqual
            | TokenKind::PercentEqual
            | TokenKind::CaretEqual
            | TokenKind::AmpersandEqual
            | TokenKind::PipeEqual
            | TokenKind::ShiftLeftEqual
            | TokenKind::ShiftRightEqual,
        ) = self.get_current_kind()
        {
            return self.parse_statement_assignment(expression);
        }

        self.consume(
            TokenKind::Semicolon,
            ParserError::expected_semicolon(self.get_current().unwrap().clone()),
        )?;

        Ok(self.ast.add_statement_expression(expression))
    }

    fn parse_statement_assignment(&mut self, assignee: ExpressionID) -> ParserResult<StatementID> {
        let assignment_operator = self.advance().unwrap().clone();
        let assignment_value = self.parse_expression(Precedence::lowest())?;

        let _semicolon = self.consume(
            TokenKind::Semicolon,
            ParserError::expected_semicolon(self.get_current().unwrap().clone()),
        );

        Ok(self
            .ast
            .add_statement_assignment(assignee, assignment_operator, assignment_value))
    }
}
