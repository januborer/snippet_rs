use snippets::{
    custom_types::{
        constant::test_constant, constant::LANGUAGE, enumerate::test_enum, structure::test_struct,
    },
    flow_of_control::test_flow_of_control,
    functions::functional_programming::test_fn_pro,
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

    // test_struct();
    //
    //
    // test_enum();

    // test_constant();
    // println!("const:{}", LANGUAGE);

    types::test_types();

    // custom_types::conversion::test_conversion();
    //
    //
    test_flow_of_control();

    test_fn_pro();
}
