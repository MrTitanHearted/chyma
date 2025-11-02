use crate::{ast::{PrimitiveType, TypeID}, parser::{Parser, ParserError, ParserResult}, token::TokenKind};

impl Parser {
    pub(super) fn parse_type(&mut self) -> ParserResult<TypeID> {
        let current = self.get_current().unwrap().clone();

        match current.kind {
            TokenKind::Void | TokenKind::Identifier if current.is_primitive_type() => {
                self.parse_type_primitive()
            }

            TokenKind::Equal => Err(ParserError::expected_type(current)),

            _ => Err(ParserError::unknown_type(current)),
        }
    }

    fn parse_type_primitive(&mut self) -> ParserResult<TypeID> {
        let lexeme = self.advance().unwrap().lexeme.as_str();

        let primitive_type = match lexeme {
            "void" => PrimitiveType::Void,
            "bool" => PrimitiveType::Bool,
            "i8" => PrimitiveType::I8,
            "i16" => PrimitiveType::I16,
            "i32" => PrimitiveType::I32,
            "i64" => PrimitiveType::I64,
            "i128" => PrimitiveType::I128,
            "isize" => PrimitiveType::ISize,
            "u8" => PrimitiveType::U8,
            "u16" => PrimitiveType::U16,
            "u32" => PrimitiveType::U32,
            "u64" => PrimitiveType::U64,
            "u128" => PrimitiveType::U128,
            "usize" => PrimitiveType::USize,
            "f32" => PrimitiveType::F32,
            "f64" => PrimitiveType::F64,
            "char" => PrimitiveType::Char,

            _ => unreachable!(),
        };

        Ok(self.ast.intern_type_primitive(primitive_type))
    }
}
