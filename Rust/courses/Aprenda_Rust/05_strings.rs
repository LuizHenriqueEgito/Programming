/*
Trabalhando com texto em RUST
Aspas simples '' caracter
Aspas duplas "" strng liteal

No Rust existe o tipo str, mas na hora de tipar usamos &str
&str -> string slice
Ele é uma referencia para uma porção de string que temos armazenado
na memória estatica.
Mas porque &str e não str, por que str não tem um tamanho fixo a gente não sabe
onde começa e onde termina a string no binario que está na memória estatica,
quando usamos string slice &str temos a nossa referencia a string que queremos
no nosso binario e ai tudo faz sentido.
Pense assim:
Quando famozer let nome: &str = "Luiz"
a string Luiz vai para a memoria estatica e lá ficam todo o binario do programa
Luiz está junto, quando usamos &str apontamos para Luiz no binario, só str (não
necessariamente é isso) apontaria para todo o binario.
&str tem uma referencia de onde essa string começa e onde ela termina.
&str é imutavel

Quando não sabemos qual será o tamanho da string usamos String::
String:: -> String dinamica, vai para o Heap

*/
use std::io;


fn main() {
    // &str
    let nome: &str = "Luiz";  // Faz parte da memória estatica nunca será desalocado do programa
    println!(nome)
    // String que está na Heap -> String owned
    let mut nome2: String = String::new();
    nome2.push('L');
    nome2.push('u');
    nome2.push('i');
    nome2.push('z');
    println!(nome2)
    let mut nome3 = String::new();
    nome3.push_str("Luiz");
    println!(nome3);
    let nome4: String = "Luiz".to_string();
    let nome5: String = Strinng::from("Luiz");
    let nome_vec = ['L', 'u', 'i', 'z'];
    let nome6: String = String::from_iter(nome_vec);
    println!(nome6);
    let nome7: String = "Luiz".into();  // só funciona se for tipado String
    println!(nome5, nome6, nome7);
    
    // Exemplo
    let mut s: String = String::new(); // String vazia
    println!("Digite um texto:")
    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    println!("Você digitou: {string}");

}