use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::{
    ast::{
        AST, DeclarationID, Expression, IDeclarationVisitor, IExpressionVisitor, IStatementVisitor,
        ITypeVisitor, TypeID,
    },
    token::{Token, TokenStringID},
};

#[derive(Debug)]
pub struct SemanticResolver<'ast, 'interner> {
    ast: &'ast AST<'interner>,
    scopes: Vec<HashMap<TokenStringID, VariableMetadata>>,
    current_function_type: FunctionType,

    visited_declarations: HashSet<DeclarationID>,
}

#[derive(Debug)]
struct VariableMetadata {
    is_mutable: bool,
    is_assigned: bool,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionType {
    #[default]
    None,
    Function,
}

impl<'ast, 'interner> SemanticResolver<'ast, 'interner> {
    pub fn resolve(ast: &'ast AST<'interner>) -> SemanticResolverResult {
        let mut resolver = Self::new(ast);

        for i in (0..resolver.ast.get_declarations_count()).rev() {
            let declaration_id = DeclarationID(i);
            if !resolver.visited_declarations.contains(&declaration_id)
                && let Some(declaration) = resolver.ast.get_declaration(declaration_id)
            {
                resolver.visited_declarations.insert(declaration_id);
                declaration.accept_visitor(&mut resolver)?;
            }
        }

        Ok(())
    }

