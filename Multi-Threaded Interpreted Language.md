# Multi-Threaded Interpreted Language

MIL?

### Design Patterns (Change Name)

- All variables are implicitly immutable for safety and cannot be passed by reference.
- Variables can be declared as mutable and be passed by reference.
- Modules are loaded in with multiple threads

### Operators

| Name | Operator | Example | Description | Implementation Notes |
|------|----------|---------|-------------|----------------------|
| Assignment | `=` | `x = 2` | Assigns a value to an immutable variable | Under the hood theses will be Rust Immutable References |
| Mutable Assignment | `:=` | `x := 2` | Assigns a value to a mutable variable | Under the hood these will be Rust Mutable References |
| Indexing | `[]` | `list[2]` | Allows for accessing a data structure via an index value | Can be redefined with a Certain Type Class |
| Attribute Operator | `@` | `@Atomic`<br />`x := 2`<br />`Thread-Spawn`<br />`fn calculate(x : Int) -> Int` | Used to add Attributes to global variables and functions. |  |
| Product Type Access | `.` | `data.member` | Unnecessary With Style function calls<br />Used to access named parts of a product type. | This might get merged into the Koka-Style Function Call |
| Koka-Style Function Call | `.` | `'a'.toInt()`<br />`map.get("Hello")`<br />`[1,2,3].map() fn (x) {x + 1}`<br /><br />`card.Suit` = `card.Suit()` | If the type of the first parameter of a function matches the type the first value being passed in then it can be called in this way. If the last parameter is a lambda then we can put the lambda outside of the parenthesis.<br />We might also take it one step further and make it so that if a function only takes one argument, we can drop the parenthesis. |  |
| Namespace | `::` | `functional_io::open("data.csv", 'r')`  | Used for namespaces to prevent name collisions in files. |  |
| Sum Type Access | `::` | `Maybe::None` | Used to specify the particular Sum Type and its value to prevent name collisions. |  |
| Tuple Operator | `()` | `()`<br />`(3, 4)` | Used to create a new tuple or unpack one | This might be replaced by a function that takes a `...` or a `H-List` that returns a tuple for constructing one |
| Exclusive Range | `..` | `1..10` | Used to create an iterable thing that we can convert into a list. |  |
| Inclusive Range | `..=` | `1..=10` | Used to create an iterable thing that we can convert into a list. |  |
| Wild Card | `_` | `for _ in 1..10` | Used to match anything in and disregard the value |  |
| Match Arm | `=>` | `Maybe::None =>` | For use in match statements. |  |
| Function Return Arrow | `->` | `fn add1(x: Int) -> Int` | For use in functions to indicate return type. | If the return value doesn't match the type, then we crash. |
### Keywords

| Name | Description |
|------|-------------|
| `class` | The way you declare a type class |
| `instance` | The way you implement a type class |
| `default` | Used with instance to declare a default implementation for a type class |
| `sum` | Used with `data` to declare a sum type |
| `product` | Used with `data` to declare a product type |
| `type` | Used to declare a type alias or to be used with `sum` or `product` to declare an algebraic type. |
| `fn` | Used to declare a function or a lambda |
| `match` | Used for the match statement |
| `while` | Used to declare a while loop |
| `elwhile` | Used to declare an alternate branch to take if the first while condition is not satisfied. |
| `for` | Used to declare a for loop |
| `loop` | Used to declare an infinite loop |
| `if` | Used to declare an if statement |
| `elif` | Use for an else-if statement |
| `else` | Used to declare an else block for if statements and while loops |
| `continue` | Used to skip to next iteration of loop |
| `break` | Used to escape current loop |
| `break label` | Used to break a labeled loop to escape nested loops |
| `in` | Used with a for loop to iterate |
| `typeis` | Used to check if a variable is a certain type. Can also be used to check for a type class implementation. |
| `effect` | For defining Algebraic effects and their associated functions and control flow |
| `with` | For use with Algebraic effect handlers |
| `return` | Used for forcing a function to return |
| `mod` | For declaring a module. |
| `import` | For accessing a module |

