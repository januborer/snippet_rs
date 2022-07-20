mod back_of_house;
mod front_of_house;
mod garden;

use crate::modules::back_of_house::Appetizer;

fn deliver_order() {
    println!("deliver order");
}

pub fn eat_at_restaurant() {
    crate::modules::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    front_of_house::service::kitchen::wash_plates();

    //note:this is not `use crate::modules::back_of_house::Breakfast;`
    let mut breakfast = back_of_house::Breakfast::summer("Rye");
    println!("{:#?}", breakfast);
    breakfast.toast = String::from("rice").to_string();
    println!("{:#?}", breakfast);

    let order1 = Appetizer::Soup;

    println! {"{:#?}",order1};
}
