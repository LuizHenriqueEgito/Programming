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

    // Função associada (não começa com &self)
    // retorna um novo retangulo (Self)
    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size,
        }
    }
}

// Isso funciona!
impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

fn main() {
    let rec1 = Rectangle {
        w: 10,
        h: 20,
    };

    let square: Rectangle = Rectangle::square(3);  // Da struct Rectangle :: (utilize) square()

    println!("A área do 1º retângulo {:#?} é {}.", rec1, rec1.area());
    println!("A área do 2º retângulo {:#?} é {}.", square, square.area());
}