### Attributes

| Name | Declaration | Description | Implementation Notes |
|------|-------------|-------------|----------------------|
| Operator Order | `Op-Ord(tier)` | Takes in a number to decide how the parser should treat operator precedence. | By default, all functions will go first and have 0 priority. This overwrites that so that we can have an order of operations for infix functions. |
| Atomic | `Atomic` | Makes a number Atomic and available to all threads. |  |
| Thread Local | `Thread-Local` | Makes a global variable local to a thread. These can be mutable. | This is the default behavior for all globals. |
| Thread Shared | `Thread-Shared` | Makes a global variable available to all threads but not mutable in any thread. |  |
| Thread Mutable | `Thread-Mutable` | Makes a global variable available and mutable to all threads but it will be protected by a Mutex. |  |
| Thread Spawn | `Thread-Spawn` | Makes a function spawn a new thread when called. | Under the hood we should give out a promise that will eventually hold the return value of the function. Likely implemented as a mutex lock that we already have in the thread we spawn. |
| Derive | `Derive(typeclass, ...)` | Allows for using the default implementation of a type class | This might be tricky to implement. |
| Minimal | `Minimal` | Allows for marking what functions are needed to implement a type class |  |
| Default | `Default` | Allows for marking functions in a type class that are not needed to be implemented if the Minimal function is implemented or if the implementation can be inferred from another function. |  |
| Control | `Control` | For defining that a function impacts control flow, For use with effects. This should only be for functions that will modify control flow. Exceptions are an example of this. | This is how we can define exceptions if we want to. Control, tells us to travel down the stack until we hit the with statement. Otherwise we use the handler provided by the handler block. |
| Final | `Final` | For defining what happens when there is no effect handler implemented for an effect. | This is how we indicate which function should be called when we don't hit a handler. |

### Types

| Name | Declaration | Examples | Description | Implementation Notes |
|------|-------------|----------|-------------|----------------------|
| Signed Integer | `Int` | `42`<br />`42i`<br />`0xF`<br />`0o1` | A signed 64 bit integer<br />Can be a decimal, hexadecimal, or octal constants.<br />An `i` can be added at the end to specify signed-ness. |  |
| Unsigned Integer | `UInt` | `42`<br />`42u`<br />`0xF`<br />`0o1` | An unsigned 64 bit integer<br />Can be inferred from the type or can have a `u` attached to the end to specify that the type is unsigned.<br />Hex and Octal literals work too | May not get implemented to keep the language simple |
| Floating Point | `Float` | `42.0`<br />`42.0e10` | A double precision floating point number.<br />Can use exponent form for literals |  |
| Char | `Char` | `'a'` | A character representing a utf-8 codepoint |  |
| Byte | `Byte` |  | A type representing a byte of a value. |  |
| List | `List a` | `List Int`<br />`[1..10]`<br />`[1, 2, 3, 4]` | A List of homogeneous typed values. | Implemented as a reversed Rust Vec to allow for faster insertion. Also type checked to prevent adding nonmatching types into list. |
| H-List | `H-List` | `['a', 1, 42.0, [1, 2 , 3]]` | A List of heterogeneous typed values | By default all lists are heterogeneous, we just don't check the value of the list when adding new values to the list. |
| VA-Args | `...` | `fn format(fmt: String, values: ...)` | An alias to H-List that allows for having variable length arguments to functions |  |
| Vector | `Vector a` | `Vector Int` | An immutable list with set length | Likely to be implemented as a Rust slice. |
| H-Vector | `H-Vector` | `Vector` | An immutable list with a set length that can contain non-matching values | Also to be likely implemented as a Rust slice. |
| Mutable Reference | `& a` | `&Int`<br />`&Hashmap` | A mutable reference to a mutable variable |  |
| Tuple | `()` | `(Int, String)`<br />`(42, "Hello World")` | A tuple data type, Can be mix of any type. |  |
| Function | `fn add1(x : Number a) -> a`<br />`fn (x) {...}` | `fn add1(x : Number a) -> a`  
`fn (x) {...}` | A function datatype | Likely to be a list of variables and their types and a return type |
| Promise |  |  | A Monad that implicitly will block if the thread that returns it isn't finished. | Likely to be implemented as a Mutex. |

