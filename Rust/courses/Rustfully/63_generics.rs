struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn coordinates(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

// só disponivel se o tipo for i32
impl Point<i32> {
    fn i32_method(&self) {
        println!("Só está disponivel para i32");
    }
}

fn main() {
    let p1 = Point {x: 1, y: 2};
    let p2 = Point{x: 1.5, y: 2.2};

    println!("p1={p1:?}, p2={p2:?}");
    println!("{}", p1.i32_method());

}