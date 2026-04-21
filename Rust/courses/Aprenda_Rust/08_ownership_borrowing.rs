fn main() {
    let a: &str = "Nuna";  // "Nuna" está na static memory
    let b: &str = a;  // b copia "Nuna"

    let c: String = String::from("Nuna");  // "Nuna" está no HEAP
    let d: String = c;  // c para de existir é dropado pois d virá o owner de "Nuna"

    let name_0: &str = "Egito";  // é um valor copy
    say_hello_0(name_0);
    say_goodbye_0(name_0);

    // empresta só pode visualizar (Borrow)
    let name_1: String = String::from("Luiz");
    say_hello_1(&name_1);
    say_goodbye_1(&name_1);

    // emprestimo mutavel (não tem pode apenas de leitura mas também de edição)
    let mut name_2: String = String::from("Henrique");

    add_prefix(name_2);
    println!(name_2);
    add_sufix(name_2);
    println!(name_2);
}

fn say_hello_0(text: &str) {
    println!("Hello, {text}");
}

fn say_goodbye_0(text: &str) {
    println!("Goodbye, {text}");
}

// Empretos o valor com &
fn say_hello_1(text: &String) {
    println!("Hello, {text}");
}

// Empretos o valor com &
fn say_goodbye_1(text: &String) {
    println!("Goodbye, {text}");
}

fn add_prefix(text: &mut String) {
    *text = format!("FOO_{text}");
}

fn add_sufix(text: &mut String) {
    text.push_str("_BAR");  // Rust já faz a dereferencia implicita. 
}