### Base Type Classes

These are minimum type classes needed to implement everything else.

#### Collection

| Functions | Description |
|-----------|-------------|
| `fn (in)(c) -> [c]` | For iterating over a collection, at the moment it just gets converted into a list |
| `fn (add:)(c, [c]) -> [c]` | Cons operator. For adding a single item to the front of a list. This is the R-Expression version |
| `fn (remove:)([c]) exn -> (c, [c])` | Removes first element of a list and returns a tuple of the first and rest of the list. This the L-Expression version |
| `fn (++)([c], [c]) -> [c]` | Concatenation Operator |
| `fn size(c) -> UInt` | Function to get size of the collection |
| `@Default`<br />`fn empty(c) -> Bool` | Function to check to see if a collection has any values in it. If `size` is defined then we don't have to implement `empty`. |

#### Access

This type class is for being able to use the indexing operator for a data type

| Functions | Description |
|-----------|-------------|
| `fn (get[])(c, i) exn -> v` | For getting a value from a data type via the index. |
| `fn (set[])(&c, i, v) exn -> ()` | For setting a value from a data type via the index. The first parameter must be a mutable reference since this is a mutation |

#### Number

This type class is here to allow us to perform operations on number types

| Functions | Description |
|-----------|-------------|
| `@Op-Ord(3)`<br />`fn (+)(a, a) -> a` | Addition operator |
| `@Op-Ord(3)`<br />`fn (-)(a, a) -> a` | Subtraction operator |
| `@Op-Ord(2)`<br />`fn (*)(a, a) -> a` | Multiplication Operator |
| `@Op-Ord(2)`<br />`fn (/)(a, a) -> Maybe a` | Division Operator |
| `Op-Ord(2)`<br />`fn (%)(a, a) -> Maybe a` | Modulus Operator |
| `fn abs(a) -> a` | Absolute value function |
| `fn (negate)(a) -> a` | Allows for overriding the unary minus operator |
| `fn signof(a) -> a` | Gets the sign of a number and returns -1, 0, or 1 |

#### Eq

This type class provides boolean comparisons.

| Functions | Description |
|-----------|-------------|
| `@Minimal Op-Ord(5)`<br />`fn (==)(a, a) -> Bool` | Equality checking operator. Only this operator is needed to be implemented since we can just have the default just negate the result of this operator |
| `@Default Op-Ord(5)`<br />`fn (/=)(a, a) -> Bool` | Operator for checking if things are not equal. This doesn't need to be implemented if the equality operator is implemented |

#### Ord

This type class inherits from Eq and provides for ways to order data

| Functions | Description |
|-----------|-------------|
| `@Minimal Default`<br />`fn compare(a, a) -> Ordering` | This function allows us to implement every other function in the type class. However, if we want a slight optimization then we can implement all the other functions instead, hence the `Default` also on this function. |
| `@Default Op-Ord(4)`<br />`fn (<)(a, a) -> Bool` | Less than function, If compare is implemented, then we don't need to implement. |
| `@Default Op-Ord(4)`<br />`fn (<=)(a, a) -> Bool` | Less than or equal function. We don't need to implement if `==` and `<` are implemented |
| `@Default Op-Ord(4)`<br />`fn (>)(a, a) -> Bool` | Greater than function. If compare is implemented, then we don't need to implement. |
| `@Default Op-Ord(4)`<br />`fn (>=)(a, a) -> Bool` | Greater than or equal function. We don't need to implement if `==` and `<` or compare is implemented |
| `@Default`<br />`fn max(a, a) -> a` | Function to get the max of two values. If compare or the other functions are implemented then we can infer how to get a max. |
| `@Default`<br />`fn min(a, a) -> a` | Function to get the minimum of two values. If compare or the other functions are implemented then we can infer how to get a min. |

