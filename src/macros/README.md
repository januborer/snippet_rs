## Macros


#### keywords
- 

#### Concepts
- `macros` are a way of writing code that writes other code, which
is known is `metaprogramming`.

#### Base
- `declarative macros` with macro_rules! and three kinds of procedural macros:

    1. Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
    2. Attribute-like macros that define custom attributes usable on any item
    3. Function-like macros that look like function calls but operate on the tokens specified as their argument

- the difference between macros and functions
    1. A function signature must declare the number and type of parameters the function has. Macros, on the other hand, can take a variable number of parameters.
    2. macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.
    3.  you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.
- all happens during compilation.


#### Relative
- [The little book of rust macro](https://veykril.github.io/tlborm/)
- [Rust book macro](https://doc.rust-lang.org/beta/book/ch19-06-macros.html)
- [Rust reference about macro](https://doc.rust-lang.org/beta/reference/macros-by-example.html)
- [macro by example](https://doc.rust-lang.org/beta/rust-by-example/macros.html)
