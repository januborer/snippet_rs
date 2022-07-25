use snippets::{
    custom_types::{
        constant::test_constant, constant::LANGUAGE, enumerate::test_enum, structure::test_struct,
    },
    *,
};

fn main() {
    // test_lib();

    // test_modules();

    // test_comments();

    // let myvec: Vec<&str> = myvec!["hello", "jjj"];
    // println!("{:?}", myvec);

    // test_format_print();

    // test_primitives();
    //
    test_struct();
    //
    //
    test_enum();

    test_constant();
    println!("const:{}", LANGUAGE)
}
