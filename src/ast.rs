#![allow(dead_code, unused_variables, unused_mut)]

use std::fmt;

use crate::token::Token;

pub type ExprId = usize;

#[derive(Debug, Clone)]
pub struct DisplayOption<T>(pub Option<T>);

#[derive(Debug, Clone)]
pub struct DisplayVec<T>(pub Vec<T>);

impl<T: fmt::Display> fmt::Display for DisplayOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(v) => write!(f, "{}", v),
            None => write!(f, "None"),
        }
    }
}

impl<T: fmt::Display> fmt::Display for DisplayVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

#[macro_export]
macro_rules! define_ast {
    (
        $mod_name:ident,
        $visitor_name:ident,
        $name:ident {
            $(
                $variant:ident as $new_method:ident as $method:ident { $($field:ident : $type:ty),* $(,)? }
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $(
                $variant($mod_name::$variant),
            )*
        }

        pub mod $mod_name {
            use super::*;

            $(
                #[derive(Debug, Clone)]
                pub struct $variant {
                    $( pub $field: $type ),*
                }

                impl $variant {
                    pub fn new($( $field: $type ),*) -> Self {
                        Self { $( $field ),* }
                    }
                }
            )*
        }

        pub trait $visitor_name<T> {
            $(
                fn $method(&mut self, node: &$mod_name::$variant) -> T;
            )*
        }

        impl $name {
            $(
                pub fn $new_method($( $field: $type ),*) -> Box<Self> {
                    Box::new(Self::$variant($mod_name::$variant::new($( $field ),*)))
                }
            )*

            pub fn accept<T>(&self, visitor: &mut impl $visitor_name<T>) -> T {
                match self {
                    $(
                        $name::$variant(node) => visitor.$method(node),
                    )*
                }
            }
        }

        #[allow(unused_assignments)]
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $name::$variant(inner) => {
                            write!(f, "{} {{", stringify!($variant))?;
                            let mut first = true;
                            $(
                                if !first {
                                    write!(f, ", ")?;
                                }
                                write!(f, " {}: {{ {} }}", stringify!($field), inner.$field)?;
                                first = false;
                            )*
                            write!(f, " }}")
                        }
                    )*
                }
            }
        }
    };
}

define_ast!(
    expr,
    IExprVisitor,
    Expr {
        Assignment as assignment as visit_assignment { id: ExprId, left: Box<Expr>, operator: Token, right: Box<Expr> },
        Logical as logical as visit_logical { id: ExprId, left: Box<Expr>, operator: Token, right: Box<Expr> },
        Equality as equality as visit_equality { id: ExprId, left: Box<Expr>, operator: Token, right: Box<Expr> },
        Relational as relational as visit_relational { id: ExprId, left: Box<Expr>, operator: Token, right: Box<Expr> },
        Range as range as visit_range { id: ExprId, left: Box<Expr>, operator: Token, right: Box<Expr> },
        Bitwise as bitwise as visit_bitwise { id: ExprId, left: Box<Expr>, operator: Token, right: Box<Expr> },
        Shift as shift as visit_shift { id: ExprId, left: Box<Expr>, operator: Token, right: Box<Expr> },
        Arithmetic as arithmetic as visit_arithmetic { id: ExprId, left: Box<Expr>, operator: Token, right: Box<Expr> },
        Cast as cast as visit_cast { id: ExprId, as_token: Token, r#type: Box<Type> },
        Unary as unary as visit_unary { id: ExprId, operator: Token, right: Box<Expr> },
        Literal as literal as visit_literal { id: ExprId, token: Token },
        Variable as variable as visit_variable { id: ExprId, token: Token },
    }
);

define_ast!(
    r#type,
    ITypeVisitor,
    Type {
        Primitive as primitive as visit_primitive { primitive: Token },
        Reference as reference as visit_reference { reference: Token, const_keyword: DisplayOption<Token>, r#type: Box<Type> },
        Array as array as visit_array { r#type: Box<Type>, size: DisplayOption<Box<Expr>>, capacity: DisplayOption<Box<Expr>> },
        Tuple as tuple as visit_tuple { types: DisplayVec<Box<Type>> },
        Optional as optional as visit_optional { r#type: Box<Type>, bang: Token },
        Result as result as visit_result { ok_type: Box<Type>, bang: Token, error_type: Box<Type> },
        Function as function as visit_function {
            fun: Token,
            parameters: DisplayVec<Box<Type>>,
            variadic: DisplayOption<Token>,
            variadic_type: DisplayOption<Box<Type>>,
            return_type: DisplayOption<Box<Type>>,
        },
        Generic as generic as visit_generic { path: DisplayVec<Token>, generics: DisplayVec<Box<Type>> },
        Path as path as visit_path { segments: DisplayVec<Token> },
        SelfType as self_type as visit_self_type { token: Token },
    }
);
