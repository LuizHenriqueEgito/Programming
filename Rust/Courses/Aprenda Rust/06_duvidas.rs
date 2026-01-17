/*
cargo build  --> fica muito pesado
edito o Cargo.toml  --> isso deixa o arquivo bem pequeno
```toml
[profile.release]
strip = true
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
```
*/
use std::io;

fn () {
    let mut s = String::new();
    println!("Digite um texto:");
    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    println!("Você digitou {}", s);
    // s.len() pega a string em binario assim caracteres especiais com a representação
    // maior do que um byte é contado mais vezes (como um emogi por exemplo)
    // ao invés de len() use chars()
    // trim é como o do SQL remove espaços e \n
    println!("Quantidade de letras {}", s.trim().chars().count());
    println!("Você digitou {}", s.to_uppercase());  // to_lowercase()
    println!("Você digitou {}", s.replace("L", "N"));
    println!("{}", "-".reapeat(40));  // repete - 40x
    let banner = 
        "Texto Separado \ 
        por virgula \
        exemplo: 1, 2, 3";
    println!("{:-^40}", "Calculadora"); // ----------------Calculadora------------------
    let banner = "Digite uma sequencia de números";
    println!("{banner");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");
    let numbers: Vec<i32> = s.split(",")
        .map(|c| c.trim().parse().expect("Error"))  // funções anonimas
        .collect();
    println!("Você digitou {:?}", numbers);
    let result: i32 = numbers.iter().sum();
    println!("O total é {}", result)

}