// Closures is like as lambda functions in Python
fn main() {
    let double = |x: i32| -> i32 { x * 2 };
    let result = double(7);
    println!("double(7) = {}", result);
}