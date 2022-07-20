use modules::eat_at_restaurant;
mod modules;
pub fn test_lib() {
    println!("i am a lib");
}
pub fn test_modules() {
    eat_at_restaurant();
}
