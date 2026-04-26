/*
RUST é estaticamente tipada.

Tipos Escalares:
- int
- float
- bools
- character

Tipos Compostos:
- Tuplas, os elementos podem ser de vários tipos, ela é fixa não pode aumentar ou diminuir
- Arrays, os elementos só podem ser de um unico tipo
*/

fn main() {
    // let guess = "42".parse().expect("Not a number!");  isso dá erro
    let guess: u32 = "42".parse().expect("Not a number!");  // isso funciona
    println!("Guess: {}", guess);

    let tuple: (i32, f64, u8) = (500, 6.1, 1);
    println!("Tupla={:?}", tuple);
}