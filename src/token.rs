use std::{collections::HashMap, fmt, sync::LazyLock};

pub static KEYWORDS: LazyLock<HashMap<&'static str, TokenKind>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    m.insert("and", TokenKind::And);
    m.insert("as", TokenKind::As);
    m.insert("break", TokenKind::Break);
    m.insert("class", TokenKind::Class);
    m.insert("const", TokenKind::Const);
    m.insert("continue", TokenKind::Continue);
    m.insert("crate", TokenKind::Crate);
    m.insert("defer", TokenKind::Defer);
    m.insert("else", TokenKind::Else);
    m.insert("enum", TokenKind::Enum);
    m.insert("extern", TokenKind::Extern);
    m.insert("false", TokenKind::False);
    m.insert("for", TokenKind::For);
    m.insert("fun", TokenKind::Fun);
    m.insert("if", TokenKind::If);
    m.insert("impl", TokenKind::Impl);
    m.insert("in", TokenKind::In);
    m.insert("interface", TokenKind::Interface);
    m.insert("loop", TokenKind::Loop);
    m.insert("match", TokenKind::Match);
    m.insert("mod", TokenKind::Mod);
    m.insert("null", TokenKind::Null);
    m.insert("or", TokenKind::Or);
    m.insert("panic", TokenKind::Panic);
    m.insert("pub", TokenKind::Pub);
    m.insert("return", TokenKind::Return);
    m.insert("super", TokenKind::Super);
    m.insert("self", TokenKind::SelfValue);
    m.insert("Self", TokenKind::SelfType);
    m.insert("true", TokenKind::True);
    m.insert("typeof", TokenKind::Typeof);
    m.insert("use", TokenKind::Use);
    m.insert("var", TokenKind::Var);
    m.insert("void", TokenKind::Void);
    m.insert("while", TokenKind::While);

    m
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // --- Single-character symbols ---
    LeftParen,    // "("
    RightParen,   // ")"
    LeftBrace,    // "{"
    RightBrace,   // "}"
    LeftBracket,  // "["
    RightBracket, // "]"
    Comma,        // ","
    Dot,          // "."
    Semicolon,    // ";"
    Colon,        // ":"
    // At,           // "@"
    // Hash,         // "#"

    // --- Logical operators ---
    AmpersandAmpersand, // "&&"
    PipePipe,           // "||"
    And,                // "and"
    Or,                 // "or"

    // --- Arithmetic operators ---
    Plus,      // "+"
    Minus,     // "-"
    Star,      // "*"
    Slash,     // "/"
    Percent,   // "%"
    Caret,     // "^"
    Ampersand, // "&"
    Pipe,      // "|"
    Tilde,     // "~"

    // --- Comparison operators ---
    Equal,        // "="
    EqualEqual,   // "=="
    Bang,         // "!"
    BangEqual,    // "!="
    Less,         // "<"
    LessEqual,    // "<="
    Greater,      // ">"
    GreaterEqual, // ">="

    // --- Shift operators ---
    ShiftLeft,  // "<<"
    ShiftRight, // ">>"

    // --- Assignment operators ---
    PlusEqual,       // "+="
    MinusEqual,      // "-="
    StarEqual,       // "*="
    SlashEqual,      // "/="
    PercentEqual,    // "%="
    CaretEqual,      // "^="
    AmpersandEqual,  // "&="
    PipeEqual,       // "|="
    ShiftLeftEqual,  // "<<="
    ShiftRightEqual, // ">>="

    // --- Literals ---
    Identifier,     // e.g. foo, bar, baz
    IntegerLiteral, // e.g. 123, 0xff, 42u8
    FloatLiteral,   // e.g. 3.14, 2.0f64
    StringLiteral,  // e.g. "hello"
    CharLiteral,    // e.g. 'a'

    // --- Keywords ---
    Const,     // "const"
    If,        // "if"
    Else,      // "else"
    Enum,      // "enum"
    While,     // "while"
    For,       // "for"
    In,        // "in"
    Return,    // "return"
    Super,     // "super"
    Break,     // "break"
    Continue,  // "continue"
    Crate,     // "crate"
    Defer,     // "defer"
    Fun,       // "fun"
    Class,     // "class"
    Interface, // "interface"
    Loop,      // "loop"
    Impl,      // "impl"
    SelfValue, // "self"
    SelfType,  // "Self"
    True,      // "true"
    False,     // "false"
    Null,      // "null"
    As,        // "as"
    Use,       // "use"
    Var,       // "var"
    Void,      // "void"
    Mod,       // "mod"
    Pub,       // "pub"
    Extern,    // "extern"
    Match,     // "match"
    Panic,     // "panic"
    Typeof,    // "typeof"

    // --- End of input ---
    Eof, // End of file/input
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            // --- Single-character symbols ---
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::LeftBracket => "[",
            TokenKind::RightBracket => "]",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Semicolon => ";",
            TokenKind::Colon => ":",
            // TokenKind::At => "@",
            // TokenKind::Hash => "#",

            // --- Logical operators ---
            TokenKind::AmpersandAmpersand => "&&",
            TokenKind::PipePipe => "||",
            TokenKind::And => "and",
            TokenKind::Or => "or",

            // --- Arithmetic operators ---
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Caret => "^",
            TokenKind::Ampersand => "&",
            TokenKind::Pipe => "|",
            TokenKind::Tilde => "~",

            // --- Comparison operators ---
            TokenKind::Equal => "=",
            TokenKind::EqualEqual => "==",
            TokenKind::Bang => "!",
            TokenKind::BangEqual => "!=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",

            // --- Shift operators ---
            TokenKind::ShiftLeft => "<<",
            TokenKind::ShiftRight => ">>",

            // --- Assignment operators ---
            TokenKind::PlusEqual => "+=",
            TokenKind::MinusEqual => "-=",
            TokenKind::StarEqual => "*=",
            TokenKind::SlashEqual => "/=",
            TokenKind::PercentEqual => "%=",
            TokenKind::CaretEqual => "^=",
            TokenKind::AmpersandEqual => "&=",
            TokenKind::PipeEqual => "|=",
            TokenKind::ShiftLeftEqual => "<<=",
            TokenKind::ShiftRightEqual => ">>=",

            // --- Literals ---
            TokenKind::Identifier => "identifier",
            TokenKind::IntegerLiteral => "integer literal",
            TokenKind::FloatLiteral => "float literal",
            TokenKind::StringLiteral => "string literal",
            TokenKind::CharLiteral => "char literal",

            // --- Keywords ---
            TokenKind::Const => "const",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::Enum => "enum",
            TokenKind::While => "while",
            TokenKind::For => "for",
            TokenKind::In => "in",
            TokenKind::Return => "return",
            TokenKind::Super => "super",
            TokenKind::Break => "break",
            TokenKind::Continue => "continue",
            TokenKind::Crate => "crate",
            TokenKind::Defer => "defer",
            TokenKind::Fun => "fun",
            TokenKind::Class => "class",
            TokenKind::Interface => "interface",
            TokenKind::Loop => "loop",
            TokenKind::Impl => "impl",
            TokenKind::SelfValue => "self",
            TokenKind::SelfType => "Self",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Null => "null",
            TokenKind::As => "as",
            TokenKind::Use => "use",
            TokenKind::Var => "var",
            TokenKind::Void => "void",
            TokenKind::Mod => "mod",
            TokenKind::Pub => "pub",
            TokenKind::Extern => "extern",
            TokenKind::Match => "match",
            TokenKind::Panic => "panic",
            TokenKind::Typeof => "typeof",

            // --- End of input ---
            TokenKind::Eof => "EOF",
        };
        write!(f, "{}", text)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}, col {}]: kind: {}, lexeme: '{}'",
            self.line, self.column, self.kind, self.lexeme
        )
    }
}
