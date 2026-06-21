/*
Repetition in macros uses special syntax
- `$(...)*`: Match zero or more times
- `$(...)+`: Match one or more times
- `$(...)?`: MAtch zero or one time (optional)
*/

macro_rules! tuple {
    ($($item:expr),* $(,)?) => {
        ($($item),*)
    }
}

fn main() {
    let t1 = tuple!(1, 2, 3);
    println!("Tuple: {:?}", t1);
}