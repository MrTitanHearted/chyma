use std::fmt;

use crate::{
    ast::{AST, ExpressionID, StatementID, TypeID},
    token::Token,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeclarationID(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Statement {
        statement: StatementID,
    },
    Let {
        let_token: Token,
        mut_token: Option<Token>,
        identifier: Token,
        type_token: Option<TypeID>,
        initializer: Option<ExpressionID>,
    },
    Function {
        fun_token: Token,
        identifier: Token,
        left_paren: Token,
        parameters: Vec<(Token, TypeID)>,
        right_paren: Token,
        return_type: Option<TypeID>,
        body: StatementID,
    },
}

impl AST {
    pub fn add_declaration_statement(&mut self, statement: StatementID) -> DeclarationID {
        let index = self.declarations.len();

        self.declarations.push(Declaration::Statement { statement });

        DeclarationID(index)
    }

    pub fn add_declaration_let(
        &mut self,
        let_token: Token,
        mut_token: Option<Token>,
        identifier: Token,
        type_token: Option<TypeID>,
        initializer: Option<ExpressionID>,
    ) -> DeclarationID {
        let index = self.declarations.len();

        self.declarations.push(Declaration::Let {
            let_token,
            mut_token,
            identifier,
            type_token,
            initializer,
        });

        DeclarationID(index)
    }

    pub fn add_declaration_function(
        &mut self,
        fun_token: Token,
        identifier: Token,
        left_paren: Token,
        parameters: Vec<(Token, TypeID)>,
        right_paren: Token,
        return_type: Option<TypeID>,
        body: StatementID,
    ) -> DeclarationID {
        let index = self.declarations.len();

        self.declarations.push(Declaration::Function {
            fun_token,
            identifier,
            left_paren,
            parameters,
            right_paren,
            return_type,
            body,
        });

        DeclarationID(index)
    }

    pub fn get_declaration(&self, declaration: DeclarationID) -> Option<&Declaration> {
        self.declarations.get(declaration.0)
    }
}

impl fmt::Display for DeclarationID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
