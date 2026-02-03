/*
Possui chaves e valores, é como um dicionario do Python
HashMap<K, V> -> HashMap<String, i32>

Para tipos que implementam o trait 'Copy' (ex: i32), os valores são copiados
para o hash map. Valores sem 'Copy' como 'String' são *movidos* e o hash map
passa a ser o dono (owner) deles.
*/
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name: String = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);  // get retorna Option<&V>

    match score {
        None => println!("get --> {team_name} não tem score"),
        Some(i) => println!("get --> {team_name} tem score: {i}")
    }

    for (key, value) in &scores {
        println!("iterando --> {key}: {value}");
    }

    // lifecycles
    let nome_cor = String::from("Red");
    let numero: i32 = 10;
    scores.insert(nome_cor, numero);

    // Só incluir outro valor se a chave não existir entry
}