#### Bits

This type class also inherits from Eq and provides ways to manipulate the bits of the data.

| Functions | Description |
|-----------|-------------|
| `@Op-Ord(6)`<br />`fn (&)(a, a) -> a` | Bitwise And |
| `@Op-Ord(8)`<br />`fn (|)(a, a) -> a` | Bitwise Or |
| `@Op-Ord(7)`<br />`fn (^)(a, a) -> a` | Bitwise Xor |
| `@Op-Ord(1)`<br />`fn (~)(a) -> a` | Bitwise not |
| `fn shiftL(a, UInt) -> a` | Bitwise Left Shift |
| `fn shiftR(a, UInt) -> a` | Bitwise Right Shift |

#### Show

The type class for converting a datatype into a string

| Functions | Description |
|-----------|-------------|
| `@Default`<br />`fn show(a) -> String` | Takes in any type and converts it into a string. The default implementation is similar to how Rust's Debug print looks. |

#### Drop

A very important type class. This allows types to declare how they behave when they go out of scope.

| Functions | Description |
|-----------|-------------|
| `@Default`<br />`fn drop(a) e -> ()` | If implemented then the behavior of when going out of scope will be defined. <br />This might be a useful type class to implement for debugging purposes. |

### Functions

These are the minimal functions are needed to be built into the language.

| Functions | Description |
|-----------|-------------|
| `fn print(Show a) Console -> IO ()` | Prints a value to stdout and implicitly calls show on what was passed into it. Does not add a newline. <br />Returns an IO monad to allow for Haskell style Monadic programing. |
| `fn println(Show a) Console -> IO ()` | Prints a value to stdout and implicitly calls show on what was passed into it. Adds a newline to the end.<br />Returns an IO monad to allow for Haskell style Monadic programming. |
| `fn eprint(Show a) Console -> IO ()` | Prints a value to stderr and implicitly calls show on what was passed into it. Does not add a newline to the end.<br />Returns an IO monad to allow for Haskell style Monadic programming. |
| `fn eprintln(Show a) Console -> IO ()` | Prints a value to stderr and implicitly calls show on what was passed into it. Adds a newline to the end.<br />Returns an IO monad to allow for Haskell style Monadic programing. |
| `fn format(String, ...) -> String` | Function for mapping values onto a String, Takes a String and a H-List holding the rest of the arguments. |
| `fn panic(String) -> ()` | Function for causing program to crash with a message. Maps to Rust's panic macro. |

#### Expressions

