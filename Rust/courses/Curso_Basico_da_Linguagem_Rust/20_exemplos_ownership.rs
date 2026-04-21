/*
Semantica 'copy'
- Disponivel quando o tipo implementa o 'trait' Copy
    - Inteiros
    - Booleanos
    - Ponto Flutuante
    - Caracteres
    - Tuplas e Arrays apenas com tipos que suportam Copy:
        - (i32, bool): sim
        - (i32, String): não

Semantica 'move'
- Move o proprietario do dado (valor)
*/

fn main() {
    // Semantica Copy
    let x = 5;
    let y = x;
    println!("y = {y}");
    println!("x = {x}");

    // Semantica Move
    let s1: String = String::from("hello");
    let s2: String = s1;  // Valor 'Tipo String' é MOVIDO para s2

    println!("s2= {s2}"); // s1 NÃO EXISTE mais
    
    // Podemos fazer um clone
    let s3 = String::from("hello for clone");
    let s4 = s3.clone();
    println!("s3: {s3}");
    println!("s4: {s4}");

    // Ainda é possivel só EMPRESTAR (BORROWING) imutável
    let s5: String = String::from("imutavel");
    let s6 = &s5;
    println!("s5: {s5}");
    println!("s6: {s6}");

    // Podemos emprestar mutalvel
    let mut s7: String = String::from("mutavel");
    {
        let s8: &mut String = &mut s7;
        // println!("s7: {s7}");  Não posso mexer em s7 enquanto ele estiver emprestado para s8
        s8.push_str(" agora");
        println!("s8: {s8}");
    } // s8 MORRE AQUI
    println!("s7: {s7}");
}