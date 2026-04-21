/*
Várias referências imutáveis simultaneas para o mesmo valor é aceito
Mas a cada momento pode haver APENAS UMA referencia mutavel para um valor
A existencia de uma referencia mutavel impede a existencia de outras referencias de qualquer tipo

1 Escritor, N Leitores
*/
fn main() {
    let s: String = String::from("hello");
    println!("{s}");
    // change1(&s); // &s: pode acessar o s mas não pode destrui-lo

    let mut x: String = String::from("hello");
    change2(&mut x);
}

// fn change1(some_string: &String) {
//     some_string.push_str(", world");  // imútavel
// }

fn change2(some_string: &mut String) {  // &mut String emprestimo mutavel de uma string
    some_string.push_str(", world");
    println!("{}", some_string);
}