| Name | Example | Syntax | Description |
|------|---------|--------|-------------|
| If-elif-else | `if x == 2 {`<br />`    2`<br />`}`<br />`elif x == 5 {`<br />`    5`<br />`}`<br />`else {`<br />`    3`<br />`}` | `if <Bool expr> <code block>`<br />`elif <Bool expr> <code block>`<br />`else <Bool expr> <code block>` | To be an expression there must be an else branch, otherwise we are just a statement. |
| Match | `match x {`<br />`    (a, b) => a + b,`<br />`    (2, 4) => 6,`<br />`    Just(a) => a,`<br />`    (x:xs) => {},`<br />`    _ => Maybe::None, `<br />`}` | `match <expr> {`<br />`    <pattern> => <expr/block>,`<br />`    ...`<br />`}` | Match statement is similar to the Rust one  and can be used to deconstruct tuples, lists, algebraic data types. It can also have if branches.<br />Pattern matches must include all potential branches. |
| Parenthesis | `(2 + x) * 5` | `(expr)` | Used for forcing a certain precedence of operations |
| Infix Expr | `3 + 4` | `<expr> <op> <expr>` | Syntax for infix expressions, Operator precedence must be defined for expressions to be be parsed correctly |
| Normal Function Call | `pow(2, 2)` | `<identifier>(<arg1>,<arg2>, ... <arg n>)` | C-Like function call |
| Koka Function Call A | `'a'.toInt()`<br />`card.suit()` | `<variable>.<identifier>(<arg1>, ...)`<br />`<literal>.<identifier>(<arg1>, ...)`<br />`<expr>.<identifier>(<arg1>,...)` | A Koka like function call. It is syntactic sugar for calling functions that have matching types with a value in a literal or variable. |
| Koka Function Call B | `'a'.toInt`<br />`card.suit` | `<variable>.<identifier>`<br />`<literal>.<identifier>`<br />`<expr>.<identifier>` | Another Koka like function call that drops the parenthesis of a function call if there is only one parameter |
| Literal | `"string"`<br />`2`<br />`42.0` | `<literal>` | A single literal is also an expression. |
| Code Block | `{`<br />`    //Some Code here`<br />`}` | `<code block>` | A code block. This will be implemented as a closure to prevent variables from escaping into the function. |
| Type Is | `x typeis String`<br />`x typeis Hash String` | `<var/value> typeis <typeclass> <type>` | Returns True if the var/value matches the type and type class. Type class is optional but will restrict the test. |
| Effect Handler | `with fn throw-exn(exn) {`<br />`  println(exn.show())`<br />`} {`<br />`    div(3, 0)`<br />`}` | `with <function> <code block>` | The function has to have a matching name and parameters in order to properly handle effects. If the function we are handling has the `Control` attribute then that means that we execute whatever code that comes after the effect handler. Otherwise we call the handler function in the place of the effect function in the code we were just executing. |
| Lambda Declaration | `fn (x : Int) {`<br />`    x + 1`<br />`}` | `fn (<arg 1>, ...) <code block>` | This is how we make a lambda in our language. |

#### Statements

