use std::fmt;

use crate::ast::{
    AST, Declaration, DeclarationID, Expression, ExpressionID, Statement, StatementID, Type, TypeID,
};

#[derive(Debug)]
pub struct FlatASTFormatter<'ast> {
    ast: &'ast AST,
}

#[derive(Debug)]
pub struct TreeASTFormatter<'ast> {
    ast: &'ast AST,
}

impl<'ast> FlatASTFormatter<'ast> {
    pub fn new(ast: &'ast AST) -> Self {
        Self { ast }
    }
}

impl<'ast> TreeASTFormatter<'ast> {
    pub fn new(ast: &'ast AST) -> Self {
        Self { ast }
    }

    fn format_expression(
        &self,
        f: &mut fmt::Formatter<'_>,
        expr_id: ExpressionID,
        indent: usize,
    ) -> fmt::Result {
        let indent_str = "  ".repeat(indent);

        if let Some(expr) = self.ast.get_expression(expr_id) {
            match expr {
                Expression::Binary {
                    left,
                    operator,
                    right,
                } => {
                    writeln!(f, "{}Binary '{}'", indent_str, operator.lexeme)?;
                    write!(f, "{}├─ left: ", indent_str)?;
                    self.format_expression(f, *left, indent + 1)?;
                    write!(f, "{}└─ right: ", indent_str)?;
                    self.format_expression(f, *right, indent + 1)?;
                }
                Expression::Unary { operator, right } => {
                    writeln!(f, "{}Unary '{}'", indent_str, operator.lexeme)?;
                    write!(f, "{}└─ operand: ", indent_str)?;
                    self.format_expression(f, *right, indent + 1)?;
                }
                Expression::Literal { token } => {
                    writeln!(f, "{}Literal '{}'", indent_str, token.lexeme)?;
                }
                Expression::Identifier { token } => {
                    writeln!(f, "{}Identifier '{}'", indent_str, token.lexeme)?;
                }
            }
        }

        Ok(())
    }

    fn format_type(
        &self,
        f: &mut fmt::Formatter<'_>,
        type_id: TypeID,
        indent: usize,
    ) -> fmt::Result {
        let indent_str = "  ".repeat(indent);

        if let Some(typ) = self.ast.get_type(type_id) {
            match typ {
                Type::PrimitiveType(prim) => {
                    writeln!(f, "{}{}", indent_str, prim)?;
                }
            }
        }

        Ok(())
    }

