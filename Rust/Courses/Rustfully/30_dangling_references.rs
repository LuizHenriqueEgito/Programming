/*
Uma dangling reference (referência pendurada) é uma referência que aponta para memória que já foi liberada.
Você ainda tem um “endereço”, mas o dono daquela memória não existe mais.
*/
// isso não compila
fn dangling_reference() -> &i32 {
    let x = 10;
    &x
}

// isso compila
fn func() -> i32 {
    let x = 10;
    x
}

fn main() {
    let y = func();
    println!("y={y}");
}