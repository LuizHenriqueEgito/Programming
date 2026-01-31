#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Vale para qualquer T
// essa implementação para o tipo T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// vale só para f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Eles podem ser qualquer tipo e podem ser diferentes
#[derive(Debug)]
struct PointDiferent<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> PointDiferent<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: PointDiferent<X2, Y2>,
    ) -> PointDiferent<X1, Y2> {
        PointDiferent {
            x: self.x,   // X1
            y: other.y, // Y2
        }
    }
}

fn main() {
    let inteiro: Point<i32> = Point {x: 5, y: 10};
    let flutuante: Point<f32> = Point {x: 1.0, y:4.0};
    // let diferente = Point {x: 5, y: 4.0};  isso não pode pois T é um unico tipo
    let diferente: PointDiferent<i32, f64> = PointDiferent {x: 55, y: 44.0};

    println!("Inteiro {:?}", inteiro);
    println!("flutuante {:?}", flutuante);
    println!("diferente {:?}", diferente);

    // Métodos também podem usar genéricos
    println!("inteiro.x = {}", inteiro.x());

    // Métodos podem valer apenas para um 'T' específico
    println!("distancia da origem = {}", flutuante.distance_from_origin());

    // Tipos podem ser diferentes na struct e no método
    let dif2: PointDiferent<&str, char> = PointDiferent{x: "Hello", y: 'c'};
    let dif3: PointDiferent<i32, char> = diferente.mixup(dif2);

    println!("dif3.x = {}, dif3.y = {}", dif3.x, dif3.y);
}