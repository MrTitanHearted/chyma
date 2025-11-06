use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::{
    ast::{
        AST, DeclarationID, IDeclarationVisitor, IExpressionVisitor, IStatementVisitor,
        ITypeVisitor, PrimitiveType, Type, TypeID,
    },
    token::{Token, TokenKind, TokenStringID},
};

#[derive(Debug)]
pub struct TypeResolver<'ast, 'interner> {
    ast: &'ast AST<'interner>,
    scopes: Vec<HashMap<TokenStringID, TypeID>>,
    visited_declarations: HashSet<DeclarationID>,

    current_function_expected_return_type_id: Option<TypeID>,
    current_function_found_return: bool,
}

impl<'ast, 'interner> TypeResolver<'ast, 'interner> {
    pub fn resolve(ast: &'ast AST<'interner>) -> TypeResolverResult<()> {
        let mut resolver = TypeResolver::new(ast);

        for declaration in (0..resolver.ast.get_declarations_count()).rev() {
            let declaration = DeclarationID(declaration);
            if !resolver.visited_declarations.contains(&declaration) {
                resolver.visited_declarations.insert(declaration);
                if let Some(declaration) = resolver.ast.get_declaration(declaration) {
                    declaration.accept_visitor(&mut resolver)?;
                }
            }
        }

        Ok(())
    }

