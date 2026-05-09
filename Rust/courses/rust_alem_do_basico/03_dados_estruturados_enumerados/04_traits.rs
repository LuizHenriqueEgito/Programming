// Traits: são como interfaces
// É como um contrato
trait FormaGeometrica {
    fn calcular_area(&self) -> u32;

    fn new(h: u32, l: u32) -> Self;

}

struct Retangulo {
    h: u32,
    l: u32
}

// Isso é como adicionar "methods" na classe
impl FormaGeometrica for Retangulo {
    fn calcular_area(&self) -> u32 {
        self.h * self.l
    }

    fn new(h: u32, l: u32) -> Self {
        // Self{h: h, l: l}
        Self{h, l}
    }
}

fn main() {
    let ret = Retangulo::new(1, 2);
    let area = ret.calcular_area();
    println!("Area: {}", area);
}