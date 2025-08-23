# Chyma Language BNF Grammar

**Version:** 1.0  
**Date:** August 2025

This BNF grammar defines the complete syntax for the Chyma programming language. It is designed for context-aware parsing where semantic information may be used to disambiguate syntactic constructs.

## 1. Program Structure

```bnf
<program> ::= <item>*

<item> ::= <function_definition>
        | <class_definition>
        | <interface_definition>
        | <enum_definition>
        | <impl_block>
        | <module_declaration>
        | <use_declaration>
        | <extern_block>
        | <extern_function>
        | <constant_declaration>

<module_declaration> ::= <visibility>? "mod" <identifier> (";" | "{" <item>* "}")

<use_declaration> ::= <visibility>? "use" <use_path> ";"

<use_path> ::= <simple_path>
             | <simple_path> "::" "{" <use_list> "}"
             | <simple_path> "::" "*"

<use_list> ::= <use_item> ("," <use_item>)* ","?

<use_item> ::= <identifier>
             | <identifier> "as" <identifier>
```

## 2. Identifiers and Literals

```bnf
<identifier> ::= <identifier_start> <identifier_continue>*

<identifier_start> ::= [a-zA-Z_] | <unicode_letter>

<identifier_continue> ::= <identifier_start> | [0-9]

<keyword> ::= "and" | "as" | "break" | "class" | "const" | "continue"
            | "crate" | "defer" | "else" | "enum" | "extern" | "false" | "for"
            | "fun" | "if" | "impl" | "in" | "interface" | "loop"
            | "match" | "mod" | "null" | "or" | "panic" | "pub"
            | "return" | "super" | "self" | "Self" | "true" | "typeof"
            | "use" | "var" | "void" | "while"

<literal> ::= <integer_literal>
            | <float_literal>
            | <boolean_literal>
            | <character_literal>
            | <string_literal>
            | <null_literal>
            | <self_literal>

<integer_literal> ::= <decimal_literal>
                    | <hexadecimal_literal>
                    | <octal_literal>
                    | <binary_literal>

<decimal_literal> ::= [0-9] ([0-9] | "_")*

<hexadecimal_literal> ::= "0x" [0-9a-fA-F] ([0-9a-fA-F] | "_")*

<octal_literal> ::= "0o" [0-7] ([0-7] | "_")*

<binary_literal> ::= "0b" [01] ([01] | "_")*

<float_literal> ::= <decimal_literal> "." <decimal_literal>? <exponent>?
                  | <decimal_literal> <exponent>

<exponent> ::= ("e" | "E") ("+" | "-")? <decimal_literal>

<boolean_literal> ::= "true" | "false"

<character_literal> ::= "'" (<character> | <escape_sequence>) "'"

<string_literal> ::= "\"" (<string_character> | <escape_sequence>)* "\""

<escape_sequence> ::= "\\" ("n" | "r" | "t" | "\\" | "'" | "\"" | "0")
                    | "\\x" <hex_digit> <hex_digit>
                    | "\\u{" <hex_digit>+ "}"

<null_literal> ::= "null"

<self_literal> ::= "self"
```

## 3. Types

```bnf
<type> ::= <primitive_type>
         | <reference_type>
         | <array_type>
         | <tuple_type>
         | <optional_type>
         | <result_type>
         | <function_type>
         | <generic_type>
         | <path_type>
         | <self_type>

<primitive_type> ::= "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
                   | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
                   | "f32" | "f64"
                   | "bool" | "char" | "string" | "void"

<reference_type> ::= "*" "const"? <type>

<array_type> ::= "[" <type> ((";" <expression>)? (";" <expression>)?)? "]"

<tuple_type> ::= "(" (<type> ("," <type>)* ","?)? ")"

<optional_type> ::= <type> "!"

<result_type> ::= <type> "!" <type>

<function_type> ::= "fun" "(" (<type> ("," <type>)* ("," "..." <type>?)?)? ")" (":" <type>)?

<generic_type> ::= <path_type> "<" <type> ("," <type>)* ">"

<path_type> ::= <simple_path>

<self_type> ::= "Self"

<simple_path> ::= <path_segment> ("::" <path_segment>)*

<path_segment> ::= "crate" | "super" | <identifier>

<type_parameter> ::= <identifier> (":" <type_bounds>)?

<type_bounds> ::= <type_bound> ("+" <type_bound>)*

<type_bound> ::= <path_type>
```

## 4. Patterns

```bnf
<pattern> ::= <identifier_pattern>
            | <literal_pattern>
            | <tuple_pattern>
            | <array_pattern>
            | <class_pattern>
            | <enum_pattern>
            | <optional_pattern>
            | <result_pattern>
            | <wildcard_pattern>
            | <reference_pattern>
            | <guarded_pattern>

<identifier_pattern> ::= <identifier>

<literal_pattern> ::= <literal>

<tuple_pattern> ::= "(" (<pattern> ("," <pattern>)* ","?)? ")"

<array_pattern> ::= "[" (<array_pattern_element> ("," <array_pattern_element>)* ","?)? "]"

<array_pattern_element> ::= <pattern>
                          | ".." <pattern>?

<class_pattern> ::= <path_type> "{" (<field_pattern> ("," <field_pattern>)* ("," "..")? ","?)? "}"

<field_pattern> ::= <identifier>
                  | <identifier> ":" <pattern>

<enum_pattern> ::= <path_type> "::" <identifier> ("{" (<field_pattern> ("," <field_pattern>)* ","?)? "}")?

<optional_pattern> ::= <pattern> "!"
                     | "!"

<result_pattern> ::= <pattern> "!"
                   | "!" <pattern>
                   | <pattern> "!" <pattern>

<wildcard_pattern> ::= "_"

<reference_pattern> ::= "&" <pattern>

<guarded_pattern> ::= <pattern> "if" <expression>
```

## 5. Expressions

```bnf
<expression> ::= <assignment_expression>

<assignment_expression> ::= <or_expression>
                          | <or_expression> <assignment_operator> <assignment_expression>

<assignment_operator> ::= "=" | "+=" | "-=" | "*=" | "/=" | "%="
                        | "&=" | "|=" | "^=" | "<<=" | ">>="

<or_expression> ::= <and_expression> (("or" | "||") <and_expression>)*   

<and_expression> ::= <equality_expression> (("and" | "&&") <equality_expression>)*

<equality_expression> ::= <relational_expression> (("==" | "!=") <relational_expression>)*

<relational_expression> ::= <range_expression> (("<" | "<=" | ">" | ">=") <range_expression>)*

<range_expression> ::= <bitwise_or_expression> ((".." | "..=") <bitwise_or_expression>)?

<bitwise_or_expression> ::= <bitwise_xor_expression> ("|" <bitwise_xor_expression>)*

<bitwise_xor_expression> ::= <bitwise_and_expression> ("^" <bitwise_and_expression>)*

<bitwise_and_expression> ::= <shift_expression> ("&" <shift_expression>)*

<shift_expression> ::= <additive_expression> (("<<" | ">>") <additive_expression>)*

<additive_expression> ::= <multiplicative_expression> (("+" | "-") <multiplicative_expression>)*

<multiplicative_expression> ::= <cast_expression> (("*" | "/" | "%") <cast_expression>)*

<cast_expression> ::= <unary_expression> ("as" <type>)*

<unary_expression> ::= <postfix_expression>
                     | ("!" | "-" | "~" | "*" | "&") <unary_expression>

<postfix_expression> ::= <primary_expression> <postfix_suffix>*

<postfix_suffix> ::= <call_suffix>
                   | <index_suffix>
                   | <field_access_suffix>
                   | <optional_suffix>

<call_suffix> ::= "(" (<argument> ("," <argument>)* ","?)? ")"

<argument> ::= <expression>
             | <identifier> ":" <expression>

<index_suffix> ::= "[" <expression> "]"

<field_access_suffix> ::= "." <identifier>

<optional_suffix> ::= "?"

<primary_expression> ::= <literal>
                       | <identifier>
                       | <path_expression>
                       | <tuple_expression>
                       | <array_expression>
                       | <class_expression>
                       | <lambda_expression>
                       | <block_expression>
                       | <if_expression>
                       | <match_expression>
                       | <loop_expression>
                       | <while_expression>
                       | <for_expression>
                       | <typeof_expression>
                       | <parenthesized_expression>

<path_expression> ::= <simple_path>

<tuple_expression> ::= "(" (<expression> ("," <expression>)* ","?)? ")"

<array_expression> ::= "[" (<expression> ("," <expression>)* ","?)? "]"
                     | "[" <expression> ";" <expression> "]"

<class_expression> ::= <path_type> "{" (<field_assignment> ("," <field_assignment>)* ","?)? "}"

<field_assignment> ::= <identifier>
                     | <identifier> ":" <expression>

<lambda_expression> ::= "fun" "(" (<parameter> ("," <parameter>)* ("," <variadic_parameter>)? ","?)? ")" (":" <type>)? ("=" <expression> | <block_expression>)

<block_expression> ::= "{" <statement>* <expression>? "}"

<if_expression> ::= "if" <condition> <block_expression> ("else" ("if" <condition>)? <block_expression>)?

<condition> ::= <expression>
              | "var" <pattern> "=" <expression>

<match_expression> ::= "match" <expression> "{" (<match_arm> ("," <match_arm>)* ","?)? "}"

<match_arm> ::= <pattern> ("if" <expression>)? "=>" (<expression> | <block_expression>)

<loop_expression> ::= "loop" <label>? <block_expression>

<while_expression> ::= "while" <condition> <label>? <block_expression>

<for_expression> ::= "for" <pattern> "in" <expression> <label>?  <block_expression>

<typeof_expression> ::= <expression> "typeof" <type>

<parenthesized_expression> ::= "(" <expression> ")"

<label> ::= ":" <identifier>
```

