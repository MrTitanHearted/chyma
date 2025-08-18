# Chyma Language Specification

**Date:** August 2025

## 1. Introduction

Chyma is a statically typed, memory-safe programming language designed as a comprehensive learning project for implementing various programming language features. The language serves as an educational exploration into language design, type systems, memory management, compilation techniques, and runtime implementation. Through building Chyma, the goal is to gain hands-on experience with the theoretical and practical aspects of programming language development.

### 1.1 Design Principles

**Type Safety**: Complete static type verification with comprehensive type inference eliminates entire classes of runtime errors before program execution.

**Memory Safety**: Automatic memory management removes manual memory management concerns while preventing use-after-free, double-free, and memory leak errors.

**Value Semantics**: By-value semantics for all data types provide predictable ownership and copying behavior, with explicit reference types for shared mutable state.

**Ergonomic Design**: Expression-oriented syntax with minimal syntactic overhead reduces boilerplate while maintaining code clarity.

**Interoperability**: Native C Foreign Function Interface (FFI) enables seamless integration with existing system libraries and performance-critical code.

**Learning Focus**: Language features are chosen and implemented primarily for their educational value in understanding programming language construction rather than production efficiency or developer experience optimization.

### 1.2 Language Characteristics

**Compilation Strategy**: Ahead-of-time compilation produces native machine code with full program optimization.

**Memory Model**: Automatic memory management with type-aware allocation strategies and automatic lifetime management.

**Type System**: Static typing with parametric polymorphism, interface-based abstraction, and complete type inference.

**Parsing Model**: Context-aware parsing enables disambiguation of syntactic constructs based on semantic context.

**Source Encoding**: UTF-8 encoded source files with `.chy` extension.

**Module System**: Rust-inspired module system where each file represents a module, with explicit module declarations and hierarchical organization.

## 2. Lexical Structure

### 2.1 Source Encoding

Source files MUST be encoded in UTF-8. All Unicode scalar values are valid within string literals and comments. Byte order marks (BOM) are ignored if present.

### 2.2 Comments

```chyma
// Line comment: extends to end of line
/* Block comment: may span multiple lines */
/* Nested /* block comments */ are supported */
```

### 2.3 Identifiers

