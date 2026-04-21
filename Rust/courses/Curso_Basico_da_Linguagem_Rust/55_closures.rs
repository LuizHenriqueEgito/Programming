/*
Funções anonimas em Rust também são closures e são mais
conhecidas por esse nome: `closure`
*/

fn main() {
    // Fn: Só lê o ambiente
    let x = 10;
    let f_fn = |y| {
        println!("Fn: x = {}", x + y);
    };
    // f_fn tem acesso a x, mesmo ele estando fora do escopo
    f_fn(10);
    f_fn(2);

    // FnMut: Pode modificar o ambiente
    let mut w = 0;
    println!("Primeiro valor de w = {w}");
    let mut f_fnmut = || {
        w += 1;
        println!("FnMut: w = {}", w);
    };
    f_fnmut();
    f_fnmut();
    println!("O novo valor de w: {w}");

    // FnOnce: Consome o ambiente
    let s = String::from("Hello");
    let f_fnonce = move || {
        nothing_fn(s)  // Aqui s é dropada
    };
    f_fnonce();
    // println!("s existe: {}", s);  -> s não existe mais
}

fn nothing_fn(s: String) {
    println!("s: {s}");
    println!("Do nothing...");
}