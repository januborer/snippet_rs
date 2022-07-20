## Modules Info

#### Keywords
`use`,`as`,`pub`,`mod`,`self`,`super`,`::`,`use ... as ...`,`pub use`--re-exporting

#### concepts
- `Packages`
- `crate`:`binary crate`(one or more),`lib crate`(only one),`crate root`
- `Pathes`:`create::parent::child`


#### Base
- `mod module`,`src/module.rs`,`src/module/mod.rs`
- `mod submodule`,`src/parent_module/submodule.rs`,`src/parent_module/submodule/mod.rs`
- Rules
> - everything be private by default unless annotated with pub.
> - Items in child modules can use the items in their ancestor modules.(exept `Enum`)

- Notice that the entire module tree is rooted under the implicit module named `create`
- Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.
- `mod` and `scope`
- nested pathes:`use std::io::{self,Write}`
- glob operator:`use std::collections::*`
- Note that you only need to load a file using a `mod` declaration once in your module tree(建立代码之间的联系). Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared, as covered in the “Paths for Referring to an Item in the Module Tree” section. In other words, mod is not an “include” operation that you may have seen in other programming languages.
