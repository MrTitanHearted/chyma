#![allow(dead_code)]

use crate::token::Token;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenStringID(pub u32);

#[derive(Debug)]
pub struct TokenStringInterner {
    string_to_u32: HashMap<&'static str, u32>,
    strings: Vec<String>,
}

impl TokenStringInterner {
    pub fn new() -> Self {
        Self {
            string_to_u32: HashMap::new(),
            strings: Vec::new(),
        }
    }

    pub fn intern_str(&mut self, string: &str) -> TokenStringID {
        if let Some(&id) = self.string_to_u32.get(string) {
            return TokenStringID(id);
        }

        let id = self.strings.len() as u32;
        self.strings.push(string.to_string());
        let static_str: &'static str =
            unsafe { &*(self.strings[id as usize].as_str() as *const str) };
        self.string_to_u32.insert(static_str, id);

        TokenStringID(id)
    }

    pub fn contains_and_is_equal(&self, string: &str, id: TokenStringID) -> bool {
        if let Some(&str_id) = self.string_to_u32.get(string) {
            return str_id == id.0;
        }

        false
    }

    pub fn get_str_id(&self, string: &str) -> Option<TokenStringID> {
        if let Some(&id) = self.string_to_u32.get(string) {
            return Some(TokenStringID(id));
        }

        None
    }

    pub fn contains_str(&self, str: &str) -> bool {
        self.string_to_u32.contains_key(str)
    }

    pub fn contains_str_id(&self, str_id: TokenStringID) -> bool {
        str_id.0 < self.strings.len() as u32
    }

    pub fn get_str(&self, id: TokenStringID) -> Option<&str> {
        self.strings.get(id.0 as usize).map(|s| s.as_str())
    }

    pub fn get_str_or_empty(&self, id: TokenStringID) -> &str {
        if let Some(str) = self.get_str(id) {
            return str;
        }

        ""
    }
}

impl Clone for TokenStringInterner {
    fn clone(&self) -> Self {
        let strings = self.strings.clone();
        let mut string_to_u32 = HashMap::new();

        for (id, string) in strings.iter().enumerate() {
            let static_str: &'static str = unsafe { &*(string.as_str() as *const str) };
            string_to_u32.insert(static_str, id as u32);
        }

        Self {
            string_to_u32,
            strings,
        }
    }
}

#[derive(Debug)]
pub struct TokenFormatter<'a, 'b> {
    interner: &'b TokenStringInterner,
    token: &'a Token,
}

impl<'a, 'b> TokenFormatter<'a, 'b> {
    pub fn new(interner: &'b TokenStringInterner, token: &'a Token) -> Self {
        Self { interner, token }
    }
}

impl<'a, 'b> Display for TokenFormatter<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}, col {}]: kind: {}, lexeme: '{}'",
            self.token.line,
            self.token.column,
            self.token.kind,
            self.interner.get_str_or_empty(self.token.lexeme),
        )
    }
}

impl Display for TokenStringID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
