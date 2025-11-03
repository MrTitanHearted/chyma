use std::fmt;

use crate::ast::AST;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeID(pub usize);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    PrimitiveType(PrimitiveType),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PrimitiveType {
    Void,
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    F32,
    F64,
    Char,
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveType::Void => write!(f, "void"),
            PrimitiveType::Bool => write!(f, "bool"),
            PrimitiveType::I8 => write!(f, "i8"),
            PrimitiveType::I16 => write!(f, "i16"),
            PrimitiveType::I32 => write!(f, "i32"),
            PrimitiveType::I64 => write!(f, "i64"),
            PrimitiveType::I128 => write!(f, "i128"),
            PrimitiveType::ISize => write!(f, "isize"),
            PrimitiveType::U8 => write!(f, "u8"),
            PrimitiveType::U16 => write!(f, "u16"),
            PrimitiveType::U32 => write!(f, "u32"),
            PrimitiveType::U64 => write!(f, "u64"),
            PrimitiveType::U128 => write!(f, "u128"),
            PrimitiveType::USize => write!(f, "usize"),
            PrimitiveType::F32 => write!(f, "f32"),
            PrimitiveType::F64 => write!(f, "f64"),
            PrimitiveType::Char => write!(f, "char"),
        }
    }
}

impl<'a> AST<'a> {
    pub fn intern_type_primitive(&mut self, primitive_type: PrimitiveType) -> TypeID {
        let _type = Type::PrimitiveType(primitive_type);

        if let Some(&id) = self.type_lookup.get(&_type) {
            return TypeID(id);
        }

        let id = self.types.len();

        self.types.push(_type.clone());

        self.type_lookup.insert(_type, id);

        TypeID(id)
    }
    
    pub fn get_type(&self, id: TypeID) -> Option<&Type> {
        self.types.get(id.0)
    }
}

impl fmt::Display for TypeID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
