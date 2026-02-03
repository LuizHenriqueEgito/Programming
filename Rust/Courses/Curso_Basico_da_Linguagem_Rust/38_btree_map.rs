/*
Use o BTreeMap quando:
- Quando precisar de ordenação entre chaves

Ambos possuem: remove() | contains_key() | len()
*/
use std::collections::{HashMap, BTreeMap};

fn main() {
    let mut turma_hash: HashMap<&str, i32> = HashMap::new();
    let mut turma_btree: BTreeMap<&str, i32> = BTreeMap::new();

    let nomes: Vec<&str> = vec!["ana", "bia", "cal", "duda", "eli"];
    let notas: Vec<i32> = vec![10, 8, 5, 7, 2];

    for i in 0..5 {  // isso é igual a 0.=4
        turma_hash.insert(nomes[i], notas[i]);
        turma_btree.insert(nomes[i], notas[i]);
    }

    let aluno: &str = "ana";

    match turma_hash.get(&aluno) {
        Some(n) => println!("HashMap tem {} com nota {}", aluno, n),
        None => println!("HashMap não tem nota para {}", aluno),
    }

    match turma_btree.get(&aluno) {
        Some(n) => println!("BTreeMap tem {} com nota {}", aluno, n),
        None => println!("BTreeMap não tem nota para {}", aluno),
    }

    // Não mantém a ordem das coisas
    println!("\nIteração com HashMap");
    for (nome, nota) in &turma_hash {
        println!("iterando --> {}: {}", nome, nota);
    }
    // mantém a ordem
    println!("\nIteração com BTreeMap");
    for (nome, nota) in &turma_btree {
        println!("iterando --> {}: {}", nome, nota);
    }

    // Só BTreeMap tem:
    println!("\nIteração copm BTreeMap e um intervalo");
    let intervalo = turma_btree.range("b".."d");
    for (nome, nota) in intervalo {
        println!("iterando --> {}: {}", nome, nota);
    }
}