type NanoSecond = u64;
type U64 = u64;
type Inch = u64;

pub fn test_types() {
    println!("1000 as a u16 is:{}", 1000 as u16);
    // println!("1000 as a u8 is:{}", 1000 as u8);
    println!("-1 as a u8 is:{}", -1i8 as u8);
    // println!("233 as a i8 is:{}", 233 as i8);
    println!("300.0 as u8 is {}", 300.0_f32 as u8);
    println!("-300.0 as u8 is {}", -300.0_f32 as u8);
    println!("-300.0 as i8 is {}", -300.0_f32 as i8);
    println!("nan as u8 is {}", f32::NAN as u8);

    //literals
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    let i = 1;
    let f = 1.0;
    println!("size of `x` in bytes:{}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes:{}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes:{}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes:{}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes:{}", std::mem::size_of_val(&f));

    //type
    let nanoseconds: NanoSecond = 6 as U64;
    let inches: Inch = 2 as U64;
    println!(
        "nanoseconds is {},inchies is {},add is {}",
        nanoseconds,
        inches,
        nanoseconds + inches
    );

    //expressions
    let x = 99u32;
    let y = {
        let inner = 8;
        let m = inner * x;

        m * x
    };
    let z = {
        2 * x;
    };
    println!("expression x:{},y:{},z:{:?}", x, y, z);
}
