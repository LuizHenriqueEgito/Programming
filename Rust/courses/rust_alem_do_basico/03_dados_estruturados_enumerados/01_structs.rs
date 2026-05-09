/*
Struct é uma estrutura de dados de um objeto
*/

struct Pessoa {
    name: String,
    age: i32,
    h: f32
}

fn estrutura() {
    let egito = Pessoa{
        name: String::from("Egito"),
        age: 29,
        h: 1.72
    };
    println!("Nome: {}", egito.name);
    println!("Idade: {}", egito.age);
    println!("Altura: {}", egito.h);
}

fn main() {
    estrutura();
}