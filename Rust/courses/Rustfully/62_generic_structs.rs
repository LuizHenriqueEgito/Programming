// por convenção o RUST usa T como generico mas se você precisar de outros pode seguir a ordem alfabetica
// T, U, V, W por exemplo
use num::Num;

#[derive(Debug)]
struct Point<T: Num> {  // só permite numeros
    x: T,
    y: T,
}

// Exemplo do Option
enum Option<T> {
    Some(T),
    None
}

// Exemplo do Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}


fn main() {
    let p1 = Point {x: 1, y: 2};
    let p2 = Point {x: 1.1, y: 2.2};
    println!("p1={:?}, p2={:?}", p1, p2);

    let conn = true;
    let result: Option<&str> = if conn {Some("Connected!")} else {None};
    println!("result:{result}");
}
