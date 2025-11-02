# Chyma Language Grammer

## Program
```bnf
program ::= declaration* EOF
```

## Declarations
```bnf
declaration          ::= function_declaration | let_declaration | statement
function_declaration ::= "fun" function
let_declaration      ::= "let" "mut"? identifier (":" type)? ("=" expression)? ";"
```

## Statments
```bnf
statement             ::= expression_statement | block_statement | assignment_statement | return_statement
expression_statement  ::= expression ";"
block_statement       ::= "{" declaration* "}"
assignment_statement  ::= identifier "=" expression ";"
return_statement      ::= "return" expression? ";"
```

## Types
```bnf
type            ::= primitive_type
primitive_type  ::= void | bool | char
                  | i8  | i16  | i32  | i64  | i128  | isize
                  | u8  | u16  | u32  | u64  | u128  | usize
```

## Expressions
```bnf
expression       ::= logical_or

logical_or       ::= logical_and (("or" | "||") logical_and)*
logical_and      ::= bitwise_or (("and" | "&&") bitwise_or)*
bitwise_or       ::= bitwise_xor ("|" bitwise_xor)*
bitwise_xor      ::= bitwise_and ("^" bitwise_and)*
bitwise_and      ::= equality ("&" equality)*
equality         ::= relational (("==" | "!=") relational)*
relational       ::= bitwise_shift (("<=" | "<" | ">" | ">=") bitwise_shift)*
bitwise_shift    ::= additive (("<<" | ">>") additive)*
additive         ::= multiplicative (("+" | "-") multiplicative)*
multiplicative   ::= unary (("/" | "*" | "%") unary)*
unary            ::= (("!" | "-" | "~") unary) | primary

primary          ::= "true" | "false" | "null" | "self"
                   | number | string | identifier
```

## Utility Rules
```bnf
function   ::= identifier "(" paramters? ")" (":" type)? block_statement
parameters ::= parameter ("," parameter)* ","?
parameter  ::= identifier ":" type
```

## Lexical Grammar
```bnf
number      ::= digit+ ("." digit+)?
string      ::= "\"" <any char except "\"">* "\""
identifier  ::= alpha (alpha | digit)*
alpha       ::= "a"..."z" | "A"..."Z" | "_"
digit       ::= "0"..."9"
```