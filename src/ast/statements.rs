use std::fmt;

use crate::{
    ast::{AST, DeclarationID, ExpressionID},
    token::Token,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatementID(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Expression {
        expression: ExpressionID,
    },
    Assignment {
        assignee: ExpressionID,
        assignment_operator: Token,
        assignment_value: ExpressionID,
    },
    Block {
        declarations: Vec<DeclarationID>,
    },
    Return {
        return_value: Option<ExpressionID>,
    },
}

impl<'a> AST<'a> {
    pub fn add_statement_expression(&mut self, expression: ExpressionID) -> StatementID {
        let index = self.statements.len();

        self.statements.push(Statement::Expression { expression });

        StatementID(index as u32)
    }

    pub fn add_statement_assignment(
        &mut self,
        assignee: ExpressionID,
        assignment_operator: Token,
        assignment_value: ExpressionID,
    ) -> StatementID {
        let index = self.statements.len();

        self.statements.push(Statement::Assignment {
            assignee,
            assignment_operator,
            assignment_value,
        });

        StatementID(index as u32)
    }

    pub fn add_statement_block(&mut self, declarations: Vec<DeclarationID>) -> StatementID {
        let index = self.statements.len();

        self.statements.push(Statement::Block { declarations });

        StatementID(index as u32)
    }

    pub fn add_statement_return(&mut self, return_value: Option<ExpressionID>) -> StatementID {
        let index = self.statements.len();

        self.statements.push(Statement::Return { return_value });

        StatementID(index as u32)
    }

    pub fn get_statement(&self, id: StatementID) -> Option<&Statement> {
        self.statements.get(id.0 as usize)
    }
}

impl fmt::Display for StatementID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
