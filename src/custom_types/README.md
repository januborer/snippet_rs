## Info

#### Keywords
- `struct`,`enum`,`const`,`static`,`type`(`Self`),`use`,`as`(casting:显示类型转换)

#### Concepts

#### Base
- type aliases:`type a=aaaaaaaaabbbbb;`--->`Self`
- Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:`const`,`static`
- conversion:
  > Rust addresses conversion between custom types (i.e., struct and enum) by the use of `traits`. The generic conversions will use the `From` and `Into` traits. The Into trait is simply the reciprocal of the From trait. That is, if you have implemented the From trait for your type, Into will call it when necessary. Using the Into trait will typically require specification of the type to convert into as the compiler is unable to determine this most of the time.
  >
  > `TryFrom/TryInto` traits are used for fallible conversions, and as such, return Results.
  >
  > converting to string:
  > - derectly: implementing the `ToString` trait.
  > - automagically: implementing the `fmt::Display`
  >   trait which automagically provides `ToString`.
  >
  > parsing a string:`FromStr`trait.
  > - implemented the `FromStr` trait:(numerous types(implemented the `FromStr` within the standard library)) convert into a string:
  > 1. `parse`function:`let parsed:i32="5".parse().unwrap()`
  > 2. 'turbofish' syntax:`let parsed="5".parse::<i32>().unwrap()`