    fn format_statement(
        &self,
        f: &mut fmt::Formatter<'_>,
        stmt_id: StatementID,
        indent: usize,
    ) -> fmt::Result {
        let indent_str = "  ".repeat(indent);

        if let Some(stmt) = self.ast.get_statement(stmt_id) {
            match stmt {
                Statement::Expression { expression } => {
                    writeln!(f, "Statement[{}] Expression", stmt_id)?;
                    write!(f, "{}└─ ", indent_str)?;
                    self.format_expression(f, *expression, indent + 1)?;
                }
                Statement::Assignment {
                    assignee,
                    assignment_operator,
                    assignment_value,
                } => {
                    writeln!(
                        f,
                        "Statement[{}] Assignment '{}'",
                        stmt_id, assignment_operator.lexeme
                    )?;
                    write!(f, "{}├─ target: ", indent_str)?;
                    self.format_expression(f, *assignee, indent + 1)?;
                    write!(f, "{}└─ value: ", indent_str)?;
                    self.format_expression(f, *assignment_value, indent + 1)?;
                }
                Statement::Block { declarations } => {
                    writeln!(
                        f,
                        "Statement[{}] Block ({} declarations)",
                        stmt_id,
                        declarations.len()
                    )?;
                    for (i, decl_id) in declarations.iter().enumerate() {
                        let is_last = i == declarations.len() - 1;
                        let prefix = if is_last { "└─" } else { "├─" };
                        write!(f, "{}{} ", indent_str, prefix)?;
                        self.format_declaration(f, *decl_id, indent + 1)?;
                    }
                }
                Statement::Return { return_value, .. } => {
                    writeln!(f, "Statement[{}] Return", stmt_id)?;
                    if let Some(expr_id) = return_value {
                        write!(f, "{}└─Expression[{}]", indent_str, expr_id)?;
                        self.format_expression(f, *expr_id, indent + 1)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn format_declaration(
        &self,
        f: &mut fmt::Formatter<'_>,
        decl_id: DeclarationID,
        indent: usize,
    ) -> fmt::Result {
        let indent_str = "  ".repeat(indent);

        if let Some(decl) = self.ast.get_declaration(decl_id) {
            match decl {
                Declaration::Statement { statement } => {
                    writeln!(f, "Declaration[{}] Statement", decl_id)?;
                    write!(f, "{}└─ ", indent_str)?;
                    self.format_statement(f, *statement, indent + 1)?;
                }
                Declaration::Let {
                    let_token: _,
                    mut_token,
                    identifier,
                    type_token,
                    initializer,
                } => {
                    write!(f, "Declaration[{}] Let", decl_id)?;
                    if mut_token.is_some() {
                        write!(f, " (mutable)")?;
                    }
                    writeln!(f, " '{}'", identifier.lexeme)?;

                    let has_init = initializer.is_some();

                    if let Some(type_id) = type_token {
                        let prefix = if has_init { "├─" } else { "└─" };
                        write!(f, "{}{} type: ", indent_str, prefix)?;
                        self.format_type(f, *type_id, indent + 1)?;
                    }

                    if let Some(expr_id) = initializer {
                        write!(f, "{}└─ init: ", indent_str)?;
                        self.format_expression(f, *expr_id, indent + 1)?;
                    }
                }
                Declaration::Function {
                    fun_token: _,
                    identifier,
                    left_paren: _,
                    parameters,
                    right_paren: _,
                    return_type,
                    body,
                } => {
                    writeln!(
                        f,
                        "Declaration[{}] Function '{}'",
                        decl_id, identifier.lexeme
                    )?;

                    let has_body = true;
                    let has_return = return_type.is_some();

                    // Parameters
                    if !parameters.is_empty() {
                        let prefix = if has_return || has_body {
                            "├─"
                        } else {
                            "└─"
                        };
                        writeln!(
                            f,
                            "{}{} parameters: ({} params)",
                            indent_str,
                            prefix,
                            parameters.len()
                        )?;
                        for (i, (param_name, param_type)) in parameters.iter().enumerate() {
                            let is_last = i == parameters.len() - 1;
                            let param_prefix = if is_last { "└─" } else { "├─" };
                            write!(
                                f,
                                "{}   {}{}: ",
                                indent_str, param_prefix, param_name.lexeme
                            )?;
                            self.format_type(f, *param_type, indent + 2)?;
                        }
                    }

                    // Return type
                    if let Some(ret_type) = return_type {
                        let prefix = if has_body { "├─" } else { "└─" };
                        write!(f, "{}{} return: ", indent_str, prefix)?;
                        self.format_type(f, *ret_type, indent + 1)?;
                    }

                    // Body
                    write!(f, "{}└─ body: ", indent_str)?;
                    self.format_statement(f, *body, indent + 1)?;
                }
            }
        }

        Ok(())
    }
}

impl<'ast> fmt::Display for FlatASTFormatter<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== AST Flat View ===")?;
        writeln!(
            f,
            "AST Size in total is {} bytes",
            self.ast.get_total_size()
        )?;

        // Display all types
        if !self.ast.types.is_empty() {
            writeln!(f, "TYPES:")?;
            for (id, typ) in self.ast.types.iter().enumerate() {
                match typ {
                    Type::PrimitiveType(prim) => {
                        writeln!(f, "  [{}] {}", id, prim)?;
                    }
                }
            }
            writeln!(f)?;
        }

        // Display all expressions
        if !self.ast.expressions.is_empty() {
            writeln!(f, "EXPRESSIONS:")?;
            for (id, expr) in self.ast.expressions.iter().enumerate() {
                write!(f, "  [{}] ", id)?;
                match expr {
                    Expression::Binary {
                        left,
                        operator,
                        right,
                    } => {
                        writeln!(
                            f,
                            "Binary    (left: [{}], op: '{}', right: [{}])",
                            left, operator.lexeme, right
                        )?;
                    }
                    Expression::Unary { operator, right } => {
                        writeln!(
                            f,
                            "Unary     (op: '{}', operand: [{}])",
                            operator.lexeme, right
                        )?;
                    }
                    Expression::Literal { token } => {
                        writeln!(f, "Literal   '{}'", token.lexeme)?;
                    }
                    Expression::Identifier { token } => {
                        writeln!(f, "Identifier '{}'", token.lexeme)?;
                    }
                }
            }
            writeln!(f)?;
        }

        // Display all statements
        if !self.ast.statements.is_empty() {
            writeln!(f, "STATEMENTS:")?;
            for (id, stmt) in self.ast.statements.iter().enumerate() {
                write!(f, "  [{}] ", id)?;
                match stmt {
                    Statement::Expression { expression } => {
                        writeln!(f, "Expression  (expression[{}])", expression)?;
                    }
                    Statement::Assignment {
                        assignee,
                        assignment_operator,
                        assignment_value,
                    } => {
                        writeln!(
                            f,
                            "Assignment  (target: expression[{}] {} value: expression[{}])",
                            assignee, assignment_operator.lexeme, assignment_value
                        )?;
                    }
                    Statement::Block { declarations } => {
                        write!(f, "Block       (declarations: [")?;
                        for (i, decl_id) in declarations.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{}", decl_id)?;
                        }
                        writeln!(f, "])")?;
                    }
                    Statement::Return { return_value, .. } => {
                        writeln!(
                            f,
                            "Return{}",
                            if let Some(expr_id) = return_value {
                                format!("       expression[{}]", expr_id)
                            } else {
                                String::new()
                            }
                        )?;
                    }
                }
            }
            writeln!(f)?;
        }

        // Display all declarations
        if !self.ast.declarations.is_empty() {
            writeln!(f, "DECLARATIONS:")?;
            for (id, decl) in self.ast.declarations.iter().enumerate() {
                write!(f, "  [{}] ", id)?;
                match decl {
                    Declaration::Statement { statement } => {
                        writeln!(f, "Statement   (statement[{}])", statement)?;
                    }
                    Declaration::Let {
                        let_token: _,
                        mut_token,
                        identifier,
                        type_token,
                        initializer,
                    } => {
                        write!(
                            f,
                            "Let         {}{}",
                            if mut_token.is_some() { "mut " } else { "" },
                            identifier.lexeme
                        )?;
                        if let Some(type_id) = type_token {
                            write!(f, ": type[{}]", type_id)?;
                        }
                        if let Some(expr_id) = initializer {
                            write!(f, " = expression[{}]", expr_id)?;
                        }
                        writeln!(f)?;
                    }
                    Declaration::Function {
                        fun_token: _,
                        identifier,
                        left_paren: _,
                        parameters,
                        right_paren: _,
                        return_type,
                        body,
                    } => {
                        write!(f, "Function    '{}'(", identifier.lexeme)?;
                        for (i, (param_name, param_type)) in parameters.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{}: type[{}]", param_name.lexeme, param_type)?;
                        }
                        write!(f, ")")?;
                        if let Some(ret_type) = return_type {
                            write!(f, " -> type[{}]", ret_type)?;
                        }
                        writeln!(f, " body: statement[{}]", body)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'ast> fmt::Display for TreeASTFormatter<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== AST Tree View ===")?;
        writeln!(
            f,
            "AST Size in total is {} bytes",
            self.ast.get_total_size(),
        )?;

        // Find all declarations that are referenced inside blocks
        let mut nested_declarations = std::collections::HashSet::new();
        for stmt in &self.ast.statements {
            if let Statement::Block { declarations } = stmt {
                for decl_id in declarations {
                    nested_declarations.insert(decl_id.0);
                }
            }
        }

        // Only display root-level declarations (those not nested in blocks)
        for (id, _) in self.ast.declarations.iter().enumerate() {
            if !nested_declarations.contains(&id) {
                self.format_declaration(f, DeclarationID(id), 0)?;
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
