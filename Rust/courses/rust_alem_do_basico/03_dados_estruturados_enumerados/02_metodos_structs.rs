// Essa é a estrutura da "Classe" tipo o init
struct Retangulo {
    h: u32,
    l: u32
}

// Isso é como adicionar "methods" na classe
impl Retangulo {
    fn calcular_area(&self) -> u32 {
        self.h * self.l
    }

    fn new(h: u32, l: u32) -> Self {
        Self{h: h, l: l}
        // Self{h, l}  também poderia abreviar
    }
}

fn main() {
    let ret: Retangulo = Retangulo {
        l: 10,
        h: 20
    };
    let area = ret.calcular_area();
    println!("Area: {}", area);

    let ret2 = Retangulo::new(2, 3);
    let area2 = ret2.calcular_area();
    println!("Area: {}", area2);
}