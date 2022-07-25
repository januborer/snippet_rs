pub static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn test_constant() {
    let n = 16;
    println!("language is:{}", LANGUAGE);
    println!("THRESHOLD is:{}", THRESHOLD);
    println!("big or small:{}", if is_big(n) { "Big" } else { "Small" });
}
