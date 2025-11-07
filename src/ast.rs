use crate::token::TokenStringInterner;
use std::collections::HashMap;

mod declarations;
mod expressions;
mod formatters;
mod statements;
mod types;

pub use declarations::*;
pub use expressions::*;
pub use formatters::*;
pub use statements::*;
pub use types::*;

#[derive(Debug, Clone)]
pub struct AST<'a> {
    interner: &'a TokenStringInterner,

    declarations: Vec<Declaration>,
    expressions: Vec<Expression>,
    statements: Vec<Statement>,
    types: Vec<Type>,

    type_lookup: HashMap<Type, usize>,

    global_declarations: Vec<DeclarationID>,
}

impl<'a> AST<'a> {
    pub fn new(interner: &'a TokenStringInterner) -> Self {
        let mut ast = Self {
            interner,

            declarations: Vec::new(),
            expressions: Vec::new(),
            statements: Vec::new(),
            types: Vec::new(),

            type_lookup: HashMap::new(),

            global_declarations: Vec::new(),
        };

        ast.intern_type_primitive(PrimitiveType::Void);
        ast.intern_type_primitive(PrimitiveType::Bool);
        ast.intern_type_primitive(PrimitiveType::I8);
        ast.intern_type_primitive(PrimitiveType::I16);
        ast.intern_type_primitive(PrimitiveType::I32);
        ast.intern_type_primitive(PrimitiveType::I64);
        ast.intern_type_primitive(PrimitiveType::I128);
        ast.intern_type_primitive(PrimitiveType::ISize);
        ast.intern_type_primitive(PrimitiveType::U8);
        ast.intern_type_primitive(PrimitiveType::U16);
        ast.intern_type_primitive(PrimitiveType::U32);
        ast.intern_type_primitive(PrimitiveType::U64);
        ast.intern_type_primitive(PrimitiveType::U128);
        ast.intern_type_primitive(PrimitiveType::USize);
        ast.intern_type_primitive(PrimitiveType::F32);
        ast.intern_type_primitive(PrimitiveType::F64);
        ast.intern_type_primitive(PrimitiveType::Char);

        ast
    }

    pub fn get_interner(&self) -> &'a TokenStringInterner {
        self.interner
    }

    pub fn get_global_declarations(&self) -> &[DeclarationID] {
        &self.global_declarations
    }

    pub fn set_global_declarations(&mut self) {
        self.global_declarations.clear();

        let mut local_declarations = std::collections::HashSet::new();

        for stmt in &self.statements {
            if let Statement::Block { declarations } = stmt {
                for decl in declarations {
                    local_declarations.insert(*decl);
                }
            }
        }

        for (i, _) in self.declarations.iter().enumerate() {
            let id = DeclarationID(i as u32);
            if !local_declarations.contains(&id) {
                self.global_declarations.push(id);
            }
        }
    }

    pub fn get_total_size(&self) -> usize {
        let mut size = size_of::<AST>();

        size += self.declarations.len() * size_of::<Declaration>();
        size += self.statements.len() * size_of::<Statement>();
        size += self.expressions.len() * size_of::<Expression>();
        size += self.types.len() * size_of::<Type>();

        size += self.type_lookup.len() * (size_of::<Type>() + size_of::<usize>());

        for declaration in &self.declarations {
            match declaration {
                Declaration::Function { parameters, .. } => {
                    if parameters.len() > 0 {
                        size += parameters.len() * size_of_val(parameters.get(0).unwrap());
                    }
                }
                _ => {}
            }
        }

        for statement in &self.statements {
            match statement {
                Statement::Block { declarations } => {
                    size += declarations.len() * size_of::<DeclarationID>();
                }
                _ => {}
            }
        }

        size
    }
}
