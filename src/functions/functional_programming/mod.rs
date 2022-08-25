//&F,&mut F,F
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f()
}
//&F
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn call_me<F: Fn()>(f: F) {
    f()
}
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a {}", text)
}
fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a {}", text)
}
fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a {}", text)
}
pub trait Iterator {
    type Item;
    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        f()
    }
}

pub fn test_fn_pro() {
    use std::mem;
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    println!("fn:{}", function(1));
    println!("c_a:{}", closure_annotated(1));
    println!("c_i:{}", closure_inferred(1));

    let one = || 1;
    println!("one:{}", one());

    let color = String::from("Red");
    //closure only holds an immutable reference to color.
    let print = || println!("color:{}", color);
    print();
    let _reborrow = &color;
    print();
    let _color_move = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count:{}", count);
    };
    inc();
    // let _reborrow = &mut count;
    inc();
    let _reborrow = &mut count;

    //non-copy type-->move
    let movable = Box::new(3);
    let consume = || {
        println!("movable :{:?}", movable);
        mem::drop(movable);
    };
    consume();

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    // println!("{}", haystack.len());
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("Then {}", farewell);
        println!("sleep zzz");
        mem::drop(farewell);
    };
    drop(diary);

    let double = |x| 2 + x;
    println!("{}", apply_to_3(double));

    fn function2() {
        println!("i am a function");
    }
    let closure = || println!("I am a closure");
    call_me(function2);
    call_me(closure);

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
