enum Fruta {
    Maca,
    Banana,
    Morango,
    Acai
}

enum Coordenada {
    DoisD(i32, i32),
    TresD(i32, i32, i32)
}

fn main() {
    let maca: Fruta = Fruta::Maca;
    let banana: Fruta = Fruta::Banana;
    let morango: Fruta = Fruta::Morango;
    let acai: Fruta = Fruta::Acai;
    enumeracao(maca);
    enumeracao(banana);
    enumeracao(morango);
    enumeracao(acai);

    let ponto2d: Coordenada = Coordenada::DoisD(5, 10);
    let ponto3d: Coordenada = Coordenada::TresD(3, 8, 15);

    match ponto2d {
        Coordenada::DoisD(x, y) => println!("Coordenada 2d: {}, {}", x, y),
        Coordenada::TresD(x, y, z) => println!("Coordenada 3d: {}, {}, {}", x, y, z)
    }
    // println!("{}", x);x e y não existe fora do escopo do match

    match ponto3d {
        Coordenada::DoisD(x, y) => println!("Coordenada 2d: {}, {}", x, y),
        Coordenada::TresD(x, y, z) => println!("Coordenada 3d: {}, {}, {}", x, y, z)
    }
}

fn enumeracao(fruta: Fruta) {

    match fruta {
        Fruta::Maca => println!("É uma maçã"),
        Fruta::Banana => println!("É uma banana"),
        Fruta::Morango => println!("É um morango"),
        Fruta::Acai => println!("É uma acai"),
    }
}