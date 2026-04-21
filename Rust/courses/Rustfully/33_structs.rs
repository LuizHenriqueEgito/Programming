/*
Structs são como tuplas mas mais estruturados pois podemos nomear cada elemento
*/
struct Fruit {
    name: String,
    grams: i32,
    price: f32,
}

fn create_fruit(name: String, grams: i32) -> Fruit {
    Fruit {
        name: String::from(name),
        grams: grams,
        price: (0.02 * grams as f32)
    }
}

// O RUST aceita isso
fn create_fruit_simplified(name: String, grams: i32) -> Fruit {
    Fruit {
        name,
        grams,
        price: (0.02 * grams as f32)
    }
}

fn main() {
    // Não precisa seguir a mesma ordem da struct
    // Com mut tudo se torna mutavel
    let mut apple = Fruit {
        name: String::from("Apple"),
        grams: 100,
        price: 5.5
    };
    let range = create_fruit("Orange", 156);
    let name = apple.name;
    let grams = apple.grams;
}