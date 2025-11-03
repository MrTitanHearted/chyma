use std::fmt;

use crate::{ast::AST, token::Token};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpressionID(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Binary {
        left: ExpressionID,
        operator: Token,
        right: ExpressionID,
    },
    Unary {
        operator: Token,
        right: ExpressionID,
    },
    Literal {
        token: Token,
    },
    Identifier {
        token: Token,
    },
}

impl<'a> AST<'a> {
    pub fn add_expression_binary(
        &mut self,
        left: ExpressionID,
        operator: Token,
        right: ExpressionID,
    ) -> ExpressionID {
        let index = self.expressions.len();

        self.expressions.push(Expression::Binary {
            left,
            operator,
            right,
        });

        ExpressionID(index as u32)
    }

    pub fn add_expression_unary(&mut self, operator: Token, right: ExpressionID) -> ExpressionID {
        let index = self.expressions.len();

        self.expressions.push(Expression::Unary { operator, right });

        ExpressionID(index as u32)
    }

    pub fn add_expression_literal(&mut self, token: Token) -> ExpressionID {
        let index = self.expressions.len();

        self.expressions.push(Expression::Literal { token });

        ExpressionID(index as u32)
    }

    pub fn add_expression_identifier(&mut self, token: Token) -> ExpressionID {
        let index = self.expressions.len();

        self.expressions.push(Expression::Identifier { token });

        ExpressionID(index as u32)
    }

    pub fn get_expression(&self, id: ExpressionID) -> Option<&Expression> {
        self.expressions.get(id.0 as usize)
    }
}

impl fmt::Display for ExpressionID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
