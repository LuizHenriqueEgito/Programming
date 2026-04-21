#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        w: dbg!(30 * scale),
        h: 50,
    };
    println!("A área do retângulo é {} pixels quadrados", area_fn(&rect1));
    // prints mais bonitos use :#? ao invés de :?
    println!("[print bonito] - Rec: {:#?}", rect1);
    println!("[print normal] - Rec: {:?}", rect1);
}

fn area_fn(rectangle: &Rectangle) -> u32 {
    rectangle.w * rectangle.h
}