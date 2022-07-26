## Info

#### Keywords
- `if`,`else`,`loop`,`break`,`continue`,`while`,`for in`,`match`

#### Concepts

#### Base
- `if/else`
  > 1. the boolean condition doesn't need to be
  >    surrounded by parentheses.and each condition is
  >    followed a block.
  >
  > 2. Are expressions, and all branches must return
  >    the same type.
- `loop`:infinite loop.
- nesting and labels:`'label: loop{}`,`break 'label;`,`continue 'label;`
- `break/continue`:break loop,`break 'label;`,`break value;`(return the loop expression value).
- `for`:The `for in` construct can be used to iterate through an `Iterator`.
  > - for and iterators(handling the conversion of a collection into an iterator.)
  >   1. `iter`:borrow each element of the collection.
  >
  >   2. `into_iter`:consume the collection.
  >
  >   3. `iter_mut`:this mutably borrows each element of the collection
- `match`:The first matching arm is evaluated and all possible values must be covered.`expression`.`_`:the rest of cases.
  > destructure:
  > 1.`tuple` can be destructure in a match.
  > 2. `array/slice` can be destructure.
  > 3. `enum` can be destructure.
  > 4. `struct` can be destructure.
  > 5. `pointers/ref`:Dereferencing uses `*`.Destructuring uses `&`,`ref`,`ref mut`.
  > guards:
  > 1.`guards`:a match guard can be added to filter the arm.Note that the compiler won't take guard conditions into account when checking if all patterns are covered by the match expression.
  > 2.match provides the `@` sigil for binding values to names
- `if let`:cleaner ont case match.can use `else` or `else if`.can be used to match any enum value.
- `while let`:else break.
