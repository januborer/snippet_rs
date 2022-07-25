## Info

#### Keywords
- `as`(Casting,rust has no implicit conversion,but can use `as ` explicit conversion)

#### Concepts
- Casting
- Literals:Numeric literals can be type annotated by adding the type as a suffix(`4i32`).
- `type`:Types must have UpperCamelCase names, or the compiler will raise a warning.(`type NanoSecond=u64;`)

#### Base
- A Rust program is (mostly) made up of a series of statements.
- `Statements`:The most common two statements are declaring a variable binding(`let x=5;`),and using a `;` with an expression(`x;`,`x+1;`,`15;`)
- `Expressions`:
  > - Blocks are expressions.(the last expression:1.no semicolon-->the last expression is the local variable.2.has semicolon-->return value `()`)