## 6. Statements

```bnf
<statement> ::= <expression_statement>
              | <variable_declaration>
              | <constant_declaration>
              | <break_statement>
              | <continue_statement>
              | <return_statement>
              | <defer_statement>

<expression_statement> ::= <expression> ";"?

<variable_declaration> ::= "var" <pattern> (":" <type>)? ("=" <expression>)? ";"?

<constant_declaration> ::= <visibility>? "const" <identifier> ":" <type> "=" <expression> ";"?

<break_statement> ::= "break" <expression>? <label>? ";"?

<continue_statement> ::= "continue" <label>? ";"?

<return_statement> ::= "return" <expression>? ";"?

<defer_statement> ::= "defer" (<expression> | <block_expression>)
```

## 7. Function Definitions

```bnf
<function_definition> ::= <visibility>? "fun" <identifier> <generic_parameters>? "(" <parameter_list>? ")" (":" <type>)? (<function_body> | "=" <expression> ";"?)

<generic_parameters> ::= "<" <type_parameter> ("," <type_parameter>)* ">"

<parameter_list> ::= <parameter> ("," <parameter>)* ("," <variadic_parameter>)? ","?

<parameter> ::= <self_parameter> | <normal_parameter>

<self_parameter> ::= "*"? "const"? "self"

<normal_parameter> ::= <pattern> (":" <type>)?

<variadic_parameter> ::= "..." <identifier>? (":" <type>)?

<function_body> ::= <block_expression>

<visibility> ::= "pub"
```

## 8. Class Definitions

```bnf
<class_definition> ::= <visibility>? "class" <identifier> <generic_parameters>? "{" <class_member>* "}"

<class_member> ::= <field_definition>

<field_definition> ::= <visibility>? <identifier> ":" <type> ","?
```

## 9. Interface Definitions

```bnf
<interface_definition> ::= <visibility>? "interface" <identifier> <generic_parameters>? "{" <interface_member>* "}"

<interface_member> ::= <method_signature>

<method_signature> ::= <visibility>? "fun" <identifier> "(" <parameter_list>? ")" (":" <type>)? ";"?
```

## 10. Enum Definitions

```bnf
<enum_definition> ::= <visibility>? "enum" <identifier> <generic_parameters>? "{" <enum_variant>* "}"

<enum_variant> ::= <visibility>? <identifier> <enum_variant_data>? ","?

<enum_variant_data> ::= "{" <field_definition>* "}"
```

