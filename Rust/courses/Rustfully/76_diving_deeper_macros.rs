macro_rules! result_type {
    ($name:ident = Result<$ok:ty, $err:ty>) => {
        type $name = Result<$ok, $err>;
    };
}

macro_rules! create_array {
    ($($item:expr),* $(,)?) => {
        [$($item),*]
    }
}

fn main() {
    result_type!(MyResult = Result<i32, String>);
    fn example() -> MyResult {
        Ok(42)
    }

    match example() {
        Ok(val) => println!("Success: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    let arr1 = create_array!(1, 2, 3);
    let arr2 = create_array!(4, 5, 6);

    println!("Array 1: {:?}", arr1);
    println!("Array 2: {:?}", arr2);
}