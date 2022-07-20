// use super::back_of_house::cook_order;
use crate::modules::back_of_house::cook_order;

pub mod hosting {
    pub fn add_to_waitlist() {
        println!("add to waitlist");
        super::cook_order()
    }
}
pub mod service {
    pub mod kitchen {
        pub fn wash_plates() {
            println!("wash plates");
        }
    }
}