## 11. Implementation Blocks

```bnf
<impl_block> ::= "impl" <generic_parameters>? <type> ("for" <type>)? "{" <impl_member>* "}"

<impl_member> ::= <function_definition>
```

## 12. External Declarations

```bnf
<extern_block> ::= "extern" "{" <extern_item>* "}"

<extern_item> ::= <extern_function>

<extern_function> ::= "fun" <identifier> "(" <parameter_list>? ")" (":" <type>)? ";"?
```

## 13. Comments and Whitespace

```bnf
<comment> ::= <line_comment> | <block_comment>

<line_comment> ::= "//" [^\n\r]* ("\n" | "\r\n" | "\r")

<block_comment> ::= "/*" (<block_comment> | [^*/] | "*" [^/] | "/" [^*])* "*/"

<whitespace> ::= " " | "\t" | "\n" | "\r" | "\r\n"
```

## 14. Contextual Disambiguation Rules

### 14.1 Type vs Expression Context

The parser must distinguish between types and expressions in certain contexts:

-   In variable declarations: `var x: Type` vs `var x = expression`
-   In function parameters: `fun f(x: Type)` vs `fun f(x)`
-   In generic arguments: `Container<Type>` vs `Container<expression>`

### 14.2 Pattern vs Expression Context

Patterns and expressions share similar syntax but appear in different contexts:

-   After `var`: `var pattern = expression`
-   In match arms: `pattern => expression`
-   In function parameters: `fun f(pattern: type)`
-   In for loops: `for pattern in expression`

### 14.3 Statement vs Expression Context

Statements and expressions are distinguished by context and semicolons:

-   Block final position: expression (no semicolon required)
-   Non-final position: statement (semicolon may be required)
-   Single-line context: semicolon disambiguates multiple statements

### 14.4 Path Resolution Context

Path expressions are resolved based on context:

-   Type context: resolve to type names
-   Expression context: resolve to values/functions
-   Pattern context: resolve to enum variants or constructors

### 14.5 Generic Argument Context

Generic arguments in `<...>` must be distinguished from comparison operators:

-   After type names: likely generic arguments
-   After expressions: likely comparison
-   Balanced angle brackets: generic arguments
-   Unbalanced with binary operators: comparison

## 15. Operator Precedence

From highest to lowest precedence:

1. Primary expressions, postfix operators
2. Unary operators (`!`, `-`, `~`, `*`, `&`)
3. Type casting (`as`)
4. Multiplicative (`*`, `/`, `%`)
5. Additive (`+`, `-`)
6. Shift (`<<`, `>>`)
7. Bitwise AND (`&`)
8. Bitwise XOR (`^`)
9. Bitwise OR (`|`)
10. Range (`..`, `..=`)
11. Relational (`<`, `<=`, `>`, `>=`)
12. Equality (`==`, `!=`)
13. Logical AND (`and`)
14. Logical OR (`or`)
15. Assignment (`=`, `+=`, etc.)

## 16. Grammar Notes

### 16.1 Optional Semicolons

Semicolons are optional in most contexts and inferred based on:

-   Line breaks between statements
-   Block boundaries
-   Unambiguous statement termination

### 16.2 Trailing Commas

Trailing commas are allowed in:

-   Function parameter lists
-   Function argument lists
-   Tuple expressions and patterns
-   Array expressions and patterns
-   Class field definitions and expressions
-   Enum variant lists
-   Generic parameter lists
-   Use declaration lists

### 16.3 Context-Sensitive Parsing

This grammar requires a context-aware parser that can:

-   Distinguish types from expressions based on syntactic context
-   Resolve path expressions to appropriate namespace
-   Handle optional type annotations with inference
-   Manage operator precedence and associativity
-   Parse optional semicolons correctly

### 16.4 Error Recovery

The parser should provide good error recovery for:

-   Missing semicolons
-   Unmatched delimiters
-   Invalid type annotations
-   Malformed expressions
-   Incomplete function definitions

---

This BNF grammar provides a complete specification for parsing Chyma source code into an Abstract Syntax Tree (AST). The grammar is designed to handle the language's context-sensitive features while maintaining clarity and completeness for implementation.
