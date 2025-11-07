use llvm_sys::LLVMType;
use llvm_sys::core::{
    LLVMAddFunction, LLVMDoubleTypeInContext, LLVMFloatTypeInContext, LLVMFunctionType,
    LLVMInt1TypeInContext, LLVMInt8TypeInContext, LLVMInt16TypeInContext, LLVMInt32TypeInContext,
    LLVMInt64TypeInContext, LLVMInt128TypeInContext, LLVMIntTypeInContext, LLVMVoidTypeInContext,
};
use llvm_sys::prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef};
use llvm_sys::target::{LLVMIntPtrTypeInContext, LLVMSizeOfTypeInBits};
use llvm_sys::target_machine::LLVMGetDefaultTargetTriple;

use crate::ast::{
    AST, DeclarationID, ExpressionID, IDeclarationVisitor, IExpressionVisitor, IStatementVisitor,
    ITypeVisitor, PrimitiveType, StatementID, TypeID,
};
use crate::token::Token;
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt;

#[derive(Debug)]
pub struct Compiler<'ast, 'interner, 'map> {
    ast: &'ast AST<'interner>,
    map: &'map HashMap<ExpressionID, TypeID>,

    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
}

impl<'ast, 'interner, 'map> Compiler<'ast, 'interner, 'map> {
    pub unsafe fn compile(
        ast: &'ast AST<'interner>,
        map: &'map HashMap<ExpressionID, TypeID>,
        context: LLVMContextRef,
        module: LLVMModuleRef,
        builder: LLVMBuilderRef,
    ) -> CompilerResult<()> {
        let mut compiler = Compiler::new(ast, map, context, module, builder);

        for declaration_id in compiler.ast.get_global_declarations() {
            compiler
                .ast
                .get_declaration(*declaration_id)
                .unwrap()
                .accept_visitor(&mut compiler)?;
        }

        Ok(())
    }

    fn new(
        ast: &'ast AST<'interner>,
        map: &'map HashMap<ExpressionID, TypeID>,
        context: LLVMContextRef,
        module: LLVMModuleRef,
        builder: LLVMBuilderRef,
    ) -> Self {
        Self {
            ast,
            map,

            context,
            module,
            builder,
        }
    }
}

impl<'ast, 'interner, 'map> IDeclarationVisitor<CompilerResult<()>>
    for Compiler<'ast, 'interner, 'map>
{
    fn visit_declaration_statement(&mut self, statement: &StatementID) -> CompilerResult<()> {
        self.ast
            .get_statement(*statement)
            .unwrap()
            .accept_visitor(self)?;

        Ok(())
    }

    fn visit_declaration_let(
        &mut self,
        _is_mutable: &bool,
        identifier: &Token,
        type_id: &TypeID,
        initializer: &Option<ExpressionID>,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn visit_declaration_function(
        &mut self,
        identifier: &Token,
        parameters: &[(Token, TypeID)],
        return_type: &Option<TypeID>,
        body: &StatementID,
    ) -> CompilerResult<()> {
        let return_type = self
            .ast
            .get_type(
                return_type.unwrap_or(self.ast.get_primitive_type_id(PrimitiveType::Void).unwrap()),
            )
            .unwrap()
            .accept_visitor(self)?;

        let mut parameter_types = Vec::new();

        for (_, type_id) in parameters {
            parameter_types.push(self.ast.get_type(*type_id).unwrap().accept_visitor(self)?);
        }

        let function_type = unsafe {
            LLVMFunctionType(
                return_type,
                parameter_types.as_mut_ptr(),
                parameter_types.len() as u32,
                0,
            )
        };

        let function_name =
            CString::new(self.ast.get_interner().get_str_or_empty(identifier.lexeme)).unwrap();
        let function =
            unsafe { LLVMAddFunction(self.module, function_name.as_ptr(), function_type) };
        
        

        todo!()
    }
}

impl<'ast, 'interner, 'map> IExpressionVisitor<CompilerResult<()>>
    for Compiler<'ast, 'interner, 'map>
{
    fn visit_expression_binary(
        &mut self,
        left: &ExpressionID,
        operator: &Token,
        right: &ExpressionID,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn visit_expression_unary(
        &mut self,
        operator: &Token,
        right: &ExpressionID,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn visit_expression_literal(&mut self, token: &Token) -> CompilerResult<()> {
        todo!()
    }

    fn visit_expression_identifier(&mut self, token: &Token) -> CompilerResult<()> {
        todo!()
    }
}

impl<'ast, 'interner, 'map> IStatementVisitor<CompilerResult<()>>
    for Compiler<'ast, 'interner, 'map>
{
    fn visit_statement_expression(&mut self, expression: &ExpressionID) -> CompilerResult<()> {
        todo!()
    }

    fn visit_statement_assignment(
        &mut self,
        assignee: &ExpressionID,
        assignment_operator: &Token,
        assignment_value: &ExpressionID,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn visit_statement_block(&mut self, declarations: &[DeclarationID]) -> CompilerResult<()> {
        todo!()
    }

    fn visit_statement_return(
        &mut self,
        return_token: &Token,
        return_value: &Option<ExpressionID>,
    ) -> CompilerResult<()> {
        todo!()
    }
}

impl<'ast, 'interner, 'map> ITypeVisitor<CompilerResult<LLVMTypeRef>>
    for Compiler<'ast, 'interner, 'map>
{
    fn visit_type_primitive(&self, primitive_type: &PrimitiveType) -> CompilerResult<LLVMTypeRef> {
        unsafe {
            match primitive_type {
                PrimitiveType::Void => Ok(LLVMVoidTypeInContext(self.context)),
                PrimitiveType::Bool => Ok(LLVMInt1TypeInContext(self.context)),

                // Signed integers
                PrimitiveType::I8 => Ok(LLVMInt8TypeInContext(self.context)),
                PrimitiveType::I16 => Ok(LLVMInt16TypeInContext(self.context)),
                PrimitiveType::I32 => Ok(LLVMInt32TypeInContext(self.context)),
                PrimitiveType::I64 => Ok(LLVMInt64TypeInContext(self.context)),
                PrimitiveType::I128 => Ok(LLVMInt128TypeInContext(self.context)),
                PrimitiveType::ISize => Ok(LLVMInt64TypeInContext(self.context)),
                // Unsigned integers
                PrimitiveType::U8 => Ok(LLVMInt8TypeInContext(self.context)),
                PrimitiveType::U16 => Ok(LLVMInt16TypeInContext(self.context)),
                PrimitiveType::U32 => Ok(LLVMInt32TypeInContext(self.context)),
                PrimitiveType::U64 => Ok(LLVMInt64TypeInContext(self.context)),
                PrimitiveType::U128 => Ok(LLVMInt128TypeInContext(self.context)),
                PrimitiveType::USize => Ok(LLVMInt64TypeInContext(self.context)),

                // Floating point
                PrimitiveType::F32 => Ok(LLVMFloatTypeInContext(self.context)),
                PrimitiveType::F64 => Ok(LLVMDoubleTypeInContext(self.context)),

                // Char (usually a 32-bit Unicode scalar)
                PrimitiveType::Char => Ok(LLVMInt32TypeInContext(self.context)),
            }
        }
    }
}

pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, Clone)]
pub struct CompilerError {
    token: Token,
    message: String,
}

impl CompilerError {
    fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[line {}, column {}]::[Compiler Error]: {}",
            self.token.line, self.token.column, &self.message
        )
    }
}

impl std::error::Error for CompilerError {}
