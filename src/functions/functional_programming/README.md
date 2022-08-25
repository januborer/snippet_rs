## Info

#### Keywords
- `||`,`move`

#### Concepts

#### Base
- Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re defined. We’ll demonstrate how these closure features allow for code reuse and behavior customization.
- Other characteristics of closures include:
    > - using || instead of () around input variables.
    >
    > - optional body delimination ({}) for a single expression (mandatory otherwise).
    >
    > - the ability to capture the outer environment variables.
- Closures can capture variables:
    > - by reference: &T
    >
    > - by mutable reference: &mut T
    >
    > - by value: T
    >
    > - Using move before vertical pipes forces closure to take ownership of captured variables:

