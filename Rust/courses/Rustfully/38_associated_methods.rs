struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec = Rectangle::new_square(40);
    println!("rec: {:?}", rec);
}