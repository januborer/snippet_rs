use std::fmt;

//Only types that implement fmt::Display can be formatted with `{}`.
// std::fmt::Debug---->{:?}
// std::fmt::Display---->{:?}
// `ToString` trait
pub fn my_print() {
    println!("{} day", 31);

    println!("{0},this is {1}.{1},this is {}", "janu", "borer");

    println!(
        "{subject} {verb} {object}",
        subject = "The freedom borer",
        verb = "jumps over",
        object = "the lazy dog"
    );

    println!("Base 10 repr:              {}", 69420);
    println!("Base 2 repr:               {:b}", 69420);
    println!("Base 8(octal) repr:        {:o}", 69420);
    println!("Base 16(hexadecimal) repr: {:x}", 69420);
    println!("Base 16(hexadecimal) repr: {:X}", 69420);

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);

    let name = String::from("Borer");
    let age = 29;
    let bio = String::from("Do one thing , and do it well!");
    let id = 1;
    let user = User { name, age, bio, id };

    println!("{}", Structure(32));
    println!("{:?}", Deep(Structure(3)));
    println! {"{:#?}",user}

    let pi: f64 = 3.1415926;
    println!("{pi:.2}", pi = pi);

    println!("{1:?},{:?},{actor:?}", "borer", "janu", actor = "yieg");

    //Minmax
    let minmax = MinMax(0, 13);
    println!("Display:{}", minmax);
    println!("Debug:{:?}", minmax);
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "the big range is {big},the small range is {small}",
        big = big_range,
        small = small_range
    );
    println!("the big range is {big_range},the small range is {small_range}");

    //Point2D
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display:{}", point);
    println!("Debug:{:?}", point);
    // println!("Debug:{:b}", point);
    //
    //
    let list = List(vec![1, 3, 4, 5, 6, 7]);
    println!("Display list: {}", list);

    for city in [City {
        name: "new york",
        lat: 333.4444,
        lon: -3445.33,
    }]
    .iter()
    {
        println!("city:{}", city);
    }
}

#[derive(Debug)]
struct Structure(i32);
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
struct Deep(Structure);
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    bio: String,
    id: u64,
}

#[derive(Debug)]
struct MinMax(i64, i64);
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{},y :{}", self.x, self.y)
    }
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        let mut i: i32 = 0;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, " ")?;
            }
            write!(f, "{i}: {}", v)?;
            i = i + 1;
        }
        write!(f, "]")
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}
impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}:{:.3}° {} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}
