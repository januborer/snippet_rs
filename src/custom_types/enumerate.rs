use super::linked_list;
use Status::{Poor, Rich};
enum WebEvent {
    //unit-like
    PageLoad,
    PageUnload,
    //like tuple struct
    KeyPress(char),
    Paste(String),
    //c-like
    Click { x: i64, y: i64 },
}
//c-like enum
enum Number {
    Zero,
    One,
    Two,
}
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("c:{}", c),
        WebEvent::Paste(s) => println!("s:{}", s),
        WebEvent::Click { x, y } => {
            println!("x:{},y{}", x, y);
        }
    }
}

type Operate = VeryVerboseEnumOfThingsToDoWithNumbers;

#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: f32, y: f32) -> f32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

enum Status {
    Rich,
    Poor,
}

pub fn test_enum() {
    let pressed = WebEvent::KeyPress('x');
    let pageload = WebEvent::PageLoad;
    let pageunload = WebEvent::PageUnload;
    let paste = WebEvent::Paste("my test".to_owned());
    let click = WebEvent::Click { x: 1, y: 2 };

    inspect(pressed);
    inspect(pageload);
    inspect(pageunload);
    inspect(paste);
    inspect(click);

    let x = Operate::Add;
    println!("aliases:{:?}", x);
    println!("5.0+7.0={}", x.run(5.0, 7.0));
    let y = Operate::Subtract;
    println!("6.0-2.0={}", y.run(6.0, 2.0));

    //use
    let rich = Rich;

    match rich {
        Rich => println!("Rich"),
        Poor => println!("Poor"),
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("red is #{:06x}", Color::Red as i32);

    linked_list::test_linked_list();
}
