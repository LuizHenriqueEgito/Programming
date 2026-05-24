/*
Em dependencias adicione o pacote
[dependencies]
rand = "0.8.5"
ou rode o comando no temrinal:
cargo add rand
*/

use rand::random;

fn main() {
    println!("Número Aleatório: {}", random::<i8>());
}