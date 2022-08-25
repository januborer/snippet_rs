## Info

#### Keywords
- `fn`,

#### Concepts

#### Base
- don't care the order of function definitions.
- don't return value ,actually return the unit type `()`.And this case you can omitted the return type signature.
- associated functions & methods:`&self`=suger=>`self:&Self`
  >1. Associated functions are functions that are defined on a type generally.Be called using double colons.
  >2. Methods are associated functions that are called on a particular instance of a type.Be called using the dot operator.note the first argument `&self` is implicitly passed,`rectangle.area()===rectanble::area(&rectangle)`.Mutable objects can call mutable methods.
- closures:
