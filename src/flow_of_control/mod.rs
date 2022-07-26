#![allow(unreachable_code)]

fn some_number() -> Option<u32> {
    Some(42)
}
fn age() -> u8 {
    15
}
enum Temperature {
    Celsius(u32),
    Farenheit(u32),
}

struct Foo {
    x: (u32, u32),
    y: u32,
}
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    MCYK(u32, u32, u32, u32),
}

pub fn test_flow_of_control() {
    let n = 7i32;
    let big_n = if n < 10 && n > -10 {
        println!("n is :{}", n);
        n * 10
    } else if n > 10 {
        println!("n is :{}", n);
        n / 5
    } else {
        n / 2
    };
    println!("big_n is {}", big_n);

    //loop
    let mut count = 0u32;
    let result = loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("count:{}", count);

        if count == 5 {
            println!("five");
            break count;
        }
    };
    println!("result : {}", result);

    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
            // continue 'outer;
        }
        println!("never reached");
    }
    println!("exit the outer");

    //for
    // for n in 1..101 {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    //for an iterator
    let names = vec!["Rust", "React", "Typescript"];
    for name in names.iter() {
        match name {
            &"Rust" => {
                println!("match the Rust");
            }
            _ => {
                println!("Hello {}", name);
            }
        }
    }
    println!("names are {:?}", names);
    for name in names.into_iter() {
        match name {
            "Rust" => {
                println!("match the Rust");
            }
            _ => {
                println!("Hello {}", name);
            }
        }
    }
    println!("have moved the names");
    let mut names = ["Rust", "React", "Typescript"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Rust" => "Rustacean",
            _ => "Hello",
        }
    }
    println!("mut names are {:?}", names);

    //match
    let number = 13;
    println!("Tell me the number :{}", number);
    //no semicolon
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 => println!("The prime"),
        13..=19 => println!("A teen"),
        _ => println!("the rest cases"),
    }
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("binary ans boolean:{},{}", binary, boolean);

    let triple = (0, 2, 5.0);
    match triple {
        (0, y, z) => println!("First is 0,y is {:?},z is {:?}", y, z),
        (1, ..) => println!("First is 1 ,and the rest does not matter"),
        _ => println!("It does not matter what they are."),
    }

    let array = [1, -2, 5];
    match array {
        [0, second, third] => println!("array[0]=0,array[1]={},array[2]={}", second, third),
        [1, _, third] => println!("array[0]=1,array[2]={} and array[1] is ignored.", third),
        [-1, second, ..] => println!(
            "array[0]=-1,array[1]={} and all the other ones is ignored.",
            second
        ),
        // [-1, second] => println!(""),
        [3, second, tail @ ..] => println!(
            "array[0]=3,array[1]={} and the other elements are {:?}",
            second, tail
        ),
        [first, middle @ .., last] => {
            println!("array[0]={},millde={:?},last={}", first, middle, last)
        }
    }

    let color = Color::RGB(124, 13, 18);
    match color {
        Color::Red => println!("RED"),
        Color::RGB(r, g, b) => println!("r:{},g:{},b:{}", r, g, b),
        _ => println!("other"),
    }

    //pointer & ref
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring :{:?}", val),
    }
    match *reference {
        val => println!("Got a value via destructuring :{:?}", val),
    }
    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    //references can be retriesved by `ref` and `ref mut`
    let value = 5;
    let mut mut_value = 6;
    match value {
        ref val => println!("Got a reference to a value {:?}", val),
    }
    match mut_value {
        ref mut val => {
            *val += 10;
            println!("We add 10 ,the mut_value is :{}", val);
        }
    }

    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("first of x is 1,b={},y={}", b, y),
        Foo { y: 2, x: i } => println!("y is 2,i={:?}", i),
        Foo { y, .. } => println!("y={},don't care x", y),
    }

    let temperature = Temperature::Celsius(35);
    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::Farenheit(t) if t > 85 => println!("{}F is above 85", t),
        Temperature::Farenheit(t) => println!("{}F is below 85", t),
    }

    let number: u8 = 4;
    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("greater than zero"),
        _ => println!("never happen"),
    }

    match age() {
        0 => println!("zero age"),
        n @ 1..=12 => println!("child :{}", n),
        n @ 13..=19 => println!("teen :{}", n),
        _ => println!("person"),
    }

    match some_number() {
        Some(n @ 42) => println!("The answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        None => println!("none"),
    }

    let _red = Color::Red;
    let blue = Color::Blue;
    let rgb = Color::RGB(123, 5, 6);
    let condition = true;
    if let Color::Red = blue {
        println!("red");
    } else if let Color::Blue = blue {
        println!("blue");
    } else if condition {
        println!("conditon");
    } else {
        println!("no match");
    }
    if let Color::RGB(123, ..) = rgb {
        println!("rgb");
    }
    if let Color::RGB(123, g @ 5, b) = rgb {
        println!("rgb,g is {},b is {}", g, b);
    }
}
