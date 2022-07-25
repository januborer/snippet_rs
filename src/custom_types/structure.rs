#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//unit struct
#[derive(Debug)]
struct Unit;

//tuple struct
#[derive(Debug)]
struct Pair(i32, f32);

//classic C struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
impl Point {
    fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

//structs can be reused as fields of another struct.
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
impl Rectangle {
    fn new(top_left: Point, bottom_right: Point) -> Rectangle {
        Rectangle {
            top_left,
            bottom_right,
        }
    }
    fn rect_area(&self) -> f32 {
        let (width, height) = (
            self.bottom_right.x - self.top_left.x,
            self.top_left.y - self.bottom_right.y,
        );
        width * height
    }
    fn square(top_left: Point, length: f32) -> Rectangle {
        let back_top_left = top_left.clone();
        Rectangle {
            top_left,
            bottom_right: Point {
                x: back_top_left.x + length,
                y: back_top_left.y - length,
            },
        }
    }
}

pub fn test_struct() {
    //field init shorthand
    let name = String::from("Borer");
    let age = 28u8;
    let borer = Person { name, age };
    println!("{:?}", borer);

    //access the struct
    let mut point: Point = Point { x: 2.0, y: 3.34 };
    point.x = 6.0;
    println!("x:{},y:{}", point.x, point.y);

    //update syntax
    let bottom_right = Point { x: 7.0, ..point };
    println!("second point:x:{},y:{}", bottom_right.x, bottom_right.y);

    //nested destructure
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    let rectangle = Rectangle {
        // struct instantiation is an expression too.
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    println!("rectangle:{:#?}", rectangle);

    //unit struct
    let unit = Unit;
    println!("unit:{:?}", unit);

    //tuple struct
    let pair = Pair(3, 9.9);
    println!("pari:{},{}", pair.0, pair.1);
    //destruct tuple struct
    let Pair(integer, decimal) = pair;
    println!("integer:{},decimal:{}", integer, decimal);

    //others
    let rectangle = Rectangle::new(Point { x: 2.0, y: 10.0 }, Point { x: 6.0, y: 3.0 });
    println!("rec_area:{}", rectangle.rect_area());
    let square = Rectangle::square(Point { x: 0.0, y: 0.0 }, 2.0);
    println!("square area:{}", square.rect_area());
}