Identifiers conform to Unicode identifier syntax (UAX #31):

**Initial Character**: Letter, underscore (`_`)  
**Subsequent Characters**: Letter, decimal digit, underscore  
**Case Sensitivity**: Identifiers are case-sensitive  
**Reserved Words**: Keywords cannot serve as identifiers

Valid examples:

```chyma
identifier
_private
userName
résumé
变量名
```

### 2.4 Reserved Keywords

```
and       as        break      class      const      continue
crate     defer     else       enum       extern     false
for       fun       if         impl       in         interface
loop      match     mod        null       or         panic
pub       return    super      self       Self       true
typeof    use       var        void       while
```

### 2.5 Operators and Delimiters

**Arithmetic**: `+`, `-`, `*`, `/`, `%`  
**Assignment**: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`  
**Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`  
**Logical**: `and`, `or`, `!`  
**Bitwise**: `&`, `|`, `^`, `~`, `<<`, `>>`  
**Navigation**: `::`, `:`, `.`  
**Ranges**: `..`, `..=`  
**Delimiters**: `{`, `}`, `[`, `]`, `(`, `)`, `;`, `,`  
**Other**: `?`, `!`

### 2.6 Statement Termination

Semicolons are optional when statement boundaries are unambiguous through newline analysis. Required only when multiple statements appear on a single line or in ambiguous parsing contexts.

```chyma
// Semicolon optional
var x = 42
var y = 24

// Semicolon required
var a = 10; var b = 20

// Expression statements
calculate_result()
process_data(input)
```

## 3. Type System

### 3.1 Primitive Types

#### 3.1.1 Integer Types

| Type    | Width    | Range                               |
| ------- | -------- | ----------------------------------- |
| `i8`    | 8-bit    | -128 to 127                         |
| `i16`   | 16-bit   | -32,768 to 32,767                   |
| `i32`   | 32-bit   | -2,147,483,648 to 2,147,483,647     |
| `i64`   | 64-bit   | -2⁶³ to 2⁶³-1                       |
| `i128`  | 128-bit  | -2¹²⁷ to 2¹²⁷-1                     |
| `u8`    | 8-bit    | 0 to 255                            |
| `u16`   | 16-bit   | 0 to 65,535                         |
| `u32`   | 32-bit   | 0 to 4,294,967,295                  |
| `u64`   | 64-bit   | 0 to 2⁶⁴-1                          |
| `u128`  | 128-bit  | 0 to 2¹²⁸-1                         |
| `isize` | platform | Platform-dependent signed integer   |
| `usize` | platform | Platform-dependent unsigned integer |

#### 3.1.2 Floating-Point Types

**`f32`**: IEEE 754 single-precision (32-bit) floating-point  
**`f64`**: IEEE 754 double-precision (64-bit) floating-point

#### 3.1.3 Other Primitive Types

**`bool`**: Boolean type with values `true` and `false`  
**`char`**: Unicode scalar value (21-bit value)  
**`void`**: Unit type representing absence of meaningful value

### 3.2 Value and Reference Semantics

#### 3.2.1 Value Types

All types in Chyma are value types by default. Assignment, parameter passing, and return operations perform deep copying:

```chyma
var original = Person { name: "Alice", age: 30 }
var copy = original        // Deep copy performed
copy.age = 31             // original.age remains 30

var message = "Hello, World!"
var duplicate = message   // String copied, not shared
```

#### 3.2.2 Reference Types

Reference types enable shared mutable state and are explicitly denoted with the `*` prefix:

```chyma
*T          // Mutable reference to type T
*const T    // Immutable reference to type T
*void       // Untyped reference (equivalent to C void*)
```

**Reference Properties**:

-   Only reference types may hold the value `null`
-   References are managed automatically
-   Dereferencing operator: `*reference`
-   Reference creation operator: `&value`
-   Field access and method calls use `.` operator directly

```chyma
var value = 42
var reference: *i32 = &value     // Create reference to value
var shared: *i32 = reference     // Reference copied, not value
*shared = 100                    // Modifies original value

var person = Person { name: "Alice", age: 30 }
var person_ref: *Person = &person
person_ref.age = 31              // Direct field access, no dereference needed
```

#### 3.2.3 Array Types

Arrays are dynamic collections with optional size and capacity specifications:

```chyma
[T]              // Dynamic array, size and capacity determined at runtime
[T; size]        // Array with specified initial size
[T; size; cap]   // Array with specified initial size and capacity
*[T]             // Reference to array
*const [T]       // Immutable array reference
```

Arrays support dynamic operations and direct indexing:

```chyma
var numbers = [1, 2, 3, 4, 5]           // Type: [i32]
var buffer: [i32; 10] = [0; 10]         // 10 elements, all initialized to 0
var heap_array: [string; 5; 20]         // 5 elements, capacity for 20

// Array operations
numbers.push(6)                         // Add element
var last = numbers.pop()                // Remove and return last element
var size = numbers.length               // Get array size (.size also works)
var cap = numbers.capacity              // Get array capacity

// Direct indexing for reference arrays
var shared_array: *[i32] = &[1, 2, 3]
shared_array[0] = 999                   // Modifies original array, no dereference needed
```

#### 3.2.4 String Type

**`string`**: Immutable UTF-8 encoded string

Strings are value types with copy semantics:

```chyma
var greeting = "Hello"
var message = greeting + " World"  // Creates new string
var copy = greeting               // String content copied

// For shared string references
var shared_text: *const string = &greeting
```

### 3.3 Tuple Types

```chyma
(Type1, Type2, Type3)    // Tuple with three elements
```

```chyma
var coordinates: (f64, f64) = (10.5, 20.3)
var (x, y) = coordinates              // Destructuring assignment
```

### 3.4 Optional and Result Types

#### 3.4.1 Optional Types

Optional types represent values that may or may not be present:

```chyma
T!          // Optional type containing value of type T or nothing
```

Optional types use `!` to represent absence of value:

```chyma
var maybe_number: i32! = 42!
var empty: string! = !

// Pattern matching
match maybe_number {
    42! => println("Got forty-two"),
    num! => println("Got value: {}", num),
    ! => println("Got no value")
}

// Conditional checks
if maybe_number == ! {
    println("No value present")
}

if var num = maybe_number {    // Destructure if present
    println("Value is: {}", num)
}

// For void optionals
fun might_fail(): ! {
    if condition {
        return !        // No value
    }
    return ()!          // Some void value
}
```

#### 3.4.2 Result Types

Result types handle operations that can succeed or fail:

```chyma
T!E         // Result type containing success value T or error E
```

```chyma
fun divide(a: f64, b: f64): f64!string {
    if b == 0.0 {
        return !"Division by zero"
    }
    return (a / b)!
}

var result = divide(10.0, 2.0)
match result {
    5.0! => println("Got expected result"),
    value! => println("Result: {}", value),
    !error => println("Error: {}", error)
}

// Conditional handling
if var value = result {
    println("Success: {}", value)
} else if var error = result {
    println("Failed: {}", error)
}

// Error propagation
fun process_calculation(input1: string, input2: string): f64!string {
    var num1 = parse_integer(input1)?  // Propagate error if parsing fails
    var num2 = parse_integer(input2)?  // Propagate error if parsing fails
    var result = divide_numbers(num1 as f64, num2 as f64)?
    return result!
}
```

### 3.5 Type Inference

The compiler performs comprehensive type inference while maintaining static type safety:

```chyma
var inferred = 42              // Type: i32
var decimal = 3.14             // Type: f64
var text = "Hello"             // Type: string
var collection = [1, 2, 3]     // Type: [i32]

fun generic_example(items) {   // Parameter type inferred from usage
    for item in items {
        println("{}", item)
    }
}
```

### 3.6 Type Conversions

#### 3.6.1 Explicit Casting

```chyma
var integer = 42
var floating = integer as f64           // Explicit cast
var character = 65 as char              // ASCII to character
```

#### 3.6.2 Type Checking

```chyma
if value typeof string {
    println("Value is a string: {}", value as string)
}
```

## 4. Variables and Constants

### 4.1 Variable Declarations

```chyma
var name: Type = expression     // Explicit type annotation
var name = expression           // Type inferred from expression
```

Variables are mutable by default and may be reassigned:

```chyma
var counter = 0
counter = counter + 1           // Reassignment permitted

var data = [1, 2, 3]
data = [4, 5, 6]               // Entire array replaced
```

### 4.2 Pattern Matching with var

```chyma
var (x, y) = get_coordinates()          // Destructure tuple
var Person { name, age } = user_data    // Destructure class with same field names
var [first, ..rest] = numbers           // Destructure array

// Class destructuring with field renaming
var Point { x: playerX, y: playerY } = player.get_position()
// playerX is u32 and playerY is u32 too

var Rectangle { width: w, height: h } = get_bounds()
// w and h are the new variable names

// Mixed destructuring (some renamed, some not)
var Person { name, age: years_old } = user_data
// name keeps original field name, years_old is renamed from age

// Partial destructuring with rest pattern
var ComplexData { important_field, ..rest } = complex_object
// Only extract important_field, ignore other fields

// Optional and Result destructuring in regular var declarations
var value! = maybe_get_value()    // Extract value if Some, panic if None
var result! = try_operation()     // Extract success value, panic if Error
var !error = failed_operation()   // Extract error value, panic if Success

// Dual Result destructuring - NOT allowed in regular var declarations
// var result!error = operation()  // Compile error: dual destructuring only in if/while

// Conditional destructuring with optionals and results
if var Person { name, age } = optional_person {
    println("Person: {} ({})", name, age)
}

if var Point { x: posX, y: posY } = maybe_position {
    println("Position: ({}, {})", posX, posY)
}

// Optional destructuring - only executes if Some
if var next_item! = iterator.next() {
    process_item(next_item)
}

// Result destructuring - only executes if success
if var data! = load_file("config.txt") {
    parse_config(data)
}

// Error destructuring - only executes if error
if var !error = validate_input(user_input) {
    show_error_message(error)
}

// Dual Result destructuring - binds both success and error variables
if var result!error = risky_operation() {
    // Success branch: result is accessible, error is not
    println("Success: {}", result)
} else {
    // Error branch: error is accessible, result is not
    println("Error: {}", error)
}

while var Some { value } = iterator.next() {
    process(value)
}
```

### 4.3 Constant Declarations

```chyma
const NAME: Type = expression   // Compile-time constant
```

Constants must be initialized with compile-time evaluable expressions:

```chyma
const MAX_SIZE: usize = 1024
const PI: f64 = 3.141592653589793
const GREETING: string = "Welcome"
```

### 4.4 Immutable References

```chyma
const reference_name: *Type = expression
```

```chyma
var mutable_data = [1, 2, 3, 4, 5]
const readonly_view: *const [i32] = &mutable_data
// readonly_view[0] = 999  // Compile error

mutable_data[0] = 999     // Original data still mutable
```

## 5. Functions

### 5.1 Function Definition Syntax

```chyma
fun function_name(parameter1: Type1, parameter2: Type2): ReturnType {
    // Function body with explicit return statements
    return expression
}

fun expression_function(parameters): ReturnType = expression

fun procedure_function(parameters) {
    // Procedure returning void (implicit)
    // Can return void or ()
}
```

### 5.2 Function Examples

```chyma
fun calculate_area(width: f64, height: f64): f64 {
    return width * height
}

fun greet_user(name: string): string = "Hello, " + name + "!"

fun print_status(message: string) {
    println("[STATUS] {}", message)
    return void  // or just return ()
}

fun factorial(n: u32): u32 {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
```

### 5.3 Variadic Functions

Functions can accept variable numbers of arguments:

```chyma
// Untyped variadic parameters
fun print_values(prefix: string, ...values) {
    for value in values {
        if value typeof i32 {
            println("{}: {}", prefix, value as i32)
        } else if value typeof string {
            println("{}: {}", prefix, value as string)
        }
    }
}

// Typed variadic parameters
fun add_numbers(base: f32, ...numbers: f32): f32 {
    for number in numbers {
        base += number
    }
    return base
}

// Usage
print_values("Item", 42, "hello", 3.14)
var sum = add_numbers(10.0, 1.5, 2.5, 3.0)
```

### 5.4 Advanced Function Features

#### 5.4.1 First-Class Functions

Functions can be stored in variables, passed as parameters, and returned from other functions:

```chyma
var operation = fun(x: i32, y: i32): i32 = x + y
var result = operation(10, 20)

fun apply_twice(func: fun(i32): i32, value: i32): i32 {
    return func(func(value))
}

var double = fun(x: i32): i32 = x * 2
var quadruple = apply_twice(double, 5)  // Result: 20
```

#### 5.4.2 Closures

Functions can capture variables from their enclosing scope:

```chyma
fun create_counter(initial: i32): fun(): i32 {
    var count: *i32 = &initial
    return fun(): i32 {
        *count = *count + 1
        return *count
    }
}

var counter = create_counter(0)
println("{}", counter())  // Output: 1
println("{}", counter())  // Output: 2
```

#### 5.4.3 Pattern Matching Parameters

```chyma
class Point { x: f64, y: f64 }

fun distance_from_origin(Point { x, y }: Point): f64 {
    return (x * x + y * y).sqrt()
}

// Parameter destructuring with field renaming
fun calculate_distance(Point { x: x1, y: y1 }: Point, Point { x: x2, y: y2 }: Point): f64 {
    var dx = x2 - x1
    var dy = y2 - y1
    return (dx * dx + dy * dy).sqrt()
}

fun process_tuple((first, second): (string, i32)): string {
    return first + " = " + second.to_string()
}

// Complex class destructuring in parameters
class GameState {
    player: Point,
    score: i32,
    level: u8
}

fun update_display(GameState { player: Point { x: px, y: py }, score, level }: GameState) {
    println("Player at ({}, {}) - Score: {} - Level: {}", px, py, score, level)
}
```

## 6. Data Types

### 6.1 Class Definitions

Classes define custom data types with named fields. By default, all members are private:

```chyma
class Person {
    name: string,           // Private field
    age: u32,              // Private field
    pub email: string,     // Public field
}

class BankAccount {
    account_number: string,
    balance: f64,
    pub is_active: bool
}
```

#### 6.1.1 Class Instantiation

```chyma
var customer = Person {
    name: "Alice Johnson",
    age: 28,
    email: "alice@example.com"
}
```

#### 6.1.2 Field Access

```chyma
println("Customer: {}", customer.email)  // Public field access
// customer.name  // Compile error: private field
```

### 6.2 Method Definition

Methods are defined within `impl` blocks and operate on instances:

```chyma
class Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    // Instance method with mutable reference self
    fun scale(*self, factor: f64) {
        self.width = self.width * factor
        self.height = self.height * factor
    }

    // Instance method with immutable reference self
    fun area(*const self): f64 {
        return self.width * self.height
    }

    // Associated functions (no self parameter)
    fun square(size: f64): Rectangle {
        return Rectangle { width: size, height: size }
    }

    fun new(width: f64, height: f64): Rectangle {
        return Rectangle { width: width, height: height }
    }

    fun get_width_and_height(self): (f64, f64) {
        return (self.width, self.height)
    }
}

var rect = Rectangle { width: 10.0, height: 5.0 }
rect.scale(2.0)                                // Instance method call
var area = rect.area()                         // Instance method call
var square = Rectangle:square(4.0)             // Associated function call
var new_rect = Rectangle:new(6.0, 8.0)         // Associated function call
var width_height = rect.get_width_and_height() // Instance method call where self is copied
```

### 6.3 Enumerations

Enumerations define algebraic data types with multiple variants:

```chyma
enum Status {
    Pending,
    Processing { started_at: string },
    Completed { result: i32, duration: f64 },
    Failed { error_code: i32, message: string }
}

enum Animal {
    Dog { breed: string, age: u8 },
    Cat { color: string, indoor: bool },
    Bird { species: string, can_fly: bool }
}
```

#### 6.3.1 Enum Instantiation

```chyma
var current_status = Status:Pending
var task_status = Status:Processing { started_at: "2025-08-17T10:30:00Z" }
var final_status = Status:Completed { result: 200, duration: 1.5 }

var pet = Animal:Dog { breed: "Golden Retriever", age: 3 }
var wild_bird = Animal:Bird { species: "Robin", can_fly: true }
```

### 6.4 Interface Definitions

Interfaces specify contracts that types must implement:

```chyma
interface Drawable {
    fun draw(*const self)
    fun get_bounds(*const self): (f64, f64, f64, f64)
}

interface Resizable {
    fun resize(*self, width: f64, height: f64)
    fun get_size(*const self): (f64, f64)
}

class Circle {
    center_x: f64,
    center_y: f64,
    radius: f64
}

impl Drawable for Circle {
    fun draw(*const self) {
        println("Drawing circle at ({}, {}) with radius {}",
                self.center_x, self.center_y, self.radius)
    }

    fun get_bounds(*const self): (f64, f64, f64, f64) {
        return (
            self.center_x - self.radius,
            self.center_y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0
        )
    }
}
```

#### 6.4.1 Interface Usage

```chyma
// Reference to interface (dynamic dispatch)
var drawable_objects: [*Drawable] = [
    &Circle { center_x: 10.0, center_y: 20.0, radius: 5.0 },
    &Rectangle { width: 15.0, height: 10.0 }
]

// Value-based interface usage
var shapes: [Drawable] = [
    Circle { center_x: 10.0, center_y: 20.0, radius: 5.0 },
    Rectangle { width: 15.0, height: 10.0 }
]

for object in drawable_objects {
    object.draw()  // Dynamic dispatch
}
```

## 7. Generics

### 7.1 Generic Functions

```chyma
fun swap<T>(a: *T, b: *T) {
    var temp = *a
    *a = *b
    *b = temp
}

fun find_maximum<T: Comparable>(items: [T]): T! {
    if items.length == 0 {
        return !
    }

    var maximum = items[0]
    for item in items {
        if item > maximum {
            maximum = item
        }
    }
    return maximum!
}
```

### 7.2 Generic Types

```chyma
class Container<T> {
    value: T,
    timestamp: string
}

impl<T> Container<T> {
    fun new(value: T): Container<T> {
        return Container {
            value: value,
            timestamp: current_time()
        }
    }

    fun get_value(*const self): T {
        return self.value
    }
}
```

### 7.3 Interface Constraints

```chyma
interface Comparable<T> {
    fun compare(*const self, other: T): i32
}

fun sort_items<T: Comparable<T>>(items: *[T]) {
    // Sorting implementation using T's compare method
}

fun binary_search<T: Comparable<T>>(items: [T], target: T): usize! {
    var left = 0
    var right = items.length

    while left < right {
        var mid = (left + right) / 2
        var comparison = items[mid].compare(target)

        if comparison == 0 {
            return mid!
        } else if comparison < 0 {
            left = mid + 1
        } else {
            right = mid
        }
    }

    return !
}
```

## 8. Pattern Matching

### 8.1 Match Expressions

Match expressions provide comprehensive pattern matching with exhaustiveness checking:

```chyma
match expression {
    pattern1 => result1,
    pattern2 if guard_condition => result2,
    pattern3 => {
        // Multi-statement block
        var intermediate = compute_value()
        intermediate * 2
    },
    _ => default_result
}
```

### 8.2 Pattern Varieties

#### 8.2.1 Optional and Result Patterns

```chyma
var result: f32! = divide(4.0, 2.0)
match result {
    2.0! => println("Got two"),
    num! => println("Got: {}", num),
    ! => println("Division by zero")
}

var error_result: i32!string = parse_number(input)
match error_result {
    value! => println("Parsed: {}", value),
    !error => println("Parse error: {}", error)
}
```

#### 8.2.2 Enum Destructuring

```chyma
match task_result {
    Status:Pending => "Still waiting...",
    Status:Processing { started_at } => "Started at " + started_at,
    Status:Completed { result, duration } =>
        "Completed with result {} in {}s".format(result, duration),
    Status:Failed { error_code, message } =>
        "Error {}: {}".format(error_code, message)
}
```

#### 8.2.3 Array Patterns

```chyma
match numbers {
    [] => "Empty array",
    [single] => "One element: " + single.to_string(),
    [first, second] => "Two elements: {} and {}".format(first, second),
    [head, ..tail] => "Head: {}, Tail length: {}".format(head, tail.length),
    [..init, last] => "Last element: {}".format(last)
}
```

#### 8.2.4 Class and Tuple Patterns

```chyma
// Tuple destructuring
match coordinates {
    (0, 0) => "Origin",
    (x, 0) => "On X-axis at " + x.to_string(),
    (0, y) => "On Y-axis at " + y.to_string(),
    (x, y) => "Point at ({}, {})".format(x, y)
}

// Class destructuring with field renaming
match person {
    Person { name: "Admin", .. } => "Administrator account",
    Person { age, .. } if age < 18 => "Minor account",
    Person { name, age: years } => "{} is {} years old".format(name, years)
}

// Complex class destructuring
match game_object {
    GameObject {
        position: Point { x: px, y: py },
        health: hp,
        ..
    } if hp > 0 => {
        println("Alive object at ({}, {}) with {} health", px, py, hp)
    },
    GameObject { position: Point { x, y }, health: 0, .. } => {
        println("Dead object at ({}, {})", x, y)
    },
    _ => "Unknown game object state"
}

// Nested class destructuring in match arms
match player_data {
    PlayerData {
        character: Character { name: char_name, stats: Stats { strength: str, .. } },
        inventory: items,
        ..
    } => {
        println("{} has {} strength and {} items", char_name, str, items.length)
    }
}
```

### 8.3 Conditional Destructuring

#### 8.3.1 If-Var Expressions

```chyma
if var Person { name, age } = user_data {
    println("User {} is {} years old", name, age)
} else {
    println("Invalid user data format")
}

// Class destructuring with field renaming in conditionals
if var Point { x: posX, y: posY } = get_mouse_position() {
    println("Mouse at ({}, {})", posX, posY)
}

// Complex nested destructuring
if var GameState {
    player: Point { x: px, y: py },
    enemies: [first_enemy, ..rest_enemies]
} = current_game_state {
    println("Player at ({}, {}), {} enemies remaining", px, py, rest_enemies.length + 1)
}

// Optional destructuring - extracts value if Some
if var value! = optional_result {
    process_result(value)
}

// Result destructuring - extracts success value
if var number! = divide(5, 4) {
    println("Division result: {}", number)
} else {
    println("Division failed")
}

// Result destructuring - extracts error value
if var !error = divide(3, 0) {
    println("Division error: {}", error)
} else {
    println("Division succeeded")
}

// Dual Result destructuring - bind both success and error variables
if var number!error = divide(num1, num2) {
    // Success branch: number is defined, error is not accessible
    println("Division successful: {}", number)
    return number * 2.0
} else {
    // Error branch: error is defined, number is not accessible
    println("Division failed: {}", error)
    return 0.0
}

// Dual destructuring with complex types
if var user!failure_reason = authenticate(username, password) {
    // Success: user is available
    println("Welcome {}!", user.name)
    start_user_session(user)
} else {
    // Error: failure_reason is available
    println("Authentication failed: {}", failure_reason)
    log_failed_attempt(username, failure_reason)
}

// Dual destructuring with class destructuring
if var Person { name: user_name, age }!auth_error = get_authenticated_user() {
    // Success: user_name and age are available
    println("Authenticated user: {} ({})", user_name, age)
} else {
    // Error: auth_error is available
    println("Authentication error: {}", auth_error)
}

// Optional destructuring with class fields
if var player! = get_current_player() {
    if var Point { x: px, y: py }! = player.get_position() {
        println("Player at ({}, {})", px, py)
    }
}

// Partial destructuring with conditional
if var ComplexObject { important_field: value, .. } = maybe_object {
    process_important_value(value)
}
```

#### 8.3.2 While-Var Loops

```chyma
while var Some { value } = iterator.next() {
    process_item(value)
}

// Optional destructuring in while loops
while var next_token! = lexer.next() {
    process_token(next_token)
}

// Result destructuring - continue while successful
while var data! = read_next_chunk() {
    process_data(data)
}

// Result destructuring - continue while there are errors
while var !error = validate_next_item() {
    log_error(error)
    attempt_recovery()
}

// Dual Result destructuring in while loops
while var packet!network_error = receive_network_packet() {
    // Success branch: process packet, continue loop
    process_packet(packet)
    send_acknowledgment(packet.id)
    // Loop continues while packets are successfully received
} else {
    // Error branch: handle network error, break loop
    println("Network error: {}", network_error)
    attempt_reconnection()
    break  // Exit loop on error
}

// Dual destructuring with complex parsing
while var ParsedData { tokens, metadata }!parse_error = parse_next_chunk() {
    // Success: tokens and metadata available
    process_tokens(tokens)
    update_metadata_cache(metadata)
} else {
    // Error: parse_error available, decide whether to continue or break
    if parse_error.is_recoverable() {
        println("Recoverable parse error: {}", parse_error.message)
        skip_malformed_chunk()
        continue  // Try next chunk
    } else {
        println("Fatal parse error: {}", parse_error.message)
        break  // Stop processing
    }
}

// Complex optional destructuring
while var Player { position: Point { x: px, y: py }, health: hp }! = get_next_player() {
    if hp > 0 {
        update_player_position(px, py)
    }
}

// Status-based processing with enum destructuring
while var Status:Processing { .. } = check_task_status() {
    wait_for_completion()
}

// Combining optional and class destructuring
while var GameEvent { event_type, data }! = event_queue.poll() {
    handle_event(event_type, data)
}
```

## 9. Control Flow

### 9.1 Conditional Expressions

```chyma
if condition {
    // Consequence block
} else if alternative_condition {
    // Alternative consequence
} else {
    // Default block
}

// Expression form
var message = if user.is_premium {
    "Welcome, Premium Member!"
} else {
    "Welcome!"
}
```

### 9.2 Loop Constructs

#### 9.2.1 Infinite Loops

```chyma
loop {
    var input = read_user_input()
    if input == "quit" {
        break
    }
    process_input(input)
}

// With optional labels for convenience
loop: main {
    var input = read_user_input()
    if input == "quit" {
        break: main
    }
    process_input(input)
}
```

#### 9.2.2 Conditional Loops

```chyma
while condition {
    // Loop body executes while condition is true
}

var attempts = 0
while attempts < max_retries {
    if try_operation() {
        break
    }
    attempts = attempts + 1
}

// With labels
while attempts < max_retries: retry {
    if try_operation() {
        break: retry
    }
    attempts = attempts + 1
}
```

#### 9.2.3 Iterator Loops

```chyma
// Iterate over collections
for element in collection {
    process_element(element)
}

// Range iteration
for index in 0..array.length {
    process_item(array[index])
}

for value in 1..=10 {  // Inclusive range
    println("Value: {}", value)
}

// Destructuring iteration
var pairs = [(1, "one"), (2, "two"), (3, "three")]
for (number, word) in pairs {
    println("{}: {}", number, word)
}

// With labels for nested loops
for i in 0..rows: outer {
    for j in 0..cols: inner {
        if matrix[i][j] == target {
            break: outer  // Break outer loop
        }
        if should_skip(i, j) {
            continue: inner
        }
    }
}
```

### 9.4 Defer Statements

The `defer` statement schedules code to be executed when the current scope exits, regardless of how the scope is exited (normal return, early return, break, continue, or panic). Deferred statements are executed in LIFO (Last In, First Out) order.

#### 9.4.1 Basic Defer Usage

```chyma
fun process_file(filename: string): string!string {
    var file = open_file(filename)?
    defer file.close()  // Always executed when function exits

    var data = file.read_all()?
    defer println("File processing completed")

    return process_data(data)!
    // Execution order on return:
    // 1. println("File processing completed")
    // 2. file.close()
}

fun cleanup_example() {
    var resource1 = acquire_resource1()
    defer release_resource1(resource1)

    var resource2 = acquire_resource2()
    defer release_resource2(resource2)

    var resource3 = acquire_resource3()
    defer release_resource3(resource3)

    // Do work with resources...

    // On function exit, cleanup happens in reverse order:
    // 1. release_resource3(resource3)
    // 2. release_resource2(resource2)
    // 3. release_resource1(resource1)
}
```

#### 9.4.2 Defer with Control Flow

```chyma
fun search_with_cleanup(items: [string], target: string): usize! {
    var temp_buffer = allocate_temp_buffer()
    defer free_temp_buffer(temp_buffer)

    for i in 0..items.length {
        if items[i] == target {
            return i!  // defer executes before return
        }

        if should_log_progress(i) {
            defer log_search_progress(i)  // Executes at end of iteration
        }
    }

    return !  // defer executes before return
}

fun loop_with_defer() {
    loop {
        var data = get_next_data()
        defer cleanup_iteration_data(data)

        match data.status {
            DataStatus:Complete => {
                process_final_data(data)
                break  // defer executes before break
            },
            DataStatus:Continue => {
                process_partial_data(data)
                continue  // defer executes before continue
            },
            DataStatus:Error => {
                handle_error(data.error)
                return  // defer executes before return
            }
        }
        // defer executes at end of normal iteration too
    }
}
```

#### 9.4.3 Defer in Nested Scopes

```chyma
fun nested_defer_example() {
    defer println("Function exit")

    if condition {
        defer println("If block exit")

        for i in 0..10 {
            defer println("Loop iteration {} exit", i)

            if i == 5 {
                defer println("Early loop exit")
                break
            }
        }

        defer println("After loop")
    }

    defer println("Before function end")

    // Execution order depends on control flow, but within each scope,
    // defers execute in LIFO order
}
```

#### 9.4.4 Defer with Variable Capture

```chyma
fun defer_capture_example() {
    var counter = 0

    for i in 0..5 {
        counter = counter + 1

        // Capture current value of counter and i
        defer println("Iteration {}: counter was {}", i, counter)

        if i == 2 {
            var special_value = compute_special(i)
            defer println("Special value: {}", special_value)
        }
    }

    // Output will show deferred prints in reverse order for each iteration
}

fun resource_management_with_capture() {
    var connections: [*DatabaseConnection] = []
    defer {
        // Block form of defer for complex cleanup
        println("Cleaning up {} connections", connections.length)
        for connection in connections {
            connection.close()
        }
        connections.clear()
    }

    for i in 0..3 {
        var conn = create_connection("server" + i.to_string())
        connections.push(&conn)

        // Individual connection logging
        defer println("Created connection to server{}", i)
    }

    // Use connections...
}
```

#### 9.4.5 Defer Error Handling

```chyma
fun defer_with_error_handling(): bool!string {
    var transaction = begin_transaction()?
    defer {
        if transaction.is_active() {
            transaction.rollback()
            println("Transaction rolled back due to error or early exit")
        }
    }

    var success = perform_operations(transaction)?

    if success {
        transaction.commit()
        return true!
    } else {
        // Transaction will be rolled back by defer
        return !"Operation failed"
    }
}

fun file_processing_with_defer(filename: string): string!string {
    var file = open_file(filename)?
    defer {
        if file.is_open() {
            file.close()
            println("File {} closed", filename)
        }
    }

    var lock = acquire_file_lock(file)?
    defer {
        release_file_lock(lock)
        println("File lock released")
    }

    var content = file.read_all()?
    var processed = process_content(content)?

    return processed!

    // Execution order on any exit path:
    // 1. release_file_lock(lock) + println
    // 2. file.close() + println (if file is still open)
}
```

#### 9.4.6 Defer Execution Rules

1. **LIFO Order**: Deferred statements execute in Last In, First Out order within the same scope
2. **Scope-based**: Each scope (function, loop iteration, if block) has its own defer stack
3. **All Exit Paths**: Defers execute on normal exit, early return, break, continue, and panic
4. **Variable Capture**: Deferred statements capture variables by value at the time defer is executed
5. **Exception**: Defers execute even during panic unwinding (similar to finally blocks)
6. **Block Form**: `defer { ... }` allows multiple statements in a single defer

```chyma
fun defer_rules_example(): i32 {
    defer println("1: Function defer")

    var x = 10
    defer println("2: x = {}", x)  // Captures x = 10

    x = 20

    if true {
        defer println("3: Inner scope defer")
        x = 30
        defer println("4: x = {}", x)  // Captures x = 30

        if x > 25 {
            defer println("5: Nested defer")
            return x  // Early return
        }
    }

    return x

    // On early return, execution order:
    // 5: Nested defer
    // 4: x = 30
    // 3: Inner scope defer
    // 2: x = 10
    // 1: Function defer
}
```

## 10. Modules and Visibility

### 10.1 Module System Overview

Chyma uses a Rust-inspired module system where each file automatically represents a module. The module hierarchy is established through explicit module declarations using the `mod` keyword, creating a tree structure of modules that can be nested and organized hierarchically.

### 10.2 File-Based Modules

Each `.chy` source file is automatically a module with the same name as the file (without extension):

```
src/
├── main.chy          // Module: main (root module)
├── math.chy          // Module: math
├── graphics.chy      // Module: graphics
└── utils/
    ├── mod.chy       // Module: utils (directory module)
    ├── string.chy    // Module: utils::string
    └── io.chy        // Module: utils::io
```

### 10.3 Module Declarations

#### 10.3.1 Declaring Child Modules

Use the `mod` keyword to declare child modules within a file:

```chyma
// src/main.chy
mod math        // Declares math as a child module (from math.chy)
mod graphics    // Declares graphics as a child module (from graphics.chy)
mod utils       // Declares utils as a child module (from utils/mod.chy)

fun main() {
    var result = math::calculate(10, 20)
    graphics::draw_circle(50, 50, 25)
    utils::io::print_message("Hello World")
}
```

#### 10.3.2 Inline Module Declarations

Modules can also be declared inline within a file using curly braces:

```chyma
// src/main.chy
mod helpers {
    pub fun format_number(num: i32): string {
        return num.to_string()
    }

    fun internal_helper() {
        // Private to helpers module
    }

    mod nested {
        pub fun deep_function() {
            // Accessible as helpers::nested::deep_function
        }
    }
}

mod constants {
    pub const MAX_ITEMS: usize = 1000
    pub const DEFAULT_NAME: string = "Unnamed"
}

fun main() {
    var formatted = helpers::format_number(42)
    helpers::nested::deep_function()
    println("Max items: {}", constants::MAX_ITEMS)
}
```

### 10.4 Module Paths and Visibility

#### 10.4.1 Absolute and Relative Paths

Module paths can be absolute (starting from the crate root) or relative to the current module:

```chyma
// src/graphics/shapes.chy
mod circle {
    pub fun draw(x: f64, y: f64, radius: f64) {
        // Implementation
    }
}

mod rectangle {
    pub fun draw(x: f64, y: f64, width: f64, height: f64) {
        // Implementation
    }
}

pub fun render_scene() {
    // Relative paths within same module
    circle::draw(10.0, 10.0, 5.0)
    rectangle::draw(20.0, 20.0, 15.0, 10.0)

    // Absolute path from crate root
    crate::utils::io::log("Scene rendered")
}
```

#### 10.4.2 The `super` Keyword

Use `super` to access parent/enclosing modules:

```chyma
// src/math/geometry.chy
const PI: f64 = 3.141592653589793

mod circle {
    pub fun area(radius: f64): f64 {
        return super::PI * radius * radius  // Access parent module's PI
    }

    pub fun circumference(radius: f64): f64 {
        return 2.0 * super::PI * radius
    }

    mod advanced {
        pub fun sector_area(radius: f64, angle: f64): f64 {
            // super refers to circle module
            // super::super refers to geometry module
            return super::super::PI * radius * radius * (angle / 360.0)
        }
    }
}

mod rectangle {
    pub fun area(width: f64, height: f64): f64 {
        return width * height
    }

    pub fun diagonal(width: f64, height: f64): f64 {
        return (width * width + height * height).sqrt()
    }
}

pub fun total_area_circle_and_rect(radius: f64, width: f64, height: f64): f64 {
    return circle::area(radius) + rectangle::area(width, height)
}
```

#### 10.4.3 Crate Root Access

Use `crate` to access items from the root module:

```chyma
// src/utils/string.chy
pub fun capitalize(input: string): string {
    // Access a function from the root module
    crate::validate_input(input)
    // Implementation...
}

// src/main.chy (root module)
mod utils

fun validate_input(input: string): bool {
    return input.length > 0
}

fun main() {
    var result = utils::string::capitalize("hello")
}
```

### 10.5 Visibility Control

#### 10.5.1 Default Privacy

All items (functions, classes, fields, constants, modules) are private by default and only accessible within their declaring module:

```chyma
// src/math.chy
fun private_helper() {
    // Only accessible within math module
}

pub fun public_calculate(x: i32, y: i32): i32 {
    return private_helper_calculation(x, y)
}

class InternalData {
    value: i32,           // Private field
    pub name: string,     // Public field
}

pub class PublicData {
    internal_field: i32,  // Private field (default)
    pub public_field: i32 // Public field
}
```

#### 10.5.2 Public Visibility

Use `pub` to make items visible to other modules:

```chyma
// src/database/connection.chy
pub class DatabaseConnection {
    host: string,         // Private field
    port: u16,           // Private field
    pub is_connected: bool // Public field
}

impl DatabaseConnection {
    pub fun new(host: string, port: u16): DatabaseConnection {
        return DatabaseConnection {
            host: host,
            port: port,
            is_connected: false
        }
    }

    pub fun connect(*self): bool {
        // Public method
        return self.internal_connect()
    }

    fun internal_connect(*self): bool {
        // Private method (default)
        // Connection logic
        return true
    }
}

pub const DEFAULT_PORT: u16 = 5432
const INTERNAL_BUFFER_SIZE: usize = 1024  // Private constant
```

#### 10.5.3 Re-exporting

Modules can re-export items from child modules to create cleaner public APIs:

```chyma
// src/graphics/mod.chy
mod shapes
mod colors
mod rendering

// Re-export selected items from child modules
pub use shapes::Circle
pub use shapes::Rectangle
pub use colors::{Color, RGB}
pub use rendering::render_scene

// Clients can now use:
// graphics::Circle instead of graphics::shapes::Circle
// graphics::RGB instead of graphics::colors::RGB
```

### 10.6 Module Examples

#### 10.6.1 Directory Structure Example

```
src/
├── main.chy
├── config.chy
├── database/
│   ├── mod.chy
│   ├── connection.chy
│   ├── query.chy
│   └── schema/
│       ├── mod.chy
│       ├── user.chy
│       └── product.chy
└── web/
    ├── mod.chy
    ├── server.chy
    └── routes/
        ├── mod.chy
        ├── api.chy
        └── auth.chy
```

#### 10.6.2 Main Module Setup

```chyma
// src/main.chy
mod config
mod database
mod web

fun main() {
    var db_config = config::get_database_config()
    var connection = database::connection::DatabaseConnection::new(
        db_config.host,
        db_config.port
    )

    if connection.connect() {
        web::server::start_server(connection)
    }
}
```

#### 10.6.3 Database Module Organization

```chyma
// src/database/mod.chy
pub mod connection
pub mod query
pub mod schema

// Re-export commonly used items
pub use connection::DatabaseConnection
pub use query::{QueryBuilder, execute_query}
pub use schema::user::User
pub use schema::product::Product
```

```chyma
// src/database/connection.chy
use super::schema::{User, Product}  // Import from sibling module

pub class DatabaseConnection {
    // Implementation
}

impl DatabaseConnection {
    pub fun get_user(*self, id: i32): User! {
        // Implementation using super::query
        var result = super::query::execute_query(
            "SELECT * FROM users WHERE id = ?",
            [id]
        )
        // Process result...
    }
}
```

#### 10.6.4 Schema Module with Sub-modules

```chyma
// src/database/schema/mod.chy
pub mod user
pub mod product

pub use user::User
pub use product::Product
```

```chyma
// src/database/schema/user.chy
pub class User {
    pub id: i32,
    pub name: string,
    pub email: string,
    created_at: string,  // Private field
}

impl User {
    pub fun new(name: string, email: string): User {
        return User {
            id: 0,  // Will be set by database
            name: name,
            email: email,
            created_at: crate::utils::current_time()  // Access root module
        }
    }

    pub fun is_admin(*const self): bool {
        return super::super::query::check_admin_status(self.id)
    }
}
```

### 10.7 Module Resolution Rules

1. **File Modules**: `mod foo` looks for `foo.chy` in the current directory
2. **Directory Modules**: `mod foo` looks for `foo/mod.chy` if `foo.chy` doesn't exist
3. **Inline Modules**: `mod foo { ... }` declares an inline module
4. **Visibility**: Items are private by default, use `pub` for public visibility
5. **Path Resolution**:
    - `::` separates module path segments
    - `super` refers to parent module
    - `crate` refers to root module
    - Relative paths start from current module
    - Absolute paths start with `crate::`

## 11. Memory Management

### 11.1 Automatic Memory Management

Memory allocation and deallocation are handled automatically by the runtime system. Objects are allocated on the stack when possible, and moved to the heap when necessary based on lifetime analysis and escape detection.

### 11.2 Object Allocation Strategy

**Stack Allocation**: Small values and objects with known lifetimes are allocated on the stack for optimal performance.

**Heap Allocation**: Objects that escape their local scope, large objects, and dynamically-sized collections are allocated on the heap.

**Automatic Movement**: The compiler and runtime automatically handle moving objects between stack and heap as needed.

```chyma
// Typically stack-allocated
var small_number = 42
var local_tuple = (10, 20)

// May be heap-allocated depending on usage
var person = Person { name: "John", age: 30 }
var large_array = create_large_dataset()

// Reference creation may involve heap allocation
var shared_data: *Person = &person
```

## 12. Foreign Function Interface

### 12.1 External Function Declarations

```chyma
extern {
    fun malloc(size: usize): *void
    fun free(ptr: *void)
    fun strlen(str: *const char): usize
    fun memcpy(dest: *void, src: *const void, count: usize): *void

    // C variadic function support
    fun printf(format: *const char, ...): i32
}

// Single function declaration
extern fun getpid(): i32
extern fun sleep(seconds: u32): u32
```

### 12.2 Type Mapping

| Chyma Type    | C Type        | Notes            |
| ------------- | ------------- | ---------------- |
| `i8`          | `int8_t`      | Signed 8-bit     |
| `u8`          | `uint8_t`     | Unsigned 8-bit   |
| `i32`         | `int32_t`     | Standard integer |
| `f64`         | `double`      | Double precision |
| `bool`        | `_Bool`       | C99 boolean      |
| `*void`       | `void*`       | Untyped pointer  |
| `*const char` | `const char*` | C string         |

### 12.3 String Interoperability

```chyma
extern fun puts(str: *const char): i32

fun print_c_string(message: string) {
    // Automatic conversion from Chyma string to C string
    puts(message.as_c_str())
}

fun read_c_string(): string {
    var c_str = get_c_string()  // Returns *const char
    return string.from_c_str(c_str)  // Convert to Chyma string
}
```

## 13. Compilation Model

### 13.1 Build Configuration

Build configuration specified via build manifest file:

```chyma
// build.chy
pub fun build() {
    var project = Project::new("my_project")
    project.entry_file("src/main.chy")
    project.add_dependencies("deps/*")
    project.add_dependency("vendor/vulkan-chyma")
    project.link_dynamic_library("vulkan")
    project.build()
}
```

### 13.2 Output Generation

**Target**: Single executable with native machine code  
**Optimization**: Full program optimization with dead code elimination  
**Performance**: Decent performance via Cranelift, prioritizing implementation learning over production optimization  
**Debug Information**: Integrated debug symbol generation for development builds

### 13.3 Compilation Process

**Parsing**: Context-aware parsing resolves syntactic ambiguities based on semantic information

**Type Checking**: Complete static type verification with comprehensive inference

**Module Resolution**: Rust-style module system with file-based and inline module support

**Optimization**: Dead code elimination, inlining, and other optimizations applied during compilation to explore compiler optimization techniques

**Code Generation**: Native machine code generation via Cranelift backend to learn about code generation and LLVM alternatives

---

_This specification defines the current Chyma language design as a learning project for programming language implementation. The language serves as an educational exploration into compiler design, type systems, memory management, and runtime development. Features are chosen primarily for their instructional value in understanding language construction rather than production efficiency or developer experience optimization._
