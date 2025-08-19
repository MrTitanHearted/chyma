#![allow(dead_code)]

use crate::token::Token;

pub type ExprId = usize;

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
        Literal as literal as visit_literal { id: ExprId, token: Token },
        Binary as binary as visit_binary { id: ExprId, left: Box<Expr>, operator: Token, right: Box<Expr> }
    }
);