| Name | Syntax | Example | Description |
|------|--------|---------|-------------|
| If | `if <Bool expr> <code block>` | `if x != 2 {`<br />`    print("Hello,World!")`<br />`}` | If there is no else branch then the if statement is no longer an expression and we can't return from it. |
| While | `while <Bool expr> <code block>`<br />`elwhile <Bool expr> <code block>`<br />`<...>` | `while x < 2 {`<br />`    x = x * 2`<br />`}`<br />`elwhile x > 3 {`<br />`    x = x - 4`<br />`}` | The while loop. The `elwhile` is syntactic sugar for a more complicated while loop with a bunch of if statements. It would be great for state machines |
| For | `for <pattern> in <iterable> <code block>` | `for x in [1,2.3] {`<br />`    println(x)`<br />`}` | The for loop is similar to Rust's for loop. |
| Loop | `loop <code block>` | `loop {`<br />`    //do something`<br />`}` | An infinite loop |
| Label | `<label> : <loop>` | `outer: loop {`<br />`    //do something`<br />`}` | A way of marking a loop so that it can be broken by a label. |
| Immutable Assignment | `<variable> = <expr>`<br />`<variable> <type statement> = <expr>` | `x = 2`<br />`x : Int = 2` | How to declare an immutable variable. A let binding.<br />We cannot mutate the value but we can overwrite it. |
| Mutable Assignment | `<variable> := <expr>`<br />`<variable> <type statement> := <expr>` | `x := 2`<br />`x : Int := 2` | How we declare a mutable variable. This allows us to pass it by mutable reference. |
| Type Statement | `: <typeclass> <type>` | `: Int`<br />`: a`<br />`: Ord a` | This is how we specify types for let bindings and variables. Type class doesn't need to be specified but will be enforced. |
| Function Declaration | `fn <name>(<arg1>, ...<argN>)<effects> -> <type> <code block>`<br />`fn (<op>)(<arg1>, ...<argN>)<effects> -> <code block>` | `fn hello_world() {`<br />`    println("Hello World")`<br />`}`<br />`fn (**)(Number a, Number b) -> c {`<br />`   pow(a, b)`<br />`}` | Infix functions are declared with parenthesis surrounding the operator which can be any Unicode symbol. |
| Type Class declaration | `class <Name> <types> {`<br />`    <function 1>,`<br />`    <function 2>,`<br />`    ....`<br />`}`<br />`class <Parent> <types> => <Name> <types> {`<br />`    <function 1>,`<br />`    ...`<br />`}` | `class Eq a {`<br />`    @Minimal`<br />`    fn (==)(a, a) -> Bool,`<br />`    @Default`<br />`    fn (/=)(x: a, y: a) -> Bool {`<br />`        not(a == a)`<br />`    }`<br />`}`<br />`class Eq a => Ord a {`<br />`    @Minimal Default`  
`    fn compare(a, a) -> Ordering,`<br />`    ...`<br />`}` | Type classes are how we do interface inheritance. Type classes can have parent type classes as well. |
| Type Class Implementation | `instance <Name> <type> {`<br />`    <function implementation>`<br />`    ...`<br />`}` | `instance Monad (Maybe a) {`<br />`    fn (>>=)(monad : Maybe a, transform : f(a) -> Maybe b) -> Maybe b {`<br />`    match monad {`<br />`        Maybe::None => None,`<br />`        Maybe::Just a => a.transform(),`<br />`    }`<br />`    ...`<br />`}` | This is how we implement type classes. |
| Effect Declaration | `effect <name> <type> {`<br />`    <function 1>`<br />`    @Final `<br />`    <handler func>`<br />`}`<br />     | `effect exn {`<br />`    @Control`<br />`    fn throw-exn(exn : exception) -> a,`<br />`    @Final`<br />`    fn uncaught-exn(exn : exception) -> () {`<br />`        eprintln(uncaught exception: " ++ exn.show())`<br />`    }`<br />`}` | We will need a way to check for effects leaking outside of the function without a handler.<br />We need to define a function that has a `Final` attribute so that we know what to do when an effect leaks outside of the function. |
| Product Type Declaration | `product type <name> <types> {`<br />`    <name> <type stmt>,`<br />`    ...`<br />`}` | `product type Hashmap a b {`<br />`    list: List b,`<br />`    size: UInt,`<br />`}` | Product types can also be dynamic if we leave off the types. |
| Sum Type Declaration | `sum type <name> <types> {`<br />`    <name>,`<br />`    <name>(<type>),`<br />`}` | `sum type Maybe a {`<br />`    None,`<br />`    Just(a),` | Sum types can also be dynamic if we leave off the types. |
| Type Alias Declaration | `type <name> = <value>` | `type String = List Char` | This is how we make aliases. Under the hood we should allow this to work like an interface so that things like Strings can use List functions transparently. |
| Return | `return <exp>`<br />`<exp>` | `return x + 2;`<br />`x + 2` | This is how we can return early from a function if we want to.<br />We implicitly return if we remove the semicolon off of an expression but only if we are not in a loop. |
| Generic Statement | `<expr>;`<br />`<statement>;` | `x + 2;`<br />`x = 2;` | Any statement is created by putting a semicolon at the end. |
| Module Declaration | `mod <name>;`<br />`mod <name> {<contents>}` | `mod IO;`<br />`mod Test {`<br />`    ...`<br />`}` | How we declare the name of the current module or sub modules. |
| Import  | `import <module>;`<br />`import <module>::<function/submodule/type>;`<br />`import <module>::{<func/mod/type>, <func/mod/type>};` | `import std::IO;`<br />`import std::IO::open;`<br />`import std::IO::{open,File};` | How we access modules from other files/modules. |

#### Syntactic Sugar