    fn new(ast: &'ast AST<'interner>) -> Self {
        Self {
            ast,
            scopes: vec![HashMap::new()],
            visited_declarations: HashSet::new(),

            current_function_expected_return_type_id: None,
            current_function_found_return: false,
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn put_type(&mut self, name: TokenStringID, type_id: TypeID) {
        self.scopes.last_mut().map(|scope| {
            scope.insert(name, type_id);
        });
    }

    fn get_type_binary_expression(
        &self,
        left: TypeID,
        operator: Token,
        right: TypeID,
    ) -> TypeResolverResult<TypeID> {
        if left != right {
            return Err(TypeResolverError::new(
                operator,
                format!(
                    "mismatching types for operator '{}'.",
                    self.ast.get_interner().get_str_or_empty(operator.lexeme)
                ),
            ));
        }
        match operator.kind {
            TokenKind::PipePipe
            | TokenKind::Or
            | TokenKind::AmpersandAmpersand
            | TokenKind::And => match self.ast.get_type(left).unwrap() {
                Type::PrimitiveType(PrimitiveType::Bool) => return Ok(left),
                _ => {
                    return Err(TypeResolverError::new(
                        operator,
                        format!(
                            "'{}' operator expectes 'bool' type for both operrands",
                            self.ast.get_interner().get_str_or_empty(operator.lexeme)
                        ),
                    ));
                }
            },

            TokenKind::EqualEqual
            | TokenKind::BangEqual
            | TokenKind::LessEqual
            | TokenKind::Less
            | TokenKind::Greater
            | TokenKind::GreaterEqual => {
                Ok(self.ast.get_primitive_type_id(PrimitiveType::Bool).unwrap())
            }

            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent

            | TokenKind::Pipe
            | TokenKind::Caret
            | TokenKind::Ampersand => return match self.ast.get_type(left).unwrap() {
                Type::PrimitiveType(primitive_type) => match primitive_type {
                    PrimitiveType::I8
                    | PrimitiveType::I16
                    | PrimitiveType::I32
                    | PrimitiveType::I64
                    | PrimitiveType::I128
                    | PrimitiveType::ISize
                    | PrimitiveType::U8
                    | PrimitiveType::U16
                    | PrimitiveType::U32
                    | PrimitiveType::U64
                    | PrimitiveType::U128
                    | PrimitiveType::USize => Ok(left),
                    _ => {
                        Err(TypeResolverError::new(
                            operator,
                            format!(
                                "'{}' operator expects signed or unsigned integer primitive types.",
                                self.ast.get_interner().get_str_or_empty(operator.lexeme)
                            ),
                        ))
                    }
                },
            },

            _ => {
                return Err(TypeResolverError::new(
                    operator,
                    format!(
                        "
                How?"
                    ),
                ));
            }
        }
    }

    fn get_literal_type_id(&self, token: Token) -> TypeResolverResult<TypeID> {
        match token.kind {
            TokenKind::True | TokenKind::False => {
                Ok(self.ast.get_primitive_type_id(PrimitiveType::Bool).unwrap())
            }

            TokenKind::CharLiteral => {
                Ok(self.ast.get_primitive_type_id(PrimitiveType::Char).unwrap())
            }

            TokenKind::IntegerLiteral => {
                Ok(self.ast.get_primitive_type_id(PrimitiveType::I32).unwrap())
            }

            TokenKind::FloatLiteral => {
                Ok(self.ast.get_primitive_type_id(PrimitiveType::F32).unwrap())
            }

            _ => Err(TypeResolverError::new(
                token,
                format!(
                    "'{}' is not literal",
                    self.ast.get_interner().get_str_or_empty(token.lexeme)
                ),
            )),
        }
    }

    fn get_identifier_type_id(&self, token: Token) -> TypeResolverResult<TypeID> {
        for scope in self.scopes.iter().rev() {
            if let Some(type_id) = scope.get(&token.lexeme) {
                return Ok(*type_id);
            }
        }

        Err(TypeResolverError::new(
            token,
            format!("How? Semantic Resolver Should have detected this kind of error!"),
        ))
    }
}

impl<'ast, 'interner> IDeclarationVisitor<TypeResolverResult<()>>
    for TypeResolver<'ast, 'interner>
{
    fn visit_declaration_statement(
        &mut self,
        statement: &crate::ast::StatementID,
    ) -> TypeResolverResult<()> {
        if let Some(statement) = self.ast.get_statement(*statement) {
            statement.accept_visitor(self)?;
        }
        Ok(())
    }

    fn visit_declaration_let(
        &mut self,
        _is_mutable: &bool,
        identifier: &Token,
        type_id: &TypeID,
        initializer: &Option<crate::ast::ExpressionID>,
    ) -> TypeResolverResult<()> {
        if let Some(ty) = self.ast.get_type(*type_id) {
            ty.accept_visitor(self)?;
        }

        if let Some(initializer) = initializer {
            if let Some(initializer) = self.ast.get_expression(*initializer) {
                let expression_type = initializer.accept_visitor(self)?;
                if type_id != &expression_type {
                    return Err(TypeResolverError::new(
                        *identifier,
                        format!("assignee and assignment values have different types!"),
                    ));
                }
            }
        }

        self.put_type(identifier.lexeme, *type_id);

        Ok(())
    }

    fn visit_declaration_function(
        &mut self,
        name: &Token,
        parameters: &[(Token, TypeID)],
        return_type: &Option<TypeID>,
        body: &crate::ast::StatementID,
    ) -> TypeResolverResult<()> {
        if let Some(type_id) = return_type {
            if let Some(ty) = self.ast.get_type(*type_id) {
                ty.accept_visitor(self)?;
            }
        }

        self.push_scope();

        let enclosing_function_expected_return_type_id =
            self.current_function_expected_return_type_id;
        let enclosing_function_found_return = self.current_function_found_return;

        self.current_function_expected_return_type_id = *return_type;
        self.current_function_found_return = false;

        for (parameter, type_id) in parameters {
            self.put_type(parameter.lexeme, *type_id);
        }

        if let Some(statement) = self.ast.get_statement(*body) {
            statement.accept_visitor(self)?;
        }

        self.pop_scope();

        if self.current_function_expected_return_type_id.is_some()
            && !self.current_function_found_return
        {
            return Err(TypeResolverError::new(
                *name,
                format!(
                    "'{}' function did not have any return even though it has return type",
                    self.ast.get_interner().get_str_or_empty(name.lexeme)
                ),
            ));
        }

        self.current_function_expected_return_type_id = enclosing_function_expected_return_type_id;
        self.current_function_found_return = enclosing_function_found_return;

        Ok(())
    }
}

impl<'ast, 'interner> IExpressionVisitor<TypeResolverResult<TypeID>>
    for TypeResolver<'ast, 'interner>
{
    fn visit_expression_binary(
        &mut self,
        left: &crate::ast::ExpressionID,
        operator: &Token,
        right: &crate::ast::ExpressionID,
    ) -> TypeResolverResult<TypeID> {
        let left_type_id = self
            .ast
            .get_expression(*left)
            .unwrap()
            .accept_visitor(self)?;
        let right_type_id = self
            .ast
            .get_expression(*right)
            .unwrap()
            .accept_visitor(self)?;

        let type_id = self.get_type_binary_expression(left_type_id, *operator, right_type_id)?;

        self.ast
            .get_type(left_type_id)
            .unwrap()
            .accept_visitor(self)?;
        self.ast
            .get_type(right_type_id)
            .unwrap()
            .accept_visitor(self)?;
        self.ast.get_type(type_id).unwrap().accept_visitor(self)?;

        Ok(type_id)
    }

    fn visit_expression_unary(
        &mut self,
        operator: &Token,
        right: &crate::ast::ExpressionID,
    ) -> TypeResolverResult<TypeID> {
        let type_id = self
            .ast
            .get_expression(*right)
            .unwrap()
            .accept_visitor(self)?;

        self.ast.get_type(type_id).unwrap().accept_visitor(self)?;

        match self.ast.get_type(type_id).unwrap() {
            Type::PrimitiveType(primitive_type) => match primitive_type {
                PrimitiveType::I8
                | PrimitiveType::I16
                | PrimitiveType::I32
                | PrimitiveType::I64
                | PrimitiveType::I128
                | PrimitiveType::ISize
                | PrimitiveType::U8
                | PrimitiveType::U16
                | PrimitiveType::U32
                | PrimitiveType::U64
                | PrimitiveType::U128
                | PrimitiveType::USize => Ok(type_id),

                _ => Err(TypeResolverError::new(
                    *operator,
                    format!("will right some better error message later"),
                )),
            },
        }
    }

    fn visit_expression_literal(&mut self, token: &Token) -> TypeResolverResult<TypeID> {
        self.get_literal_type_id(*token)
    }

    fn visit_expression_identifier(&mut self, token: &Token) -> TypeResolverResult<TypeID> {
        self.get_identifier_type_id(*token)
    }
}

impl<'ast, 'interner> IStatementVisitor<TypeResolverResult<()>> for TypeResolver<'ast, 'interner> {
    fn visit_statement_expression(
        &mut self,
        expression: &crate::ast::ExpressionID,
    ) -> TypeResolverResult<()> {
        self.ast
            .get_type(
                self.ast
                    .get_expression(*expression)
                    .unwrap()
                    .accept_visitor(self)?,
            )
            .unwrap()
            .accept_visitor(self)?;

        Ok(())
    }

    fn visit_statement_assignment(
        &mut self,
        assignee: &crate::ast::ExpressionID,
        assignment_operator: &Token,
        assignment_value: &crate::ast::ExpressionID,
    ) -> TypeResolverResult<()> {
        let assignee_type_id = self
            .ast
            .get_expression(*assignee)
            .unwrap()
            .accept_visitor(self)?;
        let assignment_value_type_id = self
            .ast
            .get_expression(*assignment_value)
            .unwrap()
            .accept_visitor(self)?;

        if assignee_type_id != assignment_value_type_id {
            return Err(TypeResolverError::new(
                *assignment_operator,
                format!(
                    "'{}' assignment operator expects both sides to have the same type",
                    self.ast
                        .get_interner()
                        .get_str_or_empty(assignment_operator.lexeme)
                ),
            ));
        }

        Ok(())
    }

    fn visit_statement_block(&mut self, declarations: &[DeclarationID]) -> TypeResolverResult<()> {
        self.push_scope();

        for declaration in declarations {
            self.visited_declarations.insert(*declaration);
            self.ast
                .get_declaration(*declaration)
                .unwrap()
                .accept_visitor(self)?;
        }

        self.pop_scope();

        Ok(())
    }

    fn visit_statement_return(
        &mut self,
        return_token: &Token,
        return_value: &Option<crate::ast::ExpressionID>,
    ) -> TypeResolverResult<()> {
        if let Some(expression_id) = return_value {
            let return_type_id = self
                .ast
                .get_expression(*expression_id)
                .unwrap()
                .accept_visitor(self)?;

            if Some(return_type_id) != self.current_function_expected_return_type_id {
                return Err(TypeResolverError::new(
                    *return_token,
                    format!("Wrong return type for function"),
                ));
            }

            self.current_function_found_return = true;
        }
        Ok(())
    }
}

impl<'ast, 'interner> ITypeVisitor<TypeResolverResult<()>> for TypeResolver<'ast, 'interner> {
    fn visit_type_primitive(
        &self,
        _primitive_type: &crate::ast::PrimitiveType,
    ) -> TypeResolverResult<()> {
        Ok(())
    }
}

pub type TypeResolverResult<T> = Result<T, TypeResolverError>;

#[derive(Debug, Clone)]
pub struct TypeResolverError {
    token: Token,
    message: String,
}

impl TypeResolverError {
    fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl fmt::Display for TypeResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[line {}, column {}]::[Type Resolver Error]: {}",
            self.token.line, self.token.column, &self.message
        )
    }
}

impl std::error::Error for TypeResolverError {}
