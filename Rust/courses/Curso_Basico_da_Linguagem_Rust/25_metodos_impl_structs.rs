#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    // self: &Self
    // Poderia ser &mut self ou self
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

fn main() {
    let rec1 = Rectangle {
        w: 10,
        h: 20,
    };

    let mut rec2 = Rectangle {
        w: 5,
        h: 10,
    };

    println!("A área do retângulo {:#?} é {}.", rec1, rec1.area());
    println!("Consegue rec1 conter rec2? {}", rec1.can_hold(&rec2));
}