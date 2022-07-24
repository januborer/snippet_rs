use std::{fmt, mem};

pub fn primitives() {
    //type annotated
    let logical: bool = true;
    //Regular annotated
    let a_float: f64 = 1.0;
    //Suffix annotated
    let an_integer = 5i32;
    let hex_integer = 0x8Af;
    let octal_integer = 0o2345;
    let binary_integer = 0b10100010;
    let underscore_integer = 2_999_190.000_001;
    //By default
    let default_float = 3.0; //f64
    let default_integer = 7; //i32
                             //inferred from context
    let mut inferred_type = 13; //type i64 is inferred from another line
    inferred_type = 234523454i64;

    //shadowing
    let mut mutable = 12;
    mutable = 3234;
    let mutable = true;

    //logic
    let logic = true && false;
    let logic = true || false;
    let logic = !true;

    //bitwise operations
    let bitwise = 0b101u32 & 0b00101;
    let bitwise = 0b101u32 | 0b00101;
    let bitwise = 0b101u32 ^ 0b00101;
    let bitwise = 0b101u32 >> 5;
    let bitwise = 0b101u32 << 2;

    //tuple
    let long_tuple = (
        -1i8,
        -2i16,
        -3i32,
        -4i64,
        1u8,
        2u16,
        3u32,
        4u64,
        0.1f32,
        0.2f64,
        'a',
        true,
        (),
    );
    println!("long typle 0:{}", long_tuple.0);
    println!("long typle 1:{}", long_tuple.1);
    let tuple_of_tuples = ((8, 13), ('a', true, -2i32), -2i16);
    println!("tuple of tuples:{:?}", tuple_of_tuples);
    // let long_tuple = (
    // -1i8,
    // -2i16,
    // -3i32,
    // -4i64,
    // 1u8,
    // 2u16,
    // -1i8,
    // -2i16,
    // -3i32,
    // -4i64,
    // 1u8,
    // 3u32,
    // 4u64,
    // 0.1f32,
    // 0.2f64,
    // 'a',
    // true,
    // (),
    // );
    // println!("{:?}", long_tuple);
    let pair = (33i32, true);
    println!("pair is:{:?}", pair);
    println!("reverse pair is:{:?}", reverse(pair));
    println!("one element in tuple:{:?}", (4u64,));
    println!("just an integer:{:?}", (4u64));

    //destructured
    let tuple = (1, "Hello", 3, false, 5);
    let (a, b, c, d, e) = tuple;
    println!("{:?},{:?},{:?},{:?},{:?}", a, b, c, d, e);

    let matrix = Matrix(2.2, 3.3, 4.4, 5.3);
    println!("matrix :{:?}", matrix);
    println!("matrix :{}", matrix);
    println!("matrix :{}", transpose(&matrix));
    println!("matrix :{}", matrix.transepose());

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    println!("the first element of xs:{}", xs[0]);
    println!("the second element of xs:{}", xs[1]);
    println!("the len element of xs:{}", xs.len());
    println!("array occupies {} bytes.", mem::size_of_val(&xs));
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    println!("borrow the section array as a slice");
    analyze_slice(&xs[1..4]);
    //empty slice `&[]`
    let empty_arr: [i32; 0] = [];
    assert_eq!(&empty_arr, &[]);
    assert_eq!(&empty_arr, &[][..]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}:{}", i, xval),
            None => println!("Slow down! {} is too far", i),
        }
    }
    println!("panic:{}", xs[5]);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1);
        write!(f, "\n");
        write!(f, "({},{})", self.2, self.3)
    }
}
impl Matrix {
    fn transepose(&self) -> Matrix {
        Matrix(self.0, self.2, self.1, self.3)
    }
}

fn transpose(pair: &Matrix) -> Matrix {
    Matrix(pair.0, pair.2, pair.1, pair.3)
}

fn analyze_slice(slice: &[i32]) {
    println! {"the first element of the slice:{}",slice[0]};
    println! {"the length of the slice:{}",slice.len()};
}
