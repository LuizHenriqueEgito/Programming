use std::collections::HashMap;
use std::fmt;


fn soma(x: i32, y: i32) -> i32 {
    x + y
}

struct Pessoa {
    nome: String,
    idade: u8,
}

impl fmt::Display for Pessoa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} anos)", self.nome, self.idade)
    }
}


fn main() {
    let pessoa = Pessoa {
        nome: String::from("Luiz"),
        idade: 30,
    };
    let inteiro: i32 = 42;
    let decimal: f32 = 42.0;
    let texto: String = String::from("Olá texto"); // ← faltava ponto e vírgula aqui!
    let booleano: bool = true;
    const PI: f32 = 3.14;
    let lista: Vec<i32> = vec![1, 2, 3];

    println!("inteiro: {}", inteiro);
    println!("decimal: {}", decimal);
    println!("texto: {}", texto);
    println!("booleano: {}", booleano);
    println!("PI: {}", PI);
    println!("lista: {:?}", lista);
    println!("Pessoa: {}", pessoa);

    
    let resultado = soma(5, 7);
    println!("Resultado da soma: {}", resultado);
    let mut capitais = HashMap::new();
    capitais.insert("Brasil", "Brasília");
    capitais.insert("EUA", "Washington");
    capitais.insert("Japão", "Tóquio");

    if let Some(capital) = capitais.get("Brasil") {
        println!("A capital do Brasil é {}", capital);
    }
}