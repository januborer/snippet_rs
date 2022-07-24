#[doc(inline)]
pub fn comments() {
    // //! Generate library docs for the enclosing item.
    //line comment

    /* block comment
     * block comment
     */

    /// Generate library docs for the following item.
    /// support [Markdown](https://en.wikipedia.org/wiki/Markdown)
    /// these comments will be compiled into documentation.
    /// Use `cargo doc` to build documentation in `target/doc`.
    /// `cargo test --doc` only run documentation tests.
    /// above commands will invoke `rustdoc`(and `rustc`)

    println!("comments");
}