| Name | Example | Desugar |
|------|---------|---------|
| Overloaded index (`[]`) operator | `x = a[2]`<br />`a[2] = x` | `x = a.get(2)`<br />`a.set(2, x)` |
| Koka Function Call | `x.toInt`<br />`x.toInt()` | `toInt(x)` |
| Generated Getters and Setters for Product types | `product type Dictionary {`<br />`    table: H-List,`<br />`    size: UInt,`<br />`}`<br />`dictionary.table`<br />`dictionary.table = 3`<br />`dictionary.size` | `dictionary.table()`<br />`dictionary.table(3)`<br />`table(dictionary)`<br />`table(dictionary, 3)` |

#### Example Code

##### Hello World

```
fn main() Console {
    println("Hello, World!");
}
```

##### Hello World but with Monads

```
fn main() -> IO () {
    println("Hello, World!")
}
```

##### File IO

```
import std::IO::{open, readln, writeln};

fn main(argv: List String) exn {
    file = open(argv[1], 'r');
    string = file.readln();
    file2 = open(argv[2], 'w');
    file2.writeln(string);
}
```

##### File IO but with Monads

```
import std::FIO:{open, readln, writeln};

fn main(argv: IO List String) exn -> IO () {
    argv >>= fn (argv) { open(argv[1], 'r') >>= fn (read_file) { read_file.readln >>= fn (string) { argv[2].open('w') >>= fn (write_file) { write_file.writeln(string)}}}}
}
```

##### Type Classes

```
class Access c i {
    fn (get[])(c, i) -> v,
    fn (set[])(&c, i, v) -> c,
}
```

##### Object Oriented Programming

```
//Python-like dictionary
product type Dictionary {
    table: H-List,
    size: UInt,
}

fn ({})() -> Dictionary {
    Dictionary(H-List(), 0)
}

fn rehash(&dict) -> Dictionary {
    new_table := [None];
    for _ in 0..(dict.table.size() * 2) {
        new_table := [None]:new_table;
    }
    table = dict.data;
    dict.data = new_table;
    dict.size = 0;
    for list in table {
        match list {
            None => continue,
            Just(list) => for (key, value) in list {
                dict.set(key, value);
            },
        }
    }

    dict
}

fn get(dict, Hash key) exn {
    index = hash(key);
    dict.table[index]
}

fn set(&dict, Hash key, value) exn {
    //This is an arbitrary 
    dict := if map.size / 2 > map.table.size() {
        dict.rehash()
    } else {
        dict
    };
    val index = hash(key) % dict.table.size();
    match dict.table[index] {
        None => {
            dict.table[index] = H-List((key, value));
            dict.size = dict.size + 1
        },
        Just(row) => {
            new_row = row.filter() fn (pair) { key != pair.0 };
            dict.table[index] = (key, value):new_row;
            if row.size() < new_row.size() {
                dict.size = dict.size + 1
            } else {
                dict
            }
        }
    }
}

instance Access c i {
    fn (get[])(dict: c, key: Hash i) {
        dict.get(key)
    }
    fn (set[])(dict : &c, key :Hash i, value: v) -> Dictionary {
        dict.set(key, value)
    }
}

instance Collection c {
    fn (in)(dict: c) { 
        c.table.foldl([]) fn (acc, list) { match list { None => acc, Just(list) => acc ++ list,}
    }
    fn (add:)((Hash key, value), dict: c) -> c {
        dict.[key] = value
    }
    fn (remove:)(c) exn -> (v, [c]) {
        throw("Not Possible for a dictionary.")
    }
    fn (++)(dict1, dict2) -> Dictionary {
        table = dict1.table ++ dict2.table;
        dict1.table = table;
        rehash(dict1)
    }
    fn size(dict) -> UInt {
        dict.size
    }
    //empty doesn't need to be implemented because we can just infer it from size().
}



fn main() {
    dictionary = {};
    dictionary["key"] = 22;
    dictionary["answer"] = 42;
    dictionary[33] = "a string";
    println(dictionary)
}
```

## Additional Resources

[https://faculty.cs.byu.edu/~kimball/630/effect-types.html](https://faculty.cs.byu.edu/~kimball/630/effect-types.html)

<https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/>
