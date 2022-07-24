use snippets::*;

fn main() {
    // test_lib();
    test_modules();

    test_comments();

    let myvec: Vec<&str> = myvec!["hello", "jjj"];
    println!("{:?}", myvec);

    test_format_print();
}
