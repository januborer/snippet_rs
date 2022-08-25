use macros::format_print::my_print;
use modules::eat_at_restaurant;
mod comments;
mod modules;

mod macros;

mod primitives;

pub mod custom_types;

pub mod flow_of_control;
pub mod functions;
pub mod types;
pub mod variable_bindings;

pub fn test_lib() {
    println!("i am a lib");
}

pub fn test_modules() {
    eat_at_restaurant();
}

pub fn test_comments() {
    comments::comments();
}

pub fn test_format_print() {
    my_print()
}

pub fn test_primitives() {
    primitives::primitives();
}
