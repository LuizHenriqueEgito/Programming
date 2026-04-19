/*
Todo valor em Rust possui um dono (owner)
O Rust segue as seguintes regras:
1. Só pode haver um unico dono por vez
2. Quando o dono sai do escopo, o valor é dropado
3. Você pode MOVER ou EMPRESTAR (borrow)

Objetos que implementam Copy

Objetos sem Copy apenas com Move
*/

fn main() {
    let x = 10;
    let y = x;
    // x e y existem pois um int implementa Copy
    println!("x: {}, y: {}", x, y);
    println!("Olhando o endereço da memória:");
    println!("Endereço de x: {:p}", &x);
    // y tem um endereço diferente x pois são variaveis diferentes na Stack
    // Como x e y são variaveis diferentes temos valores diferentes
    // x e y não apontam para 10 x é 10 e y é 10
    println!("Endereço de y: {:p}", &y);
    println!("---");
    /*
    Aqui o valor de string_1 (olá) foi passado para string_2
    logo string_1 não existe mais tudo que pertencia a ele
    foi MOVIDO para string_2
    */
    let string_1 = String::from("olá");
    println!("Olhando o endereço da memória:");
    println!("Endereço de string_1: {:p}", string_1.as_ptr());
    let string_2 = string_1;
    // println!(string_1);  Isso não funciona mais string_1 não existe mais
    println!("Endereço de string_2: {:p}", string_2.as_ptr());
    println!("string_2: {}", string_2);

    // Usando Clone
    let string_3 = string_2.clone();
    /*
    Agora string_2 e string_3 existem
    Nos clonamos o valor de string_2 para outro lugar para o string_3
    Clone faz uma copia profunda
    */
    println!("string_2: {}", string_2);
    println!("string_2: {}", string_3);
    
    println!("---");
    println!("Olhando o endereço da memória:");
    println!("Endereço de string_2: {:p}", string_2.as_ptr());
    println!("Endereço de string_3: {:p}", string_3.as_ptr());

    // Borrowing 
    // Referencia IMUTÁVEL
    let string_main = String::from("eu vou ser emprestado");
    // Eu posso ver ela mas nunca alterar, isso não transfere o ownership
    let string_emprestada_imutavel = &string_main;
    println!("Valor original: {}", string_main);
    println!("Valor emprestado: {}", string_emprestada_imutavel);

    // Referencia MUTÁVEL
    let mut string_main = String::from("eu vou ser emprestado");
    let string_emprestada_mutavel = &mut string_main;
    // println!("Valor original: {}", string_main);  Não posso fazer isso eu preciso terminar o emprestimo antes de usar a variavel novamente
    println!("Valor original: {}", string_emprestada_mutavel);
    // Alterando a variavel
    string_emprestada_mutavel.push_str(" FUI ALTERADO!!!");
    println!("Valor original: {}", string_emprestada_mutavel);
    println!("Valor original: {}", string_main);
    println!("---");
    println!("Ownership em Funções:");
    let s = String::from("oi");
    take(s);  // sai vai cair aqui dentro e não vai mais voltar
    // println!("{}", s);  Isso não funciona mais
    let s = String::from("oi");
    read(&s);  // s vai continuar existindo
    println!("{}", s);
    let mut s = String::from("oi");
    change(&mut s);  // s vai existir e ser modificado
    println!("{}", s);
}

// Move
// isso move o que entra como s deixa de existir e a referncia morre após a função
fn take(s: String) {
    println!("{}", s);
}

// Borrow
// isso empresta então ainda continua existindo a referencia não vai para dentro da função
fn read(s: &String) {
    println!("{}", s)
}

// Mut Borrow
fn change(s: &mut String) {
    s.push_str("!!!");
}