    fn new(ast: &'ast AST<'interner>) -> Self {
        Self {
            ast,
            scopes: vec![HashMap::new()],
            current_function_type: FunctionType::None,

            visited_declarations: HashSet::new(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_variable(&mut self, name: &Token, is_mutable: bool) -> SemanticResolverResult {
        if let Some(scope) = self.scopes.last_mut() {
            if let Some(_) = scope.insert(
                name.lexeme,
                VariableMetadata {
                    is_mutable,
                    is_assigned: false,
                },
            ) {
                return Err(SemanticResolverError::new(
                    *name,
                    format!(
                        "Variable with a name '{}' already exsits in the current scope.",
                        self.ast.get_interner().get_str_or_empty(name.lexeme)
                    ),
                ));
            }
        } else {
            return Err(SemanticResolverError::new(
                *name,
                format!("No scopes! How?!"),
            ));
        }

        Ok(())
    }

    fn assign_variable(&mut self, name: &Token) -> SemanticResolverResult {
        if let Some(scope) = self.scopes.last_mut() {
            if let Some(metadata) = scope.get_mut(&name.lexeme) {
                if metadata.is_assigned && !metadata.is_mutable {
                    return Err(SemanticResolverError::new(
                        *name,
                        format!(
                            "Variable with a name '{}' in the current scope is not mutable.",
                            self.ast.get_interner().get_str_or_empty(name.lexeme)
                        ),
                    ));
                }

                metadata.is_assigned = true;
            } else {
                return Err(SemanticResolverError::new(
                    *name,
                    format!(
                        "Variable with a name '{}' is not declared in the current scope.",
                        self.ast.get_interner().get_str_or_empty(name.lexeme)
                    ),
                ));
            }
        } else {
            return Err(SemanticResolverError::new(
                *name,
                format!("No scopes! How?!"),
            ));
        }
        Ok(())
    }

    fn resolve_function(
        &mut self,
        _identifier: &Token,
        parameters: &[(Token, crate::ast::TypeID)],
        return_type: &Option<crate::ast::TypeID>,
        body: &crate::ast::StatementID,
        function_type: FunctionType,
    ) -> SemanticResolverResult {
        let enclosing_function_type = self.current_function_type;
        self.current_function_type = function_type;

        self.begin_scope();

        for (name, _) in parameters {
            self.declare_variable(name, false)?;
            self.assign_variable(name)?;
        }

        if let Some(statement) = self.ast.get_statement(*body) {
            statement.accept_visitor(self)?;
        }

        self.end_scope();

        if let Some(return_type) = return_type {
            if let Some(return_type) = self.ast.get_type(*return_type) {
                return_type.accept_visitor(self)?;
            }
        }

        self.current_function_type = enclosing_function_type;
        Ok(())
    }
}

impl<'ast, 'interner> IDeclarationVisitor<SemanticResolverResult>
    for SemanticResolver<'ast, 'interner>
{
    fn visit_declaration_statement(
        &mut self,
        statement: &crate::ast::StatementID,
    ) -> SemanticResolverResult {
        if let Some(statement) = self.ast.get_statement(*statement) {
            statement.accept_visitor(self)?;
        }
        Ok(())
    }

    fn visit_declaration_let(
        &mut self,
        is_mutable: &bool,
        identifier: &Token,
        type_id: &TypeID,
        initializer: &Option<crate::ast::ExpressionID>,
    ) -> SemanticResolverResult {
        self.declare_variable(identifier, *is_mutable)?;

        if let Some(ty) = self.ast.get_type(*type_id) {
            ty.accept_visitor(self)?;
        }

        if let Some(initializer) = initializer {
            if let Some(expression) = self.ast.get_expression(*initializer) {
                expression.accept_visitor(self)?;
            }

            self.assign_variable(identifier)?;
        }

        Ok(())
    }

    fn visit_declaration_function(
        &mut self,
        identifier: &Token,
        parameters: &[(Token, TypeID)],
        return_type: &Option<TypeID>,
        body: &crate::ast::StatementID,
    ) -> SemanticResolverResult {
        self.declare_variable(identifier, false)?;
        self.assign_variable(identifier)?;
        self.resolve_function(
            identifier,
            parameters,
            return_type,
            body,
            FunctionType::Function,
        )?;
        Ok(())
    }
}

impl<'ast, 'interner> IExpressionVisitor<SemanticResolverResult>
    for SemanticResolver<'ast, 'interner>
{
    fn visit_expression_binary(
        &mut self,
        left: &crate::ast::ExpressionID,
        _operator: &Token,
        right: &crate::ast::ExpressionID,
    ) -> SemanticResolverResult {
        if let Some(expression) = self.ast.get_expression(*left) {
            expression.accept_visitor(self)?;
        }
        if let Some(expression) = self.ast.get_expression(*right) {
            expression.accept_visitor(self)?;
        }
        Ok(())
    }

    fn visit_expression_unary(
        &mut self,
        _operator: &Token,
        right: &crate::ast::ExpressionID,
    ) -> SemanticResolverResult {
        if let Some(expression) = self.ast.get_expression(*right) {
            expression.accept_visitor(self)?;
        }
        Ok(())
    }

    fn visit_expression_literal(&mut self, _token: &Token) -> SemanticResolverResult {
        Ok(())
    }

    fn visit_expression_identifier(&mut self, token: &Token) -> SemanticResolverResult {
        let mut found = false;
        for scope in self.scopes.iter().rev() {
            if let Some(metadata) = scope.get(&token.lexeme) {
                found = true;

                if !metadata.is_assigned {
                    return Err(SemanticResolverError::new(
                        *token,
                        format!(
                            "Variable '{}' is delcared, but is not assigned.",
                            self.ast.get_interner().get_str_or_empty(token.lexeme)
                        ),
                    ));
                }

                break;
            }
        }

        if !found {
            return Err(SemanticResolverError::new(
                *token,
                format!(
                    "Variable '{}' is not declared in any accessible scope.",
                    self.ast.get_interner().get_str_or_empty(token.lexeme)
                ),
            ));
        }

        Ok(())
    }
}

impl<'ast, 'interner> IStatementVisitor<SemanticResolverResult>
    for SemanticResolver<'ast, 'interner>
{
    fn visit_statement_expression(
        &mut self,
        expression: &crate::ast::ExpressionID,
    ) -> SemanticResolverResult {
        if let Some(expression) = self.ast.get_expression(*expression) {
            expression.accept_visitor(self)?;
        }

        Ok(())
    }

    fn visit_statement_assignment(
        &mut self,
        assignee: &crate::ast::ExpressionID,
        assignment_operator: &Token,
        assignment_value: &crate::ast::ExpressionID,
    ) -> SemanticResolverResult {
        if let Some(expression) = self.ast.get_expression(*assignee) {
            match expression {
                Expression::Identifier { token } => {
                    let mut found = false;
                    for scope in self.scopes.iter_mut().rev() {
                        if let Some(metadata) = scope.get_mut(&token.lexeme) {
                            if metadata.is_assigned && !metadata.is_mutable {
                                return Err(SemanticResolverError::new(
                                    *token,
                                    format!(
                                        "Can only assign to unassigned or mutable variables. '{}'",
                                        self.ast.get_interner().get_str_or_empty(token.lexeme)
                                    ),
                                ));
                            }

                            found = true;
                            metadata.is_assigned = true;
                            break;
                        }
                    }

                    if !found {
                        return Err(SemanticResolverError::new(
                            *token,
                            format!(
                                "Variable '{}' is not declared in any accessible scope.",
                                self.ast.get_interner().get_str_or_empty(token.lexeme)
                            ),
                        ));
                    }
                }
                _ => {
                    return Err(SemanticResolverError::new(
                        *assignment_operator,
                        format!("Only variables are assignable."),
                    ));
                }
            }
            expression.accept_visitor(self)?;
        }

        if let Some(expression) = self.ast.get_expression(*assignment_value) {
            expression.accept_visitor(self)?;
        }

        Ok(())
    }

    fn visit_statement_block(&mut self, declarations: &[DeclarationID]) -> SemanticResolverResult {
        self.begin_scope();

        for declaration in declarations {
            self.visited_declarations.insert(*declaration);
            if let Some(declaration) = self.ast.get_declaration(*declaration) {
                declaration.accept_visitor(self)?;
            }
        }

        self.end_scope();
        Ok(())
    }

    fn visit_statement_return(
        &mut self,
        return_token: &Token,
        return_value: &Option<crate::ast::ExpressionID>,
    ) -> SemanticResolverResult {
        if self.current_function_type == FunctionType::None {
            return Err(SemanticResolverError::new(
                *return_token,
                format!("Cannot use 'return' statement outside of a function."),
            ));
        }

        if let Some(result_value) = return_value {
            if let Some(expression) = self.ast.get_expression(*result_value) {
                expression.accept_visitor(self)?;
            }
        }

        Ok(())
    }
}

impl<'ast, 'interner> ITypeVisitor<SemanticResolverResult> for SemanticResolver<'ast, 'interner> {
    fn visit_type_primitive(
        &self,
        _primitive_type: &crate::ast::PrimitiveType,
    ) -> SemanticResolverResult {
        Ok(())
    }
}

pub type SemanticResolverResult = Result<(), SemanticResolverError>;

#[derive(Debug, Clone)]
pub struct SemanticResolverError {
    token: Token,
    message: String,
}

impl SemanticResolverError {
    fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl fmt::Display for SemanticResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[line {}, column {}]::[Semantic Resolver Error]: {}",
            self.token.line, self.token.column, &self.message
        )
    }
}

impl std::error::Error for SemanticResolverError {}
