use std::convert::From;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug)]
struct EventNumber(i32);
impl TryFrom<i32> for EventNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EventNumber(value))
        } else {
            Err(())
        }
    }
}

struct Circle {
    radius: i32,
}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "circle of radius {}", self.radius)
    }
}

pub fn test_conversion() {
    //from & into
    let num = Number::from(3);
    println!("from is :{:?}", num);

    let num: Number = 5.into();
    println!("into is : {:?}", num);

    //try_from && try_into
    println!("try_from ok:{:?}", EventNumber::try_from(8));
    println!("try_from err:{:?}", EventNumber::try_from(5));
    let result: Result<EventNumber, ()> = 8.try_into();
    println!("try_into ok:{:?}", result);
    let result: Result<EventNumber, ()> = 5.try_into();
    println!("try_into err:{:?}", result);

    //conver to string
    let circle = Circle { radius: 9 };
    println!("{}", circle);
    println!("{}", circle.to_string());

    //parsing a string
    let parsed: i32 = "5".parse().unwrap();
    println!("parse {}", parsed);
    let parsed = "5".parse::<i32>().unwrap();
    println!("parse {}", parsed);
}
