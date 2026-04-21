struct Rectangle {
    width: u32,
    height: u32,
}

fn get_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rec = Rectangle {
        width: 20,
        height: 30
    }
    println!("{:?}", rec);
    println!("Area: {}", get_area(rec));
}