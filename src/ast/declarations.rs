use std::fmt;

use crate::{
    ast::{AST, ExpressionID, StatementID, TypeID},
    token::Token,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeclarationID(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Statement {
        statement: StatementID,
    },
    Let {
        is_mutable: bool,
        identifier: Token,
        type_id: Option<TypeID>,
        initializer: Option<ExpressionID>,
    },
    Function {
        identifier: Token,
        parameters: Vec<(Token, TypeID)>,
        return_type: Option<TypeID>,
        body: StatementID,
    },
}

impl<'a> AST<'a> {
    pub fn add_declaration_statement(&mut self, statement: StatementID) -> DeclarationID {
        let index = self.declarations.len();

        self.declarations.push(Declaration::Statement { statement });

        DeclarationID(index as u32)
    }

    pub fn add_declaration_let(
        &mut self,
        is_mutable: bool,
        identifier: Token,
        type_id: Option<TypeID>,
        initializer: Option<ExpressionID>,
    ) -> DeclarationID {
        let index = self.declarations.len();

        self.declarations.push(Declaration::Let {
            is_mutable,
            identifier,
            type_id,
            initializer,
        });

        DeclarationID(index as u32)
    }

    pub fn add_declaration_function(
        &mut self,
        identifier: Token,
        parameters: Vec<(Token, TypeID)>,
        return_type: Option<TypeID>,
        body: StatementID,
    ) -> DeclarationID {
        let index = self.declarations.len();

        self.declarations.push(Declaration::Function {
            identifier,
            parameters,
            return_type,
            body,
        });

        DeclarationID(index as u32)
    }

    pub fn get_declaration(&self, declaration: DeclarationID) -> Option<&Declaration> {
        self.declarations.get(declaration.0 as usize)
    }
}

impl fmt::Display for DeclarationID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
