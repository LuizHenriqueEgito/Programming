use std::collections::HashMap;  // armazena no HEAP
// As chaves e os valores devem ser os mesmos sempre
// exemplo: String como chave e i32 como valor todas as chaves serão String e todos os valores serão i32

fn main() {
    let mut scores: HashMap = HashMap::new();
    scores.insert(String::from("Bob"), 42);  // 1º vem a chave 2º o valor
    scores.insert(Strng::from("James"), 40);

    let score: i32 = scores.get("Bob").copied().unwrap_or(0);
    println!("Bob score={score}");
    for (k, v) in &scores {
        println!("{k}: {v} points.");
    }

    let name = String::from("Nuna");
    let age = 28;

    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(name, age);  // aqui você passou o ownership de name, mas age ainda existe
    println!("users={:?}", users)
}