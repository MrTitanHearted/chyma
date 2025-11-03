use std::fmt;

use crate::{
    ast::{AST, DeclarationID, ExpressionID},
    token::Token,
};

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        return_token: Token,
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

    pub fn add_statement_return(&mut self, return_token: Token, return_value: Option<ExpressionID>) -> StatementID {
        let index = self.statements.len();

        self.statements.push(Statement::Return { return_token, return_value });

        StatementID(index as u32)
    }

    pub fn get_statement(&self, id: StatementID) -> Option<&Statement> {
        self.statements.get(id.0 as usize)
    }
}

impl Statement {
    pub fn accept_visitor<T>(&self, visitor: &mut impl IStatementVisitor<T>) -> T {
        match self {
            Statement::Expression { expression } => visitor.visit_statement_expression(expression),
            Statement::Assignment {
                assignee,
                assignment_operator,
                assignment_value,
            } => {
                visitor.visit_statement_assignment(assignee, assignment_operator, assignment_value)
            }
            Statement::Block { declarations } => visitor.visit_statement_block(declarations),
            Statement::Return { return_token, return_value } => visitor.visit_statement_return(return_token, return_value),
        }
    }
}

pub trait IStatementVisitor<T> {
    fn visit_statement_expression(&mut self, expression: &ExpressionID) -> T;

    fn visit_statement_assignment(
        &mut self,
        assignee: &ExpressionID,
        assignment_operator: &Token,
        assignment_value: &ExpressionID,
    ) -> T;

    fn visit_statement_block(&mut self, declarations: &[DeclarationID]) -> T;

    fn visit_statement_return(&mut self, return_token: &Token, return_value: &Option<ExpressionID>) -> T;
}

impl fmt::Display for StatementID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
